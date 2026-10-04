//! Championship : saisons et classements (feature 013).
//! - `GET /api/jeu/saisons` (public)
//! - `GET /api/jeu/classements/membres` (public ; `moi` si un jeton est présent)
//! - `GET /api/jeu/classements/pays` (public)
//! - `GET /api/jeu/pays/{pays_id}` (public : la fiche d'un pays sur la carte)
//!
//! Les classements ne sont pas stockés : ils se lisent dans `jeu.score_saison`,
//! l'agrégat tenu à chaque gain. Deux règles valent pour toutes les lectures :
//!
//! - un compte suspendu ou supprimé n'y figure pas, et y revient avec son score
//!   s'il est rétabli (le filtre est à la lecture, rien n'est effacé) ;
//! - à égalité de score, le premier à l'avoir atteint est classé devant.
//!
//! Rien ne « passe » d'une saison à l'autre : l'état d'une saison se déduit de
//! ses dates. Seule l'attribution des distinctions de podium est un effet, et
//! elle se fait paresseusement, à la première lecture après la fin.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::constants::afripulse_pays_autorises::PAYS_AFRICAINS_ISO2;
use crate::errors::ApiErreur;
use crate::handlers::jeu::joueur_optionnel;
use crate::models::jeu::{
    ClassementMembres, ClassementQueryParams, FichePaysJeu, LigneClassement, LignePays,
    MaPosition, ModulePays, SaisonResponse, SaisonRow, SaisonsResponse, SAISON_COLONNES,
};
use crate::services::jeu::SERVABLE_SQL;
use crate::services::{engagement, jeu as moteur};
use crate::ApiResponse;

/// Places d'honneur distinguées en fin de saison, tous membres confondus.
const PODIUM_GLOBAL: i64 = 3;
/// Voisins affichés de part et d'autre du membre.
const VOISINS: i64 = 2;

fn codes_pays() -> Vec<String> {
    PAYS_AFRICAINS_ISO2.iter().map(|c| c.to_lowercase()).collect()
}

/// Classement des membres d'une saison, éventuellement restreint à un pays.
///
/// `$1` : saison, `$2` : pays (ou NULL). Un membre qui a changé de pays en cours
/// de saison a plusieurs lignes dans `score_saison` : il figure UNE fois au
/// classement de tous les membres, avec la somme, et une fois dans chaque
/// classement de pays, pour le score gagné sous ce pays (FR-054).
///
/// Le pays affiché est celui du filtre s'il y en a un, sinon le pays de
/// rattachement actuel du membre.
const CLASSEMENT_MEMBRES: &str = "
    WITH cumul AS (
        SELECT ss.utilisateur_id, SUM(ss.score)::int AS score, MAX(ss.atteint_at) AS atteint_at
          FROM jeu.score_saison ss
          JOIN iam.utilisateur u ON u.id = ss.utilisateur_id
         WHERE ss.saison_id = $1
           AND ($2::uuid IS NULL OR ss.pays_id = $2)
           AND u.etat = 'actif' AND u.deleted_at IS NULL
         GROUP BY ss.utilisateur_id
        HAVING SUM(ss.score) > 0
    ),
    classe AS (
        SELECT c.*, ROW_NUMBER() OVER (ORDER BY c.score DESC, c.atteint_at ASC, c.utilisateur_id) AS rang
          FROM cumul c
    )
    SELECT cl.rang, u.id AS utilisateur_id, u.nom, u.prenom, u.slug, u.photo_url,
           p.nom AS pays, LOWER(p.code_iso2) AS pays_iso2,
           n.code AS niveau_code, n.libelle AS niveau_libelle, cl.score
      FROM classe cl
      JOIN iam.utilisateur u ON u.id = cl.utilisateur_id
      LEFT JOIN shared.pays p
             ON p.id = COALESCE($2::uuid, u.pays_origine_id, u.pays_residence_id)
      LEFT JOIN engagement.compte ec ON ec.utilisateur_id = u.id
      LEFT JOIN engagement.niveau n ON n.code = COALESCE(ec.niveau_code, 'membre')";

pub async fn saison_courante(pool: &PgPool) -> Result<Option<SaisonRow>, sqlx::Error> {
    sqlx::query_as::<_, SaisonRow>(&format!(
        "SELECT {SAISON_COLONNES} FROM jeu.saison WHERE debut_at <= NOW() AND NOW() < fin_at LIMIT 1"
    ))
    .fetch_optional(pool)
    .await
}

/// Rang et score d'un membre dans une saison (et, s'il est donné, un pays).
/// `None` s'il n'y a aucun score.
pub async fn position_du_membre(
    pool: &PgPool,
    saison_id: Uuid,
    pays_id: Option<Uuid>,
    utilisateur_id: Uuid,
) -> Result<Option<(i64, i32)>, sqlx::Error> {
    let ligne = sqlx::query_as::<_, LigneClassement>(&format!(
        "SELECT * FROM ({CLASSEMENT_MEMBRES}) c WHERE c.utilisateur_id = $3"
    ))
    .bind(saison_id)
    .bind(pays_id)
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?;
    Ok(ligne.map(|l| (l.rang, l.score)))
}

/// Clôture PARESSEUSE des saisons échues : attribue les distinctions de podium,
/// une fois.
///
/// `UPDATE … WHERE cloturee_at IS NULL AND fin_at <= NOW() RETURNING id` : seul
/// l'appel qui pose `cloturee_at` voit la saison revenir, donc deux lecteurs
/// simultanés n'attribuent pas deux fois. La clé d'idempotence du mouvement
/// (`jeu:podium:{saison}:{membre}`) reste le second filet.
///
/// Le classement lui-même n'a pas besoin d'être « figé » : plus aucun gain ne
/// porte cette saison une fois sa date de fin passée.
pub async fn cloturer_saisons_echues(pool: &PgPool) -> Result<(), sqlx::Error> {
    let echues: Vec<Uuid> = sqlx::query_scalar(
        "UPDATE jeu.saison SET cloturee_at = NOW(), updated_at = NOW()
          WHERE cloturee_at IS NULL AND fin_at <= NOW()
          RETURNING id",
    )
    .fetch_all(pool)
    .await?;

    for saison_id in echues {
        // Les premiers, tous membres confondus.
        let mut laureats: Vec<Uuid> = sqlx::query_scalar(&format!(
            "SELECT c.utilisateur_id FROM ({CLASSEMENT_MEMBRES}) c
              WHERE c.rang <= {PODIUM_GLOBAL} ORDER BY c.rang"
        ))
        .bind(saison_id)
        .bind(None::<Uuid>)
        .fetch_all(pool)
        .await?;

        // Le premier de chaque pays d'Afrique.
        let premiers: Vec<Uuid> = sqlx::query_scalar(
            "SELECT DISTINCT ON (ss.pays_id) ss.utilisateur_id
               FROM jeu.score_saison ss
               JOIN iam.utilisateur u ON u.id = ss.utilisateur_id
               JOIN shared.pays p ON p.id = ss.pays_id
              WHERE ss.saison_id = $1 AND ss.score > 0
                AND u.etat = 'actif' AND u.deleted_at IS NULL
                AND LOWER(p.code_iso2) = ANY($2)
              ORDER BY ss.pays_id, ss.score DESC, ss.atteint_at ASC",
        )
        .bind(saison_id)
        .bind(codes_pays())
        .fetch_all(pool)
        .await?;

        laureats.extend(premiers);
        laureats.sort();
        laureats.dedup();

        for utilisateur_id in laureats {
            engagement::attribuer(
                pool,
                utilisateur_id,
                "jeu_podium_saison",
                Some("saison"),
                Some(saison_id),
                &format!("jeu:podium:{saison_id}:{utilisateur_id}"),
            )
            .await;
        }
    }
    Ok(())
}

/// GET /api/jeu/saisons : la saison en cours, celles à venir, les archives.
pub async fn lister_saisons(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    cloturer_saisons_echues(pool.get_ref()).await?;

    let toutes = sqlx::query_as::<_, SaisonRow>(&format!(
        "SELECT {SAISON_COLONNES} FROM jeu.saison ORDER BY debut_at DESC"
    ))
    .fetch_all(pool.get_ref())
    .await?;

    let mut reponse = SaisonsResponse { courante: None, a_venir: Vec::new(), archives: Vec::new() };
    for saison in toutes.into_iter().map(SaisonResponse::from) {
        match saison.etat {
            "en_cours" => reponse.courante = Some(saison),
            "a_venir" => reponse.a_venir.push(saison),
            _ => reponse.archives.push(saison),
        }
    }
    // Les saisons à venir dans l'ordre où elles arriveront.
    reponse.a_venir.reverse();

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(reponse), error: None }))
}

/// Saison demandée, ou à défaut la saison en cours. `None` hors saison.
async fn resoudre_saison(pool: &PgPool, demandee: Option<Uuid>) -> Result<Option<Uuid>, ApiErreur> {
    match demandee {
        Some(id) => {
            let existe: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM jeu.saison WHERE id = $1)")
                    .bind(id)
                    .fetch_one(pool)
                    .await?;
            if !existe {
                return Err(ApiErreur::NonTrouve("Saison introuvable".into()));
            }
            Ok(Some(id))
        }
        None => Ok(saison_courante(pool).await?.map(|s| s.id)),
    }
}

/// GET /api/jeu/classements/membres : tous les membres, ou ceux d'un pays.
pub async fn classement_membres(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<ClassementQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(50).clamp(1, 100);

    let Some(saison_id) = resoudre_saison(pool.get_ref(), params.saison).await? else {
        // Hors saison : un classement vide, pas une erreur.
        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(ClassementMembres { elements: Vec::new(), total: 0, page, taille, moi: None }),
            error: None,
        }));
    };

    let total: i64 =
        sqlx::query_scalar(&format!("SELECT COUNT(*) FROM ({CLASSEMENT_MEMBRES}) c"))
            .bind(saison_id)
            .bind(params.pays)
            .fetch_one(pool.get_ref())
            .await?;

    let elements = sqlx::query_as::<_, LigneClassement>(&format!(
        "SELECT * FROM ({CLASSEMENT_MEMBRES}) c ORDER BY c.rang LIMIT $3 OFFSET $4"
    ))
    .bind(saison_id)
    .bind(params.pays)
    .bind(taille)
    .bind((page - 1) * taille)
    .fetch_all(pool.get_ref())
    .await?;

    // Le membre voit son rang même hors des premières places (FR-059).
    let mut moi = None;
    if let Some(utilisateur_id) = joueur_optionnel(&req) {
        if let Some((rang, score)) =
            position_du_membre(pool.get_ref(), saison_id, params.pays, utilisateur_id).await?
        {
            let voisins = sqlx::query_as::<_, LigneClassement>(&format!(
                "SELECT * FROM ({CLASSEMENT_MEMBRES}) c
                  WHERE c.rang BETWEEN $3 AND $4 ORDER BY c.rang"
            ))
            .bind(saison_id)
            .bind(params.pays)
            .bind(rang - VOISINS)
            .bind(rang + VOISINS)
            .fetch_all(pool.get_ref())
            .await?;
            moi = Some(MaPosition { rang, score, voisins });
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ClassementMembres { elements, total, page, taille, moi }),
        error: None,
    }))
}

/// Classement des pays d'une saison. `$1` : saison, `$2` : nombre de joueurs
/// comptés par pays, `$3` : codes ISO2 des pays d'Afrique.
///
/// Le score d'un pays est la somme de ses N meilleurs joueurs : une somme de
/// tous donnerait la victoire au pays le plus peuplé, une moyenne au pays d'un
/// seul joueur brillant. Les 55 pays y figurent TOUS, y compris à score 0.
pub const CLASSEMENT_PAYS: &str = "
    WITH par_membre AS (
        SELECT ss.pays_id, ss.score, ss.atteint_at,
               ROW_NUMBER() OVER (PARTITION BY ss.pays_id
                                  ORDER BY ss.score DESC, ss.atteint_at ASC) AS rn
          FROM jeu.score_saison ss
          JOIN iam.utilisateur u ON u.id = ss.utilisateur_id
         WHERE ss.saison_id = $1 AND ss.pays_id IS NOT NULL AND ss.score > 0
           AND u.etat = 'actif' AND u.deleted_at IS NULL
    ),
    par_pays AS (
        SELECT pays_id,
               COALESCE(SUM(score) FILTER (WHERE rn <= $2), 0)::int AS score,
               COUNT(*) AS joueurs,
               MAX(atteint_at) FILTER (WHERE rn <= $2) AS atteint_at
          FROM par_membre
         GROUP BY pays_id
    )
    SELECT CASE WHEN COALESCE(pp.score, 0) > 0
                THEN ROW_NUMBER() OVER (ORDER BY COALESCE(pp.score, 0) DESC,
                                                 pp.atteint_at ASC NULLS LAST, p.nom)
           END AS rang,
           p.id AS pays_id, p.nom, LOWER(p.code_iso2) AS iso2,
           COALESCE(pp.score, 0) AS score, COALESCE(pp.joueurs, 0) AS joueurs
      FROM shared.pays p
      LEFT JOIN par_pays pp ON pp.pays_id = p.id
     WHERE LOWER(p.code_iso2) = ANY($3)";

/// GET /api/jeu/classements/pays : les 55 pays d'Afrique. C'est la source de
/// la carte.
pub async fn classement_pays(
    pool: web::Data<PgPool>,
    params: web::Query<ClassementQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let regles = moteur::charger_regles(pool.get_ref()).await?;
    let saison_id = resoudre_saison(pool.get_ref(), params.saison).await?;

    // Hors saison, on sert quand même les 55 pays, tous à zéro : la carte reste
    // dessinable. `$1` NULL ne correspond à aucune ligne de `score_saison`.
    let pays = sqlx::query_as::<_, LignePays>(&format!(
        "SELECT * FROM ({CLASSEMENT_PAYS}) c ORDER BY c.score DESC, c.rang NULLS LAST, c.nom"
    ))
    .bind(saison_id)
    .bind(i64::from(regles.joueurs_par_pays))
    .bind(codes_pays())
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(pays), error: None }))
}

/// Joueurs d'un pays montrés sur sa fiche.
const MEILLEURS_DU_PAYS: i64 = 5;

/// GET /api/jeu/pays/{pays_id} : ce que montre la carte quand on choisit un
/// pays (FR-065). Un pays sans score n'est pas une erreur : il s'annonce comme
/// sans joueur (FR-067), et l'on peut quand même jouer sur lui.
pub async fn fiche_pays_jeu(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let pays_id = path.into_inner();
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let pays: Option<(String, Option<String>)> = sqlx::query_as(
        "SELECT nom, LOWER(code_iso2) FROM shared.pays WHERE id = $1 AND LOWER(code_iso2) = ANY($2)",
    )
    .bind(pays_id)
    .bind(codes_pays())
    .fetch_optional(pool.get_ref())
    .await?;
    let Some((nom, iso2)) = pays else {
        return Err(ApiErreur::NonTrouve("Pays introuvable".into()));
    };

    let saison = saison_courante(pool.get_ref()).await?;
    let (rang, score, joueurs, meilleurs) = match &saison {
        Some(s) => {
            let ligne = sqlx::query_as::<_, LignePays>(&format!(
                "SELECT * FROM ({CLASSEMENT_PAYS}) c WHERE c.pays_id = $4"
            ))
            .bind(s.id)
            .bind(i64::from(regles.joueurs_par_pays))
            .bind(codes_pays())
            .bind(pays_id)
            .fetch_optional(pool.get_ref())
            .await?;
            let meilleurs = sqlx::query_as::<_, LigneClassement>(&format!(
                "SELECT * FROM ({CLASSEMENT_MEMBRES}) c ORDER BY c.rang LIMIT {MEILLEURS_DU_PAYS}"
            ))
            .bind(s.id)
            .bind(Some(pays_id))
            .fetch_all(pool.get_ref())
            .await?;
            match ligne {
                Some(l) => (l.rang, l.score, l.joueurs, meilleurs),
                None => (None, 0, 0, meilleurs),
            }
        }
        None => (None, 0, 0, Vec::new()),
    };

    // Épreuves servables portant sur CE pays, par module : de quoi lancer une
    // partie « sur ce pays » depuis la carte (FR-066).
    let mut modules = sqlx::query_as::<_, ModulePays>(&format!(
        "SELECT mo.code, mo.libelle, COUNT(j.id) AS epreuves_jouables
           FROM jeu.module mo
           JOIN (SELECT e.id, e.module_code {SERVABLE_SQL} AND e.pays_id = $1) j
             ON j.module_code = mo.code
          WHERE mo.ouvert
          GROUP BY mo.code, mo.libelle, mo.ordre
          ORDER BY mo.ordre"
    ))
    .bind(pays_id)
    .fetch_all(pool.get_ref())
    .await?;
    for module in &mut modules {
        module.disponible = module.epreuves_jouables >= i64::from(regles.taille_partie);
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(FichePaysJeu { pays_id, nom, iso2, rang, score, joueurs, meilleurs, modules }),
        error: None,
    }))
}

//! Endpoints publics et membres du jeu (feature 013).
//! - `GET  /api/jeu/modules` (public)
//! - `POST /api/jeu/parties` (membre)
//! - `GET  /api/jeu/parties/{id}` (membre)
//! - `POST /api/jeu/parties/{id}/suivante` (membre)
//! - `POST /api/jeu/parties/{id}/repondre` (membre)
//! - `POST /api/jeu/parties/{id}/injouable` (membre)
//! - `POST /api/jeu/epreuves/{id}/signaler` (membre)
//! - `GET  /api/jeu/moi` (membre)
//! - `GET  /api/jeu/moi/parties` (membre)
//!
//! Contrat : `specs/013-activites-ludiques/contracts/api-membre.md`.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::jeu_classement::{position_du_membre, saison_courante};
use crate::handlers::jeu_duel::apres_partie_de_duel;
use crate::jwt;
use crate::models::jeu::{
    CreerPartieRequest, HistoriqueParties, MaSaison, ModuleJeu, MonJeu, PageQueryParams,
    PartieHistorique, PartieResponse, PartieRow, PaysRattachement, RangRequest, ReglesJeu,
    ReponseBilan, RepondreRequest, ScoreParModule, SignalerEpreuveRequest, MOTIFS_SIGNALEMENT,
    PARTIE_COLONNES,
};
use crate::services::jeu::{self as moteur, SERVABLE_SQL};
use crate::services::messagerie_sse::RegistreSse;
use crate::ApiResponse;

/// Durée pendant laquelle une partie interrompue peut être reprise (FR-019).
const REPRISE_HEURES: i64 = 24;

/// Lit l'identifiant du membre dans le JWT, sans consulter la base.
pub fn joueur_optionnel(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Garde de toute route de jeu réservée aux membres.
///
/// Le JWT ne porte que `sub` : un jeton émis avant une suspension reste accepté
/// jusqu'à son expiration. L'état du compte se vérifie donc EN BASE, à chaque
/// appel (FR-002).
pub async fn garde_joueur(pool: &PgPool, req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    let utilisateur_id = joueur_optionnel(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let etat: Option<String> = sqlx::query_scalar(
        "SELECT etat::text FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?;

    match etat.as_deref() {
        Some("actif") => Ok(utilisateur_id),
        Some(_) => Err(ApiErreur::AccesInterdit(
            "Votre compte ne permet pas de jouer pour le moment".into(),
        )),
        None => Err(ApiErreur::NonAutorise("Compte introuvable".into())),
    }
}

/// GET /api/jeu/modules : modules ouverts et leur disponibilité.
pub async fn lister_modules(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let mut modules = sqlx::query_as::<_, ModuleJeu>(&format!(
        "SELECT mo.code, mo.libelle, mo.route, mo.icone, COUNT(j.id) AS epreuves_jouables
           FROM jeu.module mo
           LEFT JOIN (SELECT e.id, e.module_code {SERVABLE_SQL}) j ON j.module_code = mo.code
          WHERE mo.ouvert
          GROUP BY mo.code, mo.libelle, mo.route, mo.icone, mo.ordre
          ORDER BY mo.ordre"
    ))
    .fetch_all(pool.get_ref())
    .await?;

    for module in &mut modules {
        module.disponible = module.epreuves_jouables >= i64::from(regles.taille_partie);
    }

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(modules), error: None }))
}

/// Charge la partie du membre sous verrou. 404 si elle n'est pas la sienne :
/// on ne dit pas qu'elle existe.
async fn charger_partie_verrouillee(
    conn: &mut PgConnection,
    partie_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<PartieRow, ApiErreur> {
    sqlx::query_as::<_, PartieRow>(&format!(
        "SELECT {PARTIE_COLONNES} FROM jeu.partie
          WHERE id = $1 AND utilisateur_id = $2 FOR UPDATE"
    ))
    .bind(partie_id)
    .bind(utilisateur_id)
    .fetch_optional(conn)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Partie introuvable".into()))
}

fn reponse_partie(partie: &PartieRow, regles: &ReglesJeu) -> PartieResponse {
    PartieResponse {
        id: partie.id,
        cadre: partie.cadre.clone(),
        module: partie.module_code.clone(),
        defi_id: partie.defi_id,
        duel_id: partie.duel_id,
        nombre_epreuves: partie.epreuve_ids.len(),
        rang_courant: partie.rang_courant,
        etat: partie.etat.clone(),
        temps_epreuve_s: regles.temps_epreuve_s,
        bonnes: partie.bonnes,
        score_gagne: partie.score_gagne,
        score_total: None,
        reponses: None,
        rang_saison: None,
    }
}

/// POST /api/jeu/parties : lance une partie libre, ou un entraînement s'il est
/// demandé explicitement.
pub async fn creer_partie(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerPartieRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = garde_joueur(pool.get_ref(), &req).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let module = body.module.trim();
    let theme = body.theme.as_deref().map(str::trim).filter(|t| !t.is_empty());

    let ouvert: Option<bool> = sqlx::query_scalar("SELECT ouvert FROM jeu.module WHERE code = $1")
        .bind(module)
        .fetch_optional(pool.get_ref())
        .await?;
    match ouvert {
        None => return Err(ApiErreur::NonTrouve("Module inconnu".into())),
        Some(false) => {
            return Err(ApiErreur::Conflit("Les activités de ce module sont fermées".into()))
        }
        Some(true) => {}
    }

    let servables =
        moteur::compter_servables(pool.get_ref(), Some(module), body.pays_id, theme).await?;
    if servables < i64::from(regles.taille_partie) {
        return Err(ApiErreur::Conflit(
            "Il n'y a pas encore assez d'épreuves pour lancer une partie ici".into(),
        ));
    }

    let mut tx = pool.begin().await?;

    // Une seule partie libre ouverte à la fois : en lancer une nouvelle clôt la
    // précédente, et compte sans réponse l'épreuve qui y était affichée. Sans
    // cela, on pourrait ouvrir une partie, lire la question, l'abandonner, et
    // retrouver la même question « neuve » dans la partie suivante.
    let ouvertes = sqlx::query_as::<_, PartieRow>(&format!(
        "SELECT {PARTIE_COLONNES} FROM jeu.partie
          WHERE utilisateur_id = $1 AND etat = 'en_cours'
            AND cadre IN ('libre', 'entrainement')
          FOR UPDATE"
    ))
    .bind(utilisateur_id)
    .fetch_all(&mut *tx)
    .await?;
    for mut ouverte in ouvertes {
        moteur::clore(&mut tx, &mut ouverte, &regles).await?;
    }

    let mut cadre = "libre";
    let mut serie = moteur::composer_serie(
        &mut tx,
        utilisateur_id,
        Some(module),
        body.pays_id,
        theme,
        regles.taille_partie,
        true,
    )
    .await?;

    if serie.len() < regles.taille_partie as usize {
        if !body.entrainement.unwrap_or(false) {
            // Le 409 est ce qui garantit que l'entraînement est ANNONCÉ avant de
            // commencer : le client ne peut pas y tomber sans l'avoir demandé.
            return Err(ApiErreur::Conflit(
                "Vous avez joué toutes les épreuves : seule une partie d'entraînement est possible"
                    .into(),
            ));
        }
        cadre = "entrainement";
        serie = moteur::composer_serie(
            &mut tx,
            utilisateur_id,
            Some(module),
            body.pays_id,
            theme,
            regles.taille_partie,
            false,
        )
        .await?;
    }

    sqlx::query(
        "INSERT INTO jeu.joueur (utilisateur_id) VALUES ($1) ON CONFLICT (utilisateur_id) DO NOTHING",
    )
    .bind(utilisateur_id)
    .execute(&mut *tx)
    .await?;

    let partie = sqlx::query_as::<_, PartieRow>(&format!(
        "INSERT INTO jeu.partie (utilisateur_id, module_code, cadre, theme, pays_id, epreuve_ids)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING {PARTIE_COLONNES}"
    ))
    .bind(utilisateur_id)
    .bind(module)
    .bind(cadre)
    .bind(theme)
    .bind(body.pays_id)
    .bind(&serie)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(reponse_partie(&partie, &regles)),
        error: None,
    }))
}

/// GET /api/jeu/parties/{id} : état de la partie, et son bilan si elle est
/// terminée ou close.
pub async fn obtenir_partie(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = garde_joueur(pool.get_ref(), &req).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let mut tx = pool.begin().await?;
    let mut partie = charger_partie_verrouillee(&mut tx, path.into_inner(), utilisateur_id).await?;

    // Une partie interrompue ne se reprend que pendant 24 h : passé ce délai,
    // elle est close en l'état, à la lecture, sans traitement de fond.
    let perimee = chrono::Utc::now() - partie.created_at > chrono::Duration::hours(REPRISE_HEURES);
    if partie.etat == "en_cours" && perimee {
        moteur::clore(&mut tx, &mut partie, &regles).await?;
    }
    tx.commit().await?;

    let mut reponse = reponse_partie(&partie, &regles);

    if partie.etat != "en_cours" {
        reponse.reponses = Some(
            sqlx::query_as::<_, ReponseBilan>(
                "SELECT rang, epreuve_id, issue FROM jeu.reponse
                  WHERE partie_id = $1 ORDER BY rang",
            )
            .bind(partie.id)
            .fetch_all(pool.get_ref())
            .await?,
        );
        reponse.score_total = Some(
            sqlx::query_scalar::<_, i32>(
                "SELECT score_total FROM jeu.joueur WHERE utilisateur_id = $1",
            )
            .bind(utilisateur_id)
            .fetch_optional(pool.get_ref())
            .await?
            .unwrap_or(0),
        );
        if let Some(saison) = saison_courante(pool.get_ref()).await? {
            reponse.rang_saison =
                position_du_membre(pool.get_ref(), saison.id, None, utilisateur_id)
                    .await?
                    .map(|(rang, _)| rang);
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(reponse), error: None }))
}

/// POST /api/jeu/parties/{id}/suivante : présente l'épreuve suivante.
pub async fn partie_suivante(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = garde_joueur(pool.get_ref(), &req).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let mut tx = pool.begin().await?;
    let mut partie = charger_partie_verrouillee(&mut tx, path.into_inner(), utilisateur_id).await?;
    let (presentation, vient_de_finir) =
        moteur::presenter_suivante(&mut tx, &mut partie, &regles).await?;
    tx.commit().await?;

    if vient_de_finir {
        moteur::apres_fin_de_partie(pool.get_ref(), &partie).await;
        // Une partie de duel qui se termine fait avancer le duel.
        apres_partie_de_duel(pool.get_ref(), &sse, &partie).await;
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(presentation),
        error: None,
    }))
}

/// POST /api/jeu/parties/{id}/repondre : enregistre la réponse à l'épreuve en
/// cours et renvoie sa correction.
pub async fn partie_repondre(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
    body: web::Json<RepondreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = garde_joueur(pool.get_ref(), &req).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let mut tx = pool.begin().await?;
    let mut partie = charger_partie_verrouillee(&mut tx, path.into_inner(), utilisateur_id).await?;
    let (correction, vient_de_finir) =
        moteur::repondre(&mut tx, &mut partie, &regles, body.rang, body.cle, None).await?;
    tx.commit().await?;

    if vient_de_finir {
        moteur::apres_fin_de_partie(pool.get_ref(), &partie).await;
        // Une partie de duel qui se termine fait avancer le duel.
        apres_partie_de_duel(pool.get_ref(), &sse, &partie).await;
    }

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(correction), error: None }))
}

/// POST /api/jeu/parties/{id}/injouable : le média de l'épreuve ne se charge
/// pas. L'épreuve est consommée, ne rapporte rien et n'est pas comptée comme
/// mauvaise. Déclarer une épreuve injouable ne fait donc jamais gagner de score.
pub async fn partie_injouable(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
    body: web::Json<RangRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = garde_joueur(pool.get_ref(), &req).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let mut tx = pool.begin().await?;
    let mut partie = charger_partie_verrouillee(&mut tx, path.into_inner(), utilisateur_id).await?;
    let (correction, vient_de_finir) =
        moteur::repondre(&mut tx, &mut partie, &regles, body.rang, None, Some("injouable")).await?;
    tx.commit().await?;

    if vient_de_finir {
        moteur::apres_fin_de_partie(pool.get_ref(), &partie).await;
        // Une partie de duel qui se termine fait avancer le duel.
        apres_partie_de_duel(pool.get_ref(), &sse, &partie).await;
    }

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(correction), error: None }))
}

/// POST /api/jeu/epreuves/{id}/signaler : un membre signale une épreuve à
/// laquelle il a répondu (réponse erronée, énoncé ambigu, contenu déplacé).
///
/// Une seule fois par épreuve. Sur une épreuve déjà retirée, le signalement est
/// accepté et classé d'office : le refuser laisserait le membre sans réponse.
pub async fn signaler_epreuve(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<SignalerEpreuveRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = garde_joueur(pool.get_ref(), &req).await?;
    let epreuve_id = path.into_inner();

    let motif = body.motif.trim();
    if !MOTIFS_SIGNALEMENT.contains(&motif) {
        return Err(ApiErreur::Validation("Motif de signalement inconnu".into()));
    }
    let commentaire = body.commentaire.as_deref().map(str::trim).filter(|c| !c.is_empty());

    let etat_epreuve: Option<String> =
        sqlx::query_scalar("SELECT etat FROM jeu.epreuve WHERE id = $1")
            .bind(epreuve_id)
            .fetch_optional(pool.get_ref())
            .await?;
    let Some(etat_epreuve) = etat_epreuve else {
        return Err(ApiErreur::NonTrouve("Épreuve introuvable".into()));
    };

    // On ne signale que ce qu'on a vu : une réponse `injouable` ne compte pas,
    // l'épreuve n'a pas été réellement présentée.
    let a_repondu: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM jeu.reponse
                        WHERE utilisateur_id = $1 AND epreuve_id = $2 AND issue <> 'injouable')",
    )
    .bind(utilisateur_id)
    .bind(epreuve_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !a_repondu {
        return Err(ApiErreur::AccesInterdit(
            "Vous ne pouvez signaler qu'une épreuve à laquelle vous avez répondu".into(),
        ));
    }

    let deja_traitee = matches!(etat_epreuve.as_str(), "retiree" | "rejetee");
    let inseres = sqlx::query(
        "INSERT INTO jeu.signalement_epreuve (epreuve_id, utilisateur_id, motif, commentaire, etat)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (epreuve_id, utilisateur_id) DO NOTHING",
    )
    .bind(epreuve_id)
    .bind(utilisateur_id)
    .bind(motif)
    .bind(commentaire)
    .bind(if deja_traitee { "classe" } else { "en_attente" })
    .execute(pool.get_ref())
    .await?
    .rows_affected();

    if inseres == 0 {
        return Err(ApiErreur::Conflit("Vous avez déjà signalé cette épreuve".into()));
    }

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "deja_traitee": deja_traitee })),
        error: None,
    }))
}

/// GET /api/jeu/moi : le score du membre, sa saison, son pays de rattachement.
pub async fn mon_jeu(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = garde_joueur(pool.get_ref(), &req).await?;

    // Série AFFICHÉE : elle tient tant que le dernier défi terminé date
    // d'aujourd'hui ou d'hier (UTC).
    let joueur: Option<(i32, i16)> = sqlx::query_as(
        "SELECT score_total,
                (CASE WHEN serie_dernier_jour >= (NOW() AT TIME ZONE 'UTC')::date - 1
                      THEN serie_jours ELSE 0 END)::smallint
           FROM jeu.joueur WHERE utilisateur_id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (score_total, serie_jours) = joueur.unwrap_or((0, 0));

    // Le pays sous lequel le PROCHAIN gain sera classé : origine, repli sur la
    // résidence. Les gains déjà acquis gardent le pays qu'ils avaient alors.
    let pays_rattachement = sqlx::query_as::<_, PaysRattachement>(
        "SELECT p.id, p.nom
           FROM iam.utilisateur u
           JOIN shared.pays p ON p.id = COALESCE(u.pays_origine_id, u.pays_residence_id)
          WHERE u.id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let saison = match saison_courante(pool.get_ref()).await? {
        Some(saison) => {
            let position =
                position_du_membre(pool.get_ref(), saison.id, None, utilisateur_id).await?;
            Some(MaSaison {
                id: saison.id,
                nom: saison.nom,
                score: position.map(|(_, score)| score).unwrap_or(0),
                rang: position.map(|(rang, _)| rang),
            })
        }
        None => None,
    };

    let par_module = sqlx::query_as::<_, ScoreParModule>(
        "SELECT m.code, m.libelle, SUM(g.montant)::int AS score
           FROM jeu.gain g
           JOIN jeu.module m ON m.code = g.module_code
          WHERE g.utilisateur_id = $1 AND g.annule_at IS NULL
          GROUP BY m.code, m.libelle, m.ordre
          ORDER BY m.ordre",
    )
    .bind(utilisateur_id)
    .fetch_all(pool.get_ref())
    .await?;

    let (score_parties, score_defis, score_duels): (i32, i32, i32) = sqlx::query_as(
        "SELECT COALESCE(SUM(montant) FILTER (WHERE origine = 'partie'), 0)::int,
                COALESCE(SUM(montant) FILTER (WHERE origine = 'defi'), 0)::int,
                COALESCE(SUM(montant) FILTER (WHERE origine = 'duel'), 0)::int
           FROM jeu.gain WHERE utilisateur_id = $1 AND annule_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MonJeu {
            score_total,
            saison,
            pays_rattachement,
            serie_jours,
            par_module,
            score_parties,
            score_defis,
            score_duels,
        }),
        error: None,
    }))
}

/// GET /api/jeu/moi/parties : l'historique des parties terminées ou closes.
pub async fn mes_parties(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<PageQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = garde_joueur(pool.get_ref(), &req).await?;
    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(20).clamp(1, 100);

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM jeu.partie WHERE utilisateur_id = $1 AND etat <> 'en_cours'",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    let elements = sqlx::query_as::<_, PartieHistorique>(
        "SELECT id, cadre, module_code, etat, bonnes,
                cardinality(epreuve_ids) AS nombre_epreuves, score_gagne, created_at
           FROM jeu.partie
          WHERE utilisateur_id = $1 AND etat <> 'en_cours'
          ORDER BY created_at DESC
          LIMIT $2 OFFSET $3",
    )
    .bind(utilisateur_id)
    .bind(taille)
    .bind((page - 1) * taille)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(HistoriqueParties { elements, total, page, taille }),
        error: None,
    }))
}

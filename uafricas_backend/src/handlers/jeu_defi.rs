//! Défis du jour et de la semaine (feature 013).
//! - `GET  /api/jeu/defis/courants` (public ; `ma_partie` et la série si un jeton est présent)
//! - `POST /api/jeu/defis/{id}/jouer` (membre)
//! - `GET  /api/jeu/defis/{id}/resultats` (public ; `moi` si un jeton est présent)
//!
//! Un défi est une série IDENTIQUE pour tous, ouverte pendant une période en
//! temps universel. La participation d'un membre EST une partie (`cadre =
//! 'defi'`) : elle se joue par les routes de `handlers::jeu`, sur le même écran.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::jeu::{garde_joueur, joueur_optionnel};
use crate::models::jeu::{
    DefiResponse, DefiRow, DefisCourants, LigneResultatDefi, MaPartieDefi, PartieRow,
    ResultatsDefi, DEFI_COLONNES, PARTIE_COLONNES,
};
use crate::services::jeu as moteur;
use crate::ApiResponse;

/// Nombre de lignes du podium d'un défi.
const TAILLE_PODIUM: i64 = 20;

async fn ma_partie(
    pool: &PgPool,
    defi_id: Uuid,
    utilisateur_id: Option<Uuid>,
) -> Result<Option<MaPartieDefi>, sqlx::Error> {
    let Some(utilisateur_id) = utilisateur_id else {
        return Ok(None);
    };
    sqlx::query_as::<_, MaPartieDefi>(
        "SELECT id, etat, bonnes, score_gagne FROM jeu.partie
          WHERE defi_id = $1 AND utilisateur_id = $2",
    )
    .bind(defi_id)
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await
}

async fn reponse_defi(
    pool: &PgPool,
    defi: DefiRow,
    utilisateur_id: Option<Uuid>,
) -> Result<DefiResponse, sqlx::Error> {
    let ma_partie = ma_partie(pool, defi.id, utilisateur_id).await?;
    Ok(DefiResponse {
        fin_at: defi.fin_at(),
        id: defi.id,
        periodicite: defi.periodicite,
        periode_debut: defi.periode_debut,
        nombre_epreuves: defi.epreuve_ids.len(),
        titre: defi.titre,
        module: defi.module_code,
        ma_partie,
    })
}

/// GET /api/jeu/defis/courants : le défi du jour et celui de la semaine.
///
/// Les crée s'ils n'existent pas encore : le premier visiteur de la période les
/// compose, tous les suivants lisent les mêmes.
pub async fn defis_courants(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = joueur_optionnel(&req);
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let mut courants = DefisCourants {
        jour: None,
        semaine: None,
        serie_jours: 0,
        maintenant: chrono::Utc::now(),
    };
    if let Some(defi) = moteur::defi_courant(pool.get_ref(), &regles, "jour").await? {
        courants.jour = Some(reponse_defi(pool.get_ref(), defi, utilisateur_id).await?);
    }
    if let Some(defi) = moteur::defi_courant(pool.get_ref(), &regles, "semaine").await? {
        courants.semaine = Some(reponse_defi(pool.get_ref(), defi, utilisateur_id).await?);
    }

    // Série AFFICHÉE : la rupture se constate à la lecture, sans rien écrire.
    // Elle tient tant que le dernier défi terminé date d'aujourd'hui ou d'hier.
    if let Some(utilisateur_id) = utilisateur_id {
        courants.serie_jours = sqlx::query_scalar::<_, i16>(
            "SELECT (CASE WHEN serie_dernier_jour >= (NOW() AT TIME ZONE 'UTC')::date - 1
                          THEN serie_jours ELSE 0 END)::smallint
               FROM jeu.joueur WHERE utilisateur_id = $1",
        )
        .bind(utilisateur_id)
        .fetch_optional(pool.get_ref())
        .await?
        .unwrap_or(0);
    }

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(courants), error: None }))
}

async fn charger_defi(pool: &PgPool, id: Uuid) -> Result<DefiRow, ApiErreur> {
    sqlx::query_as::<_, DefiRow>(&format!("SELECT {DEFI_COLONNES} FROM jeu.defi WHERE id = $1"))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Défi introuvable".into()))
}

/// POST /api/jeu/defis/{id}/jouer : ouvre la participation du membre.
///
/// Une seule participation par défi (index unique `uq_partie_defi`). Si elle
/// existe déjà, c'est ELLE qui est renvoyée, jamais une seconde : le client y
/// reprend la partie en cours, ou en affiche le résultat. Créer la partie
/// consomme le défi.
pub async fn jouer_defi(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = garde_joueur(pool.get_ref(), &req).await?;
    let defi = charger_defi(pool.get_ref(), path.into_inner()).await?;

    let existante = sqlx::query_as::<_, PartieRow>(&format!(
        "SELECT {PARTIE_COLONNES} FROM jeu.partie WHERE defi_id = $1 AND utilisateur_id = $2"
    ))
    .bind(defi.id)
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (partie, deja_commencee) = match existante {
        Some(partie) => (partie, true),
        None => {
            // On ne COMMENCE pas un défi dont la période est passée ; on peut en
            // revanche finir celui qu'on avait commencé à temps.
            let periode = moteur::periode_courante(pool.get_ref(), &defi.periodicite).await?;
            if defi.periode_debut != periode {
                return Err(ApiErreur::Conflit(
                    "Ce défi est terminé : il ne peut plus être joué".into(),
                ));
            }

            sqlx::query(
                "INSERT INTO jeu.joueur (utilisateur_id) VALUES ($1)
                 ON CONFLICT (utilisateur_id) DO NOTHING",
            )
            .bind(utilisateur_id)
            .execute(pool.get_ref())
            .await?;

            // Deux clics simultanés : le second ne crée rien et relit la partie
            // du premier.
            sqlx::query(
                "INSERT INTO jeu.partie (utilisateur_id, module_code, cadre, defi_id, epreuve_ids)
                 VALUES ($1, $2, 'defi', $3, $4)
                 ON CONFLICT (utilisateur_id, defi_id) WHERE defi_id IS NOT NULL DO NOTHING",
            )
            .bind(utilisateur_id)
            .bind(&defi.module_code)
            .bind(defi.id)
            .bind(&defi.epreuve_ids)
            .execute(pool.get_ref())
            .await?;

            let partie = sqlx::query_as::<_, PartieRow>(&format!(
                "SELECT {PARTIE_COLONNES} FROM jeu.partie
                  WHERE defi_id = $1 AND utilisateur_id = $2"
            ))
            .bind(defi.id)
            .bind(utilisateur_id)
            .fetch_one(pool.get_ref())
            .await?;
            (partie, false)
        }
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "partie_id": partie.id,
            "etat": partie.etat,
            "deja_commencee": deja_commencee,
        })),
        error: None,
    }))
}

/// GET /api/jeu/defis/{id}/resultats : participants, podium, rang du membre.
/// Reste consultable une fois la période passée (FR-038).
pub async fn resultats_defi(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = joueur_optionnel(&req);
    let defi = charger_defi(pool.get_ref(), path.into_inner()).await?;
    let periode = moteur::periode_courante(pool.get_ref(), &defi.periodicite).await?;
    let en_cours = defi.periode_debut == periode;

    // Le classement d'un défi ne compte que les parties TERMINÉES, des comptes
    // actifs : plus de bonnes réponses d'abord, puis le temps le plus court.
    const CLASSEMENT: &str = "
        SELECT ROW_NUMBER() OVER (ORDER BY p.bonnes DESC, p.temps_total_ms ASC, p.terminee_at ASC) AS rang,
               u.id AS utilisateur_id, u.nom, u.prenom, u.slug, u.photo_url,
               p.bonnes, p.temps_total_ms
          FROM jeu.partie p
          JOIN iam.utilisateur u ON u.id = p.utilisateur_id
         WHERE p.defi_id = $1 AND p.etat = 'terminee'
           AND u.etat = 'actif' AND u.deleted_at IS NULL";

    let participants: i64 =
        sqlx::query_scalar(&format!("SELECT COUNT(*) FROM ({CLASSEMENT}) c"))
            .bind(defi.id)
            .fetch_one(pool.get_ref())
            .await?;

    let podium = sqlx::query_as::<_, LigneResultatDefi>(&format!(
        "SELECT * FROM ({CLASSEMENT}) c ORDER BY rang LIMIT {TAILLE_PODIUM}"
    ))
    .bind(defi.id)
    .fetch_all(pool.get_ref())
    .await?;

    let moi = match utilisateur_id {
        Some(utilisateur_id) => {
            sqlx::query_as::<_, LigneResultatDefi>(&format!(
                "SELECT * FROM ({CLASSEMENT}) c WHERE utilisateur_id = $2"
            ))
            .bind(defi.id)
            .bind(utilisateur_id)
            .fetch_optional(pool.get_ref())
            .await?
        }
        None => None,
    };

    let resultats = ResultatsDefi {
        defi: reponse_defi(pool.get_ref(), defi, utilisateur_id).await?,
        en_cours,
        participants,
        podium,
        moi,
    };

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(resultats), error: None }))
}

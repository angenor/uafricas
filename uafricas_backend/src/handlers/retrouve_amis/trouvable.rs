//! Tableau de bord et profil trouvable / parcours (Retrouve Amis).

use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::retrouve_amis::*;
use crate::services::audit;
use crate::ApiResponse;

use super::commun::*;


/// GET /api/retrouve-amis/tableau-de-bord
pub async fn tableau_de_bord(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    #[derive(sqlx::FromRow)]
    struct StatsRow {
        avis_actifs: i64,
        avis_clotures: i64,
        correspondances_en_attente: i64,
        correspondances_mutuelles: i64,
        notifications_non_lues: i64,
        est_trouvable: bool,
        nb_parcours: i64,
    }

    let stats: StatsRow = sqlx::query_as(
        "SELECT
            (SELECT COUNT(*) FROM retrouve_amis.avis_recherche WHERE auteur_id = $1 AND etat = 'actif' AND deleted_at IS NULL) AS avis_actifs,
            (SELECT COUNT(*) FROM retrouve_amis.avis_recherche WHERE auteur_id = $1 AND etat = 'cloture' AND deleted_at IS NULL) AS avis_clotures,
            (SELECT COUNT(*) FROM retrouve_amis.correspondance c
             JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
             WHERE (a.auteur_id = $1 OR c.cible_utilisateur_id = $1) AND c.etat = 'en_attente') AS correspondances_en_attente,
            (SELECT COUNT(*) FROM retrouve_amis.correspondance c
             JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
             WHERE (a.auteur_id = $1 OR c.cible_utilisateur_id = $1) AND c.etat = 'mutuelle') AS correspondances_mutuelles,
            (SELECT COUNT(*) FROM retrouve_amis.notification_retrouve WHERE utilisateur_id = $1 AND lu = FALSE) AS notifications_non_lues,
            (SELECT est_trouvable FROM iam.utilisateur WHERE id = $1) AS est_trouvable,
            (SELECT COUNT(*) FROM retrouve_amis.parcours_trouvable WHERE utilisateur_id = $1) AS nb_parcours"
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(TableauDeBord {
            avis_actifs: stats.avis_actifs,
            avis_clotures: stats.avis_clotures,
            correspondances_en_attente: stats.correspondances_en_attente,
            correspondances_mutuelles: stats.correspondances_mutuelles,
            notifications_non_lues: stats.notifications_non_lues,
            est_trouvable: stats.est_trouvable,
            nb_parcours: stats.nb_parcours,
        }),
        error: None,
    }))
}



/// PATCH /api/profil/trouvable
pub async fn basculer_trouvable(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<BasculerTrouvable>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let data = body.into_inner();

    sqlx::query("UPDATE iam.utilisateur SET est_trouvable = $2 WHERE id = $1")
        .bind(utilisateur_id)
        .bind(data.est_trouvable)
        .execute(pool.get_ref())
        .await?;

    let mut nb_correspondances: i64 = 0;

    if data.est_trouvable {
        // Matching du profil contre tous les avis actifs
        // On utilise une approche inverse : pour chaque avis actif, vérifier si ce profil correspond
        let avis_actifs: Vec<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM retrouve_amis.avis_recherche WHERE etat = 'actif' AND deleted_at IS NULL AND auteur_id != $1"
        )
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?;

        for (a_id,) in &avis_actifs {
            let resultats: Vec<CorrespondanceResultat> = sqlx::query_as(
                "SELECT cible_type::text AS type_cible, cible_id, score_total::float8 AS score_total, details
                 FROM retrouve_amis.calculer_correspondances($1)
                 WHERE cible_type = 'profil' AND cible_id = $2 AND score_total >= 60"
            )
            .bind(a_id)
            .bind(utilisateur_id)
            .fetch_all(pool.get_ref())
            .await?;

            for corr in &resultats {
                let corr_id: (Uuid,) = sqlx::query_as(
                    "INSERT INTO retrouve_amis.correspondance
                     (avis_id, type_cible, cible_utilisateur_id, score, details_score, expire_at)
                     VALUES ($1, 'profil', $2, $3, $4, NOW() + INTERVAL '30 days')
                     ON CONFLICT DO NOTHING
                     RETURNING id"
                )
                .bind(a_id)
                .bind(utilisateur_id)
                .bind(corr.score_total)
                .bind(&corr.details)
                .fetch_optional(pool.get_ref())
                .await?
                .unwrap_or_default();

                if corr_id.0 != Uuid::nil() {
                    // Notifications
                    let auteur_id: (Uuid,) = sqlx::query_as(
                        "SELECT auteur_id FROM retrouve_amis.avis_recherche WHERE id = $1"
                    )
                    .bind(a_id)
                    .fetch_one(pool.get_ref())
                    .await?;

                    sqlx::query(
                        "INSERT INTO retrouve_amis.notification_retrouve
                         (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'nouvelle_correspondance')"
                    )
                    .bind(auteur_id.0)
                    .bind(corr_id.0)
                    .execute(pool.get_ref())
                    .await?;

                    sqlx::query(
                        "INSERT INTO retrouve_amis.notification_retrouve
                         (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'nouvelle_correspondance')"
                    )
                    .bind(utilisateur_id)
                    .bind(corr_id.0)
                    .execute(pool.get_ref())
                    .await?;

                    nb_correspondances += 1;
                }
            }
        }
    } else {
        // Désactivation : annuler les correspondances en_attente basées sur ce profil
        sqlx::query(
            "UPDATE retrouve_amis.correspondance SET etat = 'archivee'
             WHERE cible_utilisateur_id = $1 AND etat = 'en_attente'"
        )
        .bind(utilisateur_id)
        .execute(pool.get_ref())
        .await?;
    }

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "iam",
        "utilisateur",
        Some(utilisateur_id),
        None,
        Some(serde_json::json!({"est_trouvable": data.est_trouvable})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(BasculerTrouvableResponse {
            est_trouvable: data.est_trouvable,
            correspondances_trouvees: nb_correspondances,
        }),
        error: None,
    }))
}

/// GET /api/profil/parcours
pub async fn lister_parcours(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    #[derive(sqlx::FromRow)]
    struct ParcoursRow {
        id: Uuid,
        type_entree: String,
        nom: String,
        ville: Option<String>,
        pays_id: Option<Uuid>,
        pays_nom: Option<String>,
        periode_debut: Option<i32>,
        periode_fin: Option<i32>,
    }

    let rows: Vec<ParcoursRow> = sqlx::query_as(
        "SELECT pt.id, pt.type_entree::text AS type_entree, pt.nom, pt.ville,
                pt.pays_id, p.nom AS pays_nom,
                pt.periode_debut, pt.periode_fin
         FROM retrouve_amis.parcours_trouvable pt
         LEFT JOIN shared.pays p ON p.id = pt.pays_id
         WHERE pt.utilisateur_id = $1
         ORDER BY pt.periode_debut DESC NULLS LAST, pt.created_at DESC"
    )
    .bind(utilisateur_id)
    .fetch_all(pool.get_ref())
    .await?;

    let parcours: Vec<ParcoursTrouvableResponse> = rows
        .into_iter()
        .map(|r| ParcoursTrouvableResponse {
            id: r.id,
            type_entree: r.type_entree,
            nom: r.nom,
            ville: r.ville,
            pays: r.pays_id.map(|id| PaysInfo {
                id,
                nom: r.pays_nom.unwrap_or_default(),
            }),
            periode_debut: r.periode_debut,
            periode_fin: r.periode_fin,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(parcours),
        error: None,
    }))
}

/// POST /api/profil/parcours
pub async fn ajouter_parcours(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerParcours>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let data = body.into_inner();

    // Valider type_entree
    if !["ecole", "ville_residence"].contains(&data.type_entree.as_str()) {
        return Err(ApiErreur::Validation("Type d'entrée invalide (ecole ou ville_residence)".into()));
    }

    if data.nom.trim().is_empty() {
        return Err(ApiErreur::Validation("Le nom est obligatoire".into()));
    }

    if let (Some(debut), Some(fin)) = (data.periode_debut, data.periode_fin) {
        if debut > fin {
            return Err(ApiErreur::Validation("La période de début doit être antérieure à la fin".into()));
        }
    }

    let id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.parcours_trouvable
         (utilisateur_id, type_entree, nom, ville, pays_id, periode_debut, periode_fin)
         VALUES ($1, $2::retrouve_amis.type_parcours, $3, $4, $5, $6, $7)
         RETURNING id"
    )
    .bind(utilisateur_id)
    .bind(&data.type_entree)
    .bind(&data.nom)
    .bind(&data.ville)
    .bind(data.pays_id)
    .bind(data.periode_debut)
    .bind(data.periode_fin)
    .fetch_one(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "parcours_trouvable",
        Some(id.0),
        None,
        Some(serde_json::json!({"type_entree": &data.type_entree, "nom": &data.nom})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": id.0})),
        error: None,
    }))
}

/// PUT /api/profil/parcours/{id}
pub async fn modifier_parcours(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierParcours>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let parcours_id = path.into_inner();
    let data = body.into_inner();

    let result = sqlx::query(
        "UPDATE retrouve_amis.parcours_trouvable SET
            type_entree = $3::retrouve_amis.type_parcours, nom = $4, ville = $5,
            pays_id = $6, periode_debut = $7, periode_fin = $8
         WHERE id = $1 AND utilisateur_id = $2"
    )
    .bind(parcours_id)
    .bind(utilisateur_id)
    .bind(&data.type_entree)
    .bind(&data.nom)
    .bind(&data.ville)
    .bind(data.pays_id)
    .bind(data.periode_debut)
    .bind(data.periode_fin)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Entrée de parcours introuvable".into()));
    }

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "parcours_trouvable",
        Some(parcours_id),
        None,
        None,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// DELETE /api/profil/parcours/{id}
pub async fn supprimer_parcours(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let parcours_id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM retrouve_amis.parcours_trouvable WHERE id = $1 AND utilisateur_id = $2"
    )
    .bind(parcours_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Entrée de parcours introuvable".into()));
    }

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "DELETE",
        "retrouve_amis",
        "parcours_trouvable",
        Some(parcours_id),
        None,
        None,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}



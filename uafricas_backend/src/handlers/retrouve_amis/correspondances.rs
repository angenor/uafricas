//! Correspondances et signalement d'avis (Retrouve Amis).

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use futures_util::StreamExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::retrouve_amis::*;
use crate::services::audit;
use crate::ApiResponse;

use super::commun::*;


/// GET /api/retrouve-amis/correspondances
/// Lister les correspondances de l'utilisateur
pub async fn lister_correspondances(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    let etat = query.get("etat").cloned();
    let avis_id = query.get("avis_id").and_then(|v| v.parse::<Uuid>().ok());
    let page: i64 = query.get("page").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let par_page: i64 = query.get("par_page").and_then(|v| v.parse().ok()).unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * par_page;

    // Lazy archival : archiver les correspondances expirées
    sqlx::query(
        "UPDATE retrouve_amis.correspondance
         SET etat = 'archivee'
         WHERE etat IN ('en_attente', 'acceptee_a', 'acceptee_b')
           AND created_at < NOW() - INTERVAL '30 days'"
    )
    .execute(pool.get_ref())
    .await?;

    #[derive(sqlx::FromRow)]
    struct CorrListeRow {
        id: Uuid,
        avis_id: Uuid,
        score: f64,
        etat: String,
        type_cible: String,
        cible_avis_id: Option<Uuid>,
        cible_utilisateur_id: Option<Uuid>,
        details_score: serde_json::Value,
        expire_at: Option<chrono::DateTime<Utc>>,
        created_at: chrono::DateTime<Utc>,
        // Pour résumé anonymisé
        cible_nom: Option<String>,
        cible_prenom: Option<String>,
        cible_ville: Option<String>,
        cible_periode_debut: Option<i32>,
        cible_periode_fin: Option<i32>,
        // Pour mon_role
        avis_auteur_id: Uuid,
    }

    let base_where = format!(
        "(c.avis_id IN (SELECT id FROM retrouve_amis.avis_recherche WHERE auteur_id = $1 AND deleted_at IS NULL)
          OR c.cible_utilisateur_id = $1
          OR c.cible_avis_id IN (SELECT id FROM retrouve_amis.avis_recherche WHERE auteur_id = $1 AND deleted_at IS NULL))
        {}
        {}",
        if etat.is_some() { " AND c.etat::text = $2" } else { "" },
        if avis_id.is_some() {
            if etat.is_some() { " AND c.avis_id = $3" } else { " AND c.avis_id = $2" }
        } else { "" }
    );

    let count_sql = format!(
        "SELECT COUNT(*) FROM retrouve_amis.correspondance c WHERE {}",
        base_where
    );

    let list_sql = format!(
        "SELECT c.id, c.avis_id, c.score::float8 AS score, c.etat::text AS etat,
                c.type_cible::text AS type_cible, c.cible_avis_id, c.cible_utilisateur_id,
                c.details_score, c.expire_at, c.created_at,
                a.auteur_id AS avis_auteur_id,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.nom_recherche
                    WHEN c.type_cible = 'profil' THEN u.nom
                END AS cible_nom,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.prenom_recherche
                    WHEN c.type_cible = 'profil' THEN u.prenom
                END AS cible_prenom,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.ville
                    WHEN c.type_cible = 'profil' THEN u.ville
                END AS cible_ville,
                CASE WHEN c.type_cible = 'avis' THEN a2.periode_debut END AS cible_periode_debut,
                CASE WHEN c.type_cible = 'avis' THEN a2.periode_fin END AS cible_periode_fin
         FROM retrouve_amis.correspondance c
         JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
         LEFT JOIN retrouve_amis.avis_recherche a2 ON c.cible_avis_id = a2.id
         LEFT JOIN iam.utilisateur u ON c.cible_utilisateur_id = u.id
         WHERE {}
         ORDER BY c.score DESC, c.created_at DESC
         LIMIT {} OFFSET {}",
        base_where, par_page, offset
    );

    // Bind dynamique
    let total: (i64,);
    let rows: Vec<CorrListeRow>;

    match (etat.as_ref(), avis_id) {
        (Some(e), Some(aid)) => {
            total = sqlx::query_as(&count_sql).bind(utilisateur_id).bind(e).bind(aid).fetch_one(pool.get_ref()).await?;
            rows = sqlx::query_as(&list_sql).bind(utilisateur_id).bind(e).bind(aid).fetch_all(pool.get_ref()).await?;
        }
        (Some(e), None) => {
            total = sqlx::query_as(&count_sql).bind(utilisateur_id).bind(e).fetch_one(pool.get_ref()).await?;
            rows = sqlx::query_as(&list_sql).bind(utilisateur_id).bind(e).fetch_all(pool.get_ref()).await?;
        }
        (None, Some(aid)) => {
            total = sqlx::query_as(&count_sql).bind(utilisateur_id).bind(aid).fetch_one(pool.get_ref()).await?;
            rows = sqlx::query_as(&list_sql).bind(utilisateur_id).bind(aid).fetch_all(pool.get_ref()).await?;
        }
        (None, None) => {
            total = sqlx::query_as(&count_sql).bind(utilisateur_id).fetch_one(pool.get_ref()).await?;
            rows = sqlx::query_as(&list_sql).bind(utilisateur_id).fetch_all(pool.get_ref()).await?;
        }
    }

    let correspondances: Vec<CorrespondanceResponse> = rows
        .into_iter()
        .map(|c| {
            let mon_role = if c.avis_auteur_id == utilisateur_id { "auteur" } else { "cible" };
            let initiales = construire_initiales(c.cible_nom.as_deref(), c.cible_prenom.as_deref());
            let periode = construire_periode(c.cible_periode_debut, c.cible_periode_fin);
            let criteres = construire_criteres_communs(&c.details_score);

            CorrespondanceResponse {
                id: c.id,
                avis_id: c.avis_id,
                score: c.score,
                etat: c.etat,
                type_cible: c.type_cible,
                resume_anonymise: ResumeAnonyme {
                    initiales,
                    ville: c.cible_ville,
                    periode,
                    criteres_communs: criteres,
                },
                mon_role: mon_role.to_string(),
                created_at: c.created_at,
                expire_at: c.expire_at,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(CorrespondanceListeResponse {
            correspondances,
            total: total.0,
            page,
            par_page,
        }),
        error: None,
    }))
}

/// GET /api/retrouve-amis/correspondances/{id}
/// Détail d'une correspondance
pub async fn detail_correspondance(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let corr_id = path.into_inner();

    #[derive(sqlx::FromRow)]
    struct CorrDetailRow {
        id: Uuid,
        avis_id: Uuid,
        score: f64,
        etat: String,
        type_cible: String,
        details_score: serde_json::Value,
        coordonnees_a: Option<serde_json::Value>,
        coordonnees_b: Option<serde_json::Value>,
        expire_at: Option<chrono::DateTime<Utc>>,
        created_at: chrono::DateTime<Utc>,
        avis_auteur_id: Uuid,
        cible_utilisateur_id: Option<Uuid>,
        cible_nom: Option<String>,
        cible_prenom: Option<String>,
        cible_ville: Option<String>,
        cible_periode_debut: Option<i32>,
        cible_periode_fin: Option<i32>,
        message_reponse: Option<String>,
        type_reponse_publique: Option<String>,
    }

    let corr: CorrDetailRow = sqlx::query_as(
        "SELECT c.id, c.avis_id, c.score::float8 AS score, c.etat::text AS etat,
                c.type_cible::text AS type_cible, c.details_score,
                c.coordonnees_a, c.coordonnees_b,
                c.expire_at, c.created_at,
                a.auteur_id AS avis_auteur_id, c.cible_utilisateur_id,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.nom_recherche
                    WHEN c.type_cible = 'profil' THEN u.nom
                END AS cible_nom,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.prenom_recherche
                    WHEN c.type_cible = 'profil' THEN u.prenom
                END AS cible_prenom,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.ville
                    WHEN c.type_cible = 'profil' THEN u.ville
                END AS cible_ville,
                CASE WHEN c.type_cible = 'avis' THEN a2.periode_debut END AS cible_periode_debut,
                CASE WHEN c.type_cible = 'avis' THEN a2.periode_fin END AS cible_periode_fin,
                rp.message AS message_reponse,
                rp.type_reponse::text AS type_reponse_publique
         FROM retrouve_amis.correspondance c
         JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
         LEFT JOIN retrouve_amis.avis_recherche a2 ON c.cible_avis_id = a2.id
         LEFT JOIN iam.utilisateur u ON c.cible_utilisateur_id = u.id
         LEFT JOIN retrouve_amis.reponse_publique rp ON rp.correspondance_id = c.id
         WHERE c.id = $1"
    )
    .bind(corr_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Correspondance introuvable".into()))?;

    // Vérifier que l'utilisateur est participant
    let est_auteur = corr.avis_auteur_id == utilisateur_id;
    let est_cible = corr.cible_utilisateur_id == Some(utilisateur_id);
    // Vérifier aussi si la cible est un avis de l'utilisateur
    let est_cible_avis = if !est_auteur && !est_cible {
        // TODO: check if cible_avis_id belongs to user
        false
    } else {
        false
    };

    if !est_auteur && !est_cible && !est_cible_avis {
        return Err(ApiErreur::AccesInterdit("Vous n'êtes pas participant à cette correspondance".into()));
    }

    let mon_role = if est_auteur { "auteur" } else { "cible" };
    let initiales = construire_initiales(corr.cible_nom.as_deref(), corr.cible_prenom.as_deref());
    let periode = construire_periode(corr.cible_periode_debut, corr.cible_periode_fin);
    let criteres = construire_criteres_communs(&corr.details_score);

    // Coordonnées partagées (seulement si état mutuelle).
    // Les colonnes coordonnees_a / coordonnees_b stockent le CHOIX (booléens) ;
    // on les résout ici en coordonnées réelles de l'autre participant.
    let coordonnees_partagees = if corr.etat == "mutuelle" {
        let choix = if est_auteur { corr.coordonnees_b.as_ref() } else { corr.coordonnees_a.as_ref() };
        let autre_id = if est_auteur { corr.cible_utilisateur_id } else { Some(corr.avis_auteur_id) };

        match (choix, autre_id) {
            (Some(choix), Some(autre_id)) => {
                let contact: Option<(String, Option<String>)> = sqlx::query_as(
                    "SELECT email::text, telephone FROM iam.utilisateur WHERE id = $1"
                )
                .bind(autre_id)
                .fetch_optional(pool.get_ref())
                .await?;

                let mut resolu = serde_json::Map::new();
                let choisi = |cle: &str| choix.get(cle) == Some(&serde_json::Value::Bool(true));

                if let Some((email, telephone)) = contact {
                    if choisi("email") {
                        resolu.insert("email".into(), serde_json::json!(email));
                    }
                    if choisi("telephone") {
                        if let Some(tel) = telephone.filter(|t| !t.trim().is_empty()) {
                            resolu.insert("telephone".into(), serde_json::json!(tel));
                        }
                    }
                }
                if choisi("messagerie") {
                    resolu.insert("messagerie".into(), serde_json::json!(true));
                    resolu.insert("messagerie_utilisateur_id".into(), serde_json::json!(autre_id));
                }

                if resolu.is_empty() { None } else { Some(serde_json::Value::Object(resolu)) }
            }
            _ => None,
        }
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(CorrespondanceDetailResponse {
            id: corr.id,
            avis_id: corr.avis_id,
            score: corr.score,
            details_score: corr.details_score,
            etat: corr.etat,
            type_cible: corr.type_cible,
            mon_role: mon_role.to_string(),
            resume_anonymise: ResumeAnonyme {
                initiales,
                ville: corr.cible_ville,
                periode,
                criteres_communs: criteres,
            },
            coordonnees_partagees,
            message_reponse: corr.message_reponse,
            type_reponse_publique: corr.type_reponse_publique,
            created_at: corr.created_at,
            expire_at: corr.expire_at,
        }),
        error: None,
    }))
}

/// POST /api/retrouve-amis/correspondances/{id}/accepter
/// Accepter le contact pour une correspondance
pub async fn accepter_correspondance(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AccepterCorrespondance>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let corr_id = path.into_inner();
    let data = body.into_inner();

    #[derive(sqlx::FromRow)]
    struct CorrInfo {
        id: Uuid,
        avis_id: Uuid,
        etat: String,
        cible_utilisateur_id: Option<Uuid>,
        avis_auteur_id: Uuid,
    }

    let corr: CorrInfo = sqlx::query_as(
        "SELECT c.id, c.avis_id, c.etat::text AS etat, c.cible_utilisateur_id, a.auteur_id AS avis_auteur_id
         FROM retrouve_amis.correspondance c
         JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
         WHERE c.id = $1"
    )
    .bind(corr_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Correspondance introuvable".into()))?;

    let est_a = corr.avis_auteur_id == utilisateur_id;
    let est_b = corr.cible_utilisateur_id == Some(utilisateur_id);

    if !est_a && !est_b {
        return Err(ApiErreur::AccesInterdit("Vous n'êtes pas participant".into()));
    }

    // Déterminer la transition d'état
    let coordonnees_json = serde_json::to_value(&data.coordonnees)
        .map_err(|e| ApiErreur::Validation(e.to_string()))?;

    let (nouvel_etat, consentement_mutuel) = if est_a {
        match corr.etat.as_str() {
            "en_attente" => {
                sqlx::query(
                    "UPDATE retrouve_amis.correspondance
                     SET etat = 'acceptee_a', accepte_par_a_at = NOW(), coordonnees_a = $2
                     WHERE id = $1"
                )
                .bind(corr_id)
                .bind(&coordonnees_json)
                .execute(pool.get_ref())
                .await?;
                ("acceptee_a", false)
            }
            "acceptee_b" => {
                sqlx::query(
                    "UPDATE retrouve_amis.correspondance
                     SET etat = 'mutuelle', accepte_par_a_at = NOW(), coordonnees_a = $2
                     WHERE id = $1"
                )
                .bind(corr_id)
                .bind(&coordonnees_json)
                .execute(pool.get_ref())
                .await?;
                ("mutuelle", true)
            }
            _ => return Err(ApiErreur::Validation("Cette correspondance ne peut pas être acceptée dans son état actuel".into())),
        }
    } else {
        match corr.etat.as_str() {
            "en_attente" => {
                sqlx::query(
                    "UPDATE retrouve_amis.correspondance
                     SET etat = 'acceptee_b', accepte_par_b_at = NOW(), coordonnees_b = $2
                     WHERE id = $1"
                )
                .bind(corr_id)
                .bind(&coordonnees_json)
                .execute(pool.get_ref())
                .await?;
                ("acceptee_b", false)
            }
            "acceptee_a" => {
                sqlx::query(
                    "UPDATE retrouve_amis.correspondance
                     SET etat = 'mutuelle', accepte_par_b_at = NOW(), coordonnees_b = $2
                     WHERE id = $1"
                )
                .bind(corr_id)
                .bind(&coordonnees_json)
                .execute(pool.get_ref())
                .await?;
                ("mutuelle", true)
            }
            _ => return Err(ApiErreur::Validation("Cette correspondance ne peut pas être acceptée dans son état actuel".into())),
        }
    };

    // Créer notification
    let notif_type = if consentement_mutuel { "coordonnees_partagees" } else { "acceptation_contact" };
    let autre_utilisateur = if est_a {
        corr.cible_utilisateur_id.unwrap_or(corr.avis_auteur_id)
    } else {
        corr.avis_auteur_id
    };

    sqlx::query(
        "INSERT INTO retrouve_amis.notification_retrouve
         (utilisateur_id, correspondance_id, type) VALUES ($1, $2, $3::retrouve_amis.type_notification)"
    )
    .bind(autre_utilisateur)
    .bind(corr_id)
    .bind(notif_type)
    .execute(pool.get_ref())
    .await?;

    // Si mutuelle, notifier aussi l'accepteur
    if consentement_mutuel {
        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'coordonnees_partagees')"
        )
        .bind(utilisateur_id)
        .bind(corr_id)
        .execute(pool.get_ref())
        .await?;
    }

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "correspondance",
        Some(corr_id),
        None,
        Some(serde_json::json!({"action": "accepter", "nouvel_etat": nouvel_etat})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": corr_id,
            "etat": nouvel_etat,
            "consentement_mutuel": consentement_mutuel
        })),
        error: None,
    }))
}

/// POST /api/retrouve-amis/correspondances/{id}/refuser
/// Refuser le contact. Crée une blacklist automatique.
pub async fn refuser_correspondance(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let corr_id = path.into_inner();

    #[derive(sqlx::FromRow)]
    struct CorrInfo {
        id: Uuid,
        etat: String,
        cible_utilisateur_id: Option<Uuid>,
        avis_auteur_id: Uuid,
    }

    let corr: CorrInfo = sqlx::query_as(
        "SELECT c.id, c.etat::text AS etat, c.cible_utilisateur_id, a.auteur_id AS avis_auteur_id
         FROM retrouve_amis.correspondance c
         JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
         WHERE c.id = $1"
    )
    .bind(corr_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Correspondance introuvable".into()))?;

    let est_a = corr.avis_auteur_id == utilisateur_id;
    let est_b = corr.cible_utilisateur_id == Some(utilisateur_id);

    if !est_a && !est_b {
        return Err(ApiErreur::AccesInterdit("Vous n'êtes pas participant".into()));
    }

    if !["en_attente", "acceptee_a", "acceptee_b"].contains(&corr.etat.as_str()) {
        return Err(ApiErreur::Validation("Cette correspondance ne peut pas être refusée".into()));
    }

    // Mettre à jour l'état
    sqlx::query("UPDATE retrouve_amis.correspondance SET etat = 'declinee' WHERE id = $1")
        .bind(corr_id)
        .execute(pool.get_ref())
        .await?;

    // Insérer dans la blacklist
    let autre_uid = if est_a {
        corr.cible_utilisateur_id.unwrap_or(corr.avis_auteur_id)
    } else {
        corr.avis_auteur_id
    };

    let (uid_a, uid_b) = if utilisateur_id < autre_uid {
        (utilisateur_id, autre_uid)
    } else {
        (autre_uid, utilisateur_id)
    };

    sqlx::query(
        "INSERT INTO retrouve_amis.blacklist (utilisateur_a_id, utilisateur_b_id, correspondance_id)
         VALUES ($1, $2, $3)
         ON CONFLICT DO NOTHING"
    )
    .bind(uid_a)
    .bind(uid_b)
    .bind(corr_id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "correspondance",
        Some(corr_id),
        None,
        Some(serde_json::json!({"action": "refuser", "blacklist": true})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": corr_id, "etat": "declinee"})),
        error: None,
    }))
}

/// POST /api/retrouve-amis/avis/{id}/signaler
/// Signaler un avis de recherche
pub async fn signaler_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<SignalerAvis>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let avis_id = path.into_inner();
    let data = body.into_inner();

    // Vérifier que l'utilisateur a une correspondance avec cet avis
    let a_correspondance: (bool,) = sqlx::query_as(
        "SELECT EXISTS(
            SELECT 1 FROM retrouve_amis.correspondance c
            JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
            WHERE c.avis_id = $1 AND (c.cible_utilisateur_id = $2
                OR c.cible_avis_id IN (SELECT id FROM retrouve_amis.avis_recherche WHERE auteur_id = $2))
        )"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !a_correspondance.0 {
        return Err(ApiErreur::AccesInterdit(
            "Vous devez avoir une correspondance avec cet avis pour le signaler".into(),
        ));
    }

    // Valider le motif
    let motifs_valides = ["contenu_abusif", "usurpation_identite", "harcelement", "autre"];
    if !motifs_valides.contains(&data.motif.as_str()) {
        return Err(ApiErreur::Validation("Motif de signalement invalide".into()));
    }

    let signalement_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.signalement (avis_id, signale_par, motif, description)
         VALUES ($1, $2, $3::retrouve_amis.motif_signalement, $4)
         RETURNING id"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .bind(&data.motif)
    .bind(&data.description)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("idx_signalement_unique") {
            ApiErreur::Conflit("Vous avez déjà signalé cet avis".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "signalement",
        Some(signalement_id.0),
        None,
        Some(serde_json::json!({"avis_id": avis_id, "motif": &data.motif})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": signalement_id.0})),
        error: None,
    }))
}



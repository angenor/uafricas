//! Publication publique : partage, signalement, retrait, réponse (Retrouve Amis).

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::retrouve_amis::*;
use crate::services::audit;
use crate::ApiResponse;

use super::commun::*;


/// PATCH /api/retrouve-amis/avis/{id}/publier
/// Activer ou desactiver la visibilite publique d'un avis
pub async fn publier_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<PublierAvisRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let avis_id = path.into_inner();
    let est_public = body.est_public;

    // Verifier que l'avis existe, appartient a l'auteur et est actif
    let avis: Option<(Uuid, String, String, Option<String>, Option<String>, bool, Option<String>, Option<chrono::DateTime<Utc>>)> = sqlx::query_as(
        "SELECT id, etat::text, nom_recherche, prenom_recherche, slug, est_public, slug, date_publication_publique
         FROM retrouve_amis.avis_recherche
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(avis_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let avis = avis.ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve".into()))?;

    // Verifier l'auteur
    let auteur_id: Option<(Uuid,)> = sqlx::query_as(
        "SELECT auteur_id FROM retrouve_amis.avis_recherche WHERE id = $1"
    )
    .bind(avis_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let auteur_id = auteur_id.ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve".into()))?;
    if auteur_id.0 != utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Vous n'etes pas l'auteur de cet avis".into()));
    }

    // Verifier que l'etat est actif
    if avis.1 != "actif" {
        return Err(ApiErreur::Validation(
            "Seuls les avis actifs peuvent etre rendus publics".into(),
        ));
    }

    // Generer le slug si premiere publication
    let slug_actuel = avis.4.clone();
    let slug = if slug_actuel.is_some() {
        slug_actuel.unwrap()
    } else {
        generer_slug_avis(&avis.2, avis.3.as_deref())
    };

    // Mettre a jour l'avis
    let now = Utc::now();
    if avis.7.is_some() {
        // Slug et date deja definis, juste toggle est_public
        sqlx::query(
            "UPDATE retrouve_amis.avis_recherche
             SET est_public = $1, slug = $2, updated_at = NOW()
             WHERE id = $3"
        )
        .bind(est_public)
        .bind(&slug)
        .bind(avis_id)
        .execute(pool.get_ref())
        .await?;
    } else {
        // Premiere publication : set slug + date_publication_publique
        sqlx::query(
            "UPDATE retrouve_amis.avis_recherche
             SET est_public = $1, slug = $2, date_publication_publique = $3, updated_at = NOW()
             WHERE id = $4"
        )
        .bind(est_public)
        .bind(&slug)
        .bind(now)
        .bind(avis_id)
        .execute(pool.get_ref())
        .await?;
    }

    // Recuperer la date de publication (existante ou nouvelle)
    let date_pub = if avis.7.is_some() { avis.7 } else if est_public { Some(now) } else { None };

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "avis_recherche",
        Some(avis_id),
        Some(serde_json::json!({ "est_public": avis.5 })),
        Some(serde_json::json!({ "est_public": est_public, "slug": &slug })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PublierAvisResponse {
            id: avis_id,
            est_public,
            slug,
            date_publication_publique: date_pub,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// SIGNALEMENT DEPUIS LA PAGE PUBLIQUE
// ════════════════════════════════════════════════════════════════════════════

/// POST /api/retrouve-amis/public/{slug}/signaler
/// Signaler un avis depuis la page publique (connexion requise)
/// Auto-suspension si >= 3 signalements distincts
pub async fn signaler_avis_public(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    body: web::Json<SignalerPublicRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let slug = path.into_inner();
    let data = body.into_inner();

    // Verifier que l'avis existe, est public et actif
    let avis: Option<(Uuid, Uuid, String, bool)> = sqlx::query_as(
        "SELECT id, auteur_id, etat::text, est_public
         FROM retrouve_amis.avis_recherche
         WHERE slug = $1 AND deleted_at IS NULL"
    )
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?;

    let (avis_id, auteur_id, etat, est_public) = avis
        .ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve ou non disponible".into()))?;

    if !est_public || etat != "actif" {
        return Err(ApiErreur::NonTrouve("Avis non disponible".into()));
    }

    // Le signaleur ne peut pas etre l'auteur
    if auteur_id == utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Vous ne pouvez pas signaler votre propre avis".into()));
    }

    // Valider le motif
    let motifs_valides = ["contenu_abusif", "usurpation_identite", "harcelement", "autre"];
    if !motifs_valides.contains(&data.motif.as_str()) {
        return Err(ApiErreur::Validation("Motif de signalement invalide".into()));
    }

    // Inserer le signalement avec source = 'page_publique'
    let signalement_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.signalement (avis_id, signale_par, motif, description, source)
         VALUES ($1, $2, $3::retrouve_amis.motif_signalement, $4, 'page_publique'::retrouve_amis.source_signalement)
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
            ApiErreur::Conflit("Vous avez deja signale cet avis".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    // Compter les signalements distincts et auto-suspendre si >= 3
    let nb_signalements: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT signale_par) FROM retrouve_amis.signalement WHERE avis_id = $1"
    )
    .bind(avis_id)
    .fetch_one(pool.get_ref())
    .await?;

    if nb_signalements.0 >= 3 {
        sqlx::query(
            "UPDATE retrouve_amis.avis_recherche SET etat = 'suspendu'::retrouve_amis.etat_avis, updated_at = NOW()
             WHERE id = $1 AND etat = 'actif'"
        )
        .bind(avis_id)
        .execute(pool.get_ref())
        .await?;

        // Notifier l'auteur de la suspension
        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, type) VALUES ($1, 'avis_suspendu'::retrouve_amis.type_notification)"
        )
        .bind(auteur_id)
        .execute(pool.get_ref())
        .await?;
    }

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "signalement",
        Some(signalement_id.0),
        None,
        Some(serde_json::json!({
            "avis_id": avis_id,
            "motif": &data.motif,
            "source": "page_publique",
            "auto_suspension": nb_signalements.0 >= 3
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": signalement_id.0})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// DEMANDE DE RETRAIT
// ════════════════════════════════════════════════════════════════════════════

/// POST /api/retrouve-amis/public/{slug}/demande-retrait
/// Demander le retrait d'un avis (suspension immediate)
pub async fn demander_retrait(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    body: web::Json<DemandeRetraitRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let slug = path.into_inner();
    let data = body.into_inner();

    // Verifier que l'avis existe et est public
    let avis: Option<(Uuid, Uuid, String, bool)> = sqlx::query_as(
        "SELECT id, auteur_id, etat::text, est_public
         FROM retrouve_amis.avis_recherche
         WHERE slug = $1 AND deleted_at IS NULL"
    )
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?;

    let (avis_id, auteur_id, _etat, est_public) = avis
        .ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve ou non disponible".into()))?;

    if !est_public {
        return Err(ApiErreur::NonTrouve("Avis non disponible".into()));
    }

    // Le demandeur ne peut pas etre l'auteur
    if auteur_id == utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Vous ne pouvez pas demander le retrait de votre propre avis. Vous pouvez le depublier directement.".into(),
        ));
    }

    // Valider le motif
    if data.motif.trim().is_empty() {
        return Err(ApiErreur::Validation("Le motif est obligatoire".into()));
    }

    // Inserer la demande de retrait
    let demande_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.demande_retrait (avis_id, demandeur_id, motif)
         VALUES ($1, $2, $3)
         RETURNING id"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .bind(&data.motif)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("idx_demande_retrait_unique") || e.to_string().contains("demande_retrait_avis_id_demandeur_id_key") {
            ApiErreur::Conflit("Vous avez deja soumis une demande de retrait pour cet avis".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    // Suspension immediate de l'avis
    sqlx::query(
        "UPDATE retrouve_amis.avis_recherche SET etat = 'suspendu'::retrouve_amis.etat_avis, updated_at = NOW()
         WHERE id = $1"
    )
    .bind(avis_id)
    .execute(pool.get_ref())
    .await?;

    // Notifier l'auteur
    sqlx::query(
        "INSERT INTO retrouve_amis.notification_retrouve
         (utilisateur_id, type) VALUES ($1, 'demande_retrait'::retrouve_amis.type_notification)"
    )
    .bind(auteur_id)
    .execute(pool.get_ref())
    .await?;

    // Notifier les administrateurs
    let admins: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT DISTINCT u.id
         FROM iam.utilisateur u
         JOIN iam.utilisateur_role ur ON ur.utilisateur_id = u.id
         JOIN iam.role r ON r.id = ur.role_id
         WHERE r.nom = 'admin' AND u.deleted_at IS NULL"
    )
    .fetch_all(pool.get_ref())
    .await?;

    for (admin_id,) in &admins {
        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, type) VALUES ($1, 'demande_retrait'::retrouve_amis.type_notification)"
        )
        .bind(admin_id)
        .execute(pool.get_ref())
        .await?;
    }

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "demande_retrait",
        Some(demande_id.0),
        None,
        Some(serde_json::json!({
            "avis_id": avis_id,
            "suspension_immediate": true
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(DemandeRetraitResponse {
            id: demande_id.0,
            message: "L'avis a ete immediatement suspendu. Un administrateur examinera votre demande sous 72h.".to_string(),
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// REPONSE PUBLIQUE
// ════════════════════════════════════════════════════════════════════════════

/// POST /api/retrouve-amis/public/{slug}/repondre
/// Repondre a un avis public (cree une correspondance automatiquement)
pub async fn repondre_avis_public(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    body: web::Json<ReponsePubliqueRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let slug = path.into_inner();
    let data = body.into_inner();

    // Valider le type de reponse
    let types_valides = ["je_suis_cette_personne", "je_la_connais", "jai_des_informations"];
    if !types_valides.contains(&data.type_reponse.as_str()) {
        return Err(ApiErreur::Validation("Type de reponse invalide".into()));
    }

    // Valider le message
    if data.message.trim().is_empty() {
        return Err(ApiErreur::Validation("Le message est obligatoire".into()));
    }

    // Verifier que l'avis existe, est public et actif
    let avis: Option<(Uuid, Uuid, String, bool)> = sqlx::query_as(
        "SELECT id, auteur_id, etat::text, est_public
         FROM retrouve_amis.avis_recherche
         WHERE slug = $1 AND deleted_at IS NULL"
    )
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?;

    let (avis_id, auteur_id, etat, est_public) = avis
        .ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve ou non disponible".into()))?;

    if !est_public || etat != "actif" {
        return Err(ApiErreur::NonTrouve("Avis non disponible".into()));
    }

    // Le repondeur ne peut pas etre l'auteur
    if auteur_id == utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Vous ne pouvez pas repondre a votre propre avis".into()));
    }

    // Verifier la blacklist (ordre canonique)
    let dans_blacklist: Option<(bool,)> = sqlx::query_as(
        "SELECT TRUE FROM retrouve_amis.blacklist
         WHERE utilisateur_a_id = $1 AND utilisateur_b_id = $2"
    )
    .bind(std::cmp::min(auteur_id, utilisateur_id))
    .bind(std::cmp::max(auteur_id, utilisateur_id))
    .fetch_optional(pool.get_ref())
    .await?;

    if dans_blacklist.is_some() {
        return Err(ApiErreur::AccesInterdit("Vous ne pouvez pas repondre a cet avis".into()));
    }

    // Rate limit : max 10 reponses par jour
    let nb_reponses_jour: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM retrouve_amis.reponse_publique
         WHERE repondeur_id = $1 AND created_at > NOW() - INTERVAL '1 day'"
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if nb_reponses_jour.0 >= 10 {
        return Err(ApiErreur::LimiteAtteinte("Limite de 10 reponses par jour atteinte. Reessayez demain.".into()));
    }

    // Creer la correspondance automatiquement (type_cible = 'profil', score = 70)
    let details_score = serde_json::json!({
        "source": "reponse_publique",
        "type_reponse": &data.type_reponse
    });

    let correspondance_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.correspondance
         (avis_id, type_cible, cible_utilisateur_id, score, details_score, expire_at)
         VALUES ($1, 'profil', $2, 70, $3, NOW() + INTERVAL '30 days')
         RETURNING id"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .bind(&details_score)
    .fetch_one(pool.get_ref())
    .await?;

    // Inserer la reponse publique
    let reponse_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.reponse_publique
         (avis_id, repondeur_id, type_reponse, message, correspondance_id)
         VALUES ($1, $2, $3::retrouve_amis.type_reponse_publique, $4, $5)
         RETURNING id"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .bind(&data.type_reponse)
    .bind(&data.message)
    .bind(correspondance_id.0)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("uq_reponse_publique_avis_repondeur") {
            ApiErreur::Conflit("Vous avez deja repondu a cet avis".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    // Notifier l'auteur de l'avis
    sqlx::query(
        "INSERT INTO retrouve_amis.notification_retrouve
         (utilisateur_id, correspondance_id, type)
         VALUES ($1, $2, 'reponse_publique'::retrouve_amis.type_notification)"
    )
    .bind(auteur_id)
    .bind(correspondance_id.0)
    .execute(pool.get_ref())
    .await?;

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "reponse_publique",
        Some(reponse_id.0),
        None,
        Some(serde_json::json!({
            "avis_id": avis_id,
            "type_reponse": &data.type_reponse,
            "correspondance_id": correspondance_id.0
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(ReponsePubliqueResponse {
            id: reponse_id.0,
            correspondance_id: correspondance_id.0,
            message: "Votre reponse a ete envoyee. L'auteur de l'avis sera notifie.".to_string(),
        }),
        error: None,
    }))
}


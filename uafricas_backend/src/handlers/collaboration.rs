// ════════════════════════════════════════════════════════════════════════════
// Handlers : Collaboration et partage d'arbres
// ════════════════════════════════════════════════════════════════════════════

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::collaboration::*;
use crate::services::audit;
use crate::ApiResponse;

// ─── Helper JWT ─────────────────────────────────────────────────────────

fn extraire_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    let header = req.headers().get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiErreur::NonAutorise("Token manquant".into()))?;
    let token = crate::jwt::extraire_token_du_header(header)?;
    let jwt_config = req.app_data::<web::Data<crate::config::JwtConfig>>()
        .ok_or_else(|| ApiErreur::BaseDeDonnees("Configuration JWT manquante".into()))?;
    let claims = crate::jwt::valider_token(token, &jwt_config.secret)?;
    claims.sub.parse::<Uuid>().map_err(|_| ApiErreur::NonAutorise("ID invalide".into()))
}

async fn obtenir_arbre_proprietaire(pool: &PgPool, utilisateur_id: Uuid) -> Result<Uuid, ApiErreur> {
    sqlx::query_scalar("SELECT id FROM arbre_genealogique.arbres WHERE utilisateur_id = $1 AND deleted_at IS NULL")
        .bind(utilisateur_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Aucun arbre trouvé".into()))
}

// ════════════════════════════════════════════════════════════════════════════
// GET /api/arbre/mes-arbres
// ════════════════════════════════════════════════════════════════════════════

pub async fn mes_arbres(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;

    // Mon arbre
    let mon_arbre: Option<(Uuid, i64, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT a.id, COUNT(r.id)::BIGINT AS nb, a.created_at
         FROM arbre_genealogique.arbres a
         LEFT JOIN arbre_genealogique.rattachements r ON r.arbre_id = a.id AND r.deleted_at IS NULL
         WHERE a.utilisateur_id = $1 AND a.deleted_at IS NULL
         GROUP BY a.id",
    ).bind(uid).fetch_optional(pool.get_ref()).await?;

    let mon_arbre_resp = mon_arbre.map(|(id, nb, ca)| ArbreResumeResponse { id, nb_personnes: nb, created_at: ca });

    // Arbres partagés
    let partages = sqlx::query_as::<_, ArbrePartageRow>(
        "SELECT c.arbre_id, u.nom AS proprietaire_nom, u.prenom AS proprietaire_prenom,
                c.permission, COUNT(r.id)::BIGINT AS nb_personnes, c.created_at AS partage_depuis
         FROM arbre_genealogique.collaborateurs c
         JOIN arbre_genealogique.arbres a ON a.id = c.arbre_id
         JOIN iam.utilisateur u ON u.id = a.utilisateur_id
         LEFT JOIN arbre_genealogique.rattachements r ON r.arbre_id = c.arbre_id AND r.deleted_at IS NULL
         WHERE c.utilisateur_id = $1 AND a.deleted_at IS NULL
         GROUP BY c.arbre_id, u.nom, u.prenom, c.permission, c.created_at",
    ).bind(uid).fetch_all(pool.get_ref()).await?;

    let arbres_partages: Vec<ArbrePartageResponse> = partages.into_iter().map(|r| ArbrePartageResponse {
        arbre_id: r.arbre_id,
        proprietaire_nom: format!("{} {}", r.proprietaire_prenom, r.proprietaire_nom),
        permission: r.permission,
        nb_personnes: r.nb_personnes,
        partage_depuis: r.partage_depuis,
    }).collect();

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(MesArbresResponse { mon_arbre: mon_arbre_resp, arbres_partages }), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// POST /api/arbre/invitations
// ════════════════════════════════════════════════════════════════════════════

pub async fn creer_invitation(req: HttpRequest, pool: web::Data<PgPool>, body: web::Json<CreerInvitationDto>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_arbre_proprietaire(pool.get_ref(), uid).await?;

    if !["lecture_seule", "edition"].contains(&body.permission.as_str()) {
        return Err(ApiErreur::Validation("Permission invalide".into()));
    }

    // Vérifier limite 20
    let nb_collabs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM arbre_genealogique.collaborateurs WHERE arbre_id = $1")
        .bind(arbre_id).fetch_one(pool.get_ref()).await?;
    if nb_collabs.0 >= 20 {
        return Err(ApiErreur::Validation("Limite de 20 collaborateurs atteinte".into()));
    }

    // Vérifier pas déjà invité/collaborateur
    let existe: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM arbre_genealogique.invitations WHERE arbre_id = $1 AND email_invite = $2 AND statut = 'en_attente'",
    ).bind(arbre_id).bind(&body.email).fetch_optional(pool.get_ref()).await?;
    if existe.is_some() {
        return Err(ApiErreur::Conflit("Cette personne est déjà invitée".into()));
    }

    // Chercher si l'email correspond à un utilisateur inscrit
    let utilisateur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM iam.utilisateur WHERE email = $1 AND deleted_at IS NULL",
    ).bind(&body.email).fetch_optional(pool.get_ref()).await?;

    let inv_id: Uuid = sqlx::query_scalar(
        "INSERT INTO arbre_genealogique.invitations (arbre_id, email_invite, utilisateur_invite_id, permission, invite_par)
         VALUES ($1, $2, $3, $4, $5) RETURNING id",
    ).bind(arbre_id).bind(&body.email).bind(utilisateur_id).bind(&body.permission).bind(uid)
    .fetch_one(pool.get_ref()).await?;

    audit::log_action(pool.get_ref(), Some(uid), "creer_invitation", "arbre_genealogique", "invitations", Some(inv_id), None, Some(json!({"email": &body.email, "permission": &body.permission})), audit::extraire_ip(&req).as_deref(), audit::extraire_user_agent(&req).as_deref()).await;

    Ok(HttpResponse::Created().json(ApiResponse { success: true, data: Some(json!({"id": inv_id, "statut": "en_attente"})), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// GET /api/arbre/invitations : Invitations reçues par l'utilisateur
// ════════════════════════════════════════════════════════════════════════════

pub async fn lister_invitations_recues(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;
    let email: Option<String> = sqlx::query_scalar("SELECT email FROM iam.utilisateur WHERE id = $1")
        .bind(uid).fetch_optional(pool.get_ref()).await?;
    let Some(email) = email else { return Err(ApiErreur::NonTrouve("Utilisateur introuvable".into())); };

    let invitations = sqlx::query_as::<_, InvitationAvecProprietaireRow>(
        "SELECT i.id, i.arbre_id, i.permission, i.statut, i.created_at,
                u.nom AS proprietaire_nom, u.prenom AS proprietaire_prenom
         FROM arbre_genealogique.invitations i
         JOIN arbre_genealogique.arbres a ON a.id = i.arbre_id
         JOIN iam.utilisateur u ON u.id = a.utilisateur_id
         WHERE i.email_invite = $1 AND i.statut = 'en_attente'
         ORDER BY i.created_at DESC",
    ).bind(&email).fetch_all(pool.get_ref()).await?;

    let resp: Vec<InvitationResponse> = invitations.into_iter().map(|i| InvitationResponse {
        id: i.id, arbre_id: i.arbre_id, permission: i.permission, statut: i.statut,
        proprietaire_nom: format!("{} {}", i.proprietaire_prenom, i.proprietaire_nom),
        created_at: i.created_at,
    }).collect();

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(resp), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// POST /api/arbre/invitations/{id}/accepter
// ════════════════════════════════════════════════════════════════════════════

pub async fn accepter_invitation(req: HttpRequest, pool: web::Data<PgPool>, chemin: web::Path<Uuid>) -> Result<HttpResponse, ApiErreur> {
    let inv_id = chemin.into_inner();
    let uid = extraire_utilisateur_id(&req)?;

    let inv: Option<(Uuid, String, String)> = sqlx::query_as(
        "SELECT arbre_id, permission, email_invite FROM arbre_genealogique.invitations WHERE id = $1 AND statut = 'en_attente'",
    ).bind(inv_id).fetch_optional(pool.get_ref()).await?;

    let Some((arbre_id, permission, _)) = inv else {
        return Err(ApiErreur::NonTrouve("Invitation introuvable ou déjà traitée".into()));
    };

    // Créer le collaborateur
    sqlx::query("INSERT INTO arbre_genealogique.collaborateurs (arbre_id, utilisateur_id, permission, invitation_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
        .bind(arbre_id).bind(uid).bind(&permission).bind(inv_id)
        .execute(pool.get_ref()).await?;

    sqlx::query("UPDATE arbre_genealogique.invitations SET statut = 'acceptee', traitee_le = NOW() WHERE id = $1")
        .bind(inv_id).execute(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"arbre_id": arbre_id, "permission": permission})), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// POST /api/arbre/invitations/{id}/refuser
// ════════════════════════════════════════════════════════════════════════════

pub async fn refuser_invitation(req: HttpRequest, pool: web::Data<PgPool>, chemin: web::Path<Uuid>) -> Result<HttpResponse, ApiErreur> {
    let inv_id = chemin.into_inner();
    let _uid = extraire_utilisateur_id(&req)?;

    sqlx::query("UPDATE arbre_genealogique.invitations SET statut = 'refusee', traitee_le = NOW() WHERE id = $1 AND statut = 'en_attente'")
        .bind(inv_id).execute(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"message": "Invitation refusée"})), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// GET /api/arbre/{arbre_id}/collaborateurs
// ════════════════════════════════════════════════════════════════════════════

pub async fn lister_collaborateurs(req: HttpRequest, pool: web::Data<PgPool>, chemin: web::Path<Uuid>) -> Result<HttpResponse, ApiErreur> {
    let arbre_id = chemin.into_inner();
    let uid = extraire_utilisateur_id(&req)?;

    let acces = verifier_acces_arbre(pool.get_ref(), uid, arbre_id).await?;
    if acces.as_deref() != Some("proprietaire") {
        return Err(ApiErreur::AccesInterdit("Seul le propriétaire peut voir les collaborateurs".into()));
    }

    let collabs = sqlx::query_as::<_, CollaborateurAvecProfilRow>(
        "SELECT c.id, c.utilisateur_id, u.nom, u.prenom, u.email, c.permission, c.created_at
         FROM arbre_genealogique.collaborateurs c
         JOIN iam.utilisateur u ON u.id = c.utilisateur_id
         WHERE c.arbre_id = $1
         ORDER BY c.created_at DESC",
    ).bind(arbre_id).fetch_all(pool.get_ref()).await?;

    let resp: Vec<CollaborateurResponse> = collabs.into_iter().map(|c| CollaborateurResponse {
        id: c.id, utilisateur_id: c.utilisateur_id, nom: c.nom, prenom: c.prenom,
        email: c.email, permission: c.permission, created_at: c.created_at,
    }).collect();

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(resp), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// PUT /api/arbre/collaborateurs/{id}
// ════════════════════════════════════════════════════════════════════════════

pub async fn modifier_permission(req: HttpRequest, pool: web::Data<PgPool>, chemin: web::Path<Uuid>, body: web::Json<ModifierPermissionDto>) -> Result<HttpResponse, ApiErreur> {
    let collab_id = chemin.into_inner();
    let uid = extraire_utilisateur_id(&req)?;

    let arbre_id: Option<Uuid> = sqlx::query_scalar("SELECT arbre_id FROM arbre_genealogique.collaborateurs WHERE id = $1")
        .bind(collab_id).fetch_optional(pool.get_ref()).await?;
    let Some(arbre_id) = arbre_id else { return Err(ApiErreur::NonTrouve("Collaborateur introuvable".into())); };

    let acces = verifier_acces_arbre(pool.get_ref(), uid, arbre_id).await?;
    if acces.as_deref() != Some("proprietaire") {
        return Err(ApiErreur::AccesInterdit("Seul le propriétaire peut modifier les permissions".into()));
    }

    sqlx::query("UPDATE arbre_genealogique.collaborateurs SET permission = $1 WHERE id = $2")
        .bind(&body.permission).bind(collab_id).execute(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"message": "Permission mise à jour"})), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// DELETE /api/arbre/collaborateurs/{id}
// ════════════════════════════════════════════════════════════════════════════

pub async fn revoquer_collaborateur(req: HttpRequest, pool: web::Data<PgPool>, chemin: web::Path<Uuid>) -> Result<HttpResponse, ApiErreur> {
    let collab_id = chemin.into_inner();
    let uid = extraire_utilisateur_id(&req)?;

    let arbre_id: Option<Uuid> = sqlx::query_scalar("SELECT arbre_id FROM arbre_genealogique.collaborateurs WHERE id = $1")
        .bind(collab_id).fetch_optional(pool.get_ref()).await?;
    let Some(arbre_id) = arbre_id else { return Err(ApiErreur::NonTrouve("Collaborateur introuvable".into())); };

    let acces = verifier_acces_arbre(pool.get_ref(), uid, arbre_id).await?;
    if acces.as_deref() != Some("proprietaire") {
        return Err(ApiErreur::AccesInterdit("Seul le propriétaire peut révoquer".into()));
    }

    sqlx::query("DELETE FROM arbre_genealogique.collaborateurs WHERE id = $1").bind(collab_id).execute(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"message": "Accès révoqué"})), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// PUT /api/arbre/{arbre_id}/confidentialite
// ════════════════════════════════════════════════════════════════════════════

pub async fn modifier_confidentialite_arbre(req: HttpRequest, pool: web::Data<PgPool>, chemin: web::Path<Uuid>, body: web::Json<ConfidentialiteArbreDto>) -> Result<HttpResponse, ApiErreur> {
    let arbre_id = chemin.into_inner();
    let uid = extraire_utilisateur_id(&req)?;

    let acces = verifier_acces_arbre(pool.get_ref(), uid, arbre_id).await?;
    if acces.as_deref() != Some("proprietaire") {
        return Err(ApiErreur::AccesInterdit("Seul le propriétaire peut modifier la confidentialité".into()));
    }

    sqlx::query("UPDATE arbre_genealogique.arbres SET arbre_prive = $1 WHERE id = $2")
        .bind(body.arbre_prive).bind(arbre_id).execute(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"arbre_prive": body.arbre_prive})), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// PUT /api/arbre/personnes/{id}/confidentialite
// ════════════════════════════════════════════════════════════════════════════

pub async fn modifier_confidentialite_personne(req: HttpRequest, pool: web::Data<PgPool>, chemin: web::Path<Uuid>, body: web::Json<ConfidentialitePersonneDto>) -> Result<HttpResponse, ApiErreur> {
    let personne_id = chemin.into_inner();
    let uid = extraire_utilisateur_id(&req)?;

    // Vérifier que la personne appartient à l'arbre de l'utilisateur (propriétaire)
    let arbre_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT r.arbre_id FROM arbre_genealogique.rattachements r
         JOIN arbre_genealogique.arbres a ON a.id = r.arbre_id
         WHERE r.personne_id = $1 AND a.utilisateur_id = $2 AND r.deleted_at IS NULL",
    ).bind(personne_id).bind(uid).fetch_optional(pool.get_ref()).await?;

    if arbre_id.is_none() {
        return Err(ApiErreur::AccesInterdit("Seul le propriétaire peut modifier la confidentialité".into()));
    }

    sqlx::query("UPDATE arbre_genealogique.personnes SET visible_matching = $1 WHERE id = $2")
        .bind(body.visible_matching).bind(personne_id).execute(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"visible_matching": body.visible_matching})), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// GET /api/arbre/{arbre_id}/historique
// ════════════════════════════════════════════════════════════════════════════

pub async fn obtenir_historique(req: HttpRequest, pool: web::Data<PgPool>, chemin: web::Path<Uuid>) -> Result<HttpResponse, ApiErreur> {
    let arbre_id = chemin.into_inner();
    let uid = extraire_utilisateur_id(&req)?;

    let acces = verifier_acces_arbre(pool.get_ref(), uid, arbre_id).await?;
    if acces.is_none() {
        return Err(ApiErreur::AccesInterdit("Accès refusé à cet arbre".into()));
    }

    // Filtrer audit_log pour cet arbre (les actions sur arbre_genealogique avec record_id dans les rattachements de cet arbre)
    let entrees: Vec<(Uuid, String, Option<Uuid>, String, String, Option<serde_json::Value>, Option<serde_json::Value>, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT al.id, al.action, al.utilisateur_id, COALESCE(u.nom, '') AS nom, COALESCE(u.prenom, '') AS prenom,
                al.avant, al.apres, al.created_at
         FROM shared.audit_log al
         LEFT JOIN iam.utilisateur u ON u.id = al.utilisateur_id
         WHERE al.schema_name = 'arbre_genealogique'
         ORDER BY al.created_at DESC
         LIMIT 50",
    ).fetch_all(pool.get_ref()).await?;

    let resp: Vec<serde_json::Value> = entrees.into_iter().map(|(id, action, _, nom, prenom, avant, apres, ca)| {
        json!({ "id": id, "action": action, "auteur_nom": nom, "auteur_prenom": prenom, "details_avant": avant, "details_apres": apres, "created_at": ca })
    }).collect();

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"entrees": resp, "total": resp.len()})), error: None }))
}

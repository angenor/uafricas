//! Co-détention des supports médias et mise en relation
//! (feature 001-refonte-tele-radio, US5 et US6, migration 09m).
//!
//! Endpoints membre :
//!   GET    /api/medias/{type_support}/{support_id}/detenteurs
//!   POST   /api/medias/{type_support}/{support_id}/invitations
//!   DELETE /api/medias/{type_support}/{support_id}/detenteurs/{utilisateur_id}
//!   POST   /api/medias/{type_support}/{support_id}/contacter
//!   GET    /api/medias/supports/moi
//!   GET    /api/medias/invitations/moi
//!   PATCH  /api/medias/invitations/{id}/accepter · /refuser
//!
//! Ces routes s'adressent à des MEMBRES. L'extracteur `AdminUtilisateur`
//! rejetterait tout non-admin (`middleware/admin.rs:100-105`) : la garde est
//! `garde_detenteur`, sur le modèle de `garde_proprietaire`
//! (`handlers/annonces.rs:111`).

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_social::extraire_utilisateur_id;
use crate::models::amitie::paire_canonique;
use crate::models::media_detention::{
    rang_role, table_pour_support, valider_type_support, ContacterSupportRequest,
    InvitationDetenteurRow, InviterDetenteurRequest, SupportDetenteurRow, CONTENU_MESSAGE_MAX,
    INVITATION_DETENTEUR_COLONNES, SUPPORT_DETENTEUR_COLONNES,
};
use crate::models::messagerie::{evt_message, MessageResponse};
use crate::models::notification;
use crate::services::messagerie_sse::RegistreSse;
use crate::services::audit;
use crate::ApiResponse;

pub fn exiger_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

// ═══════════════════════════════════════════════════════════════════════════
// Garde d'autorisation : le cœur du domaine
// ═══════════════════════════════════════════════════════════════════════════

/// Vérifie que `moi` détient le support à un rôle au moins aussi étendu que
/// `role_minimal`.
///
/// Les rôles sont ordonnés : `proprietaire` ⊃ `co_detenteur` ⊃ `programmateur`.
/// Exiger `programmateur` admet donc les trois, exiger `proprietaire` n'admet
/// que lui.
///
/// Renvoie 404 si le support n'existe pas, un membre qui n'y a pas accès n'a
/// pas à distinguer « inexistant » de « interdit », 403 s'il n'est pas
/// détenteur actif ou si son rôle est insuffisant.
pub async fn garde_detenteur(
    pool: &PgPool,
    type_support: &str,
    support_id: Uuid,
    moi: Uuid,
    role_minimal: &str,
) -> Result<String, ApiErreur> {
    valider_type_support(type_support)?;
    let table = table_pour_support(type_support).expect("type de support validé");

    let existe: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table} WHERE id = $1 AND deleted_at IS NULL)"
    ))
    .bind(support_id)
    .fetch_one(pool)
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Support introuvable".into()));
    }

    let role: Option<String> = sqlx::query_scalar(
        "SELECT role::text FROM media_content.support_detenteur
          WHERE type_support = $1::media_content.type_support_media
            AND support_id = $2 AND utilisateur_id = $3 AND actif = TRUE",
    )
    .bind(type_support)
    .bind(support_id)
    .bind(moi)
    .fetch_optional(pool)
    .await?;

    let role = role.ok_or_else(|| {
        ApiErreur::AccesInterdit("Vous ne détenez pas ce support".to_string())
    })?;

    let rang_requis = rang_role(role_minimal).unwrap_or(usize::MAX);
    let rang_effectif = rang_role(&role).unwrap_or(usize::MAX);
    if rang_effectif > rang_requis {
        return Err(ApiErreur::AccesInterdit(format!(
            "Cette action requiert le rôle « {} » sur ce support",
            role_minimal
        )));
    }

    Ok(role)
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/medias/{type_support}/{support_id}/detenteurs
// ═══════════════════════════════════════════════════════════════════════════

pub async fn lister_detenteurs(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id) = chemin.into_inner();
    // Le rôle le moins étendu suffit à consulter la liste : un programmateur
    // doit savoir à qui s'adresser.
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "programmateur").await?;

    let rows = sqlx::query_as::<_, SupportDetenteurRow>(&format!(
        "SELECT {SUPPORT_DETENTEUR_COLONNES},
                u.nom AS utilisateur_nom, u.prenom AS utilisateur_prenom,
                u.email AS utilisateur_email, u.photo_url AS utilisateur_photo
           FROM media_content.support_detenteur sd
           LEFT JOIN iam.utilisateur u ON u.id = sd.utilisateur_id
          WHERE sd.type_support = $1::media_content.type_support_media
            AND sd.support_id = $2
          ORDER BY sd.actif DESC, sd.role ASC, sd.designe_at ASC"
    ))
    .bind(&type_support)
    .bind(support_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(
            rows.into_iter()
                .map(|r| r.to_response())
                .collect::<Vec<_>>(),
        ),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/medias/supports/moi : « quels supports est-ce que je détiens ? »
// ═══════════════════════════════════════════════════════════════════════════

pub async fn lister_mes_supports(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;

    // Les deux tables de support sont interrogées puis fusionnées : elles ne
    // partagent ni schéma de colonnes ni contrainte, une UNION explicite est
    // plus lisible qu'un LATERAL polymorphe.
    let rows = sqlx::query_as::<_, SupportDetenteurRow>(&format!(
        "SELECT {SUPPORT_DETENTEUR_COLONNES},
                NULL::varchar AS utilisateur_nom, NULL::varchar AS utilisateur_prenom,
                NULL::varchar AS utilisateur_email, NULL::varchar AS utilisateur_photo,
                c.nom AS support_nom, c.slug AS support_slug,
                c.image_couverture_url AS support_image
           FROM media_content.support_detenteur sd
           JOIN media_content.chaine_tv c ON c.id = sd.support_id AND c.deleted_at IS NULL
          WHERE sd.utilisateur_id = $1 AND sd.actif = TRUE
            AND sd.type_support = 'chaine_tv'
          UNION ALL
         SELECT {SUPPORT_DETENTEUR_COLONNES},
                NULL::varchar, NULL::varchar, NULL::varchar, NULL::varchar,
                s.nom, s.slug, s.image_couverture_url
           FROM media_content.support_detenteur sd
           JOIN media_content.station_radio s ON s.id = sd.support_id AND s.deleted_at IS NULL
          WHERE sd.utilisateur_id = $1 AND sd.actif = TRUE
            AND sd.type_support = 'station_radio'
          ORDER BY support_nom ASC"
    ))
    .bind(moi)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(
            rows.into_iter()
                .map(|r| r.to_response())
                .collect::<Vec<_>>(),
        ),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// POST /api/medias/{type_support}/{support_id}/invitations
// ═══════════════════════════════════════════════════════════════════════════

/// Seul le propriétaire associe d'autres membres à son support.
///
/// L'invitation porte un courriel : on invite aussi des membres qu'on ne sait
/// pas nommer dans l'annuaire. Si le destinataire est reconnu, il est notifié
/// immédiatement ; sinon l'invitation l'attend à son inscription.
pub async fn inviter_detenteur(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<InviterDetenteurRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id) = chemin.into_inner();
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "proprietaire").await?;

    let role = body.role_valide()?;
    let email = body.email.trim().to_lowercase();
    if email.is_empty() || !email.contains('@') {
        return Err(ApiErreur::Validation(
            "L'adresse électronique de l'invité est requise".into(),
        ));
    }

    let destinataire: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM iam.utilisateur
          WHERE LOWER(email) = $1 AND deleted_at IS NULL",
    )
    .bind(&email)
    .fetch_optional(pool.get_ref())
    .await?;

    // Inviter quelqu'un qui détient déjà le support n'a pas de sens et
    // produirait une invitation impossible à accepter (409, cf. contrat).
    if let Some(id) = destinataire {
        let deja: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM media_content.support_detenteur
                WHERE type_support = $1::media_content.type_support_media
                  AND support_id = $2 AND utilisateur_id = $3 AND actif = TRUE)",
        )
        .bind(&type_support)
        .bind(support_id)
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?;
        if deja {
            return Err(ApiErreur::Conflit(
                "Ce membre détient déjà ce support".into(),
            ));
        }
    }

    let invitation_id: Uuid = sqlx::query_scalar(
        "INSERT INTO media_content.invitation_detenteur
            (type_support, support_id, email_invite, utilisateur_invite_id, role, invite_par)
         VALUES ($1::media_content.type_support_media, $2, $3, $4,
                 $5::media_content.role_detenteur, $6)
         RETURNING id",
    )
    .bind(&type_support)
    .bind(support_id)
    .bind(&email)
    .bind(destinataire)
    .bind(role)
    .bind(moi)
    .fetch_one(pool.get_ref())
    .await?;

    if let Some(id) = destinataire {
        let nom_support = nom_support(pool.get_ref(), &type_support, support_id).await?;
        notification::creer_notification(
            pool.get_ref(),
            id,
            notification::media::CODETENTEUR_AJOUTE,
            &format!(
                "Vous êtes invité à co-détenir « {} ». Acceptez ou refusez depuis vos invitations.",
                nom_support
            ),
            Some("/mon-compte/invitations-medias"),
        )
        .await;
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "media_content",
        "invitation_detenteur",
        Some(invitation_id),
        None,
        Some(serde_json::json!({
            "type_support": type_support,
            "support_id": support_id,
            "email_invite": email,
            "role": role,
            "statut": "en_attente",
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": invitation_id,
            "email_invite": email,
            "role": role,
            "destinataire_reconnu": destinataire.is_some(),
        })),
        error: None,
    }))
}

async fn nom_support(
    pool: &PgPool,
    type_support: &str,
    support_id: Uuid,
) -> Result<String, ApiErreur> {
    let table = table_pour_support(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;
    let nom: Option<String> =
        sqlx::query_scalar(&format!("SELECT nom FROM {table} WHERE id = $1"))
            .bind(support_id)
            .fetch_optional(pool)
            .await?;
    Ok(nom.unwrap_or_else(|| "ce support".to_string()))
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/medias/invitations/moi
// ═══════════════════════════════════════════════════════════════════════════

/// Les invitations reçues, qu'elles aient été adressées à l'identifiant du
/// membre ou seulement à son courriel : ce second cas est celui d'un membre
/// invité avant d'être reconnu dans l'annuaire.
pub async fn lister_mes_invitations(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;

    let rows = sqlx::query_as::<_, InvitationDetenteurRow>(&format!(
        "SELECT {INVITATION_DETENTEUR_COLONNES},
                COALESCE(c.nom, s.nom) AS support_nom,
                COALESCE(c.slug, s.slug) AS support_slug,
                up.nom AS invite_par_nom, up.prenom AS invite_par_prenom
           FROM media_content.invitation_detenteur i
           LEFT JOIN media_content.chaine_tv c
                  ON c.id = i.support_id AND i.type_support = 'chaine_tv'
           LEFT JOIN media_content.station_radio s
                  ON s.id = i.support_id AND i.type_support = 'station_radio'
           LEFT JOIN iam.utilisateur up ON up.id = i.invite_par
          WHERE i.utilisateur_invite_id = $1
             OR LOWER(i.email_invite) = (SELECT LOWER(email) FROM iam.utilisateur WHERE id = $1)
          ORDER BY i.created_at DESC"
    ))
    .bind(moi)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(
            rows.into_iter()
                .map(|r| r.to_response())
                .collect::<Vec<_>>(),
        ),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PATCH /api/medias/invitations/{id}/accepter · /refuser
// ═══════════════════════════════════════════════════════════════════════════

/// Acceptation : la ligne de détention et la bascule de l'invitation sont
/// écrites dans une même transaction : une invitation acceptée sans détention
/// créée laisserait le membre persuadé d'un accès qu'il n'a pas.
pub async fn accepter_invitation(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let invitation_id = chemin.into_inner();

    let mut tx = pool.begin().await?;

    let ligne: Option<(String, Uuid, String, String, String, Option<Uuid>, DateTime<Utc>)> =
        sqlx::query_as(
            "SELECT i.type_support::text, i.support_id, i.role::text, i.statut, i.email_invite,
                    i.utilisateur_invite_id, i.expire_at
               FROM media_content.invitation_detenteur i
              WHERE i.id = $1
              FOR UPDATE",
        )
        .bind(invitation_id)
        .fetch_optional(&mut *tx)
        .await?;

    let (type_support, support_id, role, statut, email_invite, invite_id, expire_at) =
        ligne.ok_or_else(|| ApiErreur::NonTrouve("Invitation introuvable".into()))?;

    verifier_destinataire(&mut tx, moi, invite_id, &email_invite).await?;

    if statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette invitation a déjà été traitée".into(),
        ));
    }
    // Aucune tâche de fond ne bascule les invitations périmées : c'est la
    // lecture qui tranche, ici comme dans `InvitationDetenteurRow::to_response`.
    if expire_at < Utc::now() {
        sqlx::query(
            "UPDATE media_content.invitation_detenteur
                SET statut = 'expiree', traitee_le = NOW() WHERE id = $1",
        )
        .bind(invitation_id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        return Err(ApiErreur::Conflit(
            "Cette invitation a expiré : demandez-en une nouvelle".into(),
        ));
    }

    // Upsert-réactivation à trois branches (modèle
    // `admin/moderateurs_afrolang.rs:59-190`) : le retrait étant un soft
    // delete, une ligne inactive peut préexister.
    sqlx::query(
        "INSERT INTO media_content.support_detenteur
            (type_support, support_id, utilisateur_id, role, designe_par)
         VALUES ($1::media_content.type_support_media, $2, $3,
                 $4::media_content.role_detenteur, $5)
         ON CONFLICT (type_support, support_id, utilisateur_id)
         DO UPDATE SET actif = TRUE,
                       retire_at = NULL,
                       role = EXCLUDED.role,
                       designe_par = EXCLUDED.designe_par,
                       designe_at = NOW(),
                       updated_at = NOW()",
    )
    .bind(&type_support)
    .bind(support_id)
    .bind(moi)
    .bind(&role)
    .bind(moi)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE media_content.invitation_detenteur
            SET statut = 'acceptee', utilisateur_invite_id = $2, traitee_le = NOW()
          WHERE id = $1",
    )
    .bind(invitation_id)
    .bind(moi)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "media_content",
        "invitation_detenteur",
        Some(invitation_id),
        Some(serde_json::json!({ "statut": "en_attente" })),
        Some(serde_json::json!({
            "statut": "acceptee",
            "type_support": type_support,
            "support_id": support_id,
            "role": role,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "statut": "acceptee",
            "type_support": type_support,
            "support_id": support_id,
            "role": role,
        })),
        error: None,
    }))
}

pub async fn refuser_invitation(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let invitation_id = chemin.into_inner();

    let mut tx = pool.begin().await?;

    let ligne: Option<(String, String, Option<Uuid>)> = sqlx::query_as(
        "SELECT statut, email_invite, utilisateur_invite_id
           FROM media_content.invitation_detenteur
          WHERE id = $1 FOR UPDATE",
    )
    .bind(invitation_id)
    .fetch_optional(&mut *tx)
    .await?;

    let (statut, email_invite, invite_id) =
        ligne.ok_or_else(|| ApiErreur::NonTrouve("Invitation introuvable".into()))?;

    verifier_destinataire(&mut tx, moi, invite_id, &email_invite).await?;

    if statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette invitation a déjà été traitée".into(),
        ));
    }

    sqlx::query(
        "UPDATE media_content.invitation_detenteur
            SET statut = 'refusee', utilisateur_invite_id = $2, traitee_le = NOW()
          WHERE id = $1",
    )
    .bind(invitation_id)
    .bind(moi)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "media_content",
        "invitation_detenteur",
        Some(invitation_id),
        Some(serde_json::json!({ "statut": "en_attente" })),
        Some(serde_json::json!({ "statut": "refusee" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "statut": "refusee" })),
        error: None,
    }))
}

/// Une invitation ne se traite que par son destinataire, reconnu par son
/// identifiant, ou à défaut par son courriel.
async fn verifier_destinataire(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    moi: Uuid,
    utilisateur_invite_id: Option<Uuid>,
    email_invite: &str,
) -> Result<(), ApiErreur> {
    if utilisateur_invite_id == Some(moi) {
        return Ok(());
    }
    let mon_email: Option<String> =
        sqlx::query_scalar("SELECT LOWER(email) FROM iam.utilisateur WHERE id = $1")
            .bind(moi)
            .fetch_optional(&mut **tx)
            .await?;
    if mon_email.as_deref() == Some(email_invite.to_lowercase().as_str()) {
        return Ok(());
    }
    Err(ApiErreur::AccesInterdit(
        "Cette invitation ne vous est pas adressée".into(),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// DELETE /api/medias/{type_support}/{support_id}/detenteurs/{utilisateur_id}
// ═══════════════════════════════════════════════════════════════════════════

/// Retrait par le propriétaire. Soft delete : l'historique de détention n'est
/// jamais effacé, et une ligne inactive se réactive à la prochaine invitation.
pub async fn retirer_detenteur(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id, utilisateur_id) = chemin.into_inner();
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "proprietaire").await?;

    // Le propriétaire ne se retire pas lui-même : le support se retrouverait
    // sans propriétaire alors qu'un seul peut être désigné.
    if utilisateur_id == moi {
        return Err(ApiErreur::Validation(
            "Le propriétaire ne peut pas se retirer lui-même de son support".into(),
        ));
    }

    let modifie = sqlx::query(
        "UPDATE media_content.support_detenteur
            SET actif = FALSE, retire_at = NOW(), updated_at = NOW()
          WHERE type_support = $1::media_content.type_support_media
            AND support_id = $2 AND utilisateur_id = $3 AND actif = TRUE",
    )
    .bind(&type_support)
    .bind(support_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?
    .rows_affected();

    if modifie == 0 {
        return Err(ApiErreur::NonTrouve(
            "Ce membre ne détient pas ce support".into(),
        ));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "DELETE",
        "media_content",
        "support_detenteur",
        None,
        Some(serde_json::json!({
            "actif": true,
            "type_support": type_support,
            "support_id": support_id,
            "utilisateur_id": utilisateur_id,
        })),
        Some(serde_json::json!({
            "actif": false,
            "type_support": type_support,
            "support_id": support_id,
            "utilisateur_id": utilisateur_id,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// POST /api/medias/{type_support}/{support_id}/contacter (FR-046)
// ═══════════════════════════════════════════════════════════════════════════

/// Ouvre une conversation avec le propriétaire du support et y dépose un
/// premier message.
///
/// La messagerie n'autorise l'envoi que si amitié active **ou** conversation
/// préexistante (`handlers/messagerie.rs:291-302`) : seul un handler métier
/// peut créer ce canal. Ce code duplique `contacter_auteur`
/// (`handlers/annonces.rs:893`) : aucun endpoint générique d'ouverture de
/// conversation n'existe (R17).
pub async fn contacter_support(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<ContacterSupportRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;

    let contenu = body.message.trim().to_string();
    let longueur = contenu.chars().count();
    if contenu.is_empty() || longueur > CONTENU_MESSAGE_MAX {
        return Err(ApiErreur::Validation(
            "Le message doit contenir entre 1 et 2000 caractères".into(),
        ));
    }

    let table = table_pour_support(&type_support).expect("type de support validé");
    let publie: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table}
            WHERE id = $1 AND etat = 'publie' AND deleted_at IS NULL)"
    ))
    .bind(support_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !publie {
        return Err(ApiErreur::NonTrouve(
            "Support introuvable ou non disponible".into(),
        ));
    }

    // On s'adresse au propriétaire ; à défaut, le retrait du dernier
    // détenteur est un cas admis : au co-détenteur actif le plus ancien.
    let destinataire: Option<Uuid> = sqlx::query_scalar(
        "SELECT utilisateur_id FROM media_content.support_detenteur
          WHERE type_support = $1::media_content.type_support_media
            AND support_id = $2 AND actif = TRUE
          ORDER BY (role = 'proprietaire') DESC, designe_at ASC
          LIMIT 1",
    )
    .bind(&type_support)
    .bind(support_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let destinataire = destinataire.ok_or_else(|| {
        ApiErreur::NonTrouve("Ce support n'a aucun détenteur à contacter".into())
    })?;

    if destinataire == moi {
        return Err(ApiErreur::Validation(
            "Vous détenez déjà ce support".into(),
        ));
    }

    let bloque: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM social.blocage
            WHERE (bloqueur_id = $1 AND bloque_id = $2) OR (bloqueur_id = $2 AND bloque_id = $1)
        )",
    )
    .bind(moi)
    .bind(destinataire)
    .fetch_one(pool.get_ref())
    .await?;
    if bloque {
        return Err(ApiErreur::AccesInterdit(
            "Échange impossible avec ce membre".into(),
        ));
    }

    // Contrainte `ck_conversation_ordre : a_id < b_id`.
    let (a, b) = paire_canonique(moi, destinataire);
    let conv_id: Uuid = sqlx::query_scalar(
        "INSERT INTO social.conversation (utilisateur_a_id, utilisateur_b_id)
         VALUES ($1, $2)
         ON CONFLICT (utilisateur_a_id, utilisateur_b_id)
            DO UPDATE SET annonce_id = social.conversation.annonce_id
         RETURNING id",
    )
    .bind(a)
    .bind(b)
    .fetch_one(pool.get_ref())
    .await?;

    let (message_id, created_at): (Uuid, DateTime<Utc>) = sqlx::query_as(
        "INSERT INTO social.message (conversation_id, expediteur_id, contenu)
         VALUES ($1, $2, $3) RETURNING id, created_at",
    )
    .bind(conv_id)
    .bind(moi)
    .bind(&contenu)
    .fetch_one(pool.get_ref())
    .await?;

    sqlx::query("UPDATE social.conversation SET dernier_message_at = NOW() WHERE id = $1")
        .bind(conv_id)
        .execute(pool.get_ref())
        .await?;

    let message = MessageResponse {
        id: message_id,
        expediteur_id: moi,
        contenu: Some(contenu),
        supprime: false,
        created_at,
        lu_at: None,
    };

    let evt = evt_message(conv_id, &message);
    sse.publier(destinataire, &evt);
    sse.publier(moi, &evt);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "social",
        "message",
        Some(message_id),
        None,
        Some(serde_json::json!({
            "conversation_id": conv_id,
            "type_support": type_support,
            "support_id": support_id,
            "longueur": longueur,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "conversation_id": conv_id,
            "destinataire_id": destinataire,
            "message": message,
        })),
        error: None,
    }))
}

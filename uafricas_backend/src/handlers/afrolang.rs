use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::config::LivekitConfig;
use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::afrolang::{
    AfrolangStatsResponse, CreerMessageRequest, CreerRessourceLienRequest,
    CreerSallePriveePubliquePayload, CreerSessionRequest, DemarrerRejoindreResponse,
    GroupeEthniqueFiltres, GroupeEthniqueListeResponse, GroupeEthniqueResume,
    MessageSessionResponse, MessageSessionRow, MessagesFiltres, ModerateurResponse,
    ModifierCodeAccesRequest, ModifierMaxParticipantsRequest, ModifierSallePriveeRequest,
    ModifierSalleRequest, PropositionListeResponse, PropositionMesFiltres, PropositionResponse,
    PropositionSalleRow, PropositionStatut, RejoindreRequest, RessourceSalleResponse,
    RessourceSalleRow, SalleDetailResponse, SalleFiltres, SalleListeResponse, SallePriveeAPI,
    SallePriveeDetailResponse, SallePriveeRow,
    SalleRow, SessionDetailResponse, SessionFiltres, SessionListeResponse, SessionParticipantRow,
    SessionRow, SoumettrePropositionRequest, TerritoireResponse, TerritoireRow,
    TransfererModerationRequest, VerifierCodeAccesRequest,
    VerifierCodeAccesResponse,
    COLONNES_PROPOSITION, GROUPE_ETHNIQUE_RESUME_COLONNES, MESSAGE_SESSION_COLONNES,
    RESSOURCE_SALLE_COLONNES, SALLE_COLONNES, SALLE_PRIVEE_COLONNES, SESSION_COLONNES,
    generer_slug,
};
use crate::models::notification;
use crate::services::{afrolang_rate_limit, audit, livekit_moderation};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ══════════════════════════════════════════════════════════════════════════
// Fonctions utilitaires
// ══════════════════════════════════════════════════════════════════════════

/// Extraire l'utilisateur connecte depuis le header Authorization
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Verifier si l'utilisateur a le role admin
async fn verifier_admin(pool: &PgPool, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.utilisateur_role ur
            JOIN iam.role r ON ur.role_id = r.id
            WHERE ur.utilisateur_id = $1 AND r.slug = 'admin'
        )",
    )
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(is_admin)
}

/// Lire le contenu texte d'un champ multipart
async fn lire_champ_texte(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut contenu = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture champ: {}", e)))?;
        contenu.extend_from_slice(&data);
    }
    String::from_utf8(contenu)
        .map_err(|e| ApiErreur::Upload(format!("Encodage UTF-8 invalide: {}", e)))
}

/// Sauvegarder un fichier uploade sur le disque local
async fn sauvegarder_fichier(field: &mut actix_multipart::Field, chemin: &str) -> Result<(), ApiErreur> {
    if let Some(parent) = std::path::Path::new(chemin).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le repertoire: {}", e)))?;
    }
    let mut fichier = std::fs::File::create(chemin)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le fichier: {}", e)))?;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
        fichier
            .write_all(&data)
            .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture fichier: {}", e)))?;
    }
    Ok(())
}

fn calculer_total_pages(total: i64, par_page: i64) -> i64 {
    if total == 0 { 1 } else { (total as f64 / par_page as f64).ceil() as i64 }
}

/// Indique si `utilisateur_id` a un accès actif à la salle privée `salle_privee_id`
/// (feature 001-ressources-fermeture-session, FR-001 option C).
///
/// Un accès est dit actif s'il existe une ligne dans `afrolang.acces_salle_privee`
/// avec `revoque_at IS NULL`. Le créateur de la salle privée a toujours accès
/// (test d'identité direct sans passage par la table).
pub async fn a_acces_salle_privee_actif(
    pool: &PgPool,
    salle_privee_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<bool, sqlx::Error> {
    // Court-circuit créateur : pas de saisie de code requise pour son auteur.
    let est_createur: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.salle_privee
            WHERE id = $1 AND cree_par = $2
              AND archivee_at IS NULL AND deleted_at IS NULL
        )",
    )
    .bind(salle_privee_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    if est_createur {
        return Ok(true);
    }

    sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.acces_salle_privee
            WHERE salle_privee_id = $1 AND utilisateur_id = $2 AND revoque_at IS NULL
        )",
    )
    .bind(salle_privee_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await
}

/// Helper d'autorisation centralisé (FR-019, feature 001-admin-salles-publiques).
///
/// Retourne `true` si l'utilisateur a une nomination active comme administrateur
/// de la salle publique passée en argument. Aucune capacité concrète n'est encore
/// branchée à cette table — ce helper sert de point d'autorisation unique pour
/// toutes les futures capacités du rôle (modération étendue, gestion des
/// ressources, etc.) sans rupture de compatibilité.
pub async fn est_administrateur_salle(
    pool: &PgPool,
    salle_id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.salle_administrateur
            WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
        )",
    )
    .bind(salle_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}

/// Identifie le niveau de modérateur EFFECTIF d'un utilisateur pour une session
/// (refonte modération multi-modérateurs, 2026-06).
///
/// Ordre :
/// 1. Admin plateforme (`iam.role.slug='admin'`) → `AdminPlateforme` (« quoi qu'il arrive »).
/// 2. Salle publique : admin de salle actif → `AdminSalle`.
/// 3. Salle privée : créateur → `CreateurSallePrivee`.
/// 4. Sinon, GATING par présence effective : une ligne `session_participant`
///    avec `role_session='moderateur'` → `ModerateurAttitre` si l'utilisateur est
///    aussi un attitré actif de la salle, sinon `Demarreur` (placeholder).
/// 5. Sinon → `None`.
///
/// Différence clé avec l'ancienne version (`est_moderateur_session`) : un
/// modérateur attitré n'est reconnu comme modérateur de SESSION qu'une fois
/// ACTIVÉ (`role_session='moderateur'`). Tant qu'une demande de passation est en
/// attente, il reste `participant` et cette fonction renvoie `None` pour lui
/// (c'est le gating du consentement). Aucun cache : recalcul à chaque appel.
pub async fn est_moderateur_actif(
    pool: &PgPool,
    session_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<Option<crate::models::afrolang::NiveauModerateur>, ApiErreur> {
    use crate::models::afrolang::NiveauModerateur;

    let ctx: Option<(Option<Uuid>, Option<Uuid>)> = sqlx::query_as(
        "SELECT salle_id, salle_privee_id FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;
    let Some((salle_id, salle_privee_id)) = ctx else {
        return Ok(None);
    };

    // 1/2/3 — rôles « office » (indépendants de la présence / role_session)
    if let Some(n) = niveau_office_ctx(pool, salle_id, salle_privee_id, utilisateur_id).await? {
        return Ok(Some(n));
    }

    // 4 — gating par role_session effectif
    let est_mod_role: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.session_participant
            WHERE session_id = $1 AND utilisateur_id = $2 AND role_session = 'moderateur'
        )",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    if est_mod_role {
        if let Some(sid) = salle_id {
            if est_attitre_actif(pool, sid, utilisateur_id).await? {
                return Ok(Some(NiveauModerateur::ModerateurAttitre));
            }
        }
        return Ok(Some(NiveauModerateur::Demarreur));
    }

    Ok(None)
}

/// Niveaux « office » (admin plateforme / admin de salle / créateur de salle
/// privée) — modérateurs « quoi qu'il arrive », jamais soumis à la passation.
async fn niveau_office_ctx(
    pool: &PgPool,
    salle_id: Option<Uuid>,
    salle_privee_id: Option<Uuid>,
    utilisateur_id: Uuid,
) -> Result<Option<crate::models::afrolang::NiveauModerateur>, ApiErreur> {
    use crate::models::afrolang::NiveauModerateur;

    if verifier_admin(pool, utilisateur_id).await? {
        return Ok(Some(NiveauModerateur::AdminPlateforme));
    }
    if let Some(sid) = salle_id {
        let est_admin_salle: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1 FROM afrolang.salle_administrateur
                WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
            )",
        )
        .bind(sid)
        .bind(utilisateur_id)
        .fetch_one(pool)
        .await?;
        if est_admin_salle {
            return Ok(Some(NiveauModerateur::AdminSalle));
        }
    }
    if let Some(sp_id) = salle_privee_id {
        let est_createur: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1 FROM afrolang.salle_privee WHERE id = $1 AND cree_par = $2
            )",
        )
        .bind(sp_id)
        .bind(utilisateur_id)
        .fetch_one(pool)
        .await?;
        if est_createur {
            return Ok(Some(NiveauModerateur::CreateurSallePrivee));
        }
    }
    Ok(None)
}

/// Vrai si `utilisateur_id` est un modérateur attitré ACTIF de la salle publique.
async fn est_attitre_actif(
    pool: &PgPool,
    salle_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<bool, ApiErreur> {
    Ok(sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.salle_moderateur
            WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
        )",
    )
    .bind(salle_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?)
}

// ══════════════════════════════════════════════════════════════════════════
// Passation de modération (placeholder → modérateur attitré)
// ══════════════════════════════════════════════════════════════════════════

/// Effet d'arrivée à exécuter APRÈS commit (LiveKit + diffusion temps réel).
enum EffetArrivee {
    Aucun,
    /// Un modérateur (office ou attitré) vient d'être activé immédiatement.
    ModerateurAjoute(Uuid),
    /// Une demande de passation est ouverte (ou déjà ouverte) — prévenir le placeholder.
    DemandeOuverte,
}

/// Effet de résolution de passation à exécuter APRÈS commit.
enum EffetResolution {
    /// Passation réalisée : placeholder démis, attitrés présents promus.
    Resolue { ancien_placeholder: Uuid, promus: Vec<Uuid> },
    /// Demande annulée (plus aucun attitré présent) : placeholder conservé.
    Annulee,
}

/// Règles d'arrivée d'un participant dans une session PUBLIQUE en cours.
/// À appeler DANS une tx ayant verrouillé la ligne session (FOR UPDATE), après
/// l'INSERT du participant. `moderateur_id_courant`/`demande_at_courant` = état
/// lu sous verrou avant cet appel. Les office-holders coexistent (n'évincent pas
/// le placeholder) ; seul le flux attitré (consentement / délai) le démet.
async fn appliquer_arrivee_moderation_publique_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    session_id: Uuid,
    salle_pub_id: Uuid,
    moderateur_id_courant: Option<Uuid>,
    demande_at_courant: Option<chrono::DateTime<chrono::Utc>>,
    utilisateur_id: Uuid,
) -> Result<EffetArrivee, ApiErreur> {
    // Office ? (admin plateforme OU admin de salle) — modérateur immédiat.
    let est_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.utilisateur_role ur JOIN iam.role r ON ur.role_id=r.id
                       WHERE ur.utilisateur_id=$1 AND r.slug='admin')",
    )
    .bind(utilisateur_id)
    .fetch_one(&mut **tx)
    .await?;
    let est_admin_salle: bool = if est_admin {
        false
    } else {
        sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM afrolang.salle_administrateur
                           WHERE salle_id=$1 AND utilisateur_id=$2 AND actif=TRUE)",
        )
        .bind(salle_pub_id)
        .bind(utilisateur_id)
        .fetch_one(&mut **tx)
        .await?
    };
    if est_admin || est_admin_salle {
        sqlx::query(
            "UPDATE afrolang.session_participant SET role_session='moderateur'
             WHERE session_id=$1 AND utilisateur_id=$2",
        )
        .bind(session_id)
        .bind(utilisateur_id)
        .execute(&mut **tx)
        .await?;
        return Ok(EffetArrivee::ModerateurAjoute(utilisateur_id));
    }

    // Attitré ?
    let est_attitre: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle_moderateur
                       WHERE salle_id=$1 AND utilisateur_id=$2 AND actif=TRUE)",
    )
    .bind(salle_pub_id)
    .bind(utilisateur_id)
    .fetch_one(&mut **tx)
    .await?;
    if est_attitre {
        if moderateur_id_courant.is_some() {
            // Placeholder présent → demande de passation (consentement + délai 60 s).
            if demande_at_courant.is_none() {
                sqlx::query(
                    "UPDATE afrolang.session
                     SET demande_passation_at=NOW(), demande_passation_par=$2, updated_at=NOW()
                     WHERE id=$1 AND demande_passation_at IS NULL",
                )
                .bind(session_id)
                .bind(utilisateur_id)
                .execute(&mut **tx)
                .await?;
            }
            return Ok(EffetArrivee::DemandeOuverte);
        }
        // Pas de placeholder → co-modérateur immédiat.
        sqlx::query(
            "UPDATE afrolang.session_participant SET role_session='moderateur'
             WHERE session_id=$1 AND utilisateur_id=$2",
        )
        .bind(session_id)
        .bind(utilisateur_id)
        .execute(&mut **tx)
        .await?;
        return Ok(EffetArrivee::ModerateurAjoute(utilisateur_id));
    }

    Ok(EffetArrivee::Aucun)
}

/// Résolution (acceptation OU promotion auto) DANS une tx. Verrou FOR UPDATE en
/// tête → anti-race (le perdant relit `moderateur_id=NULL` et ne fait rien).
/// `exige_placeholder=Some(caller)` pour l'acceptation (le caller doit être le
/// placeholder courant) ; `exige_delai=true` pour la promotion auto (≥ 60 s).
async fn resoudre_passation_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    session_id: Uuid,
    salle_pub_id: Uuid,
    exige_placeholder: Option<Uuid>,
    exige_delai: bool,
) -> Result<Option<EffetResolution>, ApiErreur> {
    let etat: Option<(Option<Uuid>, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT moderateur_id, demande_passation_at FROM afrolang.session WHERE id=$1 FOR UPDATE",
    )
    .bind(session_id)
    .fetch_optional(&mut **tx)
    .await?;
    // Il faut un placeholder ET une demande ouverte.
    let Some((Some(placeholder), Some(_))) = etat else {
        return Ok(None);
    };
    if let Some(c) = exige_placeholder {
        if c != placeholder {
            return Ok(None);
        }
    }
    if exige_delai {
        let echue: bool = sqlx::query_scalar(
            "SELECT (demande_passation_at + interval '60 seconds') <= NOW()
             FROM afrolang.session WHERE id=$1",
        )
        .bind(session_id)
        .fetch_one(&mut **tx)
        .await?;
        if !echue {
            return Ok(None);
        }
    }

    // Promouvoir TOUS les attitrés présents (multi-modérateurs).
    let promus: Vec<Uuid> = sqlx::query_scalar(
        "UPDATE afrolang.session_participant sp SET role_session='moderateur'
         WHERE sp.session_id=$1 AND sp.quitte_at IS NULL
           AND EXISTS (SELECT 1 FROM afrolang.salle_moderateur sm
                       WHERE sm.salle_id=$2 AND sm.utilisateur_id=sp.utilisateur_id AND sm.actif=TRUE)
         RETURNING sp.utilisateur_id",
    )
    .bind(session_id)
    .bind(salle_pub_id)
    .fetch_all(&mut **tx)
    .await?;

    if promus.is_empty() {
        // Demandeur(s) parti(s) : on annule la demande, le placeholder reste modérateur.
        sqlx::query(
            "UPDATE afrolang.session
             SET demande_passation_at=NULL, demande_passation_par=NULL, updated_at=NOW()
             WHERE id=$1",
        )
        .bind(session_id)
        .execute(&mut **tx)
        .await?;
        return Ok(Some(EffetResolution::Annulee));
    }

    // Démettre le placeholder, clore la demande, NULLer moderateur_id (plus de placeholder).
    sqlx::query(
        "UPDATE afrolang.session
         SET moderateur_id=NULL, demande_passation_at=NULL, demande_passation_par=NULL, updated_at=NOW()
         WHERE id=$1",
    )
    .bind(session_id)
    .execute(&mut **tx)
    .await?;
    sqlx::query(
        "UPDATE afrolang.session_participant SET role_session='participant'
         WHERE session_id=$1 AND utilisateur_id=$2",
    )
    .bind(session_id)
    .bind(placeholder)
    .execute(&mut **tx)
    .await?;
    // Nettoyer d'éventuelles permissions tableau blanc individuelles devenues redondantes.
    sqlx::query(
        "DELETE FROM afrolang.session_permission_tableau_blanc
         WHERE session_id=$1 AND utilisateur_id = ANY($2)",
    )
    .bind(session_id)
    .bind(&promus)
    .execute(&mut **tx)
    .await?;

    Ok(Some(EffetResolution::Resolue {
        ancien_placeholder: placeholder,
        promus,
    }))
}

/// Diffuse un DataPacket `moderation.<subtype>` à toute la room (best-effort).
async fn diffuser_moderation(
    livekit: &LivekitConfig,
    session_id: Uuid,
    subtype: &str,
    payload: serde_json::Value,
) {
    let env = serde_json::json!({ "type": "moderation", "subtype": subtype, "payload": payload });
    if let Err(e) =
        livekit_moderation::publier_evenement_moderation(livekit, &room_name_session(session_id), &env)
            .await
    {
        log::warn!("diffuser_moderation({}) échec: {}", subtype, e);
    }
}

/// État dérivé d'une demande de passation en attente (None hors demande).
async fn charger_passation_en_attente(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<Option<crate::models::afrolang::PassationEnAttenteResponse>, ApiErreur> {
    use crate::models::afrolang::{ModerateurResponse, PassationEnAttenteResponse};
    let row: Option<(Uuid, Uuid, String, Option<String>, Option<String>, chrono::DateTime<chrono::Utc>)> =
        sqlx::query_as(
            "SELECT s.moderateur_id, s.demande_passation_par,
                    u.nom, u.prenom, u.photo_url,
                    (s.demande_passation_at + interval '60 seconds') AS expire_at
             FROM afrolang.session s
             JOIN iam.utilisateur u ON u.id = s.demande_passation_par
             WHERE s.id = $1
               AND s.demande_passation_at IS NOT NULL
               AND s.demande_passation_par IS NOT NULL
               AND s.moderateur_id IS NOT NULL",
        )
        .bind(session_id)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(
        |(cible_id, demandeur_id, nom, prenom, photo, expire_at)| PassationEnAttenteResponse {
            demandeur: ModerateurResponse {
                id: demandeur_id,
                nom,
                prenom,
                photo_url: photo,
            },
            cible_id,
            expire_at,
        },
    ))
}

/// Exécute l'effet d'arrivée APRÈS commit (LiveKit + diffusion + notif).
async fn executer_effet_arrivee(
    pool: &PgPool,
    livekit: &LivekitConfig,
    session_id: Uuid,
    effet: EffetArrivee,
) {
    match effet {
        EffetArrivee::Aucun => {}
        EffetArrivee::ModerateurAjoute(uid) => {
            let _ = livekit_moderation::update_participant_can_publish_data(
                livekit,
                &room_name_session(session_id),
                &uid.to_string(),
                true,
            )
            .await;
            diffuser_moderation(
                livekit,
                session_id,
                "moderateur_ajoute",
                serde_json::json!({ "session_id": session_id, "utilisateur_id": uid }),
            )
            .await;
        }
        EffetArrivee::DemandeOuverte => {
            if let Ok(Some(p)) = charger_passation_en_attente(pool, session_id).await {
                diffuser_moderation(
                    livekit,
                    session_id,
                    "passation_demande",
                    serde_json::json!({
                        "session_id": session_id,
                        "demandeur": p.demandeur,
                        "cible_id": p.cible_id,
                        "expire_at": p.expire_at,
                    }),
                )
                .await;
                let lien = format!("/afrolang/session/{}", session_id);
                notification::creer_notification(
                    pool,
                    p.cible_id,
                    notification::afrolang::MODERATION_DEMANDE_PASSATION,
                    "Un modérateur désigné vient d'entrer. Acceptez de lui passer la modération.",
                    Some(&lien),
                )
                .await;
            }
        }
    }
}

/// Exécute l'effet de résolution APRÈS commit (LiveKit + diffusion + notif + audit).
async fn executer_effet_resolution(
    pool: &PgPool,
    livekit: &LivekitConfig,
    session_id: Uuid,
    auteur: Option<Uuid>,
    effet: EffetResolution,
) {
    match effet {
        EffetResolution::Annulee => {
            diffuser_moderation(
                livekit,
                session_id,
                "passation_resolue",
                serde_json::json!({ "session_id": session_id, "promus": Vec::<Uuid>::new() }),
            )
            .await;
        }
        EffetResolution::Resolue {
            ancien_placeholder,
            promus,
        } => {
            let room = room_name_session(session_id);
            let _ = livekit_moderation::update_participant_can_publish_data(
                livekit,
                &room,
                &ancien_placeholder.to_string(),
                false,
            )
            .await;
            for u in &promus {
                let _ = livekit_moderation::update_participant_can_publish_data(
                    livekit,
                    &room,
                    &u.to_string(),
                    true,
                )
                .await;
            }
            diffuser_moderation(
                livekit,
                session_id,
                "passation_resolue",
                serde_json::json!({ "session_id": session_id, "promus": promus }),
            )
            .await;
            let lien = format!("/afrolang/session/{}", session_id);
            notification::creer_notification(
                pool,
                ancien_placeholder,
                notification::afrolang::MODERATION_REPRISE,
                "Un modérateur désigné a pris la modération de la session.",
                Some(&lien),
            )
            .await;
            for u in &promus {
                notification::creer_notification(
                    pool,
                    *u,
                    notification::afrolang::MODERATION_REPRISE,
                    "Vous êtes désormais modérateur de cette session.",
                    Some(&lien),
                )
                .await;
            }
            audit::log_action(
                pool,
                auteur,
                "PASSATION_MODERATION",
                "afrolang",
                "session",
                Some(session_id),
                Some(serde_json::json!({ "ancien_moderateur_id": ancien_placeholder })),
                Some(serde_json::json!({ "promus": promus, "auto": auteur.is_none() })),
                None,
                None,
            )
            .await;
        }
    }
}

/// Résolution paresseuse : promeut automatiquement si la demande est échue (≥ 60 s)
/// — modèle « cloturer_si_necessaire », sans cron. Best-effort, appelée en tête
/// des lectures/arrivées/départs pour ne pas dépendre du seul timer client.
async fn resoudre_passation_si_due(
    pool: &PgPool,
    livekit: &LivekitConfig,
    session_id: Uuid,
) -> Result<(), ApiErreur> {
    let due: Option<Option<Uuid>> = sqlx::query_scalar(
        "SELECT salle_id FROM afrolang.session
         WHERE id=$1 AND moderateur_id IS NOT NULL AND demande_passation_at IS NOT NULL
           AND (demande_passation_at + interval '60 seconds') <= NOW()",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;
    let Some(Some(salle_pub_id)) = due else {
        return Ok(());
    };
    let mut tx = pool.begin().await?;
    let effet = resoudre_passation_tx(&mut tx, session_id, salle_pub_id, None, true).await?;
    tx.commit().await?;
    if let Some(e) = effet {
        executer_effet_resolution(pool, livekit, session_id, None, e).await;
    }
    Ok(())
}

// ══════════════════════════════════════════════════════════════════════════
// 1.4 — Handlers annuaire groupes ethniques (feature 005, US1)
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/groupes-ethniques — Annuaire ethnique avec état de salle
pub async fn lister_groupes_ethniques(
    pool: web::Data<PgPool>,
    params: web::Query<GroupeEthniqueFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(24).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = Vec::new();
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref q) = params.q {
        if !q.trim().is_empty() {
            conditions.push(format!(
                "lower(unaccent(ge.nom)) LIKE lower(unaccent(${}))",
                bind_index
            ));
            str_binds.push(format!("%{}%", q.trim()));
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("fp.pays_id = ${}", bind_index));
        uuid_binds.push(pays_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count
    let count_sql = format!(
        "SELECT COUNT(*)
         FROM country_profile.groupe_ethnique ge
         LEFT JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
         {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    let mut str_idx = 0usize;
    let mut uuid_idx = 0usize;
    for pt in &param_types {
        match *pt {
            "str" => {
                count_q = count_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                count_q = count_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Data
    let data_sql = format!(
        "SELECT {}
         FROM country_profile.groupe_ethnique ge
         LEFT JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
         LEFT JOIN shared.pays p ON p.id = fp.pays_id
         LEFT JOIN afrolang.salle s
            ON s.groupe_ethnique_id = ge.id
           AND s.actif = TRUE
           AND s.deleted_at IS NULL
         {}
         ORDER BY ge.nom ASC
         LIMIT ${} OFFSET ${}",
        GROUPE_ETHNIQUE_RESUME_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut data_q = sqlx::query_as::<_, GroupeEthniqueResume>(&data_sql);
    str_idx = 0;
    uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => {
                data_q = data_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                data_q = data_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    data_q = data_q.bind(par_page).bind(offset);

    let rows = data_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(GroupeEthniqueListeResponse {
            groupes: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages: calculer_total_pages(total, par_page),
        }),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// 1.5 — Handlers salles publiques
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/salles — Liste paginee des salles publiques actives
pub async fn lister_salles(
    pool: web::Data<PgPool>,
    params: web::Query<SalleFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec!["s.actif = true".to_string(), "s.deleted_at IS NULL".to_string()];
    let mut bind_index = 1u32;
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();

    if let Some(ref langue) = params.langue {
        if !langue.trim().is_empty() {
            conditions.push(format!("LOWER(s.langue_cible) = LOWER(${})", bind_index));
            str_binds.push(langue.trim().to_string());
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref code) = params.langue_code {
        if !code.trim().is_empty() {
            conditions.push(format!("LOWER(s.langue_code) = LOWER(${})", bind_index));
            str_binds.push(code.trim().to_string());
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(groupe_id) = params.groupe_ethnique_id {
        conditions.push(format!("s.groupe_ethnique_id = ${}", bind_index));
        uuid_binds.push(groupe_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    // Filtre pays d'origine (feature 001-afrolang-pays-origine, US2 — Q3 archivés masqués)
    if let Some(pays_id) = params.pays_id {
        conditions.push(format!(
            "EXISTS (SELECT 1 FROM afrolang.salle_pays_origine spo \
              JOIN shared.pays p2 ON p2.id = spo.pays_id \
              WHERE spo.salle_id = s.id AND spo.pays_id = ${} AND p2.actif = TRUE)",
            bind_index
        ));
        uuid_binds.push(pays_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            let terme = format!("%{}%", recherche.trim().to_lowercase());
            conditions.push(format!(
                "(LOWER(s.titre) LIKE ${idx} OR LOWER(s.description) LIKE ${idx})",
                idx = bind_index
            ));
            str_binds.push(terme);
            param_types.push("str");
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!("SELECT COUNT(*) FROM afrolang.salle s WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    let mut str_idx = 0usize;
    let mut uuid_idx = 0usize;
    for pt in &param_types {
        match *pt {
            "str" => {
                count_q = count_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                count_q = count_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les salles enrichies (groupe ethnique, comptages)
    let select_query = format!(
        "SELECT {},
            ge.nom AS groupe_ethnique_nom,
            ge.fiche_pays_id AS fiche_pays_id,
            p.nom AS pays_nom,
            (SELECT COUNT(*) FROM afrolang.salle_privee sp2
             WHERE sp2.salle_id = s.id AND sp2.actif = true
               AND sp2.archivee_at IS NULL AND sp2.deleted_at IS NULL)
                AS nombre_salles_privees,
            (SELECT COUNT(*) FROM afrolang.session ses2
             JOIN afrolang.salle_privee sp3 ON sp3.id = ses2.salle_privee_id
             WHERE sp3.salle_id = s.id AND ses2.etat = 'en_cours') AS sessions_en_cours,
            (SELECT COUNT(*) FROM afrolang.salle_moderateur sm
             WHERE sm.salle_id = s.id AND sm.actif = TRUE) AS nombre_moderateurs_attitres,
            (SELECT COUNT(*) FROM afrolang.ressource_salle rs
             WHERE rs.salle_id = s.id AND rs.etat = 'publiee' AND rs.deleted_at IS NULL)
                AS ressources_count,
            COALESCE((SELECT json_agg(json_build_object(
                        'id', po.id,
                        'nom', po.nom,
                        'code_iso2', po.code_iso2
                     ) ORDER BY po.nom)
                     FROM afrolang.salle_pays_origine spo
                     JOIN shared.pays po ON po.id = spo.pays_id
                     WHERE spo.salle_id = s.id AND po.actif = TRUE),
                     '[]'::json) AS pays_origine_json,
            COALESCE((SELECT json_agg(json_build_object(
                        'utilisateur_id', sa.utilisateur_id,
                        'nom', ua.nom,
                        'prenom', ua.prenom,
                        'photo_url', ua.photo_url,
                        'nomme_at', sa.nomme_at
                     ) ORDER BY sa.nomme_at ASC)
                     FROM afrolang.salle_administrateur sa
                     JOIN iam.utilisateur ua ON ua.id = sa.utilisateur_id
                     WHERE sa.salle_id = s.id AND sa.actif = TRUE),
                     '[]'::json) AS administrateurs_json
         FROM afrolang.salle s
         LEFT JOIN country_profile.groupe_ethnique ge ON ge.id = s.groupe_ethnique_id
         LEFT JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
         LEFT JOIN shared.pays p ON p.id = fp.pays_id
         WHERE {}
         ORDER BY s.created_at DESC
         LIMIT ${} OFFSET ${}",
        SALLE_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, SalleRow>(&select_query);
    str_idx = 0;
    uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => {
                select_q = select_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                select_q = select_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SalleListeResponse {
            salles: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages: calculer_total_pages(total, par_page),
        }),
        error: None,
    }))
}

/// GET /api/afrolang/salles/{id} — Detail d'une salle publique (feature 005)
pub async fn obtenir_salle(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    use crate::models::afrolang::SalleModerateurRow;

    let id = chemin.into_inner();

    let query = format!(
        "SELECT {},
            ge.nom AS groupe_ethnique_nom,
            ge.fiche_pays_id AS fiche_pays_id,
            p.nom AS pays_nom,
            (SELECT COUNT(*) FROM afrolang.salle_privee sp2
             WHERE sp2.salle_id = s.id AND sp2.actif = true
               AND sp2.archivee_at IS NULL AND sp2.deleted_at IS NULL)
                AS nombre_salles_privees,
            (SELECT COUNT(*) FROM afrolang.session ses2
             JOIN afrolang.salle_privee sp3 ON sp3.id = ses2.salle_privee_id
             WHERE sp3.salle_id = s.id AND ses2.etat = 'en_cours') AS sessions_en_cours,
            (SELECT COUNT(*) FROM afrolang.salle_moderateur sm
             WHERE sm.salle_id = s.id AND sm.actif = TRUE) AS nombre_moderateurs_attitres,
            (SELECT COUNT(*) FROM afrolang.ressource_salle rs
             WHERE rs.salle_id = s.id AND rs.etat = 'publiee' AND rs.deleted_at IS NULL)
                AS ressources_count,
            COALESCE((SELECT json_agg(json_build_object(
                        'id', po.id,
                        'nom', po.nom,
                        'code_iso2', po.code_iso2
                     ) ORDER BY po.nom)
                     FROM afrolang.salle_pays_origine spo
                     JOIN shared.pays po ON po.id = spo.pays_id
                     WHERE spo.salle_id = s.id AND po.actif = TRUE),
                     '[]'::json) AS pays_origine_json,
            COALESCE((SELECT json_agg(json_build_object(
                        'utilisateur_id', sa.utilisateur_id,
                        'nom', ua.nom,
                        'prenom', ua.prenom,
                        'photo_url', ua.photo_url,
                        'nomme_at', sa.nomme_at
                     ) ORDER BY sa.nomme_at ASC)
                     FROM afrolang.salle_administrateur sa
                     JOIN iam.utilisateur ua ON ua.id = sa.utilisateur_id
                     WHERE sa.salle_id = s.id AND sa.actif = TRUE),
                     '[]'::json) AS administrateurs_json
         FROM afrolang.salle s
         LEFT JOIN country_profile.groupe_ethnique ge ON ge.id = s.groupe_ethnique_id
         LEFT JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
         LEFT JOIN shared.pays p ON p.id = fp.pays_id
         WHERE s.id = $1 AND s.deleted_at IS NULL",
        SALLE_COLONNES
    );

    let salle = sqlx::query_as::<_, SalleRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle {} non trouvee", id)))?;

    // Modérateurs attitrés actifs
    let mod_query = format!(
        "SELECT {},
            u.nom AS utilisateur_nom,
            u.prenom AS utilisateur_prenom,
            u.photo_url AS utilisateur_photo,
            u.email AS utilisateur_email
         FROM afrolang.salle_moderateur sm
         LEFT JOIN iam.utilisateur u ON u.id = sm.utilisateur_id
         WHERE sm.salle_id = $1 AND sm.actif = TRUE
         ORDER BY sm.designe_at ASC",
        crate::models::afrolang::SALLE_MODERATEUR_COLONNES
    );

    let moderateurs_attitres = sqlx::query_as::<_, SalleModerateurRow>(&mod_query)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await?;

    // Charger les salles privees associees (actives et non archivées)
    let sp_query = format!(
        "SELECT {},
            u.nom AS createur_nom, u.prenom AS createur_prenom,
            u.photo_url AS createur_photo,
            s2.titre AS salle_titre, s2.langue_cible AS salle_langue,
            EXISTS(SELECT 1 FROM afrolang.session ses
                   WHERE ses.salle_privee_id = sp.id AND ses.etat = 'en_cours') AS session_en_cours
         FROM afrolang.salle_privee sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.cree_par
         LEFT JOIN afrolang.salle s2 ON s2.id = sp.salle_id
         WHERE sp.salle_id = $1 AND sp.actif = true
           AND sp.archivee_at IS NULL AND sp.deleted_at IS NULL
         ORDER BY sp.created_at DESC",
        SALLE_PRIVEE_COLONNES
    );

    let salles_privees = sqlx::query_as::<_, SallePriveeRow>(&sp_query)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SalleDetailResponse {
            id: salle.id,
            titre: salle.titre.clone(),
            slug: salle.slug.clone(),
            description: salle.description.clone(),
            image_couverture_url: salle.image_couverture_url.clone(),
            langue_cible: salle.langue_cible.clone(),
            langue_code: salle.langue_code.clone(),
            alphabet: salle.alphabet.clone(),
            dictionnaire_url: salle.dictionnaire_url.clone(),
            groupe_ethnique_id: salle.groupe_ethnique_id,
            groupe_ethnique_libre: salle.groupe_ethnique_libre.clone(),
            groupe_ethnique: salle.to_groupe_ethnique_light(),
            actif: salle.actif,
            moderateurs_attitres: moderateurs_attitres
                .iter()
                .map(|m| m.to_response())
                .collect(),
            nombre_salles_privees: salle.nombre_salles_privees.unwrap_or(0),
            sessions_en_cours: salle.sessions_en_cours.unwrap_or(0),
            ressources_count: salle.ressources_count.unwrap_or(0),
            salles_privees: salles_privees.iter().map(|sp| sp.to_response()).collect(),
            pays_origine: salle.to_pays_origine(),
            administrateurs: salle.to_administrateurs(),
            // Vue publique : motif masqué (FR-020). Les endpoints admin
            // (cf. `admin/sessions_moderation.rs`) utilisent
            // `to_desactivation_admin()` pour exposer le motif détaillé.
            desactivee_admin: salle.to_desactivation_public(),
            created_at: salle.created_at,
            updated_at: salle.updated_at,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles — Creation multipart (image + metadonnees) [Admin]
pub async fn creer_salle(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    if !verifier_admin(pool.get_ref(), utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise("Droits administrateur requis".into()));
    }

    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut langue_cible: Option<String> = None;
    let mut langue_code: Option<String> = None;
    let mut alphabet: Option<String> = None;
    let mut dictionnaire_url: Option<String> = None;
    let mut groupe_ethnique_id: Option<Uuid> = None;
    let mut image_couverture_url: Option<String> = None;

    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| ApiErreur::Upload(format!("Erreur multipart: {}", e)))?;
        let nom_champ = field
            .content_disposition()
            .and_then(|cd| cd.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        match nom_champ.as_str() {
            "titre" => titre = Some(lire_champ_texte(&mut field).await?),
            "description" => description = Some(lire_champ_texte(&mut field).await?),
            "langue_cible" => langue_cible = Some(lire_champ_texte(&mut field).await?),
            "langue_code" => langue_code = Some(lire_champ_texte(&mut field).await?),
            "alphabet" => alphabet = Some(lire_champ_texte(&mut field).await?),
            "dictionnaire_url" => dictionnaire_url = Some(lire_champ_texte(&mut field).await?),
            "groupe_ethnique_id" => {
                let val = lire_champ_texte(&mut field).await?;
                groupe_ethnique_id = Uuid::parse_str(val.trim()).ok();
            }
            "couverture" | "image" => {
                let nom_original = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));
                let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
                let chemin_complet = format!("{}/couvertures/{}", upload_dir, nom_fichier);
                sauvegarder_fichier(&mut field, &chemin_complet).await?;
                image_couverture_url = Some(format!("/uploads/couvertures/{}", nom_fichier));
            }
            _ => {
                while let Some(Ok(_)) = field.next().await {}
            }
        }
    }

    let titre = titre
        .filter(|t| !t.trim().is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le titre est obligatoire".into()))?;

    let groupe_ethnique_id = groupe_ethnique_id.ok_or_else(|| {
        ApiErreur::Validation("Le groupe ethnique de rattachement est obligatoire".into())
    })?;

    let slug = generer_slug(&titre);

    let row = sqlx::query_as::<_, SalleRow>(
        &format!(
            "INSERT INTO afrolang.salle
                (titre, slug, description, image_couverture_url,
                 langue_cible, langue_code, alphabet, dictionnaire_url,
                 groupe_ethnique_id, cree_par)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             RETURNING {}",
            SALLE_COLONNES.replace("s.", "")
        ),
    )
    .bind(titre.trim())
    .bind(&slug)
    .bind(description.as_deref().map(str::trim))
    .bind(&image_couverture_url)
    .bind(langue_cible.as_deref().map(str::trim))
    .bind(langue_code.as_deref().map(str::trim))
    .bind(alphabet.as_deref().map(str::trim))
    .bind(dictionnaire_url.as_deref().map(str::trim))
    .bind(groupe_ethnique_id)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    log::info!("Salle afrolang creee: {} ({})", row.titre, row.id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// PUT /api/afrolang/salles/{id} — Modifier une salle [Admin]
pub async fn modifier_salle(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierSalleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    if !verifier_admin(pool.get_ref(), utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise("Droits administrateur requis".into()));
    }

    let id = chemin.into_inner();

    // Construire la requete UPDATE dynamiquement
    let mut sets: Vec<String> = vec!["updated_at = NOW()".to_string()];
    let mut bind_index = 2u32; // $1 = id
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref titre) = body.titre {
        if !titre.trim().is_empty() {
            sets.push(format!("titre = ${}", bind_index));
            bind_values.push(titre.trim().to_string());
            bind_index += 1;
            let slug = generer_slug(titre);
            sets.push(format!("slug = ${}", bind_index));
            bind_values.push(slug);
            bind_index += 1;
        }
    }
    if let Some(ref desc) = body.description {
        sets.push(format!("description = ${}", bind_index));
        bind_values.push(desc.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref langue) = body.langue_cible {
        sets.push(format!("langue_cible = ${}", bind_index));
        bind_values.push(langue.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref code) = body.langue_code {
        sets.push(format!("langue_code = ${}", bind_index));
        bind_values.push(code.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref alpha) = body.alphabet {
        sets.push(format!("alphabet = ${}", bind_index));
        bind_values.push(alpha.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref dict) = body.dictionnaire_url {
        sets.push(format!("dictionnaire_url = ${}", bind_index));
        bind_values.push(dict.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref groupe_id) = body.groupe_ethnique_id {
        sets.push(format!("groupe_ethnique_id = ${}::UUID", bind_index));
        bind_values.push(groupe_id.to_string());
        bind_index += 1;
    }

    let _ = bind_index; // supprimer le warning unused

    let query = format!(
        "UPDATE afrolang.salle SET {} WHERE id = $1 RETURNING {}",
        sets.join(", "),
        SALLE_COLONNES.replace("s.", "")
    );

    let mut q = sqlx::query_as::<_, SalleRow>(&query).bind(id);
    for val in &bind_values {
        q = q.bind(val);
    }

    let row = q
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle {} non trouvee", id)))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// DELETE /api/afrolang/salles/{id} — Soft delete [Admin]
pub async fn supprimer_salle(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    if !verifier_admin(pool.get_ref(), utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise("Droits administrateur requis".into()));
    }

    let id = chemin.into_inner();

    let result = sqlx::query(
        "UPDATE afrolang.salle SET actif = false, updated_at = NOW() WHERE id = $1 AND actif = true",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(format!("Salle {} non trouvee", id)));
    }

    log::info!("Salle afrolang desactivee: {}", id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// 1.6 — Handlers salles privees
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/salles/{salle_id}/salles-privees — Salles privées listées
/// dans le widget d'une salle publique (contrat endpoint 2, refonte 2026-04).
///
/// Toute salle privée non archivée et non supprimée est retournée : la
/// protection repose uniquement sur le code secret vérifié côté serveur à
/// l'endpoint `verifier-code`. L'auteur courant est signalé via `est_auteur`
/// pour permettre au frontend de court-circuiter la modale (FR-014).
pub async fn lister_salles_privees_par_salle_publique(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    // Accès public : un visiteur non authentifié doit pouvoir voir la liste.
    // L'authentification sera exigée au moment d'intégrer une salle (via
    // `verifier-code` / `demarrer-ou-rejoindre`). Sans utilisateur courant,
    // `est_auteur` vaudra toujours `false` (comparaison à `Uuid::nil()`).
    let utilisateur_id = extraire_utilisateur_id(&req).unwrap_or_else(Uuid::nil);

    let salle_id = chemin.into_inner();

    let select_query = format!(
        "SELECT {},
            u.nom AS createur_nom, u.prenom AS createur_prenom,
            u.photo_url AS createur_photo,
            s.titre AS salle_titre, s.langue_cible AS salle_langue,
            EXISTS(SELECT 1 FROM afrolang.session ses
                   WHERE ses.salle_privee_id = sp.id AND ses.etat = 'en_cours') AS session_en_cours
         FROM afrolang.salle_privee sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.cree_par
         LEFT JOIN afrolang.salle s ON s.id = sp.salle_id
         WHERE sp.salle_id = $1
           AND sp.actif = TRUE
           AND sp.archivee_at IS NULL
           AND sp.deleted_at IS NULL
         ORDER BY sp.created_at DESC",
        SALLE_PRIVEE_COLONNES
    );

    let rows = sqlx::query_as::<_, SallePriveeRow>(&select_query)
        .bind(salle_id)
        .fetch_all(pool.get_ref())
        .await?;

    let salles: Vec<SallePriveeAPI> =
        rows.iter().map(|r| r.to_api(utilisateur_id)).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(salles),
        error: None,
    }))
}

/// GET /api/afrolang/salles-privees/{id} — Detail d'une salle privee avec sessions
pub async fn obtenir_salle_privee(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {},
            u.nom AS createur_nom, u.prenom AS createur_prenom,
            u.photo_url AS createur_photo,
            s.titre AS salle_titre, s.langue_cible AS salle_langue,
            EXISTS(SELECT 1 FROM afrolang.session ses
                   WHERE ses.salle_privee_id = sp.id AND ses.etat = 'en_cours') AS session_en_cours
         FROM afrolang.salle_privee sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.cree_par
         LEFT JOIN afrolang.salle s ON s.id = sp.salle_id
         WHERE sp.id = $1",
        SALLE_PRIVEE_COLONNES
    );

    let salle_privee = sqlx::query_as::<_, SallePriveeRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle privee {} non trouvee", id)))?;

    // Charger les sessions associees
    let ses_query = format!(
        "SELECT {}
         FROM afrolang.session ses
         WHERE ses.salle_privee_id = $1
         ORDER BY ses.date_debut_prevue DESC NULLS LAST, ses.created_at DESC",
        SESSION_COLONNES
    );

    let sessions = sqlx::query_as::<_, SessionRow>(&ses_query)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await?;

    let resp = salle_privee.to_response();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SallePriveeDetailResponse {
            id: resp.id,
            salle_id: resp.salle_id,
            titre: resp.titre,
            description: resp.description,
            image_couverture_url: resp.image_couverture_url,
            max_participants: resp.max_participants,
            archivee_at: resp.archivee_at,
            actif: resp.actif,
            createur: resp.createur,
            salle_titre: resp.salle_titre,
            salle_langue: resp.salle_langue,
            session_en_cours: resp.session_en_cours,
            sessions: sessions.iter().map(|s| s.to_response()).collect(),
            created_at: resp.created_at,
            updated_at: resp.updated_at,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles-privees — Création d'une salle privée par
/// l'utilisateur courant (refonte 2026-04, endpoint 1 du contrat).
///
/// Valide titre, description, code d'accès, vérifie que la salle publique
/// cible existe et est active, puis hashe le code avant l'INSERT. Retourne
/// 409 si l'utilisateur possède déjà une salle privée active pour la même
/// salle publique (FR-010).
pub async fn creer_salle_privee_publique(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreerSallePriveePubliquePayload>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let titre = body.titre.trim().to_string();
    let titre_len = titre.chars().count();
    if !(5..=350).contains(&titre_len) {
        return Err(ApiErreur::Validation(
            "Le titre doit contenir entre 5 et 350 caractères".into(),
        ));
    }

    let description = body
        .description
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    if let Some(ref d) = description {
        if d.chars().count() > 1000 {
            return Err(ApiErreur::Validation(
                "La description ne peut dépasser 1000 caractères".into(),
            ));
        }
    }

    valider_format_code_acces(body.code_acces.as_str())?;

    // Vérifier l'existence et l'activité de la salle publique.
    let salle_info: Option<(bool, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT actif, deleted_at FROM afrolang.salle WHERE id = $1",
    )
    .bind(body.salle_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (salle_active, salle_deleted) = salle_info
        .ok_or_else(|| ApiErreur::Validation("Salle publique inexistante".into()))?;
    if salle_deleted.is_some() {
        return Err(ApiErreur::Validation("Salle publique supprimée".into()));
    }
    if !salle_active {
        // 422 selon le contrat — nous utilisons Validation (400) par défaut,
        // le contrat distingue « inactive » (422) de « inexistante » (400) :
        // on exprime cela via le message sans créer de variant supplémentaire.
        return Err(ApiErreur::Validation(
            "Salle publique inactive — création impossible".into(),
        ));
    }

    // Vérifier l'unicité (salle_id, utilisateur) active avant l'INSERT pour
    // fournir un 409 porteur d'information (salle_privee_existante_id).
    let existante_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM afrolang.salle_privee
         WHERE salle_id = $1 AND cree_par = $2
           AND archivee_at IS NULL AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(body.salle_id)
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    if let Some(existante) = existante_id {
        return Ok(HttpResponse::Conflict().json(ApiResponse {
            success: false,
            data: Some(serde_json::json!({
                "salle_privee_existante_id": existante,
            })),
            error: Some(
                "Vous avez déjà une salle privée pour cette salle publique".into(),
            ),
        }));
    }

    let code_hash = hasher_code_acces(body.code_acces.as_str())?;

    let insert_result = sqlx::query_as::<_, SallePriveeRow>(
        &format!(
            "INSERT INTO afrolang.salle_privee
                (salle_id, titre, description, code_acces_hash, cree_par)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING {}",
            SALLE_PRIVEE_COLONNES.replace("sp.", "")
        ),
    )
    .bind(body.salle_id)
    .bind(&titre)
    .bind(description.as_deref())
    .bind(&code_hash)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await;

    let mut row = match insert_result {
        Ok(r) => r,
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            // Course critique : une salle a été créée entre notre vérification
            // et notre INSERT. Retourner le 409 de la même manière.
            let existante_id: Option<Uuid> = sqlx::query_scalar(
                "SELECT id FROM afrolang.salle_privee
                 WHERE salle_id = $1 AND cree_par = $2
                   AND archivee_at IS NULL AND deleted_at IS NULL
                 LIMIT 1",
            )
            .bind(body.salle_id)
            .bind(utilisateur_id)
            .fetch_optional(pool.get_ref())
            .await?;
            return Ok(HttpResponse::Conflict().json(ApiResponse {
                success: false,
                data: existante_id.map(|id| serde_json::json!({
                    "salle_privee_existante_id": id,
                })),
                error: Some(
                    "Vous avez déjà une salle privée pour cette salle publique".into(),
                ),
            }));
        }
        Err(e) => return Err(ApiErreur::from(e)),
    };

    // Hydrater les JOINs manquants avec une requête légère (auteur).
    let auteur: Option<(Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT nom, prenom, photo_url FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;
    if let Some((nom, prenom, photo)) = auteur {
        row.createur_nom = nom;
        row.createur_prenom = prenom;
        row.createur_photo = photo;
    }
    row.session_en_cours = Some(false);

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "creer_salle_privee",
        "afrolang",
        "salle_privee",
        Some(row.id),
        None,
        Some(serde_json::json!({
            "salle_id": body.salle_id,
            "titre": titre,
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    log::info!(
        "Salle privée créée : {} ({}) pour utilisateur {}",
        row.titre, row.id, utilisateur_id
    );

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_api(utilisateur_id)),
        error: None,
    }))
}

/// PUT /api/afrolang/salles-privees/{id} — Modifier sa salle privee [JWT createur]
pub async fn modifier_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierSallePriveeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Verifier que l'utilisateur est le createur
    let createur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1 AND actif = true",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    match createur_id {
        None => return Err(ApiErreur::NonTrouve(format!("Salle privee {} non trouvee", id))),
        Some(cid) if cid != utilisateur_id => {
            return Err(ApiErreur::NonAutorise("Seul le createur peut modifier cette salle".into()));
        }
        _ => {}
    }

    let mut sets: Vec<String> = vec!["updated_at = NOW()".to_string()];
    let mut bind_index = 2u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref titre) = body.titre {
        if !titre.trim().is_empty() {
            sets.push(format!("titre = ${}", bind_index));
            bind_values.push(titre.trim().to_string());
            bind_index += 1;
        }
    }
    if let Some(ref desc) = body.description {
        sets.push(format!("description = ${}", bind_index));
        bind_values.push(desc.trim().to_string());
        bind_index += 1;
    }
    if let Some(max) = body.max_participants {
        sets.push(format!("max_participants = ${}", bind_index));
        bind_values.push(max.to_string());
        bind_index += 1;
    }

    let _ = bind_index;

    let query = format!(
        "UPDATE afrolang.salle_privee SET {} WHERE id = $1 RETURNING {}",
        sets.join(", "),
        SALLE_PRIVEE_COLONNES.replace("sp.", "")
    );

    let mut q = sqlx::query_as::<_, SallePriveeRow>(&query).bind(id);
    for val in &bind_values {
        q = q.bind(val);
    }

    let row = q
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle privee {} non trouvee", id)))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// DELETE /api/afrolang/salles-privees/{id} — Soft delete [JWT createur]
pub async fn supprimer_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    let createur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1 AND actif = true",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    match createur_id {
        None => return Err(ApiErreur::NonTrouve(format!("Salle privee {} non trouvee", id))),
        Some(cid) if cid != utilisateur_id => {
            return Err(ApiErreur::NonAutorise("Seul le createur peut supprimer cette salle".into()));
        }
        _ => {}
    }

    sqlx::query("UPDATE afrolang.salle_privee SET actif = false, updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    log::info!("Salle privee desactivee: {}", id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// 1.7 — Handlers sessions
// ══════════════════════════════════════════════════════════════════════════

/// Vérifie que l'utilisateur peut démarrer/terminer une session.
/// Règle :
///   - Tout modérateur effectif de la session (office, attitré activé, placeholder).
///   - Salle privée : créateur de la salle privée.
///   - Salle publique : créateur de la session OU modérateur attitré actif.
async fn peut_gerer_cycle_session(
    pool: &PgPool,
    session: &SessionRow,
    utilisateur_id: Uuid,
) -> Result<bool, ApiErreur> {
    // Tout modérateur effectif (multi-modérateurs) peut gérer le cycle.
    if est_moderateur_actif(pool, session.id, utilisateur_id)
        .await?
        .is_some()
    {
        return Ok(true);
    }
    if let Some(sp_id) = session.salle_privee_id {
        let createur: Uuid = sqlx::query_scalar(
            "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1",
        )
        .bind(sp_id)
        .fetch_one(pool)
        .await?;
        return Ok(createur == utilisateur_id);
    }
    if let Some(salle_id) = session.salle_id {
        if session.cree_par == utilisateur_id {
            return Ok(true);
        }
        let attitre: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1 FROM afrolang.salle_moderateur
                WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
            )",
        )
        .bind(salle_id)
        .bind(utilisateur_id)
        .fetch_one(pool)
        .await?;
        return Ok(attitre);
    }
    Ok(false)
}

/// POST /api/afrolang/salles/{salle_id}/sessions — Créer une session dans une salle publique [JWT]
///
/// Règle d'autorisation : tout utilisateur authentifié peut créer une session
/// (la salle publique n'appartient à personne). Le créateur devient modérateur
/// par défaut ; si un modérateur attitré rejoint plus tard, il prend la main
/// (FR-011 gérée par `rejoindre_session`).
pub async fn creer_session_salle_publique(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CreerSessionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_id = chemin.into_inner();

    let salle_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND actif = TRUE AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_active {
        return Err(ApiErreur::NonTrouve("Salle publique introuvable".into()));
    }

    let date_debut_prevue = body
        .date_debut_prevue
        .as_ref()
        .map(|s| {
            chrono::DateTime::parse_from_rfc3339(s)
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
                        .map(|dt| dt.and_utc().fixed_offset())
                })
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|_| ApiErreur::Validation("Format de date invalide".into()))
        })
        .transpose()?;

    let row = sqlx::query_as::<_, SessionRow>(
        &format!(
            "INSERT INTO afrolang.session
                (salle_id, titre, moderateur_id, date_debut_prevue,
                 max_participants, tableau_blanc_actif, cree_par)
             VALUES ($1, $2, $3, $4, $5, $6, $3)
             RETURNING {}",
            SESSION_COLONNES.replace("ses.", "")
        ),
    )
    .bind(salle_id)
    .bind(body.titre.as_deref().map(str::trim))
    .bind(utilisateur_id)
    .bind(date_debut_prevue)
    .bind(body.max_participants.unwrap_or(50))
    .bind(body.tableau_blanc_actif.unwrap_or(true))
    .fetch_one(pool.get_ref())
    .await?;

    log::info!(
        "Session salle publique planifiée: {:?} ({})",
        row.titre, row.id
    );

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// GET /api/afrolang/salles/{salle_id}/sessions — Sessions d'une salle publique
pub async fn lister_sessions_salle_publique(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    params: web::Query<SessionFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let salle_id = chemin.into_inner();
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec!["ses.salle_id = $1".to_string()];
    let mut bind_index = 2u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref etat) = params.etat {
        if !etat.trim().is_empty() {
            conditions.push(format!("ses.etat::TEXT = ${}", bind_index));
            bind_values.push(etat.trim().to_string());
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    let count_query = format!(
        "SELECT COUNT(*) FROM afrolang.session ses WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query).bind(salle_id);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_query = format!(
        "SELECT {}
         FROM afrolang.session ses
         WHERE {}
         ORDER BY ses.date_debut_prevue DESC NULLS LAST, ses.created_at DESC
         LIMIT ${} OFFSET ${}",
        SESSION_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, SessionRow>(&select_query).bind(salle_id);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SessionListeResponse {
            sessions: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages: calculer_total_pages(total, par_page),
        }),
        error: None,
    }))
}

/// GET /api/afrolang/salles-privees/{sp_id}/sessions — Sessions d'une salle privee
pub async fn lister_sessions(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    params: web::Query<SessionFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let sp_id = chemin.into_inner();
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec!["ses.salle_privee_id = $1".to_string()];
    let mut bind_index = 2u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref etat) = params.etat {
        if !etat.trim().is_empty() {
            conditions.push(format!("ses.etat::TEXT = ${}", bind_index));
            bind_values.push(etat.trim().to_string());
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    let count_query = format!(
        "SELECT COUNT(*) FROM afrolang.session ses WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query).bind(sp_id);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_query = format!(
        "SELECT {}
         FROM afrolang.session ses
         WHERE {}
         ORDER BY ses.date_debut_prevue DESC NULLS LAST, ses.created_at DESC
         LIMIT ${} OFFSET ${}",
        SESSION_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, SessionRow>(&select_query).bind(sp_id);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SessionListeResponse {
            sessions: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages: calculer_total_pages(total, par_page),
        }),
        error: None,
    }))
}

/// GET /api/afrolang/sessions/{id} — Detail d'une session avec participants
pub async fn obtenir_session(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    // Résolution paresseuse d'une demande de passation échue (≥ 60 s) avant lecture.
    let _ = resoudre_passation_si_due(pool.get_ref(), livekit_config.get_ref(), id).await;

    let query = format!(
        "SELECT {}
         FROM afrolang.session ses
         WHERE ses.id = $1",
        SESSION_COLONNES
    );

    let session = sqlx::query_as::<_, SessionRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", id)))?;

    // Charger les participants avec info utilisateur
    let participants = sqlx::query_as::<_, SessionParticipantRow>(
        "SELECT sp.id, sp.session_id, sp.utilisateur_id, sp.role_session,
                sp.rejoint_at, sp.quitte_at, sp.duree_secondes,
                u.nom, u.prenom, u.photo_url
         FROM afrolang.session_participant sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.utilisateur_id
         WHERE sp.session_id = $1
         ORDER BY sp.rejoint_at ASC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    // Charger le moderateur
    let moderateur = if let Some(mod_id) = session.moderateur_id {
        sqlx::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
            "SELECT id, nom, prenom, photo_url FROM iam.utilisateur WHERE id = $1",
        )
        .bind(mod_id)
        .fetch_optional(pool.get_ref())
        .await?
        .map(|(id, nom, prenom, photo_url)| ModerateurResponse {
            id,
            nom,
            prenom,
            photo_url,
        })
    } else {
        None
    };

    // Feature 001-session-moderation (FR-024) : état spotlight + nb permissions TB
    let spotlight = charger_spotlight(pool.get_ref(), id).await?;
    let permissions_tableau_blanc_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session_permission_tableau_blanc WHERE session_id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    // Demande de passation en attente (filet de lecture si le DataPacket a été perdu).
    let passation_en_attente = charger_passation_en_attente(pool.get_ref(), id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SessionDetailResponse {
            id: session.id,
            salle_privee_id: session.salle_privee_id,
            salle_id: session.salle_id,
            titre: session.titre.clone(),
            etat: session.etat.clone(),
            moderateur,
            date_debut_prevue: session.date_debut_prevue,
            demarre_at: session.demarre_at,
            termine_at: session.termine_at,
            duree_secondes: session.duree_secondes,
            max_participants: session.max_participants,
            nombre_participants_pic: session.nombre_participants_pic,
            tableau_blanc_actif: session.tableau_blanc_actif,
            participants: participants.iter().map(|p| p.to_response()).collect(),
            created_at: session.created_at,
            updated_at: session.updated_at,
            spotlight,
            permissions_tableau_blanc_count,
            passation_en_attente,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles-privees/{sp_id}/sessions — Planifier une session [JWT moderateur]
pub async fn creer_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CreerSessionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let sp_id = chemin.into_inner();

    // Verifier que l'utilisateur est le createur (moderateur) de la salle privee
    let createur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1 AND actif = true",
    )
    .bind(sp_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match createur_id {
        None => return Err(ApiErreur::NonTrouve("Salle privee non trouvee".into())),
        Some(cid) if cid != utilisateur_id => {
            return Err(ApiErreur::NonAutorise(
                "Seul le moderateur (createur) peut planifier une session".into(),
            ));
        }
        _ => {}
    }

    // Parser la date prevue si fournie
    let date_debut_prevue = body
        .date_debut_prevue
        .as_ref()
        .map(|s| {
            chrono::DateTime::parse_from_rfc3339(s)
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
                        .map(|dt| dt.and_utc().fixed_offset())
                })
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|_| ApiErreur::Validation("Format de date invalide".into()))
        })
        .transpose()?;

    let row = sqlx::query_as::<_, SessionRow>(
        &format!(
            "INSERT INTO afrolang.session
                (salle_privee_id, titre, moderateur_id, date_debut_prevue,
                 max_participants, tableau_blanc_actif, cree_par)
             VALUES ($1, $2, $3, $4, $5, $6, $3)
             RETURNING {}",
            SESSION_COLONNES.replace("ses.", "")
        ),
    )
    .bind(sp_id)
    .bind(body.titre.as_deref().map(str::trim))
    .bind(utilisateur_id)
    .bind(date_debut_prevue)
    .bind(body.max_participants.unwrap_or(50))
    .bind(body.tableau_blanc_actif.unwrap_or(true))
    .fetch_one(pool.get_ref())
    .await?;

    log::info!("Session planifiee: {:?} ({})", row.titre, row.id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// PUT /api/afrolang/sessions/{id}/demarrer — Demarrer une session [JWT moderateur]
pub async fn demarrer_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Charger la session et verifier l'etat
    let session = sqlx::query_as::<_, SessionRow>(
        &format!(
            "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
            SESSION_COLONNES
        ),
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", id)))?;

    if !peut_gerer_cycle_session(pool.get_ref(), &session, utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise(
            "Seul le moderateur peut demarrer la session".into(),
        ));
    }

    if session.etat != "planifiee" {
        return Err(ApiErreur::Validation(format!(
            "La session ne peut etre demarree (etat actuel: {})",
            session.etat
        )));
    }

    // Mettre a jour l'etat
    let row = sqlx::query_as::<_, SessionRow>(
        &format!(
            "UPDATE afrolang.session
             SET etat = 'en_cours', demarre_at = NOW(), updated_at = NOW()
             WHERE id = $1
             RETURNING {}",
            SESSION_COLONNES.replace("ses.", "")
        ),
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    // Ajouter le moderateur comme participant
    sqlx::query(
        "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
         VALUES ($1, $2, 'moderateur')
         ON CONFLICT (session_id, utilisateur_id) DO NOTHING",
    )
    .bind(id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    // Mettre a jour le pic de participants
    sqlx::query(
        "UPDATE afrolang.session SET nombre_participants_pic = GREATEST(nombre_participants_pic,
            (SELECT COUNT(*) FROM afrolang.session_participant
             WHERE session_id = $1 AND quitte_at IS NULL))
         WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Session demarree: {}", id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// PUT /api/afrolang/sessions/{id}/terminer — Terminer une session [JWT moderateur]
pub async fn terminer_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Charger la session
    let session = sqlx::query_as::<_, SessionRow>(
        &format!(
            "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
            SESSION_COLONNES
        ),
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", id)))?;

    if !peut_gerer_cycle_session(pool.get_ref(), &session, utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise(
            "Seul le moderateur peut terminer la session".into(),
        ));
    }

    if session.etat != "en_cours" {
        return Err(ApiErreur::Validation(format!(
            "La session ne peut etre terminee (etat actuel: {})",
            session.etat
        )));
    }

    // Terminer la session et calculer la duree
    // FR-017 (feature 001-session-moderation) : nettoyer les permissions tableau
    // blanc et l'éventuel spotlight à la clôture, en même transaction.
    let mut tx = pool.begin().await?;

    let row = sqlx::query_as::<_, SessionRow>(
        &format!(
            "UPDATE afrolang.session
             SET etat = 'terminee', termine_at = NOW(),
                 duree_secondes = EXTRACT(EPOCH FROM (NOW() - demarre_at))::INT,
                 participant_mis_en_evidence_id = NULL,
                 mis_en_evidence_par = NULL,
                 mis_en_evidence_at = NULL,
                 updated_at = NOW()
             WHERE id = $1
             RETURNING {}",
            SESSION_COLONNES.replace("ses.", "")
        ),
    )
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "DELETE FROM afrolang.session_permission_tableau_blanc WHERE session_id = $1",
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;

    // Mettre a jour tous les participants encore actifs
    sqlx::query(
        "UPDATE afrolang.session_participant
         SET quitte_at = NOW(),
             duree_secondes = EXTRACT(EPOCH FROM (NOW() - rejoint_at))::INT
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    log::info!("Session terminee: {}", id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/afrolang/sessions/{id}/rejoindre — Rejoindre une session [JWT]
pub async fn rejoindre_session(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<RejoindreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Charger la session
    let session = sqlx::query_as::<_, SessionRow>(
        &format!(
            "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
            SESSION_COLONNES
        ),
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", id)))?;

    // Verifier que la session est en cours
    if session.etat != "en_cours" {
        return Err(ApiErreur::Validation(
            "La session n'est pas en cours".into(),
        ));
    }

    // Note (refonte 2026-04) : la vérification du code secret d'une salle
    // privée se fait désormais à l'endpoint dédié `verifier-code` + jeton
    // d'accès porté par `demarrer-ou-rejoindre`. Cet endpoint n'applique
    // plus de contrôle de code ici.
    let _ = &body;

    // Verifier max_participants
    let nb_actifs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session_participant
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    if let Some(max) = session.max_participants {
        if nb_actifs >= max as i64 {
            return Err(ApiErreur::Validation(
                "Nombre maximum de participants atteint".into(),
            ));
        }
    }

    // ── Règles de modération multi-modérateurs (refonte 2026-06) ──
    // Le passage d'un modérateur attitré n'est PLUS une reprise automatique
    // immédiate (ancienne FR-011) : il déclenche une demande de passation
    // (consentement du placeholder OU promotion auto après 60 s). La logique
    // est partagée avec `demarrer-ou-rejoindre` via le helper transactionnel.
    if let Some(salle_pub_id) = session.salle_id {
        let mut tx = pool.begin().await?;

        let etat: (Option<Uuid>, Option<chrono::DateTime<chrono::Utc>>) = sqlx::query_as(
            "SELECT moderateur_id, demande_passation_at FROM afrolang.session WHERE id = $1 FOR UPDATE",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;

        let mut effet_resolution: Option<EffetResolution> = None;
        let (mod_id, dem_at) = if etat.1.is_some() {
            effet_resolution = resoudre_passation_tx(&mut tx, id, salle_pub_id, None, true).await?;
            if effet_resolution.is_some() {
                sqlx::query_as::<_, (Option<Uuid>, Option<chrono::DateTime<chrono::Utc>>)>(
                    "SELECT moderateur_id, demande_passation_at FROM afrolang.session WHERE id=$1",
                )
                .bind(id)
                .fetch_one(&mut *tx)
                .await?
            } else {
                etat
            }
        } else {
            etat
        };

        sqlx::query(
            "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
             VALUES ($1, $2, 'participant')
             ON CONFLICT (session_id, utilisateur_id)
             DO UPDATE SET quitte_at = NULL, rejoint_at = NOW()",
        )
        .bind(id)
        .bind(utilisateur_id)
        .execute(&mut *tx)
        .await?;

        let effet_arrivee = appliquer_arrivee_moderation_publique_tx(
            &mut tx, id, salle_pub_id, mod_id, dem_at, utilisateur_id,
        )
        .await?;

        tx.commit().await?;

        if let Some(e) = effet_resolution {
            executer_effet_resolution(pool.get_ref(), livekit_config.get_ref(), id, None, e).await;
        }
        executer_effet_arrivee(pool.get_ref(), livekit_config.get_ref(), id, effet_arrivee).await;
    } else {
        // Session privée : pas de passation (le créateur est le seul modérateur d'office).
        sqlx::query(
            "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
             VALUES ($1, $2, 'participant')
             ON CONFLICT (session_id, utilisateur_id)
             DO UPDATE SET quitte_at = NULL, rejoint_at = NOW()",
        )
        .bind(id)
        .bind(utilisateur_id)
        .execute(pool.get_ref())
        .await?;
    }

    // Mettre à jour le pic de participants
    sqlx::query(
        "UPDATE afrolang.session SET nombre_participants_pic = GREATEST(nombre_participants_pic,
            (SELECT COUNT(*) FROM afrolang.session_participant
             WHERE session_id = $1 AND quitte_at IS NULL)),
            updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Utilisateur {} a rejoint la session {}", utilisateur_id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// POST /api/afrolang/sessions/{id}/quitter — Quitter une session [JWT]
pub async fn quitter_session(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Charger la session AVANT la mise à jour pour pouvoir détecter si le partant
    // est le modérateur actif (règle FR-012)
    let session_opt = sqlx::query_as::<_, SessionRow>(&format!(
        "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
        SESSION_COLONNES
    ))
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let Some(session) = session_opt else {
        return Err(ApiErreur::NonTrouve(format!("Session {} non trouvée", id)));
    };

    let result = sqlx::query(
        "UPDATE afrolang.session_participant
         SET quitte_at = NOW(),
             duree_secondes = EXTRACT(EPOCH FROM (NOW() - rejoint_at))::INT
         WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL",
    )
    .bind(id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Participant non trouvé dans cette session".into(),
        ));
    }

    log::info!("Utilisateur {} a quitté la session {}", utilisateur_id, id);

    // Feature 001-session-moderation (FR-025) : si le partant était mis en évidence,
    // remettre les 3 colonnes spotlight à NULL et publier un DataPacket
    // `moderation.spotlight: null` pour notifier les clients en temps réel.
    let cascade_spotlight = sqlx::query(
        "UPDATE afrolang.session
         SET participant_mis_en_evidence_id = NULL,
             mis_en_evidence_par = NULL,
             mis_en_evidence_at = NULL,
             updated_at = NOW()
         WHERE id = $1 AND participant_mis_en_evidence_id = $2",
    )
    .bind(id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if cascade_spotlight.rows_affected() > 0 {
        let ip = audit::extraire_ip(&req);
        let ua = audit::extraire_user_agent(&req);
        audit::log_action(
            pool.get_ref(),
            Some(utilisateur_id),
            "UPDATE",
            "afrolang",
            "session",
            Some(id),
            Some(serde_json::json!({ "spotlight_id": utilisateur_id })),
            Some(serde_json::json!({ "spotlight_id": serde_json::Value::Null })),
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;

        let payload = serde_json::json!({
            "type": "moderation",
            "subtype": "spotlight",
            "payload": serde_json::Value::Null,
        });
        // Erreur LiveKit non bloquante : on log mais on n'interrompt pas la sortie de session
        if let Err(e) = livekit_moderation::publier_evenement_moderation(
            livekit_config.get_ref(),
            &room_name_session(id),
            &payload,
        )
        .await
        {
            log::warn!(
                "Échec publication DataPacket spotlight=null (cascade quitter): {}",
                e
            );
        }
    }

    // Vérifier s'il reste des participants actifs
    let participants_actifs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session_participant
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    let session_terminee = if participants_actifs == 0 {
        // Terminer la session si elle est en cours
        let rows = sqlx::query(
            "UPDATE afrolang.session
             SET etat = 'terminee', termine_at = NOW(),
                 duree_secondes = EXTRACT(EPOCH FROM (NOW() - demarre_at))::INT,
                 updated_at = NOW()
             WHERE id = $1 AND etat = 'en_cours'",
        )
        .bind(id)
        .execute(pool.get_ref())
        .await?;

        if rows.rows_affected() > 0 {
            log::info!("Session {} terminée automatiquement (dernier participant parti)", id);
            true
        } else {
            false
        }
    } else if session.salle_id.is_some()
        && session.moderateur_id == Some(utilisateur_id)
        && session.etat == "en_cours"
    {
        // Le placeholder quitte (session publique). Refonte multi-modérateurs :
        // si des attitrés sont présents → on les active (co-modérateurs) et
        // moderateur_id passe à NULL ; sinon nouveau placeholder = plus ancien
        // participant NON-office présent. On ne pointe JAMAIS moderateur_id vers
        // un attitré/office (invariant placeholder non-office).
        let salle_pub_id = session.salle_id.unwrap();

        let mut tx = pool.begin().await?;
        let mod_courant: Option<Uuid> = sqlx::query_scalar(
            "SELECT moderateur_id FROM afrolang.session WHERE id = $1 FOR UPDATE",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;

        let mut promus: Vec<Uuid> = Vec::new();
        let mut nouveau_placeholder: Option<Uuid> = None;

        if mod_courant == Some(utilisateur_id) {
            promus = sqlx::query_scalar(
                "UPDATE afrolang.session_participant sp SET role_session='moderateur'
                 WHERE sp.session_id=$1 AND sp.quitte_at IS NULL
                   AND EXISTS (SELECT 1 FROM afrolang.salle_moderateur sm
                               WHERE sm.salle_id=$2 AND sm.utilisateur_id=sp.utilisateur_id AND sm.actif=TRUE)
                 RETURNING sp.utilisateur_id",
            )
            .bind(id)
            .bind(salle_pub_id)
            .fetch_all(&mut *tx)
            .await?;

            if !promus.is_empty() {
                sqlx::query(
                    "UPDATE afrolang.session SET moderateur_id=NULL, demande_passation_at=NULL,
                            demande_passation_par=NULL, updated_at=NOW() WHERE id=$1",
                )
                .bind(id)
                .execute(&mut *tx)
                .await?;
            } else {
                let succ: Option<Uuid> = sqlx::query_scalar(
                    "SELECT sp.utilisateur_id FROM afrolang.session_participant sp
                     WHERE sp.session_id=$1 AND sp.quitte_at IS NULL
                       AND NOT EXISTS (SELECT 1 FROM iam.utilisateur_role ur JOIN iam.role r ON ur.role_id=r.id
                                       WHERE ur.utilisateur_id=sp.utilisateur_id AND r.slug='admin')
                       AND NOT EXISTS (SELECT 1 FROM afrolang.salle_administrateur sa
                                       WHERE sa.salle_id=$2 AND sa.utilisateur_id=sp.utilisateur_id AND sa.actif=TRUE)
                       AND NOT EXISTS (SELECT 1 FROM afrolang.salle_moderateur sm
                                       WHERE sm.salle_id=$2 AND sm.utilisateur_id=sp.utilisateur_id AND sm.actif=TRUE)
                     ORDER BY sp.rejoint_at ASC LIMIT 1",
                )
                .bind(id)
                .bind(salle_pub_id)
                .fetch_optional(&mut *tx)
                .await?;
                if let Some(n) = succ {
                    sqlx::query(
                        "UPDATE afrolang.session SET moderateur_id=$2, demande_passation_at=NULL,
                                demande_passation_par=NULL, updated_at=NOW() WHERE id=$1",
                    )
                    .bind(id)
                    .bind(n)
                    .execute(&mut *tx)
                    .await?;
                    sqlx::query(
                        "UPDATE afrolang.session_participant SET role_session='moderateur'
                         WHERE session_id=$1 AND utilisateur_id=$2",
                    )
                    .bind(id)
                    .bind(n)
                    .execute(&mut *tx)
                    .await?;
                    nouveau_placeholder = Some(n);
                } else {
                    // Aucun successeur non-office (il reste éventuellement des office présents).
                    sqlx::query(
                        "UPDATE afrolang.session SET moderateur_id=NULL, demande_passation_at=NULL,
                                demande_passation_par=NULL, updated_at=NOW() WHERE id=$1",
                    )
                    .bind(id)
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }

        tx.commit().await?;

        // Effets temps réel post-commit.
        let lk = livekit_config.get_ref();
        let lien = format!("/afrolang/session/{}", id);
        if !promus.is_empty() {
            let room = room_name_session(id);
            for u in &promus {
                let _ = livekit_moderation::update_participant_can_publish_data(
                    lk, &room, &u.to_string(), true,
                )
                .await;
            }
            diffuser_moderation(
                lk, id, "passation_resolue",
                serde_json::json!({ "session_id": id, "promus": promus }),
            )
            .await;
            for u in &promus {
                notification::creer_notification(
                    pool.get_ref(), *u, notification::afrolang::MODERATION_REPRISE,
                    "Vous êtes désormais modérateur de cette session.", Some(&lien),
                )
                .await;
            }
        } else if let Some(n) = nouveau_placeholder {
            let _ = livekit_moderation::update_participant_can_publish_data(
                lk, &room_name_session(id), &n.to_string(), true,
            )
            .await;
            diffuser_moderation(
                lk, id, "moderateur_ajoute",
                serde_json::json!({ "session_id": id, "utilisateur_id": n }),
            )
            .await;
            notification::creer_notification(
                pool.get_ref(), n, notification::afrolang::MODERATION_REPRISE,
                "Vous êtes désormais modérateur de cette session.", Some(&lien),
            )
            .await;
        } else if mod_courant == Some(utilisateur_id) {
            // Placeholder parti sans successeur : éteindre une éventuelle demande côté clients.
            diffuser_moderation(
                lk, id, "passation_resolue",
                serde_json::json!({ "session_id": id, "promus": Vec::<Uuid>::new() }),
            )
            .await;
        }

        false
    } else {
        false
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "session_terminee": session_terminee
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Phase 3 — Token LiveKit pour visioconference
// ══════════════════════════════════════════════════════════════════════════

/// POST /api/afrolang/sessions/{id}/token — Generer un token LiveKit pour rejoindre la visio
pub async fn generer_token_session(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<RejoindreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let session_id = chemin.into_inner();

    // 1. Charger la session
    let session = sqlx::query_as::<_, SessionRow>(
        &format!(
            "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
            SESSION_COLONNES
        ),
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", session_id)))?;

    // 2. Verifier que la session est en_cours
    if session.etat != "en_cours" {
        return Err(ApiErreur::Validation("La session n'est pas en cours".into()));
    }

    // 3. Note (refonte 2026-04) : le contrôle du code secret passe par
    //    l'endpoint dédié `verifier-code` + jeton d'accès présenté à
    //    `demarrer-ou-rejoindre`. Plus de vérification ici.
    let _ = &body;

    // 4. Verifier max_participants
    let nb_actifs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session_participant
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(session_id)
    .fetch_one(pool.get_ref())
    .await?;

    let max = session.max_participants.unwrap_or(50);
    if nb_actifs >= max as i64 {
        return Err(ApiErreur::Validation("Session complete".into()));
    }

    // 5. Charger les infos utilisateur
    let (user_nom, user_prenom): (String, Option<String>) = sqlx::query_as(
        "SELECT nom, prenom FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Résolution paresseuse d'une demande de passation échue avant de calculer le rôle.
    let _ = resoudre_passation_si_due(pool.get_ref(), livekit_config.get_ref(), session_id).await;

    let room_name = format!("afrolang-{}", session_id);
    let display_name = format!(
        "{} {}",
        user_prenom.as_deref().unwrap_or(""),
        user_nom
    ).trim().to_string();

    // 5b. Refonte multi-modérateurs — droit d'écriture tableau blanc + statut modérateur :
    //     is_moderator dérive du SET de modérateurs (est_moderateur_actif), PAS de
    //     session.moderateur_id (qui ne désigne que le placeholder, NULL en multi-mod).
    //     (1) modérateur de session → autorisé ;
    //     (2) sinon, permission individuelle déjà accordée → autorisé ;
    //     (3) sinon → refusé (le SFU LiveKit rejettera les DataPacket).
    let niveau_moderateur = est_moderateur_actif(pool.get_ref(), session_id, utilisateur_id).await?;
    let is_moderator = niveau_moderateur.is_some();
    let can_publish_data = if is_moderator {
        true
    } else {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(
                SELECT 1 FROM afrolang.session_permission_tableau_blanc
                WHERE session_id = $1 AND utilisateur_id = $2
            )",
        )
        .bind(session_id)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?
    };

    // 6. Generer le token LiveKit
    let token = livekit_api::access_token::AccessToken::with_api_key(
        &livekit_config.api_key,
        &livekit_config.api_secret,
    )
    .with_identity(&utilisateur_id.to_string())
    .with_name(&display_name)
    .with_grants(livekit_api::access_token::VideoGrants {
        room_join: true,
        room: room_name.clone(),
        can_publish: true,
        can_subscribe: true,
        can_publish_data,
        ..Default::default()
    })
    .to_jwt()
    .map_err(|e| ApiErreur::Validation(format!("Erreur generation token LiveKit: {}", e)))?;

    // 7. Enregistrer le participant (ON CONFLICT pour gerer les re-connexions)
    let role = if is_moderator { "moderateur" } else { "participant" };
    sqlx::query(
        "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
         VALUES ($1, $2, $3)
         ON CONFLICT (session_id, utilisateur_id)
         DO UPDATE SET quitte_at = NULL, rejoint_at = NOW()",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .bind(role)
    .execute(pool.get_ref())
    .await?;

    // Mettre a jour le pic de participants
    sqlx::query(
        "UPDATE afrolang.session SET nombre_participants_pic = GREATEST(nombre_participants_pic,
            (SELECT COUNT(*) FROM afrolang.session_participant
             WHERE session_id = $1 AND quitte_at IS NULL)),
            updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Token LiveKit genere pour utilisateur {} session {}", utilisateur_id, session_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "token": token,
            "room_name": room_name,
            "livekit_url": livekit_config.url,
            "is_moderator": is_moderator,
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Phase 4 — Tableau blanc collaboratif
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/sessions/{id}/tableau-blanc — Obtenir le snapshot du tableau blanc
pub async fn obtenir_tableau_blanc(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let session_id = chemin.into_inner();

    // Verifier que la session existe
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.session WHERE id = $1)",
    )
    .bind(session_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve(format!("Session {} non trouvee", session_id)));
    }

    let row = sqlx::query_as::<_, (serde_json::Value, i32)>(
        "SELECT donnees, version FROM afrolang.tableau_blanc WHERE session_id = $1",
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match row {
        Some((donnees, version)) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({ "donnees": donnees, "version": version })),
            error: None,
        })),
        None => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({ "donnees": {}, "version": 0 })),
            error: None,
        })),
    }
}

/// PUT /api/afrolang/sessions/{id}/tableau-blanc — Sauvegarder le snapshot
pub async fn sauvegarder_tableau_blanc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, ApiErreur> {
    let session_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Token invalide ou manquant".into()))?;

    // Vérifier l'existence de la session (404 sinon) ; autorisation faite ci-dessous.
    let _session = sqlx::query_as::<_, SessionRow>(&format!(
        "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
        SESSION_COLONNES
    ))
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", session_id)))?;

    if est_moderateur_actif(pool.get_ref(), session_id, utilisateur_id)
        .await?
        .is_none()
    {
        return Err(ApiErreur::NonAutorise(
            "Seul un modérateur peut sauvegarder le tableau blanc".into(),
        ));
    }

    // UPSERT dans afrolang.tableau_blanc
    sqlx::query(
        "INSERT INTO afrolang.tableau_blanc (session_id, donnees, version)
         VALUES ($1, $2, 1)
         ON CONFLICT (session_id)
         DO UPDATE SET donnees = $2, version = afrolang.tableau_blanc.version + 1, updated_at = NOW()",
    )
    .bind(session_id)
    .bind(&body.0)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some("ok"),
        error: None,
    }))
}

/// DELETE /api/afrolang/sessions/{id}/tableau-blanc — Effacer le tableau blanc
pub async fn effacer_tableau_blanc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let session_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Token invalide ou manquant".into()))?;

    // Vérifier l'existence de la session (404 sinon) ; autorisation faite ci-dessous.
    let _session = sqlx::query_as::<_, SessionRow>(&format!(
        "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
        SESSION_COLONNES
    ))
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", session_id)))?;

    if est_moderateur_actif(pool.get_ref(), session_id, utilisateur_id)
        .await?
        .is_none()
    {
        return Err(ApiErreur::NonAutorise(
            "Seul un modérateur peut effacer le tableau blanc".into(),
        ));
    }

    sqlx::query(
        "UPDATE afrolang.tableau_blanc SET donnees = '{}', version = version + 1, updated_at = NOW() WHERE session_id = $1",
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some("ok"),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// 1.8 — Handlers utilitaires
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/stats — Statistiques globales Afrolang
pub async fn obtenir_stats(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let total_salles: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.salle WHERE actif = true",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let total_salles_privees: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.salle_privee WHERE actif = true",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let sessions_en_cours: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session WHERE etat = 'en_cours'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let sessions_terminees: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session WHERE etat = 'terminee'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let total_participants_uniques: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT utilisateur_id) FROM afrolang.session_participant",
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AfrolangStatsResponse {
            total_salles,
            total_salles_privees,
            sessions_en_cours,
            sessions_terminees,
            total_participants_uniques,
        }),
        error: None,
    }))
}

/// GET /api/afrolang/langues — Liste des langues disponibles
pub async fn lister_langues(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let langues: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT langue_cible FROM afrolang.salle
         WHERE actif = true AND langue_cible IS NOT NULL
         ORDER BY langue_cible ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(langues),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Legacy supprimé par la refonte 2026-04 (feature 001-afrolang-salles-refonte)
// ══════════════════════════════════════════════════════════════════════════
// Les handlers `creer_proposition`, `lister_mes_propositions`,
// `changer_visibilite_salle_privee`, `charger_salle_privee_active`,
// `demander_adhesion`, `inviter_membre`, `decision_adhesion`,
// `lister_adhesions_salle_privee`, `retirer_abonne` ont été retirés.
// La création de salles publiques est désormais réservée aux admins et le
// contrôle d'accès aux salles privées repose uniquement sur le code secret
// (voir endpoints `verifier-code`, `sessions/demarrer-ou-rejoindre`,
//  `code-acces`, `archiver` ajoutés en fin de fichier).

// ══════════════════════════════════════════════════════════════════════════
// Feature 005 — Transfert de modération de session (US3)
// ══════════════════════════════════════════════════════════════════════════

/// PUT /api/afrolang/sessions/{id}/moderation/transferer — Promotion d'un
/// participant en CO-modérateur [JWT modérateur].
///
/// Refonte multi-modérateurs : un transfert n'évince plus l'appelant. N'importe
/// quel modérateur effectif (office, attitré activé ou placeholder) peut promouvoir
/// un participant présent au rang de co-modérateur. `moderateur_id` (placeholder)
/// reste inchangé.
pub async fn transferer_moderation_session(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<TransfererModerationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let session_id = chemin.into_inner();
    let destinataire_id = body.destinataire_id;

    if destinataire_id == utilisateur_id {
        return Err(ApiErreur::Validation(
            "Le destinataire doit être différent de l'appelant".into(),
        ));
    }

    // Charger la session
    let session = sqlx::query_as::<_, SessionRow>(&format!(
        "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
        SESSION_COLONNES
    ))
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvée", session_id)))?;

    if session.etat != "en_cours" {
        return Err(ApiErreur::Validation("La session n'est pas en cours".into()));
    }

    // L'appelant doit être un modérateur effectif de la session.
    if est_moderateur_actif(pool.get_ref(), session_id, utilisateur_id)
        .await?
        .is_none()
    {
        return Err(ApiErreur::NonAutorise(
            "Seul un modérateur peut promouvoir un participant".into(),
        ));
    }

    // Vérifier que le destinataire est participant actif
    let destinataire_actif: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.session_participant
            WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL
        )",
    )
    .bind(session_id)
    .bind(destinataire_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !destinataire_actif {
        return Err(ApiErreur::Validation(
            "Le destinataire n'est pas un participant actif de la session".into(),
        ));
    }

    // Promotion en co-modérateur (placeholder inchangé).
    sqlx::query(
        "UPDATE afrolang.session_participant SET role_session = 'moderateur'
         WHERE session_id = $1 AND utilisateur_id = $2",
    )
    .bind(session_id)
    .bind(destinataire_id)
    .execute(pool.get_ref())
    .await?;

    // LiveKit : autoriser l'écriture data + diffuser l'ajout.
    let _ = livekit_moderation::update_participant_can_publish_data(
        livekit_config.get_ref(),
        &room_name_session(session_id),
        &destinataire_id.to_string(),
        true,
    )
    .await;
    diffuser_moderation(
        livekit_config.get_ref(),
        session_id,
        "moderateur_ajoute",
        serde_json::json!({ "session_id": session_id, "utilisateur_id": destinataire_id }),
    )
    .await;

    let lien = format!("/afrolang/session/{}", session_id);
    notification::creer_notification(
        pool.get_ref(),
        destinataire_id,
        notification::afrolang::MODERATION_REPRISE,
        "Vous êtes désormais modérateur de cette session.",
        Some(&lien),
    )
    .await;

    log::info!(
        "Session {} : {} a promu {} en co-modérateur",
        session_id, utilisateur_id, destinataire_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "session_id": session_id,
            "moderateur_id": destinataire_id,
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Passation de modération — endpoints (refonte multi-modérateurs, 2026-06)
// ══════════════════════════════════════════════════════════════════════════

/// POST /api/afrolang/sessions/{id}/passation/accepter — Le placeholder accepte
/// de céder la modération au(x) modérateur(s) attitré(s) présent(s) [JWT placeholder].
pub async fn accepter_passation(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let session_id = chemin.into_inner();

    let salle_pub_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT salle_id FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvée", session_id)))?;
    let Some(salle_pub_id) = salle_pub_id else {
        return Err(ApiErreur::Validation(
            "Passation indisponible pour cette session".into(),
        ));
    };

    // Garde : seul le placeholder courant (exige_placeholder=Some(caller)).
    let mut tx = pool.begin().await?;
    let effet =
        resoudre_passation_tx(&mut tx, session_id, salle_pub_id, Some(utilisateur_id), false).await?;
    tx.commit().await?;

    match effet {
        Some(e) => {
            executer_effet_resolution(
                pool.get_ref(),
                livekit_config.get_ref(),
                session_id,
                Some(utilisateur_id),
                e,
            )
            .await;
            Ok(HttpResponse::Ok().json(ApiResponse::<()> {
                success: true,
                data: None,
                error: None,
            }))
        }
        None => Err(ApiErreur::Validation(
            "Aucune passation à accepter (vous n'êtes pas le démarreur, ou la demande est déjà résolue).".into(),
        )),
    }
}

/// POST /api/afrolang/sessions/{id}/passation/finaliser — Promotion automatique
/// après le délai (≥ 60 s) : un modérateur attitré présent réclame la modération
/// si le placeholder n'a pas répondu [JWT attitré présent]. Idempotent.
pub async fn finaliser_passation(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let session_id = chemin.into_inner();

    let salle_pub_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT salle_id FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvée", session_id)))?;
    let Some(salle_pub_id) = salle_pub_id else {
        return Err(ApiErreur::Validation(
            "Passation indisponible pour cette session".into(),
        ));
    };

    // L'appelant doit être un modérateur désigné présent.
    let appelant_attitre_present: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.session_participant sp
            JOIN afrolang.salle_moderateur sm
              ON sm.utilisateur_id=sp.utilisateur_id AND sm.salle_id=$2 AND sm.actif=TRUE
            WHERE sp.session_id=$1 AND sp.utilisateur_id=$3 AND sp.quitte_at IS NULL
        )",
    )
    .bind(session_id)
    .bind(salle_pub_id)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !appelant_attitre_present {
        return Err(ApiErreur::AccesInterdit(
            "Seul un modérateur désigné présent peut finaliser la passation.".into(),
        ));
    }

    // Garde : exige_delai=true → ne promeut que si la demande a ≥ 60 s.
    let mut tx = pool.begin().await?;
    let effet = resoudre_passation_tx(&mut tx, session_id, salle_pub_id, None, true).await?;
    tx.commit().await?;

    let resolu = effet.is_some();
    if let Some(e) = effet {
        executer_effet_resolution(
            pool.get_ref(),
            livekit_config.get_ref(),
            session_id,
            Some(utilisateur_id),
            e,
        )
        .await;
    }
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "resolu": resolu })),
        error: None,
    }))
}

/// PATCH /api/afrolang/salles-privees/{id}/max-participants — Modifier la
/// limite de participants d'une salle privée (auteur uniquement).
pub async fn modifier_max_participants_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierMaxParticipantsRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();
    let nouvelle = body.max_participants;
    if nouvelle < 1 {
        return Err(ApiErreur::Validation(
            "La limite de participants doit être supérieure ou égale à 1".into(),
        ));
    }

    let salle: Option<(Uuid, Option<i32>, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT cree_par, max_participants, archivee_at
         FROM afrolang.salle_privee
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (createur, ancienne, archivee_at) = salle
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle privée {} non trouvée", id)))?;

    if createur != utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Seul le créateur peut modifier la limite".into(),
        ));
    }
    if archivee_at.is_some() {
        return Err(ApiErreur::Validation("La salle privée est archivée".into()));
    }

    sqlx::query(
        "UPDATE afrolang.salle_privee
         SET max_participants = $2, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(nouvelle)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "modifier_max_participants_salle_privee",
        "afrolang",
        "salle_privee",
        Some(id),
        Some(serde_json::json!({ "max_participants": ancienne })),
        Some(serde_json::json!({ "max_participants": nouvelle })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "max_participants": nouvelle })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Feature 005 — US6 : Messagerie de session
// ══════════════════════════════════════════════════════════════════════════

/// Vérifie que l'utilisateur est participant actif d'une session
async fn verifier_participant_actif(
    pool: &PgPool,
    session_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<(), ApiErreur> {
    let actif: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.session_participant
            WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL
        )",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    if !actif {
        return Err(ApiErreur::NonAutorise(
            "Vous devez être participant actif de la session".into(),
        ));
    }
    Ok(())
}

/// GET /api/afrolang/sessions/{id}/messages — Historique [JWT participant]
pub async fn lister_messages_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    params: web::Query<MessagesFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let session_id = chemin.into_inner();
    verifier_participant_actif(pool.get_ref(), session_id, utilisateur_id).await?;

    let limit = params.limit.unwrap_or(100).clamp(1, 500);

    let sql = format!(
        "SELECT {},
            u.nom AS auteur_nom,
            u.prenom AS auteur_prenom,
            u.photo_url AS auteur_photo
         FROM afrolang.message_session ms
         LEFT JOIN iam.utilisateur u ON u.id = ms.auteur_id
         WHERE ms.session_id = $1 AND ms.deleted_at IS NULL
           AND ($2::timestamptz IS NULL OR ms.created_at > $2)
         ORDER BY ms.created_at ASC
         LIMIT $3",
        MESSAGE_SESSION_COLONNES
    );

    let rows = sqlx::query_as::<_, MessageSessionRow>(&sql)
        .bind(session_id)
        .bind(params.since)
        .bind(limit)
        .fetch_all(pool.get_ref())
        .await?;

    let items: Vec<MessageSessionResponse> = rows.iter().map(|r| r.to_response()).collect();
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
}

/// POST /api/afrolang/sessions/{id}/messages — Envoyer un message [JWT participant]
pub async fn envoyer_message_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CreerMessageRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let session_id = chemin.into_inner();

    let contenu = body.contenu.trim();
    if contenu.is_empty() || contenu.chars().count() > 4000 {
        return Err(ApiErreur::Validation(
            "Le contenu doit faire entre 1 et 4000 caractères".into(),
        ));
    }

    verifier_participant_actif(pool.get_ref(), session_id, utilisateur_id).await?;

    let message_id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.message_session (session_id, auteur_id, contenu)
         VALUES ($1, $2, $3)
         RETURNING id",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .bind(contenu)
    .fetch_one(pool.get_ref())
    .await?;

    let sql = format!(
        "SELECT {},
            u.nom AS auteur_nom,
            u.prenom AS auteur_prenom,
            u.photo_url AS auteur_photo
         FROM afrolang.message_session ms
         LEFT JOIN iam.utilisateur u ON u.id = ms.auteur_id
         WHERE ms.id = $1",
        MESSAGE_SESSION_COLONNES
    );
    let row = sqlx::query_as::<_, MessageSessionRow>(&sql)
        .bind(message_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Feature 005 — US6 : Ressources de salle publique
// ══════════════════════════════════════════════════════════════════════════

const RESSOURCES_EXTENSIONS_AUTORISEES: &[&str] = &[
    "pdf", "png", "jpg", "jpeg", "mp3", "mp4", "webm", "ogg", "wav",
];
const RESSOURCE_TAILLE_MAX: usize = 50 * 1024 * 1024;

/// Vérifie si l'utilisateur est modérateur attitré actif d'une salle
async fn est_moderateur_attitre(
    pool: &PgPool,
    salle_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<bool, ApiErreur> {
    let v: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.salle_moderateur
            WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
        )",
    )
    .bind(salle_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(v)
}

/// GET /api/afrolang/salles/{salle_id}/ressources — Liste publique
pub async fn lister_ressources(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let salle_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req);

    let sql = format!(
        "SELECT {},
            u.nom AS auteur_nom,
            u.prenom AS auteur_prenom
         FROM afrolang.ressource_salle rs
         LEFT JOIN iam.utilisateur u ON u.id = rs.ajoute_par
         WHERE rs.salle_id = $1 AND rs.deleted_at IS NULL
           AND (
             rs.etat = 'publiee'
             OR ($2::uuid IS NOT NULL AND rs.ajoute_par = $2 AND rs.etat = 'en_attente_validation')
           )
         ORDER BY rs.created_at DESC",
        RESSOURCE_SALLE_COLONNES
    );

    let rows = sqlx::query_as::<_, RessourceSalleRow>(&sql)
        .bind(salle_id)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?;

    let items: Vec<RessourceSalleResponse> = rows.iter().map(|r| r.to_response()).collect();
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
}

/// POST /api/afrolang/salles/{salle_id}/ressources/fichier — Upload fichier
/// [JWT modérateur attitré ou admin]
pub async fn uploader_ressource_fichier(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_id = chemin.into_inner();

    let salle_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND actif = TRUE AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_active {
        return Err(ApiErreur::NonTrouve("Salle publique introuvable".into()));
    }

    let admin = verifier_admin(pool.get_ref(), utilisateur_id).await?;
    let moderateur = est_moderateur_attitre(pool.get_ref(), salle_id, utilisateur_id).await?;
    if !admin && !moderateur {
        return Err(ApiErreur::NonAutorise(
            "Seul un modérateur attitré ou admin peut publier un fichier".into(),
        ));
    }

    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut fichier_url: Option<String> = None;

    while let Some(field_res) = payload.next().await {
        let mut field =
            field_res.map_err(|e| ApiErreur::Upload(format!("Erreur multipart : {}", e)))?;
        let nom = field
            .content_disposition()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();

        match nom.as_str() {
            "titre" => titre = Some(lire_champ_texte(&mut field).await?.trim().to_string()),
            "description" => {
                description = Some(lire_champ_texte(&mut field).await?.trim().to_string())
            }
            "fichier" => {
                let filename_original = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
                    .ok_or_else(|| ApiErreur::Upload("Nom de fichier manquant".into()))?;
                let sanitized = sanitize_filename::sanitize(&filename_original);
                let ext = std::path::Path::new(&sanitized)
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_lowercase())
                    .unwrap_or_default();
                if !RESSOURCES_EXTENSIONS_AUTORISEES.contains(&ext.as_str()) {
                    return Err(ApiErreur::Validation(format!(
                        "Extension '{}' non autorisée",
                        ext
                    )));
                }
                let id = Uuid::new_v4();
                let rel = format!("uploads/afrolang/ressources/{}-{}", id, sanitized);
                let abs = format!("./{}", rel);
                // Sauvegarde avec contrôle taille
                if let Some(parent) = std::path::Path::new(&abs).parent() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        ApiErreur::Upload(format!("Création répertoire : {}", e))
                    })?;
                }
                let mut out = std::fs::File::create(&abs)
                    .map_err(|e| ApiErreur::Upload(format!("Création fichier : {}", e)))?;
                let mut total: usize = 0;
                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|e| {
                        ApiErreur::Upload(format!("Lecture fichier : {}", e))
                    })?;
                    total += data.len();
                    if total > RESSOURCE_TAILLE_MAX {
                        let _ = std::fs::remove_file(&abs);
                        return Err(ApiErreur::Validation(
                            "Fichier trop volumineux (max 50 Mo)".into(),
                        ));
                    }
                    out.write_all(&data).map_err(|e| {
                        ApiErreur::Upload(format!("Écriture fichier : {}", e))
                    })?;
                }
                fichier_url = Some(format!("/{}", rel));
            }
            _ => { let _ = lire_champ_texte(&mut field).await; }
        }
    }

    let titre = titre
        .filter(|t| !t.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le titre est obligatoire".into()))?;
    let fichier_url = fichier_url
        .ok_or_else(|| ApiErreur::Validation("Aucun fichier fourni".into()))?;

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.ressource_salle
            (salle_id, titre, description, type, fichier_url, etat, ajoute_par,
             valide_par, valide_at)
         VALUES ($1, $2, $3, 'fichier'::afrolang.type_ressource, $4,
                 'publiee'::afrolang.etat_ressource, $5, $5, NOW())
         RETURNING id",
    )
    .bind(salle_id)
    .bind(&titre)
    .bind(description.as_deref())
    .bind(&fichier_url)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "afrolang.ressource.fichier_publie",
        "afrolang",
        "ressource_salle",
        Some(id),
        None,
        Some(serde_json::json!({
            "salle_id": salle_id, "titre": titre, "fichier_url": fichier_url,
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": id,
            "fichier_url": fichier_url,
            "etat": "publiee",
        })),
        error: None,
    }))
}

/// POST /api/afrolang/salles/{salle_id}/ressources/lien — Soumettre un lien [JWT]
pub async fn soumettre_lien_externe(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CreerRessourceLienRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_id = chemin.into_inner();

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est obligatoire".into()));
    }
    let lien = body.lien_url.trim();
    if !(lien.starts_with("http://") || lien.starts_with("https://")) {
        return Err(ApiErreur::Validation(
            "L'URL doit commencer par http:// ou https://".into(),
        ));
    }
    if lien.len() > 1000 || lien.chars().any(|c| c.is_control()) {
        return Err(ApiErreur::Validation(
            "URL invalide (longueur ou caractères non autorisés)".into(),
        ));
    }

    let salle_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND actif = TRUE AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_active {
        return Err(ApiErreur::NonTrouve("Salle publique introuvable".into()));
    }

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.ressource_salle
            (salle_id, titre, description, type, lien_url, etat, ajoute_par)
         VALUES ($1, $2, $3, 'lien_externe'::afrolang.type_ressource, $4,
                 'en_attente_validation'::afrolang.etat_ressource, $5)
         RETURNING id",
    )
    .bind(salle_id)
    .bind(titre)
    .bind(body.description.as_deref())
    .bind(lien)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": id,
            "etat": "en_attente_validation",
        })),
        error: None,
    }))
}

/// DELETE /api/afrolang/ressources/{id} — Suppression (auteur, modérateur, admin) [JWT]
pub async fn supprimer_ressource(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    let info: Option<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT salle_id, ajoute_par FROM afrolang.ressource_salle
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (salle_id, ajoute_par) =
        info.ok_or_else(|| ApiErreur::NonTrouve("Ressource introuvable".into()))?;

    let admin = verifier_admin(pool.get_ref(), utilisateur_id).await?;
    let moderateur = est_moderateur_attitre(pool.get_ref(), salle_id, utilisateur_id).await?;
    if ajoute_par != utilisateur_id && !admin && !moderateur {
        return Err(ApiErreur::NonAutorise(
            "Vous n'êtes pas autorisé à supprimer cette ressource".into(),
        ));
    }

    sqlx::query(
        "UPDATE afrolang.ressource_salle
         SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "afrolang.ressource.suppression",
        "afrolang",
        "ressource_salle",
        Some(id),
        None,
        None,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::NoContent().finish())
}

// ══════════════════════════════════════════════════════════════════════════
// Refonte 2026-04 — Salles privées : code secret, rate limit, jeton d'accès
// ══════════════════════════════════════════════════════════════════════════

/// Durée de vie d'un jeton d'accès salle privée (4 heures).
const ACCES_JETON_TTL_SECONDES: i64 = 4 * 60 * 60;

/// Nom du header HTTP portant le jeton d'accès salle privée
const HEADER_ACCES_JETON: &str = "X-Afrolang-Acces-Jeton";

/// Valide le format du code d'accès saisi par un utilisateur.
///
/// Règle (R2) : 4 à 16 caractères, alphanumérique + symboles courants
/// `!@#$%&*?-`. Les espaces, unicode étendu et autres symboles sont refusés
/// pour éviter les confusions orales / saisie mobile.
pub fn valider_format_code_acces(code: &str) -> Result<(), ApiErreur> {
    let long = code.chars().count();
    if !(4..=16).contains(&long) {
        return Err(ApiErreur::Validation(
            "Le code d'accès doit contenir entre 4 et 16 caractères".into(),
        ));
    }
    let charset_ok = code.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || matches!(c, '!' | '@' | '#' | '$' | '%' | '&' | '*' | '?' | '-')
    });
    if !charset_ok {
        return Err(ApiErreur::Validation(
            "Le code d'accès ne peut contenir que des caractères alphanumériques ou les symboles !@#$%&*?-"
                .into(),
        ));
    }
    Ok(())
}

/// Calcule le hash bcrypt (cost 10) d'un code d'accès en clair.
///
/// Cost 10 choisi (vs 12 pour les mots de passe) : le code est à faible
/// entropie ; la protection principale repose sur le rate limit (R4).
pub fn hasher_code_acces(code: &str) -> Result<String, ApiErreur> {
    bcrypt::hash(code, 10).map_err(|e| {
        ApiErreur::BaseDeDonnees(format!("Erreur hashage code accès : {}", e))
    })
}

/// Vérifie un code en clair contre son hash bcrypt.
pub fn verifier_code_acces_plain(code: &str, hash: &str) -> Result<bool, ApiErreur> {
    bcrypt::verify(code, hash).map_err(|e| {
        ApiErreur::BaseDeDonnees(format!("Erreur vérification code accès : {}", e))
    })
}

/// Charge la ligne complète d'une salle privée (y compris `code_acces_hash`)
/// pour vérification serveur. Filtre les salles supprimées.
async fn charger_salle_privee_interne(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<SallePriveeRow>, ApiErreur> {
    let sql = format!(
        "SELECT {},
            u.nom AS createur_nom, u.prenom AS createur_prenom,
            u.photo_url AS createur_photo,
            s.titre AS salle_titre, s.langue_cible AS salle_langue,
            EXISTS(SELECT 1 FROM afrolang.session ses
                   WHERE ses.salle_privee_id = sp.id AND ses.etat = 'en_cours') AS session_en_cours
         FROM afrolang.salle_privee sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.cree_par
         LEFT JOIN afrolang.salle s ON s.id = sp.salle_id
         WHERE sp.id = $1 AND sp.deleted_at IS NULL",
        SALLE_PRIVEE_COLONNES
    );
    let row = sqlx::query_as::<_, SallePriveeRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// POST /api/afrolang/salles-privees/{id}/verifier-code
///
/// Vérifie le code d'accès saisi. Cas particuliers :
///  1. Utilisateur == auteur → jeton remis sans vérification (FR-014).
///  2. Rate limit (5 échecs / 1 min, verrou 5 min).
///  3. Salle archivée ou supprimée → 404 (message générique, ne rien fuiter).
///
/// Audit : `verifier_code_salle_privee_echec` sur échec uniquement (les
/// succès sont loggés implicitement lors du démarrage de session).
pub async fn verifier_code_acces_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<VerifierCodeAccesRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_privee_id = chemin.into_inner();

    // Charger la salle (sans vérifier le code) pour détecter 404/archivée.
    let salle = charger_salle_privee_interne(pool.get_ref(), salle_privee_id).await?;
    let Some(salle) = salle else {
        return Err(ApiErreur::NonTrouve("Salle privée inexistante".into()));
    };
    if salle.archivee_at.is_some() {
        // Même message générique que « inexistante » pour ne rien fuiter.
        return Err(ApiErreur::NonTrouve("Salle privée inexistante".into()));
    }

    // Salle publique parente désactivée par l'administration (FR-019).
    let parent_desactivee: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND desactivee_admin_at IS NOT NULL)",
    )
    .bind(salle.salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if parent_desactivee {
        return Err(ApiErreur::AccesInterdit(
            "Salle désactivée par l'administration".into(),
        ));
    }

    // Auteur : court-circuit (FR-014) — pas de vérification, pas d'audit.
    if salle.cree_par == utilisateur_id {
        let (jeton, expires_at) = jwt::creer_acces_jeton_salle_privee(
            salle_privee_id,
            utilisateur_id,
            ACCES_JETON_TTL_SECONDES,
        )?;
        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(VerifierCodeAccesResponse {
                salle_privee_id,
                acces_jeton: jeton,
                expires_at,
            }),
            error: None,
        }));
    }

    // Rate limit avant toute vérification du hash.
    if afrolang_rate_limit::est_verrouillee(pool.get_ref(), salle_privee_id, utilisateur_id)
        .await?
    {
        return Err(ApiErreur::LimiteAtteinte(
            "Trop de tentatives, réessayez dans quelques minutes".into(),
        ));
    }

    let code = body.code_acces.as_str();
    let succes = verifier_code_acces_plain(code, &salle.code_acces_hash)?;

    // Enregistrer la tentative (succès ou échec) pour le rate limit.
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    afrolang_rate_limit::enregistrer_tentative(
        pool.get_ref(),
        salle_privee_id,
        utilisateur_id,
        succes,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await?;

    if !succes {
        // Audit échec uniquement — jamais le code en clair.
        audit::log_action(
            pool.get_ref(),
            Some(utilisateur_id),
            "verifier_code_salle_privee_echec",
            "afrolang",
            "salle_privee",
            Some(salle_privee_id),
            None,
            None,
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
        return Err(ApiErreur::AccesInterdit("Code incorrect".into()));
    }

    // Mémoriser l'accès à la salle privée pour la lecture future des ressources
    // contribuées (feature 001-ressources-fermeture-session, FR-001 option C).
    // ON CONFLICT idempotent : pas d'audit sur les re-validations.
    let inserted_acces = sqlx::query(
        "INSERT INTO afrolang.acces_salle_privee (salle_privee_id, utilisateur_id, valide_at)
         VALUES ($1, $2, NOW())
         ON CONFLICT (salle_privee_id, utilisateur_id) WHERE revoque_at IS NULL
         DO NOTHING",
    )
    .bind(salle_privee_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if inserted_acces.rows_affected() == 1 {
        audit::log_action(
            pool.get_ref(),
            Some(utilisateur_id),
            "CREATE",
            "afrolang",
            "acces_salle_privee",
            Some(salle_privee_id),
            None,
            Some(serde_json::json!({ "salle_privee_id": salle_privee_id })),
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
    }

    let (jeton, expires_at) = jwt::creer_acces_jeton_salle_privee(
        salle_privee_id,
        utilisateur_id,
        ACCES_JETON_TTL_SECONDES,
    )?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(VerifierCodeAccesResponse {
            salle_privee_id,
            acces_jeton: jeton,
            expires_at,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles-privees/{id}/sessions/demarrer-ou-rejoindre
///
/// Démarre une nouvelle session si aucune n'est en cours, ou rejoint la
/// session `en_cours` existante. Requiert un jeton d'accès valide (obtenu
/// via `verifier-code`) dans le header `X-Afrolang-Acces-Jeton`.
pub async fn demarrer_ou_rejoindre_session_salle_privee(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_privee_id = chemin.into_inner();

    // Récupérer et valider le jeton d'accès.
    let jeton_header = req
        .headers()
        .get(HEADER_ACCES_JETON)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| ApiErreur::NonAutorise(
            format!("Header {} manquant", HEADER_ACCES_JETON)
        ))?;
    jwt::valider_acces_jeton_salle_privee(&jeton_header, salle_privee_id, utilisateur_id)?;

    // Charger la salle pour vérifier son état (410 si archivée).
    let salle = charger_salle_privee_interne(pool.get_ref(), salle_privee_id).await?
        .ok_or_else(|| ApiErreur::NonTrouve("Salle privée inexistante".into()))?;
    if salle.archivee_at.is_some() {
        return Ok(HttpResponse::Gone().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Salle privée archivée".into()),
        }));
    }

    // Salle publique parente désactivée par l'administration (FR-019).
    let parent_desactivee: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND desactivee_admin_at IS NOT NULL)",
    )
    .bind(salle.salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if parent_desactivee {
        return Err(ApiErreur::AccesInterdit(
            "Salle désactivée par l'administration".into(),
        ));
    }

    let moderateur_id = salle.cree_par;

    // Transaction pour garantir au plus UNE session en_cours par salle privée.
    let mut tx = pool.begin().await?;

    let session_existante: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM afrolang.session
         WHERE salle_privee_id = $1 AND etat = 'en_cours'
         FOR UPDATE",
    )
    .bind(salle_privee_id)
    .fetch_optional(&mut *tx)
    .await?;

    let session_id = match session_existante {
        Some(id) => id,
        None => {
            // Créer une nouvelle session en_cours ; moderateur = auteur de la
            // salle privée, cree_par = utilisateur courant (peut différer).
            sqlx::query_scalar(
                "INSERT INTO afrolang.session
                    (salle_privee_id, etat, moderateur_id, demarre_at,
                     max_participants, tableau_blanc_actif, cree_par)
                 VALUES ($1, 'en_cours', $2, NOW(), $3, TRUE, $4)
                 RETURNING id",
            )
            .bind(salle_privee_id)
            .bind(moderateur_id)
            .bind(salle.max_participants.unwrap_or(50))
            .bind(utilisateur_id)
            .fetch_one(&mut *tx)
            .await?
        }
    };

    // INSERT du participant (idempotent — reconnexion possible).
    let role = if utilisateur_id == moderateur_id {
        "moderateur"
    } else {
        "participant"
    };
    sqlx::query(
        "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
         VALUES ($1, $2, $3)
         ON CONFLICT (session_id, utilisateur_id)
         DO UPDATE SET quitte_at = NULL, rejoint_at = NOW()",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .bind(role)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // Mettre à jour le pic de participants (hors transaction).
    sqlx::query(
        "UPDATE afrolang.session
         SET nombre_participants_pic = GREATEST(
                nombre_participants_pic,
                (SELECT COUNT(*) FROM afrolang.session_participant
                 WHERE session_id = $1 AND quitte_at IS NULL)
             ), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    // Générer le token LiveKit.
    let (user_nom, user_prenom): (String, Option<String>) = sqlx::query_as(
        "SELECT nom, prenom FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;
    let display_name = format!(
        "{} {}",
        user_prenom.as_deref().unwrap_or(""),
        user_nom
    )
    .trim()
    .to_string();
    let room_name = format!("afrolang-{}", session_id);

    let livekit_token = livekit_api::access_token::AccessToken::with_api_key(
        &livekit_config.api_key,
        &livekit_config.api_secret,
    )
    .with_identity(&utilisateur_id.to_string())
    .with_name(&display_name)
    .with_grants(livekit_api::access_token::VideoGrants {
        room_join: true,
        room: room_name.clone(),
        can_publish: true,
        can_subscribe: true,
        can_publish_data: true,
        ..Default::default()
    })
    .to_jwt()
    .map_err(|e| ApiErreur::Validation(format!("Erreur génération token LiveKit : {}", e)))?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "rejoindre_session_salle_privee",
        "afrolang",
        "session",
        Some(session_id),
        None,
        Some(serde_json::json!({
            "salle_privee_id": salle_privee_id,
            "role": role,
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(DemarrerRejoindreResponse {
            session_id,
            livekit_url: livekit_config.url.clone(),
            livekit_token,
            moderateur_id: Some(moderateur_id),
            // Salle privée : le créateur est le modérateur d'office ; pas de passation.
            suis_je_moderateur: utilisateur_id == moderateur_id,
            passation_en_attente: None,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles/{salle_id}/sessions/demarrer-ou-rejoindre
///
/// US1 — Refonte 2026-04. Démarre une nouvelle session live si aucune
/// n'est en cours dans la salle publique, sinon rejoint la session
/// existante. Ouvert à n'importe quel utilisateur connecté (FR-005b) :
/// le premier arrivé devient modérateur de session ; si un modérateur
/// attitré arrive ensuite, `rejoindre_session` (endpoint compat)
/// gère la reprise automatique côté legacy — ici on se limite au
/// démarrage/jointure pour tenir SC-001 (≤ 3 s).
pub async fn demarrer_ou_rejoindre_session_salle_publique(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_id = chemin.into_inner();

    // Vérifier que la salle publique existe et est active.
    let salle_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND actif = TRUE AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_active {
        return Err(ApiErreur::NonTrouve("Salle publique introuvable".into()));
    }

    // Désactivation administrative (feature 001-ressources-fermeture-session, FR-019)
    let desactivee_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND desactivee_admin_at IS NOT NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if desactivee_admin {
        return Err(ApiErreur::AccesInterdit(
            "Salle désactivée par l'administration".into(),
        ));
    }

    // Transaction : garantir au plus UNE session en_cours par salle publique.
    let mut tx = pool.begin().await?;

    let session_existante: Option<(Uuid, Option<Uuid>, Option<chrono::DateTime<chrono::Utc>>)> =
        sqlx::query_as(
            "SELECT id, moderateur_id, demande_passation_at FROM afrolang.session
             WHERE salle_id = $1 AND etat = 'en_cours'
             FOR UPDATE",
        )
        .bind(salle_id)
        .fetch_optional(&mut *tx)
        .await?;

    // Effets à diffuser APRÈS commit (LiveKit + temps réel).
    let mut effet_resolution: Option<EffetResolution> = None;
    let mut effet_arrivee = EffetArrivee::Aucun;

    let (session_id, est_nouveau) = match session_existante {
        None => {
            // Aucun live en cours : créer et démarrer immédiatement. Le créateur
            // démarre modérateur ; s'il est office/attitré, PAS de placeholder
            // (moderateur_id=NULL), sinon il EST le placeholder (moderateur_id=lui).
            let createur_mod_legitime: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM iam.utilisateur_role ur JOIN iam.role r ON ur.role_id=r.id
                               WHERE ur.utilisateur_id=$1 AND r.slug='admin')
                     OR EXISTS(SELECT 1 FROM afrolang.salle_administrateur
                               WHERE salle_id=$2 AND utilisateur_id=$1 AND actif=TRUE)
                     OR EXISTS(SELECT 1 FROM afrolang.salle_moderateur
                               WHERE salle_id=$2 AND utilisateur_id=$1 AND actif=TRUE)",
            )
            .bind(utilisateur_id)
            .bind(salle_id)
            .fetch_one(&mut *tx)
            .await?;
            let moderateur_initial: Option<Uuid> =
                if createur_mod_legitime { None } else { Some(utilisateur_id) };

            let nouvelle_id: Uuid = sqlx::query_scalar(
                "INSERT INTO afrolang.session
                    (salle_id, etat, moderateur_id, demarre_at,
                     max_participants, tableau_blanc_actif, cree_par)
                 VALUES ($1, 'en_cours', $2, NOW(), $3, TRUE, $4)
                 RETURNING id",
            )
            .bind(salle_id)
            .bind(moderateur_initial)
            .bind(50_i32)
            .bind(utilisateur_id)
            .fetch_one(&mut *tx)
            .await?;

            sqlx::query(
                "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
                 VALUES ($1, $2, 'moderateur')
                 ON CONFLICT (session_id, utilisateur_id)
                 DO UPDATE SET quitte_at = NULL, rejoint_at = NOW(), role_session = 'moderateur'",
            )
            .bind(nouvelle_id)
            .bind(utilisateur_id)
            .execute(&mut *tx)
            .await?;

            (nouvelle_id, true)
        }
        Some((id, moderateur_courant, demande_courante)) => {
            // Résolution paresseuse d'une éventuelle demande échue, DANS la tx.
            let (mod_id, dem_at) = if demande_courante.is_some() {
                effet_resolution = resoudre_passation_tx(&mut tx, id, salle_id, None, true).await?;
                if effet_resolution.is_some() {
                    sqlx::query_as::<_, (Option<Uuid>, Option<chrono::DateTime<chrono::Utc>>)>(
                        "SELECT moderateur_id, demande_passation_at FROM afrolang.session WHERE id=$1",
                    )
                    .bind(id)
                    .fetch_one(&mut *tx)
                    .await?
                } else {
                    (moderateur_courant, demande_courante)
                }
            } else {
                (moderateur_courant, demande_courante)
            };

            // INSERT participant (rôle 'participant' par défaut ; le helper promeut si besoin).
            sqlx::query(
                "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
                 VALUES ($1, $2, 'participant')
                 ON CONFLICT (session_id, utilisateur_id)
                 DO UPDATE SET quitte_at = NULL, rejoint_at = NOW()",
            )
            .bind(id)
            .bind(utilisateur_id)
            .execute(&mut *tx)
            .await?;

            effet_arrivee = appliquer_arrivee_moderation_publique_tx(
                &mut tx, id, salle_id, mod_id, dem_at, utilisateur_id,
            )
            .await?;

            (id, false)
        }
    };

    tx.commit().await?;

    // Effets temps réel post-commit (LiveKit + diffusion).
    if let Some(e) = effet_resolution {
        executer_effet_resolution(pool.get_ref(), livekit_config.get_ref(), session_id, None, e).await;
    }
    executer_effet_arrivee(pool.get_ref(), livekit_config.get_ref(), session_id, effet_arrivee).await;

    // Pic de participants (hors transaction).
    sqlx::query(
        "UPDATE afrolang.session
         SET nombre_participants_pic = GREATEST(
                nombre_participants_pic,
                (SELECT COUNT(*) FROM afrolang.session_participant
                 WHERE session_id = $1 AND quitte_at IS NULL)
             ), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    // Niveau effectif → token + réponse (post-effets).
    let niveau = est_moderateur_actif(pool.get_ref(), session_id, utilisateur_id).await?;
    let is_moderator = niveau.is_some();
    let can_publish_data = if is_moderator {
        true
    } else {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM afrolang.session_permission_tableau_blanc
                           WHERE session_id=$1 AND utilisateur_id=$2)",
        )
        .bind(session_id)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?
    };

    // Générer le token LiveKit.
    let (user_nom, user_prenom): (String, Option<String>) = sqlx::query_as(
        "SELECT nom, prenom FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;
    let display_name = format!("{} {}", user_prenom.as_deref().unwrap_or(""), user_nom)
        .trim()
        .to_string();
    let room_name = format!("afrolang-{}", session_id);

    let livekit_token = livekit_api::access_token::AccessToken::with_api_key(
        &livekit_config.api_key,
        &livekit_config.api_secret,
    )
    .with_identity(&utilisateur_id.to_string())
    .with_name(&display_name)
    .with_grants(livekit_api::access_token::VideoGrants {
        room_join: true,
        room: room_name.clone(),
        can_publish: true,
        can_subscribe: true,
        can_publish_data,
        ..Default::default()
    })
    .to_jwt()
    .map_err(|e| ApiErreur::Validation(format!("Erreur génération token LiveKit : {}", e)))?;

    let action = if est_nouveau {
        "demarrer_session_salle_publique"
    } else {
        "rejoindre_session_salle_publique"
    };
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        action,
        "afrolang",
        "session",
        Some(session_id),
        None,
        Some(serde_json::json!({
            "salle_id": salle_id,
            "role": if is_moderator { "moderateur" } else { "participant" },
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    // moderateur_id (placeholder courant) + état de passation pour la réponse.
    let moderateur_id_actuel: Option<Uuid> = sqlx::query_scalar(
        "SELECT moderateur_id FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_one(pool.get_ref())
    .await?;
    let passation = charger_passation_en_attente(pool.get_ref(), session_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(DemarrerRejoindreResponse {
            session_id,
            livekit_url: livekit_config.url.clone(),
            livekit_token,
            moderateur_id: moderateur_id_actuel,
            suis_je_moderateur: is_moderator,
            passation_en_attente: passation,
        }),
        error: None,
    }))
}

/// PATCH /api/afrolang/salles-privees/{id}/code-acces
///
/// Met à jour le code d'accès (auteur uniquement). Hash before/after
/// tracé dans l'audit — jamais les plaintexts.
pub async fn modifier_code_acces_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierCodeAccesRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_privee_id = chemin.into_inner();

    valider_format_code_acces(body.nouveau_code_acces.as_str())?;

    let actuel: Option<(Uuid, String, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT cree_par, code_acces_hash, archivee_at
         FROM afrolang.salle_privee
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(salle_privee_id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (createur, ancien_hash, archivee_at) = actuel
        .ok_or_else(|| ApiErreur::NonTrouve("Salle privée inexistante".into()))?;

    if createur != utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Seul le créateur peut modifier le code d'accès".into(),
        ));
    }
    if archivee_at.is_some() {
        return Err(ApiErreur::Validation("La salle privée est archivée".into()));
    }

    let nouveau_hash = hasher_code_acces(body.nouveau_code_acces.as_str())?;

    // Transaction : mise à jour du hash + révocation des accès mémorisés
    // (feature 001-ressources-fermeture-session, FR-001 option C).
    let mut tx = pool.begin().await?;

    sqlx::query(
        "UPDATE afrolang.salle_privee
         SET code_acces_hash = $2, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(salle_privee_id)
    .bind(&nouveau_hash)
    .execute(&mut *tx)
    .await?;

    let revoques = sqlx::query(
        "UPDATE afrolang.acces_salle_privee
            SET revoque_at = NOW()
          WHERE salle_privee_id = $1 AND revoque_at IS NULL",
    )
    .bind(salle_privee_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);

    // Audit : on ne trace JAMAIS le plaintext, seulement les hashes
    // (pour permettre la reconstitution d'historique sans fuite).
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "modifier_code_salle_privee",
        "afrolang",
        "salle_privee",
        Some(salle_privee_id),
        Some(serde_json::json!({ "code_acces_hash": ancien_hash })),
        Some(serde_json::json!({ "code_acces_hash": nouveau_hash })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    if revoques.rows_affected() > 0 {
        audit::log_action(
            pool.get_ref(),
            Some(utilisateur_id),
            "UPDATE",
            "afrolang",
            "acces_salle_privee",
            Some(salle_privee_id),
            None,
            Some(serde_json::json!({
                "revoque_at": "NOW()",
                "motif": "changement_code",
                "lignes_revoquees": revoques.rows_affected(),
            })),
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
    }

    Ok(HttpResponse::NoContent().finish())
}

/// POST /api/afrolang/salles-privees/{id}/archiver
///
/// Archive la salle privée (auteur uniquement). Si une session est en cours,
/// elle est terminée. L'archivage libère le verrou d'unicité
/// (salle_id, cree_par) → l'utilisateur peut en recréer une.
pub async fn archiver_salle_privee_par_auteur(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_privee_id = chemin.into_inner();

    let mut tx = pool.begin().await?;

    let salle: Option<(Uuid, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT cree_par, archivee_at
         FROM afrolang.salle_privee
         WHERE id = $1 AND deleted_at IS NULL
         FOR UPDATE",
    )
    .bind(salle_privee_id)
    .fetch_optional(&mut *tx)
    .await?;
    let (createur, archivee_at) = salle
        .ok_or_else(|| ApiErreur::NonTrouve("Salle privée inexistante".into()))?;

    if createur != utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Seul le créateur peut archiver cette salle".into(),
        ));
    }
    if archivee_at.is_some() {
        return Err(ApiErreur::Validation("Salle déjà archivée".into()));
    }

    // Archiver la salle.
    sqlx::query(
        "UPDATE afrolang.salle_privee
         SET archivee_at = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(salle_privee_id)
    .execute(&mut *tx)
    .await?;

    // Terminer la session en cours s'il y en a une.
    sqlx::query(
        "UPDATE afrolang.session
         SET etat = 'terminee', termine_at = NOW(),
             duree_secondes = EXTRACT(EPOCH FROM (NOW() - COALESCE(demarre_at, created_at)))::INT,
             updated_at = NOW()
         WHERE salle_privee_id = $1 AND etat = 'en_cours'",
    )
    .bind(salle_privee_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "archiver_salle_privee",
        "afrolang",
        "salle_privee",
        Some(salle_privee_id),
        Some(serde_json::json!({ "archivee_at": null })),
        Some(serde_json::json!({ "archivee_at": "NOW()" })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::NoContent().finish())
}

// ══════════════════════════════════════════════════════════════════════════
// Feature 001-admin-salles-publiques — Propositions communautaires (US1)
// ══════════════════════════════════════════════════════════════════════════

/// Construit les jointures + json_agg communs aux endpoints proposition.
fn proposition_select_query(where_clause: &str, order_limit: &str) -> String {
    format!(
        "SELECT {cols},
            ua.nom AS auteur_nom, ua.prenom AS auteur_prenom,
            ud.nom AS decideur_nom, ud.prenom AS decideur_prenom,
            ge.nom AS groupe_ethnique_nom,
            COALESCE((SELECT json_agg(json_build_object(
                        'id', p.id, 'nom', p.nom, 'code_iso2', p.code_iso2
                     ) ORDER BY p.nom)
                     FROM unnest(ps.pays_origine_ids) AS pid(id)
                     JOIN shared.pays p ON p.id = pid.id),
                     '[]'::json) AS pays_origine_json
         FROM afrolang.proposition_salle ps
         LEFT JOIN iam.utilisateur ua ON ua.id = ps.auteur_id
         LEFT JOIN iam.utilisateur ud ON ud.id = ps.decideur
         LEFT JOIN country_profile.groupe_ethnique ge ON ge.id = ps.groupe_ethnique_id
         WHERE {where_clause}
         {order_limit}",
        cols = COLONNES_PROPOSITION,
        where_clause = where_clause,
        order_limit = order_limit,
    )
}

/// GET /api/afrolang/pays-disponibles — Liste légère des pays actifs (US1)
/// pour alimenter le formulaire de proposition.
pub async fn lister_pays_disponibles(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    // Uniquement les pays effectivement référencés par au moins une salle
    // publique active (alignement sur `lister_langues`). Évite d'exposer dans le
    // filtre des territoires sans aucune salle associée.
    let rows: Vec<(Uuid, String, Option<String>)> = sqlx::query_as(
        "SELECT DISTINCT p.id, p.nom, p.code_iso2
         FROM shared.pays p
         JOIN afrolang.salle_pays_origine spo ON spo.pays_id = p.id
         JOIN afrolang.salle s ON s.id = spo.salle_id
         WHERE p.actif = TRUE AND s.actif = TRUE
         ORDER BY p.nom ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    let items: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|(id, nom, code_iso2)| serde_json::json!({
            "id": id,
            "nom": nom,
            "code_iso2": code_iso2,
        }))
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
}

/// GET /api/afrolang/territoires
/// Tous les territoires actifs pour le formulaire de proposition (Afrique d'abord,
/// puis autres continents — diaspora où des langues africaines ont essaimé).
pub async fn lister_territoires(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let rows = sqlx::query_as::<_, TerritoireRow>(
        "SELECT id, nom, code_iso2, continent
         FROM shared.pays
         WHERE actif = TRUE
         ORDER BY (continent = 'Afrique') DESC, continent ASC, nom ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    let data: Vec<TerritoireResponse> = rows
        .into_iter()
        .map(|r| TerritoireResponse {
            id: r.id,
            nom: r.nom,
            code_iso2: r.code_iso2,
            continent: r.continent,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
    }))
}

/// Helper interne : recharge une proposition avec ses jointures.
async fn charger_proposition_par_id(
    pool: &PgPool,
    proposition_id: Uuid,
) -> Result<PropositionResponse, ApiErreur> {
    let query = proposition_select_query("ps.id = $1", "");
    let row = sqlx::query_as::<_, PropositionSalleRow>(&query)
        .bind(proposition_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;
    Ok(row.to_response())
}

/// POST /api/afrolang/propositions
/// Soumet une nouvelle proposition de salle publique (US1, FR-001..FR-007).
pub async fn soumettre_proposition(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<SoumettrePropositionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let titre = body.titre.trim();
    let description = body.description.trim();
    let justification = body.justification.trim();
    let langue_cible = body.langue_cible.trim();
    if titre.is_empty()
        || description.is_empty()
        || justification.is_empty()
        || langue_cible.is_empty()
    {
        return Err(ApiErreur::Validation(
            "Tous les champs textuels sont obligatoires".into(),
        ));
    }
    if titre.chars().count() > 350 {
        return Err(ApiErreur::Validation(
            "Le titre ne peut excéder 350 caractères".into(),
        ));
    }
    if langue_cible.chars().count() > 100 {
        return Err(ApiErreur::Validation(
            "La langue cible ne peut excéder 100 caractères".into(),
        ));
    }
    if let Some(ref code) = body.langue_code {
        if code.trim().chars().count() > 40 {
            return Err(ApiErreur::Validation(
                "Le code de langue ne peut excéder 40 caractères".into(),
            ));
        }
    }
    if body.pays_origine_ids.is_empty() {
        return Err(ApiErreur::Validation(
            "Au moins un pays d'origine est requis".into(),
        ));
    }

    // Groupe ethnique : SOIT un groupe référencé (groupe_ethnique_id),
    // SOIT un nom libre « Autre » (groupe_ethnique_libre) — jamais les deux,
    // jamais aucun.
    let groupe_libre = body
        .groupe_ethnique_libre
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    match (body.groupe_ethnique_id, &groupe_libre) {
        (Some(_), Some(_)) => {
            return Err(ApiErreur::Validation(
                "Choisissez un groupe ethnique existant OU précisez « Autre », pas les deux".into(),
            ));
        }
        (None, None) => {
            return Err(ApiErreur::Validation(
                "Le groupe ethnique est requis (sélectionnez-en un ou précisez « Autre »)".into(),
            ));
        }
        _ => {}
    }
    if let Some(ref libre) = groupe_libre {
        if libre.chars().count() > 250 {
            return Err(ApiErreur::Validation(
                "Le nom du groupe ethnique ne peut excéder 250 caractères".into(),
            ));
        }
    }

    let auteur_actif: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.utilisateur
            WHERE id = $1 AND etat::text = 'actif' AND deleted_at IS NULL
        )",
    )
    .bind(auteur_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !auteur_actif {
        return Err(ApiErreur::AccesInterdit(
            "Compte non actif — vérifiez votre e-mail".into(),
        ));
    }

    // Les vérifications liées à un groupe référencé ne s'appliquent que si un
    // identifiant est fourni (cas « Autre » = texte libre, aucun id).
    if let Some(groupe_id) = body.groupe_ethnique_id {
        let groupe_existe: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM country_profile.groupe_ethnique WHERE id = $1)",
        )
        .bind(groupe_id)
        .fetch_one(pool.get_ref())
        .await?;
        if !groupe_existe {
            return Err(ApiErreur::Validation("Groupe ethnique introuvable".into()));
        }

        let salle_existe: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1 FROM afrolang.salle
                WHERE groupe_ethnique_id = $1 AND actif = TRUE AND deleted_at IS NULL
            )",
        )
        .bind(groupe_id)
        .fetch_one(pool.get_ref())
        .await?;
        if salle_existe {
            return Err(ApiErreur::Conflit(
                "Une salle publique existe déjà pour ce groupe ethnique".into(),
            ));
        }

        let proposition_existe: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1 FROM afrolang.proposition_salle
                WHERE auteur_id = $1 AND groupe_ethnique_id = $2 AND statut = 'en_attente'
            )",
        )
        .bind(auteur_id)
        .bind(groupe_id)
        .fetch_one(pool.get_ref())
        .await?;
        if proposition_existe {
            return Err(ApiErreur::Conflit(
                "Vous avez déjà une proposition en attente pour ce groupe ethnique".into(),
            ));
        }
    }

    let pays_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM shared.pays
         WHERE id = ANY($1) AND actif = TRUE",
    )
    .bind(&body.pays_origine_ids)
    .fetch_one(pool.get_ref())
    .await?;
    if pays_count != body.pays_origine_ids.len() as i64 {
        return Err(ApiErreur::Validation(
            "Un ou plusieurs pays d'origine sont introuvables ou inactifs".into(),
        ));
    }

    // Anti-spam : ≥ 5 rejets sur 7 jours (Décision 6 research.md)
    let rejets_recents: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.proposition_salle
         WHERE auteur_id = $1 AND statut = 'rejetee'
           AND decide_at > NOW() - INTERVAL '7 days'",
    )
    .bind(auteur_id)
    .fetch_one(pool.get_ref())
    .await?;
    if rejets_recents >= 5 {
        return Err(ApiErreur::LimiteAtteinte(
            "Trop de propositions rejetées récemment ; réessayez plus tard".into(),
        ));
    }

    let langue_code_clean = body
        .langue_code
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let proposition_id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.proposition_salle
            (auteur_id, titre, description, justification,
             langue_cible, langue_code, groupe_ethnique_id, groupe_ethnique_libre, pays_origine_ids)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id",
    )
    .bind(auteur_id)
    .bind(titre)
    .bind(description)
    .bind(justification)
    .bind(langue_cible)
    .bind(langue_code_clean.as_deref())
    .bind(body.groupe_ethnique_id)
    .bind(groupe_libre.as_deref())
    .bind(&body.pays_origine_ids)
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "CREATE",
        "afrolang",
        "proposition_salle",
        Some(proposition_id),
        None,
        Some(serde_json::json!({
            "titre": titre,
            "groupe_ethnique_id": body.groupe_ethnique_id,
            "groupe_ethnique_libre": groupe_libre,
            "langue_cible": langue_cible,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let proposition = charger_proposition_par_id(pool.get_ref(), proposition_id).await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(proposition),
        error: None,
    }))
}

/// GET /api/afrolang/propositions/moi
/// Liste les propositions de l'utilisateur authentifié (US1).
pub async fn lister_mes_propositions(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<PropositionMesFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * taille;

    let mut conditions: Vec<String> = vec!["ps.auteur_id = $1".to_string()];
    let mut bind_index = 2u32;

    if params.statut.is_some() {
        conditions.push(format!("ps.statut = ${}", bind_index));
        bind_index += 1;
    }

    let where_clause = conditions.join(" AND ");

    let count_sql = format!(
        "SELECT COUNT(*) FROM afrolang.proposition_salle ps WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql).bind(auteur_id);
    if let Some(ref statut) = params.statut {
        count_q = count_q.bind(statut);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let order_limit = format!(
        "ORDER BY ps.created_at DESC LIMIT ${} OFFSET ${}",
        bind_index,
        bind_index + 1
    );
    let select_sql = proposition_select_query(&where_clause, &order_limit);
    let mut select_q = sqlx::query_as::<_, PropositionSalleRow>(&select_sql).bind(auteur_id);
    if let Some(ref statut) = params.statut {
        select_q = select_q.bind(statut);
    }
    select_q = select_q.bind(taille).bind(offset);
    let rows = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PropositionListeResponse {
            items: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            taille,
        }),
        error: None,
    }))
}

/// PATCH /api/afrolang/propositions/{id}/retirer
/// Retire une proposition `en_attente` (auteur uniquement) — US1.
pub async fn retirer_ma_proposition(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let proposition_id = path.into_inner();

    let mut tx = pool.begin().await?;

    let row: Option<(Uuid, PropositionStatut)> = sqlx::query_as(
        "SELECT auteur_id, statut FROM afrolang.proposition_salle
         WHERE id = $1 FOR UPDATE",
    )
    .bind(proposition_id)
    .fetch_optional(&mut *tx)
    .await?;

    let (auteur_proposition, statut) = row
        .ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;

    if auteur_proposition != auteur_id {
        return Err(ApiErreur::AccesInterdit(
            "Cette proposition ne vous appartient pas".into(),
        ));
    }
    if statut != PropositionStatut::EnAttente {
        return Err(ApiErreur::Conflit(
            "Seule une proposition en attente peut être retirée".into(),
        ));
    }

    sqlx::query(
        "UPDATE afrolang.proposition_salle
         SET statut = 'retiree', updated_at = NOW()
         WHERE id = $1",
    )
    .bind(proposition_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "UPDATE",
        "afrolang",
        "proposition_salle",
        Some(proposition_id),
        Some(serde_json::json!({ "statut": "en_attente" })),
        Some(serde_json::json!({ "statut": "retiree" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let proposition = charger_proposition_par_id(pool.get_ref(), proposition_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(proposition),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Feature 001-session-moderation — permissions tableau blanc + spotlight
// ══════════════════════════════════════════════════════════════════════════

use crate::models::afrolang::{
    AccorderPermissionPayload, MettreEnEvidencePayload, ModerateurOfficeResponse,
    NiveauModerateur, PermissionTableauBlancResponse,
    PermissionsTableauBlancListeResponse, SpotlightInfo,
};
/// Nom de la room LiveKit associée à une session Afrolang.
fn room_name_session(session_id: Uuid) -> String {
    format!("afrolang-{}", session_id)
}

/// Charge le contexte salle d'une session pour calculer les modérateurs d'office.
async fn charger_contexte_salle(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<(Option<Uuid>, Option<Uuid>), ApiErreur> {
    let row: Option<(Option<Uuid>, Option<Uuid>)> = sqlx::query_as(
        "SELECT salle_id, salle_privee_id FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;
    row.ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvée", session_id)))
}

/// Liste les modérateurs d'office d'une session (admin plateforme connecté
/// inclus uniquement s'ils participent — l'admin global est calculé côté
/// `est_moderateur_session` à l'arrivée). Retourne au moins :
/// - administrateurs actifs de la salle publique (`AdminSalle`)
/// - modérateurs attitrés actifs (`ModerateurAttitre`)
/// - créateur de la salle privée (`CreateurSallePrivee`)
async fn lister_moderateurs_office(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<Vec<ModerateurOfficeResponse>, ApiErreur> {
    // Refonte multi-modérateurs : on liste les modérateurs RÉELLEMENT actifs et
    // présents (role_session='moderateur'), pas tous les attitrés de la salle —
    // un attitré entré mais en attente de passation (role 'participant') ne doit
    // PAS apparaître ici (cohérence avec le gating de est_moderateur_actif).
    let rows = sqlx::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
        "SELECT sp.utilisateur_id, u.nom, u.prenom, u.photo_url
         FROM afrolang.session_participant sp
         JOIN iam.utilisateur u ON u.id = sp.utilisateur_id
         WHERE sp.session_id = $1 AND sp.quitte_at IS NULL AND sp.role_session = 'moderateur'
         ORDER BY sp.rejoint_at ASC",
    )
    .bind(session_id)
    .fetch_all(pool)
    .await?;

    let mut resultats: Vec<ModerateurOfficeResponse> = Vec::with_capacity(rows.len());
    for (uid, nom, prenom, photo) in rows {
        let niveau = est_moderateur_actif(pool, session_id, uid)
            .await?
            .unwrap_or(NiveauModerateur::Demarreur);
        resultats.push(ModerateurOfficeResponse {
            utilisateur_id: uid,
            nom_complet: format!("{} {}", prenom.unwrap_or_default(), nom).trim().to_string(),
            avatar_url: photo,
            niveau,
        });
    }
    Ok(resultats)
}

/// Charge l'état spotlight courant pour une session (FR-024).
async fn charger_spotlight(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<Option<SpotlightInfo>, ApiErreur> {
    let row: Option<(Uuid, Uuid, chrono::DateTime<chrono::Utc>, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT s.participant_mis_en_evidence_id, s.mis_en_evidence_par, s.mis_en_evidence_at,
                u.nom, u.prenom, u.photo_url
         FROM afrolang.session s
         JOIN iam.utilisateur u ON u.id = s.participant_mis_en_evidence_id
         WHERE s.id = $1 AND s.participant_mis_en_evidence_id IS NOT NULL",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(uid, par, at, nom, prenom, photo)| SpotlightInfo {
        utilisateur_id: uid,
        nom_complet: format!("{} {}", prenom.unwrap_or_default(), nom).trim().to_string(),
        avatar_url: photo,
        mis_en_evidence_par: par,
        mis_en_evidence_at: at,
    }))
}

/// GET /api/afrolang/sessions/{id}/permissions-tableau-blanc
pub async fn lister_permissions_tableau_blanc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let session_id = chemin.into_inner();

    let moderateurs_office = lister_moderateurs_office(pool.get_ref(), session_id).await?;

    let perms = sqlx::query_as::<_, (Uuid, Uuid, chrono::DateTime<chrono::Utc>, String, Option<String>, Option<String>)>(
        "SELECT sptb.utilisateur_id, sptb.accorde_par, sptb.accorde_at,
                u.nom, u.prenom, u.photo_url
         FROM afrolang.session_permission_tableau_blanc sptb
         JOIN iam.utilisateur u ON u.id = sptb.utilisateur_id
         WHERE sptb.session_id = $1
         ORDER BY sptb.accorde_at ASC",
    )
    .bind(session_id)
    .fetch_all(pool.get_ref())
    .await?;

    let permissions_individuelles: Vec<PermissionTableauBlancResponse> = perms
        .into_iter()
        .map(|(uid, par, at, nom, prenom, photo)| PermissionTableauBlancResponse {
            utilisateur_id: uid,
            nom_complet: format!("{} {}", prenom.unwrap_or_default(), nom).trim().to_string(),
            avatar_url: photo,
            accorde_par: par,
            accorde_at: at,
        })
        .collect();

    let mon_niveau = est_moderateur_actif(pool.get_ref(), session_id, utilisateur_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PermissionsTableauBlancListeResponse {
            session_id,
            moderateurs_office,
            permissions_individuelles,
            mon_niveau_moderateur: mon_niveau,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/sessions/{id}/permissions-tableau-blanc
pub async fn accorder_permission_tableau_blanc(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<AccorderPermissionPayload>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let session_id = chemin.into_inner();
    let cible_id = body.utilisateur_id;

    // 1) Auteur doit être modérateur de session
    let niveau = est_moderateur_actif(pool.get_ref(), session_id, auteur_id).await?;
    if niveau.is_none() {
        return Err(ApiErreur::AccesInterdit(
            "Seul un modérateur de session peut accorder cette permission.".into(),
        ));
    }

    // 2) Refuser si cible est déjà modérateur de session
    let niveau_cible = est_moderateur_actif(pool.get_ref(), session_id, cible_id).await?;
    if niveau_cible.is_some() {
        return Err(ApiErreur::Conflit(
            "L'utilisateur est déjà modérateur de session.".into(),
        ));
    }

    // 3) INSERT idempotent
    sqlx::query(
        "INSERT INTO afrolang.session_permission_tableau_blanc (session_id, utilisateur_id, accorde_par)
         VALUES ($1, $2, $3)
         ON CONFLICT (session_id, utilisateur_id) DO NOTHING",
    )
    .bind(session_id)
    .bind(cible_id)
    .bind(auteur_id)
    .execute(pool.get_ref())
    .await?;

    // 4) Charger les infos cible pour la réponse
    let (nom, prenom, photo, accorde_at): (String, Option<String>, Option<String>, chrono::DateTime<chrono::Utc>) = sqlx::query_as(
        "SELECT u.nom, u.prenom, u.photo_url, sptb.accorde_at
         FROM afrolang.session_permission_tableau_blanc sptb
         JOIN iam.utilisateur u ON u.id = sptb.utilisateur_id
         WHERE sptb.session_id = $1 AND sptb.utilisateur_id = $2",
    )
    .bind(session_id)
    .bind(cible_id)
    .fetch_one(pool.get_ref())
    .await?;

    let nom_complet = format!("{} {}", prenom.unwrap_or_default(), nom).trim().to_string();
    let reponse = PermissionTableauBlancResponse {
        utilisateur_id: cible_id,
        nom_complet: nom_complet.clone(),
        avatar_url: photo.clone(),
        accorde_par: auteur_id,
        accorde_at,
    };

    // 5) LiveKit : autoriser data publish
    let room_name = room_name_session(session_id);
    livekit_moderation::update_participant_can_publish_data(
        livekit_config.get_ref(),
        &room_name,
        &cible_id.to_string(),
        true,
    )
    .await?;

    // 6) Audit
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "CREATE",
        "afrolang",
        "session_permission_tableau_blanc",
        Some(session_id),
        None,
        Some(serde_json::json!({ "utilisateur_id": cible_id })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    // 7) DataPacket modération
    let payload = serde_json::json!({
        "type": "moderation",
        "subtype": "permission_update",
        "payload": {
            "session_id": session_id,
            "utilisateur_id": cible_id,
            "action": "accordee",
            "accorde_par": auteur_id,
            "accorde_at": accorde_at,
        }
    });
    livekit_moderation::publier_evenement_moderation(
        livekit_config.get_ref(),
        &room_name,
        &payload,
    )
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

/// DELETE /api/afrolang/sessions/{id}/permissions-tableau-blanc/{user_id}
pub async fn retirer_permission_tableau_blanc(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let (session_id, cible_id) = chemin.into_inner();

    let niveau = est_moderateur_actif(pool.get_ref(), session_id, auteur_id).await?;
    if niveau.is_none() {
        return Err(ApiErreur::AccesInterdit(
            "Seul un modérateur de session peut retirer cette permission.".into(),
        ));
    }

    // FR-013 : refuser de retirer la permission d'un modérateur de session
    let niveau_cible = est_moderateur_actif(pool.get_ref(), session_id, cible_id).await?;
    if niveau_cible.is_some() {
        return Err(ApiErreur::Conflit(
            "Cette permission ne peut pas être retirée à un modérateur.".into(),
        ));
    }

    let res = sqlx::query(
        "DELETE FROM afrolang.session_permission_tableau_blanc
         WHERE session_id = $1 AND utilisateur_id = $2",
    )
    .bind(session_id)
    .bind(cible_id)
    .execute(pool.get_ref())
    .await?;

    if res.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Aucune permission individuelle à retirer.".into(),
        ));
    }

    let room_name = room_name_session(session_id);
    livekit_moderation::update_participant_can_publish_data(
        livekit_config.get_ref(),
        &room_name,
        &cible_id.to_string(),
        false,
    )
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "DELETE",
        "afrolang",
        "session_permission_tableau_blanc",
        Some(session_id),
        Some(serde_json::json!({ "utilisateur_id": cible_id })),
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let payload = serde_json::json!({
        "type": "moderation",
        "subtype": "permission_update",
        "payload": {
            "session_id": session_id,
            "utilisateur_id": cible_id,
            "action": "retiree",
            "accorde_par": auteur_id,
            "accorde_at": chrono::Utc::now(),
        }
    });
    livekit_moderation::publier_evenement_moderation(
        livekit_config.get_ref(),
        &room_name,
        &payload,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// POST /api/afrolang/sessions/{id}/spotlight
pub async fn mettre_en_evidence(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<MettreEnEvidencePayload>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let session_id = chemin.into_inner();
    let cible_id = body.utilisateur_id;

    let niveau = est_moderateur_actif(pool.get_ref(), session_id, auteur_id).await?;
    let Some(n) = niveau else {
        return Err(ApiErreur::AccesInterdit(
            "Modérateur de session requis.".into(),
        ));
    };
    if !n.peut_spotlight() {
        return Err(ApiErreur::AccesInterdit(
            "Les modérateurs attitrés ne peuvent pas mettre en évidence (FR-001b).".into(),
        ));
    }

    let (salle_id, salle_privee_id) = charger_contexte_salle(pool.get_ref(), session_id).await?;
    if salle_privee_id.is_some() || salle_id.is_none() {
        return Err(ApiErreur::Validation(
            "Spotlight indisponible dans une session privée (FR-027).".into(),
        ));
    }

    let cible_presente: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.session_participant
            WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL
        )",
    )
    .bind(session_id)
    .bind(cible_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !cible_presente {
        return Err(ApiErreur::NonTrouve("Participant absent de la session.".into()));
    }

    let ancien: Option<Uuid> = sqlx::query_scalar(
        "SELECT participant_mis_en_evidence_id FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_one(pool.get_ref())
    .await?;

    sqlx::query(
        "UPDATE afrolang.session
         SET participant_mis_en_evidence_id = $2,
             mis_en_evidence_par = $3,
             mis_en_evidence_at = NOW(),
             updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session_id)
    .bind(cible_id)
    .bind(auteur_id)
    .execute(pool.get_ref())
    .await?;

    let spotlight = charger_spotlight(pool.get_ref(), session_id).await?
        .ok_or_else(|| ApiErreur::BaseDeDonnees("Spotlight non rechargé".into()))?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "UPDATE",
        "afrolang",
        "session",
        Some(session_id),
        Some(serde_json::json!({ "spotlight_id": ancien })),
        Some(serde_json::json!({ "spotlight_id": cible_id })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let payload = serde_json::json!({
        "type": "moderation",
        "subtype": "spotlight",
        "payload": spotlight,
    });
    livekit_moderation::publier_evenement_moderation(
        livekit_config.get_ref(),
        &room_name_session(session_id),
        &payload,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(spotlight),
        error: None,
    }))
}

/// DELETE /api/afrolang/sessions/{id}/spotlight
pub async fn retirer_mise_en_evidence(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let session_id = chemin.into_inner();

    let niveau = est_moderateur_actif(pool.get_ref(), session_id, auteur_id).await?;
    let Some(n) = niveau else {
        return Err(ApiErreur::AccesInterdit("Modérateur de session requis.".into()));
    };
    if !n.peut_spotlight() {
        return Err(ApiErreur::AccesInterdit("Capacité spotlight requise.".into()));
    }

    let (salle_id, salle_privee_id) = charger_contexte_salle(pool.get_ref(), session_id).await?;
    if salle_privee_id.is_some() || salle_id.is_none() {
        return Err(ApiErreur::Validation(
            "Spotlight indisponible dans une session privée.".into(),
        ));
    }

    let ancien: Option<Uuid> = sqlx::query_scalar(
        "SELECT participant_mis_en_evidence_id FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_one(pool.get_ref())
    .await?;

    sqlx::query(
        "UPDATE afrolang.session
         SET participant_mis_en_evidence_id = NULL,
             mis_en_evidence_par = NULL,
             mis_en_evidence_at = NULL,
             updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "UPDATE",
        "afrolang",
        "session",
        Some(session_id),
        Some(serde_json::json!({ "spotlight_id": ancien })),
        Some(serde_json::json!({ "spotlight_id": serde_json::Value::Null })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let payload = serde_json::json!({
        "type": "moderation",
        "subtype": "spotlight",
        "payload": serde_json::Value::Null,
    });
    livekit_moderation::publier_evenement_moderation(
        livekit_config.get_ref(),
        &room_name_session(session_id),
        &payload,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

// ══════════════════════════════════════════════════════════════════════════
// Fermeture pour abus par un admin de session (admin plateforme OU admin de salle)
// Feature 001-ressources-fermeture-session, FR-019 — depuis la salle live.
// ══════════════════════════════════════════════════════════════════════════

/// POST /api/afrolang/sessions/{session_id}/fermer-pour-abus
///
/// Variante session-level de l'endpoint admin `fermer-admin`.
/// Ouvert à tout utilisateur authentifié dont `est_moderateur_session`
/// renvoie `AdminPlateforme` OU `AdminSalle` (cf. FR-019).
pub async fn fermer_session_pour_abus(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<crate::models::admin::sessions_moderation::FermetureAdminRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let session_id = chemin.into_inner();

    let niveau = est_moderateur_actif(pool.get_ref(), session_id, utilisateur_id).await?;
    let niveau = niveau.ok_or_else(|| {
        ApiErreur::AccesInterdit(
            "Réservé aux administrateurs de salle ou de la plateforme".into(),
        )
    })?;
    if !niveau.peut_fermer_pour_abus() {
        return Err(ApiErreur::AccesInterdit(
            "Réservé aux administrateurs de salle ou de la plateforme".into(),
        ));
    }

    let resultat = crate::handlers::admin::sessions_moderation::fermer_session_pour_abus_impl(
        pool.get_ref(),
        livekit_config.get_ref(),
        &req,
        session_id,
        &body.motif,
        utilisateur_id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "salle_id": resultat.salle_id,
            "session_id": resultat.session_id,
            "fermeture": {
                "admin_id": resultat.acteur_id,
                "motif": resultat.motif,
                "created_at": resultat.created_at,
            },
            "participants_notifies_count": resultat.participants_notifies_count,
        })),
        error: None,
    }))
}

// Modèles du domaine social : demandes d'amitié, amitiés, blocages, notifications.
// Source de vérité : doc/bd/schemas/29_social.sql

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ────────────────────────────────────────────────────────────────
// DTO partagé MembreLight (champs publics uniquement — jamais d'email/téléphone)
// ────────────────────────────────────────────────────────────────

/// Colonnes pour construire un MembreLight depuis iam.utilisateur u
/// (JOIN LEFT shared.pays p ON p.id = u.pays_residence_id).
/// À préfixer/aliaser selon le contexte d'utilisation (voir handlers).
pub const MEMBRE_LIGHT_COLONNES: &str =
    "u.id, u.nom, u.prenom, u.slug, u.photo_url, u.fonction, p.nom AS pays";

/// Charge le MembreLight d'un utilisateur (pour enrichir les évènements SSE).
pub async fn obtenir_membre_light(
    pool: &sqlx::PgPool,
    utilisateur_id: Uuid,
) -> Result<Option<MembreLight>, sqlx::Error> {
    let query = format!(
        "SELECT {} FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE u.id = $1",
        MEMBRE_LIGHT_COLONNES
    );
    sqlx::query_as::<_, MembreLight>(&query)
        .bind(utilisateur_id)
        .fetch_optional(pool)
        .await
}

/// `{ type: "demande_recue", demande_id, demandeur: MembreLight }`
pub fn evt_demande_recue(demande_id: Uuid, demandeur: &MembreLight) -> serde_json::Value {
    serde_json::json!({
        "type": "demande_recue",
        "demande_id": demande_id,
        "demandeur": demandeur,
    })
}

/// `{ type: "demande_acceptee", utilisateur: MembreLight }`
pub fn evt_demande_acceptee(utilisateur: &MembreLight) -> serde_json::Value {
    serde_json::json!({
        "type": "demande_acceptee",
        "utilisateur": utilisateur,
    })
}

/// Représentation publique légère d'un membre, partagée par tous les
/// endpoints du domaine social.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MembreLight {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub slug: Option<String>,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    pub fonction: Option<String>,
    pub pays: Option<String>,
}

/// Ordre canonique d'une paire d'utilisateurs (Décision 4) :
/// le plus petit UUID en premier. Utilisé pour `amitie` et `conversation`.
pub fn paire_canonique(a: Uuid, b: Uuid) -> (Uuid, Uuid) {
    if a <= b { (a, b) } else { (b, a) }
}

// ────────────────────────────────────────────────────────────────
// Demande d'amitié
// ────────────────────────────────────────────────────────────────

/// Ligne brute social.demande_amitie
#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct DemandeAmitie {
    pub id: Uuid,
    pub demandeur_id: Uuid,
    pub destinataire_id: Uuid,
    pub statut: String,
    pub created_at: DateTime<Utc>,
    pub traite_at: Option<DateTime<Utc>>,
}

/// Réponse à la création d'une demande (FR-001).
#[derive(Debug, Serialize)]
pub struct DemandeResponse {
    pub demande_id: Uuid,
    pub statut: String,
}

/// État de la relation entre l'utilisateur courant et une cible (FR-016).
#[derive(Debug, Serialize)]
pub struct EtatRelationResponse {
    pub etat: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demande_id: Option<Uuid>,
}

/// Corps de la requête d'envoi de demande.
#[derive(Debug, Deserialize)]
pub struct CreerDemandeBody {
    pub destinataire_id: Uuid,
}

/// Corps de la requête d'états relationnels en lot (annuaire, FR-016).
#[derive(Debug, Deserialize)]
pub struct EtatsLotBody {
    pub utilisateur_ids: Vec<Uuid>,
}

/// Une demande reçue, enrichie du demandeur (US2).
#[derive(Debug, Serialize)]
pub struct DemandeRecueResponse {
    pub demande_id: Uuid,
    pub demandeur: MembreLight,
    pub created_at: DateTime<Utc>,
}

/// Une demande envoyée, enrichie du destinataire (US4).
#[derive(Debug, Serialize)]
pub struct DemandeEnvoyeeResponse {
    pub demande_id: Uuid,
    pub destinataire: MembreLight,
    pub created_at: DateTime<Utc>,
}

/// Ligne jointe demande + membre (demandeur ou destinataire selon le sens).
#[derive(Debug, FromRow)]
pub struct DemandeAvecMembreRow {
    pub demande_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub membre_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub fonction: Option<String>,
    pub pays: Option<String>,
}

impl DemandeAvecMembreRow {
    /// Extrait le MembreLight associé à la ligne.
    pub fn membre(&self) -> MembreLight {
        MembreLight {
            id: self.membre_id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            slug: self.slug.clone(),
            photo_url: self.photo_url.clone(),
            fonction: self.fonction.clone(),
            pays: self.pays.clone(),
        }
    }
}

// ────────────────────────────────────────────────────────────────
// Amitié
// ────────────────────────────────────────────────────────────────

/// Ligne brute social.amitie
#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct Amitie {
    pub id: Uuid,
    pub utilisateur_a_id: Uuid,
    pub utilisateur_b_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Un ami de l'utilisateur courant (FR-011).
#[derive(Debug, Serialize)]
pub struct AmiResponse {
    pub utilisateur: MembreLight,
    pub ami_depuis: DateTime<Utc>,
}

/// Un membre bloqué par l'utilisateur courant (FR-013).
#[derive(Debug, Serialize)]
pub struct BlocageResponse {
    pub utilisateur: MembreLight,
    pub depuis: DateTime<Utc>,
}

/// Corps de la requête de blocage (FR-013).
#[derive(Debug, Deserialize)]
pub struct BloquerBody {
    pub utilisateur_id: Uuid,
}

/// Ligne jointe membre + date de relation (amitié ou blocage selon le contexte).
/// La date est aliasée en `date_relation` dans les deux requêtes (US4).
#[derive(Debug, FromRow)]
pub struct MembreAvecDateRow {
    pub membre_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub fonction: Option<String>,
    pub pays: Option<String>,
    pub date_relation: DateTime<Utc>,
}

impl MembreAvecDateRow {
    /// Extrait le MembreLight associé à la ligne.
    pub fn membre(&self) -> MembreLight {
        MembreLight {
            id: self.membre_id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            slug: self.slug.clone(),
            photo_url: self.photo_url.clone(),
            fonction: self.fonction.clone(),
            pays: self.pays.clone(),
        }
    }
}

/// Crée l'amitié entre deux membres dans l'ordre canonique (idempotent) **et**
/// la conversation associée. Pré-créer la conversation fait coïncider « liste
/// des conversations » et « liste des amis » côté messagerie (US3), sans
/// dépendre d'un endpoint de liste d'amis dédié.
pub async fn creer_amitie(
    conn: &mut sqlx::PgConnection,
    a: Uuid,
    b: Uuid,
) -> Result<(), sqlx::Error> {
    let (min, max) = paire_canonique(a, b);
    sqlx::query(
        "INSERT INTO social.amitie (utilisateur_a_id, utilisateur_b_id)
         VALUES ($1, $2)
         ON CONFLICT (utilisateur_a_id, utilisateur_b_id) DO NOTHING",
    )
    .bind(min)
    .bind(max)
    .execute(&mut *conn)
    .await?;
    sqlx::query(
        "INSERT INTO social.conversation (utilisateur_a_id, utilisateur_b_id)
         VALUES ($1, $2)
         ON CONFLICT (utilisateur_a_id, utilisateur_b_id) DO NOTHING",
    )
    .bind(min)
    .bind(max)
    .execute(&mut *conn)
    .await?;
    Ok(())
}

// ────────────────────────────────────────────────────────────────
// Notification sociale
// ────────────────────────────────────────────────────────────────

/// Ligne jointe notification + acteur (MembreLight).
#[derive(Debug, FromRow)]
pub struct NotificationRow {
    pub id: Uuid,
    pub type_notif: String,
    pub demande_id: Option<Uuid>,
    pub lu: bool,
    pub created_at: DateTime<Utc>,
    pub acteur_id: Option<Uuid>,
    pub acteur_nom: Option<String>,
    pub acteur_prenom: Option<String>,
    pub acteur_slug: Option<String>,
    pub acteur_photo_url: Option<String>,
    pub acteur_fonction: Option<String>,
    pub acteur_pays: Option<String>,
}

/// Notification relationnelle exposée (FR-017).
#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_notif: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demande_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acteur: Option<MembreLight>,
    pub lu: bool,
    pub created_at: DateTime<Utc>,
}

impl NotificationRow {
    pub fn to_response(&self) -> NotificationResponse {
        let acteur = self.acteur_id.map(|id| MembreLight {
            id,
            nom: self.acteur_nom.clone().unwrap_or_default(),
            prenom: self.acteur_prenom.clone().unwrap_or_default(),
            slug: self.acteur_slug.clone(),
            photo_url: self.acteur_photo_url.clone(),
            fonction: self.acteur_fonction.clone(),
            pays: self.acteur_pays.clone(),
        });
        NotificationResponse {
            id: self.id,
            type_notif: self.type_notif.clone(),
            demande_id: self.demande_id,
            acteur,
            lu: self.lu,
            created_at: self.created_at,
        }
    }
}

/// Insère une notification sociale. Fonctionne avec un pool ou une transaction.
pub async fn creer_notification<'e, E>(
    executor: E,
    destinataire_id: Uuid,
    type_notif: &str,
    demande_id: Option<Uuid>,
    acteur_id: Option<Uuid>,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    sqlx::query(
        "INSERT INTO social.notification (destinataire_id, type, demande_id, acteur_id)
         VALUES ($1, $2::social.type_notification_social, $3, $4)",
    )
    .bind(destinataire_id)
    .bind(type_notif)
    .bind(demande_id)
    .bind(acteur_id)
    .execute(executor)
    .await?;
    Ok(())
}

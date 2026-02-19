use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ────────────────────────────────────────────────

/// Colonnes pour le listing admin (query SELECT principale)
pub const ADMIN_UTILISATEUR_LISTE_COLONNES: &str =
    "u.id, u.nom, u.prenom, u.email, u.photo_url,
     u.etat::text AS etat, u.email_verifie, u.derniere_connexion,
     u.created_at,
     COALESCE(
       (SELECT array_agg(r.slug ORDER BY r.slug)
        FROM iam.role r
        JOIN iam.utilisateur_role ur ON r.id = ur.role_id
        WHERE ur.utilisateur_id = u.id), '{}'
     ) AS roles";

/// Colonnes pour le detail admin
pub const ADMIN_UTILISATEUR_DETAIL_COLONNES: &str =
    "u.id, u.nom, u.prenom, u.email, u.slug, u.telephone, u.photo_url,
     u.genre::text AS genre, u.date_naissance, u.fonction, u.localite, u.ville,
     u.pays_origine_id, u.pays_residence_id, u.organisation_id,
     u.biographie, u.etat::text AS etat, u.email_verifie, u.telephone_verifie,
     u.double_facteur_active, u.documents_verifie, u.bibliotheque_humain,
     u.langue_preferee, u.derniere_connexion,
     u.created_at, u.updated_at";

// ── Colonnes de tri autorisees ──────────────────────────────────

pub const UTILISATEUR_TRI_COLONNES: &[&str] = &[
    "created_at", "updated_at", "nom", "prenom", "email", "etat", "derniere_connexion",
];

// ── Row structs (FromRow) ───────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminUtilisateurListeRow {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub etat: String,
    pub email_verifie: bool,
    pub derniere_connexion: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub roles: Vec<String>,
}

#[derive(Debug, FromRow)]
pub struct AdminUtilisateurDetailRow {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub slug: Option<String>,
    pub telephone: Option<String>,
    pub photo_url: Option<String>,
    pub genre: String,
    pub date_naissance: Option<NaiveDate>,
    pub fonction: Option<String>,
    pub localite: Option<String>,
    pub ville: Option<String>,
    pub pays_origine_id: Option<Uuid>,
    pub pays_residence_id: Option<Uuid>,
    pub organisation_id: Option<Uuid>,
    pub biographie: Option<String>,
    pub etat: String,
    pub email_verifie: bool,
    pub telephone_verifie: bool,
    pub double_facteur_active: bool,
    pub documents_verifie: bool,
    pub bibliotheque_humain: bool,
    pub langue_preferee: String,
    pub derniere_connexion: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Response DTOs (Serialize) ───────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminUtilisateurListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub etat: String,
    pub email_verifie: bool,
    pub derniere_connexion: Option<DateTime<Utc>>,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminUtilisateurDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub slug: Option<String>,
    pub telephone: Option<String>,
    pub photo_url: Option<String>,
    pub genre: String,
    pub date_naissance: Option<NaiveDate>,
    pub fonction: Option<String>,
    pub localite: Option<String>,
    pub ville: Option<String>,
    pub pays_origine: Option<String>,
    pub pays_residence: Option<String>,
    pub organisation: Option<OrganisationMinimalInfo>,
    pub biographie: Option<String>,
    pub etat: String,
    pub email_verifie: bool,
    pub telephone_verifie: bool,
    pub double_facteur_active: bool,
    pub documents_verifie: bool,
    pub bibliotheque_humain: bool,
    pub langue_preferee: String,
    pub derniere_connexion: Option<DateTime<Utc>>,
    pub roles: Vec<RoleInfo>,
    pub specialites: Vec<SpecialiteInfo>,
    pub permissions_specifiques: Vec<PermissionSpecifiqueInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct OrganisationMinimalInfo {
    pub id: Uuid,
    pub denomination: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct RoleInfo {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub attribue_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SpecialiteInfo {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PermissionSpecifiqueInfo {
    pub id: Uuid,
    pub permission_slug: String,
    pub type_ressource: String,
    pub action: String,
    pub ressource_type: String,
    pub ressource_id: Uuid,
    pub expire_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ── Request DTOs (Deserialize) ──────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerUtilisateurRequest {
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub mot_de_passe: String,
    pub telephone: Option<String>,
    pub genre: Option<String>,
    pub role_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierUtilisateurRequest {
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub genre: Option<String>,
    pub date_naissance: Option<NaiveDate>,
    pub fonction: Option<String>,
    pub localite: Option<String>,
    pub ville: Option<String>,
    pub pays_origine_id: Option<Uuid>,
    pub pays_residence_id: Option<Uuid>,
    pub organisation_id: Option<Uuid>,
    pub biographie: Option<String>,
    pub langue_preferee: Option<String>,
    pub bibliotheque_humain: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatRequest {
    pub etat: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignerRoleRequest {
    pub role_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct AssignerSpecialiteRequest {
    pub specialite_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct AjouterPermissionRequest {
    pub permission_id: Uuid,
    pub ressource_type: String,
    pub ressource_id: Uuid,
    pub expire_at: Option<DateTime<Utc>>,
}

// ── Query Params ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminUtilisateurQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub role: Option<String>,
    pub pays: Option<String>,
}

// ── Conversions ─────────────────────────────────────────────────

impl AdminUtilisateurListeRow {
    pub fn to_response(&self) -> AdminUtilisateurListeResponse {
        AdminUtilisateurListeResponse {
            id: self.id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            email: self.email.clone(),
            photo_url: self.photo_url.clone(),
            etat: self.etat.clone(),
            email_verifie: self.email_verifie,
            derniere_connexion: self.derniere_connexion,
            roles: self.roles.clone(),
            created_at: self.created_at,
        }
    }
}

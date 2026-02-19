use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ════════════════════════════════════════════════════════════════
// FactCheck
// ════════════════════════════════════════════════════════════════

pub const ADMIN_FACTCHECK_LISTE_COLONNES: &str =
    "f.id, f.contenu, f.verdict, f.etat,
     f.nombre_likes, f.nombre_dislikes, f.created_at,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const ADMIN_FACTCHECK_DETAIL_COLONNES: &str =
    "f.id, f.contenu, f.source_originale, f.verdict,
     f.image_couverture_url, f.couleur_fond,
     f.etat, f.nombre_likes, f.nombre_dislikes,
     f.pays_id, f.cree_par, f.created_at, f.updated_at,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const FACTCHECK_TRI_COLONNES: &[&str] = &[
    "created_at", "nombre_likes", "nombre_dislikes",
];

pub const FACTCHECK_JOINS: &str =
    "LEFT JOIN shared.pays p ON p.id = f.pays_id
     JOIN iam.utilisateur u ON u.id = f.cree_par";

#[derive(Debug, Serialize, FromRow)]
pub struct AdminFactcheckListeResponse {
    pub id: Uuid,
    pub contenu: String,
    pub verdict: Option<String>,
    pub etat: String,
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    pub created_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

#[derive(Debug, FromRow)]
pub struct AdminFactcheckDetailRow {
    pub id: Uuid,
    pub contenu: String,
    pub source_originale: Option<String>,
    pub verdict: Option<String>,
    pub image_couverture_url: Option<String>,
    pub couleur_fond: Option<String>,
    pub etat: String,
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    pub pays_id: Option<Uuid>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

#[derive(Debug, Serialize)]
pub struct AdminFactcheckDetailResponse {
    pub id: Uuid,
    pub contenu: String,
    pub source_originale: Option<String>,
    pub verdict: Option<String>,
    pub image_couverture_url: Option<String>,
    pub couleur_fond: Option<String>,
    pub etat: String,
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    pub pays_id: Option<Uuid>,
    pub cree_par: Uuid,
    pub cree_par_nom: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub nombre_commentaires: i64,
}

impl AdminFactcheckDetailRow {
    pub fn to_response(&self, nombre_commentaires: i64) -> AdminFactcheckDetailResponse {
        AdminFactcheckDetailResponse {
            id: self.id,
            contenu: self.contenu.clone(),
            source_originale: self.source_originale.clone(),
            verdict: self.verdict.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            couleur_fond: self.couleur_fond.clone(),
            etat: self.etat.clone(),
            nombre_likes: self.nombre_likes,
            nombre_dislikes: self.nombre_dislikes,
            pays_id: self.pays_id,
            cree_par: self.cree_par,
            cree_par_nom: format!("{} {}", self.auteur_prenom, self.auteur_nom),
            created_at: self.created_at,
            updated_at: self.updated_at,
            pays_nom: self.pays_nom.clone(),
            nombre_commentaires,
        }
    }
}

// ── FactCheck Commentaire ────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminFactcheckCommentaireRow {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub contenu: String,
    pub type_commentaire: String,
    pub cree_par: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub nombre_likes: i32,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct AdminFactcheckCommentaireResponse {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub contenu: String,
    pub type_commentaire: String,
    pub cree_par: Uuid,
    pub auteur_nom: String,
    pub nombre_likes: i32,
    pub created_at: DateTime<Utc>,
    pub supprime: bool,
    pub enfants: Vec<AdminFactcheckCommentaireResponse>,
}

// ── FactCheck Reactions ──────────────────────────────────────
#[derive(Debug, Serialize)]
pub struct AdminFactcheckReactionStats {
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
}

// ── FactCheck Requests ───────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct CreerFactcheckRequest {
    pub contenu: String,
    pub source_originale: Option<String>,
    pub verdict: Option<String>,
    pub image_couverture_url: Option<String>,
    pub couleur_fond: Option<String>,
    pub pays_id: Option<Uuid>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierFactcheckRequest {
    pub contenu: Option<String>,
    pub source_originale: Option<String>,
    pub verdict: Option<String>,
    pub image_couverture_url: Option<String>,
    pub couleur_fond: Option<String>,
    pub pays_id: Option<Uuid>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminFactcheckQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub verdict: Option<String>,
    pub pays_id: Option<Uuid>,
    pub etat: Option<String>,
}

// ════════════════════════════════════════════════════════════════
// Bad Habits (Mauvaises pratiques)
// ════════════════════════════════════════════════════════════════

pub const ADMIN_BAD_HABIT_LISTE_COLONNES: &str =
    "b.id, b.titre, b.slug, b.categorie_probleme,
     b.gravite::text AS gravite, b.etat, b.nombre_soutiens,
     b.publication_anonyme, b.created_at,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const ADMIN_BAD_HABIT_DETAIL_COLONNES: &str =
    "b.id, b.titre, b.slug, b.description_generale, b.details_problematique,
     b.categorie_probleme, b.categorie_probleme_detail,
     b.gravite::text AS gravite, b.preuves_temoignages, b.solutions_proposees,
     b.publication_anonyme, b.geolocalisation_autorisee,
     b.longitude::float8 AS longitude, b.latitude::float8 AS latitude,
     b.region, b.ville_quartier_zone,
     b.etat, b.nombre_soutiens,
     b.pays_id, b.cree_par, b.created_at, b.updated_at,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const BAD_HABIT_TRI_COLONNES: &[&str] = &[
    "created_at", "nombre_soutiens", "titre",
];

pub const BAD_HABIT_JOINS: &str =
    "LEFT JOIN shared.pays p ON p.id = b.pays_id
     JOIN iam.utilisateur u ON u.id = b.cree_par";

#[derive(Debug, Serialize, FromRow)]
pub struct AdminBadHabitListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub categorie_probleme: String,
    pub gravite: String,
    pub etat: String,
    pub nombre_soutiens: i32,
    pub publication_anonyme: bool,
    pub created_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminBadHabitDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description_generale: String,
    pub details_problematique: String,
    pub categorie_probleme: String,
    pub categorie_probleme_detail: Option<String>,
    pub gravite: String,
    pub preuves_temoignages: Option<String>,
    pub solutions_proposees: Option<String>,
    pub publication_anonyme: bool,
    pub geolocalisation_autorisee: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub region: Option<String>,
    pub ville_quartier_zone: Option<String>,
    pub etat: String,
    pub nombre_soutiens: i32,
    pub pays_id: Option<Uuid>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminBadHabitMedia {
    pub id: Uuid,
    pub media_url: String,
    pub type_mime: Option<String>,
    pub ordre: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerBadHabitRequest {
    pub titre: String,
    pub description_generale: String,
    pub details_problematique: String,
    pub categorie_probleme: String,
    pub categorie_probleme_detail: Option<String>,
    pub gravite: Option<String>,
    pub preuves_temoignages: Option<String>,
    pub solutions_proposees: Option<String>,
    pub publication_anonyme: Option<bool>,
    pub geolocalisation_autorisee: Option<bool>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub pays_id: Option<Uuid>,
    pub region: Option<String>,
    pub ville_quartier_zone: Option<String>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierBadHabitRequest {
    pub titre: Option<String>,
    pub description_generale: Option<String>,
    pub details_problematique: Option<String>,
    pub categorie_probleme: Option<String>,
    pub categorie_probleme_detail: Option<String>,
    pub gravite: Option<String>,
    pub preuves_temoignages: Option<String>,
    pub solutions_proposees: Option<String>,
    pub publication_anonyme: Option<bool>,
    pub geolocalisation_autorisee: Option<bool>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub pays_id: Option<Uuid>,
    pub region: Option<String>,
    pub ville_quartier_zone: Option<String>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AjouterMediaBadHabitRequest {
    pub media_url: String,
    pub type_mime: Option<String>,
    pub ordre: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AdminBadHabitQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub categorie_probleme: Option<String>,
    pub gravite: Option<String>,
    pub pays_id: Option<Uuid>,
    pub etat: Option<String>,
}

// ════════════════════════════════════════════════════════════════
// Idea Forces (Idees forces)
// ════════════════════════════════════════════════════════════════

pub const ADMIN_IDEA_FORCE_LISTE_COLONNES: &str =
    "i.id, i.titre, i.slug, i.categorie_proposition,
     i.urgence::text AS urgence, i.etat, i.nombre_soutiens,
     i.created_at,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const ADMIN_IDEA_FORCE_DETAIL_COLONNES: &str =
    "i.id, i.titre, i.slug, i.description_generale, i.details_proposition,
     i.categorie_proposition, i.categorie_proposition_detail,
     i.urgence::text AS urgence, i.plan_implementation,
     i.ressources_necessaires, i.impact_attendu,
     i.etat, i.nombre_soutiens,
     i.region, i.ville_quartier_zone,
     i.pays_id, i.cree_par, i.created_at, i.updated_at,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const IDEA_FORCE_TRI_COLONNES: &[&str] = &[
    "created_at", "nombre_soutiens", "titre",
];

pub const IDEA_FORCE_JOINS: &str =
    "LEFT JOIN shared.pays p ON p.id = i.pays_id
     JOIN iam.utilisateur u ON u.id = i.cree_par";

#[derive(Debug, Serialize, FromRow)]
pub struct AdminIdeaForceListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub categorie_proposition: String,
    pub urgence: String,
    pub etat: String,
    pub nombre_soutiens: i32,
    pub created_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminIdeaForceDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description_generale: String,
    pub details_proposition: String,
    pub categorie_proposition: String,
    pub categorie_proposition_detail: Option<String>,
    pub urgence: String,
    pub plan_implementation: Option<String>,
    pub ressources_necessaires: Option<String>,
    pub impact_attendu: Option<String>,
    pub etat: String,
    pub nombre_soutiens: i32,
    pub region: Option<String>,
    pub ville_quartier_zone: Option<String>,
    pub pays_id: Option<Uuid>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminIdeaForceMedia {
    pub id: Uuid,
    pub media_url: String,
    pub type_mime: Option<String>,
    pub ordre: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerIdeaForceRequest {
    pub titre: String,
    pub description_generale: String,
    pub details_proposition: String,
    pub categorie_proposition: String,
    pub categorie_proposition_detail: Option<String>,
    pub urgence: Option<String>,
    pub plan_implementation: Option<String>,
    pub ressources_necessaires: Option<String>,
    pub impact_attendu: Option<String>,
    pub pays_id: Option<Uuid>,
    pub region: Option<String>,
    pub ville_quartier_zone: Option<String>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierIdeaForceRequest {
    pub titre: Option<String>,
    pub description_generale: Option<String>,
    pub details_proposition: Option<String>,
    pub categorie_proposition: Option<String>,
    pub categorie_proposition_detail: Option<String>,
    pub urgence: Option<String>,
    pub plan_implementation: Option<String>,
    pub ressources_necessaires: Option<String>,
    pub impact_attendu: Option<String>,
    pub pays_id: Option<Uuid>,
    pub region: Option<String>,
    pub ville_quartier_zone: Option<String>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AjouterMediaIdeaForceRequest {
    pub media_url: String,
    pub type_mime: Option<String>,
    pub ordre: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AdminIdeaForceQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub categorie_proposition: Option<String>,
    pub urgence: Option<String>,
    pub pays_id: Option<Uuid>,
    pub etat: Option<String>,
}

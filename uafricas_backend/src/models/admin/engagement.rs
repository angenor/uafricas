//! DTO admin du système d'engagement (barème + journal).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Règle de barème (édition admin).
///
/// `instrumentee` dit si le **code** émet réellement ce `type_action` (catalogue
/// const de `handlers/admin/engagement.rs`) et `nombre_mouvements` combien de fois
/// la règle a effectivement crédité : une règle active, instrumentée et à 0
/// mouvement est un signal de branchement cassé (R3).
#[derive(Serialize, FromRow)]
pub struct RegleAdmin {
    pub id: Uuid,
    pub type_action: String,
    pub libelle: String,
    pub points: i32,
    pub reputation_delta: i32,
    pub plafond_journalier: Option<i32>,
    pub plafond_mensuel: Option<i32>,
    pub seuil_declencheur: Option<i32>,
    pub categorie_id: Option<Uuid>,
    pub categorie_code: Option<String>,
    pub categorie_libelle: Option<String>,
    pub actif: bool,
    #[sqlx(default)]
    pub instrumentee: bool,
    pub nombre_mouvements: i64,
}

#[derive(Deserialize)]
pub struct CreerRegleRequest {
    pub type_action: String,
    pub libelle: String,
    pub points: i32,
    #[serde(default)]
    pub reputation_delta: i32,
    pub plafond_journalier: Option<i32>,
    pub plafond_mensuel: Option<i32>,
    pub seuil_declencheur: Option<i32>,
    pub categorie_id: Option<Uuid>,
    pub actif: Option<bool>,
}

#[derive(Deserialize)]
pub struct ModifierRegleRequest {
    pub libelle: Option<String>,
    pub points: Option<i32>,
    pub reputation_delta: Option<i32>,
    pub plafond_journalier: Option<Option<i32>>,
    pub plafond_mensuel: Option<Option<i32>>,
    pub seuil_declencheur: Option<Option<i32>>,
    pub categorie_id: Option<Option<Uuid>>,
    pub actif: Option<bool>,
}

/// Une action **réellement instrumentée par le code** (catalogue const, R3).
/// Ce n'est pas une table : le code reste seul juge de ce qu'il émet.
#[derive(Serialize)]
pub struct ActionDisponible {
    pub type_action: &'static str,
    pub libelle_defaut: &'static str,
    pub types_objet: &'static [&'static str],
    pub module: &'static str,
    pub regle_existante: bool,
}

/// Catégorie de ventilation (édition admin).
#[derive(Serialize, FromRow)]
pub struct CategorieAdmin {
    pub id: Uuid,
    pub code: String,
    pub libelle: String,
    pub description: Option<String>,
    pub ordre: i16,
    pub couleur: Option<String>,
    pub icone: Option<String>,
    pub actif: bool,
    pub nombre_regles: i64,
    pub nombre_mouvements: i64,
}

#[derive(Deserialize)]
pub struct CreerCategorieRequest {
    pub code: String,
    pub libelle: String,
    pub description: Option<String>,
    pub ordre: Option<i16>,
    pub couleur: Option<String>,
    pub icone: Option<String>,
    pub actif: Option<bool>,
}

/// `code` absent : il est **immuable** après création (le front l'utilise pour
/// ses icônes ; le libellé, lui, est libre).
#[derive(Deserialize)]
pub struct ModifierCategorieRequest {
    pub libelle: Option<String>,
    pub description: Option<Option<String>>,
    pub ordre: Option<i16>,
    pub couleur: Option<Option<String>>,
    pub icone: Option<Option<String>>,
    pub actif: Option<bool>,
}

/// Palier de popularité. `type_objet` nul = palier **global** ; sinon le palier
/// est restreint à une famille de contenus et **remplace** les paliers globaux
/// pour cette famille (règle de substitution, R4).
#[derive(Serialize, FromRow)]
pub struct PalierAdmin {
    pub id: Uuid,
    pub seuil_likes: i32,
    pub points: i32,
    pub type_objet: Option<String>,
    pub actif: bool,
}

#[derive(Deserialize)]
pub struct CreerPalierRequest {
    pub seuil_likes: i32,
    pub points: i32,
    pub type_objet: Option<String>,
}

#[derive(Deserialize)]
pub struct ModifierPalierRequest {
    pub points: Option<i32>,
    pub type_objet: Option<Option<String>>,
    pub actif: Option<bool>,
}

/// Seuil de niveau.
#[derive(Serialize, FromRow)]
pub struct NiveauAdmin {
    pub id: Uuid,
    pub code: String,
    pub libelle: String,
    pub seuil_min: i32,
    pub ordre: i16,
    pub badge_couleur: Option<String>,
    pub badge_icone: Option<String>,
}

#[derive(Deserialize)]
pub struct CreerNiveauRequest {
    pub code: String,
    pub libelle: String,
    pub seuil_min: i32,
    pub badge_couleur: Option<String>,
    pub badge_icone: Option<String>,
}

/// `code` absent : il est **immuable** (`compte.niveau_code` le référence par
/// valeur, sans FK, le renommer orphelinerait tous les comptes concernés).
#[derive(Deserialize)]
pub struct ModifierNiveauRequest {
    pub libelle: Option<String>,
    pub seuil_min: Option<i32>,
    pub badge_couleur: Option<String>,
    pub badge_icone: Option<String>,
}

/// Réponse des mutations de niveau : le nombre de comptes rebasculés est affiché
/// à l'administrateur, qui voit ainsi l'effet réel de son geste (R5).
#[derive(Serialize)]
pub struct NiveauxRecalculesResponse {
    pub niveaux: Vec<NiveauAdmin>,
    pub comptes_recalcules: u64,
}

/// Entrée du journal global (enrichie du nom du membre).
#[derive(Serialize, FromRow)]
pub struct JournalAdminRow {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub utilisateur_nom: Option<String>,
    pub type_action: String,
    pub categorie_code: Option<String>,
    pub categorie_libelle: Option<String>,
    pub type_objet: Option<String>,
    pub objet_id: Option<Uuid>,
    pub points: i32,
    pub reputation_delta: i32,
    pub solde_apres: i32,
    pub plafond_atteint: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct JournalAdminPage {
    pub elements: Vec<JournalAdminRow>,
    pub total: i64,
    pub page: i64,
    pub taille: i64,
}

#[derive(Deserialize)]
pub struct JournalAdminParams {
    pub utilisateur_id: Option<Uuid>,
    pub type_action: Option<String>,
    /// Code de catégorie (`engagement.categorie_points.code`).
    pub categorie: Option<String>,
    pub depuis: Option<String>,
    pub jusqu_a: Option<String>,
    pub page: Option<i64>,
    pub taille: Option<i64>,
}

#[derive(Deserialize)]
pub struct AjustementRequest {
    pub utilisateur_id: Uuid,
    pub points: i32,
    #[serde(default)]
    pub reputation_delta: i32,
    pub motif: Option<String>,
}

// ── Badges ──────────────────────────────────────────────────────────────────

/// Badge du catalogue (édition admin). `type_condition` est lu en texte : l'enum
/// PostgreSQL à 5 valeurs n'a pas besoin d'un type Rust dédié.
#[derive(Serialize, FromRow)]
pub struct BadgeAdmin {
    pub id: Uuid,
    pub code: String,
    pub libelle: String,
    pub description: String,
    pub couleur: Option<String>,
    pub icone: Option<String>,
    pub manuel: bool,
    pub type_condition: Option<String>,
    pub parametre_action: Option<String>,
    pub parametre_categorie_id: Option<Uuid>,
    pub parametre_niveau_code: Option<String>,
    pub seuil: Option<i32>,
    pub ordre: i16,
    pub actif: bool,
    pub nombre_detenteurs: i64,
}

#[derive(Deserialize)]
pub struct CreerBadgeRequest {
    pub code: String,
    pub libelle: String,
    pub description: String,
    pub couleur: Option<String>,
    pub icone: Option<String>,
    #[serde(default)]
    pub manuel: bool,
    pub type_condition: Option<String>,
    pub parametre_action: Option<String>,
    pub parametre_categorie_id: Option<Uuid>,
    pub parametre_niveau_code: Option<String>,
    pub seuil: Option<i32>,
    pub ordre: Option<i16>,
    pub actif: Option<bool>,
}

/// `code` absent : immuable après création.
#[derive(Deserialize)]
pub struct ModifierBadgeRequest {
    pub libelle: Option<String>,
    pub description: Option<String>,
    pub couleur: Option<Option<String>>,
    pub icone: Option<Option<String>>,
    pub manuel: Option<bool>,
    pub type_condition: Option<Option<String>>,
    pub parametre_action: Option<Option<String>>,
    pub parametre_categorie_id: Option<Option<Uuid>>,
    pub parametre_niveau_code: Option<Option<String>>,
    pub seuil: Option<Option<i32>>,
    pub ordre: Option<i16>,
    pub actif: Option<bool>,
}

/// Attribution manuelle d'un badge à un membre. Le motif est **tracé dans
/// l'audit** : un badge donné à la main doit rester explicable des mois après.
#[derive(Deserialize)]
pub struct AttribuerBadgeRequest {
    pub utilisateur_id: Uuid,
    pub motif: Option<String>,
}

/// Marque une contribution comme « mise en avant » (déclenche le +5 à l'auteur).
#[derive(Deserialize)]
pub struct MiseEnAvantRequest {
    pub type_objet: String,
    pub objet_id: Uuid,
}

/// État de mise en avant d'une contribution (pour le bouton admin).
#[derive(Serialize)]
pub struct MiseEnAvantEtat {
    pub mis_en_avant: bool,
}

//! DTO publics du système d'engagement (lecture). Reflètent le schéma
//! `engagement` (Principe III).

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// Niveau/statut d'un membre (issu de `engagement.niveau`).
#[derive(Serialize, FromRow)]
pub struct NiveauInfo {
    pub code: String,
    pub libelle: String,
    pub seuil_min: i32,
    pub badge_couleur: Option<String>,
    pub badge_icone: Option<String>,
}

/// Prochain palier de niveau à atteindre (null si niveau maximal).
#[derive(Serialize)]
pub struct ProchainNiveau {
    pub code: String,
    pub libelle: String,
    pub seuil_min: i32,
    pub points_restants: i32,
}

/// Ligne brute du compte d'engagement.
#[derive(FromRow)]
pub struct CompteRow {
    pub solde_points: i32,
    pub solde_points_mensuel: i32,
    pub reputation: i32,
    pub niveau_code: String,
    pub dernier_mouvement_at: Option<DateTime<Utc>>,
}

/// Réponse « Mon compte d'engagement ».
#[derive(Serialize)]
pub struct CompteResponse {
    pub solde_points: i32,
    pub solde_points_mensuel: i32,
    pub reputation: i32,
    pub niveau: NiveauInfo,
    pub prochain_niveau: Option<ProchainNiveau>,
    pub dernier_mouvement_at: Option<DateTime<Utc>>,
}

/// Un mouvement de points (entrée de journal).
///
/// Asymétrie assumée (contrats/api-membre.md) : `libelle` est relu dans la
/// **règle courante** (renommer une règle renomme l'action dans tout l'historique
/// affiché), tandis que `categorie_code` / `categorie_libelle` viennent de la
/// catégorie **figée à l'écriture** (une re-catégorisation ne réécrit pas le
/// passé — R1).
#[derive(Serialize, FromRow)]
pub struct MouvementResponse {
    pub id: Uuid,
    pub type_action: String,
    pub libelle: Option<String>,
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

/// Page du journal des points.
#[derive(Serialize)]
pub struct JournalPage {
    pub elements: Vec<MouvementResponse>,
    pub total: i64,
    pub page: i64,
    pub taille: i64,
}

/// Une ligne de la ventilation des points par catégorie.
/// `code` est `NULL` pour le regroupement « Autres » (mouvements sans catégorie).
#[derive(Serialize, FromRow)]
pub struct CategorieVentilation {
    pub code: Option<String>,
    pub libelle: String,
    pub couleur: Option<String>,
    pub icone: Option<String>,
    pub ordre: i16,
    pub points: i64,
    pub nombre_mouvements: i64,
}

/// Réponse « Mes catégories » — ventilation des points gagnés.
///
/// `total_gagne` (cumul du journal) peut **dépasser** `solde_points` (solde
/// courant) à cause du plancher 0 appliqué par `appliquer` : les deux notions
/// sont exposées séparément pour rendre l'écart compréhensible (R2, SC-005).
#[derive(Serialize)]
pub struct VentilationResponse {
    pub solde_points: i32,
    pub total_gagne: i64,
    pub categories: Vec<CategorieVentilation>,
}

/// Un badge **obtenu** par un membre. Sert l'espace membre et le profil public.
#[derive(Serialize, FromRow)]
pub struct BadgeObtenuResponse {
    pub code: String,
    pub libelle: String,
    pub description: String,
    pub couleur: Option<String>,
    pub icone: Option<String>,
    pub origine: String,
    pub obtenu_at: DateTime<Utc>,
}

/// Un badge **à débloquer** — jamais renvoyé sur le profil public.
///
/// `progression_actuelle` / `progression_cible` sont nuls quand la condition ne
/// s'exprime pas comme « N sur M » (le niveau se compare sur un ordre) : l'UI
/// n'affiche alors aucune barre plutôt qu'une barre trompeuse.
#[derive(Serialize)]
pub struct BadgeADebloquerResponse {
    pub code: String,
    pub libelle: String,
    pub description: String,
    pub couleur: Option<String>,
    pub icone: Option<String>,
    pub progression_actuelle: Option<i64>,
    pub progression_cible: Option<i64>,
}

/// Réponse « Mes badges » : obtenus + catalogue restant à débloquer.
#[derive(Serialize)]
pub struct MesBadgesResponse {
    pub obtenus: Vec<BadgeObtenuResponse>,
    pub a_debloquer: Vec<BadgeADebloquerResponse>,
}

/// Ligne brute d'un badge non encore obtenu (avant calcul de la progression).
#[derive(FromRow)]
pub struct BadgeCatalogueRow {
    pub id: Uuid,
    pub code: String,
    pub libelle: String,
    pub description: String,
    pub couleur: Option<String>,
    pub icone: Option<String>,
}

/// Une règle du barème, telle qu'exposée publiquement (état vide pédagogique).
/// Source unique des libellés du barème côté front (FR-016) : aucun montant ni
/// libellé n'est écrit en dur dans le frontend.
#[derive(Serialize, FromRow)]
pub struct ActionRecompensee {
    pub type_action: String,
    pub libelle: String,
    pub points: i32,
    pub reputation_delta: i32,
    pub categorie_code: Option<String>,
    pub categorie_libelle: Option<String>,
    pub categorie_icone: Option<String>,
    pub plafond_journalier: Option<i32>,
    pub seuil_declencheur: Option<i32>,
}

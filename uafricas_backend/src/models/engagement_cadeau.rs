//! Modèles du domaine « cadeaux virtuels » (feature 008).
//!
//! **Règle transversale** : les montants circulent en **unité entière de la
//! devise** (`i32`), jamais en flottant. Le franc CFA n'a pas de subdivision, et
//! un `NUMERIC(12,2)` imposerait un flottant côté TypeScript avec un risque de
//! dérive de sérialisation entre les deux langages. Le formatage monétaire
//! (séparateurs, symbole) est **exclusivement frontal** : deux représentations
//! d'un même montant finiraient par diverger.
//!
//! **Aucun montant, aucun bénéficiaire n'est accepté du client.** Les payloads
//! ci-dessous n'expriment que *quel cadeau* et *sur quoi* ; le prix, les points,
//! le taux et le bénéficiaire sont résolus côté serveur.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ════════════════════════════════════════════════════════════════════════════
// COLONNES
// ════════════════════════════════════════════════════════════════════════════

pub const COLONNES_CADEAU: &str =
    "id, code, libelle, description, icone, couleur, prix, points, ordre, actif, \
     created_at, updated_at";

pub const COLONNES_TRANSACTION: &str =
    "id, offreur_id, beneficiaire_id, cadeau_id, type_objet, objet_id, mode::text AS mode, \
     montant, points, taux_commission, part_beneficiaire, part_plateforme, \
     etat::text AS etat, simule, reference_paiement, message, created_at, finalise_at";

// ════════════════════════════════════════════════════════════════════════════
// LIGNES BRUTES
// ════════════════════════════════════════════════════════════════════════════

/// Un cadeau du catalogue.
#[derive(Serialize, FromRow)]
pub struct Cadeau {
    pub id: Uuid,
    pub code: String,
    pub libelle: String,
    pub description: Option<String>,
    pub icone: Option<String>,
    pub couleur: Option<String>,
    pub prix: i32,
    pub points: i32,
    pub ordre: i16,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Une transaction du journal comptable.
///
/// `mode` et `etat` sont lus en `text` : les enums PostgreSQL n'ont que deux et
/// cinq valeurs, consommées par de simples comparaisons, un type Rust dédié
/// n'apporterait rien qu'un `CHECK` ne garantisse déjà.
#[derive(Serialize, FromRow)]
pub struct TransactionCadeau {
    pub id: Uuid,
    pub offreur_id: Uuid,
    pub beneficiaire_id: Uuid,
    pub cadeau_id: Uuid,
    pub type_objet: String,
    pub objet_id: Uuid,
    pub mode: String,
    pub montant: i32,
    pub points: i32,
    pub taux_commission: i16,
    pub part_beneficiaire: i32,
    pub part_plateforme: i32,
    pub etat: String,
    pub simule: bool,
    pub reference_paiement: String,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub finalise_at: Option<DateTime<Utc>>,
}

/// Cagnotte de soutien d'un membre.
///
/// Les deux lectures actuelles ne sélectionnent que deux colonnes et se
/// contentent d'un tuple ; la struct complète existe pour le jour où le
/// versement arrivera : c'est-à-dire dès que `montant_verse` cessera d'être
/// constant.
#[allow(dead_code)]
#[derive(Serialize, FromRow)]
pub struct Cagnotte {
    pub utilisateur_id: Uuid,
    pub montant_cumule: i32,
    pub montant_verse: i32,
    pub updated_at: DateTime<Utc>,
}

/// Paramètres de monétisation (singleton).
#[derive(Serialize, FromRow)]
pub struct ParametreMonetisation {
    pub taux_commission: i16,
    pub devise: String,
    pub paiement_reel_actif: bool,
    pub updated_at: DateTime<Utc>,
}

// ════════════════════════════════════════════════════════════════════════════
// DTO DE RÉPONSE : CÔTÉ MEMBRE
// ════════════════════════════════════════════════════════════════════════════

/// Un cadeau tel que présenté au membre (sans les dates de gestion).
#[derive(Serialize, FromRow)]
pub struct CadeauPublic {
    pub id: Uuid,
    pub code: String,
    pub libelle: String,
    pub description: Option<String>,
    pub icone: Option<String>,
    pub couleur: Option<String>,
    pub prix: i32,
    pub points: i32,
    pub ordre: i16,
}

/// `GET /cadeaux` : catalogue + contexte de monétisation.
///
/// `paiement_simule` vaut `NOT paiement_reel_actif` : c'est lui qui pilote le
/// bandeau d'avertissement de phase de test (FR-020a).
#[derive(Serialize)]
pub struct CatalogueResponse {
    pub devise: String,
    pub taux_commission: i16,
    pub paiement_simule: bool,
    pub cadeaux: Vec<CadeauPublic>,
}

/// Identité minimale d'un membre, telle qu'exposée publiquement.
#[derive(Serialize, FromRow)]
pub struct MembreBref {
    pub id: Uuid,
    pub nom_affiche: String,
}

/// `POST /cadeaux/envoyer` : intention de paiement créée.
#[derive(Serialize)]
pub struct IntentionResponse {
    pub transaction_id: Uuid,
    pub reference_paiement: String,
    pub etat: String,
    pub montant: i32,
    pub points: i32,
    pub part_beneficiaire: i32,
    pub part_plateforme: i32,
    pub beneficiaire: MembreBref,
    pub simule: bool,
    pub expire_at: DateTime<Utc>,
}

/// `POST /paiements/{reference}/confirmer`, issue du paiement.
///
/// `points_credites` vaut 0 quand la règle `cadeau_recu` est désactivée : la
/// transaction et la répartition sont journalisées, mais aucun point n'est
/// attribué (FR-020, scénario 8).
#[derive(Serialize)]
pub struct ConfirmationResponse {
    pub transaction_id: Uuid,
    pub etat: String,
    pub points_credites: i32,
    pub beneficiaire: MembreBref,
}

/// Une ligne du résumé « quels cadeaux ce contenu a-t-il reçus ».
#[derive(Serialize, FromRow)]
pub struct ResumeCadeau {
    pub code: String,
    pub libelle: String,
    pub icone: Option<String>,
    pub couleur: Option<String>,
    pub nombre: i64,
}

/// Un des derniers cadeaux offerts sur un contenu. **Aucun montant** (FR-027).
#[derive(Serialize)]
pub struct CadeauOffertPublic {
    pub offreur: MembreBref,
    pub cadeau: CadeauBref,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Identité minimale d'un cadeau.
#[derive(Serialize, FromRow)]
pub struct CadeauBref {
    pub code: String,
    pub libelle: String,
    pub icone: Option<String>,
    pub couleur: Option<String>,
}

/// `GET /cadeaux/{type_objet}/{objet_id}`, cadeaux reçus par un contenu.
#[derive(Serialize)]
pub struct CadeauxContenuResponse {
    pub total: i64,
    pub resume: Vec<ResumeCadeau>,
    pub derniers: Vec<CadeauOffertPublic>,
}

/// Une ligne de « mes cadeaux ».
///
/// `montant` n'est renseigné que sur le sens `offerts` : l'offreur a le droit de
/// savoir ce qu'il a dépensé, le bénéficiaire n'a pas à voir le prix ligne à
/// ligne : il ne connaît que le cumul de sa cagnotte (FR-027).
#[derive(Serialize)]
pub struct MonCadeauResponse {
    pub id: Uuid,
    pub cadeau: CadeauBref,
    pub contrepartie: MembreBref,
    pub type_objet: String,
    pub objet_id: Uuid,
    pub titre_cible: Option<String>,
    pub points: i32,
    pub mode: String,
    pub montant: Option<i32>,
    pub message: Option<String>,
    pub simule: bool,
    pub created_at: DateTime<Utc>,
}

/// Page paginée de « mes cadeaux ».
#[derive(Serialize)]
pub struct MesCadeauxPage {
    pub elements: Vec<MonCadeauResponse>,
    pub total: i64,
    pub page: i64,
    pub taille: i64,
}

/// `GET /ma-cagnotte`.
///
/// `part_simulee` est la fraction issue de transactions simulées, c'est-à-dire
/// exactement ce que la purge de fin de phase de test retirera. L'exposer au
/// membre est ce qui empêche la purge d'être une mauvaise surprise.
#[derive(Serialize)]
pub struct CagnotteResponse {
    pub montant_cumule: i32,
    pub montant_verse: i32,
    pub devise: String,
    pub versement_disponible: bool,
    pub part_simulee: i32,
}

// ════════════════════════════════════════════════════════════════════════════
// DTO DE RÉPONSE : CÔTÉ ADMINISTRATION
// ════════════════════════════════════════════════════════════════════════════

/// Un cadeau du catalogue enrichi de son usage réel.
///
/// `nombre_envois` conditionne l'affichage du bouton de suppression : au-delà de
/// zéro, seule la désactivation reste possible (FR-028).
#[derive(Serialize, FromRow)]
pub struct CadeauAdmin {
    pub id: Uuid,
    pub code: String,
    pub libelle: String,
    pub description: Option<String>,
    pub icone: Option<String>,
    pub couleur: Option<String>,
    pub prix: i32,
    pub points: i32,
    pub ordre: i16,
    pub actif: bool,
    pub nombre_envois: i64,
    pub montant_collecte: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Cible d'une transaction, telle qu'affichée dans le journal.
/// `titre` est `null` pour un cadeau offert depuis un profil.
#[derive(Serialize)]
pub struct CibleTransaction {
    pub type_objet: String,
    pub objet_id: Uuid,
    pub titre: Option<String>,
}

/// Une ligne du journal comptable d'administration.
#[derive(Serialize)]
pub struct LigneJournalAdmin {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub finalise_at: Option<DateTime<Utc>>,
    pub offreur: MembreBref,
    pub beneficiaire: MembreBref,
    pub cible: CibleTransaction,
    pub cadeau: CadeauBref,
    pub mode: String,
    pub montant: i32,
    pub points: i32,
    pub taux_commission: i16,
    pub part_beneficiaire: i32,
    pub part_plateforme: i32,
    pub etat: String,
    pub simule: bool,
    pub reference_paiement: String,
}

/// Totaux du journal, calculés **sur le filtre courant** et non sur la page.
///
/// Invariant vérifiable en recette :
/// `recettes_plateforme + cagnottes_dues = montant_total` (SC-009).
#[derive(Serialize, FromRow)]
pub struct TotauxJournal {
    pub montant_total: i64,
    pub recettes_plateforme: i64,
    pub cagnottes_dues: i64,
    pub nombre_abouti: i64,
    pub nombre_simule: i64,
}

/// `GET /admin/engagement/transactions`.
#[derive(Serialize)]
pub struct JournalAdminPage {
    pub elements: Vec<LigneJournalAdmin>,
    pub pagination: PaginationInfo,
    pub totaux: TotauxJournal,
}

#[derive(Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub taille: i64,
    pub total: i64,
}

/// `POST /admin/engagement/purger-phase-test`, décomptes de la purge.
#[derive(Serialize)]
pub struct ResultatPurge {
    pub transactions_purgees: i64,
    pub mouvements_supprimes: i64,
    pub comptes_recalcules: i64,
    pub montant_cagnottes_annule: i64,
}

// ════════════════════════════════════════════════════════════════════════════
// PAYLOADS DE REQUÊTE
// ════════════════════════════════════════════════════════════════════════════

/// Cible d'un cadeau. **Une seule forme**, y compris pour un profil
/// (`type_objet = "profil"`, `objet_id = utilisateur_id`), par cohérence avec le
/// partage de profil déjà livré. Deux représentations du même objet dans un même
/// schéma finiraient par diverger dans les requêtes d'agrégation.
#[derive(Deserialize)]
pub struct CibleCadeau {
    pub type_objet: String,
    pub objet_id: Uuid,
}

/// `POST /cadeaux/envoyer`. Ne porte **ni montant, ni points, ni bénéficiaire**.
#[derive(Deserialize)]
pub struct EnvoyerCadeauRequest {
    pub cadeau_id: Uuid,
    pub mode: String,
    pub cible: CibleCadeau,
    pub message: Option<String>,
}

/// `POST /paiements/{reference}/confirmer`.
/// `aboutir` disparaîtra le jour où cette route deviendra un webhook signé.
#[derive(Deserialize)]
pub struct ConfirmerPaiementRequest {
    pub aboutir: bool,
}

/// Filtres de `GET /mes-cadeaux`.
#[derive(Deserialize)]
pub struct MesCadeauxQuery {
    pub page: Option<i64>,
    pub taille: Option<i64>,
    /// `recus` (défaut) ou `offerts`.
    pub sens: Option<String>,
}

/// Création / modification d'un cadeau (back-office).
/// `code` est ignoré en modification : c'est une clé stable.
#[derive(Deserialize)]
pub struct CadeauPayload {
    pub code: Option<String>,
    pub libelle: String,
    pub description: Option<String>,
    pub icone: Option<String>,
    pub couleur: Option<String>,
    pub prix: i32,
    pub points: i32,
    pub ordre: Option<i16>,
    pub actif: Option<bool>,
}

/// `PUT /admin/engagement/parametres-monetisation`.
#[derive(Deserialize)]
pub struct ParametreMonetisationPayload {
    pub taux_commission: i16,
    pub devise: String,
    pub paiement_reel_actif: bool,
}

/// Filtres du journal comptable d'administration.
#[derive(Deserialize)]
pub struct JournalAdminQuery {
    pub membre_id: Option<Uuid>,
    /// `offreur` ou `beneficiaire` : sens de lecture de `membre_id`.
    pub sens: Option<String>,
    pub etat: Option<String>,
    pub mode: Option<String>,
    pub simule: Option<bool>,
    pub debut: Option<DateTime<Utc>>,
    pub fin: Option<DateTime<Utc>>,
    pub page: Option<i64>,
    pub taille: Option<i64>,
}

/// `POST /admin/engagement/purger-phase-test`, garde-fou explicite contre le
/// déclenchement accidentel d'une opération irréversible.
#[derive(Deserialize)]
pub struct PurgeRequest {
    pub confirmation: String,
}

// ════════════════════════════════════════════════════════════════════════════
// CONSTANTES DE DOMAINE
// ════════════════════════════════════════════════════════════════════════════

/// Familles éditoriales **connues mais sans auteur enregistré**.
///
/// Elles ne sont pas « inconnues » : la requête est parfaitement formée, c'est
/// la cible qui n'a personne à créditer. D'où un `409` (conflit d'état) et non
/// un `400` (requête invalide) : la nuance compte pour le client, qui doit
/// afficher « ce contenu n'a pas d'auteur » et non « votre requête est erronée ».
pub const FAMILLES_SANS_AUTEUR: [&str; 2] = ["site_touristique", "secteur_developpement"];

/// Familles admises comme cible d'un cadeau.
///
/// Identiques à celles éligibles au « j'aime », **moins** les deux familles de
/// `FAMILLES_SANS_AUTEUR` (FR-008c).
pub const FAMILLES_CADEAU: [&str; 14] = [
    "codimoi",
    "factcheck",
    "biblio_humaine",
    "video",
    "fiche_pays",
    "chaine_tv",
    "station_radio",
    "emission_tele",
    "emission_radio",
    "episode_tele",
    "episode_radio",
    "personnalite_connue",
    "recette_culinaire",
    "profil",
];

/// Modes d'offre acceptés (miroir de l'enum `engagement.mode_cadeau`).
pub const MODES_CADEAU: [&str; 2] = ["soutien_financier", "points"];

/// Mot de confirmation exigé par la purge de fin de phase de test.
pub const CONFIRMATION_PURGE: &str = "PURGER";

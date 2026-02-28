use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ════════════════════════════════════════════════════════════════
// Avis de Recherche (Admin)
// ════════════════════════════════════════════════════════════════

// ── Colonnes SQL ────────────────────────────────────────────

pub const ADMIN_AVIS_LISTE_COLONNES: &str =
    "a.id, a.nom_recherche, a.prenom_recherche, a.ecole, a.ville,
     a.etat::text AS etat, a.created_at, a.updated_at,
     u.id AS auteur_id, u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email,
     p.nom AS pays_nom,
     (SELECT COUNT(*) FROM retrouve_amis.correspondance c WHERE c.avis_id = a.id) AS nb_correspondances,
     (SELECT COUNT(*) FROM retrouve_amis.signalement s WHERE s.avis_id = a.id) AS nb_signalements";

pub const ADMIN_AVIS_DETAIL_COLONNES: &str =
    "a.id, a.nom_recherche, a.prenom_recherche, a.surnom,
     a.ecole, a.ville, a.pays_id,
     a.periode_debut, a.periode_fin, a.description,
     a.etat::text AS etat, a.created_at, a.updated_at,
     u.id AS auteur_id, u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email,
     p.nom AS pays_nom";

pub const ADMIN_AVIS_TRI_COLONNES: &[&str] = &["created_at", "updated_at", "nom_recherche", "etat"];

pub const ADMIN_AVIS_JOINS: &str =
    "JOIN iam.utilisateur u ON u.id = a.auteur_id
     LEFT JOIN shared.pays p ON p.id = a.pays_id";

// ── Colonnes SQL Signalements ───────────────────────────────

pub const ADMIN_SIGNALEMENT_LISTE_COLONNES: &str =
    "s.id, s.avis_id, s.motif::text AS motif, s.description, s.etat::text AS etat, s.created_at,
     ar.nom_recherche AS avis_nom_recherche,
     ua.nom AS avis_auteur_nom, ua.prenom AS avis_auteur_prenom, ua.id AS avis_auteur_id, ua.email AS avis_auteur_email,
     us.id AS signale_par_id, us.nom AS signale_par_nom, us.prenom AS signale_par_prenom, us.email AS signale_par_email";

pub const ADMIN_SIGNALEMENT_JOINS: &str =
    "JOIN retrouve_amis.avis_recherche ar ON ar.id = s.avis_id
     JOIN iam.utilisateur ua ON ua.id = ar.auteur_id
     JOIN iam.utilisateur us ON us.id = s.signale_par";

pub const ADMIN_SIGNALEMENT_TRI_COLONNES: &[&str] = &["created_at", "motif", "etat"];

// ── Structs de listing (FromRow) ────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminAvisRechercheListe {
    pub id: Uuid,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub etat: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
    pub pays_nom: Option<String>,
    pub nb_correspondances: i64,
    pub nb_signalements: i64,
}

#[derive(Debug, FromRow)]
pub struct AdminAvisRechercheDetailRow {
    pub id: Uuid,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub surnom: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub pays_id: Option<Uuid>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub description: Option<String>,
    pub etat: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
    pub pays_nom: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct AdminSignalementListe {
    pub id: Uuid,
    pub avis_id: Uuid,
    pub avis_nom_recherche: String,
    pub avis_auteur_nom: String,
    pub avis_auteur_prenom: String,
    pub avis_auteur_id: Uuid,
    pub avis_auteur_email: String,
    pub signale_par_id: Uuid,
    pub signale_par_nom: String,
    pub signale_par_prenom: String,
    pub signale_par_email: String,
    pub motif: String,
    pub description: Option<String>,
    pub etat: String,
    pub created_at: DateTime<Utc>,
}

// ── Sous-entités (FromRow) ──────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminCorrespondanceRow {
    pub id: Uuid,
    pub score: f64,
    pub etat: String,
    pub type_cible: String,
    pub cible_utilisateur_id: Option<Uuid>,
    pub cible_utilisateur_nom: Option<String>,
    pub cible_utilisateur_prenom: Option<String>,
    pub cible_utilisateur_email: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminSignalementRow {
    pub id: Uuid,
    pub signale_par_id: Uuid,
    pub signale_par_nom: String,
    pub signale_par_prenom: String,
    pub signale_par_email: String,
    pub motif: String,
    pub description: Option<String>,
    pub etat: String,
    pub created_at: DateTime<Utc>,
}

// ════════════════════════════════════════════════════════════════
// Response DTOs (Serialize)
// ════════════════════════════════════════════════════════════════

// ── Structures partagées ────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
pub struct AdminAuteurInfo {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AdminPaysInfo {
    pub id: Uuid,
    pub nom: String,
}

// ── Avis de recherche — Liste ───────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminAvisRechercheResponse {
    pub id: Uuid,
    pub auteur: AdminAuteurInfo,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub pays: Option<AdminPaysInfo>,
    pub etat: String,
    pub nb_correspondances: i64,
    pub nb_signalements: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminAvisRechercheListeResponse {
    pub avis: Vec<AdminAvisRechercheResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
}

// ── Avis de recherche — Détail ──────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminCorrespondanceInfo {
    pub id: Uuid,
    pub score: f64,
    pub etat: String,
    pub type_cible: String,
    pub cible_utilisateur: Option<AdminAuteurInfo>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminSignalementInfo {
    pub id: Uuid,
    pub signale_par: AdminAuteurInfo,
    pub motif: String,
    pub description: Option<String>,
    pub etat: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminAvisRechercheDetailResponse {
    pub id: Uuid,
    pub auteur: AdminAuteurInfo,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub surnom: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub pays: Option<AdminPaysInfo>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub description: Option<String>,
    pub etat: String,
    pub correspondances: Vec<AdminCorrespondanceInfo>,
    pub signalements: Vec<AdminSignalementInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Signalements — Liste ────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminAvisResume {
    pub id: Uuid,
    pub nom_recherche: String,
    pub auteur: AdminAuteurInfo,
}

#[derive(Debug, Serialize)]
pub struct AdminSignalementResponse {
    pub id: Uuid,
    pub avis: AdminAvisResume,
    pub signale_par: AdminAuteurInfo,
    pub motif: String,
    pub description: Option<String>,
    pub etat: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminSignalementListeResponse {
    pub signalements: Vec<AdminSignalementResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
}

// ── Statistiques ────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminStatistiques {
    pub total_avis: i64,
    pub avis_actifs: i64,
    pub avis_clotures: i64,
    pub avis_suspendus: i64,
    pub total_correspondances: i64,
    pub correspondances_mutuelles: i64,
    pub correspondances_en_attente: i64,
    pub correspondances_declinees: i64,
    pub correspondances_archivees: i64,
    pub utilisateurs_trouvables: i64,
    pub signalements_en_attente: i64,
    pub signalements_total: i64,
    pub blacklists_total: i64,
}

// ════════════════════════════════════════════════════════════════
// Request DTOs (Deserialize)
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct ChangerEtatAvis {
    pub etat: String,
}

#[derive(Debug, Deserialize)]
pub struct ModererSignalement {
    pub decision: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminAvisQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub auteur_id: Option<String>,
    pub pays_id: Option<String>,
    pub date_debut: Option<String>,
    pub date_fin: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminSignalementQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub etat: Option<String>,
    pub motif: Option<String>,
}

// ════════════════════════════════════════════════════════════════
// Conversions FromRow → Response DTO
// ════════════════════════════════════════════════════════════════

impl AdminAvisRechercheListe {
    /// Convertit la ligne SQL en DTO de réponse API
    pub fn to_response(&self, pays_id: Option<Uuid>) -> AdminAvisRechercheResponse {
        AdminAvisRechercheResponse {
            id: self.id,
            auteur: AdminAuteurInfo {
                id: self.auteur_id,
                nom: self.auteur_nom.clone(),
                prenom: self.auteur_prenom.clone(),
                email: self.auteur_email.clone(),
            },
            nom_recherche: self.nom_recherche.clone(),
            prenom_recherche: self.prenom_recherche.clone(),
            ecole: self.ecole.clone(),
            ville: self.ville.clone(),
            pays: self.pays_nom.as_ref().and_then(|nom| {
                pays_id.map(|id| AdminPaysInfo {
                    id,
                    nom: nom.clone(),
                })
            }),
            etat: self.etat.clone(),
            nb_correspondances: self.nb_correspondances,
            nb_signalements: self.nb_signalements,
            created_at: self.created_at,
        }
    }
}

impl AdminAvisRechercheDetailRow {
    /// Convertit la ligne SQL en DTO de détail avec correspondances et signalements
    pub fn to_response(
        &self,
        correspondances: Vec<AdminCorrespondanceInfo>,
        signalements: Vec<AdminSignalementInfo>,
    ) -> AdminAvisRechercheDetailResponse {
        AdminAvisRechercheDetailResponse {
            id: self.id,
            auteur: AdminAuteurInfo {
                id: self.auteur_id,
                nom: self.auteur_nom.clone(),
                prenom: self.auteur_prenom.clone(),
                email: self.auteur_email.clone(),
            },
            nom_recherche: self.nom_recherche.clone(),
            prenom_recherche: self.prenom_recherche.clone(),
            surnom: self.surnom.clone(),
            ecole: self.ecole.clone(),
            ville: self.ville.clone(),
            pays: self.pays_id.and_then(|id| {
                self.pays_nom.as_ref().map(|nom| AdminPaysInfo {
                    id,
                    nom: nom.clone(),
                })
            }),
            periode_debut: self.periode_debut,
            periode_fin: self.periode_fin,
            description: self.description.clone(),
            etat: self.etat.clone(),
            correspondances,
            signalements,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl AdminCorrespondanceRow {
    /// Convertit la ligne SQL en DTO de correspondance
    pub fn to_info(&self) -> AdminCorrespondanceInfo {
        AdminCorrespondanceInfo {
            id: self.id,
            score: self.score,
            etat: self.etat.clone(),
            type_cible: self.type_cible.clone(),
            cible_utilisateur: self.cible_utilisateur_id.map(|id| AdminAuteurInfo {
                id,
                nom: self.cible_utilisateur_nom.clone().unwrap_or_default(),
                prenom: self.cible_utilisateur_prenom.clone().unwrap_or_default(),
                email: self.cible_utilisateur_email.clone().unwrap_or_default(),
            }),
            created_at: self.created_at,
        }
    }
}

impl AdminSignalementRow {
    /// Convertit la ligne SQL en DTO de signalement
    pub fn to_info(&self) -> AdminSignalementInfo {
        AdminSignalementInfo {
            id: self.id,
            signale_par: AdminAuteurInfo {
                id: self.signale_par_id,
                nom: self.signale_par_nom.clone(),
                prenom: self.signale_par_prenom.clone(),
                email: self.signale_par_email.clone(),
            },
            motif: self.motif.clone(),
            description: self.description.clone(),
            etat: self.etat.clone(),
            created_at: self.created_at,
        }
    }
}

impl AdminSignalementListe {
    /// Convertit la ligne SQL en DTO de réponse pour la liste des signalements
    pub fn to_response(&self) -> AdminSignalementResponse {
        AdminSignalementResponse {
            id: self.id,
            avis: AdminAvisResume {
                id: self.avis_id,
                nom_recherche: self.avis_nom_recherche.clone(),
                auteur: AdminAuteurInfo {
                    id: self.avis_auteur_id,
                    nom: self.avis_auteur_nom.clone(),
                    prenom: self.avis_auteur_prenom.clone(),
                    email: self.avis_auteur_email.clone(),
                },
            },
            signale_par: AdminAuteurInfo {
                id: self.signale_par_id,
                nom: self.signale_par_nom.clone(),
                prenom: self.signale_par_prenom.clone(),
                email: self.signale_par_email.clone(),
            },
            motif: self.motif.clone(),
            description: self.description.clone(),
            etat: self.etat.clone(),
            created_at: self.created_at,
        }
    }
}

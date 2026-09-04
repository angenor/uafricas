//! Soumission de médias par les parties prenantes et modération administrative
//! (feature 001-refonte-tele-radio, US4, migration 09l).
//!
//! Une seule table polymorphe porte les six types de proposition : le workflow
//! est identique pour une chaîne, une station ou un contenu, et le
//! polymorphisme donne UNE file de modération et UN écran de suivi.
//!
//! **Invariant central** : rien de non validé n'atteint le public (FR-031). Le
//! contenu proposé vit dans `donnees` jusqu'à la validation, qui crée alors
//! seulement l'objet métier réel.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::ApiErreur;

/// Les types de proposition, tels que déclarés par l'ENUM
/// `media_content.type_objet_propose`.
///
/// Depuis 09q, les contenus se proposent à deux niveaux : une **émission**
/// (programme conteneur) ou un **épisode** versé dans une émission existante.
/// Les anciennes valeurs `programme_tele` / `programme_radio` subsistent dans
/// l'enum pour l'historique : PostgreSQL ne sait pas retirer une valeur, mais
/// ne sont plus produites, d'où leur absence de cette liste.
pub const TYPES_OBJET_PROPOSE: [&str; 8] = [
    "chaine_tv",
    "station_radio",
    "emission_tele",
    "emission_radio",
    "episode_tele",
    "episode_radio",
    "animation_programme",
    "idee_contenu",
];

/// Types dont la validation crée un objet dans une table métier.
pub const TYPES_CREANT_UN_OBJET: [&str; 6] = [
    "chaine_tv",
    "station_radio",
    "emission_tele",
    "emission_radio",
    "episode_tele",
    "episode_radio",
];

/// Types dont la proposition exige une `target_id` désignant l'émission
/// d'accueil : un épisode n'existe pas hors d'un programme (FR-002).
pub const TYPES_EPISODE_PROPOSE: [&str; 2] = ["episode_tele", "episode_radio"];

/// Les neuf rôles de partie prenante déclarables (FR-029), miroir du CHECK posé
/// par 09j sur `chaine_tv` et `station_radio`.
pub const ROLES_PARTIE_PRENANTE: [&str; 9] = [
    "chaine_tele",
    "radio",
    "journaliste",
    "communicateur",
    "createur_contenu",
    "influenceur",
    "realisateur",
    "producteur",
    "autre",
];

/// Longueur minimale du motif de rejet, l'auteur doit pouvoir comprendre le
/// refus depuis son écran de suivi (FR-033).
pub const LONGUEUR_MIN_MOTIF_REJET: usize = 10;

pub fn type_objet_valide(valeur: &str) -> bool {
    TYPES_OBJET_PROPOSE.contains(&valeur)
}

/// Table métier créée à la validation d'une proposition, ou None pour les types
/// qui n'en créent aucune (`animation_programme`, `idee_contenu`).
pub fn table_cible(type_objet: &str) -> Option<&'static str> {
    match type_objet {
        "chaine_tv" => Some("media_content.chaine_tv"),
        "station_radio" => Some("media_content.station_radio"),
        "emission_tele" => Some("media_content.emission_tele"),
        "emission_radio" => Some("media_content.emission_radio"),
        "episode_tele" => Some("media_content.episode_tele"),
        "episode_radio" => Some("media_content.episode_radio"),
        _ => None,
    }
}

/// Type de support auquel se rattache l'objet proposé, ou None pour les types
/// qui n'en désignent pas.
pub fn support_pour_type_objet(type_objet: &str) -> Option<&'static str> {
    match type_objet {
        "emission_tele" | "episode_tele" => Some("chaine_tv"),
        "emission_radio" | "episode_radio" => Some("station_radio"),
        _ => None,
    }
}

// ────────────────────────────────────────────────────────────────
// Payload métier : le contenu de `donnees`
// ────────────────────────────────────────────────────────────────

/// Champs de l'objet proposé, communs aux quatre types créateurs.
///
/// Les champs non pertinents pour un type donné sont simplement ignorés à la
/// création : un programme n'a pas de `stream_url`, une chaîne pas de
/// `theme_phare_id`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DonneesProposition {
    pub nom: Option<String>,
    pub description: Option<String>,
    pub langue: Option<String>,
    pub pays: Option<String>,
    /// Chaîne ou station (FR-029).
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Contenu (FR-030).
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    /// Support de rattachement du contenu proposé.
    pub chaine_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
    /// Lien externe, ou chemin renvoyé par le téléversement (FR-056).
    pub video_url: Option<String>,
    pub audio_url: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    /// Source déclarée du média et auteur réel, présentés en évidence à
    /// l'administrateur, seul à se prononcer sur la licéité (H-012, FR-033).
    pub source_declaree: Option<String>,
    pub auteur_declare: Option<String>,
    /// Portée du support proposé (09v), supports uniquement.
    ///
    /// Deux natures s'excluent, et le formulaire ne présente jamais les deux
    /// jeux de champs à la fois :
    ///   • thématique  : `est_thematique`, `thematique_id` : une seule, et
    ///     tous les territoires d'office, d'où l'absence de champ de
    ///     couverture ;
    ///   • territoriale : `territoires` (1..N) ou `couverture_continentale`.
    ///
    /// Rien n'est écrit dans les tables métier avant la validation : ces champs
    /// dorment dans `donnees` comme le reste de la proposition.
    pub est_thematique: Option<bool>,
    pub thematique_id: Option<Uuid>,
    #[serde(default)]
    pub territoires: Vec<Uuid>,
    pub couverture_continentale: Option<bool>,
    /// Coordonnées publiques de l'équipe, pour les supports uniquement (09p).
    ///
    /// Elles ne deviennent publiques qu'à la validation, comme le reste de la
    /// proposition : tant qu'elle est en attente, elles ne vivent que dans
    /// `donnees`, sous les yeux du seul administrateur.
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
}

impl DonneesProposition {
    /// Valide le payload au regard du type proposé.
    ///
    /// Les règles « Autre exige une précision » sont doublées d'un CHECK SQL ;
    /// c'est ici qu'elles produisent un message en français plutôt qu'une
    /// violation de contrainte (FR-029, FR-030).
    pub fn valider(&self, type_objet: &str) -> Result<(), ApiErreur> {
        let nom_requis = TYPES_CREANT_UN_OBJET.contains(&type_objet);
        if nom_requis && self.nom.as_deref().map(str::trim).unwrap_or("").is_empty() {
            return Err(ApiErreur::Validation(
                "Le nom du contenu proposé est requis".into(),
            ));
        }

        // Rôle de partie prenante : supports uniquement.
        if matches!(type_objet, "chaine_tv" | "station_radio") {
            match self.role_partie_prenante.as_deref().map(str::trim) {
                None | Some("") => {
                    return Err(ApiErreur::Validation(
                        "Le rôle de partie prenante est requis".into(),
                    ));
                }
                Some(role) if !ROLES_PARTIE_PRENANTE.contains(&role) => {
                    return Err(ApiErreur::Validation(format!(
                        "Rôle de partie prenante « {} » inconnu",
                        role
                    )));
                }
                Some("autre")
                    if self
                        .role_partie_prenante_autre
                        .as_deref()
                        .map(str::trim)
                        .unwrap_or("")
                        .is_empty() =>
                {
                    return Err(ApiErreur::Validation(
                        "Précisez le rôle de partie prenante choisi (« Autre »)".into(),
                    ));
                }
                _ => {}
            }

            // Portée du support (09v). Elle est demandée dès la soumission :
            // laisser l'administrateur la deviner reviendrait à lui faire
            // arbitrer un choix éditorial qui appartient au proposant.
            if self.est_thematique.unwrap_or(false) {
                if self.thematique_id.is_none() {
                    return Err(ApiErreur::Validation(
                        "Un support thématique doit désigner sa thématique".into(),
                    ));
                }
            } else if !self.couverture_continentale.unwrap_or(false)
                && self.territoires.is_empty()
            {
                return Err(ApiErreur::Validation(
                    "Indiquez le ou les territoires concernés, ou cochez « tous les territoires »"
                        .into(),
                ));
            }
        }

        // Thème phare : émissions uniquement. Un épisode hérite du thème de son
        // programme : le lui redemander à chaque versement serait du bruit.
        if matches!(type_objet, "emission_tele" | "emission_radio") {
            let autre_renseigne = !self
                .theme_phare_autre
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty();
            if self.theme_phare_id.is_none() && !autre_renseigne {
                return Err(ApiErreur::Validation(
                    "Choisissez un thème phare, ou précisez-en un au titre de « Autre »".into(),
                ));
            }
        }

        Ok(())
    }
}

// ────────────────────────────────────────────────────────────────
// Soumission (membre)
// ────────────────────────────────────────────────────────────────

/// Champs texte reçus en multipart, avant désérialisation de `donnees`.
///
/// `statut` et `origine_publication` n'y figurent PAS : ils sont fixés par le
/// serveur et ne sont jamais pilotables par le client (FR-031, FR-036).
#[derive(Debug, Default)]
pub struct SoumissionBrute {
    pub type_objet: String,
    pub target_id: Option<Uuid>,
    pub justification: String,
    pub donnees: DonneesProposition,
    pub pieces_jointes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MesPropositionsFiltres {
    pub statut: Option<String>,
    pub type_objet: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ────────────────────────────────────────────────────────────────
// Modération (administrateur)
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PropositionsAdminFiltres {
    pub statut: Option<String>,
    pub type_objet: Option<String>,
    pub auteur: Option<Uuid>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct DecisionMediaRequest {
    pub commentaire: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Édition d'un contenu par son détenteur (FR-032)
// ────────────────────────────────────────────────────────────────

/// Métadonnées éditables sans revalidation, publiées immédiatement.
#[derive(Debug, Deserialize)]
pub struct MetadonneesRequest {
    pub nom: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
}

/// Remplacement du fichier ou du lien média, bascule le contenu en
/// `'en_attente'` et ouvre une proposition de modification.
#[derive(Debug, Deserialize)]
pub struct RemplacerMediaRequest {
    pub media_url: String,
    pub justification: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Lecture
// ────────────────────────────────────────────────────────────────

pub const PROPOSITION_MEDIA_COLONNES: &str =
    "pm.id, pm.auteur_id, pm.type_objet::text AS type_objet, pm.target_id,
     pm.donnees, pm.pieces_jointes, pm.justification,
     pm.statut::text AS statut, pm.decideur, pm.decide_at,
     pm.commentaire_decision, pm.objet_id_cree,
     pm.created_at, pm.updated_at";

#[derive(Debug, FromRow)]
pub struct PropositionMediaRow {
    pub id: Uuid,
    pub auteur_id: Uuid,
    pub type_objet: String,
    pub target_id: Option<Uuid>,
    pub donnees: serde_json::Value,
    pub pieces_jointes: serde_json::Value,
    pub justification: String,
    pub statut: String,
    pub decideur: Option<Uuid>,
    pub decide_at: Option<DateTime<Utc>>,
    pub commentaire_decision: Option<String>,
    pub objet_id_cree: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs de jointure, absents des lectures qui ne les demandent pas.
    #[sqlx(default)]
    pub auteur_nom: Option<String>,
    #[sqlx(default)]
    pub auteur_prenom: Option<String>,
    #[sqlx(default)]
    pub auteur_email: Option<String>,
    #[sqlx(default)]
    pub decideur_nom: Option<String>,
    #[sqlx(default)]
    pub decideur_prenom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PropositionMediaResponse {
    pub id: Uuid,
    pub auteur_id: Uuid,
    pub auteur_nom: Option<String>,
    pub auteur_prenom: Option<String>,
    pub auteur_email: Option<String>,
    pub type_objet: String,
    pub target_id: Option<Uuid>,
    pub donnees: serde_json::Value,
    pub pieces_jointes: serde_json::Value,
    pub justification: String,
    pub statut: String,
    pub decideur: Option<Uuid>,
    pub decideur_nom: Option<String>,
    pub decideur_prenom: Option<String>,
    pub decide_at: Option<DateTime<Utc>>,
    pub commentaire_decision: Option<String>,
    pub objet_id_cree: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PropositionMediaListeResponse {
    pub propositions: Vec<PropositionMediaResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

impl PropositionMediaRow {
    pub fn to_response(self) -> PropositionMediaResponse {
        PropositionMediaResponse {
            id: self.id,
            auteur_id: self.auteur_id,
            auteur_nom: self.auteur_nom,
            auteur_prenom: self.auteur_prenom,
            auteur_email: self.auteur_email,
            type_objet: self.type_objet,
            target_id: self.target_id,
            donnees: self.donnees,
            pieces_jointes: self.pieces_jointes,
            justification: self.justification,
            statut: self.statut,
            decideur: self.decideur,
            decideur_nom: self.decideur_nom,
            decideur_prenom: self.decideur_prenom,
            decide_at: self.decide_at,
            commentaire_decision: self.commentaire_decision,
            objet_id_cree: self.objet_id_cree,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

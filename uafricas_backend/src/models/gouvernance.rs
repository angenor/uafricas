use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Stats de gouvernance ──────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct GouvernanceStats {
    pub total: i64,
    pub factcheck: i64,
    pub badhabits: i64,
    pub ideaforces: i64,
    pub total_likes: i64,
}

// ── Row unifie pour la requete UNION ALL ──────────────────────

#[derive(Debug, FromRow)]
pub struct ContributionRow {
    pub id: Uuid,
    pub type_contribution: String,
    pub titre: String,
    pub description: String,
    pub nombre_likes: i32,
    pub nombre_soutiens: i32,
    pub statut: String,
    pub created_at: DateTime<Utc>,
    pub auteur_id: Option<Uuid>,
    pub auteur_prenom: Option<String>,
    pub auteur_nom: Option<String>,
    pub auteur_photo_url: Option<String>,
    pub pays_nom: Option<String>,
    pub region: Option<String>,
    pub ville: Option<String>,
    pub categorie: Option<String>,
    pub gravite: Option<String>,
    // Volets factcheck (préjugé / réalité) — NULL pour badhabits/ideaforces
    pub prejuge_titre: Option<String>,
    pub prejuge_description: Option<String>,
    pub prejuge_likes: i32,
    pub realite_titre: Option<String>,
    pub realite_description: Option<String>,
    pub realite_likes: i32,
    // Compteurs de réaction globale (emojis)
    pub nombre_coeur: i32,
    pub nombre_pouce: i32,
    pub nombre_rire: i32,
    pub nombre_jaime_pas: i32,
    // État de réaction de l'utilisateur courant (NULL/false si anonyme ou non-factcheck)
    pub ma_reaction_general: Option<String>,
    pub a_like_prejuge: bool,
    pub a_like_realite: bool,
    pub a_signale: bool,
    pub total_count: i64,
}

// ── DTOs de reponse API ───────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ContributionAuteurResponse {
    pub id: Uuid,
    pub prenom: String,
    pub nom: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContributionLocalisationResponse {
    pub pays: String,
    pub region: Option<String>,
    pub ville: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContributionStatsResponse {
    pub likes: i32,
    pub soutiens: i32,
}

/// Un volet d'un factcheck (préjugé ou réalité) avec son compteur de cœurs
#[derive(Debug, Serialize)]
pub struct FactcheckVoletResponse {
    pub titre: String,
    pub description: String,
    pub likes: i32,
}

/// Compteurs de réaction globale (emojis) d'un factcheck
#[derive(Debug, Serialize)]
pub struct FactcheckReactionsResponse {
    pub coeur: i32,
    pub pouce: i32,
    pub rire: i32,
    pub jaime_pas: i32,
    /// Réaction de l'utilisateur courant ('coeur'|'pouce'|'rire'|'jaime_pas') ou null
    pub ma_reaction: Option<String>,
}

/// Détail factcheck : volets préjugé/réalité + réactions globales + état utilisateur
#[derive(Debug, Serialize)]
pub struct FactcheckDetailResponse {
    pub prejuge: FactcheckVoletResponse,
    #[serde(rename = "contrePrejuge")]
    pub contre_prejuge: FactcheckVoletResponse,
    pub reactions: FactcheckReactionsResponse,
    pub a_like_prejuge: bool,
    pub a_like_realite: bool,
    /// L'utilisateur courant a déjà signalé ce factcheck
    pub a_signale: bool,
}

#[derive(Debug, Serialize)]
pub struct ContributionResponse {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_contribution: String,
    pub statut: String,
    pub titre: String,
    pub description: String,
    pub auteur: ContributionAuteurResponse,
    pub localisation: ContributionLocalisationResponse,
    pub date_creation: DateTime<Utc>,
    pub stats: ContributionStatsResponse,
    pub categorie: Option<String>,
    pub gravite: Option<String>,
    pub type_pratique: Option<String>,
    /// Présent uniquement pour les contributions de type factcheck
    pub factcheck: Option<FactcheckDetailResponse>,
}

/// Corps de requête pour réagir à un factcheck
#[derive(Debug, Deserialize)]
pub struct ReactionFactcheckRequest {
    /// 'general' | 'prejuge' | 'realite'
    pub cible: String,
    /// pour general : 'coeur'|'pouce'|'rire'|'jaime_pas' ; pour prejuge/realite : 'coeur'
    pub type_reaction: Option<String>,
}

/// État de réaction renvoyé après un toggle
#[derive(Debug, Serialize, FromRow)]
pub struct FactcheckReactionEtat {
    pub nombre_coeur: i32,
    pub nombre_pouce: i32,
    pub nombre_rire: i32,
    pub nombre_jaime_pas: i32,
    pub prejuge_nombre_likes: i32,
    pub realite_nombre_likes: i32,
    pub ma_reaction_general: Option<String>,
    pub a_like_prejuge: bool,
    pub a_like_realite: bool,
}

/// Corps de requête pour signaler un factcheck
#[derive(Debug, Deserialize)]
pub struct SignalementRequest {
    pub motif: Option<String>,
    pub commentaire: Option<String>,
}

/// État renvoyé après un signalement
#[derive(Debug, Serialize)]
pub struct SignalementEtat {
    pub nombre_signalements: i32,
    pub etat: String,
    pub deja_signale: bool,
    pub suspendu: bool,
}

#[derive(Debug, Serialize)]
pub struct ContributionListeResponse {
    pub contributions: Vec<ContributionResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

impl ContributionRow {
    pub fn to_response(&self) -> ContributionResponse {
        // Pour l'instant, le backend ne stocke que des mauvaises pratiques
        // dans governance.bad_habit. Les bonnes pratiques n'ont pas encore
        // de table dediee : on expose donc "mauvaise" pour badhabits.
        let type_pratique = match self.type_contribution.as_str() {
            "badhabits" => Some("mauvaise".to_string()),
            _ => None,
        };

        // Détail factcheck (volets + réactions) uniquement pour les factchecks
        let factcheck = if self.type_contribution == "factcheck" {
            Some(FactcheckDetailResponse {
                prejuge: FactcheckVoletResponse {
                    titre: self.prejuge_titre.clone().unwrap_or_default(),
                    description: self.prejuge_description.clone().unwrap_or_default(),
                    likes: self.prejuge_likes,
                },
                contre_prejuge: FactcheckVoletResponse {
                    titre: self.realite_titre.clone().unwrap_or_default(),
                    description: self.realite_description.clone().unwrap_or_default(),
                    likes: self.realite_likes,
                },
                reactions: FactcheckReactionsResponse {
                    coeur: self.nombre_coeur,
                    pouce: self.nombre_pouce,
                    rire: self.nombre_rire,
                    jaime_pas: self.nombre_jaime_pas,
                    ma_reaction: self.ma_reaction_general.clone(),
                },
                a_like_prejuge: self.a_like_prejuge,
                a_like_realite: self.a_like_realite,
                a_signale: self.a_signale,
            })
        } else {
            None
        };

        ContributionResponse {
            id: self.id,
            type_contribution: self.type_contribution.clone(),
            statut: self.statut.clone(),
            titre: self.titre.clone(),
            description: self.description.clone(),
            auteur: ContributionAuteurResponse {
                id: self.auteur_id.unwrap_or_default(),
                prenom: self.auteur_prenom.clone().unwrap_or_default(),
                nom: self.auteur_nom.clone().unwrap_or_default(),
                photo_url: self.auteur_photo_url.clone(),
            },
            localisation: ContributionLocalisationResponse {
                pays: self.pays_nom.clone().unwrap_or_else(|| "Inconnu".to_string()),
                region: self.region.clone(),
                ville: self.ville.clone(),
            },
            date_creation: self.created_at,
            stats: ContributionStatsResponse {
                likes: self.nombre_likes,
                soutiens: self.nombre_soutiens,
            },
            categorie: self.categorie.clone(),
            gravite: self.gravite.clone(),
            type_pratique,
            factcheck,
        }
    }
}

// ── Parametres de requete ─────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ContributionQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    #[serde(rename = "type")]
    pub type_contribution: Option<String>,
}

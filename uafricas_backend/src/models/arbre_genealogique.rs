// ════════════════════════════════════════════════════════════════════════════
// Modèles — Arbre généalogique
// ════════════════════════════════════════════════════════════════════════════

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Constante de sélection ───────────────────────────────────────────────

pub const PERSONNE_COLONNES: &str =
    "p.id, p.nom, p.prenoms, p.genre, \
     p.naissance_annee, p.naissance_mois, p.naissance_jour, p.naissance_lieu, \
     p.deces_annee, p.deces_mois, p.deces_jour, p.deces_lieu, \
     p.photo_url, p.cree_par, p.created_at, p.updated_at";

// ─── Structs BDD (FromRow) ────────────────────────────────────────────────

/// Personne réelle — entité partageable entre arbres
#[derive(Debug, sqlx::FromRow)]
pub struct Personne {
    pub id: Uuid,
    pub nom: String,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance_annee: Option<i16>,
    pub naissance_mois: Option<i16>,
    pub naissance_jour: Option<i16>,
    pub naissance_lieu: Option<String>,
    pub deces_annee: Option<i16>,
    pub deces_mois: Option<i16>,
    pub deces_jour: Option<i16>,
    pub deces_lieu: Option<String>,
    pub photo_url: Option<String>,
    pub cree_par: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Arbre généalogique — un par utilisateur
#[derive(Debug, sqlx::FromRow)]
pub struct Arbre {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Rattachement — lie une Personne réelle à un Arbre
#[derive(Debug, sqlx::FromRow)]
pub struct Rattachement {
    pub id: Uuid,
    pub arbre_id: Uuid,
    pub personne_id: Uuid,
    pub ajoute_le: DateTime<Utc>,
}

/// Lien familial — relation typée entre deux Rattachements du même Arbre
#[derive(Debug, sqlx::FromRow)]
pub struct LienFamilial {
    pub id: Uuid,
    pub arbre_id: Uuid,
    pub rattachement_source_id: Uuid,
    pub rattachement_cible_id: Uuid,
    pub type_lien: String,
    pub created_at: DateTime<Utc>,
}

// ─── DTOs de réponse (Serialize) ──────────────────────────────────────────

/// Date à granularité variable (année seule, mois+année, ou complète)
#[derive(Debug, Serialize)]
pub struct DatePartielle {
    pub annee: Option<i16>,
    pub mois: Option<i16>,
    pub jour: Option<i16>,
}

/// Réponse fiche personne (sans liens)
#[derive(Debug, Serialize)]
pub struct PersonneResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance: Option<DatePartielle>,
    pub naissance_lieu: Option<String>,
    pub deces: Option<DatePartielle>,
    pub deces_lieu: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl PersonneResponse {
    pub fn from_row(p: Personne) -> Self {
        let naissance = if p.naissance_annee.is_some()
            || p.naissance_mois.is_some()
            || p.naissance_jour.is_some()
        {
            Some(DatePartielle {
                annee: p.naissance_annee,
                mois: p.naissance_mois,
                jour: p.naissance_jour,
            })
        } else {
            None
        };

        let deces = if p.deces_annee.is_some()
            || p.deces_mois.is_some()
            || p.deces_jour.is_some()
        {
            Some(DatePartielle {
                annee: p.deces_annee,
                mois: p.deces_mois,
                jour: p.deces_jour,
            })
        } else {
            None
        };

        Self {
            id: p.id,
            nom: p.nom,
            prenoms: p.prenoms,
            genre: p.genre,
            naissance,
            naissance_lieu: p.naissance_lieu,
            deces,
            deces_lieu: p.deces_lieu,
            photo_url: p.photo_url,
            created_at: p.created_at,
        }
    }
}

/// Résumé d'un lien (pour l'affichage dans la fiche détail)
#[derive(Debug, Serialize)]
pub struct LienResumeResponse {
    pub lien_id: Uuid,
    pub type_lien: String,
    pub personne: PersonneResponse,
}

/// Réponse fiche personne complète avec liens familiaux directs
#[derive(Debug, Serialize)]
pub struct PersonneDetailResponse {
    pub rattachement_id: Uuid,
    pub personne: PersonneResponse,
    pub parents: Vec<LienResumeResponse>,
    pub enfants: Vec<LienResumeResponse>,
    pub conjoints: Vec<LienResumeResponse>,
}

/// Réponse liste paginée des personnes
#[derive(Debug, Serialize)]
pub struct PersonneListeResponse {
    pub personnes: Vec<PersonneResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Réponse lien familial créé
#[derive(Debug, Serialize)]
pub struct LienFamilialResponse {
    pub id: Uuid,
    pub type_lien: String,
    pub rattachement_source_id: Uuid,
    pub rattachement_cible_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<LienFamilial> for LienFamilialResponse {
    fn from(l: LienFamilial) -> Self {
        Self {
            id: l.id,
            type_lien: l.type_lien,
            rattachement_source_id: l.rattachement_source_id,
            rattachement_cible_id: l.rattachement_cible_id,
            created_at: l.created_at,
        }
    }
}

// ─── DTOs — Arbre complet (visualisation) ─────────────────────────────────

/// Nœud personne pour la visualisation d'arbre (inclut rattachement_id)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PersonneNoeud {
    pub id: Uuid,
    pub rattachement_id: Uuid,
    pub nom: String,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance_annee: Option<i16>,
    pub naissance_mois: Option<i16>,
    pub naissance_jour: Option<i16>,
    pub naissance_lieu: Option<String>,
    pub deces_annee: Option<i16>,
    pub deces_mois: Option<i16>,
    pub deces_jour: Option<i16>,
    pub deces_lieu: Option<String>,
    pub photo_url: Option<String>,
}

/// Nœud personne sérialisé avec dates partielles
#[derive(Debug, Serialize)]
pub struct PersonneNoeudResponse {
    pub id: Uuid,
    pub rattachement_id: Uuid,
    pub nom: String,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance: Option<DatePartielle>,
    pub naissance_lieu: Option<String>,
    pub deces: Option<DatePartielle>,
    pub deces_lieu: Option<String>,
    pub photo_url: Option<String>,
}

impl PersonneNoeudResponse {
    pub fn from_row(p: PersonneNoeud) -> Self {
        let naissance = if p.naissance_annee.is_some()
            || p.naissance_mois.is_some()
            || p.naissance_jour.is_some()
        {
            Some(DatePartielle {
                annee: p.naissance_annee,
                mois: p.naissance_mois,
                jour: p.naissance_jour,
            })
        } else {
            None
        };

        let deces = if p.deces_annee.is_some()
            || p.deces_mois.is_some()
            || p.deces_jour.is_some()
        {
            Some(DatePartielle {
                annee: p.deces_annee,
                mois: p.deces_mois,
                jour: p.deces_jour,
            })
        } else {
            None
        };

        Self {
            id: p.id,
            rattachement_id: p.rattachement_id,
            nom: p.nom,
            prenoms: p.prenoms,
            genre: p.genre,
            naissance,
            naissance_lieu: p.naissance_lieu,
            deces,
            deces_lieu: p.deces_lieu,
            photo_url: p.photo_url,
        }
    }
}

/// Lien familial simplifié pour la visualisation
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct LienArbreResponse {
    pub id: Uuid,
    pub rattachement_source_id: Uuid,
    pub rattachement_cible_id: Uuid,
    pub type_lien: String,
}

/// Réponse arbre complet (toutes personnes + tous liens)
#[derive(Debug, Serialize)]
pub struct ArbreCompletResponse {
    pub arbre_id: Option<Uuid>,
    pub personnes: Vec<PersonneNoeudResponse>,
    pub liens: Vec<LienArbreResponse>,
}

// ─── DTOs de requête (Deserialize) ────────────────────────────────────────

/// Date partielle dans les requêtes (create/update)
#[derive(Debug, Deserialize)]
pub struct DatePartielleDto {
    pub annee: Option<i16>,
    pub mois: Option<i16>,
    pub jour: Option<i16>,
}

/// Corps de la requête de création d'une personne
#[derive(Debug, Deserialize)]
pub struct CreerPersonneDto {
    pub nom: String,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance: Option<DatePartielleDto>,
    pub naissance_lieu: Option<String>,
    pub deces: Option<DatePartielleDto>,
    pub deces_lieu: Option<String>,
}

/// Corps de la requête de modification d'une personne (tous champs optionnels)
#[derive(Debug, Deserialize)]
pub struct ModifierPersonneDto {
    pub nom: Option<String>,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance: Option<DatePartielleDto>,
    pub naissance_lieu: Option<String>,
    pub deces: Option<DatePartielleDto>,
    pub deces_lieu: Option<String>,
}

/// Corps de la requête de création d'un lien familial
#[derive(Debug, Deserialize)]
pub struct CreerLienDto {
    pub rattachement_source_id: Uuid,
    pub rattachement_cible_id: Uuid,
    pub type_lien: String,
}

/// Paramètres de query pour lister les personnes
#[derive(Debug, Deserialize)]
pub struct PersonneQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub recherche: Option<String>,
}

//! Africanité : publications éphémères du fil d'actualité.
//!
//! Spec : `specs/012-africanite-ephemere/spec.md`
//!
//! Trois règles gouvernent tout ce fichier :
//!
//! 1. **L'échéance se constate à la lecture.** Aucune tâche de fond ne fait
//!    expirer quoi que ce soit ; chaque requête porte `expire_at > NOW()`.
//! 2. **Le public est le cercle d'ami(e)s**, recalculé à chaque lecture depuis
//!    `social.amitie` : une rupture coupe l'accès sans traitement différé.
//! 3. **Une vue est unique par lecteur**, garantie par la clé primaire
//!    composite de `social.africanite_vue`, pas par un contrôle applicatif.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes de `social.africanite`, dans l'ordre du `FromRow`.
pub const AFRICANITE_COLONNES: &str =
    "a.id, a.auteur_id, a.forme, a.media_url, a.texte, a.couleur_fond, \
     a.legende, a.expire_at, a.created_at";

/// Ligne brute de `social.africanite`, enrichie de l'auteur et de l'état de vue.
#[derive(Debug, FromRow)]
pub struct AfricaniteRow {
    pub id: Uuid,
    pub auteur_id: Uuid,
    pub forme: String,
    pub media_url: Option<String>,
    pub texte: Option<String>,
    pub couleur_fond: Option<String>,
    pub legende: Option<String>,
    pub expire_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    // Jointures
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_photo_url: Option<String>,
    /// Le lecteur courant l'a-t-il déjà regardée ?
    pub vue: bool,
    /// Nombre de lecteurs distincts. Servi seulement à l'auteur (FR-020).
    pub nombre_vues: i64,
}

/// Une africanité telle qu'elle est servie au client.
#[derive(Debug, Serialize)]
pub struct AfricaniteResponse {
    pub id: Uuid,
    pub forme: String,
    pub media_url: Option<String>,
    pub texte: Option<String>,
    pub couleur_fond: Option<String>,
    pub legende: Option<String>,
    pub expire_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub vue: bool,
    /// Absent quand le lecteur n'est pas l'auteur : nul ne voit qui a regardé
    /// une africanité qui n'est pas la sienne (FR-020).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre_vues: Option<i64>,
}

/// Les africanités d'un auteur, regroupées : c'est l'unité de la rangée.
#[derive(Debug, Serialize)]
pub struct AuteurAfricanitesResponse {
    pub auteur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
    /// `true` quand le lecteur est l'auteur : sa pastille ouvre la marche.
    pub est_moi: bool,
    /// `true` tant qu'il reste une africanité non vue : c'est l'état de l'anneau.
    pub a_du_nouveau: bool,
    pub africanites: Vec<AfricaniteResponse>,
}

/// Corps de `POST /api/africanites` pour la forme `texte`.
///
/// Les formes `image` et `video` passent par multipart, pas par ce corps :
/// un fichier ne se transporte pas en JSON.
#[derive(Debug, Deserialize)]
pub struct CreerAfricaniteTexteBody {
    pub texte: String,
    pub couleur_fond: Option<String>,
    pub legende: Option<String>,
}

/// Durée de vie d'une africanité, en heures.
///
/// 24 h : la convention du format, et un repère que les membres connaissent
/// sans qu'on ait à le leur expliquer.
pub const DUREE_VIE_HEURES: i64 = 24;

/// Longueur maximale du texte, alignée sur le CHECK SQL. Répétée ici pour que
/// le refus arrive avant l'aller-retour base, jamais à sa place.
pub const TEXTE_MAX: usize = 280;
pub const LEGENDE_MAX: usize = 200;

/// Convertit une ligne en réponse. `est_auteur` décide si le décompte de vues
/// est sérialisé : la décision se prend ici, une seule fois, plutôt que dans
/// chaque handler.
pub fn construire_africanite_response(row: &AfricaniteRow, est_auteur: bool) -> AfricaniteResponse {
    AfricaniteResponse {
        id: row.id,
        forme: row.forme.clone(),
        media_url: row.media_url.clone(),
        texte: row.texte.clone(),
        couleur_fond: row.couleur_fond.clone(),
        legende: row.legende.clone(),
        expire_at: row.expire_at,
        created_at: row.created_at,
        vue: row.vue,
        nombre_vues: if est_auteur { Some(row.nombre_vues) } else { None },
    }
}

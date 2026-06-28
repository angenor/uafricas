use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::afrolang::{AdministrateurLight, DesactivationAdminInfo, PaysOrigineLight};

// ── Colonnes SQL ──────────────────────────────────────────────
// feature 005 : la colonne `salle.moderateur_id` est supprimée —
// la modération attitrée passe désormais par `salle_moderateur`.

pub const ADMIN_SALLE_LISTE_COLONNES: &str =
    "s.id, s.titre, s.slug, s.langue_cible, s.langue_code, s.actif,
     s.desactivee_admin_at, s.created_at, s.groupe_ethnique_libre,
     ge.nom AS groupe_ethnique_nom,
     (SELECT COUNT(*) FROM afrolang.salle_privee sp
        WHERE sp.salle_id = s.id AND sp.actif = true
          AND sp.archivee_at IS NULL AND sp.deleted_at IS NULL)
        AS nombre_salles_privees,
     (SELECT COUNT(*) FROM afrolang.session se
        JOIN afrolang.salle_privee sp2 ON se.salle_privee_id = sp2.id
        WHERE sp2.salle_id = s.id AND sp2.actif = true) AS nombre_sessions,
     (SELECT COUNT(*) FROM afrolang.salle_moderateur sm
        WHERE sm.salle_id = s.id AND sm.actif = TRUE) AS nombre_moderateurs_attitres";

pub const ADMIN_SALLE_DETAIL_COLONNES: &str =
    "s.id, s.titre, s.slug, s.description, s.image_couverture_url,
     s.langue_cible, s.langue_code, s.alphabet, s.dictionnaire_url,
     s.groupe_ethnique_id, s.groupe_ethnique_libre, s.actif,
     s.cree_par, s.created_at, s.updated_at,
     s.desactivee_admin_at, s.motif_desactivation,
     ge.nom AS groupe_ethnique_nom,
     cr.nom AS createur_nom, cr.prenom AS createur_prenom,
     (SELECT COUNT(*) FROM afrolang.salle_privee sp
        WHERE sp.salle_id = s.id AND sp.actif = true
          AND sp.archivee_at IS NULL AND sp.deleted_at IS NULL)
        AS nombre_salles_privees,
     (SELECT COUNT(*) FROM afrolang.session se
        JOIN afrolang.salle_privee sp2 ON se.salle_privee_id = sp2.id
        WHERE sp2.salle_id = s.id AND sp2.actif = true) AS nombre_sessions,
     (SELECT COUNT(*) FROM afrolang.salle_moderateur sm
        WHERE sm.salle_id = s.id AND sm.actif = TRUE) AS nombre_moderateurs_attitres,
     COALESCE((SELECT json_agg(json_build_object(
                  'id', po.id,
                  'nom', po.nom,
                  'code_iso2', po.code_iso2
               ) ORDER BY po.nom)
               FROM afrolang.salle_pays_origine spo
               JOIN shared.pays po ON po.id = spo.pays_id
               WHERE spo.salle_id = s.id),
              '[]'::json) AS pays_origine_json,
     COALESCE((SELECT json_agg(json_build_object(
                  'utilisateur_id', sa.utilisateur_id,
                  'nom', ua.nom,
                  'prenom', ua.prenom,
                  'photo_url', ua.photo_url,
                  'nomme_at', sa.nomme_at
               ) ORDER BY sa.nomme_at ASC)
               FROM afrolang.salle_administrateur sa
               JOIN iam.utilisateur ua ON ua.id = sa.utilisateur_id
               WHERE sa.salle_id = s.id AND sa.actif = TRUE),
              '[]'::json) AS administrateurs_json";

pub const SALLE_TRI_COLONNES: &[&str] = &[
    "created_at", "titre", "langue_cible", "langue_code",
];

// ── Structs liste ─────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSalleListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub langue_cible: Option<String>,
    pub langue_code: Option<String>,
    pub actif: bool,
    #[sqlx(default)]
    pub desactivee_admin_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub groupe_ethnique_nom: Option<String>,
    pub nombre_salles_privees: Option<i64>,
    pub nombre_sessions: Option<i64>,
    pub nombre_moderateurs_attitres: Option<i64>,
}

// ── Structs détail ────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminSalleDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub langue_cible: Option<String>,
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub groupe_ethnique_libre: Option<String>,
    pub actif: bool,
    pub cree_par: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(default)]
    pub desactivee_admin_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub motif_desactivation: Option<String>,
    pub groupe_ethnique_nom: Option<String>,
    pub createur_nom: Option<String>,
    pub createur_prenom: Option<String>,
    pub nombre_salles_privees: Option<i64>,
    pub nombre_sessions: Option<i64>,
    pub nombre_moderateurs_attitres: Option<i64>,
    pub pays_origine_json: Option<serde_json::Value>,
    #[sqlx(default)]
    pub administrateurs_json: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct AdminSalleDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub langue_cible: Option<String>,
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub groupe_ethnique_libre: Option<String>,
    pub groupe_ethnique_nom: Option<String>,
    pub actif: bool,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub nombre_salles_privees: i64,
    pub nombre_sessions: i64,
    pub nombre_moderateurs_attitres: i64,
    pub pays_origine: Vec<PaysOrigineLight>,
    pub administrateurs: Vec<AdministrateurLight>,
    pub desactivee_admin: Option<DesactivationAdminInfo>,
}

impl AdminSalleDetailRow {
    pub fn to_response(self) -> AdminSalleDetailResponse {
        let cree_par_nom = match (&self.createur_prenom, &self.createur_nom) {
            (Some(p), Some(n)) => Some(format!("{} {}", p, n)),
            _ => None,
        };
        let pays_origine = self
            .pays_origine_json
            .as_ref()
            .and_then(|v| serde_json::from_value::<Vec<PaysOrigineLight>>(v.clone()).ok())
            .unwrap_or_default();
        let administrateurs = self
            .administrateurs_json
            .as_ref()
            .and_then(|v| serde_json::from_value::<Vec<AdministrateurLight>>(v.clone()).ok())
            .unwrap_or_default();
        AdminSalleDetailResponse {
            id: self.id,
            titre: self.titre,
            slug: self.slug,
            description: self.description,
            image_couverture_url: self.image_couverture_url,
            langue_cible: self.langue_cible,
            langue_code: self.langue_code,
            alphabet: self.alphabet,
            dictionnaire_url: self.dictionnaire_url,
            groupe_ethnique_id: self.groupe_ethnique_id,
            groupe_ethnique_libre: self.groupe_ethnique_libre,
            groupe_ethnique_nom: self.groupe_ethnique_nom,
            actif: self.actif,
            cree_par_nom,
            created_at: self.created_at,
            updated_at: self.updated_at,
            nombre_salles_privees: self.nombre_salles_privees.unwrap_or(0),
            nombre_sessions: self.nombre_sessions.unwrap_or(0),
            nombre_moderateurs_attitres: self.nombre_moderateurs_attitres.unwrap_or(0),
            pays_origine,
            administrateurs,
            // Contexte admin : on expose le motif détaillé (FR-020).
            desactivee_admin: self
                .desactivee_admin_at
                .map(|desactivee_at| DesactivationAdminInfo {
                    desactivee_at,
                    motif: self.motif_desactivation,
                }),
        }
    }
}

// ── Requêtes ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerSalleRequest {
    pub titre: String,
    pub description: Option<String>,
    pub langue_cible: Option<String>,
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
    pub groupe_ethnique_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ModifierSalleRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub langue_cible: Option<String>,
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub actif: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AdminSalleQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub langue_cible: Option<String>,
    pub langue_code: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub actif: Option<bool>,
}

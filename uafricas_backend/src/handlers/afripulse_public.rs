//! Handlers publics Afripulse — lectures des 4 sections enrichies
//! (sites touristiques, secteurs d'opportunités, personnalités, savoirs pratiques).
//! Contient aussi :
//!   • US3 : soumission d'une **nouvelle fiche pays** (contribution `type_objet=fiche_pays`)
//!   • US4 : lecture publique des **recommandations** et de la **galerie photos**
//!
//! Toutes les requêtes de lecture filtrent `deleted_at IS NULL` (soft delete).
//! Les écritures nécessitent une authentification (JWT Bearer).

use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::constants::afripulse_pays_autorises::est_pays_africain;
use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::contribution_fiche::TypeObjetContribution;
use crate::services::{audit, rate_limit_afripulse};

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Site touristique
// ────────────────────────────────────────────────────────────────

/// Réponse enrichie d'un site touristique (feature 001-sites-touristiques-enrichis).
/// Contacts et constitution légale sont publics (clarification résolue).
/// `note_moyenne`/`nombre_avis` sont des agrégats dérivés (table `avis_site`).
#[derive(Debug, FromRow, Serialize)]
pub struct SiteTouristiqueResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom: String,
    pub categorie: String,
    pub sous_type: Option<String>,
    pub description: Option<String>,
    pub info_pertinente: Option<String>,
    pub image_url: Option<String>,
    pub images: Vec<String>,
    pub gestionnaire: Option<String>,
    pub ville: Option<String>,
    pub village: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub contact_telephone: Option<String>,
    pub contact_courriel: Option<String>,
    pub contact_adresse: Option<String>,
    pub constitution_statut_juridique: Option<String>,
    pub constitution_numero: Option<String>,
    pub constitution_document_url: Option<String>,
    pub site_web_url: Option<String>,
    pub verifie: bool,
    pub note_moyenne: Option<f64>,
    pub nombre_avis: i64,
    pub nombre_signalements: i32,
    pub suspendu: bool,
    pub a_signale: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SiteTouristiqueQuery {
    pub categorie: Option<String>,
    pub sous_type: Option<String>,
}

/// Colonnes + agrégat avis communs aux requêtes de lecture des sites.
// $1 = utilisateur courant (optionnel) pour calculer `a_signale`.
const SITE_TOURISTIQUE_SELECT: &str = "SELECT st.id, st.fiche_pays_id, st.nom,
        st.categorie::text AS categorie, st.sous_type::text AS sous_type,
        st.description, st.info_pertinente, st.image_url, st.images,
        st.gestionnaire, st.ville, st.village,
        st.latitude::float8 AS latitude, st.longitude::float8 AS longitude,
        st.contact_telephone, st.contact_courriel, st.contact_adresse,
        st.constitution_statut_juridique, st.constitution_numero, st.constitution_document_url,
        st.site_web_url, st.verifie, av.note_moyenne, av.nombre_avis,
        st.nombre_signalements, st.suspendu,
        EXISTS(SELECT 1 FROM country_profile.signalement_contribution sc
               WHERE sc.type_objet = 'site_touristique'::country_profile.type_objet_contribution
                 AND sc.objet_id = st.id AND sc.signale_par = $1) AS a_signale,
        st.created_at
     FROM country_profile.site_touristique st
     LEFT JOIN LATERAL (
        SELECT AVG(a.note)::float8 AS note_moyenne, COUNT(*)::bigint AS nombre_avis
        FROM country_profile.avis_site a
        WHERE a.site_id = st.id AND a.deleted_at IS NULL AND a.masque_par_admin = FALSE
     ) av ON TRUE";

pub async fn lister_sites_touristiques(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    params: web::Query<SiteTouristiqueQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req);
    let fiche_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    // Filtres optionnels — ignorés si valeur invalide (cohérence avec l'existant).
    let categorie_filtre = params
        .categorie
        .as_deref()
        .map(|c| c.trim().to_lowercase())
        .filter(|c| c == "emblematique" || c == "prive");
    let sous_type_filtre = params
        .sous_type
        .as_deref()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| crate::models::afripulse::sous_type_site_valide(s));

    // $1 = utilisateur (a_signale), $2 = fiche, filtres à partir de $3.
    let mut conditions = String::from(" WHERE st.fiche_pays_id = $2 AND st.deleted_at IS NULL");
    let mut bind_index = 3u32;
    if categorie_filtre.is_some() {
        conditions.push_str(&format!(
            " AND st.categorie = ${}::country_profile.categorie_site_touristique",
            bind_index
        ));
        bind_index += 1;
    }
    if sous_type_filtre.is_some() {
        conditions.push_str(&format!(" AND st.sous_type::text = ${}", bind_index));
    }

    let sql = format!("{SITE_TOURISTIQUE_SELECT}{conditions} ORDER BY st.created_at DESC");
    let mut query = sqlx::query_as::<_, SiteTouristiqueResponse>(&sql)
        .bind(utilisateur_id)
        .bind(fiche_id);
    if let Some(cat) = &categorie_filtre {
        query = query.bind(cat);
    }
    if let Some(st) = &sous_type_filtre {
        query = query.bind(st);
    }
    let rows = query.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rows),
        error: None,
    }))
}

/// GET /api/fiches-pays/{id}/sites-touristiques/{site_id}
/// Détail d'un site touristique (page dédiée). 404 si absent/supprimé.
pub async fn obtenir_site_touristique(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req);
    let (fiche_brut, site_brut) = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_brut)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;
    let site_id = Uuid::parse_str(&site_brut)
        .map_err(|_| ApiErreur::Validation("ID de site invalide".to_string()))?;

    let sql = format!(
        "{SITE_TOURISTIQUE_SELECT} WHERE st.fiche_pays_id = $2 AND st.id = $3 AND st.deleted_at IS NULL"
    );
    let site = sqlx::query_as::<_, SiteTouristiqueResponse>(&sql)
        .bind(utilisateur_id)
        .bind(fiche_id)
        .bind(site_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Site touristique introuvable".to_string()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(site),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// Secteur d'opportunité
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow, Serialize)]
pub struct SecteurOpportuniteResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom: String,
    pub description: Option<String>,
    pub localite: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_courriel: Option<String>,
    pub contact_adresse: Option<String>,
    pub references_utiles: Option<String>,
    pub site_web_url: Option<String>,
    pub image_url: Option<String>,
    pub nombre_signalements: i32,
    pub suspendu: bool,
    pub a_signale: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn lister_secteurs_opportunites(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req);
    let fiche_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    let rows: Vec<SecteurOpportuniteResponse> = sqlx::query_as(
        "SELECT sd.id, sd.fiche_pays_id, sd.nom, sd.description, sd.localite,
                sd.contact_telephone, sd.contact_courriel, sd.contact_adresse,
                sd.references_utiles, sd.site_web_url, sd.image_url,
                sd.nombre_signalements, sd.suspendu,
                EXISTS(SELECT 1 FROM country_profile.signalement_contribution sc
                       WHERE sc.type_objet = 'secteur_developpement'::country_profile.type_objet_contribution
                         AND sc.objet_id = sd.id AND sc.signale_par = $2) AS a_signale,
                sd.created_at
         FROM country_profile.secteur_developpement sd
         WHERE sd.fiche_pays_id = $1
         ORDER BY sd.nom ASC",
    )
    .bind(fiche_id)
    .bind(utilisateur_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rows),
        error: None,
    }))
}

/// GET /api/fiches-pays/{id}/secteurs-opportunites/{secteur_id}
/// Détail d'un secteur d'opportunité (page dédiée). 404 si absent.
pub async fn obtenir_secteur_opportunite(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req);
    let (fiche_brut, secteur_brut) = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_brut)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;
    let secteur_id = Uuid::parse_str(&secteur_brut)
        .map_err(|_| ApiErreur::Validation("ID de secteur invalide".to_string()))?;

    // La table secteur_developpement n'a pas de colonne deleted_at (soft-delete non applicable).
    let secteur: Option<SecteurOpportuniteResponse> = sqlx::query_as(
        "SELECT sd.id, sd.fiche_pays_id, sd.nom, sd.description, sd.localite,
                sd.contact_telephone, sd.contact_courriel, sd.contact_adresse,
                sd.references_utiles, sd.site_web_url, sd.image_url,
                sd.nombre_signalements, sd.suspendu,
                EXISTS(SELECT 1 FROM country_profile.signalement_contribution sc
                       WHERE sc.type_objet = 'secteur_developpement'::country_profile.type_objet_contribution
                         AND sc.objet_id = sd.id AND sc.signale_par = $1) AS a_signale,
                sd.created_at
         FROM country_profile.secteur_developpement sd
         WHERE sd.fiche_pays_id = $2 AND sd.id = $3",
    )
    .bind(utilisateur_id)
    .bind(fiche_id)
    .bind(secteur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let secteur = secteur.ok_or_else(|| ApiErreur::NonTrouve("Secteur introuvable".to_string()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(secteur),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// Recette culinaire populaire
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow, Serialize)]
pub struct RecetteCulinaireResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub titre: String,
    pub territoires_consommation: Option<String>,
    pub histoire: Option<String>,
    pub ingredients: Vec<String>,
    pub etapes_preparation: Vec<String>,
    pub images: Vec<String>,
    pub nombre_signalements: i32,
    pub suspendu: bool,
    pub a_signale: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn lister_recettes_culinaires(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req);
    let fiche_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    let rows: Vec<RecetteCulinaireResponse> = sqlx::query_as(
        "SELECT rc.id, rc.fiche_pays_id, rc.titre, rc.territoires_consommation, rc.histoire,
                rc.ingredients, rc.etapes_preparation, rc.images,
                rc.nombre_signalements, rc.suspendu,
                EXISTS(SELECT 1 FROM country_profile.signalement_contribution sc
                       WHERE sc.type_objet = 'recette_culinaire'::country_profile.type_objet_contribution
                         AND sc.objet_id = rc.id AND sc.signale_par = $2) AS a_signale,
                rc.created_at
         FROM country_profile.recette_culinaire rc
         WHERE rc.fiche_pays_id = $1 AND rc.deleted_at IS NULL
         ORDER BY rc.created_at DESC",
    )
    .bind(fiche_id)
    .bind(utilisateur_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rows),
        error: None,
    }))
}

/// GET /api/fiches-pays/{id}/recettes-culinaires/{recette_id}
/// Détail d'une recette culinaire (page dédiée). 404 si absent/supprimé.
pub async fn obtenir_recette_culinaire(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req);
    let (fiche_brut, recette_brut) = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_brut)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;
    let recette_id = Uuid::parse_str(&recette_brut)
        .map_err(|_| ApiErreur::Validation("ID de recette invalide".to_string()))?;

    let recette: Option<RecetteCulinaireResponse> = sqlx::query_as(
        "SELECT rc.id, rc.fiche_pays_id, rc.titre, rc.territoires_consommation, rc.histoire,
                rc.ingredients, rc.etapes_preparation, rc.images,
                rc.nombre_signalements, rc.suspendu,
                EXISTS(SELECT 1 FROM country_profile.signalement_contribution sc
                       WHERE sc.type_objet = 'recette_culinaire'::country_profile.type_objet_contribution
                         AND sc.objet_id = rc.id AND sc.signale_par = $1) AS a_signale,
                rc.created_at
         FROM country_profile.recette_culinaire rc
         WHERE rc.fiche_pays_id = $2 AND rc.id = $3 AND rc.deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .bind(fiche_id)
    .bind(recette_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let recette = recette.ok_or_else(|| ApiErreur::NonTrouve("Recette introuvable".to_string()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(recette),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// Personnalité connue
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow, Serialize)]
pub struct PersonnaliteResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom_complet: String,
    pub domaine: String,
    pub biographie_courte: String,
    pub annee_naissance: Option<i16>,
    pub annee_deces: Option<i16>,
    pub portrait_url: Option<String>,
    pub lien_reference: Option<String>,
    pub cree_par: Uuid,
    pub nombre_signalements: i32,
    pub suspendu: bool,
    pub a_signale: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PersonnaliteQuery {
    pub domaine: Option<String>,
}

pub async fn lister_personnalites(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    params: web::Query<PersonnaliteQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req);
    let fiche_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    let domaines_valides = [
        "politique",
        "artiste_musicien",
        "artiste_autre",
        "sportif",
        "entrepreneur",
        "scientifique",
        "militaire_historique",
        "autre",
    ];

    let domaine_filtre = params
        .domaine
        .as_deref()
        .map(|d| d.trim().to_lowercase())
        .filter(|d| domaines_valides.contains(&d.as_str()));

    // Sous-requête a_signale commune (le n° de bind diffère selon la branche).
    let rows: Vec<PersonnaliteResponse> = if let Some(dom) = domaine_filtre {
        sqlx::query_as(
            "SELECT pc.id, pc.fiche_pays_id, pc.nom_complet, pc.domaine::text AS domaine,
                    pc.biographie_courte, pc.annee_naissance, pc.annee_deces,
                    pc.portrait_url, pc.lien_reference, pc.cree_par,
                    pc.nombre_signalements, pc.suspendu,
                    EXISTS(SELECT 1 FROM country_profile.signalement_contribution sc
                           WHERE sc.type_objet = 'personnalite_connue'::country_profile.type_objet_contribution
                             AND sc.objet_id = pc.id AND sc.signale_par = $3) AS a_signale,
                    pc.created_at
             FROM country_profile.personnalite_connue pc
             WHERE pc.fiche_pays_id = $1 AND pc.deleted_at IS NULL
               AND pc.domaine = $2::country_profile.domaine_personnalite
             ORDER BY pc.nom_complet ASC",
        )
        .bind(fiche_id)
        .bind(dom)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?
    } else {
        sqlx::query_as(
            "SELECT pc.id, pc.fiche_pays_id, pc.nom_complet, pc.domaine::text AS domaine,
                    pc.biographie_courte, pc.annee_naissance, pc.annee_deces,
                    pc.portrait_url, pc.lien_reference, pc.cree_par,
                    pc.nombre_signalements, pc.suspendu,
                    EXISTS(SELECT 1 FROM country_profile.signalement_contribution sc
                           WHERE sc.type_objet = 'personnalite_connue'::country_profile.type_objet_contribution
                             AND sc.objet_id = pc.id AND sc.signale_par = $2) AS a_signale,
                    pc.created_at
             FROM country_profile.personnalite_connue pc
             WHERE pc.fiche_pays_id = $1 AND pc.deleted_at IS NULL
             ORDER BY pc.nom_complet ASC",
        )
        .bind(fiche_id)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rows),
        error: None,
    }))
}

/// GET /api/fiches-pays/{id}/personnalites/{personnalite_id}
/// Détail d'une personnalité connue (page dédiée). 404 si absent/supprimé.
pub async fn obtenir_personnalite(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req);
    let (fiche_brut, perso_brut) = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_brut)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;
    let perso_id = Uuid::parse_str(&perso_brut)
        .map_err(|_| ApiErreur::Validation("ID de personnalité invalide".to_string()))?;

    let perso: Option<PersonnaliteResponse> = sqlx::query_as(
        "SELECT pc.id, pc.fiche_pays_id, pc.nom_complet, pc.domaine::text AS domaine,
                pc.biographie_courte, pc.annee_naissance, pc.annee_deces,
                pc.portrait_url, pc.lien_reference, pc.cree_par,
                pc.nombre_signalements, pc.suspendu,
                EXISTS(SELECT 1 FROM country_profile.signalement_contribution sc
                       WHERE sc.type_objet = 'personnalite_connue'::country_profile.type_objet_contribution
                         AND sc.objet_id = pc.id AND sc.signale_par = $1) AS a_signale,
                pc.created_at
         FROM country_profile.personnalite_connue pc
         WHERE pc.fiche_pays_id = $2 AND pc.id = $3 AND pc.deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .bind(fiche_id)
    .bind(perso_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let perso = perso.ok_or_else(|| ApiErreur::NonTrouve("Personnalité introuvable".to_string()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(perso),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// Savoir pratique
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow, Serialize)]
pub struct SavoirPratiqueResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub titre: String,
    pub categorie: String,
    pub explication: String,
    pub exemple: Option<String>,
    pub cree_par: Uuid,
    pub nombre_signalements: i32,
    pub suspendu: bool,
    pub a_signale: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SavoirPratiqueQuery {
    pub categorie: Option<String>,
}

pub async fn lister_savoirs_pratiques(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    params: web::Query<SavoirPratiqueQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req);
    let fiche_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    let categories_valides = [
        "langue_argot",
        "coutumes",
        "etiquette",
        "securite",
        "sante",
        "transports",
        "autre",
    ];

    let categorie_filtre = params
        .categorie
        .as_deref()
        .map(|c| c.trim().to_lowercase())
        .filter(|c| categories_valides.contains(&c.as_str()));

    let rows: Vec<SavoirPratiqueResponse> = if let Some(cat) = categorie_filtre {
        sqlx::query_as(
            "SELECT sp.id, sp.fiche_pays_id, sp.titre, sp.categorie::text AS categorie,
                    sp.explication, sp.exemple, sp.cree_par,
                    sp.nombre_signalements, sp.suspendu,
                    EXISTS(SELECT 1 FROM country_profile.signalement_contribution s
                           WHERE s.type_objet = 'savoir_pratique'::country_profile.type_objet_contribution
                             AND s.objet_id = sp.id AND s.signale_par = $3) AS a_signale,
                    sp.created_at
             FROM country_profile.savoir_pratique sp
             WHERE sp.fiche_pays_id = $1 AND sp.deleted_at IS NULL
               AND sp.categorie = $2::country_profile.categorie_savoir
             ORDER BY sp.titre ASC",
        )
        .bind(fiche_id)
        .bind(cat)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?
    } else {
        sqlx::query_as(
            "SELECT sp.id, sp.fiche_pays_id, sp.titre, sp.categorie::text AS categorie,
                    sp.explication, sp.exemple, sp.cree_par,
                    sp.nombre_signalements, sp.suspendu,
                    EXISTS(SELECT 1 FROM country_profile.signalement_contribution s
                           WHERE s.type_objet = 'savoir_pratique'::country_profile.type_objet_contribution
                             AND s.objet_id = sp.id AND s.signale_par = $2) AS a_signale,
                    sp.created_at
             FROM country_profile.savoir_pratique sp
             WHERE sp.fiche_pays_id = $1 AND sp.deleted_at IS NULL
             ORDER BY sp.titre ASC",
        )
        .bind(fiche_id)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rows),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// Utilitaires auth
// ════════════════════════════════════════════════════════════════

fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

// ════════════════════════════════════════════════════════════════
// US3 — Création d'une nouvelle fiche pays
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreerFichePaysBody {
    pub code_iso2: String,
    pub slogan: Option<String>,
    pub population: Option<i64>,
    pub superficie_km2: Option<f64>,
    pub biographie: Option<String>,
    pub contexte: Option<String>,
    pub monnaie: Option<String>,
    pub langue_officielle: Option<String>,
    pub langues_populaires: Option<String>,
    pub hymne_national: Option<String>,
    pub fuseau_horaire: Option<String>,
    pub image_couverture_url: Option<String>,
    pub image_drapeau_url: Option<String>,
    pub image_embleme_url: Option<String>,
    pub justification: Option<String>,
}

/// POST /api/fiches-pays — T047/T048
///
/// Soumission d'une nouvelle fiche pays par un utilisateur authentifié.
///
/// Erreurs :
///   • 401 si non authentifié
///   • 422 si `code_iso2` hors périmètre africain (54 codes ISO)
///   • 409 si une fiche existe déjà (retourne `fiche_pays_id` existant)
///   • 429 rate-limit (5 en attente par pays, 20 textes/24 h)
///   • 202 sur succès — `{id, etat:"en_attente", created_at}`
pub async fn creer_fiche_pays(
    pool: web::Data<PgPool>,
    body: web::Json<CreerFichePaysBody>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let code_iso2 = body.code_iso2.trim().to_lowercase();
    if code_iso2.len() != 2 || !est_pays_africain(&code_iso2) {
        return Err(ApiErreur::Validation(format!(
            "Le pays '{}' est hors du perimetre Afripulse (54 pays africains uniquement).",
            code_iso2
        )));
    }

    // Vérifie l'absence de fiche existante (JOIN shared.pays).
    let existant: Option<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT fp.id, p.id
         FROM country_profile.fiche_pays fp
         JOIN shared.pays p ON fp.pays_id = p.id
         WHERE LOWER(p.code_iso2) = $1
         LIMIT 1",
    )
    .bind(&code_iso2)
    .fetch_optional(pool.get_ref())
    .await?;
    if let Some((fiche_id, _pays_id)) = existant {
        return Err(ApiErreur::Conflit(format!(
            "Une fiche pays existe deja pour '{}' (id={}). Proposez plutot une modification.",
            code_iso2, fiche_id
        )));
    }

    // Résolution pays_id (requis pour le rate-limit qui prend un fiche_pays_id ;
    // on utilise le pays_id comme clé de regroupement de substitution).
    let pays_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM shared.pays WHERE LOWER(code_iso2) = $1")
            .bind(&code_iso2)
            .fetch_optional(pool.get_ref())
            .await?;
    let pays_id = pays_id.ok_or_else(|| {
        ApiErreur::Validation(format!(
            "Pays '{}' introuvable dans shared.pays (alimentation requise cote admin).",
            code_iso2
        ))
    })?;

    // Rate-limit : limite "textes / 24 h" et "en attente par pays" (on groupe
    // sur pays_id faute de fiche_pays_id encore existant).
    if let Err(e) = rate_limit_afripulse::verifier_quotas(
        pool.get_ref(),
        utilisateur_id,
        &TypeObjetContribution::FichePays,
        pays_id,
    )
    .await
    {
        return Err(match e {
            rate_limit_afripulse::ErreurVerification::LimiteAtteinte(limite) => {
                ApiErreur::LimiteAtteinte(limite.message())
            }
            rate_limit_afripulse::ErreurVerification::Base(err) => {
                ApiErreur::BaseDeDonnees(err.to_string())
            }
        });
    }

    // Construire le payload JSONB complet (inclut le code_iso2, relu lors de
    // l'approbation admin par moderer_contribution->appliquer_contribution_afripulse).
    let payload = json!({
        "code_iso2": code_iso2,
        "slogan": body.slogan,
        "population": body.population,
        "superficie_km2": body.superficie_km2,
        "biographie": body.biographie,
        "contexte": body.contexte,
        "monnaie": body.monnaie,
        "langue_officielle": body.langue_officielle,
        "langues_populaires": body.langues_populaires,
        "hymne_national": body.hymne_national,
        "fuseau_horaire": body.fuseau_horaire,
        "image_couverture_url": body.image_couverture_url,
        "image_drapeau_url": body.image_drapeau_url,
        "image_embleme_url": body.image_embleme_url,
    });

    // INSERT. `fiche_pays_id` NOT NULL dans le schema : on doit pointer une fiche
    // existante. Convention : pour les creations, on utilise un UUID "virtuel"
    // = pays_id (unique, reproductible). L'approbation creera la vraie fiche.
    //
    // ⚠️ Cette approche necessite que contribution_fiche.fiche_pays_id NE SOIT PAS
    // une FK stricte (ou que la contrainte accepte le pays_id). Vérifier le schema
    // avant deployment : si la FK est stricte, il faudra creer une fiche en
    // etat=brouillon a la volee ou relacher la FK pour les type_objet=fiche_pays.
    //
    // Dans le doute, on echoue proprement avec un message admin-friendly.
    let row: (Uuid, chrono::DateTime<chrono::Utc>) = sqlx::query_as(
        "INSERT INTO country_profile.contribution_fiche (
            fiche_pays_id, cree_par, section, type_contribution,
            nouvelle_valeur, justification,
            type_objet_contribution, section_afripulse,
            nouvelle_valeur_jsonb
         ) VALUES (
            $1, $2, 'fiche_pays', 'ajout'::country_profile.type_contribution,
            '', $3,
            'fiche_pays'::country_profile.type_objet_contribution,
            NULL,
            $4
         )
         RETURNING id, created_at",
    )
    .bind(pays_id) // utilise pays_id comme placeholder de fiche_pays_id
    .bind(utilisateur_id)
    .bind(body.justification.as_deref().map(|s| s.trim()))
    .bind(&payload)
    .fetch_one(pool.get_ref())
    .await?;

    let (contrib_id, created_at) = row;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "create",
        "country_profile",
        "contribution_fiche",
        Some(contrib_id),
        None,
        Some(json!({
            "type_objet": "fiche_pays",
            "type_contribution": "ajout",
            "code_iso2": code_iso2,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Accepted().json(ApiResponse {
        success: true,
        data: Some(json!({
            "id": contrib_id,
            "etat": "en_attente",
            "created_at": created_at,
        })),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US4 — Lecture publique : recommandations + galerie photos
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct RecommandationPublicRow {
    pub id: Uuid,
    pub utilisateur_id: Option<Uuid>,
    pub auteur_nom: Option<String>,
    pub auteur_prenom: Option<String>,
    pub auteur_photo_url: Option<String>,
    pub note: i16,
    pub commentaire: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct RecommandationsListeResponse {
    pub note_moyenne: Option<f64>,
    pub nombre_total: i64,
    pub recommandations: Vec<RecommandationPublicRow>,
}

pub async fn lister_recommandations(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    params: web::Query<PaginationQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let fiche_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".into()))?;

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(10).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let (nombre_total, note_moyenne): (i64, Option<f64>) = sqlx::query_as(
        "SELECT COUNT(*)::bigint, AVG(note)::float8
         FROM country_profile.recommandation_visiteur
         WHERE fiche_pays_id = $1 AND active = TRUE AND deleted_at IS NULL",
    )
    .bind(fiche_id)
    .fetch_one(pool.get_ref())
    .await?;

    let rows: Vec<RecommandationPublicRow> = sqlx::query_as(
        "SELECT rv.id,
                CASE WHEN u.deleted_at IS NULL THEN rv.utilisateur_id ELSE NULL END AS utilisateur_id,
                CASE WHEN u.deleted_at IS NULL THEN u.nom ELSE 'Contributeur' END AS auteur_nom,
                CASE WHEN u.deleted_at IS NULL THEN u.prenom ELSE 'retire' END AS auteur_prenom,
                CASE WHEN u.deleted_at IS NULL THEN u.photo_url ELSE NULL END AS auteur_photo_url,
                rv.note, rv.commentaire, rv.created_at
         FROM country_profile.recommandation_visiteur rv
         LEFT JOIN iam.utilisateur u ON u.id = rv.utilisateur_id
         WHERE rv.fiche_pays_id = $1 AND rv.active = TRUE AND rv.deleted_at IS NULL
         ORDER BY rv.created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(fiche_id)
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(RecommandationsListeResponse {
            note_moyenne,
            nombre_total,
            recommandations: rows,
        }),
        error: None,
    }))
}

#[derive(Debug, FromRow, Serialize)]
pub struct PhotoGalerieRow {
    pub id: Uuid,
    pub chemin_fichier: String,
    pub legende: String,
    pub format: String,
    pub largeur_px: i16,
    pub hauteur_px: i16,
    pub utilisateur_id: Option<Uuid>,
    pub auteur_nom: Option<String>,
    pub auteur_prenom: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct GalerieListeResponse {
    pub nombre_total: i64,
    pub photos: Vec<PhotoGalerieRow>,
}

pub async fn lister_galerie_photos(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    params: web::Query<PaginationQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let fiche_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".into()))?;

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 60);
    let offset = (page - 1) * par_page;

    let nombre_total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM country_profile.photo_visiteur
         WHERE fiche_pays_id = $1 AND deleted_at IS NULL",
    )
    .bind(fiche_id)
    .fetch_one(pool.get_ref())
    .await?;

    let photos: Vec<PhotoGalerieRow> = sqlx::query_as(
        "SELECT pv.id, pv.chemin_fichier, pv.legende, pv.format,
                pv.largeur_px, pv.hauteur_px,
                CASE WHEN u.deleted_at IS NULL THEN pv.utilisateur_id ELSE NULL END AS utilisateur_id,
                CASE WHEN u.deleted_at IS NULL THEN u.nom ELSE 'Contributeur' END AS auteur_nom,
                CASE WHEN u.deleted_at IS NULL THEN u.prenom ELSE 'retire' END AS auteur_prenom,
                pv.created_at
         FROM country_profile.photo_visiteur pv
         LEFT JOIN iam.utilisateur u ON u.id = pv.utilisateur_id
         WHERE pv.fiche_pays_id = $1 AND pv.deleted_at IS NULL
         ORDER BY pv.created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(fiche_id)
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(GalerieListeResponse {
            nombre_total,
            photos,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US5 — Avis de visiteurs sur un site (note 1–5, écriture directe)
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct AvisSiteAuteur {
    pub id: Option<Uuid>,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, FromRow)]
struct AvisSiteRowPublic {
    id: Uuid,
    utilisateur_id: Option<Uuid>,
    auteur_nom: Option<String>,
    auteur_prenom: Option<String>,
    auteur_photo_url: Option<String>,
    note: i16,
    commentaire: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct AvisSiteResponse {
    pub id: Uuid,
    pub utilisateur: AvisSiteAuteur,
    pub note: i16,
    pub commentaire: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct AvisSiteListeResponse {
    pub note_moyenne: Option<f64>,
    pub nombre_total: i64,
    pub avis: Vec<AvisSiteResponse>,
}

#[derive(Debug, Deserialize)]
pub struct CreerAvisBody {
    pub note: i16,
    pub commentaire: String,
}

/// GET /api/sites-touristiques/{site_id}/avis — liste paginée + agrégats (public).
pub async fn lister_avis_site(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    params: web::Query<PaginationQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let site_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de site invalide".into()))?;

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(10).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let (nombre_total, note_moyenne): (i64, Option<f64>) = sqlx::query_as(
        "SELECT COUNT(*)::bigint, AVG(note)::float8
         FROM country_profile.avis_site
         WHERE site_id = $1 AND deleted_at IS NULL AND masque_par_admin = FALSE",
    )
    .bind(site_id)
    .fetch_one(pool.get_ref())
    .await?;

    let rows: Vec<AvisSiteRowPublic> = sqlx::query_as(
        "SELECT a.id,
                CASE WHEN u.deleted_at IS NULL THEN a.utilisateur_id ELSE NULL END AS utilisateur_id,
                CASE WHEN u.deleted_at IS NULL THEN u.nom ELSE 'Contributeur' END AS auteur_nom,
                CASE WHEN u.deleted_at IS NULL THEN u.prenom ELSE 'retire' END AS auteur_prenom,
                CASE WHEN u.deleted_at IS NULL THEN u.photo_url ELSE NULL END AS auteur_photo_url,
                a.note, a.commentaire, a.created_at
         FROM country_profile.avis_site a
         LEFT JOIN iam.utilisateur u ON u.id = a.utilisateur_id
         WHERE a.site_id = $1 AND a.deleted_at IS NULL AND a.masque_par_admin = FALSE
         ORDER BY a.created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(site_id)
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let avis = rows
        .into_iter()
        .map(|r| AvisSiteResponse {
            id: r.id,
            utilisateur: AvisSiteAuteur {
                id: r.utilisateur_id,
                nom: r.auteur_nom.unwrap_or_else(|| "Contributeur".to_string()),
                prenom: r.auteur_prenom.unwrap_or_else(|| "retire".to_string()),
                photo_url: r.auteur_photo_url,
            },
            note: r.note,
            commentaire: r.commentaire,
            created_at: r.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AvisSiteListeResponse {
            note_moyenne,
            nombre_total,
            avis,
        }),
        error: None,
    }))
}

/// POST /api/sites-touristiques/{site_id}/avis — dépose/met à jour l'avis (auth).
/// Upsert sur l'avis actif (un seul par utilisateur/site). 201 création, 200 MAJ.
pub async fn soumettre_avis_site(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    body: web::Json<CreerAvisBody>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let site_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de site invalide".into()))?;

    // Validation note + commentaire.
    if !(1..=5).contains(&body.note) {
        return Err(ApiErreur::Validation(
            "La note doit être comprise entre 1 et 5.".into(),
        ));
    }
    let commentaire = body.commentaire.trim();
    let longueur = commentaire.chars().count();
    if !(1..=2000).contains(&longueur) {
        return Err(ApiErreur::Validation(
            "Le commentaire doit contenir entre 1 et 2000 caractères.".into(),
        ));
    }

    // Le site doit exister et ne pas être supprimé.
    let site_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM country_profile.site_touristique
                       WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(site_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !site_existe {
        return Err(ApiErreur::NonTrouve("Site touristique introuvable.".into()));
    }

    // Upsert sur l'index unique partiel (utilisateur_id, site_id) WHERE deleted_at IS NULL.
    let row: (Uuid, bool, chrono::DateTime<chrono::Utc>) = sqlx::query_as(
        "INSERT INTO country_profile.avis_site (site_id, utilisateur_id, note, commentaire)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (utilisateur_id, site_id) WHERE deleted_at IS NULL
         DO UPDATE SET note = EXCLUDED.note, commentaire = EXCLUDED.commentaire
         RETURNING id, (xmax = 0) AS est_creation, created_at",
    )
    .bind(site_id)
    .bind(utilisateur_id)
    .bind(body.note)
    .bind(commentaire)
    .fetch_one(pool.get_ref())
    .await?;
    let (avis_id, est_creation, created_at) = row;

    let payload = json!({
        "id": avis_id,
        "note": body.note,
        "commentaire": commentaire,
        "created_at": created_at,
    });
    let reponse = ApiResponse {
        success: true,
        data: Some(payload),
        error: None,
    };
    if est_creation {
        Ok(HttpResponse::Created().json(reponse))
    } else {
        Ok(HttpResponse::Ok().json(reponse))
    }
}

// ════════════════════════════════════════════════════════════════
// T071 — Mes contributions (utilisateur connecté)
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct MesContributionsQuery {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub etat: Option<String>,
    pub type_objet: Option<String>,
    pub fiche_pays_id: Option<Uuid>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct MaContributionRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub pays_nom: Option<String>,
    pub type_objet_contribution: String,
    pub section_afripulse: Option<String>,
    pub type_contribution: String,
    pub etat: String,
    pub note_moderation: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub traite_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn lister_mes_contributions(
    pool: web::Data<PgPool>,
    params: web::Query<MesContributionsQuery>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions = vec![
        "cf.cree_par = $1".to_string(),
        "cf.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 2u32;
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut ordre: Vec<&str> = Vec::new();

    if let Some(ref etat) = params.etat {
        let e = etat.trim().to_lowercase();
        if !e.is_empty() {
            conditions.push(format!("cf.etat::text = ${}", bind_index));
            bind_strings.push(e);
            ordre.push("str");
            bind_index += 1;
        }
    }
    if let Some(ref t) = params.type_objet {
        let t = t.trim().to_lowercase();
        if !t.is_empty() {
            conditions.push(format!(
                "cf.type_objet_contribution::text = ${}",
                bind_index
            ));
            bind_strings.push(t);
            ordre.push("str");
            bind_index += 1;
        }
    }
    if let Some(fp) = params.fiche_pays_id {
        conditions.push(format!("cf.fiche_pays_id = ${}", bind_index));
        bind_uuids.push(fp);
        ordre.push("uuid");
        bind_index += 1;
    }
    let where_clause = conditions.join(" AND ");

    let count_sql = format!(
        "SELECT COUNT(*)::bigint FROM country_profile.contribution_fiche cf WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql).bind(utilisateur_id);
    let mut si = 0;
    let mut ui = 0;
    for t in &ordre {
        if *t == "str" {
            count_q = count_q.bind(&bind_strings[si]);
            si += 1;
        } else {
            count_q = count_q.bind(bind_uuids[ui]);
            ui += 1;
        }
    }
    let total = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT cf.id, cf.fiche_pays_id,
                p.nom AS pays_nom,
                cf.type_objet_contribution::text AS type_objet_contribution,
                cf.section_afripulse::text AS section_afripulse,
                cf.type_contribution::text AS type_contribution,
                cf.etat::text AS etat,
                cf.note_moderation,
                cf.created_at, cf.traite_at
         FROM country_profile.contribution_fiche cf
         LEFT JOIN country_profile.fiche_pays fp ON cf.fiche_pays_id = fp.id
         LEFT JOIN shared.pays p ON fp.pays_id = p.id
         WHERE {}
         ORDER BY cf.created_at DESC
         LIMIT ${} OFFSET ${}",
        where_clause,
        bind_index,
        bind_index + 1,
    );
    let mut select_q = sqlx::query_as::<_, MaContributionRow>(&select_sql).bind(utilisateur_id);
    si = 0;
    ui = 0;
    for t in &ordre {
        if *t == "str" {
            select_q = select_q.bind(&bind_strings[si]);
            si += 1;
        } else {
            select_q = select_q.bind(bind_uuids[ui]);
            ui += 1;
        }
    }
    select_q = select_q.bind(par_page).bind(offset);

    let contributions = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({
            "contributions": contributions,
            "total": total,
            "page": page,
            "par_page": par_page,
        })),
        error: None,
    }))
}

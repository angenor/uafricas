use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::africantive::{
    AfricantiveDetailRow, AfricantiveListeResponse, AfricantiveListeRow,
    AfricantiveQueryParams, generer_slug,
    AFRICANTIVE_DETAIL_COLONNES, AFRICANTIVE_LISTE_COLONNES,
};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Extraire l'utilisateur connecte depuis le header Authorization
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

// ──────────────────────────────────────────────────────────────
// GET /api/africantives : Lister les africantives avec filtres
// ──────────────────────────────────────────────────────────────
pub async fn lister_africantives(
    pool: web::Data<PgPool>,
    params: web::Query<AfricantiveQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let terme_recherche: Option<String> = params
        .recherche
        .as_ref()
        .filter(|r| !r.trim().is_empty())
        .map(|r| format!("%{}%", r.to_lowercase()));

    let domaine_filtre: Option<String> = params
        .domaine
        .as_ref()
        .filter(|d| !d.trim().is_empty())
        .cloned();

    let pays_filtre: Option<String> = params
        .pays
        .as_ref()
        .filter(|p| !p.trim().is_empty())
        .cloned();

    let order_clause = match params.tri.as_deref() {
        Some("ancien") => "ORDER BY a.created_at ASC",
        Some("titre") => "ORDER BY a.titre ASC",
        _ => "ORDER BY a.created_at DESC",
    };

    // ── Requete COUNT ──────────────────────────────────────
    let count_query = format!(
        "SELECT COUNT(*) FROM innovation.africantive a
         LEFT JOIN shared.domaine_secteur d ON d.id = a.domaine_id
         LEFT JOIN shared.pays p ON p.id = a.pays_id
         WHERE a.deleted_at IS NULL
           AND a.etat = 'publie'
           AND ($1::VARCHAR IS NULL OR d.nom = $1 OR d.slug = $1)
           AND ($2::VARCHAR IS NULL OR p.nom = $2)
           AND ($3::VARCHAR IS NULL OR (
               LOWER(a.titre) LIKE $3
               OR LOWER(a.description) LIKE $3
               OR LOWER(COALESCE(a.ville, '')) LIKE $3
           ))"
    );

    let total: (i64,) = sqlx::query_as(&count_query)
        .bind(&domaine_filtre)
        .bind(&pays_filtre)
        .bind(&terme_recherche)
        .fetch_one(pool.get_ref())
        .await?;

    // ── Requete principale ─────────────────────────────────
    let query = format!(
        "SELECT {colonnes}
         FROM innovation.africantive a
         LEFT JOIN shared.domaine_secteur d ON d.id = a.domaine_id
         LEFT JOIN shared.pays p ON p.id = a.pays_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.deleted_at IS NULL
           AND a.etat = 'publie'
           AND ($1::VARCHAR IS NULL OR d.nom = $1 OR d.slug = $1)
           AND ($2::VARCHAR IS NULL OR p.nom = $2)
           AND ($3::VARCHAR IS NULL OR (
               LOWER(a.titre) LIKE $3
               OR LOWER(a.description) LIKE $3
               OR LOWER(COALESCE(a.ville, '')) LIKE $3
           ))
         {order}
         LIMIT $4 OFFSET $5",
        colonnes = AFRICANTIVE_LISTE_COLONNES,
        order = order_clause
    );

    let rows = sqlx::query_as::<_, AfricantiveListeRow>(&query)
        .bind(&domaine_filtre)
        .bind(&pays_filtre)
        .bind(&terme_recherche)
        .bind(par_page)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

    let total_count = total.0;
    let total_pages = if total_count == 0 {
        1
    } else {
        (total_count as f64 / par_page as f64).ceil() as i64
    };

    let reponse = AfricantiveListeResponse {
        africantives: rows.iter().map(|r| r.to_response()).collect(),
        total: total_count,
        page,
        par_page,
        total_pages,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/africantives/{id} : Obtenir le detail d'une africantive
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_africantive(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {}
         FROM innovation.africantive a
         LEFT JOIN shared.domaine_secteur d ON d.id = a.domaine_id
         LEFT JOIN shared.pays p ON p.id = a.pays_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.id = $1 AND a.deleted_at IS NULL",
        AFRICANTIVE_DETAIL_COLONNES
    );

    let africantive = sqlx::query_as::<_, AfricantiveDetailRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Africantive avec id {} non trouvee", id)))?;

    let reponse = africantive.to_detail_response();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/africantives : Creer une africantive (multipart)
// ──────────────────────────────────────────────────────────────
pub async fn creer_africantive(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Token invalide ou manquant".into()))?;

    // Champs texte collectes generiquement (vides ignores)
    let mut champs: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut image_url: Option<String> = None;

    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let couvertures_dir = format!("{}/couvertures", upload_dir);
    std::fs::create_dir_all(&couvertures_dir)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le dossier uploads: {}", e)))?;

    while let Some(Ok(mut field)) = payload.next().await {
        let field_name = field.name().map(|n| n.to_string()).unwrap_or_default();

        match field_name.as_str() {
            "couverture" => {
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));

                let ext = std::path::Path::new(&filename)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("jpg");
                let safe_name = format!("{}.{}", Uuid::new_v4(), ext);
                let filepath = format!("{}/{}", couvertures_dir, safe_name);

                let mut file = std::fs::File::create(&filepath)
                    .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture fichier: {}", e)))?;

                while let Some(Ok(chunk)) = field.next().await {
                    file.write_all(&chunk)
                        .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture: {}", e)))?;
                }

                image_url = Some(format!("/uploads/couvertures/{}", safe_name));
            }
            _ => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() {
                    champs.insert(field_name, val);
                }
            }
        }
    }

    // Helper : recuperer un champ texte optionnel
    let champ = |cle: &str| champs.get(cle).cloned();

    let titre = champ("titre")
        .filter(|t| !t.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le titre est requis".into()))?;
    let description = champ("description")
        .filter(|d| !d.is_empty())
        .ok_or_else(|| ApiErreur::Validation("La description est requise".into()))?;

    let domaine = champ("domaine");
    let pays = champ("pays");
    let ville = champ("ville");
    let site_web_url = champ("site_web_url");
    let lien_reseau_social = champ("lien_reseau_social");
    // Precision conservee uniquement si le domaine choisi est « Autre »
    let domaine_autre = if domaine.as_deref() == Some("Autre") {
        champ("domaine_autre")
    } else {
        None
    };
    let contact1_courriel = champ("contact1_courriel");
    let contact1_telephone = champ("contact1_telephone");
    let contact1_adresse = champ("contact1_adresse");
    let contact2_courriel = champ("contact2_courriel");
    let contact2_telephone = champ("contact2_telephone");
    let contact2_adresse = champ("contact2_adresse");

    let slug = generer_slug(&titre);

    // Resoudre domaine_id par nom ou slug
    let domaine_id: Option<Uuid> = if let Some(ref dom) = domaine {
        sqlx::query_scalar(
            "SELECT id FROM shared.domaine_secteur WHERE nom = $1 OR slug = $1 LIMIT 1",
        )
        .bind(dom)
        .fetch_optional(pool.get_ref())
        .await?
    } else {
        None
    };

    // Resoudre pays_id par nom
    let pays_id: Option<Uuid> = if let Some(ref p) = pays {
        sqlx::query_scalar("SELECT id FROM shared.pays WHERE nom = $1 LIMIT 1")
            .bind(p)
            .fetch_optional(pool.get_ref())
            .await?
    } else {
        None
    };

    // Inserer
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO innovation.africantive
            (titre, slug, description, image_couverture_url, domaine_id, domaine_autre,
             pays_id, ville, site_web_url, lien_reseau_social,
             contact1_courriel, contact1_telephone, contact1_adresse,
             contact2_courriel, contact2_telephone, contact2_adresse, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                 $11, $12, $13, $14, $15, $16, $17)
         RETURNING id",
    )
    .bind(&titre)
    .bind(&slug)
    .bind(&description)
    .bind(&image_url)
    .bind(domaine_id)
    .bind(&domaine_autre)
    .bind(pays_id)
    .bind(&ville)
    .bind(&site_web_url)
    .bind(&lien_reseau_social)
    .bind(&contact1_courriel)
    .bind(&contact1_telephone)
    .bind(&contact1_adresse)
    .bind(&contact2_courriel)
    .bind(&contact2_telephone)
    .bind(&contact2_adresse)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Charger la row inseree pour la reponse
    let query = format!(
        "SELECT {}
         FROM innovation.africantive a
         LEFT JOIN shared.domaine_secteur d ON d.id = a.domaine_id
         LEFT JOIN shared.pays p ON p.id = a.pays_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.id = $1",
        AFRICANTIVE_DETAIL_COLONNES
    );

    let row = sqlx::query_as::<_, AfricantiveDetailRow>(&query)
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_detail_response()),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/africantives/domaines : Lister les domaines disponibles
// ──────────────────────────────────────────────────────────────
pub async fn lister_domaines(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT d.nom
         FROM innovation.africantive a
         JOIN shared.domaine_secteur d ON d.id = a.domaine_id
         WHERE a.deleted_at IS NULL AND a.etat = 'publie' AND d.actif = TRUE
         ORDER BY d.nom",
    )
    .fetch_all(pool.get_ref())
    .await?;

    let domaines: Vec<String> = rows.into_iter().map(|r| r.0).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(domaines),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/africantives/pays : Lister les pays avec des africantives
// ──────────────────────────────────────────────────────────────
pub async fn lister_pays(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT p.nom
         FROM innovation.africantive a
         JOIN shared.pays p ON p.id = a.pays_id
         WHERE a.deleted_at IS NULL AND a.etat = 'publie'
         ORDER BY p.nom",
    )
    .fetch_all(pool.get_ref())
    .await?;

    let pays: Vec<String> = rows.into_iter().map(|r| r.0).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(pays),
        error: None,
    }))
}

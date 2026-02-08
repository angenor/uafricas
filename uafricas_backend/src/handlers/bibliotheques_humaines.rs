use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::bibliotheque_humaine::{
    BiblioHumaineListeResponse, BiblioHumaineQueryParams, BiblioHumaineRow,
    InscriptionBiblioBody, SpecialiteRow, BIBLIO_HUMAINE_COLONNES,
};

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

// ────────────────────────────────────────────────────────────────
// Endpoints
// ────────────────────────────────────────────────────────────────

/// GET /api/bibliotheques-humaines — Liste paginee avec filtres
pub async fn lister_biblios(
    pool: web::Data<PgPool>,
    params: web::Query<BiblioHumaineQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec![
        "u.bibliotheque_humain = TRUE".to_string(),
        "u.deleted_at IS NULL".to_string(),
        "u.etat = 'actif'".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par specialite
    if let Some(ref specialite) = params.specialite {
        let trimmed = specialite.trim();
        if !trimmed.is_empty() && trimmed != "Tous" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM iam.utilisateur_specialite us
                 JOIN iam.specialite_bibliotheque sb ON sb.id = us.specialite_id
                 WHERE us.utilisateur_id = u.id AND LOWER(sb.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    // Filtre par pays
    if let Some(ref pays) = params.pays {
        let trimmed = pays.trim();
        if !trimmed.is_empty() {
            conditions.push(format!("LOWER(p.nom) = LOWER(${})", bind_index));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    // Recherche textuelle (nom, prenom, fonction, biographie, specialite)
    if let Some(ref recherche) = params.recherche {
        let trimmed = recherche.trim();
        if !trimmed.is_empty() {
            let terme = format!("%{}%", trimmed.to_lowercase());
            conditions.push(format!(
                "(LOWER(u.nom) LIKE ${idx} OR LOWER(u.prenom) LIKE ${idx} \
                 OR LOWER(COALESCE(u.fonction, '')) LIKE ${idx} \
                 OR LOWER(COALESCE(u.biographie, '')) LIKE ${idx} \
                 OR EXISTS (SELECT 1 FROM iam.utilisateur_specialite us4
                    JOIN iam.specialite_bibliotheque sb4 ON sb4.id = us4.specialite_id
                    WHERE us4.utilisateur_id = u.id AND LOWER(sb4.nom) LIKE ${idx}))",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_origine_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les bibliotheques humaines
    let select_query = format!(
        "SELECT {} FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_origine_id
         WHERE {} ORDER BY u.created_at DESC LIMIT ${} OFFSET ${}",
        BIBLIO_HUMAINE_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, BiblioHumaineRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    let bibliotheques: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(BiblioHumaineListeResponse {
            bibliotheques,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/bibliotheques-humaines/{id} — Detail d'une bibliotheque humaine
pub async fn obtenir_biblio(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_origine_id
         WHERE u.id = $1 AND u.bibliotheque_humain = TRUE AND u.deleted_at IS NULL",
        BIBLIO_HUMAINE_COLONNES
    );

    let row = sqlx::query_as::<_, BiblioHumaineRow>(&query)
        .bind(utilisateur_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| {
            ApiErreur::NonTrouve(format!(
                "Bibliotheque humaine avec id {} non trouvee",
                utilisateur_id
            ))
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/bibliotheques-humaines/inscription — Devenir bibliotheque humaine (JWT requis)
pub async fn inscrire_biblio(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<InscriptionBiblioBody>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    // Verifier que l'utilisateur existe
    let user_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !user_exists {
        return Err(ApiErreur::NonTrouve(
            "Utilisateur non trouve".to_string(),
        ));
    }

    // Verifier qu'il n'est pas deja bibliotheque humaine
    let deja_biblio: bool = sqlx::query_scalar(
        "SELECT COALESCE(bibliotheque_humain, FALSE) FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if deja_biblio {
        return Err(ApiErreur::Conflit(
            "Vous etes deja inscrit comme bibliotheque humaine".to_string(),
        ));
    }

    // Valider qu'au moins une specialite est fournie
    if body.specialites.is_empty() {
        return Err(ApiErreur::Validation(
            "Au moins une specialite est requise".to_string(),
        ));
    }

    // Activer le flag bibliotheque_humain
    sqlx::query("UPDATE iam.utilisateur SET bibliotheque_humain = TRUE, updated_at = NOW() WHERE id = $1")
        .bind(utilisateur_id)
        .execute(pool.get_ref())
        .await?;

    // Inserer les specialites
    for nom_specialite in &body.specialites {
        // Recuperer l'id de la specialite par nom
        let specialite_id: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM iam.specialite_bibliotheque WHERE LOWER(nom) = LOWER($1)",
        )
        .bind(nom_specialite)
        .fetch_optional(pool.get_ref())
        .await?;

        if let Some(spec_id) = specialite_id {
            sqlx::query(
                "INSERT INTO iam.utilisateur_specialite (utilisateur_id, specialite_id)
                 VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(utilisateur_id)
            .bind(spec_id)
            .execute(pool.get_ref())
            .await?;
        }
    }

    log::info!(
        "Utilisateur {} inscrit comme bibliotheque humaine avec {} specialites",
        utilisateur_id,
        body.specialites.len()
    );

    // Recuperer le profil complet pour le retourner
    let row_query = format!(
        "SELECT {} FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_origine_id
         WHERE u.id = $1",
        BIBLIO_HUMAINE_COLONNES
    );

    let row = sqlx::query_as::<_, BiblioHumaineRow>(&row_query)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// GET /api/bibliotheques-humaines/specialites — Liste des specialites disponibles
pub async fn lister_specialites(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let rows = sqlx::query_as::<_, SpecialiteRow>(
        "SELECT id, nom, slug FROM iam.specialite_bibliotheque ORDER BY nom",
    )
    .fetch_all(pool.get_ref())
    .await?;

    let specialites: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(specialites),
        error: None,
    }))
}

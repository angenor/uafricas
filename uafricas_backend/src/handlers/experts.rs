use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::expert::{
    CandidatureExpertBody, ExpertListeResponse, ExpertQueryParams,
    ExpertRow, MaCandidatureRow, EXPERT_COLONNES, mapper_domaine_db,
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

/// GET /api/experts — Liste paginee des experts valides avec filtres
pub async fn lister_experts(
    pool: web::Data<PgPool>,
    params: web::Query<ExpertQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Construction dynamique du WHERE
    let mut conditions: Vec<String> = vec![
        "e.statut = 'valide'".to_string(),
        "e.deleted_at IS NULL".to_string(),
        "u.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par domaine
    if let Some(ref domaine) = params.domaine {
        let trimmed = domaine.trim();
        if !trimmed.is_empty() && trimmed != "Tout" {
            let domaine_db = mapper_domaine_db(trimmed);
            conditions.push(format!("e.domaine::text = ${}", bind_index));
            bind_values.push(domaine_db);
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

    // Filtre par situation professionnelle (match dans le tableau PostgreSQL)
    if let Some(ref situation) = params.situation {
        let trimmed = situation.trim();
        if !trimmed.is_empty() && trimmed != "tous" {
            conditions.push(format!(
                "${}::text = ANY(e.situations_professionnelles::text[])",
                bind_index
            ));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    // Recherche textuelle (nom, prenom, biographie, domaine)
    if let Some(ref recherche) = params.recherche {
        let trimmed = recherche.trim();
        if !trimmed.is_empty() {
            let terme = format!("%{}%", trimmed.to_lowercase());
            conditions.push(format!(
                "(LOWER(u.nom) LIKE ${idx} OR LOWER(u.prenom) LIKE ${idx} \
                 OR LOWER(e.biographie) LIKE ${idx} OR LOWER(e.domaine::text) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Clause ORDER BY
    let order_clause = match params.tri.as_deref() {
        Some("experience") => "ORDER BY e.nb_annees_experience DESC, e.updated_at DESC",
        Some("rating") => "ORDER BY e.rating DESC, e.updated_at DESC",
        _ => "ORDER BY e.updated_at DESC",
    };

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les experts
    let select_query = format!(
        "SELECT {} FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE {} {} LIMIT ${} OFFSET ${}",
        EXPERT_COLONNES, where_clause, order_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, ExpertRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    let experts: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ExpertListeResponse {
            experts,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/experts/{id} — Detail d'un expert par son utilisateur_id
pub async fn obtenir_expert(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE e.utilisateur_id = $1 AND e.deleted_at IS NULL AND u.deleted_at IS NULL",
        EXPERT_COLONNES
    );

    let row = sqlx::query_as::<_, ExpertRow>(&query)
        .bind(utilisateur_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| {
            ApiErreur::NonTrouve(format!("Expert avec id {} non trouve", utilisateur_id))
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/experts/candidature — Soumettre une candidature expert (JWT requis)
pub async fn creer_candidature(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CandidatureExpertBody>,
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

    // Etat de la candidature active eventuelle (FR-006 + FR-015)
    let statut_actif: Option<String> = sqlx::query_scalar(
        "SELECT statut::text FROM iam.expertise
         WHERE utilisateur_id = $1 AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    // Bloquer si une demande active en_attente ou valide existe deja
    if let Some(ref statut) = statut_actif {
        if statut == "en_attente" || statut == "valide" {
            return Err(ApiErreur::Conflit(
                "Vous avez deja une demande d'expertise active".to_string(),
            ));
        }
    }

    // Mapper le domaine
    let domaine_db = mapper_domaine_db(&body.domaine);

    // Précision libre obligatoire (et conservée) uniquement quand domaine = "autre"
    let domaine_autre: Option<String> = if domaine_db == "autre" {
        let precision = body
            .domaine_autre
            .as_deref()
            .map(str::trim)
            .filter(|p| !p.is_empty());
        match precision {
            Some(p) => Some(p.to_string()),
            None => {
                return Err(ApiErreur::Validation(
                    "Veuillez préciser votre domaine d'expertise".to_string(),
                ))
            }
        }
    } else {
        None
    };

    // Transaction : archiver une eventuelle demande refusee puis inserer la nouvelle
    let mut tx = pool.begin().await?;

    if statut_actif.as_deref() == Some("refuse") {
        sqlx::query(
            "UPDATE iam.expertise SET deleted_at = NOW(), updated_at = NOW()
             WHERE utilisateur_id = $1 AND deleted_at IS NULL",
        )
        .bind(utilisateur_id)
        .execute(&mut *tx)
        .await?;
    }

    let expertise_id: Uuid = sqlx::query_scalar(
        "INSERT INTO iam.expertise
            (utilisateur_id, domaine, domaine_autre, biographie, nb_annees_experience,
             portfolio, situations_professionnelles, statut)
         VALUES ($1, $2::iam.domaine_expertise, $3, $4, $5, $6,
                 $7::iam.situation_professionnelle[], 'en_attente')
         RETURNING id",
    )
    .bind(utilisateur_id)
    .bind(&domaine_db)
    .bind(&domaine_autre)
    .bind(&body.biographie)
    .bind(body.nb_annees_experience)
    .bind(&body.portfolio)
    .bind(&body.situations_professionnelles)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    log::info!(
        "Candidature expert creee: {} pour utilisateur {}",
        expertise_id,
        utilisateur_id
    );

    // Recuperer la candidature creee pour la retourner
    let row_query = format!(
        "SELECT {} FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE e.id = $1",
        EXPERT_COLONNES
    );

    let row = sqlx::query_as::<_, ExpertRow>(&row_query)
        .bind(expertise_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// GET /api/experts/moi — Candidature active du membre connecte (suivi US3)
pub async fn ma_candidature(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let row = sqlx::query_as::<_, MaCandidatureRow>(
        "SELECT id, domaine::text AS domaine, domaine_autre, biographie, nb_annees_experience,
                portfolio,
                situations_professionnelles::text[] AS situations_professionnelles,
                statut::text AS statut, commentaire_admin, date_validation, created_at
         FROM iam.expertise
         WHERE utilisateur_id = $1 AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: row.map(|r| r.to_response()),
        error: None,
    }))
}

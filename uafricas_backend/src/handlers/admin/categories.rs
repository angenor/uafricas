use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::services::audit;
use crate::models::admin::categorie::{
    AdminCategorieDetailResponse, AdminCategorieEnfant, AdminCategorieListeResponse,
    AdminCategorieQueryParams, CreerCategorieRequest, ModifierCategorieRequest,
    ADMIN_CATEGORIE_DETAIL_COLONNES, ADMIN_CATEGORIE_LISTE_COLONNES, CATEGORIE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/categories
pub async fn lister_categories(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminCategorieQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["c.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!("(LOWER(c.nom) LIKE ${bi} OR LOWER(c.slug) LIKE ${bi})", bi = bind_index));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref contexte) = params.contexte {
        let ctx = contexte.trim();
        if !ctx.is_empty() {
            conditions.push(format!("c.contexte = ${}", bind_index));
            bind_values.push(ctx.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref parent) = params.parent_id {
        let p = parent.trim();
        if p == "null" || p == "racine" {
            conditions.push("c.parent_id IS NULL".to_string());
        } else if let Ok(pid) = Uuid::parse_str(p) {
            conditions.push(format!("c.parent_id = ${}", bind_index));
            bind_values.push(pid.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(CATEGORIE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!("SELECT COUNT(*) FROM shared.categorie c WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM shared.categorie c WHERE {} ORDER BY c.{} {} LIMIT {} OFFSET {}",
        ADMIN_CATEGORIE_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminCategorieListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/categories/:id
pub async fn obtenir_categorie(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM shared.categorie c WHERE c.id = $1 AND c.deleted_at IS NULL",
        ADMIN_CATEGORIE_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminCategorieDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Categorie non trouvee".into()))?;

    let enfants = sqlx::query_as::<_, AdminCategorieEnfant>(
        "SELECT id, nom, slug, icone, ordre, actif FROM shared.categorie
         WHERE parent_id = $1 AND deleted_at IS NULL ORDER BY ordre, nom"
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": row.id,
            "nom": row.nom,
            "slug": row.slug,
            "contexte": row.contexte,
            "parent_id": row.parent_id,
            "description": row.description,
            "icone": row.icone,
            "ordre": row.ordre,
            "actif": row.actif,
            "created_at": row.created_at,
            "updated_at": row.updated_at,
            "enfants": enfants,
        })),
        error: None,
    }))
}

/// POST /api/admin/categories
pub async fn creer_categorie(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerCategorieRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de la categorie est requis".into()));
    }

    let slug = nom.to_lowercase().replace(' ', "-").replace(['\'', '"', '.', ','], "");
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO shared.categorie (id, nom, slug, contexte, parent_id, description, icone, ordre)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(id)
    .bind(nom)
    .bind(&slug)
    .bind(body.contexte.as_deref().map(|s| s.trim()))
    .bind(body.parent_id)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.icone.as_deref().map(|s| s.trim()))
    .bind(body.ordre.unwrap_or(0))
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree la categorie {} ({})", admin.id, nom, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "shared",
        "categorie",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/categories/:id
pub async fn modifier_categorie(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierCategorieRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM shared.categorie WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Categorie non trouvee".into()));
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! ajouter_champ_str {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    ajouter_champ_str!(body.nom, "nom");
    ajouter_champ_str!(body.contexte, "contexte");
    ajouter_champ_str!(body.description, "description");
    ajouter_champ_str!(body.icone, "icone");

    if let Some(ref nom) = body.nom {
        let slug = nom.trim().to_lowercase().replace(' ', "-").replace(['\'', '"', '.', ','], "");
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(slug);
        bind_index += 1;
    }

    if let Some(parent_id) = body.parent_id {
        sets.push(format!("parent_id = ${}", bind_index));
        bind_uuids.push(parent_id);
        bind_index += 1;
    }

    if let Some(ordre) = body.ordre {
        sets.push(format!("ordre = {}", ordre));
    }
    if let Some(actif) = body.actif {
        sets.push(format!("actif = {}", actif));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE shared.categorie SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    for v in &bind_uuids { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie la categorie {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "shared",
        "categorie",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/categories/:id
pub async fn supprimer_categorie(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "supprimer");
    let id = path.into_inner();

    let enfants_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM shared.categorie WHERE parent_id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    if enfants_count > 0 {
        return Err(ApiErreur::Validation(
            format!("Impossible de supprimer : {} categorie(s) enfant(s) active(s)", enfants_count)
        ));
    }

    let result = sqlx::query(
        "UPDATE shared.categorie SET deleted_at = NOW(), actif = false, updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Categorie non trouvee".into()));
    }

    log::info!("Admin {} a supprime la categorie {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "shared",
        "categorie",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

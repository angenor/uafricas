use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::africantive::{
    AdminAfricantiveDetailResponse, AdminAfricantiveListeResponse, AdminAfricantiveQueryParams,
    ChangerEtatAfricantiveRequest, CreerAfricantiveRequest, ModifierAfricantiveRequest,
    ADMIN_AFRICANTIVE_DETAIL_COLONNES, ADMIN_AFRICANTIVE_LISTE_COLONNES, AFRICANTIVE_JOINS,
    AFRICANTIVE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

// ── Etats valides pour africantive (etat_contenu) ───────────
const ETATS_VALIDES: &[&str] = &["brouillon", "publie", "suspendu", "supprime"];

/// GET /api/admin/africantives
pub async fn lister_africantives(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminAfricantiveQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["af.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(af.titre) LIKE ${bi} OR LOWER(af.description) LIKE ${bi} OR LOWER(COALESCE(af.ville, '')) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("af.etat::text = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref domaine_id) = params.domaine_id {
        let d = domaine_id.trim();
        if !d.is_empty() {
            if Uuid::parse_str(d).is_ok() {
                conditions.push(format!("af.domaine_id::text = ${}", bind_index));
                bind_values.push(d.to_string());
                bind_index += 1;
            }
        }
    }

    if let Some(ref pays_id) = params.pays_id {
        let p = pays_id.trim();
        if !p.is_empty() {
            if Uuid::parse_str(p).is_ok() {
                conditions.push(format!("af.pays_id::text = ${}", bind_index));
                bind_values.push(p.to_string());
                bind_index += 1;
            }
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(AFRICANTIVE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    // COUNT
    let count_sql = format!(
        "SELECT COUNT(*) FROM innovation.africantive af {} WHERE {}",
        AFRICANTIVE_JOINS, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT
    let select_sql = format!(
        "SELECT {} FROM innovation.africantive af {} WHERE {} ORDER BY af.{} {} LIMIT {} OFFSET {}",
        ADMIN_AFRICANTIVE_LISTE_COLONNES, AFRICANTIVE_JOINS, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminAfricantiveListeResponse>(&select_sql);
    for v in &bind_values {
        select_q = select_q.bind(v);
    }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/africantives/:id
pub async fn obtenir_africantive(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM innovation.africantive af {} WHERE af.id = $1 AND af.deleted_at IS NULL",
        ADMIN_AFRICANTIVE_DETAIL_COLONNES, AFRICANTIVE_JOINS
    );
    let row = sqlx::query_as::<_, AdminAfricantiveDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Africantive non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row),
        error: None,
    }))
}

/// POST /api/admin/africantives
pub async fn creer_africantive(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<CreerAfricantiveRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }
    let description = body.description.trim();
    if description.is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }

    let etat = body.etat.as_deref().unwrap_or("brouillon").trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide. Valeurs acceptees: {}",
            ETATS_VALIDES.join(", ")
        )));
    }

    let slug = titre
        .to_lowercase()
        .replace(' ', "-")
        .replace(['\'', '"', '.', ','], "");
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO innovation.africantive
         (id, titre, slug, description, image_couverture_url, domaine_id, pays_id, ville, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::innovation.etat_contenu, $10)",
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(description)
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.domaine_id)
    .bind(body.pays_id)
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(etat)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree l'africantive {} ({})", admin.id, titre, id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/africantives/:id
pub async fn modifier_africantive(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierAfricantiveRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM innovation.africantive WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Africantive non trouvee".into()));
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

    ajouter_champ_str!(body.titre, "titre");
    ajouter_champ_str!(body.description, "description");
    ajouter_champ_str!(body.image_couverture_url, "image_couverture_url");
    ajouter_champ_str!(body.ville, "ville");

    // Slug auto-genere si titre change
    if let Some(ref titre) = body.titre {
        let slug = titre
            .trim()
            .to_lowercase()
            .replace(' ', "-")
            .replace(['\'', '"', '.', ','], "");
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(slug);
        bind_index += 1;
    }

    // UUIDs
    if let Some(domaine_id) = body.domaine_id {
        sets.push(format!("domaine_id = ${}", bind_index));
        bind_uuids.push(domaine_id);
        bind_index += 1;
    }
    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE innovation.africantive SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "),
        bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
    for v in &bind_uuids {
        q = q.bind(v);
    }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie l'africantive {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PATCH /api/admin/africantives/:id/etat
pub async fn changer_etat_africantive(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatAfricantiveRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide. Valeurs acceptees: {}",
            ETATS_VALIDES.join(", ")
        )));
    }

    let result = sqlx::query(
        "UPDATE innovation.africantive SET etat = $1::innovation.etat_contenu, updated_at = NOW()
         WHERE id = $2 AND deleted_at IS NULL",
    )
    .bind(etat)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Africantive non trouvee".into()));
    }

    log::info!(
        "Admin {} a change l'etat de l'africantive {} en '{}'",
        admin.id,
        id,
        etat
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": etat })),
        error: None,
    }))
}

/// DELETE /api/admin/africantives/:id
pub async fn supprimer_africantive(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE innovation.africantive SET deleted_at = NOW(), etat = 'supprime'::innovation.etat_contenu, updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Africantive non trouvee".into()));
    }

    log::info!("Admin {} a supprime l'africantive {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

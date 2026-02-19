use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::audit::{
    AuditListeResponse, AuditDetailRow, AuditQueryParams,
    AUDIT_LISTE_COLONNES, AUDIT_DETAIL_COLONNES, AUDIT_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/audit
pub async fn lister_audit(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AuditQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "audit", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions: Vec<String> = Vec::new();
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;
    let mut bind_types: Vec<&str> = Vec::new();

    // Filtre par action
    if let Some(ref action) = params.action {
        let a = action.trim().to_uppercase();
        if !a.is_empty() {
            conditions.push(format!("a.action = ${}", bind_index));
            bind_values.push(a);
            bind_types.push("str");
            bind_index += 1;
        }
    }

    // Filtre par utilisateur
    if let Some(utilisateur_id) = params.utilisateur_id {
        conditions.push(format!("a.utilisateur_id = ${}", bind_index));
        bind_uuids.push(utilisateur_id);
        bind_types.push("uuid");
        bind_index += 1;
    }

    // Filtre par schema
    if let Some(ref schema) = params.schema_name {
        let s = schema.trim();
        if !s.is_empty() {
            conditions.push(format!("a.schema_name = ${}", bind_index));
            bind_values.push(s.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    // Filtre par table
    if let Some(ref table) = params.table_name {
        let t = table.trim();
        if !t.is_empty() {
            conditions.push(format!("a.table_name = ${}", bind_index));
            bind_values.push(t.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    // Filtre par IP
    if let Some(ref ip) = params.ip_address {
        let i = ip.trim();
        if !i.is_empty() {
            conditions.push(format!("a.ip_address::TEXT LIKE ${}", bind_index));
            bind_values.push(format!("%{}%", i));
            bind_types.push("str");
            bind_index += 1;
        }
    }

    // Filtre par date debut
    if let Some(ref date_debut) = params.date_debut {
        let d = date_debut.trim();
        if !d.is_empty() {
            conditions.push(format!("a.created_at >= ${}::TIMESTAMPTZ", bind_index));
            bind_values.push(d.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    // Filtre par date fin
    if let Some(ref date_fin) = params.date_fin {
        let d = date_fin.trim();
        if !d.is_empty() {
            conditions.push(format!("a.created_at <= ${}::TIMESTAMPTZ", bind_index));
            bind_values.push(d.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    // Recherche full-text dans ancien_etat / nouvel_etat
    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(a.ancien_etat::TEXT ILIKE ${bi} OR a.nouvel_etat::TEXT ILIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r));
            bind_types.push("str");
            bind_index += 1;
        }
    }

    let _ = bind_index;

    let where_clause = if conditions.is_empty() {
        "TRUE".to_string()
    } else {
        conditions.join(" AND ")
    };

    let colonne = pagination.colonne_tri(AUDIT_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN iam.utilisateur u ON a.utilisateur_id = u.id";

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM shared.audit_log a {} WHERE {}",
        joins, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    let mut str_idx = 0;
    let mut uuid_idx = 0;
    for t in &bind_types {
        if *t == "str" { count_q = count_q.bind(&bind_values[str_idx]); str_idx += 1; }
        else { count_q = count_q.bind(bind_uuids[uuid_idx]); uuid_idx += 1; }
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Select
    let select_sql = format!(
        "SELECT {} FROM shared.audit_log a {} WHERE {} ORDER BY a.{} {} LIMIT {} OFFSET {}",
        AUDIT_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AuditListeResponse>(&select_sql);
    str_idx = 0; uuid_idx = 0;
    for t in &bind_types {
        if *t == "str" { select_q = select_q.bind(&bind_values[str_idx]); str_idx += 1; }
        else { select_q = select_q.bind(bind_uuids[uuid_idx]); uuid_idx += 1; }
    }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/audit/{id}
pub async fn obtenir_audit(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "audit", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN iam.utilisateur u ON a.utilisateur_id = u.id";

    let sql = format!(
        "SELECT {} FROM shared.audit_log a {} WHERE a.id = $1",
        AUDIT_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AuditDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Entree d'audit non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

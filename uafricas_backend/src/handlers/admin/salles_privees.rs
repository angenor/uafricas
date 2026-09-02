use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::salle_privee::{
    AdminSallePriveeDetailRow, AdminSallePriveeListeResponse, AdminSallePriveeQueryParams,
    ADMIN_SALLE_PRIVEE_DETAIL_COLONNES, ADMIN_SALLE_PRIVEE_LISTE_COLONNES, SALLE_PRIVEE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/salles-privees : supervision lecture seule
pub async fn lister_salles_privees(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    query: web::Query<AdminSallePriveeQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let pagination = PaginationParams {
        page: query.page,
        par_page: query.par_page,
        tri_par: query.tri_par.clone(),
        tri_dir: query.tri_dir.clone(),
    };
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();
    let tri = pagination.colonne_tri(SALLE_PRIVEE_TRI_COLONNES, "created_at");
    let dir = pagination.direction_tri();

    let mut conditions = vec!["sp.actif = true".to_string()];
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = query.recherche {
        if !recherche.is_empty() {
            conditions.push(format!(
                "(sp.titre ILIKE ${bi} OR sp.description ILIKE ${bi})",
                bi = bind_index
            ));
            str_binds.push(format!("%{}%", recherche));
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(salle_id) = query.salle_id {
        conditions.push(format!("sp.salle_id = ${}", bind_index));
        uuid_binds.push(salle_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    let _ = bind_index;
    let where_clause = conditions.join(" AND ");

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM afrolang.salle_privee sp WHERE {}", where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    let mut str_idx = 0;
    let mut uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => { count_q = count_q.bind(&str_binds[str_idx]); str_idx += 1; }
            "uuid" => { count_q = count_q.bind(uuid_binds[uuid_idx]); uuid_idx += 1; }
            _ => {}
        }
    }
    let total = count_q.fetch_one(pool.get_ref()).await?;

    // Data
    let data_sql = format!(
        "SELECT {} FROM afrolang.salle_privee sp
         LEFT JOIN afrolang.salle s ON sp.salle_id = s.id
         LEFT JOIN iam.utilisateur u ON sp.cree_par = u.id
         WHERE {} ORDER BY sp.{} {} LIMIT {} OFFSET {}",
        ADMIN_SALLE_PRIVEE_LISTE_COLONNES, where_clause, tri, dir, par_page, offset
    );
    let mut data_q = sqlx::query_as::<_, AdminSallePriveeListeResponse>(&data_sql);
    str_idx = 0;
    uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => { data_q = data_q.bind(&str_binds[str_idx]); str_idx += 1; }
            "uuid" => { data_q = data_q.bind(uuid_binds[uuid_idx]); uuid_idx += 1; }
            _ => {}
        }
    }
    let salles = data_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(salles, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/salles-privees/{id} : détail lecture seule
pub async fn obtenir_salle_privee(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM afrolang.salle_privee sp
         LEFT JOIN afrolang.salle s ON sp.salle_id = s.id
         LEFT JOIN iam.utilisateur u ON sp.cree_par = u.id
         WHERE sp.id = $1 AND sp.actif = true",
        ADMIN_SALLE_PRIVEE_DETAIL_COLONNES
    );

    let row = sqlx::query_as::<_, AdminSallePriveeDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Salle privée non trouvée".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

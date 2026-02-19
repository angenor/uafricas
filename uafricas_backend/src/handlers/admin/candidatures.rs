use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::services::audit;
use crate::models::admin::candidature::{
    AdminCandidatureDetailRow, AdminCandidatureListeResponse, AdminCandidatureQueryParams,
    ChangerStatutCandidatureRequest, ADMIN_CANDIDATURE_DETAIL_COLONNES,
    ADMIN_CANDIDATURE_LISTE_COLONNES, CANDIDATURE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

const STATUTS_VALIDES: &[&str] = &[
    "soumise", "en_revue", "acceptee", "refusee", "retiree",
];

/// GET /api/admin/candidatures
pub async fn lister_candidatures(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminCandidatureQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["1=1".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(u.nom || ' ' || u.prenom) LIKE ${bi} OR LOWER(u.email) LIKE ${bi} OR LOWER(p.titre) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref statut) = params.statut {
        let s = statut.trim();
        if !s.is_empty() {
            conditions.push(format!("c.statut::TEXT = ${}", bind_index));
            bind_values.push(s.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(programme_id) = params.programme_id {
        conditions.push(format!("c.programme_id = ${}", bind_index));
        bind_uuids.push(programme_id);
        bind_types.push("uuid");
        bind_index += 1;
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(CANDIDATURE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "JOIN exchange.programme p ON c.programme_id = p.id
                 JOIN iam.utilisateur u ON c.candidat_id = u.id";

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM exchange.candidature c {} WHERE {}",
        joins, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    let mut str_idx = 0;
    let mut uuid_idx = 0;
    for t in &bind_types {
        if *t == "str" {
            count_q = count_q.bind(&bind_values[str_idx]);
            str_idx += 1;
        } else {
            count_q = count_q.bind(bind_uuids[uuid_idx]);
            uuid_idx += 1;
        }
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Select
    let select_sql = format!(
        "SELECT {} FROM exchange.candidature c {} WHERE {} ORDER BY c.{} {} LIMIT {} OFFSET {}",
        ADMIN_CANDIDATURE_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminCandidatureListeResponse>(&select_sql);
    str_idx = 0;
    uuid_idx = 0;
    for t in &bind_types {
        if *t == "str" {
            select_q = select_q.bind(&bind_values[str_idx]);
            str_idx += 1;
        } else {
            select_q = select_q.bind(bind_uuids[uuid_idx]);
            uuid_idx += 1;
        }
    }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/candidatures/{id}
pub async fn obtenir_candidature(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "voir");
    let id = path.into_inner();

    let joins = "JOIN exchange.programme p ON c.programme_id = p.id
                 JOIN iam.utilisateur u ON c.candidat_id = u.id
                 LEFT JOIN iam.utilisateur u_traite ON c.traite_par = u_traite.id";

    let sql = format!(
        "SELECT {} FROM exchange.candidature c {} WHERE c.id = $1",
        ADMIN_CANDIDATURE_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminCandidatureDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Candidature non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// PATCH /api/admin/candidatures/{id}/etat
pub async fn changer_statut_candidature(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerStatutCandidatureRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "modifier");
    let id = path.into_inner();

    let statut = body.statut.trim();
    if !STATUTS_VALIDES.contains(&statut) {
        return Err(ApiErreur::Validation(format!(
            "Statut invalide: {}. Valeurs possibles: {:?}", statut, STATUTS_VALIDES
        )));
    }

    let result = sqlx::query(
        "UPDATE exchange.candidature
         SET statut = $1::exchange.etat_candidature,
             notes_internes = COALESCE($2, notes_internes),
             traite_par = $3,
             updated_at = NOW()
         WHERE id = $4"
    )
    .bind(statut)
    .bind(body.notes_internes.as_deref().map(|s| s.trim()))
    .bind(admin.id)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Candidature non trouvee".into()));
    }

    log::info!("Admin {} a change le statut de la candidature {} vers {}", admin.id, id, statut);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "exchange",
        "candidature",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "statut": statut })),
        error: None,
    }))
}

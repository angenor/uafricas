use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::session_afrolang::{
    AdminSessionDetailRow, AdminSessionListeResponse, AdminSessionParticipantResponse,
    AdminSessionQueryParams, AdminTableauBlancResponse,
    ADMIN_SESSION_DETAIL_COLONNES, ADMIN_SESSION_LISTE_COLONNES, SESSION_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/sessions — historique paginé + filtres
pub async fn lister_sessions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    query: web::Query<AdminSessionQueryParams>,
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
    let tri = pagination.colonne_tri(SESSION_TRI_COLONNES, "created_at");
    let dir = pagination.direction_tri();

    let mut conditions = vec!["1=1".to_string()];
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = query.recherche {
        if !recherche.is_empty() {
            conditions.push(format!("se.titre ILIKE ${}", bind_index));
            str_binds.push(format!("%{}%", recherche));
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref etat) = query.etat {
        if !etat.is_empty() {
            conditions.push(format!("se.etat::text = ${}", bind_index));
            str_binds.push(etat.clone());
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(salle_id) = query.salle_id {
        conditions.push(format!(
            "sp.salle_id = ${}", bind_index
        ));
        uuid_binds.push(salle_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(salle_privee_id) = query.salle_privee_id {
        conditions.push(format!("se.salle_privee_id = ${}", bind_index));
        uuid_binds.push(salle_privee_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(moderateur_id) = query.moderateur_id {
        conditions.push(format!("se.moderateur_id = ${}", bind_index));
        uuid_binds.push(moderateur_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(ref date_debut) = query.date_debut {
        if !date_debut.is_empty() {
            conditions.push(format!("se.created_at >= ${}::timestamptz", bind_index));
            str_binds.push(date_debut.clone());
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref date_fin) = query.date_fin {
        if !date_fin.is_empty() {
            conditions.push(format!("se.created_at <= ${}::timestamptz", bind_index));
            str_binds.push(date_fin.clone());
            param_types.push("str");
            bind_index += 1;
        }
    }

    let _ = bind_index;
    let where_clause = conditions.join(" AND ");

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM afrolang.session se
         JOIN afrolang.salle_privee sp ON se.salle_privee_id = sp.id
         WHERE {}",
        where_clause
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
        "SELECT {} FROM afrolang.session se
         JOIN afrolang.salle_privee sp ON se.salle_privee_id = sp.id
         LEFT JOIN afrolang.salle s ON sp.salle_id = s.id
         LEFT JOIN iam.utilisateur u ON se.moderateur_id = u.id
         WHERE {} ORDER BY se.{} {} LIMIT {} OFFSET {}",
        ADMIN_SESSION_LISTE_COLONNES, where_clause, tri, dir, par_page, offset
    );
    let mut data_q = sqlx::query_as::<_, AdminSessionListeResponse>(&data_sql);
    str_idx = 0;
    uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => { data_q = data_q.bind(&str_binds[str_idx]); str_idx += 1; }
            "uuid" => { data_q = data_q.bind(uuid_binds[uuid_idx]); uuid_idx += 1; }
            _ => {}
        }
    }
    let sessions = data_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(sessions, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/sessions/{id} — détail avec participants
pub async fn obtenir_session(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM afrolang.session se
         JOIN afrolang.salle_privee sp ON se.salle_privee_id = sp.id
         LEFT JOIN afrolang.salle s ON sp.salle_id = s.id
         LEFT JOIN iam.utilisateur u ON se.moderateur_id = u.id
         LEFT JOIN iam.utilisateur cr ON se.cree_par = cr.id
         WHERE se.id = $1",
        ADMIN_SESSION_DETAIL_COLONNES
    );

    let row = sqlx::query_as::<_, AdminSessionDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Session non trouvée".into()))?;

    // Charger les participants
    let participants = sqlx::query_as::<_, AdminSessionParticipantResponse>(
        "SELECT p.id, p.utilisateur_id,
                u.nom AS utilisateur_nom, u.prenom AS utilisateur_prenom,
                p.role_session, p.rejoint_at, p.quitte_at, p.duree_secondes
         FROM afrolang.session_participant p
         LEFT JOIN iam.utilisateur u ON p.utilisateur_id = u.id
         WHERE p.session_id = $1
         ORDER BY p.rejoint_at ASC"
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(participants)),
        error: None,
    }))
}

/// GET /api/admin/sessions/{id}/participants
pub async fn lister_participants(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let session_id = path.into_inner();

    // Vérifier que la session existe
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM afrolang.session WHERE id = $1)"
    )
    .bind(session_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !exists {
        return Err(ApiErreur::NonTrouve("Session non trouvée".into()));
    }

    let participants = sqlx::query_as::<_, AdminSessionParticipantResponse>(
        "SELECT p.id, p.utilisateur_id,
                u.nom AS utilisateur_nom, u.prenom AS utilisateur_prenom,
                p.role_session, p.rejoint_at, p.quitte_at, p.duree_secondes
         FROM afrolang.session_participant p
         LEFT JOIN iam.utilisateur u ON p.utilisateur_id = u.id
         WHERE p.session_id = $1
         ORDER BY p.rejoint_at ASC"
    )
    .bind(session_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(participants),
        error: None,
    }))
}

/// GET /api/admin/sessions/{id}/tableau-blanc
pub async fn obtenir_tableau_blanc(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let session_id = path.into_inner();

    let tableau = sqlx::query_as::<_, AdminTableauBlancResponse>(
        "SELECT tb.id, tb.session_id, tb.donnees, tb.version, tb.created_at, tb.updated_at
         FROM afrolang.tableau_blanc tb
         WHERE tb.session_id = $1"
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match tableau {
        Some(tb) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(tb),
            error: None,
        })),
        None => Ok(HttpResponse::Ok().json(ApiResponse::<()> {
            success: true,
            data: None,
            error: None,
        })),
    }
}

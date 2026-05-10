use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::salle::{
    AdminSalleDetailRow, AdminSalleListeResponse, AdminSalleQueryParams,
    CreerSalleRequest, ModifierSalleRequest,
    ADMIN_SALLE_DETAIL_COLONNES, ADMIN_SALLE_LISTE_COLONNES, SALLE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct AjouterPaysOrigineRequest {
    pub pays_id: Uuid,
}

fn generer_slug(titre: &str) -> String {
    titre
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

/// GET /api/admin/salles
pub async fn lister_salles(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    query: web::Query<AdminSalleQueryParams>,
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
    let tri = pagination.colonne_tri(SALLE_TRI_COLONNES, "created_at");
    let dir = pagination.direction_tri();

    let mut conditions = vec!["s.deleted_at IS NULL".to_string()];
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut bool_binds: Vec<bool> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = query.recherche {
        if !recherche.is_empty() {
            conditions.push(format!(
                "(s.titre ILIKE ${bi} OR s.description ILIKE ${bi} OR s.langue_cible ILIKE ${bi} OR s.langue_code ILIKE ${bi})",
                bi = bind_index
            ));
            str_binds.push(format!("%{}%", recherche));
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref langue) = query.langue_cible {
        if !langue.is_empty() {
            conditions.push(format!("s.langue_cible ILIKE ${}", bind_index));
            str_binds.push(format!("%{}%", langue));
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref code) = query.langue_code {
        if !code.is_empty() {
            conditions.push(format!("s.langue_code ILIKE ${}", bind_index));
            str_binds.push(format!("%{}%", code));
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(groupe_id) = query.groupe_ethnique_id {
        conditions.push(format!("s.groupe_ethnique_id = ${}", bind_index));
        uuid_binds.push(groupe_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(actif) = query.actif {
        conditions.push(format!("s.actif = ${}", bind_index));
        bool_binds.push(actif);
        param_types.push("bool");
        bind_index += 1;
    }

    let _ = bind_index;
    let where_clause = conditions.join(" AND ");

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM afrolang.salle s WHERE {}", where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    let mut str_idx = 0usize;
    let mut uuid_idx = 0usize;
    let mut bool_idx = 0usize;
    for pt in &param_types {
        match *pt {
            "str" => { count_q = count_q.bind(&str_binds[str_idx]); str_idx += 1; }
            "uuid" => { count_q = count_q.bind(uuid_binds[uuid_idx]); uuid_idx += 1; }
            "bool" => { count_q = count_q.bind(bool_binds[bool_idx]); bool_idx += 1; }
            _ => {}
        }
    }
    let total = count_q.fetch_one(pool.get_ref()).await?;

    // Data
    let data_sql = format!(
        "SELECT {} FROM afrolang.salle s
         LEFT JOIN country_profile.groupe_ethnique ge ON ge.id = s.groupe_ethnique_id
         WHERE {} ORDER BY s.{} {} LIMIT {} OFFSET {}",
        ADMIN_SALLE_LISTE_COLONNES, where_clause, tri, dir, par_page, offset
    );
    let mut data_q = sqlx::query_as::<_, AdminSalleListeResponse>(&data_sql);
    str_idx = 0;
    uuid_idx = 0;
    bool_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => { data_q = data_q.bind(&str_binds[str_idx]); str_idx += 1; }
            "uuid" => { data_q = data_q.bind(uuid_binds[uuid_idx]); uuid_idx += 1; }
            "bool" => { data_q = data_q.bind(bool_binds[bool_idx]); bool_idx += 1; }
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

/// GET /api/admin/salles/{id}
pub async fn obtenir_salle(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM afrolang.salle s
         LEFT JOIN country_profile.groupe_ethnique ge ON ge.id = s.groupe_ethnique_id
         LEFT JOIN iam.utilisateur cr ON s.cree_par = cr.id
         WHERE s.id = $1",
        ADMIN_SALLE_DETAIL_COLONNES
    );

    let row = sqlx::query_as::<_, AdminSalleDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Salle non trouvée".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/admin/salles
pub async fn creer_salle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerSalleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");

    let slug = generer_slug(&body.titre);
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO afrolang.salle
            (id, titre, slug, description, langue_cible, langue_code,
             alphabet, dictionnaire_url, groupe_ethnique_id, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
    )
    .bind(id)
    .bind(&body.titre)
    .bind(&slug)
    .bind(&body.description)
    .bind(&body.langue_cible)
    .bind(&body.langue_code)
    .bind(&body.alphabet)
    .bind(&body.dictionnaire_url)
    .bind(body.groupe_ethnique_id)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "afrolang",
        "salle",
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

/// PUT /api/admin/salles/{id}
pub async fn modifier_salle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierSalleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");

    let id = path.into_inner();
    let mut sets: Vec<String> = Vec::new();
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut bool_binds: Vec<bool> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! ajouter_champ_str {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                str_binds.push(val.clone());
                param_types.push("str");
                bind_index += 1;
            }
        };
    }

    ajouter_champ_str!(body.titre, "titre");
    ajouter_champ_str!(body.description, "description");
    ajouter_champ_str!(body.langue_cible, "langue_cible");
    ajouter_champ_str!(body.langue_code, "langue_code");
    ajouter_champ_str!(body.alphabet, "alphabet");
    ajouter_champ_str!(body.dictionnaire_url, "dictionnaire_url");

    if let Some(groupe_id) = body.groupe_ethnique_id {
        sets.push(format!("groupe_ethnique_id = ${}", bind_index));
        uuid_binds.push(groupe_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(actif) = body.actif {
        sets.push(format!("actif = ${}", bind_index));
        bool_binds.push(actif);
        param_types.push("bool");
        bind_index += 1;
    }

    if let Some(ref titre) = body.titre {
        sets.push(format!("slug = ${}", bind_index));
        str_binds.push(generer_slug(titre));
        param_types.push("str");
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ à modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE afrolang.salle SET {} WHERE id = ${}",
        sets.join(", "),
        bind_index
    );

    let mut query = sqlx::query(&sql);
    let mut str_idx = 0usize;
    let mut uuid_idx = 0usize;
    let mut bool_idx = 0usize;
    for pt in &param_types {
        match *pt {
            "str" => { query = query.bind(&str_binds[str_idx]); str_idx += 1; }
            "uuid" => { query = query.bind(uuid_binds[uuid_idx]); uuid_idx += 1; }
            "bool" => { query = query.bind(bool_binds[bool_idx]); bool_idx += 1; }
            _ => {}
        }
    }
    query = query.bind(id);

    let result = query.execute(pool.get_ref()).await?;
    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Salle non trouvée".into()));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "afrolang",
        "salle",
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

/// DELETE /api/admin/salles/{id}
pub async fn supprimer_salle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "supprimer");

    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE afrolang.salle SET actif = false, updated_at = NOW() WHERE id = $1 AND actif = true"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Salle non trouvée".into()));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "afrolang",
        "salle",
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

/// POST /api/admin/afrolang/salles/{id}/pays
/// Associe un pays au tableau « pays d'origine » d'une salle publique
/// (feature 001-afrolang-pays-origine).
pub async fn ajouter_pays_origine_salle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterPaysOrigineRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");
    let salle_id = path.into_inner();

    // Vérifier l'existence de la salle (non supprimée)
    let salle_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_existe {
        return Err(ApiErreur::NonTrouve("Salle non trouvée".into()));
    }

    // Vérifier l'existence du pays actif
    let pays_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM shared.pays WHERE id = $1 AND actif = true)",
    )
    .bind(body.pays_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !pays_existe {
        return Err(ApiErreur::NonTrouve("Pays non trouvé ou archivé".into()));
    }

    // Insertion idempotente
    sqlx::query(
        "INSERT INTO afrolang.salle_pays_origine (salle_id, pays_id) \
         VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(salle_id)
    .bind(body.pays_id)
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a ajouté le pays {} à la salle afrolang {}",
        admin.id, body.pays_id, salle_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "afrolang",
        "salle_pays_origine",
        Some(salle_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "salle_id": salle_id, "pays_id": body.pays_id })),
        error: None,
    }))
}

/// DELETE /api/admin/afrolang/salles/{id}/pays/{pays_id}
/// Retire un pays du tableau « pays d'origine » d'une salle publique.
pub async fn retirer_pays_origine_salle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");
    let (salle_id, pays_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM afrolang.salle_pays_origine WHERE salle_id = $1 AND pays_id = $2",
    )
    .bind(salle_id)
    .bind(pays_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Couple salle/pays d'origine non trouvé".into(),
        ));
    }

    log::info!(
        "Admin {} a retiré le pays {} de la salle afrolang {}",
        admin.id, pays_id, salle_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "afrolang",
        "salle_pays_origine",
        Some(salle_id),
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

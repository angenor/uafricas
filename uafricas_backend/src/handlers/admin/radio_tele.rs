use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::radio_tele::{
    AdminStationRadioListeResponse, AdminStationRadioDetailRow, AdminStationRadioQueryParams,
    CreerStationRadioRequest, ModifierStationRadioRequest,
    AdminChaineTvListeResponse, AdminChaineTvDetailRow, AdminChaineTvQueryParams,
    CreerChaineTvRequest, ModifierChaineTvRequest,
    AdminProgrammeMediaListeResponse, AdminProgrammeMediaDetailRow, AdminProgrammeMediaQueryParams,
    CreerProgrammeMediaRequest, ModifierProgrammeMediaRequest,
    ADMIN_STATION_RADIO_LISTE_COLONNES, ADMIN_STATION_RADIO_DETAIL_COLONNES, STATION_RADIO_TRI_COLONNES,
    ADMIN_CHAINE_TV_LISTE_COLONNES, ADMIN_CHAINE_TV_DETAIL_COLONNES, CHAINE_TV_TRI_COLONNES,
    ADMIN_PROGRAMME_MEDIA_LISTE_COLONNES, ADMIN_PROGRAMME_MEDIA_DETAIL_COLONNES, PROGRAMME_MEDIA_TRI_COLONNES,
    generer_slug,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

const TYPES_STATION_VALIDES: &[&str] = &["nationale", "locale", "internationale"];
const CATEGORIES_CHAINE_VALIDES: &[&str] = &[
    "generaliste", "info", "sport", "culture", "divertissement", "religieux", "education", "musique",
];
const TYPES_PROGRAMME_VALIDES: &[&str] = &["radio", "tele"];

// ══════════════════════════════════════════════════════════════
// STATIONS RADIO
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/stations-radio
pub async fn lister_stations_radio(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminStationRadioQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["s.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;
    let mut bind_types: Vec<&str> = Vec::new();

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(s.nom) LIKE ${bi} OR LOWER(s.description) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref type_station) = params.type_station {
        let t = type_station.trim();
        if !t.is_empty() {
            conditions.push(format!("s.type_station::TEXT = ${}", bind_index));
            bind_values.push(t.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("s.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("s.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_types.push("uuid");
        bind_index += 1;
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(STATION_RADIO_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN shared.pays ON s.pays_id = pays.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.station_radio s {} WHERE {}",
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

    let select_sql = format!(
        "SELECT {} FROM media_content.station_radio s {} WHERE {} ORDER BY s.{} {} LIMIT {} OFFSET {}",
        ADMIN_STATION_RADIO_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminStationRadioListeResponse>(&select_sql);
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

/// GET /api/admin/stations-radio/{id}
pub async fn obtenir_station_radio(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN shared.pays ON s.pays_id = pays.id
                 LEFT JOIN iam.utilisateur u ON s.cree_par = u.id";

    let sql = format!(
        "SELECT {} FROM media_content.station_radio s {} WHERE s.id = $1 AND s.deleted_at IS NULL",
        ADMIN_STATION_RADIO_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminStationRadioDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Station radio non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/admin/stations-radio
pub async fn creer_station_radio(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerStationRadioRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de la station est requis".into()));
    }
    let stream_url = body.stream_url.trim();
    if stream_url.is_empty() {
        return Err(ApiErreur::Validation("L'URL de stream est requise".into()));
    }

    if let Some(ref ts) = body.type_station {
        if !TYPES_STATION_VALIDES.contains(&ts.as_str()) {
            return Err(ApiErreur::Validation(format!("Type station invalide: {}", ts)));
        }
    }

    let id = Uuid::new_v4();
    let slug = generer_slug(nom);
    let type_station = body.type_station.as_deref().unwrap_or("nationale");
    let genres = body.genres_liste.clone().unwrap_or_default();

    sqlx::query(
        "INSERT INTO media_content.station_radio
         (id, nom, slug, description, stream_url, image_couverture_url, genre, genres_liste,
          pays_id, ville, type_station, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                 $11::media_content.type_station, 'brouillon', $12)"
    )
    .bind(id)
    .bind(nom)
    .bind(&slug)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(stream_url)
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.genre.as_deref().map(|s| s.trim()))
    .bind(&genres)
    .bind(body.pays_id)
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(type_station)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree la station radio {} ({})", admin.id, nom, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "media_content",
        "station_radio",
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

/// PUT /api/admin/stations-radio/{id}
pub async fn modifier_station_radio(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierStationRadioRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.station_radio WHERE id = $1 AND deleted_at IS NULL)"
    ).bind(id).fetch_one(pool.get_ref()).await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Station radio non trouvee".into()));
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! champ_str {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    champ_str!(body.nom, "nom");
    champ_str!(body.description, "description");
    champ_str!(body.stream_url, "stream_url");
    champ_str!(body.image_couverture_url, "image_couverture_url");
    champ_str!(body.genre, "genre");
    champ_str!(body.ville, "ville");

    if let Some(ref ts) = body.type_station {
        if !TYPES_STATION_VALIDES.contains(&ts.as_str()) {
            return Err(ApiErreur::Validation(format!("Type station invalide: {}", ts)));
        }
        sets.push(format!("type_station = ${}::media_content.type_station", bind_index));
        bind_strings.push(ts.clone());
        bind_index += 1;
    }

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }

    if let Some(ref genres) = body.genres_liste {
        let arr: Vec<String> = genres.iter().map(|g| format!("'{}'", g.replace('\'', "''"))).collect();
        sets.push(format!("genres_liste = ARRAY[{}]::TEXT[]", arr.join(",")));
    }

    if body.nom.is_some() {
        let nom = body.nom.as_ref().unwrap().trim();
        let slug = generer_slug(nom);
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(slug);
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE media_content.station_radio SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie la station radio {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "station_radio",
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

/// DELETE /api/admin/stations-radio/{id}
pub async fn supprimer_station_radio(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.station_radio SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    ).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Station radio non trouvee".into()));
    }

    log::info!("Admin {} a supprime la station radio {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "media_content",
        "station_radio",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// CHAINES TV
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/chaines-tv
pub async fn lister_chaines_tv(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminChaineTvQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["c.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;
    let mut bind_types: Vec<&str> = Vec::new();

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(c.nom) LIKE ${bi} OR LOWER(c.description) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref categorie) = params.categorie {
        let cat = categorie.trim();
        if !cat.is_empty() {
            conditions.push(format!("c.categorie::TEXT = ${}", bind_index));
            bind_values.push(cat.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("c.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("c.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_types.push("uuid");
        bind_index += 1;
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(CHAINE_TV_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN shared.pays ON c.pays_id = pays.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.chaine_tv c {} WHERE {}",
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

    let select_sql = format!(
        "SELECT {} FROM media_content.chaine_tv c {} WHERE {} ORDER BY c.{} {} LIMIT {} OFFSET {}",
        ADMIN_CHAINE_TV_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminChaineTvListeResponse>(&select_sql);
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

/// GET /api/admin/chaines-tv/{id}
pub async fn obtenir_chaine_tv(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN shared.pays ON c.pays_id = pays.id
                 LEFT JOIN iam.utilisateur u ON c.cree_par = u.id";

    let sql = format!(
        "SELECT {} FROM media_content.chaine_tv c {} WHERE c.id = $1 AND c.deleted_at IS NULL",
        ADMIN_CHAINE_TV_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminChaineTvDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Chaine TV non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/admin/chaines-tv
pub async fn creer_chaine_tv(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerChaineTvRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de la chaine est requis".into()));
    }
    let stream_url = body.stream_url.trim();
    if stream_url.is_empty() {
        return Err(ApiErreur::Validation("L'URL de stream est requise".into()));
    }

    if let Some(ref cat) = body.categorie {
        if !CATEGORIES_CHAINE_VALIDES.contains(&cat.as_str()) {
            return Err(ApiErreur::Validation(format!("Categorie invalide: {}", cat)));
        }
    }

    let id = Uuid::new_v4();
    let slug = generer_slug(nom);
    let categorie = body.categorie.as_deref().unwrap_or("generaliste");
    let langue = body.langue.as_deref().unwrap_or("Français");

    sqlx::query(
        "INSERT INTO media_content.chaine_tv
         (id, nom, slug, description, stream_url, image_couverture_url,
          categorie, pays_id, langue, est_en_direct, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6,
                 $7::media_content.categorie_chaine_tv, $8, $9, $10, 'brouillon', $11)"
    )
    .bind(id)
    .bind(nom)
    .bind(&slug)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(stream_url)
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(categorie)
    .bind(body.pays_id)
    .bind(langue)
    .bind(body.est_en_direct.unwrap_or(true))
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree la chaine TV {} ({})", admin.id, nom, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "media_content",
        "chaine_tv",
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

/// PUT /api/admin/chaines-tv/{id}
pub async fn modifier_chaine_tv(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierChaineTvRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.chaine_tv WHERE id = $1 AND deleted_at IS NULL)"
    ).bind(id).fetch_one(pool.get_ref()).await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Chaine TV non trouvee".into()));
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! champ_str {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    champ_str!(body.nom, "nom");
    champ_str!(body.description, "description");
    champ_str!(body.stream_url, "stream_url");
    champ_str!(body.image_couverture_url, "image_couverture_url");
    champ_str!(body.langue, "langue");

    if let Some(ref cat) = body.categorie {
        if !CATEGORIES_CHAINE_VALIDES.contains(&cat.as_str()) {
            return Err(ApiErreur::Validation(format!("Categorie invalide: {}", cat)));
        }
        sets.push(format!("categorie = ${}::media_content.categorie_chaine_tv", bind_index));
        bind_strings.push(cat.clone());
        bind_index += 1;
    }

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }
    if let Some(v) = body.est_en_direct {
        sets.push(format!("est_en_direct = {}", v));
    }

    if body.nom.is_some() {
        let nom = body.nom.as_ref().unwrap().trim();
        let slug = generer_slug(nom);
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(slug);
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE media_content.chaine_tv SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie la chaine TV {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "chaine_tv",
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

/// DELETE /api/admin/chaines-tv/{id}
pub async fn supprimer_chaine_tv(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.chaine_tv SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    ).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Chaine TV non trouvee".into()));
    }

    log::info!("Admin {} a supprime la chaine TV {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "media_content",
        "chaine_tv",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// PROGRAMMES RADIO / TELE
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/programmes-media
pub async fn lister_programmes_media(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminProgrammeMediaQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["p.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(p.nom_emission) LIKE ${bi} OR LOWER(p.description) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref tp) = params.type_programme {
        let t = tp.trim();
        if !t.is_empty() {
            conditions.push(format!("p.type::TEXT = ${}", bind_index));
            bind_values.push(t.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref cat) = params.categorie_radio {
        let c = cat.trim();
        if !c.is_empty() {
            conditions.push(format!("p.categorie_radio::TEXT = ${}", bind_index));
            bind_values.push(c.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("p.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(PROGRAMME_MEDIA_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN shared.pays ON p.pays_id = pays.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.programme_radio_tele p {} WHERE {}",
        joins, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM media_content.programme_radio_tele p {} WHERE {} ORDER BY p.{} {} LIMIT {} OFFSET {}",
        ADMIN_PROGRAMME_MEDIA_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminProgrammeMediaListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/programmes-media/{id}
pub async fn obtenir_programme_media(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN shared.pays ON p.pays_id = pays.id
                 LEFT JOIN iam.utilisateur u ON p.cree_par = u.id";

    let sql = format!(
        "SELECT {} FROM media_content.programme_radio_tele p {} WHERE p.id = $1 AND p.deleted_at IS NULL",
        ADMIN_PROGRAMME_MEDIA_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminProgrammeMediaDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Programme media non trouve".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/admin/programmes-media
pub async fn creer_programme_media(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerProgrammeMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");

    let nom = body.nom_emission.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de l'emission est requis".into()));
    }
    if !TYPES_PROGRAMME_VALIDES.contains(&body.type_programme.as_str()) {
        return Err(ApiErreur::Validation(format!("Type programme invalide: {}", body.type_programme)));
    }

    let id = Uuid::new_v4();
    let slug = generer_slug(nom);
    let langue = body.langue.as_deref().unwrap_or("Français");

    sqlx::query(
        "INSERT INTO media_content.programme_radio_tele
         (id, nom_emission, slug, type, description, image_couverture_url, video_url,
          info_animateur, info_producteur, pays_id, est_international, langue,
          categorie_radio, etat, cree_par)
         VALUES ($1, $2, $3, $4::media_content.type_programme_media, $5, $6, $7,
                 $8, $9, $10, $11, $12,
                 $13::media_content.categorie_radio, 'brouillon', $14)"
    )
    .bind(id)
    .bind(nom)
    .bind(&slug)
    .bind(&body.type_programme)
    .bind(body.description.trim())
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.video_url.as_deref().map(|s| s.trim()))
    .bind(body.info_animateur.as_deref().map(|s| s.trim()))
    .bind(body.info_producteur.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.est_international.unwrap_or(false))
    .bind(langue)
    .bind(body.categorie_radio.as_deref())
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le programme media {} ({})", admin.id, nom, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "media_content",
        "programme_media",
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

/// PUT /api/admin/programmes-media/{id}
pub async fn modifier_programme_media(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierProgrammeMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.programme_radio_tele WHERE id = $1 AND deleted_at IS NULL)"
    ).bind(id).fetch_one(pool.get_ref()).await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Programme media non trouve".into()));
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! champ_str {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    champ_str!(body.nom_emission, "nom_emission");
    champ_str!(body.description, "description");
    champ_str!(body.image_couverture_url, "image_couverture_url");
    champ_str!(body.video_url, "video_url");
    champ_str!(body.info_animateur, "info_animateur");
    champ_str!(body.info_producteur, "info_producteur");
    champ_str!(body.langue, "langue");

    if let Some(ref tp) = body.type_programme {
        if !TYPES_PROGRAMME_VALIDES.contains(&tp.as_str()) {
            return Err(ApiErreur::Validation(format!("Type programme invalide: {}", tp)));
        }
        sets.push(format!("type = ${}::media_content.type_programme_media", bind_index));
        bind_strings.push(tp.clone());
        bind_index += 1;
    }

    if let Some(ref cat) = body.categorie_radio {
        sets.push(format!("categorie_radio = ${}::media_content.categorie_radio", bind_index));
        bind_strings.push(cat.clone());
        bind_index += 1;
    }

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }
    if let Some(v) = body.est_international {
        sets.push(format!("est_international = {}", v));
    }

    if body.nom_emission.is_some() {
        let nom = body.nom_emission.as_ref().unwrap().trim();
        let slug = generer_slug(nom);
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(slug);
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE media_content.programme_radio_tele SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie le programme media {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "programme_media",
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

/// DELETE /api/admin/programmes-media/{id}
pub async fn supprimer_programme_media(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.programme_radio_tele SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    ).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Programme media non trouve".into()));
    }

    log::info!("Admin {} a supprime le programme media {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "media_content",
        "programme_media",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

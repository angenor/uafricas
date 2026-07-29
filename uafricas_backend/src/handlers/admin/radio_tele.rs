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
    AdminProgrammeRadioListeResponse, AdminProgrammeRadioDetailRow, AdminProgrammeRadioQueryParams,
    CreerProgrammeRadioRequest, ModifierProgrammeRadioRequest,
    AdminProgrammeTeleListeResponse, AdminProgrammeTeleDetailRow, AdminProgrammeTeleQueryParams,
    CreerProgrammeTeleRequest, ModifierProgrammeTeleRequest,
    ADMIN_STATION_RADIO_LISTE_COLONNES, ADMIN_STATION_RADIO_DETAIL_COLONNES, STATION_RADIO_TRI_COLONNES,
    ADMIN_CHAINE_TV_LISTE_COLONNES, ADMIN_CHAINE_TV_DETAIL_COLONNES, CHAINE_TV_TRI_COLONNES,
    ADMIN_PROGRAMME_RADIO_LISTE_COLONNES, ADMIN_PROGRAMME_RADIO_DETAIL_COLONNES, PROGRAMME_RADIO_TRI_COLONNES,
    ADMIN_PROGRAMME_TELE_LISTE_COLONNES, ADMIN_PROGRAMME_TELE_DETAIL_COLONNES, PROGRAMME_TELE_TRI_COLONNES,
    generer_slug,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::services::contacts_media::{normaliser_url, texte_non_vide};
use crate::verifier_permission;
use crate::ApiResponse;

const TYPES_STATION_VALIDES: &[&str] = &["nationale", "locale", "internationale"];
const CATEGORIES_CHAINE_VALIDES: &[&str] = &[
    "generaliste", "info", "sport", "culture", "divertissement", "religieux", "education", "musique",
];
// États valides pour les médias radio/télé (stations, chaînes, programmes)
const ETATS_MEDIA_VALIDES: &[&str] = &["brouillon", "publie", "suspendu", "supprime"];

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
    // Flux live et audio (fichier/lien) sont tous deux optionnels — au moins un est attendu.
    let stream_url = body.stream_url.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let audio_url = body.audio_url.as_deref().map(str::trim).filter(|s| !s.is_empty());
    if stream_url.is_none() && audio_url.is_none() {
        return Err(ApiErreur::Validation(
            "Fournissez au moins un fichier/lien audio ou une URL de flux live".into(),
        ));
    }

    if let Some(ref ts) = body.type_station {
        if !TYPES_STATION_VALIDES.contains(&ts.as_str()) {
            return Err(ApiErreur::Validation(format!("Type station invalide: {}", ts)));
        }
    }

    // L'origine décide de LA PAGE sur laquelle la station apparaîtra : une
    // valeur hors whitelist violerait le CHECK en base, autant la refuser ici
    // avec un message clair.
    let origine = body.origine_publication.as_deref().unwrap_or("territoire");
    if !crate::models::station_radio::origine_valide(origine) {
        return Err(ApiErreur::Validation(format!(
            "Origine de publication invalide: {} (attendu : africans ou territoire)",
            origine
        )));
    }

    let id = Uuid::new_v4();
    let slug = generer_slug(nom);
    let type_station = body.type_station.as_deref().unwrap_or("nationale");
    let genres = body.genres_liste.clone().unwrap_or_default();

    sqlx::query(
        "INSERT INTO media_content.station_radio
         (id, nom, slug, description, stream_url, audio_url, image_couverture_url, genre, genres_liste,
          pays_id, ville, type_station, a_la_une, origine_publication,
          role_partie_prenante, role_partie_prenante_autre,
          contact_email, contact_telephone, contact_whatsapp, contact_site_web, contact_adresse,
          etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
                 $12::media_content.type_station, $13, $14, $15, $16,
                 $17, $18, $19, $20, $21, 'brouillon', $22)"
    )
    .bind(id)
    .bind(nom)
    .bind(&slug)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(stream_url)
    .bind(audio_url)
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.genre.as_deref().map(|s| s.trim()))
    .bind(&genres)
    .bind(body.pays_id)
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(type_station)
    .bind(body.a_la_une.unwrap_or(false))
    // Défaut 'territoire' : une station soumise relève de son territoire ; la
    // publication sous la bannière Radio Africans est une décision éditoriale.
    .bind(origine)
    .bind(body.role_partie_prenante.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .bind(body.role_partie_prenante_autre.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .bind(texte_non_vide(body.contact_email.as_deref()))
    .bind(texte_non_vide(body.contact_telephone.as_deref()))
    .bind(texte_non_vide(body.contact_whatsapp.as_deref()))
    .bind(normaliser_url(body.contact_site_web.as_deref()))
    .bind(texte_non_vide(body.contact_adresse.as_deref()))
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
    champ_str!(body.audio_url, "audio_url");
    champ_str!(body.image_couverture_url, "image_couverture_url");
    champ_str!(body.genre, "genre");
    champ_str!(body.ville, "ville");
    champ_str!(body.contact_email, "contact_email");
    champ_str!(body.contact_telephone, "contact_telephone");
    champ_str!(body.contact_whatsapp, "contact_whatsapp");
    champ_str!(body.contact_adresse, "contact_adresse");

    // Le site web ne passe pas par `champ_str!` : il lui faut son schéma, sans
    // quoi « www.radio.fm » serait rendu comme un lien relatif.
    if let Some(ref val) = body.contact_site_web {
        sets.push(format!("contact_site_web = ${}", bind_index));
        bind_strings.push(normaliser_url(Some(val)).unwrap_or_default());
        bind_index += 1;
    }

    if let Some(ref etat) = body.etat {
        let e = etat.trim();
        if !ETATS_MEDIA_VALIDES.contains(&e) {
            return Err(ApiErreur::Validation(format!("État invalide: {}", e)));
        }
        sets.push(format!("etat = ${}", bind_index));
        bind_strings.push(e.to_string());
        bind_index += 1;
    }

    if let Some(ref ts) = body.type_station {
        if !TYPES_STATION_VALIDES.contains(&ts.as_str()) {
            return Err(ApiErreur::Validation(format!("Type station invalide: {}", ts)));
        }
        sets.push(format!("type_station = ${}::media_content.type_station", bind_index));
        bind_strings.push(ts.clone());
        bind_index += 1;
    }

    if let Some(v) = body.a_la_une {
        sets.push(format!("a_la_une = {}", v));
    }

    // Changer l'origine fait BASCULER la station d'une page Radio à l'autre :
    // c'est une décision éditoriale, tracée comme telle dans l'audit ci-dessous.
    if let Some(ref origine) = body.origine_publication {
        if !crate::models::station_radio::origine_valide(origine) {
            return Err(ApiErreur::Validation(format!(
                "Origine de publication invalide: {} (attendu : africans ou territoire)",
                origine
            )));
        }
        sets.push(format!("origine_publication = ${}", bind_index));
        bind_strings.push(origine.clone());
        bind_index += 1;
    }

    if let Some(ref role) = body.role_partie_prenante {
        sets.push(format!("role_partie_prenante = ${}", bind_index));
        bind_strings.push(role.clone());
        bind_index += 1;
    }
    if let Some(ref role_autre) = body.role_partie_prenante_autre {
        sets.push(format!("role_partie_prenante_autre = ${}", bind_index));
        bind_strings.push(role_autre.clone());
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

    if let Some(ref origine) = params.origine {
        let o = origine.trim();
        if !o.is_empty() {
            conditions.push(format!("c.origine_publication = ${}", bind_index));
            bind_values.push(o.to_string());
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
    // Flux live optionnel — le cœur de la télé = les programmes (cf. migration 09d).
    let stream_url = body.stream_url.as_deref().map(str::trim).filter(|s| !s.is_empty());

    if let Some(ref cat) = body.categorie {
        if !CATEGORIES_CHAINE_VALIDES.contains(&cat.as_str()) {
            return Err(ApiErreur::Validation(format!("Categorie invalide: {}", cat)));
        }
    }

    // Même référentiel que la radio (09o reprend le CHECK de 09j) : une valeur
    // hors whitelist violerait la contrainte en base, autant la refuser ici.
    let origine = body.origine_publication.as_deref().unwrap_or("territoire");
    if !crate::models::station_radio::origine_valide(origine) {
        return Err(ApiErreur::Validation(format!(
            "Origine de publication invalide: {} (attendu : africans ou territoire)",
            origine
        )));
    }

    let id = Uuid::new_v4();
    let slug = generer_slug(nom);
    let categorie = body.categorie.as_deref().unwrap_or("generaliste");
    let langue = body.langue.as_deref().unwrap_or("Français");

    sqlx::query(
        "INSERT INTO media_content.chaine_tv
         (id, nom, slug, description, stream_url, image_couverture_url,
          categorie, pays_id, langue, est_en_direct, origine_publication,
          contact_email, contact_telephone, contact_whatsapp, contact_site_web, contact_adresse,
          etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6,
                 $7::media_content.categorie_chaine_tv, $8, $9, $10, $11,
                 $12, $13, $14, $15, $16, 'brouillon', $17)"
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
    .bind(origine)
    .bind(texte_non_vide(body.contact_email.as_deref()))
    .bind(texte_non_vide(body.contact_telephone.as_deref()))
    .bind(texte_non_vide(body.contact_whatsapp.as_deref()))
    .bind(normaliser_url(body.contact_site_web.as_deref()))
    .bind(texte_non_vide(body.contact_adresse.as_deref()))
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
    champ_str!(body.contact_email, "contact_email");
    champ_str!(body.contact_telephone, "contact_telephone");
    champ_str!(body.contact_whatsapp, "contact_whatsapp");
    champ_str!(body.contact_adresse, "contact_adresse");

    // Cf. `modifier_station_radio` : le site web exige son schéma.
    if let Some(ref val) = body.contact_site_web {
        sets.push(format!("contact_site_web = ${}", bind_index));
        bind_strings.push(normaliser_url(Some(val)).unwrap_or_default());
        bind_index += 1;
    }

    if let Some(ref etat) = body.etat {
        let e = etat.trim();
        if !ETATS_MEDIA_VALIDES.contains(&e) {
            return Err(ApiErreur::Validation(format!("État invalide: {}", e)));
        }
        sets.push(format!("etat = ${}", bind_index));
        bind_strings.push(e.to_string());
        bind_index += 1;
    }

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

    // Bascule la chaîne dans (ou hors de) « Africans Télé International » : la
    // chaîne reste sur /medias/tele, seul le filtre qui la remonte change.
    if let Some(ref origine) = body.origine_publication {
        if !crate::models::station_radio::origine_valide(origine) {
            return Err(ApiErreur::Validation(format!(
                "Origine de publication invalide: {} (attendu : africans ou territoire)",
                origine
            )));
        }
        sets.push(format!("origine_publication = ${}", bind_index));
        bind_strings.push(origine.clone());
        bind_index += 1;
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
// PROGRAMMES RADIO (émissions)
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/programmes-radio
pub async fn lister_programmes_radio(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminProgrammeRadioQueryParams>,
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

    if let Some(ref cat) = params.categorie_radio {
        let c = cat.trim();
        if !c.is_empty() {
            conditions.push(format!("p.categorie_radio::TEXT = ${}", bind_index));
            bind_values.push(c.to_string());
            bind_index += 1;
        }
    }

    if let Some(station_id) = params.station_id {
        conditions.push(format!("p.station_id = ${}::uuid", bind_index));
        bind_values.push(station_id.to_string());
        bind_index += 1;
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
    let colonne = pagination.colonne_tri(PROGRAMME_RADIO_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN shared.pays ON p.pays_id = pays.id
                 LEFT JOIN media_content.station_radio st ON p.station_id = st.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.programme_radio p {} WHERE {}",
        joins, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM media_content.programme_radio p {} WHERE {} ORDER BY p.{} {} LIMIT {} OFFSET {}",
        ADMIN_PROGRAMME_RADIO_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminProgrammeRadioListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/programmes-radio/{id}
pub async fn obtenir_programme_radio(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN shared.pays ON p.pays_id = pays.id
                 LEFT JOIN media_content.station_radio st ON p.station_id = st.id
                 LEFT JOIN iam.utilisateur u ON p.cree_par = u.id";

    let sql = format!(
        "SELECT {} FROM media_content.programme_radio p {} WHERE p.id = $1 AND p.deleted_at IS NULL",
        ADMIN_PROGRAMME_RADIO_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminProgrammeRadioDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Programme radio non trouve".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/admin/programmes-radio
pub async fn creer_programme_radio(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerProgrammeRadioRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");

    let nom = body.nom_emission.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de l'emission est requis".into()));
    }

    let id = Uuid::new_v4();
    let slug = generer_slug(nom);
    let langue = body.langue.as_deref().unwrap_or("Français");
    let a_la_une = body.a_la_une.unwrap_or(false);

    // L'index unique partiel sur (station_id, a_la_une) rejetterait deux mises à la une concurrentes : on sérialise démarcation et insertion.
    let mut tx = pool.begin().await?;

    // Une seule émission « à la une » par station : on retire le marqueur des autres
    if a_la_une {
        if let Some(station_id) = body.station_id {
            sqlx::query(
                "UPDATE media_content.programme_radio SET a_la_une = FALSE, updated_at = NOW()
                 WHERE station_id = $1 AND a_la_une = TRUE AND deleted_at IS NULL"
            )
            .bind(station_id)
            .execute(&mut *tx)
            .await?;
        }
    }

    sqlx::query(
        "INSERT INTO media_content.programme_radio
         (id, nom_emission, slug, description, image_couverture_url, audio_url,
          info_animateur, info_producteur, pays_id, est_international, langue,
          categorie_radio, station_id, a_la_une, theme_phare_id, theme_phare_autre, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6,
                 $7, $8, $9, $10, $11,
                 $12::media_content.categorie_radio, $13, $14, $15, $16, 'brouillon', $17)"
    )
    .bind(id)
    .bind(nom)
    .bind(&slug)
    .bind(body.description.as_deref().map(|s| s.trim()).unwrap_or(""))
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.audio_url.as_deref().map(|s| s.trim()))
    .bind(body.info_animateur.as_deref().map(|s| s.trim()))
    .bind(body.info_producteur.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.est_international.unwrap_or(false))
    .bind(langue)
    .bind(body.categorie_radio.as_deref())
    .bind(body.station_id)
    .bind(a_la_une)
    .bind(body.theme_phare_id)
    .bind(body.theme_phare_autre.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .bind(admin.id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    log::info!("Admin {} a cree le programme radio {} ({})", admin.id, nom, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "media_content", "programme_radio",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/programmes-radio/{id}
pub async fn modifier_programme_radio(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierProgrammeRadioRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.programme_radio WHERE id = $1 AND deleted_at IS NULL)"
    ).bind(id).fetch_one(pool.get_ref()).await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Programme radio non trouve".into()));
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
    champ_str!(body.audio_url, "audio_url");
    champ_str!(body.info_animateur, "info_animateur");
    champ_str!(body.info_producteur, "info_producteur");
    champ_str!(body.langue, "langue");

    if let Some(ref etat) = body.etat {
        let e = etat.trim();
        if !ETATS_MEDIA_VALIDES.contains(&e) {
            return Err(ApiErreur::Validation(format!("État invalide: {}", e)));
        }
        sets.push(format!("etat = ${}", bind_index));
        bind_strings.push(e.to_string());
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

    // Rattachement à une station
    if let Some(station_id) = body.station_id {
        sets.push(format!("station_id = '{}'", station_id));
    }

    // L'index unique partiel sur (station_id, a_la_une) rejetterait deux mises à la une concurrentes : on sérialise démarcation et mise à jour.
    let mut tx = pool.begin().await?;

    // Émission « à la une » : une seule par station
    if let Some(a_la_une) = body.a_la_une {
        if a_la_une {
            let station_eff: Option<Uuid> = match body.station_id {
                Some(s) => Some(s),
                None => sqlx::query_scalar(
                    "SELECT station_id FROM media_content.programme_radio WHERE id = $1"
                ).bind(id).fetch_one(&mut *tx).await?,
            };
            if let Some(st) = station_eff {
                sqlx::query(
                    "UPDATE media_content.programme_radio SET a_la_une = FALSE, updated_at = NOW()
                     WHERE station_id = $1 AND id <> $2 AND a_la_une = TRUE AND deleted_at IS NULL"
                ).bind(st).bind(id).execute(&mut *tx).await?;
            }
        }
        sets.push(format!("a_la_une = {}", a_la_une));
    }

    // Thème phare : un identifiant de référentiel, ou une précision libre quand
    // le contributeur a choisi « Autre ». Les deux sont mutuellement exclusifs
    // côté formulaire, mais rien n'interdit de les effacer l'un après l'autre.
    if let Some(theme_id) = body.theme_phare_id {
        sets.push(format!("theme_phare_id = '{}'", theme_id));
    }
    if let Some(ref theme_autre) = body.theme_phare_autre {
        let valeur = theme_autre.trim();
        if valeur.is_empty() {
            sets.push("theme_phare_autre = NULL".to_string());
        } else {
            sets.push(format!("theme_phare_autre = ${}", bind_index));
            bind_strings.push(valeur.to_string());
            bind_index += 1;
        }
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
        "UPDATE media_content.programme_radio SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(&mut *tx).await?;

    tx.commit().await?;

    log::info!("Admin {} a modifie le programme radio {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "media_content", "programme_radio",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/programmes-radio/{id}
pub async fn supprimer_programme_radio(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.programme_radio SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    ).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Programme radio non trouve".into()));
    }

    log::info!("Admin {} a supprime le programme radio {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "media_content", "programme_radio",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// PROGRAMMES TÉLÉ
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/programmes-tele
pub async fn lister_programmes_tele(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminProgrammeTeleQueryParams>,
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

    if let Some(chaine_id) = params.chaine_id {
        conditions.push(format!("p.chaine_id = ${}::uuid", bind_index));
        bind_values.push(chaine_id.to_string());
        bind_index += 1;
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
    let colonne = pagination.colonne_tri(PROGRAMME_TELE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN shared.pays ON p.pays_id = pays.id
                 LEFT JOIN media_content.chaine_tv ch ON p.chaine_id = ch.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.programme_tele p {} WHERE {}",
        joins, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM media_content.programme_tele p {} WHERE {} ORDER BY p.{} {} LIMIT {} OFFSET {}",
        ADMIN_PROGRAMME_TELE_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminProgrammeTeleListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/programmes-tele/{id}
pub async fn obtenir_programme_tele(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN shared.pays ON p.pays_id = pays.id
                 LEFT JOIN media_content.chaine_tv ch ON p.chaine_id = ch.id
                 LEFT JOIN iam.utilisateur u ON p.cree_par = u.id";

    let sql = format!(
        "SELECT {} FROM media_content.programme_tele p {} WHERE p.id = $1 AND p.deleted_at IS NULL",
        ADMIN_PROGRAMME_TELE_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminProgrammeTeleDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Programme tele non trouve".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/admin/programmes-tele
pub async fn creer_programme_tele(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerProgrammeTeleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");

    let nom = body.nom_emission.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de l'emission est requis".into()));
    }

    let id = Uuid::new_v4();
    let slug = generer_slug(nom);
    let langue = body.langue.as_deref().unwrap_or("Français");
    let a_la_une = body.a_la_une.unwrap_or(false);

    // L'index unique partiel sur (chaine_id, a_la_une) rejetterait deux mises à la une concurrentes : on sérialise démarcation et insertion.
    let mut tx = pool.begin().await?;

    // Un seul programme « à la une » par chaîne
    if a_la_une {
        if let Some(chaine_id) = body.chaine_id {
            sqlx::query(
                "UPDATE media_content.programme_tele SET a_la_une = FALSE, updated_at = NOW()
                 WHERE chaine_id = $1 AND a_la_une = TRUE AND deleted_at IS NULL"
            )
            .bind(chaine_id)
            .execute(&mut *tx)
            .await?;
        }
    }

    sqlx::query(
        "INSERT INTO media_content.programme_tele
         (id, nom_emission, slug, description, image_couverture_url, video_url,
          info_animateur, info_producteur, pays_id, est_international, langue,
          chaine_id, a_la_une, theme_phare_id, theme_phare_autre, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6,
                 $7, $8, $9, $10, $11,
                 $12, $13, $14, $15, 'brouillon', $16)"
    )
    .bind(id)
    .bind(nom)
    .bind(&slug)
    .bind(body.description.as_deref().map(|s| s.trim()).unwrap_or(""))
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.video_url.as_deref().map(|s| s.trim()))
    .bind(body.info_animateur.as_deref().map(|s| s.trim()))
    .bind(body.info_producteur.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.est_international.unwrap_or(false))
    .bind(langue)
    .bind(body.chaine_id)
    .bind(a_la_une)
    .bind(body.theme_phare_id)
    .bind(body.theme_phare_autre.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .bind(admin.id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    log::info!("Admin {} a cree le programme tele {} ({})", admin.id, nom, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "media_content", "programme_tele",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/programmes-tele/{id}
pub async fn modifier_programme_tele(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierProgrammeTeleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.programme_tele WHERE id = $1 AND deleted_at IS NULL)"
    ).bind(id).fetch_one(pool.get_ref()).await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Programme tele non trouve".into()));
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

    if let Some(ref etat) = body.etat {
        let e = etat.trim();
        if !ETATS_MEDIA_VALIDES.contains(&e) {
            return Err(ApiErreur::Validation(format!("État invalide: {}", e)));
        }
        // Une vidéo est requise pour publier un programme télé (contrainte chk_video_tele).
        if e == "publie" {
            let aura_video = match body.video_url.as_deref().map(str::trim) {
                Some(v) if !v.is_empty() => true,
                _ => sqlx::query_scalar::<_, bool>(
                    "SELECT video_url IS NOT NULL AND video_url <> '' \
                     FROM media_content.programme_tele WHERE id = $1",
                )
                .bind(id)
                .fetch_optional(pool.get_ref())
                .await?
                .unwrap_or(false),
            };
            if !aura_video {
                return Err(ApiErreur::Validation(
                    "Ajoutez une vidéo au programme avant de le publier.".into(),
                ));
            }
        }
        sets.push(format!("etat = ${}", bind_index));
        bind_strings.push(e.to_string());
        bind_index += 1;
    }

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }
    if let Some(v) = body.est_international {
        sets.push(format!("est_international = {}", v));
    }

    // Rattachement à une chaîne
    if let Some(chaine_id) = body.chaine_id {
        sets.push(format!("chaine_id = '{}'", chaine_id));
    }

    // L'index unique partiel sur (chaine_id, a_la_une) rejetterait deux mises à la une concurrentes : on sérialise démarcation et mise à jour.
    let mut tx = pool.begin().await?;

    // Programme « à la une » : un seul par chaîne
    if let Some(a_la_une) = body.a_la_une {
        if a_la_une {
            let chaine_eff: Option<Uuid> = match body.chaine_id {
                Some(c) => Some(c),
                None => sqlx::query_scalar(
                    "SELECT chaine_id FROM media_content.programme_tele WHERE id = $1"
                ).bind(id).fetch_one(&mut *tx).await?,
            };
            if let Some(ch) = chaine_eff {
                sqlx::query(
                    "UPDATE media_content.programme_tele SET a_la_une = FALSE, updated_at = NOW()
                     WHERE chaine_id = $1 AND id <> $2 AND a_la_une = TRUE AND deleted_at IS NULL"
                ).bind(ch).bind(id).execute(&mut *tx).await?;
            }
        }
        sets.push(format!("a_la_une = {}", a_la_une));
    }

    // Thème phare : un identifiant de référentiel, ou une précision libre quand
    // le contributeur a choisi « Autre ». Les deux sont mutuellement exclusifs
    // côté formulaire, mais rien n'interdit de les effacer l'un après l'autre.
    if let Some(theme_id) = body.theme_phare_id {
        sets.push(format!("theme_phare_id = '{}'", theme_id));
    }
    if let Some(ref theme_autre) = body.theme_phare_autre {
        let valeur = theme_autre.trim();
        if valeur.is_empty() {
            sets.push("theme_phare_autre = NULL".to_string());
        } else {
            sets.push(format!("theme_phare_autre = ${}", bind_index));
            bind_strings.push(valeur.to_string());
            bind_index += 1;
        }
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
        "UPDATE media_content.programme_tele SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(&mut *tx).await?;

    tx.commit().await?;

    log::info!("Admin {} a modifie le programme tele {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "media_content", "programme_tele",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/programmes-tele/{id}
pub async fn supprimer_programme_tele(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.programme_tele SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    ).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Programme tele non trouve".into()));
    }

    log::info!("Admin {} a supprime le programme tele {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "media_content", "programme_tele",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

/// PATCH /api/admin/programmes-tele/{id}/vedette-globale
///
/// Désigne le programme mis en avant sur TOUTE la page Télé (FR-001). Un index
/// unique partiel garantit l'unicité côté base : la démarcation de l'ancienne
/// vedette et la promotion de la nouvelle DOIVENT donc tenir dans une même
/// transaction, faute de quoi deux administrateurs agissant simultanément
/// feraient échouer la seconde requête sur violation de contrainte.
pub async fn definir_vedette_globale(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let id = path.into_inner();

    let mut tx = pool.begin().await?;

    // Un programme non publié ne peut pas devenir la vedette : la page publique
    // filtre sur `etat = 'publie'` et n'afficherait que son repli.
    let etat: Option<String> = sqlx::query_scalar(
        "SELECT etat FROM media_content.programme_tele
          WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;

    let etat = etat.ok_or_else(|| ApiErreur::NonTrouve("Programme tele non trouve".into()))?;
    if etat != "publie" {
        return Err(ApiErreur::Validation(
            "Seul un programme publié peut devenir la vedette de la page Télé".into(),
        ));
    }

    // Ancienne vedette conservée pour l'audit : sans cet instantané, la trace ne
    // dirait pas ce que la décision a remplacé.
    let ancienne: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM media_content.programme_tele
          WHERE a_la_une_globale = TRUE AND deleted_at IS NULL",
    )
    .fetch_optional(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE media_content.programme_tele
            SET a_la_une_globale = FALSE, updated_at = NOW()
          WHERE a_la_une_globale = TRUE AND id <> $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE media_content.programme_tele
            SET a_la_une_globale = TRUE, updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    log::info!(
        "Admin {} a designe le programme tele {} comme vedette globale",
        admin.id,
        id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "VEDETTE_GLOBALE",
        "media_content",
        "programme_tele",
        Some(id),
        Some(serde_json::json!({ "a_la_une_globale": ancienne })),
        Some(serde_json::json!({ "a_la_une_globale": id })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "ancienne_vedette": ancienne })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// UPLOAD DE FICHIERS MÉDIA (vidéo / audio)
// ══════════════════════════════════════════════════════════════

const EXTENSIONS_VIDEO: &[&str] = &["mp4", "webm", "mov", "m4v", "ogv"];
const EXTENSIONS_AUDIO: &[&str] = &["mp3", "ogg", "oga", "wav", "m4a", "aac"];
const TAILLE_MAX_VIDEO: usize = 300 * 1024 * 1024; // 300 Mo
const TAILLE_MAX_AUDIO: usize = 80 * 1024 * 1024; //  80 Mo

/// POST /api/admin/medias/upload
/// Reçoit un fichier vidéo OU audio (champ multipart `fichier`), le stocke sous
/// `/uploads/medias/{videos|audios}/` et renvoie son URL relative.
/// Permet aux formulaires radio/télé d'uploader directement un média (ou de
/// continuer à coller un lien externe sans passer par ici).
pub async fn uploader_media(
    admin: AdminUtilisateur,
    upload_dir: web::Data<String>,
    mut payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ApiErreur> {
    use futures_util::StreamExt;
    use std::io::Write;

    verifier_permission!(admin, "media", "modifier");

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| ApiErreur::Upload(format!("Champ multipart invalide: {}", e)))?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();
        if nom_champ != "fichier" {
            // Vider le champ ignoré
            while let Some(chunk) = field.next().await {
                let _ = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture: {}", e)))?;
            }
            continue;
        }

        // Déterminer l'extension depuis le nom de fichier
        let nom_fichier = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_filename())
            .map(|s| s.to_string())
            .unwrap_or_default();
        let ext = std::path::Path::new(&nom_fichier)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        let (sous_dossier, taille_max) = if EXTENSIONS_VIDEO.contains(&ext.as_str()) {
            ("videos", TAILLE_MAX_VIDEO)
        } else if EXTENSIONS_AUDIO.contains(&ext.as_str()) {
            ("audios", TAILLE_MAX_AUDIO)
        } else {
            return Err(ApiErreur::Validation(format!(
                "Format de fichier non supporté: « {} ». Vidéos: {} ; Audios: {}.",
                if ext.is_empty() { "inconnu" } else { &ext },
                EXTENSIONS_VIDEO.join(", "),
                EXTENSIONS_AUDIO.join(", "),
            )));
        };

        let nom_stocke = format!("{}.{}", Uuid::new_v4(), ext);
        let chemin_relatif = format!("/uploads/medias/{}/{}", sous_dossier, nom_stocke);
        let chemin_complet = format!("{}/medias/{}/{}", upload_dir.get_ref(), sous_dossier, nom_stocke);

        if let Some(parent) = std::path::Path::new(&chemin_complet).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le répertoire: {}", e)))?;
        }
        let mut fichier = std::fs::File::create(&chemin_complet)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le fichier: {}", e)))?;

        let mut taille: usize = 0;
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
            taille += data.len();
            if taille > taille_max {
                drop(fichier);
                let _ = std::fs::remove_file(&chemin_complet);
                return Err(ApiErreur::Validation(format!(
                    "Fichier trop volumineux (max {} Mo)",
                    taille_max / (1024 * 1024)
                )));
            }
            fichier
                .write_all(&data)
                .map_err(|e| ApiErreur::Upload(format!("Erreur écriture fichier: {}", e)))?;
        }

        if taille == 0 {
            let _ = std::fs::remove_file(&chemin_complet);
            return Err(ApiErreur::Validation("Fichier vide".into()));
        }

        log::info!("Admin {} a uploadé un média {} ({} octets)", admin.id, chemin_relatif, taille);

        return Ok(HttpResponse::Created().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({ "url": chemin_relatif })),
            error: None,
        }));
    }

    Err(ApiErreur::Validation("Aucun fichier reçu (champ « fichier » attendu)".into()))
}

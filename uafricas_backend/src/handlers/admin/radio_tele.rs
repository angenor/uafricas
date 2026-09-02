use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::handlers::media_emission::{
    charger_emissions_du_support, contexte_emission, journaliser, refuser_si_episodes_publies,
    slug_unique,
};
use crate::handlers::media_episode::{
    basculer_a_la_une, charger_episodes, contexte_episode, journaliser_episode,
};
use crate::models::admin::radio_tele::{
    AdminStationRadioListeResponse, AdminStationRadioDetailRow, AdminStationRadioQueryParams,
    CreerStationRadioRequest, ModifierStationRadioRequest,
    AdminChaineTvListeResponse, AdminChaineTvDetailRow, AdminChaineTvQueryParams,
    CreerChaineTvRequest, ModifierChaineTvRequest,
    AdminEmissionRequest, ChangerEtatEmissionRequest,
    ADMIN_STATION_RADIO_LISTE_COLONNES, ADMIN_STATION_RADIO_DETAIL_COLONNES, STATION_RADIO_TRI_COLONNES,
    ADMIN_CHAINE_TV_LISTE_COLONNES, ADMIN_CHAINE_TV_DETAIL_COLONNES, CHAINE_TV_TRI_COLONNES,
    generer_slug,
};
use crate::models::media_emission::{
    colonne_support, colonnes_emission, valider_cadence, EmissionQueryParams, EmissionResponse,
    EmissionRow,
};
use crate::models::media_episode::{
    colonne_media, table_episode, type_media_episode, valider_etat_episode, EpisodeQueryParams,
    EpisodeRequest, ReordonnancementRequest,
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
// ENGAGEMENT : mise à la une d'un contenu média (règle `media_a_la_une`, US4)
// ══════════════════════════════════════════════════════════════

/// Crédite le créateur d'un contenu que l'équipe vient de mettre à la une.
///
/// Appelée **après** la mutation, jamais dans une transaction : `attribuer` est
/// non-bloquant et une erreur de sa part ne doit pas annuler une décision
/// éditoriale déjà prise.
///
/// La clé d'idempotence est portée par le **contenu**, pas par l'événement :
/// retirer puis reposer la mise à la une ne recrédite donc pas, comportement
/// demandé, et cohérent avec l'absence de reprise de points (pas de clawback).
///
/// Anti-auto-attribution : un administrateur qui met en avant un contenu qu'il a
/// lui-même créé ne se crédite pas. En pratique, les contenus créés depuis le
/// back-office portent `cree_par = admin.id` et ne rapportent donc rien ; seuls
/// les contenus nés d'une proposition de membre créditent leur auteur.
///
/// N.B. `chaine_tv` n'a **pas** de colonne `a_la_une` : les trois tables qui la
/// portent sont `station_radio`, `episode_tele` et `episode_radio`, à quoi
/// s'ajoute `episode_tele.a_la_une_globale` (vedette de la page Télé). Depuis
/// 09q c'est bien l'ÉPISODE que l'on met en avant, non le programme : la vedette
/// désigne une unité diffusable (FR-052).
async fn crediter_mise_a_la_une(
    pool: &PgPool,
    type_objet: &str,
    objet_id: Uuid,
    admin_id: Uuid,
) {
    // Littéraux fixes → interpolation SQL sûre (même pattern que `table_pour_type`).
    let table = match type_objet {
        "station_radio" => "media_content.station_radio",
        "episode_tele" => "media_content.episode_tele",
        "episode_radio" => "media_content.episode_radio",
        autre => {
            log::warn!("Engagement: type_objet non éligible à la mise à la une « {autre} »");
            return;
        }
    };

    let sql = format!("SELECT cree_par FROM {table} WHERE id = $1 AND deleted_at IS NULL");
    let cree_par: Option<Uuid> = match sqlx::query_scalar(&sql)
        .bind(objet_id)
        .fetch_optional(pool)
        .await
    {
        Ok(v) => v.flatten(),
        Err(e) => {
            log::error!("Engagement: échec résolution du créateur de {type_objet} {objet_id}: {e}");
            return;
        }
    };

    let Some(cree_par) = cree_par else { return };
    if cree_par == admin_id {
        return;
    }

    crate::services::engagement::attribuer(
        pool,
        cree_par,
        "media_a_la_une",
        Some(type_objet),
        Some(objet_id),
        &format!("alaune:{type_objet}:{objet_id}"),
    )
    .await;
}

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
    // Flux live et audio (fichier/lien) sont tous deux optionnels, au moins un est attendu.
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

    // Mise à la une dès la création : crédite le créateur du contenu (jamais l'admin).
    if body.a_la_une.unwrap_or(false) {
        crediter_mise_a_la_une(pool.get_ref(), "station_radio", id, admin.id).await;
    }

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

    // Seule la POSE de la mise à la une crédite ; la retirer ne reprend rien.
    if body.a_la_une == Some(true) {
        crediter_mise_a_la_une(pool.get_ref(), "station_radio", id, admin.id).await;
    }

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

    // L'équipe du support suit son porteur (FR-019) : `porteur_id` n'a pas de
    // FK, aucune cascade ne s'en charge.
    crate::handlers::media_equipe::supprimer_equipe_du_porteur_pool(
        pool.get_ref(),
        "station_radio",
        id,
    )
    .await?;

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
    // Flux live optionnel : le cœur de la télé = les programmes (cf. migration 09d).
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

    // L'équipe du support suit son porteur (FR-019) : `porteur_id` n'a pas de
    // FK, aucune cascade ne s'en charge.
    crate::handlers::media_equipe::supprimer_equipe_du_porteur_pool(
        pool.get_ref(),
        "chaine_tv",
        id,
    )
    .await?;

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
// PROGRAMMES CONTENEURS ET ÉPISODES (009)
// ══════════════════════════════════════════════════════════════
// Remplacent `/api/admin/programmes-tele` et `/api/admin/programmes-radio`,
// supprimées avec leurs tables par la migration 09q.
//
// **Asymétrie assumée** : un épisode créé par un administrateur naît `publie`,
// un épisode créé par un co-détenteur naît `en_attente`. C'est la conséquence
// directe de FR-040 : l'administrateur *est* le validateur, le faire passer par
// sa propre file n'aurait pas de sens.

/// GET /api/admin/medias/emissions
///
/// Liste paginée, filtrable par famille, support, état, cadence et recherche
/// (FR-046). Les deux familles sont interrogées séparément puis fusionnées :
/// leurs tables n'ont pas la même colonne de rattachement, et un UNION imposerait
/// de dupliquer la liste de colonnes au lieu de réutiliser les constantes.
pub async fn lister_emissions_admin(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<EmissionQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");

    let familles: Vec<&str> = match params.r#type.as_deref() {
        Some("tele") => vec!["chaine_tv"],
        Some("radio") => vec!["station_radio"],
        _ => vec!["chaine_tv", "station_radio"],
    };

    let mut toutes: Vec<EmissionResponse> = Vec::new();

    for type_support in familles {
        let colonnes = colonnes_emission(type_support).expect("type de support validé");
        let table_emission =
            crate::models::media_detention::table_contenu_pour_support(type_support)
                .expect("type de support validé");
        let table_support =
            crate::models::media_detention::table_pour_support(type_support).expect("type validé");
        let table_ep = table_episode(type_support).expect("type de support validé");
        let colonne = colonne_support(type_support).expect("type de support validé");

        let recherche = params
            .recherche
            .as_deref()
            .map(str::trim)
            .filter(|r| !r.is_empty())
            .map(|r| format!("%{}%", r));

        let rows = sqlx::query_as::<_, EmissionRow>(&format!(
            "SELECT {colonnes},
                    s.nom AS support_nom, s.slug AS support_slug,
                    cat.nom AS theme_phare_nom,
                    (SELECT COUNT(*) FROM {table_ep} ep
                      WHERE ep.emission_id = e.id AND ep.etat = 'publie' AND ep.deleted_at IS NULL)
                        AS nombre_episodes,
                    (SELECT MAX(ep.valide_at) FROM {table_ep} ep
                      WHERE ep.emission_id = e.id AND ep.etat = 'publie' AND ep.deleted_at IS NULL)
                        AS dernier_episode_at,
                    (SELECT COUNT(*) FROM {table_ep} ep
                      WHERE ep.emission_id = e.id AND ep.etat = 'en_attente' AND ep.deleted_at IS NULL)
                        AS episodes_en_attente,
                    (SELECT COUNT(*) FROM {table_ep} ep
                      WHERE ep.emission_id = e.id AND ep.etat = 'rejete' AND ep.deleted_at IS NULL)
                        AS episodes_rejetes
               FROM {table_emission} e
               JOIN {table_support} s ON s.id = e.{colonne}
               LEFT JOIN shared.categorie cat ON cat.id = e.theme_phare_id
              WHERE e.deleted_at IS NULL
                AND ($1::uuid IS NULL OR e.{colonne} = $1)
                AND ($2::text IS NULL OR e.etat = $2)
                AND ($3::text IS NULL OR e.cadence = $3)
                AND ($4::text IS NULL OR e.titre ILIKE $4 OR s.nom ILIKE $4)
              ORDER BY e.updated_at DESC, e.id"
        ))
        .bind(params.support_id)
        .bind(params.etat.as_deref())
        .bind(params.cadence.as_deref())
        .bind(recherche.as_deref())
        .fetch_all(pool.get_ref())
        .await?;

        toutes.extend(rows.iter().map(|r| r.to_response(type_support)));
    }

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(25).clamp(1, 100);
    let total = toutes.len() as i64;
    let debut = ((page - 1) * par_page).min(total) as usize;
    let fin = (debut as i64 + par_page).min(total) as usize;
    let donnees: Vec<_> = toutes.drain(debut..fin).collect();

    // Enveloppe `PaginatedResponse` du back-office (clé `data`) : `listerPagine`
    // côté frontend ne sait lire que celle-là. Une clé `donnees` propre à cette
    // route obligerait à un chargeur dédié pour aucun gain.
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(donnees, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/medias/emissions/{id}
pub async fn obtenir_emission_admin(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let emission_id = chemin.into_inner();
    let (type_support, support_id, _) = contexte_emission(pool.get_ref(), emission_id).await?;

    let emissions = charger_emissions_du_support(pool.get_ref(), &type_support, support_id).await?;
    let emission = emissions
        .into_iter()
        .find(|e| e.id == emission_id)
        .ok_or_else(|| ApiErreur::NonTrouve("Programme introuvable".into()))?;
    let episodes = charger_episodes(pool.get_ref(), &type_support, emission_id, None).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "emission": emission, "episodes": episodes })),
        error: None,
    }))
}

/// POST /api/admin/medias/emissions
pub async fn creer_emission_admin(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<AdminEmissionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");

    let type_support = body
        .type_support
        .as_deref()
        .ok_or_else(|| ApiErreur::Validation("Le type de support est requis".into()))?;
    crate::models::media_detention::valider_type_support(type_support)?;
    let support_id = body
        .support_id
        .ok_or_else(|| ApiErreur::Validation("Le support de rattachement est requis".into()))?;

    if body.titre.trim().is_empty() {
        return Err(ApiErreur::Validation(
            "Le titre du programme est obligatoire".into(),
        ));
    }
    let cadence = body.cadence.as_deref().unwrap_or("ponctuelle");
    valider_cadence(cadence)?;

    let table_emission = crate::models::media_detention::table_contenu_pour_support(type_support)
        .expect("type de support validé");
    let colonne = colonne_support(type_support).expect("type de support validé");
    let slug = slug_unique(pool.get_ref(), table_emission, &generer_slug(&body.titre)).await?;

    let emission_id: Uuid = if type_support == "station_radio" {
        sqlx::query_scalar(&format!(
            "INSERT INTO {table_emission}
                (station_id, titre, slug, description, image_couverture_url,
                 info_animateur, info_producteur, langue, categorie_radio,
                 theme_phare_id, theme_phare_autre, cadence, etat, cree_par)
             VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6, $7, COALESCE($8, 'Français'),
                     $9::media_content.categorie_radio, $10, $11, $12, 'brouillon', $13)
             RETURNING id"
        ))
        .bind(support_id)
        .bind(body.titre.trim())
        .bind(&slug)
        .bind(body.description.as_deref().map(str::trim))
        .bind(body.image_couverture_url.as_deref().map(str::trim))
        .bind(body.info_animateur.as_deref().map(str::trim))
        .bind(body.info_producteur.as_deref().map(str::trim))
        .bind(body.langue.as_deref().map(str::trim))
        .bind(body.categorie_radio.as_deref())
        .bind(body.theme_phare_id)
        .bind(body.theme_phare_autre.as_deref().map(str::trim))
        .bind(cadence)
        .bind(admin.id)
        .fetch_one(pool.get_ref())
        .await?
    }
    else {
        sqlx::query_scalar(&format!(
            "INSERT INTO {table_emission}
                ({colonne}, titre, slug, description, image_couverture_url,
                 info_animateur, info_producteur, langue,
                 theme_phare_id, theme_phare_autre, cadence, etat, cree_par)
             VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6, $7, COALESCE($8, 'Français'),
                     $9, $10, $11, 'brouillon', $12)
             RETURNING id"
        ))
        .bind(support_id)
        .bind(body.titre.trim())
        .bind(&slug)
        .bind(body.description.as_deref().map(str::trim))
        .bind(body.image_couverture_url.as_deref().map(str::trim))
        .bind(body.info_animateur.as_deref().map(str::trim))
        .bind(body.info_producteur.as_deref().map(str::trim))
        .bind(body.langue.as_deref().map(str::trim))
        .bind(body.theme_phare_id)
        .bind(body.theme_phare_autre.as_deref().map(str::trim))
        .bind(cadence)
        .bind(admin.id)
        .fetch_one(pool.get_ref())
        .await?
    };

    journaliser(
        &req,
        pool.get_ref(),
        admin.id,
        "CREATE",
        type_support,
        emission_id,
        None,
        Some(serde_json::json!({ "titre": body.titre.trim(), "cadence": cadence })),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": emission_id, "slug": slug })),
        error: None,
    }))
}

/// PUT /api/admin/medias/emissions/{id}
pub async fn modifier_emission_admin(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<AdminEmissionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let emission_id = chemin.into_inner();
    let (type_support, _, ancien) = contexte_emission(pool.get_ref(), emission_id).await?;

    if body.titre.trim().is_empty() {
        return Err(ApiErreur::Validation(
            "Le titre du programme est obligatoire".into(),
        ));
    }
    let cadence = body.cadence.as_deref().unwrap_or("ponctuelle");
    valider_cadence(cadence)?;

    let table_emission = crate::models::media_detention::table_contenu_pour_support(&type_support)
        .expect("type de support validé");

    sqlx::query(&format!(
        "UPDATE {table_emission}
            SET titre = $2, description = COALESCE($3, description),
                image_couverture_url = $4, info_animateur = $5, info_producteur = $6,
                langue = COALESCE($7, langue), theme_phare_id = $8,
                theme_phare_autre = $9, cadence = $10, updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(emission_id)
    .bind(body.titre.trim())
    .bind(body.description.as_deref().map(str::trim))
    .bind(body.image_couverture_url.as_deref().map(str::trim))
    .bind(body.info_animateur.as_deref().map(str::trim))
    .bind(body.info_producteur.as_deref().map(str::trim))
    .bind(body.langue.as_deref().map(str::trim))
    .bind(body.theme_phare_id)
    .bind(body.theme_phare_autre.as_deref().map(str::trim))
    .bind(cadence)
    .execute(pool.get_ref())
    .await?;

    journaliser(
        &req,
        pool.get_ref(),
        admin.id,
        "UPDATE",
        &type_support,
        emission_id,
        Some(ancien),
        Some(serde_json::json!({ "titre": body.titre.trim(), "cadence": cadence })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// PATCH /api/admin/medias/emissions/{id}/etat
///
/// Suspendre un programme retire ses épisodes de l'espace public **sans les
/// supprimer** (FR-011) : la lecture publique filtre sur l'état des deux
/// niveaux, aucune cascade n'est nécessaire. Le rétablissement remet
/// `nombre_signalements = 0`, sans quoi le seuil de suspension resterait franchi.
pub async fn changer_etat_emission_admin(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ChangerEtatEmissionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let emission_id = chemin.into_inner();
    let (type_support, _, ancien) = contexte_emission(pool.get_ref(), emission_id).await?;

    let etat = body.etat.trim();
    if !["brouillon", "en_attente", "publie", "suspendu", "supprime"].contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "État « {} » inconnu pour un programme",
            etat
        )));
    }

    let table_emission = crate::models::media_detention::table_contenu_pour_support(&type_support)
        .expect("type de support validé");

    sqlx::query(&format!(
        "UPDATE {table_emission}
            SET etat = $2,
                nombre_signalements = CASE WHEN $2 = 'publie' THEN 0 ELSE nombre_signalements END,
                updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(emission_id)
    .bind(etat)
    .execute(pool.get_ref())
    .await?;

    journaliser(
        &req,
        pool.get_ref(),
        admin.id,
        "UPDATE",
        &type_support,
        emission_id,
        Some(ancien),
        Some(serde_json::json!({ "etat": etat })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// DELETE /api/admin/medias/emissions/{id}, `409` si épisodes publiés (FR-010).
pub async fn supprimer_emission_admin(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "supprimer");
    let emission_id = chemin.into_inner();
    let (type_support, _, ancien) = contexte_emission(pool.get_ref(), emission_id).await?;

    refuser_si_episodes_publies(pool.get_ref(), &type_support, emission_id).await?;

    let table_emission = crate::models::media_detention::table_contenu_pour_support(&type_support)
        .expect("type de support validé");
    sqlx::query(&format!(
        "UPDATE {table_emission} SET etat = 'supprime', deleted_at = NOW(), updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(emission_id)
    .execute(pool.get_ref())
    .await?;

    // L'équipe du programme suit son porteur (FR-019) : voir la note du chemin
    // membre, `media_emission::supprimer_emission`.
    crate::handlers::media_equipe::supprimer_equipe_du_porteur_pool(
        pool.get_ref(),
        crate::models::media_equipe::type_porteur_emission(&type_support)?,
        emission_id,
    )
    .await?;

    journaliser(
        &req,
        pool.get_ref(),
        admin.id,
        "DELETE",
        &type_support,
        emission_id,
        Some(ancien),
        Some(serde_json::json!({ "etat": "supprime" })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// GET /api/admin/medias/emissions/{id}/episodes
pub async fn lister_episodes_admin(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    params: web::Query<EpisodeQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let emission_id = chemin.into_inner();
    let (type_support, _, _) = contexte_emission(pool.get_ref(), emission_id).await?;

    let episodes = charger_episodes(
        pool.get_ref(),
        &type_support,
        emission_id,
        params.etat.as_deref(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "episodes": episodes })),
        error: None,
    }))
}

/// POST /api/admin/medias/emissions/{id}/episodes
///
/// L'épisode naît **`publie`** : l'administration est l'autorité de validation.
pub async fn creer_episode_admin(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<EpisodeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let emission_id = chemin.into_inner();
    let (type_support, _, _) = contexte_emission(pool.get_ref(), emission_id).await?;

    body.valider()?;
    let media = body.media();
    // `ck_episode_*_media_publie` refuserait la ligne ; le dire ici donne un
    // message en français plutôt qu'une violation de contrainte.
    if media.is_none() {
        return Err(ApiErreur::Validation(
            "Un épisode publié doit porter son fichier ou son lien".into(),
        ));
    }

    let table_ep = table_episode(&type_support).expect("type de support validé");
    let colonne = colonne_media(&type_support).expect("type de support validé");
    let slug = slug_unique(pool.get_ref(), table_ep, &generer_slug(&body.titre)).await?;

    let episode_id: Uuid = sqlx::query_scalar(&format!(
        "INSERT INTO {table_ep}
            (emission_id, titre, slug, description, image_couverture_url, {colonne},
             numero_episode, duree_minutes, ordre, etat, valide_par, valide_at, cree_par)
         VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6, $7, $8,
                 (SELECT COALESCE(MAX(ordre), -1) + 1 FROM {table_ep}
                   WHERE emission_id = $1 AND deleted_at IS NULL),
                 'publie', $9, NOW(), $9)
         RETURNING id"
    ))
    .bind(emission_id)
    .bind(body.titre.trim())
    .bind(&slug)
    .bind(body.description.as_deref().map(str::trim))
    .bind(body.image_couverture_url.as_deref().map(str::trim))
    .bind(media.as_deref())
    .bind(body.numero_episode)
    .bind(body.duree_minutes)
    .bind(admin.id)
    .fetch_one(pool.get_ref())
    .await?;

    journaliser_episode(
        &req,
        pool.get_ref(),
        admin.id,
        "CREATE",
        &type_support,
        episode_id,
        None,
        Some(serde_json::json!({ "emission_id": emission_id, "etat": "publie" })),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": episode_id, "slug": slug, "etat": "publie" })),
        error: None,
    }))
}

/// PUT /api/admin/medias/episodes/{id}
///
/// À la différence de la route membre, l'administration **ne renvoie pas**
/// l'épisode en file : elle a autorité pour publier, la modifier ne remet donc
/// rien en attente.
pub async fn modifier_episode_admin(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<EpisodeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let episode_id = chemin.into_inner();
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;

    body.valider()?;
    let table_ep = table_episode(&ctx.type_support).expect("type de support validé");
    let colonne = colonne_media(&ctx.type_support).expect("type de support validé");
    let media = body.media();

    let etat = match body.etat.as_deref() {
        Some(e) => {
            valider_etat_episode(e)?;
            Some(e)
        }
        None => None,
    };

    sqlx::query(&format!(
        "UPDATE {table_ep}
            SET titre = $2,
                description = COALESCE($3, description),
                image_couverture_url = $4,
                {colonne} = COALESCE($5, {colonne}),
                numero_episode = $6,
                duree_minutes = $7,
                etat = COALESCE($8, etat),
                valide_par = CASE WHEN COALESCE($8, etat) IN ('publie','rejete')
                                  THEN COALESCE(valide_par, $9) ELSE valide_par END,
                valide_at  = CASE WHEN COALESCE($8, etat) IN ('publie','rejete')
                                  THEN COALESCE(valide_at, NOW()) ELSE valide_at END,
                updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(episode_id)
    .bind(body.titre.trim())
    .bind(body.description.as_deref().map(str::trim))
    .bind(body.image_couverture_url.as_deref().map(str::trim))
    .bind(media.as_deref())
    .bind(body.numero_episode)
    .bind(body.duree_minutes)
    .bind(etat)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    journaliser_episode(
        &req,
        pool.get_ref(),
        admin.id,
        "UPDATE",
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        Some(serde_json::json!({ "titre": body.titre.trim(), "etat": etat })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// DELETE /api/admin/medias/episodes/{id}, suppression douce.
pub async fn supprimer_episode_admin(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "supprimer");
    let episode_id = chemin.into_inner();
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;

    let table_ep = table_episode(&ctx.type_support).expect("type de support validé");
    sqlx::query(&format!(
        "UPDATE {table_ep} SET etat = 'supprime', a_la_une = FALSE,
                deleted_at = NOW(), updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(episode_id)
    .execute(pool.get_ref())
    .await?;

    journaliser_episode(
        &req,
        pool.get_ref(),
        admin.id,
        "DELETE",
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        Some(serde_json::json!({ "etat": "supprime" })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// PUT /api/admin/medias/emissions/{id}/episodes/reordonner
///
/// Réécriture atomique : même invariant que la route membre : la liste doit
/// couvrir exactement les épisodes du programme.
pub async fn reordonner_episodes_admin(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ReordonnancementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let emission_id = chemin.into_inner();
    let (type_support, _, _) = contexte_emission(pool.get_ref(), emission_id).await?;
    let table_ep = table_episode(&type_support).expect("type de support validé");

    let mut tx = pool.begin().await?;

    let existants: Vec<Uuid> = sqlx::query_scalar(&format!(
        "SELECT id FROM {table_ep}
          WHERE emission_id = $1 AND deleted_at IS NULL FOR UPDATE"
    ))
    .bind(emission_id)
    .fetch_all(&mut *tx)
    .await?;

    let soumis: std::collections::HashSet<Uuid> = body.ordres.iter().map(|o| o.episode_id).collect();
    let attendus: std::collections::HashSet<Uuid> = existants.iter().copied().collect();
    if soumis != attendus {
        return Err(ApiErreur::Validation(format!(
            "La liste doit couvrir exactement les {} épisode(s) du programme : {} reçu(s).",
            attendus.len(),
            soumis.len()
        )));
    }

    for ordre in &body.ordres {
        sqlx::query(&format!(
            "UPDATE {table_ep} SET ordre = $2, updated_at = NOW()
              WHERE id = $1 AND emission_id = $3 AND deleted_at IS NULL"
        ))
        .bind(ordre.episode_id)
        .bind(ordre.ordre)
        .bind(emission_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    journaliser(
        &req,
        pool.get_ref(),
        admin.id,
        "UPDATE",
        &type_support,
        emission_id,
        None,
        Some(serde_json::json!({ "reordonnancement": body.ordres.len() })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// PATCH /api/admin/medias/episodes/{id}/a-la-une
///
/// Mise en avant au sein de son SUPPORT. Bascule et désignation dans une seule
/// transaction : l'index unique partiel est violé en concurrence sinon.
pub async fn definir_a_la_une_admin(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let episode_id = chemin.into_inner();
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;

    if ctx.etat != "publie" {
        return Err(ApiErreur::Conflit(
            "Seul un épisode publié peut être mis en avant".into(),
        ));
    }

    basculer_a_la_une(pool.get_ref(), &ctx.type_support, ctx.support_id, episode_id).await?;

    let type_objet = type_media_episode(&ctx.type_support).expect("type de support validé");
    crediter_mise_a_la_une(pool.get_ref(), type_objet, episode_id, admin.id).await;

    journaliser_episode(
        &req,
        pool.get_ref(),
        admin.id,
        "UPDATE",
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        Some(serde_json::json!({ "a_la_une": true })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// PATCH /api/admin/medias/episodes/{id}/vedette-globale
///
/// Remplace `/api/admin/programmes-tele/{id}/vedette-globale`. La vedette
/// désigne une **unité diffusable** (FR-052) : elle porte donc sur un épisode.
///
/// La bascule de l'ancienne vedette et la désignation de la nouvelle tiennent
/// dans **une seule transaction** : l'index unique partiel sur l'expression
/// constante `((TRUE))` est violé dès que deux désignations se croisent.
pub async fn definir_vedette_globale(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let episode_id = chemin.into_inner();
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;

    // La vedette plein écran n'existe que sur l'espace Télé.
    if ctx.type_support != "chaine_tv" {
        return Err(ApiErreur::Validation(
            "La vedette globale ne concerne que l'espace Télé".into(),
        ));
    }
    if ctx.etat != "publie" {
        return Err(ApiErreur::Conflit(
            "Seul un épisode publié peut devenir la vedette de la page Télé".into(),
        ));
    }

    let mut tx = pool.begin().await?;

    sqlx::query(
        "UPDATE media_content.episode_tele SET a_la_une_globale = FALSE, updated_at = NOW()
          WHERE a_la_une_globale = TRUE AND deleted_at IS NULL",
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE media_content.episode_tele SET a_la_une_globale = TRUE, updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(episode_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    crediter_mise_a_la_une(pool.get_ref(), "episode_tele", episode_id, admin.id).await;

    journaliser_episode(
        &req,
        pool.get_ref(),
        admin.id,
        "UPDATE",
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        Some(serde_json::json!({ "a_la_une_globale": true })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
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

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::mooc::{
    AdminMoocListeResponse, AdminMoocDetailRow, AdminMoocQueryParams,
    AdminMoocInscriptionResponse, AdminMoocInscriptionStats,
    AdminMoocInscriptionQueryParams,
    CreerMoocRequest, ModifierMoocRequest, ChangerEtatMoocRequest,
    ADMIN_MOOC_LISTE_COLONNES, ADMIN_MOOC_DETAIL_COLONNES, MOOC_TRI_COLONNES,
    generer_slug,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

const ETATS_VALIDES: &[&str] = &["brouillon", "publie", "en_cours", "termine", "annule", "suspendu"];
const FORMATS_VALIDES: &[&str] = &["presentiel", "en_ligne", "hybride"];

/// GET /api/admin/mooc
pub async fn lister_moocs(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminMoocQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["m.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(m.titre) LIKE ${bi} OR LOWER(m.description) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("m.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref format) = params.format {
        let f = format.trim();
        if !f.is_empty() {
            conditions.push(format!("m.format::TEXT = ${}", bind_index));
            bind_values.push(f.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref type_formation) = params.type_formation {
        let t = type_formation.trim();
        if !t.is_empty() {
            conditions.push(format!("LOWER(m.type) LIKE ${}", bind_index));
            bind_values.push(format!("%{}%", t.to_lowercase()));
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(MOOC_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN shared.pays ON m.pays_id = pays.id
                 LEFT JOIN iam.utilisateur u ON m.cree_par = u.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.mooc m {} WHERE {}",
        joins, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM media_content.mooc m {} WHERE {} ORDER BY m.{} {} LIMIT {} OFFSET {}",
        ADMIN_MOOC_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminMoocListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/mooc/{id}
pub async fn obtenir_mooc(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN shared.pays ON m.pays_id = pays.id
                 LEFT JOIN iam.utilisateur u ON m.cree_par = u.id";

    let sql = format!(
        "SELECT {} FROM media_content.mooc m {} WHERE m.id = $1 AND m.deleted_at IS NULL",
        ADMIN_MOOC_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminMoocDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("MOOC non trouve".into()))?;

    let nombre_inscriptions: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc_inscription WHERE mooc_id = $1"
    ).bind(id).fetch_one(pool.get_ref()).await?;

    let progression_moyenne: f64 = sqlx::query_scalar(
        "SELECT COALESCE(AVG(progression), 0) FROM media_content.mooc_inscription WHERE mooc_id = $1"
    ).bind(id).fetch_one(pool.get_ref()).await
    .unwrap_or(0.0);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(nombre_inscriptions, progression_moyenne)),
        error: None,
    }))
}

/// POST /api/admin/mooc
pub async fn creer_mooc(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerMoocRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }

    if let Some(ref fmt) = body.format {
        if !FORMATS_VALIDES.contains(&fmt.as_str()) {
            return Err(ApiErreur::Validation(format!("Format invalide: {}", fmt)));
        }
    }

    let date_debut = chrono::DateTime::parse_from_rfc3339(&body.date_heure_debut)
        .map(|d| d.with_timezone(&chrono::Utc))
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(&body.date_heure_debut, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(&body.date_heure_debut, "%Y-%m-%d %H:%M:%S"))
                .map(|d| d.and_utc())
        })
        .map_err(|_| ApiErreur::Validation("Format de date debut invalide".into()))?;

    let date_fin = body.date_heure_fin.as_deref().and_then(|df| {
        chrono::DateTime::parse_from_rfc3339(df)
            .map(|d| d.with_timezone(&chrono::Utc))
            .or_else(|_| {
                chrono::NaiveDateTime::parse_from_str(df, "%Y-%m-%dT%H:%M:%S")
                    .or_else(|_| chrono::NaiveDateTime::parse_from_str(df, "%Y-%m-%d %H:%M:%S"))
                    .map(|d| d.and_utc())
            })
            .ok()
    });

    let id = Uuid::new_v4();
    let slug = generer_slug(titre);
    let format = body.format.as_deref().unwrap_or("en_ligne");
    let langue = body.langue.as_deref().unwrap_or("Français");

    sqlx::query(
        "INSERT INTO media_content.mooc
         (id, titre, slug, description, type, pays_id, ville,
          date_heure_debut, date_heure_fin, image_couverture_url,
          format, lien_en_ligne, langue, nombre_places, prerequis,
          objectif, presentation, a_evaluation, est_certifiante, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                 $11::media_content.format_evenement, $12, $13, $14, $15,
                 $16, $17, $18, $19, 'brouillon', $20)"
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(body.description.trim())
    .bind(body.type_formation.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(date_debut)
    .bind(date_fin)
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(format)
    .bind(body.lien_en_ligne.as_deref().map(|s| s.trim()))
    .bind(langue)
    .bind(body.nombre_places)
    .bind(body.prerequis.as_deref().map(|s| s.trim()))
    .bind(body.objectif.as_deref().map(|s| s.trim()))
    .bind(body.presentation.as_deref().map(|s| s.trim()))
    .bind(body.a_evaluation.unwrap_or(false))
    .bind(body.est_certifiante.unwrap_or(false))
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le MOOC {} ({})", admin.id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "media_content",
        "mooc",
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

/// PUT /api/admin/mooc/{id}
pub async fn modifier_mooc(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierMoocRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.mooc WHERE id = $1 AND deleted_at IS NULL)"
    ).bind(id).fetch_one(pool.get_ref()).await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("MOOC non trouve".into()));
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

    champ_str!(body.titre, "titre");
    champ_str!(body.description, "description");
    champ_str!(body.type_formation, "type");
    champ_str!(body.ville, "ville");
    champ_str!(body.image_couverture_url, "image_couverture_url");
    champ_str!(body.lien_en_ligne, "lien_en_ligne");
    champ_str!(body.langue, "langue");
    champ_str!(body.prerequis, "prerequis");
    champ_str!(body.objectif, "objectif");
    champ_str!(body.presentation, "presentation");

    if let Some(v) = body.a_evaluation {
        sets.push(format!("a_evaluation = {}", v));
    }
    if let Some(v) = body.est_certifiante {
        sets.push(format!("est_certifiante = {}", v));
    }

    if let Some(ref fmt) = body.format {
        if !FORMATS_VALIDES.contains(&fmt.as_str()) {
            return Err(ApiErreur::Validation(format!("Format invalide: {}", fmt)));
        }
        sets.push(format!("format = ${}::media_content.format_evenement", bind_index));
        bind_strings.push(fmt.clone());
        bind_index += 1;
    }

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }
    if let Some(v) = body.nombre_places {
        sets.push(format!("nombre_places = {}", v));
    }

    if let Some(ref d) = body.date_heure_debut {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(d) {
            sets.push(format!("date_heure_debut = '{}'", dt));
        } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S") {
            sets.push(format!("date_heure_debut = '{}'", dt));
        }
    }
    if let Some(ref d) = body.date_heure_fin {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(d) {
            sets.push(format!("date_heure_fin = '{}'", dt));
        } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S") {
            sets.push(format!("date_heure_fin = '{}'", dt));
        }
    }

    if body.titre.is_some() {
        let titre = body.titre.as_ref().unwrap().trim();
        let slug = generer_slug(titre);
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(slug);
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE media_content.mooc SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie le MOOC {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "mooc",
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

/// PATCH /api/admin/mooc/{id}/etat
pub async fn changer_etat_mooc(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatMoocRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide: {}. Valeurs possibles: {:?}", etat, ETATS_VALIDES
        )));
    }

    let result = sqlx::query(
        "UPDATE media_content.mooc SET etat = $1, updated_at = NOW() WHERE id = $2 AND deleted_at IS NULL"
    ).bind(etat).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("MOOC non trouve".into()));
    }

    log::info!("Admin {} a change l'etat du MOOC {} vers {}", admin.id, id, etat);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "mooc",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": etat })),
        error: None,
    }))
}

/// DELETE /api/admin/mooc/{id}
pub async fn supprimer_mooc(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.mooc SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    ).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("MOOC non trouve".into()));
    }

    log::info!("Admin {} a supprime le MOOC {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "media_content",
        "mooc",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Inscriptions MOOC ────────────────────────────────────────

/// GET /api/admin/mooc/{id}/inscriptions
pub async fn lister_inscriptions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    params: web::Query<AdminMoocInscriptionQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "voir");
    let mooc_id = path.into_inner();

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).min(100);
    let offset = (page - 1) * par_page;

    let mut conditions = vec!["mi.mooc_id = $1".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 2;

    if let Some(ref statut) = params.statut {
        let s = statut.trim();
        if !s.is_empty() {
            conditions.push(format!("mi.statut = ${}", bind_index));
            bind_values.push(s.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.mooc_inscription mi
         JOIN iam.utilisateur u ON mi.utilisateur_id = u.id WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql).bind(mooc_id);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT mi.id, mi.utilisateur_id, u.nom, u.prenom, u.email,
                mi.statut, mi.progression::FLOAT8 as progression, mi.created_at
         FROM media_content.mooc_inscription mi
         JOIN iam.utilisateur u ON mi.utilisateur_id = u.id
         WHERE {} ORDER BY mi.created_at DESC LIMIT {} OFFSET {}",
        where_clause, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminMoocInscriptionResponse>(&select_sql)
        .bind(mooc_id);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/mooc/{id}/inscriptions/stats
pub async fn stats_inscriptions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "voir");
    let mooc_id = path.into_inner();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc_inscription WHERE mooc_id = $1"
    ).bind(mooc_id).fetch_one(pool.get_ref()).await?;

    let inscrits: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc_inscription WHERE mooc_id = $1 AND statut = 'inscrit'"
    ).bind(mooc_id).fetch_one(pool.get_ref()).await?;

    let en_cours: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc_inscription WHERE mooc_id = $1 AND statut = 'en_cours'"
    ).bind(mooc_id).fetch_one(pool.get_ref()).await?;

    let completes: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc_inscription WHERE mooc_id = $1 AND statut = 'complete'"
    ).bind(mooc_id).fetch_one(pool.get_ref()).await?;

    let abandonnes: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc_inscription WHERE mooc_id = $1 AND statut = 'abandonne'"
    ).bind(mooc_id).fetch_one(pool.get_ref()).await?;

    let progression_moyenne: f64 = sqlx::query_scalar(
        "SELECT COALESCE(AVG(progression), 0)::FLOAT8 FROM media_content.mooc_inscription WHERE mooc_id = $1"
    ).bind(mooc_id).fetch_one(pool.get_ref()).await
    .unwrap_or(0.0);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminMoocInscriptionStats {
            total, inscrits, en_cours, completes, abandonnes, progression_moyenne,
        }),
        error: None,
    }))
}

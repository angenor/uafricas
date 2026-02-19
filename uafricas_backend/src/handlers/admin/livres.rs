use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::livre::{
    AdminLivreListeResponse, AdminLivreDetailRow, AdminLivreQueryParams,
    AdminLivreTagResponse, AjouterTagRequest,
    CreerLivreRequest, ModifierLivreRequest, ChangerEtatLivreRequest,
    ADMIN_LIVRE_LISTE_COLONNES, ADMIN_LIVRE_DETAIL_COLONNES, LIVRE_TRI_COLONNES,
    generer_slug,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

const ETATS_VALIDES: &[&str] = &["brouillon", "publie", "suspendu", "supprime"];
const ACCES_VALIDES: &[&str] = &["lecture_seule", "lecture_telechargement"];

/// GET /api/admin/livres
pub async fn lister_livres(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminLivreQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "livre", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["l.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;
    let mut bind_types: Vec<&str> = Vec::new();

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(l.titre) LIKE ${bi} OR LOWER(l.description) LIKE ${bi} OR LOWER(l.info_auteur) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref type_document) = params.type_document {
        let t = type_document.trim();
        if !t.is_empty() {
            conditions.push(format!("LOWER(l.type_document) = ${}", bind_index));
            bind_values.push(t.to_lowercase());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref acces) = params.acces {
        let a = acces.trim();
        if !a.is_empty() {
            conditions.push(format!("l.acces::TEXT = ${}", bind_index));
            bind_values.push(a.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("l.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(categorie_id) = params.categorie_id {
        conditions.push(format!("l.categorie_id = ${}", bind_index));
        bind_uuids.push(categorie_id);
        bind_types.push("uuid");
        bind_index += 1;
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(LIVRE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN shared.categorie cat ON l.categorie_id = cat.id
                 LEFT JOIN iam.utilisateur u ON l.cree_par = u.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.livre l {} WHERE {}",
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
        "SELECT {} FROM media_content.livre l {} WHERE {} ORDER BY l.{} {} LIMIT {} OFFSET {}",
        ADMIN_LIVRE_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminLivreListeResponse>(&select_sql);
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

/// GET /api/admin/livres/{id}
pub async fn obtenir_livre(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "livre", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN shared.categorie cat ON l.categorie_id = cat.id
                 LEFT JOIN iam.utilisateur u ON l.cree_par = u.id";

    let sql = format!(
        "SELECT {} FROM media_content.livre l {} WHERE l.id = $1 AND l.deleted_at IS NULL",
        ADMIN_LIVRE_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminLivreDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Livre non trouve".into()))?;

    let tags = sqlx::query_as::<_, AdminLivreTagResponse>(
        "SELECT lt.tag_id, t.nom AS tag_nom
         FROM media_content.livre_tag lt
         JOIN shared.tag t ON lt.tag_id = t.id
         WHERE lt.livre_id = $1"
    ).bind(id).fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(tags)),
        error: None,
    }))
}

/// POST /api/admin/livres
pub async fn creer_livre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerLivreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "livre", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }
    let document_pdf_url = body.document_pdf_url.trim();
    if document_pdf_url.is_empty() {
        return Err(ApiErreur::Validation("L'URL du document PDF est requise".into()));
    }

    if let Some(ref acces) = body.acces {
        if !ACCES_VALIDES.contains(&acces.as_str()) {
            return Err(ApiErreur::Validation(format!("Acces invalide: {}", acces)));
        }
    }

    let date_publication = body.date_publication.as_deref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let id = Uuid::new_v4();
    let slug = generer_slug(titre);
    let acces = body.acces.as_deref().unwrap_or("lecture_seule");
    let langue = body.langue.as_deref().unwrap_or("Français");

    sqlx::query(
        "INSERT INTO media_content.livre
         (id, titre, slug, description, image_couverture_url, document_pdf_url,
          type_document, categorie_id, acces, info_auteur,
          date_publication, rapport_auteur, condition_diffusion, acceptation_diffusion,
          langue, nombre_pages, isbn, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8,
                 $9::media_content.acces_livre, $10, $11, $12, $13, $14,
                 $15, $16, $17, 'brouillon', $18)"
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(body.description.trim())
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(document_pdf_url)
    .bind(body.type_document.trim())
    .bind(body.categorie_id)
    .bind(acces)
    .bind(body.info_auteur.trim())
    .bind(date_publication)
    .bind(body.rapport_auteur.as_deref().map(|s| s.trim()))
    .bind(body.condition_diffusion.as_deref().map(|s| s.trim()))
    .bind(body.acceptation_diffusion.unwrap_or(false))
    .bind(langue)
    .bind(body.nombre_pages)
    .bind(body.isbn.as_deref().map(|s| s.trim()))
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le livre {} ({})", admin.id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "media_content",
        "livre",
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

/// PUT /api/admin/livres/{id}
pub async fn modifier_livre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierLivreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "livre", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.livre WHERE id = $1 AND deleted_at IS NULL)"
    ).bind(id).fetch_one(pool.get_ref()).await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Livre non trouve".into()));
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
    champ_str!(body.image_couverture_url, "image_couverture_url");
    champ_str!(body.document_pdf_url, "document_pdf_url");
    champ_str!(body.type_document, "type_document");
    champ_str!(body.info_auteur, "info_auteur");
    champ_str!(body.rapport_auteur, "rapport_auteur");
    champ_str!(body.condition_diffusion, "condition_diffusion");
    champ_str!(body.langue, "langue");
    champ_str!(body.isbn, "isbn");

    if let Some(ref acces) = body.acces {
        if !ACCES_VALIDES.contains(&acces.as_str()) {
            return Err(ApiErreur::Validation(format!("Acces invalide: {}", acces)));
        }
        sets.push(format!("acces = ${}::media_content.acces_livre", bind_index));
        bind_strings.push(acces.clone());
        bind_index += 1;
    }

    if let Some(categorie_id) = body.categorie_id {
        sets.push(format!("categorie_id = '{}'", categorie_id));
    }
    if let Some(v) = body.acceptation_diffusion {
        sets.push(format!("acceptation_diffusion = {}", v));
    }
    if let Some(v) = body.nombre_pages {
        sets.push(format!("nombre_pages = {}", v));
    }

    if let Some(ref d) = body.date_publication {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            sets.push(format!("date_publication = '{}'", date));
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
        "UPDATE media_content.livre SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie le livre {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "livre",
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

/// PATCH /api/admin/livres/{id}/etat
pub async fn changer_etat_livre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatLivreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "livre", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide: {}. Valeurs possibles: {:?}", etat, ETATS_VALIDES
        )));
    }

    let result = sqlx::query(
        "UPDATE media_content.livre SET etat = $1, updated_at = NOW() WHERE id = $2 AND deleted_at IS NULL"
    ).bind(etat).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Livre non trouve".into()));
    }

    log::info!("Admin {} a change l'etat du livre {} vers {}", admin.id, id, etat);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "livre",
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

/// DELETE /api/admin/livres/{id}
pub async fn supprimer_livre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "livre", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.livre SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    ).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Livre non trouve".into()));
    }

    log::info!("Admin {} a supprime le livre {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "media_content",
        "livre",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Tags ─────────────────────────────────────────────────────

/// POST /api/admin/livres/{id}/tags
pub async fn ajouter_tag(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterTagRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "livre", "modifier");
    let livre_id = path.into_inner();

    let livre_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.livre WHERE id = $1 AND deleted_at IS NULL)"
    ).bind(livre_id).fetch_one(pool.get_ref()).await?;
    if !livre_existe {
        return Err(ApiErreur::NonTrouve("Livre non trouve".into()));
    }

    let tag_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM shared.tag WHERE id = $1)"
    ).bind(body.tag_id).fetch_one(pool.get_ref()).await?;
    if !tag_existe {
        return Err(ApiErreur::NonTrouve("Tag non trouve".into()));
    }

    sqlx::query(
        "INSERT INTO media_content.livre_tag (livre_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
    ).bind(livre_id).bind(body.tag_id)
    .execute(pool.get_ref()).await?;

    log::info!("Admin {} a ajoute le tag {} au livre {}", admin.id, body.tag_id, livre_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "media_content",
        "livre_tag",
        Some(livre_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "livre_id": livre_id, "tag_id": body.tag_id })),
        error: None,
    }))
}

/// DELETE /api/admin/livres/{id}/tags/{tag_id}
pub async fn retirer_tag(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "livre", "modifier");
    let (livre_id, tag_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM media_content.livre_tag WHERE livre_id = $1 AND tag_id = $2"
    ).bind(livre_id).bind(tag_id)
    .execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Association livre-tag non trouvee".into()));
    }

    log::info!("Admin {} a retire le tag {} du livre {}", admin.id, tag_id, livre_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "media_content",
        "livre_tag",
        Some(livre_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

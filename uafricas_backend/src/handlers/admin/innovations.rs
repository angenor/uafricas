use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::innovation::{
    AdminInnovationDetailResponse, AdminInnovationListeResponse, AdminInnovationMedia,
    AdminInnovationQueryParams, AjouterMediaInnovationRequest, ChangerEtatInnovationRequest,
    CreerInnovationRequest, ModifierInnovationRequest, ADMIN_INNOVATION_DETAIL_COLONNES,
    ADMIN_INNOVATION_LISTE_COLONNES, INNOVATION_JOINS, INNOVATION_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

// ── Etats valides pour innovation (etat_contenu) ────────────
const ETATS_VALIDES: &[&str] = &["brouillon", "publie", "suspendu", "supprime"];

/// GET /api/admin/innovations
pub async fn lister_innovations(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminInnovationQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["i.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(i.titre) LIKE ${bi} OR LOWER(COALESCE(i.description, '')) LIKE ${bi} OR LOWER(COALESCE(i.ville, '')) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("i.etat::text = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref domaine_id) = params.domaine_id {
        let d = domaine_id.trim();
        if !d.is_empty() {
            if Uuid::parse_str(d).is_ok() {
                conditions.push(format!("i.domaine_id::text = ${}", bind_index));
                bind_values.push(d.to_string());
                bind_index += 1;
            }
        }
    }

    if let Some(ref pays_id) = params.pays_id {
        let p = pays_id.trim();
        if !p.is_empty() {
            if Uuid::parse_str(p).is_ok() {
                conditions.push(format!("i.pays_id::text = ${}", bind_index));
                bind_values.push(p.to_string());
                bind_index += 1;
            }
        }
    }

    if let Some(ref organisation_id) = params.organisation_id {
        let o = organisation_id.trim();
        if !o.is_empty() {
            if Uuid::parse_str(o).is_ok() {
                conditions.push(format!("i.organisation_id::text = ${}", bind_index));
                bind_values.push(o.to_string());
                bind_index += 1;
            }
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(INNOVATION_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    // COUNT
    let count_sql = format!(
        "SELECT COUNT(*) FROM innovation.innovation i {} WHERE {}",
        INNOVATION_JOINS, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT
    let select_sql = format!(
        "SELECT {} FROM innovation.innovation i {} WHERE {} ORDER BY i.{} {} LIMIT {} OFFSET {}",
        ADMIN_INNOVATION_LISTE_COLONNES, INNOVATION_JOINS, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminInnovationListeResponse>(&select_sql);
    for v in &bind_values {
        select_q = select_q.bind(v);
    }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/innovations/:id
pub async fn obtenir_innovation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM innovation.innovation i {} WHERE i.id = $1 AND i.deleted_at IS NULL",
        ADMIN_INNOVATION_DETAIL_COLONNES, INNOVATION_JOINS
    );
    let row = sqlx::query_as::<_, AdminInnovationDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Innovation non trouvee".into()))?;

    // Charger les medias
    let medias = sqlx::query_as::<_, AdminInnovationMedia>(
        "SELECT id, media_url, type_mime, ordre, created_at
         FROM innovation.innovation_media
         WHERE innovation_id = $1
         ORDER BY ordre ASC, created_at ASC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": row.id,
            "titre": row.titre,
            "slug": row.slug,
            "description": row.description,
            "etat": row.etat,
            "image_couverture_url": row.image_couverture_url,
            "ville": row.ville,
            "nombre_vues": row.nombre_vues,
            "domaine_id": row.domaine_id,
            "organisation_id": row.organisation_id,
            "pays_id": row.pays_id,
            "cree_par": row.cree_par,
            "created_at": row.created_at,
            "updated_at": row.updated_at,
            "domaine_nom": row.domaine_nom,
            "organisation_nom": row.organisation_nom,
            "pays_nom": row.pays_nom,
            "auteur_nom": row.auteur_nom,
            "auteur_prenom": row.auteur_prenom,
            "auteur_email": row.auteur_email,
            "medias": medias,
        })),
        error: None,
    }))
}

/// POST /api/admin/innovations
pub async fn creer_innovation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<CreerInnovationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }

    let etat = body.etat.as_deref().unwrap_or("brouillon").trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide. Valeurs acceptees: {}",
            ETATS_VALIDES.join(", ")
        )));
    }

    let slug = titre
        .to_lowercase()
        .replace(' ', "-")
        .replace(['\'', '"', '.', ','], "");
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO innovation.innovation
         (id, titre, slug, description, image_couverture_url, domaine_id, organisation_id, pays_id, ville, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::innovation.etat_contenu, $11)",
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.domaine_id)
    .bind(body.organisation_id)
    .bind(body.pays_id)
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(etat)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree l'innovation {} ({})", admin.id, titre, id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/innovations/:id
pub async fn modifier_innovation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierInnovationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM innovation.innovation WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Innovation non trouvee".into()));
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! ajouter_champ_str {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    ajouter_champ_str!(body.titre, "titre");
    ajouter_champ_str!(body.description, "description");
    ajouter_champ_str!(body.image_couverture_url, "image_couverture_url");
    ajouter_champ_str!(body.ville, "ville");

    // Slug auto-genere si titre change
    if let Some(ref titre) = body.titre {
        let slug = titre
            .trim()
            .to_lowercase()
            .replace(' ', "-")
            .replace(['\'', '"', '.', ','], "");
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(slug);
        bind_index += 1;
    }

    // UUIDs
    if let Some(domaine_id) = body.domaine_id {
        sets.push(format!("domaine_id = ${}", bind_index));
        bind_uuids.push(domaine_id);
        bind_index += 1;
    }
    if let Some(organisation_id) = body.organisation_id {
        sets.push(format!("organisation_id = ${}", bind_index));
        bind_uuids.push(organisation_id);
        bind_index += 1;
    }
    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE innovation.innovation SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "),
        bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
    for v in &bind_uuids {
        q = q.bind(v);
    }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie l'innovation {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PATCH /api/admin/innovations/:id/etat
pub async fn changer_etat_innovation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatInnovationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide. Valeurs acceptees: {}",
            ETATS_VALIDES.join(", ")
        )));
    }

    let result = sqlx::query(
        "UPDATE innovation.innovation SET etat = $1::innovation.etat_contenu, updated_at = NOW()
         WHERE id = $2 AND deleted_at IS NULL",
    )
    .bind(etat)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Innovation non trouvee".into()));
    }

    log::info!(
        "Admin {} a change l'etat de l'innovation {} en '{}'",
        admin.id,
        id,
        etat
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": etat })),
        error: None,
    }))
}

/// DELETE /api/admin/innovations/:id
pub async fn supprimer_innovation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE innovation.innovation SET deleted_at = NOW(), etat = 'supprime'::innovation.etat_contenu, updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Innovation non trouvee".into()));
    }

    log::info!("Admin {} a supprime l'innovation {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// POST /api/admin/innovations/:id/medias
pub async fn ajouter_media_innovation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterMediaInnovationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");
    let innovation_id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM innovation.innovation WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(innovation_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Innovation non trouvee".into()));
    }

    let media_url = body.media_url.trim();
    if media_url.is_empty() {
        return Err(ApiErreur::Validation("L'URL du media est requise".into()));
    }

    let media_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO innovation.innovation_media (id, innovation_id, media_url, type_mime, ordre)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(media_id)
    .bind(innovation_id)
    .bind(media_url)
    .bind(body.type_mime.as_deref())
    .bind(body.ordre.unwrap_or(0))
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a ajoute un media {} a l'innovation {}",
        admin.id,
        media_id,
        innovation_id
    );

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": media_id })),
        error: None,
    }))
}

/// DELETE /api/admin/innovations/:id/medias/:media_id
pub async fn retirer_media_innovation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");
    let (innovation_id, media_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM innovation.innovation_media WHERE id = $1 AND innovation_id = $2",
    )
    .bind(media_id)
    .bind(innovation_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Media non trouve".into()));
    }

    log::info!(
        "Admin {} a retire le media {} de l'innovation {}",
        admin.id,
        media_id,
        innovation_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

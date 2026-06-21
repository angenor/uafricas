use actix_web::{web, HttpRequest, HttpResponse};
use chrono::DateTime;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::programmation::{
    AdminProgDetailRow, AdminProgInscriptionResponse, AdminProgListeResponse, AdminProgQueryParams,
    CreerProgRequest, ModifierProgRequest,
    ADMIN_PROG_DETAIL_COLONNES, ADMIN_PROG_LISTE_COLONNES, PROG_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/programmations
pub async fn lister_programmations(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminProgQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["1=1".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(pc.titre) LIKE ${bi} OR LOWER(pc.lieu) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(centre_id) = params.centre_culturel_id {
        conditions.push(format!("pc.centre_culturel_id = ${}", bind_index));
        bind_uuids.push(centre_id);
        bind_index += 1;
    }

    if let Some(ref mode) = params.mode {
        let m = mode.trim();
        if !m.is_empty() {
            conditions.push(format!("pc.mode::text = ${}", bind_index));
            bind_values.push(m.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(PROG_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM culture.programmation_centre pc
         LEFT JOIN culture.centre_culturel cc ON cc.id = pc.centre_culturel_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    for v in &bind_uuids { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM culture.programmation_centre pc
         LEFT JOIN culture.centre_culturel cc ON cc.id = pc.centre_culturel_id
         WHERE {} ORDER BY pc.{} {} LIMIT {} OFFSET {}",
        ADMIN_PROG_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminProgListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    for v in &bind_uuids { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/programmations/:id
pub async fn obtenir_programmation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM culture.programmation_centre pc
         LEFT JOIN culture.centre_culturel cc ON cc.id = pc.centre_culturel_id
         LEFT JOIN iam.utilisateur u ON u.id = pc.cree_par
         WHERE pc.id = $1",
        ADMIN_PROG_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminProgDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Programmation non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/admin/programmations
pub async fn creer_programmation(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerProgRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre de la programmation est requis".into()));
    }

    let date_debut = DateTime::parse_from_rfc3339(&body.date_heure_debut)
        .map_err(|_| ApiErreur::Validation("Format de date invalide pour date_heure_debut (RFC 3339)".into()))?;

    let date_fin = if let Some(ref df) = body.date_heure_fin {
        Some(DateTime::parse_from_rfc3339(df)
            .map_err(|_| ApiErreur::Validation("Format de date invalide pour date_heure_fin (RFC 3339)".into()))?)
    } else {
        None
    };

    let mode = body.mode.as_deref().unwrap_or("presentiel");
    let modes_valides = ["en_ligne", "presentiel", "hybride"];
    if !modes_valides.contains(&mode) {
        return Err(ApiErreur::Validation(format!(
            "Mode invalide. Valeurs acceptees: {}", modes_valides.join(", ")
        )));
    }

    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO culture.programmation_centre
         (id, centre_culturel_id, titre, description, image_couverture_url, lieu, mode, lien_en_ligne,
          date_heure_debut, date_heure_fin, nombre_places, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7::culture.mode_evenement, $8, $9, $10, $11, $12)"
    )
    .bind(id)
    .bind(body.centre_culturel_id)
    .bind(titre)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.lieu.as_deref().map(|s| s.trim()))
    .bind(mode)
    .bind(body.lien_en_ligne.as_deref().map(|s| s.trim()))
    .bind(date_debut.with_timezone(&chrono::Utc))
    .bind(date_fin.map(|d| d.with_timezone(&chrono::Utc)))
    .bind(body.nombre_places)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree la programmation {} ({})", admin.id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "culture",
        "programmation",
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

/// PUT /api/admin/programmations/:id
pub async fn modifier_programmation(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierProgRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM culture.programmation_centre WHERE id = $1)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Programmation non trouvee".into()));
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
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
    ajouter_champ_str!(body.lieu, "lieu");
    ajouter_champ_str!(body.lien_en_ligne, "lien_en_ligne");

    if let Some(ref mode) = body.mode {
        let modes_valides = ["en_ligne", "presentiel", "hybride"];
        if !modes_valides.contains(&mode.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Mode invalide. Valeurs acceptees: {}", modes_valides.join(", ")
            )));
        }
        sets.push(format!("mode = ${}::culture.mode_evenement", bind_index));
        bind_strings.push(mode.clone());
        bind_index += 1;
    }

    if let Some(ref date_str) = body.date_heure_debut {
        let dt = DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| ApiErreur::Validation("Format de date invalide pour date_heure_debut".into()))?;
        sets.push(format!("date_heure_debut = ${}::timestamptz", bind_index));
        bind_strings.push(dt.with_timezone(&chrono::Utc).to_rfc3339());
        bind_index += 1;
    }

    if let Some(ref date_str) = body.date_heure_fin {
        let dt = DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| ApiErreur::Validation("Format de date invalide pour date_heure_fin".into()))?;
        sets.push(format!("date_heure_fin = ${}::timestamptz", bind_index));
        bind_strings.push(dt.with_timezone(&chrono::Utc).to_rfc3339());
        bind_index += 1;
    }

    if let Some(centre_id) = body.centre_culturel_id {
        sets.push(format!("centre_culturel_id = '{}'", centre_id));
    }

    if let Some(places) = body.nombre_places {
        sets.push(format!("nombre_places = {}", places));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE culture.programmation_centre SET {} WHERE id = ${}",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie la programmation {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "culture",
        "programmation",
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

/// DELETE /api/admin/programmations/:id
pub async fn supprimer_programmation(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM culture.programmation_centre WHERE id = $1"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Programmation non trouvee".into()));
    }

    log::info!("Admin {} a supprime la programmation {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "culture",
        "programmation",
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

/// GET /api/admin/programmations/:id/inscriptions
pub async fn lister_inscriptions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "voir");
    let id = path.into_inner();

    let inscriptions = sqlx::query_as::<_, AdminProgInscriptionResponse>(
        "SELECT pi.id, pi.utilisateur_id,
                COALESCE(pi.nom, u.nom) AS nom,
                COALESCE(pi.prenom, u.prenom) AS prenom,
                u.email, u.telephone,
                pi.pays, pi.lieu_residence, pi.titre,
                pi.statut, pi.created_at
         FROM culture.programmation_inscription pi
         JOIN iam.utilisateur u ON u.id = pi.utilisateur_id
         WHERE pi.programmation_id = $1 AND pi.statut != 'annule'
         ORDER BY pi.created_at DESC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(inscriptions),
        error: None,
    }))
}

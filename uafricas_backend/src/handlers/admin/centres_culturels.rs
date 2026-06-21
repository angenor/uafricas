use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::centre_culturel::{
    AdminCentreDetailRow, AdminCentreListeResponse, AdminCentreQueryParams,
    AdminMembreCentreResponse, AjouterMembreRequest, CreerCentreRequest,
    ModifierCentreRequest, ModifierMembreRequest,
    ADMIN_CENTRE_DETAIL_COLONNES, ADMIN_CENTRE_LISTE_COLONNES, CENTRE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/centres-culturels
pub async fn lister_centres(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminCentreQueryParams>,
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
                "(LOWER(cc.nom) LIKE ${bi} OR LOWER(cc.ville) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("cc.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_index += 1;
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(CENTRE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM culture.centre_culturel cc
         LEFT JOIN shared.pays p ON p.id = cc.pays_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    for v in &bind_uuids { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM culture.centre_culturel cc
         LEFT JOIN shared.pays p ON p.id = cc.pays_id
         WHERE {} ORDER BY cc.{} {} LIMIT {} OFFSET {}",
        ADMIN_CENTRE_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminCentreListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    for v in &bind_uuids { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/centres-culturels/:id
pub async fn obtenir_centre(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM culture.centre_culturel cc
         LEFT JOIN shared.pays p ON p.id = cc.pays_id
         LEFT JOIN iam.utilisateur u ON u.id = cc.cree_par
         WHERE cc.id = $1",
        ADMIN_CENTRE_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminCentreDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Centre culturel non trouve".into()))?;

    let membres = sqlx::query_as::<_, AdminMembreCentreResponse>(
        "SELECT mc.id, mc.utilisateur_id,
                u.nom AS utilisateur_nom, u.prenom AS utilisateur_prenom,
                mc.role::text AS role, mc.created_at
         FROM culture.membre_centre mc
         JOIN iam.utilisateur u ON u.id = mc.utilisateur_id
         WHERE mc.centre_culturel_id = $1
         ORDER BY mc.created_at"
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(membres)),
        error: None,
    }))
}

/// POST /api/admin/centres-culturels
pub async fn creer_centre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerCentreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom du centre culturel est requis".into()));
    }

    let type_centre = valider_type_centre(body.type_centre.as_deref())?;

    let id = Uuid::new_v4();
    let slug = generer_slug(nom);

    sqlx::query(
        "INSERT INTO culture.centre_culturel
         (id, nom, slug, type_centre, description, image_couverture_url, pays_id, ville, adresse, longitude, latitude, cree_par)
         VALUES ($1, $2, $3, $4::culture.type_centre_culturel, $5, $6, $7, $8, $9, $10, $11, $12)"
    )
    .bind(id)
    .bind(nom)
    .bind(&slug)
    .bind(type_centre)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(body.adresse.as_deref().map(|s| s.trim()))
    .bind(body.longitude)
    .bind(body.latitude)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le centre culturel {} ({})", admin.id, nom, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "culture",
        "centre_culturel",
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

/// PUT /api/admin/centres-culturels/:id
pub async fn modifier_centre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierCentreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM culture.centre_culturel WHERE id = $1)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Centre culturel non trouve".into()));
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

    ajouter_champ_str!(body.nom, "nom");

    if body.type_centre.is_some() {
        let type_centre = valider_type_centre(body.type_centre.as_deref())?;
        sets.push(format!("type_centre = ${}::culture.type_centre_culturel", bind_index));
        bind_strings.push(type_centre.to_string());
        bind_index += 1;
    }

    ajouter_champ_str!(body.description, "description");
    ajouter_champ_str!(body.image_couverture_url, "image_couverture_url");
    ajouter_champ_str!(body.ville, "ville");
    ajouter_champ_str!(body.adresse, "adresse");

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }
    if let Some(lon) = body.longitude {
        sets.push(format!("longitude = {}", lon));
    }
    if let Some(lat) = body.latitude {
        sets.push(format!("latitude = {}", lat));
    }
    if let Some(actif) = body.actif {
        sets.push(format!("actif = {}", actif));
    }

    // Regenerer slug si nom change
    if body.nom.is_some() {
        if let Some(ref nom) = body.nom {
            let slug = generer_slug(nom.trim());
            sets.push(format!("slug = ${}", bind_index));
            bind_strings.push(slug);
            bind_index += 1;
        }
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE culture.centre_culturel SET {} WHERE id = ${}",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie le centre culturel {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "culture",
        "centre_culturel",
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

/// DELETE /api/admin/centres-culturels/:id (soft delete via actif=false)
pub async fn supprimer_centre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE culture.centre_culturel SET actif = false, updated_at = NOW() WHERE id = $1 AND actif = true"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Centre culturel non trouve".into()));
    }

    log::info!("Admin {} a supprime le centre culturel {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "culture",
        "centre_culturel",
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

/// GET /api/admin/centres-culturels/:id/membres
pub async fn lister_membres(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "voir");
    let centre_id = path.into_inner();

    let membres = sqlx::query_as::<_, AdminMembreCentreResponse>(
        "SELECT mc.id, mc.utilisateur_id,
                u.nom AS utilisateur_nom, u.prenom AS utilisateur_prenom,
                mc.role::text AS role, mc.created_at
         FROM culture.membre_centre mc
         JOIN iam.utilisateur u ON u.id = mc.utilisateur_id
         WHERE mc.centre_culturel_id = $1
         ORDER BY mc.created_at"
    )
    .bind(centre_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(membres),
        error: None,
    }))
}

/// POST /api/admin/centres-culturels/:id/membres
pub async fn ajouter_membre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterMembreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");
    let centre_id = path.into_inner();

    let roles_valides = ["president", "vice_president", "resp_communication", "membre"];
    if !roles_valides.contains(&body.role.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Role invalide. Valeurs acceptees: {}",
            roles_valides.join(", ")
        )));
    }

    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO culture.membre_centre (id, centre_culturel_id, utilisateur_id, role)
         VALUES ($1, $2, $3, $4::culture.role_membre_centre)"
    )
    .bind(id)
    .bind(centre_id)
    .bind(body.utilisateur_id)
    .bind(&body.role)
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
            ApiErreur::Conflit("Cet utilisateur est deja membre de ce centre".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    log::info!("Admin {} a ajoute le membre {} au centre {}", admin.id, body.utilisateur_id, centre_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "culture",
        "membre_centre_culturel",
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

/// PUT /api/admin/centres-culturels/:centre_id/membres/:membre_id
pub async fn modifier_membre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ModifierMembreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");
    let (centre_id, membre_id) = path.into_inner();

    let roles_valides = ["president", "vice_president", "resp_communication", "membre"];
    if !roles_valides.contains(&body.role.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Role invalide. Valeurs acceptees: {}",
            roles_valides.join(", ")
        )));
    }

    let result = sqlx::query(
        "UPDATE culture.membre_centre SET role = $1::culture.role_membre_centre
         WHERE id = $2 AND centre_culturel_id = $3"
    )
    .bind(&body.role)
    .bind(membre_id)
    .bind(centre_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Membre non trouve".into()));
    }

    log::info!("Admin {} a modifie le role du membre {} dans le centre {}", admin.id, membre_id, centre_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "culture",
        "membre_centre_culturel",
        Some(membre_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": membre_id })),
        error: None,
    }))
}

/// DELETE /api/admin/centres-culturels/:centre_id/membres/:membre_id
pub async fn retirer_membre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");
    let (centre_id, membre_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM culture.membre_centre WHERE id = $1 AND centre_culturel_id = $2"
    )
    .bind(membre_id)
    .bind(centre_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Membre non trouve".into()));
    }

    log::info!("Admin {} a retire le membre {} du centre {}", admin.id, membre_id, centre_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "culture",
        "membre_centre_culturel",
        Some(membre_id),
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

/// Valider le type de centre (international/local). Defaut: "local".
fn valider_type_centre(valeur: Option<&str>) -> Result<&'static str, ApiErreur> {
    match valeur.map(|s| s.trim()) {
        None | Some("") | Some("local") => Ok("local"),
        Some("international") => Ok("international"),
        Some(_) => Err(ApiErreur::Validation(
            "Type de centre invalide. Valeurs acceptees: international, local".into(),
        )),
    }
}

/// Generer un slug URL-safe a partir du titre
fn generer_slug(titre: &str) -> String {
    titre
        .to_lowercase()
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â', 'ä'], "a")
        .replace(['ù', 'û', 'ü'], "u")
        .replace(['î', 'ï'], "i")
        .replace(['ô', 'ö'], "o")
        .replace('ç', "c")
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

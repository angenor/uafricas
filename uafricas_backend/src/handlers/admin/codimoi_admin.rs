use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::codimoi_admin::{
    AdminCodimoiCommentaireResponse, AdminCodimoiCommentaireRow,
    AdminCodimoiDetailRow, AdminCodimoiListeResponse, AdminCodimoiQueryParams,
    AdminCodimoiReactionStats, AdminCodimoiTagResponse,
    AjouterTagRequest, CreerCodimoiRequest, ModifierCodimoiRequest,
    ADMIN_CODIMOI_DETAIL_COLONNES, ADMIN_CODIMOI_LISTE_COLONNES, CODIMOI_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/codimoi
pub async fn lister_codimoi(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminCodimoiQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "voir");

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

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(c.search_vector @@ plainto_tsquery('french', ${bi}) OR LOWER(c.contenu) LIKE ${next})",
                bi = bind_index, next = bind_index + 1
            ));
            bind_values.push(r.to_string());
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 2;
        }
    }

    if let Some(ref type_codimoi) = params.type_codimoi {
        let t = type_codimoi.trim();
        if !t.is_empty() {
            conditions.push(format!("c.type::text = ${}", bind_index));
            bind_values.push(t.to_string());
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("c.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_index += 1;
    }

    if let Some(ref ethnie) = params.groupe_ethnique {
        let e = ethnie.trim();
        if !e.is_empty() {
            conditions.push(format!("LOWER(c.groupe_ethnique) LIKE ${}", bind_index));
            bind_values.push(format!("%{}%", e.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("c.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(CODIMOI_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM culture.codimoi c
         LEFT JOIN shared.pays p ON p.id = c.pays_id
         LEFT JOIN iam.utilisateur u ON u.id = c.cree_par
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    for v in &bind_uuids { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM culture.codimoi c
         LEFT JOIN shared.pays p ON p.id = c.pays_id
         LEFT JOIN iam.utilisateur u ON u.id = c.cree_par
         WHERE {} ORDER BY c.{} {} LIMIT {} OFFSET {}",
        ADMIN_CODIMOI_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminCodimoiListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    for v in &bind_uuids { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/codimoi/:id
pub async fn obtenir_codimoi(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM culture.codimoi c
         LEFT JOIN shared.pays p ON p.id = c.pays_id
         LEFT JOIN iam.utilisateur u ON u.id = c.cree_par
         WHERE c.id = $1 AND c.deleted_at IS NULL",
        ADMIN_CODIMOI_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminCodimoiDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Post Codi-Moi non trouve".into()))?;

    let tags = sqlx::query_as::<_, AdminCodimoiTagResponse>(
        "SELECT ct.tag_id, t.nom AS tag_nom
         FROM culture.codimoi_tag ct
         JOIN shared.tag t ON t.id = ct.tag_id
         WHERE ct.codimoi_id = $1
         ORDER BY t.nom"
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let nombre_commentaires: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM culture.codimoi_commentaire
         WHERE codimoi_id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(tags, nombre_commentaires)),
        error: None,
    }))
}

/// POST /api/admin/codimoi
pub async fn creer_codimoi(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerCodimoiRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");

    let contenu = body.contenu.trim();
    if contenu.is_empty() {
        return Err(ApiErreur::Validation("Le contenu est requis".into()));
    }

    let types_valides = ["proverbe_adage", "citation", "ressource_historique", "bonne_pratique"];
    if !types_valides.contains(&body.type_codimoi.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Type invalide. Valeurs acceptees: {}", types_valides.join(", ")
        )));
    }

    let etat = body.etat.as_deref().unwrap_or("publie");

    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO culture.codimoi
         (id, type, contenu, explication, nom_auteur_originel, pays_id, groupe_ethnique,
          couleur_fond, image_couverture_url, image_arriere_plan_url, etat, cree_par)
         VALUES ($1, $2::culture.type_codimoi, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"
    )
    .bind(id)
    .bind(&body.type_codimoi)
    .bind(contenu)
    .bind(body.explication.as_deref().map(|s| s.trim()))
    .bind(body.nom_auteur_originel.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.groupe_ethnique.as_deref().map(|s| s.trim()))
    .bind(body.couleur_fond.as_deref().map(|s| s.trim()))
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.image_arriere_plan_url.as_deref().map(|s| s.trim()))
    .bind(etat)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    // Mettre a jour le vecteur de recherche
    sqlx::query(
        "UPDATE culture.codimoi SET search_vector =
         to_tsvector('french', COALESCE(contenu, '') || ' ' || COALESCE(explication, '') || ' ' || COALESCE(nom_auteur_originel, ''))
         WHERE id = $1"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le post Codi-Moi {} ({})", admin.id, body.type_codimoi, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "culture",
        "codimoi",
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

/// PUT /api/admin/codimoi/:id
pub async fn modifier_codimoi(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierCodimoiRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM culture.codimoi WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Post Codi-Moi non trouve".into()));
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

    if let Some(ref type_codimoi) = body.type_codimoi {
        let types_valides = ["proverbe_adage", "citation", "ressource_historique", "bonne_pratique"];
        if !types_valides.contains(&type_codimoi.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Type invalide. Valeurs acceptees: {}", types_valides.join(", ")
            )));
        }
        sets.push(format!("type = ${}::culture.type_codimoi", bind_index));
        bind_strings.push(type_codimoi.clone());
        bind_index += 1;
    }

    ajouter_champ_str!(body.contenu, "contenu");
    ajouter_champ_str!(body.explication, "explication");
    ajouter_champ_str!(body.nom_auteur_originel, "nom_auteur_originel");
    ajouter_champ_str!(body.groupe_ethnique, "groupe_ethnique");
    ajouter_champ_str!(body.couleur_fond, "couleur_fond");
    ajouter_champ_str!(body.image_couverture_url, "image_couverture_url");
    ajouter_champ_str!(body.image_arriere_plan_url, "image_arriere_plan_url");
    ajouter_champ_str!(body.etat, "etat");

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE culture.codimoi SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    // Mettre a jour le vecteur de recherche si contenu ou explication modifie
    if body.contenu.is_some() || body.explication.is_some() || body.nom_auteur_originel.is_some() {
        sqlx::query(
            "UPDATE culture.codimoi SET search_vector =
             to_tsvector('french', COALESCE(contenu, '') || ' ' || COALESCE(explication, '') || ' ' || COALESCE(nom_auteur_originel, ''))
             WHERE id = $1"
        )
        .bind(id)
        .execute(pool.get_ref())
        .await?;
    }

    log::info!("Admin {} a modifie le post Codi-Moi {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "culture",
        "codimoi",
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

/// DELETE /api/admin/codimoi/:id (soft delete)
pub async fn supprimer_codimoi(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE culture.codimoi SET deleted_at = NOW(), etat = 'supprime', updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Post Codi-Moi non trouve".into()));
    }

    log::info!("Admin {} a supprime le post Codi-Moi {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "culture",
        "codimoi",
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

/// POST /api/admin/codimoi/:id/tags
pub async fn ajouter_tag(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterTagRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");
    let codimoi_id = path.into_inner();

    sqlx::query(
        "INSERT INTO culture.codimoi_tag (codimoi_id, tag_id) VALUES ($1, $2)"
    )
    .bind(codimoi_id)
    .bind(body.tag_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
            ApiErreur::Conflit("Ce tag est deja associe a ce post".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    log::info!("Admin {} a ajoute le tag {} au post {}", admin.id, body.tag_id, codimoi_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "culture",
        "codimoi_tag",
        Some(codimoi_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// DELETE /api/admin/codimoi/:codimoi_id/tags/:tag_id
pub async fn retirer_tag(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "modifier");
    let (codimoi_id, tag_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM culture.codimoi_tag WHERE codimoi_id = $1 AND tag_id = $2"
    )
    .bind(codimoi_id)
    .bind(tag_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Tag non trouve pour ce post".into()));
    }

    log::info!("Admin {} a retire le tag {} du post {}", admin.id, tag_id, codimoi_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "culture",
        "codimoi_tag",
        Some(codimoi_id),
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

/// GET /api/admin/codimoi/:id/commentaires
pub async fn lister_commentaires(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "voir");
    let codimoi_id = path.into_inner();

    let rows = sqlx::query_as::<_, AdminCodimoiCommentaireRow>(
        "SELECT cc.id, cc.parent_id, cc.contenu, cc.cree_par,
                u.nom AS auteur_nom, u.prenom AS auteur_prenom,
                cc.nombre_likes, cc.created_at, cc.deleted_at
         FROM culture.codimoi_commentaire cc
         JOIN iam.utilisateur u ON u.id = cc.cree_par
         WHERE cc.codimoi_id = $1
         ORDER BY cc.created_at"
    )
    .bind(codimoi_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Construire l'arborescence
    let commentaires = construire_arborescence(&rows);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(commentaires),
        error: None,
    }))
}

/// DELETE /api/admin/codimoi/:codimoi_id/commentaires/:commentaire_id
pub async fn supprimer_commentaire(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "supprimer");
    let (codimoi_id, commentaire_id) = path.into_inner();

    let result = sqlx::query(
        "UPDATE culture.codimoi_commentaire SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND codimoi_id = $2 AND deleted_at IS NULL"
    )
    .bind(commentaire_id)
    .bind(codimoi_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Commentaire non trouve".into()));
    }

    log::info!("Admin {} a supprime le commentaire {} du post {}", admin.id, commentaire_id, codimoi_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "culture",
        "codimoi_commentaire",
        Some(commentaire_id),
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

/// GET /api/admin/codimoi/:id/reactions
pub async fn obtenir_reactions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "culture", "voir");
    let codimoi_id = path.into_inner();

    let row = sqlx::query_as::<_, (i32, i32)>(
        "SELECT nombre_likes, nombre_dislikes FROM culture.codimoi WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(codimoi_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Post Codi-Moi non trouve".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminCodimoiReactionStats {
            nombre_likes: row.0,
            nombre_dislikes: row.1,
        }),
        error: None,
    }))
}

// ── Helper: construire l'arborescence de commentaires ─────
fn construire_arborescence(rows: &[AdminCodimoiCommentaireRow]) -> Vec<AdminCodimoiCommentaireResponse> {
    use std::collections::HashMap;

    // Convertir tous les rows en responses sans enfants
    let mut map: HashMap<Uuid, AdminCodimoiCommentaireResponse> = HashMap::new();
    let mut order: Vec<Uuid> = Vec::new();

    for row in rows {
        let resp = AdminCodimoiCommentaireResponse {
            id: row.id,
            parent_id: row.parent_id,
            contenu: if row.deleted_at.is_some() { "[Commentaire supprime]".to_string() } else { row.contenu.clone() },
            cree_par: row.cree_par,
            auteur_nom: format!("{} {}", row.auteur_prenom, row.auteur_nom),
            nombre_likes: row.nombre_likes,
            created_at: row.created_at,
            supprime: row.deleted_at.is_some(),
            enfants: Vec::new(),
        };
        order.push(row.id);
        map.insert(row.id, resp);
    }

    // Construire l'arborescence en regroupant les enfants
    let mut roots: Vec<AdminCodimoiCommentaireResponse> = Vec::new();
    let mut children_map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

    for &id in &order {
        if let Some(resp) = map.get(&id) {
            if let Some(parent_id) = resp.parent_id {
                children_map.entry(parent_id).or_default().push(id);
            }
        }
    }

    // Fonction recursive pour construire l'arbre
    fn build_tree(
        id: Uuid,
        map: &mut HashMap<Uuid, AdminCodimoiCommentaireResponse>,
        children_map: &HashMap<Uuid, Vec<Uuid>>,
    ) -> AdminCodimoiCommentaireResponse {
        let children_ids = children_map.get(&id).cloned().unwrap_or_default();
        let enfants: Vec<AdminCodimoiCommentaireResponse> = children_ids
            .into_iter()
            .filter_map(|child_id| {
                if map.contains_key(&child_id) {
                    Some(build_tree(child_id, map, children_map))
                } else {
                    None
                }
            })
            .collect();

        let mut resp = map.remove(&id).unwrap();
        resp.enfants = enfants;
        resp
    }

    for &id in &order {
        if map.contains_key(&id) {
            let resp = map.get(&id).unwrap();
            if resp.parent_id.is_none() {
                roots.push(build_tree(id, &mut map, &children_map));
            }
        }
    }

    roots
}

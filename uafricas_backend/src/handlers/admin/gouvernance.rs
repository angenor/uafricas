use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::gouvernance::*;
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ════════════════════════════════════════════════════════════════
// FactCheck
// ════════════════════════════════════════════════════════════════

/// GET /api/admin/factcheck
pub async fn lister_factchecks(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminFactcheckQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["f.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(f.search_vector @@ plainto_tsquery('french', ${bi}) OR LOWER(f.contenu) LIKE ${next})",
                bi = bind_index, next = bind_index + 1
            ));
            bind_values.push(r.to_string());
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 2;
        }
    }

    if let Some(ref verdict) = params.verdict {
        let v = verdict.trim();
        if !v.is_empty() {
            conditions.push(format!("f.verdict = ${}", bind_index));
            bind_values.push(v.to_string());
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("f.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_index += 1;
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("f.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(FACTCHECK_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM governance.factcheck f {} WHERE {}",
        FACTCHECK_JOINS, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    for v in &bind_uuids { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM governance.factcheck f {} WHERE {} ORDER BY f.{} {} LIMIT {} OFFSET {}",
        ADMIN_FACTCHECK_LISTE_COLONNES, FACTCHECK_JOINS, where_clause,
        colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminFactcheckListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    for v in &bind_uuids { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/factcheck/:id
pub async fn obtenir_factcheck(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM governance.factcheck f {} WHERE f.id = $1 AND f.deleted_at IS NULL",
        ADMIN_FACTCHECK_DETAIL_COLONNES, FACTCHECK_JOINS
    );
    let row = sqlx::query_as::<_, AdminFactcheckDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("FactCheck non trouve".into()))?;

    let nombre_commentaires: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM governance.factcheck_commentaire
         WHERE factcheck_id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(nombre_commentaires)),
        error: None,
    }))
}

/// POST /api/admin/factcheck
pub async fn creer_factcheck(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerFactcheckRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");

    let contenu = body.contenu.trim();
    if contenu.is_empty() {
        return Err(ApiErreur::Validation("Le contenu est requis".into()));
    }

    if let Some(ref verdict) = body.verdict {
        let verdicts_valides = ["vrai", "faux", "partiellement_vrai", "trompeur", "non_verifie"];
        if !verdicts_valides.contains(&verdict.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Verdict invalide. Valeurs acceptees: {}", verdicts_valides.join(", ")
            )));
        }
    }

    let etat = body.etat.as_deref().unwrap_or("publie");
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO governance.factcheck
         (id, contenu, source_originale, verdict, image_couverture_url, couleur_fond, pays_id, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(id)
    .bind(contenu)
    .bind(body.source_originale.as_deref().map(|s| s.trim()))
    .bind(body.verdict.as_deref())
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.couleur_fond.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(etat)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    // Mettre a jour le vecteur de recherche
    sqlx::query(
        "UPDATE governance.factcheck SET search_vector =
         to_tsvector('french', COALESCE(contenu, '') || ' ' || COALESCE(source_originale, ''))
         WHERE id = $1"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le factcheck {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "governance",
        "factcheck",
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

/// PUT /api/admin/factcheck/:id
pub async fn modifier_factcheck(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierFactcheckRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM governance.factcheck WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("FactCheck non trouve".into()));
    }

    if let Some(ref verdict) = body.verdict {
        let verdicts_valides = ["vrai", "faux", "partiellement_vrai", "trompeur", "non_verifie"];
        if !verdicts_valides.contains(&verdict.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Verdict invalide. Valeurs acceptees: {}", verdicts_valides.join(", ")
            )));
        }
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! ajouter_champ {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    ajouter_champ!(body.contenu, "contenu");
    ajouter_champ!(body.source_originale, "source_originale");
    ajouter_champ!(body.verdict, "verdict");
    ajouter_champ!(body.image_couverture_url, "image_couverture_url");
    ajouter_champ!(body.couleur_fond, "couleur_fond");
    ajouter_champ!(body.etat, "etat");

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE governance.factcheck SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    if body.contenu.is_some() || body.source_originale.is_some() {
        sqlx::query(
            "UPDATE governance.factcheck SET search_vector =
             to_tsvector('french', COALESCE(contenu, '') || ' ' || COALESCE(source_originale, ''))
             WHERE id = $1"
        )
        .bind(id)
        .execute(pool.get_ref())
        .await?;
    }

    // Engagement : jugement de modération du factcheck (non-bloquant, idempotent, anti-auto-attribution).
    // publie = jugé correct (+points/+réputation) ; suspendu = jugé faux/abusif (malus points + réputation).
    if let Some(nouvel_etat) = body.etat.as_deref() {
        if nouvel_etat == "publie" || nouvel_etat == "suspendu" {
            if let Ok(Some(cree_par)) = sqlx::query_scalar::<_, Uuid>(
                "SELECT cree_par FROM governance.factcheck WHERE id = $1",
            )
            .bind(id)
            .fetch_optional(pool.get_ref())
            .await
            {
                if cree_par != admin.id {
                    if nouvel_etat == "publie" {
                        crate::services::engagement::attribuer(
                            pool.get_ref(), cree_par, "factcheck_valide",
                            Some("factcheck"), Some(id), &format!("factcheck_valide:{id}"),
                        )
                        .await;
                    } else {
                        crate::services::engagement::retirer(
                            pool.get_ref(), cree_par, "factcheck_faux",
                            Some("factcheck"), Some(id), &format!("factcheck_faux:{id}"),
                        )
                        .await;
                    }
                }
            }
        }
    }

    log::info!("Admin {} a modifie le factcheck {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "governance",
        "factcheck",
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

/// DELETE /api/admin/factcheck/:id
pub async fn supprimer_factcheck(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE governance.factcheck SET deleted_at = NOW(), etat = 'supprime', updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("FactCheck non trouve".into()));
    }

    log::info!("Admin {} a supprime le factcheck {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "governance",
        "factcheck",
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

/// GET /api/admin/factcheck/:id/commentaires
pub async fn lister_commentaires_factcheck(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");
    let factcheck_id = path.into_inner();

    let rows = sqlx::query_as::<_, AdminFactcheckCommentaireRow>(
        "SELECT fc.id, fc.parent_id, fc.contenu, fc.type_commentaire,
                fc.cree_par, u.nom AS auteur_nom, u.prenom AS auteur_prenom,
                fc.nombre_likes, fc.created_at, fc.deleted_at
         FROM governance.factcheck_commentaire fc
         JOIN iam.utilisateur u ON u.id = fc.cree_par
         WHERE fc.factcheck_id = $1
         ORDER BY fc.created_at"
    )
    .bind(factcheck_id)
    .fetch_all(pool.get_ref())
    .await?;

    let commentaires = construire_arborescence_factcheck(&rows);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(commentaires),
        error: None,
    }))
}

/// DELETE /api/admin/factcheck/:id/commentaires/:commentaire_id
pub async fn supprimer_commentaire_factcheck(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "supprimer");
    let (factcheck_id, commentaire_id) = path.into_inner();

    let result = sqlx::query(
        "UPDATE governance.factcheck_commentaire SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND factcheck_id = $2 AND deleted_at IS NULL"
    )
    .bind(commentaire_id)
    .bind(factcheck_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Commentaire non trouve".into()));
    }

    log::info!("Admin {} a supprime le commentaire {} du factcheck {}", admin.id, commentaire_id, factcheck_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "governance",
        "factcheck_commentaire",
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

/// GET /api/admin/factcheck/:id/reactions
pub async fn obtenir_reactions_factcheck(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");
    let factcheck_id = path.into_inner();

    let row = sqlx::query_as::<_, (i32, i32)>(
        "SELECT nombre_likes, nombre_dislikes FROM governance.factcheck WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(factcheck_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("FactCheck non trouve".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminFactcheckReactionStats {
            nombre_likes: row.0,
            nombre_dislikes: row.1,
        }),
        error: None,
    }))
}

// ── Helper: arborescence commentaires factcheck ──────────────
fn construire_arborescence_factcheck(rows: &[AdminFactcheckCommentaireRow]) -> Vec<AdminFactcheckCommentaireResponse> {
    use std::collections::HashMap;

    let mut map: HashMap<Uuid, AdminFactcheckCommentaireResponse> = HashMap::new();
    let mut order: Vec<Uuid> = Vec::new();

    for row in rows {
        let resp = AdminFactcheckCommentaireResponse {
            id: row.id,
            parent_id: row.parent_id,
            contenu: if row.deleted_at.is_some() { "[Commentaire supprime]".to_string() } else { row.contenu.clone() },
            type_commentaire: row.type_commentaire.clone(),
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

    let mut children_map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for &id in &order {
        if let Some(resp) = map.get(&id) {
            if let Some(parent_id) = resp.parent_id {
                children_map.entry(parent_id).or_default().push(id);
            }
        }
    }

    fn build_tree(
        id: Uuid,
        map: &mut HashMap<Uuid, AdminFactcheckCommentaireResponse>,
        children_map: &HashMap<Uuid, Vec<Uuid>>,
    ) -> AdminFactcheckCommentaireResponse {
        let children_ids = children_map.get(&id).cloned().unwrap_or_default();
        let enfants: Vec<AdminFactcheckCommentaireResponse> = children_ids
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

    let mut roots = Vec::new();
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

// ════════════════════════════════════════════════════════════════
// Bad Habits (Mauvaises pratiques)
// ════════════════════════════════════════════════════════════════

/// GET /api/admin/bad-habits
pub async fn lister_bad_habits(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminBadHabitQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["b.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(b.titre) LIKE ${bi} OR LOWER(b.description_generale) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref categorie) = params.categorie_probleme {
        let c = categorie.trim();
        if !c.is_empty() {
            conditions.push(format!("b.categorie_probleme = ${}", bind_index));
            bind_values.push(c.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref gravite) = params.gravite {
        let g = gravite.trim();
        if !g.is_empty() {
            conditions.push(format!("b.gravite::text = ${}", bind_index));
            bind_values.push(g.to_string());
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("b.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_index += 1;
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("b.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(BAD_HABIT_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM governance.bad_habit b {} WHERE {}",
        BAD_HABIT_JOINS, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    for v in &bind_uuids { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM governance.bad_habit b {} WHERE {} ORDER BY b.{} {} LIMIT {} OFFSET {}",
        ADMIN_BAD_HABIT_LISTE_COLONNES, BAD_HABIT_JOINS, where_clause,
        colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminBadHabitListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    for v in &bind_uuids { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/bad-habits/:id
pub async fn obtenir_bad_habit(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM governance.bad_habit b {} WHERE b.id = $1 AND b.deleted_at IS NULL",
        ADMIN_BAD_HABIT_DETAIL_COLONNES, BAD_HABIT_JOINS
    );
    let row = sqlx::query_as::<_, AdminBadHabitDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Mauvaise pratique non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row),
        error: None,
    }))
}

/// POST /api/admin/bad-habits
pub async fn creer_bad_habit(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerBadHabitRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }
    let description = body.description_generale.trim();
    if description.is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }

    let categories_valides = [
        "corruption", "service_public_defaillant", "infrastructure_degradee",
        "acces_services_limite", "insalubrite", "probleme_securite", "autre",
    ];
    if !categories_valides.contains(&body.categorie_probleme.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Categorie invalide. Valeurs acceptees: {}", categories_valides.join(", ")
        )));
    }

    if let Some(ref gravite) = body.gravite {
        let gravites_valides = ["faible", "elevee", "critique"];
        if !gravites_valides.contains(&gravite.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Gravite invalide. Valeurs acceptees: {}", gravites_valides.join(", ")
            )));
        }
    }

    let etat = body.etat.as_deref().unwrap_or("en_attente");
    let gravite = body.gravite.as_deref().unwrap_or("faible");
    let id = Uuid::new_v4();
    let slug = generer_slug(titre);

    sqlx::query(
        "INSERT INTO governance.bad_habit
         (id, titre, slug, description_generale, details_problematique,
          categorie_probleme, categorie_probleme_detail, gravite,
          preuves_temoignages, solutions_proposees,
          publication_anonyme, geolocalisation_autorisee, longitude, latitude,
          pays_id, region, ville_quartier_zone, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8::governance.niveau_gravite, $9, $10,
                 $11, $12, $13, $14, $15, $16, $17, $18, $19)"
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(description)
    .bind(body.details_problematique.trim())
    .bind(&body.categorie_probleme)
    .bind(body.categorie_probleme_detail.as_deref().map(|s| s.trim()))
    .bind(gravite)
    .bind(body.preuves_temoignages.as_deref().map(|s| s.trim()))
    .bind(body.solutions_proposees.as_deref().map(|s| s.trim()))
    .bind(body.publication_anonyme.unwrap_or(false))
    .bind(body.geolocalisation_autorisee.unwrap_or(false))
    .bind(body.longitude)
    .bind(body.latitude)
    .bind(body.pays_id)
    .bind(body.region.as_deref().map(|s| s.trim()))
    .bind(body.ville_quartier_zone.as_deref().map(|s| s.trim()))
    .bind(etat)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree la mauvaise pratique {} ({})", admin.id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "governance",
        "mauvaise_pratique",
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

/// PUT /api/admin/bad-habits/:id
pub async fn modifier_bad_habit(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierBadHabitRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM governance.bad_habit WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Mauvaise pratique non trouvee".into()));
    }

    if let Some(ref categorie) = body.categorie_probleme {
        let categories_valides = [
            "corruption", "service_public_defaillant", "infrastructure_degradee",
            "acces_services_limite", "insalubrite", "probleme_securite", "autre",
        ];
        if !categories_valides.contains(&categorie.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Categorie invalide. Valeurs acceptees: {}", categories_valides.join(", ")
            )));
        }
    }

    if let Some(ref gravite) = body.gravite {
        let gravites_valides = ["faible", "elevee", "critique"];
        if !gravites_valides.contains(&gravite.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Gravite invalide. Valeurs acceptees: {}", gravites_valides.join(", ")
            )));
        }
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! ajouter_champ {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    ajouter_champ!(body.titre, "titre");
    ajouter_champ!(body.description_generale, "description_generale");
    ajouter_champ!(body.details_problematique, "details_problematique");
    ajouter_champ!(body.categorie_probleme, "categorie_probleme");
    ajouter_champ!(body.categorie_probleme_detail, "categorie_probleme_detail");
    ajouter_champ!(body.preuves_temoignages, "preuves_temoignages");
    ajouter_champ!(body.solutions_proposees, "solutions_proposees");
    ajouter_champ!(body.region, "region");
    ajouter_champ!(body.ville_quartier_zone, "ville_quartier_zone");
    ajouter_champ!(body.etat, "etat");

    if let Some(ref gravite) = body.gravite {
        sets.push(format!("gravite = ${}::governance.niveau_gravite", bind_index));
        bind_strings.push(gravite.clone());
        bind_index += 1;
    }

    if let Some(publication_anonyme) = body.publication_anonyme {
        sets.push(format!("publication_anonyme = {}", publication_anonyme));
    }
    if let Some(geolocalisation_autorisee) = body.geolocalisation_autorisee {
        sets.push(format!("geolocalisation_autorisee = {}", geolocalisation_autorisee));
    }
    if let Some(longitude) = body.longitude {
        sets.push(format!("longitude = {}", longitude));
    }
    if let Some(latitude) = body.latitude {
        sets.push(format!("latitude = {}", latitude));
    }
    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }

    // Slug si titre modifie
    if let Some(ref titre) = body.titre {
        let new_slug = generer_slug(titre.trim());
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(new_slug);
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE governance.bad_habit SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    // Engagement : contribution validée quand la bonne pratique passe à « publié ».
    if body.etat.as_deref() == Some("publie") {
        if let Ok(Some(cree_par)) = sqlx::query_scalar::<_, Uuid>(
            "SELECT cree_par FROM governance.bad_habit WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await
        {
            if cree_par != admin.id {
                crate::services::engagement::attribuer(
                    pool.get_ref(), cree_par, "contribution_validee",
                    Some("bad_habit"), Some(id), &format!("contribution_validee:bad_habit:{id}"),
                )
                .await;
            }
        }
    }

    log::info!("Admin {} a modifie la mauvaise pratique {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "governance",
        "mauvaise_pratique",
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

/// DELETE /api/admin/bad-habits/:id
pub async fn supprimer_bad_habit(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE governance.bad_habit SET deleted_at = NOW(), etat = 'supprime', updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Mauvaise pratique non trouvee".into()));
    }

    log::info!("Admin {} a supprime la mauvaise pratique {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "governance",
        "mauvaise_pratique",
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

/// POST /api/admin/bad-habits/:id/medias
pub async fn ajouter_media_bad_habit(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterMediaBadHabitRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");
    let bad_habit_id = path.into_inner();

    let media_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO governance.bad_habit_media (id, bad_habit_id, media_url, type_mime, ordre)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(media_id)
    .bind(bad_habit_id)
    .bind(body.media_url.trim())
    .bind(body.type_mime.as_deref())
    .bind(body.ordre.unwrap_or(0))
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a ajoute un media a la mauvaise pratique {}", admin.id, bad_habit_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "governance",
        "mauvaise_pratique_media",
        Some(media_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": media_id })),
        error: None,
    }))
}

/// DELETE /api/admin/bad-habits/:id/medias/:media_id
pub async fn retirer_media_bad_habit(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");
    let (bad_habit_id, media_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM governance.bad_habit_media WHERE id = $1 AND bad_habit_id = $2"
    )
    .bind(media_id)
    .bind(bad_habit_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Media non trouve".into()));
    }

    log::info!("Admin {} a retire le media {} de la mauvaise pratique {}", admin.id, media_id, bad_habit_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "governance",
        "mauvaise_pratique_media",
        Some(media_id),
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

/// GET /api/admin/bad-habits/:id/medias (helper interne, utilise dans obtenir_bad_habit si besoin)
pub async fn lister_medias_bad_habit(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");
    let bad_habit_id = path.into_inner();

    let medias = sqlx::query_as::<_, AdminBadHabitMedia>(
        "SELECT id, media_url, type_mime, ordre, created_at
         FROM governance.bad_habit_media
         WHERE bad_habit_id = $1
         ORDER BY ordre, created_at"
    )
    .bind(bad_habit_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(medias),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// Idea Forces (Idees forces)
// ════════════════════════════════════════════════════════════════

/// GET /api/admin/idea-forces
pub async fn lister_idea_forces(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminIdeaForceQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["i.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(i.titre) LIKE ${bi} OR LOWER(i.description_generale) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref categorie) = params.categorie_proposition {
        let c = categorie.trim();
        if !c.is_empty() {
            conditions.push(format!("i.categorie_proposition = ${}", bind_index));
            bind_values.push(c.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref urgence) = params.urgence {
        let u = urgence.trim();
        if !u.is_empty() {
            conditions.push(format!("i.urgence::text = ${}", bind_index));
            bind_values.push(u.to_string());
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("i.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_index += 1;
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("i.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(IDEA_FORCE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM governance.idea_force i {} WHERE {}",
        IDEA_FORCE_JOINS, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    for v in &bind_uuids { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM governance.idea_force i {} WHERE {} ORDER BY i.{} {} LIMIT {} OFFSET {}",
        ADMIN_IDEA_FORCE_LISTE_COLONNES, IDEA_FORCE_JOINS, where_clause,
        colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminIdeaForceListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    for v in &bind_uuids { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/idea-forces/:id
pub async fn obtenir_idea_force(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM governance.idea_force i {} WHERE i.id = $1 AND i.deleted_at IS NULL",
        ADMIN_IDEA_FORCE_DETAIL_COLONNES, IDEA_FORCE_JOINS
    );
    let row = sqlx::query_as::<_, AdminIdeaForceDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Idee force non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row),
        error: None,
    }))
}

/// POST /api/admin/idea-forces
pub async fn creer_idea_force(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerIdeaForceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }
    let description = body.description_generale.trim();
    if description.is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }

    let categories_valides = [
        "amelioration_gouvernance", "education_formation", "sante_publique",
        "emploi_jeunes", "environnement", "transport", "autre",
    ];
    if !categories_valides.contains(&body.categorie_proposition.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Categorie invalide. Valeurs acceptees: {}", categories_valides.join(", ")
        )));
    }

    if let Some(ref urgence) = body.urgence {
        let urgences_valides = ["faible", "elevee", "critique"];
        if !urgences_valides.contains(&urgence.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Urgence invalide. Valeurs acceptees: {}", urgences_valides.join(", ")
            )));
        }
    }

    let etat = body.etat.as_deref().unwrap_or("en_attente");
    let urgence = body.urgence.as_deref().unwrap_or("faible");
    let id = Uuid::new_v4();
    let slug = generer_slug(titre);

    sqlx::query(
        "INSERT INTO governance.idea_force
         (id, titre, slug, description_generale, details_proposition,
          categorie_proposition, categorie_proposition_detail, urgence,
          plan_implementation, ressources_necessaires, impact_attendu,
          pays_id, region, ville_quartier_zone, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8::governance.niveau_gravite, $9, $10,
                 $11, $12, $13, $14, $15, $16)"
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(description)
    .bind(body.details_proposition.trim())
    .bind(&body.categorie_proposition)
    .bind(body.categorie_proposition_detail.as_deref().map(|s| s.trim()))
    .bind(urgence)
    .bind(body.plan_implementation.as_deref().map(|s| s.trim()))
    .bind(body.ressources_necessaires.as_deref().map(|s| s.trim()))
    .bind(body.impact_attendu.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.region.as_deref().map(|s| s.trim()))
    .bind(body.ville_quartier_zone.as_deref().map(|s| s.trim()))
    .bind(etat)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree l'idee force {} ({})", admin.id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "governance",
        "idee_force",
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

/// PUT /api/admin/idea-forces/:id
pub async fn modifier_idea_force(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierIdeaForceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM governance.idea_force WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Idee force non trouvee".into()));
    }

    if let Some(ref categorie) = body.categorie_proposition {
        let categories_valides = [
            "amelioration_gouvernance", "education_formation", "sante_publique",
            "emploi_jeunes", "environnement", "transport", "autre",
        ];
        if !categories_valides.contains(&categorie.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Categorie invalide. Valeurs acceptees: {}", categories_valides.join(", ")
            )));
        }
    }

    if let Some(ref urgence) = body.urgence {
        let urgences_valides = ["faible", "elevee", "critique"];
        if !urgences_valides.contains(&urgence.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Urgence invalide. Valeurs acceptees: {}", urgences_valides.join(", ")
            )));
        }
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! ajouter_champ {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    ajouter_champ!(body.titre, "titre");
    ajouter_champ!(body.description_generale, "description_generale");
    ajouter_champ!(body.details_proposition, "details_proposition");
    ajouter_champ!(body.categorie_proposition, "categorie_proposition");
    ajouter_champ!(body.categorie_proposition_detail, "categorie_proposition_detail");
    ajouter_champ!(body.plan_implementation, "plan_implementation");
    ajouter_champ!(body.ressources_necessaires, "ressources_necessaires");
    ajouter_champ!(body.impact_attendu, "impact_attendu");
    ajouter_champ!(body.region, "region");
    ajouter_champ!(body.ville_quartier_zone, "ville_quartier_zone");
    ajouter_champ!(body.etat, "etat");

    if let Some(ref urgence) = body.urgence {
        sets.push(format!("urgence = ${}::governance.niveau_gravite", bind_index));
        bind_strings.push(urgence.clone());
        bind_index += 1;
    }

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }

    if let Some(ref titre) = body.titre {
        let new_slug = generer_slug(titre.trim());
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(new_slug);
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE governance.idea_force SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    // Engagement : contribution validée quand l'idée force passe à « publié ».
    if body.etat.as_deref() == Some("publie") {
        if let Ok(Some(cree_par)) = sqlx::query_scalar::<_, Uuid>(
            "SELECT cree_par FROM governance.idea_force WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await
        {
            if cree_par != admin.id {
                crate::services::engagement::attribuer(
                    pool.get_ref(), cree_par, "contribution_validee",
                    Some("ideaforce"), Some(id), &format!("contribution_validee:ideaforce:{id}"),
                )
                .await;
            }
        }
    }

    log::info!("Admin {} a modifie l'idee force {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "governance",
        "idee_force",
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

/// DELETE /api/admin/idea-forces/:id
pub async fn supprimer_idea_force(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE governance.idea_force SET deleted_at = NOW(), etat = 'supprime', updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Idee force non trouvee".into()));
    }

    log::info!("Admin {} a supprime l'idee force {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "governance",
        "idee_force",
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

/// POST /api/admin/idea-forces/:id/medias
pub async fn ajouter_media_idea_force(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterMediaIdeaForceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");
    let idea_force_id = path.into_inner();

    let media_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO governance.idea_force_media (id, idea_force_id, media_url, type_mime, ordre)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(media_id)
    .bind(idea_force_id)
    .bind(body.media_url.trim())
    .bind(body.type_mime.as_deref())
    .bind(body.ordre.unwrap_or(0))
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a ajoute un media a l'idee force {}", admin.id, idea_force_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "governance",
        "idee_force_media",
        Some(media_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": media_id })),
        error: None,
    }))
}

/// DELETE /api/admin/idea-forces/:id/medias/:media_id
pub async fn retirer_media_idea_force(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "modifier");
    let (idea_force_id, media_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM governance.idea_force_media WHERE id = $1 AND idea_force_id = $2"
    )
    .bind(media_id)
    .bind(idea_force_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Media non trouve".into()));
    }

    log::info!("Admin {} a retire le media {} de l'idee force {}", admin.id, media_id, idea_force_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "governance",
        "idee_force_media",
        Some(media_id),
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

/// GET /api/admin/idea-forces/:id/medias
pub async fn lister_medias_idea_force(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "gouvernance", "voir");
    let idea_force_id = path.into_inner();

    let medias = sqlx::query_as::<_, AdminIdeaForceMedia>(
        "SELECT id, media_url, type_mime, ordre, created_at
         FROM governance.idea_force_media
         WHERE idea_force_id = $1
         ORDER BY ordre, created_at"
    )
    .bind(idea_force_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(medias),
        error: None,
    }))
}

// ── Helper: generer un slug ──────────────────────────────────
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
        .collect::<Vec<_>>()
        .join("-")
}

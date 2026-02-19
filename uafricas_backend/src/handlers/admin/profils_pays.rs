use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::profils_pays::{
    AdminFichePayListeResponse, AdminFichePayDetailRow, AdminFichePayQueryParams,
    CreerFichePayRequest, ModifierFichePayRequest, SousEntiteCounts,
    AdminRegionResponse, CreerRegionRequest, ModifierRegionRequest,
    AdminGroupeEthniqueResponse, CreerGroupeEthniqueRequest, ModifierGroupeEthniqueRequest,
    AdminAllianceResponse, CreerAllianceRequest, ModifierAllianceRequest,
    AdminConteResponse, AdminConteQueryParams, CreerConteRequest, ModifierConteRequest,
    AdminSiteTouristiqueResponse, CreerSiteTouristiqueRequest, ModifierSiteTouristiqueRequest,
    AdminSecteurResponse, CreerSecteurRequest, ModifierSecteurRequest,
    AdminSaisonResponse, CreerSaisonRequest, ModifierSaisonRequest,
    AdminLienInterethniqueResponse, CreerLienInterethniqueRequest, ModifierLienInterethniqueRequest,
    AdminContributionListeResponse, AdminContributionDetailRow, AdminContributionQueryParams,
    ModererContributionRequest,
    ADMIN_FICHE_PAYS_LISTE_COLONNES, ADMIN_FICHE_PAYS_DETAIL_COLONNES, FICHE_PAYS_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

const TYPES_CONTE_VALIDES: &[&str] = &["conte", "histoire_drole", "legende", "mythe"];
const ETATS_CONTRIBUTION_VALIDES: &[&str] = &["approuvee", "rejetee"];

/// Verifie qu'une fiche pays existe et retourne son ID
async fn verifier_fiche_existe(pool: &PgPool, fiche_id: Uuid) -> Result<(), ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM country_profile.fiche_pays WHERE id = $1)"
    ).bind(fiche_id).fetch_one(pool).await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Fiche pays non trouvee".into()));
    }
    Ok(())
}

// ══════════════════════════════════════════════════════════════
// ── Fiche Pays (CRUD principal) ──────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays
pub async fn lister_fiches_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminFichePayQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = Vec::new();
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(p.nom) LIKE ${bi} OR LOWER(fp.slogan) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref continent) = params.continent {
        let c = continent.trim();
        if !c.is_empty() {
            conditions.push(format!("LOWER(p.continent) = ${}", bind_index));
            bind_values.push(c.to_lowercase());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = if conditions.is_empty() {
        "TRUE".to_string()
    } else {
        conditions.join(" AND ")
    };

    let joins = "JOIN shared.pays p ON fp.pays_id = p.id
                 LEFT JOIN iam.utilisateur u ON fp.cree_par = u.id";

    let colonne = pagination.colonne_tri(FICHE_PAYS_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM country_profile.fiche_pays fp {} WHERE {}",
        joins, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM country_profile.fiche_pays fp {} WHERE {} ORDER BY fp.{} {} LIMIT {} OFFSET {}",
        ADMIN_FICHE_PAYS_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminFichePayListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/profils-pays/{id}
pub async fn obtenir_fiche_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let id = path.into_inner();

    let joins = "JOIN shared.pays p ON fp.pays_id = p.id
                 LEFT JOIN iam.utilisateur u ON fp.cree_par = u.id";

    let sql = format!(
        "SELECT {} FROM country_profile.fiche_pays fp {} WHERE fp.id = $1",
        ADMIN_FICHE_PAYS_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminFichePayDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Fiche pays non trouvee".into()))?;

    let counts = SousEntiteCounts {
        nb_regions: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.region WHERE fiche_pays_id = $1"
        ).bind(id).fetch_one(pool.get_ref()).await?,
        nb_groupes_ethniques: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.groupe_ethnique WHERE fiche_pays_id = $1"
        ).bind(id).fetch_one(pool.get_ref()).await?,
        nb_alliances: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.alliance_interethnique WHERE fiche_pays_id = $1"
        ).bind(id).fetch_one(pool.get_ref()).await?,
        nb_contes: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.conte_histoire WHERE fiche_pays_id = $1"
        ).bind(id).fetch_one(pool.get_ref()).await?,
        nb_sites_touristiques: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.site_touristique WHERE fiche_pays_id = $1"
        ).bind(id).fetch_one(pool.get_ref()).await?,
        nb_secteurs: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.secteur_developpement WHERE fiche_pays_id = $1"
        ).bind(id).fetch_one(pool.get_ref()).await?,
        nb_saisons: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.saison WHERE fiche_pays_id = $1"
        ).bind(id).fetch_one(pool.get_ref()).await?,
        nb_liens_interethniques: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.lien_interethnique WHERE fiche_pays_id = $1"
        ).bind(id).fetch_one(pool.get_ref()).await?,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(counts)),
        error: None,
    }))
}

/// POST /api/admin/profils-pays
pub async fn creer_fiche_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<CreerFichePayRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");

    // Verifier que le pays existe
    let pays_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM shared.pays WHERE id = $1)"
    ).bind(body.pays_id).fetch_one(pool.get_ref()).await?;
    if !pays_existe {
        return Err(ApiErreur::Validation("Pays non trouve".into()));
    }

    // Verifier unicite (1 fiche par pays)
    let deja_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM country_profile.fiche_pays WHERE pays_id = $1)"
    ).bind(body.pays_id).fetch_one(pool.get_ref()).await?;
    if deja_existe {
        return Err(ApiErreur::Validation("Une fiche existe deja pour ce pays".into()));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.fiche_pays
         (id, pays_id, slogan, superficie_km2, population, biographie, contexte,
          contexte_historique, image_couverture_url, image_drapeau_url, image_embleme_url,
          image_devise_url, hymne_national, langue_officielle, langues_populaires,
          monnaie, fuseau_horaire, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"
    )
    .bind(id)
    .bind(body.pays_id)
    .bind(body.slogan.as_deref().map(|s| s.trim()))
    .bind(body.superficie_km2)
    .bind(body.population)
    .bind(body.biographie.as_deref().map(|s| s.trim()))
    .bind(body.contexte.as_deref().map(|s| s.trim()))
    .bind(body.contexte_historique.as_deref().map(|s| s.trim()))
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.image_drapeau_url.as_deref().map(|s| s.trim()))
    .bind(body.image_embleme_url.as_deref().map(|s| s.trim()))
    .bind(body.image_devise_url.as_deref().map(|s| s.trim()))
    .bind(body.hymne_national.as_deref().map(|s| s.trim()))
    .bind(body.langue_officielle.as_deref().map(|s| s.trim()))
    .bind(body.langues_populaires.as_deref().map(|s| s.trim()))
    .bind(body.monnaie.as_deref().map(|s| s.trim()))
    .bind(body.fuseau_horaire.as_deref().map(|s| s.trim()))
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree la fiche pays {} pour pays {}", admin.id, id, body.pays_id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/profils-pays/{id}
pub async fn modifier_fiche_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierFichePayRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let id = path.into_inner();

    verifier_fiche_existe(pool.get_ref(), id).await?;

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

    champ_str!(body.slogan, "slogan");
    champ_str!(body.biographie, "biographie");
    champ_str!(body.contexte, "contexte");
    champ_str!(body.contexte_historique, "contexte_historique");
    champ_str!(body.image_couverture_url, "image_couverture_url");
    champ_str!(body.image_drapeau_url, "image_drapeau_url");
    champ_str!(body.image_embleme_url, "image_embleme_url");
    champ_str!(body.image_devise_url, "image_devise_url");
    champ_str!(body.hymne_national, "hymne_national");
    champ_str!(body.langue_officielle, "langue_officielle");
    champ_str!(body.langues_populaires, "langues_populaires");
    champ_str!(body.monnaie, "monnaie");
    champ_str!(body.fuseau_horaire, "fuseau_horaire");

    if let Some(v) = body.superficie_km2 {
        sets.push(format!("superficie_km2 = {}", v));
    }
    if let Some(v) = body.population {
        sets.push(format!("population = {}", v));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE country_profile.fiche_pays SET {} WHERE id = ${}",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie la fiche pays {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/profils-pays/{id}
pub async fn supprimer_fiche_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM country_profile.fiche_pays WHERE id = $1"
    ).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Fiche pays non trouvee".into()));
    }

    log::info!("Admin {} a supprime la fiche pays {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// ── Regions ──────────────────────────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays/{id}/regions
pub async fn lister_regions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let fiche_id = path.into_inner();

    let items = sqlx::query_as::<_, AdminRegionResponse>(
        "SELECT id, fiche_pays_id, nom, chef_lieu, description, population, created_at, updated_at
         FROM country_profile.region WHERE fiche_pays_id = $1 ORDER BY nom"
    ).bind(fiche_id).fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(items), error: None }))
}

/// POST /api/admin/profils-pays/{id}/regions
pub async fn creer_region(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerRegionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let fiche_id = path.into_inner();
    verifier_fiche_existe(pool.get_ref(), fiche_id).await?;

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom est requis".into()));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.region (id, fiche_pays_id, nom, chef_lieu, description, population)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(id).bind(fiche_id).bind(nom)
    .bind(body.chef_lieu.as_deref().map(|s| s.trim()))
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.population)
    .execute(pool.get_ref()).await?;

    log::info!("Admin {} a cree la region {} pour fiche {}", admin.id, id, fiche_id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/profils-pays/{id}/regions/{region_id}
pub async fn modifier_region(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ModifierRegionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (fiche_id, region_id) = path.into_inner();

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
    champ_str!(body.chef_lieu, "chef_lieu");
    champ_str!(body.description, "description");
    if let Some(v) = body.population {
        sets.push(format!("population = {}", v));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }
    sets.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE country_profile.region SET {} WHERE id = ${} AND fiche_pays_id = ${}",
        sets.join(", "), bind_index, bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(region_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Region non trouvee".into()));
    }

    log::info!("Admin {} a modifie la region {}", admin.id, region_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": region_id })),
        error: None,
    }))
}

/// DELETE /api/admin/profils-pays/{id}/regions/{region_id}
pub async fn supprimer_region(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "supprimer");
    let (fiche_id, region_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM country_profile.region WHERE id = $1 AND fiche_pays_id = $2"
    ).bind(region_id).bind(fiche_id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Region non trouvee".into()));
    }

    log::info!("Admin {} a supprime la region {}", admin.id, region_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// ── Groupes ethniques ────────────────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays/{id}/groupes-ethniques
pub async fn lister_groupes_ethniques(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let fiche_id = path.into_inner();

    let items = sqlx::query_as::<_, AdminGroupeEthniqueResponse>(
        "SELECT ge.id, ge.fiche_pays_id, ge.nom, ge.description,
                ge.objets_culturels_distinctifs, ge.population_estimee, ge.langues,
                ge.region_id, r.nom AS region_nom,
                ge.created_at, ge.updated_at
         FROM country_profile.groupe_ethnique ge
         LEFT JOIN country_profile.region r ON ge.region_id = r.id
         WHERE ge.fiche_pays_id = $1 ORDER BY ge.nom"
    ).bind(fiche_id).fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(items), error: None }))
}

/// POST /api/admin/profils-pays/{id}/groupes-ethniques
pub async fn creer_groupe_ethnique(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerGroupeEthniqueRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let fiche_id = path.into_inner();
    verifier_fiche_existe(pool.get_ref(), fiche_id).await?;

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom est requis".into()));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.groupe_ethnique
         (id, fiche_pays_id, nom, description, objets_culturels_distinctifs,
          population_estimee, langues, region_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(id).bind(fiche_id).bind(nom)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.objets_culturels_distinctifs.as_deref().map(|s| s.trim()))
    .bind(body.population_estimee.as_deref().map(|s| s.trim()))
    .bind(body.langues.as_deref().map(|s| s.trim()))
    .bind(body.region_id)
    .execute(pool.get_ref()).await?;

    log::info!("Admin {} a cree le groupe ethnique {} pour fiche {}", admin.id, id, fiche_id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/profils-pays/{id}/groupes-ethniques/{ge_id}
pub async fn modifier_groupe_ethnique(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ModifierGroupeEthniqueRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (fiche_id, ge_id) = path.into_inner();

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
    champ_str!(body.objets_culturels_distinctifs, "objets_culturels_distinctifs");
    champ_str!(body.population_estimee, "population_estimee");
    champ_str!(body.langues, "langues");

    if let Some(region_id) = body.region_id {
        sets.push(format!("region_id = '{}'", region_id));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }
    sets.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE country_profile.groupe_ethnique SET {} WHERE id = ${} AND fiche_pays_id = ${}",
        sets.join(", "), bind_index, bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(ge_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Groupe ethnique non trouve".into()));
    }

    log::info!("Admin {} a modifie le groupe ethnique {}", admin.id, ge_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": ge_id })),
        error: None,
    }))
}

/// DELETE /api/admin/profils-pays/{id}/groupes-ethniques/{ge_id}
pub async fn supprimer_groupe_ethnique(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "supprimer");
    let (fiche_id, ge_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM country_profile.groupe_ethnique WHERE id = $1 AND fiche_pays_id = $2"
    ).bind(ge_id).bind(fiche_id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Groupe ethnique non trouve".into()));
    }

    log::info!("Admin {} a supprime le groupe ethnique {}", admin.id, ge_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// ── Alliances interethniques ─────────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays/{id}/alliances
pub async fn lister_alliances(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let fiche_id = path.into_inner();

    let items = sqlx::query_as::<_, AdminAllianceResponse>(
        "SELECT id, fiche_pays_id, nom, description, groupes_impliques, signification,
                created_at, updated_at
         FROM country_profile.alliance_interethnique WHERE fiche_pays_id = $1 ORDER BY nom"
    ).bind(fiche_id).fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(items), error: None }))
}

/// POST /api/admin/profils-pays/{id}/alliances
pub async fn creer_alliance(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerAllianceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let fiche_id = path.into_inner();
    verifier_fiche_existe(pool.get_ref(), fiche_id).await?;

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom est requis".into()));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.alliance_interethnique
         (id, fiche_pays_id, nom, description, groupes_impliques, signification)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(id).bind(fiche_id).bind(nom)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.groupes_impliques.as_deref().map(|s| s.trim()))
    .bind(body.signification.as_deref().map(|s| s.trim()))
    .execute(pool.get_ref()).await?;

    log::info!("Admin {} a cree l'alliance {} pour fiche {}", admin.id, id, fiche_id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/profils-pays/{id}/alliances/{alliance_id}
pub async fn modifier_alliance(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ModifierAllianceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (fiche_id, alliance_id) = path.into_inner();

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
    champ_str!(body.groupes_impliques, "groupes_impliques");
    champ_str!(body.signification, "signification");

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }
    sets.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE country_profile.alliance_interethnique SET {} WHERE id = ${} AND fiche_pays_id = ${}",
        sets.join(", "), bind_index, bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(alliance_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Alliance non trouvee".into()));
    }

    log::info!("Admin {} a modifie l'alliance {}", admin.id, alliance_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": alliance_id })),
        error: None,
    }))
}

/// DELETE /api/admin/profils-pays/{id}/alliances/{alliance_id}
pub async fn supprimer_alliance(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "supprimer");
    let (fiche_id, alliance_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM country_profile.alliance_interethnique WHERE id = $1 AND fiche_pays_id = $2"
    ).bind(alliance_id).bind(fiche_id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Alliance non trouvee".into()));
    }

    log::info!("Admin {} a supprime l'alliance {}", admin.id, alliance_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// ── Contes & Histoires ───────────────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays/{id}/contes
pub async fn lister_contes(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    params: web::Query<AdminConteQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let fiche_id = path.into_inner();

    let mut conditions = vec!["c.fiche_pays_id = $1".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 2;

    if let Some(ref type_conte) = params.type_conte {
        let t = type_conte.trim();
        if !t.is_empty() {
            conditions.push(format!("c.type = ${}", bind_index));
            bind_values.push(t.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let sql = format!(
        "SELECT c.id, c.fiche_pays_id, c.titre, c.contenu, c.type AS type_conte,
                c.groupe_ethnique_id, ge.nom AS groupe_ethnique_nom, c.image_url,
                c.created_at, c.updated_at
         FROM country_profile.conte_histoire c
         LEFT JOIN country_profile.groupe_ethnique ge ON c.groupe_ethnique_id = ge.id
         WHERE {} ORDER BY c.titre",
        where_clause
    );

    let mut q = sqlx::query_as::<_, AdminConteResponse>(&sql).bind(fiche_id);
    for v in &bind_values { q = q.bind(v); }
    let items = q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(items), error: None }))
}

/// POST /api/admin/profils-pays/{id}/contes
pub async fn creer_conte(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerConteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let fiche_id = path.into_inner();
    verifier_fiche_existe(pool.get_ref(), fiche_id).await?;

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }

    if let Some(ref t) = body.type_conte {
        if !TYPES_CONTE_VALIDES.contains(&t.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Type invalide: {}. Valeurs possibles: {:?}", t, TYPES_CONTE_VALIDES
            )));
        }
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.conte_histoire
         (id, fiche_pays_id, titre, contenu, type, groupe_ethnique_id, image_url)
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(id).bind(fiche_id).bind(titre)
    .bind(body.contenu.as_deref().map(|s| s.trim()))
    .bind(body.type_conte.as_deref())
    .bind(body.groupe_ethnique_id)
    .bind(body.image_url.as_deref().map(|s| s.trim()))
    .execute(pool.get_ref()).await?;

    log::info!("Admin {} a cree le conte {} pour fiche {}", admin.id, id, fiche_id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/profils-pays/{id}/contes/{conte_id}
pub async fn modifier_conte(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ModifierConteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (fiche_id, conte_id) = path.into_inner();

    if let Some(ref t) = body.type_conte {
        if !TYPES_CONTE_VALIDES.contains(&t.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Type invalide: {}. Valeurs possibles: {:?}", t, TYPES_CONTE_VALIDES
            )));
        }
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
    champ_str!(body.contenu, "contenu");
    champ_str!(body.type_conte, "type");
    champ_str!(body.image_url, "image_url");

    if let Some(ge_id) = body.groupe_ethnique_id {
        sets.push(format!("groupe_ethnique_id = '{}'", ge_id));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }
    sets.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE country_profile.conte_histoire SET {} WHERE id = ${} AND fiche_pays_id = ${}",
        sets.join(", "), bind_index, bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(conte_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Conte non trouve".into()));
    }

    log::info!("Admin {} a modifie le conte {}", admin.id, conte_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": conte_id })),
        error: None,
    }))
}

/// DELETE /api/admin/profils-pays/{id}/contes/{conte_id}
pub async fn supprimer_conte(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "supprimer");
    let (fiche_id, conte_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM country_profile.conte_histoire WHERE id = $1 AND fiche_pays_id = $2"
    ).bind(conte_id).bind(fiche_id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Conte non trouve".into()));
    }

    log::info!("Admin {} a supprime le conte {}", admin.id, conte_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// ── Sites touristiques ───────────────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays/{id}/sites-touristiques
pub async fn lister_sites_touristiques(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let fiche_id = path.into_inner();

    let items = sqlx::query_as::<_, AdminSiteTouristiqueResponse>(
        "SELECT st.id, st.fiche_pays_id, st.nom, st.description, st.image_url,
                st.longitude::float8 AS longitude, st.latitude::float8 AS latitude,
                st.region_id, r.nom AS region_nom,
                st.created_at, st.updated_at
         FROM country_profile.site_touristique st
         LEFT JOIN country_profile.region r ON st.region_id = r.id
         WHERE st.fiche_pays_id = $1 ORDER BY st.nom"
    ).bind(fiche_id).fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(items), error: None }))
}

/// POST /api/admin/profils-pays/{id}/sites-touristiques
pub async fn creer_site_touristique(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerSiteTouristiqueRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let fiche_id = path.into_inner();
    verifier_fiche_existe(pool.get_ref(), fiche_id).await?;

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom est requis".into()));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.site_touristique
         (id, fiche_pays_id, nom, description, image_url, longitude, latitude, region_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(id).bind(fiche_id).bind(nom)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.image_url.as_deref().map(|s| s.trim()))
    .bind(body.longitude)
    .bind(body.latitude)
    .bind(body.region_id)
    .execute(pool.get_ref()).await?;

    log::info!("Admin {} a cree le site touristique {} pour fiche {}", admin.id, id, fiche_id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/profils-pays/{id}/sites-touristiques/{site_id}
pub async fn modifier_site_touristique(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ModifierSiteTouristiqueRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (fiche_id, site_id) = path.into_inner();

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
    champ_str!(body.image_url, "image_url");

    if let Some(v) = body.longitude {
        sets.push(format!("longitude = {}", v));
    }
    if let Some(v) = body.latitude {
        sets.push(format!("latitude = {}", v));
    }
    if let Some(region_id) = body.region_id {
        sets.push(format!("region_id = '{}'", region_id));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }
    sets.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE country_profile.site_touristique SET {} WHERE id = ${} AND fiche_pays_id = ${}",
        sets.join(", "), bind_index, bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(site_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Site touristique non trouve".into()));
    }

    log::info!("Admin {} a modifie le site touristique {}", admin.id, site_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": site_id })),
        error: None,
    }))
}

/// DELETE /api/admin/profils-pays/{id}/sites-touristiques/{site_id}
pub async fn supprimer_site_touristique(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "supprimer");
    let (fiche_id, site_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM country_profile.site_touristique WHERE id = $1 AND fiche_pays_id = $2"
    ).bind(site_id).bind(fiche_id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Site touristique non trouve".into()));
    }

    log::info!("Admin {} a supprime le site touristique {}", admin.id, site_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// ── Secteurs de developpement ────────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays/{id}/secteurs
pub async fn lister_secteurs(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let fiche_id = path.into_inner();

    let items = sqlx::query_as::<_, AdminSecteurResponse>(
        "SELECT id, fiche_pays_id, nom, description, created_at
         FROM country_profile.secteur_developpement WHERE fiche_pays_id = $1 ORDER BY nom"
    ).bind(fiche_id).fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(items), error: None }))
}

/// POST /api/admin/profils-pays/{id}/secteurs
pub async fn creer_secteur(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerSecteurRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let fiche_id = path.into_inner();
    verifier_fiche_existe(pool.get_ref(), fiche_id).await?;

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom est requis".into()));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.secteur_developpement (id, fiche_pays_id, nom, description)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(id).bind(fiche_id).bind(nom)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .execute(pool.get_ref()).await?;

    log::info!("Admin {} a cree le secteur {} pour fiche {}", admin.id, id, fiche_id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/profils-pays/{id}/secteurs/{secteur_id}
pub async fn modifier_secteur(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ModifierSecteurRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (fiche_id, secteur_id) = path.into_inner();

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref nom) = body.nom {
        sets.push(format!("nom = ${}", bind_index));
        bind_strings.push(nom.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref desc) = body.description {
        sets.push(format!("description = ${}", bind_index));
        bind_strings.push(desc.trim().to_string());
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    let sql = format!(
        "UPDATE country_profile.secteur_developpement SET {} WHERE id = ${} AND fiche_pays_id = ${}",
        sets.join(", "), bind_index, bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(secteur_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Secteur non trouve".into()));
    }

    log::info!("Admin {} a modifie le secteur {}", admin.id, secteur_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": secteur_id })),
        error: None,
    }))
}

/// DELETE /api/admin/profils-pays/{id}/secteurs/{secteur_id}
pub async fn supprimer_secteur(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "supprimer");
    let (fiche_id, secteur_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM country_profile.secteur_developpement WHERE id = $1 AND fiche_pays_id = $2"
    ).bind(secteur_id).bind(fiche_id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Secteur non trouve".into()));
    }

    log::info!("Admin {} a supprime le secteur {}", admin.id, secteur_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// ── Saisons ──────────────────────────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays/{id}/saisons
pub async fn lister_saisons(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let fiche_id = path.into_inner();

    let items = sqlx::query_as::<_, AdminSaisonResponse>(
        "SELECT id, fiche_pays_id, nom, description, mois_debut, mois_fin, created_at
         FROM country_profile.saison WHERE fiche_pays_id = $1 ORDER BY mois_debut"
    ).bind(fiche_id).fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(items), error: None }))
}

/// POST /api/admin/profils-pays/{id}/saisons
pub async fn creer_saison(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerSaisonRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let fiche_id = path.into_inner();
    verifier_fiche_existe(pool.get_ref(), fiche_id).await?;

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom est requis".into()));
    }

    if let Some(m) = body.mois_debut {
        if !(1..=12).contains(&m) {
            return Err(ApiErreur::Validation("mois_debut doit etre entre 1 et 12".into()));
        }
    }
    if let Some(m) = body.mois_fin {
        if !(1..=12).contains(&m) {
            return Err(ApiErreur::Validation("mois_fin doit etre entre 1 et 12".into()));
        }
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.saison (id, fiche_pays_id, nom, description, mois_debut, mois_fin)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(id).bind(fiche_id).bind(nom)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.mois_debut)
    .bind(body.mois_fin)
    .execute(pool.get_ref()).await?;

    log::info!("Admin {} a cree la saison {} pour fiche {}", admin.id, id, fiche_id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/profils-pays/{id}/saisons/{saison_id}
pub async fn modifier_saison(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ModifierSaisonRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (fiche_id, saison_id) = path.into_inner();

    if let Some(m) = body.mois_debut {
        if !(1..=12).contains(&m) {
            return Err(ApiErreur::Validation("mois_debut doit etre entre 1 et 12".into()));
        }
    }
    if let Some(m) = body.mois_fin {
        if !(1..=12).contains(&m) {
            return Err(ApiErreur::Validation("mois_fin doit etre entre 1 et 12".into()));
        }
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref nom) = body.nom {
        sets.push(format!("nom = ${}", bind_index));
        bind_strings.push(nom.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref desc) = body.description {
        sets.push(format!("description = ${}", bind_index));
        bind_strings.push(desc.trim().to_string());
        bind_index += 1;
    }
    if let Some(v) = body.mois_debut {
        sets.push(format!("mois_debut = {}", v));
    }
    if let Some(v) = body.mois_fin {
        sets.push(format!("mois_fin = {}", v));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    let sql = format!(
        "UPDATE country_profile.saison SET {} WHERE id = ${} AND fiche_pays_id = ${}",
        sets.join(", "), bind_index, bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(saison_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Saison non trouvee".into()));
    }

    log::info!("Admin {} a modifie la saison {}", admin.id, saison_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": saison_id })),
        error: None,
    }))
}

/// DELETE /api/admin/profils-pays/{id}/saisons/{saison_id}
pub async fn supprimer_saison(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "supprimer");
    let (fiche_id, saison_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM country_profile.saison WHERE id = $1 AND fiche_pays_id = $2"
    ).bind(saison_id).bind(fiche_id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Saison non trouvee".into()));
    }

    log::info!("Admin {} a supprime la saison {}", admin.id, saison_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// ── Liens interethniques ─────────────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays/{id}/liens-interethniques
pub async fn lister_liens_interethniques(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let fiche_id = path.into_inner();

    let items = sqlx::query_as::<_, AdminLienInterethniqueResponse>(
        "SELECT li.id, li.fiche_pays_id, li.pays_lie_id, p.nom AS pays_lie_nom,
                li.description, li.type_lien, li.created_at, li.updated_at
         FROM country_profile.lien_interethnique li
         LEFT JOIN shared.pays p ON li.pays_lie_id = p.id
         WHERE li.fiche_pays_id = $1 ORDER BY li.type_lien"
    ).bind(fiche_id).fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(items), error: None }))
}

/// POST /api/admin/profils-pays/{id}/liens-interethniques
pub async fn creer_lien_interethnique(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerLienInterethniqueRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let fiche_id = path.into_inner();
    verifier_fiche_existe(pool.get_ref(), fiche_id).await?;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.lien_interethnique
         (id, fiche_pays_id, pays_lie_id, description, type_lien)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id).bind(fiche_id)
    .bind(body.pays_lie_id)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.type_lien.as_deref().map(|s| s.trim()))
    .execute(pool.get_ref()).await?;

    log::info!("Admin {} a cree le lien interethnique {} pour fiche {}", admin.id, id, fiche_id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/profils-pays/{id}/liens-interethniques/{lien_id}
pub async fn modifier_lien_interethnique(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ModifierLienInterethniqueRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (fiche_id, lien_id) = path.into_inner();

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref desc) = body.description {
        sets.push(format!("description = ${}", bind_index));
        bind_strings.push(desc.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref tl) = body.type_lien {
        sets.push(format!("type_lien = ${}", bind_index));
        bind_strings.push(tl.trim().to_string());
        bind_index += 1;
    }
    if let Some(pays_id) = body.pays_lie_id {
        sets.push(format!("pays_lie_id = '{}'", pays_id));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }
    sets.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE country_profile.lien_interethnique SET {} WHERE id = ${} AND fiche_pays_id = ${}",
        sets.join(", "), bind_index, bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(lien_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Lien interethnique non trouve".into()));
    }

    log::info!("Admin {} a modifie le lien interethnique {}", admin.id, lien_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": lien_id })),
        error: None,
    }))
}

/// DELETE /api/admin/profils-pays/{id}/liens-interethniques/{lien_id}
pub async fn supprimer_lien_interethnique(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "supprimer");
    let (fiche_id, lien_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM country_profile.lien_interethnique WHERE id = $1 AND fiche_pays_id = $2"
    ).bind(lien_id).bind(fiche_id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Lien interethnique non trouve".into()));
    }

    log::info!("Admin {} a supprime le lien interethnique {}", admin.id, lien_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ══════════════════════════════════════════════════════════════
// ── Contributions collaboratives ─────────────────────────────
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/profils-pays/contributions
pub async fn lister_contributions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminContributionQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).min(100);
    let offset = (page - 1) * par_page;

    let mut conditions = vec!["cf.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;
    let mut bind_types: Vec<&str> = Vec::new();

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("cf.etat::TEXT = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(fiche_id) = params.fiche_pays_id {
        conditions.push(format!("cf.fiche_pays_id = ${}", bind_index));
        bind_uuids.push(fiche_id);
        bind_types.push("uuid");
        bind_index += 1;
    }

    if let Some(cree_par) = params.cree_par {
        conditions.push(format!("cf.cree_par = ${}", bind_index));
        bind_uuids.push(cree_par);
        bind_types.push("uuid");
        bind_index += 1;
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let joins = "JOIN country_profile.fiche_pays fp ON cf.fiche_pays_id = fp.id
                 JOIN shared.pays p ON fp.pays_id = p.id
                 LEFT JOIN iam.utilisateur uc ON cf.cree_par = uc.id
                 LEFT JOIN iam.utilisateur ut ON cf.traite_par = ut.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM country_profile.contribution_fiche cf {} WHERE {}",
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
        "SELECT cf.id, cf.fiche_pays_id, p.nom AS pays_nom,
                cf.section, cf.type_contribution::TEXT AS type_contribution,
                cf.etat::TEXT AS etat,
                uc.nom || ' ' || uc.prenom AS contributeur_nom,
                ut.nom || ' ' || ut.prenom AS traite_par_nom,
                cf.created_at, cf.traite_at
         FROM country_profile.contribution_fiche cf {} WHERE {}
         ORDER BY cf.created_at DESC LIMIT {} OFFSET {}",
        joins, where_clause, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminContributionListeResponse>(&select_sql);
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

/// GET /api/admin/profils-pays/contributions/{id}
pub async fn obtenir_contribution(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let id = path.into_inner();

    let row = sqlx::query_as::<_, AdminContributionDetailRow>(
        "SELECT cf.id, cf.fiche_pays_id, p.nom AS pays_nom,
                cf.section, cf.type_contribution::TEXT AS type_contribution,
                cf.ancienne_valeur, cf.nouvelle_valeur, cf.justification,
                cf.etat::TEXT AS etat,
                cf.cree_par, uc.nom || ' ' || uc.prenom AS contributeur_nom,
                cf.traite_par, ut.nom || ' ' || ut.prenom AS traite_par_nom,
                cf.note_moderation, cf.traite_at,
                cf.created_at, cf.updated_at
         FROM country_profile.contribution_fiche cf
         JOIN country_profile.fiche_pays fp ON cf.fiche_pays_id = fp.id
         JOIN shared.pays p ON fp.pays_id = p.id
         LEFT JOIN iam.utilisateur uc ON cf.cree_par = uc.id
         LEFT JOIN iam.utilisateur ut ON cf.traite_par = ut.id
         WHERE cf.id = $1 AND cf.deleted_at IS NULL"
    ).bind(id).fetch_optional(pool.get_ref()).await?
    .ok_or_else(|| ApiErreur::NonTrouve("Contribution non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(row), error: None }))
}

/// PATCH /api/admin/profils-pays/contributions/{id}/etat
pub async fn moderer_contribution(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModererContributionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_CONTRIBUTION_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide: {}. Valeurs possibles: {:?}", etat, ETATS_CONTRIBUTION_VALIDES
        )));
    }

    let result = sqlx::query(
        "UPDATE country_profile.contribution_fiche
         SET etat = $1::country_profile.etat_contribution,
             traite_par = $2, note_moderation = $3, traite_at = NOW(), updated_at = NOW()
         WHERE id = $4 AND deleted_at IS NULL AND etat = 'en_attente'"
    )
    .bind(etat)
    .bind(admin.id)
    .bind(body.note_moderation.as_deref().map(|s| s.trim()))
    .bind(id)
    .execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Contribution non trouvee ou deja traitee".into()));
    }

    log::info!("Admin {} a {} la contribution {}", admin.id, etat, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": etat })),
        error: None,
    }))
}

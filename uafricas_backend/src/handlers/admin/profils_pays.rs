use actix_web::{HttpRequest, HttpResponse, web};
use serde_json::{Value, json};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::ApiResponse;
use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::profils_pays::{
    ADMIN_FICHE_PAYS_DETAIL_COLONNES, ADMIN_FICHE_PAYS_LISTE_COLONNES, AdminAllianceResponse,
    AdminConteQueryParams, AdminConteResponse, AdminContributionConcurrente,
    AdminContributionDetailResponse, AdminContributionDetailRow, AdminContributionListeResponse,
    AdminContributionPieceJointe, AdminContributionQueryParams, AdminFichePayDetailRow,
    AdminFichePayListeResponse, AdminFichePayQueryParams, AdminGroupeEthniqueResponse,
    AdminLienInterethniqueResponse, AdminRegionResponse, AdminSaisonResponse, AdminSecteurResponse,
    AdminSiteTouristiqueResponse, CreerAllianceRequest, CreerConteRequest, CreerFichePayRequest,
    CreerGroupeEthniqueRequest, CreerLienInterethniqueRequest, CreerRegionRequest,
    CreerSaisonRequest, CreerSecteurRequest, CreerSiteTouristiqueRequest, FICHE_PAYS_TRI_COLONNES,
    MasquerAvisBody, ModererContributionRequest, ModifierAllianceRequest, ModifierConteRequest,
    ModifierFichePayRequest, ModifierGroupeEthniqueRequest, ModifierLienInterethniqueRequest,
    ModifierRegionRequest, ModifierSaisonRequest, ModifierSecteurRequest,
    ModifierSiteTouristiqueRequest, RetirerContributionRequest, SousEntiteCounts,
    VerificationSiteBody,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;

const TYPES_CONTE_VALIDES: &[&str] = &["conte", "histoire_drole", "legende", "mythe"];
const ETATS_CONTRIBUTION_VALIDES: &[&str] = &["approuvee", "rejetee"];
const TYPES_OBJET_AFRIPULSE: &[&str] = &[
    "fiche_pays",
    "site_touristique",
    "secteur_developpement",
    "personnalite_connue",
    "savoir_pratique",
    "recommandation_visiteur",
    "photo_visiteur",
];
const SECTIONS_AFRIPULSE_VALIDES: &[&str] = &[
    "sites_emblematiques",
    "sites_prives",
    "secteurs_opportunites",
    "personnalites",
    "savoir_avant_voyager",
    "recommandations",
    "galerie_photos",
];

/// Verifie qu'une fiche pays existe et retourne son ID
async fn verifier_fiche_existe(pool: &PgPool, fiche_id: Uuid) -> Result<(), ApiErreur> {
    let existe: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM country_profile.fiche_pays WHERE id = $1)")
            .bind(fiche_id)
            .fetch_one(pool)
            .await?;
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
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM country_profile.fiche_pays fp {} WHERE {} ORDER BY fp.{} {} LIMIT {} OFFSET {}",
        ADMIN_FICHE_PAYS_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminFichePayListeResponse>(&select_sql);
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
            "SELECT COUNT(*) FROM country_profile.region WHERE fiche_pays_id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?,
        nb_groupes_ethniques: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.groupe_ethnique WHERE fiche_pays_id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?,
        nb_alliances: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.alliance_interethnique WHERE fiche_pays_id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?,
        nb_contes: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.conte_histoire WHERE fiche_pays_id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?,
        nb_sites_touristiques: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.site_touristique WHERE fiche_pays_id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?,
        nb_secteurs: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.secteur_developpement WHERE fiche_pays_id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?,
        nb_saisons: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.saison WHERE fiche_pays_id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?,
        nb_liens_interethniques: sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM country_profile.lien_interethnique WHERE fiche_pays_id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?,
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
    let pays_existe: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM shared.pays WHERE id = $1)")
            .bind(body.pays_id)
            .fetch_one(pool.get_ref())
            .await?;
    if !pays_existe {
        return Err(ApiErreur::Validation("Pays non trouve".into()));
    }

    // Verifier unicite (1 fiche par pays)
    let deja_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM country_profile.fiche_pays WHERE pays_id = $1)",
    )
    .bind(body.pays_id)
    .fetch_one(pool.get_ref())
    .await?;
    if deja_existe {
        return Err(ApiErreur::Validation(
            "Une fiche existe deja pour ce pays".into(),
        ));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.fiche_pays
         (id, pays_id, slogan, superficie_km2, population, biographie, contexte,
          contexte_historique, image_couverture_url, image_drapeau_url, image_embleme_url,
          image_devise_url, hymne_national, langue_officielle, langues_populaires,
          monnaie, fuseau_horaire, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)",
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

    log::info!(
        "Admin {} a cree la fiche pays {} pour pays {}",
        admin.id,
        id,
        body.pays_id
    );

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
        sets.join(", "),
        bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
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

    let result = sqlx::query("DELETE FROM country_profile.fiche_pays WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Fiche pays non trouvee".into()));
    }

    log::info!("Admin {} a supprime la fiche pays {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// PATCH /api/admin/profils-pays/{id}/debloquer
/// Débloque une fiche bloquée par signalements communautaires (seuil atteint) :
/// remet `bloquee=false`, purge les signalements et remet le compteur à zéro
/// (ardoise vierge : sinon un seul nouveau signalement la re-bloquerait aussitôt).
pub async fn debloquer_fiche_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let id = path.into_inner();

    // État courant (pour l'audit + 404 si inexistante).
    let avant: Option<(bool, i32)> = sqlx::query_as(
        "SELECT bloquee, nombre_signalements FROM country_profile.fiche_pays WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (etait_bloquee, nb_avant) =
        avant.ok_or_else(|| ApiErreur::NonTrouve("Fiche pays non trouvee".into()))?;

    // Purge des signalements + déblocage (ardoise vierge).
    sqlx::query("DELETE FROM country_profile.signalement_fiche WHERE fiche_pays_id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;
    sqlx::query(
        "UPDATE country_profile.fiche_pays
         SET bloquee = FALSE, nombre_signalements = 0 WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DEBLOCAGE",
        "country_profile",
        "fiche_pays",
        Some(id),
        Some(json!({ "bloquee": etait_bloquee, "nombre_signalements": nb_avant })),
        Some(json!({ "bloquee": false, "nombre_signalements": 0 })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    log::info!("Admin {} a debloque la fiche pays {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({ "id": id, "bloquee": false, "nombre_signalements": 0 })),
        error: None,
    }))
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
         FROM country_profile.region WHERE fiche_pays_id = $1 ORDER BY nom",
    )
    .bind(fiche_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
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

    log::info!(
        "Admin {} a cree la region {} pour fiche {}",
        admin.id,
        id,
        fiche_id
    );

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
        sets.join(", "),
        bind_index,
        bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
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

    let result =
        sqlx::query("DELETE FROM country_profile.region WHERE id = $1 AND fiche_pays_id = $2")
            .bind(region_id)
            .bind(fiche_id)
            .execute(pool.get_ref())
            .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Region non trouvee".into()));
    }

    log::info!("Admin {} a supprime la region {}", admin.id, region_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
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
         WHERE ge.fiche_pays_id = $1 ORDER BY ge.nom",
    )
    .bind(fiche_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
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
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(id)
    .bind(fiche_id)
    .bind(nom)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(
        body.objets_culturels_distinctifs
            .as_deref()
            .map(|s| s.trim()),
    )
    .bind(body.population_estimee.as_deref().map(|s| s.trim()))
    .bind(body.langues.as_deref().map(|s| s.trim()))
    .bind(body.region_id)
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a cree le groupe ethnique {} pour fiche {}",
        admin.id,
        id,
        fiche_id
    );

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
    champ_str!(
        body.objets_culturels_distinctifs,
        "objets_culturels_distinctifs"
    );
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
        sets.join(", "),
        bind_index,
        bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
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
        "DELETE FROM country_profile.groupe_ethnique WHERE id = $1 AND fiche_pays_id = $2",
    )
    .bind(ge_id)
    .bind(fiche_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Groupe ethnique non trouve".into()));
    }

    log::info!("Admin {} a supprime le groupe ethnique {}", admin.id, ge_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
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
         FROM country_profile.alliance_interethnique WHERE fiche_pays_id = $1 ORDER BY nom",
    )
    .bind(fiche_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
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
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(fiche_id)
    .bind(nom)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.groupes_impliques.as_deref().map(|s| s.trim()))
    .bind(body.signification.as_deref().map(|s| s.trim()))
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a cree l'alliance {} pour fiche {}",
        admin.id,
        id,
        fiche_id
    );

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
        sets.join(", "),
        bind_index,
        bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
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
        "DELETE FROM country_profile.alliance_interethnique WHERE id = $1 AND fiche_pays_id = $2",
    )
    .bind(alliance_id)
    .bind(fiche_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Alliance non trouvee".into()));
    }

    log::info!("Admin {} a supprime l'alliance {}", admin.id, alliance_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
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
    for v in &bind_values {
        q = q.bind(v);
    }
    let items = q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
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
                "Type invalide: {}. Valeurs possibles: {:?}",
                t, TYPES_CONTE_VALIDES
            )));
        }
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.conte_histoire
         (id, fiche_pays_id, titre, contenu, type, groupe_ethnique_id, image_url)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(id)
    .bind(fiche_id)
    .bind(titre)
    .bind(body.contenu.as_deref().map(|s| s.trim()))
    .bind(body.type_conte.as_deref())
    .bind(body.groupe_ethnique_id)
    .bind(body.image_url.as_deref().map(|s| s.trim()))
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a cree le conte {} pour fiche {}",
        admin.id,
        id,
        fiche_id
    );

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
                "Type invalide: {}. Valeurs possibles: {:?}",
                t, TYPES_CONTE_VALIDES
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
        sets.join(", "),
        bind_index,
        bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
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
        "DELETE FROM country_profile.conte_histoire WHERE id = $1 AND fiche_pays_id = $2",
    )
    .bind(conte_id)
    .bind(fiche_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Conte non trouve".into()));
    }

    log::info!("Admin {} a supprime le conte {}", admin.id, conte_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
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
        "SELECT st.id, st.fiche_pays_id, st.nom,
                st.categorie::text AS categorie, st.sous_type::text AS sous_type,
                st.description, st.info_pertinente, st.image_url, st.images,
                st.gestionnaire, st.ville, st.village,
                st.longitude::float8 AS longitude, st.latitude::float8 AS latitude,
                st.contact_telephone, st.contact_courriel, st.contact_adresse,
                st.constitution_statut_juridique, st.constitution_numero, st.constitution_document_url,
                st.site_web_url, st.verifie,
                st.region_id, r.nom AS region_nom,
                st.created_at, st.updated_at
         FROM country_profile.site_touristique st
         LEFT JOIN country_profile.region r ON st.region_id = r.id
         WHERE st.fiche_pays_id = $1 AND st.deleted_at IS NULL ORDER BY st.nom"
    ).bind(fiche_id).fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
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

    let categorie = body.categorie.as_deref().unwrap_or("emblematique");
    // Galerie (≤5) + couverture image_url = 1re image (rétrocompat).
    let images: Vec<String> = body
        .images
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .take(5)
        .collect();
    let cover = images
        .first()
        .map(|s| s.as_str())
        .or_else(|| body.image_url.as_deref().map(|s| s.trim()));
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO country_profile.site_touristique
         (id, fiche_pays_id, nom, categorie, sous_type, description, info_pertinente, image_url,
          gestionnaire, ville, village, longitude, latitude,
          contact_telephone, contact_courriel, contact_adresse,
          constitution_statut_juridique, constitution_numero, constitution_document_url, region_id, images,
          site_web_url)
         VALUES ($1, $2, $3, $4::country_profile.categorie_site_touristique,
                 $5::country_profile.sous_type_site, $6, $7, $8,
                 $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)",
    )
    .bind(id)
    .bind(fiche_id)
    .bind(nom)
    .bind(categorie)
    .bind(
        body.sous_type
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty()),
    )
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.info_pertinente.as_deref().map(|s| s.trim()))
    .bind(cover)
    .bind(body.gestionnaire.as_deref().map(|s| s.trim()))
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(body.village.as_deref().map(|s| s.trim()))
    .bind(body.longitude)
    .bind(body.latitude)
    .bind(body.contact_telephone.as_deref().map(|s| s.trim()))
    .bind(body.contact_courriel.as_deref().map(|s| s.trim()))
    .bind(body.contact_adresse.as_deref().map(|s| s.trim()))
    .bind(
        body.constitution_statut_juridique
            .as_deref()
            .map(|s| s.trim()),
    )
    .bind(body.constitution_numero.as_deref().map(|s| s.trim()))
    .bind(body.constitution_document_url.as_deref().map(|s| s.trim()))
    .bind(body.region_id)
    .bind(&images)
    .bind(
        body.site_web_url
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty()),
    )
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a cree le site touristique {} pour fiche {}",
        admin.id,
        id,
        fiche_id
    );

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

    // Variante avec cast d'enum SQL sur le placeholder.
    macro_rules! champ_str_cast {
        ($field:expr, $col:expr, $cast:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}{}", $col, bind_index, $cast));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    champ_str!(body.nom, "nom");
    champ_str_cast!(
        body.categorie,
        "categorie",
        "::country_profile.categorie_site_touristique"
    );
    champ_str_cast!(
        body.sous_type,
        "sous_type",
        "::country_profile.sous_type_site"
    );
    champ_str!(body.description, "description");
    champ_str!(body.info_pertinente, "info_pertinente");
    champ_str!(body.image_url, "image_url");
    champ_str!(body.gestionnaire, "gestionnaire");
    champ_str!(body.ville, "ville");
    champ_str!(body.village, "village");
    champ_str!(body.contact_telephone, "contact_telephone");
    champ_str!(body.contact_courriel, "contact_courriel");
    champ_str!(body.contact_adresse, "contact_adresse");
    champ_str!(
        body.constitution_statut_juridique,
        "constitution_statut_juridique"
    );
    champ_str!(body.constitution_numero, "constitution_numero");
    champ_str!(body.constitution_document_url, "constitution_document_url");
    champ_str!(body.site_web_url, "site_web_url");

    if let Some(v) = body.longitude {
        sets.push(format!("longitude = {}", v));
    }
    if let Some(v) = body.latitude {
        sets.push(format!("latitude = {}", v));
    }
    if let Some(region_id) = body.region_id {
        sets.push(format!("region_id = '{}'", region_id));
    }

    // Galerie (≤5) : si fournie, on remplace `images` et la couverture `image_url`
    // (sauf si image_url explicitement fourni ci-dessus).
    let images_vec: Option<Vec<String>> = body.images.as_ref().map(|v| {
        v.iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .take(5)
            .collect()
    });
    if let Some(ref imgs) = images_vec {
        if body.image_url.is_none() {
            if let Some(cover) = imgs.first() {
                sets.push(format!("image_url = ${}", bind_index));
                bind_strings.push(cover.clone());
                bind_index += 1;
            }
        }
        sets.push(format!("images = ${}::text[]", bind_index));
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }
    sets.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE country_profile.site_touristique SET {} WHERE id = ${} AND fiche_pays_id = ${}",
        sets.join(", "),
        bind_index,
        bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
    if let Some(ref imgs) = images_vec {
        q = q.bind(imgs);
    }
    q = q.bind(site_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Site touristique non trouve".into()));
    }

    log::info!(
        "Admin {} a modifie le site touristique {}",
        admin.id,
        site_id
    );

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
        "DELETE FROM country_profile.site_touristique WHERE id = $1 AND fiche_pays_id = $2",
    )
    .bind(site_id)
    .bind(fiche_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Site touristique non trouve".into()));
    }

    log::info!(
        "Admin {} a supprime le site touristique {}",
        admin.id,
        site_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// PATCH /api/admin/profils-pays/{id}/sites-touristiques/{site_id}/verification
///
/// Active ou retire le badge « Vérifié » d'un site (US3, réservé admin, FR-012).
/// Journalise l'avant/après via `audit::log_action` (Principe VII).
pub async fn definir_verification_site(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<VerificationSiteBody>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (fiche_id, site_id) = path.into_inner();

    // État courant (pour l'audit + 404 si inexistant/supprimé).
    let avant: Option<bool> = sqlx::query_scalar(
        "SELECT verifie FROM country_profile.site_touristique
         WHERE id = $1 AND fiche_pays_id = $2 AND deleted_at IS NULL",
    )
    .bind(site_id)
    .bind(fiche_id)
    .fetch_optional(pool.get_ref())
    .await?;
    let avant = avant.ok_or_else(|| ApiErreur::NonTrouve("Site touristique non trouvé".into()))?;

    let verifie_at: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar(
        "UPDATE country_profile.site_touristique
         SET verifie = $3,
             verifie_par = CASE WHEN $3 THEN $4 ELSE NULL END,
             verifie_at = CASE WHEN $3 THEN NOW() ELSE NULL END
         WHERE id = $1 AND fiche_pays_id = $2 AND deleted_at IS NULL
         RETURNING verifie_at",
    )
    .bind(site_id)
    .bind(fiche_id)
    .bind(body.verifie)
    .bind(admin.id)
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "update",
        "country_profile",
        "site_touristique",
        Some(site_id),
        Some(json!({ "verifie": avant })),
        Some(json!({ "verifie": body.verifie })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({ "id": site_id, "verifie": body.verifie, "verifie_at": verifie_at })),
        error: None,
    }))
}

/// PATCH /api/admin/sites-touristiques/avis/{avis_id}/masquer
///
/// Masque ou réaffiche un avis inapproprié (US5, FR-015d). Audité.
pub async fn masquer_avis_site(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<MasquerAvisBody>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let avis_id = path.into_inner();

    let avant: Option<bool> = sqlx::query_scalar(
        "SELECT masque_par_admin FROM country_profile.avis_site
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(avis_id)
    .fetch_optional(pool.get_ref())
    .await?;
    let avant = avant.ok_or_else(|| ApiErreur::NonTrouve("Avis non trouvé".into()))?;

    sqlx::query(
        "UPDATE country_profile.avis_site SET masque_par_admin = $2
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(avis_id)
    .bind(body.masque)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "update",
        "country_profile",
        "avis_site",
        Some(avis_id),
        Some(json!({ "masque_par_admin": avant })),
        Some(json!({ "masque_par_admin": body.masque })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({ "id": avis_id, "masque": body.masque })),
        error: None,
    }))
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
         FROM country_profile.secteur_developpement WHERE fiche_pays_id = $1 ORDER BY nom",
    )
    .bind(fiche_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
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
         VALUES ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(fiche_id)
    .bind(nom)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a cree le secteur {} pour fiche {}",
        admin.id,
        id,
        fiche_id
    );

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
        sets.join(", "),
        bind_index,
        bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
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
        "DELETE FROM country_profile.secteur_developpement WHERE id = $1 AND fiche_pays_id = $2",
    )
    .bind(secteur_id)
    .bind(fiche_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Secteur non trouve".into()));
    }

    log::info!("Admin {} a supprime le secteur {}", admin.id, secteur_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
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
         FROM country_profile.saison WHERE fiche_pays_id = $1 ORDER BY mois_debut",
    )
    .bind(fiche_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
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
            return Err(ApiErreur::Validation(
                "mois_debut doit etre entre 1 et 12".into(),
            ));
        }
    }
    if let Some(m) = body.mois_fin {
        if !(1..=12).contains(&m) {
            return Err(ApiErreur::Validation(
                "mois_fin doit etre entre 1 et 12".into(),
            ));
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

    log::info!(
        "Admin {} a cree la saison {} pour fiche {}",
        admin.id,
        id,
        fiche_id
    );

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
            return Err(ApiErreur::Validation(
                "mois_debut doit etre entre 1 et 12".into(),
            ));
        }
    }
    if let Some(m) = body.mois_fin {
        if !(1..=12).contains(&m) {
            return Err(ApiErreur::Validation(
                "mois_fin doit etre entre 1 et 12".into(),
            ));
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
        sets.join(", "),
        bind_index,
        bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
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

    let result =
        sqlx::query("DELETE FROM country_profile.saison WHERE id = $1 AND fiche_pays_id = $2")
            .bind(saison_id)
            .bind(fiche_id)
            .execute(pool.get_ref())
            .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Saison non trouvee".into()));
    }

    log::info!("Admin {} a supprime la saison {}", admin.id, saison_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
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
         WHERE li.fiche_pays_id = $1 ORDER BY li.type_lien",
    )
    .bind(fiche_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
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
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(id)
    .bind(fiche_id)
    .bind(body.pays_lie_id)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.type_lien.as_deref().map(|s| s.trim()))
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a cree le lien interethnique {} pour fiche {}",
        admin.id,
        id,
        fiche_id
    );

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
        sets.join(", "),
        bind_index,
        bind_index + 1
    );
    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
    q = q.bind(lien_id).bind(fiche_id);
    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Lien interethnique non trouve".into()));
    }

    log::info!(
        "Admin {} a modifie le lien interethnique {}",
        admin.id,
        lien_id
    );

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
        "DELETE FROM country_profile.lien_interethnique WHERE id = $1 AND fiche_pays_id = $2",
    )
    .bind(lien_id)
    .bind(fiche_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Lien interethnique non trouve".into()));
    }

    log::info!(
        "Admin {} a supprime le lien interethnique {}",
        admin.id,
        lien_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
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

    // T039 : filtres Afripulse (type_objet, section)
    if let Some(ref type_objet) = params.type_objet {
        let t = type_objet.trim().to_lowercase();
        if TYPES_OBJET_AFRIPULSE.contains(&t.as_str()) {
            conditions.push(format!(
                "cf.type_objet_contribution::TEXT = ${}",
                bind_index
            ));
            bind_values.push(t);
            bind_types.push("str");
            bind_index += 1;
        }
    }
    if let Some(ref section) = params.section {
        let s = section.trim().to_lowercase();
        if SECTIONS_AFRIPULSE_VALIDES.contains(&s.as_str()) {
            conditions.push(format!("cf.section_afripulse::TEXT = ${}", bind_index));
            bind_values.push(s);
            bind_types.push("str");
            bind_index += 1;
        }
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
        if *t == "str" {
            count_q = count_q.bind(&bind_values[str_idx]);
            str_idx += 1;
        } else {
            count_q = count_q.bind(bind_uuids[uuid_idx]);
            uuid_idx += 1;
        }
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
    str_idx = 0;
    uuid_idx = 0;
    for t in &bind_types {
        if *t == "str" {
            select_q = select_q.bind(&bind_values[str_idx]);
            str_idx += 1;
        } else {
            select_q = select_q.bind(bind_uuids[uuid_idx]);
            uuid_idx += 1;
        }
    }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/profils-pays/contributions/{id}
///
/// T040 : renvoie le diff structure (JSONB ancienne/nouvelle) + pieces jointes
/// + contributions concurrentes (memes fiche_pays/type_objet/target_id en
/// attente). Aligne sur `AdminContributionDetailResponse`.
pub async fn obtenir_contribution(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");
    let id = path.into_inner();

    // Ligne principale (colonnes legacy + base)
    let base = sqlx::query_as::<_, AdminContributionDetailRow>(
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
         WHERE cf.id = $1 AND cf.deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Contribution non trouvee".into()))?;

    // Colonnes Afripulse (JSONB + type_objet + section_afripulse + target_id + pieces_jointes)
    let (type_objet, section_afripulse, target_id, nouvelle_jsonb, ancienne_jsonb, pieces_json): (
        String,
        Option<String>,
        Option<Uuid>,
        Option<Value>,
        Option<Value>,
        Value,
    ) = sqlx::query_as(
        "SELECT cf.type_objet_contribution::TEXT,
                cf.section_afripulse::TEXT,
                cf.target_id,
                cf.nouvelle_valeur_jsonb,
                cf.ancienne_valeur_jsonb,
                cf.pieces_jointes
         FROM country_profile.contribution_fiche cf WHERE cf.id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    // Pieces jointes : deserialiser le JSONB et projeter url_signee
    let pieces_jointes = parse_pieces_jointes(&pieces_json);

    // Contributions concurrentes : memes (fiche_pays, type_objet, target_id) en attente
    // exclure l'ID courant.
    let concurrentes = sqlx::query_as::<_, AdminContributionConcurrente>(
        "SELECT cf.id,
                uc.nom || ' ' || uc.prenom AS cree_par_nom,
                cf.created_at
         FROM country_profile.contribution_fiche cf
         LEFT JOIN iam.utilisateur uc ON cf.cree_par = uc.id
         WHERE cf.id <> $1
           AND cf.fiche_pays_id = $2
           AND cf.type_objet_contribution::text = $3
           AND cf.etat = 'en_attente'::country_profile.etat_contribution
           AND cf.deleted_at IS NULL
           AND (
               ($4::uuid IS NULL AND cf.target_id IS NULL)
               OR cf.target_id = $4::uuid
           )
         ORDER BY cf.created_at DESC",
    )
    .bind(id)
    .bind(base.fiche_pays_id)
    .bind(&type_objet)
    .bind(target_id)
    .fetch_all(pool.get_ref())
    .await?;

    let response = AdminContributionDetailResponse {
        base,
        type_objet_contribution: type_objet,
        section_afripulse,
        target_id,
        nouvelle_valeur_jsonb: nouvelle_jsonb,
        ancienne_valeur_jsonb: ancienne_jsonb,
        pieces_jointes,
        contributions_concurrentes: concurrentes,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

/// Desserialise le JSONB `pieces_jointes` d'une contribution photo_visiteur
/// en `Vec<AdminContributionPieceJointe>` avec URL signee prete a servir.
fn parse_pieces_jointes(json: &Value) -> Vec<AdminContributionPieceJointe> {
    let arr = match json.as_array() {
        Some(a) => a,
        None => return Vec::new(),
    };
    arr.iter()
        .filter_map(|item| {
            let obj = item.as_object()?;
            let chemin = obj.get("chemin_fichier")?.as_str()?.to_string();
            let legende = obj
                .get("legende")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let format = obj
                .get("format")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let taille = obj
                .get("taille_octets")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let largeur = obj.get("largeur").and_then(|v| v.as_i64()).unwrap_or(0);
            let hauteur = obj.get("hauteur").and_then(|v| v.as_i64()).unwrap_or(0);
            // chemin_fichier est deja relatif a `./uploads/` (p. ex. "opportunite-afrique/photos/<uuid>.jpg")
            let url_signee = format!("/uploads/{}", chemin.trim_start_matches('/'));
            Some(AdminContributionPieceJointe {
                chemin_fichier: chemin,
                legende,
                format,
                taille_octets: taille,
                largeur,
                hauteur,
                url_signee,
            })
        })
        .collect()
}

/// PATCH /api/admin/profils-pays/contributions/{id}/etat, T036/T037/T045
///
/// Transaction SQL UNIQUE :
///   1. UPDATE contribution_fiche SET etat, traite_par, traite_at, note_moderation
///   2. Si approuvee : apply_effect_contribution (INSERT/UPDATE/soft-DELETE sur
///      table cible selon (type_objet, type_contribution))
///   3. Si approuvee : marquer obsoletes les contributions concurrentes
///      (meme fiche_pays, meme type_objet, meme target_id, etat=en_attente)
///   4. Notifier l'auteur via arbre_genealogique.notifications
///   5. audit::log_action (hors transaction, non-bloquant)
pub async fn moderer_contribution(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModererContributionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim().to_lowercase();
    if !ETATS_CONTRIBUTION_VALIDES.contains(&etat.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide: {}. Valeurs possibles: {:?}",
            etat, ETATS_CONTRIBUTION_VALIDES
        )));
    }

    let note = body
        .note_moderation
        .as_deref()
        .map(|s| s.trim().to_string());
    if etat == "rejetee" && note.as_deref().map(str::is_empty).unwrap_or(true) {
        return Err(ApiErreur::Validation(
            "Un motif (note_moderation) est obligatoire pour rejeter une contribution.".into(),
        ));
    }

    // Charger la contribution (JSONB Afripulse + champs legacy scalaires + metadata)
    let row: (
        Uuid,
        Uuid,
        Uuid,
        String,
        String,
        Option<String>,
        Option<Uuid>,
        Option<Value>,
        Option<Value>,
        Value,
        String,
        String,
        Option<String>,
    ) = sqlx::query_as(
        "SELECT cf.id, cf.fiche_pays_id, cf.cree_par,
                cf.etat::text,
                cf.type_objet_contribution::text,
                cf.section_afripulse::text,
                cf.target_id,
                cf.nouvelle_valeur_jsonb,
                cf.ancienne_valeur_jsonb,
                cf.pieces_jointes,
                cf.type_contribution::text,
                cf.section,
                cf.nouvelle_valeur
         FROM country_profile.contribution_fiche cf
         WHERE cf.id = $1 AND cf.deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Contribution non trouvee".into()))?;

    let (
        _id,
        fiche_pays_id,
        auteur_id,
        etat_actuel,
        type_objet,
        _section_afripulse,
        target_id,
        nouvelle_jsonb,
        _ancienne_jsonb,
        pieces_json,
        type_contribution,
        section_legacy,
        nouvelle_valeur_legacy,
    ) = row;

    if etat_actuel != "en_attente" {
        return Err(ApiErreur::Validation(format!(
            "Seules les contributions en attente peuvent etre moderees (etat actuel : {}).",
            etat_actuel
        )));
    }

    let mut tx = pool.get_ref().begin().await?;

    // 1. UPDATE etat
    sqlx::query(
        "UPDATE country_profile.contribution_fiche
         SET etat = $1::country_profile.etat_contribution,
             traite_par = $2, note_moderation = $3,
             traite_at = NOW(), updated_at = NOW()
         WHERE id = $4",
    )
    .bind(&etat)
    .bind(admin.id)
    .bind(note.as_deref())
    .bind(id)
    .execute(&mut *tx)
    .await?;

    let mut target_applique: Option<Uuid> = target_id;

    if etat == "approuvee" {
        // 2. Appliquer l'effet sur la table cible.
        //    Cas legacy : contribution scalaire sur la fiche pays elle-même
        //    (population, slogan, biographie, etc.) stockée en `section` + `nouvelle_valeur`.
        if type_objet == "fiche_pays" && type_contribution == "modification" {
            appliquer_fiche_scalaire(
                &mut tx,
                fiche_pays_id,
                &section_legacy,
                nouvelle_valeur_legacy.as_deref().unwrap_or(""),
            )
            .await?;
            target_applique = None;
        } else {
            target_applique = appliquer_contribution_afripulse(
                &mut tx,
                fiche_pays_id,
                auteur_id,
                &type_objet,
                &type_contribution,
                target_id,
                nouvelle_jsonb.as_ref(),
                &pieces_json,
            )
            .await?;
        }

        // 3. Marquer obsoletes les contributions concurrentes (T036 pt. 3)
        sqlx::query(
            "UPDATE country_profile.contribution_fiche
             SET etat = 'obsolete'::country_profile.etat_contribution,
                 updated_at = NOW()
             WHERE id <> $1
               AND fiche_pays_id = $2
               AND type_objet_contribution::text = $3
               AND etat = 'en_attente'::country_profile.etat_contribution
               AND deleted_at IS NULL
               AND (
                   ($4::uuid IS NULL AND target_id IS NULL)
                   OR target_id = $4::uuid
               )",
        )
        .bind(id)
        .bind(fiche_pays_id)
        .bind(&type_objet)
        .bind(target_id)
        .execute(&mut *tx)
        .await?;
    }

    // 4. Notification a l'auteur (dans la meme transaction, FR-019)
    let (titre_notif, message_notif) =
        construire_message_notification(&etat, &type_objet, note.as_deref());
    let lien_action = format!("/mon-compte/contributions?id={}", id);
    sqlx::query(
        "INSERT INTO arbre_genealogique.notifications
            (destinataire_id, type, message, lien_action)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(auteur_id)
    .bind(&titre_notif)
    .bind(&message_notif)
    .bind(&lien_action)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // 5. Audit (non-bloquant)
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        if etat == "approuvee" {
            "approve"
        } else {
            "reject"
        },
        "country_profile",
        "contribution_fiche",
        Some(id),
        Some(json!({ "etat": "en_attente" })),
        Some(json!({
            "etat": etat,
            "type_objet": type_objet,
            "target_id": target_applique,
            "note_moderation": note,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    log::info!("Admin {} a {} la contribution {}", admin.id, etat, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({
            "id": id,
            "etat": etat,
            "target_id": target_applique,
        })),
        error: None,
    }))
}

/// POST /api/admin/profils-pays/contributions/{id}/retirer, T038/T045
///
/// Retire une contribution deja approuvee :
///   - soft-DELETE la ligne cible (table determinee par type_objet_contribution)
///   - passe la contribution en `etat='obsolete'` (reutilisation : cette valeur
///     represente "surclassee ou retiree post-approbation")
///   - exige un motif 10..1000 car. (store dans note_moderation, append)
///   - notifie l'auteur et log audit
pub async fn retirer_contribution_approuvee(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<RetirerContributionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let id = path.into_inner();

    let motif = body.motif.trim().to_string();
    if motif.chars().count() < 10 || motif.chars().count() > 1000 {
        return Err(ApiErreur::Validation(
            "Le motif doit contenir entre 10 et 1000 caracteres.".into(),
        ));
    }

    // Charger contribution approuvee
    let row: (Uuid, Uuid, String, String, Option<Uuid>, Option<Value>) = sqlx::query_as(
        "SELECT cf.id, cf.fiche_pays_id, cf.cree_par,
                cf.etat::text,
                cf.type_objet_contribution::text,
                cf.target_id,
                cf.nouvelle_valeur_jsonb
         FROM country_profile.contribution_fiche cf
         WHERE cf.id = $1 AND cf.deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Contribution non trouvee".into()))?;

    // N.B. serde_json::Value = type_objet renvoye en 4e position suite au SELECT ci-dessus
    let (_id, fiche_pays_id, auteur_id, etat_actuel, target_id_opt, _nv) = row;
    // Recharger type_objet propre (le tuple ci-dessus confond ordre; refactor)
    let type_objet: String = sqlx::query_scalar(
        "SELECT type_objet_contribution::text FROM country_profile.contribution_fiche WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    if etat_actuel != "approuvee" {
        return Err(ApiErreur::Validation(format!(
            "Seule une contribution approuvee peut etre retiree (etat actuel : {}).",
            etat_actuel
        )));
    }

    let mut tx = pool.get_ref().begin().await?;

    // Soft-delete la ligne cible si target_id present (fiche_pays/photo non couverts ici)
    if let Some(target_id) = target_id_opt {
        let table = match type_objet.as_str() {
            "site_touristique" => Some("country_profile.site_touristique"),
            "secteur_developpement" => Some("country_profile.secteur_developpement"),
            "personnalite_connue" => Some("country_profile.personnalite_connue"),
            "savoir_pratique" => Some("country_profile.savoir_pratique"),
            "recommandation_visiteur" => Some("country_profile.recommandation_visiteur"),
            "photo_visiteur" => Some("country_profile.photo_visiteur"),
            _ => None,
        };
        if let Some(t) = table {
            let sql = format!(
                "UPDATE {} SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
                t
            );
            sqlx::query(&sql).bind(target_id).execute(&mut *tx).await?;
        }
    }

    // Passer la contribution a 'obsolete'
    sqlx::query(
        "UPDATE country_profile.contribution_fiche
         SET etat = 'obsolete'::country_profile.etat_contribution,
             note_moderation = COALESCE(note_moderation || E'\n\n[Retrait] ', '[Retrait] ') || $1,
             traite_par = $2,
             traite_at = NOW(),
             updated_at = NOW()
         WHERE id = $3",
    )
    .bind(&motif)
    .bind(admin.id)
    .bind(id)
    .execute(&mut *tx)
    .await?;

    // Notification auteur
    let message_notif = format!(
        "Votre contribution a ete retiree. Motif : {}",
        if motif.chars().count() > 200 {
            format!("{}…", motif.chars().take(200).collect::<String>())
        } else {
            motif.clone()
        }
    );
    let lien_action = format!("/mon-compte/contributions?id={}", id);
    sqlx::query(
        "INSERT INTO arbre_genealogique.notifications
            (destinataire_id, type, message, lien_action)
         VALUES ($1, 'afripulse_retrait', $2, $3)",
    )
    .bind(auteur_id)
    .bind(&message_notif)
    .bind(&lien_action)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "retire",
        "country_profile",
        "contribution_fiche",
        Some(id),
        Some(json!({ "etat": "approuvee" })),
        Some(json!({
            "etat": "obsolete",
            "type_objet": type_objet,
            "target_id": target_id_opt,
            "motif": motif,
            "fiche_pays_id": fiche_pays_id,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({ "id": id, "etat": "obsolete" })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// ── Helpers application des contributions approuvees (T036/T037)
// ══════════════════════════════════════════════════════════════

/// Applique l'effet d'une contribution approuvee sur la table cible. Retourne
/// `target_id` effectivement impacte (utile pour la reponse API et l'audit).
/// Applique une contribution legacy scalaire directement sur la fiche pays
/// (champs texte/numériques de `country_profile.fiche_pays`). La colonne ciblée
/// est déterminée par `section`. Les sections inconnues sont ignorées sans erreur.
async fn appliquer_fiche_scalaire(
    tx: &mut Transaction<'_, Postgres>,
    fiche_pays_id: Uuid,
    section: &str,
    nouvelle_valeur: &str,
) -> Result<(), ApiErreur> {
    // Colonne SQL (avec cast éventuel) autorisée par section, liste fermée.
    let set_clause = match section {
        "population" => "population = $1::bigint",
        "superficie_km2" => "superficie_km2 = $1::decimal",
        "biographie" => "biographie = $1",
        "contexte" => "contexte = $1",
        "contexte_historique" => "contexte_historique = $1",
        "slogan" => "slogan = $1",
        "hymne_national" => "hymne_national = $1",
        "langue_officielle" => "langue_officielle = $1",
        "langues_populaires" => "langues_populaires = $1",
        "monnaie" => "monnaie = $1",
        "fuseau_horaire" => "fuseau_horaire = $1",
        // Bloc « À savoir avant de voyager »
        "voyage_langue_internationale" => "voyage_langue_internationale = $1",
        "voyage_langue_locale" => "voyage_langue_locale = $1",
        "voyage_infos_visa" => "voyage_infos_visa = $1",
        "voyage_infos_sanitaires" => "voyage_infos_sanitaires = $1",
        "voyage_meteo" => "voyage_meteo = $1",
        "voyage_prises_electriques" => "voyage_prises_electriques = $1",
        "voyage_contacts_tourisme" => "voyage_contacts_tourisme = $1",
        "voyage_recommandations_securite" => "voyage_recommandations_securite = $1",
        _ => return Ok(()),
    };

    let requete = format!(
        "UPDATE country_profile.fiche_pays SET {}, updated_at = NOW() WHERE id = $2",
        set_clause
    );
    sqlx::query(&requete)
        .bind(nouvelle_valeur)
        .bind(fiche_pays_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

async fn appliquer_contribution_afripulse(
    tx: &mut Transaction<'_, Postgres>,
    fiche_pays_id: Uuid,
    auteur_id: Uuid,
    type_objet: &str,
    type_contribution: &str,
    target_id: Option<Uuid>,
    nouvelle_jsonb: Option<&Value>,
    pieces_json: &Value,
) -> Result<Option<Uuid>, ApiErreur> {
    // Le type d'action provient directement de la colonne `type_contribution`
    // (fiable) : l'enum SQL utilise « modification », les branches internes « edition ».
    let type_contrib = if type_contribution == "modification" {
        "edition"
    } else {
        type_contribution
    };

    match (type_objet, type_contrib) {
        // ── Site touristique ────────────────────────────────────
        ("site_touristique", "ajout") => {
            let payload = nouvelle_jsonb.ok_or_else(|| {
                ApiErreur::Validation("nouvelle_valeur_jsonb requise pour ajout site".into())
            })?;
            // Galerie (≤3) + couverture image_url = 1re image (rétrocompat lectures).
            let images = images_site_field(payload).unwrap_or_default();
            let cover = images
                .first()
                .map(|s| s.as_str())
                .or_else(|| opt_str_field(payload, "image_url"));
            // Le badge `verifie` n'est jamais piloté par le canal de contribution (FR-012).
            let nouvel_id: Uuid = sqlx::query_scalar(
                "INSERT INTO country_profile.site_touristique
                    (fiche_pays_id, nom, categorie, sous_type, description, image_url,
                     gestionnaire, ville, village, info_pertinente, latitude, longitude,
                     contact_telephone, contact_courriel, contact_adresse,
                     constitution_statut_juridique, constitution_numero, constitution_document_url,
                     images, site_web_url)
                 VALUES ($1, $2, $3::country_profile.categorie_site_touristique,
                         $4::country_profile.sous_type_site, $5, $6,
                         $7, $8, $9, $10, $11::numeric, $12::numeric,
                         $13, $14, $15, $16, $17, $18, $19, $20)
                 RETURNING id",
            )
            .bind(fiche_pays_id)
            .bind(str_field(payload, "nom"))
            .bind(str_field(payload, "categorie"))
            .bind(opt_str_field(payload, "sous_type"))
            .bind(opt_str_field(payload, "description"))
            .bind(cover)
            .bind(opt_str_field(payload, "gestionnaire"))
            .bind(opt_str_field(payload, "ville"))
            .bind(opt_str_field(payload, "village"))
            .bind(opt_str_field(payload, "info_pertinente"))
            .bind(opt_f64_field(payload, "latitude"))
            .bind(opt_f64_field(payload, "longitude"))
            .bind(opt_str_field(payload, "contact_telephone"))
            .bind(opt_str_field(payload, "contact_courriel"))
            .bind(opt_str_field(payload, "contact_adresse"))
            .bind(opt_str_field(payload, "constitution_statut_juridique"))
            .bind(opt_str_field(payload, "constitution_numero"))
            .bind(opt_str_field(payload, "constitution_document_url"))
            .bind(&images)
            .bind(opt_str_field(payload, "site_web_url"))
            .fetch_one(&mut **tx)
            .await?;
            Ok(Some(nouvel_id))
        }
        ("site_touristique", "edition") => {
            let payload = nouvelle_jsonb.unwrap();
            let tid = target_id.unwrap();
            // Galerie fournie ? alors on remplace images + couverture ; sinon on garde.
            let images_opt = images_site_field(payload);
            let cover = images_opt
                .as_ref()
                .and_then(|v| v.first())
                .map(|s| s.as_str())
                .or_else(|| opt_str_field(payload, "image_url"));
            sqlx::query(
                "UPDATE country_profile.site_touristique SET
                    nom = COALESCE($2, nom),
                    categorie = COALESCE($3::country_profile.categorie_site_touristique, categorie),
                    sous_type = COALESCE($4::country_profile.sous_type_site, sous_type),
                    description = COALESCE($5, description),
                    image_url = COALESCE($6, image_url),
                    gestionnaire = COALESCE($7, gestionnaire),
                    ville = COALESCE($8, ville),
                    village = COALESCE($9, village),
                    info_pertinente = COALESCE($10, info_pertinente),
                    latitude = COALESCE($11::numeric, latitude),
                    longitude = COALESCE($12::numeric, longitude),
                    contact_telephone = COALESCE($13, contact_telephone),
                    contact_courriel = COALESCE($14, contact_courriel),
                    contact_adresse = COALESCE($15, contact_adresse),
                    constitution_statut_juridique = COALESCE($16, constitution_statut_juridique),
                    constitution_numero = COALESCE($17, constitution_numero),
                    constitution_document_url = COALESCE($18, constitution_document_url),
                    images = COALESCE($19::text[], images),
                    site_web_url = COALESCE($20, site_web_url)
                 WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(tid)
            .bind(opt_str_field(payload, "nom"))
            .bind(opt_str_field(payload, "categorie"))
            .bind(opt_str_field(payload, "sous_type"))
            .bind(opt_str_field(payload, "description"))
            .bind(cover)
            .bind(opt_str_field(payload, "gestionnaire"))
            .bind(opt_str_field(payload, "ville"))
            .bind(opt_str_field(payload, "village"))
            .bind(opt_str_field(payload, "info_pertinente"))
            .bind(opt_f64_field(payload, "latitude"))
            .bind(opt_f64_field(payload, "longitude"))
            .bind(opt_str_field(payload, "contact_telephone"))
            .bind(opt_str_field(payload, "contact_courriel"))
            .bind(opt_str_field(payload, "contact_adresse"))
            .bind(opt_str_field(payload, "constitution_statut_juridique"))
            .bind(opt_str_field(payload, "constitution_numero"))
            .bind(opt_str_field(payload, "constitution_document_url"))
            .bind(images_opt.as_ref())
            .bind(opt_str_field(payload, "site_web_url"))
            .execute(&mut **tx)
            .await?;
            Ok(Some(tid))
        }
        ("site_touristique", "suppression") => {
            let tid = target_id.unwrap();
            sqlx::query(
                "UPDATE country_profile.site_touristique SET deleted_at = NOW()
                 WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(tid)
            .execute(&mut **tx)
            .await?;
            Ok(Some(tid))
        }

        // ── Secteur developpement ───────────────────────────────
        ("secteur_developpement", "ajout") => {
            let payload = nouvelle_jsonb.ok_or_else(|| {
                ApiErreur::Validation("nouvelle_valeur_jsonb requise pour secteur".into())
            })?;
            let nouvel_id: Uuid = sqlx::query_scalar(
                "INSERT INTO country_profile.secteur_developpement
                    (fiche_pays_id, nom, description, localite,
                     contact_telephone, contact_courriel, contact_adresse,
                     references_utiles, site_web_url, image_url)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id",
            )
            .bind(fiche_pays_id)
            .bind(str_field(payload, "nom"))
            .bind(opt_str_field(payload, "description"))
            .bind(opt_str_field(payload, "localite"))
            .bind(opt_str_field(payload, "contact_telephone"))
            .bind(opt_str_field(payload, "contact_courriel"))
            .bind(opt_str_field(payload, "contact_adresse"))
            .bind(opt_str_field(payload, "references_utiles"))
            .bind(opt_str_field(payload, "site_web_url"))
            .bind(opt_str_field(payload, "image_url"))
            .fetch_one(&mut **tx)
            .await?;
            Ok(Some(nouvel_id))
        }
        ("secteur_developpement", "edition") => {
            let payload = nouvelle_jsonb.unwrap();
            let tid = target_id.unwrap();
            sqlx::query(
                "UPDATE country_profile.secteur_developpement SET
                    nom = COALESCE($2, nom),
                    description = COALESCE($3, description),
                    localite = COALESCE($4, localite),
                    contact_telephone = COALESCE($5, contact_telephone),
                    contact_courriel = COALESCE($6, contact_courriel),
                    contact_adresse = COALESCE($7, contact_adresse),
                    references_utiles = COALESCE($8, references_utiles),
                    site_web_url = COALESCE($9, site_web_url),
                    image_url = COALESCE($10, image_url)
                 WHERE id = $1",
            )
            .bind(tid)
            .bind(opt_str_field(payload, "nom"))
            .bind(opt_str_field(payload, "description"))
            .bind(opt_str_field(payload, "localite"))
            .bind(opt_str_field(payload, "contact_telephone"))
            .bind(opt_str_field(payload, "contact_courriel"))
            .bind(opt_str_field(payload, "contact_adresse"))
            .bind(opt_str_field(payload, "references_utiles"))
            .bind(opt_str_field(payload, "site_web_url"))
            .bind(opt_str_field(payload, "image_url"))
            .execute(&mut **tx)
            .await?;
            Ok(Some(tid))
        }
        ("secteur_developpement", "suppression") => {
            let tid = target_id.unwrap();
            // Pas de soft delete sur secteur_developpement (table historique), DELETE simple
            sqlx::query("DELETE FROM country_profile.secteur_developpement WHERE id = $1")
                .bind(tid)
                .execute(&mut **tx)
                .await?;
            Ok(Some(tid))
        }

        // ── Recette culinaire ───────────────────────────────────
        ("recette_culinaire", "ajout") => {
            let payload = nouvelle_jsonb.ok_or_else(|| {
                ApiErreur::Validation("nouvelle_valeur_jsonb requise pour recette".into())
            })?;
            let ingredients = str_array_field(payload, "ingredients", 50).unwrap_or_default();
            let etapes = str_array_field(payload, "etapes_preparation", 10).unwrap_or_default();
            let images = str_array_field(payload, "images", 5).unwrap_or_default();
            let nouvel_id: Uuid = sqlx::query_scalar(
                "INSERT INTO country_profile.recette_culinaire
                    (fiche_pays_id, titre, territoires_consommation, histoire,
                     ingredients, etapes_preparation, images, cree_par)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
            )
            .bind(fiche_pays_id)
            .bind(str_field(payload, "titre"))
            .bind(opt_str_field(payload, "territoires_consommation"))
            .bind(opt_str_field(payload, "histoire"))
            .bind(&ingredients)
            .bind(&etapes)
            .bind(&images)
            .bind(auteur_id)
            .fetch_one(&mut **tx)
            .await?;
            Ok(Some(nouvel_id))
        }
        ("recette_culinaire", "edition") => {
            let payload = nouvelle_jsonb.unwrap();
            let tid = target_id.unwrap();
            let ingredients = str_array_field(payload, "ingredients", 50);
            let etapes = str_array_field(payload, "etapes_preparation", 10);
            let images = str_array_field(payload, "images", 5);
            sqlx::query(
                "UPDATE country_profile.recette_culinaire SET
                    titre = COALESCE($2, titre),
                    territoires_consommation = COALESCE($3, territoires_consommation),
                    histoire = COALESCE($4, histoire),
                    ingredients = COALESCE($5::text[], ingredients),
                    etapes_preparation = COALESCE($6::text[], etapes_preparation),
                    images = COALESCE($7::text[], images)
                 WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(tid)
            .bind(opt_str_field(payload, "titre"))
            .bind(opt_str_field(payload, "territoires_consommation"))
            .bind(opt_str_field(payload, "histoire"))
            .bind(ingredients.as_ref())
            .bind(etapes.as_ref())
            .bind(images.as_ref())
            .execute(&mut **tx)
            .await?;
            Ok(Some(tid))
        }
        ("recette_culinaire", "suppression") => {
            let tid = target_id.unwrap();
            sqlx::query(
                "UPDATE country_profile.recette_culinaire SET deleted_at = NOW()
                 WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(tid)
            .execute(&mut **tx)
            .await?;
            Ok(Some(tid))
        }

        // ── Personnalite connue ─────────────────────────────────
        ("personnalite_connue", "ajout") => {
            let payload = nouvelle_jsonb.ok_or_else(|| {
                ApiErreur::Validation("nouvelle_valeur_jsonb requise pour personnalite".into())
            })?;
            let nouvel_id: Uuid = sqlx::query_scalar(
                "INSERT INTO country_profile.personnalite_connue
                    (fiche_pays_id, nom_complet, domaine, biographie_courte,
                     annee_naissance, annee_deces, portrait_url, lien_reference, cree_par)
                 VALUES ($1, $2, $3::country_profile.domaine_personnalite, $4,
                         $5, $6, $7, $8, $9) RETURNING id",
            )
            .bind(fiche_pays_id)
            .bind(str_field(payload, "nom_complet"))
            .bind(str_field(payload, "domaine"))
            .bind(str_field(payload, "biographie_courte"))
            .bind(i16_field(payload, "annee_naissance"))
            .bind(i16_field(payload, "annee_deces"))
            .bind(opt_str_field(payload, "portrait_url"))
            .bind(opt_str_field(payload, "lien_reference"))
            .bind(auteur_id)
            .fetch_one(&mut **tx)
            .await?;
            Ok(Some(nouvel_id))
        }
        ("personnalite_connue", "edition") => {
            let payload = nouvelle_jsonb.unwrap();
            let tid = target_id.unwrap();
            sqlx::query(
                "UPDATE country_profile.personnalite_connue SET
                    nom_complet = COALESCE($2, nom_complet),
                    domaine = COALESCE($3::country_profile.domaine_personnalite, domaine),
                    biographie_courte = COALESCE($4, biographie_courte),
                    annee_naissance = COALESCE($5, annee_naissance),
                    annee_deces = COALESCE($6, annee_deces),
                    portrait_url = COALESCE($7, portrait_url),
                    lien_reference = COALESCE($8, lien_reference)
                 WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(tid)
            .bind(opt_str_field(payload, "nom_complet"))
            .bind(opt_str_field(payload, "domaine"))
            .bind(opt_str_field(payload, "biographie_courte"))
            .bind(i16_field(payload, "annee_naissance"))
            .bind(i16_field(payload, "annee_deces"))
            .bind(opt_str_field(payload, "portrait_url"))
            .bind(opt_str_field(payload, "lien_reference"))
            .execute(&mut **tx)
            .await?;
            Ok(Some(tid))
        }
        ("personnalite_connue", "suppression") => {
            let tid = target_id.unwrap();
            sqlx::query(
                "UPDATE country_profile.personnalite_connue SET deleted_at = NOW()
                 WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(tid)
            .execute(&mut **tx)
            .await?;
            Ok(Some(tid))
        }

        // ── Savoir pratique ─────────────────────────────────────
        ("savoir_pratique", "ajout") => {
            let payload = nouvelle_jsonb.ok_or_else(|| {
                ApiErreur::Validation("nouvelle_valeur_jsonb requise pour savoir".into())
            })?;
            let nouvel_id: Uuid = sqlx::query_scalar(
                "INSERT INTO country_profile.savoir_pratique
                    (fiche_pays_id, titre, categorie, explication, exemple, cree_par)
                 VALUES ($1, $2, $3::country_profile.categorie_savoir, $4, $5, $6)
                 RETURNING id",
            )
            .bind(fiche_pays_id)
            .bind(str_field(payload, "titre"))
            .bind(str_field(payload, "categorie"))
            .bind(str_field(payload, "explication"))
            .bind(opt_str_field(payload, "exemple"))
            .bind(auteur_id)
            .fetch_one(&mut **tx)
            .await?;
            Ok(Some(nouvel_id))
        }
        ("savoir_pratique", "edition") => {
            let payload = nouvelle_jsonb.unwrap();
            let tid = target_id.unwrap();
            sqlx::query(
                "UPDATE country_profile.savoir_pratique SET
                    titre = COALESCE($2, titre),
                    categorie = COALESCE($3::country_profile.categorie_savoir, categorie),
                    explication = COALESCE($4, explication),
                    exemple = COALESCE($5, exemple)
                 WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(tid)
            .bind(opt_str_field(payload, "titre"))
            .bind(opt_str_field(payload, "categorie"))
            .bind(opt_str_field(payload, "explication"))
            .bind(opt_str_field(payload, "exemple"))
            .execute(&mut **tx)
            .await?;
            Ok(Some(tid))
        }
        ("savoir_pratique", "suppression") => {
            let tid = target_id.unwrap();
            sqlx::query(
                "UPDATE country_profile.savoir_pratique SET deleted_at = NOW()
                 WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(tid)
            .execute(&mut **tx)
            .await?;
            Ok(Some(tid))
        }

        // ── Recommandation visiteur ─────────────────────────────
        // T037 : quel que soit (ajout|edition), on desactive TOUTE reco active
        // de l'auteur sur ce pays, puis on INSERT la nouvelle en active=TRUE.
        ("recommandation_visiteur", _) => {
            let payload = nouvelle_jsonb.ok_or_else(|| {
                ApiErreur::Validation("nouvelle_valeur_jsonb requise pour recommandation".into())
            })?;
            sqlx::query(
                "UPDATE country_profile.recommandation_visiteur
                 SET active = FALSE
                 WHERE utilisateur_id = $1 AND fiche_pays_id = $2
                   AND active = TRUE AND deleted_at IS NULL",
            )
            .bind(auteur_id)
            .bind(fiche_pays_id)
            .execute(&mut **tx)
            .await?;

            let nouvel_id: Uuid = sqlx::query_scalar(
                "INSERT INTO country_profile.recommandation_visiteur
                    (fiche_pays_id, utilisateur_id, note, commentaire, active)
                 VALUES ($1, $2, $3, $4, TRUE) RETURNING id",
            )
            .bind(fiche_pays_id)
            .bind(auteur_id)
            .bind(i16_field(payload, "note").unwrap_or(5))
            .bind(str_field(payload, "commentaire"))
            .fetch_one(&mut **tx)
            .await?;
            Ok(Some(nouvel_id))
        }

        // ── Photo visiteur : INSERT 1 ligne par piece jointe ────
        ("photo_visiteur", _) => {
            let pieces = pieces_json.as_array().cloned().unwrap_or_default();
            let mut dernier_id = None;
            for piece in pieces {
                let obj = match piece.as_object() {
                    Some(o) => o,
                    None => continue,
                };
                let chemin = obj
                    .get("chemin_fichier")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if chemin.is_empty() {
                    continue;
                }
                let nouvel_id: Uuid = sqlx::query_scalar(
                    "INSERT INTO country_profile.photo_visiteur
                        (fiche_pays_id, utilisateur_id, chemin_fichier, legende,
                         format, taille_octets, largeur_px, hauteur_px)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
                )
                .bind(fiche_pays_id)
                .bind(auteur_id)
                .bind(chemin)
                .bind(obj.get("legende").and_then(|v| v.as_str()).unwrap_or(""))
                .bind(obj.get("format").and_then(|v| v.as_str()).unwrap_or("jpeg"))
                .bind(
                    obj.get("taille_octets")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0) as i32,
                )
                .bind(obj.get("largeur").and_then(|v| v.as_i64()).unwrap_or(0) as i16)
                .bind(obj.get("hauteur").and_then(|v| v.as_i64()).unwrap_or(0) as i16)
                .fetch_one(&mut **tx)
                .await?;
                dernier_id = Some(nouvel_id);
            }
            Ok(dernier_id)
        }

        // ── Fiche pays (creation) : US3 ─────────────────────────
        ("fiche_pays", "ajout") => {
            let payload = nouvelle_jsonb.ok_or_else(|| {
                ApiErreur::Validation("nouvelle_valeur_jsonb requise pour fiche_pays".into())
            })?;
            let code_iso2 = str_field(payload, "code_iso2").to_lowercase();

            // Resoudre pays_id depuis shared.pays (le pays doit deja exister)
            let pays_id: Option<Uuid> =
                sqlx::query_scalar("SELECT id FROM shared.pays WHERE LOWER(code_iso2) = $1")
                    .bind(&code_iso2)
                    .fetch_optional(&mut **tx)
                    .await?;
            let pays_id = pays_id.ok_or_else(|| {
                ApiErreur::Validation(format!(
                    "Pays '{}' introuvable dans shared.pays (pre-alimentation requise).",
                    code_iso2
                ))
            })?;

            let nouvel_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO country_profile.fiche_pays
                    (id, pays_id, slogan, superficie_km2, population, biographie,
                     contexte, image_couverture_url, image_drapeau_url, image_embleme_url,
                     hymne_national, langue_officielle, langues_populaires,
                     monnaie, fuseau_horaire, cree_par)
                 VALUES ($1, $2, $3, $4::decimal, $5::bigint, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                 ON CONFLICT (pays_id) DO NOTHING",
            )
            .bind(nouvel_id)
            .bind(pays_id)
            .bind(opt_str_field(payload, "slogan"))
            .bind(opt_str_field(payload, "superficie_km2"))
            .bind(opt_str_field(payload, "population"))
            .bind(opt_str_field(payload, "biographie"))
            .bind(opt_str_field(payload, "contexte"))
            .bind(opt_str_field(payload, "image_couverture_url"))
            .bind(opt_str_field(payload, "image_drapeau_url"))
            .bind(opt_str_field(payload, "image_embleme_url"))
            .bind(opt_str_field(payload, "hymne_national"))
            .bind(opt_str_field(payload, "langue_officielle"))
            .bind(opt_str_field(payload, "langues_populaires"))
            .bind(opt_str_field(payload, "monnaie"))
            .bind(opt_str_field(payload, "fuseau_horaire"))
            .bind(auteur_id)
            .execute(&mut **tx)
            .await?;
            Ok(Some(nouvel_id))
        }

        // Autres combos : pas d'effet (ex. fiche_pays + edition : hors scope US3)
        _ => Ok(target_id),
    }
}

fn str_field<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(|v| v.as_str()).unwrap_or("")
}

fn opt_str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
}

fn i16_field(value: &Value, key: &str) -> Option<i16> {
    value.get(key).and_then(|v| v.as_i64()).map(|n| n as i16)
}

fn opt_f64_field(value: &Value, key: &str) -> Option<f64> {
    value.get(key).and_then(|v| v.as_f64())
}

/// Extrait la galerie d'images d'un payload de site (clé `images`), plafonnée à 5.
/// Repli sur `image_url` (legacy) si `images` absent. `None` => rien fourni
/// (permet de conserver l'existant en édition via COALESCE).
fn images_site_field(value: &Value) -> Option<Vec<String>> {
    if let Some(arr) = value.get("images").and_then(|v| v.as_array()) {
        let images: Vec<String> = arr
            .iter()
            .filter_map(|x| x.as_str())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .take(5)
            .map(String::from)
            .collect();
        return Some(images);
    }
    opt_str_field(value, "image_url").map(|s| vec![s.to_string()])
}

/// Extrait un tableau de chaînes d'un payload JSONB (clé `key`), plafonné à `max`.
/// `None` => clé absente (permet de conserver l'existant en édition via COALESCE).
fn str_array_field(value: &Value, key: &str, max: usize) -> Option<Vec<String>> {
    value.get(key).and_then(|v| v.as_array()).map(|arr| {
        arr.iter()
            .filter_map(|x| x.as_str())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .take(max)
            .map(String::from)
            .collect()
    })
}

fn construire_message_notification(
    etat: &str,
    type_objet: &str,
    note: Option<&str>,
) -> (String, String) {
    let type_ = match etat {
        "approuvee" => "afripulse_approuvee",
        "rejetee" => "afripulse_rejetee",
        _ => "afripulse_moderation",
    };
    let libelle_objet = match type_objet {
        "site_touristique" => "site touristique",
        "secteur_developpement" => "secteur d'opportunite",
        "personnalite_connue" => "personnalite",
        "savoir_pratique" => "conseil pratique",
        "recommandation_visiteur" => "recommandation",
        "photo_visiteur" => "photo",
        "fiche_pays" => "fiche pays",
        _ => "contribution",
    };
    let message = match etat {
        "approuvee" => format!(
            "Votre contribution ({}) a ete approuvee et est desormais publiee.",
            libelle_objet
        ),
        "rejetee" => {
            let motif = note.unwrap_or("(aucun motif)");
            format!(
                "Votre contribution ({}) a ete rejetee. Motif : {}",
                libelle_objet, motif
            )
        }
        _ => format!("Votre contribution ({}) a ete traitee.", libelle_objet),
    };
    (type_.to_string(), message)
}

// ══════════════════════════════════════════════════════════════
// ── Contributions suspendues (signalement communautaire) ──────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct ContributionSuspendueRow {
    pub type_objet: String,
    pub objet_id: Uuid,
    pub libelle: String,
    pub fiche_pays_id: Uuid,
    pub pays_nom: Option<String>,
    pub nombre_signalements: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// GET /api/admin/profils-pays/contributions-suspendues
/// Liste toutes les contributions suspendues (>10 signalements), tous types confondus.
pub async fn lister_contributions_suspendues(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "voir");

    // UNION ALL des 5 objets signalables ; libellé = colonne nom/titre propre à chaque table.
    let rows: Vec<ContributionSuspendueRow> = sqlx::query_as(
        "SELECT 'site_touristique' AS type_objet, st.id AS objet_id, st.nom AS libelle,
                st.fiche_pays_id, p.nom AS pays_nom, st.nombre_signalements, st.created_at
           FROM country_profile.site_touristique st
           JOIN country_profile.fiche_pays fp ON fp.id = st.fiche_pays_id
           LEFT JOIN shared.pays p ON p.id = fp.pays_id
          WHERE st.suspendu = TRUE AND st.deleted_at IS NULL
         UNION ALL
         SELECT 'secteur_developpement', sd.id, sd.nom,
                sd.fiche_pays_id, p.nom, sd.nombre_signalements, sd.created_at
           FROM country_profile.secteur_developpement sd
           JOIN country_profile.fiche_pays fp ON fp.id = sd.fiche_pays_id
           LEFT JOIN shared.pays p ON p.id = fp.pays_id
          WHERE sd.suspendu = TRUE
         UNION ALL
         SELECT 'recette_culinaire', rc.id, rc.titre,
                rc.fiche_pays_id, p.nom, rc.nombre_signalements, rc.created_at
           FROM country_profile.recette_culinaire rc
           JOIN country_profile.fiche_pays fp ON fp.id = rc.fiche_pays_id
           LEFT JOIN shared.pays p ON p.id = fp.pays_id
          WHERE rc.suspendu = TRUE AND rc.deleted_at IS NULL
         UNION ALL
         SELECT 'personnalite_connue', pc.id, pc.nom_complet,
                pc.fiche_pays_id, p.nom, pc.nombre_signalements, pc.created_at
           FROM country_profile.personnalite_connue pc
           JOIN country_profile.fiche_pays fp ON fp.id = pc.fiche_pays_id
           LEFT JOIN shared.pays p ON p.id = fp.pays_id
          WHERE pc.suspendu = TRUE AND pc.deleted_at IS NULL
         UNION ALL
         SELECT 'savoir_pratique', sp.id, sp.titre,
                sp.fiche_pays_id, p.nom, sp.nombre_signalements, sp.created_at
           FROM country_profile.savoir_pratique sp
           JOIN country_profile.fiche_pays fp ON fp.id = sp.fiche_pays_id
           LEFT JOIN shared.pays p ON p.id = fp.pays_id
          WHERE sp.suspendu = TRUE AND sp.deleted_at IS NULL
         ORDER BY nombre_signalements DESC, created_at DESC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rows),
        error: None,
    }))
}

/// POST /api/admin/profils-pays/contributions-suspendues/{type_objet}/{objet_id}/reactiver
/// Lève la suspension d'une contribution + purge ses signalements (ardoise vierge).
pub async fn reactiver_contribution(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(String, Uuid)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "profil_pays", "modifier");
    let (type_objet, objet_id) = path.into_inner();

    let (table, _) = crate::handlers::contribution_signalement::table_et_softdelete(&type_objet)
        .ok_or_else(|| ApiErreur::Validation("Type de contribution invalide".into()))?;

    // Lève la suspension + remet le compteur à zéro.
    let maj: Option<Uuid> = sqlx::query_scalar(&format!(
        "UPDATE {table} SET suspendu = FALSE, nombre_signalements = 0
         WHERE id = $1 RETURNING id"
    ))
    .bind(objet_id)
    .fetch_optional(pool.get_ref())
    .await?;
    if maj.is_none() {
        return Err(ApiErreur::NonTrouve("Contribution introuvable".into()));
    }

    // Purge les signalements de cet objet (évite une re-suspension immédiate).
    sqlx::query(
        "DELETE FROM country_profile.signalement_contribution
         WHERE type_objet = $1::country_profile.type_objet_contribution AND objet_id = $2",
    )
    .bind(&type_objet)
    .bind(objet_id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "REACTIVATION",
        "country_profile",
        &type_objet,
        Some(objet_id),
        Some(json!({ "suspendu": true })),
        Some(json!({ "suspendu": false, "nombre_signalements": 0 })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    log::info!(
        "Admin {} a reactive la contribution {}/{}",
        admin.id, type_objet, objet_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({ "type_objet": type_objet, "objet_id": objet_id, "suspendu": false })),
        error: None,
    }))
}

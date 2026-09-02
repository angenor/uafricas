use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::faculte::{
    AdminEcoleDetailResponse, AdminEcoleListeResponse, AdminEcoleQueryParams,
    AdminFaculteDetailResponse, AdminFaculteListeResponse, AdminFaculteQueryParams,
    CreerEcoleRequest, CreerFaculteRequest, ModifierEcoleRequest, ModifierFaculteRequest,
    ADMIN_ECOLE_DETAIL_COLONNES, ADMIN_ECOLE_LISTE_COLONNES, ADMIN_FACULTE_DETAIL_COLONNES,
    ADMIN_FACULTE_LISTE_COLONNES, ECOLE_TRI_COLONNES, FACULTE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ════════════════════════════════════════════════════════════════════
// Helpers
// ════════════════════════════════════════════════════════════════════

/// Valider le type d'ecole (publique/privee).
fn valider_type_ecole(valeur: &str) -> Result<&'static str, ApiErreur> {
    match valeur.trim() {
        "publique" => Ok("publique"),
        "privee" => Ok("privee"),
        _ => Err(ApiErreur::Validation(
            "Type d'ecole invalide. Valeurs acceptees: publique, privee".into(),
        )),
    }
}

/// Valider le statut de faculte (active/inactive). Defaut: "active".
fn valider_statut(valeur: Option<&str>) -> Result<&'static str, ApiErreur> {
    match valeur.map(|s| s.trim()) {
        None | Some("") | Some("active") => Ok("active"),
        Some("inactive") => Ok("inactive"),
        Some(_) => Err(ApiErreur::Validation(
            "Statut invalide. Valeurs acceptees: active, inactive".into(),
        )),
    }
}

/// Generer un slug URL-safe a partir du titre.
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

/// Nettoyer un tableau de chaines (trim + supprimer les vides).
fn nettoyer_liste(liste: &[String]) -> Vec<String> {
    liste
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Convertir une chaine optionnelle videe en None.
fn texte_optionnel(valeur: Option<&str>) -> Option<String> {
    valeur
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

// ════════════════════════════════════════════════════════════════════
// ÉCOLES PARTENAIRES : CRUD
// ════════════════════════════════════════════════════════════════════

/// GET /api/admin/ecoles-partenaires
pub async fn lister_ecoles(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminEcoleQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "voir");

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
                "(LOWER(e.nom) LIKE ${bi} OR LOWER(e.ville) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }
    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("e.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_index += 1;
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(ECOLE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM exchange.ecole_partenaire e
         LEFT JOIN shared.pays p ON p.id = e.pays_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    for v in &bind_uuids { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM exchange.ecole_partenaire e
         LEFT JOIN shared.pays p ON p.id = e.pays_id
         WHERE {} ORDER BY e.{} {} LIMIT {} OFFSET {}",
        ADMIN_ECOLE_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminEcoleListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    for v in &bind_uuids { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/ecoles-partenaires/{id}
pub async fn obtenir_ecole(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM exchange.ecole_partenaire e
         LEFT JOIN shared.pays p ON p.id = e.pays_id
         WHERE e.id = $1",
        ADMIN_ECOLE_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminEcoleDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Ecole partenaire non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row),
        error: None,
    }))
}

/// POST /api/admin/ecoles-partenaires
pub async fn creer_ecole(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerEcoleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "modifier");

    let nom = body.nom.trim();
    let ville = body.ville.trim();
    let email = body.email_contact.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de l'ecole est requis".into()));
    }
    if ville.is_empty() {
        return Err(ApiErreur::Validation("La ville est requise".into()));
    }
    if email.is_empty() {
        return Err(ApiErreur::Validation("L'email de contact est requis".into()));
    }
    let type_ecole = valider_type_ecole(&body.type_ecole)?;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO exchange.ecole_partenaire
         (id, nom, ville, pays_id, type, site_web, email_contact, telephone_contact, whatsapp_contact)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(id)
    .bind(nom)
    .bind(ville)
    .bind(body.pays_id)
    .bind(type_ecole)
    .bind(texte_optionnel(body.site_web.as_deref()))
    .bind(email)
    .bind(texte_optionnel(body.telephone_contact.as_deref()))
    .bind(texte_optionnel(body.whatsapp_contact.as_deref()))
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree l'ecole partenaire {} ({})", admin.id, nom, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "exchange", "ecole_partenaire",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/ecoles-partenaires/{id}
pub async fn modifier_ecole(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierEcoleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "modifier");
    let id = path.into_inner();

    let nom = body.nom.trim();
    let ville = body.ville.trim();
    let email = body.email_contact.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de l'ecole est requis".into()));
    }
    if ville.is_empty() {
        return Err(ApiErreur::Validation("La ville est requise".into()));
    }
    if email.is_empty() {
        return Err(ApiErreur::Validation("L'email de contact est requis".into()));
    }
    let type_ecole = valider_type_ecole(&body.type_ecole)?;

    let result = sqlx::query(
        "UPDATE exchange.ecole_partenaire
         SET nom = $1, ville = $2, pays_id = $3, type = $4, site_web = $5,
             email_contact = $6, telephone_contact = $7, whatsapp_contact = $8,
             actif = COALESCE($9, actif), updated_at = NOW()
         WHERE id = $10",
    )
    .bind(nom)
    .bind(ville)
    .bind(body.pays_id)
    .bind(type_ecole)
    .bind(texte_optionnel(body.site_web.as_deref()))
    .bind(email)
    .bind(texte_optionnel(body.telephone_contact.as_deref()))
    .bind(texte_optionnel(body.whatsapp_contact.as_deref()))
    .bind(body.actif)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Ecole partenaire non trouvee".into()));
    }

    log::info!("Admin {} a modifie l'ecole partenaire {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "exchange", "ecole_partenaire",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/ecoles-partenaires/{id} (soft delete via actif=false)
pub async fn supprimer_ecole(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "supprimer");
    let id = path.into_inner();

    // Empecher la desactivation si des facultes actives y sont rattachees.
    let facultes_actives: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM exchange.faculte
         WHERE ecole_partenaire_id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if facultes_actives > 0 {
        return Err(ApiErreur::Conflit(format!(
            "Impossible de desactiver : {} faculte(s) rattachee(s) a cette ecole",
            facultes_actives
        )));
    }

    let result = sqlx::query(
        "UPDATE exchange.ecole_partenaire SET actif = false, updated_at = NOW()
         WHERE id = $1 AND actif = true",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Ecole partenaire non trouvee".into()));
    }

    log::info!("Admin {} a desactive l'ecole partenaire {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "exchange", "ecole_partenaire",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════
// FACULTÉS : CRUD
// ════════════════════════════════════════════════════════════════════

/// GET /api/admin/facultes
pub async fn lister_facultes(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminFaculteQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "voir");

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
                "(LOWER(f.titre) LIKE ${bi} OR LOWER(f.acronyme) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }
    if let Some(ecole_id) = params.ecole_partenaire_id {
        conditions.push(format!("f.ecole_partenaire_id = ${}", bind_index));
        bind_uuids.push(ecole_id);
        bind_index += 1;
    }
    if let Some(ref statut) = params.statut {
        let s = statut.trim();
        if !s.is_empty() {
            conditions.push(format!("f.statut = ${}", bind_index));
            bind_values.push(s.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(FACULTE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM exchange.faculte f
         JOIN exchange.ecole_partenaire e ON e.id = f.ecole_partenaire_id
         LEFT JOIN shared.pays p ON p.id = e.pays_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    for v in &bind_uuids { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM exchange.faculte f
         JOIN exchange.ecole_partenaire e ON e.id = f.ecole_partenaire_id
         LEFT JOIN shared.pays p ON p.id = e.pays_id
         WHERE {} ORDER BY f.{} {} LIMIT {} OFFSET {}",
        ADMIN_FACULTE_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminFaculteListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    for v in &bind_uuids { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/facultes/{id}
pub async fn obtenir_faculte(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM exchange.faculte f
         JOIN exchange.ecole_partenaire e ON e.id = f.ecole_partenaire_id
         WHERE f.id = $1 AND f.deleted_at IS NULL",
        ADMIN_FACULTE_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminFaculteDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Faculte non trouvee".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row),
        error: None,
    }))
}

/// Verifier l'existence d'une ecole partenaire active.
async fn verifier_ecole_existe(pool: &PgPool, ecole_id: Uuid) -> Result<(), ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM exchange.ecole_partenaire WHERE id = $1)",
    )
    .bind(ecole_id)
    .fetch_one(pool)
    .await?;
    if !existe {
        return Err(ApiErreur::Validation("Ecole partenaire introuvable".into()));
    }
    Ok(())
}

/// POST /api/admin/facultes
pub async fn creer_faculte(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerFaculteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "modifier");

    let titre = body.titre.trim();
    let acronyme = body.acronyme.trim();
    let description = body.description.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre de la faculte est requis".into()));
    }
    if acronyme.is_empty() {
        return Err(ApiErreur::Validation("L'acronyme est requis".into()));
    }
    if description.is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }
    let statut = valider_statut(body.statut.as_deref())?;
    verifier_ecole_existe(pool.get_ref(), body.ecole_partenaire_id).await?;

    let id = Uuid::new_v4();
    let slug = format!("{}-{}", generer_slug(titre), &id.to_string()[..8]);

    sqlx::query(
        "INSERT INTO exchange.faculte
         (id, titre, acronyme, slug, description, image_couverture_url, logo_url, ecole_partenaire_id,
          domaines_etudes, programmes_licence, programmes_master, programmes_doctorat, programmes_certificats,
          diplome_minimum, langues_enseignement, frais_scolarite_min, frais_scolarite_max, bourses_possibles,
          periodes_inscription, points_forts, accepte_nouveaux_inscrits, statut, referent_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)",
    )
    .bind(id)
    .bind(titre)
    .bind(acronyme)
    .bind(&slug)
    .bind(description)
    .bind(texte_optionnel(body.image_couverture_url.as_deref()))
    .bind(texte_optionnel(body.logo_url.as_deref()))
    .bind(body.ecole_partenaire_id)
    .bind(nettoyer_liste(&body.domaines_etudes))
    .bind(nettoyer_liste(&body.programmes_licence))
    .bind(nettoyer_liste(&body.programmes_master))
    .bind(nettoyer_liste(&body.programmes_doctorat))
    .bind(nettoyer_liste(&body.programmes_certificats))
    .bind(texte_optionnel(body.diplome_minimum.as_deref()))
    .bind(nettoyer_liste(&body.langues_enseignement))
    .bind(body.frais_scolarite_min)
    .bind(body.frais_scolarite_max)
    .bind(body.bourses_possibles)
    .bind(texte_optionnel(body.periodes_inscription.as_deref()))
    .bind(nettoyer_liste(&body.points_forts))
    .bind(body.accepte_nouveaux_inscrits)
    .bind(statut)
    .bind(body.referent_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree la faculte {} ({})", admin.id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "exchange", "faculte",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/facultes/{id}
pub async fn modifier_faculte(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierFaculteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "modifier");
    let id = path.into_inner();

    let titre = body.titre.trim();
    let acronyme = body.acronyme.trim();
    let description = body.description.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre de la faculte est requis".into()));
    }
    if acronyme.is_empty() {
        return Err(ApiErreur::Validation("L'acronyme est requis".into()));
    }
    if description.is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }
    let statut = valider_statut(body.statut.as_deref())?;
    verifier_ecole_existe(pool.get_ref(), body.ecole_partenaire_id).await?;

    let result = sqlx::query(
        "UPDATE exchange.faculte SET
            titre = $1, acronyme = $2, description = $3, image_couverture_url = $4, logo_url = $5,
            ecole_partenaire_id = $6, domaines_etudes = $7, programmes_licence = $8, programmes_master = $9,
            programmes_doctorat = $10, programmes_certificats = $11, diplome_minimum = $12,
            langues_enseignement = $13, frais_scolarite_min = $14, frais_scolarite_max = $15,
            bourses_possibles = $16, periodes_inscription = $17, points_forts = $18,
            accepte_nouveaux_inscrits = $19, statut = $20, referent_id = $21,
            nombre_inscrits_total = COALESCE($22, nombre_inscrits_total),
            nombre_inscrits_annee = COALESCE($23, nombre_inscrits_annee),
            updated_at = NOW()
         WHERE id = $24 AND deleted_at IS NULL",
    )
    .bind(titre)
    .bind(acronyme)
    .bind(description)
    .bind(texte_optionnel(body.image_couverture_url.as_deref()))
    .bind(texte_optionnel(body.logo_url.as_deref()))
    .bind(body.ecole_partenaire_id)
    .bind(nettoyer_liste(&body.domaines_etudes))
    .bind(nettoyer_liste(&body.programmes_licence))
    .bind(nettoyer_liste(&body.programmes_master))
    .bind(nettoyer_liste(&body.programmes_doctorat))
    .bind(nettoyer_liste(&body.programmes_certificats))
    .bind(texte_optionnel(body.diplome_minimum.as_deref()))
    .bind(nettoyer_liste(&body.langues_enseignement))
    .bind(body.frais_scolarite_min)
    .bind(body.frais_scolarite_max)
    .bind(body.bourses_possibles)
    .bind(texte_optionnel(body.periodes_inscription.as_deref()))
    .bind(nettoyer_liste(&body.points_forts))
    .bind(body.accepte_nouveaux_inscrits)
    .bind(statut)
    .bind(body.referent_id)
    .bind(body.nombre_inscrits_total)
    .bind(body.nombre_inscrits_annee)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Faculte non trouvee".into()));
    }

    log::info!("Admin {} a modifie la faculte {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "exchange", "faculte",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/facultes/{id} (soft delete via deleted_at)
pub async fn supprimer_faculte(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE exchange.faculte SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Faculte non trouvee".into()));
    }

    log::info!("Admin {} a supprime la faculte {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "exchange", "faculte",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

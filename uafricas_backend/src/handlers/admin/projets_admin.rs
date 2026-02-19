use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::projet::{
    AdminProjetDetailResponse, AdminProjetDocument, AdminProjetListeResponse,
    AdminProjetQueryParams, AjouterDocumentProjetRequest, ChangerEtatProjetRequest,
    CreerProjetRequest, ModifierProjetRequest, ADMIN_PROJET_DETAIL_COLONNES,
    ADMIN_PROJET_LISTE_COLONNES, PROJET_JOINS, PROJET_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ── Etats valides pour projet (etat_projet) ─────────────────
const ETATS_VALIDES: &[&str] = &[
    "soumis", "en_revue", "approuve", "en_cours", "termine", "suspendu", "rejete",
];

/// GET /api/admin/projets
pub async fn lister_projets(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminProjetQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["pj.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(pj.titre) LIKE ${bi} OR LOWER(pj.description) LIKE ${bi} OR LOWER(COALESCE(pj.nom_organisation, '')) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("pj.etat::text = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref pays_id) = params.pays_id {
        let p = pays_id.trim();
        if !p.is_empty() {
            if Uuid::parse_str(p).is_ok() {
                conditions.push(format!("pj.pays_id::text = ${}", bind_index));
                bind_values.push(p.to_string());
                bind_index += 1;
            }
        }
    }

    if let Some(ref organisation) = params.organisation {
        let o = organisation.trim();
        if !o.is_empty() {
            conditions.push(format!("LOWER(pj.nom_organisation) LIKE ${}", bind_index));
            bind_values.push(format!("%{}%", o.to_lowercase()));
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(PROJET_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    // COUNT
    let count_sql = format!(
        "SELECT COUNT(*) FROM innovation.projet pj {} WHERE {}",
        PROJET_JOINS, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT
    let select_sql = format!(
        "SELECT {} FROM innovation.projet pj {} WHERE {} ORDER BY pj.{} {} LIMIT {} OFFSET {}",
        ADMIN_PROJET_LISTE_COLONNES, PROJET_JOINS, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminProjetListeResponse>(&select_sql);
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

/// GET /api/admin/projets/:id
pub async fn obtenir_projet(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM innovation.projet pj {} WHERE pj.id = $1 AND pj.deleted_at IS NULL",
        ADMIN_PROJET_DETAIL_COLONNES, PROJET_JOINS
    );
    let row = sqlx::query_as::<_, AdminProjetDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Projet non trouve".into()))?;

    // Charger les documents
    let documents = sqlx::query_as::<_, AdminProjetDocument>(
        "SELECT id, nom, url, type_mime, created_at
         FROM innovation.projet_document
         WHERE projet_id = $1
         ORDER BY created_at ASC",
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
            "nom_organisation": row.nom_organisation,
            "description_organisation": row.description_organisation,
            "site_web": row.site_web,
            "pays_id": row.pays_id,
            "ville": row.ville,
            "contact_email": row.contact_email,
            "contact_telephone": row.contact_telephone,
            "cout_total": row.cout_total,
            "devise": row.devise,
            "duree_mois": row.duree_mois,
            "date_commencement_souhaitee": row.date_commencement_souhaitee,
            "description": row.description,
            "objectifs": row.objectifs,
            "resultats_attendus": row.resultats_attendus,
            "activites_programmees": row.activites_programmees,
            "echeanciers": row.echeanciers,
            "contribution_autonomisation": row.contribution_autonomisation,
            "difficultes_risques": row.difficultes_risques,
            "etat": row.etat,
            "cree_par": row.cree_par,
            "traite_par": row.traite_par,
            "created_at": row.created_at,
            "updated_at": row.updated_at,
            "pays_nom": row.pays_nom,
            "auteur_nom": row.auteur_nom,
            "auteur_prenom": row.auteur_prenom,
            "auteur_email": row.auteur_email,
            "traite_par_nom": row.traite_par_nom,
            "traite_par_prenom": row.traite_par_prenom,
            "documents": documents,
        })),
        error: None,
    }))
}

/// POST /api/admin/projets
pub async fn creer_projet(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerProjetRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }
    let description = body.description.trim();
    if description.is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }
    let objectifs = body.objectifs.trim();
    if objectifs.is_empty() {
        return Err(ApiErreur::Validation("Les objectifs sont requis".into()));
    }

    let etat = body.etat.as_deref().unwrap_or("soumis").trim();
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

    let date_commencement: Option<chrono::NaiveDate> =
        body.date_commencement_souhaitee.as_ref().and_then(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
        });

    sqlx::query(
        "INSERT INTO innovation.projet
         (id, titre, slug, description, objectifs, nom_organisation, description_organisation,
          site_web, pays_id, ville, contact_email, contact_telephone,
          cout_total, devise, duree_mois, date_commencement_souhaitee,
          resultats_attendus, activites_programmees, echeanciers,
          contribution_autonomisation, difficultes_risques,
          etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
                 $13, $14, $15, $16, $17, $18, $19, $20, $21,
                 $22::innovation.etat_projet, $23)",
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(description)
    .bind(objectifs)
    .bind(body.nom_organisation.as_deref().map(|s| s.trim()))
    .bind(body.description_organisation.as_deref().map(|s| s.trim()))
    .bind(body.site_web.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(body.contact_email.as_deref().map(|s| s.trim()))
    .bind(body.contact_telephone.as_deref().map(|s| s.trim()))
    .bind(body.cout_total)
    .bind(body.devise.as_deref().unwrap_or("XOF"))
    .bind(body.duree_mois)
    .bind(date_commencement)
    .bind(body.resultats_attendus.as_deref().map(|s| s.trim()))
    .bind(body.activites_programmees.as_deref().map(|s| s.trim()))
    .bind(body.echeanciers.as_deref().map(|s| s.trim()))
    .bind(body.contribution_autonomisation.as_deref().map(|s| s.trim()))
    .bind(body.difficultes_risques.as_deref().map(|s| s.trim()))
    .bind(etat)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le projet {} ({})", admin.id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "innovation",
        "projet",
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

/// PUT /api/admin/projets/:id
pub async fn modifier_projet(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierProjetRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM innovation.projet WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Projet non trouve".into()));
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
    ajouter_champ_str!(body.objectifs, "objectifs");
    ajouter_champ_str!(body.nom_organisation, "nom_organisation");
    ajouter_champ_str!(body.description_organisation, "description_organisation");
    ajouter_champ_str!(body.site_web, "site_web");
    ajouter_champ_str!(body.ville, "ville");
    ajouter_champ_str!(body.contact_email, "contact_email");
    ajouter_champ_str!(body.contact_telephone, "contact_telephone");
    ajouter_champ_str!(body.devise, "devise");
    ajouter_champ_str!(body.resultats_attendus, "resultats_attendus");
    ajouter_champ_str!(body.activites_programmees, "activites_programmees");
    ajouter_champ_str!(body.echeanciers, "echeanciers");
    ajouter_champ_str!(body.contribution_autonomisation, "contribution_autonomisation");
    ajouter_champ_str!(body.difficultes_risques, "difficultes_risques");

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

    // UUID
    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_index += 1;
    }

    // Numeriques (inline)
    if let Some(cout) = body.cout_total {
        sets.push(format!("cout_total = {}", cout));
    }
    if let Some(duree) = body.duree_mois {
        sets.push(format!("duree_mois = {}", duree));
    }

    // Date
    if let Some(ref date_str) = body.date_commencement_souhaitee {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            sets.push(format!("date_commencement_souhaitee = ${}", bind_index));
            bind_strings.push(d.to_string());
            bind_index += 1;
        }
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE innovation.projet SET {} WHERE id = ${} AND deleted_at IS NULL",
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

    log::info!("Admin {} a modifie le projet {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "innovation",
        "projet",
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

/// PATCH /api/admin/projets/:id/etat
pub async fn changer_etat_projet(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatProjetRequest>,
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
        "UPDATE innovation.projet SET etat = $1::innovation.etat_projet, traite_par = $2, updated_at = NOW()
         WHERE id = $3 AND deleted_at IS NULL",
    )
    .bind(etat)
    .bind(admin.id)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Projet non trouve".into()));
    }

    log::info!(
        "Admin {} a change l'etat du projet {} en '{}'",
        admin.id,
        id,
        etat
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "innovation",
        "projet",
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

/// DELETE /api/admin/projets/:id
pub async fn supprimer_projet(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE innovation.projet SET deleted_at = NOW(), etat = 'rejete'::innovation.etat_projet, updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Projet non trouve".into()));
    }

    log::info!("Admin {} a supprime le projet {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "innovation",
        "projet",
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

/// POST /api/admin/projets/:id/documents
pub async fn ajouter_document_projet(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterDocumentProjetRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");
    let projet_id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM innovation.projet WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(projet_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Projet non trouve".into()));
    }

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom du document est requis".into()));
    }
    let url = body.url.trim();
    if url.is_empty() {
        return Err(ApiErreur::Validation("L'URL du document est requise".into()));
    }

    let doc_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO innovation.projet_document (id, projet_id, nom, url, type_mime)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(doc_id)
    .bind(projet_id)
    .bind(nom)
    .bind(url)
    .bind(body.type_mime.as_deref())
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a ajoute un document {} au projet {}",
        admin.id,
        doc_id,
        projet_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "innovation",
        "projet_document",
        Some(doc_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": doc_id })),
        error: None,
    }))
}

/// DELETE /api/admin/projets/:id/documents/:doc_id
pub async fn retirer_document_projet(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "innovation", "modifier");
    let (projet_id, doc_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM innovation.projet_document WHERE id = $1 AND projet_id = $2",
    )
    .bind(doc_id)
    .bind(projet_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Document non trouve".into()));
    }

    log::info!(
        "Admin {} a retire le document {} du projet {}",
        admin.id,
        doc_id,
        projet_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "innovation",
        "projet_document",
        Some(doc_id),
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

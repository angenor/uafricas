use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::programme::{
    AdminProgrammeDetailRow, AdminProgrammeListeResponse, AdminProgrammeQueryParams,
    ChangerEtatProgrammeRequest, CreerProgrammeRequest, ModifierProgrammeRequest,
    ADMIN_PROGRAMME_DETAIL_COLONNES, ADMIN_PROGRAMME_LISTE_COLONNES, PROGRAMME_TRI_COLONNES,
    generer_slug,
};
use crate::models::admin::candidature::AdminCandidatureListeResponse;
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

const ETATS_VALIDES: &[&str] = &[
    "brouillon", "en_attente_validation", "publie", "en_cours", "termine", "suspendu", "annule",
];

const DUREES_VALIDES: &[&str] = &[
    "1_semaine", "2_semaines", "3_semaines", "6_semaines",
    "1_mois", "2_mois", "3_mois", "6_mois", "1_an",
];

/// GET /api/admin/programmes
pub async fn lister_programmes(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminProgrammeQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["p.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;
    // Track which bind params are UUIDs vs strings
    let mut bind_types: Vec<&str> = Vec::new();

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(p.titre) LIKE ${bi} OR LOWER(p.description) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("p.etat::TEXT = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref duree) = params.duree {
        let d = duree.trim();
        if !d.is_empty() {
            conditions.push(format!("p.duree::TEXT = ${}", bind_index));
            bind_values.push(d.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(domaine_id) = params.domaine_id {
        conditions.push(format!("p.domaine_id = ${}", bind_index));
        bind_uuids.push(domaine_id);
        bind_types.push("uuid");
        bind_index += 1;
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("p.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_types.push("uuid");
        bind_index += 1;
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(PROGRAMME_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN shared.pays ON p.pays_id = pays.id
                 LEFT JOIN shared.domaine_secteur ds ON p.domaine_id = ds.id
                 LEFT JOIN iam.utilisateur u ON p.cree_par = u.id";

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM exchange.programme p {} WHERE {}",
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

    // Select
    let select_sql = format!(
        "SELECT {} FROM exchange.programme p {} WHERE {} ORDER BY p.{} {} LIMIT {} OFFSET {}",
        ADMIN_PROGRAMME_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminProgrammeListeResponse>(&select_sql);
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

/// GET /api/admin/programmes/{id}
pub async fn obtenir_programme(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN shared.pays ON p.pays_id = pays.id
                 LEFT JOIN shared.domaine_secteur ds ON p.domaine_id = ds.id
                 LEFT JOIN iam.utilisateur u_cree ON p.cree_par = u_cree.id
                 LEFT JOIN iam.utilisateur u_val ON p.valide_par = u_val.id";

    let sql = format!(
        "SELECT {} FROM exchange.programme p {} WHERE p.id = $1 AND p.deleted_at IS NULL",
        ADMIN_PROGRAMME_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminProgrammeDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Programme non trouve".into()))?;

    let nombre_candidatures: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM exchange.candidature WHERE programme_id = $1"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(nombre_candidatures)),
        error: None,
    }))
}

/// POST /api/admin/programmes
pub async fn creer_programme(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<CreerProgrammeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre du programme est requis".into()));
    }

    let id = Uuid::new_v4();
    let slug = generer_slug(titre);

    // Parse dates
    let date_debut = body.date_debut.as_deref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    let date_fin = body.date_fin.as_deref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    // Validate duree enum
    if let Some(ref duree) = body.duree {
        if !DUREES_VALIDES.contains(&duree.as_str()) {
            return Err(ApiErreur::Validation(format!("Duree invalide: {}", duree)));
        }
    }

    sqlx::query(
        "INSERT INTO exchange.programme
         (id, titre, slug, description, pays_id, ville, adresse,
          prise_en_charge_billet, prise_en_charge_hebergement,
          prise_en_charge_subsistance, prise_en_charge_details,
          duree, domaine_id, date_debut, date_fin, nombre_places,
          prerequis, langues_requises, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
                 $12::exchange.duree_programme, $13, $14, $15, $16, $17, $18,
                 'brouillon'::exchange.etat_programme, $19)"
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(body.adresse.as_deref().map(|s| s.trim()))
    .bind(body.prise_en_charge_billet.unwrap_or(false))
    .bind(body.prise_en_charge_hebergement.unwrap_or(false))
    .bind(body.prise_en_charge_subsistance.unwrap_or(false))
    .bind(body.prise_en_charge_details.as_deref().map(|s| s.trim()))
    .bind(body.duree.as_deref())
    .bind(body.domaine_id)
    .bind(date_debut)
    .bind(date_fin)
    .bind(body.nombre_places)
    .bind(body.prerequis.as_deref().map(|s| s.trim()))
    .bind(&body.langues_requises)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le programme {} ({})", admin.id, titre, id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/programmes/{id}
pub async fn modifier_programme(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierProgrammeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM exchange.programme WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Programme non trouve".into()));
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
    ajouter_champ_str!(body.ville, "ville");
    ajouter_champ_str!(body.adresse, "adresse");
    ajouter_champ_str!(body.prise_en_charge_details, "prise_en_charge_details");
    ajouter_champ_str!(body.prerequis, "prerequis");

    if let Some(ref duree) = body.duree {
        if !DUREES_VALIDES.contains(&duree.as_str()) {
            return Err(ApiErreur::Validation(format!("Duree invalide: {}", duree)));
        }
        sets.push(format!("duree = ${}::exchange.duree_programme", bind_index));
        bind_strings.push(duree.clone());
        bind_index += 1;
    }

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }
    if let Some(domaine_id) = body.domaine_id {
        sets.push(format!("domaine_id = '{}'", domaine_id));
    }
    if let Some(v) = body.prise_en_charge_billet {
        sets.push(format!("prise_en_charge_billet = {}", v));
    }
    if let Some(v) = body.prise_en_charge_hebergement {
        sets.push(format!("prise_en_charge_hebergement = {}", v));
    }
    if let Some(v) = body.prise_en_charge_subsistance {
        sets.push(format!("prise_en_charge_subsistance = {}", v));
    }
    if let Some(v) = body.nombre_places {
        sets.push(format!("nombre_places = {}", v));
    }

    if let Some(ref d) = body.date_debut {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            sets.push(format!("date_debut = '{}'", date));
        }
    }
    if let Some(ref d) = body.date_fin {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            sets.push(format!("date_fin = '{}'", date));
        }
    }

    if let Some(ref langues) = body.langues_requises {
        let arr: Vec<String> = langues.iter().map(|l| format!("'{}'", l.replace('\'', "''"))).collect();
        sets.push(format!("langues_requises = ARRAY[{}]::TEXT[]", arr.join(",")));
    }

    // Also update slug if title changed
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
        "UPDATE exchange.programme SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie le programme {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PATCH /api/admin/programmes/{id}/etat
pub async fn changer_etat_programme(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatProgrammeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide: {}. Valeurs possibles: {:?}", etat, ETATS_VALIDES
        )));
    }

    // Si on publie, enregistrer le validateur
    let result = if etat == "publie" || etat == "en_cours" {
        sqlx::query(
            "UPDATE exchange.programme SET etat = $1::exchange.etat_programme, valide_par = $2, valide_at = NOW(), updated_at = NOW() WHERE id = $3 AND deleted_at IS NULL"
        )
        .bind(etat)
        .bind(admin.id)
        .bind(id)
        .execute(pool.get_ref())
        .await?
    } else {
        sqlx::query(
            "UPDATE exchange.programme SET etat = $1::exchange.etat_programme, updated_at = NOW() WHERE id = $2 AND deleted_at IS NULL"
        )
        .bind(etat)
        .bind(id)
        .execute(pool.get_ref())
        .await?
    };

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Programme non trouve".into()));
    }

    log::info!("Admin {} a change l'etat du programme {} vers {}", admin.id, id, etat);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": etat })),
        error: None,
    }))
}

/// DELETE /api/admin/programmes/{id}
pub async fn supprimer_programme(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE exchange.programme SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Programme non trouve".into()));
    }

    log::info!("Admin {} a supprime le programme {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// GET /api/admin/programmes/{id}/candidatures
pub async fn lister_candidatures_programme(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    params: web::Query<crate::models::admin::candidature::AdminCandidatureQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "programme", "voir");
    let programme_id = path.into_inner();

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec![format!("c.programme_id = $1")];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 2;

    if let Some(ref statut) = params.statut {
        let s = statut.trim();
        if !s.is_empty() {
            conditions.push(format!("c.statut::TEXT = ${}", bind_index));
            bind_values.push(s.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(crate::models::admin::candidature::CANDIDATURE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "JOIN exchange.programme p ON c.programme_id = p.id
                 JOIN iam.utilisateur u ON c.candidat_id = u.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM exchange.candidature c {} WHERE {}", joins, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql).bind(programme_id);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM exchange.candidature c {} WHERE {} ORDER BY c.{} {} LIMIT {} OFFSET {}",
        crate::models::admin::candidature::ADMIN_CANDIDATURE_LISTE_COLONNES,
        joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminCandidatureListeResponse>(&select_sql)
        .bind(programme_id);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

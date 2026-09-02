use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::LivekitConfig;
use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::evenement::{
    AdminEvenementListeResponse, AdminEvenementDetailRow, AdminEvenementQueryParams,
    AdminEvenementInscriptionResponse, AdminEvenementInscriptionStats,
    AdminInscriptionQueryParams,
    CreerEvenementRequest, ModifierEvenementRequest, ChangerEtatEvenementRequest,
    ChangerStatutInscriptionRequest,
    ADMIN_EVENEMENT_LISTE_COLONNES, ADMIN_EVENEMENT_DETAIL_COLONNES, EVENEMENT_TRI_COLONNES,
    generer_slug,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

const ETATS_VALIDES: &[&str] = &["brouillon", "publie", "annule", "termine", "suspendu"];
const FORMATS_VALIDES: &[&str] = &["presentiel", "en_ligne", "hybride"];
const TYPES_ORGANISATEUR_VALIDES: &[&str] = &["personnel", "organisation"];
const STATUTS_INSCRIPTION_VALIDES: &[&str] = &["inscrit", "confirme", "annule", "present", "absent"];

/// GET /api/admin/evenements
pub async fn lister_evenements(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminEvenementQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "evenement", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["e.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;
    let mut bind_types: Vec<&str> = Vec::new();

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(e.titre) LIKE ${bi} OR LOWER(e.description) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref format) = params.format {
        let f = format.trim();
        if !f.is_empty() {
            conditions.push(format!("e.format::TEXT = ${}", bind_index));
            bind_values.push(f.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("e.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("e.pays_id = ${}", bind_index));
        bind_uuids.push(pays_id);
        bind_types.push("uuid");
        bind_index += 1;
    }

    if let Some(ref date_debut) = params.date_debut {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(date_debut, "%Y-%m-%d") {
            conditions.push(format!("e.date_heure_debut >= '{}'", d));
        }
    }
    if let Some(ref date_fin) = params.date_fin {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(date_fin, "%Y-%m-%d") {
            conditions.push(format!("e.date_heure_debut <= '{}'", d));
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(EVENEMENT_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let joins = "LEFT JOIN shared.pays ON e.pays_id = pays.id
                 LEFT JOIN iam.utilisateur u ON e.cree_par = u.id";

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.evenement e {} WHERE {}",
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
        "SELECT {} FROM media_content.evenement e {} WHERE {} ORDER BY e.{} {} LIMIT {} OFFSET {}",
        ADMIN_EVENEMENT_LISTE_COLONNES, joins, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminEvenementListeResponse>(&select_sql);
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

/// GET /api/admin/evenements/{id}
pub async fn obtenir_evenement(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "evenement", "voir");
    let id = path.into_inner();

    let joins = "LEFT JOIN shared.pays ON e.pays_id = pays.id
                 LEFT JOIN iam.utilisateur u ON e.cree_par = u.id";

    let sql = format!(
        "SELECT {} FROM media_content.evenement e {} WHERE e.id = $1 AND e.deleted_at IS NULL",
        ADMIN_EVENEMENT_DETAIL_COLONNES, joins
    );
    let row = sqlx::query_as::<_, AdminEvenementDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Evenement non trouve".into()))?;

    let nombre_inscriptions: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.evenement_inscription WHERE evenement_id = $1"
    ).bind(id).fetch_one(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(nombre_inscriptions)),
        error: None,
    }))
}

/// POST /api/admin/evenements
pub async fn creer_evenement(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerEvenementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "evenement", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }

    if let Some(ref fmt) = body.format {
        if !FORMATS_VALIDES.contains(&fmt.as_str()) {
            return Err(ApiErreur::Validation(format!("Format invalide: {}", fmt)));
        }
    }

    // Type d'organisateur (nom propre vs organisation), important pour les stats.
    let type_organisateur = body.type_organisateur.as_deref().unwrap_or("personnel");
    if !TYPES_ORGANISATEUR_VALIDES.contains(&type_organisateur) {
        return Err(ApiErreur::Validation(
            "Type d'organisateur invalide (personnel ou organisation)".into(),
        ));
    }
    let nettoyer = |o: &Option<String>| {
        o.as_deref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    };
    let contact_nom = nettoyer(&body.contact_nom);
    if type_organisateur == "organisation" && contact_nom.is_none() {
        return Err(ApiErreur::Validation(
            "Le nom de l'organisation est requis".into(),
        ));
    }
    // En nom propre : le nom d'organisation n'a pas de sens.
    let contact_nom = if type_organisateur == "organisation" { contact_nom } else { None };
    let contact_email = nettoyer(&body.contact_email);
    let contact_telephone = nettoyer(&body.contact_telephone);
    let contact_site_web = nettoyer(&body.contact_site_web);

    let date_debut = chrono::DateTime::parse_from_rfc3339(&body.date_heure_debut)
        .map(|d| d.with_timezone(&chrono::Utc))
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(&body.date_heure_debut, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(&body.date_heure_debut, "%Y-%m-%d %H:%M:%S"))
                .map(|d| d.and_utc())
        })
        .map_err(|_| ApiErreur::Validation("Format de date debut invalide".into()))?;

    let date_fin = body.date_heure_fin.as_deref().and_then(|df| {
        chrono::DateTime::parse_from_rfc3339(df)
            .map(|d| d.with_timezone(&chrono::Utc))
            .or_else(|_| {
                chrono::NaiveDateTime::parse_from_str(df, "%Y-%m-%dT%H:%M:%S")
                    .or_else(|_| chrono::NaiveDateTime::parse_from_str(df, "%Y-%m-%d %H:%M:%S"))
                    .map(|d| d.and_utc())
            })
            .ok()
    });

    let id = Uuid::new_v4();
    let slug = generer_slug(titre);
    let format = body.format.as_deref().unwrap_or("presentiel");
    let langue = body.langue.as_deref().unwrap_or("Français");

    sqlx::query(
        "INSERT INTO media_content.evenement
         (id, titre, slug, description, type, pays_id, ville, adresse,
          date_heure_debut, date_heure_fin, image_couverture_url,
          format, lien_en_ligne, langue, nombre_places,
          type_organisateur, contact_nom, contact_email, contact_telephone, contact_site_web,
          enregistrement_url, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
                 $12::media_content.format_evenement, $13, $14, $15,
                 $16::media_content.type_organisateur, $17, $18, $19, $20,
                 $21, 'brouillon', $22)"
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(body.description.trim())
    .bind(body.type_evenement.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(body.adresse.as_deref().map(|s| s.trim()))
    .bind(date_debut)
    .bind(date_fin)
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(format)
    .bind(body.lien_en_ligne.as_deref().map(|s| s.trim()))
    .bind(langue)
    .bind(body.nombre_places)
    .bind(type_organisateur)
    .bind(&contact_nom)
    .bind(&contact_email)
    .bind(&contact_telephone)
    .bind(&contact_site_web)
    .bind(body.enregistrement_url.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree l'evenement {} ({})", admin.id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "media_content",
        "evenement",
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

/// PUT /api/admin/evenements/{id}
pub async fn modifier_evenement(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierEvenementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "evenement", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.evenement WHERE id = $1 AND deleted_at IS NULL)"
    ).bind(id).fetch_one(pool.get_ref()).await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Evenement non trouve".into()));
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
    champ_str!(body.description, "description");
    champ_str!(body.type_evenement, "type");
    champ_str!(body.ville, "ville");
    champ_str!(body.adresse, "adresse");
    champ_str!(body.image_couverture_url, "image_couverture_url");
    champ_str!(body.lien_en_ligne, "lien_en_ligne");
    champ_str!(body.langue, "langue");
    champ_str!(body.contact_email, "contact_email");
    champ_str!(body.contact_telephone, "contact_telephone");
    champ_str!(body.contact_site_web, "contact_site_web");
    champ_str!(body.enregistrement_url, "enregistrement_url");

    if let Some(ref fmt) = body.format {
        if !FORMATS_VALIDES.contains(&fmt.as_str()) {
            return Err(ApiErreur::Validation(format!("Format invalide: {}", fmt)));
        }
        sets.push(format!("format = ${}::media_content.format_evenement", bind_index));
        bind_strings.push(fmt.clone());
        bind_index += 1;
    }

    // Type d'organisateur : nom propre vs organisation (important pour les stats).
    if let Some(ref t) = body.type_organisateur {
        if !TYPES_ORGANISATEUR_VALIDES.contains(&t.as_str()) {
            return Err(ApiErreur::Validation(
                "Type d'organisateur invalide (personnel ou organisation)".into(),
            ));
        }
        sets.push(format!("type_organisateur = ${}::media_content.type_organisateur", bind_index));
        bind_strings.push(t.clone());
        bind_index += 1;

        if t == "personnel" {
            // En nom propre : on efface le nom d'organisation.
            sets.push("contact_nom = NULL".to_string());
        } else {
            // Au nom d'une organisation : le nom est requis.
            let nom = body
                .contact_nom
                .as_deref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());
            match nom {
                Some(n) => {
                    sets.push(format!("contact_nom = ${}", bind_index));
                    bind_strings.push(n.to_string());
                    bind_index += 1;
                }
                None => {
                    return Err(ApiErreur::Validation(
                        "Le nom de l'organisation est requis".into(),
                    ));
                }
            }
        }
    } else {
        // Type inchangé : on met à jour le nom d'organisation s'il est fourni.
        champ_str!(body.contact_nom, "contact_nom");
    }

    if let Some(pays_id) = body.pays_id {
        sets.push(format!("pays_id = '{}'", pays_id));
    }
    if let Some(v) = body.nombre_places {
        sets.push(format!("nombre_places = {}", v));
    }

    if let Some(ref d) = body.date_heure_debut {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(d) {
            sets.push(format!("date_heure_debut = '{}'", dt));
        } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S") {
            sets.push(format!("date_heure_debut = '{}'", dt));
        }
    }
    if let Some(ref d) = body.date_heure_fin {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(d) {
            sets.push(format!("date_heure_fin = '{}'", dt));
        } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S") {
            sets.push(format!("date_heure_fin = '{}'", dt));
        }
    }

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
        "UPDATE media_content.evenement SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie l'evenement {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "evenement",
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

/// PATCH /api/admin/evenements/{id}/etat
pub async fn changer_etat_evenement(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    livekit: web::Data<LivekitConfig>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatEvenementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "evenement", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide: {}. Valeurs possibles: {:?}", etat, ETATS_VALIDES
        )));
    }

    let result = sqlx::query(
        "UPDATE media_content.evenement SET etat = $1, updated_at = NOW() WHERE id = $2 AND deleted_at IS NULL"
    ).bind(etat).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Evenement non trouve".into()));
    }

    // Cascade d'annulation (FR-016) : clôt immédiatement un direct en cours.
    if etat == "annule" {
        let _ = crate::handlers::evenement_streaming::forcer_cloture_session(
            pool.get_ref(),
            livekit.get_ref(),
            id,
        )
        .await;
    }

    log::info!("Admin {} a change l'etat de l'evenement {} vers {}", admin.id, id, etat);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "evenement",
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

/// DELETE /api/admin/evenements/{id}
pub async fn supprimer_evenement(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "evenement", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.evenement SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    ).bind(id).execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Evenement non trouve".into()));
    }

    log::info!("Admin {} a supprime l'evenement {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "media_content",
        "evenement",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Inscriptions ─────────────────────────────────────────────

/// GET /api/admin/evenements/{id}/inscriptions
pub async fn lister_inscriptions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    params: web::Query<AdminInscriptionQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "evenement", "voir");
    let evenement_id = path.into_inner();

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).min(100);
    let offset = (page - 1) * par_page;

    let mut conditions = vec!["ei.evenement_id = $1".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 2;

    if let Some(ref statut) = params.statut {
        let s = statut.trim();
        if !s.is_empty() {
            conditions.push(format!("ei.statut = ${}", bind_index));
            bind_values.push(s.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");

    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.evenement_inscription ei
         JOIN iam.utilisateur u ON ei.utilisateur_id = u.id WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql).bind(evenement_id);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT ei.id, ei.utilisateur_id, u.nom, u.prenom, u.email, ei.statut, ei.created_at
         FROM media_content.evenement_inscription ei
         JOIN iam.utilisateur u ON ei.utilisateur_id = u.id
         WHERE {} ORDER BY ei.created_at DESC LIMIT {} OFFSET {}",
        where_clause, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminEvenementInscriptionResponse>(&select_sql)
        .bind(evenement_id);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// PATCH /api/admin/evenements/{id}/inscriptions/{insc_id}/statut
pub async fn changer_statut_inscription(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ChangerStatutInscriptionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "evenement", "modifier");
    let (evenement_id, insc_id) = path.into_inner();

    let statut = body.statut.trim();
    if !STATUTS_INSCRIPTION_VALIDES.contains(&statut) {
        return Err(ApiErreur::Validation(format!(
            "Statut invalide: {}. Valeurs possibles: {:?}", statut, STATUTS_INSCRIPTION_VALIDES
        )));
    }

    let result = sqlx::query(
        "UPDATE media_content.evenement_inscription SET statut = $1, updated_at = NOW()
         WHERE id = $2 AND evenement_id = $3"
    ).bind(statut).bind(insc_id).bind(evenement_id)
    .execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Inscription non trouvee".into()));
    }

    log::info!("Admin {} a change le statut de l'inscription {} vers {}", admin.id, insc_id, statut);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "media_content",
        "inscription_evenement",
        Some(insc_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": insc_id, "statut": statut })),
        error: None,
    }))
}

/// GET /api/admin/evenements/{id}/inscriptions/stats
pub async fn stats_inscriptions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "evenement", "voir");
    let evenement_id = path.into_inner();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.evenement_inscription WHERE evenement_id = $1"
    ).bind(evenement_id).fetch_one(pool.get_ref()).await?;

    let inscrits: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.evenement_inscription WHERE evenement_id = $1 AND statut = 'inscrit'"
    ).bind(evenement_id).fetch_one(pool.get_ref()).await?;

    let confirmes: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.evenement_inscription WHERE evenement_id = $1 AND statut = 'confirme'"
    ).bind(evenement_id).fetch_one(pool.get_ref()).await?;

    let annules: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.evenement_inscription WHERE evenement_id = $1 AND statut = 'annule'"
    ).bind(evenement_id).fetch_one(pool.get_ref()).await?;

    let presents: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.evenement_inscription WHERE evenement_id = $1 AND statut = 'present'"
    ).bind(evenement_id).fetch_one(pool.get_ref()).await?;

    let absents: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.evenement_inscription WHERE evenement_id = $1 AND statut = 'absent'"
    ).bind(evenement_id).fetch_one(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminEvenementInscriptionStats {
            total, inscrits, confirmes, annules, presents, absents,
        }),
        error: None,
    }))
}

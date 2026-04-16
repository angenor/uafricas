use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::session_afrolang::{
    AdminSessionDetailRow, AdminSessionListeResponse, AdminSessionParticipantResponse,
    AdminSessionQueryParams, AdminTableauBlancResponse,
    ADMIN_SESSION_DETAIL_COLONNES, ADMIN_SESSION_LISTE_COLONNES, SESSION_TRI_COLONNES,
};
use crate::models::afrolang::{
    RefuserLienRequest, RessourceSalleResponse, RessourceSalleRow, RESSOURCE_SALLE_COLONNES,
};
use crate::models::notification;
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/sessions — historique paginé + filtres
pub async fn lister_sessions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    query: web::Query<AdminSessionQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let pagination = PaginationParams {
        page: query.page,
        par_page: query.par_page,
        tri_par: query.tri_par.clone(),
        tri_dir: query.tri_dir.clone(),
    };
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();
    let tri = pagination.colonne_tri(SESSION_TRI_COLONNES, "created_at");
    let dir = pagination.direction_tri();

    let mut conditions = vec!["1=1".to_string()];
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = query.recherche {
        if !recherche.is_empty() {
            conditions.push(format!("se.titre ILIKE ${}", bind_index));
            str_binds.push(format!("%{}%", recherche));
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref etat) = query.etat {
        if !etat.is_empty() {
            conditions.push(format!("se.etat::text = ${}", bind_index));
            str_binds.push(etat.clone());
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(salle_id) = query.salle_id {
        conditions.push(format!(
            "sp.salle_id = ${}", bind_index
        ));
        uuid_binds.push(salle_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(salle_privee_id) = query.salle_privee_id {
        conditions.push(format!("se.salle_privee_id = ${}", bind_index));
        uuid_binds.push(salle_privee_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(moderateur_id) = query.moderateur_id {
        conditions.push(format!("se.moderateur_id = ${}", bind_index));
        uuid_binds.push(moderateur_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(ref date_debut) = query.date_debut {
        if !date_debut.is_empty() {
            conditions.push(format!("se.created_at >= ${}::timestamptz", bind_index));
            str_binds.push(date_debut.clone());
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref date_fin) = query.date_fin {
        if !date_fin.is_empty() {
            conditions.push(format!("se.created_at <= ${}::timestamptz", bind_index));
            str_binds.push(date_fin.clone());
            param_types.push("str");
            bind_index += 1;
        }
    }

    let _ = bind_index;
    let where_clause = conditions.join(" AND ");

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM afrolang.session se
         JOIN afrolang.salle_privee sp ON se.salle_privee_id = sp.id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    let mut str_idx = 0;
    let mut uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => { count_q = count_q.bind(&str_binds[str_idx]); str_idx += 1; }
            "uuid" => { count_q = count_q.bind(uuid_binds[uuid_idx]); uuid_idx += 1; }
            _ => {}
        }
    }
    let total = count_q.fetch_one(pool.get_ref()).await?;

    // Data
    let data_sql = format!(
        "SELECT {} FROM afrolang.session se
         JOIN afrolang.salle_privee sp ON se.salle_privee_id = sp.id
         LEFT JOIN afrolang.salle s ON sp.salle_id = s.id
         LEFT JOIN iam.utilisateur u ON se.moderateur_id = u.id
         WHERE {} ORDER BY se.{} {} LIMIT {} OFFSET {}",
        ADMIN_SESSION_LISTE_COLONNES, where_clause, tri, dir, par_page, offset
    );
    let mut data_q = sqlx::query_as::<_, AdminSessionListeResponse>(&data_sql);
    str_idx = 0;
    uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => { data_q = data_q.bind(&str_binds[str_idx]); str_idx += 1; }
            "uuid" => { data_q = data_q.bind(uuid_binds[uuid_idx]); uuid_idx += 1; }
            _ => {}
        }
    }
    let sessions = data_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(sessions, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/sessions/{id} — détail avec participants
pub async fn obtenir_session(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM afrolang.session se
         JOIN afrolang.salle_privee sp ON se.salle_privee_id = sp.id
         LEFT JOIN afrolang.salle s ON sp.salle_id = s.id
         LEFT JOIN iam.utilisateur u ON se.moderateur_id = u.id
         LEFT JOIN iam.utilisateur cr ON se.cree_par = cr.id
         WHERE se.id = $1",
        ADMIN_SESSION_DETAIL_COLONNES
    );

    let row = sqlx::query_as::<_, AdminSessionDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Session non trouvée".into()))?;

    // Charger les participants
    let participants = sqlx::query_as::<_, AdminSessionParticipantResponse>(
        "SELECT p.id, p.utilisateur_id,
                u.nom AS utilisateur_nom, u.prenom AS utilisateur_prenom,
                p.role_session, p.rejoint_at, p.quitte_at, p.duree_secondes
         FROM afrolang.session_participant p
         LEFT JOIN iam.utilisateur u ON p.utilisateur_id = u.id
         WHERE p.session_id = $1
         ORDER BY p.rejoint_at ASC"
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(participants)),
        error: None,
    }))
}

/// GET /api/admin/sessions/{id}/participants
pub async fn lister_participants(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let session_id = path.into_inner();

    // Vérifier que la session existe
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM afrolang.session WHERE id = $1)"
    )
    .bind(session_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !exists {
        return Err(ApiErreur::NonTrouve("Session non trouvée".into()));
    }

    let participants = sqlx::query_as::<_, AdminSessionParticipantResponse>(
        "SELECT p.id, p.utilisateur_id,
                u.nom AS utilisateur_nom, u.prenom AS utilisateur_prenom,
                p.role_session, p.rejoint_at, p.quitte_at, p.duree_secondes
         FROM afrolang.session_participant p
         LEFT JOIN iam.utilisateur u ON p.utilisateur_id = u.id
         WHERE p.session_id = $1
         ORDER BY p.rejoint_at ASC"
    )
    .bind(session_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(participants),
        error: None,
    }))
}

/// GET /api/admin/sessions/{id}/tableau-blanc
pub async fn obtenir_tableau_blanc(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let session_id = path.into_inner();

    let tableau = sqlx::query_as::<_, AdminTableauBlancResponse>(
        "SELECT tb.id, tb.session_id, tb.donnees, tb.version, tb.created_at, tb.updated_at
         FROM afrolang.tableau_blanc tb
         WHERE tb.session_id = $1"
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match tableau {
        Some(tb) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(tb),
            error: None,
        })),
        None => Ok(HttpResponse::Ok().json(ApiResponse::<()> {
            success: true,
            data: None,
            error: None,
        })),
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Feature 005 — US6 : Validation des liens externes
// ══════════════════════════════════════════════════════════════════════════

/// Vérifie qu'un admin a la permission OU est modérateur attitré de la salle ciblée
async fn admin_ou_moderateur(
    admin: &AdminUtilisateur,
    pool: &PgPool,
    salle_id: Uuid,
) -> Result<(), ApiErreur> {
    if admin.a_permission("afrolang", "moderer") || admin.a_permission("afrolang", "modifier") {
        return Ok(());
    }
    let mod_actif: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.salle_moderateur
            WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
        )",
    )
    .bind(salle_id)
    .bind(admin.id)
    .fetch_one(pool)
    .await?;
    if mod_actif {
        Ok(())
    } else {
        Err(ApiErreur::NonAutorise(
            "Seul un admin ou un modérateur attitré peut modérer ce lien".into(),
        ))
    }
}

/// GET /api/admin/afrolang/ressources/en-attente — File des liens à modérer
pub async fn lister_liens_en_attente(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let sql = format!(
        "SELECT {},
            u.nom AS auteur_nom,
            u.prenom AS auteur_prenom
         FROM afrolang.ressource_salle rs
         LEFT JOIN iam.utilisateur u ON u.id = rs.ajoute_par
         WHERE rs.type = 'lien_externe'
           AND rs.etat = 'en_attente_validation'
           AND rs.deleted_at IS NULL
         ORDER BY rs.created_at ASC",
        RESSOURCE_SALLE_COLONNES
    );

    let rows = sqlx::query_as::<_, RessourceSalleRow>(&sql)
        .fetch_all(pool.get_ref())
        .await?;

    let items: Vec<RessourceSalleResponse> = rows.iter().map(|r| r.to_response()).collect();
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
}

/// POST /api/admin/afrolang/ressources/{id}/publier — Publier un lien en attente
pub async fn publier_lien(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = path.into_inner();

    let info: Option<(Uuid, Uuid, String)> = sqlx::query_as(
        "SELECT salle_id, ajoute_par, etat::TEXT FROM afrolang.ressource_salle
         WHERE id = $1 AND type = 'lien_externe' AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (salle_id, ajoute_par, etat) =
        info.ok_or_else(|| ApiErreur::NonTrouve("Lien introuvable".into()))?;
    if etat != "en_attente_validation" {
        return Err(ApiErreur::Validation(format!(
            "Le lien n'est pas en attente (état actuel : {})",
            etat
        )));
    }

    admin_ou_moderateur(&admin, pool.get_ref(), salle_id).await?;

    sqlx::query(
        "UPDATE afrolang.ressource_salle
         SET etat = 'publiee'::afrolang.etat_ressource,
             valide_par = $2, valide_at = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "afrolang.lien.publie",
        "afrolang",
        "ressource_salle",
        Some(id),
        Some(serde_json::json!({ "etat": "en_attente_validation" })),
        Some(serde_json::json!({ "etat": "publiee" })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    let lien = format!("/afrolang/{}", salle_id);
    notification::creer_notification(
        pool.get_ref(),
        ajoute_par,
        notification::afrolang::LIEN_EXTERNE_PUBLIE,
        "Votre lien externe a été publié.",
        Some(&lien),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": "publiee" })),
        error: None,
    }))
}

/// POST /api/admin/afrolang/ressources/{id}/refuser — Refuser un lien en attente
pub async fn refuser_lien(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<RefuserLienRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let id = path.into_inner();

    let motif = body.motif_refus.trim();
    if motif.len() < 5 {
        return Err(ApiErreur::Validation(
            "Le motif de refus doit contenir au moins 5 caractères".into(),
        ));
    }

    let info: Option<(Uuid, Uuid, String)> = sqlx::query_as(
        "SELECT salle_id, ajoute_par, etat::TEXT FROM afrolang.ressource_salle
         WHERE id = $1 AND type = 'lien_externe' AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (salle_id, ajoute_par, etat) =
        info.ok_or_else(|| ApiErreur::NonTrouve("Lien introuvable".into()))?;
    if etat != "en_attente_validation" {
        return Err(ApiErreur::Validation(format!(
            "Le lien n'est pas en attente (état actuel : {})",
            etat
        )));
    }

    admin_ou_moderateur(&admin, pool.get_ref(), salle_id).await?;

    sqlx::query(
        "UPDATE afrolang.ressource_salle
         SET etat = 'refusee'::afrolang.etat_ressource,
             motif_refus = $2,
             valide_par = $3, valide_at = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(motif)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "afrolang.lien.refuse",
        "afrolang",
        "ressource_salle",
        Some(id),
        Some(serde_json::json!({ "etat": "en_attente_validation" })),
        Some(serde_json::json!({ "etat": "refusee", "motif_refus": motif })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    let lien = format!("/afrolang/{}", salle_id);
    notification::creer_notification(
        pool.get_ref(),
        ajoute_par,
        notification::afrolang::LIEN_EXTERNE_REFUSE,
        &format!("Votre lien externe a été refusé. Motif : {}", motif),
        Some(&lien),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": id,
            "etat": "refusee",
            "motif_refus": motif,
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Feature 005 — Phase 9 : Archivage & désactivation cascade
// ══════════════════════════════════════════════════════════════════════════

/// Helper interne : archive toutes les salles privées actives d'un utilisateur.
///
/// Refonte 2026-04 : plus de mécanisme d'adhésion, donc plus de liste
/// d'abonnés à notifier. Seul l'auteur de chaque salle est impacté (il est
/// à l'origine de l'action ou ciblé par la modération admin).
pub async fn archiver_salles_privees_utilisateur_impl(
    pool: &PgPool,
    admin_id: Option<Uuid>,
    utilisateur_cible: Uuid,
    ip: Option<&str>,
    user_agent: Option<&str>,
) -> Result<i64, ApiErreur> {
    let ids: Vec<Uuid> = sqlx::query_scalar(
        "UPDATE afrolang.salle_privee
         SET archivee_at = NOW(), updated_at = NOW()
         WHERE cree_par = $1
           AND archivee_at IS NULL
           AND deleted_at IS NULL
         RETURNING id",
    )
    .bind(utilisateur_cible)
    .fetch_all(pool)
    .await?;

    let total = ids.len() as i64;

    for salle_privee_id in ids {
        audit::log_action(
            pool,
            admin_id,
            "afrolang.salle_privee.archivee",
            "afrolang",
            "salle_privee",
            Some(salle_privee_id),
            Some(serde_json::json!({ "archivee_at": null })),
            Some(serde_json::json!({ "archivee_at": "NOW()" })),
            ip,
            user_agent,
        )
        .await;
    }

    Ok(total)
}

/// POST /api/admin/afrolang/salles-privees/archiver-batch-utilisateur — Archivage en lot
pub async fn archiver_batch_utilisateur(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");

    let utilisateur_id = body
        .get("utilisateur_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| ApiErreur::Validation("utilisateur_id requis".into()))?;

    let total = archiver_salles_privees_utilisateur_impl(
        pool.get_ref(),
        Some(admin.id),
        utilisateur_id,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "utilisateur_id": utilisateur_id,
            "salles_archivees": total,
        })),
        error: None,
    }))
}

/// POST /api/admin/afrolang/salles-privees/{id}/archiver — Archivage manuel d'une salle
pub async fn archiver_salle_privee(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");

    let id = path.into_inner();

    let deja_archivee: Option<Option<chrono::DateTime<chrono::Utc>>> = sqlx::query_scalar(
        "SELECT archivee_at FROM afrolang.salle_privee
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    match deja_archivee {
        None => return Err(ApiErreur::NonTrouve("Salle privée introuvable".into())),
        Some(Some(_)) => {
            return Err(ApiErreur::Validation("Salle déjà archivée".into()));
        }
        _ => {}
    }

    sqlx::query(
        "UPDATE afrolang.salle_privee
         SET archivee_at = NOW(), updated_at = NOW() WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    // Refonte 2026-04 : plus d'abonnés à notifier (table `salle_privee_adhesion`
    // supprimée). Seul l'auteur est informé via la notification ci-dessous.
    let auteur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    if let Some(auteur) = auteur_id {
        let lien = format!("/afrolang/salle-privee/{}", id);
        notification::creer_notification(
            pool.get_ref(),
            auteur,
            notification::afrolang::SALLE_PRIVEE_ARCHIVEE,
            "Votre salle privée a été archivée par un administrateur.",
            Some(&lien),
        )
        .await;
    }

    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "afrolang.salle_privee.archivee_manuel",
        "afrolang",
        "salle_privee",
        Some(id),
        None,
        Some(serde_json::json!({ "archivee_at": "NOW()" })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "archivee": true })),
        error: None,
    }))
}

/// POST /api/admin/afrolang/salles/{id}/desactiver — Désactivation cascade (T108a)
pub async fn desactiver_salle_publique_avec_cascade(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");

    let salle_id = path.into_inner();

    let mut tx = pool.begin().await?;

    let salle_etat: Option<(bool, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT actif, deleted_at FROM afrolang.salle WHERE id = $1 FOR UPDATE",
    )
    .bind(salle_id)
    .fetch_optional(&mut *tx)
    .await?;
    let (actif, _deleted_at) =
        salle_etat.ok_or_else(|| ApiErreur::NonTrouve("Salle publique introuvable".into()))?;
    if !actif {
        return Err(ApiErreur::Validation("Salle déjà désactivée".into()));
    }

    // Refonte 2026-04 : archivage en cascade des salles privées d'une salle
    // publique désactivée. Plus de table d'adhésion — on remonte juste les
    // paires (salle_privée_id, auteur) pour notifier les auteurs.
    let salles_privees: Vec<(Uuid, Uuid)> = sqlx::query_as(
        "UPDATE afrolang.salle_privee
         SET archivee_at = NOW(), updated_at = NOW()
         WHERE salle_id = $1
           AND archivee_at IS NULL
           AND deleted_at IS NULL
         RETURNING id, cree_par",
    )
    .bind(salle_id)
    .fetch_all(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE afrolang.salle
         SET actif = FALSE, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(salle_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // Notifications + audit par salle privée
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);

    for (sp_id, auteur_id) in &salles_privees {
        let lien = format!("/afrolang/salle-privee/{}", sp_id);
        notification::creer_notification(
            pool.get_ref(),
            *auteur_id,
            notification::afrolang::SALLE_PRIVEE_ARCHIVEE,
            "La salle publique associée a été désactivée — votre salle privée a été archivée.",
            Some(&lien),
        )
        .await;
        audit::log_action(
            pool.get_ref(),
            Some(admin.id),
            "afrolang.salle_privee.archivee_cascade",
            "afrolang",
            "salle_privee",
            Some(*sp_id),
            None,
            Some(serde_json::json!({ "archivee_at": "NOW()", "raison": "salle_publique_desactivee" })),
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
    }

    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "afrolang.salle.desactivee",
        "afrolang",
        "salle",
        Some(salle_id),
        Some(serde_json::json!({ "actif": true })),
        Some(serde_json::json!({
            "actif": false,
            "salles_privees_archivees": salles_privees.len(),
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": salle_id,
            "salles_privees_archivees": salles_privees.len(),
        })),
        error: None,
    }))
}

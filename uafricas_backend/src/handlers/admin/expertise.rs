use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::SmtpConfig;
use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::expertise::{
    AdminDemandeExpertiseListeResponse, AdminDemandeExpertiseQueryParams,
    AdminDemandeExpertiseResponse, AdminDemandeExpertiseRow, RejeterExpertiseBody,
};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

const ADMIN_EXPERTISE_COLONNES: &str = "
    e.id,
    e.utilisateur_id,
    u.nom,
    u.prenom,
    u.email,
    u.photo_url,
    e.domaine::text AS domaine,
    e.biographie,
    e.nb_annees_experience,
    e.portfolio,
    e.situations_professionnelles::text[] AS situations_professionnelles,
    p.nom AS pays_nom,
    e.statut::text AS statut,
    e.commentaire_admin,
    (SELECT v.nom || ' ' || v.prenom
     FROM iam.utilisateur v WHERE v.id = e.valide_par) AS valide_par_nom,
    e.date_validation,
    e.created_at
";

/// Envoyer un email de decision en arriere-plan (fire-and-forget)
fn envoyer_decision_async(
    smtp_config: SmtpConfig,
    validee: bool,
    email: String,
    prenom: String,
    lien_ou_motif: String,
) {
    tokio::spawn(async move {
        let resultat = if validee {
            crate::email::envoyer_email_expertise_validee(&smtp_config, &email, &prenom, &lien_ou_motif)
                .await
        } else {
            crate::email::envoyer_email_expertise_refusee(&smtp_config, &email, &prenom, &lien_ou_motif)
                .await
        };
        if let Err(e) = resultat {
            log::error!("Echec envoi email de decision d'expertise a {}: {}", email, e);
        }
    });
}

// ────────────────────────────────────────────────────────────────
// US2 — Liste des demandes
// ────────────────────────────────────────────────────────────────

/// GET /api/admin/experts
pub async fn lister_demandes(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminDemandeExpertiseQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "expertise", "voir");

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions = vec!["e.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref statut) = params.statut {
        let s = statut.trim();
        if !s.is_empty() {
            conditions.push(format!("e.statut::text = ${}", bind_index));
            bind_values.push(s.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            let terme = format!("%{}%", r.to_lowercase());
            conditions.push(format!(
                "(LOWER(u.nom) LIKE ${bi} OR LOWER(u.prenom) LIKE ${bi} \
                 OR LOWER(e.biographie) LIKE ${bi} OR LOWER(e.domaine::text) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    let count_query = format!(
        "SELECT COUNT(*) FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_query = format!(
        "SELECT {} FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE {}
         ORDER BY e.created_at DESC
         LIMIT ${} OFFSET ${}",
        ADMIN_EXPERTISE_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, AdminDemandeExpertiseRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;
    let demandes: Vec<AdminDemandeExpertiseResponse> =
        rows.iter().map(|r| r.to_response()).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminDemandeExpertiseListeResponse {
            demandes,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/admin/experts/{id}
pub async fn obtenir_demande(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "expertise", "voir");

    let demande_id = chemin.into_inner();

    let select_query = format!(
        "SELECT {} FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE e.id = $1 AND e.deleted_at IS NULL",
        ADMIN_EXPERTISE_COLONNES
    );

    let row = sqlx::query_as::<_, AdminDemandeExpertiseRow>(&select_query)
        .bind(demande_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Demande {} non trouvee", demande_id)))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_detail_response()),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// US2 — Decision (validation / refus)
// ────────────────────────────────────────────────────────────────

#[derive(sqlx::FromRow)]
struct DemandeDecisionRow {
    utilisateur_id: Uuid,
    statut: String,
    email: String,
    prenom: String,
}

/// Charger la demande pour decision (commun valider/rejeter) — garde 404 + 409
async fn charger_pour_decision(
    pool: &PgPool,
    demande_id: Uuid,
) -> Result<DemandeDecisionRow, ApiErreur> {
    let demande = sqlx::query_as::<_, DemandeDecisionRow>(
        "SELECT e.utilisateur_id, e.statut::text AS statut, u.email, u.prenom
         FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         WHERE e.id = $1 AND e.deleted_at IS NULL",
    )
    .bind(demande_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Demande {} non trouvee", demande_id)))?;

    if demande.statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette demande a deja ete traitee".to_string(),
        ));
    }

    Ok(demande)
}

/// PATCH /api/admin/experts/{id}/valider
pub async fn valider_demande(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    smtp_config: web::Data<SmtpConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "expertise", "valider");

    let demande_id = chemin.into_inner();
    let demande = charger_pour_decision(pool.get_ref(), demande_id).await?;

    let mut tx = pool.begin().await?;
    sqlx::query(
        "UPDATE iam.expertise
         SET statut = 'valide', valide_par = $2, date_validation = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(demande_id)
    .bind(admin.id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    // Email d'approbation (fire-and-forget)
    let lien_fiche = format!(
        "{}/experts/{}",
        smtp_config.frontend_url, demande.utilisateur_id
    );
    envoyer_decision_async(
        smtp_config.get_ref().clone(),
        true,
        demande.email.clone(),
        demande.prenom.clone(),
        lien_fiche,
    );

    // Audit non-bloquant
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "VALIDATE",
        "iam",
        "expertise",
        Some(demande_id),
        None,
        Some(serde_json::json!({ "statut": "valide", "valide_par": admin.id })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": demande_id, "statut": "valide" })),
        error: None,
    }))
}

/// PATCH /api/admin/experts/{id}/rejeter
pub async fn rejeter_demande(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    smtp_config: web::Data<SmtpConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<RejeterExpertiseBody>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "expertise", "valider");

    let demande_id = chemin.into_inner();
    let commentaire = body.commentaire_admin.trim().to_string();
    if commentaire.is_empty() {
        return Err(ApiErreur::Validation(
            "Un commentaire expliquant le refus est obligatoire".to_string(),
        ));
    }

    let demande = charger_pour_decision(pool.get_ref(), demande_id).await?;

    let mut tx = pool.begin().await?;
    sqlx::query(
        "UPDATE iam.expertise
         SET statut = 'refuse', commentaire_admin = $2,
             valide_par = $3, date_validation = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(demande_id)
    .bind(&commentaire)
    .bind(admin.id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    // Email de refus (avec motif, fire-and-forget)
    envoyer_decision_async(
        smtp_config.get_ref().clone(),
        false,
        demande.email.clone(),
        demande.prenom.clone(),
        commentaire.clone(),
    );

    // Audit non-bloquant
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "REJECT",
        "iam",
        "expertise",
        Some(demande_id),
        None,
        Some(serde_json::json!({ "statut": "refuse", "valide_par": admin.id })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": demande_id, "statut": "refuse" })),
        error: None,
    }))
}

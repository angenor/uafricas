use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::biblio_humaine::{
    AdminDemandeBiblioListeResponse, AdminDemandeBiblioQueryParams,
    AdminDemandeBiblioResponse, AdminDemandeBiblioRow, TraiterDemandeBody,
};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

const ADMIN_BIBLIO_COLONNES: &str = "
    d.id,
    d.utilisateur_id,
    u.nom,
    u.prenom,
    u.email,
    u.photo_url,
    d.fonction,
    d.biographie,
    p.nom AS pays_nom,
    d.statut::TEXT AS statut,
    COALESCE(
        (SELECT string_agg(sb.nom, ', ' ORDER BY sb.nom)
         FROM iam.demande_biblio_specialite ds2
         JOIN iam.specialite_bibliotheque sb ON sb.id = ds2.specialite_id
         WHERE ds2.demande_id = d.id),
        ''
    ) AS specialites_noms,
    d.commentaire_admin,
    (SELECT u2.nom || ' ' || u2.prenom
     FROM iam.utilisateur u2 WHERE u2.id = d.traite_par) AS traite_par_nom,
    d.traite_le,
    d.created_at
";

// ────────────────────────────────────────────────────────────────
// US1 — Liste des demandes
// ────────────────────────────────────────────────────────────────

/// GET /api/admin/bibliotheques-humaines
pub async fn lister_demandes(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminDemandeBiblioQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "bibliotheque_humaine", "voir");

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions = vec!["d.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref statut) = params.statut {
        let s = statut.trim();
        if !s.is_empty() {
            conditions.push(format!("d.statut::TEXT = ${}", bind_index));
            bind_values.push(s.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            let terme = format!("%{}%", r.to_lowercase());
            conditions.push(format!(
                "(LOWER(u.nom) LIKE ${bi} OR LOWER(u.prenom) LIKE ${bi} OR LOWER(u.email) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    let count_query = format!(
        "SELECT COUNT(*) FROM iam.demande_biblio_humaine d
         JOIN iam.utilisateur u ON u.id = d.utilisateur_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_query = format!(
        "SELECT {} FROM iam.demande_biblio_humaine d
         JOIN iam.utilisateur u ON u.id = d.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = d.pays_origine_id
         WHERE {}
         ORDER BY d.created_at DESC
         LIMIT ${} OFFSET ${}",
        ADMIN_BIBLIO_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, AdminDemandeBiblioRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;
    let demandes: Vec<AdminDemandeBiblioResponse> = rows.iter().map(|r| r.to_response()).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminDemandeBiblioListeResponse {
            demandes,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// US2 — Détail, validation, rejet
// ────────────────────────────────────────────────────────────────

/// GET /api/admin/bibliotheques-humaines/{id}
pub async fn obtenir_demande(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "bibliotheque_humaine", "voir");

    let demande_id = chemin.into_inner();

    let select_query = format!(
        "SELECT {} FROM iam.demande_biblio_humaine d
         JOIN iam.utilisateur u ON u.id = d.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = d.pays_origine_id
         WHERE d.id = $1 AND d.deleted_at IS NULL",
        ADMIN_BIBLIO_COLONNES
    );

    let row = sqlx::query_as::<_, AdminDemandeBiblioRow>(&select_query)
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

/// PATCH /api/admin/bibliotheques-humaines/{id}/valider
/// Transaction atomique : statut → valide, utilisateur.bibliotheque_humain = TRUE,
/// copie fonction/biographie/pays, insere specialites, notification, audit
pub async fn valider_demande(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "bibliotheque_humaine", "modifier");

    let demande_id = chemin.into_inner();

    #[derive(sqlx::FromRow)]
    struct DemandeInfoRow {
        utilisateur_id: Uuid,
        fonction: String,
        biographie: String,
        pays_origine_id: Option<Uuid>,
    }

    let demande = sqlx::query_as::<_, DemandeInfoRow>(
        "SELECT utilisateur_id, fonction, biographie, pays_origine_id
         FROM iam.demande_biblio_humaine
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(demande_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Demande {} non trouvee", demande_id)))?;

    let utilisateur_id = demande.utilisateur_id;

    let mut tx = pool.begin().await?;

    // 1. Mettre a jour le statut de la demande
    sqlx::query(
        "UPDATE iam.demande_biblio_humaine
         SET statut = 'valide', traite_par = $2, traite_le = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(demande_id)
    .bind(admin.id)
    .execute(&mut *tx)
    .await?;

    // 2. Mettre a jour le profil utilisateur
    sqlx::query(
        "UPDATE iam.utilisateur
         SET bibliotheque_humain = TRUE,
             fonction = $2,
             biographie = $3,
             pays_origine_id = $4,
             updated_at = NOW()
         WHERE id = $1",
    )
    .bind(utilisateur_id)
    .bind(&demande.fonction)
    .bind(&demande.biographie)
    .bind(demande.pays_origine_id)
    .execute(&mut *tx)
    .await?;

    // 3. Remplacer les specialites actives par celles de la demande
    sqlx::query("DELETE FROM iam.utilisateur_specialite WHERE utilisateur_id = $1")
        .bind(utilisateur_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "INSERT INTO iam.utilisateur_specialite (utilisateur_id, specialite_id)
         SELECT $1, ds.specialite_id
         FROM iam.demande_biblio_specialite ds
         WHERE ds.demande_id = $2
         ON CONFLICT DO NOTHING",
    )
    .bind(utilisateur_id)
    .bind(demande_id)
    .execute(&mut *tx)
    .await?;

    // 4. Notification in-app (US5)
    sqlx::query(
        "INSERT INTO iam.notification_biblio_humaine (utilisateur_id, type)
         VALUES ($1, 'approuvee')",
    )
    .bind(utilisateur_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // 5. Audit non-bloquant
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "valider",
        "iam",
        "demande_biblio_humaine",
        Some(demande_id),
        None,
        Some(serde_json::json!({ "statut": "valide", "traite_par": admin.id })),
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

/// PATCH /api/admin/bibliotheques-humaines/{id}/rejeter
pub async fn rejeter_demande(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<Option<TraiterDemandeBody>>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "bibliotheque_humaine", "modifier");

    let demande_id = chemin.into_inner();
    let commentaire = body
        .as_ref()
        .and_then(|b| b.commentaire.as_deref())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    #[derive(sqlx::FromRow)]
    struct DemandeStatutRow {
        utilisateur_id: Uuid,
        statut: String,
    }

    let demande = sqlx::query_as::<_, DemandeStatutRow>(
        "SELECT utilisateur_id, statut::TEXT AS statut
         FROM iam.demande_biblio_humaine
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(demande_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Demande {} non trouvee", demande_id)))?;

    let utilisateur_id = demande.utilisateur_id;

    let mut tx = pool.begin().await?;

    // 1. Rejeter la demande
    sqlx::query(
        "UPDATE iam.demande_biblio_humaine
         SET statut = 'rejete', commentaire_admin = $2,
             traite_par = $3, traite_le = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(demande_id)
    .bind(commentaire.as_deref())
    .bind(admin.id)
    .execute(&mut *tx)
    .await?;

    // 2. Repasser bibliotheque_humain a FALSE si la demande etait validee
    if demande.statut == "valide" {
        sqlx::query(
            "UPDATE iam.utilisateur
             SET bibliotheque_humain = FALSE, updated_at = NOW()
             WHERE id = $1",
        )
        .bind(utilisateur_id)
        .execute(&mut *tx)
        .await?;
    }

    // 3. Notification in-app (US5)
    sqlx::query(
        "INSERT INTO iam.notification_biblio_humaine (utilisateur_id, type, commentaire)
         VALUES ($1, 'rejetee', $2)",
    )
    .bind(utilisateur_id)
    .bind(commentaire.as_deref())
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // 4. Audit non-bloquant
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "rejeter",
        "iam",
        "demande_biblio_humaine",
        Some(demande_id),
        None,
        Some(serde_json::json!({ "statut": "rejete", "traite_par": admin.id })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": demande_id, "statut": "rejete" })),
        error: None,
    }))
}

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::propositions_afrolang::{
    AdminPropositionQueryParams, ApprouverPropositionRequest, RefuserPropositionRequest,
};
use crate::models::afrolang::{
    PropositionSalleAdminResponse, PropositionSalleRow, PROPOSITION_SALLE_COLONNES,
};
use crate::models::notification;
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ── Colonnes SQL pour la liste admin ──────────────────────────────

const ADMIN_PROPOSITION_LISTE_COLONNES: &str =
    "ps.id, ps.nom_groupe_ethnique, ps.pays_id, ps.groupe_ethnique_id,
     ps.langue_cible, ps.description, ps.etat::TEXT AS etat,
     ps.motif_refus, ps.salle_id_creee, ps.propose_par,
     ps.decide_par, ps.decide_at,
     ps.created_at, ps.updated_at, ps.deleted_at,
     u.nom AS proposant_nom,
     u.prenom AS proposant_prenom,
     u.email AS proposant_email";

const ADMIN_PROPOSITION_TRI_COLONNES: &[&str] = &["created_at", "updated_at", "etat"];

// ── Helpers ───────────────────────────────────────────────────────

fn row_to_admin_response(
    row: PropositionSalleRow,
    salle_existante_id: Option<Uuid>,
    proposition_doublon_id: Option<Uuid>,
) -> PropositionSalleAdminResponse {
    let proposant_nom_complet = match (&row.proposant_prenom, &row.proposant_nom) {
        (Some(p), Some(n)) => Some(format!("{} {}", p, n).trim().to_string()),
        (None, Some(n)) => Some(n.clone()),
        _ => None,
    };
    PropositionSalleAdminResponse {
        id: row.id,
        nom_groupe_ethnique: row.nom_groupe_ethnique,
        pays_id: row.pays_id,
        groupe_ethnique_id: row.groupe_ethnique_id,
        langue_cible: row.langue_cible,
        description: row.description,
        etat: row.etat,
        motif_refus: row.motif_refus,
        salle_id_creee: row.salle_id_creee,
        propose_par: row.propose_par,
        proposant_nom_complet,
        proposant_email: row.proposant_email,
        decide_par: row.decide_par,
        decide_at: row.decide_at,
        salle_existante_id,
        proposition_doublon_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

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

// ══════════════════════════════════════════════════════════════════
// GET /api/admin/afrolang/propositions
// ══════════════════════════════════════════════════════════════════

pub async fn lister_propositions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    query: web::Query<AdminPropositionQueryParams>,
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
    let tri = pagination.colonne_tri(ADMIN_PROPOSITION_TRI_COLONNES, "created_at");
    let dir = pagination.direction_tri();

    let mut conditions = vec!["ps.deleted_at IS NULL".to_string()];
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    let etat = query.etat.as_deref().unwrap_or("en_attente");
    if !etat.is_empty() && etat != "tous" {
        conditions.push(format!("ps.etat::TEXT = ${}", bind_index));
        str_binds.push(etat.to_string());
        param_types.push("str");
        bind_index += 1;
    }

    if let Some(ref q) = query.q {
        if !q.trim().is_empty() {
            conditions.push(format!("ps.nom_groupe_ethnique ILIKE ${}", bind_index));
            str_binds.push(format!("%{}%", q.trim()));
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(pays_id) = query.pays_id {
        conditions.push(format!("ps.pays_id = ${}", bind_index));
        uuid_binds.push(pays_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    let _ = bind_index;
    let where_clause = conditions.join(" AND ");

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM afrolang.proposition_salle ps WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    let mut str_idx = 0usize;
    let mut uuid_idx = 0usize;
    for pt in &param_types {
        match *pt {
            "str" => {
                count_q = count_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                count_q = count_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    let total = count_q.fetch_one(pool.get_ref()).await?;

    // Data
    let data_sql = format!(
        "SELECT {}
         FROM afrolang.proposition_salle ps
         LEFT JOIN iam.utilisateur u ON u.id = ps.propose_par
         WHERE {}
         ORDER BY ps.{} {}
         LIMIT {} OFFSET {}",
        ADMIN_PROPOSITION_LISTE_COLONNES, where_clause, tri, dir, par_page, offset
    );
    let mut data_q = sqlx::query_as::<_, PropositionSalleRow>(&data_sql);
    str_idx = 0;
    uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => {
                data_q = data_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                data_q = data_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    let rows = data_q.fetch_all(pool.get_ref()).await?;
    let items: Vec<PropositionSalleAdminResponse> = rows
        .into_iter()
        .map(|r| row_to_admin_response(r, None, None))
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════
// GET /api/admin/afrolang/propositions/{id}
// ══════════════════════════════════════════════════════════════════

pub async fn obtenir_proposition(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let id = path.into_inner();

    let sql = format!(
        "SELECT {}
         FROM afrolang.proposition_salle ps
         LEFT JOIN iam.utilisateur u ON u.id = ps.propose_par
         WHERE ps.id = $1 AND ps.deleted_at IS NULL",
        ADMIN_PROPOSITION_LISTE_COLONNES
    );

    let row = sqlx::query_as::<_, PropositionSalleRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Proposition non trouvée".into()))?;

    // Rechercher un doublon de salle (nom normalisé)
    let salle_existante_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT s.id FROM afrolang.salle s
         JOIN country_profile.groupe_ethnique ge ON ge.id = s.groupe_ethnique_id
         WHERE lower(unaccent(ge.nom)) = lower(unaccent($1))
           AND s.deleted_at IS NULL
         LIMIT 1",
    )
    .bind(&row.nom_groupe_ethnique)
    .fetch_optional(pool.get_ref())
    .await?;

    let proposition_doublon_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM afrolang.proposition_salle
         WHERE lower(unaccent(nom_groupe_ethnique)) = lower(unaccent($1))
           AND etat = 'en_attente'
           AND id <> $2
           AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(&row.nom_groupe_ethnique)
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    let response = row_to_admin_response(row, salle_existante_id, proposition_doublon_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════
// POST /api/admin/afrolang/propositions/{id}/approuver
// ══════════════════════════════════════════════════════════════════

pub async fn approuver_proposition(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ApprouverPropositionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");

    let id = path.into_inner();

    // Charger l'état actuel
    let proposition = sqlx::query_as::<_, PropositionSalleRow>(
        &format!(
            "SELECT {} FROM afrolang.proposition_salle ps WHERE ps.id = $1 AND ps.deleted_at IS NULL",
            PROPOSITION_SALLE_COLONNES
        ),
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Proposition non trouvée".into()))?;

    if proposition.etat != "en_attente" {
        return Err(ApiErreur::Validation(format!(
            "La proposition n'est pas en attente (état actuel : {})",
            proposition.etat
        )));
    }

    // Vérifier que le groupe ethnique existe
    let groupe_nom: Option<String> = sqlx::query_scalar(
        "SELECT nom FROM country_profile.groupe_ethnique WHERE id = $1",
    )
    .bind(body.groupe_ethnique_id)
    .fetch_optional(pool.get_ref())
    .await?;
    let groupe_nom = groupe_nom.ok_or_else(|| {
        ApiErreur::Validation("Groupe ethnique cible introuvable".into())
    })?;

    // Vérifier qu'aucune salle active n'est déjà attachée à ce groupe
    let salle_existante: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM afrolang.salle
         WHERE groupe_ethnique_id = $1 AND actif = TRUE AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(body.groupe_ethnique_id)
    .fetch_optional(pool.get_ref())
    .await?;
    if let Some(sid) = salle_existante {
        return Err(ApiErreur::Validation(format!(
            "Une salle active existe déjà pour ce groupe ethnique (id={})",
            sid
        )));
    }

    let titre = body
        .titre
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| groupe_nom.clone());
    let slug = generer_slug(&titre);
    let langue_code = body.langue_code.as_deref().map(str::trim);
    let alphabet = body.alphabet.as_deref().map(str::trim);
    let dictionnaire_url = body.dictionnaire_url.as_deref().map(str::trim);
    let image = body.image_couverture_url.as_deref().map(str::trim);

    let mut tx = pool.begin().await?;

    // Créer la salle
    let salle_id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.salle
            (titre, slug, description, image_couverture_url,
             langue_cible, langue_code, alphabet, dictionnaire_url,
             groupe_ethnique_id, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING id",
    )
    .bind(&titre)
    .bind(&slug)
    .bind(proposition.description.as_deref())
    .bind(image)
    .bind(proposition.langue_cible.as_deref())
    .bind(langue_code)
    .bind(alphabet)
    .bind(dictionnaire_url)
    .bind(body.groupe_ethnique_id)
    .bind(admin.id)
    .fetch_one(&mut *tx)
    .await?;

    // Mettre à jour la proposition
    let ancien = serde_json::json!({
        "etat": proposition.etat,
        "salle_id_creee": proposition.salle_id_creee,
    });
    let nouveau = serde_json::json!({
        "etat": "approuvee",
        "salle_id_creee": salle_id,
    });

    sqlx::query(
        "UPDATE afrolang.proposition_salle
         SET etat = 'approuvee',
             salle_id_creee = $2,
             decide_par = $3,
             decide_at = NOW(),
             updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(salle_id)
    .bind(admin.id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // Notification (non-bloquant)
    let message = format!(
        "Votre proposition « {} » a été validée. La salle est disponible.",
        proposition.nom_groupe_ethnique
    );
    let lien = format!("/afrolang/{}", salle_id);
    notification::creer_notification(
        pool.get_ref(),
        proposition.propose_par,
        notification::afrolang::PROPOSITION_VALIDEE,
        &message,
        Some(&lien),
    )
    .await;

    // Audit
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "afrolang",
        "proposition_salle",
        Some(id),
        Some(ancien),
        Some(nouveau),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "afrolang",
        "salle",
        Some(salle_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "proposition_id": id,
            "salle_id": salle_id,
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════
// POST /api/admin/afrolang/propositions/{id}/refuser
// ══════════════════════════════════════════════════════════════════

pub async fn refuser_proposition(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<RefuserPropositionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");

    let id = path.into_inner();

    let motif = body.motif_refus.trim();
    if motif.len() < 5 {
        return Err(ApiErreur::Validation(
            "Le motif de refus doit contenir au moins 5 caractères".into(),
        ));
    }

    let proposition = sqlx::query_as::<_, PropositionSalleRow>(
        &format!(
            "SELECT {} FROM afrolang.proposition_salle ps WHERE ps.id = $1 AND ps.deleted_at IS NULL",
            PROPOSITION_SALLE_COLONNES
        ),
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Proposition non trouvée".into()))?;

    if proposition.etat != "en_attente" {
        return Err(ApiErreur::Validation(format!(
            "La proposition n'est pas en attente (état actuel : {})",
            proposition.etat
        )));
    }

    let ancien = serde_json::json!({
        "etat": proposition.etat,
        "motif_refus": proposition.motif_refus,
    });
    let nouveau = serde_json::json!({
        "etat": "refusee",
        "motif_refus": motif,
    });

    sqlx::query(
        "UPDATE afrolang.proposition_salle
         SET etat = 'refusee',
             motif_refus = $2,
             decide_par = $3,
             decide_at = NOW(),
             updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(motif)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    // Notification
    let message = format!(
        "Votre proposition « {} » a été refusée : {}",
        proposition.nom_groupe_ethnique, motif
    );
    notification::creer_notification(
        pool.get_ref(),
        proposition.propose_par,
        notification::afrolang::PROPOSITION_REFUSEE,
        &message,
        Some("/afrolang/proposer"),
    )
    .await;

    // Audit
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "afrolang",
        "proposition_salle",
        Some(id),
        Some(ancien),
        Some(nouveau),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "proposition_id": id,
            "etat": "refusee",
        })),
        error: None,
    }))
}

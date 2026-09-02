// ════════════════════════════════════════════════════════════════════════════
// Feature 001-admin-salles-publiques : US2
// Handlers admin pour la modération des propositions communautaires de salles.
//
// Endpoints :
//   GET    /api/admin/afrolang/propositions
//   GET    /api/admin/afrolang/propositions/{id}
//   PATCH  /api/admin/afrolang/propositions/{id}/valider
//   PATCH  /api/admin/afrolang/propositions/{id}/rejeter
// ════════════════════════════════════════════════════════════════════════════

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::afrolang::{
    DecisionRequest, PropositionAdminFiltres, PropositionListeResponse, PropositionResponse,
    PropositionSalleRow, PropositionStatut, COLONNES_PROPOSITION,
};
use crate::models::notification;
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ── Helpers SQL communs ────────────────────────────────────────────────

fn proposition_select(where_clause: &str, order_limit: &str) -> String {
    format!(
        "SELECT {cols},
            ua.nom AS auteur_nom, ua.prenom AS auteur_prenom,
            ud.nom AS decideur_nom, ud.prenom AS decideur_prenom,
            ge.nom AS groupe_ethnique_nom,
            COALESCE((SELECT json_agg(json_build_object(
                        'id', p.id, 'nom', p.nom, 'code_iso2', p.code_iso2
                     ) ORDER BY p.nom)
                     FROM unnest(ps.pays_origine_ids) AS pid(id)
                     JOIN shared.pays p ON p.id = pid.id),
                     '[]'::json) AS pays_origine_json
         FROM afrolang.proposition_salle ps
         LEFT JOIN iam.utilisateur ua ON ua.id = ps.auteur_id
         LEFT JOIN iam.utilisateur ud ON ud.id = ps.decideur
         LEFT JOIN country_profile.groupe_ethnique ge ON ge.id = ps.groupe_ethnique_id
         WHERE {where_clause}
         {order_limit}",
        cols = COLONNES_PROPOSITION,
        where_clause = where_clause,
        order_limit = order_limit,
    )
}

async fn charger_par_id(pool: &PgPool, id: Uuid) -> Result<PropositionResponse, ApiErreur> {
    let sql = proposition_select("ps.id = $1", "");
    let row = sqlx::query_as::<_, PropositionSalleRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;
    Ok(row.to_response())
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

// ══════════════════════════════════════════════════════════════════════
// GET /api/admin/afrolang/propositions
// ══════════════════════════════════════════════════════════════════════

pub async fn lister_propositions_admin(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<PropositionAdminFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * taille;

    let mut conditions: Vec<String> = vec!["1=1".to_string()];
    let mut bind_index: u32 = 1;

    let statut = params.statut.clone();
    let langue_code = params.langue_code.clone();
    let groupe_ethnique_id = params.groupe_ethnique_id;
    let auteur_id = params.auteur_id;
    let date_debut = params.date_debut;
    let date_fin = params.date_fin;

    if statut.is_some() {
        conditions.push(format!("ps.statut = ${}", bind_index));
        bind_index += 1;
    }
    if langue_code.is_some() {
        conditions.push(format!("ps.langue_code = ${}", bind_index));
        bind_index += 1;
    }
    if groupe_ethnique_id.is_some() {
        conditions.push(format!("ps.groupe_ethnique_id = ${}", bind_index));
        bind_index += 1;
    }
    if auteur_id.is_some() {
        conditions.push(format!("ps.auteur_id = ${}", bind_index));
        bind_index += 1;
    }
    if date_debut.is_some() {
        conditions.push(format!("ps.created_at >= ${}", bind_index));
        bind_index += 1;
    }
    if date_fin.is_some() {
        conditions.push(format!("ps.created_at <= ${}", bind_index));
        bind_index += 1;
    }

    let where_clause = conditions.join(" AND ");

    let order = match params.tri.as_deref() {
        Some("created_at_asc") => "ORDER BY ps.created_at ASC",
        Some("decide_at_desc") => "ORDER BY ps.decide_at DESC NULLS LAST",
        _ => "ORDER BY ps.created_at DESC",
    };

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM afrolang.proposition_salle ps WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    if let Some(ref s) = statut { count_q = count_q.bind(s); }
    if let Some(ref c) = langue_code { count_q = count_q.bind(c); }
    if let Some(g) = groupe_ethnique_id { count_q = count_q.bind(g); }
    if let Some(a) = auteur_id { count_q = count_q.bind(a); }
    if let Some(d) = date_debut { count_q = count_q.bind(d); }
    if let Some(d) = date_fin { count_q = count_q.bind(d); }
    let total = count_q.fetch_one(pool.get_ref()).await?;

    let order_limit = format!(
        "{} LIMIT ${} OFFSET ${}",
        order,
        bind_index,
        bind_index + 1
    );
    let select_sql = proposition_select(&where_clause, &order_limit);
    let mut q = sqlx::query_as::<_, PropositionSalleRow>(&select_sql);
    if let Some(ref s) = statut { q = q.bind(s); }
    if let Some(ref c) = langue_code { q = q.bind(c); }
    if let Some(g) = groupe_ethnique_id { q = q.bind(g); }
    if let Some(a) = auteur_id { q = q.bind(a); }
    if let Some(d) = date_debut { q = q.bind(d); }
    if let Some(d) = date_fin { q = q.bind(d); }
    q = q.bind(taille).bind(offset);
    let rows = q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PropositionListeResponse {
            items: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            taille,
        }),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════
// GET /api/admin/afrolang/propositions/{id}
// ══════════════════════════════════════════════════════════════════════

pub async fn obtenir_proposition_admin(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");
    let id = path.into_inner();
    let proposition = charger_par_id(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(proposition),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════
// PATCH /api/admin/afrolang/propositions/{id}/valider
// Transaction atomique : crée la salle + lignes pays_origine + bascule la
// proposition (cf. research.md Décision 3).
// ══════════════════════════════════════════════════════════════════════

pub async fn valider_proposition(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<DecisionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");
    let proposition_id = path.into_inner();

    let commentaire = body
        .commentaire
        .as_ref()
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty());

    let mut tx = pool.begin().await?;

    // SELECT FOR UPDATE
    let row: Option<(
        Uuid,
        Uuid,
        String,
        String,
        String,
        Option<String>,
        Option<Uuid>,
        Option<String>,
        Vec<Uuid>,
        PropositionStatut,
    )> = sqlx::query_as(
        "SELECT id, auteur_id, titre, description, langue_cible, langue_code,
                groupe_ethnique_id, groupe_ethnique_libre, pays_origine_ids, statut
         FROM afrolang.proposition_salle
         WHERE id = $1 FOR UPDATE",
    )
    .bind(proposition_id)
    .fetch_optional(&mut *tx)
    .await?;

    let (
        _id,
        auteur_id,
        titre,
        description,
        langue_cible,
        langue_code,
        groupe_ethnique_id,
        groupe_ethnique_libre,
        pays_origine_ids,
        statut,
    ) = row.ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;

    if statut != PropositionStatut::EnAttente {
        return Err(ApiErreur::Conflit(
            "Cette proposition a déjà été décidée".into(),
        ));
    }

    let salle_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.salle
            WHERE groupe_ethnique_id = $1 AND actif = TRUE AND deleted_at IS NULL
        )",
    )
    .bind(groupe_ethnique_id)
    .fetch_one(&mut *tx)
    .await?;
    if salle_existe {
        return Err(ApiErreur::Conflit(
            "Une salle publique existe déjà pour ce groupe ethnique".into(),
        ));
    }

    let slug = generer_slug(&titre);
    let salle_id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.salle
            (titre, slug, description, langue_cible, langue_code,
             groupe_ethnique_id, groupe_ethnique_libre, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING id",
    )
    .bind(&titre)
    .bind(&slug)
    .bind(&description)
    .bind(&langue_cible)
    .bind(langue_code.as_deref())
    .bind(groupe_ethnique_id)
    .bind(groupe_ethnique_libre.as_deref())
    .bind(auteur_id)
    .fetch_one(&mut *tx)
    .await?;

    // Pays d'origine
    for pays_id in &pays_origine_ids {
        sqlx::query(
            "INSERT INTO afrolang.salle_pays_origine (salle_id, pays_id)
             VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(salle_id)
        .bind(pays_id)
        .execute(&mut *tx)
        .await?;
    }

    sqlx::query(
        "UPDATE afrolang.proposition_salle
         SET statut = 'validee',
             decideur = $1,
             decide_at = NOW(),
             commentaire_decision = $2,
             salle_id_creee = $3,
             updated_at = NOW()
         WHERE id = $4",
    )
    .bind(admin.id)
    .bind(commentaire.as_deref())
    .bind(salle_id)
    .bind(proposition_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "VALIDATE",
        "afrolang",
        "proposition_salle",
        Some(proposition_id),
        Some(serde_json::json!({ "statut": "en_attente" })),
        Some(serde_json::json!({
            "statut": "validee",
            "salle_id_creee": salle_id,
            "commentaire_decision": commentaire,
        })),
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
        Some(serde_json::json!({
            "titre": titre,
            "groupe_ethnique_id": groupe_ethnique_id,
            "langue_cible": langue_cible,
            "via_proposition": proposition_id,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let lien = format!("/afrolang/salles/{}", salle_id);
    let message = if let Some(ref c) = commentaire {
        format!("Votre proposition a été validée. Commentaire admin : {}", c)
    } else {
        "Votre proposition de salle a été validée, la salle est créée.".to_string()
    };
    notification::creer_notification(
        pool.get_ref(),
        auteur_id,
        notification::afrolang::PROPOSITION_VALIDEE,
        &message,
        Some(&lien),
    )
    .await;

    let proposition = charger_par_id(pool.get_ref(), proposition_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "proposition": proposition,
            "salle_id": salle_id,
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════
// PATCH /api/admin/afrolang/propositions/{id}/rejeter
// ══════════════════════════════════════════════════════════════════════

pub async fn rejeter_proposition(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<DecisionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");
    let proposition_id = path.into_inner();

    let commentaire = body
        .commentaire
        .as_ref()
        .map(|c| c.trim().to_string())
        .unwrap_or_default();
    if commentaire.chars().count() < 10 {
        return Err(ApiErreur::Validation(
            "Un commentaire de rejet (≥ 10 caractères) est obligatoire".into(),
        ));
    }

    let mut tx = pool.begin().await?;

    let row: Option<(Uuid, PropositionStatut)> = sqlx::query_as(
        "SELECT auteur_id, statut FROM afrolang.proposition_salle
         WHERE id = $1 FOR UPDATE",
    )
    .bind(proposition_id)
    .fetch_optional(&mut *tx)
    .await?;

    let (auteur_id, statut) =
        row.ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;

    if statut != PropositionStatut::EnAttente {
        return Err(ApiErreur::Conflit(
            "Cette proposition a déjà été décidée".into(),
        ));
    }

    sqlx::query(
        "UPDATE afrolang.proposition_salle
         SET statut = 'rejetee',
             decideur = $1,
             decide_at = NOW(),
             commentaire_decision = $2,
             updated_at = NOW()
         WHERE id = $3",
    )
    .bind(admin.id)
    .bind(&commentaire)
    .bind(proposition_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "REJECT",
        "afrolang",
        "proposition_salle",
        Some(proposition_id),
        Some(serde_json::json!({ "statut": "en_attente" })),
        Some(serde_json::json!({
            "statut": "rejetee",
            "commentaire_decision": commentaire,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    notification::creer_notification(
        pool.get_ref(),
        auteur_id,
        notification::afrolang::PROPOSITION_REFUSEE,
        &format!("Votre proposition a été rejetée. Motif : {}", commentaire),
        Some("/afrolang/proposer"),
    )
    .await;

    let proposition = charger_par_id(pool.get_ref(), proposition_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(proposition),
        error: None,
    }))
}

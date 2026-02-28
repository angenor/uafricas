use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::retrouve_amis::{
    AdminAvisQueryParams, AdminAvisRechercheListe, AdminAvisRechercheDetailRow,
    AdminAvisRechercheListeResponse, AdminCorrespondanceRow, AdminSignalementRow,
    AdminSignalementQueryParams, AdminSignalementListe, AdminSignalementListeResponse,
    AdminStatistiques, ChangerEtatAvis, ModererSignalement,
    ADMIN_AVIS_LISTE_COLONNES, ADMIN_AVIS_DETAIL_COLONNES, ADMIN_AVIS_JOINS, ADMIN_AVIS_TRI_COLONNES,
    ADMIN_SIGNALEMENT_LISTE_COLONNES, ADMIN_SIGNALEMENT_JOINS, ADMIN_SIGNALEMENT_TRI_COLONNES,
};
use crate::models::pagination::PaginationParams;
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ── Etats valides pour les transitions admin ─────────────────
const ETATS_AVIS_TRANSITION: &[&str] = &["actif", "suspendu"];

// ══════════════════════════════════════════════════════════════
// Avis de recherche
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/retrouve-amis/avis
pub async fn lister_avis_admin(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminAvisQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "retrouve_amis", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["a.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    // Filtre recherche (nom_recherche, prenom_recherche, ville)
    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(a.nom_recherche) LIKE ${bi} OR LOWER(COALESCE(a.prenom_recherche, '')) LIKE ${bi} OR LOWER(COALESCE(a.ville, '')) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    // Filtre etat
    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("a.etat::text = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }

    // Filtre auteur_id
    if let Some(ref auteur_id) = params.auteur_id {
        let a = auteur_id.trim();
        if !a.is_empty() {
            if Uuid::parse_str(a).is_ok() {
                conditions.push(format!("a.auteur_id::text = ${}", bind_index));
                bind_values.push(a.to_string());
                bind_index += 1;
            }
        }
    }

    // Filtre pays_id
    if let Some(ref pays_id) = params.pays_id {
        let p = pays_id.trim();
        if !p.is_empty() {
            if Uuid::parse_str(p).is_ok() {
                conditions.push(format!("a.pays_id::text = ${}", bind_index));
                bind_values.push(p.to_string());
                bind_index += 1;
            }
        }
    }

    // Filtre date_debut
    if let Some(ref date_debut) = params.date_debut {
        let d = date_debut.trim();
        if !d.is_empty() {
            conditions.push(format!("a.created_at >= ${}::timestamptz", bind_index));
            bind_values.push(d.to_string());
            bind_index += 1;
        }
    }

    // Filtre date_fin
    if let Some(ref date_fin) = params.date_fin {
        let d = date_fin.trim();
        if !d.is_empty() {
            conditions.push(format!("a.created_at <= ${}::timestamptz", bind_index));
            bind_values.push(d.to_string());
            bind_index += 1;
        }
    }

    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(ADMIN_AVIS_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    // COUNT
    let count_sql = format!(
        "SELECT COUNT(*) FROM retrouve_amis.avis_recherche a
         {} WHERE {}",
        ADMIN_AVIS_JOINS, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT
    let select_sql = format!(
        "SELECT {} FROM retrouve_amis.avis_recherche a
         {} WHERE {} ORDER BY a.{} {} LIMIT {} OFFSET {}",
        ADMIN_AVIS_LISTE_COLONNES, ADMIN_AVIS_JOINS, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminAvisRechercheListe>(&select_sql);
    for v in &bind_values {
        select_q = select_q.bind(v);
    }
    let rows = select_q.fetch_all(pool.get_ref()).await?;

    // Convertir les lignes en DTOs de réponse
    // On a besoin du pays_id pour la conversion — on le récupère depuis la ligne SQL
    // Malheureusement la struct de listing n'a pas pays_id, on reconstruit depuis les params
    // En fait, la ligne n'a pas pays_id directement, mais on peut le déduire du filtre
    // ou simplement passer None et utiliser un pays_id optionnel
    // Le modèle to_response prend un Option<Uuid> pays_id — on fait une requête complémentaire
    // Solution: on query aussi a.pays_id dans une struct intermédiaire — mais le modèle existant
    // ne l'inclut pas. On passe None quand pas de filtre pays, sinon le filtre pays.
    // Pour une liste, le pays_id de chaque avis peut varier. On fait un workaround:
    // on query a.pays_id séparément ou on utilise le fait que pays_nom est déjà dans la row.
    // Le to_response demande un pays_id optionnel pour construire AdminPaysInfo.
    // Comme on n'a pas pays_id dans AdminAvisRechercheListe, on query les pays_id séparément.

    // Requête complémentaire pour récupérer les pays_id des avis listés
    let avis_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
    let pays_map: std::collections::HashMap<Uuid, Uuid> = if !avis_ids.is_empty() {
        let placeholders: Vec<String> = (1..=avis_ids.len())
            .map(|i| format!("${}", i))
            .collect();
        let pays_sql = format!(
            "SELECT id, pays_id FROM retrouve_amis.avis_recherche WHERE id IN ({}) AND pays_id IS NOT NULL",
            placeholders.join(", ")
        );
        let mut pays_q = sqlx::query_as::<_, (Uuid, Option<Uuid>)>(&pays_sql);
        for aid in &avis_ids {
            pays_q = pays_q.bind(aid);
        }
        let pays_rows = pays_q.fetch_all(pool.get_ref()).await.unwrap_or_default();
        pays_rows
            .into_iter()
            .filter_map(|(id, pays_id)| pays_id.map(|pid| (id, pid)))
            .collect()
    } else {
        std::collections::HashMap::new()
    };

    let items: Vec<_> = rows
        .iter()
        .map(|r| r.to_response(pays_map.get(&r.id).copied()))
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminAvisRechercheListeResponse {
            avis: items,
            total,
            page,
            par_page,
        }),
        error: None,
    }))
}

/// GET /api/admin/retrouve-amis/avis/{id}
pub async fn detail_avis_admin(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "retrouve_amis", "voir");
    let id = path.into_inner();

    // Requête principale
    let sql = format!(
        "SELECT {} FROM retrouve_amis.avis_recherche a
         {} WHERE a.id = $1 AND a.deleted_at IS NULL",
        ADMIN_AVIS_DETAIL_COLONNES, ADMIN_AVIS_JOINS
    );
    let row = sqlx::query_as::<_, AdminAvisRechercheDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Avis de recherche non trouve".into()))?;

    // Charger les correspondances
    let correspondances_rows = sqlx::query_as::<_, AdminCorrespondanceRow>(
        "SELECT c.id, c.score::float8 AS score, c.etat::text AS etat, c.type_cible::text AS type_cible,
                c.cible_utilisateur_id,
                u.nom AS cible_utilisateur_nom, u.prenom AS cible_utilisateur_prenom, u.email AS cible_utilisateur_email,
                c.created_at
         FROM retrouve_amis.correspondance c
         LEFT JOIN iam.utilisateur u ON u.id = c.cible_utilisateur_id
         WHERE c.avis_id = $1
         ORDER BY c.score DESC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let correspondances = correspondances_rows.iter().map(|c| c.to_info()).collect();

    // Charger les signalements
    let signalements_rows = sqlx::query_as::<_, AdminSignalementRow>(
        "SELECT s.id, s.signale_par AS signale_par_id,
                u.nom AS signale_par_nom, u.prenom AS signale_par_prenom, u.email AS signale_par_email,
                s.motif::text AS motif, s.description, s.etat::text AS etat, s.created_at
         FROM retrouve_amis.signalement s
         JOIN iam.utilisateur u ON u.id = s.signale_par
         WHERE s.avis_id = $1
         ORDER BY s.created_at DESC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let signalements = signalements_rows.iter().map(|s| s.to_info()).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(correspondances, signalements)),
        error: None,
    }))
}

/// PATCH /api/admin/retrouve-amis/avis/{id}/etat
pub async fn changer_etat_avis(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatAvis>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "retrouve_amis", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_AVIS_TRANSITION.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Transition d'etat invalide. Seules les valeurs suivantes sont acceptees: {}",
            ETATS_AVIS_TRANSITION.join(", ")
        )));
    }

    // Recuperer l'etat actuel pour l'audit
    let ancien_etat: Option<String> = sqlx::query_scalar(
        "SELECT etat::text FROM retrouve_amis.avis_recherche WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    let ancien_etat = ancien_etat
        .ok_or_else(|| ApiErreur::NonTrouve("Avis de recherche non trouve".into()))?;

    // Verifier que la transition est valide (actif <-> suspendu)
    let transition_valide = matches!(
        (ancien_etat.as_str(), etat),
        ("actif", "suspendu") | ("suspendu", "actif")
    );
    if !transition_valide {
        return Err(ApiErreur::Validation(format!(
            "Transition impossible de '{}' vers '{}'",
            ancien_etat, etat
        )));
    }

    let result = sqlx::query(
        "UPDATE retrouve_amis.avis_recherche SET etat = $1::retrouve_amis.etat_avis, updated_at = NOW()
         WHERE id = $2 AND deleted_at IS NULL",
    )
    .bind(etat)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Avis de recherche non trouve".into()));
    }

    log::info!(
        "Admin {} a change l'etat de l'avis {} de '{}' a '{}'",
        admin.id, id, ancien_etat, etat
    );

    let ancien_json = serde_json::json!({ "etat": ancien_etat });
    let nouveau_json = serde_json::json!({ "etat": etat });

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "retrouve_amis",
        "avis_recherche",
        Some(id),
        Some(ancien_json),
        Some(nouveau_json),
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": etat })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// Signalements
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/retrouve-amis/signalements
pub async fn lister_signalements(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminSignalementQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "retrouve_amis", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions: Vec<String> = Vec::new();
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    // Filtre etat
    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("s.etat::text = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }

    // Filtre motif
    if let Some(ref motif) = params.motif {
        let m = motif.trim();
        if !m.is_empty() {
            conditions.push(format!("s.motif::text = ${}", bind_index));
            bind_values.push(m.to_string());
            bind_index += 1;
        }
    }

    let _ = bind_index;

    let where_clause = if conditions.is_empty() {
        "TRUE".to_string()
    } else {
        conditions.join(" AND ")
    };
    let colonne = pagination.colonne_tri(ADMIN_SIGNALEMENT_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    // COUNT
    let count_sql = format!(
        "SELECT COUNT(*) FROM retrouve_amis.signalement s
         {} WHERE {}",
        ADMIN_SIGNALEMENT_JOINS, where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT
    let select_sql = format!(
        "SELECT {} FROM retrouve_amis.signalement s
         {} WHERE {} ORDER BY s.{} {} LIMIT {} OFFSET {}",
        ADMIN_SIGNALEMENT_LISTE_COLONNES, ADMIN_SIGNALEMENT_JOINS, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminSignalementListe>(&select_sql);
    for v in &bind_values {
        select_q = select_q.bind(v);
    }
    let rows = select_q.fetch_all(pool.get_ref()).await?;

    let items: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminSignalementListeResponse {
            signalements: items,
            total,
            page,
            par_page,
        }),
        error: None,
    }))
}

/// GET /api/admin/retrouve-amis/signalements/{id}
pub async fn detail_signalement(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "retrouve_amis", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM retrouve_amis.signalement s
         {} WHERE s.id = $1",
        ADMIN_SIGNALEMENT_LISTE_COLONNES, ADMIN_SIGNALEMENT_JOINS
    );
    let row = sqlx::query_as::<_, AdminSignalementListe>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Signalement non trouve".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// PATCH /api/admin/retrouve-amis/signalements/{id}/moderer
pub async fn moderer_signalement(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModererSignalement>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "retrouve_amis", "modifier");
    let id = path.into_inner();

    let decision = body.decision.trim();
    if decision != "approuve" && decision != "rejete" {
        return Err(ApiErreur::Validation(
            "Decision invalide. Valeurs acceptees: approuve, rejete".into(),
        ));
    }

    // Verifier que le signalement existe et recuperer l'avis_id
    let signalement_row: Option<(Uuid, String)> = sqlx::query_as(
        "SELECT avis_id, etat::text FROM retrouve_amis.signalement WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (avis_id, ancien_etat_signalement) = signalement_row
        .ok_or_else(|| ApiErreur::NonTrouve("Signalement non trouve".into()))?;

    if ancien_etat_signalement != "en_attente" {
        return Err(ApiErreur::Validation(format!(
            "Le signalement est deja traite (etat: {})",
            ancien_etat_signalement
        )));
    }

    // Mettre a jour le signalement
    sqlx::query(
        "UPDATE retrouve_amis.signalement
         SET etat = $1::retrouve_amis.etat_signalement, modere_par = $2, modere_at = NOW(), updated_at = NOW()
         WHERE id = $3",
    )
    .bind(decision)
    .bind(admin.id)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);

    // Audit du signalement
    let ancien_json_signalement = serde_json::json!({ "etat": ancien_etat_signalement });
    let nouveau_json_signalement = serde_json::json!({ "etat": decision, "modere_par": admin.id });
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "retrouve_amis",
        "signalement",
        Some(id),
        Some(ancien_json_signalement),
        Some(nouveau_json_signalement),
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    // Si approuve, suspendre l'avis
    let mut avis_suspendu = false;
    if decision == "approuve" {
        let result = sqlx::query(
            "UPDATE retrouve_amis.avis_recherche
             SET etat = 'suspendu'::retrouve_amis.etat_avis, updated_at = NOW()
             WHERE id = $1 AND deleted_at IS NULL AND etat != 'suspendu'::retrouve_amis.etat_avis",
        )
        .bind(avis_id)
        .execute(pool.get_ref())
        .await?;

        avis_suspendu = result.rows_affected() > 0;

        if avis_suspendu {
            let ancien_json_avis = serde_json::json!({ "etat": "actif" });
            let nouveau_json_avis = serde_json::json!({ "etat": "suspendu" });
            audit::log_action(
                pool.get_ref(),
                Some(admin.id),
                "UPDATE",
                "retrouve_amis",
                "avis_recherche",
                Some(avis_id),
                Some(ancien_json_avis),
                Some(nouveau_json_avis),
                ip.as_deref(),
                ua.as_deref(),
            ).await;
        }
    }

    log::info!(
        "Admin {} a modere le signalement {} avec decision '{}' (avis_suspendu: {})",
        admin.id, id, decision, avis_suspendu
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": id,
            "etat": decision,
            "avis_suspendu": avis_suspendu,
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// Statistiques
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/retrouve-amis/statistiques
pub async fn statistiques(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "retrouve_amis", "voir");

    // Total avis
    let total_avis: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.avis_recherche WHERE deleted_at IS NULL",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Avis actifs
    let avis_actifs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.avis_recherche WHERE deleted_at IS NULL AND etat = 'actif'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Avis clotures
    let avis_clotures: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.avis_recherche WHERE deleted_at IS NULL AND etat = 'cloture'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Avis suspendus
    let avis_suspendus: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.avis_recherche WHERE deleted_at IS NULL AND etat = 'suspendu'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Total correspondances
    let total_correspondances: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.correspondance",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Correspondances mutuelles
    let correspondances_mutuelles: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.correspondance WHERE etat = 'mutuelle'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Correspondances en attente
    let correspondances_en_attente: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.correspondance WHERE etat = 'en_attente'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Correspondances declinees
    let correspondances_declinees: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.correspondance WHERE etat = 'declinee'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Correspondances archivees
    let correspondances_archivees: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.correspondance WHERE etat = 'archivee'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Utilisateurs trouvables
    let utilisateurs_trouvables: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM iam.utilisateur WHERE est_trouvable = true AND deleted_at IS NULL",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Signalements en attente
    let signalements_en_attente: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.signalement WHERE etat = 'en_attente'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Signalements total
    let signalements_total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.signalement",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Blacklists total
    let blacklists_total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM retrouve_amis.blacklist",
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminStatistiques {
            total_avis,
            avis_actifs,
            avis_clotures,
            avis_suspendus,
            total_correspondances,
            correspondances_mutuelles,
            correspondances_en_attente,
            correspondances_declinees,
            correspondances_archivees,
            utilisateurs_trouvables,
            signalements_en_attente,
            signalements_total,
            blacklists_total,
        }),
        error: None,
    }))
}

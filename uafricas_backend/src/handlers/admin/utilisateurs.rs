use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::utilisateur::*;
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ────────────────────────────────────────────────────────────────
// GET /api/admin/utilisateurs : Liste paginee avec filtres
// ────────────────────────────────────────────────────────────────

pub async fn lister_utilisateurs(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminUtilisateurQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    // Construction dynamique du WHERE
    let mut conditions: Vec<String> = vec!["u.deleted_at IS NULL".to_string()];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par etat
    if let Some(ref etat) = params.etat {
        let trimmed = etat.trim();
        if !trimmed.is_empty() {
            conditions.push(format!("u.etat::text = ${}", bind_index));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    // Filtre par role
    if let Some(ref role) = params.role {
        let trimmed = role.trim();
        if !trimmed.is_empty() {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM iam.utilisateur_role ur2
                 JOIN iam.role r2 ON r2.id = ur2.role_id
                 WHERE ur2.utilisateur_id = u.id AND r2.slug = ${})",
                bind_index
            ));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    // Filtre par pays
    if let Some(ref pays) = params.pays {
        let trimmed = pays.trim();
        if !trimmed.is_empty() {
            conditions.push(format!(
                "(EXISTS (SELECT 1 FROM shared.pays pp WHERE pp.id = u.pays_residence_id AND LOWER(pp.nom) = LOWER(${idx}))
                 OR EXISTS (SELECT 1 FROM shared.pays pp WHERE pp.id = u.pays_origine_id AND LOWER(pp.nom) = LOWER(${idx})))",
                idx = bind_index
            ));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    // Recherche textuelle
    if let Some(ref recherche) = params.recherche {
        let trimmed = recherche.trim();
        if !trimmed.is_empty() {
            let terme = format!("%{}%", trimmed.to_lowercase());
            conditions.push(format!(
                "(LOWER(u.nom) LIKE ${idx} OR LOWER(u.prenom) LIKE ${idx} OR LOWER(u.email) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Tri
    let tri_colonne = pagination.colonne_tri(UTILISATEUR_TRI_COLONNES, "created_at");
    let tri_direction = pagination.direction_tri();
    let order_clause = format!("ORDER BY u.{} {}", tri_colonne, tri_direction);

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM iam.utilisateur u WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les utilisateurs avec roles agrégés
    let select_query = format!(
        "SELECT {} FROM iam.utilisateur u
         WHERE {} {} LIMIT ${} OFFSET ${}",
        ADMIN_UTILISATEUR_LISTE_COLONNES, where_clause, order_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, AdminUtilisateurListeRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;
    let utilisateurs: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(utilisateurs, total, page, par_page)),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// GET /api/admin/utilisateurs/{id} : Detail complet
// ────────────────────────────────────────────────────────────────

pub async fn obtenir_utilisateur(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "voir");

    let utilisateur_id = chemin.into_inner();

    // Profil
    let query = format!(
        "SELECT {} FROM iam.utilisateur u
         WHERE u.id = $1 AND u.deleted_at IS NULL",
        ADMIN_UTILISATEUR_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminUtilisateurDetailRow>(&query)
        .bind(utilisateur_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Utilisateur {} non trouve", utilisateur_id)))?;

    // Pays
    let pays_origine = if let Some(pid) = row.pays_origine_id {
        sqlx::query_scalar::<_, String>("SELECT nom FROM shared.pays WHERE id = $1")
            .bind(pid)
            .fetch_optional(pool.get_ref())
            .await?
    } else {
        None
    };

    let pays_residence = if let Some(pid) = row.pays_residence_id {
        sqlx::query_scalar::<_, String>("SELECT nom FROM shared.pays WHERE id = $1")
            .bind(pid)
            .fetch_optional(pool.get_ref())
            .await?
    } else {
        None
    };

    // Organisation
    let organisation = if let Some(oid) = row.organisation_id {
        sqlx::query_as::<_, (Uuid, String)>(
            "SELECT id, denomination FROM iam.organisation WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(oid)
        .fetch_optional(pool.get_ref())
        .await?
        .map(|(id, denomination)| OrganisationMinimalInfo { id, denomination })
    } else {
        None
    };

    // Roles
    let roles = sqlx::query_as::<_, RoleInfo>(
        "SELECT r.id, r.nom, r.slug,
                CONCAT(adm.prenom, ' ', adm.nom) AS attribue_par_nom,
                ur.created_at
         FROM iam.role r
         JOIN iam.utilisateur_role ur ON r.id = ur.role_id
         LEFT JOIN iam.utilisateur adm ON adm.id = ur.attribue_par
         WHERE ur.utilisateur_id = $1
         ORDER BY ur.created_at"
    )
    .bind(utilisateur_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Specialites
    let specialites = sqlx::query_as::<_, SpecialiteInfo>(
        "SELECT s.id, s.nom, s.slug
         FROM iam.specialite_bibliotheque s
         JOIN iam.utilisateur_specialite us ON s.id = us.specialite_id
         WHERE us.utilisateur_id = $1
         ORDER BY s.nom"
    )
    .bind(utilisateur_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Permissions specifiques
    let permissions_specifiques = sqlx::query_as::<_, PermissionSpecifiqueInfo>(
        "SELECT ps.id, p.slug AS permission_slug,
                p.type_ressource, p.action,
                ps.ressource_type, ps.ressource_id,
                ps.expire_at, ps.created_at
         FROM iam.permission_specifique ps
         JOIN iam.permission p ON p.id = ps.permission_id
         WHERE ps.utilisateur_id = $1
         ORDER BY ps.created_at"
    )
    .bind(utilisateur_id)
    .fetch_all(pool.get_ref())
    .await?;

    let response = AdminUtilisateurDetailResponse {
        id: row.id,
        nom: row.nom,
        prenom: row.prenom,
        email: row.email,
        slug: row.slug,
        telephone: row.telephone,
        photo_url: row.photo_url,
        genre: row.genre,
        date_naissance: row.date_naissance,
        fonction: row.fonction,
        localite: row.localite,
        ville: row.ville,
        pays_origine,
        pays_residence,
        organisation,
        biographie: row.biographie,
        etat: row.etat,
        email_verifie: row.email_verifie,
        telephone_verifie: row.telephone_verifie,
        double_facteur_active: row.double_facteur_active,
        documents_verifie: row.documents_verifie,
        compte_verifie_admin: row.compte_verifie_admin,
        bibliotheque_humain: row.bibliotheque_humain,
        langue_preferee: row.langue_preferee,
        derniere_connexion: row.derniere_connexion,
        roles,
        specialites,
        permissions_specifiques,
        created_at: row.created_at,
        updated_at: row.updated_at,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// POST /api/admin/utilisateurs : Creation
// ────────────────────────────────────────────────────────────────

pub async fn creer_utilisateur(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerUtilisateurRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    // Validation basique
    if body.nom.trim().is_empty() || body.prenom.trim().is_empty() {
        return Err(ApiErreur::Validation("Nom et prenom requis".into()));
    }
    if body.email.trim().is_empty() {
        return Err(ApiErreur::Validation("Email requis".into()));
    }
    if body.mot_de_passe.len() < 8 {
        return Err(ApiErreur::Validation("Le mot de passe doit contenir au moins 8 caracteres".into()));
    }

    // Verifier unicite email
    let email_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.utilisateur WHERE LOWER(email) = LOWER($1) AND deleted_at IS NULL)"
    )
    .bind(body.email.trim())
    .fetch_one(pool.get_ref())
    .await?;

    if email_existe {
        return Err(ApiErreur::Conflit("Cet email est deja utilise".into()));
    }

    // Hash du mot de passe
    let mot_de_passe_hash = bcrypt::hash(&body.mot_de_passe, 12)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur hash: {}", e)))?;

    // Generer le slug
    let slug = format!(
        "{}-{}",
        body.prenom.trim().to_lowercase().replace(' ', "-"),
        body.nom.trim().to_lowercase().replace(' ', "-")
    );

    // Genre
    let genre = body.genre.as_deref().unwrap_or("non_precise");

    // Inserer l'utilisateur (admin cree directement en actif + email verifie)
    let utilisateur_id: Uuid = sqlx::query_scalar(
        "INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, slug, telephone, genre, etat, email_verifie)
         VALUES ($1, $2, $3, $4, $5, $6, $7::iam.genre, 'actif', true)
         RETURNING id"
    )
    .bind(body.nom.trim())
    .bind(body.prenom.trim())
    .bind(body.email.trim())
    .bind(&mot_de_passe_hash)
    .bind(&slug)
    .bind(body.telephone.as_deref())
    .bind(genre)
    .fetch_one(pool.get_ref())
    .await?;

    // Assigner role optionnel
    if let Some(role_id) = body.role_id {
        sqlx::query(
            "INSERT INTO iam.utilisateur_role (utilisateur_id, role_id, attribue_par)
             VALUES ($1, $2, $3)
             ON CONFLICT DO NOTHING"
        )
        .bind(utilisateur_id)
        .bind(role_id)
        .bind(admin.id)
        .execute(pool.get_ref())
        .await?;
    }

    log::info!("Admin {} a cree l'utilisateur {}", admin.id, utilisateur_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "iam",
        "utilisateur",
        Some(utilisateur_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": utilisateur_id })),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// PUT /api/admin/utilisateurs/{id} : Modification profil
// ────────────────────────────────────────────────────────────────

pub async fn modifier_utilisateur(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierUtilisateurRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let utilisateur_id = chemin.into_inner();

    // Verifier existence
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Utilisateur non trouve".into()));
    }

    // Construction dynamique de l'UPDATE
    let mut sets: Vec<String> = Vec::new();
    let mut bind_index = 1u32;

    // Pour simplifier le binding dynamique, on construit la requête manuellement
    let mut query_parts: Vec<String> = Vec::new();

    if let Some(ref nom) = body.nom {
        query_parts.push(format!("nom = '{}'", nom.trim().replace('\'', "''")));
    }
    if let Some(ref prenom) = body.prenom {
        query_parts.push(format!("prenom = '{}'", prenom.trim().replace('\'', "''")));
    }
    if let Some(ref email) = body.email {
        query_parts.push(format!("email = '{}'", email.trim().replace('\'', "''")));
    }
    if let Some(ref telephone) = body.telephone {
        query_parts.push(format!("telephone = '{}'", telephone.trim().replace('\'', "''")));
    }
    if let Some(ref genre) = body.genre {
        query_parts.push(format!("genre = '{}'::iam.genre", genre.trim().replace('\'', "''")));
    }
    if let Some(ref date_naissance) = body.date_naissance {
        query_parts.push(format!("date_naissance = '{}'", date_naissance));
    }
    if let Some(ref fonction) = body.fonction {
        query_parts.push(format!("fonction = '{}'", fonction.trim().replace('\'', "''")));
    }
    if let Some(ref localite) = body.localite {
        query_parts.push(format!("localite = '{}'", localite.trim().replace('\'', "''")));
    }
    if let Some(ref ville) = body.ville {
        query_parts.push(format!("ville = '{}'", ville.trim().replace('\'', "''")));
    }
    if let Some(ref biographie) = body.biographie {
        query_parts.push(format!("biographie = '{}'", biographie.trim().replace('\'', "''")));
    }
    if let Some(ref langue) = body.langue_preferee {
        query_parts.push(format!("langue_preferee = '{}'", langue.trim().replace('\'', "''")));
    }
    if let Some(biblio) = body.bibliotheque_humain {
        query_parts.push(format!("bibliotheque_humain = {}", biblio));
    }

    if query_parts.is_empty() && body.pays_origine_id.is_none() && body.pays_residence_id.is_none() && body.organisation_id.is_none() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    // Pour les UUID, on utilise des parametres bindes
    let mut uuid_params: Vec<(String, Option<Uuid>)> = Vec::new();
    if body.pays_origine_id.is_some() {
        sets.push(format!("pays_origine_id = ${}", bind_index));
        uuid_params.push(("pays_origine_id".into(), body.pays_origine_id));
        bind_index += 1;
    }
    if body.pays_residence_id.is_some() {
        sets.push(format!("pays_residence_id = ${}", bind_index));
        uuid_params.push(("pays_residence_id".into(), body.pays_residence_id));
        bind_index += 1;
    }
    if body.organisation_id.is_some() {
        sets.push(format!("organisation_id = ${}", bind_index));
        uuid_params.push(("organisation_id".into(), body.organisation_id));
        bind_index += 1;
    }

    // Combiner
    let mut all_sets = query_parts;
    all_sets.extend(sets);
    all_sets.push("updated_at = NOW()".to_string());

    let update_query = format!(
        "UPDATE iam.utilisateur SET {} WHERE id = ${} AND deleted_at IS NULL",
        all_sets.join(", "),
        bind_index
    );

    let mut q = sqlx::query(&update_query);
    for (_, uuid_val) in &uuid_params {
        q = q.bind(*uuid_val);
    }
    q = q.bind(utilisateur_id);

    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Utilisateur non trouve".into()));
    }

    log::info!("Admin {} a modifie l'utilisateur {}", admin.id, utilisateur_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "iam",
        "utilisateur",
        Some(utilisateur_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": utilisateur_id })),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// PATCH /api/admin/utilisateurs/{id}/etat, Changement d'etat
// ────────────────────────────────────────────────────────────────

pub async fn changer_etat_utilisateur(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ChangerEtatRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "bloquer");

    let utilisateur_id = chemin.into_inner();
    let etat = body.etat.trim();

    // Valider l'etat
    let etats_valides = ["actif", "suspendu", "bloque"];
    if !etats_valides.contains(&etat) {
        return Err(ApiErreur::Validation(
            format!("Etat invalide. Valeurs acceptees: {}", etats_valides.join(", "))
        ));
    }

    let result = sqlx::query(
        "UPDATE iam.utilisateur SET etat = $1::iam.etat_utilisateur, updated_at = NOW()
         WHERE id = $2 AND deleted_at IS NULL"
    )
    .bind(etat)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Utilisateur non trouve".into()));
    }

    log::info!("Admin {} a change l'etat de {} a {}", admin.id, utilisateur_id, etat);

    // Cascade FR-022 (feature 001-admin-salles-publiques T045) :
    // si l'utilisateur n'est plus actif, suspendre toutes ses nominations
    // d'administrateur de salle publique.
    if matches!(etat, "suspendu" | "bloque" | "inactif" | "supprime") {
        crate::handlers::admin::salles::cascader_suspendre_admins_utilisateur(
            pool.get_ref(),
            utilisateur_id,
            "compte_desactive",
            admin.id,
            &req,
        )
        .await;
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "iam",
        "utilisateur",
        Some(utilisateur_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": utilisateur_id, "etat": etat })),
        error: None,
    }))
}

/// PATCH /api/admin/utilisateurs/{id}/compte-verifie-admin
/// Validation manuelle d'un compte par un administrateur (badge de crédibilité).
pub async fn valider_compte_admin(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ValiderCompteAdminRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let utilisateur_id = chemin.into_inner();
    let verifie = body.compte_verifie_admin;

    let result = sqlx::query(
        "UPDATE iam.utilisateur
         SET compte_verifie_admin = $1,
             compte_verifie_admin_par = CASE WHEN $1 THEN $2 ELSE NULL END,
             compte_verifie_admin_at  = CASE WHEN $1 THEN NOW() ELSE NULL END,
             updated_at = NOW()
         WHERE id = $3 AND deleted_at IS NULL",
    )
    .bind(verifie)
    .bind(admin.id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Utilisateur non trouve".into()));
    }

    log::info!(
        "Admin {} a {} la validation du compte {}",
        admin.id,
        if verifie { "accordé" } else { "retiré" },
        utilisateur_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "iam",
        "utilisateur",
        Some(utilisateur_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": utilisateur_id,
            "compte_verifie_admin": verifie
        })),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// DELETE /api/admin/utilisateurs/{id}, Soft delete
// ────────────────────────────────────────────────────────────────

pub async fn supprimer_utilisateur(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "supprimer");

    let utilisateur_id = chemin.into_inner();

    let result = sqlx::query(
        "UPDATE iam.utilisateur SET deleted_at = NOW(), etat = 'supprime'::iam.etat_utilisateur, updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Utilisateur non trouve".into()));
    }

    log::info!("Admin {} a supprime l'utilisateur {}", admin.id, utilisateur_id);

    // Cascade FR-022 (feature 001-admin-salles-publiques T045)
    crate::handlers::admin::salles::cascader_suspendre_admins_utilisateur(
        pool.get_ref(),
        utilisateur_id,
        "compte_desactive",
        admin.id,
        &req,
    )
    .await;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "iam",
        "utilisateur",
        Some(utilisateur_id),
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

// ────────────────────────────────────────────────────────────────
// POST /api/admin/utilisateurs/{id}/roles, Assigner un role
// ────────────────────────────────────────────────────────────────

pub async fn assigner_role(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<AssignerRoleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let utilisateur_id = chemin.into_inner();

    // Verifier que l'utilisateur existe
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Utilisateur non trouve".into()));
    }

    // Verifier que le role existe
    let role_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.role WHERE id = $1)"
    )
    .bind(body.role_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !role_existe {
        return Err(ApiErreur::NonTrouve("Role non trouve".into()));
    }

    sqlx::query(
        "INSERT INTO iam.utilisateur_role (utilisateur_id, role_id, attribue_par)
         VALUES ($1, $2, $3)
         ON CONFLICT (utilisateur_id, role_id) DO NOTHING"
    )
    .bind(utilisateur_id)
    .bind(body.role_id)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a assigne le role {} a {}", admin.id, body.role_id, utilisateur_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "iam",
        "utilisateur_role",
        Some(utilisateur_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "utilisateur_id": utilisateur_id,
            "role_id": body.role_id
        })),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// DELETE /api/admin/utilisateurs/{id}/roles/{role_id}, Retirer un role
// ────────────────────────────────────────────────────────────────

pub async fn retirer_role(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let (utilisateur_id, role_id) = chemin.into_inner();

    let result = sqlx::query(
        "DELETE FROM iam.utilisateur_role WHERE utilisateur_id = $1 AND role_id = $2"
    )
    .bind(utilisateur_id)
    .bind(role_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Assignation de role non trouvee".into()));
    }

    log::info!("Admin {} a retire le role {} de {}", admin.id, role_id, utilisateur_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "iam",
        "utilisateur_role",
        Some(utilisateur_id),
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

// ────────────────────────────────────────────────────────────────
// POST /api/admin/utilisateurs/{id}/specialites, Assigner specialite
// ────────────────────────────────────────────────────────────────

pub async fn assigner_specialite(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<AssignerSpecialiteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let utilisateur_id = chemin.into_inner();

    sqlx::query(
        "INSERT INTO iam.utilisateur_specialite (utilisateur_id, specialite_id)
         VALUES ($1, $2)
         ON CONFLICT (utilisateur_id, specialite_id) DO NOTHING"
    )
    .bind(utilisateur_id)
    .bind(body.specialite_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a assigne la specialite {} a {}", admin.id, body.specialite_id, utilisateur_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "iam",
        "utilisateur_specialite",
        Some(utilisateur_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "utilisateur_id": utilisateur_id,
            "specialite_id": body.specialite_id
        })),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// DELETE /api/admin/utilisateurs/{id}/specialites/{spec_id}
// ────────────────────────────────────────────────────────────────

pub async fn retirer_specialite(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let (utilisateur_id, specialite_id) = chemin.into_inner();

    let result = sqlx::query(
        "DELETE FROM iam.utilisateur_specialite WHERE utilisateur_id = $1 AND specialite_id = $2"
    )
    .bind(utilisateur_id)
    .bind(specialite_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Specialite non assignee a cet utilisateur".into()));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "iam",
        "utilisateur_specialite",
        Some(utilisateur_id),
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

// ────────────────────────────────────────────────────────────────
// POST /api/admin/utilisateurs/{id}/permissions, Ajouter permission specifique
// ────────────────────────────────────────────────────────────────

pub async fn ajouter_permission_specifique(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<AjouterPermissionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let utilisateur_id = chemin.into_inner();

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO iam.permission_specifique
            (utilisateur_id, permission_id, ressource_type, ressource_id, attribue_par, expire_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (utilisateur_id, permission_id, ressource_type, ressource_id) DO NOTHING
         RETURNING id"
    )
    .bind(utilisateur_id)
    .bind(body.permission_id)
    .bind(&body.ressource_type)
    .bind(body.ressource_id)
    .bind(admin.id)
    .bind(body.expire_at)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::Conflit("Cette permission specifique existe deja".into()))?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "iam",
        "permission_specifique",
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

// ────────────────────────────────────────────────────────────────
// DELETE /api/admin/utilisateurs/{id}/permissions/{perm_id}
// ────────────────────────────────────────────────────────────────

pub async fn retirer_permission_specifique(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let (utilisateur_id, perm_id) = chemin.into_inner();

    let result = sqlx::query(
        "DELETE FROM iam.permission_specifique WHERE id = $1 AND utilisateur_id = $2"
    )
    .bind(perm_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Permission specifique non trouvee".into()));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "iam",
        "permission_specifique",
        Some(perm_id),
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

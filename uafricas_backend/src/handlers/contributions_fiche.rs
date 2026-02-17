use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::contribution_fiche::{
    ContributionFicheRow, ContributionListeResponse, ContributionQueryParams,
    ContributeurRow, CreerContributionBody, ModerationBody,
    CONTRIBUTION_COLONNES, construire_contribution_response, construire_contributeur_response,
    section_est_valide,
};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ══════════════════════════════════════════════════════════════════════════
// Fonctions utilitaires
// ══════════════════════════════════════════════════════════════════════════

/// Extraire l'utilisateur connecte depuis le header Authorization
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Verifier si l'utilisateur a le role admin
async fn verifier_admin(pool: &PgPool, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.utilisateur_role ur
            JOIN iam.role r ON ur.role_id = r.id
            WHERE ur.utilisateur_id = $1 AND r.slug = 'admin'
        )",
    )
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(is_admin)
}

/// Recuperer la valeur actuelle d'un champ de la fiche pays
async fn obtenir_valeur_actuelle(
    pool: &PgPool,
    fiche_id: Uuid,
    section: &str,
) -> Result<Option<String>, ApiErreur> {
    let valeur: Option<String> = match section {
        "population" => {
            sqlx::query_scalar("SELECT population::text FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "superficie_km2" => {
            sqlx::query_scalar("SELECT superficie_km2::text FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "biographie" => {
            sqlx::query_scalar("SELECT biographie FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "contexte" => {
            sqlx::query_scalar("SELECT contexte FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "contexte_historique" => {
            sqlx::query_scalar("SELECT contexte_historique FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "slogan" => {
            sqlx::query_scalar("SELECT slogan FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "hymne_national" => {
            sqlx::query_scalar("SELECT hymne_national FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "langue_officielle" => {
            sqlx::query_scalar("SELECT langue_officielle FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "langues_populaires" => {
            sqlx::query_scalar("SELECT langues_populaires FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "monnaie" => {
            sqlx::query_scalar("SELECT monnaie FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "fuseau_horaire" => {
            sqlx::query_scalar("SELECT fuseau_horaire FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        // Pour les tables liees (groupe_ethnique, site_touristique, etc.), pas de valeur actuelle
        _ => None,
    };
    Ok(valeur)
}

/// Appliquer une contribution validee sur la fiche pays
async fn appliquer_contribution(
    pool: &PgPool,
    fiche_id: Uuid,
    section: &str,
    type_contribution: &str,
    nouvelle_valeur: &str,
) -> Result<(), ApiErreur> {
    match section {
        "population" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET population = $1::bigint, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "superficie_km2" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET superficie_km2 = $1::decimal, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "biographie" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET biographie = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "contexte" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET contexte = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "contexte_historique" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET contexte_historique = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "slogan" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET slogan = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "hymne_national" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET hymne_national = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "langue_officielle" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET langue_officielle = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "langues_populaires" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET langues_populaires = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "monnaie" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET monnaie = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "fuseau_horaire" => {
            sqlx::query(
                "UPDATE country_profile.fiche_pays SET fuseau_horaire = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(nouvelle_valeur)
            .bind(fiche_id)
            .execute(pool)
            .await?;
        }
        "groupe_ethnique" => {
            if type_contribution == "ajout" {
                sqlx::query(
                    "INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES ($1, $2)",
                )
                .bind(fiche_id)
                .bind(nouvelle_valeur)
                .execute(pool)
                .await?;
            }
        }
        "site_touristique" => {
            if type_contribution == "ajout" {
                sqlx::query(
                    "INSERT INTO country_profile.site_touristique (fiche_pays_id, nom) VALUES ($1, $2)",
                )
                .bind(fiche_id)
                .bind(nouvelle_valeur)
                .execute(pool)
                .await?;
            }
        }
        "secteur_developpement" => {
            if type_contribution == "ajout" {
                sqlx::query(
                    "INSERT INTO country_profile.secteur_developpement (fiche_pays_id, nom) VALUES ($1, $2)",
                )
                .bind(fiche_id)
                .bind(nouvelle_valeur)
                .execute(pool)
                .await?;
            }
        }
        _ => {}
    }
    Ok(())
}

// ══════════════════════════════════════════════════════════════════════════
// Endpoints
// ══════════════════════════════════════════════════════════════════════════

/// POST /api/fiches-pays/{id}/contributions — Soumettre une contribution
pub async fn soumettre_contribution(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    body: web::Json<CreerContributionBody>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let fiche_id_str = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_id_str)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    // Valider la section
    let section = body.section.trim().to_lowercase();
    if !section_est_valide(&section) {
        return Err(ApiErreur::Validation(format!(
            "Section invalide: '{}'. Sections valides: population, biographie, contexte, slogan, etc.",
            section
        )));
    }

    // Valider le type de contribution
    let type_contribution = body
        .type_contribution
        .as_deref()
        .unwrap_or("modification")
        .to_lowercase();
    if !["modification", "ajout", "suppression"].contains(&type_contribution.as_str()) {
        return Err(ApiErreur::Validation(
            "Type de contribution invalide. Valeurs acceptees: modification, ajout, suppression"
                .to_string(),
        ));
    }

    // Valider la nouvelle valeur
    let nouvelle_valeur = body.nouvelle_valeur.trim().to_string();
    if nouvelle_valeur.is_empty() {
        return Err(ApiErreur::Validation(
            "La nouvelle valeur ne peut pas etre vide".to_string(),
        ));
    }

    // Verifier que la fiche existe
    let fiche_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM country_profile.fiche_pays WHERE id = $1)",
    )
    .bind(fiche_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !fiche_existe {
        return Err(ApiErreur::NonTrouve("Fiche pays non trouvee".to_string()));
    }

    // Recuperer la valeur actuelle pour les modifications
    let ancienne_valeur = if type_contribution == "modification" {
        obtenir_valeur_actuelle(pool.get_ref(), fiche_id, &section).await?
    } else {
        None
    };

    let justification = body.justification.as_deref().map(|s| s.trim().to_string());

    // Inserer la contribution
    let row = sqlx::query_as::<_, ContributionFicheRow>(
        &format!(
            "WITH inserted AS (
                INSERT INTO country_profile.contribution_fiche
                    (fiche_pays_id, cree_par, section, type_contribution, ancienne_valeur, nouvelle_valeur, justification)
                VALUES ($1, $2, $3, $4::country_profile.type_contribution, $5, $6, $7)
                RETURNING *
            )
            SELECT {} FROM inserted cf
            JOIN iam.utilisateur u ON u.id = cf.cree_par",
            CONTRIBUTION_COLONNES
        ),
    )
    .bind(fiche_id)
    .bind(utilisateur_id)
    .bind(&section)
    .bind(&type_contribution)
    .bind(&ancienne_valeur)
    .bind(&nouvelle_valeur)
    .bind(&justification)
    .fetch_one(pool.get_ref())
    .await?;

    let response = construire_contribution_response(&row);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

/// GET /api/fiches-pays/{id}/contributions — Lister les contributions d'une fiche
pub async fn lister_contributions(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    params: web::Query<ContributionQueryParams>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let fiche_id_str = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_id_str)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Verifier si l'utilisateur est admin (acces a toutes les contributions)
    let est_admin = if let Some(uid) = extraire_utilisateur_id(&req) {
        verifier_admin(pool.get_ref(), uid).await.unwrap_or(false)
    } else {
        false
    };

    // Construction dynamique des conditions
    let mut conditions = vec![
        "cf.fiche_pays_id = $1".to_string(),
        "cf.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 2u32;
    let mut bind_values: Vec<String> = Vec::new();

    if est_admin {
        // Admin peut filtrer par etat
        if let Some(ref etat) = params.etat {
            let etat = etat.trim().to_lowercase();
            if ["en_attente", "approuvee", "rejetee"].contains(&etat.as_str()) {
                conditions.push(format!("cf.etat::text = ${}", bind_index));
                bind_values.push(etat);
                bind_index += 1;
            }
        }
    } else {
        // Public: seulement les approuvees
        conditions.push("cf.etat = 'approuvee'".to_string());
    }

    if let Some(ref section) = params.section {
        let section = section.trim().to_lowercase();
        if section_est_valide(&section) {
            conditions.push(format!("cf.section = ${}", bind_index));
            bind_values.push(section);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM country_profile.contribution_fiche cf WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query).bind(fiche_id);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les contributions
    let select_query = format!(
        "SELECT {} FROM country_profile.contribution_fiche cf
         JOIN iam.utilisateur u ON u.id = cf.cree_par
         WHERE {} ORDER BY cf.created_at DESC LIMIT ${} OFFSET ${}",
        CONTRIBUTION_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, ContributionFicheRow>(&select_query).bind(fiche_id);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    let contributions: Vec<_> = rows.iter().map(construire_contribution_response).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ContributionListeResponse {
            contributions,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/fiches-pays/{id}/contributeurs — Lister les contributeurs valides
pub async fn lister_contributeurs(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let fiche_id_str = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_id_str)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    let rows = sqlx::query_as::<_, ContributeurRow>(
        "SELECT
            u.id AS utilisateur_id,
            u.nom,
            u.prenom,
            u.photo_url,
            COUNT(*) AS nombre_contributions
         FROM country_profile.contribution_fiche cf
         JOIN iam.utilisateur u ON u.id = cf.cree_par
         WHERE cf.fiche_pays_id = $1
           AND cf.etat = 'approuvee'
           AND cf.deleted_at IS NULL
         GROUP BY u.id, u.nom, u.prenom, u.photo_url
         ORDER BY nombre_contributions DESC",
    )
    .bind(fiche_id)
    .fetch_all(pool.get_ref())
    .await?;

    let contributeurs: Vec<_> = rows.iter().map(construire_contributeur_response).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(contributeurs),
        error: None,
    }))
}

/// PUT /api/fiches-pays/contributions/{id}/valider — Valider une contribution (admin)
pub async fn valider_contribution(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    body: web::Json<ModerationBody>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let admin_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    if !verifier_admin(pool.get_ref(), admin_id).await? {
        return Err(ApiErreur::NonAutorise(
            "Droits administrateur requis".to_string(),
        ));
    }

    let contribution_id_str = chemin.into_inner();
    let contribution_id = Uuid::parse_str(&contribution_id_str)
        .map_err(|_| ApiErreur::Validation("ID de contribution invalide".to_string()))?;

    // Recuperer la contribution en attente
    let contrib = sqlx::query_as::<_, ContributionFicheRow>(
        &format!(
            "SELECT {} FROM country_profile.contribution_fiche cf
             JOIN iam.utilisateur u ON u.id = cf.cree_par
             WHERE cf.id = $1 AND cf.deleted_at IS NULL",
            CONTRIBUTION_COLONNES
        ),
    )
    .bind(contribution_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Contribution non trouvee".to_string()))?;

    if contrib.etat != "en_attente" {
        return Err(ApiErreur::Validation(format!(
            "Seules les contributions en attente peuvent etre validees (etat actuel: {})",
            contrib.etat
        )));
    }

    // Appliquer la contribution sur la fiche
    appliquer_contribution(
        pool.get_ref(),
        contrib.fiche_pays_id,
        &contrib.section,
        &contrib.type_contribution,
        &contrib.nouvelle_valeur,
    )
    .await?;

    // Mettre a jour l'etat de la contribution
    let note = body.note_moderation.as_deref().map(|s| s.trim().to_string());

    let row = sqlx::query_as::<_, ContributionFicheRow>(
        &format!(
            "WITH updated AS (
                UPDATE country_profile.contribution_fiche
                SET etat = 'approuvee'::country_profile.etat_contribution,
                    traite_par = $1,
                    note_moderation = $2,
                    traite_at = NOW(),
                    updated_at = NOW()
                WHERE id = $3
                RETURNING *
            )
            SELECT {} FROM updated cf
            JOIN iam.utilisateur u ON u.id = cf.cree_par",
            CONTRIBUTION_COLONNES
        ),
    )
    .bind(admin_id)
    .bind(&note)
    .bind(contribution_id)
    .fetch_one(pool.get_ref())
    .await?;

    let response = construire_contribution_response(&row);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

/// PUT /api/fiches-pays/contributions/{id}/rejeter — Rejeter une contribution (admin)
pub async fn rejeter_contribution(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    body: web::Json<ModerationBody>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let admin_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    if !verifier_admin(pool.get_ref(), admin_id).await? {
        return Err(ApiErreur::NonAutorise(
            "Droits administrateur requis".to_string(),
        ));
    }

    let contribution_id_str = chemin.into_inner();
    let contribution_id = Uuid::parse_str(&contribution_id_str)
        .map_err(|_| ApiErreur::Validation("ID de contribution invalide".to_string()))?;

    // Verifier que la contribution existe et est en attente
    let etat_actuel: Option<String> = sqlx::query_scalar(
        "SELECT etat::text FROM country_profile.contribution_fiche WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(contribution_id)
    .fetch_optional(pool.get_ref())
    .await?
    .flatten();

    match etat_actuel.as_deref() {
        None => {
            return Err(ApiErreur::NonTrouve(
                "Contribution non trouvee".to_string(),
            ))
        }
        Some(etat) if etat != "en_attente" => {
            return Err(ApiErreur::Validation(format!(
                "Seules les contributions en attente peuvent etre rejetees (etat actuel: {})",
                etat
            )))
        }
        _ => {}
    }

    let note = body.note_moderation.as_deref().map(|s| s.trim().to_string());

    let row = sqlx::query_as::<_, ContributionFicheRow>(
        &format!(
            "WITH updated AS (
                UPDATE country_profile.contribution_fiche
                SET etat = 'rejetee'::country_profile.etat_contribution,
                    traite_par = $1,
                    note_moderation = $2,
                    traite_at = NOW(),
                    updated_at = NOW()
                WHERE id = $3
                RETURNING *
            )
            SELECT {} FROM updated cf
            JOIN iam.utilisateur u ON u.id = cf.cree_par",
            CONTRIBUTION_COLONNES
        ),
    )
    .bind(admin_id)
    .bind(&note)
    .bind(contribution_id)
    .fetch_one(pool.get_ref())
    .await?;

    let response = construire_contribution_response(&row);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::mooc::{
    FormateurInfo, FormateurResponse, MoocDetailResponse, MoocListeResponse,
    MoocQueryParams, MoocResponse, MoocRow, MOOC_COLONNES, calculer_statut_mooc,
};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Extraire l'utilisateur connecte depuis le header Authorization
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Charger le formateur depuis iam.utilisateur
async fn charger_formateur(pool: &PgPool, utilisateur_id: Uuid) -> Result<FormateurInfo, ApiErreur> {
    sqlx::query_as::<_, FormateurInfo>(
        "SELECT id, nom, prenom, email, photo_url FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Formateur non trouve".to_string()))
}

/// Compter les inscrits a un MOOC
async fn compter_inscrits(pool: &PgPool, mooc_id: Uuid) -> Result<i64, ApiErreur> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc_inscription
         WHERE mooc_id = $1 AND statut != 'abandonne'",
    )
    .bind(mooc_id)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Verifier si un utilisateur est inscrit a un MOOC
async fn est_inscrit(pool: &PgPool, mooc_id: Uuid, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.mooc_inscription
            WHERE mooc_id = $1 AND utilisateur_id = $2 AND statut != 'abandonne'
        )",
    )
    .bind(mooc_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

/// Recuperer le nom du pays
async fn charger_nom_pays(pool: &PgPool, pays_id: Option<Uuid>) -> Result<Option<String>, ApiErreur> {
    if let Some(pid) = pays_id {
        Ok(sqlx::query_scalar("SELECT nom FROM shared.pays WHERE id = $1")
            .bind(pid)
            .fetch_optional(pool)
            .await?)
    } else {
        Ok(None)
    }
}

/// Construire un MoocResponse a partir d'un MoocRow
async fn construire_response(
    pool: &PgPool,
    row: &MoocRow,
) -> Result<MoocResponse, ApiErreur> {
    let formateur = charger_formateur(pool, row.cree_par).await?;
    let nombre_inscrits = compter_inscrits(pool, row.id).await?;

    Ok(MoocResponse {
        id: row.id,
        titre: row.titre.clone(),
        description: row.description.clone(),
        type_formation: row.type_formation.clone(),
        langue: row.langue.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        statut: calculer_statut_mooc(
            &row.etat,
            &row.date_heure_debut,
            row.date_heure_fin.as_ref(),
            nombre_inscrits,
            row.nombre_places,
        ),
        nombre_places: row.nombre_places,
        nombre_inscrits,
        formateur: FormateurResponse {
            uid: formateur.id,
            nom: formateur.nom,
            prenom: formateur.prenom,
            email: formateur.email,
            photo_url: formateur.photo_url,
        },
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

// ──────────────────────────────────────────────────────────────
// GET /api/moocs — Lister les formations avec filtres et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_moocs(
    pool: web::Data<PgPool>,
    params: web::Query<MoocQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Construire les conditions WHERE dynamiquement
    let mut conditions: Vec<String> = vec![
        "m.etat IN ('publie','en_cours','termine')".to_string(),
        "m.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref type_f) = params.type_formation {
        if !type_f.is_empty() {
            conditions.push(format!("LOWER(m.type) = LOWER(${})", bind_index));
            bind_values.push(type_f.clone());
            bind_index += 1;
        }
    }

    if let Some(ref recherche) = params.recherche {
        if !recherche.is_empty() {
            let terme = format!("%{}%", recherche.to_lowercase());
            conditions.push(format!(
                "(LOWER(m.titre) LIKE ${idx} OR LOWER(m.description) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM media_content.mooc m WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les MOOCs
    let select_query = format!(
        "SELECT {} FROM media_content.mooc m WHERE {} ORDER BY m.date_heure_debut DESC LIMIT ${} OFFSET ${}",
        MOOC_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, MoocRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    // Construire les reponses
    let mut formations = Vec::with_capacity(rows.len());
    for row in &rows {
        formations.push(construire_response(pool.get_ref(), row).await?);
    }

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MoocListeResponse {
            formations,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/moocs/{id} — Obtenir le detail d'un MOOC
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_mooc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let current_user = extraire_utilisateur_id(&req);
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM media_content.mooc m
         WHERE m.id = $1 AND m.deleted_at IS NULL",
        MOOC_COLONNES
    );

    let row = sqlx::query_as::<_, MoocRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Formation avec id {} non trouvee", id)))?;

    let formateur = charger_formateur(pool.get_ref(), row.cree_par).await?;
    let nombre_inscrits = compter_inscrits(pool.get_ref(), row.id).await?;
    let pays_nom = charger_nom_pays(pool.get_ref(), row.pays_id).await?;

    let inscrit = if let Some(uid) = current_user {
        est_inscrit(pool.get_ref(), id, uid).await?
    } else {
        false
    };

    let reponse = MoocDetailResponse {
        id: row.id,
        titre: row.titre.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        type_formation: row.type_formation.clone(),
        pays: pays_nom,
        ville: row.ville.clone(),
        langue: row.langue.clone(),
        format: row.format.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        lien_en_ligne: row.lien_en_ligne.clone(),
        statut: calculer_statut_mooc(
            &row.etat,
            &row.date_heure_debut,
            row.date_heure_fin.as_ref(),
            nombre_inscrits,
            row.nombre_places,
        ),
        nombre_places: row.nombre_places,
        nombre_inscrits,
        prerequis: row.prerequis.clone(),
        est_inscrit: inscrit,
        formateur: FormateurResponse {
            uid: formateur.id,
            nom: formateur.nom,
            prenom: formateur.prenom,
            email: formateur.email,
            photo_url: formateur.photo_url,
        },
        created_at: row.created_at,
        updated_at: row.updated_at,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/moocs/{id}/inscription — S'inscrire a un MOOC
// ──────────────────────────────────────────────────────────────
pub async fn inscrire_mooc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let mooc_id = chemin.into_inner();

    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    // Verifier que le MOOC existe et est publie
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.mooc
            WHERE id = $1 AND etat = 'publie' AND deleted_at IS NULL
        )",
    )
    .bind(mooc_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Formation non trouvee".to_string()));
    }

    // Verifier la capacite
    let row = sqlx::query_as::<_, (Option<i32>,)>(
        "SELECT nombre_places FROM media_content.mooc WHERE id = $1",
    )
    .bind(mooc_id)
    .fetch_one(pool.get_ref())
    .await?;

    if let Some(max_places) = row.0 {
        let inscrits = compter_inscrits(pool.get_ref(), mooc_id).await?;
        if inscrits >= max_places as i64 {
            return Err(ApiErreur::Validation("La formation est complete".to_string()));
        }
    }

    // Inserer l'inscription (ON CONFLICT pour eviter les doublons)
    sqlx::query(
        "INSERT INTO media_content.mooc_inscription (mooc_id, utilisateur_id, statut)
         VALUES ($1, $2, 'inscrit')
         ON CONFLICT (mooc_id, utilisateur_id) DO UPDATE SET statut = 'inscrit', updated_at = NOW()",
    )
    .bind(mooc_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Inscription MOOC {} par utilisateur {}", mooc_id, utilisateur_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::television::*;

#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ── Extraction JWT ────────────────────────────────────────────────────

fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = crate::jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

// ═══════════════════════════════════════════════════════════════════════════
// CHAÎNES TV
// ═══════════════════════════════════════════════════════════════════════════

// ── GET /api/television/chaines ───────────────────────────────────────

pub async fn lister_chaines(
    pool: web::Data<PgPool>,
    params: web::Query<ChaineTvQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).min(100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec![
        "ct.etat = 'publie'".to_string(),
        "ct.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par catégorie
    if let Some(ref categorie) = params.categorie {
        if categorie != "Toutes les catégories" {
            let cat_db = mapper_categorie_chaine_db(categorie);
            conditions.push(format!("ct.categorie::text = ${}", bind_index));
            bind_values.push(cat_db);
            bind_index += 1;
        }
    }

    // Filtre par pays
    if let Some(ref pays) = params.pays {
        if pays != "Tous les pays" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p2 WHERE p2.id = ct.pays_id AND LOWER(p2.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(pays.clone());
            bind_index += 1;
        }
    }

    // Recherche textuelle
    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            conditions.push(format!(
                "(LOWER(ct.nom) LIKE LOWER(${bi}) OR LOWER(ct.description) LIKE LOWER(${bi}))",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", recherche.trim()));
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM media_content.chaine_tv ct WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage chaînes TV: {}", e)))?;

    // Récupérer les chaînes avec jointure pays
    let query = format!(
        "SELECT {}, p.nom AS pays_nom
         FROM media_content.chaine_tv ct
         LEFT JOIN shared.pays p ON p.id = ct.pays_id
         WHERE {}
         ORDER BY ct.nom ASC
         LIMIT ${} OFFSET ${}",
        CHAINE_TV_COLONNES,
        where_clause,
        bind_index,
        bind_index + 1,
    );

    let mut q = sqlx::query_as::<_, ChaineTvRow>(&query);
    for val in &bind_values {
        q = q.bind(val);
    }
    q = q.bind(par_page).bind(offset);

    let chaines = q.fetch_all(pool.get_ref()).await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing chaînes TV: {}", e)))?;

    let total_pages = (total as f64 / par_page as f64).ceil() as i64;

    let reponse = ChaineTvListeResponse {
        chaines: chaines.iter().map(|c| c.to_response()).collect(),
        total,
        page,
        par_page,
        total_pages,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ── GET /api/television/chaines/{id} ──────────────────────────────────

pub async fn obtenir_chaine(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let id_str = chemin.into_inner();
    let chaine_id = Uuid::parse_str(&id_str)
        .map_err(|_| ApiErreur::Validation("ID de chaîne invalide".into()))?;

    let query = format!(
        "SELECT {}, p.nom AS pays_nom
         FROM media_content.chaine_tv ct
         LEFT JOIN shared.pays p ON p.id = ct.pays_id
         WHERE ct.id = $1 AND ct.etat = 'publie' AND ct.deleted_at IS NULL",
        CHAINE_TV_COLONNES
    );

    let chaine = sqlx::query_as::<_, ChaineTvRow>(&query)
        .bind(chaine_id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture chaîne TV: {}", e)))?
        .ok_or_else(|| ApiErreur::NonTrouve("Chaîne TV non trouvée".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(chaine.to_response()),
        error: None,
    }))
}

// ── POST /api/television/chaines ──────────────────────────────────────

pub async fn creer_chaine(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreerChaineTvForm>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    // Validation
    if body.nom.trim().is_empty() {
        return Err(ApiErreur::Validation("Le nom de la chaîne est requis".into()));
    }
    if body.stream_url.trim().is_empty() {
        return Err(ApiErreur::Validation("L'URL de streaming est requise".into()));
    }

    let slug = generer_slug(&body.nom);
    let categorie = body.categorie.as_deref()
        .map(mapper_categorie_chaine_db)
        .unwrap_or_else(|| "generaliste".to_string());
    let langue = body.langue.as_deref().unwrap_or("Français");

    // Résoudre pays_id si un nom de pays est fourni
    let pays_id: Option<Uuid> = if let Some(ref pays_nom) = body.pays {
        sqlx::query_scalar(
            "SELECT id FROM shared.pays WHERE LOWER(nom) = LOWER($1)"
        )
        .bind(pays_nom)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur résolution pays: {}", e)))?
    } else {
        None
    };

    let chaine_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO media_content.chaine_tv
            (id, nom, slug, description, stream_url, categorie, pays_id, langue, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6::media_content.categorie_chaine_tv, $7, $8, 'publie', $9)"
    )
    .bind(chaine_id)
    .bind(body.nom.trim())
    .bind(&slug)
    .bind(body.description.as_deref().map(str::trim))
    .bind(body.stream_url.trim())
    .bind(&categorie)
    .bind(pays_id)
    .bind(langue)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur création chaîne TV: {}", e)))?;

    // Récupérer la chaîne créée avec jointure pays
    let query = format!(
        "SELECT {}, p.nom AS pays_nom
         FROM media_content.chaine_tv ct
         LEFT JOIN shared.pays p ON p.id = ct.pays_id
         WHERE ct.id = $1",
        CHAINE_TV_COLONNES
    );

    let chaine = sqlx::query_as::<_, ChaineTvRow>(&query)
        .bind(chaine_id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture chaîne créée: {}", e)))?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(chaine.to_response()),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAMMES VEDETTES (programme_radio_tele WHERE type='tele')
// ═══════════════════════════════════════════════════════════════════════════

// ── GET /api/television/programmes-vedettes ───────────────────────────

pub async fn lister_programmes_vedettes(
    pool: web::Data<PgPool>,
    params: web::Query<ProgrammeTeleQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).min(100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec![
        "prt.type = 'tele'".to_string(),
        "prt.etat = 'publie'".to_string(),
        "prt.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par pays
    if let Some(ref pays) = params.pays {
        if pays != "Tous les pays" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p2 WHERE p2.id = prt.pays_id AND LOWER(p2.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(pays.clone());
            bind_index += 1;
        }
    }

    // Recherche textuelle
    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            conditions.push(format!(
                "(LOWER(prt.nom_emission) LIKE LOWER(${bi}) OR LOWER(prt.description) LIKE LOWER(${bi}))",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", recherche.trim()));
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM media_content.programme_radio_tele prt WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage programmes TV: {}", e)))?;

    // Récupérer les programmes avec jointure pays
    let query = format!(
        "SELECT {}, p.nom AS pays_nom
         FROM media_content.programme_radio_tele prt
         LEFT JOIN shared.pays p ON p.id = prt.pays_id
         WHERE {}
         ORDER BY prt.created_at DESC
         LIMIT ${} OFFSET ${}",
        PROGRAMME_TELE_COLONNES,
        where_clause,
        bind_index,
        bind_index + 1,
    );

    let mut q = sqlx::query_as::<_, ProgrammeTeleRow>(&query);
    for val in &bind_values {
        q = q.bind(val);
    }
    q = q.bind(par_page).bind(offset);

    let programmes = q.fetch_all(pool.get_ref()).await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing programmes TV: {}", e)))?;

    let total_pages = (total as f64 / par_page as f64).ceil() as i64;

    let reponse = ProgrammeTeleListeResponse {
        programmes: programmes.iter().map(|p| p.to_response()).collect(),
        total,
        page,
        par_page,
        total_pages,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ── GET /api/television/programmes-vedettes/{id} ──────────────────────

pub async fn obtenir_programme_vedette(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let id_str = chemin.into_inner();
    let programme_id = Uuid::parse_str(&id_str)
        .map_err(|_| ApiErreur::Validation("ID de programme invalide".into()))?;

    let query = format!(
        "SELECT {}, p.nom AS pays_nom
         FROM media_content.programme_radio_tele prt
         LEFT JOIN shared.pays p ON p.id = prt.pays_id
         WHERE prt.id = $1 AND prt.type = 'tele' AND prt.etat = 'publie' AND prt.deleted_at IS NULL",
        PROGRAMME_TELE_COLONNES
    );

    let programme = sqlx::query_as::<_, ProgrammeTeleRow>(&query)
        .bind(programme_id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture programme TV: {}", e)))?
        .ok_or_else(|| ApiErreur::NonTrouve("Programme TV non trouvé".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(programme.to_response()),
        error: None,
    }))
}

// ── POST /api/television/programmes-vedettes ──────────────────────────

pub async fn creer_programme_vedette(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreerProgrammeTeleForm>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    // Validation
    if body.nom_emission.trim().is_empty() {
        return Err(ApiErreur::Validation("Le nom de l'émission est requis".into()));
    }
    if body.description.trim().is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }
    if body.video_url.trim().is_empty() {
        return Err(ApiErreur::Validation("L'URL de la vidéo est requise".into()));
    }

    let slug = generer_slug(&body.nom_emission);
    let langue = body.langue.as_deref().unwrap_or("Français");
    let est_international = body.est_international.unwrap_or(false);

    // Résoudre pays_id si un nom de pays est fourni
    let pays_id: Option<Uuid> = if let Some(ref pays_nom) = body.pays {
        sqlx::query_scalar(
            "SELECT id FROM shared.pays WHERE LOWER(nom) = LOWER($1)"
        )
        .bind(pays_nom)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur résolution pays: {}", e)))?
    } else {
        None
    };

    let programme_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO media_content.programme_radio_tele
            (id, nom_emission, slug, type, description, video_url, image_couverture_url,
             info_animateur, info_producteur, pays_id, est_international, langue, etat, cree_par)
         VALUES ($1, $2, $3, 'tele'::media_content.type_programme_media, $4, $5, $6, $7, $8, $9, $10, $11, 'publie', $12)"
    )
    .bind(programme_id)
    .bind(body.nom_emission.trim())
    .bind(&slug)
    .bind(body.description.trim())
    .bind(body.video_url.trim())
    .bind(body.image_couverture_url.as_deref().map(str::trim))
    .bind(body.info_animateur.as_deref().map(str::trim))
    .bind(body.info_producteur.as_deref().map(str::trim))
    .bind(pays_id)
    .bind(est_international)
    .bind(langue)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur création programme TV: {}", e)))?;

    // Récupérer le programme créé avec jointure pays
    let query = format!(
        "SELECT {}, p.nom AS pays_nom
         FROM media_content.programme_radio_tele prt
         LEFT JOIN shared.pays p ON p.id = prt.pays_id
         WHERE prt.id = $1",
        PROGRAMME_TELE_COLONNES
    );

    let programme = sqlx::query_as::<_, ProgrammeTeleRow>(&query)
        .bind(programme_id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture programme créé: {}", e)))?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(programme.to_response()),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILITAIRES (pays, catégories, stats)
// ═══════════════════════════════════════════════════════════════════════════

// ── GET /api/television/pays ──────────────────────────────────────────

pub async fn lister_pays_television(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let pays: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT p.nom
         FROM (
             SELECT pays_id FROM media_content.chaine_tv WHERE etat = 'publie' AND deleted_at IS NULL AND pays_id IS NOT NULL
             UNION
             SELECT pays_id FROM media_content.programme_radio_tele WHERE type = 'tele' AND etat = 'publie' AND deleted_at IS NULL AND pays_id IS NOT NULL
         ) sub
         JOIN shared.pays p ON p.id = sub.pays_id
         ORDER BY p.nom ASC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing pays TV: {}", e)))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(pays),
        error: None,
    }))
}

// ── GET /api/television/categories ────────────────────────────────────

pub async fn lister_categories_television(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let categories_db: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT categorie::text
         FROM media_content.chaine_tv
         WHERE etat = 'publie' AND deleted_at IS NULL
         ORDER BY categorie ASC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing catégories TV: {}", e)))?;

    let categories: Vec<String> = categories_db
        .iter()
        .map(|c| mapper_categorie_chaine_frontend(c))
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(categories),
        error: None,
    }))
}

// ── GET /api/television/stats ─────────────────────────────────────────

pub async fn obtenir_stats_television(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let nombre_chaines: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.chaine_tv WHERE etat = 'publie' AND deleted_at IS NULL"
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage chaînes: {}", e)))?;

    let nombre_pays: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT pays_id) FROM media_content.chaine_tv WHERE etat = 'publie' AND deleted_at IS NULL AND pays_id IS NOT NULL"
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage pays: {}", e)))?;

    let nombre_programmes: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.programme_radio_tele WHERE type = 'tele' AND etat = 'publie' AND deleted_at IS NULL"
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage programmes: {}", e)))?;

    let nombre_chaines_en_direct: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.chaine_tv WHERE etat = 'publie' AND deleted_at IS NULL AND est_en_direct = true"
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage en direct: {}", e)))?;

    let stats = TelevisionStats {
        nombre_chaines,
        nombre_pays,
        nombre_programmes,
        nombre_chaines_en_direct,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(stats),
        error: None,
    }))
}

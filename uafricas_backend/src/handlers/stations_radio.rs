use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::station_radio::*;

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

// ── GET /api/stations-radio ───────────────────────────────────────────

pub async fn lister_stations(
    pool: web::Data<PgPool>,
    params: web::Query<StationRadioQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).min(100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec![
        "sr.etat = 'publie'".to_string(),
        "sr.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par type de station
    if let Some(ref type_station) = params.type_station {
        if type_station != "Tous les types" {
            let type_db = mapper_type_station_db(type_station);
            conditions.push(format!("sr.type_station::text = ${}", bind_index));
            bind_values.push(type_db);
            bind_index += 1;
        }
    }

    // Filtre par pays
    if let Some(ref pays) = params.pays {
        if pays != "Tous les pays" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p2 WHERE p2.id = sr.pays_id AND LOWER(p2.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(pays.clone());
            bind_index += 1;
        }
    }

    // Filtre par genre
    if let Some(ref genre) = params.genre {
        if genre != "Tous les genres" {
            conditions.push(format!("${} = ANY(sr.genres_liste)", bind_index));
            bind_values.push(genre.clone());
            bind_index += 1;
        }
    }

    // Recherche textuelle
    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            conditions.push(format!(
                "(LOWER(sr.nom) LIKE LOWER(${bi}) OR LOWER(sr.description) LIKE LOWER(${bi}) OR LOWER(sr.genre) LIKE LOWER(${bi}))",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", recherche.trim()));
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM media_content.station_radio sr WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage stations: {}", e)))?;

    // Récupérer les stations avec jointure pays
    let query = format!(
        "SELECT {}, p.nom AS pays_nom
         FROM media_content.station_radio sr
         LEFT JOIN shared.pays p ON p.id = sr.pays_id
         WHERE {}
         ORDER BY sr.a_la_une DESC, sr.nom ASC
         LIMIT ${} OFFSET ${}",
        STATION_RADIO_COLONNES,
        where_clause,
        bind_index,
        bind_index + 1,
    );

    let mut q = sqlx::query_as::<_, StationRadioRow>(&query);
    for val in &bind_values {
        q = q.bind(val);
    }
    q = q.bind(par_page).bind(offset);

    let stations = q.fetch_all(pool.get_ref()).await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing stations: {}", e)))?;

    let total_pages = (total as f64 / par_page as f64).ceil() as i64;

    let reponse = StationRadioListeResponse {
        stations: stations.iter().map(|s| s.to_response()).collect(),
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

// ── GET /api/stations-radio/{id} ──────────────────────────────────────

pub async fn obtenir_station(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let id_str = chemin.into_inner();
    let station_id = Uuid::parse_str(&id_str)
        .map_err(|_| ApiErreur::Validation("ID de station invalide".into()))?;

    let query = format!(
        "SELECT {}, p.nom AS pays_nom
         FROM media_content.station_radio sr
         LEFT JOIN shared.pays p ON p.id = sr.pays_id
         WHERE sr.id = $1 AND sr.etat = 'publie' AND sr.deleted_at IS NULL",
        STATION_RADIO_COLONNES
    );

    let station = sqlx::query_as::<_, StationRadioRow>(&query)
        .bind(station_id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture station: {}", e)))?
        .ok_or_else(|| ApiErreur::NonTrouve("Station radio non trouvée".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(station.to_response()),
        error: None,
    }))
}

// ── GET /api/stations-radio/pays ──────────────────────────────────────

pub async fn lister_pays_stations(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let pays: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT p.nom
         FROM media_content.station_radio sr
         JOIN shared.pays p ON p.id = sr.pays_id
         WHERE sr.etat = 'publie' AND sr.deleted_at IS NULL
         ORDER BY p.nom ASC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing pays: {}", e)))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(pays),
        error: None,
    }))
}

// ── GET /api/stations-radio/genres ────────────────────────────────────

pub async fn lister_genres_stations(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let genres: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT unnest(genres_liste) AS genre
         FROM media_content.station_radio
         WHERE etat = 'publie' AND deleted_at IS NULL
         ORDER BY genre ASC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing genres: {}", e)))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(genres),
        error: None,
    }))
}

// ── POST /api/stations-radio ──────────────────────────────────────────

pub async fn creer_station(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreerStationRadioForm>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    // Validation : audio (fichier/lien) OU flux live, au moins l'un des deux
    if body.nom.trim().is_empty() {
        return Err(ApiErreur::Validation("Le nom de la station est requis".into()));
    }
    let stream_url = body.stream_url.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let audio_url = body.audio_url.as_deref().map(str::trim).filter(|s| !s.is_empty());
    if stream_url.is_none() && audio_url.is_none() {
        return Err(ApiErreur::Validation(
            "Fournissez au moins un fichier/lien audio ou une URL de flux live".into(),
        ));
    }

    let slug = generer_slug(&body.nom);
    let type_station = body.type_station.as_deref()
        .map(mapper_type_station_db)
        .unwrap_or_else(|| "nationale".to_string());

    let genres_liste = body.genres_liste.clone().unwrap_or_default();

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

    let station_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO media_content.station_radio
            (id, nom, slug, description, stream_url, audio_url, image_couverture_url, genre, genres_liste, pays_id, ville, type_station, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12::media_content.type_station, 'publie', $13)"
    )
    .bind(station_id)
    .bind(body.nom.trim())
    .bind(&slug)
    .bind(body.description.as_deref().map(str::trim))
    .bind(stream_url)
    .bind(audio_url)
    .bind(body.image_couverture_url.as_deref().map(str::trim))
    .bind(body.genre.as_deref().map(str::trim))
    .bind(&genres_liste)
    .bind(pays_id)
    .bind(body.ville.as_deref().map(str::trim))
    .bind(&type_station)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur création station: {}", e)))?;

    // Récupérer la station créée avec jointure pays
    let query = format!(
        "SELECT {}, p.nom AS pays_nom
         FROM media_content.station_radio sr
         LEFT JOIN shared.pays p ON p.id = sr.pays_id
         WHERE sr.id = $1",
        STATION_RADIO_COLONNES
    );

    let station = sqlx::query_as::<_, StationRadioRow>(&query)
        .bind(station_id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture station créée: {}", e)))?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(station.to_response()),
        error: None,
    }))
}

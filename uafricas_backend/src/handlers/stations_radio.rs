use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_social;
use crate::models::programme_radio::*;
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

    // Filtre par pays — la sentinelle « aucun filtre » doit correspondre à ce
    // que le frontend envoie réellement. L'interface dit « territoire » là où le
    // code et la base disent « pays » : la valeur « Tous les pays » attendue ici
    // n'arrivait jamais, et le filtre s'appliquait donc à une valeur littérale
    // inexistante, ne renvoyant aucune station.
    if let Some(ref pays) = params.pays {
        if pays != "Tous les territoires" && pays != "Tous les pays" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p2 WHERE p2.id = sr.pays_id AND LOWER(p2.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(pays.clone());
            bind_index += 1;
        }
    }

    // Origine de publication : portée par la page appelante, jamais par un
    // filtre offert au visiteur (FR-014). C'est elle qui rend enfin distinctes
    // les deux pages Radio.
    if let Some(ref origine) = params.origine {
        if !origine_valide(origine) {
            return Err(ApiErreur::Validation(
                "Origine de publication invalide (attendu : africans ou territoire)".into(),
            ));
        }
        conditions.push(format!("sr.origine_publication = ${}", bind_index));
        bind_values.push(origine.clone());
        bind_index += 1;
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

// ── GET /api/stations-radio/slug/{slug} ───────────────────────────────

/// Détail d'une station par son slug — les pages SSR et les aperçus sociaux
/// exigent une URL lisible, que la résolution par identifiant ne donne pas.
pub async fn obtenir_station_par_slug(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = chemin.into_inner();
    let moi = media_social::extraire_utilisateur_id(&req);

    let query = format!(
        "SELECT {}, p.nom AS pays_nom
           FROM media_content.station_radio sr
           LEFT JOIN shared.pays p ON p.id = sr.pays_id
          WHERE sr.slug = $1 AND sr.etat = 'publie' AND sr.deleted_at IS NULL",
        STATION_RADIO_COLONNES
    );

    // Un contenu retiré est indiscernable d'un contenu inexistant (FR-028).
    let station = sqlx::query_as::<_, StationRadioRow>(&query)
        .bind(&slug)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture station: {}", e)))?
        .ok_or_else(|| ApiErreur::NonTrouve("Station radio non trouvée".into()))?;

    let mut reponse = station.to_response();
    let compteurs =
        media_social::compteurs_pour(pool.get_ref(), "station_radio", &[reponse.id], moi).await?;
    reponse.interactions = compteurs.get(&reponse.id).cloned();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PAGES RADIO — SECTIONS PAR STATION (US2)
// ═══════════════════════════════════════════════════════════════════════════

/// Jointures communes aux lectures d'émissions radio. Le libellé du thème phare
/// exige une jointure explicite : `theme_phare_id` est une référence logique,
/// sans clé étrangère.
const JOINTURES_PROGRAMME_RADIO: &str =
    "LEFT JOIN shared.pays p ON p.id = pr.pays_id
     LEFT JOIN media_content.station_radio st ON st.id = pr.station_id
     LEFT JOIN shared.categorie cat ON cat.id = pr.theme_phare_id";

const CHAMPS_JOINTS_PROGRAMME_RADIO: &str =
    "p.nom AS pays_nom, st.nom AS station_nom, st.slug AS station_slug, cat.nom AS theme_phare_nom";

// ── GET /api/stations-radio/sections ──────────────────────────────────

/// Une section par station, prête à l'affichage.
///
/// Le paramètre `origine` est porté par la PAGE (`africans` ou `territoire`) et
/// non par l'utilisateur : c'est lui qui garantit qu'aucune station n'apparaît
/// sur les deux pages (FR-013, FR-014). Les autres filtres restent à la main du
/// visiteur et s'appliquent en plus.
pub async fn lister_sections_stations(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<StationRadioQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = media_social::extraire_utilisateur_id(&req);
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(6).clamp(1, 20);
    let offset = (page - 1) * par_page;
    let contenus_par_section = params.contenus_par_section.unwrap_or(12).clamp(1, 30);

    let mut conditions: Vec<String> = vec![
        "sr.etat = 'publie'".to_string(),
        "sr.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref origine) = params.origine {
        if !origine_valide(origine) {
            return Err(ApiErreur::Validation(
                "Origine de publication invalide (attendu : africans ou territoire)".into(),
            ));
        }
        conditions.push(format!("sr.origine_publication = ${}", bind_index));
        bind_values.push(origine.clone());
        bind_index += 1;
    }

    if let Some(ref type_station) = params.type_station {
        if type_station != "Tous les types" {
            conditions.push(format!("sr.type_station::text = ${}", bind_index));
            bind_values.push(mapper_type_station_db(type_station));
            bind_index += 1;
        }
    }

    if let Some(ref pays) = params.pays {
        if pays != "Tous les territoires" && pays != "Tous les pays" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p2 WHERE p2.id = sr.pays_id AND LOWER(p2.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(pays.clone());
            bind_index += 1;
        }
    }

    if let Some(ref genre) = params.genre {
        if genre != "Tous les genres" {
            conditions.push(format!("${} = ANY(sr.genres_liste)", bind_index));
            bind_values.push(genre.clone());
            bind_index += 1;
        }
    }

    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            conditions.push(format!(
                "(LOWER(sr.nom) LIKE LOWER(${bi}) OR LOWER(sr.description) LIKE LOWER(${bi}))",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", recherche.trim()));
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    let count_query = format!(
        "SELECT COUNT(*) FROM media_content.station_radio sr WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage sections radio: {}", e)))?;

    // Ordre stable entre deux visites : le nom seul ne départage pas deux
    // stations homonymes, d'où l'identifiant en second critère.
    let query_stations = format!(
        "SELECT {}, p.nom AS pays_nom
           FROM media_content.station_radio sr
           LEFT JOIN shared.pays p ON p.id = sr.pays_id
          WHERE {}
          ORDER BY sr.a_la_une DESC, sr.nom ASC, sr.id ASC
          LIMIT ${} OFFSET ${}",
        STATION_RADIO_COLONNES,
        where_clause,
        bind_index,
        bind_index + 1
    );

    let mut q = sqlx::query_as::<_, StationRadioRow>(&query_stations);
    for val in &bind_values {
        q = q.bind(val);
    }
    q = q.bind(par_page).bind(offset);

    let stations = q
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing sections radio: {}", e)))?;

    let requete_contenus = format!(
        "SELECT {}, {}
           FROM media_content.programme_radio pr
           {}
          WHERE pr.station_id = $1
            AND pr.etat = 'publie'
            AND pr.deleted_at IS NULL
          ORDER BY pr.a_la_une DESC, pr.created_at DESC
          LIMIT $2",
        PROGRAMME_RADIO_COLONNES, CHAMPS_JOINTS_PROGRAMME_RADIO, JOINTURES_PROGRAMME_RADIO
    );

    let mut sections: Vec<StationSectionResponse> = Vec::with_capacity(stations.len());

    for station in &stations {
        let total_contenus: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM media_content.programme_radio
              WHERE station_id = $1 AND etat = 'publie' AND deleted_at IS NULL",
        )
        .bind(station.id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| {
            ApiErreur::BaseDeDonnees(format!("Erreur comptage contenus station: {}", e))
        })?;

        // +1 : le contenu mis en évidence est extrait de cette liste, la rangée
        // doit malgré tout compter `contenus_par_section` éléments.
        let contenus = sqlx::query_as::<_, ProgrammeRadioRow>(&requete_contenus)
            .bind(station.id)
            .bind(contenus_par_section + 1)
            .fetch_all(pool.get_ref())
            .await
            .map_err(|e| {
                ApiErreur::BaseDeDonnees(format!("Erreur lecture contenus station: {}", e))
            })?;

        let mut iter = contenus.iter();
        let mis_en_evidence = iter.next().map(|p| p.to_response());
        let autres: Vec<_> = iter.map(|p| p.to_response()).collect();

        // Le direct est offert au même titre qu'une émission enregistrée (FR-016).
        let direct_disponible = station
            .stream_url
            .as_deref()
            .map(|u| !u.trim().is_empty())
            .unwrap_or(false);

        // Résolution paresseuse de la grille : deux requêtes SQL, aucun état
        // conservé, aucune tâche de fond (R7, FR-038).
        let diffusion = crate::handlers::media_programmation::diffusion_pour_support(
            pool.get_ref(),
            "station_radio",
            station.id,
        )
        .await?;

        sections.push(StationSectionResponse {
            station: station.to_response(),
            direct_disponible,
            mis_en_evidence,
            contenus: autres,
            total_contenus,
            diffusion_en_cours: diffusion.diffusion_en_cours,
            creneau_suivant: diffusion.creneau_suivant,
        });
    }

    // Compteurs d'interaction de TOUTE la page en deux requêtes — une par type
    // de cible — plutôt qu'une par carte affichée (FR-027).
    greffer_compteurs_sections_radio(pool.get_ref(), &mut sections, moi).await?;

    let total_pages = (total as f64 / par_page as f64).ceil() as i64;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(StationSectionsListeResponse {
            sections,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAMMES RADIO (émissions) — exposition publique
// ═══════════════════════════════════════════════════════════════════════════
// Ces contenus n'avaient aucun endpoint public, alors que leur équivalent
// télévision en comptait trois (FR-020).

// ── GET /api/programmes-radio ─────────────────────────────────────────

pub async fn lister_programmes_radio(
    pool: web::Data<PgPool>,
    params: web::Query<ProgrammeRadioQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).min(100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec![
        "pr.etat = 'publie'".to_string(),
        "pr.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref pays) = params.pays {
        if pays != "Tous les territoires" && pays != "Tous les pays" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p2 WHERE p2.id = pr.pays_id AND LOWER(p2.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(pays.clone());
            bind_index += 1;
        }
    }

    if let Some(station_id) = params.station {
        conditions.push(format!("pr.station_id = ${}::uuid", bind_index));
        bind_values.push(station_id.to_string());
        bind_index += 1;
    }

    if let Some(ref categorie) = params.categorie_radio {
        if !categorie.trim().is_empty() {
            conditions.push(format!("pr.categorie_radio::text = ${}", bind_index));
            bind_values.push(categorie.clone());
            bind_index += 1;
        }
    }

    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            conditions.push(format!(
                "(LOWER(pr.nom_emission) LIKE LOWER(${bi}) OR LOWER(pr.description) LIKE LOWER(${bi}))",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", recherche.trim()));
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    let count_query = format!(
        "SELECT COUNT(*) FROM media_content.programme_radio pr WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage émissions radio: {}", e)))?;

    let query = format!(
        "SELECT {}, {}
           FROM media_content.programme_radio pr
           {}
          WHERE {}
          ORDER BY pr.a_la_une DESC, pr.created_at DESC
          LIMIT ${} OFFSET ${}",
        PROGRAMME_RADIO_COLONNES,
        CHAMPS_JOINTS_PROGRAMME_RADIO,
        JOINTURES_PROGRAMME_RADIO,
        where_clause,
        bind_index,
        bind_index + 1
    );

    let mut q = sqlx::query_as::<_, ProgrammeRadioRow>(&query);
    for val in &bind_values {
        q = q.bind(val);
    }
    q = q.bind(par_page).bind(offset);

    let programmes = q
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing émissions radio: {}", e)))?;

    let total_pages = (total as f64 / par_page as f64).ceil() as i64;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ProgrammeRadioListeResponse {
            programmes: programmes.iter().map(|p| p.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ── GET /api/programmes-radio/slug/{slug} ─────────────────────────────

pub async fn obtenir_programme_radio_par_slug(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = chemin.into_inner();
    let moi = media_social::extraire_utilisateur_id(&req);

    let query = format!(
        "SELECT {}, {}
           FROM media_content.programme_radio pr
           {}
          WHERE pr.slug = $1 AND pr.etat = 'publie' AND pr.deleted_at IS NULL",
        PROGRAMME_RADIO_COLONNES, CHAMPS_JOINTS_PROGRAMME_RADIO, JOINTURES_PROGRAMME_RADIO
    );

    let programme = sqlx::query_as::<_, ProgrammeRadioRow>(&query)
        .bind(&slug)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture émission radio: {}", e)))?
        .ok_or_else(|| ApiErreur::NonTrouve("Émission radio non trouvée".into()))?;

    let mut reponse = programme.to_response();
    let compteurs =
        media_social::compteurs_pour(pool.get_ref(), "programme_radio", &[reponse.id], moi).await?;
    reponse.interactions = compteurs.get(&reponse.id).cloned();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
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

    // FAILLE FERMÉE (FR-031, FR-032) : cette route publique insérait
    // `etat = 'publie'` en dur, sans contrôle de rôle — tout membre connecté
    // publiait donc directement. Le contenu naît désormais en `'en_attente'`,
    // invisible tant qu'un administrateur ne l'a pas validé. La voie de
    // contribution nominale reste `POST /api/medias/propositions` (US4).
    sqlx::query(
        "INSERT INTO media_content.station_radio
            (id, nom, slug, description, stream_url, audio_url, image_couverture_url, genre, genres_liste, pays_id, ville, type_station, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12::media_content.type_station, 'en_attente', $13)"
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

/// Greffe les compteurs d'interaction sur toutes les cartes d'un lot de
/// sections radio : deux requêtes au total, quel que soit le nombre de cartes.
async fn greffer_compteurs_sections_radio(
    pool: &PgPool,
    sections: &mut [StationSectionResponse],
    moi: Option<Uuid>,
) -> Result<(), ApiErreur> {
    let ids_stations: Vec<Uuid> = sections.iter().map(|s| s.station.id).collect();
    let ids_programmes: Vec<Uuid> = sections
        .iter()
        .flat_map(|s| {
            s.mis_en_evidence
                .iter()
                .map(|p| p.id)
                .chain(s.contenus.iter().map(|p| p.id))
        })
        .collect();

    let compteurs_stations =
        media_social::compteurs_pour(pool, "station_radio", &ids_stations, moi).await?;
    let compteurs_programmes =
        media_social::compteurs_pour(pool, "programme_radio", &ids_programmes, moi).await?;

    for section in sections.iter_mut() {
        section.station.interactions = compteurs_stations.get(&section.station.id).cloned();
        if let Some(ref mut p) = section.mis_en_evidence {
            p.interactions = compteurs_programmes.get(&p.id).cloned();
        }
        for p in section.contenus.iter_mut() {
            p.interactions = compteurs_programmes.get(&p.id).cloned();
        }
    }
    Ok(())
}

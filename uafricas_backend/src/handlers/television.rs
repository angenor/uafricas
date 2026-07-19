use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_social;
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
    // Flux live optionnel — le cœur de la télé = les programmes (vidéos fichier/lien).
    let stream_url = body.stream_url.as_deref().map(str::trim).filter(|s| !s.is_empty());

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

    // FAILLE FERMÉE (FR-031, FR-032) : cette route publique insérait
    // `etat = 'publie'` en dur, sans le moindre contrôle de rôle — tout membre
    // connecté publiait donc directement sur les pages Télé. Le contenu naît
    // désormais en `'en_attente'` : il n'est visible nulle part tant qu'un
    // administrateur ne l'a pas validé depuis le back-office. La voie de
    // contribution nominale reste `POST /api/medias/propositions`, qui alimente
    // la file de modération (US4).
    sqlx::query(
        "INSERT INTO media_content.chaine_tv
            (id, nom, slug, description, stream_url, categorie, pays_id, langue, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6::media_content.categorie_chaine_tv, $7, $8, 'en_attente', $9)"
    )
    .bind(chaine_id)
    .bind(body.nom.trim())
    .bind(&slug)
    .bind(body.description.as_deref().map(str::trim))
    .bind(stream_url)
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
// PROGRAMMES TÉLÉ (media_content.programme_tele — table dédiée depuis 09g)
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

    // Filtre par chaîne (télé)
    if let Some(chaine_id) = params.chaine {
        conditions.push(format!("prt.chaine_id = ${}::uuid", bind_index));
        bind_values.push(chaine_id.to_string());
        bind_index += 1;
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
        "SELECT COUNT(*) FROM media_content.programme_tele prt WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage programmes TV: {}", e)))?;

    // Récupérer les programmes avec jointure pays + chaîne (télé de rattachement)
    let query = format!(
        "SELECT {}, p.nom AS pays_nom, c.nom AS chaine_nom
         FROM media_content.programme_tele prt
         LEFT JOIN shared.pays p ON p.id = prt.pays_id
         LEFT JOIN media_content.chaine_tv c ON c.id = prt.chaine_id
         WHERE {}
         ORDER BY prt.a_la_une DESC, prt.created_at DESC
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
        "SELECT {}, p.nom AS pays_nom, c.nom AS chaine_nom
         FROM media_content.programme_tele prt
         LEFT JOIN shared.pays p ON p.id = prt.pays_id
         LEFT JOIN media_content.chaine_tv c ON c.id = prt.chaine_id
         WHERE prt.id = $1 AND prt.etat = 'publie' AND prt.deleted_at IS NULL",
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

    // FAILLE FERMÉE (FR-031, FR-032) : cette route publique insérait
    // `etat = 'publie'` en dur, sans contrôle de rôle — tout membre connecté
    // publiait donc directement. Le contenu naît désormais en `'en_attente'`,
    // invisible tant qu'un administrateur ne l'a pas validé. La voie de
    // contribution nominale reste `POST /api/medias/propositions` (US4).
    sqlx::query(
        "INSERT INTO media_content.programme_tele
            (id, nom_emission, slug, description, video_url, image_couverture_url,
             info_animateur, info_producteur, pays_id, est_international, langue, chaine_id, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 'en_attente', $13)"
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
    .bind(body.chaine_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur création programme TV: {}", e)))?;

    // Récupérer le programme créé avec jointure pays + chaîne
    let query = format!(
        "SELECT {}, p.nom AS pays_nom, c.nom AS chaine_nom
         FROM media_content.programme_tele prt
         LEFT JOIN shared.pays p ON p.id = prt.pays_id
         LEFT JOIN media_content.chaine_tv c ON c.id = prt.chaine_id
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
// PAGE TÉLÉ — VEDETTE ET SECTIONS (US1)
// ═══════════════════════════════════════════════════════════════════════════

/// Jointures communes aux lectures de programmes télé : pays, chaîne (nom et
/// slug, ce dernier servant les liens vers la page de détail) et libellé du
/// thème phare — `theme_phare_id` étant une référence logique sans FK, seule
/// une jointure explicite en rapporte le nom.
const JOINTURES_PROGRAMME_TELE: &str =
    "LEFT JOIN shared.pays p ON p.id = prt.pays_id
     LEFT JOIN media_content.chaine_tv c ON c.id = prt.chaine_id
     LEFT JOIN shared.categorie cat ON cat.id = prt.theme_phare_id";

const CHAMPS_JOINTS_PROGRAMME_TELE: &str =
    "p.nom AS pays_nom, c.nom AS chaine_nom, c.slug AS chaine_slug, cat.nom AS theme_phare_nom";

// ── GET /api/television/vedette ───────────────────────────────────────

/// Programme mis en avant sur toute la page Télé (FR-001).
///
/// Repli déterministe (FR-007) : faute de vedette désignée ET publiée, on sert
/// le programme publié le plus récent, signalé par `est_repli`. Sans aucun
/// programme publié, `data` vaut `null` — la page affiche alors son message
/// d'état vide plutôt qu'un lecteur en erreur.
pub async fn obtenir_vedette(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = media_social::extraire_utilisateur_id(&req);
    let requete_vedette = format!(
        "SELECT {}, {}
           FROM media_content.programme_tele prt
           {}
          WHERE prt.a_la_une_globale = TRUE
            AND prt.etat = 'publie'
            AND prt.deleted_at IS NULL
          LIMIT 1",
        PROGRAMME_TELE_COLONNES, CHAMPS_JOINTS_PROGRAMME_TELE, JOINTURES_PROGRAMME_TELE
    );

    let vedette = sqlx::query_as::<_, ProgrammeTeleRow>(&requete_vedette)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture vedette télé: {}", e)))?;

    let (programme, est_repli) = match vedette {
        Some(p) => (Some(p), false),
        None => {
            let requete_repli = format!(
                "SELECT {}, {}
                   FROM media_content.programme_tele prt
                   {}
                  WHERE prt.etat = 'publie'
                    AND prt.deleted_at IS NULL
                    AND prt.video_url IS NOT NULL
                  ORDER BY prt.created_at DESC
                  LIMIT 1",
                PROGRAMME_TELE_COLONNES, CHAMPS_JOINTS_PROGRAMME_TELE, JOINTURES_PROGRAMME_TELE
            );
            let repli = sqlx::query_as::<_, ProgrammeTeleRow>(&requete_repli)
                .fetch_optional(pool.get_ref())
                .await
                .map_err(|e| {
                    ApiErreur::BaseDeDonnees(format!("Erreur lecture repli vedette télé: {}", e))
                })?;
            (repli, true)
        }
    };

    let data = match programme {
        Some(p) => {
            let mut reponse = p.to_response();
            let compteurs =
                media_social::compteurs_pour(pool.get_ref(), "programme_tele", &[reponse.id], moi)
                    .await?;
            reponse.interactions = compteurs.get(&reponse.id).cloned();
            Some(ProgrammeVedetteResponse {
                programme: reponse,
                est_repli,
            })
        }
        None => None,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data,
        error: None,
    }))
}

// ── GET /api/television/sections ──────────────────────────────────────

/// Une section par chaîne, prête à l'affichage (FR-004, FR-005, FR-008).
///
/// Les sections sont paginées et chargées au défilement : servir d'un bloc les
/// programmes de toutes les chaînes est précisément ce que la page faisait, et
/// ce que FR-054 proscrit.
pub async fn lister_sections(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<TeleSectionsQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = media_social::extraire_utilisateur_id(&req);
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(6).clamp(1, 20);
    let offset = (page - 1) * par_page;
    let contenus_par_section = params.contenus_par_section.unwrap_or(12).clamp(1, 30);

    // Une chaîne sans aucun contenu publié ne donne pas de section (FR-008).
    let mut conditions: Vec<String> = vec![
        "ct.etat = 'publie'".to_string(),
        "ct.deleted_at IS NULL".to_string(),
        "EXISTS (SELECT 1 FROM media_content.programme_tele pt
                  WHERE pt.chaine_id = ct.id
                    AND pt.etat = 'publie'
                    AND pt.deleted_at IS NULL)"
            .to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref categorie) = params.categorie {
        if categorie != "Toutes les catégories" {
            conditions.push(format!("ct.categorie::text = ${}", bind_index));
            bind_values.push(mapper_categorie_chaine_db(categorie));
            bind_index += 1;
        }
    }

    if let Some(ref pays) = params.pays {
        // Le frontend envoie « Tous les territoires » : la terminologie d'affichage
        // dit « territoire » là où le code et la base disent « pays ».
        if pays != "Tous les territoires" && pays != "Tous les pays" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p2 WHERE p2.id = ct.pays_id AND LOWER(p2.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(pays.clone());
            bind_index += 1;
        }
    }

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

    let count_query = format!(
        "SELECT COUNT(*) FROM media_content.chaine_tv ct WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage sections télé: {}", e)))?;

    // Ordre stable entre deux visites (FR-004) : le nom seul ne départage pas
    // deux chaînes homonymes, d'où l'identifiant en second critère.
    let query_chaines = format!(
        "SELECT {}, p.nom AS pays_nom
           FROM media_content.chaine_tv ct
           LEFT JOIN shared.pays p ON p.id = ct.pays_id
          WHERE {}
          ORDER BY ct.nom ASC, ct.id ASC
          LIMIT ${} OFFSET ${}",
        CHAINE_TV_COLONNES,
        where_clause,
        bind_index,
        bind_index + 1
    );

    let mut q = sqlx::query_as::<_, ChaineTvRow>(&query_chaines);
    for val in &bind_values {
        q = q.bind(val);
    }
    q = q.bind(par_page).bind(offset);

    let chaines = q
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing sections télé: {}", e)))?;

    // Contenus de chaque chaîne. Une requête par section, sur au plus `par_page`
    // sections : le coût reste borné, et le SQL demeure lisible.
    let requete_contenus = format!(
        "SELECT {}, {}
           FROM media_content.programme_tele prt
           {}
          WHERE prt.chaine_id = $1
            AND prt.etat = 'publie'
            AND prt.deleted_at IS NULL
          ORDER BY prt.a_la_une DESC, prt.created_at DESC
          LIMIT $2",
        PROGRAMME_TELE_COLONNES, CHAMPS_JOINTS_PROGRAMME_TELE, JOINTURES_PROGRAMME_TELE
    );

    let mut sections: Vec<TeleSectionResponse> = Vec::with_capacity(chaines.len());

    for chaine in &chaines {
        let total_contenus: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM media_content.programme_tele
              WHERE chaine_id = $1 AND etat = 'publie' AND deleted_at IS NULL",
        )
        .bind(chaine.id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage contenus chaîne: {}", e)))?;

        // +1 : le contenu mis en évidence est extrait de cette liste, la rangée
        // doit malgré tout compter `contenus_par_section` éléments.
        let contenus = sqlx::query_as::<_, ProgrammeTeleRow>(&requete_contenus)
            .bind(chaine.id)
            .bind(contenus_par_section + 1)
            .fetch_all(pool.get_ref())
            .await
            .map_err(|e| {
                ApiErreur::BaseDeDonnees(format!("Erreur lecture contenus chaîne: {}", e))
            })?;

        // `ORDER BY a_la_une DESC` place la mise en avant de la chaîne en tête ;
        // à défaut, c'est le contenu le plus récent qui occupe la place (FR-005).
        let mut iter = contenus.iter();
        let mis_en_evidence = iter.next().map(|p| p.to_response());
        let autres: Vec<_> = iter.map(|p| p.to_response()).collect();

        // Résolution paresseuse de la grille : deux requêtes SQL, aucun état
        // conservé, aucune tâche de fond (R7, FR-038).
        let diffusion = crate::handlers::media_programmation::diffusion_pour_support(
            pool.get_ref(),
            "chaine_tv",
            chaine.id,
        )
        .await?;

        sections.push(TeleSectionResponse {
            chaine: chaine.to_response(),
            mis_en_evidence,
            contenus: autres,
            total_contenus,
            diffusion_en_cours: diffusion.diffusion_en_cours,
            creneau_suivant: diffusion.creneau_suivant,
        });
    }

    // Compteurs d'interaction de TOUTE la page en deux requêtes — une par type
    // de cible — plutôt qu'une par carte affichée (FR-027).
    greffer_compteurs_sections(pool.get_ref(), &mut sections, moi).await?;

    let total_pages = (total as f64 / par_page as f64).ceil() as i64;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(TeleSectionsListeResponse {
            sections,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ── GET /api/television/chaines-slug/{slug} ───────────────────────────

/// Détail d'une chaîne par son slug — les pages SSR et les aperçus sociaux
/// exigent une URL lisible, que la résolution par identifiant ne donne pas.
pub async fn obtenir_chaine_par_slug(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = chemin.into_inner();
    let moi = media_social::extraire_utilisateur_id(&req);

    let query = format!(
        "SELECT {}, p.nom AS pays_nom
           FROM media_content.chaine_tv ct
           LEFT JOIN shared.pays p ON p.id = ct.pays_id
          WHERE ct.slug = $1 AND ct.etat = 'publie' AND ct.deleted_at IS NULL",
        CHAINE_TV_COLONNES
    );

    // Un contenu retiré est indiscernable d'un contenu inexistant (FR-028).
    let chaine = sqlx::query_as::<_, ChaineTvRow>(&query)
        .bind(&slug)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture chaîne TV: {}", e)))?
        .ok_or_else(|| ApiErreur::NonTrouve("Chaîne TV non trouvée".into()))?;

    let mut reponse = chaine.to_response();
    let compteurs =
        media_social::compteurs_pour(pool.get_ref(), "chaine_tv", &[reponse.id], moi).await?;
    reponse.interactions = compteurs.get(&reponse.id).cloned();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ── GET /api/television/programmes-slug/{slug} ───────────────────────

pub async fn obtenir_programme_par_slug(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = chemin.into_inner();
    let moi = media_social::extraire_utilisateur_id(&req);

    let query = format!(
        "SELECT {}, {}
           FROM media_content.programme_tele prt
           {}
          WHERE prt.slug = $1 AND prt.etat = 'publie' AND prt.deleted_at IS NULL",
        PROGRAMME_TELE_COLONNES, CHAMPS_JOINTS_PROGRAMME_TELE, JOINTURES_PROGRAMME_TELE
    );

    let programme = sqlx::query_as::<_, ProgrammeTeleRow>(&query)
        .bind(&slug)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture programme télé: {}", e)))?
        .ok_or_else(|| ApiErreur::NonTrouve("Programme télé non trouvé".into()))?;

    let mut reponse = programme.to_response();
    let compteurs =
        media_social::compteurs_pour(pool.get_ref(), "programme_tele", &[reponse.id], moi).await?;
    reponse.interactions = compteurs.get(&reponse.id).cloned();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
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
             SELECT pays_id FROM media_content.programme_tele WHERE etat = 'publie' AND deleted_at IS NULL AND pays_id IS NOT NULL
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
        "SELECT COUNT(*) FROM media_content.programme_tele WHERE etat = 'publie' AND deleted_at IS NULL"
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

/// Greffe les compteurs d'interaction sur toutes les cartes d'un lot de
/// sections : deux requêtes au total (chaînes d'une part, programmes de
/// l'autre), quel que soit le nombre de cartes affichées.
async fn greffer_compteurs_sections(
    pool: &PgPool,
    sections: &mut [TeleSectionResponse],
    moi: Option<Uuid>,
) -> Result<(), ApiErreur> {
    let ids_chaines: Vec<Uuid> = sections.iter().map(|s| s.chaine.id).collect();
    let ids_programmes: Vec<Uuid> = sections
        .iter()
        .flat_map(|s| {
            s.mis_en_evidence
                .iter()
                .map(|p| p.id)
                .chain(s.contenus.iter().map(|p| p.id))
        })
        .collect();

    let compteurs_chaines =
        media_social::compteurs_pour(pool, "chaine_tv", &ids_chaines, moi).await?;
    let compteurs_programmes =
        media_social::compteurs_pour(pool, "programme_tele", &ids_programmes, moi).await?;

    for section in sections.iter_mut() {
        section.chaine.interactions = compteurs_chaines.get(&section.chaine.id).cloned();
        if let Some(ref mut p) = section.mis_en_evidence {
            p.interactions = compteurs_programmes.get(&p.id).cloned();
        }
        for p in section.contenus.iter_mut() {
            p.interactions = compteurs_programmes.get(&p.id).cloned();
        }
    }
    Ok(())
}

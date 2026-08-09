//! Espace Télé — chaînes, programmes et épisodes
//! (feature 001-refonte-tele-radio ; recadré par 009-medias-programmes-episodes).
//!
//! La page ne présente plus une vignette par vidéo mais **une rangée par
//! programme** : chaque chaîne porte ses programmes publiés, chaque programme
//! annonce son nombre d'épisodes et un aperçu borné. Au-delà, la page du
//! programme prend le relais — c'est ce qui tient la promesse de navigabilité à
//! 500 épisodes.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_emission::{
    emissions_publiees_par_supports, greffer_apercus_et_compteurs, lister_episodes_emission,
    obtenir_emission_par_slug,
};
use crate::handlers::media_episode::obtenir_episode_par_slug;
use crate::handlers::media_social;
// Parsing du filtre `?thematique=` : utilitaire de MODÈLE, pas un handler.
use crate::models::media_support::thematiques_demandees;
use crate::handlers::media_support::{
    couverture_par_supports, territoires_disponibles, thematiques_disponibles,
    thematiques_par_supports,
};
use crate::models::media_emission::EmissionResponse;
use crate::models::media_episode::{colonnes_episode, EpisodeRow, ORDRE_EPISODES};
use crate::models::television::*;

const TYPE_SUPPORT: &str = "chaine_tv";

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

    if let Some(ref categorie) = params.categorie
        && categorie != "Toutes les catégories"
    {
        let cat_db = mapper_categorie_chaine_db(categorie);
        conditions.push(format!("ct.categorie::text = ${}", bind_index));
        bind_values.push(cat_db);
        bind_index += 1;
    }

    if let Some(ref pays) = params.pays
        && pays != "Tous les pays"
    {
        conditions.push(format!(
            "EXISTS (SELECT 1 FROM shared.pays p2 WHERE p2.id = ct.pays_id AND LOWER(p2.nom) = LOWER(${}))",
            bind_index
        ));
        bind_values.push(pays.clone());
        bind_index += 1;
    }

    if let Some(ref recherche) = params.recherche
        && !recherche.trim().is_empty()
    {
        conditions.push(format!(
            "(LOWER(ct.nom) LIKE LOWER(${bi}) OR LOWER(ct.description) LIKE LOWER(${bi}))",
            bi = bind_index
        ));
        bind_values.push(format!("%{}%", recherche.trim()));
        bind_index += 1;
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
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur comptage chaînes TV: {}", e)))?;

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

    let chaines = q
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur listing chaînes TV: {}", e)))?;

    let total_pages = (total as f64 / par_page as f64).ceil() as i64;

    let mut reponses: Vec<ChaineTvResponse> = chaines.iter().map(|c| c.to_response()).collect();
    greffer_fiche_support(pool.get_ref(), &mut reponses).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ChaineTvListeResponse {
            chaines: reponses,
            total,
            page,
            par_page,
            total_pages,
        }),
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

    let mut reponses = vec![chaine.to_response()];
    greffer_fiche_support(pool.get_ref(), &mut reponses).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: reponses.pop(),
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

    if body.nom.trim().is_empty() {
        return Err(ApiErreur::Validation(
            "Le nom de la chaîne est requis".into(),
        ));
    }
    let stream_url = body
        .stream_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let slug = generer_slug(&body.nom);
    let categorie = body
        .categorie
        .as_deref()
        .map(mapper_categorie_chaine_db)
        .unwrap_or_else(|| "generaliste".to_string());
    let langue = body.langue.as_deref().unwrap_or("Français");

    let pays_id: Option<Uuid> = if let Some(ref pays_nom) = body.pays {
        sqlx::query_scalar("SELECT id FROM shared.pays WHERE LOWER(nom) = LOWER($1)")
            .bind(pays_nom)
            .fetch_optional(pool.get_ref())
            .await
            .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur résolution pays: {}", e)))?
    }
    else {
        None
    };

    let chaine_id = Uuid::new_v4();

    // FAILLE FERMÉE (FR-031, FR-032) : cette route publique insérait
    // `etat = 'publie'` en dur, sans le moindre contrôle de rôle. Le contenu
    // naît désormais en `'en_attente'` : il n'est visible nulle part tant qu'un
    // administrateur ne l'a pas validé.
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
// PAGE TÉLÉ — VEDETTE ET SECTIONS
// ═══════════════════════════════════════════════════════════════════════════

// ── GET /api/television/vedette ───────────────────────────────────────

/// Vedette de toute la page Télé.
///
/// Elle désigne une **unité diffusable** (FR-052) : l'épisode portant
/// `a_la_une_globale`, accompagné de son programme et de sa chaîne. Repli
/// déterministe faute de vedette désignée : l'épisode publié le plus récent,
/// signalé par `est_repli`.
pub async fn obtenir_vedette(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = media_social::extraire_utilisateur_id(&req);
    let colonnes = colonnes_episode(TYPE_SUPPORT).expect("type de support constant");

    let base = format!(
        "SELECT {colonnes},
                m.titre AS emission_titre, m.slug AS emission_slug,
                m.cadence AS emission_cadence,
                s.id AS support_id, s.nom AS support_nom, s.slug AS support_slug
           FROM media_content.episode_tele ep
           JOIN media_content.emission_tele m ON m.id = ep.emission_id AND m.deleted_at IS NULL
           JOIN media_content.chaine_tv s ON s.id = m.chaine_id
          WHERE ep.etat = 'publie' AND ep.deleted_at IS NULL
            AND m.etat = 'publie' AND s.etat = 'publie' AND s.deleted_at IS NULL"
    );

    let vedette = sqlx::query_as::<_, EpisodeRow>(&format!(
        "{base} AND ep.a_la_une_globale = TRUE LIMIT 1"
    ))
    .fetch_optional(pool.get_ref())
    .await?;

    let (episode, est_repli) = match vedette {
        Some(e) => (Some(e), false),
        None => {
            let repli = sqlx::query_as::<_, EpisodeRow>(&format!(
                "{base} AND ep.video_url IS NOT NULL ORDER BY ep.created_at DESC LIMIT 1"
            ))
            .fetch_optional(pool.get_ref())
            .await?;
            (repli, true)
        }
    };

    let data = match episode {
        Some(row) => {
            let mut reponse = row.to_response(TYPE_SUPPORT);
            let compteurs =
                media_social::compteurs_pour(pool.get_ref(), "episode_tele", &[reponse.id], moi)
                    .await?;
            reponse.interactions = compteurs.get(&reponse.id).cloned();
            Some(VedetteTeleResponse {
                episode: reponse,
                emission: None,
                chaine: None,
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

/// Une section par chaîne, prête à l'affichage.
///
/// **Aucune requête N+1** : les programmes de toutes les chaînes de la page sont
/// chargés en une passe (fenêtre `ROW_NUMBER() OVER (PARTITION BY chaine)`),
/// leurs aperçus d'épisodes en une seconde, et les compteurs d'interaction en
/// deux — quel que soit le nombre de chaînes affichées (SC-010).
pub async fn lister_sections(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<TeleSectionsQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = media_social::extraire_utilisateur_id(&req);
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(6).clamp(1, 20);
    let offset = (page - 1) * par_page;
    let emissions_par_section = params.contenus_par_section.unwrap_or(12).clamp(1, 30);

    // Une chaîne dont aucun programme n'a d'épisode publié ne donne pas de
    // section : elle n'aurait rien à montrer (FR-011).
    let mut conditions: Vec<String> = vec![
        "ct.etat = 'publie'".to_string(),
        "ct.deleted_at IS NULL".to_string(),
        "EXISTS (SELECT 1 FROM media_content.emission_tele m
                  JOIN media_content.episode_tele ep
                    ON ep.emission_id = m.id AND ep.etat = 'publie' AND ep.deleted_at IS NULL
                  WHERE m.chaine_id = ct.id
                    AND m.etat = 'publie' AND m.deleted_at IS NULL)"
            .to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref categorie) = params.categorie
        && categorie != "Toutes les catégories"
    {
        conditions.push(format!("ct.categorie::text = ${}", bind_index));
        bind_values.push(mapper_categorie_chaine_db(categorie));
        bind_index += 1;
    }

    if let Some(ref pays) = params.pays {
        // Le frontend envoie « Tous les territoires » : la terminologie
        // d'affichage dit « territoire » là où la base dit « pays ».
        if pays != "Tous les territoires" && pays != "Tous les pays" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p2 WHERE p2.id = ct.pays_id AND LOWER(p2.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(pays.clone());
            bind_index += 1;
        }
    }

    // « Africans Télé International » : les chaînes produites par la plateforme
    // (09o). Une valeur hors référentiel est ignorée plutôt que rejetée — un
    // filtre inconnu ne doit pas casser la page.
    if let Some(ref origine) = params.origine {
        let o = origine.trim();
        if o == "africans" || o == "territoire" {
            conditions.push(format!("ct.origine_publication = ${}", bind_index));
            bind_values.push(o.to_string());
            bind_index += 1;
        }
    }

    // « Chaînes thématiques » : le thème phare est porté par les PROGRAMMES. Une
    // chaîne remonte dès qu'elle diffuse au moins un programme publié sur ce
    // thème — la section décrit la chaîne, pas le thème.
    if let Some(theme_id) = params.theme {
        conditions.push(format!(
            "EXISTS (SELECT 1 FROM media_content.emission_tele m2
                      WHERE m2.chaine_id = ct.id
                        AND m2.theme_phare_id = ${}::uuid
                        AND m2.etat = 'publie'
                        AND m2.deleted_at IS NULL)",
            bind_index
        ));
        bind_values.push(theme_id.to_string());
        bind_index += 1;
    }

    // Thématiques DÉCLARÉES par la chaîne (US3), entendues comme un **OU** :
    // la barre de filtres affiche un décompte par thème, cocher deux thèmes doit
    // donc élargir la sélection, non la restreindre à leur intersection — qui
    // serait vide presque à coup sûr.
    //
    // Un seul `EXISTS` sur `= ANY(...)` plutôt qu'un `EXISTS` par thème joint en
    // AND : c'est ce qui porte le OU, et l'`EXISTS` garantit du même coup qu'une
    // chaîne portant deux des thèmes demandés ne remonte qu'**une fois** (FR-030).
    let thematiques_filtrees = thematiques_demandees(params.thematique.as_deref())?;
    if !thematiques_filtrees.is_empty() {
        conditions.push(format!(
            "EXISTS (SELECT 1 FROM media_content.support_thematique st
                      WHERE st.type_support = 'chaine_tv'
                        AND st.support_id = ct.id
                        AND st.categorie_id = ANY(${}::uuid[]))",
            bind_index
        ));
        bind_values.push(format!(
            "{{{}}}",
            thematiques_filtrees
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
        bind_index += 1;
    }

    // Territoire couvert (US4) : les chaînes continentales remontent sur
    // **chaque** territoire — c'est FR-036 en une clause.
    if let Some(territoire) = params.territoire {
        conditions.push(format!(
            "(ct.couverture_continentale = TRUE
              OR EXISTS (SELECT 1 FROM media_content.support_territoire ste
                          WHERE ste.type_support = 'chaine_tv'
                            AND ste.support_id = ct.id
                            AND ste.pays_id = ${}::uuid))",
            bind_index
        ));
        bind_values.push(territoire.to_string());
        bind_index += 1;
    }

    if params.en_direct.unwrap_or(false) {
        conditions.push("ct.est_en_direct = TRUE".to_string());
    }

    if let Some(ref recherche) = params.recherche
        && !recherche.trim().is_empty()
    {
        conditions.push(format!(
            "(LOWER(ct.nom) LIKE LOWER(${bi}) OR LOWER(ct.description) LIKE LOWER(${bi}))",
            bi = bind_index
        ));
        bind_values.push(format!("%{}%", recherche.trim()));
        bind_index += 1;
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

    // Ordre stable entre deux visites : le nom seul ne départage pas deux
    // chaînes homonymes, d'où l'identifiant en second critère.
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

    let ids: Vec<Uuid> = chaines.iter().map(|c| c.id).collect();
    let mut par_chaine = emissions_publiees_par_supports(
        pool.get_ref(),
        TYPE_SUPPORT,
        &ids,
        emissions_par_section,
    )
    .await?;

    let mut sections: Vec<TeleSectionResponse> = Vec::with_capacity(chaines.len());
    for chaine in &chaines {
        let (emissions, total_emissions) = par_chaine.remove(&chaine.id).unwrap_or((Vec::new(), 0));

        // Résolution paresseuse de la grille : deux requêtes SQL, aucun état
        // conservé, aucune tâche de fond.
        let diffusion = crate::handlers::media_programmation::diffusion_pour_support(
            pool.get_ref(),
            TYPE_SUPPORT,
            chaine.id,
        )
        .await?;

        sections.push(TeleSectionResponse {
            chaine: chaine.to_response(),
            emissions,
            total_emissions,
            diffusion_en_cours: diffusion.diffusion_en_cours,
            creneau_suivant: diffusion.creneau_suivant,
        });
    }

    // Aperçus d'épisodes et compteurs de TOUTE la page, en un lot.
    let mut toutes: Vec<&mut EmissionResponse> = sections
        .iter_mut()
        .flat_map(|s| s.emissions.iter_mut())
        .collect();
    greffer_apercus_et_compteurs(pool.get_ref(), TYPE_SUPPORT, &mut toutes, moi).await?;

    let ids_chaines: Vec<Uuid> = sections.iter().map(|s| s.chaine.id).collect();
    let compteurs_chaines =
        media_social::compteurs_pour(pool.get_ref(), "chaine_tv", &ids_chaines, moi).await?;
    for section in sections.iter_mut() {
        section.chaine.interactions = compteurs_chaines.get(&section.chaine.id).cloned();
    }

    greffer_fiche_support_sections(pool.get_ref(), &mut sections).await?;

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

    // Un contenu retiré est indiscernable d'un contenu inexistant.
    let chaine = sqlx::query_as::<_, ChaineTvRow>(&query)
        .bind(&slug)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur lecture chaîne TV: {}", e)))?
        .ok_or_else(|| ApiErreur::NonTrouve("Chaîne TV non trouvée".into()))?;

    let mut reponses = vec![chaine.to_response()];
    greffer_fiche_support(pool.get_ref(), &mut reponses).await?;
    let mut reponse = reponses.pop().expect("une chaîne");

    let compteurs =
        media_social::compteurs_pour(pool.get_ref(), "chaine_tv", &[reponse.id], moi).await?;
    reponse.interactions = compteurs.get(&reponse.id).cloned();

    // Les programmes de la chaîne, avec leur aperçu d'épisodes : la page déplie
    // ainsi le catalogue à deux niveaux sans second appel.
    let mut par_chaine =
        emissions_publiees_par_supports(pool.get_ref(), TYPE_SUPPORT, &[reponse.id], 50).await?;
    let (mut emissions, total_emissions) =
        par_chaine.remove(&reponse.id).unwrap_or((Vec::new(), 0));
    let mut refs: Vec<&mut EmissionResponse> = emissions.iter_mut().collect();
    greffer_apercus_et_compteurs(pool.get_ref(), TYPE_SUPPORT, &mut refs, moi).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "chaine": reponse,
            "emissions": emissions,
            "total_emissions": total_emissions,
        })),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PROGRAMMES ET ÉPISODES
// ═══════════════════════════════════════════════════════════════════════════

// ── GET /api/television/emissions/slug/{slug} ─────────────────────────

pub async fn obtenir_emission_slug(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = chemin.into_inner();
    let moi = media_social::extraire_utilisateur_id(&req);
    let emission = obtenir_emission_par_slug(pool.get_ref(), TYPE_SUPPORT, &slug, moi).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(emission),
        error: None,
    }))
}

// ── GET /api/television/emissions/{id}/episodes ───────────────────────

#[derive(serde::Deserialize)]
pub struct PaginationEpisodes {
    pub page: Option<i64>,
    pub taille: Option<i64>,
}

pub async fn lister_episodes_publics(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    params: web::Query<PaginationEpisodes>,
) -> Result<HttpResponse, ApiErreur> {
    let emission_id = chemin.into_inner();
    let moi = media_social::extraire_utilisateur_id(&req);
    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(24).clamp(1, 100);

    let data = lister_episodes_emission(
        pool.get_ref(),
        TYPE_SUPPORT,
        emission_id,
        page,
        taille,
        moi,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
    }))
}

// ── GET /api/television/episodes/slug/{slug} ──────────────────────────

/// Remplace `GET /api/television/programmes-slug/{slug}`. Les slugs ayant été
/// conservés par 09q, les adresses publiques existantes continuent de résoudre
/// (FR-056) et affichent désormais la page d'épisode.
pub async fn obtenir_episode_slug(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = chemin.into_inner();
    let moi = media_social::extraire_utilisateur_id(&req);
    let data = obtenir_episode_par_slug(pool.get_ref(), TYPE_SUPPORT, &slug, moi).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// RÉFÉRENTIELS DE FILTRE ET STATS
// ═══════════════════════════════════════════════════════════════════════════

// ── GET /api/television/thematiques ───────────────────────────────────

pub async fn lister_thematiques(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let data = thematiques_disponibles(pool.get_ref(), TYPE_SUPPORT).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
    }))
}

// ── GET /api/television/territoires ───────────────────────────────────

pub async fn lister_territoires(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let data = territoires_disponibles(pool.get_ref(), TYPE_SUPPORT).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
    }))
}

// ── GET /api/television/pays ──────────────────────────────────────────
//
// Conservé le temps du portage frontend, puis retiré au profit de
// `/territoires` : il n'expose que le pays unique de la chaîne.

pub async fn lister_pays_television(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let pays: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT p.nom
           FROM media_content.chaine_tv ct
           JOIN shared.pays p ON p.id = ct.pays_id
          WHERE ct.etat = 'publie' AND ct.deleted_at IS NULL AND ct.pays_id IS NOT NULL
          ORDER BY p.nom ASC",
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

// ── GET /api/television/stats ─────────────────────────────────────────

pub async fn obtenir_stats_television(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let nombre_chaines: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.chaine_tv WHERE etat = 'publie' AND deleted_at IS NULL",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let nombre_pays: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT pays_id) FROM media_content.chaine_tv
          WHERE etat = 'publie' AND deleted_at IS NULL AND pays_id IS NOT NULL",
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Le décompte annoncé porte sur les PROGRAMMES : c'est l'unité que le
    // public parcourt désormais, l'épisode n'étant qu'un élément de série.
    let nombre_programmes: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.emission_tele
          WHERE etat = 'publie' AND deleted_at IS NULL",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let nombre_chaines_en_direct: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.chaine_tv
          WHERE etat = 'publie' AND deleted_at IS NULL AND est_en_direct = true",
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(TelevisionStats {
            nombre_chaines,
            nombre_pays,
            nombre_programmes,
            nombre_chaines_en_direct,
        }),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Greffes de fiche support (thématiques et couverture)
// ═══════════════════════════════════════════════════════════════════════════

/// Deux requêtes pour toute une liste — jamais une par chaîne.
async fn greffer_fiche_support(
    pool: &PgPool,
    chaines: &mut [ChaineTvResponse],
) -> Result<(), ApiErreur> {
    let ids: Vec<Uuid> = chaines.iter().map(|c| c.id).collect();
    if ids.is_empty() {
        return Ok(());
    }
    let thematiques = thematiques_par_supports(pool, TYPE_SUPPORT, &ids).await?;
    let couvertures = couverture_par_supports(pool, TYPE_SUPPORT, &ids).await?;

    for chaine in chaines.iter_mut() {
        chaine.thematiques = thematiques.get(&chaine.id).cloned().unwrap_or_default();
        chaine.couverture = couvertures.get(&chaine.id).cloned();
    }
    Ok(())
}

async fn greffer_fiche_support_sections(
    pool: &PgPool,
    sections: &mut [TeleSectionResponse],
) -> Result<(), ApiErreur> {
    let ids: Vec<Uuid> = sections.iter().map(|s| s.chaine.id).collect();
    if ids.is_empty() {
        return Ok(());
    }
    let thematiques = thematiques_par_supports(pool, TYPE_SUPPORT, &ids).await?;
    let couvertures = couverture_par_supports(pool, TYPE_SUPPORT, &ids).await?;

    for section in sections.iter_mut() {
        section.chaine.thematiques = thematiques
            .get(&section.chaine.id)
            .cloned()
            .unwrap_or_default();
        section.chaine.couverture = couvertures.get(&section.chaine.id).cloned();
    }
    Ok(())
}

/// Réexport : les épisodes se trient partout dans le même ordre.
pub const ORDRE_EPISODES_TELE: &str = ORDRE_EPISODES;

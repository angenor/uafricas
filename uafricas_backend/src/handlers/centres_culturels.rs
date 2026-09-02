use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::centre_culturel::{
    CentreCulturel, CentreCulturelQueryParams, CentreCulturelResponse,
    InscriptionProgRequest, MembreCentre, ProgrammationCentre, ProgrammationDetailResponse,
    CENTRE_CULTUREL_COLONNES, PROGRAMMATION_COLONNES,
};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Extraire l'utilisateur connecte depuis le header Authorization (optionnel)
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Compter les inscrits actifs (statut != 'annule') a une programmation
async fn compter_inscrits(pool: &PgPool, programmation_id: Uuid) -> Result<i64, ApiErreur> {
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM culture.programmation_inscription
         WHERE programmation_id = $1 AND statut != 'annule'",
    )
    .bind(programmation_id)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Verifier si un utilisateur est inscrit (statut != 'annule') a une programmation
async fn est_inscrit(pool: &PgPool, programmation_id: Uuid, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM culture.programmation_inscription
            WHERE programmation_id = $1 AND utilisateur_id = $2 AND statut != 'annule'
        )",
    )
    .bind(programmation_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

// ──────────────────────────────────────────────────────────────
// GET /api/centres-culturels : Lister les centres culturels actifs
// ──────────────────────────────────────────────────────────────
pub async fn lister_centres(
    pool: web::Data<PgPool>,
    params: web::Query<CentreCulturelQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let centres: Vec<CentreCulturel> = if let Some(ref recherche) = params.recherche {
        let terme = format!("%{}%", recherche.to_lowercase());

        let query = format!(
            "SELECT {} FROM culture.centre_culturel
             WHERE actif = TRUE
               AND (LOWER(nom) LIKE $1 OR LOWER(ville) LIKE $1 OR LOWER(adresse) LIKE $1)
             ORDER BY nom ASC",
            CENTRE_CULTUREL_COLONNES
        );

        sqlx::query_as::<_, CentreCulturel>(&query)
            .bind(&terme)
            .fetch_all(pool.get_ref())
            .await?
    } else {
        let query = format!(
            "SELECT {} FROM culture.centre_culturel
             WHERE actif = TRUE
             ORDER BY nom ASC",
            CENTRE_CULTUREL_COLONNES
        );

        sqlx::query_as::<_, CentreCulturel>(&query)
            .fetch_all(pool.get_ref())
            .await?
    };

    // Recuperer le nombre de programmations pour chaque centre
    let mut reponse_centres: Vec<CentreCulturelResponse> = Vec::with_capacity(centres.len());

    for centre in &centres {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM culture.programmation_centre WHERE centre_culturel_id = $1",
        )
        .bind(centre.id)
        .fetch_one(pool.get_ref())
        .await?;

        reponse_centres.push(CentreCulturelResponse {
            id: centre.id,
            nom: centre.nom.clone(),
            slug: centre.slug.clone(),
            type_centre: centre.type_centre.clone(),
            description: centre.description.clone(),
            image_couverture_url: centre.image_couverture_url.clone(),
            ville: centre.ville.clone(),
            adresse: centre.adresse.clone(),
            latitude: centre.latitude,
            longitude: centre.longitude,
            nombre_programmations: count,
            created_at: centre.created_at,
            updated_at: centre.updated_at,
        });
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse_centres),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/centres-culturels/{id} : Obtenir un centre avec ses programmations
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_centre(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM culture.centre_culturel WHERE id = $1 AND actif = TRUE",
        CENTRE_CULTUREL_COLONNES
    );

    let centre = sqlx::query_as::<_, CentreCulturel>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Centre culturel avec id {} non trouve", id)))?;

    // Recuperer les membres du centre
    let membres = sqlx::query_as::<_, MembreCentre>(
        "SELECT mc.id, mc.role::text AS role, u.nom, u.prenom, u.email, u.telephone
         FROM culture.membre_centre mc
         JOIN iam.utilisateur u ON u.id = mc.utilisateur_id
         WHERE mc.centre_culturel_id = $1
         ORDER BY mc.role ASC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let membre_responses: Vec<_> = membres.iter().map(|m| m.to_response()).collect();

    // Recuperer les programmations du centre
    let query_prog = format!(
        "SELECT {} FROM culture.programmation_centre
         WHERE centre_culturel_id = $1
         ORDER BY date_heure_debut ASC",
        PROGRAMMATION_COLONNES
    );

    let programmations = sqlx::query_as::<_, ProgrammationCentre>(&query_prog)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await?;

    let prog_responses: Vec<_> = programmations.iter().map(|p| p.to_response()).collect();
    let reponse = centre.to_detail_response(membre_responses, prog_responses);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/centres-culturels/{centre_id}/programmations/{id}
// Obtenir le detail d'une programmation
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_programmation(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (centre_id, programmation_id) = chemin.into_inner();

    // Recuperer le centre
    let query_centre = format!(
        "SELECT {} FROM culture.centre_culturel WHERE id = $1 AND actif = TRUE",
        CENTRE_CULTUREL_COLONNES
    );

    let centre = sqlx::query_as::<_, CentreCulturel>(&query_centre)
        .bind(centre_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Centre culturel avec id {} non trouve", centre_id)))?;

    // Recuperer la programmation
    let query_prog = format!(
        "SELECT {} FROM culture.programmation_centre
         WHERE id = $1 AND centre_culturel_id = $2",
        PROGRAMMATION_COLONNES
    );

    let programmation = sqlx::query_as::<_, ProgrammationCentre>(&query_prog)
        .bind(programmation_id)
        .bind(centre_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| {
            ApiErreur::NonTrouve(format!(
                "Programmation avec id {} non trouvee dans le centre {}",
                programmation_id, centre_id
            ))
        })?;

    let mut prog_response = programmation.to_response();
    prog_response.nombre_inscrits = compter_inscrits(pool.get_ref(), programmation_id).await?;
    if let Some(uid) = extraire_utilisateur_id(&req) {
        prog_response.est_inscrit = est_inscrit(pool.get_ref(), programmation_id, uid).await?;
    }

    let reponse = ProgrammationDetailResponse {
        programmation: prog_response,
        centre: centre.to_info_response(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/centres-culturels/{centre_id}/programmations/{id}/inscription
// S'inscrire a une programmation (utilisateur connecte)
// ──────────────────────────────────────────────────────────────
pub async fn inscrire_programmation(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
    body: web::Json<InscriptionProgRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let (centre_id, programmation_id) = chemin.into_inner();

    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    // Champs d'inscription (nom & prenom requis)
    let nettoyer = |v: &Option<String>| v.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let nom = nettoyer(&body.nom);
    let prenom = nettoyer(&body.prenom);
    let pays = nettoyer(&body.pays);
    let lieu_residence = nettoyer(&body.lieu_residence);
    let titre = nettoyer(&body.titre);
    if nom.is_none() || prenom.is_none() {
        return Err(ApiErreur::Validation("Le nom et le prénom sont requis".to_string()));
    }

    // Verifier que la programmation existe, appartient au centre et que le centre est actif
    let nombre_places: Option<Option<i32>> = sqlx::query_scalar(
        "SELECT pc.nombre_places
         FROM culture.programmation_centre pc
         JOIN culture.centre_culturel cc ON cc.id = pc.centre_culturel_id
         WHERE pc.id = $1 AND pc.centre_culturel_id = $2 AND cc.actif = TRUE",
    )
    .bind(programmation_id)
    .bind(centre_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let nombre_places = nombre_places
        .ok_or_else(|| ApiErreur::NonTrouve("Programmation non trouvee".to_string()))?;

    // Controle de capacite si un nombre de places est defini
    if let Some(places) = nombre_places {
        let deja_inscrit = est_inscrit(pool.get_ref(), programmation_id, utilisateur_id).await?;
        if !deja_inscrit {
            let inscrits = compter_inscrits(pool.get_ref(), programmation_id).await?;
            if inscrits >= places as i64 {
                return Err(ApiErreur::Conflit("Plus de places disponibles pour cette programmation".to_string()));
            }
        }
    }

    sqlx::query(
        "INSERT INTO culture.programmation_inscription
            (programmation_id, utilisateur_id, nom, prenom, pays, lieu_residence, titre, statut)
         VALUES ($1, $2, $3, $4, $5, $6, $7, 'inscrit')
         ON CONFLICT (programmation_id, utilisateur_id)
         DO UPDATE SET statut = 'inscrit',
                       nom = EXCLUDED.nom,
                       prenom = EXCLUDED.prenom,
                       pays = EXCLUDED.pays,
                       lieu_residence = EXCLUDED.lieu_residence,
                       titre = EXCLUDED.titre,
                       updated_at = NOW()",
    )
    .bind(programmation_id)
    .bind(utilisateur_id)
    .bind(&nom)
    .bind(&prenom)
    .bind(&pays)
    .bind(&lieu_residence)
    .bind(&titre)
    .execute(pool.get_ref())
    .await?;

    log::info!("Inscription programmation {} par utilisateur {}", programmation_id, utilisateur_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// DELETE /api/centres-culturels/{centre_id}/programmations/{id}/inscription
// Se desinscrire d'une programmation (utilisateur connecte)
// ──────────────────────────────────────────────────────────────
pub async fn desinscrire_programmation(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (_centre_id, programmation_id) = chemin.into_inner();

    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    sqlx::query(
        "UPDATE culture.programmation_inscription
         SET statut = 'annule', updated_at = NOW()
         WHERE programmation_id = $1 AND utilisateur_id = $2",
    )
    .bind(programmation_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Desinscription programmation {} par utilisateur {}", programmation_id, utilisateur_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

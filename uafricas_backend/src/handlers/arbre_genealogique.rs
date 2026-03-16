// ════════════════════════════════════════════════════════════════════════════
// Handlers — Arbre généalogique
// ════════════════════════════════════════════════════════════════════════════
//
// Endpoints :
//   GET    /api/arbre/personnes          — lister_personnes  (US3)
//   POST   /api/arbre/personnes          — creer_personne    (US1)
//   GET    /api/arbre/personnes/{id}     — obtenir_personne  (US1)
//   PUT    /api/arbre/personnes/{id}     — modifier_personne (US1)
//   DELETE /api/arbre/personnes/{id}     — supprimer_personne(US1)
//   POST   /api/arbre/personnes/{id}/photo — uploader_photo  (US1)
//   POST   /api/arbre/liens              — creer_lien        (US2)
//   DELETE /api/arbre/liens/{id}         — supprimer_lien    (US2)
// ════════════════════════════════════════════════════════════════════════════

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use serde_json::json;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use chrono::{DateTime, Utc};

use crate::errors::ApiErreur;
use crate::models::arbre_genealogique::{
    ArbreCompletResponse, CreerLienDto, CreerPersonneDto, LienArbreResponse, LienFamilial,
    LienFamilialResponse, LienResumeResponse, ModifierPersonneDto, Personne,
    PersonneDetailResponse, PersonneListeResponse, PersonneNoeud, PersonneNoeudResponse,
    PersonneQueryParams, PersonneResponse, PERSONNE_COLONNES,
};
use crate::services::audit;
use crate::ApiResponse;

// ─── Struct locale pour les requêtes lien + personne ──────────────────────

#[derive(sqlx::FromRow)]
struct LienPersonneRow {
    lien_id: Uuid,
    type_lien: String,
    id: Uuid,
    nom: String,
    prenoms: Option<String>,
    genre: Option<String>,
    naissance_annee: Option<i16>,
    naissance_mois: Option<i16>,
    naissance_jour: Option<i16>,
    naissance_lieu: Option<String>,
    deces_annee: Option<i16>,
    deces_mois: Option<i16>,
    deces_jour: Option<i16>,
    deces_lieu: Option<String>,
    photo_url: Option<String>,
    cree_par: Option<Uuid>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl LienPersonneRow {
    fn en_resume(self) -> LienResumeResponse {
        let personne = PersonneResponse::from_row(Personne {
            id: self.id,
            nom: self.nom,
            prenoms: self.prenoms,
            genre: self.genre,
            naissance_annee: self.naissance_annee,
            naissance_mois: self.naissance_mois,
            naissance_jour: self.naissance_jour,
            naissance_lieu: self.naissance_lieu,
            deces_annee: self.deces_annee,
            deces_mois: self.deces_mois,
            deces_jour: self.deces_jour,
            deces_lieu: self.deces_lieu,
            photo_url: self.photo_url,
            cree_par: self.cree_par,
            created_at: self.created_at,
            updated_at: self.updated_at,
        });
        LienResumeResponse {
            lien_id: self.lien_id,
            type_lien: self.type_lien,
            personne,
        }
    }
}

// ─── Helper JWT ───────────────────────────────────────────────────────────

/// Extrait l'identifiant utilisateur depuis le header Authorization
fn extraire_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiErreur::NonAutorise("Token manquant".into()))?;

    let token = crate::jwt::extraire_token_du_header(header)?;
    let jwt_config = req
        .app_data::<web::Data<crate::config::JwtConfig>>()
        .ok_or_else(|| ApiErreur::BaseDeDonnees("Configuration JWT manquante".into()))?;
    let claims = crate::jwt::valider_token(token, &jwt_config.secret)?;
    claims
        .sub
        .parse::<Uuid>()
        .map_err(|_| ApiErreur::NonAutorise("ID utilisateur invalide".into()))
}

// ─── Helper : résoudre ou créer l'arbre de l'utilisateur ─────────────────

async fn obtenir_ou_creer_arbre(pool: &PgPool, utilisateur_id: Uuid) -> Result<Uuid, ApiErreur> {
    let arbre_id: Uuid = sqlx::query_scalar(
        "INSERT INTO arbre_genealogique.arbres (utilisateur_id)
         VALUES ($1)
         ON CONFLICT (utilisateur_id) DO UPDATE SET utilisateur_id = EXCLUDED.utilisateur_id
         RETURNING id",
    )
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(arbre_id)
}

// ─── Helper : vérifier appartenance (rattachement actif dans l'arbre) ─────

async fn verifier_appartenance(
    pool: &PgPool,
    personne_id: Uuid,
    arbre_id: Uuid,
) -> Result<Uuid, ApiErreur> {
    let rattachement_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM arbre_genealogique.rattachements
         WHERE personne_id = $1 AND arbre_id = $2 AND deleted_at IS NULL",
    )
    .bind(personne_id)
    .bind(arbre_id)
    .fetch_optional(pool)
    .await?;

    rattachement_id.ok_or_else(|| {
        ApiErreur::AccesInterdit("Cette personne ne fait pas partie de votre arbre".into())
    })
}

// ─── Helper : charger une personne avec son rattachement_id ───────────────

async fn charger_detail(
    pool: &PgPool,
    rattachement_id: Uuid,
    _arbre_id: Uuid,
) -> Result<PersonneDetailResponse, ApiErreur> {
    // Récupérer la personne via le rattachement
    let personne_row = sqlx::query_as::<_, Personne>(&format!(
        "SELECT {} FROM arbre_genealogique.personnes p
         JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
         WHERE r.id = $1 AND r.deleted_at IS NULL AND p.deleted_at IS NULL",
        PERSONNE_COLONNES
    ))
    .bind(rattachement_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Personne introuvable".into()))?;

    // Parents : liens où ce rattachement est la cible (type parent)
    let parents: Vec<LienResumeResponse> = sqlx::query_as::<_, LienPersonneRow>(
        "SELECT lf.id as lien_id, lf.type_lien,
                p.id, p.nom, p.prenoms, p.genre,
                p.naissance_annee, p.naissance_mois, p.naissance_jour, p.naissance_lieu,
                p.deces_annee, p.deces_mois, p.deces_jour, p.deces_lieu,
                p.photo_url, p.cree_par, p.created_at, p.updated_at
         FROM arbre_genealogique.liens_familiaux lf
         JOIN arbre_genealogique.rattachements r ON r.id = lf.rattachement_source_id
         JOIN arbre_genealogique.personnes p ON p.id = r.personne_id
         WHERE lf.rattachement_cible_id = $1
           AND lf.type_lien IN ('pere', 'mere', 'parent')
           AND lf.deleted_at IS NULL AND r.deleted_at IS NULL AND p.deleted_at IS NULL",
    )
    .bind(rattachement_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(LienPersonneRow::en_resume)
    .collect();

    // Enfants : liens où ce rattachement est la source (type parent)
    let enfants: Vec<LienResumeResponse> = sqlx::query_as::<_, LienPersonneRow>(
        "SELECT lf.id as lien_id, lf.type_lien,
                p.id, p.nom, p.prenoms, p.genre,
                p.naissance_annee, p.naissance_mois, p.naissance_jour, p.naissance_lieu,
                p.deces_annee, p.deces_mois, p.deces_jour, p.deces_lieu,
                p.photo_url, p.cree_par, p.created_at, p.updated_at
         FROM arbre_genealogique.liens_familiaux lf
         JOIN arbre_genealogique.rattachements r ON r.id = lf.rattachement_cible_id
         JOIN arbre_genealogique.personnes p ON p.id = r.personne_id
         WHERE lf.rattachement_source_id = $1
           AND lf.type_lien IN ('pere', 'mere', 'parent')
           AND lf.deleted_at IS NULL AND r.deleted_at IS NULL AND p.deleted_at IS NULL",
    )
    .bind(rattachement_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(LienPersonneRow::en_resume)
    .collect();

    // Conjoints : liens bidirectionnels de type conjoint
    let conjoints: Vec<LienResumeResponse> = sqlx::query_as::<_, LienPersonneRow>(
        "SELECT lf.id as lien_id, lf.type_lien,
                p.id, p.nom, p.prenoms, p.genre,
                p.naissance_annee, p.naissance_mois, p.naissance_jour, p.naissance_lieu,
                p.deces_annee, p.deces_mois, p.deces_jour, p.deces_lieu,
                p.photo_url, p.cree_par, p.created_at, p.updated_at
         FROM arbre_genealogique.liens_familiaux lf
         JOIN arbre_genealogique.rattachements r ON
               r.id = CASE WHEN lf.rattachement_source_id = $1
                           THEN lf.rattachement_cible_id
                           ELSE lf.rattachement_source_id END
         JOIN arbre_genealogique.personnes p ON p.id = r.personne_id
         WHERE (lf.rattachement_source_id = $1 OR lf.rattachement_cible_id = $1)
           AND lf.type_lien = 'conjoint'
           AND lf.deleted_at IS NULL AND r.deleted_at IS NULL AND p.deleted_at IS NULL",
    )
    .bind(rattachement_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(LienPersonneRow::en_resume)
    .collect();

    Ok(PersonneDetailResponse {
        rattachement_id,
        personne: PersonneResponse::from_row(personne_row),
        parents,
        enfants,
        conjoints,
    })
}

// ─── Validation des dates ─────────────────────────────────────────────────

fn valider_coherence_dates(
    naissance_annee: Option<i16>,
    deces_annee: Option<i16>,
) -> Result<(), ApiErreur> {
    if let (Some(n), Some(d)) = (naissance_annee, deces_annee) {
        if d < n {
            return Err(ApiErreur::Validation(
                "La date de décès ne peut pas être antérieure à la date de naissance".into(),
            ));
        }
    }
    Ok(())
}

// ════════════════════════════════════════════════════════════════════════════
// GET /api/arbre/arbre-complet — Arbre complet pour visualisation
// ════════════════════════════════════════════════════════════════════════════

pub async fn obtenir_arbre_complet(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    // Vérifier si l'utilisateur a un arbre
    let arbre_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM arbre_genealogique.arbres
         WHERE utilisateur_id = $1 AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let Some(arbre_id) = arbre_id else {
        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(ArbreCompletResponse {
                arbre_id: None,
                personnes: vec![],
                liens: vec![],
            }),
            error: None,
        }));
    };

    // Toutes les personnes rattachées à l'arbre
    let personnes_rows = sqlx::query_as::<_, PersonneNoeud>(
        "SELECT p.id, r.id AS rattachement_id,
                p.nom, p.prenoms, p.genre,
                p.naissance_annee, p.naissance_mois, p.naissance_jour, p.naissance_lieu,
                p.deces_annee, p.deces_mois, p.deces_jour, p.deces_lieu,
                p.photo_url
         FROM arbre_genealogique.personnes p
         JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
         WHERE r.arbre_id = $1 AND r.deleted_at IS NULL AND p.deleted_at IS NULL
         ORDER BY p.nom ASC, p.prenoms ASC",
    )
    .bind(arbre_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Tous les liens familiaux de l'arbre
    let liens = sqlx::query_as::<_, LienArbreResponse>(
        "SELECT id, rattachement_source_id, rattachement_cible_id, type_lien
         FROM arbre_genealogique.liens_familiaux
         WHERE arbre_id = $1 AND deleted_at IS NULL",
    )
    .bind(arbre_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ArbreCompletResponse {
            arbre_id: Some(arbre_id),
            personnes: personnes_rows
                .into_iter()
                .map(PersonneNoeudResponse::from_row)
                .collect(),
            liens,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// US3 — GET /api/arbre/personnes
// ════════════════════════════════════════════════════════════════════════════

pub async fn lister_personnes(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<PersonneQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_ou_creer_arbre(pool.get_ref(), utilisateur_id).await?;

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let (personnes, total): (Vec<Personne>, i64) = match &params.recherche {
        Some(recherche) if !recherche.trim().is_empty() => {
            let terme = format!("%{}%", recherche.to_lowercase());

            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM arbre_genealogique.personnes p
                 JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
                 WHERE r.arbre_id = $1 AND r.deleted_at IS NULL AND p.deleted_at IS NULL
                   AND (LOWER(p.nom) LIKE $2 OR LOWER(COALESCE(p.prenoms, '')) LIKE $2)",
            )
            .bind(arbre_id)
            .bind(&terme)
            .fetch_one(pool.get_ref())
            .await?;

            let personnes = sqlx::query_as::<_, Personne>(&format!(
                "SELECT {} FROM arbre_genealogique.personnes p
                 JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
                 WHERE r.arbre_id = $1 AND r.deleted_at IS NULL AND p.deleted_at IS NULL
                   AND (LOWER(p.nom) LIKE $2 OR LOWER(COALESCE(p.prenoms, '')) LIKE $2)
                 ORDER BY p.nom ASC, p.prenoms ASC
                 LIMIT $3 OFFSET $4",
                PERSONNE_COLONNES
            ))
            .bind(arbre_id)
            .bind(&terme)
            .bind(par_page)
            .bind(offset)
            .fetch_all(pool.get_ref())
            .await?;

            (personnes, total.0)
        }
        _ => {
            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM arbre_genealogique.personnes p
                 JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
                 WHERE r.arbre_id = $1 AND r.deleted_at IS NULL AND p.deleted_at IS NULL",
            )
            .bind(arbre_id)
            .fetch_one(pool.get_ref())
            .await?;

            let personnes = sqlx::query_as::<_, Personne>(&format!(
                "SELECT {} FROM arbre_genealogique.personnes p
                 JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
                 WHERE r.arbre_id = $1 AND r.deleted_at IS NULL AND p.deleted_at IS NULL
                 ORDER BY p.nom ASC, p.prenoms ASC
                 LIMIT $2 OFFSET $3",
                PERSONNE_COLONNES
            ))
            .bind(arbre_id)
            .bind(par_page)
            .bind(offset)
            .fetch_all(pool.get_ref())
            .await?;

            (personnes, total.0)
        }
    };

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PersonneListeResponse {
            personnes: personnes.into_iter().map(PersonneResponse::from_row).collect(),
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// US1 — POST /api/arbre/personnes
// ════════════════════════════════════════════════════════════════════════════

pub async fn creer_personne(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerPersonneDto>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    // Validation nom
    if body.nom.trim().is_empty() {
        return Err(ApiErreur::Validation("Le nom est obligatoire".into()));
    }

    // Validation cohérence dates
    let naissance_annee = body.naissance.as_ref().and_then(|d| d.annee);
    let deces_annee = body.deces.as_ref().and_then(|d| d.annee);
    valider_coherence_dates(naissance_annee, deces_annee)?;

    // Créer ou récupérer l'arbre de l'utilisateur
    let arbre_id = obtenir_ou_creer_arbre(pool.get_ref(), utilisateur_id).await?;

    // Insérer la personne réelle
    let naissance = body.naissance.as_ref();
    let deces = body.deces.as_ref();

    let personne_id: Uuid = sqlx::query_scalar(
        "INSERT INTO arbre_genealogique.personnes
             (nom, prenoms, genre,
              naissance_annee, naissance_mois, naissance_jour, naissance_lieu,
              deces_annee, deces_mois, deces_jour, deces_lieu,
              cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
         RETURNING id",
    )
    .bind(body.nom.trim())
    .bind(&body.prenoms)
    .bind(&body.genre)
    .bind(naissance.and_then(|d| d.annee))
    .bind(naissance.and_then(|d| d.mois))
    .bind(naissance.and_then(|d| d.jour))
    .bind(&body.naissance_lieu)
    .bind(deces.and_then(|d| d.annee))
    .bind(deces.and_then(|d| d.mois))
    .bind(deces.and_then(|d| d.jour))
    .bind(&body.deces_lieu)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Créer le rattachement
    let rattachement_id: Uuid = sqlx::query_scalar(
        "INSERT INTO arbre_genealogique.rattachements (arbre_id, personne_id)
         VALUES ($1, $2)
         RETURNING id",
    )
    .bind(arbre_id)
    .bind(personne_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Normalisation des noms pour le matching
    let nom_normalise = crate::services::matching::normaliser_nom(body.nom.trim());
    let prenoms_normalise = crate::services::matching::normaliser_nom(
        body.prenoms.as_deref().unwrap_or(""),
    );

    sqlx::query(
        "UPDATE arbre_genealogique.personnes
         SET nom_normalise = $1, prenoms_normalise = $2
         WHERE id = $3",
    )
    .bind(&nom_normalise)
    .bind(&prenoms_normalise)
    .bind(personne_id)
    .execute(pool.get_ref())
    .await?;

    // Matching profond en tâche de fond (fire-and-forget)
    let pool_clone = pool.get_ref().clone();
    let ratt_id = rattachement_id;
    let arb_id = arbre_id;
    tokio::spawn(async move {
        if let Err(e) = crate::services::matching::matching_profond(&pool_clone, ratt_id, arb_id).await {
            log::error!("Matching profond échoué pour rattachement {}: {}", ratt_id, e);
        }
    });

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "creer_personne",
        "arbre_genealogique",
        "personnes",
        Some(personne_id),
        None,
        Some(json!({"nom": body.nom.trim(), "arbre_id": arbre_id})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    let detail = charger_detail(pool.get_ref(), rattachement_id, arbre_id).await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(detail),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// US1 — GET /api/arbre/personnes/{id}
// ════════════════════════════════════════════════════════════════════════════

pub async fn obtenir_personne(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let personne_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_ou_creer_arbre(pool.get_ref(), utilisateur_id).await?;

    let rattachement_id = verifier_appartenance(pool.get_ref(), personne_id, arbre_id).await?;
    let detail = charger_detail(pool.get_ref(), rattachement_id, arbre_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(detail),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// US1 — PUT /api/arbre/personnes/{id}
// ════════════════════════════════════════════════════════════════════════════

pub async fn modifier_personne(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierPersonneDto>,
) -> Result<HttpResponse, ApiErreur> {
    let personne_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_ou_creer_arbre(pool.get_ref(), utilisateur_id).await?;
    let rattachement_id = verifier_appartenance(pool.get_ref(), personne_id, arbre_id).await?;

    // Validation nom si fourni
    if let Some(ref nom) = body.nom {
        if nom.trim().is_empty() {
            return Err(ApiErreur::Validation("Le nom ne peut pas être vide".into()));
        }
    }

    // Charger les valeurs actuelles pour la validation et l'audit
    let actuel = sqlx::query_as::<_, Personne>(&format!(
        "SELECT {} FROM arbre_genealogique.personnes p WHERE p.id = $1 AND p.deleted_at IS NULL",
        PERSONNE_COLONNES
    ))
    .bind(personne_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Personne introuvable".into()))?;

    // Valider cohérence des dates fusionnées
    let naissance_annee = body
        .naissance
        .as_ref()
        .and_then(|d| d.annee)
        .or(actuel.naissance_annee);
    let deces_annee = body
        .deces
        .as_ref()
        .and_then(|d| d.annee)
        .or(actuel.deces_annee);
    valider_coherence_dates(naissance_annee, deces_annee)?;

    let avant = json!({
        "nom": &actuel.nom,
        "prenoms": &actuel.prenoms,
        "naissance_annee": actuel.naissance_annee,
        "deces_annee": actuel.deces_annee,
    });

    // Mise à jour avec COALESCE pour ne modifier que les champs fournis
    let naissance = body.naissance.as_ref();
    let deces = body.deces.as_ref();

    sqlx::query(
        "UPDATE arbre_genealogique.personnes SET
             nom             = COALESCE($1, nom),
             prenoms         = CASE WHEN $2::VARCHAR IS NOT NULL THEN $2 ELSE prenoms END,
             genre           = CASE WHEN $3::VARCHAR IS NOT NULL THEN $3 ELSE genre END,
             naissance_annee = CASE WHEN $4::SMALLINT IS NOT NULL THEN $4 ELSE naissance_annee END,
             naissance_mois  = CASE WHEN $5::SMALLINT IS NOT NULL THEN $5 ELSE naissance_mois END,
             naissance_jour  = CASE WHEN $6::SMALLINT IS NOT NULL THEN $6 ELSE naissance_jour END,
             naissance_lieu  = CASE WHEN $7::VARCHAR IS NOT NULL THEN $7 ELSE naissance_lieu END,
             deces_annee     = CASE WHEN $8::SMALLINT IS NOT NULL THEN $8 ELSE deces_annee END,
             deces_mois      = CASE WHEN $9::SMALLINT IS NOT NULL THEN $9 ELSE deces_mois END,
             deces_jour      = CASE WHEN $10::SMALLINT IS NOT NULL THEN $10 ELSE deces_jour END,
             deces_lieu      = CASE WHEN $11::VARCHAR IS NOT NULL THEN $11 ELSE deces_lieu END,
             updated_at      = NOW()
         WHERE id = $12 AND deleted_at IS NULL",
    )
    .bind(body.nom.as_deref().map(|s| s.trim()))
    .bind(&body.prenoms)
    .bind(&body.genre)
    .bind(naissance.and_then(|d| d.annee))
    .bind(naissance.and_then(|d| d.mois))
    .bind(naissance.and_then(|d| d.jour))
    .bind(&body.naissance_lieu)
    .bind(deces.and_then(|d| d.annee))
    .bind(deces.and_then(|d| d.mois))
    .bind(deces.and_then(|d| d.jour))
    .bind(&body.deces_lieu)
    .bind(personne_id)
    .execute(pool.get_ref())
    .await?;

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "modifier_personne",
        "arbre_genealogique",
        "personnes",
        Some(personne_id),
        Some(avant),
        Some(json!({"nom": &body.nom, "modifie_le": chrono::Utc::now()})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    let detail = charger_detail(pool.get_ref(), rattachement_id, arbre_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(detail),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// US1 — DELETE /api/arbre/personnes/{id}
// ════════════════════════════════════════════════════════════════════════════

pub async fn supprimer_personne(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let personne_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_ou_creer_arbre(pool.get_ref(), utilisateur_id).await?;
    let rattachement_id = verifier_appartenance(pool.get_ref(), personne_id, arbre_id).await?;

    // Transaction atomique : soft-delete rattachement + liens + personne si orpheline
    let mut tx = pool.begin().await?;

    // 1. Soft-delete des liens familiaux impliquant ce rattachement
    sqlx::query(
        "UPDATE arbre_genealogique.liens_familiaux
         SET deleted_at = NOW()
         WHERE (rattachement_source_id = $1 OR rattachement_cible_id = $1)
           AND deleted_at IS NULL",
    )
    .bind(rattachement_id)
    .execute(&mut *tx)
    .await?;

    // 2. Soft-delete du rattachement
    sqlx::query(
        "UPDATE arbre_genealogique.rattachements
         SET deleted_at = NOW()
         WHERE id = $1",
    )
    .bind(rattachement_id)
    .execute(&mut *tx)
    .await?;

    // 3. Vérifier si la personne est encore rattachée à d'autres arbres
    let nb_rattachements_restants: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM arbre_genealogique.rattachements
         WHERE personne_id = $1 AND deleted_at IS NULL",
    )
    .bind(personne_id)
    .fetch_one(&mut *tx)
    .await?;

    // 4. Si aucun rattachement actif, soft-delete de la personne réelle
    if nb_rattachements_restants.0 == 0 {
        sqlx::query(
            "UPDATE arbre_genealogique.personnes
             SET deleted_at = NOW()
             WHERE id = $1",
        )
        .bind(personne_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "supprimer_personne",
        "arbre_genealogique",
        "personnes",
        Some(personne_id),
        Some(json!({"rattachement_id": rattachement_id, "arbre_id": arbre_id})),
        None,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({"message": "Personne supprimée de l'arbre"})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// US1 — POST /api/arbre/personnes/{id}/photo
// ════════════════════════════════════════════════════════════════════════════

pub async fn uploader_photo(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    chemin: web::Path<Uuid>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let personne_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_ou_creer_arbre(pool.get_ref(), utilisateur_id).await?;

    // Vérifier appartenance
    let _ = verifier_appartenance(pool.get_ref(), personne_id, arbre_id).await?;

    let mut photo_url: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| ApiErreur::Upload(format!("Erreur lecture: {}", e)))?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name())
            .unwrap_or("");

        if nom_champ == "photo" {
            // Valider le type MIME
            let content_type = field.content_type().map(|ct| ct.to_string()).unwrap_or_default();
            if !["image/jpeg", "image/png", "image/webp"].contains(&content_type.as_str()) {
                return Err(ApiErreur::Validation(
                    "Format d'image invalide. Formats acceptés : JPEG, PNG, WebP".into(),
                ));
            }

            let nom_original = content_disposition
                .as_ref()
                .and_then(|cd| cd.get_filename())
                .map(sanitize_filename::sanitize)
                .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));

            let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
            let dossier = format!("{}/personnes", upload_dir.get_ref());
            let chemin_complet = format!("{}/{}", dossier, nom_fichier);

            // Vérifier taille (max 5 Mo)
            let mut contenu = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture: {}", e)))?;
                contenu.extend_from_slice(&data);
                if contenu.len() > 5 * 1024 * 1024 {
                    return Err(ApiErreur::Validation("La photo ne doit pas dépasser 5 Mo".into()));
                }
            }

            std::fs::create_dir_all(&dossier)
                .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le dossier: {}", e)))?;

            let mut fichier = std::fs::File::create(&chemin_complet)
                .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le fichier: {}", e)))?;
            fichier
                .write_all(&contenu)
                .map_err(|e| ApiErreur::Upload(format!("Erreur écriture: {}", e)))?;

            photo_url = Some(format!("/uploads/personnes/{}", nom_fichier));
        }
    }

    let url = photo_url
        .ok_or_else(|| ApiErreur::Validation("Aucune photo fournie".into()))?;

    sqlx::query(
        "UPDATE arbre_genealogique.personnes
         SET photo_url = $1, updated_at = NOW()
         WHERE id = $2 AND deleted_at IS NULL",
    )
    .bind(&url)
    .bind(personne_id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "uploader_photo_personne",
        "arbre_genealogique",
        "personnes",
        Some(personne_id),
        None,
        Some(json!({"photo_url": &url})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({"photo_url": url})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// US2 — POST /api/arbre/liens  (stub — implémenté en Phase 4 / T018)
// ════════════════════════════════════════════════════════════════════════════

pub async fn creer_lien(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerLienDto>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_ou_creer_arbre(pool.get_ref(), utilisateur_id).await?;

    // Validation : source ≠ cible
    if body.rattachement_source_id == body.rattachement_cible_id {
        return Err(ApiErreur::Validation(
            "Une personne ne peut pas être reliée à elle-même".into(),
        ));
    }

    // Validation : type de lien
    let types_valides = ["pere", "mere", "parent", "conjoint"];
    if !types_valides.contains(&body.type_lien.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Type de lien invalide. Valeurs acceptées : {}",
            types_valides.join(", ")
        )));
    }

    // Vérifier que les deux rattachements appartiennent à l'arbre de l'utilisateur
    let nb_dans_arbre: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM arbre_genealogique.rattachements
         WHERE id IN ($1, $2) AND arbre_id = $3 AND deleted_at IS NULL",
    )
    .bind(body.rattachement_source_id)
    .bind(body.rattachement_cible_id)
    .bind(arbre_id)
    .fetch_one(pool.get_ref())
    .await?;

    if nb_dans_arbre.0 < 2 {
        return Err(ApiErreur::AccesInterdit(
            "Les deux personnes doivent appartenir à votre arbre".into(),
        ));
    }

    // Vérifier absence de doublon
    let existe: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM arbre_genealogique.liens_familiaux
         WHERE arbre_id = $1
           AND rattachement_source_id = $2
           AND rattachement_cible_id = $3
           AND type_lien = $4
           AND deleted_at IS NULL",
    )
    .bind(arbre_id)
    .bind(body.rattachement_source_id)
    .bind(body.rattachement_cible_id)
    .bind(&body.type_lien)
    .fetch_optional(pool.get_ref())
    .await?;

    if existe.is_some() {
        return Err(ApiErreur::Conflit("Ce lien familial existe déjà".into()));
    }

    // Détection de cycle pour les liens parent-enfant
    if ["pere", "mere", "parent"].contains(&body.type_lien.as_str()) {
        let cycle_detecte: (i64,) = sqlx::query_as(
            "WITH RECURSIVE ancetres AS (
                 SELECT rattachement_source_id AS id
                 FROM arbre_genealogique.liens_familiaux
                 WHERE rattachement_cible_id = $1
                   AND type_lien IN ('pere', 'mere', 'parent')
                   AND deleted_at IS NULL
                 UNION ALL
                 SELECT lf.rattachement_source_id
                 FROM arbre_genealogique.liens_familiaux lf
                 INNER JOIN ancetres a ON lf.rattachement_cible_id = a.id
                 WHERE lf.type_lien IN ('pere', 'mere', 'parent')
                   AND lf.deleted_at IS NULL
             )
             SELECT COUNT(*) FROM ancetres WHERE id = $2",
        )
        .bind(body.rattachement_source_id) // le futur parent
        .bind(body.rattachement_cible_id)  // le futur enfant → ses ancêtres ne doivent pas contenir le parent
        .fetch_one(pool.get_ref())
        .await?;

        if cycle_detecte.0 > 0 {
            return Err(ApiErreur::Validation(
                "Lien circulaire détecté : cette relation créerait un cycle dans la hiérarchie familiale".into(),
            ));
        }
    }

    // Pour les conjoints : appliquer la convention min/max UUID pour l'unicité
    let (source_id, cible_id) = if body.type_lien == "conjoint" {
        let (s, c) = (body.rattachement_source_id, body.rattachement_cible_id);
        if s.to_string() < c.to_string() {
            (s, c)
        } else {
            (c, s)
        }
    } else {
        (body.rattachement_source_id, body.rattachement_cible_id)
    };

    // Insérer le lien
    let lien = sqlx::query_as::<_, LienFamilial>(
        "INSERT INTO arbre_genealogique.liens_familiaux
             (arbre_id, rattachement_source_id, rattachement_cible_id, type_lien)
         VALUES ($1, $2, $3, $4)
         RETURNING id, arbre_id, rattachement_source_id, rattachement_cible_id, type_lien, created_at, deleted_at",
    )
    .bind(arbre_id)
    .bind(source_id)
    .bind(cible_id)
    .bind(&body.type_lien)
    .fetch_one(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "creer_lien_familial",
        "arbre_genealogique",
        "liens_familiaux",
        Some(lien.id),
        None,
        Some(json!({"type_lien": &body.type_lien, "source": source_id, "cible": cible_id})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(LienFamilialResponse::from(lien)),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// US2 — DELETE /api/arbre/liens/{id}  (stub — implémenté en Phase 4 / T019)
// ════════════════════════════════════════════════════════════════════════════

pub async fn supprimer_lien(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let lien_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_ou_creer_arbre(pool.get_ref(), utilisateur_id).await?;

    // Vérifier que le lien appartient à l'arbre de l'utilisateur
    let lien: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM arbre_genealogique.liens_familiaux
         WHERE id = $1 AND arbre_id = $2 AND deleted_at IS NULL",
    )
    .bind(lien_id)
    .bind(arbre_id)
    .fetch_optional(pool.get_ref())
    .await?;

    if lien.is_none() {
        return Err(ApiErreur::NonTrouve("Lien familial introuvable".into()));
    }

    // Soft-delete
    sqlx::query(
        "UPDATE arbre_genealogique.liens_familiaux
         SET deleted_at = NOW()
         WHERE id = $1",
    )
    .bind(lien_id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "supprimer_lien_familial",
        "arbre_genealogique",
        "liens_familiaux",
        Some(lien_id),
        Some(json!({"lien_id": lien_id, "arbre_id": arbre_id})),
        None,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({"message": "Lien familial supprimé"})),
        error: None,
    }))
}

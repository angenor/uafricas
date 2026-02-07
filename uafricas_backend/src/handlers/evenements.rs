use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::evenement::{
    EvenementDetailResponse, EvenementListeResponse, EvenementQueryParams,
    EvenementResponse, EvenementRow, OrganisateurInfo, OrganisateurResponse,
    EVENEMENT_COLONNES, calculer_statut, generer_slug, mapper_format_db,
    mapper_format_frontend,
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

/// Charger l'organisateur depuis iam.utilisateur
async fn charger_organisateur(pool: &PgPool, utilisateur_id: Uuid) -> Result<OrganisateurInfo, ApiErreur> {
    sqlx::query_as::<_, OrganisateurInfo>(
        "SELECT id, nom, prenom, email, photo_url FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Organisateur non trouve".to_string()))
}

/// Compter les inscrits a un evenement
async fn compter_inscrits(pool: &PgPool, evenement_id: Uuid) -> Result<i64, ApiErreur> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.evenement_inscription
         WHERE evenement_id = $1 AND statut != 'annule'",
    )
    .bind(evenement_id)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Verifier si un utilisateur est inscrit a un evenement
async fn est_inscrit(pool: &PgPool, evenement_id: Uuid, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.evenement_inscription
            WHERE evenement_id = $1 AND utilisateur_id = $2 AND statut != 'annule'
        )",
    )
    .bind(evenement_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

/// Recuperer le nom du pays a partir de son ID
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

/// Construire un EvenementResponse a partir d'un EvenementRow
async fn construire_response(
    pool: &PgPool,
    row: &EvenementRow,
) -> Result<EvenementResponse, ApiErreur> {
    let organisateur = charger_organisateur(pool, row.cree_par).await?;
    let nombre_inscrits = compter_inscrits(pool, row.id).await?;
    let pays_nom = charger_nom_pays(pool, row.pays_id).await?;

    Ok(EvenementResponse {
        id: row.id,
        titre: row.titre.clone(),
        description: row.description.clone(),
        type_format: mapper_format_frontend(&row.format),
        pays: pays_nom,
        ville: row.ville.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        statut: calculer_statut(&row.etat, &row.date_heure_debut, row.date_heure_fin.as_ref()),
        nombre_places: row.nombre_places,
        nombre_inscrits,
        user: OrganisateurResponse {
            uid: organisateur.id,
            nom: organisateur.nom,
            prenom: organisateur.prenom,
            email: organisateur.email,
            photo_url: organisateur.photo_url,
        },
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

// ──────────────────────────────────────────────────────────────
// GET /api/evenements — Lister les evenements avec filtres et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_evenements(
    pool: web::Data<PgPool>,
    params: web::Query<EvenementQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Construire les conditions WHERE dynamiquement
    let mut conditions: Vec<String> = vec![
        "e.etat IN ('publie','termine','annule')".to_string(),
        "e.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref format) = params.format {
        let format_db = mapper_format_db(format);
        conditions.push(format!("e.format::text = ${}", bind_index));
        bind_values.push(format_db);
        bind_index += 1;
    }

    if let Some(ref pays) = params.pays {
        conditions.push(format!(
            "EXISTS (SELECT 1 FROM shared.pays p WHERE p.id = e.pays_id AND LOWER(p.nom) = LOWER(${})) ",
            bind_index
        ));
        bind_values.push(pays.clone());
        bind_index += 1;
    }

    if let Some(annee) = params.annee {
        conditions.push(format!("EXTRACT(YEAR FROM e.date_heure_debut) = ${}::numeric", bind_index));
        bind_values.push(annee.to_string());
        bind_index += 1;
    }

    if let Some(ref recherche) = params.recherche {
        let terme = format!("%{}%", recherche.to_lowercase());
        conditions.push(format!(
            "(LOWER(e.titre) LIKE ${idx} OR LOWER(e.description) LIKE ${idx})",
            idx = bind_index
        ));
        bind_values.push(terme);
        bind_index += 1;
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM media_content.evenement e WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les evenements
    let select_query = format!(
        "SELECT {} FROM media_content.evenement e WHERE {} ORDER BY e.date_heure_debut DESC LIMIT ${} OFFSET ${}",
        EVENEMENT_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, EvenementRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    // Construire les reponses
    let mut evenements = Vec::with_capacity(rows.len());
    for row in &rows {
        evenements.push(construire_response(pool.get_ref(), row).await?);
    }

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(EvenementListeResponse {
            evenements,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/evenements/{id} — Obtenir le detail d'un evenement
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_evenement(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let current_user = extraire_utilisateur_id(&req);
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM media_content.evenement e
         WHERE e.id = $1 AND e.deleted_at IS NULL",
        EVENEMENT_COLONNES
    );

    let row = sqlx::query_as::<_, EvenementRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Evenement avec id {} non trouve", id)))?;

    let organisateur = charger_organisateur(pool.get_ref(), row.cree_par).await?;
    let nombre_inscrits = compter_inscrits(pool.get_ref(), row.id).await?;
    let pays_nom = charger_nom_pays(pool.get_ref(), row.pays_id).await?;

    let inscrit = if let Some(uid) = current_user {
        est_inscrit(pool.get_ref(), id, uid).await?
    } else {
        false
    };

    let reponse = EvenementDetailResponse {
        id: row.id,
        titre: row.titre.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        type_format: mapper_format_frontend(&row.format),
        pays: pays_nom,
        ville: row.ville.clone(),
        adresse: row.adresse.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        lien_en_ligne: row.lien_en_ligne.clone(),
        statut: calculer_statut(&row.etat, &row.date_heure_debut, row.date_heure_fin.as_ref()),
        nombre_places: row.nombre_places,
        nombre_inscrits,
        est_inscrit: inscrit,
        user: OrganisateurResponse {
            uid: organisateur.id,
            nom: organisateur.nom,
            prenom: organisateur.prenom,
            email: organisateur.email,
            photo_url: organisateur.photo_url,
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
// POST /api/evenements — Creer un evenement (multipart/form-data)
// ──────────────────────────────────────────────────────────────
pub async fn creer_evenement(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut format_evt: Option<String> = None;
    let mut pays: Option<String> = None;
    let mut ville: Option<String> = None;
    let mut date_heure_debut: Option<String> = None;
    let mut date_heure_fin: Option<String> = None;
    let mut lien_en_ligne: Option<String> = None;
    let mut nombre_places: Option<i32> = None;
    let mut image_couverture_url: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            ApiErreur::Upload(format!("Erreur lecture multipart: {}", e))
        })?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        match nom_champ.as_str() {
            "titre" => titre = Some(lire_champ_texte(&mut field).await?),
            "description" => description = Some(lire_champ_texte(&mut field).await?),
            "format" | "type" => {
                let valeur = lire_champ_texte(&mut field).await?;
                format_evt = Some(mapper_format_db(&valeur));
            }
            "pays" => pays = Some(lire_champ_texte(&mut field).await?),
            "ville" => ville = Some(lire_champ_texte(&mut field).await?),
            "date_heure_debut" => date_heure_debut = Some(lire_champ_texte(&mut field).await?),
            "date_heure_fin" => date_heure_fin = Some(lire_champ_texte(&mut field).await?),
            "lien_en_ligne" => lien_en_ligne = Some(lire_champ_texte(&mut field).await?),
            "nombre_places" => {
                let val = lire_champ_texte(&mut field).await?;
                nombre_places = val.parse::<i32>().ok();
            }
            "couverture" | "image" => {
                let nom_original = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));

                let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
                let chemin_complet = format!("{}/couvertures/{}", upload_dir.get_ref(), nom_fichier);

                sauvegarder_fichier(&mut field, &chemin_complet).await?;
                image_couverture_url = Some(format!("/uploads/couvertures/{}", nom_fichier));
            }
            _ => {
                log::warn!("Champ multipart evenement inconnu ignore: {}", nom_champ);
            }
        }
    }

    // Validation des champs obligatoires
    let titre = titre.ok_or_else(|| ApiErreur::Validation("Le titre est obligatoire".into()))?;
    let description = description
        .ok_or_else(|| ApiErreur::Validation("La description est obligatoire".into()))?;
    let format_evt = format_evt
        .ok_or_else(|| ApiErreur::Validation("Le format est obligatoire".into()))?;
    let date_debut_str = date_heure_debut
        .ok_or_else(|| ApiErreur::Validation("La date de debut est obligatoire".into()))?;

    // Parser les dates
    let date_debut = chrono::DateTime::parse_from_rfc3339(&date_debut_str)
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(&date_debut_str, "%Y-%m-%dT%H:%M")
                .map(|dt| dt.and_utc().fixed_offset())
        })
        .map_err(|_| ApiErreur::Validation("Format de date de debut invalide".into()))?
        .with_timezone(&chrono::Utc);

    let date_fin = date_heure_fin
        .as_ref()
        .map(|s| {
            chrono::DateTime::parse_from_rfc3339(s)
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
                        .map(|dt| dt.and_utc().fixed_offset())
                })
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|_| ApiErreur::Validation("Format de date de fin invalide".into()))
        })
        .transpose()?;

    // Resoudre pays_id
    let pays_id: Option<Uuid> = if let Some(ref nom_pays) = pays {
        sqlx::query_scalar("SELECT id FROM shared.pays WHERE LOWER(nom) = LOWER($1)")
            .bind(nom_pays)
            .fetch_optional(pool.get_ref())
            .await?
    } else {
        None
    };

    // Utilisateur connecte via JWT, sinon premier en BDD
    let utilisateur_id = if let Some(uid) = extraire_utilisateur_id(&req) {
        uid
    } else {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM iam.utilisateur ORDER BY created_at ASC LIMIT 1",
        )
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::Validation("Aucun utilisateur trouve en base".to_string()))?
    };

    let slug = generer_slug(&titre);

    // Inserer l'evenement
    let query = format!(
        "INSERT INTO media_content.evenement
            (titre, slug, description, format, pays_id, ville, adresse,
             date_heure_debut, date_heure_fin, image_couverture_url,
             lien_en_ligne, nombre_places, etat, cree_par)
         VALUES ($1, $2, $3, $4::media_content.format_evenement, $5, $6, NULL,
                 $7, $8, $9, $10, $11, 'publie', $12)
         RETURNING {}",
        EVENEMENT_COLONNES.replace("e.", "")
    );

    let row = sqlx::query_as::<_, EvenementRow>(&query)
        .bind(&titre)
        .bind(&slug)
        .bind(&description)
        .bind(&format_evt)
        .bind(pays_id)
        .bind(&ville)
        .bind(date_debut)
        .bind(date_fin)
        .bind(&image_couverture_url)
        .bind(&lien_en_ligne)
        .bind(nombre_places)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?;

    log::info!("Evenement cree: {} ({})", row.titre, row.id);

    // Construire la reponse detail
    let organisateur = charger_organisateur(pool.get_ref(), row.cree_par).await?;
    let pays_nom = charger_nom_pays(pool.get_ref(), row.pays_id).await?;

    let reponse = EvenementDetailResponse {
        id: row.id,
        titre: row.titre.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        type_format: mapper_format_frontend(&row.format),
        pays: pays_nom,
        ville: row.ville.clone(),
        adresse: row.adresse.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        lien_en_ligne: row.lien_en_ligne.clone(),
        statut: calculer_statut(&row.etat, &row.date_heure_debut, row.date_heure_fin.as_ref()),
        nombre_places: row.nombre_places,
        nombre_inscrits: 0,
        est_inscrit: false,
        user: OrganisateurResponse {
            uid: organisateur.id,
            nom: organisateur.nom,
            prenom: organisateur.prenom,
            email: organisateur.email,
            photo_url: organisateur.photo_url,
        },
        created_at: row.created_at,
        updated_at: row.updated_at,
    };

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/evenements/{id}/inscription — S'inscrire a un evenement
// ──────────────────────────────────────────────────────────────
pub async fn inscrire_evenement(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let evenement_id = chemin.into_inner();

    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    // Verifier que l'evenement existe et est publie
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.evenement
            WHERE id = $1 AND etat = 'publie' AND deleted_at IS NULL
        )",
    )
    .bind(evenement_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Evenement non trouve".to_string()));
    }

    // Inserer l'inscription (ON CONFLICT pour eviter les doublons)
    sqlx::query(
        "INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
         VALUES ($1, $2, 'inscrit')
         ON CONFLICT (evenement_id, utilisateur_id) DO UPDATE SET statut = 'inscrit', updated_at = NOW()",
    )
    .bind(evenement_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Inscription evenement {} par utilisateur {}", evenement_id, utilisateur_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/// Lire le contenu texte d'un champ multipart
async fn lire_champ_texte(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut contenu = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture champ: {}", e)))?;
        contenu.extend_from_slice(&data);
    }
    String::from_utf8(contenu)
        .map_err(|e| ApiErreur::Upload(format!("Encodage UTF-8 invalide: {}", e)))
}

/// Sauvegarder un fichier uploade sur le disque local
async fn sauvegarder_fichier(field: &mut actix_multipart::Field, chemin: &str) -> Result<(), ApiErreur> {
    if let Some(parent) = std::path::Path::new(chemin).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le repertoire: {}", e)))?;
    }

    let mut fichier = std::fs::File::create(chemin)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le fichier: {}", e)))?;

    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
        fichier
            .write_all(&data)
            .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture fichier: {}", e)))?;
    }

    Ok(())
}

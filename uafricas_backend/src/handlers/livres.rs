use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::livre::{Livre, LivreListeResponse, LivreQueryParams, LivreResponse, LIVRE_COLONNES};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// GET /api/livres : Lister les livres avec recherche, filtre et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_livres(
    pool: web::Data<PgPool>,
    params: web::Query<LivreQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let (livres, total): (Vec<Livre>, i64) = if let Some(ref recherche) = params.recherche {
        let terme = format!("%{}%", recherche.to_lowercase());

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM media_content.livre
             WHERE deleted_at IS NULL
               AND etat = 'publie'
               AND (LOWER(titre) LIKE $1 OR LOWER(description) LIKE $1)
               AND ($2::VARCHAR IS NULL OR type_document = $2)",
        )
        .bind(&terme)
        .bind(&params.type_document)
        .fetch_one(pool.get_ref())
        .await?;

        let query = format!(
            "SELECT {} FROM media_content.livre
             WHERE deleted_at IS NULL
               AND etat = 'publie'
               AND (LOWER(titre) LIKE $1 OR LOWER(description) LIKE $1)
               AND ($2::VARCHAR IS NULL OR type_document = $2)
             ORDER BY created_at DESC
             LIMIT $3 OFFSET $4",
            LIVRE_COLONNES
        );

        let livres = sqlx::query_as::<_, Livre>(&query)
            .bind(&terme)
            .bind(&params.type_document)
            .bind(par_page)
            .bind(offset)
            .fetch_all(pool.get_ref())
            .await?;

        (livres, total.0)
    } else {
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM media_content.livre
             WHERE deleted_at IS NULL
               AND etat = 'publie'
               AND ($1::VARCHAR IS NULL OR type_document = $1)",
        )
        .bind(&params.type_document)
        .fetch_one(pool.get_ref())
        .await?;

        let query = format!(
            "SELECT {} FROM media_content.livre
             WHERE deleted_at IS NULL
               AND etat = 'publie'
               AND ($1::VARCHAR IS NULL OR type_document = $1)
             ORDER BY created_at DESC
             LIMIT $2 OFFSET $3",
            LIVRE_COLONNES
        );

        let livres = sqlx::query_as::<_, Livre>(&query)
            .bind(&params.type_document)
            .bind(par_page)
            .bind(offset)
            .fetch_all(pool.get_ref())
            .await?;

        (livres, total.0)
    };

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    let reponse = LivreListeResponse {
        livres: livres.into_iter().map(LivreResponse::from).collect(),
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

// ──────────────────────────────────────────────────────────────
// GET /api/livres/{id} : Obtenir un livre par son ID
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_livre(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM media_content.livre WHERE id = $1 AND deleted_at IS NULL",
        LIVRE_COLONNES
    );

    let livre = sqlx::query_as::<_, Livre>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Livre avec id {} non trouve", id)))?;

    // Incrementer le nombre de vues
    let _ = sqlx::query("UPDATE media_content.livre SET nombre_vues = nombre_vues + 1 WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(LivreResponse::from(livre)),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/livres : Creer un nouveau livre (multipart/form-data)
// ──────────────────────────────────────────────────────────────
pub async fn creer_livre(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut type_document: Option<String> = None;
    let mut acces: Option<String> = None;
    let mut info_auteur: Option<String> = None;
    let mut date_publication: Option<String> = None;
    let mut rapport_auteur: Option<String> = None;
    let mut acceptation_diffusion: bool = false;

    let mut image_couverture_url: Option<String> = None;
    let mut document_pdf_url: Option<String> = None;

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
            "type" | "type_document" => type_document = Some(lire_champ_texte(&mut field).await?),
            "acces" => {
                let valeur = lire_champ_texte(&mut field).await?;
                acces = Some(mapper_acces_vers_db(&valeur));
            }
            "auteurBiblio" | "info_auteur" => {
                info_auteur = Some(lire_champ_texte(&mut field).await?)
            }
            "datePublication" | "date_publication" => {
                date_publication = Some(lire_champ_texte(&mut field).await?)
            }
            "rapport" | "rapport_auteur" => {
                rapport_auteur = Some(lire_champ_texte(&mut field).await?)
            }
            "consent" | "acceptation_diffusion" => {
                let val = lire_champ_texte(&mut field).await?;
                acceptation_diffusion = val == "true" || val == "on" || val == "1";
            }
            "image" | "couverture" => {
                let nom_original = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));

                let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
                let chemin_complet = format!("{}/couvertures/{}", upload_dir.get_ref(), nom_fichier);

                sauvegarder_fichier(&mut field, &chemin_complet).await?;
                image_couverture_url = Some(format!("/uploads/couvertures/{}", nom_fichier));
            }
            "document" | "pdf" => {
                let nom_original = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                    .unwrap_or_else(|| format!("{}.pdf", Uuid::new_v4()));

                let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
                let chemin_complet = format!("{}/documents/{}", upload_dir.get_ref(), nom_fichier);

                sauvegarder_fichier(&mut field, &chemin_complet).await?;
                document_pdf_url = Some(format!("/uploads/documents/{}", nom_fichier));
            }
            _ => {
                log::warn!("Champ multipart inconnu ignore: {}", nom_champ);
            }
        }
    }

    // Validation des champs obligatoires
    let titre = titre.ok_or_else(|| ApiErreur::Validation("Le titre est obligatoire".into()))?;
    let description = description
        .ok_or_else(|| ApiErreur::Validation("La description est obligatoire".into()))?;
    let type_document = type_document
        .ok_or_else(|| ApiErreur::Validation("Le type de document est obligatoire".into()))?;
    let acces = acces
        .ok_or_else(|| ApiErreur::Validation("Le type d'acces est obligatoire".into()))?;
    let info_auteur = info_auteur
        .ok_or_else(|| ApiErreur::Validation("Les informations sur l'auteur sont obligatoires".into()))?;
    let document_pdf_url = document_pdf_url
        .ok_or_else(|| ApiErreur::Validation("Le document PDF est obligatoire".into()))?;

    let date_pub = date_publication
        .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());

    let slug = generer_slug(&titre);

    // TODO: Remplacer par l'ID de l'utilisateur authentifie
    // Pour l'instant, on recupere ou cree un utilisateur de test
    let cree_par = obtenir_ou_creer_utilisateur_test(pool.get_ref()).await?;

    let query = format!(
        "INSERT INTO media_content.livre
            (titre, slug, description, image_couverture_url, document_pdf_url,
             type_document, acces, info_auteur, date_publication,
             rapport_auteur, acceptation_diffusion, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7::media_content.acces_livre, $8, $9, $10, $11, 'publie', $12)
         RETURNING {}",
        LIVRE_COLONNES
    );

    let livre = sqlx::query_as::<_, Livre>(&query)
        .bind(&titre)
        .bind(&slug)
        .bind(&description)
        .bind(&image_couverture_url)
        .bind(&document_pdf_url)
        .bind(&type_document)
        .bind(&acces)
        .bind(&info_auteur)
        .bind(date_pub)
        .bind(&rapport_auteur)
        .bind(acceptation_diffusion)
        .bind(cree_par)
        .fetch_one(pool.get_ref())
        .await?;

    log::info!("Livre cree: {} ({})", livre.titre, livre.id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(LivreResponse::from(livre)),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// DELETE /api/livres/{id} : Suppression douce d'un livre
// ──────────────────────────────────────────────────────────────
pub async fn supprimer_livre(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let resultat = sqlx::query(
        "UPDATE media_content.livre
         SET deleted_at = NOW(), etat = 'supprime'
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if resultat.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(format!(
            "Livre avec id {} non trouve",
            id
        )));
    }

    log::info!("Livre supprime (soft): {}", id);

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

/// Mapper la valeur d'acces frontend vers la valeur de l'enum PostgreSQL
fn mapper_acces_vers_db(valeur_frontend: &str) -> String {
    match valeur_frontend {
        "Lecture" | "lecture_seule" => "lecture_seule".to_string(),
        "Téléchargeable" | "lecture_telechargement" => "lecture_telechargement".to_string(),
        autre => autre.to_string(),
    }
}

/// Generer un slug URL-safe a partir du titre
fn generer_slug(titre: &str) -> String {
    titre
        .to_lowercase()
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â', 'ä'], "a")
        .replace(['ù', 'û', 'ü'], "u")
        .replace(['î', 'ï'], "i")
        .replace(['ô', 'ö'], "o")
        .replace('ç', "c")
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

/// Obtenir ou creer un utilisateur de test pour le champ cree_par
/// Temporaire : a remplacer par l'authentification reelle
async fn obtenir_ou_creer_utilisateur_test(pool: &PgPool) -> Result<Uuid, ApiErreur> {
    // Chercher un utilisateur existant
    let existant: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM iam.utilisateur WHERE email = 'test@uafricas.dev' AND deleted_at IS NULL LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some((id,)) = existant {
        return Ok(id);
    }

    // Creer un utilisateur de test
    let nouvel_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat)
         VALUES ('Test', 'Utilisateur', 'test@uafricas.dev', 'hash_temporaire', 'actif')
         RETURNING id",
    )
    .fetch_one(pool)
    .await?;

    log::info!("Utilisateur de test cree: {}", nouvel_id.0);
    Ok(nouvel_id.0)
}

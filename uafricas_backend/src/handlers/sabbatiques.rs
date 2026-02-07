use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::sabbatique::{
    OrganisateurInfo, OrganisateurResponse, SabbatiqueDetailResponse,
    SabbatiqueListeResponse, SabbatiqueQueryParams, SabbatiqueResponse,
    SabbatiqueRow, PROGRAMME_COLONNES, calculer_statut, duree_label,
    generer_slug, prises_en_charge,
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
async fn charger_organisateur(
    pool: &PgPool,
    utilisateur_id: Uuid,
) -> Result<OrganisateurInfo, ApiErreur> {
    sqlx::query_as::<_, OrganisateurInfo>(
        "SELECT id, nom, prenom, email, photo_url FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Organisateur non trouve".to_string()))
}

/// Recuperer le nom du pays
async fn charger_nom_pays(
    pool: &PgPool,
    pays_id: Uuid,
) -> Result<Option<String>, ApiErreur> {
    Ok(
        sqlx::query_scalar("SELECT nom FROM shared.pays WHERE id = $1")
            .bind(pays_id)
            .fetch_optional(pool)
            .await?,
    )
}

/// Recuperer le nom du domaine
async fn charger_nom_domaine(
    pool: &PgPool,
    domaine_id: Option<Uuid>,
) -> Result<Option<String>, ApiErreur> {
    if let Some(did) = domaine_id {
        Ok(
            sqlx::query_scalar("SELECT nom FROM shared.domaine_secteur WHERE id = $1")
                .bind(did)
                .fetch_optional(pool)
                .await?,
        )
    } else {
        Ok(None)
    }
}

/// Compter les candidatures d'un programme
async fn compter_candidatures(
    pool: &PgPool,
    programme_id: Uuid,
) -> Result<i64, ApiErreur> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM exchange.candidature
         WHERE programme_id = $1 AND statut != 'retiree'",
    )
    .bind(programme_id)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Construire un SabbatiqueResponse a partir d'un SabbatiqueRow
async fn construire_response(
    pool: &PgPool,
    row: &SabbatiqueRow,
) -> Result<SabbatiqueResponse, ApiErreur> {
    let organisateur = charger_organisateur(pool, row.cree_par).await?;
    let pays_nom = charger_nom_pays(pool, row.pays_id).await?;
    let domaine_nom = charger_nom_domaine(pool, row.domaine_id).await?;
    let nombre_candidatures = compter_candidatures(pool, row.id).await?;

    Ok(SabbatiqueResponse {
        id: row.id,
        titre: row.titre.clone(),
        description: row.description.clone(),
        couverture_url: row.image_couverture_url.clone(),
        pays: pays_nom,
        ville: row.ville.clone(),
        domaine: domaine_nom,
        duree: row.duree.clone(),
        duree_label: duree_label(&row.duree),
        date_debut: row.date_debut,
        date_fin: row.date_fin,
        interafricain: row.interafricain,
        statut: calculer_statut(&row.etat, &row.date_debut, row.date_fin.as_ref()),
        prise_en_charge: prises_en_charge(
            row.prise_en_charge_billet,
            row.prise_en_charge_hebergement,
            row.prise_en_charge_subsistance,
        ),
        nombre_places: row.nombre_places,
        nombre_candidatures,
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
// GET /api/sabbatiques — Lister les programmes avec filtres et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_programmes(
    pool: web::Data<PgPool>,
    params: web::Query<SabbatiqueQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Construire les conditions WHERE dynamiquement
    let mut conditions: Vec<String> = vec![
        "p.etat IN ('publie','en_cours','termine')".to_string(),
        "p.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par type (interafricain / hors_afrique)
    if let Some(ref type_prog) = params.type_programme {
        if type_prog != "tous" {
            let est_interafricain = type_prog == "interafricain";
            conditions.push(format!("p.interafricain = ${}::BOOLEAN", bind_index));
            bind_values.push(est_interafricain.to_string());
            bind_index += 1;
        }
    }

    // Filtre par pays
    if let Some(ref pays) = params.pays {
        if !pays.is_empty() {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays sp WHERE sp.id = p.pays_id AND LOWER(sp.nom) = LOWER(${})) ",
                bind_index
            ));
            bind_values.push(pays.clone());
            bind_index += 1;
        }
    }

    // Filtre par domaine
    if let Some(ref domaine) = params.domaine {
        if !domaine.is_empty() {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.domaine_secteur ds WHERE ds.id = p.domaine_id AND ds.slug = ${})",
                bind_index
            ));
            bind_values.push(domaine.clone());
            bind_index += 1;
        }
    }

    // Filtre par recherche textuelle
    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            let terme = format!("%{}%", recherche.to_lowercase());
            conditions.push(format!(
                "(LOWER(p.titre) LIKE ${idx} OR LOWER(p.description) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM exchange.programme p WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les programmes
    let select_query = format!(
        "SELECT {} FROM exchange.programme p WHERE {} ORDER BY p.date_debut DESC LIMIT ${} OFFSET ${}",
        PROGRAMME_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, SabbatiqueRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    // Construire les reponses
    let mut programmes = Vec::with_capacity(rows.len());
    for row in &rows {
        programmes.push(construire_response(pool.get_ref(), row).await?);
    }

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SabbatiqueListeResponse {
            programmes,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/sabbatiques/{id} — Obtenir le detail d'un programme
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_programme(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM exchange.programme p
         WHERE p.id = $1 AND p.deleted_at IS NULL",
        PROGRAMME_COLONNES
    );

    let row = sqlx::query_as::<_, SabbatiqueRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| {
            ApiErreur::NonTrouve(format!("Programme avec id {} non trouve", id))
        })?;

    let organisateur = charger_organisateur(pool.get_ref(), row.cree_par).await?;
    let pays_nom = charger_nom_pays(pool.get_ref(), row.pays_id).await?;
    let domaine_nom = charger_nom_domaine(pool.get_ref(), row.domaine_id).await?;
    let nombre_candidatures = compter_candidatures(pool.get_ref(), row.id).await?;

    let reponse = SabbatiqueDetailResponse {
        id: row.id,
        titre: row.titre.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        couverture_url: row.image_couverture_url.clone(),
        document_url: row.document_legal_url.clone(),
        pays: pays_nom,
        ville: row.ville.clone(),
        adresse: row.adresse.clone(),
        domaine: domaine_nom,
        duree: row.duree.clone(),
        duree_label: duree_label(&row.duree),
        date_debut: row.date_debut,
        date_fin: row.date_fin,
        interafricain: row.interafricain,
        statut: calculer_statut(&row.etat, &row.date_debut, row.date_fin.as_ref()),
        prise_en_charge: prises_en_charge(
            row.prise_en_charge_billet,
            row.prise_en_charge_hebergement,
            row.prise_en_charge_subsistance,
        ),
        prise_en_charge_details: row.prise_en_charge_details.clone(),
        nombre_places: row.nombre_places,
        nombre_candidatures,
        prerequis: row.prerequis.clone(),
        langues_requises: row.langues_requises.clone(),
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
// POST /api/sabbatiques — Creer un programme (multipart/form-data)
// ──────────────────────────────────────────────────────────────
pub async fn creer_programme(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut type_programme: Option<String> = None;
    let mut pays: Option<String> = None;
    let mut ville: Option<String> = None;
    let mut domaine: Option<String> = None;
    let mut duree: Option<String> = None;
    let mut date_debut: Option<String> = None;
    let mut date_fin: Option<String> = None;
    let mut prise_billet = false;
    let mut prise_hebergement = false;
    let mut prise_subsistance = false;
    let mut _organisateur_nom: Option<String> = None;
    let mut _organisateur_email: Option<String> = None;
    let mut image_couverture_url: Option<String> = None;
    let mut document_legal_url: Option<String> = None;

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
            "type" | "type_programme" => {
                type_programme = Some(lire_champ_texte(&mut field).await?)
            }
            "pays" => pays = Some(lire_champ_texte(&mut field).await?),
            "ville" => ville = Some(lire_champ_texte(&mut field).await?),
            "domaine" => domaine = Some(lire_champ_texte(&mut field).await?),
            "duree" => duree = Some(lire_champ_texte(&mut field).await?),
            "date_debut" | "dateDebut" => {
                date_debut = Some(lire_champ_texte(&mut field).await?)
            }
            "date_fin" | "dateFin" => {
                date_fin = Some(lire_champ_texte(&mut field).await?)
            }
            "prise_billet" | "billet_avion" => {
                let val = lire_champ_texte(&mut field).await?;
                prise_billet = val == "true" || val == "1" || val == "billet_avion";
            }
            "prise_hebergement" | "hebergement" => {
                let val = lire_champ_texte(&mut field).await?;
                prise_hebergement = val == "true" || val == "1" || val == "hebergement";
            }
            "prise_subsistance" | "frais_subsistance" => {
                let val = lire_champ_texte(&mut field).await?;
                prise_subsistance = val == "true" || val == "1" || val == "frais_subsistance";
            }
            "prisesEnCharge" | "prises_en_charge" => {
                let val = lire_champ_texte(&mut field).await?;
                if val.contains("billet_avion") {
                    prise_billet = true;
                }
                if val.contains("hebergement") {
                    prise_hebergement = true;
                }
                if val.contains("frais_subsistance") {
                    prise_subsistance = true;
                }
            }
            "organisateur_nom" | "organisateurNom" => {
                _organisateur_nom = Some(lire_champ_texte(&mut field).await?)
            }
            "organisateur_email" | "organisateurEmail" => {
                _organisateur_email = Some(lire_champ_texte(&mut field).await?)
            }
            "couverture" | "image" | "couvertureFile" => {
                let nom_original = content_disposition
                    .as_ref()
                    .and_then(|cd| {
                        cd.get_filename().map(|f| sanitize_filename::sanitize(f))
                    })
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));

                let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
                let chemin = format!(
                    "{}/couvertures/{}",
                    upload_dir.get_ref(),
                    nom_fichier
                );

                sauvegarder_fichier(&mut field, &chemin).await?;
                image_couverture_url =
                    Some(format!("/uploads/couvertures/{}", nom_fichier));
            }
            "document" | "documentFile" => {
                let nom_original = content_disposition
                    .as_ref()
                    .and_then(|cd| {
                        cd.get_filename().map(|f| sanitize_filename::sanitize(f))
                    })
                    .unwrap_or_else(|| format!("{}.pdf", Uuid::new_v4()));

                let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
                let chemin = format!(
                    "{}/documents/{}",
                    upload_dir.get_ref(),
                    nom_fichier
                );

                sauvegarder_fichier(&mut field, &chemin).await?;
                document_legal_url =
                    Some(format!("/uploads/documents/{}", nom_fichier));
            }
            _ => {
                log::warn!(
                    "Champ multipart sabbatique inconnu ignore: {}",
                    nom_champ
                );
            }
        }
    }

    // Validation des champs obligatoires
    let titre = titre
        .ok_or_else(|| ApiErreur::Validation("Le titre est obligatoire".into()))?;
    let description = description
        .ok_or_else(|| ApiErreur::Validation("La description est obligatoire".into()))?;
    let type_programme = type_programme
        .ok_or_else(|| ApiErreur::Validation("Le type de programme est obligatoire".into()))?;
    let duree = duree
        .ok_or_else(|| ApiErreur::Validation("La duree est obligatoire".into()))?;
    let date_debut_str = date_debut
        .ok_or_else(|| ApiErreur::Validation("La date de debut est obligatoire".into()))?;

    let interafricain = type_programme == "interafricain";

    // Parser les dates (format YYYY-MM-DD ou YYYY-MM-DDTHH:MM)
    let date_part = date_debut_str.split('T').next().unwrap_or(&date_debut_str);
    let date_debut_parsed = chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d")
        .map_err(|_| ApiErreur::Validation("Format de date de debut invalide".into()))?;

    let date_fin_parsed = date_fin
        .as_ref()
        .map(|s| {
            let part = s.split('T').next().unwrap_or(s);
            chrono::NaiveDate::parse_from_str(part, "%Y-%m-%d")
                .map_err(|_| {
                    ApiErreur::Validation("Format de date de fin invalide".into())
                })
        })
        .transpose()?;

    // Resoudre pays_id
    let pays_id: Uuid = if let Some(ref nom_pays) = pays {
        sqlx::query_scalar("SELECT id FROM shared.pays WHERE LOWER(nom) = LOWER($1)")
            .bind(nom_pays)
            .fetch_optional(pool.get_ref())
            .await?
            .ok_or_else(|| {
                ApiErreur::Validation(format!("Pays '{}' non trouve", nom_pays))
            })?
    } else {
        return Err(ApiErreur::Validation(
            "Le pays est obligatoire".into(),
        ));
    };

    // Resoudre domaine_id
    let domaine_id: Option<Uuid> = if let Some(ref slug_domaine) = domaine {
        if !slug_domaine.is_empty() {
            sqlx::query_scalar(
                "SELECT id FROM shared.domaine_secteur WHERE slug = $1",
            )
            .bind(slug_domaine)
            .fetch_optional(pool.get_ref())
            .await?
        } else {
            None
        }
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
        .ok_or_else(|| {
            ApiErreur::Validation("Aucun utilisateur trouve en base".to_string())
        })?
    };

    let slug = generer_slug(&titre);

    // Inserer le programme
    let query = format!(
        "INSERT INTO exchange.programme
            (titre, slug, description, image_couverture_url, document_legal_url,
             pays_id, ville,
             prise_en_charge_billet, prise_en_charge_hebergement, prise_en_charge_subsistance,
             duree, domaine_id, date_debut, date_fin,
             interafricain, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5,
                 $6, $7,
                 $8, $9, $10,
                 $11::exchange.duree_programme, $12, $13, $14,
                 $15, 'publie', $16)
         RETURNING {}",
        PROGRAMME_COLONNES.replace("p.", "")
    );

    let row = sqlx::query_as::<_, SabbatiqueRow>(&query)
        .bind(&titre)
        .bind(&slug)
        .bind(&description)
        .bind(&image_couverture_url)
        .bind(&document_legal_url)
        .bind(pays_id)
        .bind(&ville)
        .bind(prise_billet)
        .bind(prise_hebergement)
        .bind(prise_subsistance)
        .bind(&duree)
        .bind(domaine_id)
        .bind(date_debut_parsed)
        .bind(date_fin_parsed)
        .bind(interafricain)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?;

    log::info!("Programme sabbatique cree: {} ({})", row.titre, row.id);

    let organisateur = charger_organisateur(pool.get_ref(), row.cree_par).await?;
    let pays_nom = charger_nom_pays(pool.get_ref(), row.pays_id).await?;
    let domaine_nom = charger_nom_domaine(pool.get_ref(), row.domaine_id).await?;

    let reponse = SabbatiqueDetailResponse {
        id: row.id,
        titre: row.titre.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        couverture_url: row.image_couverture_url.clone(),
        document_url: row.document_legal_url.clone(),
        pays: pays_nom,
        ville: row.ville.clone(),
        adresse: row.adresse.clone(),
        domaine: domaine_nom,
        duree: row.duree.clone(),
        duree_label: duree_label(&row.duree),
        date_debut: row.date_debut,
        date_fin: row.date_fin,
        interafricain: row.interafricain,
        statut: calculer_statut(&row.etat, &row.date_debut, row.date_fin.as_ref()),
        prise_en_charge: prises_en_charge(
            row.prise_en_charge_billet,
            row.prise_en_charge_hebergement,
            row.prise_en_charge_subsistance,
        ),
        prise_en_charge_details: row.prise_en_charge_details.clone(),
        nombre_places: row.nombre_places,
        nombre_candidatures: 0,
        prerequis: row.prerequis.clone(),
        langues_requises: row.langues_requises.clone(),
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
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/// Lire le contenu texte d'un champ multipart
async fn lire_champ_texte(
    field: &mut actix_multipart::Field,
) -> Result<String, ApiErreur> {
    let mut contenu = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| {
            ApiErreur::Upload(format!("Erreur lecture champ: {}", e))
        })?;
        contenu.extend_from_slice(&data);
    }
    String::from_utf8(contenu)
        .map_err(|e| ApiErreur::Upload(format!("Encodage UTF-8 invalide: {}", e)))
}

/// Sauvegarder un fichier uploade sur le disque local
async fn sauvegarder_fichier(
    field: &mut actix_multipart::Field,
    chemin: &str,
) -> Result<(), ApiErreur> {
    if let Some(parent) = std::path::Path::new(chemin).parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            ApiErreur::Upload(format!("Impossible de creer le repertoire: {}", e))
        })?;
    }

    let mut fichier = std::fs::File::create(chemin).map_err(|e| {
        ApiErreur::Upload(format!("Impossible de creer le fichier: {}", e))
    })?;

    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| {
            ApiErreur::Upload(format!("Erreur lecture fichier: {}", e))
        })?;
        fichier.write_all(&data).map_err(|e| {
            ApiErreur::Upload(format!("Erreur ecriture fichier: {}", e))
        })?;
    }

    Ok(())
}

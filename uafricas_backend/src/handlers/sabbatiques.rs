use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::sabbatique::{
    CandidatRetenuResponse, CandidatureResponse, CandidatureRow, OrganisateurInfo,
    OrganisateurResponse, SabbatiqueDetailResponse, SabbatiqueListeResponse,
    SabbatiqueQueryParams, SabbatiqueResponse, SabbatiqueRow, PROGRAMME_COLONNES,
    calculer_statut, duree_label, generer_slug, prises_en_charge,
    statut_emploi_label, type_organisation_label,
};
use crate::services::audit;

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

/// Charger le candidat retenu (selection finale, affichage public)
async fn charger_candidat_retenu(
    pool: &PgPool,
    row: &SabbatiqueRow,
) -> Result<Option<CandidatRetenuResponse>, ApiErreur> {
    let Some(candidat_id) = row.candidat_retenu_id else {
        return Ok(None);
    };
    let info = sqlx::query_as::<_, OrganisateurInfo>(
        "SELECT id, nom, prenom, email, photo_url FROM iam.utilisateur WHERE id = $1",
    )
    .bind(candidat_id)
    .fetch_optional(pool)
    .await?;

    Ok(info.map(|u| CandidatRetenuResponse {
        uid: u.id,
        nom: u.nom,
        prenom: u.prenom,
        retenu_at: row.candidat_retenu_at,
    }))
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
    let candidat_retenu = charger_candidat_retenu(pool, row).await?;

    Ok(SabbatiqueResponse {
        id: row.id,
        titre: row.titre.clone(),
        description: row.description.clone(),
        couverture_url: row.image_couverture_url.clone(),
        pays: pays_nom,
        ville: row.ville.clone(),
        domaine: row.domaine_libre.clone().or(domaine_nom),
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
        type_organisation: row.type_organisation.clone(),
        type_organisation_label: row
            .type_organisation
            .as_deref()
            .map(type_organisation_label),
        statut_legal: row.statut_legal.clone(),
        candidat_retenu,
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
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req);

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
    let candidat_retenu = charger_candidat_retenu(pool.get_ref(), &row).await?;

    let est_organisateur = utilisateur_id == Some(row.cree_par);
    let a_deja_candidate = if let Some(uid) = utilisateur_id {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM exchange.candidature
             WHERE programme_id = $1 AND candidat_id = $2 AND statut != 'retiree'",
        )
        .bind(row.id)
        .bind(uid)
        .fetch_one(pool.get_ref())
        .await?
            > 0
    } else {
        false
    };

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
        domaine: row.domaine_libre.clone().or(domaine_nom),
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
        type_organisation: row.type_organisation.clone(),
        type_organisation_label: row
            .type_organisation
            .as_deref()
            .map(type_organisation_label),
        statut_legal: row.statut_legal.clone(),
        candidat_retenu,
        est_organisateur,
        a_deja_candidate,
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
    let mut domaine_precision: Option<String> = None;
    let mut statut_legal: Option<String> = None;
    let mut duree: Option<String> = None;
    let mut date_debut: Option<String> = None;
    let mut date_fin: Option<String> = None;
    let mut type_organisation: Option<String> = None;
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
            "domaine_precision" | "domainePrecision" => {
                domaine_precision = Some(lire_champ_texte(&mut field).await?)
            }
            "statut_legal" | "statutLegal" => {
                statut_legal = Some(lire_champ_texte(&mut field).await?)
            }
            "duree" => duree = Some(lire_champ_texte(&mut field).await?),
            "type_organisation" | "typeOrganisation" => {
                type_organisation = Some(lire_champ_texte(&mut field).await?)
            }
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

    // Type d'organisation soumettante (association / entreprise / service public)
    let type_organisation = type_organisation
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            ApiErreur::Validation("Le type d'organisation est obligatoire".into())
        })?;
    if !["association", "entreprise", "service_public"]
        .contains(&type_organisation.as_str())
    {
        return Err(ApiErreur::Validation(
            "Type d'organisation invalide".into(),
        ));
    }

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

    // Domaine libre : précision saisie quand le domaine choisi est « Autre »
    // (dans ce cas domaine_id reste NULL, le texte libre prend le relais)
    let domaine_libre: Option<String> = if domaine.as_deref() == Some("autre") {
        domaine_precision
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
    } else {
        None
    };

    // Statut légal de l'organisation soumettante (texte libre facultatif)
    let statut_legal: Option<String> = statut_legal
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from);

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
             interafricain, type_organisation, domaine_libre, statut_legal, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5,
                 $6, $7,
                 $8, $9, $10,
                 $11::exchange.duree_programme, $12, $13, $14,
                 $15, $16::exchange.type_organisation_soumettante, $17, $18, 'publie', $19)
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
        .bind(&type_organisation)
        .bind(&domaine_libre)
        .bind(&statut_legal)
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
        domaine: row.domaine_libre.clone().or(domaine_nom),
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
        type_organisation: row.type_organisation.clone(),
        type_organisation_label: row
            .type_organisation
            .as_deref()
            .map(type_organisation_label),
        statut_legal: row.statut_legal.clone(),
        candidat_retenu: None,
        est_organisateur: true,
        a_deja_candidate: false,
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
// GET /api/sabbatiques/mes-programmes — Programmes créés par l'utilisateur
// ──────────────────────────────────────────────────────────────
pub async fn lister_mes_programmes(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req).ok_or_else(|| {
        ApiErreur::NonAutorise("Authentification requise".into())
    })?;

    let query = format!(
        "SELECT {} FROM exchange.programme p
         WHERE p.cree_par = $1 AND p.deleted_at IS NULL
         ORDER BY p.created_at DESC",
        PROGRAMME_COLONNES
    );

    let rows = sqlx::query_as::<_, SabbatiqueRow>(&query)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?;

    let mut programmes = Vec::with_capacity(rows.len());
    for row in &rows {
        programmes.push(construire_response(pool.get_ref(), row).await?);
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(programmes),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/sabbatiques/{id}/candidatures — Candidater a un programme
// ──────────────────────────────────────────────────────────────
pub async fn candidater(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    chemin: web::Path<Uuid>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let programme_id = chemin.into_inner();

    let candidat_id = extraire_utilisateur_id(&req).ok_or_else(|| {
        ApiErreur::NonAutorise("Vous devez être connecté pour candidater".into())
    })?;

    // Le programme doit exister, etre publie et non supprime
    let createur: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM exchange.programme
         WHERE id = $1 AND deleted_at IS NULL
           AND etat IN ('publie','en_cours')",
    )
    .bind(programme_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let createur = createur.ok_or_else(|| {
        ApiErreur::NonTrouve("Programme introuvable ou clôturé".into())
    })?;

    if createur == candidat_id {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas candidater à votre propre programme".into(),
        ));
    }

    let mut nom_etat_civil: Option<String> = None;
    let mut fonction_actuelle: Option<String> = None;
    let mut lieu_residence: Option<String> = None;
    let mut statut_emploi: Option<String> = None;
    let mut repond_profil = false;
    let mut lettre_motivation: Option<String> = None;
    let mut lien_expertise: Option<String> = None;
    let mut cv_url: Option<String> = None;

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
            "nom_etat_civil" | "nomEtatCivil" => {
                nom_etat_civil = Some(lire_champ_texte(&mut field).await?)
            }
            "fonction_actuelle" | "fonctionActuelle" => {
                fonction_actuelle = Some(lire_champ_texte(&mut field).await?)
            }
            "lieu_residence" | "lieuResidence" => {
                lieu_residence = Some(lire_champ_texte(&mut field).await?)
            }
            "statut_emploi" | "statutEmploi" => {
                statut_emploi = Some(lire_champ_texte(&mut field).await?)
            }
            "repond_profil" | "repondProfil" => {
                let val = lire_champ_texte(&mut field).await?;
                repond_profil =
                    val == "true" || val == "1" || val.eq_ignore_ascii_case("oui");
            }
            "lettre_motivation" | "lettreMotivation" => {
                lettre_motivation = Some(lire_champ_texte(&mut field).await?)
            }
            "lien_expertise" | "lienExpertise" => {
                lien_expertise = Some(lire_champ_texte(&mut field).await?)
            }
            "cv" | "cvFile" => {
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
                cv_url = Some(format!("/uploads/documents/{}", nom_fichier));
            }
            _ => {
                log::warn!("Champ multipart candidature inconnu ignore: {}", nom_champ);
            }
        }
    }

    // Validation
    let nom_etat_civil = nom_etat_civil
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| {
            ApiErreur::Validation("Le nom et prénoms à l'état civil sont obligatoires".into())
        })?;
    let fonction_actuelle = fonction_actuelle
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| ApiErreur::Validation("La fonction actuelle est obligatoire".into()))?;
    let lieu_residence = lieu_residence
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| {
            ApiErreur::Validation("Le lieu de résidence ou de fonction est obligatoire".into())
        })?;
    let statut_emploi = statut_emploi
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            ApiErreur::Validation(
                "Vous devez être en emploi ou retraité pour candidater".into(),
            )
        })?;
    // Condition d'éligibilité : seuls « en_emploi » et « retraite » sont admis
    if !["en_emploi", "retraite"].contains(&statut_emploi.as_str()) {
        return Err(ApiErreur::Validation(
            "Candidature réservée aux personnes en emploi ou retraitées".into(),
        ));
    }
    // Lettre de motivation obligatoire
    let lettre_motivation = lettre_motivation
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| {
            ApiErreur::Validation("La lettre de motivation est obligatoire".into())
        })?;
    // Au moins un justificatif : CV téléversé ou lien vers le compte expertise
    let lien_expertise = lien_expertise.filter(|s| !s.trim().is_empty());
    if cv_url.is_none() && lien_expertise.is_none() {
        return Err(ApiErreur::Validation(
            "Veuillez joindre un CV ou le lien de votre compte expertise".into(),
        ));
    }

    // Upsert (re-candidature = mise à jour, remise à l'état « soumise »)
    let candidature_id: Uuid = sqlx::query_scalar(
        "INSERT INTO exchange.candidature
            (programme_id, candidat_id, lettre_motivation, cv_url, lien_expertise,
             nom_etat_civil, fonction_actuelle, lieu_residence,
             statut_emploi, repond_profil, statut)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8,
                 $9::exchange.statut_emploi_candidat, $10, 'soumise')
         ON CONFLICT (programme_id, candidat_id) DO UPDATE SET
             lettre_motivation = EXCLUDED.lettre_motivation,
             cv_url            = COALESCE(EXCLUDED.cv_url, exchange.candidature.cv_url),
             lien_expertise    = EXCLUDED.lien_expertise,
             nom_etat_civil    = EXCLUDED.nom_etat_civil,
             fonction_actuelle = EXCLUDED.fonction_actuelle,
             lieu_residence    = EXCLUDED.lieu_residence,
             statut_emploi     = EXCLUDED.statut_emploi,
             repond_profil     = EXCLUDED.repond_profil,
             statut            = 'soumise',
             updated_at        = NOW()
         RETURNING id",
    )
    .bind(programme_id)
    .bind(candidat_id)
    .bind(&lettre_motivation)
    .bind(&cv_url)
    .bind(&lien_expertise)
    .bind(&nom_etat_civil)
    .bind(&fonction_actuelle)
    .bind(&lieu_residence)
    .bind(&statut_emploi)
    .bind(repond_profil)
    .fetch_one(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(candidat_id),
        "candidature_soumise",
        "exchange",
        "candidature",
        Some(candidature_id),
        None,
        None,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": candidature_id })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/sabbatiques/{id}/candidatures — Lister (organisateur uniquement)
// ──────────────────────────────────────────────────────────────
pub async fn lister_candidatures(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let programme_id = chemin.into_inner();

    let utilisateur_id = extraire_utilisateur_id(&req).ok_or_else(|| {
        ApiErreur::NonAutorise("Authentification requise".into())
    })?;

    // Verifier que l'utilisateur est bien l'organisateur + recuperer le candidat retenu
    let infos: Option<(Uuid, Option<Uuid>)> = sqlx::query_as(
        "SELECT cree_par, candidat_retenu_id FROM exchange.programme
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(programme_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (cree_par, candidat_retenu_id) =
        infos.ok_or_else(|| ApiErreur::NonTrouve("Programme introuvable".into()))?;

    if cree_par != utilisateur_id {
        return Err(ApiErreur::NonAutorise(
            "Seul l'organisateur peut consulter les candidatures".into(),
        ));
    }

    let rows = sqlx::query_as::<_, CandidatureRow>(
        "SELECT id, candidat_id, lettre_motivation, cv_url, lien_expertise,
                nom_etat_civil, fonction_actuelle, lieu_residence,
                statut_emploi::text AS statut_emploi, repond_profil,
                statut::text AS statut, created_at
         FROM exchange.candidature
         WHERE programme_id = $1 AND statut != 'retiree'
         ORDER BY created_at DESC",
    )
    .bind(programme_id)
    .fetch_all(pool.get_ref())
    .await?;

    let mut candidatures = Vec::with_capacity(rows.len());
    for row in &rows {
        let candidat = charger_organisateur(pool.get_ref(), row.candidat_id).await?;
        candidatures.push(CandidatureResponse {
            id: row.id,
            candidat: OrganisateurResponse {
                uid: candidat.id,
                nom: candidat.nom,
                prenom: candidat.prenom,
                email: candidat.email,
                photo_url: candidat.photo_url,
            },
            nom_etat_civil: row.nom_etat_civil.clone(),
            fonction_actuelle: row.fonction_actuelle.clone(),
            lieu_residence: row.lieu_residence.clone(),
            statut_emploi: row.statut_emploi.clone(),
            statut_emploi_label: row.statut_emploi.as_deref().map(statut_emploi_label),
            repond_profil: row.repond_profil,
            lettre_motivation: row.lettre_motivation.clone(),
            cv_url: row.cv_url.clone(),
            lien_expertise: row.lien_expertise.clone(),
            statut: row.statut.clone(),
            est_retenu: candidat_retenu_id == Some(row.candidat_id),
            created_at: row.created_at,
        });
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(candidatures),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/sabbatiques/{id}/candidatures/{candidature_id}/retenir
// Selection du candidat final par l'organisateur (affichage public)
// ──────────────────────────────────────────────────────────────
pub async fn selectionner_candidat(
    pool: web::Data<PgPool>,
    chemin: web::Path<(Uuid, Uuid)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let (programme_id, candidature_id) = chemin.into_inner();

    let utilisateur_id = extraire_utilisateur_id(&req).ok_or_else(|| {
        ApiErreur::NonAutorise("Authentification requise".into())
    })?;

    let cree_par: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM exchange.programme WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(programme_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let cree_par =
        cree_par.ok_or_else(|| ApiErreur::NonTrouve("Programme introuvable".into()))?;

    if cree_par != utilisateur_id {
        return Err(ApiErreur::NonAutorise(
            "Seul l'organisateur peut sélectionner le candidat final".into(),
        ));
    }

    // La candidature doit appartenir au programme
    let candidat_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT candidat_id FROM exchange.candidature
         WHERE id = $1 AND programme_id = $2",
    )
    .bind(candidature_id)
    .bind(programme_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let candidat_id = candidat_id
        .ok_or_else(|| ApiErreur::NonTrouve("Candidature introuvable".into()))?;

    let mut tx = pool.begin().await?;

    sqlx::query(
        "UPDATE exchange.programme
         SET candidat_retenu_id = $1, candidat_retenu_at = NOW(), updated_at = NOW()
         WHERE id = $2",
    )
    .bind(candidat_id)
    .bind(programme_id)
    .execute(&mut *tx)
    .await?;

    // La candidature retenue passe a « acceptee »
    sqlx::query(
        "UPDATE exchange.candidature SET statut = 'acceptee', updated_at = NOW()
         WHERE id = $1",
    )
    .bind(candidature_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "candidat_retenu",
        "exchange",
        "programme",
        Some(programme_id),
        None,
        Some(serde_json::json!({ "candidat_id": candidat_id })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "candidat_retenu_id": candidat_id })),
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

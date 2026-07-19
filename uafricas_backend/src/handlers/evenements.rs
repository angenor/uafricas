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
    EvenementResponse, EvenementRow, InscritEvenementResponse,
    ModifierMonEvenementRequest, OrganisateurInfo, OrganisateurResponse,
    EVENEMENT_COLONNES, calculer_statut, generer_slug, mapper_format_db,
    mapper_format_frontend,
};

/// Types d'organisateur acceptes (nom propre vs organisation)
const TYPES_ORGANISATEUR_VALIDES: &[&str] = &["personnel", "organisation"];

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
        thematique: row.type_categorie.clone(),
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
            email: String::new(), // e-mail de compte non exposé publiquement (audit #26)
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

    // Filtre par zone geographique (Afrique / Hors Afrique).
    // La liste des codes ISO2 africains est une constante figee (aucun risque
    // d'injection) : on l'injecte directement dans le IN, sans bind parametre.
    if let Some(ref zone) = params.zone {
        let liste_iso = crate::constants::afripulse_pays_autorises::PAYS_AFRICAINS_ISO2
            .iter()
            .map(|c| format!("'{}'", c))
            .collect::<Vec<_>>()
            .join(",");
        match zone.as_str() {
            "afrique" => conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p WHERE p.id = e.pays_id \
                 AND LOWER(p.code_iso2) IN ({}))",
                liste_iso
            )),
            "hors_afrique" => conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays p WHERE p.id = e.pays_id \
                 AND (p.code_iso2 IS NULL OR LOWER(p.code_iso2) NOT IN ({})))",
                liste_iso
            )),
            _ => {}
        }
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
        thematique: row.type_categorie.clone(),
        pays: pays_nom,
        ville: row.ville.clone(),
        adresse: row.adresse.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        lien_en_ligne: row.lien_en_ligne.clone(),
        statut: calculer_statut(&row.etat, &row.date_heure_debut, row.date_heure_fin.as_ref()),
        etat: row.etat.clone(),
        nombre_places: row.nombre_places,
        nombre_inscrits,
        est_inscrit: inscrit,
        type_organisateur: row.type_organisateur.clone(),
        contact_nom: row.contact_nom.clone(),
        contact_email: row.contact_email.clone(),
        contact_telephone: row.contact_telephone.clone(),
        contact_site_web: row.contact_site_web.clone(),
        enregistrement_url: row.enregistrement_url.clone(),
        user: OrganisateurResponse {
            uid: organisateur.id,
            nom: organisateur.nom,
            prenom: organisateur.prenom,
            email: String::new(), // e-mail de compte non exposé publiquement (audit #26)
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
    let mut thematique: Option<String> = None;
    let mut pays: Option<String> = None;
    let mut ville: Option<String> = None;
    let mut date_heure_debut: Option<String> = None;
    let mut date_heure_fin: Option<String> = None;
    let mut adresse: Option<String> = None;
    let mut lien_en_ligne: Option<String> = None;
    let mut nombre_places: Option<i32> = None;
    let mut type_organisateur: Option<String> = None;
    let mut contact_nom: Option<String> = None;
    let mut contact_email: Option<String> = None;
    let mut contact_telephone: Option<String> = None;
    let mut contact_site_web: Option<String> = None;
    let mut enregistrement_url: Option<String> = None;
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
            "thematique" => thematique = texte_optionnel(lire_champ_texte(&mut field).await?),
            "pays" => pays = Some(lire_champ_texte(&mut field).await?),
            "ville" => ville = Some(lire_champ_texte(&mut field).await?),
            "date_heure_debut" => date_heure_debut = Some(lire_champ_texte(&mut field).await?),
            "date_heure_fin" => date_heure_fin = Some(lire_champ_texte(&mut field).await?),
            "adresse" => adresse = texte_optionnel(lire_champ_texte(&mut field).await?),
            "lien_en_ligne" => lien_en_ligne = texte_optionnel(lire_champ_texte(&mut field).await?),
            "nombre_places" => {
                let val = lire_champ_texte(&mut field).await?;
                nombre_places = val.trim().parse::<i32>().ok().filter(|n| *n > 0);
            }
            "type_organisateur" => {
                type_organisateur = texte_optionnel(lire_champ_texte(&mut field).await?)
                    .map(|v| v.to_lowercase());
            }
            "contact_nom" => contact_nom = texte_optionnel(lire_champ_texte(&mut field).await?),
            "contact_email" => contact_email = texte_optionnel(lire_champ_texte(&mut field).await?),
            "contact_telephone" => contact_telephone = texte_optionnel(lire_champ_texte(&mut field).await?),
            "contact_site_web" => contact_site_web = texte_optionnel(lire_champ_texte(&mut field).await?),
            "enregistrement_url" => enregistrement_url = texte_optionnel(lire_champ_texte(&mut field).await?),
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

    // Type d'organisateur (nom propre vs organisation) — important pour les stats.
    let type_organisateur = type_organisateur.unwrap_or_else(|| "personnel".to_string());
    if !TYPES_ORGANISATEUR_VALIDES.contains(&type_organisateur.as_str()) {
        return Err(ApiErreur::Validation(
            "Type d'organisateur invalide (personnel ou organisation)".into(),
        ));
    }
    // Au nom d'une organisation : le nom de l'organisation (contact_nom) est requis.
    if type_organisateur == "organisation" && contact_nom.is_none() {
        return Err(ApiErreur::Validation(
            "Le nom de l'organisation est requis".into(),
        ));
    }

    // Inserer l'evenement
    let query = format!(
        "INSERT INTO media_content.evenement
            (titre, slug, description, type, format, pays_id, ville, adresse,
             date_heure_debut, date_heure_fin, image_couverture_url,
             lien_en_ligne, nombre_places, type_organisateur,
             contact_nom, contact_email, contact_telephone, contact_site_web,
             enregistrement_url, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5::media_content.format_evenement, $6, $7, $8,
                 $9, $10, $11, $12, $13, $14::media_content.type_organisateur,
                 $15, $16, $17, $18,
                 $19, 'publie', $20)
         RETURNING {}",
        EVENEMENT_COLONNES.replace("e.", "")
    );

    let row = sqlx::query_as::<_, EvenementRow>(&query)
        .bind(&titre)
        .bind(&slug)
        .bind(&description)
        .bind(&thematique)
        .bind(&format_evt)
        .bind(pays_id)
        .bind(&ville)
        .bind(&adresse)
        .bind(date_debut)
        .bind(date_fin)
        .bind(&image_couverture_url)
        .bind(&lien_en_ligne)
        .bind(nombre_places)
        .bind(&type_organisateur)
        .bind(&contact_nom)
        .bind(&contact_email)
        .bind(&contact_telephone)
        .bind(&contact_site_web)
        .bind(&enregistrement_url)
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
        thematique: row.type_categorie.clone(),
        pays: pays_nom,
        ville: row.ville.clone(),
        adresse: row.adresse.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        lien_en_ligne: row.lien_en_ligne.clone(),
        statut: calculer_statut(&row.etat, &row.date_heure_debut, row.date_heure_fin.as_ref()),
        etat: row.etat.clone(),
        nombre_places: row.nombre_places,
        nombre_inscrits: 0,
        est_inscrit: false,
        type_organisateur: row.type_organisateur.clone(),
        contact_nom: row.contact_nom.clone(),
        contact_email: row.contact_email.clone(),
        contact_telephone: row.contact_telephone.clone(),
        contact_site_web: row.contact_site_web.clone(),
        enregistrement_url: row.enregistrement_url.clone(),
        user: OrganisateurResponse {
            uid: organisateur.id,
            nom: organisateur.nom,
            prenom: organisateur.prenom,
            email: String::new(), // e-mail de compte non exposé publiquement (audit #26)
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

// ══════════════════════════════════════════════════════════════
// Gestion par l'organisateur (membre) — « Mes evenements »
// ══════════════════════════════════════════════════════════════

/// Construire un EvenementDetailResponse complet a partir d'une ligne.
async fn construire_detail(
    pool: &PgPool,
    row: &EvenementRow,
    est_inscrit_flag: bool,
) -> Result<EvenementDetailResponse, ApiErreur> {
    let organisateur = charger_organisateur(pool, row.cree_par).await?;
    let nombre_inscrits = compter_inscrits(pool, row.id).await?;
    let pays_nom = charger_nom_pays(pool, row.pays_id).await?;

    Ok(EvenementDetailResponse {
        id: row.id,
        titre: row.titre.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        type_format: mapper_format_frontend(&row.format),
        thematique: row.type_categorie.clone(),
        pays: pays_nom,
        ville: row.ville.clone(),
        adresse: row.adresse.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        lien_en_ligne: row.lien_en_ligne.clone(),
        statut: calculer_statut(&row.etat, &row.date_heure_debut, row.date_heure_fin.as_ref()),
        etat: row.etat.clone(),
        nombre_places: row.nombre_places,
        nombre_inscrits,
        est_inscrit: est_inscrit_flag,
        type_organisateur: row.type_organisateur.clone(),
        contact_nom: row.contact_nom.clone(),
        contact_email: row.contact_email.clone(),
        contact_telephone: row.contact_telephone.clone(),
        contact_site_web: row.contact_site_web.clone(),
        enregistrement_url: row.enregistrement_url.clone(),
        user: OrganisateurResponse {
            uid: organisateur.id,
            nom: organisateur.nom,
            prenom: organisateur.prenom,
            email: String::new(), // e-mail de compte non exposé publiquement (audit #26)
            photo_url: organisateur.photo_url,
        },
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

/// Charger une ligne evenement appartenant a l'utilisateur, sinon 404/403.
async fn charger_mon_evenement(
    pool: &PgPool,
    id: Uuid,
    utilisateur_id: Uuid,
) -> Result<EvenementRow, ApiErreur> {
    let query = format!(
        "SELECT {} FROM media_content.evenement e WHERE e.id = $1 AND e.deleted_at IS NULL",
        EVENEMENT_COLONNES
    );
    let row = sqlx::query_as::<_, EvenementRow>(&query)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Evenement non trouve".to_string()))?;
    if row.cree_par != utilisateur_id {
        return Err(ApiErreur::NonAutorise(
            "Vous n'etes pas l'organisateur de cet evenement".to_string(),
        ));
    }
    Ok(row)
}

// GET /api/evenements/mes-evenements — Lister les evenements de l'organisateur connecte
pub async fn lister_mes_evenements(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let query = format!(
        "SELECT {} FROM media_content.evenement e
         WHERE e.cree_par = $1 AND e.deleted_at IS NULL
         ORDER BY e.date_heure_debut DESC",
        EVENEMENT_COLONNES
    );
    let rows = sqlx::query_as::<_, EvenementRow>(&query)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?;

    let mut evenements = Vec::with_capacity(rows.len());
    for row in &rows {
        evenements.push(construire_detail(pool.get_ref(), row, false).await?);
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(evenements),
        error: None,
    }))
}

// PUT /api/evenements/{id} — Modifier son propre evenement (JSON)
pub async fn modifier_mon_evenement(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierMonEvenementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;
    let id = chemin.into_inner();

    // Verifie l'existence + la propriete.
    let actuel = charger_mon_evenement(pool.get_ref(), id, utilisateur_id).await?;

    let nettoyer = |o: &Option<String>| {
        o.as_deref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    };

    let mut sets: Vec<String> = Vec::new();
    let mut binds: Vec<String> = Vec::new();
    let mut idx: u32 = 1;

    macro_rules! set_str_opt {
        ($val:expr, $col:expr) => {
            // None = champ absent (inchange) ; "" = effacement (NULL).
            if let Some(ref v) = $val {
                let t = v.trim();
                if t.is_empty() {
                    sets.push(format!("{} = NULL", $col));
                } else {
                    sets.push(format!("{} = ${}", $col, idx));
                    binds.push(t.to_string());
                    idx += 1;
                }
            }
        };
    }

    // Titre (obligatoire si fourni) + slug.
    if let Some(ref t) = body.titre {
        let t = t.trim();
        if t.is_empty() {
            return Err(ApiErreur::Validation("Le titre ne peut pas etre vide".into()));
        }
        sets.push(format!("titre = ${}", idx));
        binds.push(t.to_string());
        idx += 1;
        sets.push(format!("slug = ${}", idx));
        binds.push(generer_slug(t));
        idx += 1;
    }
    if let Some(ref d) = body.description {
        let d = d.trim();
        if d.is_empty() {
            return Err(ApiErreur::Validation("La description ne peut pas etre vide".into()));
        }
        sets.push(format!("description = ${}", idx));
        binds.push(d.to_string());
        idx += 1;
    }

    // Format (frontend ou DB).
    if let Some(ref f) = body.type_format {
        let fmt = mapper_format_db(f);
        sets.push(format!("format = ${}::media_content.format_evenement", idx));
        binds.push(fmt);
        idx += 1;
    }

    set_str_opt!(body.ville, "ville");
    set_str_opt!(body.adresse, "adresse");
    set_str_opt!(body.lien_en_ligne, "lien_en_ligne");
    set_str_opt!(body.contact_email, "contact_email");
    set_str_opt!(body.contact_telephone, "contact_telephone");
    set_str_opt!(body.contact_site_web, "contact_site_web");
    set_str_opt!(body.enregistrement_url, "enregistrement_url");

    // Territoire -> pays_id.
    if let Some(ref pays_nom) = body.pays {
        let pays_nom = pays_nom.trim();
        if pays_nom.is_empty() {
            sets.push("pays_id = NULL".to_string());
        } else {
            let pays_id: Option<Uuid> =
                sqlx::query_scalar("SELECT id FROM shared.pays WHERE LOWER(nom) = LOWER($1)")
                    .bind(pays_nom)
                    .fetch_optional(pool.get_ref())
                    .await?;
            if let Some(pid) = pays_id {
                sets.push(format!("pays_id = ${}", idx));
                binds.push(pid.to_string());
                idx += 1;
            }
        }
    }

    // Nombre de places.
    if let Some(n) = body.nombre_places {
        if n > 0 {
            sets.push(format!("nombre_places = {}", n));
        } else {
            sets.push("nombre_places = NULL".to_string());
        }
    }

    // Dates.
    if let Some(ref d) = body.date_heure_debut {
        let dt = parser_date(d).ok_or_else(|| ApiErreur::Validation("Date de debut invalide".into()))?;
        sets.push(format!("date_heure_debut = '{}'", dt.to_rfc3339()));
    }
    if let Some(ref d) = body.date_heure_fin {
        if d.trim().is_empty() {
            sets.push("date_heure_fin = NULL".to_string());
        } else {
            let dt = parser_date(d).ok_or_else(|| ApiErreur::Validation("Date de fin invalide".into()))?;
            sets.push(format!("date_heure_fin = '{}'", dt.to_rfc3339()));
        }
    }

    // Type d'organisateur (nom propre vs organisation) — important pour les stats.
    if let Some(ref t) = body.type_organisateur {
        if t != "personnel" && t != "organisation" {
            return Err(ApiErreur::Validation(
                "Type d'organisateur invalide (personnel ou organisation)".into(),
            ));
        }
        sets.push(format!("type_organisateur = ${}::media_content.type_organisateur", idx));
        binds.push(t.clone());
        idx += 1;
        if t == "personnel" {
            sets.push("contact_nom = NULL".to_string());
        } else {
            match nettoyer(&body.contact_nom) {
                Some(n) => {
                    sets.push(format!("contact_nom = ${}", idx));
                    binds.push(n);
                    idx += 1;
                }
                None => {
                    // Pas de nouveau nom : exiger qu'il en existe deja un.
                    if actuel.contact_nom.is_none() {
                        return Err(ApiErreur::Validation(
                            "Le nom de l'organisation est requis".into(),
                        ));
                    }
                }
            }
        }
    } else {
        set_str_opt!(body.contact_nom, "contact_nom");
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE media_content.evenement SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "),
        idx
    );
    let mut q = sqlx::query(&sql);
    for b in &binds {
        q = q.bind(b);
    }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Organisateur {} a modifie son evenement {}", utilisateur_id, id);

    // Recharger pour la reponse.
    let row = charger_mon_evenement(pool.get_ref(), id, utilisateur_id).await?;
    let reponse = construire_detail(pool.get_ref(), &row, false).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// DELETE /api/evenements/{id} — Supprimer (soft) son propre evenement
pub async fn supprimer_mon_evenement(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;
    let id = chemin.into_inner();

    // Verifie la propriete.
    charger_mon_evenement(pool.get_ref(), id, utilisateur_id).await?;

    sqlx::query(
        "UPDATE media_content.evenement SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Organisateur {} a supprime son evenement {}", utilisateur_id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// GET /api/evenements/{id}/inscrits — Lister les inscrits a son propre evenement
pub async fn lister_inscrits_mon_evenement(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;
    let id = chemin.into_inner();

    // Verifie la propriete.
    charger_mon_evenement(pool.get_ref(), id, utilisateur_id).await?;

    let inscrits = sqlx::query_as::<_, InscritEvenementResponse>(
        "SELECT ei.utilisateur_id, u.nom, u.prenom, u.email, ei.statut, ei.created_at
         FROM media_content.evenement_inscription ei
         JOIN iam.utilisateur u ON ei.utilisateur_id = u.id
         WHERE ei.evenement_id = $1 AND ei.statut != 'annule'
         ORDER BY ei.created_at DESC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(inscrits),
        error: None,
    }))
}

// POST /api/evenements/{id}/couverture — Remplacer l'image de couverture (multipart)
pub async fn changer_couverture_mon_evenement(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;
    let id = chemin.into_inner();

    // Verifie l'existence + la propriete.
    charger_mon_evenement(pool.get_ref(), id, utilisateur_id).await?;

    let mut image_couverture_url: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| ApiErreur::Upload(format!("Erreur lecture multipart: {}", e)))?;
        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        if nom_champ == "couverture" || nom_champ == "image" {
            let nom_original = content_disposition
                .as_ref()
                .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));
            let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
            let chemin_complet = format!("{}/couvertures/{}", upload_dir.get_ref(), nom_fichier);
            sauvegarder_fichier(&mut field, &chemin_complet).await?;
            image_couverture_url = Some(format!("/uploads/couvertures/{}", nom_fichier));
        }
    }

    let url = image_couverture_url
        .ok_or_else(|| ApiErreur::Validation("Aucune image fournie".to_string()))?;

    sqlx::query(
        "UPDATE media_content.evenement SET image_couverture_url = $1, updated_at = NOW()
         WHERE id = $2 AND deleted_at IS NULL",
    )
    .bind(&url)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Organisateur {} a change la couverture de son evenement {}", utilisateur_id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "couverture_url": url })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/// Parser une date RFC3339 ou "%Y-%m-%dT%H:%M" en UTC.
fn parser_date(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&chrono::Utc))
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S"))
                .map(|d| d.and_utc())
        })
        .ok()
}

/// Normaliser un champ texte optionnel : trim + None si vide
fn texte_optionnel(valeur: String) -> Option<String> {
    let v = valeur.trim();
    if v.is_empty() {
        None
    } else {
        Some(v.to_string())
    }
}

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

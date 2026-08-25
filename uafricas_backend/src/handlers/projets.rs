use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::projet::{
    ProjetDetailRow, ProjetDocumentResponse, ProjetListeResponse, ProjetListeRow,
    ProjetQueryParams, ProjetStatistiquesResponse, generer_slug, joindre_objectifs,
    PROJET_DETAIL_COLONNES, PROJET_LISTE_COLONNES,
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

// ──────────────────────────────────────────────────────────────
// GET /api/projets : Lister les projets avec filtres et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_projets(
    pool: web::Data<PgPool>,
    params: web::Query<ProjetQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let terme_recherche: Option<String> = params
        .recherche
        .as_ref()
        .filter(|r| !r.trim().is_empty())
        .map(|r| format!("%{}%", r.to_lowercase()));

    let pays_filtre: Option<String> = params
        .pays
        .as_ref()
        .filter(|p| !p.trim().is_empty())
        .cloned();

    let order_clause = match params.tri.as_deref() {
        Some("titre") => "ORDER BY p.titre ASC",
        Some("cout_asc") => "ORDER BY p.cout_total ASC NULLS LAST",
        Some("cout_desc") => "ORDER BY p.cout_total DESC NULLS LAST",
        _ => "ORDER BY p.created_at DESC",
    };

    // Construire les conditions dynamiques pour duree et budget_max
    let mut conditions_extra = String::new();
    let mut bind_index = 4u32; // $1=pays, $2=recherche, $3 libre, on commence a $4

    if let Some(budget_max) = params.budget_max {
        if budget_max > 0.0 {
            bind_index += 1;
            conditions_extra.push_str(&format!(" AND p.cout_total <= ${}", bind_index - 1));
            // On va gerer le bind manuellement ci-dessous
        }
    }

    let duree_condition = match params.duree.as_deref() {
        Some("court") => Some("AND p.duree_mois IS NOT NULL AND p.duree_mois < 6"),
        Some("moyen") => Some("AND p.duree_mois IS NOT NULL AND p.duree_mois >= 6 AND p.duree_mois <= 24"),
        Some("long") => Some("AND p.duree_mois IS NOT NULL AND p.duree_mois > 24"),
        _ => None,
    };

    if let Some(cond) = duree_condition {
        conditions_extra.push_str(&format!(" {}", cond));
    }

    // ── Requete COUNT ──────────────────────────────────────
    let count_query = format!(
        "SELECT COUNT(*) FROM innovation.projet p
         LEFT JOIN shared.pays pays ON pays.id = p.pays_id
         WHERE p.deleted_at IS NULL
           AND p.etat::text IN ('approuve', 'en_cours', 'termine')
           AND ($1::VARCHAR IS NULL OR pays.nom = $1)
           AND ($2::VARCHAR IS NULL OR (
               LOWER(p.titre) LIKE $2
               OR LOWER(p.description) LIKE $2
               OR LOWER(COALESCE(p.nom_organisation, '')) LIKE $2
           )){}",
        conditions_extra
    );

    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query)
        .bind(&pays_filtre)
        .bind(&terme_recherche);

    if let Some(budget_max) = params.budget_max {
        if budget_max > 0.0 {
            count_q = count_q.bind(budget_max);
        }
    }

    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // ── Requete principale ─────────────────────────────────
    let query = format!(
        "SELECT {colonnes}
         FROM innovation.projet p
         LEFT JOIN shared.pays pays ON pays.id = p.pays_id
         JOIN iam.utilisateur u ON u.id = p.cree_par
         WHERE p.deleted_at IS NULL
           AND p.etat::text IN ('approuve', 'en_cours', 'termine')
           AND ($1::VARCHAR IS NULL OR pays.nom = $1)
           AND ($2::VARCHAR IS NULL OR (
               LOWER(p.titre) LIKE $2
               OR LOWER(p.description) LIKE $2
               OR LOWER(COALESCE(p.nom_organisation, '')) LIKE $2
           )){extra}
         {order}
         LIMIT $3 OFFSET ${offset_idx}",
        colonnes = PROJET_LISTE_COLONNES,
        extra = conditions_extra,
        order = order_clause,
        offset_idx = if params.budget_max.map_or(false, |b| b > 0.0) { 5 } else { 4 },
    );

    let mut main_q = sqlx::query_as::<_, ProjetListeRow>(&query)
        .bind(&pays_filtre)
        .bind(&terme_recherche)
        .bind(par_page);

    if let Some(budget_max) = params.budget_max {
        if budget_max > 0.0 {
            main_q = main_q.bind(budget_max);
        }
    }

    let rows = main_q.bind(offset).fetch_all(pool.get_ref()).await?;

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    let reponse = ProjetListeResponse {
        projets: rows.iter().map(|r| r.to_response()).collect(),
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
// GET /api/projets/{id} : Obtenir le detail d'un projet
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_projet(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {}
         FROM innovation.projet p
         LEFT JOIN shared.pays pays ON pays.id = p.pays_id
         JOIN iam.utilisateur u ON u.id = p.cree_par
         WHERE p.id = $1 AND p.deleted_at IS NULL",
        PROJET_DETAIL_COLONNES
    );

    let projet = sqlx::query_as::<_, ProjetDetailRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Projet avec id {} non trouve", id)))?;

    // Charger les documents associes
    let documents = sqlx::query_as::<_, ProjetDocumentResponse>(
        "SELECT id, nom, url, type_mime
         FROM innovation.projet_document
         WHERE projet_id = $1
         ORDER BY created_at ASC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let reponse = projet.to_detail_response(documents);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/projets : Creer un projet (multipart)
// ──────────────────────────────────────────────────────────────
pub async fn creer_projet(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Token invalide ou manquant".into()))?;

    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut objectifs: Option<String> = None;
    let mut nom_organisation: Option<String> = None;
    let mut description_organisation: Option<String> = None;
    let mut site_web: Option<String> = None;
    let mut pays: Option<String> = None;
    let mut ville: Option<String> = None;
    let mut contact_email: Option<String> = None;
    let mut contact_telephone: Option<String> = None;
    let mut cout_total: Option<f64> = None;
    let mut devise: Option<String> = None;
    let mut duree_mois: Option<i32> = None;
    let mut date_commencement: Option<String> = None;
    let mut resultats_attendus: Option<String> = None;
    let mut activites_programmees: Option<String> = None;
    let mut echeanciers: Option<String> = None;
    let mut contribution_autonomisation: Option<String> = None;
    let mut difficultes_risques: Option<String> = None;
    let mut image_url: Option<String> = None;

    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let couvertures_dir = format!("{}/couvertures", upload_dir);
    std::fs::create_dir_all(&couvertures_dir)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le dossier uploads: {}", e)))?;

    while let Some(Ok(mut field)) = payload.next().await {
        let field_name = field.name().map(|n| n.to_string()).unwrap_or_default();

        match field_name.as_str() {
            "titre" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                titre = Some(String::from_utf8_lossy(&data).trim().to_string());
            }
            "description" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                description = Some(String::from_utf8_lossy(&data).trim().to_string());
            }
            "objectifs" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                objectifs = Some(String::from_utf8_lossy(&data).trim().to_string());
            }
            "nom_organisation" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { nom_organisation = Some(val); }
            }
            "description_organisation" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { description_organisation = Some(val); }
            }
            "site_web" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { site_web = Some(val); }
            }
            "pays" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { pays = Some(val); }
            }
            "ville" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { ville = Some(val); }
            }
            "contact_email" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { contact_email = Some(val); }
            }
            "contact_telephone" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { contact_telephone = Some(val); }
            }
            "cout_total" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                cout_total = val.parse::<f64>().ok();
            }
            "devise" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { devise = Some(val); }
            }
            "duree_mois" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                duree_mois = val.parse::<i32>().ok();
            }
            "date_commencement_souhaitee" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { date_commencement = Some(val); }
            }
            "resultats_attendus" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { resultats_attendus = Some(val); }
            }
            "activites_programmees" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { activites_programmees = Some(val); }
            }
            "echeanciers" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { echeanciers = Some(val); }
            }
            "contribution_autonomisation" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { contribution_autonomisation = Some(val); }
            }
            "difficultes_risques" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                let val = String::from_utf8_lossy(&data).trim().to_string();
                if !val.is_empty() { difficultes_risques = Some(val); }
            }
            "couverture" => {
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));

                let ext = std::path::Path::new(&filename)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("jpg");
                let safe_name = format!("{}.{}", Uuid::new_v4(), ext);
                let filepath = format!("{}/{}", couvertures_dir, safe_name);

                let mut file = std::fs::File::create(&filepath)
                    .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture fichier: {}", e)))?;

                while let Some(Ok(chunk)) = field.next().await {
                    file.write_all(&chunk)
                        .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture: {}", e)))?;
                }

                image_url = Some(format!("/uploads/couvertures/{}", safe_name));
            }
            _ => {
                while let Some(Ok(_)) = field.next().await {}
            }
        }
    }

    // Validation des champs requis
    let titre = titre
        .filter(|t| !t.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le titre est requis".into()))?;
    let description = description
        .filter(|d| !d.is_empty())
        .ok_or_else(|| ApiErreur::Validation("La description est requise".into()))?;
    let objectifs_text = objectifs
        .filter(|o| !o.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Les objectifs sont requis".into()))?;

    let slug = generer_slug(&titre);

    // Formater les objectifs s'ils sont fournis comme JSON array
    let objectifs_stockage = if objectifs_text.starts_with('[') {
        // Tenter de parser comme JSON array
        if let Ok(arr) = serde_json::from_str::<Vec<String>>(&objectifs_text) {
            joindre_objectifs(&arr)
        } else {
            objectifs_text
        }
    } else {
        objectifs_text
    };

    // Resoudre pays_id par nom
    let pays_id: Option<Uuid> = if let Some(ref p) = pays {
        sqlx::query_scalar("SELECT id FROM shared.pays WHERE nom = $1 LIMIT 1")
            .bind(p)
            .fetch_optional(pool.get_ref())
            .await?
    } else {
        None
    };

    // Parser la date de commencement
    let date_debut: Option<chrono::NaiveDate> = date_commencement.and_then(|d| {
        chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()
    });

    // Inserer le projet
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO innovation.projet
            (titre, slug, description, objectifs, nom_organisation, description_organisation,
             site_web, pays_id, ville, contact_email, contact_telephone,
             cout_total, devise, duree_mois, date_commencement_souhaitee,
             resultats_attendus, activites_programmees, echeanciers,
             contribution_autonomisation, difficultes_risques, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
         RETURNING id",
    )
    .bind(&titre)
    .bind(&slug)
    .bind(&description)
    .bind(&objectifs_stockage)
    .bind(&nom_organisation)
    .bind(&description_organisation)
    .bind(&site_web)
    .bind(pays_id)
    .bind(&ville)
    .bind(&contact_email)
    .bind(&contact_telephone)
    .bind(cout_total)
    .bind(devise.as_deref().unwrap_or("XOF"))
    .bind(duree_mois)
    .bind(date_debut)
    .bind(&resultats_attendus)
    .bind(&activites_programmees)
    .bind(&echeanciers)
    .bind(&contribution_autonomisation)
    .bind(&difficultes_risques)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Inserer l'image de couverture comme document si presente
    if let Some(ref url) = image_url {
        sqlx::query(
            "INSERT INTO innovation.projet_document (projet_id, nom, url, type_mime)
             VALUES ($1, 'couverture', $2, 'image/jpeg')",
        )
        .bind(id)
        .bind(url)
        .execute(pool.get_ref())
        .await?;
    }

    // Charger la row inseree pour la reponse
    let query = format!(
        "SELECT {}
         FROM innovation.projet p
         LEFT JOIN shared.pays pays ON pays.id = p.pays_id
         JOIN iam.utilisateur u ON u.id = p.cree_par
         WHERE p.id = $1",
        PROJET_DETAIL_COLONNES
    );

    let row = sqlx::query_as::<_, ProjetDetailRow>(&query)
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?;

    let documents = sqlx::query_as::<_, ProjetDocumentResponse>(
        "SELECT id, nom, url, type_mime
         FROM innovation.projet_document
         WHERE projet_id = $1
         ORDER BY created_at ASC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_detail_response(documents)),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/projets/statistiques : Statistiques des projets
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_statistiques(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let stats = sqlx::query_as::<_, (i64, i64, i64, i64)>(
        "SELECT
            COUNT(*) FILTER (WHERE etat::text IN ('approuve','en_cours','termine','soumis','en_revue')) AS total,
            COUNT(*) FILTER (WHERE etat::text = 'approuve') AS valides,
            COUNT(*) FILTER (WHERE etat::text = 'en_cours') AS en_cours,
            COUNT(*) FILTER (WHERE etat::text = 'termine') AS termines
         FROM innovation.projet
         WHERE deleted_at IS NULL",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let reponse = ProjetStatistiquesResponse {
        total: stats.0,
        valides: stats.1,
        en_cours: stats.2,
        termines: stats.3,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

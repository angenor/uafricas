use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::expert::{
    CandidatureExpertBody, ExpertListeResponse, ExpertQueryParams,
    ExpertRow, MaCandidatureRow, NoterExpertBody, NoteExpertResponse,
    EXPERT_COLONNES, mapper_domaine_db, OBJECTIFS_VALIDES,
};

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

// ────────────────────────────────────────────────────────────────
// Endpoints
// ────────────────────────────────────────────────────────────────

/// GET /api/experts — Liste paginee des experts valides avec filtres
pub async fn lister_experts(
    pool: web::Data<PgPool>,
    params: web::Query<ExpertQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Construction dynamique du WHERE
    let mut conditions: Vec<String> = vec![
        "e.statut = 'valide'".to_string(),
        "e.deleted_at IS NULL".to_string(),
        "u.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par domaine
    if let Some(ref domaine) = params.domaine {
        let trimmed = domaine.trim();
        if !trimmed.is_empty() && trimmed != "Tout" {
            let domaine_db = mapper_domaine_db(trimmed);
            conditions.push(format!("e.domaine::text = ${}", bind_index));
            bind_values.push(domaine_db);
            bind_index += 1;
        }
    }

    // Filtre par pays
    if let Some(ref pays) = params.pays {
        let trimmed = pays.trim();
        if !trimmed.is_empty() {
            conditions.push(format!("LOWER(p.nom) = LOWER(${})", bind_index));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    // Filtre par situation professionnelle (match dans le tableau PostgreSQL)
    if let Some(ref situation) = params.situation {
        let trimmed = situation.trim();
        if !trimmed.is_empty() && trimmed != "tous" {
            conditions.push(format!(
                "${}::text = ANY(e.situations_professionnelles::text[])",
                bind_index
            ));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    // Recherche textuelle (nom, prenom, biographie, domaine)
    if let Some(ref recherche) = params.recherche {
        let trimmed = recherche.trim();
        if !trimmed.is_empty() {
            let terme = format!("%{}%", trimmed.to_lowercase());
            conditions.push(format!(
                "(LOWER(u.nom) LIKE ${idx} OR LOWER(u.prenom) LIKE ${idx} \
                 OR LOWER(e.biographie) LIKE ${idx} OR LOWER(e.domaine::text) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Clause ORDER BY
    let order_clause = match params.tri.as_deref() {
        Some("experience") => "ORDER BY e.nb_annees_experience DESC, e.updated_at DESC",
        Some("rating") => "ORDER BY e.rating DESC, e.updated_at DESC",
        _ => "ORDER BY e.updated_at DESC",
    };

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les experts
    let select_query = format!(
        "SELECT {} FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE {} {} LIMIT ${} OFFSET ${}",
        EXPERT_COLONNES, where_clause, order_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, ExpertRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    let experts: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ExpertListeResponse {
            experts,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/experts/{id} — Detail d'un expert par son utilisateur_id.
/// Le JWT est optionnel : s'il est présent, `maNote` reflète la note de l'appelant.
pub async fn obtenir_expert(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE e.utilisateur_id = $1 AND e.deleted_at IS NULL AND u.deleted_at IS NULL",
        EXPERT_COLONNES
    );

    let row = sqlx::query_as::<_, ExpertRow>(&query)
        .bind(utilisateur_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| {
            ApiErreur::NonTrouve(format!("Expert avec id {} non trouve", utilisateur_id))
        })?;

    let mut response = row.to_response();

    // Note du membre connecté (le cas échéant)
    if let Some(auteur_id) = extraire_utilisateur_id(&req) {
        let ma_note: Option<i16> = sqlx::query_scalar(
            "SELECT note FROM iam.note_expertise WHERE expertise_id = $1 AND auteur_id = $2",
        )
        .bind(row.expertise_id)
        .bind(auteur_id)
        .fetch_optional(pool.get_ref())
        .await?;
        response.expertise_info.ma_note = ma_note;
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

/// POST /api/experts/{id}/note — Noter un expert (1–5, JWT requis).
/// Upsert d'une note par membre, recalcule la moyenne `iam.expertise.rating`.
pub async fn noter_expert(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<NoterExpertBody>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;
    let utilisateur_id = chemin.into_inner();

    if !(1..=5).contains(&body.note) {
        return Err(ApiErreur::Validation(
            "La note doit être comprise entre 1 et 5".to_string(),
        ));
    }

    if auteur_id == utilisateur_id {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas noter votre propre profil".to_string(),
        ));
    }

    // L'expertise doit exister et être validée
    let expertise_id: Uuid = sqlx::query_scalar(
        "SELECT id FROM iam.expertise
         WHERE utilisateur_id = $1 AND deleted_at IS NULL AND statut = 'valide'",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Expert non trouvé".to_string()))?;

    let mut tx = pool.begin().await?;

    // Upsert de la note
    sqlx::query(
        "INSERT INTO iam.note_expertise (expertise_id, auteur_id, note)
         VALUES ($1, $2, $3)
         ON CONFLICT (expertise_id, auteur_id)
         DO UPDATE SET note = EXCLUDED.note, updated_at = NOW()",
    )
    .bind(expertise_id)
    .bind(auteur_id)
    .bind(body.note)
    .execute(&mut *tx)
    .await?;

    // Recalcul de la moyenne (arrondie au dixième, contrainte rating 0–5)
    let rating: f64 = sqlx::query_scalar(
        "UPDATE iam.expertise
         SET rating = COALESCE(
                 (SELECT ROUND(AVG(note)::numeric, 1)
                  FROM iam.note_expertise WHERE expertise_id = $1),
                 0.0),
             updated_at = NOW()
         WHERE id = $1
         RETURNING rating::float8",
    )
    .bind(expertise_id)
    .fetch_one(&mut *tx)
    .await?;

    let nombre_notes: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM iam.note_expertise WHERE expertise_id = $1",
    )
    .bind(expertise_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(NoteExpertResponse {
            rating,
            nombre_notes,
            ma_note: body.note,
        }),
        error: None,
    }))
}

/// POST /api/experts/candidature — Soumettre une candidature expert (JWT requis)
pub async fn creer_candidature(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CandidatureExpertBody>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    // Verifier que l'utilisateur existe
    let user_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !user_exists {
        return Err(ApiErreur::NonTrouve(
            "Utilisateur non trouve".to_string(),
        ));
    }

    // Etat de la candidature active eventuelle (FR-006 + FR-015)
    let statut_actif: Option<String> = sqlx::query_scalar(
        "SELECT statut::text FROM iam.expertise
         WHERE utilisateur_id = $1 AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    // Bloquer si une demande active en_attente ou valide existe deja
    if let Some(ref statut) = statut_actif {
        if statut == "en_attente" || statut == "valide" {
            return Err(ApiErreur::Conflit(
                "Vous avez deja une demande d'expertise active".to_string(),
            ));
        }
    }

    // Mapper le domaine
    let domaine_db = mapper_domaine_db(&body.domaine);

    // Précision libre obligatoire (et conservée) uniquement quand domaine = "autre"
    let domaine_autre: Option<String> = if domaine_db == "autre" {
        let precision = body
            .domaine_autre
            .as_deref()
            .map(str::trim)
            .filter(|p| !p.is_empty());
        match precision {
            Some(p) => Some(p.to_string()),
            None => {
                return Err(ApiErreur::Validation(
                    "Veuillez préciser votre domaine d'expertise".to_string(),
                ))
            }
        }
    } else {
        None
    };

    // Nettoyer / valider les champs additionnels
    let nettoyer_liste = |valeurs: &[String]| -> Vec<String> {
        valeurs
            .iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };
    let specialites = nettoyer_liste(&body.specialites);
    let realisations = nettoyer_liste(&body.realisations);
    let objectifs: Vec<String> = body
        .objectifs
        .iter()
        .map(|o| o.trim().to_lowercase())
        .filter(|o| OBJECTIFS_VALIDES.contains(&o.as_str()))
        .collect();
    let linkedin_url = body
        .linkedin_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from);
    let cv_url = body
        .cv_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from);

    // Transaction : archiver une eventuelle demande refusee puis inserer la nouvelle
    let mut tx = pool.begin().await?;

    if statut_actif.as_deref() == Some("refuse") {
        sqlx::query(
            "UPDATE iam.expertise SET deleted_at = NOW(), updated_at = NOW()
             WHERE utilisateur_id = $1 AND deleted_at IS NULL",
        )
        .bind(utilisateur_id)
        .execute(&mut *tx)
        .await?;
    }

    let expertise_id: Uuid = sqlx::query_scalar(
        "INSERT INTO iam.expertise
            (utilisateur_id, domaine, domaine_autre, biographie, nb_annees_experience,
             portfolio, linkedin_url, cv_url, specialites, objectifs, realisations,
             situations_professionnelles, statut)
         VALUES ($1, $2::iam.domaine_expertise, $3, $4, $5,
                 $6, $7, $8, $9, $10::iam.objectif_expertise[], $11,
                 $12::iam.situation_professionnelle[], 'en_attente')
         RETURNING id",
    )
    .bind(utilisateur_id)
    .bind(&domaine_db)
    .bind(&domaine_autre)
    .bind(&body.biographie)
    .bind(body.nb_annees_experience)
    .bind(&body.portfolio)
    .bind(&linkedin_url)
    .bind(&cv_url)
    .bind(&specialites)
    .bind(&objectifs)
    .bind(&realisations)
    .bind(&body.situations_professionnelles)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    log::info!(
        "Candidature expert creee: {} pour utilisateur {}",
        expertise_id,
        utilisateur_id
    );

    // Recuperer la candidature creee pour la retourner
    let row_query = format!(
        "SELECT {} FROM iam.expertise e
         JOIN iam.utilisateur u ON u.id = e.utilisateur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE e.id = $1",
        EXPERT_COLONNES
    );

    let row = sqlx::query_as::<_, ExpertRow>(&row_query)
        .bind(expertise_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// GET /api/experts/moi — Candidature active du membre connecte (suivi US3)
pub async fn ma_candidature(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let row = sqlx::query_as::<_, MaCandidatureRow>(
        "SELECT id, domaine::text AS domaine, domaine_autre, biographie, nb_annees_experience,
                portfolio, linkedin_url, cv_url, specialites,
                objectifs::text[] AS objectifs, realisations,
                situations_professionnelles::text[] AS situations_professionnelles,
                statut::text AS statut, commentaire_admin, date_validation, created_at
         FROM iam.expertise
         WHERE utilisateur_id = $1 AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: row.map(|r| r.to_response()),
        error: None,
    }))
}

/// POST /api/experts/cv — Uploader un CV (PDF) pour une candidature d'expertise.
/// Retourne l'URL à inclure dans le corps de `creer_candidature`.
pub async fn uploader_cv(
    req: HttpRequest,
    upload_dir: web::Data<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let mut cv_url: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|e| ApiErreur::Upload(format!("Erreur lecture multipart: {}", e)))?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        if nom_champ != "cv" && nom_champ != "fichier" && nom_champ != "file" {
            continue;
        }

        let nom_original = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_filename().map(sanitize_filename::sanitize))
            .unwrap_or_else(|| format!("{}.pdf", Uuid::new_v4()));
        let extension = nom_original.rsplit('.').next().unwrap_or("pdf").to_lowercase();
        if extension != "pdf" {
            return Err(ApiErreur::Validation(
                "Le CV doit être un fichier PDF".into(),
            ));
        }

        let nom_fichier = format!("{}.pdf", Uuid::new_v4());
        let dossier = format!("{}/cv-experts", upload_dir.get_ref());
        std::fs::create_dir_all(&dossier)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le repertoire: {}", e)))?;
        let chemin_complet = format!("{}/{}", dossier, nom_fichier);
        let mut fichier = std::fs::File::create(&chemin_complet)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le fichier: {}", e)))?;

        let mut taille_totale: usize = 0;
        let mut premier_chunk = true;
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture: {}", e)))?;
            // Vérifier la signature PDF (%PDF) sur le premier morceau
            if premier_chunk {
                premier_chunk = false;
                if data.len() < 4 || &data[0..4] != b"%PDF" {
                    let _ = std::fs::remove_file(&chemin_complet);
                    return Err(ApiErreur::Validation(
                        "Le fichier n'est pas un PDF valide".into(),
                    ));
                }
            }
            taille_totale += data.len();
            if taille_totale > 10 * 1024 * 1024 {
                let _ = std::fs::remove_file(&chemin_complet);
                return Err(ApiErreur::Validation(
                    "Le CV ne doit pas depasser 10 Mo".into(),
                ));
            }
            fichier
                .write_all(&data)
                .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture: {}", e)))?;
        }

        cv_url = Some(format!("/uploads/cv-experts/{}", nom_fichier));
    }

    let url = cv_url.ok_or_else(|| ApiErreur::Validation("Aucun fichier CV recu".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "cv_url": url })),
        error: None,
    }))
}

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::formation_contenu::{
    ChapitrePublicResponse, ChapitreRow, FormationContenuResponse, LeconPublicResponse,
    LeconRow, ProgressionResponse,
};
use crate::models::mooc::{
    FormateurInfo, FormateurResponse, MoocDetailResponse, MoocListeResponse,
    MoocQueryParams, MoocResponse, MoocRow, MOOC_COLONNES, calculer_statut_mooc,
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

/// Charger le formateur depuis iam.utilisateur
async fn charger_formateur(pool: &PgPool, utilisateur_id: Uuid) -> Result<FormateurInfo, ApiErreur> {
    sqlx::query_as::<_, FormateurInfo>(
        "SELECT id, nom, prenom, email, photo_url FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Formateur non trouve".to_string()))
}

/// Compter les inscrits a un MOOC
async fn compter_inscrits(pool: &PgPool, mooc_id: Uuid) -> Result<i64, ApiErreur> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc_inscription
         WHERE mooc_id = $1 AND statut != 'abandonne'",
    )
    .bind(mooc_id)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Verifier si un utilisateur est inscrit a un MOOC
async fn est_inscrit(pool: &PgPool, mooc_id: Uuid, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.mooc_inscription
            WHERE mooc_id = $1 AND utilisateur_id = $2 AND statut != 'abandonne'
        )",
    )
    .bind(mooc_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

/// Recuperer le nom du pays
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

/// Construire un MoocResponse a partir d'un MoocRow
async fn construire_response(
    pool: &PgPool,
    row: &MoocRow,
) -> Result<MoocResponse, ApiErreur> {
    let formateur = charger_formateur(pool, row.cree_par).await?;
    let nombre_inscrits = compter_inscrits(pool, row.id).await?;

    Ok(MoocResponse {
        id: row.id,
        titre: row.titre.clone(),
        description: row.description.clone(),
        type_formation: row.type_formation.clone(),
        langue: row.langue.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        statut: calculer_statut_mooc(
            &row.etat,
            &row.date_heure_debut,
            row.date_heure_fin.as_ref(),
            nombre_inscrits,
            row.nombre_places,
        ),
        nombre_places: row.nombre_places,
        nombre_inscrits,
        formateur: FormateurResponse {
            uid: formateur.id,
            nom: formateur.nom,
            prenom: formateur.prenom,
            email: String::new(), // e-mail de compte non exposé publiquement (audit #26)
            photo_url: formateur.photo_url,
        },
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

// ──────────────────────────────────────────────────────────────
// GET /api/moocs — Lister les formations avec filtres et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_moocs(
    pool: web::Data<PgPool>,
    params: web::Query<MoocQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Construire les conditions WHERE dynamiquement
    let mut conditions: Vec<String> = vec![
        "m.etat IN ('publie','en_cours','termine')".to_string(),
        "m.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref type_f) = params.type_formation {
        if !type_f.is_empty() {
            conditions.push(format!("LOWER(m.type) = LOWER(${})", bind_index));
            bind_values.push(type_f.clone());
            bind_index += 1;
        }
    }

    if let Some(ref recherche) = params.recherche {
        if !recherche.is_empty() {
            let terme = format!("%{}%", recherche.to_lowercase());
            conditions.push(format!(
                "(LOWER(m.titre) LIKE ${idx} OR LOWER(m.description) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM media_content.mooc m WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les MOOCs
    let select_query = format!(
        "SELECT {} FROM media_content.mooc m WHERE {} ORDER BY m.date_heure_debut DESC LIMIT ${} OFFSET ${}",
        MOOC_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, MoocRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    // Construire les reponses
    let mut formations = Vec::with_capacity(rows.len());
    for row in &rows {
        formations.push(construire_response(pool.get_ref(), row).await?);
    }

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MoocListeResponse {
            formations,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/moocs/{id} — Obtenir le detail d'un MOOC
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_mooc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let current_user = extraire_utilisateur_id(&req);
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM media_content.mooc m
         WHERE m.id = $1 AND m.deleted_at IS NULL",
        MOOC_COLONNES
    );

    let row = sqlx::query_as::<_, MoocRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Formation avec id {} non trouvee", id)))?;

    let formateur = charger_formateur(pool.get_ref(), row.cree_par).await?;
    let nombre_inscrits = compter_inscrits(pool.get_ref(), row.id).await?;
    let pays_nom = charger_nom_pays(pool.get_ref(), row.pays_id).await?;

    let inscrit = if let Some(uid) = current_user {
        est_inscrit(pool.get_ref(), id, uid).await?
    } else {
        false
    };

    let reponse = MoocDetailResponse {
        id: row.id,
        titre: row.titre.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        type_formation: row.type_formation.clone(),
        pays: pays_nom,
        ville: row.ville.clone(),
        langue: row.langue.clone(),
        format: row.format.clone(),
        date_heure_debut: row.date_heure_debut,
        date_heure_fin: row.date_heure_fin,
        couverture_url: row.image_couverture_url.clone(),
        lien_en_ligne: row.lien_en_ligne.clone(),
        statut: calculer_statut_mooc(
            &row.etat,
            &row.date_heure_debut,
            row.date_heure_fin.as_ref(),
            nombre_inscrits,
            row.nombre_places,
        ),
        nombre_places: row.nombre_places,
        nombre_inscrits,
        prerequis: row.prerequis.clone(),
        est_inscrit: inscrit,
        formateur: FormateurResponse {
            uid: formateur.id,
            nom: formateur.nom,
            prenom: formateur.prenom,
            email: String::new(), // e-mail de compte non exposé publiquement (audit #26)
            photo_url: formateur.photo_url,
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
// POST /api/moocs/{id}/inscription — S'inscrire a un MOOC
// ──────────────────────────────────────────────────────────────
pub async fn inscrire_mooc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let mooc_id = chemin.into_inner();

    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    // Verifier que le MOOC existe et est publie
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.mooc
            WHERE id = $1 AND etat = 'publie' AND deleted_at IS NULL
        )",
    )
    .bind(mooc_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Formation non trouvee".to_string()));
    }

    // Verifier la capacite
    let row = sqlx::query_as::<_, (Option<i32>,)>(
        "SELECT nombre_places FROM media_content.mooc WHERE id = $1",
    )
    .bind(mooc_id)
    .fetch_one(pool.get_ref())
    .await?;

    if let Some(max_places) = row.0 {
        let inscrits = compter_inscrits(pool.get_ref(), mooc_id).await?;
        if inscrits >= max_places as i64 {
            return Err(ApiErreur::Validation("La formation est complete".to_string()));
        }
    }

    // Inserer l'inscription (ON CONFLICT pour eviter les doublons)
    sqlx::query(
        "INSERT INTO media_content.mooc_inscription (mooc_id, utilisateur_id, statut)
         VALUES ($1, $2, 'inscrit')
         ON CONFLICT (mooc_id, utilisateur_id) DO UPDATE SET statut = 'inscrit', updated_at = NOW()",
    )
    .bind(mooc_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Inscription MOOC {} par utilisateur {}", mooc_id, utilisateur_id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// Programme : chapitres & leçons
// ──────────────────────────────────────────────────────────────

/// Recalculer la progression d'un inscrit et la persister sur son inscription.
/// Renvoie (progression %, nombre_lecons_total, nombre_lecons_terminees).
async fn recalculer_progression(
    pool: &PgPool,
    mooc_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<(f64, i64, i64), ApiErreur> {
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.formation_lecon l
         JOIN media_content.formation_chapitre c ON l.chapitre_id = c.id
         WHERE c.mooc_id = $1 AND c.deleted_at IS NULL AND l.deleted_at IS NULL",
    )
    .bind(mooc_id)
    .fetch_one(pool)
    .await?;

    let done: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.formation_lecon_completion lc
         JOIN media_content.formation_lecon l ON lc.lecon_id = l.id
         JOIN media_content.formation_chapitre c ON l.chapitre_id = c.id
         WHERE c.mooc_id = $1 AND lc.utilisateur_id = $2
           AND c.deleted_at IS NULL AND l.deleted_at IS NULL",
    )
    .bind(mooc_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;

    let progression = if total > 0 {
        ((done as f64 / total as f64) * 10000.0).round() / 100.0
    } else {
        0.0
    };

    // La progression est une colonne DECIMAL(5,2) : on l'inscrit comme littéral
    // numérique formaté (valeur calculée par nos soins, aucune injection possible)
    // pour éviter les soucis d'encodage f64 -> NUMERIC.
    let sql = format!(
        "UPDATE media_content.mooc_inscription
         SET progression = {p:.2},
             statut = CASE WHEN {p:.2} >= 100 THEN 'complete'
                           WHEN $1 > 0 THEN 'en_cours'
                           ELSE 'inscrit' END,
             updated_at = NOW()
         WHERE mooc_id = $2 AND utilisateur_id = $3 AND statut <> 'abandonne'",
        p = progression
    );
    sqlx::query(&sql)
        .bind(done)
        .bind(mooc_id)
        .bind(utilisateur_id)
        .execute(pool)
        .await?;

    Ok((progression, total, done))
}

/// Vérifier qu'une leçon appartient bien à une formation (et n'est pas supprimée).
async fn lecon_appartient_au_mooc(
    pool: &PgPool,
    lecon_id: Uuid,
    mooc_id: Uuid,
) -> Result<bool, ApiErreur> {
    Ok(sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.formation_lecon l
            JOIN media_content.formation_chapitre c ON l.chapitre_id = c.id
            WHERE l.id = $1 AND c.mooc_id = $2
              AND l.deleted_at IS NULL AND c.deleted_at IS NULL
        )",
    )
    .bind(lecon_id)
    .bind(mooc_id)
    .fetch_one(pool)
    .await?)
}

// ──────────────────────────────────────────────────────────────
// GET /api/moocs/{id}/contenu — Programme (chapitres + leçons)
// La structure est publique ; le contenu des leçons n'est servi qu'aux inscrits
// (ou au créateur de la formation).
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_contenu_mooc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let current_user = extraire_utilisateur_id(&req);
    let mooc_id = chemin.into_inner();

    // Récupérer le créateur + l'état (et vérifier l'existence)
    let (cree_par, etat): (Uuid, String) = sqlx::query_as(
        "SELECT cree_par, etat FROM media_content.mooc WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(mooc_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Formation non trouvee".to_string()))?;

    // La structure n'est publique que pour les formations publiées ; brouillon/annulée/
    // suspendue restent réservées à leur créateur (renvoie NonTrouve aux tiers — ne
    // révèle pas l'existence).
    let etat_public = matches!(etat.as_str(), "publie" | "en_cours" | "termine");
    if !etat_public && current_user != Some(cree_par) {
        return Err(ApiErreur::NonTrouve("Formation non trouvee".to_string()));
    }

    let inscrit = if let Some(uid) = current_user {
        est_inscrit(pool.get_ref(), mooc_id, uid).await?
    } else {
        false
    };
    let accessible = inscrit || current_user == Some(cree_par);

    // Chapitres
    let chapitres = sqlx::query_as::<_, ChapitreRow>(
        "SELECT id, titre, description, ordre FROM media_content.formation_chapitre
         WHERE mooc_id = $1 AND deleted_at IS NULL
         ORDER BY ordre ASC, created_at ASC",
    )
    .bind(mooc_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Leçons (toutes celles des chapitres non supprimés)
    let lecons = sqlx::query_as::<_, LeconRow>(
        "SELECT l.id, l.chapitre_id, l.titre, l.contenu, l.video_url,
                l.document_url, l.duree_minutes, l.ordre
         FROM media_content.formation_lecon l
         JOIN media_content.formation_chapitre c ON l.chapitre_id = c.id
         WHERE c.mooc_id = $1 AND c.deleted_at IS NULL AND l.deleted_at IS NULL
         ORDER BY l.ordre ASC, l.created_at ASC",
    )
    .bind(mooc_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Leçons terminées par l'utilisateur courant
    let terminees: HashSet<Uuid> = if let Some(uid) = current_user {
        let rows: Vec<Uuid> = sqlx::query_scalar(
            "SELECT lc.lecon_id FROM media_content.formation_lecon_completion lc
             JOIN media_content.formation_lecon l ON lc.lecon_id = l.id
             JOIN media_content.formation_chapitre c ON l.chapitre_id = c.id
             WHERE c.mooc_id = $1 AND lc.utilisateur_id = $2",
        )
        .bind(mooc_id)
        .bind(uid)
        .fetch_all(pool.get_ref())
        .await?;
        rows.into_iter().collect()
    } else {
        HashSet::new()
    };

    let nombre_lecons = lecons.len() as i64;
    let nombre_lecons_terminees =
        lecons.iter().filter(|l| terminees.contains(&l.id)).count() as i64;
    let progression = if nombre_lecons > 0 {
        ((nombre_lecons_terminees as f64 / nombre_lecons as f64) * 10000.0).round() / 100.0
    } else {
        0.0
    };

    // NB gel d'accès : on masque ici l'URL du contenu (contenu/video_url/document_url)
    // aux non-inscrits. Pour les fichiers UPLOADÉS, ils restent servis par actix-files
    // sous /uploads/ sans authentification : le gel ne protège donc que la DÉCOUVERTE de
    // l'URL (UUID aléatoire), pas le fichier lui-même. Suffisant pour le MVP (les vidéos
    // sont principalement des liens YouTube/externes, publics par nature). Pour un vrai
    // huis clos : stocker hors du dossier statique + servir via un endpoint authentifié
    // à jeton signé (les balises <video>/<embed> n'envoient pas d'en-tête Authorization).
    let chapitres_resp: Vec<ChapitrePublicResponse> = chapitres
        .iter()
        .map(|ch| {
            let lecons_resp: Vec<LeconPublicResponse> = lecons
                .iter()
                .filter(|l| l.chapitre_id == ch.id)
                .map(|l| {
                    let a_video = l.video_url.as_deref().map(|s| !s.is_empty()).unwrap_or(false);
                    let a_document =
                        l.document_url.as_deref().map(|s| !s.is_empty()).unwrap_or(false);
                    LeconPublicResponse {
                        id: l.id,
                        titre: l.titre.clone(),
                        duree_minutes: l.duree_minutes,
                        ordre: l.ordre,
                        accessible,
                        a_video,
                        a_document,
                        contenu: if accessible { l.contenu.clone() } else { None },
                        video_url: if accessible { l.video_url.clone() } else { None },
                        document_url: if accessible { l.document_url.clone() } else { None },
                        terminee: terminees.contains(&l.id),
                    }
                })
                .collect();
            ChapitrePublicResponse {
                id: ch.id,
                titre: ch.titre.clone(),
                description: ch.description.clone(),
                ordre: ch.ordre,
                lecons: lecons_resp,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(FormationContenuResponse {
            est_inscrit: inscrit,
            accessible,
            nombre_lecons,
            nombre_lecons_terminees,
            progression,
            chapitres: chapitres_resp,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/moocs/{id}/lecons/{lecon_id}/completion — Marquer terminée
// ──────────────────────────────────────────────────────────────
pub async fn marquer_lecon_terminee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (mooc_id, lecon_id) = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    if !est_inscrit(pool.get_ref(), mooc_id, utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise(
            "Vous devez être inscrit à cette formation".to_string(),
        ));
    }
    if !lecon_appartient_au_mooc(pool.get_ref(), lecon_id, mooc_id).await? {
        return Err(ApiErreur::NonTrouve("Leçon non trouvée".to_string()));
    }

    sqlx::query(
        "INSERT INTO media_content.formation_lecon_completion (lecon_id, utilisateur_id)
         VALUES ($1, $2) ON CONFLICT (lecon_id, utilisateur_id) DO NOTHING",
    )
    .bind(lecon_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    let (progression, total, done) =
        recalculer_progression(pool.get_ref(), mooc_id, utilisateur_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ProgressionResponse {
            progression,
            nombre_lecons: total,
            nombre_lecons_terminees: done,
            terminee: true,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// DELETE /api/moocs/{id}/lecons/{lecon_id}/completion — Annuler
// ──────────────────────────────────────────────────────────────
pub async fn annuler_lecon_terminee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (mooc_id, lecon_id) = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    if !est_inscrit(pool.get_ref(), mooc_id, utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise(
            "Vous devez être inscrit à cette formation".to_string(),
        ));
    }
    if !lecon_appartient_au_mooc(pool.get_ref(), lecon_id, mooc_id).await? {
        return Err(ApiErreur::NonTrouve("Leçon non trouvée".to_string()));
    }

    sqlx::query(
        "DELETE FROM media_content.formation_lecon_completion
         WHERE lecon_id = $1 AND utilisateur_id = $2",
    )
    .bind(lecon_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    let (progression, total, done) =
        recalculer_progression(pool.get_ref(), mooc_id, utilisateur_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ProgressionResponse {
            progression,
            nombre_lecons: total,
            nombre_lecons_terminees: done,
            terminee: false,
        }),
        error: None,
    }))
}

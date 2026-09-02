use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::bibliotheque_humaine::{
    BiblioHumaineListeResponse, BiblioHumaineQueryParams, BiblioHumaineRow,
    CommentaireBiblioRequest, CommentaireBiblioResponse, DemandeCreeeResponse,
    InscriptionBiblioBody, MaDemandeResponse, ReactionBiblioRequest, ReactionBiblioResponse,
    RecommandationBiblioRequest, RecommandationBiblioResponse, RecommandationEtatResponse,
    SpecialiteRow, BIBLIO_HUMAINE_COLONNES,
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

/// Exiger l'utilisateur connecte (sinon 401)
fn exiger_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

/// Verifier que la cible est bien une bibliotheque humaine valide et active
async fn verifier_biblio_valide(pool: &PgPool, biblio_id: Uuid) -> Result<(), ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.utilisateur u
            WHERE u.id = $1
              AND u.deleted_at IS NULL
              AND u.etat = 'actif'
              AND EXISTS (
                  SELECT 1 FROM iam.demande_biblio_humaine d
                  WHERE d.utilisateur_id = u.id AND d.statut = 'valide' AND d.deleted_at IS NULL
              )
         )",
    )
    .bind(biblio_id)
    .fetch_one(pool)
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve(
            "Bibliotheque humaine non trouvee".to_string(),
        ));
    }
    Ok(())
}

// ────────────────────────────────────────────────────────────────
// Endpoints
// ────────────────────────────────────────────────────────────────

/// GET /api/bibliotheques-humaines : Liste paginee avec filtres
/// Ne retourne que les profils avec une demande en statut 'valide' (US3)
pub async fn lister_biblios(
    pool: web::Data<PgPool>,
    params: web::Query<BiblioHumaineQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec![
        "EXISTS (
            SELECT 1 FROM iam.demande_biblio_humaine d
            WHERE d.utilisateur_id = u.id
              AND d.statut = 'valide'
              AND d.deleted_at IS NULL
         )"
        .to_string(),
        "u.deleted_at IS NULL".to_string(),
        "u.etat = 'actif'".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par specialite
    if let Some(ref specialite) = params.specialite {
        let trimmed = specialite.trim();
        if !trimmed.is_empty() && trimmed != "Tous" {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM iam.utilisateur_specialite us
                 JOIN iam.specialite_bibliotheque sb ON sb.id = us.specialite_id
                 WHERE us.utilisateur_id = u.id AND LOWER(sb.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(trimmed.to_string());
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

    // Recherche textuelle
    if let Some(ref recherche) = params.recherche {
        let trimmed = recherche.trim();
        if !trimmed.is_empty() {
            let terme = format!("%{}%", trimmed.to_lowercase());
            conditions.push(format!(
                "(LOWER(u.nom) LIKE ${idx} OR LOWER(u.prenom) LIKE ${idx} \
                 OR LOWER(COALESCE(u.fonction, '')) LIKE ${idx} \
                 OR LOWER(COALESCE(u.biographie, '')) LIKE ${idx} \
                 OR EXISTS (SELECT 1 FROM iam.utilisateur_specialite us4
                    JOIN iam.specialite_bibliotheque sb4 ON sb4.id = us4.specialite_id
                    WHERE us4.utilisateur_id = u.id AND LOWER(sb4.nom) LIKE ${idx}))",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_origine_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les bibliotheques humaines
    let select_query = format!(
        "SELECT {} FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_origine_id
         WHERE {} ORDER BY u.created_at DESC LIMIT ${} OFFSET ${}",
        BIBLIO_HUMAINE_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, BiblioHumaineRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;
    let bibliotheques: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(BiblioHumaineListeResponse {
            bibliotheques,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/bibliotheques-humaines/{id}, Detail d'une bibliotheque humaine
pub async fn obtenir_biblio(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_origine_id
         WHERE u.id = $1
           AND u.deleted_at IS NULL
           AND EXISTS (
               SELECT 1 FROM iam.demande_biblio_humaine d
               WHERE d.utilisateur_id = u.id AND d.statut = 'valide' AND d.deleted_at IS NULL
           )",
        BIBLIO_HUMAINE_COLONNES
    );

    let row = sqlx::query_as::<_, BiblioHumaineRow>(&query)
        .bind(utilisateur_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| {
            ApiErreur::NonTrouve(format!(
                "Bibliotheque humaine avec id {} non trouvee",
                utilisateur_id
            ))
        })?;

    let mut reponse = row.to_response();

    // Compteurs d'interactions (calcules a la lecture)
    let (nb_likes, nb_dislikes): (i64, i64) = sqlx::query_as(
        "SELECT
            COUNT(*) FILTER (WHERE type_reaction = 'like'),
            COUNT(*) FILTER (WHERE type_reaction = 'dislike')
         FROM iam.biblio_reaction WHERE utilisateur_id = $1",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;
    reponse.nombre_likes = nb_likes;
    reponse.nombre_dislikes = nb_dislikes;

    reponse.nombre_recommandations = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM iam.biblio_recommandation WHERE utilisateur_id = $1",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    reponse.nombre_commentaires = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM iam.biblio_commentaire WHERE utilisateur_id = $1 AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Etat du membre connecte (si present)
    if let Some(auteur_id) = extraire_utilisateur_id(&req) {
        reponse.ma_reaction = sqlx::query_scalar(
            "SELECT type_reaction FROM iam.biblio_reaction
             WHERE utilisateur_id = $1 AND auteur_id = $2",
        )
        .bind(utilisateur_id)
        .bind(auteur_id)
        .fetch_optional(pool.get_ref())
        .await?;

        reponse.a_recommande = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM iam.biblio_recommandation
             WHERE utilisateur_id = $1 AND auteur_id = $2)",
        )
        .bind(utilisateur_id)
        .bind(auteur_id)
        .fetch_one(pool.get_ref())
        .await?;
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

/// POST /api/bibliotheques-humaines/inscription, Soumettre une demande (JWT requis)
/// Cree une demande en statut 'en_attente' ; retourne 409 si demande active existante
pub async fn inscrire_biblio(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<InscriptionBiblioBody>,
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
        return Err(ApiErreur::NonTrouve("Utilisateur non trouve".to_string()));
    }

    // Verifier qu'il n'a pas deja une demande active (en_attente ou valide)
    let demande_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.demande_biblio_humaine
            WHERE utilisateur_id = $1
              AND statut IN ('en_attente', 'valide')
              AND deleted_at IS NULL
         )",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if demande_active {
        return Err(ApiErreur::Conflit(
            "Vous avez deja une demande active (en attente ou validee)".to_string(),
        ));
    }

    // Valider les champs obligatoires
    if body.specialites.is_empty() {
        return Err(ApiErreur::Validation(
            "Au moins une specialite est requise".to_string(),
        ));
    }

    let biographie = body.biographie.as_deref().unwrap_or("").trim().to_string();
    if biographie.is_empty() {
        return Err(ApiErreur::Validation(
            "La biographie est obligatoire".to_string(),
        ));
    }
    if biographie.len() < 20 {
        return Err(ApiErreur::Validation(
            "La biographie doit contenir au moins 20 caracteres".to_string(),
        ));
    }

    let fonction = body.fonction.as_deref().unwrap_or("").trim().to_string();
    if fonction.is_empty() {
        return Err(ApiErreur::Validation(
            "La fonction est obligatoire".to_string(),
        ));
    }

    // Resoudre le pays si fourni
    let pays_nom = body.pays.as_deref().unwrap_or("").trim().to_string();
    let pays_id: Option<Uuid> = if !pays_nom.is_empty() {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM shared.pays WHERE LOWER(nom) = LOWER($1)",
        )
        .bind(&pays_nom)
        .fetch_optional(pool.get_ref())
        .await?
    } else {
        None
    };

    // Creer la demande
    let demande_id: Uuid = sqlx::query_scalar(
        "INSERT INTO iam.demande_biblio_humaine
         (utilisateur_id, statut, fonction, biographie, pays_origine_id)
         VALUES ($1, 'en_attente', $2, $3, $4)
         RETURNING id",
    )
    .bind(utilisateur_id)
    .bind(&fonction)
    .bind(&biographie)
    .bind(pays_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Associer les specialites a la demande
    for nom_specialite in &body.specialites {
        let specialite_id: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM iam.specialite_bibliotheque WHERE LOWER(nom) = LOWER($1)",
        )
        .bind(nom_specialite)
        .fetch_optional(pool.get_ref())
        .await?;

        if let Some(spec_id) = specialite_id {
            sqlx::query(
                "INSERT INTO iam.demande_biblio_specialite (demande_id, specialite_id)
                 VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(demande_id)
            .bind(spec_id)
            .execute(pool.get_ref())
            .await?;
        }
    }

    log::info!(
        "Demande bibliotheque humaine soumise par {} (demande: {})",
        utilisateur_id,
        demande_id
    );

    // Recuperer la date de creation pour la reponse
    let created_at: chrono::DateTime<chrono::Utc> = sqlx::query_scalar(
        "SELECT created_at FROM iam.demande_biblio_humaine WHERE id = $1",
    )
    .bind(demande_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(DemandeCreeeResponse {
            id: demande_id,
            statut: "en_attente".to_string(),
            created_at,
        }),
        error: None,
    }))
}

/// GET /api/bibliotheques-humaines/moi/demande, Suivi de statut pour le candidat (US4)
pub async fn ma_demande(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    #[derive(sqlx::FromRow)]
    struct MaDemandeRow {
        id: Uuid,
        statut: String,
        fonction: String,
        biographie: String,
        pays: Option<String>,
        specialites_noms: String,
        commentaire_admin: Option<String>,
        created_at: chrono::DateTime<chrono::Utc>,
        traite_le: Option<chrono::DateTime<chrono::Utc>>,
    }

    let row = sqlx::query_as::<_, MaDemandeRow>(
        "SELECT
            d.id,
            d.statut::TEXT AS statut,
            d.fonction,
            d.biographie,
            p.nom AS pays,
            COALESCE(
                (SELECT string_agg(sb.nom, ', ' ORDER BY sb.nom)
                 FROM iam.demande_biblio_specialite ds2
                 JOIN iam.specialite_bibliotheque sb ON sb.id = ds2.specialite_id
                 WHERE ds2.demande_id = d.id),
                ''
            ) AS specialites_noms,
            d.commentaire_admin,
            d.created_at,
            d.traite_le
         FROM iam.demande_biblio_humaine d
         LEFT JOIN shared.pays p ON p.id = d.pays_origine_id
         WHERE d.utilisateur_id = $1
           AND d.deleted_at IS NULL
         ORDER BY d.created_at DESC
         LIMIT 1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Aucune demande trouvee".to_string()))?;

    let specialites: Vec<String> = if row.specialites_noms.is_empty() {
        vec![]
    } else {
        row.specialites_noms.split(", ").map(|s: &str| s.to_string()).collect()
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MaDemandeResponse {
            id: row.id,
            statut: row.statut,
            fonction: row.fonction,
            biographie: row.biographie,
            pays: row.pays,
            specialites,
            commentaire_admin: row.commentaire_admin,
            created_at: row.created_at,
            traite_le: row.traite_le,
        }),
        error: None,
    }))
}

/// GET /api/bibliotheques-humaines/specialites, Liste des specialites disponibles
pub async fn lister_specialites(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let rows = sqlx::query_as::<_, SpecialiteRow>(
        "SELECT id, nom, slug FROM iam.specialite_bibliotheque ORDER BY nom",
    )
    .fetch_all(pool.get_ref())
    .await?;

    let specialites: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(specialites),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// Interactions : réactions, commentaires, recommandations
// La cible {id} est l'utilisateur-biblio (cf. obtenir_biblio)
// ────────────────────────────────────────────────────────────────

/// Compter les likes / dislikes d'une biblio
async fn compter_reactions(pool: &PgPool, biblio_id: Uuid) -> Result<(i64, i64), ApiErreur> {
    let counts: (i64, i64) = sqlx::query_as(
        "SELECT
            COUNT(*) FILTER (WHERE type_reaction = 'like'),
            COUNT(*) FILTER (WHERE type_reaction = 'dislike')
         FROM iam.biblio_reaction WHERE utilisateur_id = $1",
    )
    .bind(biblio_id)
    .fetch_one(pool)
    .await?;
    Ok(counts)
}

/// POST /api/bibliotheques-humaines/{id}/reaction, Aimer / ne pas aimer (toggle)
pub async fn reagir_biblio(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ReactionBiblioRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;
    let biblio_id = chemin.into_inner();

    if auteur_id == biblio_id {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas reagir a votre propre profil".to_string(),
        ));
    }

    let type_reaction = body.type_reaction.trim().to_lowercase();
    if type_reaction != "like" && type_reaction != "dislike" {
        return Err(ApiErreur::Validation(
            "Type de reaction invalide (like ou dislike)".to_string(),
        ));
    }

    verifier_biblio_valide(pool.get_ref(), biblio_id).await?;

    let reaction_existante: Option<String> = sqlx::query_scalar(
        "SELECT type_reaction FROM iam.biblio_reaction
         WHERE utilisateur_id = $1 AND auteur_id = $2",
    )
    .bind(biblio_id)
    .bind(auteur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let ma_reaction: Option<String> = match reaction_existante {
        Some(ancien) if ancien == type_reaction => {
            // Bascule : retirer la reaction
            sqlx::query(
                "DELETE FROM iam.biblio_reaction WHERE utilisateur_id = $1 AND auteur_id = $2",
            )
            .bind(biblio_id)
            .bind(auteur_id)
            .execute(pool.get_ref())
            .await?;
            None
        }
        Some(_) => {
            // Changer le type
            sqlx::query(
                "UPDATE iam.biblio_reaction SET type_reaction = $1, updated_at = NOW()
                 WHERE utilisateur_id = $2 AND auteur_id = $3",
            )
            .bind(&type_reaction)
            .bind(biblio_id)
            .bind(auteur_id)
            .execute(pool.get_ref())
            .await?;
            Some(type_reaction.clone())
        }
        None => {
            // Nouvelle reaction
            sqlx::query(
                "INSERT INTO iam.biblio_reaction (utilisateur_id, auteur_id, type_reaction)
                 VALUES ($1, $2, $3)",
            )
            .bind(biblio_id)
            .bind(auteur_id)
            .bind(&type_reaction)
            .execute(pool.get_ref())
            .await?;
            Some(type_reaction.clone())
        }
    };

    let (nombre_likes, nombre_dislikes) = compter_reactions(pool.get_ref(), biblio_id).await?;

    // Engagement : 1 point au titulaire par « j'aime » reçu (non-bloquant).
    // La fiche est indexée sur son titulaire : `biblio_id` EST le bénéficiaire.
    // L'auto-réaction est déjà refusée plus haut par un 400.
    if type_reaction == "like" {
        crate::services::engagement::crediter_jaime(
            pool.get_ref(),
            "biblio_humaine",
            biblio_id,
            biblio_id,
            auteur_id,
        )
        .await;
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ReactionBiblioResponse {
            nombre_likes,
            nombre_dislikes,
            ma_reaction,
        }),
        error: None,
    }))
}

/// GET /api/bibliotheques-humaines/{id}/commentaires, Liste plate
pub async fn lister_commentaires_biblio(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let biblio_id = chemin.into_inner();

    let commentaires = sqlx::query_as::<_, CommentaireBiblioResponse>(
        "SELECT c.id,
                c.auteur_id,
                a.nom        AS auteur_nom,
                a.prenom     AS auteur_prenom,
                a.photo_url  AS auteur_photo_url,
                c.contenu,
                c.created_at
         FROM iam.biblio_commentaire c
         JOIN iam.utilisateur a ON a.id = c.auteur_id
         WHERE c.utilisateur_id = $1 AND c.deleted_at IS NULL
         ORDER BY c.created_at DESC",
    )
    .bind(biblio_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(commentaires),
        error: None,
    }))
}

/// POST /api/bibliotheques-humaines/{id}/commentaires, Commenter
pub async fn creer_commentaire_biblio(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CommentaireBiblioRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;
    let biblio_id = chemin.into_inner();

    let contenu = body.contenu.trim().to_string();
    if contenu.is_empty() {
        return Err(ApiErreur::Validation(
            "Le commentaire ne peut pas etre vide".to_string(),
        ));
    }
    if contenu.chars().count() > 2000 {
        return Err(ApiErreur::Validation(
            "Le commentaire ne peut pas depasser 2000 caracteres".to_string(),
        ));
    }

    verifier_biblio_valide(pool.get_ref(), biblio_id).await?;

    let nouvel_id: Uuid = sqlx::query_scalar(
        "INSERT INTO iam.biblio_commentaire (utilisateur_id, auteur_id, contenu)
         VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(biblio_id)
    .bind(auteur_id)
    .bind(&contenu)
    .fetch_one(pool.get_ref())
    .await?;

    let commentaire = sqlx::query_as::<_, CommentaireBiblioResponse>(
        "SELECT c.id,
                c.auteur_id,
                a.nom        AS auteur_nom,
                a.prenom     AS auteur_prenom,
                a.photo_url  AS auteur_photo_url,
                c.contenu,
                c.created_at
         FROM iam.biblio_commentaire c
         JOIN iam.utilisateur a ON a.id = c.auteur_id
         WHERE c.id = $1",
    )
    .bind(nouvel_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(commentaire),
        error: None,
    }))
}

/// DELETE /api/bibliotheques-humaines/{id}/commentaires/{commentaire_id}, Supprimer (son propre)
pub async fn supprimer_commentaire_biblio(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;
    let (_biblio_id, commentaire_id) = chemin.into_inner();

    let resultat = sqlx::query(
        "UPDATE iam.biblio_commentaire
         SET deleted_at = NOW()
         WHERE id = $1 AND auteur_id = $2 AND deleted_at IS NULL",
    )
    .bind(commentaire_id)
    .bind(auteur_id)
    .execute(pool.get_ref())
    .await?;

    if resultat.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Commentaire introuvable ou non autorise".to_string(),
        ));
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": commentaire_id })),
        error: None,
    }))
}

/// POST /api/bibliotheques-humaines/{id}/recommandation, Recommander (upsert)
pub async fn recommander_biblio(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<RecommandationBiblioRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;
    let biblio_id = chemin.into_inner();

    if auteur_id == biblio_id {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas recommander votre propre profil".to_string(),
        ));
    }

    let message: Option<String> = match body.message.as_deref().map(str::trim) {
        Some(m) if !m.is_empty() => {
            if m.chars().count() > 1000 {
                return Err(ApiErreur::Validation(
                    "Le temoignage ne peut pas depasser 1000 caracteres".to_string(),
                ));
            }
            Some(m.to_string())
        }
        _ => None,
    };

    verifier_biblio_valide(pool.get_ref(), biblio_id).await?;

    sqlx::query(
        "INSERT INTO iam.biblio_recommandation (utilisateur_id, auteur_id, message)
         VALUES ($1, $2, $3)
         ON CONFLICT (utilisateur_id, auteur_id)
         DO UPDATE SET message = EXCLUDED.message, updated_at = NOW()",
    )
    .bind(biblio_id)
    .bind(auteur_id)
    .bind(&message)
    .execute(pool.get_ref())
    .await?;

    let nombre_recommandations = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM iam.biblio_recommandation WHERE utilisateur_id = $1",
    )
    .bind(biblio_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(RecommandationEtatResponse {
            nombre_recommandations,
            a_recommande: true,
        }),
        error: None,
    }))
}

/// DELETE /api/bibliotheques-humaines/{id}/recommandation, Retirer sa recommandation
pub async fn retirer_recommandation_biblio(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;
    let biblio_id = chemin.into_inner();

    sqlx::query(
        "DELETE FROM iam.biblio_recommandation WHERE utilisateur_id = $1 AND auteur_id = $2",
    )
    .bind(biblio_id)
    .bind(auteur_id)
    .execute(pool.get_ref())
    .await?;

    let nombre_recommandations = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM iam.biblio_recommandation WHERE utilisateur_id = $1",
    )
    .bind(biblio_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(RecommandationEtatResponse {
            nombre_recommandations,
            a_recommande: false,
        }),
        error: None,
    }))
}

/// GET /api/bibliotheques-humaines/{id}/recommandations, Liste des témoignages
pub async fn lister_recommandations_biblio(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let biblio_id = chemin.into_inner();

    let recommandations = sqlx::query_as::<_, RecommandationBiblioResponse>(
        "SELECT r.id,
                r.auteur_id,
                a.nom        AS auteur_nom,
                a.prenom     AS auteur_prenom,
                a.photo_url  AS auteur_photo_url,
                a.fonction   AS auteur_fonction,
                r.message,
                r.created_at
         FROM iam.biblio_recommandation r
         JOIN iam.utilisateur a ON a.id = r.auteur_id
         WHERE r.utilisateur_id = $1
         ORDER BY r.created_at DESC",
    )
    .bind(biblio_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(recommandations),
        error: None,
    }))
}

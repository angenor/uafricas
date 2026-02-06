use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::codimoi::{
    AuteurInfo, CodiMoi, CodiMoiAuteurResponse, CodiMoiListeResponse,
    CodiMoiQueryParams, CodiMoiResponse, CreerCodiMoiRequest, TagInfo,
    CODIMOI_COLONNES,
};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Recuperer les hashtags d'un post
async fn charger_hashtags(pool: &PgPool, codimoi_id: Uuid) -> Result<Vec<String>, ApiErreur> {
    let tags = sqlx::query_as::<_, TagInfo>(
        "SELECT t.nom FROM shared.tag t
         JOIN culture.codimoi_tag ct ON ct.tag_id = t.id
         WHERE ct.codimoi_id = $1
         ORDER BY t.nom",
    )
    .bind(codimoi_id)
    .fetch_all(pool)
    .await?;

    Ok(tags.into_iter().map(|t| t.nom).collect())
}

/// Recuperer l'auteur d'un post
async fn charger_auteur(pool: &PgPool, utilisateur_id: Uuid) -> Result<AuteurInfo, ApiErreur> {
    sqlx::query_as::<_, AuteurInfo>(
        "SELECT id, nom, prenom, email FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Auteur non trouve".to_string()))
}

/// Construire un CodiMoiResponse a partir d'un CodiMoi + donnees associees
async fn construire_response(
    pool: &PgPool,
    post: &CodiMoi,
) -> Result<CodiMoiResponse, ApiErreur> {
    let hashtags = charger_hashtags(pool, post.id).await?;
    let auteur = charger_auteur(pool, post.cree_par).await?;

    // Recuperer le nom du pays
    let pays_nom: Option<String> = if let Some(pays_id) = post.pays_id {
        sqlx::query_scalar("SELECT nom FROM shared.pays WHERE id = $1")
            .bind(pays_id)
            .fetch_optional(pool)
            .await?
    } else {
        None
    };

    Ok(CodiMoiResponse {
        id: post.id,
        type_post: post.type_post.clone(),
        contenu: post.contenu.clone(),
        explication: post.explication.clone(),
        nom_auteur_originel: post.nom_auteur_originel.clone(),
        pays: pays_nom,
        groupe_ethnique: post.groupe_ethnique.clone(),
        couleur_fond: post.couleur_fond.clone(),
        image_couverture_url: post.image_couverture_url.clone(),
        image_arriere_plan_url: post.image_arriere_plan_url.clone(),
        nombre_likes: post.nombre_likes,
        nombre_dislikes: post.nombre_dislikes,
        hashtags,
        auteur: CodiMoiAuteurResponse {
            id: auteur.id,
            nom: auteur.nom,
            prenom: auteur.prenom,
        },
        created_at: post.created_at,
    })
}

// ──────────────────────────────────────────────────────────────
// GET /api/codimoi — Lister les posts avec filtres et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_posts(
    pool: web::Data<PgPool>,
    params: web::Query<CodiMoiQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(10).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Construire les conditions WHERE
    let mut conditions: Vec<String> = vec![
        "c.etat = 'publie'".to_string(),
        "c.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref type_post) = params.type_post {
        conditions.push(format!("c.type::text = ${}", bind_index));
        bind_values.push(type_post.clone());
        bind_index += 1;
    }

    if let Some(ref pays) = params.pays {
        conditions.push(format!(
            "EXISTS (SELECT 1 FROM shared.pays p WHERE p.id = c.pays_id AND LOWER(p.nom) = LOWER(${})) ",
            bind_index
        ));
        bind_values.push(pays.clone());
        bind_index += 1;
    }

    if let Some(ref recherche) = params.recherche {
        let terme = format!("%{}%", recherche.to_lowercase());
        conditions.push(format!(
            "(LOWER(c.contenu) LIKE ${idx} OR LOWER(c.explication) LIKE ${idx} OR LOWER(c.nom_auteur_originel) LIKE ${idx})",
            idx = bind_index
        ));
        bind_values.push(terme);
        bind_index += 1;
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!("SELECT COUNT(*) FROM culture.codimoi c WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les posts
    let select_query = format!(
        "SELECT {} FROM culture.codimoi c WHERE {} ORDER BY c.created_at DESC LIMIT ${} OFFSET ${}",
        CODIMOI_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, CodiMoi>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let posts = select_q.fetch_all(pool.get_ref()).await?;

    // Construire les reponses avec hashtags et auteurs
    let mut post_responses = Vec::with_capacity(posts.len());
    for post in &posts {
        post_responses.push(construire_response(pool.get_ref(), post).await?);
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(CodiMoiListeResponse {
            posts: post_responses,
            total,
            page,
            par_page,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/codimoi/{id} — Obtenir le detail d'un post
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_post(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM culture.codimoi c WHERE c.id = $1 AND c.etat = 'publie' AND c.deleted_at IS NULL",
        CODIMOI_COLONNES
    );

    let post = sqlx::query_as::<_, CodiMoi>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Post codimoi avec id {} non trouve", id)))?;

    let reponse = construire_response(pool.get_ref(), &post).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/codimoi — Creer un nouveau post
// ──────────────────────────────────────────────────────────────
pub async fn creer_post(
    pool: web::Data<PgPool>,
    body: web::Json<CreerCodiMoiRequest>,
) -> Result<HttpResponse, ApiErreur> {
    // Validation du type
    let types_valides = ["proverbe_adage", "citation", "ressource_historique", "bonne_pratique"];
    if !types_valides.contains(&body.type_post.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Type invalide: {}. Types valides: {:?}",
            body.type_post, types_valides
        )));
    }

    if body.contenu.trim().is_empty() {
        return Err(ApiErreur::Validation("Le contenu est obligatoire".to_string()));
    }

    // Resoudre pays_id a partir du nom du pays
    let pays_id: Option<Uuid> = if let Some(ref nom_pays) = body.pays {
        sqlx::query_scalar("SELECT id FROM shared.pays WHERE LOWER(nom) = LOWER($1)")
            .bind(nom_pays)
            .fetch_optional(pool.get_ref())
            .await?
    } else {
        None
    };

    // TODO: Recuperer l'utilisateur connecte via JWT
    // Pour l'instant on utilise un utilisateur fictif
    let utilisateur_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM iam.utilisateur ORDER BY created_at ASC LIMIT 1",
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::Validation("Aucun utilisateur trouve en base".to_string()))?;

    // Inserer le post
    let post_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO culture.codimoi (type, contenu, explication, nom_auteur_originel,
         pays_id, groupe_ethnique, couleur_fond, etat, cree_par)
         VALUES ($1::culture.type_codimoi, $2, $3, $4, $5, $6, $7, 'publie', $8)
         RETURNING id",
    )
    .bind(&body.type_post)
    .bind(body.contenu.trim())
    .bind(&body.explication)
    .bind(&body.nom_auteur_originel)
    .bind(pays_id)
    .bind(&body.groupe_ethnique)
    .bind(&body.couleur_fond)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Inserer les hashtags
    if let Some(ref hashtags) = body.hashtags {
        for tag_nom in hashtags {
            let tag_nom_clean = tag_nom.trim().to_lowercase();
            if tag_nom_clean.is_empty() {
                continue;
            }
            let slug = tag_nom_clean.replace(' ', "-");

            // Upsert le tag
            let tag_id = sqlx::query_scalar::<_, Uuid>(
                "INSERT INTO shared.tag (nom, slug)
                 VALUES ($1, $2)
                 ON CONFLICT (slug) DO UPDATE SET nom = EXCLUDED.nom
                 RETURNING id",
            )
            .bind(&tag_nom_clean)
            .bind(&slug)
            .fetch_one(pool.get_ref())
            .await?;

            // Lier le tag au post
            sqlx::query(
                "INSERT INTO culture.codimoi_tag (codimoi_id, tag_id)
                 VALUES ($1, $2)
                 ON CONFLICT DO NOTHING",
            )
            .bind(post_id)
            .bind(tag_id)
            .execute(pool.get_ref())
            .await?;
        }
    }

    // Recuperer le post complet pour la reponse
    let query = format!(
        "SELECT {} FROM culture.codimoi c WHERE c.id = $1",
        CODIMOI_COLONNES
    );

    let post = sqlx::query_as::<_, CodiMoi>(&query)
        .bind(post_id)
        .fetch_one(pool.get_ref())
        .await?;

    let reponse = construire_response(pool.get_ref(), &post).await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

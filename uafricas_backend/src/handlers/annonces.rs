use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::amitie::paire_canonique;
use crate::models::annonce::{
    AnnonceDetailResponse, AnnonceDetailRow, AnnonceListeResponse, AnnonceListeRow,
    AnnonceMediaResponse, AnnonceMediaRow, AnnoncePaysRow, AnnonceQueryParams,
    ContacterAuteurRequest, MembreListeParams, MesAnnonceRow, MesAnnoncesResponse,
    ANNONCE_DETAIL_COLONNES, ANNONCE_LISTE_COLONNES, MES_ANNONCES_COLONNES,
};
use crate::models::messagerie::{evt_message, MessageResponse};
use crate::services::messagerie_sse::RegistreSse;
use crate::services::{audit, image_validation};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// Helpers membre (auth, validation, messagerie) — feature marché membre
// ──────────────────────────────────────────────────────────────

const CONTENU_MESSAGE_MAX: usize = 2000;
const MAX_PHOTOS_ANNONCE: usize = 5;
const TYPES_OPERATION_MEMBRE: &[&str] = &["vente", "troc", "don"];
const CONDITIONS_VALIDES: &[&str] = &["neuf", "occasion", "reconditionne", "non_applicable"];

/// Extraire l'utilisateur connecté depuis le header Authorization (JWT Bearer).
/// Un JWT n'est émis qu'aux comptes `actif` → garantit FR-007 (D1).
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

fn utilisateur_courant(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

/// Génère un slug à partir d'un titre (même logique que l'admin).
fn generer_slug(titre: &str) -> String {
    titre
        .trim()
        .to_lowercase()
        .replace(' ', "-")
        .replace(['\'', '"', '.', ','], "")
}

/// Vérifie qu'un membre est propriétaire d'une annonce non supprimée.
/// Renvoie 404 si introuvable/supprimée, 403 si non propriétaire.
async fn garde_proprietaire(pool: &PgPool, annonce_id: Uuid, moi: Uuid) -> Result<(), ApiErreur> {
    let cree_par: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM marketplace.annonce WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(annonce_id)
    .fetch_optional(pool)
    .await?;

    match cree_par {
        None => Err(ApiErreur::NonTrouve("Annonce non trouvée".to_string())),
        Some(auteur) if auteur != moi => Err(ApiErreur::AccesInterdit(
            "Vous ne pouvez agir que sur vos propres annonces".to_string(),
        )),
        Some(_) => Ok(()),
    }
}

/// Vérifie l'existence d'un blocage dans un sens ou l'autre (chemin marketplace).
async fn blocage_existe(pool: &PgPool, a: Uuid, b: Uuid) -> Result<bool, ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM social.blocage
            WHERE (bloqueur_id = $1 AND bloque_id = $2) OR (bloqueur_id = $2 AND bloque_id = $1)
        )",
    )
    .bind(a)
    .bind(b)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

/// Obtient ou crée la conversation 1-1 entre deux membres SANS exiger l'amitié
/// (chemin marketplace, D2). À la création, renseigne `annonce_id` (COALESCE :
/// ne jamais écraser un contexte d'annonce déjà présent).
async fn obtenir_ou_creer_conversation_annonce(
    pool: &PgPool,
    a: Uuid,
    b: Uuid,
    annonce_id: Uuid,
) -> Result<Uuid, ApiErreur> {
    let (min, max) = paire_canonique(a, b);
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO social.conversation (utilisateur_a_id, utilisateur_b_id, annonce_id)
         VALUES ($1, $2, $3)
         ON CONFLICT (utilisateur_a_id, utilisateur_b_id)
            DO UPDATE SET annonce_id = COALESCE(social.conversation.annonce_id, EXCLUDED.annonce_id)
         RETURNING id",
    )
    .bind(min)
    .bind(max)
    .bind(annonce_id)
    .fetch_one(pool)
    .await?;
    Ok(id)
}

/// Lit un champ multipart en chaîne de caractères (UTF-8 lossy, trim).
async fn lire_champ_texte(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut s = String::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture chunk: {}", e)))?;
        s.push_str(&String::from_utf8_lossy(&data));
    }
    Ok(s.trim().to_string())
}

// ──────────────────────────────────────────────────────────────
// GET /api/annonces — Lister les annonces avec filtres et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_annonces(
    pool: web::Data<PgPool>,
    params: web::Query<AnnonceQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Preparer le terme de recherche
    let terme_recherche: Option<String> = params
        .recherche
        .as_ref()
        .filter(|r| !r.trim().is_empty())
        .map(|r| format!("%{}%", r.to_lowercase()));

    // Preparer le filtre type_operation (supporte valeurs multiples separees par virgules)
    let type_operation_filtre: Option<String> = params
        .type_operation
        .as_ref()
        .filter(|t| !t.trim().is_empty())
        .cloned();

    // Preparer le filtre categorie
    let categorie_filtre: Option<String> = params
        .categorie
        .as_ref()
        .filter(|c| !c.trim().is_empty() && c.as_str() != "Tout")
        .cloned();

    // Clause ORDER BY dynamique
    let order_clause = match params.tri.as_deref() {
        Some("price-asc") => "ORDER BY COALESCE(a.prix, 0) ASC, a.created_at DESC",
        Some("price-desc") => "ORDER BY COALESCE(a.prix, 0) DESC, a.created_at DESC",
        _ => "ORDER BY a.created_at DESC",
    };

    // ── Requete COUNT ────────────────────────────────────────
    let count_query = format!(
        "SELECT COUNT(*) FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         WHERE a.deleted_at IS NULL
           AND a.etat = 'publiee'
           AND ($1::VARCHAR IS NULL OR a.type_operation::text = ANY(string_to_array($1, ',')))
           AND ($2::VARCHAR IS NULL OR c.nom = $2 OR c.slug = $2)
           AND ($3::FLOAT8 IS NULL OR a.prix >= $3)
           AND ($4::FLOAT8 IS NULL OR a.prix <= $4)
           AND ($5::VARCHAR IS NULL OR (
               LOWER(a.titre) LIKE $5
               OR LOWER(a.description) LIKE $5
               OR LOWER(COALESCE(a.ville, '')) LIKE $5
           ))"
    );

    let total: (i64,) = sqlx::query_as(&count_query)
        .bind(&type_operation_filtre)
        .bind(&categorie_filtre)
        .bind(params.prix_min)
        .bind(params.prix_max)
        .bind(&terme_recherche)
        .fetch_one(pool.get_ref())
        .await?;

    // ── Requete principale ───────────────────────────────────
    let query = format!(
        "SELECT {colonnes}
         FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.deleted_at IS NULL
           AND a.etat = 'publiee'
           AND ($1::VARCHAR IS NULL OR a.type_operation::text = ANY(string_to_array($1, ',')))
           AND ($2::VARCHAR IS NULL OR c.nom = $2 OR c.slug = $2)
           AND ($3::FLOAT8 IS NULL OR a.prix >= $3)
           AND ($4::FLOAT8 IS NULL OR a.prix <= $4)
           AND ($5::VARCHAR IS NULL OR (
               LOWER(a.titre) LIKE $5
               OR LOWER(a.description) LIKE $5
               OR LOWER(COALESCE(a.ville, '')) LIKE $5
           ))
         {order}
         LIMIT $6 OFFSET $7",
        colonnes = ANNONCE_LISTE_COLONNES,
        order = order_clause
    );

    let rows = sqlx::query_as::<_, AnnonceListeRow>(&query)
        .bind(&type_operation_filtre)
        .bind(&categorie_filtre)
        .bind(params.prix_min)
        .bind(params.prix_max)
        .bind(&terme_recherche)
        .bind(par_page)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

    let total_count = total.0;
    let total_pages = if total_count == 0 {
        1
    } else {
        (total_count as f64 / par_page as f64).ceil() as i64
    };

    let reponse = AnnonceListeResponse {
        annonces: rows.iter().map(|r| r.to_response()).collect(),
        total: total_count,
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
// GET /api/annonces/{id} — Obtenir le detail d'une annonce
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_annonce(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    // 1. Requete principale avec categorie + auteur
    let query = format!(
        "SELECT {}
         FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.id = $1 AND a.deleted_at IS NULL",
        ANNONCE_DETAIL_COLONNES
    );

    let annonce = sqlx::query_as::<_, AnnonceDetailRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Annonce avec id {} non trouvee", id)))?;

    // 2. Incrementer le nombre de vues
    let _ = sqlx::query(
        "UPDATE marketplace.annonce SET nombre_vues = nombre_vues + 1 WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await;

    // 3. Charger tous les medias
    let media_rows = sqlx::query_as::<_, AnnonceMediaRow>(
        "SELECT id, media_url, type_mime, est_principale, ordre
         FROM marketplace.annonce_media
         WHERE annonce_id = $1
         ORDER BY est_principale DESC NULLS LAST, ordre ASC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let medias: Vec<AnnonceMediaResponse> = media_rows
        .into_iter()
        .map(|m| AnnonceMediaResponse {
            id: m.id,
            media_url: m.media_url,
            type_mime: m.type_mime,
            est_principale: m.est_principale.unwrap_or(false),
            ordre: m.ordre.unwrap_or(0),
        })
        .collect();

    // 4. Charger tous les pays
    let pays_rows = sqlx::query_as::<_, AnnoncePaysRow>(
        "SELECT p.nom AS pays_nom
         FROM marketplace.annonce_pays ap
         JOIN shared.pays p ON p.id = ap.pays_id
         WHERE ap.annonce_id = $1",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let pays: Vec<String> = pays_rows.into_iter().map(|p| p.pays_nom).collect();

    // 5. Construire la reponse
    let reponse = annonce.to_detail_response(pays, medias);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/annonces/categories — Catégories du marché (contexte 'annonce')
// ──────────────────────────────────────────────────────────────
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct CategorieAnnonceResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub icone: Option<String>,
}

pub async fn lister_categories_annonce(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let rows = sqlx::query_as::<_, CategorieAnnonceResponse>(
        "SELECT id, nom, slug, icone
         FROM shared.categorie
         WHERE contexte = 'annonce' AND actif = TRUE
         ORDER BY ordre ASC, nom ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rows),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// Endpoints MEMBRE — feature 001-marche-achat-vente-troc-don
// ════════════════════════════════════════════════════════════════

/// Champs texte d'une annonce parsés depuis un multipart.
#[derive(Default)]
struct ChampsAnnonce {
    titre: Option<String>,
    description: Option<String>,
    type_operation: Option<String>,
    categorie_id: Option<Uuid>,
    condition_article: Option<String>,
    prix: Option<f64>,
    devise: Option<String>,
    prix_negociable: Option<bool>,
    ville: Option<String>,
    adresse: Option<String>,
    longitude: Option<f64>,
    latitude: Option<f64>,
    quantite: Option<i32>,
    pays_ids: Vec<Uuid>,
}

/// Parse un multipart d'annonce : champs texte + fichiers `photos[]`.
/// Renvoie les champs et les photos (bytes, nom de fichier original).
async fn parser_multipart_annonce(
    mut payload: Multipart,
) -> Result<(ChampsAnnonce, Vec<(Vec<u8>, String)>), ApiErreur> {
    let mut champs = ChampsAnnonce::default();
    let mut photos: Vec<(Vec<u8>, String)> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|e| ApiErreur::Upload(format!("Erreur lecture multipart: {}", e)))?;
        let cd = field.content_disposition().cloned();
        let nom = cd
            .as_ref()
            .and_then(|c| c.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        if nom == "photos" || nom == "photos[]" {
            let filename = cd
                .as_ref()
                .and_then(|c| c.get_filename().map(sanitize_filename::sanitize))
                .unwrap_or_else(|| format!("{}.bin", Uuid::new_v4()));
            let mut buf: Vec<u8> = Vec::new();
            while let Some(chunk) = field.next().await {
                let data =
                    chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture chunk: {}", e)))?;
                buf.extend_from_slice(&data);
                if buf.len() > image_validation::TAILLE_MAX_OCTETS_ANNONCE + 1 {
                    return Err(ApiErreur::Validation(format!(
                        "Photo trop volumineuse (max {} octets).",
                        image_validation::TAILLE_MAX_OCTETS_ANNONCE
                    )));
                }
            }
            photos.push((buf, filename));
            continue;
        }

        let valeur = lire_champ_texte(&mut field).await?;
        match nom.as_str() {
            "titre" => champs.titre = Some(valeur),
            "description" => champs.description = Some(valeur),
            "type_operation" => champs.type_operation = Some(valeur.to_lowercase()),
            "categorie_id" => champs.categorie_id = Uuid::parse_str(&valeur).ok(),
            "condition_article" => champs.condition_article = Some(valeur.to_lowercase()),
            "prix" => champs.prix = valeur.parse::<f64>().ok(),
            "devise" => {
                if !valeur.is_empty() {
                    champs.devise = Some(valeur)
                }
            }
            "prix_negociable" => {
                champs.prix_negociable = Some(valeur == "true" || valeur == "1")
            }
            "ville" => champs.ville = Some(valeur),
            "adresse" => champs.adresse = Some(valeur),
            "longitude" => champs.longitude = valeur.parse::<f64>().ok(),
            "latitude" => champs.latitude = valeur.parse::<f64>().ok(),
            "quantite" => champs.quantite = valeur.parse::<i32>().ok(),
            "pays_ids" | "pays_ids[]" => {
                for part in valeur.split(',') {
                    if let Ok(id) = Uuid::parse_str(part.trim()) {
                        champs.pays_ids.push(id);
                    }
                }
            }
            _ => {}
        }
    }

    Ok((champs, photos))
}

/// Valide en mémoire puis écrit les photos sur disque (atomique : rollback si échec).
/// Insère les lignes `annonce_media`. `ordre_depart` permet d'enchaîner après des
/// photos existantes ; `forcer_principale` marque la 1ʳᵉ comme principale.
async fn ecrire_photos_annonce(
    pool: &PgPool,
    upload_dir: &str,
    annonce_id: Uuid,
    photos: Vec<(Vec<u8>, String)>,
    ordre_depart: i32,
    forcer_principale: bool,
) -> Result<Vec<AnnonceMediaResponse>, ApiErreur> {
    // 1. Validation en mémoire AVANT toute écriture disque.
    let mut valides = Vec::new();
    for (bytes, _filename) in photos.into_iter() {
        match image_validation::valider_photo_annonce(&bytes) {
            Ok(dim) => valides.push((bytes, dim)),
            Err(err) => return Err(ApiErreur::Validation(err.message())),
        }
    }

    let dossier = format!("{}/marketplace/annonces", upload_dir);
    std::fs::create_dir_all(&dossier)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le dossier annonces: {}", e)))?;

    let mut fichiers_ecrits: Vec<String> = Vec::new();
    let mut medias = Vec::new();

    for (idx, (bytes, dim)) in valides.into_iter().enumerate() {
        let ext = dim.format.extension();
        let nom_fichier = format!("{}.{}", Uuid::new_v4(), ext);
        let chemin_complet = format!("{}/{}", dossier, nom_fichier);
        let chemin_relatif = format!("/uploads/marketplace/annonces/{}", nom_fichier);

        let res = (|| -> std::io::Result<()> {
            let mut f = std::fs::File::create(&chemin_complet)?;
            f.write_all(&bytes)?;
            Ok(())
        })();
        if let Err(e) = res {
            for f in &fichiers_ecrits {
                let _ = std::fs::remove_file(f);
            }
            return Err(ApiErreur::Upload(format!("Écriture disque échouée: {}", e)));
        }
        fichiers_ecrits.push(chemin_complet);

        let est_principale = forcer_principale && idx == 0;
        let ordre = ordre_depart + idx as i32;
        let media_id: Uuid = sqlx::query_scalar(
            "INSERT INTO marketplace.annonce_media
             (annonce_id, media_url, type_mime, est_principale, ordre)
             VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(annonce_id)
        .bind(&chemin_relatif)
        .bind(dim.format.type_mime())
        .bind(est_principale)
        .bind(ordre)
        .fetch_one(pool)
        .await?;

        medias.push(AnnonceMediaResponse {
            id: media_id,
            media_url: chemin_relatif,
            type_mime: Some(dim.format.type_mime().to_string()),
            est_principale,
            ordre,
        });
    }

    Ok(medias)
}

/// Recharge le détail complet d'une annonce (réponse `AnnonceDetailResponse`).
async fn charger_detail_response(
    pool: &PgPool,
    id: Uuid,
) -> Result<AnnonceDetailResponse, ApiErreur> {
    let query = format!(
        "SELECT {}
         FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.id = $1 AND a.deleted_at IS NULL",
        ANNONCE_DETAIL_COLONNES
    );
    let annonce = sqlx::query_as::<_, AnnonceDetailRow>(&query)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Annonce non trouvée".to_string()))?;

    let media_rows = sqlx::query_as::<_, AnnonceMediaRow>(
        "SELECT id, media_url, type_mime, est_principale, ordre
         FROM marketplace.annonce_media
         WHERE annonce_id = $1
         ORDER BY est_principale DESC NULLS LAST, ordre ASC",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    let medias: Vec<AnnonceMediaResponse> = media_rows
        .into_iter()
        .map(|m| AnnonceMediaResponse {
            id: m.id,
            media_url: m.media_url,
            type_mime: m.type_mime,
            est_principale: m.est_principale.unwrap_or(false),
            ordre: m.ordre.unwrap_or(0),
        })
        .collect();

    let pays_rows = sqlx::query_as::<_, AnnoncePaysRow>(
        "SELECT p.nom AS pays_nom
         FROM marketplace.annonce_pays ap
         JOIN shared.pays p ON p.id = ap.pays_id
         WHERE ap.annonce_id = $1",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    let pays: Vec<String> = pays_rows.into_iter().map(|p| p.pays_nom).collect();

    Ok(annonce.to_detail_response(pays, medias))
}

/// Insère les liens annonce↔pays après validation d'existence (silencieux si déjà présent).
async fn lier_pays(pool: &PgPool, annonce_id: Uuid, pays_ids: &[Uuid]) -> Result<(), ApiErreur> {
    for pays_id in pays_ids {
        let existe: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM shared.pays WHERE id = $1 AND actif = true)",
        )
        .bind(pays_id)
        .fetch_one(pool)
        .await?;
        if existe {
            sqlx::query(
                "INSERT INTO marketplace.annonce_pays (annonce_id, pays_id)
                 VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(annonce_id)
            .bind(pays_id)
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}

// ──────────────────────────────────────────────────────────────
// US1 — POST /api/annonces : Publier une annonce
// ──────────────────────────────────────────────────────────────
pub async fn creer_annonce_membre(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    req: HttpRequest,
    payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let (champs, photos) = parser_multipart_annonce(payload).await?;

    // ── Validations (frontière) ──
    let titre = champs
        .titre
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le titre est requis".to_string()))?;
    if !(3..=350).contains(&titre.chars().count()) {
        return Err(ApiErreur::Validation(
            "Le titre doit contenir entre 3 et 350 caractères".to_string(),
        ));
    }
    let description = champs
        .description
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ApiErreur::Validation("La description est requise".to_string()))?;
    if description.chars().count() < 10 {
        return Err(ApiErreur::Validation(
            "La description doit contenir au moins 10 caractères".to_string(),
        ));
    }
    let type_op = champs
        .type_operation
        .as_deref()
        .unwrap_or("");
    if !TYPES_OPERATION_MEMBRE.contains(&type_op) {
        return Err(ApiErreur::Validation(
            "Type d'opération invalide (vente, troc ou don)".to_string(),
        ));
    }
    let categorie_id = champs
        .categorie_id
        .ok_or_else(|| ApiErreur::Validation("La catégorie est requise".to_string()))?;
    if let Some(ref cond) = champs.condition_article {
        if !CONDITIONS_VALIDES.contains(&cond.as_str()) {
            return Err(ApiErreur::Validation("Condition d'article invalide".to_string()));
        }
    }
    // Prix requis et > 0 si vente
    if type_op == "vente" {
        match champs.prix {
            Some(p) if p > 0.0 => {}
            _ => {
                return Err(ApiErreur::Validation(
                    "Un prix supérieur à 0 est requis pour une vente".to_string(),
                ))
            }
        }
    }
    if photos.is_empty() {
        return Err(ApiErreur::Validation(
            "Au moins une photo est requise".to_string(),
        ));
    }
    if photos.len() > MAX_PHOTOS_ANNONCE {
        return Err(ApiErreur::Validation(format!(
            "Maximum {} photos par annonce",
            MAX_PHOTOS_ANNONCE
        )));
    }

    // Catégorie existe ?
    let cat_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM shared.categorie WHERE id = $1)",
    )
    .bind(categorie_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !cat_existe {
        return Err(ApiErreur::Validation("Catégorie inconnue".to_string()));
    }

    let slug = format!("{}-{}", generer_slug(titre), &Uuid::new_v4().to_string()[..8]);
    let id = Uuid::new_v4();
    let prix = if type_op == "vente" { champs.prix } else { None };

    sqlx::query(
        "INSERT INTO marketplace.annonce
         (id, titre, slug, description, type_operation, categorie_id, condition_article,
          prix, devise, prix_negociable, ville, adresse, longitude, latitude,
          type_contact, quantite, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5::marketplace.type_operation, $6,
                 $7::marketplace.condition_article, $8, $9, $10, $11, $12, $13, $14,
                 'messagerie_plateforme'::marketplace.type_contact, $15,
                 'publiee'::marketplace.etat_annonce, $16)",
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(description)
    .bind(type_op)
    .bind(categorie_id)
    .bind(champs.condition_article.as_deref().unwrap_or("non_applicable"))
    .bind(prix)
    .bind(champs.devise.as_deref().unwrap_or("XOF"))
    .bind(champs.prix_negociable.unwrap_or(false))
    .bind(champs.ville.as_deref().map(str::trim).filter(|s| !s.is_empty()))
    .bind(champs.adresse.as_deref().map(str::trim).filter(|s| !s.is_empty()))
    .bind(champs.longitude)
    .bind(champs.latitude)
    .bind(champs.quantite.unwrap_or(1).max(1))
    .bind(moi)
    .execute(pool.get_ref())
    .await?;

    // Territoires ciblés + photos
    lier_pays(pool.get_ref(), id, &champs.pays_ids).await?;
    ecrire_photos_annonce(pool.get_ref(), upload_dir.get_ref(), id, photos, 0, true).await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "marketplace",
        "annonce",
        Some(id),
        None,
        Some(serde_json::json!({ "type_operation": type_op, "etat": "publiee" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let detail = charger_detail_response(pool.get_ref(), id).await?;
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(detail),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// US2 — POST /api/annonces/{id}/contacter : Contacter l'auteur
// ──────────────────────────────────────────────────────────────
pub async fn contacter_auteur(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ContacterAuteurRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let annonce_id = chemin.into_inner();

    let contenu = body.message.trim().to_string();
    let longueur = contenu.chars().count();
    if contenu.is_empty() || longueur > CONTENU_MESSAGE_MAX {
        return Err(ApiErreur::Validation(
            "Le message doit contenir entre 1 et 2000 caractères".to_string(),
        ));
    }

    // L'annonce doit être publiée ; on récupère son auteur.
    let auteur: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM marketplace.annonce
         WHERE id = $1 AND etat = 'publiee' AND deleted_at IS NULL",
    )
    .bind(annonce_id)
    .fetch_optional(pool.get_ref())
    .await?;
    let auteur = auteur
        .ok_or_else(|| ApiErreur::NonTrouve("Annonce introuvable ou non disponible".to_string()))?;

    // FR-013 : on ne contacte pas sa propre annonce
    if auteur == moi {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas contacter votre propre annonce".to_string(),
        ));
    }

    // Pas de blocage réciproque
    if blocage_existe(pool.get_ref(), moi, auteur).await? {
        return Err(ApiErreur::AccesInterdit(
            "Échange impossible avec ce membre".to_string(),
        ));
    }

    // Conversation (chemin marketplace : pas d'amitié requise) + message initial
    let conv_id =
        obtenir_ou_creer_conversation_annonce(pool.get_ref(), moi, auteur, annonce_id).await?;

    let (message_id, created_at): (Uuid, DateTime<Utc>) = sqlx::query_as(
        "INSERT INTO social.message (conversation_id, expediteur_id, contenu)
         VALUES ($1, $2, $3) RETURNING id, created_at",
    )
    .bind(conv_id)
    .bind(moi)
    .bind(&contenu)
    .fetch_one(pool.get_ref())
    .await?;

    sqlx::query("UPDATE social.conversation SET dernier_message_at = NOW() WHERE id = $1")
        .bind(conv_id)
        .execute(pool.get_ref())
        .await?;

    let message = MessageResponse {
        id: message_id,
        expediteur_id: moi,
        contenu: Some(contenu),
        supprime: false,
        created_at,
        lu_at: None,
    };

    // Notifie l'auteur (FR-012) + synchronise l'expéditeur via SSE messagerie.
    let evt = evt_message(conv_id, &message);
    sse.publier(auteur, &evt);
    sse.publier(moi, &evt);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "social",
        "message",
        Some(message_id),
        None,
        Some(serde_json::json!({
            "conversation_id": conv_id,
            "annonce_id": annonce_id,
            "longueur": longueur,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "conversation_id": conv_id,
            "ami_id": auteur,
            "message": message,
        })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// US3 — GET /api/annonces/mes-annonces : Lister mes annonces
// ──────────────────────────────────────────────────────────────
pub async fn mes_annonces(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<MembreListeParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;
    let etat_filtre = params
        .etat
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM marketplace.annonce a
         WHERE a.cree_par = $1 AND a.deleted_at IS NULL
           AND ($2::VARCHAR IS NULL OR a.etat::text = $2)",
    )
    .bind(moi)
    .bind(&etat_filtre)
    .fetch_one(pool.get_ref())
    .await?;

    let query = format!(
        "SELECT {colonnes}
         FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.cree_par = $1 AND a.deleted_at IS NULL
           AND ($2::VARCHAR IS NULL OR a.etat::text = $2)
         ORDER BY a.created_at DESC
         LIMIT $3 OFFSET $4",
        colonnes = MES_ANNONCES_COLONNES
    );
    let rows = sqlx::query_as::<_, MesAnnonceRow>(&query)
        .bind(moi)
        .bind(&etat_filtre)
        .bind(par_page)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

    let total_count = total.0;
    let total_pages = if total_count == 0 {
        1
    } else {
        (total_count as f64 / par_page as f64).ceil() as i64
    };

    let reponse = MesAnnoncesResponse {
        annonces: rows.iter().map(|r| r.to_response()).collect(),
        total: total_count,
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
// US3 — PUT /api/annonces/{id} : Modifier une annonce (propriétaire)
// ──────────────────────────────────────────────────────────────
pub async fn modifier_annonce_membre(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();
    garde_proprietaire(pool.get_ref(), id, moi).await?;

    let (champs, photos) = parser_multipart_annonce(payload).await?;

    let mut sets: Vec<String> = Vec::new();
    let mut binds_str: Vec<String> = Vec::new();
    let mut binds_uuid: Vec<Uuid> = Vec::new();
    let mut idx: u32 = 1;

    if let Some(ref titre) = champs.titre {
        let t = titre.trim();
        if !(3..=350).contains(&t.chars().count()) {
            return Err(ApiErreur::Validation(
                "Le titre doit contenir entre 3 et 350 caractères".to_string(),
            ));
        }
        sets.push(format!("titre = ${}", idx));
        binds_str.push(t.to_string());
        idx += 1;
        sets.push(format!("slug = ${}", idx));
        binds_str.push(format!("{}-{}", generer_slug(t), &Uuid::new_v4().to_string()[..8]));
        idx += 1;
    }
    if let Some(ref desc) = champs.description {
        let d = desc.trim();
        if d.chars().count() < 10 {
            return Err(ApiErreur::Validation(
                "La description doit contenir au moins 10 caractères".to_string(),
            ));
        }
        sets.push(format!("description = ${}", idx));
        binds_str.push(d.to_string());
        idx += 1;
    }
    if let Some(ref type_op) = champs.type_operation {
        if !TYPES_OPERATION_MEMBRE.contains(&type_op.as_str()) {
            return Err(ApiErreur::Validation(
                "Type d'opération invalide (vente, troc ou don)".to_string(),
            ));
        }
        sets.push(format!("type_operation = ${}::marketplace.type_operation", idx));
        binds_str.push(type_op.clone());
        idx += 1;
    }
    if let Some(ref cond) = champs.condition_article {
        if !CONDITIONS_VALIDES.contains(&cond.as_str()) {
            return Err(ApiErreur::Validation("Condition d'article invalide".to_string()));
        }
        sets.push(format!("condition_article = ${}::marketplace.condition_article", idx));
        binds_str.push(cond.clone());
        idx += 1;
    }
    if let Some(ref devise) = champs.devise {
        sets.push(format!("devise = ${}", idx));
        binds_str.push(devise.clone());
        idx += 1;
    }
    if let Some(ref ville) = champs.ville {
        sets.push(format!("ville = ${}", idx));
        binds_str.push(ville.trim().to_string());
        idx += 1;
    }
    if let Some(ref adresse) = champs.adresse {
        sets.push(format!("adresse = ${}", idx));
        binds_str.push(adresse.trim().to_string());
        idx += 1;
    }
    if let Some(cat_id) = champs.categorie_id {
        sets.push(format!("categorie_id = ${}", idx));
        binds_uuid.push(cat_id);
        idx += 1;
    }
    // Numériques (inline — valeurs maîtrisées)
    if let Some(prix) = champs.prix {
        sets.push(format!("prix = {}", prix));
    }
    if let Some(q) = champs.quantite {
        sets.push(format!("quantite = {}", q.max(1)));
    }
    if let Some(pn) = champs.prix_negociable {
        sets.push(format!("prix_negociable = {}", pn));
    }
    if let Some(lon) = champs.longitude {
        sets.push(format!("longitude = {}", lon));
    }
    if let Some(lat) = champs.latitude {
        sets.push(format!("latitude = {}", lat));
    }

    if !sets.is_empty() {
        sets.push("updated_at = NOW()".to_string());
        let sql = format!(
            "UPDATE marketplace.annonce SET {} WHERE id = ${} AND deleted_at IS NULL",
            sets.join(", "),
            idx
        );
        let mut q = sqlx::query(&sql);
        for v in &binds_str {
            q = q.bind(v);
        }
        for v in &binds_uuid {
            q = q.bind(v);
        }
        q = q.bind(id);
        q.execute(pool.get_ref()).await?;
    }

    // Remplacement des territoires si fournis
    if !champs.pays_ids.is_empty() {
        sqlx::query("DELETE FROM marketplace.annonce_pays WHERE annonce_id = $1")
            .bind(id)
            .execute(pool.get_ref())
            .await?;
        lier_pays(pool.get_ref(), id, &champs.pays_ids).await?;
    }

    // Ajout éventuel de nouvelles photos (respecte le plafond de 5)
    if !photos.is_empty() {
        let (existantes,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM marketplace.annonce_media WHERE annonce_id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?;
        if existantes as usize + photos.len() > MAX_PHOTOS_ANNONCE {
            return Err(ApiErreur::Validation(format!(
                "Maximum {} photos par annonce (déjà {})",
                MAX_PHOTOS_ANNONCE, existantes
            )));
        }
        let forcer_principale = existantes == 0;
        ecrire_photos_annonce(
            pool.get_ref(),
            upload_dir.get_ref(),
            id,
            photos,
            existantes as i32,
            forcer_principale,
        )
        .await?;
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "marketplace",
        "annonce",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let detail = charger_detail_response(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(detail),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// US3 — PATCH /api/annonces/{id}/conclure : Marquer conclue
// ──────────────────────────────────────────────────────────────
pub async fn conclure_annonce(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();
    garde_proprietaire(pool.get_ref(), id, moi).await?;

    let resultat = sqlx::query(
        "UPDATE marketplace.annonce
         SET etat = 'conclue'::marketplace.etat_annonce, updated_at = NOW()
         WHERE id = $1 AND etat = 'publiee'::marketplace.etat_annonce AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;
    if resultat.rows_affected() == 0 {
        return Err(ApiErreur::Conflit(
            "Seule une annonce publiée peut être marquée conclue".to_string(),
        ));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "marketplace",
        "annonce",
        Some(id),
        None,
        Some(serde_json::json!({ "etat": "conclue" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": "conclue" })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// US3 — DELETE /api/annonces/{id} : Supprimer (soft delete)
// ──────────────────────────────────────────────────────────────
pub async fn supprimer_annonce_membre(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();
    garde_proprietaire(pool.get_ref(), id, moi).await?;

    sqlx::query(
        "UPDATE marketplace.annonce
         SET etat = 'supprimee'::marketplace.etat_annonce, deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "DELETE",
        "marketplace",
        "annonce",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// US3 — POST /api/annonces/{id}/medias : Ajouter des photos (propriétaire)
// ──────────────────────────────────────────────────────────────
pub async fn ajouter_medias_membre(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();
    garde_proprietaire(pool.get_ref(), id, moi).await?;

    let (_champs, photos) = parser_multipart_annonce(payload).await?;
    if photos.is_empty() {
        return Err(ApiErreur::Validation("Aucune photo reçue".to_string()));
    }

    let (existantes,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM marketplace.annonce_media WHERE annonce_id = $1")
            .bind(id)
            .fetch_one(pool.get_ref())
            .await?;
    if existantes as usize + photos.len() > MAX_PHOTOS_ANNONCE {
        return Err(ApiErreur::Validation(format!(
            "Maximum {} photos par annonce (déjà {})",
            MAX_PHOTOS_ANNONCE, existantes
        )));
    }

    let forcer_principale = existantes == 0;
    let medias = ecrire_photos_annonce(
        pool.get_ref(),
        upload_dir.get_ref(),
        id,
        photos,
        existantes as i32,
        forcer_principale,
    )
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "marketplace",
        "annonce_media",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(medias),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// US3 — DELETE /api/annonces/{id}/medias/{media_id} : Retirer une photo
// ──────────────────────────────────────────────────────────────
pub async fn supprimer_media_membre(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let (id, media_id) = chemin.into_inner();
    garde_proprietaire(pool.get_ref(), id, moi).await?;

    // Récupérer la photo (pour suppression disque + promotion principale)
    let media: Option<(String, Option<bool>)> = sqlx::query_as(
        "SELECT media_url, est_principale FROM marketplace.annonce_media
         WHERE id = $1 AND annonce_id = $2",
    )
    .bind(media_id)
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (media_url, etait_principale) =
        media.ok_or_else(|| ApiErreur::NonTrouve("Photo introuvable".to_string()))?;

    sqlx::query("DELETE FROM marketplace.annonce_media WHERE id = $1 AND annonce_id = $2")
        .bind(media_id)
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    // Supprimer le fichier disque (chemin relatif sous /uploads/)
    if let Some(rel) = media_url.strip_prefix("/uploads/") {
        let _ = std::fs::remove_file(format!("{}/{}", upload_dir.get_ref(), rel));
    }

    // Promotion d'une nouvelle photo principale si la principale a été retirée
    if etait_principale.unwrap_or(false) {
        let suivant: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM marketplace.annonce_media
             WHERE annonce_id = $1 ORDER BY ordre ASC LIMIT 1",
        )
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?;
        if let Some(suivant_id) = suivant {
            sqlx::query(
                "UPDATE marketplace.annonce_media SET est_principale = TRUE WHERE id = $1",
            )
            .bind(suivant_id)
            .execute(pool.get_ref())
            .await?;
        }
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "DELETE",
        "marketplace",
        "annonce_media",
        Some(media_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// US4 — POST /api/annonces/{id}/favori : Ajouter aux favoris
// ──────────────────────────────────────────────────────────────
pub async fn ajouter_favori(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM marketplace.annonce
         WHERE id = $1 AND etat = 'publiee' AND deleted_at IS NULL)",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve(
            "Annonce introuvable ou non disponible".to_string(),
        ));
    }

    sqlx::query(
        "INSERT INTO marketplace.annonce_favori (utilisateur_id, annonce_id)
         VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(moi)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "marketplace",
        "annonce_favori",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "annonce_id": id, "favori": true })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// US4 — DELETE /api/annonces/{id}/favori : Retirer des favoris
// ──────────────────────────────────────────────────────────────
pub async fn retirer_favori(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();

    sqlx::query(
        "DELETE FROM marketplace.annonce_favori WHERE utilisateur_id = $1 AND annonce_id = $2",
    )
    .bind(moi)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "DELETE",
        "marketplace",
        "annonce_favori",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "annonce_id": id, "favori": false })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// US4 — GET /api/annonces/favoris : Mes favoris (annonces publiées)
// ──────────────────────────────────────────────────────────────
pub async fn mes_favoris(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<MembreListeParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM marketplace.annonce_favori f
         JOIN marketplace.annonce a ON a.id = f.annonce_id
         WHERE f.utilisateur_id = $1 AND a.etat = 'publiee' AND a.deleted_at IS NULL",
    )
    .bind(moi)
    .fetch_one(pool.get_ref())
    .await?;

    let query = format!(
        "SELECT {colonnes}
         FROM marketplace.annonce_favori f
         JOIN marketplace.annonce a ON a.id = f.annonce_id
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE f.utilisateur_id = $1 AND a.etat = 'publiee' AND a.deleted_at IS NULL
         ORDER BY f.created_at DESC
         LIMIT $2 OFFSET $3",
        colonnes = ANNONCE_LISTE_COLONNES
    );
    let rows = sqlx::query_as::<_, AnnonceListeRow>(&query)
        .bind(moi)
        .bind(par_page)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

    let total_count = total.0;
    let total_pages = if total_count == 0 {
        1
    } else {
        (total_count as f64 / par_page as f64).ceil() as i64
    };

    let reponse = AnnonceListeResponse {
        annonces: rows.iter().map(|r| r.to_response()).collect(),
        total: total_count,
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

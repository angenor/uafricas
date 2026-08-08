//! Interactions communautaires sur les médias radio et télé (US3) :
//! réactions, commentaires et partages vers le mur `/publications`.
//!
//! Les quatre types de média (`chaine_tv`, `station_radio`, `programme_tele`,
//! `programme_radio`) sont servis par les mêmes endpoints, discriminés par
//! `(type_media, media_id)` — cf. migration 09k. Calqué sur `element_social`,
//! qui rend le même service aux sous-objets afripulse.
//!
//! La LECTURE reste publique ; seule la participation exige un compte
//! (FR-027).

use actix_web::{web, HttpRequest, HttpResponse};
use std::collections::HashMap;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::media_social::{
    descripteur_pour_type, table_pour_type, AuteurApercu, CommentaireListeResponse,
    CommentaireMediaRequest, CommentaireMediaResponse, CommentaireMediaRow, CommentaireQueryParams,
    CompteursInteraction, CompteursRow, MediaApercu, PartageMediaListeResponse, PartageMediaQueryParams, PartageMediaRequest,
    PartageMediaResponse, PartageMediaRow, ReactionMediaEtat, ReactionMediaRequest,
    SignalementMediaEtat, SignalerMediaRequest, SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA,
    TYPES_MEDIA_AUTORISES,
};
use crate::services::audit;

#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ── Authentification ──────────────────────────────────────────────────

pub fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

fn exiger_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

// ── Validation de la cible ────────────────────────────────────────────

/// Confronte le type reçu à la whitelist AVANT toute interpolation SQL.
fn valider_type_media(type_media: &str) -> Result<(), ApiErreur> {
    if !TYPES_MEDIA_AUTORISES.contains(&type_media) {
        return Err(ApiErreur::Validation(format!(
            "Type de média « {} » non supporté",
            type_media
        )));
    }
    Ok(())
}

/// Le contenu doit exister ET être publié : on n'interagit pas avec un contenu
/// suspendu, en attente de validation ou retiré (FR-028).
async fn verifier_media_publie(
    pool: &PgPool,
    type_media: &str,
    media_id: Uuid,
) -> Result<(), ApiErreur> {
    valider_type_media(type_media)?;
    let table = table_pour_type(type_media)
        .ok_or_else(|| ApiErreur::Validation("Type de média non supporté".to_string()))?;

    let existe: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table}
                        WHERE id = $1 AND etat = 'publie' AND deleted_at IS NULL)"
    ))
    .bind(media_id)
    .fetch_one(pool)
    .await?;

    if !existe {
        // Un contenu retiré est indiscernable d'un contenu inexistant.
        return Err(ApiErreur::NonTrouve("Contenu introuvable".to_string()));
    }
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// RÉACTIONS — POST /api/medias/{type_media}/{media_id}/reaction
// ═══════════════════════════════════════════════════════════════════════════

/// Une seule réaction retenue par membre et par contenu (FR-023).
///
/// `type_reaction: null` retire la réaction ; réémettre la même réaction la
/// retire également, ce qui rend le bouton naturellement bascule côté interface.
pub async fn reagir_media(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<ReactionMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let (type_media, media_id) = chemin.into_inner();
    verifier_media_publie(pool.get_ref(), &type_media, media_id).await?;

    let demandee: Option<&str> = body
        .type_reaction
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty());

    if let Some(valeur) = demandee {
        if valeur != "like" && valeur != "dislike" {
            return Err(ApiErreur::Validation(
                "La réaction doit valoir « like » ou « dislike »".to_string(),
            ));
        }
    }

    let existante: Option<String> = sqlx::query_scalar(
        "SELECT type_reaction FROM media_content.media_reaction
          WHERE type_media = $1 AND media_id = $2 AND utilisateur_id = $3",
    )
    .bind(&type_media)
    .bind(media_id)
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let ma_reaction: Option<String> = match demandee {
        // Retrait explicite.
        None => {
            retirer_reaction(pool.get_ref(), &type_media, media_id, utilisateur_id).await?;
            None
        }
        // Réémettre la réaction déjà posée la retire : le bouton bascule.
        Some(voulue) if existante.as_deref() == Some(voulue) => {
            retirer_reaction(pool.get_ref(), &type_media, media_id, utilisateur_id).await?;
            None
        }
        Some(voulue) => {
            // Insertion ou changement d'avis, en une seule requête : l'unicité
            // (type_media, media_id, utilisateur_id) rend le conflit attendu.
            sqlx::query(
                "INSERT INTO media_content.media_reaction
                    (type_media, media_id, utilisateur_id, type_reaction)
                 VALUES ($1, $2, $3, $4)
                 ON CONFLICT (type_media, media_id, utilisateur_id)
                 DO UPDATE SET type_reaction = EXCLUDED.type_reaction, updated_at = NOW()",
            )
            .bind(&type_media)
            .bind(media_id)
            .bind(utilisateur_id)
            .bind(voulue)
            .execute(pool.get_ref())
            .await?;
            Some(voulue.to_string())
        }
    };

    let (nombre_likes, nombre_dislikes): (i64, i64) = sqlx::query_as(
        "SELECT COUNT(*) FILTER (WHERE type_reaction = 'like'),
                COUNT(*) FILTER (WHERE type_reaction = 'dislike')
           FROM media_content.media_reaction
          WHERE type_media = $1 AND media_id = $2",
    )
    .bind(&type_media)
    .bind(media_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Engagement : 1 point au PROPRIÉTAIRE du support par « j'aime » reçu.
    //
    // **Changement de bénéficiaire assumé** (FR-008a) : ce n'est plus `cree_par`
    // mais le détenteur déclaré du support, résolu par `resoudre_beneficiaire`.
    // Un support qui change de mains fait donc suivre ses points, ce que le
    // créateur figé ne permettait pas. Repli sur `cree_par` conservé pour les
    // supports créés avant l'existence de la co-détention.
    if ma_reaction.as_deref() == Some("like") {
        if let Some(beneficiaire_id) =
            crate::services::engagement::resoudre_beneficiaire(pool.get_ref(), &type_media, media_id)
                .await
        {
            crate::services::engagement::crediter_jaime(
                pool.get_ref(),
                &type_media,
                media_id,
                beneficiaire_id,
                utilisateur_id,
            )
            .await;
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ReactionMediaEtat {
        nombre_likes: nombre_likes as i32,
        nombre_dislikes: nombre_dislikes as i32,
        ma_reaction,
    }),
        error: None,
    }))
}

async fn retirer_reaction(
    pool: &PgPool,
    type_media: &str,
    media_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<(), ApiErreur> {
    sqlx::query(
        "DELETE FROM media_content.media_reaction
          WHERE type_media = $1 AND media_id = $2 AND utilisateur_id = $3",
    )
    .bind(type_media)
    .bind(media_id)
    .bind(utilisateur_id)
    .execute(pool)
    .await?;
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// COMMENTAIRES (FR-024)
// ═══════════════════════════════════════════════════════════════════════════

const COMMENTAIRE_SELECT: &str = "SELECT mc.id, mc.contenu, mc.created_at, mc.updated_at,
            u.id AS auteur_id,
            CASE WHEN u.deleted_at IS NULL THEN u.nom    ELSE 'Membre'  END AS auteur_nom,
            CASE WHEN u.deleted_at IS NULL THEN u.prenom ELSE 'retiré'  END AS auteur_prenom,
            CASE WHEN u.deleted_at IS NULL THEN u.photo_url ELSE NULL   END AS auteur_photo_url
       FROM media_content.media_commentaire mc
       JOIN iam.utilisateur u ON u.id = mc.auteur_id";

/// GET /api/medias/{type_media}/{media_id}/commentaires — lecture publique.
pub async fn lister_commentaires(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    params: web::Query<CommentaireQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let (type_media, media_id) = chemin.into_inner();
    valider_type_media(&type_media)?;

    let moi = extraire_utilisateur_id(&req);
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.media_commentaire
          WHERE type_media = $1 AND media_id = $2 AND deleted_at IS NULL",
    )
    .bind(&type_media)
    .bind(media_id)
    .fetch_one(pool.get_ref())
    .await?;

    let rows = sqlx::query_as::<_, CommentaireMediaRow>(&format!(
        "{COMMENTAIRE_SELECT}
          WHERE mc.type_media = $1 AND mc.media_id = $2 AND mc.deleted_at IS NULL
          ORDER BY mc.created_at DESC
          LIMIT $3 OFFSET $4"
    ))
    .bind(&type_media)
    .bind(media_id)
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(CommentaireListeResponse {
        commentaires: rows
            .into_iter()
            .map(|r| commentaire_depuis_row(r, moi))
            .collect(),
        total,
        page,
        par_page,
        total_pages,
    }),
        error: None,
    }))
}

/// POST /api/medias/{type_media}/{media_id}/commentaires
pub async fn commenter_media(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<CommentaireMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let (type_media, media_id) = chemin.into_inner();
    verifier_media_publie(pool.get_ref(), &type_media, media_id).await?;

    let contenu = body.contenu.trim();
    if contenu.is_empty() {
        return Err(ApiErreur::Validation(
            "Le commentaire ne peut pas être vide".to_string(),
        ));
    }
    if contenu.chars().count() > 2000 {
        return Err(ApiErreur::Validation(
            "Le commentaire ne doit pas dépasser 2000 caractères".to_string(),
        ));
    }

    let commentaire_id: Uuid = sqlx::query_scalar(
        "INSERT INTO media_content.media_commentaire (type_media, media_id, auteur_id, contenu)
         VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(&type_media)
    .bind(media_id)
    .bind(utilisateur_id)
    .bind(contenu)
    .fetch_one(pool.get_ref())
    .await?;

    let row = sqlx::query_as::<_, CommentaireMediaRow>(&format!(
        "{COMMENTAIRE_SELECT} WHERE mc.id = $1"
    ))
    .bind(commentaire_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(commentaire_depuis_row(
        row,
        Some(utilisateur_id),
    )),
        error: None,
    }))
}

/// DELETE /api/medias/commentaires/{id} — soft delete, auteur uniquement.
pub async fn supprimer_commentaire(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let commentaire_id = chemin.into_inner();

    let auteur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT auteur_id FROM media_content.media_commentaire
          WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(commentaire_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let auteur_id =
        auteur_id.ok_or_else(|| ApiErreur::NonTrouve("Commentaire introuvable".to_string()))?;

    if auteur_id != utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Seul l'auteur peut supprimer son commentaire".to_string(),
        ));
    }

    sqlx::query(
        "UPDATE media_content.media_commentaire
            SET deleted_at = NOW(), updated_at = NOW()
          WHERE id = $1",
    )
    .bind(commentaire_id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "supprime": true })),
        error: None,
    }))
}

fn commentaire_depuis_row(row: CommentaireMediaRow, moi: Option<Uuid>) -> CommentaireMediaResponse {
    CommentaireMediaResponse {
        id: row.id,
        contenu: row.contenu,
        created_at: row.created_at,
        updated_at: row.updated_at,
        est_mien: moi == Some(row.auteur_id),
        auteur: AuteurApercu {
            id: row.auteur_id,
            nom: row.auteur_nom,
            prenom: row.auteur_prenom,
            photo_url: row.auteur_photo_url,
        },
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PARTAGES (FR-025)
// ═══════════════════════════════════════════════════════════════════════════

/// POST /api/medias/{type_media}/{media_id}/partages
pub async fn partager_media(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<PartageMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let (type_media, media_id) = chemin.into_inner();
    verifier_media_publie(pool.get_ref(), &type_media, media_id).await?;

    let legende = body
        .legende
        .as_deref()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string);

    if let Some(ref l) = legende {
        if l.chars().count() > 500 {
            return Err(ApiErreur::Validation(
                "La légende ne doit pas dépasser 500 caractères".to_string(),
            ));
        }
    }

    let partage_id: Uuid = sqlx::query_scalar(
        "INSERT INTO media_content.partage_media (type_media, media_id, utilisateur_id, legende)
         VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(&type_media)
    .bind(media_id)
    .bind(utilisateur_id)
    .bind(&legende)
    .fetch_one(pool.get_ref())
    .await?;

    // Engagement : 1 point au propriétaire du support par partage reçu.
    // La clé de crédit ne porte pas le canal : ce même membre ne créditera plus,
    // qu'il partage ensuite vers WhatsApp, Facebook ou reposte à nouveau ici.
    if let Some(auteur_id) =
        crate::services::engagement::resoudre_beneficiaire(pool.get_ref(), &type_media, media_id)
            .await
    {
        crate::services::engagement::crediter_partage(
            pool.get_ref(),
            &type_media,
            media_id,
            auteur_id,
            utilisateur_id,
        )
        .await;
    }

    let row = sqlx::query_as::<_, PartageMediaRow>(&format!(
        "SELECT * FROM ({}) sub WHERE id = $1",
        union_partages()
    ))
    .bind(partage_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Partage introuvable".to_string()))?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(partage_depuis_row(row)),
        error: None,
    }))
}

/// GET /api/medias/partages — 8ᵉ source du mur communautaire, lecture publique.
pub async fn lister_partages_medias(
    pool: web::Data<PgPool>,
    params: web::Query<PartageMediaQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let total: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM ({}) sub",
        union_partages()
    ))
    .fetch_one(pool.get_ref())
    .await?;

    let rows = sqlx::query_as::<_, PartageMediaRow>(&format!(
        "SELECT * FROM ({}) sub ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        union_partages()
    ))
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PartageMediaListeResponse {
        partages: rows.into_iter().map(partage_depuis_row).collect(),
        total,
        page,
        par_page,
        total_pages,
    }),
        error: None,
    }))
}

// ── Construction de l'UNION des quatre types ──────────────────────────
// Le titre ne porte pas le même nom de colonne selon la table (`nom` pour les
// supports, `nom_emission` pour les contenus) : une seule requête ne peut pas
// les couvrir, d'où quatre fragments réunis par UNION ALL — patron
// `element_social::union_select`.

fn fragment_partages(type_media: &str) -> String {
    // `type_media` provient exclusivement de TYPES_MEDIA_AUTORISES, et le
    // descripteur ne renvoie que des littéraux → aucune entrée client n'est
    // interpolée ici.
    let d = descripteur_pour_type(type_media).expect("type de média supporté");
    format!(
        "SELECT pm.id, pm.legende, pm.created_at, pm.type_media, pm.media_id,
                m.{titre} AS titre, m.slug, m.image_couverture_url AS image_url,
                u.id AS auteur_id,
                CASE WHEN u.deleted_at IS NULL THEN u.nom    ELSE 'Membre' END AS auteur_nom,
                CASE WHEN u.deleted_at IS NULL THEN u.prenom ELSE 'retiré' END AS auteur_prenom,
                CASE WHEN u.deleted_at IS NULL THEN u.photo_url ELSE NULL  END AS auteur_photo_url
           FROM media_content.partage_media pm
           JOIN {table} m ON m.id = pm.media_id
           JOIN iam.utilisateur u ON u.id = pm.utilisateur_id
          WHERE pm.deleted_at IS NULL
            AND pm.type_media = '{type_media}'
            AND m.etat = 'publie' AND m.deleted_at IS NULL",
        titre = d.colonne_titre,
        table = d.table,
    )
}

fn union_partages() -> String {
    TYPES_MEDIA_AUTORISES
        .iter()
        .map(|t| fragment_partages(t))
        .collect::<Vec<_>>()
        .join(" UNION ALL ")
}

fn partage_depuis_row(row: PartageMediaRow) -> PartageMediaResponse {
    let url = descripteur_pour_type(&row.type_media)
        .zip(row.slug.as_ref())
        .map(|(d, slug)| format!("{}/{}", d.base_url, slug));

    PartageMediaResponse {
        id: row.id,
        legende: row.legende,
        created_at: row.created_at,
        media: MediaApercu {
            type_media: row.type_media,
            media_id: row.media_id,
            titre: row.titre,
            slug: row.slug,
            image_url: row.image_url,
            url,
        },
        auteur: AuteurApercu {
            id: row.auteur_id,
            nom: row.auteur_nom,
            prenom: row.auteur_prenom,
            photo_url: row.auteur_photo_url,
        },
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMPTEURS AGRÉGÉS (FR-027)
// ═══════════════════════════════════════════════════════════════════════════

/// Compteurs d'interaction d'un lot de contenus, en UNE requête.
///
/// Appelé par les endpoints de vedette, de sections et de détail : sans lui,
/// chaque carte affichée déclencherait son propre aller-retour. `moi` renseigne
/// `ma_reaction` pour un membre connecté ; il vaut `None` pour un visiteur.
pub async fn compteurs_pour(
    pool: &PgPool,
    type_media: &str,
    ids: &[Uuid],
    moi: Option<Uuid>,
) -> Result<HashMap<Uuid, CompteursInteraction>, ApiErreur> {
    if ids.is_empty() {
        return Ok(HashMap::new());
    }
    valider_type_media(type_media)?;

    let rows = sqlx::query_as::<_, CompteursRow>(
        "SELECT cible.id AS media_id,
                COALESCE(r.likes, 0)        AS nombre_likes,
                COALESCE(r.dislikes, 0)     AS nombre_dislikes,
                COALESCE(c.total, 0)        AS nombre_commentaires,
                COALESCE(p.total, 0)        AS nombre_partages,
                mienne.type_reaction        AS ma_reaction
           FROM unnest($2::uuid[]) AS cible(id)
           LEFT JOIN LATERAL (
                SELECT COUNT(*) FILTER (WHERE type_reaction = 'like')    AS likes,
                       COUNT(*) FILTER (WHERE type_reaction = 'dislike') AS dislikes
                  FROM media_content.media_reaction
                 WHERE type_media = $1 AND media_id = cible.id
           ) r ON TRUE
           LEFT JOIN LATERAL (
                SELECT COUNT(*) AS total FROM media_content.media_commentaire
                 WHERE type_media = $1 AND media_id = cible.id AND deleted_at IS NULL
           ) c ON TRUE
           LEFT JOIN LATERAL (
                SELECT COUNT(*) AS total FROM media_content.partage_media
                 WHERE type_media = $1 AND media_id = cible.id AND deleted_at IS NULL
           ) p ON TRUE
           LEFT JOIN media_content.media_reaction mienne
                  ON mienne.type_media = $1 AND mienne.media_id = cible.id
                 AND mienne.utilisateur_id = $3",
    )
    .bind(type_media)
    .bind(ids)
    .bind(moi)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| {
            (
                r.media_id,
                CompteursInteraction {
                    nombre_likes: r.nombre_likes as i32,
                    nombre_dislikes: r.nombre_dislikes as i32,
                    nombre_commentaires: r.nombre_commentaires as i32,
                    nombre_partages: r.nombre_partages as i32,
                    ma_reaction: r.ma_reaction,
                },
            )
        })
        .collect())
}

// ═══════════════════════════════════════════════════════════════════════════
// SIGNALEMENT — POST /api/medias/{type_media}/{media_id}/signalement (US7)
// ═══════════════════════════════════════════════════════════════════════════

/// Signale un contenu contraire aux règles (FR-049, FR-050).
///
/// Idempotent par `uq_signalement_media_membre` : un même membre ne peut pas
/// faire croître le compteur en signalant plusieurs fois. Au-delà du seuil, le
/// contenu bascule en `etat = 'suspendu'` — il disparaît alors de toutes les
/// pages publiques à la requête suivante, sans intervention humaine (SC-009).
///
/// La bascule ne va jamais dans l'autre sens : la désuspension est
/// exclusivement administrative et remet le compteur à zéro (FR-051).
pub async fn signaler_media(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<SignalerMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let (type_media, media_id) = chemin.into_inner();

    // Un contenu déjà suspendu est indiscernable d'un contenu inexistant : il
    // n'est plus servi publiquement, donc plus signalable.
    verifier_media_publie(pool.get_ref(), &type_media, media_id).await?;
    let table = table_pour_type(&type_media)
        .ok_or_else(|| ApiErreur::Validation("Type de média non supporté".to_string()))?;

    let motif = body
        .motif
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let description = body
        .description
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    if let Some(texte) = description {
        if texte.chars().count() > 1000 {
            return Err(ApiErreur::Validation(
                "La description ne peut pas dépasser 1000 caractères".to_string(),
            ));
        }
    }

    // 1. Insertion idempotente : un signalement au plus par membre et par contenu.
    let resultat = sqlx::query(
        "INSERT INTO media_content.signalement_media
            (type_media, media_id, signale_par, motif, description)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (type_media, media_id, signale_par) DO NOTHING",
    )
    .bind(&type_media)
    .bind(media_id)
    .bind(utilisateur_id)
    .bind(motif)
    .bind(description)
    .execute(pool.get_ref())
    .await?;
    let nouveau_signalement = resultat.rows_affected() > 0;

    // 2. Recompte des signalements DISTINCTS (l'unicité SQL le garantit).
    let nombre: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.signalement_media
          WHERE type_media = $1 AND media_id = $2",
    )
    .bind(&type_media)
    .bind(media_id)
    .fetch_one(pool.get_ref())
    .await?;

    let doit_suspendre = nombre > SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA;

    // 3. Compteur dénormalisé + bascule d'état. `etat` est une colonne texte sur
    //    ces quatre tables — il n'y existe pas de colonne booléenne `suspendu`.
    let suspendu: bool = sqlx::query_scalar(&format!(
        "UPDATE {table}
            SET etat = CASE WHEN $3 AND etat = 'publie' THEN 'suspendu' ELSE etat END,
                nombre_signalements = $2,
                updated_at = NOW()
          WHERE id = $1
      RETURNING etat = 'suspendu'"
    ))
    .bind(media_id)
    .bind(nombre as i32)
    .bind(doit_suspendre)
    .fetch_one(pool.get_ref())
    .await?;

    // 4. Audit — uniquement sur un signalement réellement nouveau, un doublon
    //    n'étant pas une mutation (FR-055).
    if nouveau_signalement {
        let ip = audit::extraire_ip(&req);
        let ua = audit::extraire_user_agent(&req);
        let action = if suspendu && doit_suspendre {
            "SIGNALEMENT_SUSPENSION"
        } else {
            "SIGNALEMENT"
        };
        audit::log_action(
            pool.get_ref(),
            Some(utilisateur_id),
            action,
            "media_content",
            &type_media,
            Some(media_id),
            Some(serde_json::json!({
                "etat": "publie",
                "nombre_signalements": nombre - 1,
            })),
            Some(serde_json::json!({
                "etat": if suspendu { "suspendu" } else { "publie" },
                "nombre_signalements": nombre,
                "motif": motif,
            })),
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
    }

    if doit_suspendre && suspendu {
        log::warn!(
            "Média {type_media}/{media_id} suspendu automatiquement ({nombre} signalements)"
        );
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SignalementMediaEtat {
            nombre_signalements: nombre as i32,
            suspendu,
            deja_signale: !nouveau_signalement,
        }),
        error: None,
    }))
}

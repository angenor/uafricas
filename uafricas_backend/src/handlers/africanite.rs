//! Africanité : publications éphémères (spec 012, périmètre P1).
//!
//! Quatre routes : lister ce que le lecteur a le droit de voir, publier sous
//! l'une des trois formes, et marquer une africanité comme vue.

use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, web};
use chrono::{Duration, Utc};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::collections::BTreeMap;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::africanite::{
    AFRICANITE_COLONNES, AfricaniteRow, AuteurAfricanitesResponse, CreerAfricaniteTexteBody,
    DUREE_VIE_HEURES, LEGENDE_MAX, TEXTE_MAX, construire_africanite_response,
};
use crate::services::{audit, image_validation};

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

fn exiger_utilisateur(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))
}

/// Taille maximale d'une vidéo courte. La plateforme ne transcode rien : la
/// borne est le seul garde-fou, et elle est volontairement basse.
const VIDEO_MAX_OCTETS: usize = 15 * 1024 * 1024;

// ══════════════════════════════════════════════════════════════════════════
// GET /api/africanites
// ══════════════════════════════════════════════════════════════════════════

/// Liste les africanités visibles du lecteur, groupées par auteur.
///
/// Le public est le cercle d'ami(e)s (décision Q1), constaté **à l'instant de
/// la lecture** : `social.amitie` est jointe ici même, si bien qu'une rupture
/// d'amitié coupe l'accès sans qu'aucun traitement n'ait à passer.
///
/// L'échéance est constatée de la même façon : `expire_at > NOW()`. Rien ne
/// s'exécute à l'heure dite.
pub async fn lister_africanites(
    pool: web::Data<PgPool>,
    req: HttpRequest) -> Result<HttpResponse, ApiErreur> {
    let lecteur_id = exiger_utilisateur(&req)?;

    let requete = format!(
        "SELECT {cols},
                u.nom  AS auteur_nom,
                u.prenom AS auteur_prenom,
                u.photo_url AS auteur_photo_url,
                EXISTS(SELECT 1 FROM social.africanite_vue v
                        WHERE v.africanite_id = a.id AND v.utilisateur_id = $1) AS vue,
                (SELECT COUNT(*) FROM social.africanite_vue v2
                  WHERE v2.africanite_id = a.id) AS nombre_vues
           FROM social.africanite a
           JOIN iam.utilisateur u ON u.id = a.auteur_id
          WHERE a.deleted_at IS NULL
            AND a.expire_at > NOW()
            AND u.deleted_at IS NULL
            AND u.etat = 'actif'
            AND (
                  a.auteur_id = $1
               OR EXISTS (SELECT 1 FROM social.amitie am
                           WHERE (am.utilisateur_a_id = $1 AND am.utilisateur_b_id = a.auteur_id)
                              OR (am.utilisateur_b_id = $1 AND am.utilisateur_a_id = a.auteur_id))
            )
          ORDER BY a.auteur_id, a.created_at",
        cols = AFRICANITE_COLONNES
    );

    let lignes: Vec<AfricaniteRow> = sqlx::query_as(&requete)
        .bind(lecteur_id)
        .fetch_all(pool.get_ref())
        .await?;

    // Regroupement par auteur. `BTreeMap` garde un ordre déterministe, ce qu'un
    // `HashMap` ne ferait pas : la rangée sauterait d'un rechargement à l'autre.
    let mut par_auteur: BTreeMap<Uuid, AuteurAfricanitesResponse> = BTreeMap::new();
    for ligne in &lignes {
        let est_auteur = ligne.auteur_id == lecteur_id;
        let entree = par_auteur
            .entry(ligne.auteur_id)
            .or_insert_with(|| AuteurAfricanitesResponse {
                auteur_id: ligne.auteur_id,
                nom: ligne.auteur_nom.clone(),
                prenom: ligne.auteur_prenom.clone(),
                photo_url: ligne.auteur_photo_url.clone(),
                est_moi: est_auteur,
                a_du_nouveau: false,
                africanites: Vec::new(),
            });
        if !ligne.vue {
            entree.a_du_nouveau = true;
        }
        entree
            .africanites
            .push(construire_africanite_response(ligne, est_auteur));
    }

    // Ordre de la rangée : le lecteur d'abord, puis ceux qui ont du nouveau,
    // puis les autres. C'est la règle FR-008 : l'anneau ne sert à rien si les
    // non vues sont noyées au milieu.
    let mut groupes: Vec<AuteurAfricanitesResponse> = par_auteur.into_values().collect();
    groupes.sort_by_key(|g| (!g.est_moi, !g.a_du_nouveau));

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(groupes),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// POST /api/africanites/texte
// ══════════════════════════════════════════════════════════════════════════

/// Publie une africanité de la forme `texte`, quelques mots sur un fond
/// coloré. Aucun fichier n'est déposé : c'est la forme qui permet de publier
/// sans photo sous la main.
pub async fn creer_africanite_texte(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    corps: web::Json<CreerAfricaniteTexteBody>) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur(&req)?;

    let texte = corps.texte.trim();
    if texte.is_empty() {
        return Err(ApiErreur::Validation(
            "Un fond coloré sans texte ne dit rien : le texte est obligatoire.".into()));
    }
    if texte.chars().count() > TEXTE_MAX {
        return Err(ApiErreur::Validation(format!(
            "Texte trop long ({} caractères, {} au maximum).",
            texte.chars().count(),
            TEXTE_MAX
        )));
    }
    if let Some(l) = corps.legende.as_deref()
        && l.chars().count() > LEGENDE_MAX
    {
        return Err(ApiErreur::Validation(format!(
            "Légende trop longue ({} caractères, {} au maximum).",
            l.chars().count(),
            LEGENDE_MAX
        )));
    }

    let expire_at = Utc::now() + Duration::hours(DUREE_VIE_HEURES);

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO social.africanite (auteur_id, forme, texte, couleur_fond, legende, expire_at)
         VALUES ($1, 'texte', $2, $3, $4, $5) RETURNING id")
    .bind(auteur_id)
    .bind(texte)
    .bind(corps.couleur_fond.as_deref().unwrap_or("#A74916"))
    .bind(corps.legende.as_deref().map(str::trim).filter(|s| !s.is_empty()))
    .bind(expire_at)
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "create",
        "social",
        "africanite",
        Some(id),
        None,
        Some(serde_json::json!({ "forme": "texte", "expire_at": expire_at })),
        ip.as_deref(),
        ua.as_deref())
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "expire_at": expire_at })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// POST /api/africanites/media
// ══════════════════════════════════════════════════════════════════════════

/// Publie une africanité de la forme `image` ou `video`.
///
/// Le fichier est validé EN MÉMOIRE avant toute écriture disque, et la taille
/// est bornée pendant la lecture du flux, refuser après avoir tout reçu
/// reviendrait à accepter le téléversement qu'on prétend refuser.
pub async fn creer_africanite_media(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    req: HttpRequest,
    mut payload: Multipart) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur(&req)?;

    let mut bytes: Option<Vec<u8>> = None;
    let mut forme = String::new();
    let mut legende: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|e| ApiErreur::Upload(format!("Erreur lecture multipart: {}", e)))?;
        let nom = field
            .content_disposition()
            .and_then(|c| c.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        match nom.as_str() {
            "media" => {
                let mut buf: Vec<u8> = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk
                        .map_err(|e| ApiErreur::Upload(format!("Erreur lecture chunk: {}", e)))?;
                    buf.extend_from_slice(&data);
                    if buf.len() > VIDEO_MAX_OCTETS + 1 {
                        return Err(ApiErreur::LimiteAtteinte(format!(
                            "Fichier trop volumineux (>{} Mo)",
                            VIDEO_MAX_OCTETS / (1024 * 1024)
                        )));
                    }
                }
                bytes = Some(buf);
            }
            "forme" | "legende" => {
                let mut buf: Vec<u8> = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk
                        .map_err(|e| ApiErreur::Upload(format!("Erreur lecture chunk: {}", e)))?;
                    buf.extend_from_slice(&data);
                }
                let valeur = String::from_utf8_lossy(&buf).trim().to_string();
                if nom == "forme" {
                    forme = valeur;
                } else if !valeur.is_empty() {
                    legende = Some(valeur);
                }
            }
            _ => {}
        }
    }

    if forme != "image" && forme != "video" {
        return Err(ApiErreur::Validation(
            "Champ `forme` attendu : « image » ou « video ».".into()));
    }
    let bytes = bytes
        .ok_or_else(|| ApiErreur::Validation("Aucun fichier reçu (champ `media`).".into()))?;
    if let Some(l) = legende.as_deref()
        && l.chars().count() > LEGENDE_MAX
    {
        return Err(ApiErreur::Validation(format!(
            "Légende trop longue ({} caractères, {} au maximum).",
            l.chars().count(),
            LEGENDE_MAX
        )));
    }

    // L'image est validée par la crate `image` comme partout ailleurs. La vidéo
    // ne l'est PAS : rien dans le dépôt ne sait inspecter un conteneur vidéo, et
    // prétendre la valider serait un contrôle de façade. Seule sa taille est
    // bornée : c'est une limite connue, pas un oubli.
    let extension = if forme == "image" {
        let dim = image_validation::valider_photo_contribution(&bytes)
            .map_err(|err| ApiErreur::Validation(err.message()))?;
        dim.format.extension().to_string()
    } else {
        "mp4".to_string()
    };

    let dossier = format!("{}/africanites", upload_dir.get_ref());
    std::fs::create_dir_all(&dossier)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le dossier: {}", e)))?;

    let nom_fichier = format!("{}.{}", Uuid::new_v4(), extension);
    let chemin_complet = format!("{}/{}", dossier, nom_fichier);
    let chemin_relatif = format!("/uploads/africanites/{}", nom_fichier);

    let ecriture = (|| -> std::io::Result<()> {
        let mut f = std::fs::File::create(&chemin_complet)?;
        f.write_all(&bytes)?;
        Ok(())
    })();
    if let Err(e) = ecriture {
        return Err(ApiErreur::Upload(format!("Ecriture disque echouee: {}", e)));
    }

    let expire_at = Utc::now() + Duration::hours(DUREE_VIE_HEURES);

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO social.africanite (auteur_id, forme, media_url, legende, expire_at)
         VALUES ($1, $2, $3, $4, $5) RETURNING id")
    .bind(auteur_id)
    .bind(&forme)
    .bind(&chemin_relatif)
    .bind(legende.as_deref())
    .bind(expire_at)
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "create",
        "social",
        "africanite",
        Some(id),
        None,
        Some(serde_json::json!({ "forme": &forme, "expire_at": expire_at })),
        ip.as_deref(),
        ua.as_deref())
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": id,
            "media_url": chemin_relatif,
            "expire_at": expire_at
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// POST /api/africanites/{id}/vue
// ══════════════════════════════════════════════════════════════════════════

/// Marque une africanité comme vue par le lecteur courant.
///
/// `ON CONFLICT DO NOTHING` : regarder deux fois ne compte qu'une fois, et
/// c'est la clé primaire composite qui le garantit, pas un contrôle applicatif
/// qu'un appel concurrent contournerait.
pub async fn marquer_vue(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>) -> Result<HttpResponse, ApiErreur> {
    let lecteur_id = exiger_utilisateur(&req)?;
    let africanite_id = chemin.into_inner();

    // On ne marque que ce que le lecteur a le droit de voir : sans ce contrôle,
    // un identifiant deviné permettrait de gonfler le compteur d'un inconnu.
    let visible: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM social.africanite a
             WHERE a.id = $1 AND a.deleted_at IS NULL AND a.expire_at > NOW()
               AND (a.auteur_id = $2
                 OR EXISTS (SELECT 1 FROM social.amitie am
                             WHERE (am.utilisateur_a_id = $2 AND am.utilisateur_b_id = a.auteur_id)
                                OR (am.utilisateur_b_id = $2 AND am.utilisateur_a_id = a.auteur_id)))
         )")
    .bind(africanite_id)
    .bind(lecteur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !visible {
        return Err(ApiErreur::NonTrouve(
            "Africanité introuvable ou expirée.".into()));
    }

    sqlx::query(
        "INSERT INTO social.africanite_vue (africanite_id, utilisateur_id)
         VALUES ($1, $2) ON CONFLICT DO NOTHING")
    .bind(africanite_id)
    .bind(lecteur_id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "vue": true })),
        error: None,
    }))
}

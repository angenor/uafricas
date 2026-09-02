//! Handlers publics : ressources contribuées par la communauté au niveau salle
//! et workflow accompagnateur (feature 001-ressources-fermeture-session).
//!
//! Endpoints :
//!   - `GET    /api/afrolang/salles/{salle_id}/ressources-contribuees`
//!   - `POST   /api/afrolang/salles/{salle_id}/ressources-contribuees` (multipart OU JSON)
//!   - `DELETE /api/afrolang/ressources-contribuees/{id}`
//!   - `GET    /api/afrolang/accompagnateur/recommandations-recues`
//!   - `POST   /api/afrolang/ressources-contribuees/{id}/accepter`
//!   - `POST   /api/afrolang/ressources-contribuees/{id}/refuser`
//!   - `POST   /api/afrolang/ressources-contribuees/{id}/retirer-consentement`
//!
//! Voir contracts/public-ressources.md et contracts/public-accompagnateur.md.

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::afrolang::a_acces_salle_privee_actif;
use crate::jwt;
use crate::models::notification;
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::models::ressource_contribuee::{
    AccompagnateurPublicInfo, AuteurLight, RessourceContribueeResponse, StatutAccompagnateur,
    TypeRessourceContribuee,
};
use crate::services::{audit, rate_limit_ressources, youtube_url};
use crate::ApiResponse;

const UPLOAD_SUBDIR: &str = "afrolang/ressources_contribuees";
const TAILLE_MAX_FICHIER: usize = 20 * 1024 * 1024; // 20 Mo
const MIMES_AUTORISES: &[&str] = &[
    "application/pdf",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.oasis.opendocument.text",
];
const EXTENSIONS_AUTORISEES: &[&str] = &["pdf", "doc", "docx", "odt"];

// ──────────────────────────────────────────────────────────────────────────
// Helpers locaux
// ──────────────────────────────────────────────────────────────────────────

fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

async fn verifier_admin_plateforme(pool: &PgPool, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.utilisateur_role ur
            JOIN iam.role r ON ur.role_id = r.id
            WHERE ur.utilisateur_id = $1 AND r.slug = 'admin'
        )",
    )
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(is_admin)
}

/// Vérifie que l'utilisateur courant existe en base et que son état est `actif`.
async fn verifier_utilisateur_actif(pool: &PgPool, utilisateur_id: Uuid) -> Result<(), ApiErreur> {
    let etat: Option<String> = sqlx::query_scalar(
        "SELECT etat::TEXT FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?;
    match etat.as_deref() {
        Some("actif") => Ok(()),
        Some(_) => Err(ApiErreur::AccesInterdit("Compte non actif".into())),
        None => Err(ApiErreur::NonAutorise("Utilisateur inexistant".into())),
    }
}

/// Charge la salle publique cible et renvoie 404 si supprimée ou absente.
/// Renvoie aussi `desactivee_admin_at` pour le contrôle d'écriture (409).
async fn charger_salle_pour_ecriture(
    pool: &PgPool,
    salle_id: Uuid,
) -> Result<Option<DateTime<Utc>>, ApiErreur> {
    let row: Option<(Option<DateTime<Utc>>,)> = sqlx::query_as(
        "SELECT desactivee_admin_at
         FROM afrolang.salle
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(salle_id)
    .fetch_optional(pool)
    .await?;
    match row {
        Some((desact,)) => Ok(desact),
        None => Err(ApiErreur::NonTrouve("Salle introuvable".into())),
    }
}

/// Vérifie qu'une session, si fournie, appartient bien à `salle_id` (directement
/// pour les sessions publiques, ou via la salle privée parente). Retourne aussi
/// l'éventuel `salle_privee_id` parent, utile pour le contrôle d'accès.
async fn resoudre_session_origine(
    pool: &PgPool,
    salle_id: Uuid,
    session_origine_id: Option<Uuid>,
) -> Result<Option<Uuid>, ApiErreur> {
    let Some(session_id) = session_origine_id else {
        return Ok(None);
    };
    let row: Option<(Option<Uuid>, Option<Uuid>)> = sqlx::query_as(
        "SELECT salle_id, salle_privee_id FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;
    let Some((session_salle_id, session_salle_privee_id)) = row else {
        return Err(ApiErreur::Validation(
            "Session d'origine introuvable".into(),
        ));
    };
    // Cas 1 : session attachée directement à une salle publique.
    if session_salle_id == Some(salle_id) {
        return Ok(None);
    }
    // Cas 2 : session de salle privée : la salle privée doit pointer vers `salle_id`.
    if let Some(spi) = session_salle_privee_id {
        let parent_id: Option<Uuid> = sqlx::query_scalar(
            "SELECT salle_id FROM afrolang.salle_privee WHERE id = $1",
        )
        .bind(spi)
        .fetch_optional(pool)
        .await?;
        if parent_id == Some(salle_id) {
            return Ok(Some(spi));
        }
    }
    Err(ApiErreur::Validation(
        "Session d'origine sans rattachement à cette salle".into(),
    ))
}

/// Verrouille l'accès en écriture aux ressources de salle privée :
/// si `salle_privee_id` est `Some`, l'utilisateur doit avoir un accès actif.
async fn verifier_acces_salle_privee_si_necessaire(
    pool: &PgPool,
    salle_privee_id: Option<Uuid>,
    utilisateur_id: Uuid,
) -> Result<(), ApiErreur> {
    if let Some(spi) = salle_privee_id {
        let ok = a_acces_salle_privee_actif(pool, spi, utilisateur_id).await?;
        if !ok {
            return Err(ApiErreur::AccesInterdit(
                "Accès à la salle privée requis".into(),
            ));
        }
    }
    Ok(())
}

/// Applique le rate-limit ressources : 10 / utilisateur / salle / 24h.
async fn verifier_rate_limit(pool: &PgPool, auteur_id: Uuid, salle_id: Uuid) -> Result<(), ApiErreur> {
    let count = rate_limit_ressources::compter_ressources_recentes(pool, auteur_id, salle_id).await?;
    if count >= rate_limit_ressources::LIMITE_24H {
        return Err(ApiErreur::LimiteAtteinte(format!(
            "Limite atteinte : {} ressources / 24 h pour cette salle",
            rate_limit_ressources::LIMITE_24H
        )));
    }
    Ok(())
}

/// Construit le DTO public à partir d'une ligne enrichie (auteur + recommandé optionnel).
#[allow(clippy::too_many_arguments)]
fn construire_response(
    id: Uuid,
    type_: TypeRessourceContribuee,
    titre: String,
    description: Option<String>,
    auteur: AuteurLight,
    session_origine_id: Option<Uuid>,
    fichier_url: Option<String>,
    fichier_taille_octets: Option<i64>,
    video_id_youtube: Option<String>,
    video_url: Option<String>,
    lien_url: Option<String>,
    accompagnateur: Option<AccompagnateurPublicInfo>,
    created_at: DateTime<Utc>,
) -> RessourceContribueeResponse {
    RessourceContribueeResponse {
        id,
        r#type: type_,
        titre,
        description,
        auteur,
        session_origine_id,
        fichier_url,
        fichier_taille_octets,
        video_id_youtube,
        video_url,
        lien_url,
        accompagnateur,
        created_at,
    }
}

#[derive(sqlx::FromRow)]
struct RessourceContribueeRow {
    id: Uuid,
    r#type: TypeRessourceContribuee,
    titre: String,
    description: Option<String>,
    auteur_id: Uuid,
    auteur_nom: String,
    auteur_prenom: String,
    auteur_avatar_url: Option<String>,
    session_origine_id: Option<Uuid>,
    fichier_url: Option<String>,
    fichier_taille_octets: Option<i64>,
    video_id_youtube: Option<String>,
    video_url: Option<String>,
    lien_url: Option<String>,
    membre_recommande_id: Option<Uuid>,
    membre_recommande_nom: Option<String>,
    membre_recommande_prenom: Option<String>,
    membre_recommande_avatar_url: Option<String>,
    motif_recommandation: Option<String>,
    statut_accompagnateur: Option<StatutAccompagnateur>,
    created_at: DateTime<Utc>,
}

impl RessourceContribueeRow {
    fn vers_response(self) -> RessourceContribueeResponse {
        let auteur = AuteurLight {
            id: self.auteur_id,
            nom: self.auteur_nom,
            prenom: self.auteur_prenom,
            avatar_url: self.auteur_avatar_url,
        };
        let accompagnateur = match (
            self.membre_recommande_id,
            self.membre_recommande_nom.clone(),
            self.membre_recommande_prenom.clone(),
            self.motif_recommandation.clone(),
            self.statut_accompagnateur,
        ) {
            (Some(id), Some(nom), Some(prenom), Some(motif), Some(statut)) => {
                Some(AccompagnateurPublicInfo {
                    membre: AuteurLight {
                        id,
                        nom,
                        prenom,
                        avatar_url: self.membre_recommande_avatar_url.clone(),
                    },
                    motif,
                    statut,
                })
            }
            _ => None,
        };
        construire_response(
            self.id,
            self.r#type,
            self.titre,
            self.description,
            auteur,
            self.session_origine_id,
            self.fichier_url,
            self.fichier_taille_octets,
            self.video_id_youtube,
            self.video_url,
            self.lien_url,
            accompagnateur,
            self.created_at,
        )
    }
}

const COLONNES_AVEC_JOINS: &str =
    "rc.id, rc.type, rc.titre, rc.description,
     rc.auteur_id, ua.nom AS auteur_nom, ua.prenom AS auteur_prenom, ua.photo_url AS auteur_avatar_url,
     rc.session_origine_id, rc.fichier_url, rc.fichier_taille_octets,
     rc.video_id_youtube, rc.video_url, rc.lien_url,
     rc.membre_recommande_id, um.nom AS membre_recommande_nom, um.prenom AS membre_recommande_prenom,
     um.photo_url AS membre_recommande_avatar_url, rc.motif_recommandation,
     rc.statut_accompagnateur, rc.created_at";

const FROM_CLAUSE: &str =
    "FROM afrolang.ressource_contribuee rc
     JOIN iam.utilisateur ua ON ua.id = rc.auteur_id
     LEFT JOIN iam.utilisateur um ON um.id = rc.membre_recommande_id";

// ──────────────────────────────────────────────────────────────────────────
// GET /api/afrolang/salles/{salle_id}/ressources-contribuees
// ──────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ListerRessourcesQuery {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub r#type: Option<String>,
}

pub async fn lister_ressources_contribuees(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    query: web::Query<ListerRessourcesQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let salle_id = chemin.into_inner();

    // Vérifier l'existence (404 si soft-deleted)
    let _ = charger_salle_pour_ecriture(pool.get_ref(), salle_id).await?;

    let pagination = PaginationParams {
        page: query.page,
        par_page: query.par_page,
        tri_par: None,
        tri_dir: None,
    };

    let me = extraire_utilisateur_id(&req);

    // Filtrage visibilité accompagnateur
    let visibilite_clause = match me {
        Some(uid) => format!(
            "(rc.type <> 'accompagnateur'
              OR rc.statut_accompagnateur = 'acceptee'
              OR rc.auteur_id = '{}'
              OR rc.membre_recommande_id = '{}')",
            uid, uid
        ),
        None => "(rc.type <> 'accompagnateur' OR rc.statut_accompagnateur = 'acceptee')".to_string(),
    };

    let mut clauses: Vec<String> = vec![
        "rc.salle_id = $1".to_string(),
        "rc.deleted_at IS NULL".to_string(),
        visibilite_clause,
    ];

    let mut type_filtre: Option<TypeRessourceContribuee> = None;
    if let Some(t) = query.r#type.as_deref() {
        type_filtre = Some(match t {
            "document" => TypeRessourceContribuee::Document,
            "video_youtube" => TypeRessourceContribuee::VideoYoutube,
            "accompagnateur" => TypeRessourceContribuee::Accompagnateur,
            "lien_web" => TypeRessourceContribuee::LienWeb,
            autre => {
                return Err(ApiErreur::Validation(format!(
                    "Type inconnu: {}",
                    autre
                )));
            }
        });
        clauses.push("rc.type = $2".to_string());
    }

    let where_clause = clauses.join(" AND ");

    // COUNT total
    let count_sql = format!(
        "SELECT COUNT(*)::BIGINT FROM afrolang.ressource_contribuee rc WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_as::<_, (i64,)>(&count_sql).bind(salle_id);
    if let Some(t) = type_filtre {
        count_q = count_q.bind(t);
    }
    let (total,) = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT page
    let select_sql = format!(
        "SELECT {cols} {from} WHERE {where_} ORDER BY rc.created_at DESC LIMIT ${lim} OFFSET ${off}",
        cols = COLONNES_AVEC_JOINS,
        from = FROM_CLAUSE,
        where_ = where_clause,
        lim = if type_filtre.is_some() { 3 } else { 2 },
        off = if type_filtre.is_some() { 4 } else { 3 },
    );
    let mut q = sqlx::query_as::<_, RessourceContribueeRow>(&select_sql).bind(salle_id);
    if let Some(t) = type_filtre {
        q = q.bind(t);
    }
    q = q.bind(pagination.par_page()).bind(pagination.offset());
    let rows = q.fetch_all(pool.get_ref()).await?;

    let data: Vec<RessourceContribueeResponse> =
        rows.into_iter().map(|r| r.vers_response()).collect();
    let paginated = PaginatedResponse::from_params(data, total, &pagination);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(paginated),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────────────────
// POST /api/afrolang/salles/{salle_id}/ressources-contribuees
//   - multipart/form-data → variant `document`
//   - application/json    → variants `video_youtube`, `lien_web`, `accompagnateur`
// ──────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AjouterRessourceJsonBody {
    VideoYoutube {
        titre: String,
        description: Option<String>,
        session_origine_id: Option<Uuid>,
        video_url: String,
    },
    LienWeb {
        titre: String,
        description: Option<String>,
        session_origine_id: Option<Uuid>,
        lien_url: String,
    },
    Accompagnateur {
        titre: String,
        description: Option<String>,
        session_origine_id: Option<Uuid>,
        membre_recommande_id: Uuid,
        motif_recommandation: String,
    },
}

/// Pré-vérifications communes (utilisateur actif, salle vivante, rate-limit, accès privée).
async fn pre_check_ecriture(
    pool: &PgPool,
    salle_id: Uuid,
    utilisateur_id: Uuid,
    session_origine_id: Option<Uuid>,
) -> Result<Option<Uuid>, ApiErreur> {
    verifier_utilisateur_actif(pool, utilisateur_id).await?;

    let desact = charger_salle_pour_ecriture(pool, salle_id).await?;
    if desact.is_some() {
        return Err(ApiErreur::Conflit("Salle désactivée par administration".into()));
    }

    let salle_privee_id = resoudre_session_origine(pool, salle_id, session_origine_id).await?;
    verifier_acces_salle_privee_si_necessaire(pool, salle_privee_id, utilisateur_id).await?;

    verifier_rate_limit(pool, utilisateur_id, salle_id).await?;
    Ok(salle_privee_id)
}

fn valider_titre(titre: &str) -> Result<String, ApiErreur> {
    let t = titre.trim();
    if t.is_empty() || t.chars().count() > 120 {
        return Err(ApiErreur::Validation(
            "Titre requis (1..120 caractères)".into(),
        ));
    }
    Ok(t.to_string())
}

fn valider_description(description: Option<String>) -> Result<Option<String>, ApiErreur> {
    Ok(match description {
        Some(d) => {
            let t = d.trim();
            if t.is_empty() {
                None
            } else if t.chars().count() > 500 {
                return Err(ApiErreur::Validation(
                    "Description ≤ 500 caractères".into(),
                ));
            } else {
                Some(t.to_string())
            }
        }
        None => None,
    })
}

fn audit_create_ressource(record_id: Uuid, after: serde_json::Value) -> serde_json::Value {
    serde_json::json!({ "id": record_id, "after": after })
}

async fn finaliser_lecture_unique(
    pool: &PgPool,
    record_id: Uuid,
) -> Result<RessourceContribueeResponse, ApiErreur> {
    let sql = format!(
        "SELECT {cols} {from} WHERE rc.id = $1",
        cols = COLONNES_AVEC_JOINS,
        from = FROM_CLAUSE,
    );
    let row: RessourceContribueeRow = sqlx::query_as(&sql)
        .bind(record_id)
        .fetch_one(pool)
        .await?;
    Ok(row.vers_response())
}

/// Route handler : branche entre multipart (document) et JSON (autres variants).
pub async fn ajouter_ressource_contribuee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    upload_dir: web::Data<String>,
    payload: Option<Multipart>,
    body: Option<web::Bytes>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let salle_id = chemin.into_inner();

    let content_type = req
        .headers()
        .get(actix_web::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if content_type.starts_with("multipart/") {
        let multipart = payload
            .ok_or_else(|| ApiErreur::Upload("Multipart payload manquant".into()))?;
        ajouter_document(
            pool.get_ref(),
            &req,
            upload_dir.get_ref(),
            multipart,
            salle_id,
            utilisateur_id,
        )
        .await
    } else {
        let bytes = body.ok_or_else(|| ApiErreur::Validation("Corps JSON manquant".into()))?;
        let parsed: AjouterRessourceJsonBody = serde_json::from_slice(&bytes)
            .map_err(|e| ApiErreur::Validation(format!("JSON invalide: {}", e)))?;
        match parsed {
            AjouterRessourceJsonBody::VideoYoutube {
                titre,
                description,
                session_origine_id,
                video_url,
            } => {
                ajouter_video_youtube(
                    pool.get_ref(),
                    &req,
                    salle_id,
                    utilisateur_id,
                    titre,
                    description,
                    session_origine_id,
                    video_url,
                )
                .await
            }
            AjouterRessourceJsonBody::LienWeb {
                titre,
                description,
                session_origine_id,
                lien_url,
            } => {
                ajouter_lien_web(
                    pool.get_ref(),
                    &req,
                    salle_id,
                    utilisateur_id,
                    titre,
                    description,
                    session_origine_id,
                    lien_url,
                )
                .await
            }
            AjouterRessourceJsonBody::Accompagnateur {
                titre,
                description,
                session_origine_id,
                membre_recommande_id,
                motif_recommandation,
            } => {
                ajouter_accompagnateur(
                    pool.get_ref(),
                    &req,
                    salle_id,
                    utilisateur_id,
                    titre,
                    description,
                    session_origine_id,
                    membre_recommande_id,
                    motif_recommandation,
                )
                .await
            }
        }
    }
}

// ── Variant document (multipart) ──────────────────────────────────────────

async fn ajouter_document(
    pool: &PgPool,
    req: &HttpRequest,
    upload_dir: &str,
    mut payload: Multipart,
    salle_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<HttpResponse, ApiErreur> {
    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut session_origine_id: Option<Uuid> = None;
    let mut fichier_meta: Option<(String, String, i64, String)> = None; // (url, mime, taille, nom)

    let record_id = Uuid::new_v4();
    let sous_dossier = format!("{}/{}/{}", upload_dir, UPLOAD_SUBDIR, record_id);

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| ApiErreur::Upload(format!("Multipart: {}", e)))?;
        let name = field
            .content_disposition()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();

        match name.as_str() {
            "type" => {
                let v = lire_champ_texte(&mut field).await?;
                if v.trim() != "document" {
                    return Err(ApiErreur::Validation(
                        "Champ `type` doit valoir 'document' pour ce variant".into(),
                    ));
                }
            }
            "titre" => titre = Some(lire_champ_texte(&mut field).await?),
            "description" => description = Some(lire_champ_texte(&mut field).await?),
            "session_origine_id" => {
                let v = lire_champ_texte(&mut field).await?;
                if !v.trim().is_empty() {
                    session_origine_id = Some(Uuid::parse_str(v.trim()).map_err(|_| {
                        ApiErreur::Validation("session_origine_id : UUID invalide".into())
                    })?);
                }
            }
            "fichier" => {
                let cd = field.content_disposition().cloned();
                let nom_original = cd
                    .as_ref()
                    .and_then(|c| c.get_filename())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "fichier".to_string());
                let nom_sain = sanitize_filename::sanitize(&nom_original);
                let extension = nom_sain
                    .rsplit_once('.')
                    .map(|(_, ext)| ext.to_ascii_lowercase())
                    .unwrap_or_default();
                if !EXTENSIONS_AUTORISEES.contains(&extension.as_str()) {
                    return Err(ApiErreur::Validation(format!(
                        "Extension non autorisée: .{}",
                        extension
                    )));
                }
                let mime = field
                    .content_type()
                    .map(|m| m.essence_str().to_string())
                    .unwrap_or_default();
                if !MIMES_AUTORISES.contains(&mime.as_str()) {
                    return Err(ApiErreur::Validation(format!(
                        "Type MIME non autorisé: {}",
                        mime
                    )));
                }
                let chemin_complet = format!("{}/{}", sous_dossier, nom_sain);
                let taille = sauvegarder_fichier_borne(&mut field, &chemin_complet).await?;
                let url = format!("/uploads/{}/{}/{}", UPLOAD_SUBDIR, record_id, nom_sain);
                fichier_meta = Some((url, mime, taille, nom_sain));
            }
            _ => { /* ignorer */ }
        }
    }

    let titre = valider_titre(&titre.ok_or_else(|| ApiErreur::Validation("titre requis".into()))?)?;
    let description = valider_description(description)?;
    let (fichier_url, fichier_mime, fichier_taille, _) = fichier_meta
        .ok_or_else(|| ApiErreur::Validation("Fichier requis pour le variant document".into()))?;

    // Pré-checks après lecture du multipart (pour pouvoir retourner 409 / 429 avant insert)
    pre_check_ecriture(pool, salle_id, utilisateur_id, session_origine_id).await?;

    sqlx::query(
        "INSERT INTO afrolang.ressource_contribuee
         (id, salle_id, session_origine_id, auteur_id, type, titre, description,
          fichier_url, fichier_taille_octets, fichier_mime)
         VALUES ($1, $2, $3, $4, 'document', $5, $6, $7, $8, $9)",
    )
    .bind(record_id)
    .bind(salle_id)
    .bind(session_origine_id)
    .bind(utilisateur_id)
    .bind(&titre)
    .bind(&description)
    .bind(&fichier_url)
    .bind(fichier_taille)
    .bind(&fichier_mime)
    .execute(pool)
    .await?;

    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(utilisateur_id),
        "CREATE",
        "afrolang",
        "ressource_contribuee",
        Some(record_id),
        None,
        Some(audit_create_ressource(
            record_id,
            serde_json::json!({
                "type": "document",
                "titre": titre,
                "salle_id": salle_id,
                "fichier_url": fichier_url,
            }),
        )),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let response = finaliser_lecture_unique(pool, record_id).await?;
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

async fn lire_champ_texte(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut contenu = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture champ: {}", e)))?;
        contenu.extend_from_slice(&data);
        if contenu.len() > 1_048_576 {
            return Err(ApiErreur::Upload("Champ texte trop volumineux".into()));
        }
    }
    String::from_utf8(contenu)
        .map_err(|e| ApiErreur::Upload(format!("Encodage UTF-8 invalide: {}", e)))
}

/// Sauvegarde un champ fichier en bornant la taille à `TAILLE_MAX_FICHIER`.
async fn sauvegarder_fichier_borne(
    field: &mut actix_multipart::Field,
    chemin: &str,
) -> Result<i64, ApiErreur> {
    if let Some(parent) = std::path::Path::new(chemin).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le répertoire: {}", e)))?;
    }
    let mut fichier = std::fs::File::create(chemin)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le fichier: {}", e)))?;
    let mut total: i64 = 0;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
        total += data.len() as i64;
        if total as usize > TAILLE_MAX_FICHIER {
            // Nettoyage du fichier partiel
            let _ = std::fs::remove_file(chemin);
            return Err(ApiErreur::Validation("Fichier > 20 Mo".into()));
        }
        fichier
            .write_all(&data)
            .map_err(|e| ApiErreur::Upload(format!("Erreur écriture fichier: {}", e)))?;
    }
    Ok(total)
}

// ── Variant video_youtube ─────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
async fn ajouter_video_youtube(
    pool: &PgPool,
    req: &HttpRequest,
    salle_id: Uuid,
    utilisateur_id: Uuid,
    titre: String,
    description: Option<String>,
    session_origine_id: Option<Uuid>,
    video_url: String,
) -> Result<HttpResponse, ApiErreur> {
    let id_youtube = youtube_url::extraire_id_youtube(&video_url)
        .ok_or_else(|| ApiErreur::Validation("URL YouTube invalide".into()))?;

    pre_check_ecriture(pool, salle_id, utilisateur_id, session_origine_id).await?;

    let titre = valider_titre(&titre)?;
    let description = valider_description(description)?;
    let record_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO afrolang.ressource_contribuee
         (id, salle_id, session_origine_id, auteur_id, type, titre, description, video_url, video_id_youtube)
         VALUES ($1, $2, $3, $4, 'video_youtube', $5, $6, $7, $8)",
    )
    .bind(record_id)
    .bind(salle_id)
    .bind(session_origine_id)
    .bind(utilisateur_id)
    .bind(&titre)
    .bind(&description)
    .bind(&video_url)
    .bind(&id_youtube)
    .execute(pool)
    .await?;

    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(utilisateur_id),
        "CREATE",
        "afrolang",
        "ressource_contribuee",
        Some(record_id),
        None,
        Some(serde_json::json!({
            "type": "video_youtube",
            "titre": titre,
            "salle_id": salle_id,
            "video_id_youtube": id_youtube,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let response = finaliser_lecture_unique(pool, record_id).await?;
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

// ── Variant lien_web ──────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
async fn ajouter_lien_web(
    pool: &PgPool,
    req: &HttpRequest,
    salle_id: Uuid,
    utilisateur_id: Uuid,
    titre: String,
    description: Option<String>,
    session_origine_id: Option<Uuid>,
    lien_url: String,
) -> Result<HttpResponse, ApiErreur> {
    let url = lien_url.trim();
    if !url.starts_with("https://") || url.len() > 1000 {
        return Err(ApiErreur::Validation(
            "Lien : URL https requise (≤ 1000 chars)".into(),
        ));
    }

    pre_check_ecriture(pool, salle_id, utilisateur_id, session_origine_id).await?;

    let titre = valider_titre(&titre)?;
    let description = valider_description(description)?;
    let record_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO afrolang.ressource_contribuee
         (id, salle_id, session_origine_id, auteur_id, type, titre, description, lien_url)
         VALUES ($1, $2, $3, $4, 'lien_web', $5, $6, $7)",
    )
    .bind(record_id)
    .bind(salle_id)
    .bind(session_origine_id)
    .bind(utilisateur_id)
    .bind(&titre)
    .bind(&description)
    .bind(url)
    .execute(pool)
    .await?;

    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(utilisateur_id),
        "CREATE",
        "afrolang",
        "ressource_contribuee",
        Some(record_id),
        None,
        Some(serde_json::json!({
            "type": "lien_web",
            "titre": titre,
            "salle_id": salle_id,
            "lien_url": url,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let response = finaliser_lecture_unique(pool, record_id).await?;
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

// ── Variant accompagnateur ────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
async fn ajouter_accompagnateur(
    pool: &PgPool,
    req: &HttpRequest,
    salle_id: Uuid,
    utilisateur_id: Uuid,
    titre: String,
    description: Option<String>,
    session_origine_id: Option<Uuid>,
    membre_recommande_id: Uuid,
    motif_recommandation: String,
) -> Result<HttpResponse, ApiErreur> {
    if membre_recommande_id == utilisateur_id {
        return Err(ApiErreur::Validation(
            "Impossible de se recommander soi-même".into(),
        ));
    }
    let motif = motif_recommandation.trim();
    if motif.chars().count() < 20 || motif.chars().count() > 2000 {
        return Err(ApiErreur::Validation(
            "Motif de recommandation : 20..2000 caractères".into(),
        ));
    }
    let etat: Option<String> = sqlx::query_scalar(
        "SELECT etat::TEXT FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(membre_recommande_id)
    .fetch_optional(pool)
    .await?;
    if etat.as_deref() != Some("actif") {
        return Err(ApiErreur::Validation(
            "Membre recommandé introuvable ou non actif".into(),
        ));
    }

    pre_check_ecriture(pool, salle_id, utilisateur_id, session_origine_id).await?;

    let titre = valider_titre(&titre)?;
    let description = valider_description(description)?;
    let record_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO afrolang.ressource_contribuee
         (id, salle_id, session_origine_id, auteur_id, type, titre, description,
          membre_recommande_id, motif_recommandation, statut_accompagnateur)
         VALUES ($1, $2, $3, $4, 'accompagnateur', $5, $6, $7, $8, 'en_attente')",
    )
    .bind(record_id)
    .bind(salle_id)
    .bind(session_origine_id)
    .bind(utilisateur_id)
    .bind(&titre)
    .bind(&description)
    .bind(membre_recommande_id)
    .bind(motif)
    .execute(pool)
    .await?;

    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(utilisateur_id),
        "CREATE",
        "afrolang",
        "ressource_contribuee",
        Some(record_id),
        None,
        Some(serde_json::json!({
            "type": "accompagnateur",
            "titre": titre,
            "salle_id": salle_id,
            "membre_recommande_id": membre_recommande_id,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    notification::creer_notification(
        pool,
        membre_recommande_id,
        notification::afrolang::ACCOMPAGNATEUR_RECOMMANDATION_RECUE,
        &format!("Vous avez été recommandé(e) comme accompagnateur : {}", titre),
        Some("/mon-compte/recommandations-accompagnateur"),
    )
    .await;

    let response = finaliser_lecture_unique(pool, record_id).await?;
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────────────────
// DELETE /api/afrolang/ressources-contribuees/{id}
// ──────────────────────────────────────────────────────────────────────────

pub async fn supprimer_ressource_contribuee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let id = chemin.into_inner();

    let row: Option<(Uuid, Option<DateTime<Utc>>)> = sqlx::query_as(
        "SELECT auteur_id, deleted_at FROM afrolang.ressource_contribuee WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (auteur_id, deleted_at) = row
        .ok_or_else(|| ApiErreur::NonTrouve("Ressource introuvable".into()))?;
    if deleted_at.is_some() {
        return Err(ApiErreur::NonTrouve("Ressource déjà supprimée".into()));
    }

    let est_admin = verifier_admin_plateforme(pool.get_ref(), utilisateur_id).await?;
    if auteur_id != utilisateur_id && !est_admin {
        return Err(ApiErreur::AccesInterdit(
            "Suppression réservée à l'auteur ou à un admin".into(),
        ));
    }

    sqlx::query(
        "UPDATE afrolang.ressource_contribuee
            SET deleted_at = NOW(), supprime_par = $2, updated_at = NOW()
          WHERE id = $1",
    )
    .bind(id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "DELETE",
        "afrolang",
        "ressource_contribuee",
        Some(id),
        None,
        Some(serde_json::json!({ "acteur_admin": est_admin })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::NoContent().finish())
}

// ══════════════════════════════════════════════════════════════════════════
// Workflow accompagnateur (T028)
// ══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct SalleLight {
    pub id: Uuid,
    pub titre: String,
    pub groupe_ethnique_nom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RecommandationRecueResponse {
    pub id: Uuid,
    pub salle: SalleLight,
    pub auteur: AuteurLight,
    pub motif_recommandation: String,
    pub statut_accompagnateur: StatutAccompagnateur,
    pub created_at: DateTime<Utc>,
    pub reponse_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct RecommandationRecueRow {
    id: Uuid,
    salle_id: Uuid,
    salle_titre: String,
    groupe_ethnique_nom: Option<String>,
    auteur_id: Uuid,
    auteur_nom: String,
    auteur_prenom: String,
    auteur_avatar_url: Option<String>,
    motif_recommandation: String,
    statut_accompagnateur: StatutAccompagnateur,
    created_at: DateTime<Utc>,
    reponse_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ListerRecommandationsQuery {
    pub statut: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

pub async fn lister_recommandations_recues(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    query: web::Query<ListerRecommandationsQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let pagination = PaginationParams {
        page: query.page,
        par_page: query.par_page,
        tri_par: None,
        tri_dir: None,
    };

    let statut_filtre: Option<StatutAccompagnateur> = match query.statut.as_deref() {
        Some("en_attente") => Some(StatutAccompagnateur::EnAttente),
        Some("acceptee") => Some(StatutAccompagnateur::Acceptee),
        Some("refusee") => Some(StatutAccompagnateur::Refusee),
        Some("retiree") => Some(StatutAccompagnateur::Retiree),
        Some(other) => {
            return Err(ApiErreur::Validation(format!("Statut inconnu: {}", other)));
        }
        None => None,
    };

    let mut where_clauses: Vec<&str> = vec![
        "rc.membre_recommande_id = $1",
        "rc.type = 'accompagnateur'",
        "rc.deleted_at IS NULL",
    ];
    if statut_filtre.is_some() {
        where_clauses.push("rc.statut_accompagnateur = $2");
    }
    let where_sql = where_clauses.join(" AND ");

    let count_sql = format!(
        "SELECT COUNT(*)::BIGINT FROM afrolang.ressource_contribuee rc WHERE {}",
        where_sql
    );
    let mut count_q = sqlx::query_as::<_, (i64,)>(&count_sql).bind(utilisateur_id);
    if let Some(s) = statut_filtre {
        count_q = count_q.bind(s);
    }
    let (total,) = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT rc.id, rc.salle_id,
                s.titre AS salle_titre,
                ge.nom AS groupe_ethnique_nom,
                rc.auteur_id, ua.nom AS auteur_nom, ua.prenom AS auteur_prenom,
                ua.photo_url AS auteur_avatar_url,
                rc.motif_recommandation,
                rc.statut_accompagnateur AS \"statut_accompagnateur!: StatutAccompagnateur\",
                rc.created_at, rc.reponse_at
         FROM afrolang.ressource_contribuee rc
         JOIN iam.utilisateur ua ON ua.id = rc.auteur_id
         JOIN afrolang.salle s ON s.id = rc.salle_id
         LEFT JOIN country_profile.groupe_ethnique ge ON ge.id = s.groupe_ethnique_id
         WHERE {}
         ORDER BY rc.created_at DESC
         LIMIT ${lim} OFFSET ${off}",
        where_sql,
        lim = if statut_filtre.is_some() { 3 } else { 2 },
        off = if statut_filtre.is_some() { 4 } else { 3 },
    );
    let mut q = sqlx::query_as::<_, RecommandationRecueRow>(&select_sql).bind(utilisateur_id);
    if let Some(s) = statut_filtre {
        q = q.bind(s);
    }
    q = q.bind(pagination.par_page()).bind(pagination.offset());
    let rows = q.fetch_all(pool.get_ref()).await?;

    let unwrap_motif = |s: String| -> String {
        if s.is_empty() {
            String::new()
        } else {
            s
        }
    };

    let data: Vec<RecommandationRecueResponse> = rows
        .into_iter()
        .map(|r| RecommandationRecueResponse {
            id: r.id,
            salle: SalleLight {
                id: r.salle_id,
                titre: r.salle_titre,
                groupe_ethnique_nom: r.groupe_ethnique_nom,
            },
            auteur: AuteurLight {
                id: r.auteur_id,
                nom: r.auteur_nom,
                prenom: r.auteur_prenom,
                avatar_url: r.auteur_avatar_url,
            },
            motif_recommandation: unwrap_motif(r.motif_recommandation),
            statut_accompagnateur: r.statut_accompagnateur,
            created_at: r.created_at,
            reponse_at: r.reponse_at,
        })
        .collect();
    let paginated = PaginatedResponse::from_params(data, total, &pagination);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(paginated),
        error: None,
    }))
}

async fn charger_recommandation_pour_action(
    pool: &PgPool,
    id: Uuid,
    utilisateur_id: Uuid,
    statuts_acceptes: &[StatutAccompagnateur],
) -> Result<(Uuid, StatutAccompagnateur, String), ApiErreur> {
    let row: Option<(
        Option<Uuid>,
        Option<StatutAccompagnateur>,
        Uuid,
        String,
        TypeRessourceContribuee,
        Option<DateTime<Utc>>,
    )> = sqlx::query_as(
        "SELECT membre_recommande_id, statut_accompagnateur, auteur_id, titre, type, deleted_at
         FROM afrolang.ressource_contribuee
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    let (membre, statut, auteur_id, titre, type_, deleted_at) =
        row.ok_or_else(|| ApiErreur::NonTrouve("Recommandation introuvable".into()))?;
    if deleted_at.is_some() {
        return Err(ApiErreur::NonTrouve("Recommandation supprimée".into()));
    }
    if !matches!(type_, TypeRessourceContribuee::Accompagnateur) {
        return Err(ApiErreur::Validation(
            "Cette ressource n'est pas une recommandation accompagnateur".into(),
        ));
    }
    if membre != Some(utilisateur_id) {
        return Err(ApiErreur::AccesInterdit(
            "Action réservée à la personne recommandée".into(),
        ));
    }
    let statut = statut.ok_or_else(|| ApiErreur::Validation("Statut absent".into()))?;
    if !statuts_acceptes.contains(&statut) {
        return Err(ApiErreur::Conflit("Statut incompatible".into()));
    }
    Ok((auteur_id, statut, titre))
}

pub async fn accepter_recommandation(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let id = chemin.into_inner();

    let (auteur_id, _statut, titre) = charger_recommandation_pour_action(
        pool.get_ref(),
        id,
        utilisateur_id,
        &[StatutAccompagnateur::EnAttente],
    )
    .await?;

    sqlx::query(
        "UPDATE afrolang.ressource_contribuee
            SET statut_accompagnateur = 'acceptee',
                reponse_at = NOW(), updated_at = NOW()
          WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "afrolang",
        "ressource_contribuee",
        Some(id),
        None,
        Some(serde_json::json!({ "statut_accompagnateur": "acceptee" })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    notification::creer_notification(
        pool.get_ref(),
        auteur_id,
        notification::afrolang::ACCOMPAGNATEUR_ACCEPTEE,
        &format!("Votre recommandation « {} » a été acceptée", titre),
        None,
    )
    .await;

    let response = finaliser_lecture_unique(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

#[derive(Debug, Deserialize)]
pub struct RefuserBody {
    pub motif_refus: Option<String>,
}

pub async fn refuser_recommandation(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: Option<web::Json<RefuserBody>>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let id = chemin.into_inner();

    let (auteur_id, _statut, titre) = charger_recommandation_pour_action(
        pool.get_ref(),
        id,
        utilisateur_id,
        &[StatutAccompagnateur::EnAttente],
    )
    .await?;

    let motif_refus = body
        .as_ref()
        .and_then(|b| b.motif_refus.as_ref())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    if let Some(m) = motif_refus.as_deref() {
        if m.chars().count() > 500 {
            return Err(ApiErreur::Validation(
                "Motif de refus ≤ 500 caractères".into(),
            ));
        }
    }

    sqlx::query(
        "UPDATE afrolang.ressource_contribuee
            SET statut_accompagnateur = 'refusee',
                motif_refus = $2,
                reponse_at = NOW(), updated_at = NOW()
          WHERE id = $1",
    )
    .bind(id)
    .bind(motif_refus.as_deref())
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "afrolang",
        "ressource_contribuee",
        Some(id),
        None,
        Some(serde_json::json!({ "statut_accompagnateur": "refusee" })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    // Motif non transmis au destinataire (consigne contrat).
    notification::creer_notification(
        pool.get_ref(),
        auteur_id,
        notification::afrolang::ACCOMPAGNATEUR_REFUSEE,
        &format!("Votre recommandation « {} » a été refusée", titre),
        None,
    )
    .await;

    let response = finaliser_lecture_unique(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

pub async fn retirer_consentement(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let id = chemin.into_inner();

    let (auteur_id, _statut, titre) = charger_recommandation_pour_action(
        pool.get_ref(),
        id,
        utilisateur_id,
        &[StatutAccompagnateur::Acceptee],
    )
    .await?;

    sqlx::query(
        "UPDATE afrolang.ressource_contribuee
            SET statut_accompagnateur = 'retiree',
                reponse_at = NOW(), updated_at = NOW()
          WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "afrolang",
        "ressource_contribuee",
        Some(id),
        None,
        Some(serde_json::json!({ "statut_accompagnateur": "retiree" })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    notification::creer_notification(
        pool.get_ref(),
        auteur_id,
        notification::afrolang::ACCOMPAGNATEUR_RETIREE,
        &format!(
            "L'accompagnateur de votre recommandation « {} » a retiré son consentement",
            titre
        ),
        None,
    )
    .await;

    let response = finaliser_lecture_unique(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

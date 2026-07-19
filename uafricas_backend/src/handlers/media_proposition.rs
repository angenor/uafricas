//! Soumission de médias par les parties prenantes, suivi par leur auteur, et
//! édition par les détenteurs (feature 001-refonte-tele-radio, US4).
//!
//! Endpoints membre :
//!   POST   /api/medias/propositions                              (multipart)
//!   GET    /api/medias/propositions/moi
//!   PATCH  /api/medias/propositions/{id}/retirer
//!   PATCH  /api/medias/contenus/{type_media}/{id}/metadonnees
//!   PUT    /api/medias/contenus/{type_media}/{id}/media
//!
//! Ces routes s'adressent à des MEMBRES : elles n'emploient jamais l'extracteur
//! `AdminUtilisateur`, qui rejette tout non-admin.

use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_detention::garde_detenteur;
use crate::handlers::media_social::extraire_utilisateur_id;
use crate::models::media_proposition::{
    type_objet_valide, DecisionMediaRequest, DonneesProposition, MesPropositionsFiltres,
    MetadonneesRequest, PropositionMediaListeResponse, PropositionMediaRow, RemplacerMediaRequest,
    SoumissionBrute, LONGUEUR_MIN_MOTIF_REJET, PROPOSITION_MEDIA_COLONNES,
};
use crate::models::media_social::{table_pour_type, TYPES_MEDIA_AUTORISES};
use crate::models::notification;
use crate::services::audit;

#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// Mêmes formats et plafonds que le téléversement back-office
// (`admin/radio_tele::uploader_media`) : un contributeur n'a pas de raison
// d'être plus contraint qu'un administrateur.
const EXTENSIONS_VIDEO: &[&str] = &["mp4", "webm", "mov", "m4v", "ogv"];
const EXTENSIONS_AUDIO: &[&str] = &["mp3", "ogg", "oga", "wav", "m4a", "aac"];
const EXTENSIONS_IMAGE: &[&str] = &["jpg", "jpeg", "png", "webp", "avif"];
const TAILLE_MAX_VIDEO: usize = 300 * 1024 * 1024; // 300 Mo
const TAILLE_MAX_AUDIO: usize = 80 * 1024 * 1024; //   80 Mo
const TAILLE_MAX_IMAGE: usize = 8 * 1024 * 1024; //     8 Mo

fn exiger_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

// ═══════════════════════════════════════════════════════════════════════════
// POST /api/medias/propositions — soumission (multipart)
// ═══════════════════════════════════════════════════════════════════════════

/// Enregistre une proposition en `'en_attente'`, quelle que soit sa forme.
///
/// **Trois garde-fous non négociables**, appliqués ici et non côté client :
///
/// 1. `statut` naît toujours `'en_attente'` — il n'est pas lu du corps de la
///    requête (FR-031) ;
/// 2. `origine_publication` sera forcée à `'territoire'` à la validation : la
///    bannière Radio Africans est une décision éditoriale de la plateforme
///    (FR-036). `DonneesProposition` ne porte tout simplement pas ce champ, de
///    sorte qu'un client ne peut pas même l'exprimer ;
/// 3. « Autre » sans précision → 400, doublé d'un CHECK SQL (FR-029, FR-030).
pub async fn soumettre_proposition(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;
    let soumission = lire_multipart(payload, upload_dir.get_ref()).await?;

    if !type_objet_valide(&soumission.type_objet) {
        return Err(ApiErreur::Validation(format!(
            "Type de proposition « {} » inconnu",
            soumission.type_objet
        )));
    }
    if soumission.justification.trim().is_empty() {
        return Err(ApiErreur::Validation(
            "La justification de votre proposition est requise".into(),
        ));
    }
    // Le CHECK ck_prop_media_cible_requise dit la même chose ; le message en
    // français vient d'ici.
    if matches!(
        soumission.type_objet.as_str(),
        "animation_programme" | "idee_contenu"
    ) && soumission.target_id.is_none()
    {
        return Err(ApiErreur::Validation(
            "Cette proposition doit désigner la chaîne ou la station visée".into(),
        ));
    }
    soumission.donnees.valider(&soumission.type_objet)?;

    let donnees = serde_json::to_value(&soumission.donnees)
        .map_err(|e| ApiErreur::Validation(format!("Données de proposition invalides: {}", e)))?;
    let pieces = serde_json::to_value(&soumission.pieces_jointes).unwrap_or_else(|_| serde_json::json!([]));

    let proposition_id: Uuid = sqlx::query_scalar(
        "INSERT INTO media_content.proposition_media
            (auteur_id, type_objet, target_id, donnees, pieces_jointes, justification)
         VALUES ($1, $2::media_content.type_objet_propose, $3, $4, $5, $6)
         RETURNING id",
    )
    .bind(auteur_id)
    .bind(&soumission.type_objet)
    .bind(soumission.target_id)
    .bind(&donnees)
    .bind(&pieces)
    .bind(soumission.justification.trim())
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "CREATE",
        "media_content",
        "proposition_media",
        Some(proposition_id),
        None,
        Some(serde_json::json!({
            "type_objet": soumission.type_objet,
            "target_id": soumission.target_id,
            "statut": "en_attente",
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let proposition = charger_proposition(pool.get_ref(), proposition_id).await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(proposition),
        error: None,
    }))
}

/// Lit le multipart : champs texte d'un côté, fichiers téléversés de l'autre.
///
/// Les fichiers sont rangés selon leur extension et leur chemin est reporté
/// dans `donnees` — un contributeur choisit de téléverser OU de fournir un lien
/// externe, jamais les deux pour le même média (FR-056).
async fn lire_multipart(
    mut payload: actix_multipart::Multipart,
    upload_dir: &str,
) -> Result<SoumissionBrute, ApiErreur> {
    let mut soumission = SoumissionBrute::default();

    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|e| ApiErreur::Upload(format!("Champ multipart invalide: {}", e)))?;
        let disposition = field.content_disposition().cloned();
        let nom_champ = disposition
            .as_ref()
            .and_then(|d| d.get_name())
            .unwrap_or("")
            .to_string();
        let nom_fichier = disposition
            .as_ref()
            .and_then(|d| d.get_filename())
            .map(str::to_string);

        match nom_fichier {
            // ── Fichier téléversé ──
            Some(nom) if !nom.is_empty() => {
                let url = ecrire_fichier(&mut field, &nom, upload_dir).await?;
                match nom_champ.as_str() {
                    "image" => soumission.donnees.image_couverture_url = Some(url.clone()),
                    "media" if url.contains("/audios/") => {
                        soumission.donnees.audio_url = Some(url.clone())
                    }
                    "media" => soumission.donnees.video_url = Some(url.clone()),
                    _ => {}
                }
                soumission.pieces_jointes.push(url);
            }
            // ── Champ texte ──
            _ => {
                let valeur = lire_texte(&mut field).await?;
                match nom_champ.as_str() {
                    "type_objet" => soumission.type_objet = valeur.trim().to_string(),
                    "justification" => soumission.justification = valeur,
                    "target_id" => {
                        let brut = valeur.trim();
                        if !brut.is_empty() && brut != "null" {
                            soumission.target_id = Some(Uuid::parse_str(brut).map_err(|_| {
                                ApiErreur::Validation("Identifiant de cible invalide".into())
                            })?);
                        }
                    }
                    "donnees" => {
                        soumission.donnees =
                            serde_json::from_str::<DonneesProposition>(&valeur).map_err(|e| {
                                ApiErreur::Validation(format!("Champ « donnees » illisible: {}", e))
                            })?;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(soumission)
}

async fn lire_texte(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut octets = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture champ: {}", e)))?;
        octets.extend_from_slice(&data);
    }
    String::from_utf8(octets)
        .map_err(|_| ApiErreur::Validation("Champ texte non encodé en UTF-8".into()))
}

/// Écrit un fichier téléversé et renvoie son URL publique.
async fn ecrire_fichier(
    field: &mut actix_multipart::Field,
    nom_fichier: &str,
    upload_dir: &str,
) -> Result<String, ApiErreur> {
    let ext = std::path::Path::new(nom_fichier)
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default();

    let (sous_dossier, taille_max) = if EXTENSIONS_VIDEO.contains(&ext.as_str()) {
        ("videos", TAILLE_MAX_VIDEO)
    } else if EXTENSIONS_AUDIO.contains(&ext.as_str()) {
        ("audios", TAILLE_MAX_AUDIO)
    } else if EXTENSIONS_IMAGE.contains(&ext.as_str()) {
        ("vignettes", TAILLE_MAX_IMAGE)
    } else {
        return Err(ApiErreur::Validation(format!(
            "Format de fichier non supporté : « {} ». Vidéos : {} ; audios : {} ; images : {}.",
            if ext.is_empty() { "inconnu" } else { &ext },
            EXTENSIONS_VIDEO.join(", "),
            EXTENSIONS_AUDIO.join(", "),
            EXTENSIONS_IMAGE.join(", "),
        )));
    };

    let nom_stocke = format!("{}.{}", Uuid::new_v4(), ext);
    let chemin_relatif = format!("/uploads/medias/{}/{}", sous_dossier, nom_stocke);
    let chemin_complet = format!("{}/medias/{}/{}", upload_dir, sous_dossier, nom_stocke);

    if let Some(parent) = std::path::Path::new(&chemin_complet).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le répertoire: {}", e)))?;
    }
    let mut fichier = std::fs::File::create(&chemin_complet)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le fichier: {}", e)))?;

    let mut taille: usize = 0;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
        taille += data.len();
        if taille > taille_max {
            drop(fichier);
            let _ = std::fs::remove_file(&chemin_complet);
            return Err(ApiErreur::Validation(format!(
                "Fichier trop volumineux (maximum {} Mo)",
                taille_max / (1024 * 1024)
            )));
        }
        fichier
            .write_all(&data)
            .map_err(|e| ApiErreur::Upload(format!("Erreur écriture fichier: {}", e)))?;
    }

    if taille == 0 {
        let _ = std::fs::remove_file(&chemin_complet);
        return Err(ApiErreur::Validation("Fichier vide".into()));
    }

    Ok(chemin_relatif)
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/medias/propositions/moi — suivi de ses soumissions (FR-034)
// ═══════════════════════════════════════════════════════════════════════════

pub async fn lister_mes_propositions(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<MesPropositionsFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions = vec!["pm.auteur_id = $1".to_string()];
    let mut index = 2u32;
    if params.statut.is_some() {
        conditions.push(format!(
            "pm.statut = ${}::media_content.statut_proposition_media",
            index
        ));
        index += 1;
    }
    if params.type_objet.is_some() {
        conditions.push(format!(
            "pm.type_objet = ${}::media_content.type_objet_propose",
            index
        ));
        index += 1;
    }
    let where_clause = conditions.join(" AND ");

    let requete_total =
        format!("SELECT COUNT(*) FROM media_content.proposition_media pm WHERE {where_clause}");
    let mut count_q = sqlx::query_scalar::<_, i64>(&requete_total).bind(auteur_id);
    if let Some(ref s) = params.statut {
        count_q = count_q.bind(s);
    }
    if let Some(ref t) = params.type_objet {
        count_q = count_q.bind(t);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let requete_liste = format!(
        "SELECT {PROPOSITION_MEDIA_COLONNES},
                ud.nom AS decideur_nom, ud.prenom AS decideur_prenom
           FROM media_content.proposition_media pm
           LEFT JOIN iam.utilisateur ud ON ud.id = pm.decideur
          WHERE {where_clause}
          ORDER BY pm.created_at DESC
          LIMIT ${index} OFFSET ${}",
        index + 1
    );
    let mut q = sqlx::query_as::<_, PropositionMediaRow>(&requete_liste).bind(auteur_id);
    if let Some(ref s) = params.statut {
        q = q.bind(s);
    }
    if let Some(ref t) = params.type_objet {
        q = q.bind(t);
    }
    let rows = q
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
        data: Some(PropositionMediaListeResponse {
            propositions: rows.into_iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PATCH /api/medias/propositions/{id}/retirer
// ═══════════════════════════════════════════════════════════════════════════

/// Retrait par l'auteur, et seulement tant que la proposition est en attente :
/// les autres états sont terminaux.
pub async fn retirer_proposition(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;
    let proposition_id = chemin.into_inner();

    let etat: Option<(Uuid, String)> = sqlx::query_as(
        "SELECT auteur_id, statut::text FROM media_content.proposition_media WHERE id = $1",
    )
    .bind(proposition_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (proprietaire, statut) =
        etat.ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;

    if proprietaire != auteur_id {
        return Err(ApiErreur::AccesInterdit(
            "Seul l'auteur peut retirer sa proposition".into(),
        ));
    }
    if statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette proposition a déjà été traitée : elle ne peut plus être retirée".into(),
        ));
    }

    sqlx::query(
        "UPDATE media_content.proposition_media
            SET statut = 'retiree', updated_at = NOW()
          WHERE id = $1",
    )
    .bind(proposition_id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "UPDATE",
        "media_content",
        "proposition_media",
        Some(proposition_id),
        Some(serde_json::json!({ "statut": "en_attente" })),
        Some(serde_json::json!({ "statut": "retiree" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let proposition = charger_proposition(pool.get_ref(), proposition_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(proposition),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// ÉDITION D'UN CONTENU PAR SON DÉTENTEUR (FR-032)
// ═══════════════════════════════════════════════════════════════════════════
//
// Deux routes, aux effets délibérément distincts :
//   • PATCH …/metadonnees — titre, description, image, thème : PUBLIÉ TOUT DE
//     SUITE, `etat` inchangé. Corriger une faute de frappe n'a pas à passer par
//     la modération.
//   • PUT   …/media       — remplace le fichier ou le lien : bascule le contenu
//     en `'en_attente'` et ouvre une proposition de modification. Le contenu
//     CESSE d'être diffusé jusqu'à revalidation — c'est le choix le plus sûr,
//     rien de non validé n'étant public (FR-031).

/// Le membre doit être l'auteur du contenu, **ou** co-détenteur du support qui
/// le porte (US5) : c'est ce qui permet à un co-détenteur de publier sur la
/// chaîne qu'il détient sans en être l'auteur d'origine.
async fn garde_auteur_contenu(
    pool: &PgPool,
    type_media: &str,
    media_id: Uuid,
    moi: Uuid,
) -> Result<(), ApiErreur> {
    if !TYPES_MEDIA_AUTORISES.contains(&type_media) {
        return Err(ApiErreur::Validation(format!(
            "Type de média « {} » non supporté",
            type_media
        )));
    }
    let table = table_pour_type(type_media).expect("type de média supporté");

    let cree_par: Option<Uuid> = sqlx::query_scalar(&format!(
        "SELECT cree_par FROM {table} WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(media_id)
    .fetch_optional(pool)
    .await?;

    match cree_par {
        None => Err(ApiErreur::NonTrouve("Contenu introuvable".into())),
        Some(auteur) if auteur == moi => Ok(()),
        // Repli sur la co-détention du support porteur.
        Some(_) => {
            let (type_support, support_id) = support_porteur(pool, type_media, media_id).await?;
            garde_detenteur(pool, &type_support, support_id, moi, "co_detenteur")
                .await
                .map(|_| ())
                .map_err(|_| {
                    ApiErreur::AccesInterdit("Vous ne détenez pas ce contenu".into())
                })
        }
    }
}

/// Support dont relève un média : lui-même s'il en est un, son parent sinon.
async fn support_porteur(
    pool: &PgPool,
    type_media: &str,
    media_id: Uuid,
) -> Result<(String, Uuid), ApiErreur> {
    let (type_support, requete) = match type_media {
        "chaine_tv" => return Ok(("chaine_tv".to_string(), media_id)),
        "station_radio" => return Ok(("station_radio".to_string(), media_id)),
        "programme_tele" => (
            "chaine_tv",
            "SELECT chaine_id FROM media_content.programme_tele WHERE id = $1",
        ),
        _ => (
            "station_radio",
            "SELECT station_id FROM media_content.programme_radio WHERE id = $1",
        ),
    };

    let support_id: Option<Uuid> = sqlx::query_scalar(requete)
        .bind(media_id)
        .fetch_optional(pool)
        .await?
        .flatten();

    // Un contenu orphelin de support n'a pas de co-détenteur à interroger : la
    // garde ne peut alors reposer que sur la qualité d'auteur.
    let support_id = support_id.ok_or_else(|| {
        ApiErreur::AccesInterdit("Vous ne détenez pas ce contenu".into())
    })?;

    Ok((type_support.to_string(), support_id))
}

/// PATCH /api/medias/contenus/{type_media}/{id}/metadonnees
pub async fn modifier_metadonnees(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<MetadonneesRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_media, media_id) = chemin.into_inner();
    garde_auteur_contenu(pool.get_ref(), &type_media, media_id, moi).await?;

    let table = table_pour_type(&type_media).expect("type de média supporté");
    let est_contenu = type_media.starts_with("programme_");

    // Les supports portent `nom`, les contenus `nom_emission`, et le thème
    // phare ne concerne que les seconds : les deux formes de requête sont
    // écrites en clair plutôt que recomposées par morceaux.
    let requete = if est_contenu {
        format!(
            "UPDATE {table}
                SET nom_emission          = COALESCE($2, nom_emission),
                    description           = COALESCE($3, description),
                    image_couverture_url  = COALESCE($4, image_couverture_url),
                    theme_phare_id        = COALESCE($5, theme_phare_id),
                    theme_phare_autre     = COALESCE($6, theme_phare_autre),
                    updated_at            = NOW()
              WHERE id = $1 AND deleted_at IS NULL"
        )
    } else {
        format!(
            "UPDATE {table}
                SET nom                   = COALESCE($2, nom),
                    description           = COALESCE($3, description),
                    image_couverture_url  = COALESCE($4, image_couverture_url),
                    updated_at            = NOW()
              WHERE id = $1 AND deleted_at IS NULL"
        )
    };

    // Un champ absent ou vide laisse la valeur en place — d'où le COALESCE.
    fn non_vide(valeur: &Option<String>) -> Option<&str> {
        valeur.as_deref().map(str::trim).filter(|s| !s.is_empty())
    }

    let mut q = sqlx::query(&requete)
        .bind(media_id)
        .bind(non_vide(&body.nom))
        .bind(non_vide(&body.description))
        .bind(non_vide(&body.image_couverture_url));
    if est_contenu {
        q = q
            .bind(body.theme_phare_id)
            .bind(non_vide(&body.theme_phare_autre));
    }

    let modifie = q.execute(pool.get_ref()).await?.rows_affected();
    if modifie == 0 {
        return Err(ApiErreur::NonTrouve("Contenu introuvable".into()));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "media_content",
        &type_media,
        Some(media_id),
        None,
        Some(serde_json::json!({
            "nom": body.nom,
            "description": body.description,
            "theme_phare_id": body.theme_phare_id,
            "image_couverture_url": body.image_couverture_url,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "modifie": true, "etat": "publie" })),
        error: None,
    }))
}

/// PUT /api/medias/contenus/{type_media}/{id}/media
pub async fn remplacer_media(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<RemplacerMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_media, media_id) = chemin.into_inner();
    garde_auteur_contenu(pool.get_ref(), &type_media, media_id, moi).await?;

    let media_url = body.media_url.trim();
    if media_url.is_empty() {
        return Err(ApiErreur::Validation(
            "L'adresse du nouveau média est requise".into(),
        ));
    }

    // Seuls les contenus portent un média remplaçable ; une chaîne ou une
    // station n'a qu'un flux, édité par ses métadonnées.
    let colonne_media = match type_media.as_str() {
        "programme_tele" => "video_url",
        "programme_radio" => "audio_url",
        _ => {
            return Err(ApiErreur::Validation(
                "Seules les émissions portent un média remplaçable".into(),
            ));
        }
    };
    let table = table_pour_type(&type_media).expect("type de média supporté");

    let mut tx = pool.begin().await?;

    // Le contenu cesse d'être diffusé jusqu'à revalidation (FR-031, FR-032).
    let ancien_media: Option<String> = sqlx::query_scalar(&format!(
        "UPDATE {table}
            SET {colonne_media} = $2, etat = 'en_attente', updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL
        RETURNING {colonne_media}"
    ))
    .bind(media_id)
    .bind(media_url)
    .fetch_optional(&mut *tx)
    .await?;

    if ancien_media.is_none() {
        return Err(ApiErreur::NonTrouve("Contenu introuvable".into()));
    }

    let justification = body
        .justification
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("Remplacement du média par son détenteur");

    let proposition_id: Uuid = sqlx::query_scalar(
        "INSERT INTO media_content.proposition_media
            (auteur_id, type_objet, target_id, donnees, justification)
         VALUES ($1, $2::media_content.type_objet_propose, $3, $4, $5)
         RETURNING id",
    )
    .bind(moi)
    .bind(&type_media)
    .bind(media_id)
    .bind(serde_json::json!({ colonne_media: media_url }))
    .bind(justification)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "media_content",
        &type_media,
        Some(media_id),
        Some(serde_json::json!({ "etat": "publie", colonne_media: null_ou(&ancien_media) })),
        Some(serde_json::json!({
            "etat": "en_attente",
            colonne_media: media_url,
            "proposition_id": proposition_id,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "etat": "en_attente",
            "proposition_id": proposition_id,
        })),
        error: None,
    }))
}

fn null_ou(valeur: &Option<String>) -> serde_json::Value {
    match valeur {
        Some(v) => serde_json::Value::String(v.clone()),
        None => serde_json::Value::Null,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ENGAGEMENT : IDÉES ET DEMANDES D'ANIMATION (US6, FR-044, FR-045, FR-047)
// ═══════════════════════════════════════════════════════════════════════════
//
// Ces deux types de proposition ne créent pas de contenu :
//   • `idee_contenu`        — un visiteur suggère un sujet. Retenue, l'idée
//     n'engendre AUCUN objet : elle est simplement marquée comme telle. C'est
//     le seul type que `ck_prop_media_validation_a_objet` exempte.
//   • `animation_programme` — une partie prenante demande à animer un programme.
//     Acceptée, elle ajoute son auteur aux co-détenteurs du support visé — c'est
//     l'objet créé, et `objet_id_cree` porte l'identifiant de cette ligne.
//
// Elles s'adressent d'abord aux **co-détenteurs du support visé**, et non aux
// seuls administrateurs (FR-047) : c'est la chaîne qui sait si elle veut de
// cette idée ou de cet animateur.

/// Détermine si `target_id` désigne une chaîne ou une station.
///
/// Une demande d'animation ne dit pas de quel type de support elle relève : les
/// deux tables sont sondées, dans un ordre arbitraire mais stable — les
/// identifiants étant des UUID v4, une collision entre les deux est hors de
/// portée.
pub async fn type_support_de(
    executeur: &PgPool,
    target_id: Uuid,
) -> Result<Option<String>, ApiErreur> {
    let est_chaine: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.chaine_tv
            WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(target_id)
    .fetch_one(executeur)
    .await?;
    if est_chaine {
        return Ok(Some("chaine_tv".to_string()));
    }

    let est_station: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.station_radio
            WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(target_id)
    .fetch_one(executeur)
    .await?;
    Ok(est_station.then(|| "station_radio".to_string()))
}

/// Effet d'une acceptation, appliqué dans la transaction de décision.
///
/// Partagée par la file de modération administrative et par la décision des
/// co-détenteurs : les deux chemins doivent produire exactement le même effet,
/// sans quoi une demande acceptée en back-office et une demande acceptée par la
/// chaîne ne vaudraient pas la même chose.
pub async fn appliquer_acceptation_engagement(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    type_objet: &str,
    type_support: &str,
    target_id: Uuid,
    auteur_id: Uuid,
    decideur: Uuid,
) -> Result<Option<Uuid>, ApiErreur> {
    match type_objet {
        // Une idée retenue ne crée rien (FR-044).
        "idee_contenu" => Ok(None),
        "animation_programme" => {
            // Upsert-réactivation : le retrait d'un co-détenteur étant un soft
            // delete, une ligne inactive peut préexister.
            let id: Uuid = sqlx::query_scalar(
                "INSERT INTO media_content.support_detenteur
                    (type_support, support_id, utilisateur_id, role, designe_par)
                 VALUES ($1::media_content.type_support_media, $2, $3, 'co_detenteur', $4)
                 ON CONFLICT (type_support, support_id, utilisateur_id)
                 DO UPDATE SET actif = TRUE,
                               retire_at = NULL,
                               designe_par = EXCLUDED.designe_par,
                               designe_at = NOW(),
                               updated_at = NOW()
                 RETURNING id",
            )
            .bind(type_support)
            .bind(target_id)
            .bind(auteur_id)
            .bind(decideur)
            .fetch_one(&mut **tx)
            .await?;
            Ok(Some(id))
        }
        autre => Err(ApiErreur::Validation(format!(
            "Le type « {} » ne relève pas des propositions d'engagement",
            autre
        ))),
    }
}

/// GET /api/medias/{type_support}/{support_id}/propositions
///
/// Les idées et demandes d'animation visant ce support, lisibles par ses
/// co-détenteurs (FR-047).
pub async fn lister_propositions_support(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    params: web::Query<MesPropositionsFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id) = chemin.into_inner();
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    let mut conditions = vec![
        "pm.target_id = $1".to_string(),
        "pm.type_objet IN ('animation_programme', 'idee_contenu')".to_string(),
    ];
    if params.statut.is_some() {
        conditions.push("pm.statut = $2::media_content.statut_proposition_media".to_string());
    }
    let where_clause = conditions.join(" AND ");

    let requete = format!(
        "SELECT {PROPOSITION_MEDIA_COLONNES},
                ua.nom AS auteur_nom, ua.prenom AS auteur_prenom, ua.email AS auteur_email,
                ud.nom AS decideur_nom, ud.prenom AS decideur_prenom
           FROM media_content.proposition_media pm
           LEFT JOIN iam.utilisateur ua ON ua.id = pm.auteur_id
           LEFT JOIN iam.utilisateur ud ON ud.id = pm.decideur
          WHERE {where_clause}
          ORDER BY pm.created_at ASC"
    );
    let mut q = sqlx::query_as::<_, PropositionMediaRow>(&requete).bind(support_id);
    if let Some(ref s) = params.statut {
        q = q.bind(s);
    }
    let rows = q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(
            rows.into_iter()
                .map(|r| r.to_response())
                .collect::<Vec<_>>(),
        ),
        error: None,
    }))
}

/// PATCH /api/medias/propositions/{id}/accepter — décision d'un co-détenteur
///
/// Le contributeur devient co-détenteur du support s'il s'agit d'une demande
/// d'animation ; une idée est simplement retenue.
pub async fn accepter_engagement(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<DecisionMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let proposition_id = chemin.into_inner();

    let commentaire = body
        .commentaire
        .as_deref()
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .map(str::to_string);

    // La garde est déterminée par la cible de la proposition — il faut donc la
    // lire avant de pouvoir l'exercer.
    let contexte: Option<(Uuid, String, Option<Uuid>, String)> = sqlx::query_as(
        "SELECT auteur_id, type_objet::text, target_id, statut::text
           FROM media_content.proposition_media WHERE id = $1",
    )
    .bind(proposition_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (auteur_id, type_objet, target_id, _) =
        contexte.ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;

    if !matches!(type_objet.as_str(), "animation_programme" | "idee_contenu") {
        return Err(ApiErreur::Validation(
            "Seules les idées et les demandes d'animation se décident ici : les autres propositions relèvent de la modération administrative".into(),
        ));
    }
    let target_id = target_id
        .ok_or_else(|| ApiErreur::Validation("Cette proposition ne vise aucun support".into()))?;
    let type_support = type_support_de(pool.get_ref(), target_id)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Le support visé n'existe plus".into()))?;

    garde_detenteur(pool.get_ref(), &type_support, target_id, moi, "co_detenteur").await?;

    let mut tx = pool.begin().await?;

    // Relecture verrouillée : deux co-détenteurs peuvent trancher en même temps.
    let statut: Option<String> = sqlx::query_scalar(
        "SELECT statut::text FROM media_content.proposition_media WHERE id = $1 FOR UPDATE",
    )
    .bind(proposition_id)
    .fetch_optional(&mut *tx)
    .await?;
    let statut = statut.ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;
    if statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette proposition a déjà été décidée".into(),
        ));
    }

    let objet_id = appliquer_acceptation_engagement(
        &mut tx,
        &type_objet,
        &type_support,
        target_id,
        auteur_id,
        moi,
    )
    .await?;

    sqlx::query(
        "UPDATE media_content.proposition_media
            SET statut = 'validee', decideur = $1, decide_at = NOW(),
                commentaire_decision = $2, objet_id_cree = $3, updated_at = NOW()
          WHERE id = $4",
    )
    .bind(moi)
    .bind(commentaire.as_deref())
    .bind(objet_id)
    .bind(proposition_id)
    .execute(&mut *tx)
    .await?;

    let message = if type_objet == "animation_programme" {
        "Votre demande d'animation a été acceptée : vous co-détenez désormais ce support."
    } else {
        "Votre idée a été retenue par l'équipe du support."
    };
    sqlx::query(
        "INSERT INTO arbre_genealogique.notifications (destinataire_id, type, message, lien_action)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(auteur_id)
    .bind(if type_objet == "animation_programme" {
        notification::media::CODETENTEUR_AJOUTE
    } else {
        notification::media::PROPOSITION_VALIDEE
    })
    .bind(message)
    .bind("/mon-compte/propositions-medias")
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "VALIDATION",
        "media_content",
        "proposition_media",
        Some(proposition_id),
        Some(serde_json::json!({ "statut": "en_attente", "objet_id_cree": null })),
        Some(serde_json::json!({
            "statut": "validee",
            "type_objet": type_objet,
            "type_support": type_support,
            "support_id": target_id,
            "objet_id_cree": objet_id,
            "commentaire_decision": commentaire,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let proposition = charger_proposition(pool.get_ref(), proposition_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(proposition),
        error: None,
    }))
}

/// PATCH /api/medias/propositions/{id}/refuser — décision d'un co-détenteur
pub async fn refuser_engagement(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<DecisionMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let proposition_id = chemin.into_inner();

    // Le motif est obligatoire et substantiel, comme en modération
    // administrative : l'auteur doit comprendre le refus (FR-033).
    let commentaire = body
        .commentaire
        .as_deref()
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le motif du refus est obligatoire".into()))?;
    if commentaire.chars().count() < LONGUEUR_MIN_MOTIF_REJET {
        return Err(ApiErreur::Validation(format!(
            "Le motif du refus doit compter au moins {} caractères",
            LONGUEUR_MIN_MOTIF_REJET
        )));
    }

    let contexte: Option<(Uuid, String, Option<Uuid>)> = sqlx::query_as(
        "SELECT auteur_id, type_objet::text, target_id
           FROM media_content.proposition_media WHERE id = $1",
    )
    .bind(proposition_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (auteur_id, type_objet, target_id) =
        contexte.ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;

    if !matches!(type_objet.as_str(), "animation_programme" | "idee_contenu") {
        return Err(ApiErreur::Validation(
            "Seules les idées et les demandes d'animation se décident ici".into(),
        ));
    }
    let target_id = target_id
        .ok_or_else(|| ApiErreur::Validation("Cette proposition ne vise aucun support".into()))?;
    let type_support = type_support_de(pool.get_ref(), target_id)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Le support visé n'existe plus".into()))?;

    garde_detenteur(pool.get_ref(), &type_support, target_id, moi, "co_detenteur").await?;

    let mut tx = pool.begin().await?;

    let statut: Option<String> = sqlx::query_scalar(
        "SELECT statut::text FROM media_content.proposition_media WHERE id = $1 FOR UPDATE",
    )
    .bind(proposition_id)
    .fetch_optional(&mut *tx)
    .await?;
    let statut = statut.ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;
    if statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette proposition a déjà été décidée".into(),
        ));
    }

    sqlx::query(
        "UPDATE media_content.proposition_media
            SET statut = 'rejetee', decideur = $1, decide_at = NOW(),
                commentaire_decision = $2, updated_at = NOW()
          WHERE id = $3",
    )
    .bind(moi)
    .bind(commentaire)
    .bind(proposition_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO arbre_genealogique.notifications (destinataire_id, type, message, lien_action)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(auteur_id)
    .bind(notification::media::PROPOSITION_REJETEE)
    .bind(format!(
        "Votre proposition n'a pas été retenue. Motif : {}",
        commentaire
    ))
    .bind("/mon-compte/propositions-medias")
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "REJET",
        "media_content",
        "proposition_media",
        Some(proposition_id),
        Some(serde_json::json!({ "statut": "en_attente" })),
        Some(serde_json::json!({
            "statut": "rejetee",
            "type_objet": type_objet,
            "support_id": target_id,
            "commentaire_decision": commentaire,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let proposition = charger_proposition(pool.get_ref(), proposition_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(proposition),
        error: None,
    }))
}

// ── Lecture unitaire, partagée par les mutations ci-dessus ────────────

pub async fn charger_proposition(
    pool: &PgPool,
    proposition_id: Uuid,
) -> Result<crate::models::media_proposition::PropositionMediaResponse, ApiErreur> {
    let row = sqlx::query_as::<_, PropositionMediaRow>(&format!(
        "SELECT {PROPOSITION_MEDIA_COLONNES},
                ua.nom AS auteur_nom, ua.prenom AS auteur_prenom, ua.email AS auteur_email,
                ud.nom AS decideur_nom, ud.prenom AS decideur_prenom
           FROM media_content.proposition_media pm
           LEFT JOIN iam.utilisateur ua ON ua.id = pm.auteur_id
           LEFT JOIN iam.utilisateur ud ON ud.id = pm.decideur
          WHERE pm.id = $1"
    ))
    .bind(proposition_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;
    Ok(row.to_response())
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/medias/themes — référentiel des thèmes phares (FR-030)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(sqlx::FromRow, serde::Serialize)]
struct ThemePhareResponse {
    id: Uuid,
    nom: String,
}

/// Les thèmes phares proposables pour un contenu, alimentant le sélecteur du
/// formulaire de proposition. Ce sont les catégories `shared.categorie` de
/// contexte `media`, seedées par la migration 09j.
///
/// Public : le formulaire est ouvert à tout membre, la liste n'a rien de
/// sensible.
pub async fn lister_themes_phares(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let themes = sqlx::query_as::<_, ThemePhareResponse>(
        "SELECT id, nom FROM shared.categorie
          WHERE contexte = 'media' AND actif = TRUE
          ORDER BY ordre ASC, nom ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(themes),
        error: None,
    }))
}

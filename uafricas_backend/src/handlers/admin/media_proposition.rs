//! File de modération des propositions de médias
//! (feature 001-refonte-tele-radio, US4, migration 09l).
//!
//! Endpoints :
//!   GET   /api/admin/medias/propositions
//!   GET   /api/admin/medias/propositions/{id}
//!   PATCH /api/admin/medias/propositions/{id}/valider
//!   PATCH /api/admin/medias/propositions/{id}/rejeter
//!
//! Garde : `verifier_permission!(admin, "media", …)`. Attention au piège de
//! nommage : `"media"` couvre radio et télé, `"media_content"` couvre vidafrica
//! et `"programme"` désigne les programmes d'échange.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_proposition::charger_proposition;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::media_detention::{
    rang_role, valider_type_support, AjouterDetenteurAdminRequest, SupportDetenteurRow,
    SUPPORT_DETENTEUR_COLONNES,
};
use crate::models::media_proposition::{
    DecisionMediaRequest, DonneesProposition, PropositionMediaListeResponse, PropositionMediaRow,
    PROPOSITION_MEDIA_COLONNES, LONGUEUR_MIN_MOTIF_REJET,
};
use crate::models::media_social::{
    descripteur_pour_type, table_pour_type, AuteurApercu, ChangerEtatMediaRequest,
    ContenuSignaleResponse, ContenuSignaleRow, ContenusSignalesListeResponse,
    SignalementDetailResponse, SignalementDetailRow, SignalementsAdminFiltres,
    ETATS_MODERATION_MEDIA, TYPES_MEDIA_AUTORISES,
};
use crate::models::notification;
use crate::services::audit;
use crate::services::contacts_media::{normaliser_url, texte_non_vide};
use crate::verifier_permission;
use crate::ApiResponse;

/// Slug ASCII, à l'image de `admin::propositions_salle::generer_slug`.
fn generer_slug(titre: &str) -> String {
    titre
        .to_lowercase()
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â', 'ä'], "a")
        .replace(['ù', 'û', 'ü'], "u")
        .replace(['î', 'ï'], "i")
        .replace(['ô', 'ö'], "o")
        .replace('ç', "c")
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

/// Les quatre tables portent un slug UNIQUE : deux chaînes homonymes validées
/// l'une après l'autre entreraient sinon en collision. On suffixe jusqu'à
/// trouver un slug libre, dans la transaction en cours.
async fn slug_libre(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    table: &str,
    base: &str,
) -> Result<String, ApiErreur> {
    let mut candidat = base.to_string();
    let mut suffixe = 1;
    loop {
        let pris: bool = sqlx::query_scalar(&format!(
            "SELECT EXISTS(SELECT 1 FROM {table} WHERE slug = $1)"
        ))
        .bind(&candidat)
        .fetch_one(&mut **tx)
        .await?;
        if !pris {
            return Ok(candidat);
        }
        suffixe += 1;
        candidat = format!("{base}-{suffixe}");
    }
}

async fn resoudre_pays(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    nom: Option<&String>,
) -> Result<Option<Uuid>, ApiErreur> {
    let Some(nom) = nom.map(|n| n.trim()).filter(|n| !n.is_empty()) else {
        return Ok(None);
    };
    let id: Option<Uuid> = sqlx::query_scalar("SELECT id FROM shared.pays WHERE LOWER(nom) = LOWER($1)")
        .bind(nom)
        .fetch_optional(&mut **tx)
        .await?;
    Ok(id)
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/admin/medias/propositions
// ═══════════════════════════════════════════════════════════════════════════

pub async fn lister_propositions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<crate::models::media_proposition::PropositionsAdminFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions = vec!["1 = 1".to_string()];
    let mut index = 1u32;
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
    if params.auteur.is_some() {
        conditions.push(format!("pm.auteur_id = ${}", index));
        index += 1;
    }
    let where_clause = conditions.join(" AND ");

    let requete_total =
        format!("SELECT COUNT(*) FROM media_content.proposition_media pm WHERE {where_clause}");
    let mut count_q = sqlx::query_scalar::<_, i64>(&requete_total);
    if let Some(ref s) = params.statut {
        count_q = count_q.bind(s);
    }
    if let Some(ref t) = params.type_objet {
        count_q = count_q.bind(t);
    }
    if let Some(a) = params.auteur {
        count_q = count_q.bind(a);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Les plus anciennes d'abord : une file de modération se traite dans
    // l'ordre d'arrivée, sans quoi les propositions les plus anciennes
    // s'enfoncent indéfiniment.
    let requete_liste = format!(
        "SELECT {PROPOSITION_MEDIA_COLONNES},
                ua.nom AS auteur_nom, ua.prenom AS auteur_prenom, ua.email AS auteur_email,
                ud.nom AS decideur_nom, ud.prenom AS decideur_prenom
           FROM media_content.proposition_media pm
           LEFT JOIN iam.utilisateur ua ON ua.id = pm.auteur_id
           LEFT JOIN iam.utilisateur ud ON ud.id = pm.decideur
          WHERE {where_clause}
          ORDER BY pm.created_at ASC
          LIMIT ${index} OFFSET ${}",
        index + 1
    );
    let mut q = sqlx::query_as::<_, PropositionMediaRow>(&requete_liste);
    if let Some(ref s) = params.statut {
        q = q.bind(s);
    }
    if let Some(ref t) = params.type_objet {
        q = q.bind(t);
    }
    if let Some(a) = params.auteur {
        q = q.bind(a);
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
// GET /api/admin/medias/propositions/{id}
// ═══════════════════════════════════════════════════════════════════════════

pub async fn obtenir_proposition(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let proposition = charger_proposition(pool.get_ref(), chemin.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(proposition),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PATCH /api/admin/medias/propositions/{id}/valider
// ═══════════════════════════════════════════════════════════════════════════

/// Crée l'objet métier et bascule la proposition, en UNE transaction.
///
/// Séquence : `SELECT … FOR UPDATE` (qui refuse de re-trancher une proposition
/// déjà décidée) → `INSERT` de l'objet → `INSERT` du premier co-détenteur en
/// `proprietaire` → `UPDATE` de la proposition → `INSERT` de la notification
/// **dans la transaction** → `COMMIT` → audit.
///
/// La notification est émise dans la transaction et non en fire-and-forget :
/// une décision de publication ne doit pas pouvoir être commitée sans que son
/// auteur en soit averti (FR-034).
pub async fn valider_proposition(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<DecisionMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let proposition_id = chemin.into_inner();

    let commentaire = body
        .commentaire
        .as_deref()
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .map(str::to_string);

    let mut tx = pool.begin().await?;

    let ligne: Option<(Uuid, String, Option<Uuid>, serde_json::Value, String)> = sqlx::query_as(
        "SELECT auteur_id, type_objet::text, target_id, donnees, statut::text
           FROM media_content.proposition_media
          WHERE id = $1
          FOR UPDATE",
    )
    .bind(proposition_id)
    .fetch_optional(&mut *tx)
    .await?;

    let (auteur_id, type_objet, target_id, donnees_brutes, statut) =
        ligne.ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;

    if statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette proposition a déjà été décidée".into(),
        ));
    }

    let donnees: DonneesProposition = serde_json::from_value(donnees_brutes.clone())
        .map_err(|e| ApiErreur::Validation(format!("Données de proposition illisibles: {}", e)))?;

    let objet_id = creer_objet(
        &mut tx,
        &type_objet,
        target_id,
        &donnees,
        auteur_id,
        admin.id,
    )
    .await?;

    // Le contributeur devient propriétaire du support qu'il a fait naître.
    if let Some(id) = objet_id {
        if matches!(type_objet.as_str(), "chaine_tv" | "station_radio") {
            sqlx::query(
                "INSERT INTO media_content.support_detenteur
                    (type_support, support_id, utilisateur_id, role, designe_par)
                 VALUES ($1::media_content.type_support_media, $2, $3, 'proprietaire', $4)
                 ON CONFLICT (type_support, support_id, utilisateur_id) DO NOTHING",
            )
            .bind(&type_objet)
            .bind(id)
            .bind(auteur_id)
            .bind(admin.id)
            .execute(&mut *tx)
            .await?;
        }
    }

    sqlx::query(
        "UPDATE media_content.proposition_media
            SET statut = 'validee',
                decideur = $1,
                decide_at = NOW(),
                commentaire_decision = $2,
                objet_id_cree = $3,
                updated_at = NOW()
          WHERE id = $4",
    )
    .bind(admin.id)
    .bind(commentaire.as_deref())
    .bind(objet_id)
    .bind(proposition_id)
    .execute(&mut *tx)
    .await?;

    let message = match commentaire {
        Some(ref c) => format!(
            "Votre proposition a été validée et publiée. Commentaire de l'administrateur : {}",
            c
        ),
        None => "Votre proposition a été validée : elle est désormais publiée.".to_string(),
    };
    sqlx::query(
        "INSERT INTO arbre_genealogique.notifications (destinataire_id, type, message, lien_action)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(auteur_id)
    .bind(notification::media::PROPOSITION_VALIDEE)
    .bind(&message)
    .bind("/mon-compte/propositions-medias")
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // ── Points d'engagement, APRÈS le COMMIT (US4) ───────────────────────────
    // `attribuer` prend un `&PgPool` et doit rester hors de la transaction
    // métier : une erreur d'attribution ne doit jamais annuler une validation
    // déjà décidée. L'anti-auto-attribution est explicite, un administrateur
    // qui valide sa propre proposition ne se crédite pas.
    if auteur_id != admin.id {
        match type_objet.as_str() {
            // Une demande d'animation acceptée par la file admin porte la MÊME
            // clé que la même demande acceptée par les co-détenteurs : quel que
            // soit le chemin, un seul crédit est possible.
            "animation_programme" => {
                crate::services::engagement::attribuer(
                    pool.get_ref(),
                    auteur_id,
                    "animation_support_acceptee",
                    Some(&type_objet),
                    Some(proposition_id),
                    &format!("animation:{proposition_id}"),
                )
                .await;
            }
            // Une idée retenue ne crée aucun contenu : aucune règle ne la couvre.
            "idee_contenu" => {}
            _ => {
                crate::services::engagement::attribuer(
                    pool.get_ref(),
                    auteur_id,
                    "proposition_media_validee",
                    Some(&type_objet),
                    objet_id,
                    &format!("prop_media:{proposition_id}"),
                )
                .await;
            }
        }
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "VALIDATION",
        "media_content",
        "proposition_media",
        Some(proposition_id),
        Some(serde_json::json!({ "statut": "en_attente", "objet_id_cree": null })),
        Some(serde_json::json!({
            "statut": "validee",
            "objet_id_cree": objet_id,
            "type_objet": type_objet,
            "commentaire_decision": commentaire,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let proposition = charger_proposition(pool.get_ref(), proposition_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "proposition": proposition,
            "objet_id_cree": objet_id,
        })),
        error: None,
    }))
}

/// Crée l'objet métier correspondant au type validé.
///
/// Renvoie `None` pour les types qui n'en créent aucun, le CHECK
/// `ck_prop_media_validation_a_objet` n'exempte que `idee_contenu`.
async fn creer_objet(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    type_objet: &str,
    target_id: Option<Uuid>,
    donnees: &DonneesProposition,
    auteur_id: Uuid,
    decideur: Uuid,
) -> Result<Option<Uuid>, ApiErreur> {
    // Les propositions d'engagement (US6) n'ont pas de nom et ne créent aucun
    // contenu : elles sont traitées avant d'exiger quoi que ce soit du payload.
    if matches!(type_objet, "animation_programme" | "idee_contenu") {
        let target_id = target_id.ok_or_else(|| {
            ApiErreur::Validation("Cette proposition ne vise aucun support".into())
        })?;
        // La cible est sondée dans la transaction en cours : un support
        // supprimé entre la soumission et la décision doit faire échouer la
        // validation, pas produire une détention orpheline.
        let type_support: Option<String> = sqlx::query_scalar(
            "SELECT CASE
                      WHEN EXISTS(SELECT 1 FROM media_content.chaine_tv
                                   WHERE id = $1 AND deleted_at IS NULL) THEN 'chaine_tv'
                      WHEN EXISTS(SELECT 1 FROM media_content.station_radio
                                   WHERE id = $1 AND deleted_at IS NULL) THEN 'station_radio'
                    END",
        )
        .bind(target_id)
        .fetch_one(&mut **tx)
        .await?;
        let type_support = type_support
            .ok_or_else(|| ApiErreur::NonTrouve("Le support visé n'existe plus".into()))?;

        return crate::handlers::media_proposition::appliquer_acceptation_engagement(
            tx,
            type_objet,
            &type_support,
            target_id,
            auteur_id,
            decideur,
        )
        .await;
    }

    let nom = donnees
        .nom
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .ok_or_else(|| ApiErreur::Validation("La proposition ne porte aucun nom".into()))?;
    let pays_id = resoudre_pays(tx, donnees.pays.as_ref()).await?;
    let langue = donnees.langue.as_deref().unwrap_or("Français");

    let id = match type_objet {
        "chaine_tv" => {
            let slug = slug_libre(tx, "media_content.chaine_tv", &generer_slug(nom)).await?;
            sqlx::query_scalar(
                "INSERT INTO media_content.chaine_tv
                    (nom, slug, description, stream_url, image_couverture_url, pays_id, langue,
                     etat, cree_par, role_partie_prenante, role_partie_prenante_autre,
                     contact_email, contact_telephone, contact_whatsapp,
                     contact_site_web, contact_adresse)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, 'publie', $8, $9, $10,
                         $11, $12, $13, $14, $15)
                 RETURNING id",
            )
            .bind(nom)
            .bind(&slug)
            .bind(donnees.description.as_deref())
            .bind(donnees.stream_url.as_deref())
            .bind(donnees.image_couverture_url.as_deref())
            .bind(pays_id)
            .bind(langue)
            .bind(auteur_id)
            .bind(donnees.role_partie_prenante.as_deref())
            .bind(donnees.role_partie_prenante_autre.as_deref())
            .bind(texte_non_vide(donnees.contact_email.as_deref()))
            .bind(texte_non_vide(donnees.contact_telephone.as_deref()))
            .bind(texte_non_vide(donnees.contact_whatsapp.as_deref()))
            .bind(normaliser_url(donnees.contact_site_web.as_deref()))
            .bind(texte_non_vide(donnees.contact_adresse.as_deref()))
            .fetch_one(&mut **tx)
            .await?
        }
        "station_radio" => {
            let slug = slug_libre(tx, "media_content.station_radio", &generer_slug(nom)).await?;
            // `origine_publication` est FORCÉE à 'territoire' : la bannière
            // Radio Africans relève d'une décision éditoriale de la plateforme,
            // jamais d'une demande du contributeur (FR-036). Le champ n'est même
            // pas exprimable côté client, `DonneesProposition` ne le portant pas.
            sqlx::query_scalar(
                "INSERT INTO media_content.station_radio
                    (nom, slug, description, stream_url, audio_url, image_couverture_url,
                     pays_id, etat, cree_par, origine_publication,
                     role_partie_prenante, role_partie_prenante_autre,
                     contact_email, contact_telephone, contact_whatsapp,
                     contact_site_web, contact_adresse)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, 'publie', $8, 'territoire', $9, $10,
                         $11, $12, $13, $14, $15)
                 RETURNING id",
            )
            .bind(nom)
            .bind(&slug)
            .bind(donnees.description.as_deref())
            .bind(donnees.stream_url.as_deref())
            .bind(donnees.audio_url.as_deref())
            .bind(donnees.image_couverture_url.as_deref())
            .bind(pays_id)
            .bind(auteur_id)
            .bind(donnees.role_partie_prenante.as_deref())
            .bind(donnees.role_partie_prenante_autre.as_deref())
            .bind(texte_non_vide(donnees.contact_email.as_deref()))
            .bind(texte_non_vide(donnees.contact_telephone.as_deref()))
            .bind(texte_non_vide(donnees.contact_whatsapp.as_deref()))
            .bind(normaliser_url(donnees.contact_site_web.as_deref()))
            .bind(texte_non_vide(donnees.contact_adresse.as_deref()))
            .fetch_one(&mut **tx)
            .await?
        }
        // ── Programmes conteneurs (009) ─────────────────────────────────
        // L'émission naît directement `publie` : c'est la décision
        // administrative qui vaut validation. Son support est celui désigné par
        // la proposition : `chaine_id` est NOT NULL depuis 09q.
        "emission_tele" => {
            let chaine_id = donnees.chaine_id.or(target_id).ok_or_else(|| {
                ApiErreur::Validation("Cette proposition ne désigne aucune chaîne".into())
            })?;
            let slug = slug_libre(tx, "media_content.emission_tele", &generer_slug(nom)).await?;
            sqlx::query_scalar(
                "INSERT INTO media_content.emission_tele
                    (chaine_id, titre, slug, description, image_couverture_url,
                     info_animateur, info_producteur, langue, theme_phare_id,
                     theme_phare_autre, cadence, etat, cree_par)
                 VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6, $7, $8, $9, $10,
                         'ponctuelle', 'publie', $11)
                 RETURNING id",
            )
            .bind(chaine_id)
            .bind(nom)
            .bind(&slug)
            .bind(donnees.description.as_deref())
            .bind(donnees.image_couverture_url.as_deref())
            .bind(donnees.info_animateur.as_deref())
            .bind(donnees.info_producteur.as_deref())
            .bind(langue)
            .bind(donnees.theme_phare_id)
            .bind(donnees.theme_phare_autre.as_deref())
            .bind(auteur_id)
            .fetch_one(&mut **tx)
            .await?
        }
        "emission_radio" => {
            let station_id = donnees.station_id.or(target_id).ok_or_else(|| {
                ApiErreur::Validation("Cette proposition ne désigne aucune station".into())
            })?;
            let slug = slug_libre(tx, "media_content.emission_radio", &generer_slug(nom)).await?;
            sqlx::query_scalar(
                "INSERT INTO media_content.emission_radio
                    (station_id, titre, slug, description, image_couverture_url,
                     info_animateur, info_producteur, langue, theme_phare_id,
                     theme_phare_autre, cadence, etat, cree_par)
                 VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6, $7, $8, $9, $10,
                         'ponctuelle', 'publie', $11)
                 RETURNING id",
            )
            .bind(station_id)
            .bind(nom)
            .bind(&slug)
            .bind(donnees.description.as_deref())
            .bind(donnees.image_couverture_url.as_deref())
            .bind(donnees.info_animateur.as_deref())
            .bind(donnees.info_producteur.as_deref())
            .bind(langue)
            .bind(donnees.theme_phare_id)
            .bind(donnees.theme_phare_autre.as_deref())
            .bind(auteur_id)
            .fetch_one(&mut **tx)
            .await?
        }

        // ── Épisodes (009) ──────────────────────────────────────────────
        // Créés directement en `publie` : la décision administrative EST la
        // validation, l'épisode ne repasse pas par la file de modération. Il
        // prend rang en fin de programme, comme tout épisode versé.
        "episode_tele" => {
            let emission_id = target_id.ok_or_else(|| {
                ApiErreur::Validation("Cette proposition ne désigne aucun programme".into())
            })?;
            let slug = slug_libre(tx, "media_content.episode_tele", &generer_slug(nom)).await?;
            sqlx::query_scalar(
                "INSERT INTO media_content.episode_tele
                    (emission_id, titre, slug, description, image_couverture_url, video_url,
                     ordre, etat, valide_par, valide_at, cree_par)
                 VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6,
                         (SELECT COALESCE(MAX(ordre), -1) + 1 FROM media_content.episode_tele
                           WHERE emission_id = $1 AND deleted_at IS NULL),
                         'publie', $7, NOW(), $8)
                 RETURNING id",
            )
            .bind(emission_id)
            .bind(nom)
            .bind(&slug)
            .bind(donnees.description.as_deref())
            .bind(donnees.image_couverture_url.as_deref())
            .bind(donnees.video_url.as_deref())
            .bind(decideur)
            .bind(auteur_id)
            .fetch_one(&mut **tx)
            .await?
        }
        "episode_radio" => {
            let emission_id = target_id.ok_or_else(|| {
                ApiErreur::Validation("Cette proposition ne désigne aucun programme".into())
            })?;
            let slug = slug_libre(tx, "media_content.episode_radio", &generer_slug(nom)).await?;
            sqlx::query_scalar(
                "INSERT INTO media_content.episode_radio
                    (emission_id, titre, slug, description, image_couverture_url, audio_url,
                     ordre, etat, valide_par, valide_at, cree_par)
                 VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6,
                         (SELECT COALESCE(MAX(ordre), -1) + 1 FROM media_content.episode_radio
                           WHERE emission_id = $1 AND deleted_at IS NULL),
                         'publie', $7, NOW(), $8)
                 RETURNING id",
            )
            .bind(emission_id)
            .bind(nom)
            .bind(&slug)
            .bind(donnees.description.as_deref())
            .bind(donnees.image_couverture_url.as_deref())
            .bind(donnees.audio_url.as_deref())
            .bind(decideur)
            .bind(auteur_id)
            .fetch_one(&mut **tx)
            .await?
        }
        // Les deux types d'engagement sont traités en tête de fonction ; tout
        // autre littéral serait un ENUM élargi sans code correspondant.
        autre => {
            return Err(ApiErreur::Validation(format!(
                "Les propositions de type « {} » ne sont pas traitées par cette file",
                autre
            )));
        }
    };

    Ok(Some(id))
}

// ═══════════════════════════════════════════════════════════════════════════
// PATCH /api/admin/medias/propositions/{id}/rejeter
// ═══════════════════════════════════════════════════════════════════════════

/// Le motif est obligatoire et substantiel : l'auteur doit pouvoir comprendre
/// le refus depuis son écran de suivi (FR-033). Doublé du CHECK SQL
/// `ck_prop_media_rejet_commente`, qui n'exige lui qu'un motif non vide.
pub async fn rejeter_proposition(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<DecisionMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let proposition_id = chemin.into_inner();

    let commentaire = body
        .commentaire
        .as_deref()
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .ok_or_else(|| {
            ApiErreur::Validation("Le motif du refus est obligatoire".into())
        })?;

    if commentaire.chars().count() < LONGUEUR_MIN_MOTIF_REJET {
        return Err(ApiErreur::Validation(format!(
            "Le motif du refus doit compter au moins {} caractères",
            LONGUEUR_MIN_MOTIF_REJET
        )));
    }

    let mut tx = pool.begin().await?;

    let ligne: Option<(Uuid, String)> = sqlx::query_as(
        "SELECT auteur_id, statut::text FROM media_content.proposition_media
          WHERE id = $1 FOR UPDATE",
    )
    .bind(proposition_id)
    .fetch_optional(&mut *tx)
    .await?;

    let (auteur_id, statut) =
        ligne.ok_or_else(|| ApiErreur::NonTrouve("Proposition introuvable".into()))?;

    if statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette proposition a déjà été décidée".into(),
        ));
    }

    sqlx::query(
        "UPDATE media_content.proposition_media
            SET statut = 'rejetee',
                decideur = $1,
                decide_at = NOW(),
                commentaire_decision = $2,
                updated_at = NOW()
          WHERE id = $3",
    )
    .bind(admin.id)
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
    .bind(format!("Votre proposition a été refusée. Motif : {}", commentaire))
    .bind("/mon-compte/propositions-medias")
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "REJET",
        "media_content",
        "proposition_media",
        Some(proposition_id),
        Some(serde_json::json!({ "statut": "en_attente" })),
        Some(serde_json::json!({
            "statut": "rejetee",
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

// ═══════════════════════════════════════════════════════════════════════════
// CO-DÉTENTEURS : vue et intervention administratives (US5)
// ═══════════════════════════════════════════════════════════════════════════
//
// Les co-détenteurs se gèrent normalement entre eux, par invitation
// (`handlers/media_detention.rs`). Ces trois routes existent pour les cas où
// personne ne peut plus le faire : support dont le dernier détenteur s'est
// retiré, propriétaire injoignable, correction d'une désignation erronée.

/// GET /api/admin/medias/{type_support}/{support_id}/detenteurs
pub async fn lister_detenteurs_admin(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;

    let rows = sqlx::query_as::<_, SupportDetenteurRow>(&format!(
        "SELECT {SUPPORT_DETENTEUR_COLONNES},
                u.nom AS utilisateur_nom, u.prenom AS utilisateur_prenom,
                u.email AS utilisateur_email, u.photo_url AS utilisateur_photo
           FROM media_content.support_detenteur sd
           LEFT JOIN iam.utilisateur u ON u.id = sd.utilisateur_id
          WHERE sd.type_support = $1::media_content.type_support_media
            AND sd.support_id = $2
          ORDER BY sd.actif DESC, sd.role ASC, sd.designe_at ASC"
    ))
    .bind(&type_support)
    .bind(support_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rows.into_iter().map(|r| r.to_response()).collect::<Vec<_>>()),
        error: None,
    }))
}

/// POST /api/admin/medias/{type_support}/{support_id}/detenteurs
///
/// Désignation directe, sans invitation : c'est le recours quand plus personne
/// ne détient le support. Le rôle `proprietaire` est admis ici, contrairement
/// à l'invitation entre membres , mais l'index unique
/// `uq_support_un_proprietaire` en refusera un second.
pub async fn ajouter_detenteur_admin(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<AjouterDetenteurAdminRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;

    let role = body.role.as_deref().map(str::trim).unwrap_or("co_detenteur");
    if rang_role(role).is_none() {
        return Err(ApiErreur::Validation(format!(
            "Rôle de détenteur « {} » inconnu",
            role
        )));
    }

    let actif: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.utilisateur
            WHERE id = $1 AND etat::text = 'actif' AND deleted_at IS NULL)",
    )
    .bind(body.utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !actif {
        return Err(ApiErreur::Validation(
            "Utilisateur introuvable ou inactif".into(),
        ));
    }

    // Upsert-réactivation : le retrait étant un soft delete, une ligne inactive
    // peut préexister (modèle `admin/moderateurs_afrolang.rs:59-190`).
    let detenteur_id: Uuid = sqlx::query_scalar(
        "INSERT INTO media_content.support_detenteur
            (type_support, support_id, utilisateur_id, role, designe_par)
         VALUES ($1::media_content.type_support_media, $2, $3,
                 $4::media_content.role_detenteur, $5)
         ON CONFLICT (type_support, support_id, utilisateur_id)
         DO UPDATE SET actif = TRUE,
                       retire_at = NULL,
                       role = EXCLUDED.role,
                       designe_par = EXCLUDED.designe_par,
                       designe_at = NOW(),
                       updated_at = NOW()
         RETURNING id",
    )
    .bind(&type_support)
    .bind(support_id)
    .bind(body.utilisateur_id)
    .bind(role)
    .bind(admin.id)
    .fetch_one(pool.get_ref())
    .await?;

    notification::creer_notification(
        pool.get_ref(),
        body.utilisateur_id,
        notification::media::CODETENTEUR_AJOUTE,
        "Vous avez été désigné détenteur d'un support média par l'administration.",
        Some("/mon-compte/mes-supports"),
    )
    .await;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "media_content",
        "support_detenteur",
        Some(detenteur_id),
        Some(serde_json::json!({ "actif": false })),
        Some(serde_json::json!({
            "actif": true,
            "type_support": type_support,
            "support_id": support_id,
            "utilisateur_id": body.utilisateur_id,
            "role": role,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": detenteur_id, "role": role })),
        error: None,
    }))
}

/// DELETE /api/admin/medias/{type_support}/{support_id}/detenteurs/{utilisateur_id}
pub async fn retirer_detenteur_admin(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let (type_support, support_id, utilisateur_id) = chemin.into_inner();
    valider_type_support(&type_support)?;

    let modifie = sqlx::query(
        "UPDATE media_content.support_detenteur
            SET actif = FALSE, retire_at = NOW(), updated_at = NOW()
          WHERE type_support = $1::media_content.type_support_media
            AND support_id = $2 AND utilisateur_id = $3 AND actif = TRUE",
    )
    .bind(&type_support)
    .bind(support_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?
    .rows_affected();

    if modifie == 0 {
        return Err(ApiErreur::NonTrouve(
            "Ce membre ne détient pas ce support".into(),
        ));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "media_content",
        "support_detenteur",
        None,
        Some(serde_json::json!({
            "actif": true,
            "type_support": type_support,
            "support_id": support_id,
            "utilisateur_id": utilisateur_id,
        })),
        Some(serde_json::json!({
            "actif": false,
            "type_support": type_support,
            "support_id": support_id,
            "utilisateur_id": utilisateur_id,
        })),
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

// ═══════════════════════════════════════════════════════════════════════════
// FILE DES CONTENUS SIGNALÉS (US7 : FR-051)
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/admin/medias/signalements
///
/// File des contenus ayant reçu au moins un signalement, triée par nombre de
/// signalements décroissant : le plus contesté remonte en tête.
///
/// Les quatre tables médias étant hétérogènes (colonne de titre différente),
/// chacune est interrogée séparément puis les résultats sont fusionnés en
/// mémoire : le volume d'un contenu signalé restant faible par nature.
pub async fn lister_signalements(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<SignalementsAdminFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);

    // Filtre de type : confronté à la whitelist avant toute interpolation.
    let types: Vec<&str> = match params.type_media.as_deref() {
        Some(t) => {
            if !TYPES_MEDIA_AUTORISES.contains(&t) {
                return Err(ApiErreur::Validation(format!(
                    "Type de média « {t} » non supporté"
                )));
            }
            vec![TYPES_MEDIA_AUTORISES
                .iter()
                .find(|v| **v == t)
                .copied()
                .expect("type validé ci-dessus")]
        }
        None => TYPES_MEDIA_AUTORISES.to_vec(),
    };

    let mut tous: Vec<ContenuSignaleResponse> = Vec::new();

    for type_media in types {
        let d = descripteur_pour_type(type_media)
            .ok_or_else(|| ApiErreur::Validation("Type de média non supporté".to_string()))?;
        let table = d.table;
        let colonne_titre = d.colonne_titre;

        // `suspendu = true` restreint aux contenus déjà retirés de l'antenne.
        let filtre_etat = match params.suspendu {
            Some(true) => "AND m.etat = 'suspendu'",
            Some(false) => "AND m.etat <> 'suspendu'",
            None => "",
        };

        let rows = sqlx::query_as::<_, ContenuSignaleRow>(&format!(
            "SELECT m.id,
                    m.{colonne_titre} AS titre,
                    m.slug,
                    m.etat,
                    m.nombre_signalements,
                    m.image_couverture_url,
                    (SELECT MAX(s.created_at)
                       FROM media_content.signalement_media s
                      WHERE s.type_media = $1 AND s.media_id = m.id) AS dernier_signalement
               FROM {table} m
              WHERE m.deleted_at IS NULL
                AND m.nombre_signalements > 0
                {filtre_etat}"
        ))
        .bind(type_media)
        .fetch_all(pool.get_ref())
        .await?;

        for r in rows {
            let url_detail = r
                .slug
                .as_ref()
                .map(|s| format!("{}/{}", d.base_url, s));
            tous.push(ContenuSignaleResponse {
                id: r.id,
                type_media: type_media.to_string(),
                titre: r.titre,
                slug: r.slug,
                etat: r.etat,
                nombre_signalements: r.nombre_signalements,
                image_couverture_url: r.image_couverture_url,
                url_detail,
                dernier_signalement: r.dernier_signalement,
            });
        }
    }

    // Le plus contesté d'abord ; à égalité, le signalement le plus récent.
    tous.sort_by(|a, b| {
        b.nombre_signalements
            .cmp(&a.nombre_signalements)
            .then(b.dernier_signalement.cmp(&a.dernier_signalement))
    });

    let total = tous.len() as i64;
    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };
    let debut = ((page - 1) * par_page).min(total) as usize;
    let fin = (debut as i64 + par_page).min(total) as usize;
    let contenus = tous.drain(debut..fin).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ContenusSignalesListeResponse {
            contenus,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/admin/medias/signalements/{type_media}/{id}
///
/// Détail des signalements individuels reçus par un contenu, pour l'arbitrage.
pub async fn detail_signalements(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    let (type_media, media_id) = chemin.into_inner();

    if !TYPES_MEDIA_AUTORISES.contains(&type_media.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Type de média « {type_media} » non supporté"
        )));
    }

    let rows = sqlx::query_as::<_, SignalementDetailRow>(
        "SELECT s.id, s.motif, s.description, s.created_at, s.signale_par,
                u.nom  AS auteur_nom,
                u.prenom AS auteur_prenom,
                CASE WHEN u.deleted_at IS NULL THEN u.photo_url ELSE NULL END AS auteur_photo_url
           FROM media_content.signalement_media s
           LEFT JOIN iam.utilisateur u ON u.id = s.signale_par
          WHERE s.type_media = $1 AND s.media_id = $2
          ORDER BY s.created_at DESC",
    )
    .bind(&type_media)
    .bind(media_id)
    .fetch_all(pool.get_ref())
    .await?;

    let signalements: Vec<SignalementDetailResponse> = rows
        .into_iter()
        .map(|r| SignalementDetailResponse {
            id: r.id,
            motif: r.motif,
            description: r.description,
            created_at: r.created_at,
            auteur: AuteurApercu {
                id: r.signale_par,
                nom: r.auteur_nom.unwrap_or_else(|| "Compte".to_string()),
                prenom: r.auteur_prenom.unwrap_or_else(|| "supprimé".to_string()),
                photo_url: r.auteur_photo_url,
            },
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(signalements),
        error: None,
    }))
}

/// PATCH /api/admin/medias/{type_media}/{id}/etat
///
/// Arbitrage d'un contenu signalé (FR-051).
///
/// **Le rétablissement remet `nombre_signalements = 0`** : sans cette remise à
/// zéro, le seuil resterait franchi et le contenu serait resuspendu au premier
/// signalement suivant. Les lignes de `signalement_media` sont conservées pour
/// l'historique : l'unicité par membre empêche toute inflation artificielle du
/// nouveau compteur.
///
/// `supprime` est un soft delete : le contenu quitte l'antenne définitivement
/// mais reste consultable en base.
pub async fn changer_etat_media(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<ChangerEtatMediaRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let (type_media, media_id) = chemin.into_inner();

    if !TYPES_MEDIA_AUTORISES.contains(&type_media.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Type de média « {type_media} » non supporté"
        )));
    }
    let nouvel_etat = body.etat.trim();
    if !ETATS_MODERATION_MEDIA.contains(&nouvel_etat) {
        return Err(ApiErreur::Validation(
            "L'état doit valoir « publie », « suspendu » ou « supprime »".to_string(),
        ));
    }

    let table = table_pour_type(&type_media)
        .ok_or_else(|| ApiErreur::Validation("Type de média non supporté".to_string()))?;

    // État antérieur, pour l'audit et pour distinguer un no-op d'un 404.
    let avant: Option<(String, i32)> = sqlx::query_as(&format!(
        "SELECT etat, nombre_signalements FROM {table}
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(media_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (ancien_etat, anciens_signalements) =
        avant.ok_or_else(|| ApiErreur::NonTrouve("Contenu introuvable".to_string()))?;

    // Rétablissement : compteur remis à zéro. Suppression : soft delete.
    let requete = if nouvel_etat == "publie" {
        format!(
            "UPDATE {table}
                SET etat = 'publie', nombre_signalements = 0, updated_at = NOW()
              WHERE id = $1 AND deleted_at IS NULL"
        )
    } else if nouvel_etat == "supprime" {
        format!(
            "UPDATE {table}
                SET etat = 'supprime', deleted_at = NOW(), updated_at = NOW()
              WHERE id = $1 AND deleted_at IS NULL"
        )
    } else {
        format!(
            "UPDATE {table}
                SET etat = 'suspendu', updated_at = NOW()
              WHERE id = $1 AND deleted_at IS NULL"
        )
    };

    sqlx::query(&requete)
        .bind(media_id)
        .execute(pool.get_ref())
        .await?;

    let action = match nouvel_etat {
        "publie" => "RETABLISSEMENT",
        "supprime" => "SUPPRESSION_MODERATION",
        _ => "SUSPENSION",
    };
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        action,
        "media_content",
        &type_media,
        Some(media_id),
        Some(serde_json::json!({
            "etat": ancien_etat,
            "nombre_signalements": anciens_signalements,
        })),
        Some(serde_json::json!({
            "etat": nouvel_etat,
            "nombre_signalements": if nouvel_etat == "publie" { 0 } else { anciens_signalements },
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": media_id,
            "type_media": type_media,
            "etat": nouvel_etat,
            "nombre_signalements": if nouvel_etat == "publie" { 0 } else { anciens_signalements },
        })),
        error: None,
    }))
}

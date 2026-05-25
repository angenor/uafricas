//! CRUD des avis de recherche (Retrouve Amis).

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use futures_util::StreamExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::retrouve_amis::*;
use crate::services::audit;
use crate::ApiResponse;

use super::commun::*;


/// POST /api/retrouve-amis/avis (multipart/form-data)
/// Créer un nouvel avis de recherche avec upload photo et déclencher le matching
pub async fn creer_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    // ── Parsing multipart ────────────────────────────────────
    let mut nom_recherche: Option<String> = None;
    let mut prenom_recherche: Option<String> = None;
    let mut surnom: Option<String> = None;
    let mut ecole: Option<String> = None;
    let mut ville: Option<String> = None;
    let mut pays_id: Option<Uuid> = None;
    let mut periode_debut: Option<i32> = None;
    let mut periode_fin: Option<i32> = None;
    let mut description: Option<String> = None;
    let mut est_anonyme: bool = false;
    let mut genre_recherche: Option<String> = None;
    let mut type_relation: Option<String> = None;
    let mut type_relation_autre: Option<String> = None;
    let mut comment_connu: Option<String> = None;
    let mut localite_rencontre: Option<String> = None;
    let mut ecole_rencontre: Option<String> = None;
    let mut ville_rencontre: Option<String> = None;
    let mut jamais_rencontre: bool = false;
    let mut rencontre_reseaux_sociaux: bool = false;
    let mut reseaux_sociaux: Option<String> = None;
    let mut description_physique: Option<String> = None;
    let mut partage_coordonnees: bool = false;
    let mut coordonnees_email: Option<String> = None;
    let mut coordonnees_telephone: Option<String> = None;
    let mut coordonnees_whatsapp: Option<String> = None;
    let mut photo_url: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            ApiErreur::Upload(format!("Erreur lecture multipart: {}", e))
        })?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        match nom_champ.as_str() {
            "nom_recherche" => nom_recherche = Some(lire_champ_texte_avis(&mut field).await?),
            "prenom_recherche" => prenom_recherche = lire_champ_option(&mut field).await?,
            "surnom" => surnom = lire_champ_option(&mut field).await?,
            "ecole" => ecole = lire_champ_option(&mut field).await?,
            "ville" => ville = lire_champ_option(&mut field).await?,
            "pays_id" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                if !val.trim().is_empty() {
                    pays_id = Some(val.trim().parse::<Uuid>().map_err(|_| {
                        ApiErreur::Validation("pays_id invalide".into())
                    })?);
                }
            }
            "periode_debut" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                if !val.trim().is_empty() {
                    periode_debut = Some(val.trim().parse::<i32>().map_err(|_| {
                        ApiErreur::Validation("periode_debut invalide".into())
                    })?);
                }
            }
            "periode_fin" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                if !val.trim().is_empty() {
                    periode_fin = Some(val.trim().parse::<i32>().map_err(|_| {
                        ApiErreur::Validation("periode_fin invalide".into())
                    })?);
                }
            }
            "description" => description = lire_champ_option(&mut field).await?,
            "est_anonyme" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                est_anonyme = val == "true" || val == "1";
            }
            "genre_recherche" => genre_recherche = lire_champ_option(&mut field).await?,
            "type_relation" => type_relation = lire_champ_option(&mut field).await?,
            "type_relation_autre" => type_relation_autre = lire_champ_option(&mut field).await?,
            "comment_connu" => comment_connu = lire_champ_option(&mut field).await?,
            "localite_rencontre" => localite_rencontre = lire_champ_option(&mut field).await?,
            "ecole_rencontre" => ecole_rencontre = lire_champ_option(&mut field).await?,
            "ville_rencontre" => ville_rencontre = lire_champ_option(&mut field).await?,
            "jamais_rencontre" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                jamais_rencontre = val == "true" || val == "1";
            }
            "rencontre_reseaux_sociaux" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                rencontre_reseaux_sociaux = val == "true" || val == "1";
            }
            "reseaux_sociaux" => reseaux_sociaux = lire_champ_option(&mut field).await?,
            "description_physique" => description_physique = lire_champ_option(&mut field).await?,
            "partage_coordonnees" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                partage_coordonnees = val == "true" || val == "1";
            }
            "coordonnees_email" => coordonnees_email = lire_champ_option(&mut field).await?,
            "coordonnees_telephone" => coordonnees_telephone = lire_champ_option(&mut field).await?,
            "coordonnees_whatsapp" => coordonnees_whatsapp = lire_champ_option(&mut field).await?,
            "photo" => {
                // Valider le MIME type
                let content_type = field.content_type().map(|ct| ct.to_string()).unwrap_or_default();
                let types_autorises = ["image/jpeg", "image/png", "image/webp"];
                if !types_autorises.iter().any(|t| content_type.starts_with(t)) {
                    return Err(ApiErreur::Validation(
                        "Format photo invalide. Formats acceptes : JPEG, PNG, WebP".into(),
                    ));
                }

                let nom_original = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));

                let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
                let chemin_complet = format!("{}/retrouve-amis/{}", upload_dir.get_ref(), nom_fichier);

                sauvegarder_photo_avis(&mut field, &chemin_complet, 5 * 1024 * 1024).await?;
                photo_url = Some(format!("/uploads/retrouve-amis/{}", nom_fichier));
            }
            _ => {
                log::warn!("Champ multipart inconnu ignore: {}", nom_champ);
            }
        }
    }

    // ── Validation ───────────────────────────────────────────
    let nom_recherche = nom_recherche
        .filter(|v| !v.trim().is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le nom recherche est obligatoire".into()))?;

    // Au moins un critere en plus du nom
    let a_critere = type_relation.is_some()
        || localite_rencontre.as_ref().map_or(false, |v| !v.trim().is_empty())
        || ecole_rencontre.as_ref().map_or(false, |v| !v.trim().is_empty())
        || ville_rencontre.as_ref().map_or(false, |v| !v.trim().is_empty())
        || jamais_rencontre
        || rencontre_reseaux_sociaux;
    if !a_critere {
        return Err(ApiErreur::Validation(
            "Au moins un critere supplementaire est requis (type de relation, lieu de rencontre, ecole ou reseaux sociaux)".into(),
        ));
    }

    // Coordonnees valides si partage active
    if partage_coordonnees {
        let a_coordonnee = coordonnees_email.as_ref().map_or(false, |v| !v.trim().is_empty())
            || coordonnees_telephone.as_ref().map_or(false, |v| !v.trim().is_empty())
            || coordonnees_whatsapp.as_ref().map_or(false, |v| !v.trim().is_empty());
        if !a_coordonnee {
            return Err(ApiErreur::Validation(
                "Au moins une coordonnee (email, telephone ou WhatsApp) est requise si le partage est active".into(),
            ));
        }
    }

    // Periode coherente
    if let (Some(debut), Some(fin)) = (periode_debut, periode_fin) {
        if debut > fin {
            return Err(ApiErreur::Validation(
                "La periode de debut doit etre anterieure a la periode de fin".into(),
            ));
        }
    }

    // Limite de 10 avis actifs
    let compte: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM retrouve_amis.avis_recherche WHERE auteur_id = $1 AND etat = 'actif' AND deleted_at IS NULL"
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if compte.0 >= 10 {
        return Err(ApiErreur::Validation(
            "Limite de 10 avis de recherche actifs atteinte".into(),
        ));
    }

    // ── Insertion ────────────────────────────────────────────
    let slug = generer_slug_avis(&nom_recherche, prenom_recherche.as_deref());

    let avis_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.avis_recherche
         (auteur_id, nom_recherche, prenom_recherche, surnom, ecole, ville, pays_id, periode_debut, periode_fin, description,
          est_anonyme, genre_recherche, type_relation, type_relation_autre, comment_connu,
          localite_rencontre, ecole_rencontre, ville_rencontre, jamais_rencontre,
          rencontre_reseaux_sociaux, reseaux_sociaux,
          photo_url, description_physique, partage_coordonnees, coordonnees_email, coordonnees_telephone, coordonnees_whatsapp,
          est_public, slug, date_publication_publique)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                 $11, $12::retrouve_amis.genre_personne, $13::retrouve_amis.type_relation_recherche, $14, $15,
                 $16, $17, $18, $19,
                 $20, $21,
                 $22, $23, $24, $25, $26, $27,
                 TRUE, $28, NOW())
         RETURNING id"
    )
    .bind(utilisateur_id)
    .bind(&nom_recherche)
    .bind(&prenom_recherche)
    .bind(&surnom)
    .bind(&ecole)
    .bind(&ville)
    .bind(pays_id)
    .bind(periode_debut)
    .bind(periode_fin)
    .bind(&description)
    .bind(est_anonyme)
    .bind(&genre_recherche)
    .bind(&type_relation)
    .bind(&type_relation_autre)
    .bind(&comment_connu)
    .bind(&localite_rencontre)
    .bind(&ecole_rencontre)
    .bind(&ville_rencontre)
    .bind(jamais_rencontre)
    .bind(rencontre_reseaux_sociaux)
    .bind(&reseaux_sociaux)
    .bind(&photo_url)
    .bind(&description_physique)
    .bind(partage_coordonnees)
    .bind(&coordonnees_email)
    .bind(&coordonnees_telephone)
    .bind(&coordonnees_whatsapp)
    .bind(&slug)
    .fetch_one(pool.get_ref())
    .await?;

    // Déclencher le matching
    let correspondances: Vec<CorrespondanceResultat> = sqlx::query_as(
        "SELECT cible_type::text AS type_cible, cible_id, score_total::float8 AS score_total, details
         FROM retrouve_amis.calculer_correspondances($1)
         WHERE score_total >= 60"
    )
    .bind(avis_id.0)
    .fetch_all(pool.get_ref())
    .await?;

    let mut nb_correspondances: i64 = 0;

    for corr in &correspondances {
        // Insérer la correspondance
        let corr_id: (Uuid,) = match corr.type_cible.as_str() {
            "avis" => {
                sqlx::query_as(
                    "INSERT INTO retrouve_amis.correspondance
                     (avis_id, type_cible, cible_avis_id, score, details_score, expire_at)
                     VALUES ($1, 'avis', $2, $3, $4, NOW() + INTERVAL '30 days')
                     RETURNING id"
                )
                .bind(avis_id.0)
                .bind(corr.cible_id)
                .bind(corr.score_total)
                .bind(&corr.details)
                .fetch_one(pool.get_ref())
                .await?
            }
            "profil" => {
                sqlx::query_as(
                    "INSERT INTO retrouve_amis.correspondance
                     (avis_id, type_cible, cible_utilisateur_id, score, details_score, expire_at)
                     VALUES ($1, 'profil', $2, $3, $4, NOW() + INTERVAL '30 days')
                     RETURNING id"
                )
                .bind(avis_id.0)
                .bind(corr.cible_id)
                .bind(corr.score_total)
                .bind(&corr.details)
                .fetch_one(pool.get_ref())
                .await?
            }
            _ => continue,
        };

        // Créer notification pour l'auteur de l'avis source
        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'nouvelle_correspondance')"
        )
        .bind(utilisateur_id)
        .bind(corr_id.0)
        .execute(pool.get_ref())
        .await?;

        // Créer notification pour la cible
        let cible_utilisateur_id = match corr.type_cible.as_str() {
            "avis" => {
                let row: (Uuid,) = sqlx::query_as(
                    "SELECT auteur_id FROM retrouve_amis.avis_recherche WHERE id = $1"
                )
                .bind(corr.cible_id)
                .fetch_one(pool.get_ref())
                .await?;
                row.0
            }
            "profil" => corr.cible_id,
            _ => continue,
        };

        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'nouvelle_correspondance')"
        )
        .bind(cible_utilisateur_id)
        .bind(corr_id.0)
        .execute(pool.get_ref())
        .await?;

        nb_correspondances += 1;
    }

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "avis_recherche",
        Some(avis_id.0),
        None,
        Some(serde_json::json!({
            "nom_recherche": &nom_recherche,
            "correspondances_trouvees": nb_correspondances
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(CreerAvisResponse {
            id: avis_id.0,
            etat: "actif".to_string(),
            slug,
            correspondances_trouvees: nb_correspondances,
        }),
        error: None,
    }))
}

/// GET /api/retrouve-amis/avis
/// Lister les avis de recherche de l'utilisateur connecté
pub async fn lister_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    let etat = query.get("etat").cloned();
    let page: i64 = query.get("page").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let par_page: i64 = query.get("par_page").and_then(|v| v.parse().ok()).unwrap_or(20).min(100).max(1);
    let tri = query.get("tri").cloned().unwrap_or_else(|| "created_at".to_string());
    let ordre = query.get("ordre").cloned().unwrap_or_else(|| "desc".to_string());

    // Valider la colonne de tri
    let tri_valide = if AVIS_TRI_COLONNES.contains(&tri.as_str()) { &tri } else { "created_at" };
    let ordre_valide = if ordre == "asc" { "ASC" } else { "DESC" };

    let offset = (page - 1) * par_page;

    // Construire la requête dynamiquement
    let mut conditions = vec!["a.auteur_id = $1".to_string(), "a.deleted_at IS NULL".to_string()];
    if etat.is_some() {
        conditions.push("a.etat::text = $2".to_string());
    }

    let where_clause = conditions.join(" AND ");
    let count_sql = format!(
        "SELECT COUNT(*) FROM retrouve_amis.avis_recherche a WHERE {}",
        where_clause
    );
    let list_sql = format!(
        "SELECT a.id, a.nom_recherche, a.prenom_recherche, a.surnom, a.ecole, a.ville,
                a.pays_id, a.etat::text AS etat, a.periode_debut, a.periode_fin, a.description,
                a.est_anonyme, a.genre_recherche::text AS genre_recherche,
                a.type_relation::text AS type_relation, a.type_relation_autre,
                a.localite_rencontre, a.ecole_rencontre, a.ville_rencontre,
                a.jamais_rencontre, a.rencontre_reseaux_sociaux, a.reseaux_sociaux,
                a.photo_url, a.description_physique,
                a.created_at, a.updated_at, a.deleted_at,
                a.est_public, a.slug, a.compteur_partages,
                p.id AS pays_info_id, p.nom AS pays_nom,
                (SELECT COUNT(*) FROM retrouve_amis.correspondance c WHERE c.avis_id = a.id) AS nb_correspondances
         FROM retrouve_amis.avis_recherche a
         LEFT JOIN shared.pays p ON p.id = a.pays_id
         WHERE {}
         ORDER BY a.{} {}
         LIMIT {} OFFSET {}",
        where_clause, tri_valide, ordre_valide, par_page, offset
    );

    let total: (i64,) = if let Some(ref e) = etat {
        sqlx::query_as(&count_sql)
            .bind(utilisateur_id)
            .bind(e)
            .fetch_one(pool.get_ref())
            .await?
    } else {
        sqlx::query_as(&count_sql)
            .bind(utilisateur_id)
            .fetch_one(pool.get_ref())
            .await?
    };

    #[derive(sqlx::FromRow)]
    struct AvisRow {
        id: Uuid,
        nom_recherche: String,
        prenom_recherche: Option<String>,
        surnom: Option<String>,
        ecole: Option<String>,
        ville: Option<String>,
        pays_id: Option<Uuid>,
        etat: String,
        periode_debut: Option<i32>,
        periode_fin: Option<i32>,
        description: Option<String>,
        est_anonyme: bool,
        genre_recherche: Option<String>,
        type_relation: Option<String>,
        type_relation_autre: Option<String>,
        localite_rencontre: Option<String>,
        ecole_rencontre: Option<String>,
        ville_rencontre: Option<String>,
        jamais_rencontre: bool,
        rencontre_reseaux_sociaux: bool,
        reseaux_sociaux: Option<String>,
        photo_url: Option<String>,
        description_physique: Option<String>,
        created_at: chrono::DateTime<Utc>,
        updated_at: chrono::DateTime<Utc>,
        #[allow(dead_code)]
        deleted_at: Option<chrono::DateTime<Utc>>,
        est_public: bool,
        slug: Option<String>,
        compteur_partages: i32,
        #[allow(dead_code)]
        pays_info_id: Option<Uuid>,
        pays_nom: Option<String>,
        nb_correspondances: i64,
    }

    let rows: Vec<AvisRow> = if let Some(ref e) = etat {
        sqlx::query_as(&list_sql)
            .bind(utilisateur_id)
            .bind(e)
            .fetch_all(pool.get_ref())
            .await?
    } else {
        sqlx::query_as(&list_sql)
            .bind(utilisateur_id)
            .fetch_all(pool.get_ref())
            .await?
    };

    let avis: Vec<AvisRechercheResponse> = rows
        .into_iter()
        .map(|r| AvisRechercheResponse {
            id: r.id,
            nom_recherche: r.nom_recherche,
            prenom_recherche: r.prenom_recherche,
            surnom: r.surnom,
            ecole: r.ecole,
            ville: r.ville,
            pays: r.pays_info_id.map(|id| PaysInfo {
                id,
                nom: r.pays_nom.unwrap_or_default(),
            }),
            periode_debut: r.periode_debut,
            periode_fin: r.periode_fin,
            etat: r.etat,
            nb_correspondances: r.nb_correspondances,
            est_public: r.est_public,
            slug: r.slug,
            compteur_partages: r.compteur_partages,
            est_anonyme: r.est_anonyme,
            genre_recherche: r.genre_recherche,
            type_relation: r.type_relation,
            type_relation_autre: r.type_relation_autre,
            localite_rencontre: r.localite_rencontre,
            ecole_rencontre: r.ecole_rencontre,
            ville_rencontre: r.ville_rencontre,
            jamais_rencontre: r.jamais_rencontre,
            rencontre_reseaux_sociaux: r.rencontre_reseaux_sociaux,
            reseaux_sociaux: r.reseaux_sociaux,
            photo_url: r.photo_url,
            description_physique: r.description_physique,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AvisRechercheListeResponse {
            avis,
            total: total.0,
            page,
            par_page,
        }),
        error: None,
    }))
}

/// GET /api/retrouve-amis/avis/{id}
/// Détail d'un avis de recherche (uniquement si auteur)
pub async fn detail_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let avis_id = path.into_inner();

    #[derive(sqlx::FromRow)]
    struct AvisDetailRow {
        id: Uuid,
        nom_recherche: String,
        prenom_recherche: Option<String>,
        surnom: Option<String>,
        ecole: Option<String>,
        ville: Option<String>,
        pays_id: Option<Uuid>,
        etat: String,
        periode_debut: Option<i32>,
        periode_fin: Option<i32>,
        description: Option<String>,
        est_anonyme: bool,
        genre_recherche: Option<String>,
        type_relation: Option<String>,
        type_relation_autre: Option<String>,
        comment_connu: Option<String>,
        localite_rencontre: Option<String>,
        ecole_rencontre: Option<String>,
        ville_rencontre: Option<String>,
        jamais_rencontre: bool,
        rencontre_reseaux_sociaux: bool,
        reseaux_sociaux: Option<String>,
        photo_url: Option<String>,
        description_physique: Option<String>,
        partage_coordonnees: bool,
        coordonnees_email: Option<String>,
        coordonnees_telephone: Option<String>,
        coordonnees_whatsapp: Option<String>,
        auteur_id: Uuid,
        created_at: chrono::DateTime<Utc>,
        updated_at: chrono::DateTime<Utc>,
        pays_nom: Option<String>,
    }

    let avis: AvisDetailRow = sqlx::query_as(
        "SELECT a.id, a.nom_recherche, a.prenom_recherche, a.surnom, a.ecole, a.ville,
                a.pays_id, a.etat::text AS etat, a.periode_debut, a.periode_fin, a.description,
                a.est_anonyme, a.genre_recherche::text AS genre_recherche,
                a.type_relation::text AS type_relation, a.type_relation_autre, a.comment_connu,
                a.localite_rencontre, a.ecole_rencontre, a.ville_rencontre,
                a.jamais_rencontre, a.rencontre_reseaux_sociaux, a.reseaux_sociaux,
                a.photo_url, a.description_physique,
                a.partage_coordonnees, a.coordonnees_email, a.coordonnees_telephone, a.coordonnees_whatsapp,
                a.auteur_id, a.created_at, a.updated_at,
                p.nom AS pays_nom
         FROM retrouve_amis.avis_recherche a
         LEFT JOIN shared.pays p ON p.id = a.pays_id
         WHERE a.id = $1 AND a.deleted_at IS NULL"
    )
    .bind(avis_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Avis de recherche introuvable".into()))?;

    if avis.auteur_id != utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Vous n'êtes pas l'auteur de cet avis".into()));
    }

    // Charger les correspondances avec résumé anonymisé
    #[derive(sqlx::FromRow)]
    struct CorrRow {
        id: Uuid,
        score: f64,
        etat: String,
        type_cible: String,
        cible_avis_id: Option<Uuid>,
        cible_utilisateur_id: Option<Uuid>,
        created_at: chrono::DateTime<Utc>,
        // Champs pour construire le résumé anonymisé
        cible_nom: Option<String>,
        cible_prenom: Option<String>,
        cible_ville: Option<String>,
        cible_periode_debut: Option<i32>,
        cible_periode_fin: Option<i32>,
        details_score: serde_json::Value,
    }

    let corrs: Vec<CorrRow> = sqlx::query_as(
        "SELECT c.id, c.score::float8 AS score, c.etat::text AS etat, c.type_cible::text AS type_cible,
                c.cible_avis_id, c.cible_utilisateur_id, c.created_at, c.details_score,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.nom_recherche
                    WHEN c.type_cible = 'profil' THEN u.nom
                END AS cible_nom,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.prenom_recherche
                    WHEN c.type_cible = 'profil' THEN u.prenom
                END AS cible_prenom,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.ville
                    WHEN c.type_cible = 'profil' THEN u.ville
                END AS cible_ville,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.periode_debut
                END AS cible_periode_debut,
                CASE
                    WHEN c.type_cible = 'avis' THEN a2.periode_fin
                END AS cible_periode_fin
         FROM retrouve_amis.correspondance c
         LEFT JOIN retrouve_amis.avis_recherche a2 ON c.cible_avis_id = a2.id
         LEFT JOIN iam.utilisateur u ON c.cible_utilisateur_id = u.id
         WHERE c.avis_id = $1
         ORDER BY c.score DESC"
    )
    .bind(avis_id)
    .fetch_all(pool.get_ref())
    .await?;

    let correspondances: Vec<CorrespondanceResponse> = corrs
        .into_iter()
        .map(|c| {
            let initiales = construire_initiales(c.cible_nom.as_deref(), c.cible_prenom.as_deref());
            let periode = construire_periode(c.cible_periode_debut, c.cible_periode_fin);
            let criteres = construire_criteres_communs(&c.details_score);

            CorrespondanceResponse {
                id: c.id,
                avis_id,
                score: c.score,
                etat: c.etat,
                type_cible: c.type_cible,
                resume_anonymise: ResumeAnonyme {
                    initiales,
                    ville: c.cible_ville.clone(),
                    periode,
                    criteres_communs: criteres,
                },
                mon_role: "auteur".to_string(),
                created_at: c.created_at,
                expire_at: None,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AvisRechercheDetailResponse {
            id: avis.id,
            nom_recherche: avis.nom_recherche,
            prenom_recherche: avis.prenom_recherche,
            surnom: avis.surnom,
            ecole: avis.ecole,
            ville: avis.ville,
            pays: avis.pays_id.map(|id| PaysInfo {
                id,
                nom: avis.pays_nom.unwrap_or_default(),
            }),
            periode_debut: avis.periode_debut,
            periode_fin: avis.periode_fin,
            description: avis.description,
            etat: avis.etat,
            est_anonyme: avis.est_anonyme,
            genre_recherche: avis.genre_recherche,
            type_relation: avis.type_relation,
            type_relation_autre: avis.type_relation_autre,
            comment_connu: avis.comment_connu,
            localite_rencontre: avis.localite_rencontre,
            ecole_rencontre: avis.ecole_rencontre,
            ville_rencontre: avis.ville_rencontre,
            jamais_rencontre: avis.jamais_rencontre,
            rencontre_reseaux_sociaux: avis.rencontre_reseaux_sociaux,
            reseaux_sociaux: avis.reseaux_sociaux,
            photo_url: avis.photo_url,
            description_physique: avis.description_physique,
            partage_coordonnees: avis.partage_coordonnees,
            coordonnees_email: avis.coordonnees_email,
            coordonnees_telephone: avis.coordonnees_telephone,
            coordonnees_whatsapp: avis.coordonnees_whatsapp,
            correspondances,
            created_at: avis.created_at,
            updated_at: avis.updated_at,
        }),
        error: None,
    }))
}

/// PUT /api/retrouve-amis/avis/{id}
/// Modifier un avis de recherche actif et relancer le matching
pub async fn modifier_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    path: web::Path<Uuid>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let avis_id = path.into_inner();

    // Vérifier que l'avis existe et appartient à l'utilisateur
    let avis: AvisRecherche = sqlx::query_as(&format!(
        "SELECT {} FROM retrouve_amis.avis_recherche WHERE id = $1 AND deleted_at IS NULL",
        AVIS_RECHERCHE_COLONNES
    ))
    .bind(avis_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Avis de recherche introuvable".into()))?;

    if avis.auteur_id != utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Vous n'êtes pas l'auteur de cet avis".into()));
    }

    if avis.etat != "actif" {
        return Err(ApiErreur::Validation("Seul un avis actif peut être modifié".into()));
    }

    // ── Parsing multipart ────────────────────────────────────
    let mut nom_recherche: Option<String> = None;
    let mut prenom_recherche: Option<String> = None;
    let mut surnom: Option<String> = None;
    let mut ecole: Option<String> = None;
    let mut ville: Option<String> = None;
    let mut pays_id: Option<Uuid> = None;
    let mut periode_debut: Option<i32> = None;
    let mut periode_fin: Option<i32> = None;
    let mut description: Option<String> = None;
    let mut est_anonyme: bool = avis.est_anonyme;
    let mut genre_recherche: Option<String> = avis.genre_recherche.clone();
    let mut type_relation: Option<String> = avis.type_relation.clone();
    let mut type_relation_autre: Option<String> = avis.type_relation_autre.clone();
    let mut comment_connu: Option<String> = avis.comment_connu.clone();
    let mut localite_rencontre: Option<String> = avis.localite_rencontre.clone();
    let mut ecole_rencontre: Option<String> = avis.ecole_rencontre.clone();
    let mut ville_rencontre: Option<String> = avis.ville_rencontre.clone();
    let mut jamais_rencontre: bool = avis.jamais_rencontre;
    let mut rencontre_reseaux_sociaux: bool = avis.rencontre_reseaux_sociaux;
    let mut reseaux_sociaux: Option<String> = avis.reseaux_sociaux.clone();
    let mut description_physique: Option<String> = avis.description_physique.clone();
    let mut partage_coordonnees: bool = avis.partage_coordonnees;
    let mut coordonnees_email: Option<String> = avis.coordonnees_email.clone();
    let mut coordonnees_telephone: Option<String> = avis.coordonnees_telephone.clone();
    let mut coordonnees_whatsapp: Option<String> = avis.coordonnees_whatsapp.clone();
    let mut nouvelle_photo: Option<String> = None;
    let mut photo_recue = false;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            ApiErreur::Upload(format!("Erreur lecture multipart: {}", e))
        })?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        match nom_champ.as_str() {
            "nom_recherche" => nom_recherche = Some(lire_champ_texte_avis(&mut field).await?),
            "prenom_recherche" => prenom_recherche = lire_champ_option(&mut field).await?,
            "surnom" => surnom = lire_champ_option(&mut field).await?,
            "ecole" => ecole = lire_champ_option(&mut field).await?,
            "ville" => ville = lire_champ_option(&mut field).await?,
            "pays_id" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                if !val.trim().is_empty() {
                    pays_id = Some(val.trim().parse::<Uuid>().map_err(|_| {
                        ApiErreur::Validation("pays_id invalide".into())
                    })?);
                }
            }
            "periode_debut" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                if !val.trim().is_empty() {
                    periode_debut = Some(val.trim().parse::<i32>().map_err(|_| {
                        ApiErreur::Validation("periode_debut invalide".into())
                    })?);
                }
            }
            "periode_fin" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                if !val.trim().is_empty() {
                    periode_fin = Some(val.trim().parse::<i32>().map_err(|_| {
                        ApiErreur::Validation("periode_fin invalide".into())
                    })?);
                }
            }
            "description" => description = lire_champ_option(&mut field).await?,
            "est_anonyme" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                est_anonyme = val == "true" || val == "1";
            }
            "genre_recherche" => genre_recherche = lire_champ_option(&mut field).await?,
            "type_relation" => type_relation = lire_champ_option(&mut field).await?,
            "type_relation_autre" => type_relation_autre = lire_champ_option(&mut field).await?,
            "comment_connu" => comment_connu = lire_champ_option(&mut field).await?,
            "localite_rencontre" => localite_rencontre = lire_champ_option(&mut field).await?,
            "ecole_rencontre" => ecole_rencontre = lire_champ_option(&mut field).await?,
            "ville_rencontre" => ville_rencontre = lire_champ_option(&mut field).await?,
            "jamais_rencontre" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                jamais_rencontre = val == "true" || val == "1";
            }
            "rencontre_reseaux_sociaux" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                rencontre_reseaux_sociaux = val == "true" || val == "1";
            }
            "reseaux_sociaux" => reseaux_sociaux = lire_champ_option(&mut field).await?,
            "description_physique" => description_physique = lire_champ_option(&mut field).await?,
            "partage_coordonnees" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                partage_coordonnees = val == "true" || val == "1";
            }
            "coordonnees_email" => coordonnees_email = lire_champ_option(&mut field).await?,
            "coordonnees_telephone" => coordonnees_telephone = lire_champ_option(&mut field).await?,
            "coordonnees_whatsapp" => coordonnees_whatsapp = lire_champ_option(&mut field).await?,
            "photo" => {
                let content_type = field.content_type().map(|ct| ct.to_string()).unwrap_or_default();
                let types_autorises = ["image/jpeg", "image/png", "image/webp"];
                if !types_autorises.iter().any(|t| content_type.starts_with(t)) {
                    return Err(ApiErreur::Validation(
                        "Format photo invalide. Formats acceptes : JPEG, PNG, WebP".into(),
                    ));
                }

                let nom_original = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));

                let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
                let chemin_complet = format!("{}/retrouve-amis/{}", upload_dir.get_ref(), nom_fichier);

                sauvegarder_photo_avis(&mut field, &chemin_complet, 5 * 1024 * 1024).await?;
                nouvelle_photo = Some(format!("/uploads/retrouve-amis/{}", nom_fichier));
                photo_recue = true;
            }
            _ => {
                log::warn!("Champ multipart inconnu ignore: {}", nom_champ);
            }
        }
    }

    // ── Validation ───────────────────────────────────────────
    let nom_recherche = nom_recherche
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| avis.nom_recherche.clone());

    // Coordonnees valides si partage active
    if partage_coordonnees {
        let a_coordonnee = coordonnees_email.as_ref().map_or(false, |v| !v.trim().is_empty())
            || coordonnees_telephone.as_ref().map_or(false, |v| !v.trim().is_empty())
            || coordonnees_whatsapp.as_ref().map_or(false, |v| !v.trim().is_empty());
        if !a_coordonnee {
            return Err(ApiErreur::Validation(
                "Au moins une coordonnee (email, telephone ou WhatsApp) est requise si le partage est active".into(),
            ));
        }
    }

    // Gestion photo : nouvelle photo remplace l'ancienne, sinon conserver l'existante
    let photo_url_finale = if photo_recue {
        // Supprimer l'ancienne photo si elle existe
        if let Some(ref ancienne) = avis.photo_url {
            let chemin_ancien = format!("{}{}", upload_dir.get_ref(), ancienne.replace("/uploads", ""));
            let _ = std::fs::remove_file(&chemin_ancien);
        }
        nouvelle_photo
    } else {
        avis.photo_url.clone()
    };

    // Mettre à jour l'avis
    sqlx::query(
        "UPDATE retrouve_amis.avis_recherche SET
            nom_recherche = $2, prenom_recherche = $3, surnom = $4,
            ecole = $5, ville = $6, pays_id = $7,
            periode_debut = $8, periode_fin = $9, description = $10,
            est_anonyme = $11,
            genre_recherche = $12::retrouve_amis.genre_personne,
            type_relation = $13::retrouve_amis.type_relation_recherche,
            type_relation_autre = $14,
            comment_connu = $15, localite_rencontre = $16,
            ecole_rencontre = $17, ville_rencontre = $18,
            jamais_rencontre = $19,
            rencontre_reseaux_sociaux = $20, reseaux_sociaux = $21,
            photo_url = $22,
            description_physique = $23,
            partage_coordonnees = $24, coordonnees_email = $25,
            coordonnees_telephone = $26, coordonnees_whatsapp = $27,
            updated_at = NOW()
         WHERE id = $1"
    )
    .bind(avis_id)
    .bind(&nom_recherche)
    .bind(&prenom_recherche)
    .bind(&surnom)
    .bind(&ecole)
    .bind(&ville)
    .bind(pays_id)
    .bind(periode_debut)
    .bind(periode_fin)
    .bind(&description)
    .bind(est_anonyme)
    .bind(&genre_recherche)
    .bind(&type_relation)
    .bind(&type_relation_autre)
    .bind(&comment_connu)
    .bind(&localite_rencontre)
    .bind(&ecole_rencontre)
    .bind(&ville_rencontre)
    .bind(jamais_rencontre)
    .bind(rencontre_reseaux_sociaux)
    .bind(&reseaux_sociaux)
    .bind(&photo_url_finale)
    .bind(&description_physique)
    .bind(partage_coordonnees)
    .bind(&coordonnees_email)
    .bind(&coordonnees_telephone)
    .bind(&coordonnees_whatsapp)
    .execute(pool.get_ref())
    .await?;

    // Supprimer les correspondances en_attente existantes
    sqlx::query(
        "DELETE FROM retrouve_amis.correspondance WHERE avis_id = $1 AND etat = 'en_attente'"
    )
    .bind(avis_id)
    .execute(pool.get_ref())
    .await?;

    // Relancer le matching
    let correspondances: Vec<CorrespondanceResultat> = sqlx::query_as(
        "SELECT cible_type::text AS type_cible, cible_id, score_total::float8 AS score_total, details
         FROM retrouve_amis.calculer_correspondances($1)
         WHERE score_total >= 60"
    )
    .bind(avis_id)
    .fetch_all(pool.get_ref())
    .await?;

    let mut nb_correspondances: i64 = 0;
    for corr in &correspondances {
        let corr_id: (Uuid,) = match corr.type_cible.as_str() {
            "avis" => {
                sqlx::query_as(
                    "INSERT INTO retrouve_amis.correspondance
                     (avis_id, type_cible, cible_avis_id, score, details_score, expire_at)
                     VALUES ($1, 'avis', $2, $3, $4, NOW() + INTERVAL '30 days')
                     RETURNING id"
                )
                .bind(avis_id)
                .bind(corr.cible_id)
                .bind(corr.score_total)
                .bind(&corr.details)
                .fetch_one(pool.get_ref())
                .await?
            }
            "profil" => {
                sqlx::query_as(
                    "INSERT INTO retrouve_amis.correspondance
                     (avis_id, type_cible, cible_utilisateur_id, score, details_score, expire_at)
                     VALUES ($1, 'profil', $2, $3, $4, NOW() + INTERVAL '30 days')
                     RETURNING id"
                )
                .bind(avis_id)
                .bind(corr.cible_id)
                .bind(corr.score_total)
                .bind(&corr.details)
                .fetch_one(pool.get_ref())
                .await?
            }
            _ => continue,
        };

        // Notifications
        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'nouvelle_correspondance')"
        )
        .bind(utilisateur_id)
        .bind(corr_id.0)
        .execute(pool.get_ref())
        .await?;

        let cible_uid = match corr.type_cible.as_str() {
            "avis" => {
                let r: (Uuid,) = sqlx::query_as(
                    "SELECT auteur_id FROM retrouve_amis.avis_recherche WHERE id = $1"
                )
                .bind(corr.cible_id)
                .fetch_one(pool.get_ref())
                .await?;
                r.0
            }
            "profil" => corr.cible_id,
            _ => continue,
        };

        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'nouvelle_correspondance')"
        )
        .bind(cible_uid)
        .bind(corr_id.0)
        .execute(pool.get_ref())
        .await?;

        nb_correspondances += 1;
    }

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "avis_recherche",
        Some(avis_id),
        None,
        Some(serde_json::json!({"correspondances_trouvees": nb_correspondances})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ModifierAvisResponse {
            id: avis_id,
            correspondances_trouvees: nb_correspondances,
        }),
        error: None,
    }))
}

/// PATCH /api/retrouve-amis/avis/{id}/cloturer
/// Clôturer un avis de recherche
pub async fn cloturer_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let avis_id = path.into_inner();

    let result = sqlx::query(
        "UPDATE retrouve_amis.avis_recherche SET etat = 'cloture'
         WHERE id = $1 AND auteur_id = $2 AND etat = 'actif' AND deleted_at IS NULL"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Avis introuvable ou non modifiable".into(),
        ));
    }

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "avis_recherche",
        Some(avis_id),
        None,
        Some(serde_json::json!({"etat": "cloture"})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": avis_id, "etat": "cloture"})),
        error: None,
    }))
}

/// DELETE /api/retrouve-amis/avis/{id}
/// Supprimer un avis de recherche (soft delete)
pub async fn supprimer_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let avis_id = path.into_inner();

    // Verifier que l'avis existe et appartient a l'auteur
    let result = sqlx::query(
        "UPDATE retrouve_amis.avis_recherche SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND auteur_id = $2 AND deleted_at IS NULL"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Avis introuvable ou deja supprime".into(),
        ));
    }

    // Supprimer les correspondances en_attente liees
    sqlx::query(
        "DELETE FROM retrouve_amis.correspondance WHERE avis_id = $1 AND etat = 'en_attente'"
    )
    .bind(avis_id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "DELETE",
        "retrouve_amis",
        "avis_recherche",
        Some(avis_id),
        None,
        Some(serde_json::json!({"soft_delete": true})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": avis_id, "supprime": true})),
        error: None,
    }))
}



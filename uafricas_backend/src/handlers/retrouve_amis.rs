// ════════════════════════════════════════════════════════════════════════════
// Handlers publics — Retrouve Amis
// ════════════════════════════════════════════════════════════════════════════
//
// Endpoints publics pour la fonctionnalité "Retrouve Amis" :
// - CRUD avis de recherche
// - Correspondances (listing, détail, accepter, refuser)
// - Signalements
// - Notifications
// - Tableau de bord
// - Profil trouvable (basculer, parcours CRUD)
// ════════════════════════════════════════════════════════════════════════════

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::retrouve_amis::*;
use crate::services::audit;
use crate::ApiResponse;

// ════════════════════════════════════════════════════════════════════════════
// PAYS — Liste publique
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/retrouve-amis/pays
/// Liste des pays actifs (id, nom) sans authentification
pub async fn lister_pays(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let pays: Vec<PaysInfo> = sqlx::query_as(
        "SELECT id, nom FROM shared.pays WHERE actif = TRUE ORDER BY nom ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(pays),
        error: None,
    }))
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// Extraire l'ID utilisateur depuis le token JWT dans le header Authorization
fn extraire_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiErreur::NonAutorise("Token manquant".into()))?;

    let token = jwt::extraire_token_du_header(header)?;
    let jwt_config = req
        .app_data::<web::Data<crate::config::JwtConfig>>()
        .ok_or_else(|| ApiErreur::BaseDeDonnees("Configuration JWT manquante".into()))?;
    let claims = jwt::valider_token(token, &jwt_config.secret)?;
    claims
        .sub
        .parse::<Uuid>()
        .map_err(|_| ApiErreur::NonAutorise("ID utilisateur invalide".into()))
}

// ════════════════════════════════════════════════════════════════════════════
// AVIS DE RECHERCHE — CRUD
// ════════════════════════════════════════════════════════════════════════════

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
    let mut comment_connu: Option<String> = None;
    let mut localite_rencontre: Option<String> = None;
    let mut ecole_rencontre: Option<String> = None;
    let mut ville_rencontre: Option<String> = None;
    let mut jamais_rencontre: bool = false;
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
            "comment_connu" => comment_connu = lire_champ_option(&mut field).await?,
            "localite_rencontre" => localite_rencontre = lire_champ_option(&mut field).await?,
            "ecole_rencontre" => ecole_rencontre = lire_champ_option(&mut field).await?,
            "ville_rencontre" => ville_rencontre = lire_champ_option(&mut field).await?,
            "jamais_rencontre" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                jamais_rencontre = val == "true" || val == "1";
            }
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
        || jamais_rencontre;
    if !a_critere {
        return Err(ApiErreur::Validation(
            "Au moins un critere supplementaire est requis (type de relation, lieu de rencontre, ecole ou 'jamais rencontre')".into(),
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
          est_anonyme, genre_recherche, type_relation, comment_connu,
          localite_rencontre, ecole_rencontre, ville_rencontre, jamais_rencontre,
          photo_url, description_physique, partage_coordonnees, coordonnees_email, coordonnees_telephone, coordonnees_whatsapp,
          est_public, slug, date_publication_publique)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                 $11, $12::retrouve_amis.genre_personne, $13::retrouve_amis.type_relation_recherche, $14,
                 $15, $16, $17, $18,
                 $19, $20, $21, $22, $23, $24,
                 TRUE, $25, NOW())
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
    .bind(&comment_connu)
    .bind(&localite_rencontre)
    .bind(&ecole_rencontre)
    .bind(&ville_rencontre)
    .bind(jamais_rencontre)
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
                a.type_relation::text AS type_relation,
                a.localite_rencontre, a.ecole_rencontre, a.ville_rencontre,
                a.jamais_rencontre, a.photo_url, a.description_physique,
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
        localite_rencontre: Option<String>,
        ecole_rencontre: Option<String>,
        ville_rencontre: Option<String>,
        jamais_rencontre: bool,
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
            localite_rencontre: r.localite_rencontre,
            ecole_rencontre: r.ecole_rencontre,
            ville_rencontre: r.ville_rencontre,
            jamais_rencontre: r.jamais_rencontre,
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
        comment_connu: Option<String>,
        localite_rencontre: Option<String>,
        ecole_rencontre: Option<String>,
        ville_rencontre: Option<String>,
        jamais_rencontre: bool,
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
                a.type_relation::text AS type_relation, a.comment_connu,
                a.localite_rencontre, a.ecole_rencontre, a.ville_rencontre,
                a.jamais_rencontre, a.photo_url, a.description_physique,
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
            comment_connu: avis.comment_connu,
            localite_rencontre: avis.localite_rencontre,
            ecole_rencontre: avis.ecole_rencontre,
            ville_rencontre: avis.ville_rencontre,
            jamais_rencontre: avis.jamais_rencontre,
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
    let mut comment_connu: Option<String> = avis.comment_connu.clone();
    let mut localite_rencontre: Option<String> = avis.localite_rencontre.clone();
    let mut ecole_rencontre: Option<String> = avis.ecole_rencontre.clone();
    let mut ville_rencontre: Option<String> = avis.ville_rencontre.clone();
    let mut jamais_rencontre: bool = avis.jamais_rencontre;
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
            "comment_connu" => comment_connu = lire_champ_option(&mut field).await?,
            "localite_rencontre" => localite_rencontre = lire_champ_option(&mut field).await?,
            "ecole_rencontre" => ecole_rencontre = lire_champ_option(&mut field).await?,
            "ville_rencontre" => ville_rencontre = lire_champ_option(&mut field).await?,
            "jamais_rencontre" => {
                let val = lire_champ_texte_avis(&mut field).await?;
                jamais_rencontre = val == "true" || val == "1";
            }
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
            comment_connu = $14, localite_rencontre = $15,
            ecole_rencontre = $16, ville_rencontre = $17,
            jamais_rencontre = $18, photo_url = $19,
            description_physique = $20,
            partage_coordonnees = $21, coordonnees_email = $22,
            coordonnees_telephone = $23, coordonnees_whatsapp = $24,
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
    .bind(&comment_connu)
    .bind(&localite_rencontre)
    .bind(&ecole_rencontre)
    .bind(&ville_rencontre)
    .bind(jamais_rencontre)
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

// ════════════════════════════════════════════════════════════════════════════
// CORRESPONDANCES
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/retrouve-amis/correspondances
/// Lister les correspondances de l'utilisateur
pub async fn lister_correspondances(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    let etat = query.get("etat").cloned();
    let avis_id = query.get("avis_id").and_then(|v| v.parse::<Uuid>().ok());
    let page: i64 = query.get("page").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let par_page: i64 = query.get("par_page").and_then(|v| v.parse().ok()).unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * par_page;

    // Lazy archival : archiver les correspondances expirées
    sqlx::query(
        "UPDATE retrouve_amis.correspondance
         SET etat = 'archivee'
         WHERE etat IN ('en_attente', 'acceptee_a', 'acceptee_b')
           AND created_at < NOW() - INTERVAL '30 days'"
    )
    .execute(pool.get_ref())
    .await?;

    #[derive(sqlx::FromRow)]
    struct CorrListeRow {
        id: Uuid,
        avis_id: Uuid,
        score: f64,
        etat: String,
        type_cible: String,
        cible_avis_id: Option<Uuid>,
        cible_utilisateur_id: Option<Uuid>,
        details_score: serde_json::Value,
        expire_at: Option<chrono::DateTime<Utc>>,
        created_at: chrono::DateTime<Utc>,
        // Pour résumé anonymisé
        cible_nom: Option<String>,
        cible_prenom: Option<String>,
        cible_ville: Option<String>,
        cible_periode_debut: Option<i32>,
        cible_periode_fin: Option<i32>,
        // Pour mon_role
        avis_auteur_id: Uuid,
    }

    let base_where = format!(
        "(c.avis_id IN (SELECT id FROM retrouve_amis.avis_recherche WHERE auteur_id = $1 AND deleted_at IS NULL)
          OR c.cible_utilisateur_id = $1
          OR c.cible_avis_id IN (SELECT id FROM retrouve_amis.avis_recherche WHERE auteur_id = $1 AND deleted_at IS NULL))
        {}
        {}",
        if etat.is_some() { " AND c.etat::text = $2" } else { "" },
        if avis_id.is_some() {
            if etat.is_some() { " AND c.avis_id = $3" } else { " AND c.avis_id = $2" }
        } else { "" }
    );

    let count_sql = format!(
        "SELECT COUNT(*) FROM retrouve_amis.correspondance c WHERE {}",
        base_where
    );

    let list_sql = format!(
        "SELECT c.id, c.avis_id, c.score::float8 AS score, c.etat::text AS etat,
                c.type_cible::text AS type_cible, c.cible_avis_id, c.cible_utilisateur_id,
                c.details_score, c.expire_at, c.created_at,
                a.auteur_id AS avis_auteur_id,
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
                CASE WHEN c.type_cible = 'avis' THEN a2.periode_debut END AS cible_periode_debut,
                CASE WHEN c.type_cible = 'avis' THEN a2.periode_fin END AS cible_periode_fin
         FROM retrouve_amis.correspondance c
         JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
         LEFT JOIN retrouve_amis.avis_recherche a2 ON c.cible_avis_id = a2.id
         LEFT JOIN iam.utilisateur u ON c.cible_utilisateur_id = u.id
         WHERE {}
         ORDER BY c.score DESC, c.created_at DESC
         LIMIT {} OFFSET {}",
        base_where, par_page, offset
    );

    // Bind dynamique
    let total: (i64,);
    let rows: Vec<CorrListeRow>;

    match (etat.as_ref(), avis_id) {
        (Some(e), Some(aid)) => {
            total = sqlx::query_as(&count_sql).bind(utilisateur_id).bind(e).bind(aid).fetch_one(pool.get_ref()).await?;
            rows = sqlx::query_as(&list_sql).bind(utilisateur_id).bind(e).bind(aid).fetch_all(pool.get_ref()).await?;
        }
        (Some(e), None) => {
            total = sqlx::query_as(&count_sql).bind(utilisateur_id).bind(e).fetch_one(pool.get_ref()).await?;
            rows = sqlx::query_as(&list_sql).bind(utilisateur_id).bind(e).fetch_all(pool.get_ref()).await?;
        }
        (None, Some(aid)) => {
            total = sqlx::query_as(&count_sql).bind(utilisateur_id).bind(aid).fetch_one(pool.get_ref()).await?;
            rows = sqlx::query_as(&list_sql).bind(utilisateur_id).bind(aid).fetch_all(pool.get_ref()).await?;
        }
        (None, None) => {
            total = sqlx::query_as(&count_sql).bind(utilisateur_id).fetch_one(pool.get_ref()).await?;
            rows = sqlx::query_as(&list_sql).bind(utilisateur_id).fetch_all(pool.get_ref()).await?;
        }
    }

    let correspondances: Vec<CorrespondanceResponse> = rows
        .into_iter()
        .map(|c| {
            let mon_role = if c.avis_auteur_id == utilisateur_id { "auteur" } else { "cible" };
            let initiales = construire_initiales(c.cible_nom.as_deref(), c.cible_prenom.as_deref());
            let periode = construire_periode(c.cible_periode_debut, c.cible_periode_fin);
            let criteres = construire_criteres_communs(&c.details_score);

            CorrespondanceResponse {
                id: c.id,
                avis_id: c.avis_id,
                score: c.score,
                etat: c.etat,
                type_cible: c.type_cible,
                resume_anonymise: ResumeAnonyme {
                    initiales,
                    ville: c.cible_ville,
                    periode,
                    criteres_communs: criteres,
                },
                mon_role: mon_role.to_string(),
                created_at: c.created_at,
                expire_at: c.expire_at,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(CorrespondanceListeResponse {
            correspondances,
            total: total.0,
            page,
            par_page,
        }),
        error: None,
    }))
}

/// GET /api/retrouve-amis/correspondances/{id}
/// Détail d'une correspondance
pub async fn detail_correspondance(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let corr_id = path.into_inner();

    #[derive(sqlx::FromRow)]
    struct CorrDetailRow {
        id: Uuid,
        avis_id: Uuid,
        score: f64,
        etat: String,
        type_cible: String,
        details_score: serde_json::Value,
        coordonnees_a: Option<serde_json::Value>,
        coordonnees_b: Option<serde_json::Value>,
        expire_at: Option<chrono::DateTime<Utc>>,
        created_at: chrono::DateTime<Utc>,
        avis_auteur_id: Uuid,
        cible_utilisateur_id: Option<Uuid>,
        cible_nom: Option<String>,
        cible_prenom: Option<String>,
        cible_ville: Option<String>,
        cible_periode_debut: Option<i32>,
        cible_periode_fin: Option<i32>,
        message_reponse: Option<String>,
        type_reponse_publique: Option<String>,
    }

    let corr: CorrDetailRow = sqlx::query_as(
        "SELECT c.id, c.avis_id, c.score::float8 AS score, c.etat::text AS etat,
                c.type_cible::text AS type_cible, c.details_score,
                c.coordonnees_a, c.coordonnees_b,
                c.expire_at, c.created_at,
                a.auteur_id AS avis_auteur_id, c.cible_utilisateur_id,
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
                CASE WHEN c.type_cible = 'avis' THEN a2.periode_debut END AS cible_periode_debut,
                CASE WHEN c.type_cible = 'avis' THEN a2.periode_fin END AS cible_periode_fin,
                rp.message AS message_reponse,
                rp.type_reponse::text AS type_reponse_publique
         FROM retrouve_amis.correspondance c
         JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
         LEFT JOIN retrouve_amis.avis_recherche a2 ON c.cible_avis_id = a2.id
         LEFT JOIN iam.utilisateur u ON c.cible_utilisateur_id = u.id
         LEFT JOIN retrouve_amis.reponse_publique rp ON rp.correspondance_id = c.id
         WHERE c.id = $1"
    )
    .bind(corr_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Correspondance introuvable".into()))?;

    // Vérifier que l'utilisateur est participant
    let est_auteur = corr.avis_auteur_id == utilisateur_id;
    let est_cible = corr.cible_utilisateur_id == Some(utilisateur_id);
    // Vérifier aussi si la cible est un avis de l'utilisateur
    let est_cible_avis = if !est_auteur && !est_cible {
        // TODO: check if cible_avis_id belongs to user
        false
    } else {
        false
    };

    if !est_auteur && !est_cible && !est_cible_avis {
        return Err(ApiErreur::AccesInterdit("Vous n'êtes pas participant à cette correspondance".into()));
    }

    let mon_role = if est_auteur { "auteur" } else { "cible" };
    let initiales = construire_initiales(corr.cible_nom.as_deref(), corr.cible_prenom.as_deref());
    let periode = construire_periode(corr.cible_periode_debut, corr.cible_periode_fin);
    let criteres = construire_criteres_communs(&corr.details_score);

    // Coordonnées partagées (seulement si état mutuelle)
    let coordonnees_partagees = if corr.etat == "mutuelle" {
        if est_auteur {
            corr.coordonnees_b.clone()
        } else {
            corr.coordonnees_a.clone()
        }
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(CorrespondanceDetailResponse {
            id: corr.id,
            avis_id: corr.avis_id,
            score: corr.score,
            details_score: corr.details_score,
            etat: corr.etat,
            type_cible: corr.type_cible,
            mon_role: mon_role.to_string(),
            resume_anonymise: ResumeAnonyme {
                initiales,
                ville: corr.cible_ville,
                periode,
                criteres_communs: criteres,
            },
            coordonnees_partagees,
            message_reponse: corr.message_reponse,
            type_reponse_publique: corr.type_reponse_publique,
            created_at: corr.created_at,
            expire_at: corr.expire_at,
        }),
        error: None,
    }))
}

/// POST /api/retrouve-amis/correspondances/{id}/accepter
/// Accepter le contact pour une correspondance
pub async fn accepter_correspondance(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AccepterCorrespondance>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let corr_id = path.into_inner();
    let data = body.into_inner();

    #[derive(sqlx::FromRow)]
    struct CorrInfo {
        id: Uuid,
        avis_id: Uuid,
        etat: String,
        cible_utilisateur_id: Option<Uuid>,
        avis_auteur_id: Uuid,
    }

    let corr: CorrInfo = sqlx::query_as(
        "SELECT c.id, c.avis_id, c.etat::text AS etat, c.cible_utilisateur_id, a.auteur_id AS avis_auteur_id
         FROM retrouve_amis.correspondance c
         JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
         WHERE c.id = $1"
    )
    .bind(corr_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Correspondance introuvable".into()))?;

    let est_a = corr.avis_auteur_id == utilisateur_id;
    let est_b = corr.cible_utilisateur_id == Some(utilisateur_id);

    if !est_a && !est_b {
        return Err(ApiErreur::AccesInterdit("Vous n'êtes pas participant".into()));
    }

    // Déterminer la transition d'état
    let coordonnees_json = serde_json::to_value(&data.coordonnees)
        .map_err(|e| ApiErreur::Validation(e.to_string()))?;

    let (nouvel_etat, consentement_mutuel) = if est_a {
        match corr.etat.as_str() {
            "en_attente" => {
                sqlx::query(
                    "UPDATE retrouve_amis.correspondance
                     SET etat = 'acceptee_a', accepte_par_a_at = NOW(), coordonnees_a = $2
                     WHERE id = $1"
                )
                .bind(corr_id)
                .bind(&coordonnees_json)
                .execute(pool.get_ref())
                .await?;
                ("acceptee_a", false)
            }
            "acceptee_b" => {
                sqlx::query(
                    "UPDATE retrouve_amis.correspondance
                     SET etat = 'mutuelle', accepte_par_a_at = NOW(), coordonnees_a = $2
                     WHERE id = $1"
                )
                .bind(corr_id)
                .bind(&coordonnees_json)
                .execute(pool.get_ref())
                .await?;
                ("mutuelle", true)
            }
            _ => return Err(ApiErreur::Validation("Cette correspondance ne peut pas être acceptée dans son état actuel".into())),
        }
    } else {
        match corr.etat.as_str() {
            "en_attente" => {
                sqlx::query(
                    "UPDATE retrouve_amis.correspondance
                     SET etat = 'acceptee_b', accepte_par_b_at = NOW(), coordonnees_b = $2
                     WHERE id = $1"
                )
                .bind(corr_id)
                .bind(&coordonnees_json)
                .execute(pool.get_ref())
                .await?;
                ("acceptee_b", false)
            }
            "acceptee_a" => {
                sqlx::query(
                    "UPDATE retrouve_amis.correspondance
                     SET etat = 'mutuelle', accepte_par_b_at = NOW(), coordonnees_b = $2
                     WHERE id = $1"
                )
                .bind(corr_id)
                .bind(&coordonnees_json)
                .execute(pool.get_ref())
                .await?;
                ("mutuelle", true)
            }
            _ => return Err(ApiErreur::Validation("Cette correspondance ne peut pas être acceptée dans son état actuel".into())),
        }
    };

    // Créer notification
    let notif_type = if consentement_mutuel { "coordonnees_partagees" } else { "acceptation_contact" };
    let autre_utilisateur = if est_a {
        corr.cible_utilisateur_id.unwrap_or(corr.avis_auteur_id)
    } else {
        corr.avis_auteur_id
    };

    sqlx::query(
        "INSERT INTO retrouve_amis.notification_retrouve
         (utilisateur_id, correspondance_id, type) VALUES ($1, $2, $3::retrouve_amis.type_notification)"
    )
    .bind(autre_utilisateur)
    .bind(corr_id)
    .bind(notif_type)
    .execute(pool.get_ref())
    .await?;

    // Si mutuelle, notifier aussi l'accepteur
    if consentement_mutuel {
        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'coordonnees_partagees')"
        )
        .bind(utilisateur_id)
        .bind(corr_id)
        .execute(pool.get_ref())
        .await?;
    }

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "correspondance",
        Some(corr_id),
        None,
        Some(serde_json::json!({"action": "accepter", "nouvel_etat": nouvel_etat})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": corr_id,
            "etat": nouvel_etat,
            "consentement_mutuel": consentement_mutuel
        })),
        error: None,
    }))
}

/// POST /api/retrouve-amis/correspondances/{id}/refuser
/// Refuser le contact. Crée une blacklist automatique.
pub async fn refuser_correspondance(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let corr_id = path.into_inner();

    #[derive(sqlx::FromRow)]
    struct CorrInfo {
        id: Uuid,
        etat: String,
        cible_utilisateur_id: Option<Uuid>,
        avis_auteur_id: Uuid,
    }

    let corr: CorrInfo = sqlx::query_as(
        "SELECT c.id, c.etat::text AS etat, c.cible_utilisateur_id, a.auteur_id AS avis_auteur_id
         FROM retrouve_amis.correspondance c
         JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
         WHERE c.id = $1"
    )
    .bind(corr_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Correspondance introuvable".into()))?;

    let est_a = corr.avis_auteur_id == utilisateur_id;
    let est_b = corr.cible_utilisateur_id == Some(utilisateur_id);

    if !est_a && !est_b {
        return Err(ApiErreur::AccesInterdit("Vous n'êtes pas participant".into()));
    }

    if !["en_attente", "acceptee_a", "acceptee_b"].contains(&corr.etat.as_str()) {
        return Err(ApiErreur::Validation("Cette correspondance ne peut pas être refusée".into()));
    }

    // Mettre à jour l'état
    sqlx::query("UPDATE retrouve_amis.correspondance SET etat = 'declinee' WHERE id = $1")
        .bind(corr_id)
        .execute(pool.get_ref())
        .await?;

    // Insérer dans la blacklist
    let autre_uid = if est_a {
        corr.cible_utilisateur_id.unwrap_or(corr.avis_auteur_id)
    } else {
        corr.avis_auteur_id
    };

    let (uid_a, uid_b) = if utilisateur_id < autre_uid {
        (utilisateur_id, autre_uid)
    } else {
        (autre_uid, utilisateur_id)
    };

    sqlx::query(
        "INSERT INTO retrouve_amis.blacklist (utilisateur_a_id, utilisateur_b_id, correspondance_id)
         VALUES ($1, $2, $3)
         ON CONFLICT DO NOTHING"
    )
    .bind(uid_a)
    .bind(uid_b)
    .bind(corr_id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "correspondance",
        Some(corr_id),
        None,
        Some(serde_json::json!({"action": "refuser", "blacklist": true})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": corr_id, "etat": "declinee"})),
        error: None,
    }))
}

/// POST /api/retrouve-amis/avis/{id}/signaler
/// Signaler un avis de recherche
pub async fn signaler_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<SignalerAvis>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let avis_id = path.into_inner();
    let data = body.into_inner();

    // Vérifier que l'utilisateur a une correspondance avec cet avis
    let a_correspondance: (bool,) = sqlx::query_as(
        "SELECT EXISTS(
            SELECT 1 FROM retrouve_amis.correspondance c
            JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
            WHERE c.avis_id = $1 AND (c.cible_utilisateur_id = $2
                OR c.cible_avis_id IN (SELECT id FROM retrouve_amis.avis_recherche WHERE auteur_id = $2))
        )"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !a_correspondance.0 {
        return Err(ApiErreur::AccesInterdit(
            "Vous devez avoir une correspondance avec cet avis pour le signaler".into(),
        ));
    }

    // Valider le motif
    let motifs_valides = ["contenu_abusif", "usurpation_identite", "harcelement", "autre"];
    if !motifs_valides.contains(&data.motif.as_str()) {
        return Err(ApiErreur::Validation("Motif de signalement invalide".into()));
    }

    let signalement_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.signalement (avis_id, signale_par, motif, description)
         VALUES ($1, $2, $3::retrouve_amis.motif_signalement, $4)
         RETURNING id"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .bind(&data.motif)
    .bind(&data.description)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("idx_signalement_unique") {
            ApiErreur::Conflit("Vous avez déjà signalé cet avis".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "signalement",
        Some(signalement_id.0),
        None,
        Some(serde_json::json!({"avis_id": avis_id, "motif": &data.motif})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": signalement_id.0})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// NOTIFICATIONS
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/retrouve-amis/notifications
pub async fn lister_notifications(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    let lu = query.get("lu").and_then(|v| v.parse::<bool>().ok());
    let page: i64 = query.get("page").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let par_page: i64 = query.get("par_page").and_then(|v| v.parse().ok()).unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * par_page;

    let mut conditions = vec!["utilisateur_id = $1".to_string()];
    if let Some(l) = lu {
        conditions.push(format!("lu = {}", l));
    }
    let where_clause = conditions.join(" AND ");

    let count_sql = format!(
        "SELECT COUNT(*) FROM retrouve_amis.notification_retrouve WHERE {}",
        where_clause
    );
    let non_lues_sql = "SELECT COUNT(*) FROM retrouve_amis.notification_retrouve WHERE utilisateur_id = $1 AND lu = FALSE";

    let list_sql = format!(
        "SELECT id, type::text AS type_notif, correspondance_id, lu, created_at
         FROM retrouve_amis.notification_retrouve
         WHERE {}
         ORDER BY created_at DESC
         LIMIT {} OFFSET {}",
        where_clause, par_page, offset
    );

    let total: (i64,) = sqlx::query_as(&count_sql)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?;

    let non_lues: (i64,) = sqlx::query_as(non_lues_sql)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?;

    let rows: Vec<NotificationRetrouve> = sqlx::query_as(&list_sql)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?;

    let notifications: Vec<NotificationResponse> = rows
        .into_iter()
        .map(|n| NotificationResponse {
            id: n.id,
            type_notif: n.type_notif,
            correspondance_id: n.correspondance_id,
            lu: n.lu,
            created_at: n.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(NotificationListeResponse {
            notifications,
            total: total.0,
            non_lues: non_lues.0,
            page,
            par_page,
        }),
        error: None,
    }))
}

/// PATCH /api/retrouve-amis/notifications/{id}/lire
pub async fn marquer_lu(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let notif_id = path.into_inner();

    let result = sqlx::query(
        "UPDATE retrouve_amis.notification_retrouve SET lu = TRUE
         WHERE id = $1 AND utilisateur_id = $2"
    )
    .bind(notif_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Notification introuvable".into()));
    }

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// PATCH /api/retrouve-amis/notifications/tout-lire
pub async fn tout_marquer_lu(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    let result = sqlx::query(
        "UPDATE retrouve_amis.notification_retrouve SET lu = TRUE
         WHERE utilisateur_id = $1 AND lu = FALSE"
    )
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"mises_a_jour": result.rows_affected()})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// TABLEAU DE BORD
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/retrouve-amis/tableau-de-bord
pub async fn tableau_de_bord(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    #[derive(sqlx::FromRow)]
    struct StatsRow {
        avis_actifs: i64,
        avis_clotures: i64,
        correspondances_en_attente: i64,
        correspondances_mutuelles: i64,
        notifications_non_lues: i64,
        est_trouvable: bool,
        nb_parcours: i64,
    }

    let stats: StatsRow = sqlx::query_as(
        "SELECT
            (SELECT COUNT(*) FROM retrouve_amis.avis_recherche WHERE auteur_id = $1 AND etat = 'actif' AND deleted_at IS NULL) AS avis_actifs,
            (SELECT COUNT(*) FROM retrouve_amis.avis_recherche WHERE auteur_id = $1 AND etat = 'cloture' AND deleted_at IS NULL) AS avis_clotures,
            (SELECT COUNT(*) FROM retrouve_amis.correspondance c
             JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
             WHERE (a.auteur_id = $1 OR c.cible_utilisateur_id = $1) AND c.etat = 'en_attente') AS correspondances_en_attente,
            (SELECT COUNT(*) FROM retrouve_amis.correspondance c
             JOIN retrouve_amis.avis_recherche a ON c.avis_id = a.id
             WHERE (a.auteur_id = $1 OR c.cible_utilisateur_id = $1) AND c.etat = 'mutuelle') AS correspondances_mutuelles,
            (SELECT COUNT(*) FROM retrouve_amis.notification_retrouve WHERE utilisateur_id = $1 AND lu = FALSE) AS notifications_non_lues,
            (SELECT est_trouvable FROM iam.utilisateur WHERE id = $1) AS est_trouvable,
            (SELECT COUNT(*) FROM retrouve_amis.parcours_trouvable WHERE utilisateur_id = $1) AS nb_parcours"
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(TableauDeBord {
            avis_actifs: stats.avis_actifs,
            avis_clotures: stats.avis_clotures,
            correspondances_en_attente: stats.correspondances_en_attente,
            correspondances_mutuelles: stats.correspondances_mutuelles,
            notifications_non_lues: stats.notifications_non_lues,
            est_trouvable: stats.est_trouvable,
            nb_parcours: stats.nb_parcours,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// PROFIL TROUVABLE
// ════════════════════════════════════════════════════════════════════════════

/// PATCH /api/profil/trouvable
pub async fn basculer_trouvable(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<BasculerTrouvable>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let data = body.into_inner();

    sqlx::query("UPDATE iam.utilisateur SET est_trouvable = $2 WHERE id = $1")
        .bind(utilisateur_id)
        .bind(data.est_trouvable)
        .execute(pool.get_ref())
        .await?;

    let mut nb_correspondances: i64 = 0;

    if data.est_trouvable {
        // Matching du profil contre tous les avis actifs
        // On utilise une approche inverse : pour chaque avis actif, vérifier si ce profil correspond
        let avis_actifs: Vec<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM retrouve_amis.avis_recherche WHERE etat = 'actif' AND deleted_at IS NULL AND auteur_id != $1"
        )
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?;

        for (a_id,) in &avis_actifs {
            let resultats: Vec<CorrespondanceResultat> = sqlx::query_as(
                "SELECT cible_type::text AS type_cible, cible_id, score_total::float8 AS score_total, details
                 FROM retrouve_amis.calculer_correspondances($1)
                 WHERE cible_type = 'profil' AND cible_id = $2 AND score_total >= 60"
            )
            .bind(a_id)
            .bind(utilisateur_id)
            .fetch_all(pool.get_ref())
            .await?;

            for corr in &resultats {
                let corr_id: (Uuid,) = sqlx::query_as(
                    "INSERT INTO retrouve_amis.correspondance
                     (avis_id, type_cible, cible_utilisateur_id, score, details_score, expire_at)
                     VALUES ($1, 'profil', $2, $3, $4, NOW() + INTERVAL '30 days')
                     ON CONFLICT DO NOTHING
                     RETURNING id"
                )
                .bind(a_id)
                .bind(utilisateur_id)
                .bind(corr.score_total)
                .bind(&corr.details)
                .fetch_optional(pool.get_ref())
                .await?
                .unwrap_or_default();

                if corr_id.0 != Uuid::nil() {
                    // Notifications
                    let auteur_id: (Uuid,) = sqlx::query_as(
                        "SELECT auteur_id FROM retrouve_amis.avis_recherche WHERE id = $1"
                    )
                    .bind(a_id)
                    .fetch_one(pool.get_ref())
                    .await?;

                    sqlx::query(
                        "INSERT INTO retrouve_amis.notification_retrouve
                         (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'nouvelle_correspondance')"
                    )
                    .bind(auteur_id.0)
                    .bind(corr_id.0)
                    .execute(pool.get_ref())
                    .await?;

                    sqlx::query(
                        "INSERT INTO retrouve_amis.notification_retrouve
                         (utilisateur_id, correspondance_id, type) VALUES ($1, $2, 'nouvelle_correspondance')"
                    )
                    .bind(utilisateur_id)
                    .bind(corr_id.0)
                    .execute(pool.get_ref())
                    .await?;

                    nb_correspondances += 1;
                }
            }
        }
    } else {
        // Désactivation : annuler les correspondances en_attente basées sur ce profil
        sqlx::query(
            "UPDATE retrouve_amis.correspondance SET etat = 'archivee'
             WHERE cible_utilisateur_id = $1 AND etat = 'en_attente'"
        )
        .bind(utilisateur_id)
        .execute(pool.get_ref())
        .await?;
    }

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "iam",
        "utilisateur",
        Some(utilisateur_id),
        None,
        Some(serde_json::json!({"est_trouvable": data.est_trouvable})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(BasculerTrouvableResponse {
            est_trouvable: data.est_trouvable,
            correspondances_trouvees: nb_correspondances,
        }),
        error: None,
    }))
}

/// GET /api/profil/parcours
pub async fn lister_parcours(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    #[derive(sqlx::FromRow)]
    struct ParcoursRow {
        id: Uuid,
        type_entree: String,
        nom: String,
        ville: Option<String>,
        pays_id: Option<Uuid>,
        pays_nom: Option<String>,
        periode_debut: Option<i32>,
        periode_fin: Option<i32>,
    }

    let rows: Vec<ParcoursRow> = sqlx::query_as(
        "SELECT pt.id, pt.type_entree::text AS type_entree, pt.nom, pt.ville,
                pt.pays_id, p.nom AS pays_nom,
                pt.periode_debut, pt.periode_fin
         FROM retrouve_amis.parcours_trouvable pt
         LEFT JOIN shared.pays p ON p.id = pt.pays_id
         WHERE pt.utilisateur_id = $1
         ORDER BY pt.periode_debut DESC NULLS LAST, pt.created_at DESC"
    )
    .bind(utilisateur_id)
    .fetch_all(pool.get_ref())
    .await?;

    let parcours: Vec<ParcoursTrouvableResponse> = rows
        .into_iter()
        .map(|r| ParcoursTrouvableResponse {
            id: r.id,
            type_entree: r.type_entree,
            nom: r.nom,
            ville: r.ville,
            pays: r.pays_id.map(|id| PaysInfo {
                id,
                nom: r.pays_nom.unwrap_or_default(),
            }),
            periode_debut: r.periode_debut,
            periode_fin: r.periode_fin,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(parcours),
        error: None,
    }))
}

/// POST /api/profil/parcours
pub async fn ajouter_parcours(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerParcours>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let data = body.into_inner();

    // Valider type_entree
    if !["ecole", "ville_residence"].contains(&data.type_entree.as_str()) {
        return Err(ApiErreur::Validation("Type d'entrée invalide (ecole ou ville_residence)".into()));
    }

    if data.nom.trim().is_empty() {
        return Err(ApiErreur::Validation("Le nom est obligatoire".into()));
    }

    if let (Some(debut), Some(fin)) = (data.periode_debut, data.periode_fin) {
        if debut > fin {
            return Err(ApiErreur::Validation("La période de début doit être antérieure à la fin".into()));
        }
    }

    let id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.parcours_trouvable
         (utilisateur_id, type_entree, nom, ville, pays_id, periode_debut, periode_fin)
         VALUES ($1, $2::retrouve_amis.type_parcours, $3, $4, $5, $6, $7)
         RETURNING id"
    )
    .bind(utilisateur_id)
    .bind(&data.type_entree)
    .bind(&data.nom)
    .bind(&data.ville)
    .bind(data.pays_id)
    .bind(data.periode_debut)
    .bind(data.periode_fin)
    .fetch_one(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "parcours_trouvable",
        Some(id.0),
        None,
        Some(serde_json::json!({"type_entree": &data.type_entree, "nom": &data.nom})),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": id.0})),
        error: None,
    }))
}

/// PUT /api/profil/parcours/{id}
pub async fn modifier_parcours(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierParcours>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let parcours_id = path.into_inner();
    let data = body.into_inner();

    let result = sqlx::query(
        "UPDATE retrouve_amis.parcours_trouvable SET
            type_entree = $3::retrouve_amis.type_parcours, nom = $4, ville = $5,
            pays_id = $6, periode_debut = $7, periode_fin = $8
         WHERE id = $1 AND utilisateur_id = $2"
    )
    .bind(parcours_id)
    .bind(utilisateur_id)
    .bind(&data.type_entree)
    .bind(&data.nom)
    .bind(&data.ville)
    .bind(data.pays_id)
    .bind(data.periode_debut)
    .bind(data.periode_fin)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Entrée de parcours introuvable".into()));
    }

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "parcours_trouvable",
        Some(parcours_id),
        None,
        None,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// DELETE /api/profil/parcours/{id}
pub async fn supprimer_parcours(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let parcours_id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM retrouve_amis.parcours_trouvable WHERE id = $1 AND utilisateur_id = $2"
    )
    .bind(parcours_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Entrée de parcours introuvable".into()));
    }

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "DELETE",
        "retrouve_amis",
        "parcours_trouvable",
        Some(parcours_id),
        None,
        None,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// HELPERS PRIVÉS
// ════════════════════════════════════════════════════════════════════════════

/// Construire les initiales pour le résumé anonymisé
fn construire_initiales(nom: Option<&str>, prenom: Option<&str>) -> String {
    let n = nom
        .and_then(|s| s.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());
    let p = prenom
        .and_then(|s| s.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "".to_string());
    if p.is_empty() {
        format!("{}.", n)
    } else {
        format!("{}.{}.", p, n)
    }
}

/// Construire la période pour le résumé anonymisé
fn construire_periode(debut: Option<i32>, fin: Option<i32>) -> Option<String> {
    match (debut, fin) {
        (Some(d), Some(f)) => Some(format!("{}-{}", d, f)),
        (Some(d), None) => Some(format!("{}-...", d)),
        (None, Some(f)) => Some(format!("...-{}", f)),
        (None, None) => None,
    }
}

/// Construire la liste des critères communs à partir des détails du score
fn construire_criteres_communs(details: &serde_json::Value) -> Vec<String> {
    let mut criteres = Vec::new();
    if let Some(obj) = details.as_object() {
        for (cle, valeur) in obj {
            if let Some(v) = valeur.as_f64() {
                if v > 0.0 {
                    criteres.push(cle.clone());
                }
            }
        }
    }
    criteres
}

// ════════════════════════════════════════════════════════════════════════════
// HELPERS MULTIPART — Upload photos avis de recherche
// ════════════════════════════════════════════════════════════════════════════

/// Lire le contenu texte d'un champ multipart avis
async fn lire_champ_texte_avis(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut contenu = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture champ: {}", e)))?;
        contenu.extend_from_slice(&data);
    }
    String::from_utf8(contenu)
        .map_err(|e| ApiErreur::Upload(format!("Encodage UTF-8 invalide: {}", e)))
}

/// Lire un champ texte optionnel (retourne None si vide)
async fn lire_champ_option(field: &mut actix_multipart::Field) -> Result<Option<String>, ApiErreur> {
    let val = lire_champ_texte_avis(field).await?;
    if val.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(val))
    }
}

/// Sauvegarder une photo uploadee avec limite de taille
/// Verifie que les premiers octets correspondent a un format image autorise (JPEG, PNG, WebP).
fn valider_magic_bytes(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }
    // JPEG : FF D8 FF
    if data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF {
        return true;
    }
    // PNG : 89 50 4E 47
    if data[0] == 0x89 && data[1] == 0x50 && data[2] == 0x4E && data[3] == 0x47 {
        return true;
    }
    // WebP : RIFF....WEBP
    if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
        return true;
    }
    false
}

async fn sauvegarder_photo_avis(
    field: &mut actix_multipart::Field,
    chemin: &str,
    taille_max: usize,
) -> Result<(), ApiErreur> {
    if let Some(parent) = std::path::Path::new(chemin).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le repertoire: {}", e)))?;
    }

    let mut fichier = std::fs::File::create(chemin)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le fichier: {}", e)))?;

    let mut taille_totale: usize = 0;
    let mut premier_chunk = true;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
        taille_totale += data.len();
        if taille_totale > taille_max {
            let _ = std::fs::remove_file(chemin);
            return Err(ApiErreur::Validation(
                "La photo depasse la taille maximale de 5 Mo".into(),
            ));
        }
        // Valider les magic bytes sur le premier chunk
        if premier_chunk {
            if !valider_magic_bytes(&data) {
                let _ = std::fs::remove_file(chemin);
                return Err(ApiErreur::Validation(
                    "Le contenu du fichier ne correspond pas a un format image valide (JPEG, PNG, WebP)".into(),
                ));
            }
            premier_chunk = false;
        }
        fichier
            .write_all(&data)
            .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture fichier: {}", e)))?;
    }

    Ok(())
}

// ════════════════════════════════════════════════════════════════════════════
// PUBLICATION PUBLIQUE — Partage des avis de recherche
// ════════════════════════════════════════════════════════════════════════════

/// Generer un slug URL-safe a partir du nom et prenom recherches
fn generer_slug_avis(nom: &str, prenom: Option<&str>) -> String {
    let base = if let Some(p) = prenom {
        format!("{}-{}", nom, p)
    } else {
        nom.to_string()
    };

    // Normaliser : minuscules, remplacer espaces et caracteres speciaux par des tirets
    let slug: String = base
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else if c == ' ' || c == '_' {
                '-'
            } else {
                // Caracteres accentues courants
                match c {
                    'é' | 'è' | 'ê' | 'ë' => 'e',
                    'à' | 'â' | 'ä' => 'a',
                    'ù' | 'û' | 'ü' => 'u',
                    'î' | 'ï' => 'i',
                    'ô' | 'ö' => 'o',
                    'ç' => 'c',
                    'ñ' => 'n',
                    _ => '-',
                }
            }
        })
        .collect();

    // Supprimer les tirets multiples et les tirets en debut/fin
    let slug = slug
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    // Ajouter un suffixe UUID8 pour garantir l'unicite
    let uuid_suffix = &Uuid::new_v4().to_string()[..8];
    format!("{}-{}", slug, uuid_suffix)
}

/// PATCH /api/retrouve-amis/avis/{id}/publier
/// Activer ou desactiver la visibilite publique d'un avis
pub async fn publier_avis(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<PublierAvisRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let avis_id = path.into_inner();
    let est_public = body.est_public;

    // Verifier que l'avis existe, appartient a l'auteur et est actif
    let avis: Option<(Uuid, String, String, Option<String>, Option<String>, bool, Option<String>, Option<chrono::DateTime<Utc>>)> = sqlx::query_as(
        "SELECT id, etat::text, nom_recherche, prenom_recherche, slug, est_public, slug, date_publication_publique
         FROM retrouve_amis.avis_recherche
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(avis_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let avis = avis.ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve".into()))?;

    // Verifier l'auteur
    let auteur_id: Option<(Uuid,)> = sqlx::query_as(
        "SELECT auteur_id FROM retrouve_amis.avis_recherche WHERE id = $1"
    )
    .bind(avis_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let auteur_id = auteur_id.ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve".into()))?;
    if auteur_id.0 != utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Vous n'etes pas l'auteur de cet avis".into()));
    }

    // Verifier que l'etat est actif
    if avis.1 != "actif" {
        return Err(ApiErreur::Validation(
            "Seuls les avis actifs peuvent etre rendus publics".into(),
        ));
    }

    // Generer le slug si premiere publication
    let slug_actuel = avis.4.clone();
    let slug = if slug_actuel.is_some() {
        slug_actuel.unwrap()
    } else {
        generer_slug_avis(&avis.2, avis.3.as_deref())
    };

    // Mettre a jour l'avis
    let now = Utc::now();
    if avis.7.is_some() {
        // Slug et date deja definis, juste toggle est_public
        sqlx::query(
            "UPDATE retrouve_amis.avis_recherche
             SET est_public = $1, slug = $2, updated_at = NOW()
             WHERE id = $3"
        )
        .bind(est_public)
        .bind(&slug)
        .bind(avis_id)
        .execute(pool.get_ref())
        .await?;
    } else {
        // Premiere publication : set slug + date_publication_publique
        sqlx::query(
            "UPDATE retrouve_amis.avis_recherche
             SET est_public = $1, slug = $2, date_publication_publique = $3, updated_at = NOW()
             WHERE id = $4"
        )
        .bind(est_public)
        .bind(&slug)
        .bind(now)
        .bind(avis_id)
        .execute(pool.get_ref())
        .await?;
    }

    // Recuperer la date de publication (existante ou nouvelle)
    let date_pub = if avis.7.is_some() { avis.7 } else if est_public { Some(now) } else { None };

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "UPDATE",
        "retrouve_amis",
        "avis_recherche",
        Some(avis_id),
        Some(serde_json::json!({ "est_public": avis.5 })),
        Some(serde_json::json!({ "est_public": est_public, "slug": &slug })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PublierAvisResponse {
            id: avis_id,
            est_public,
            slug,
            date_publication_publique: date_pub,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// SIGNALEMENT DEPUIS LA PAGE PUBLIQUE
// ════════════════════════════════════════════════════════════════════════════

/// POST /api/retrouve-amis/public/{slug}/signaler
/// Signaler un avis depuis la page publique (connexion requise)
/// Auto-suspension si >= 3 signalements distincts
pub async fn signaler_avis_public(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    body: web::Json<SignalerPublicRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let slug = path.into_inner();
    let data = body.into_inner();

    // Verifier que l'avis existe, est public et actif
    let avis: Option<(Uuid, Uuid, String, bool)> = sqlx::query_as(
        "SELECT id, auteur_id, etat::text, est_public
         FROM retrouve_amis.avis_recherche
         WHERE slug = $1 AND deleted_at IS NULL"
    )
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?;

    let (avis_id, auteur_id, etat, est_public) = avis
        .ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve ou non disponible".into()))?;

    if !est_public || etat != "actif" {
        return Err(ApiErreur::NonTrouve("Avis non disponible".into()));
    }

    // Le signaleur ne peut pas etre l'auteur
    if auteur_id == utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Vous ne pouvez pas signaler votre propre avis".into()));
    }

    // Valider le motif
    let motifs_valides = ["contenu_abusif", "usurpation_identite", "harcelement", "autre"];
    if !motifs_valides.contains(&data.motif.as_str()) {
        return Err(ApiErreur::Validation("Motif de signalement invalide".into()));
    }

    // Inserer le signalement avec source = 'page_publique'
    let signalement_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.signalement (avis_id, signale_par, motif, description, source)
         VALUES ($1, $2, $3::retrouve_amis.motif_signalement, $4, 'page_publique'::retrouve_amis.source_signalement)
         RETURNING id"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .bind(&data.motif)
    .bind(&data.description)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("idx_signalement_unique") {
            ApiErreur::Conflit("Vous avez deja signale cet avis".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    // Compter les signalements distincts et auto-suspendre si >= 3
    let nb_signalements: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT signale_par) FROM retrouve_amis.signalement WHERE avis_id = $1"
    )
    .bind(avis_id)
    .fetch_one(pool.get_ref())
    .await?;

    if nb_signalements.0 >= 3 {
        sqlx::query(
            "UPDATE retrouve_amis.avis_recherche SET etat = 'suspendu'::retrouve_amis.etat_avis, updated_at = NOW()
             WHERE id = $1 AND etat = 'actif'"
        )
        .bind(avis_id)
        .execute(pool.get_ref())
        .await?;

        // Notifier l'auteur de la suspension
        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, type) VALUES ($1, 'avis_suspendu'::retrouve_amis.type_notification)"
        )
        .bind(auteur_id)
        .execute(pool.get_ref())
        .await?;
    }

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "signalement",
        Some(signalement_id.0),
        None,
        Some(serde_json::json!({
            "avis_id": avis_id,
            "motif": &data.motif,
            "source": "page_publique",
            "auto_suspension": nb_signalements.0 >= 3
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"id": signalement_id.0})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// DEMANDE DE RETRAIT
// ════════════════════════════════════════════════════════════════════════════

/// POST /api/retrouve-amis/public/{slug}/demande-retrait
/// Demander le retrait d'un avis (suspension immediate)
pub async fn demander_retrait(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    body: web::Json<DemandeRetraitRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let slug = path.into_inner();
    let data = body.into_inner();

    // Verifier que l'avis existe et est public
    let avis: Option<(Uuid, Uuid, String, bool)> = sqlx::query_as(
        "SELECT id, auteur_id, etat::text, est_public
         FROM retrouve_amis.avis_recherche
         WHERE slug = $1 AND deleted_at IS NULL"
    )
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?;

    let (avis_id, auteur_id, _etat, est_public) = avis
        .ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve ou non disponible".into()))?;

    if !est_public {
        return Err(ApiErreur::NonTrouve("Avis non disponible".into()));
    }

    // Le demandeur ne peut pas etre l'auteur
    if auteur_id == utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Vous ne pouvez pas demander le retrait de votre propre avis. Vous pouvez le depublier directement.".into(),
        ));
    }

    // Valider le motif
    if data.motif.trim().is_empty() {
        return Err(ApiErreur::Validation("Le motif est obligatoire".into()));
    }

    // Inserer la demande de retrait
    let demande_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.demande_retrait (avis_id, demandeur_id, motif)
         VALUES ($1, $2, $3)
         RETURNING id"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .bind(&data.motif)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("idx_demande_retrait_unique") || e.to_string().contains("demande_retrait_avis_id_demandeur_id_key") {
            ApiErreur::Conflit("Vous avez deja soumis une demande de retrait pour cet avis".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    // Suspension immediate de l'avis
    sqlx::query(
        "UPDATE retrouve_amis.avis_recherche SET etat = 'suspendu'::retrouve_amis.etat_avis, updated_at = NOW()
         WHERE id = $1"
    )
    .bind(avis_id)
    .execute(pool.get_ref())
    .await?;

    // Notifier l'auteur
    sqlx::query(
        "INSERT INTO retrouve_amis.notification_retrouve
         (utilisateur_id, type) VALUES ($1, 'demande_retrait'::retrouve_amis.type_notification)"
    )
    .bind(auteur_id)
    .execute(pool.get_ref())
    .await?;

    // Notifier les administrateurs
    let admins: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT DISTINCT u.id
         FROM iam.utilisateur u
         JOIN iam.utilisateur_role ur ON ur.utilisateur_id = u.id
         JOIN iam.role r ON r.id = ur.role_id
         WHERE r.nom = 'admin' AND u.deleted_at IS NULL"
    )
    .fetch_all(pool.get_ref())
    .await?;

    for (admin_id,) in &admins {
        sqlx::query(
            "INSERT INTO retrouve_amis.notification_retrouve
             (utilisateur_id, type) VALUES ($1, 'demande_retrait'::retrouve_amis.type_notification)"
        )
        .bind(admin_id)
        .execute(pool.get_ref())
        .await?;
    }

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "demande_retrait",
        Some(demande_id.0),
        None,
        Some(serde_json::json!({
            "avis_id": avis_id,
            "suspension_immediate": true
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(DemandeRetraitResponse {
            id: demande_id.0,
            message: "L'avis a ete immediatement suspendu. Un administrateur examinera votre demande sous 72h.".to_string(),
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// REPONSE PUBLIQUE
// ════════════════════════════════════════════════════════════════════════════

/// POST /api/retrouve-amis/public/{slug}/repondre
/// Repondre a un avis public (cree une correspondance automatiquement)
pub async fn repondre_avis_public(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    body: web::Json<ReponsePubliqueRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let slug = path.into_inner();
    let data = body.into_inner();

    // Valider le type de reponse
    let types_valides = ["je_suis_cette_personne", "je_la_connais", "jai_des_informations"];
    if !types_valides.contains(&data.type_reponse.as_str()) {
        return Err(ApiErreur::Validation("Type de reponse invalide".into()));
    }

    // Valider le message
    if data.message.trim().is_empty() {
        return Err(ApiErreur::Validation("Le message est obligatoire".into()));
    }

    // Verifier que l'avis existe, est public et actif
    let avis: Option<(Uuid, Uuid, String, bool)> = sqlx::query_as(
        "SELECT id, auteur_id, etat::text, est_public
         FROM retrouve_amis.avis_recherche
         WHERE slug = $1 AND deleted_at IS NULL"
    )
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?;

    let (avis_id, auteur_id, etat, est_public) = avis
        .ok_or_else(|| ApiErreur::NonTrouve("Avis non trouve ou non disponible".into()))?;

    if !est_public || etat != "actif" {
        return Err(ApiErreur::NonTrouve("Avis non disponible".into()));
    }

    // Le repondeur ne peut pas etre l'auteur
    if auteur_id == utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Vous ne pouvez pas repondre a votre propre avis".into()));
    }

    // Verifier la blacklist (ordre canonique)
    let dans_blacklist: Option<(bool,)> = sqlx::query_as(
        "SELECT TRUE FROM retrouve_amis.blacklist
         WHERE utilisateur_a_id = $1 AND utilisateur_b_id = $2"
    )
    .bind(std::cmp::min(auteur_id, utilisateur_id))
    .bind(std::cmp::max(auteur_id, utilisateur_id))
    .fetch_optional(pool.get_ref())
    .await?;

    if dans_blacklist.is_some() {
        return Err(ApiErreur::AccesInterdit("Vous ne pouvez pas repondre a cet avis".into()));
    }

    // Rate limit : max 10 reponses par jour
    let nb_reponses_jour: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM retrouve_amis.reponse_publique
         WHERE repondeur_id = $1 AND created_at > NOW() - INTERVAL '1 day'"
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    if nb_reponses_jour.0 >= 10 {
        return Err(ApiErreur::LimiteAtteinte("Limite de 10 reponses par jour atteinte. Reessayez demain.".into()));
    }

    // Creer la correspondance automatiquement (type_cible = 'profil', score = 70)
    let details_score = serde_json::json!({
        "source": "reponse_publique",
        "type_reponse": &data.type_reponse
    });

    let correspondance_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.correspondance
         (avis_id, type_cible, cible_utilisateur_id, score, details_score, expire_at)
         VALUES ($1, 'profil', $2, 70, $3, NOW() + INTERVAL '30 days')
         RETURNING id"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .bind(&details_score)
    .fetch_one(pool.get_ref())
    .await?;

    // Inserer la reponse publique
    let reponse_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO retrouve_amis.reponse_publique
         (avis_id, repondeur_id, type_reponse, message, correspondance_id)
         VALUES ($1, $2, $3::retrouve_amis.type_reponse_publique, $4, $5)
         RETURNING id"
    )
    .bind(avis_id)
    .bind(utilisateur_id)
    .bind(&data.type_reponse)
    .bind(&data.message)
    .bind(correspondance_id.0)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("uq_reponse_publique_avis_repondeur") {
            ApiErreur::Conflit("Vous avez deja repondu a cet avis".into())
        } else {
            ApiErreur::BaseDeDonnees(e.to_string())
        }
    })?;

    // Notifier l'auteur de l'avis
    sqlx::query(
        "INSERT INTO retrouve_amis.notification_retrouve
         (utilisateur_id, correspondance_id, type)
         VALUES ($1, $2, 'reponse_publique'::retrouve_amis.type_notification)"
    )
    .bind(auteur_id)
    .bind(correspondance_id.0)
    .execute(pool.get_ref())
    .await?;

    // Audit
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "INSERT",
        "retrouve_amis",
        "reponse_publique",
        Some(reponse_id.0),
        None,
        Some(serde_json::json!({
            "avis_id": avis_id,
            "type_reponse": &data.type_reponse,
            "correspondance_id": correspondance_id.0
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(ReponsePubliqueResponse {
            id: reponse_id.0,
            correspondance_id: correspondance_id.0,
            message: "Votre reponse a ete envoyee. L'auteur de l'avis sera notifie.".to_string(),
        }),
        error: None,
    }))
}

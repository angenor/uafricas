use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::gouvernance::{
    ContributionListeResponse, ContributionQueryParams, ContributionRow, FactcheckReactionEtat,
    GouvernanceStats, ReactionFactcheckRequest, SignalementEtat, SignalementRequest,
};
use crate::services::audit;

/// Seuil de signalements distincts au-delà duquel un factcheck est suspendu
const SEUIL_SIGNALEMENTS_SUSPENSION: i64 = 20;

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Extraire l'utilisateur connecte depuis le header Authorization (obligatoire)
fn exiger_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;
    let token = header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApiErreur::NonAutorise("Token invalide".into()))?;
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| ApiErreur::BaseDeDonnees("Configuration JWT absente".into()))?;
    let claims = jwt::valider_token(token, &secret)
        .map_err(|_| ApiErreur::NonAutorise("Token invalide ou expire".into()))?;
    Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiErreur::NonAutorise("Token invalide".into()))
}

/// Extraire l'utilisateur connecte si present (optionnel, ne renvoie jamais d'erreur)
fn extraire_utilisateur_id_opt(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Generer un slug URL-friendly a partir d'un titre
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

// ──────────────────────────────────────────────────────────────
// GET /api/gouvernance/stats — Statistiques de gouvernance
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_stats(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let (factcheck,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM governance.factcheck
         WHERE deleted_at IS NULL AND etat = 'publie'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let (badhabits,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM governance.bad_habit
         WHERE deleted_at IS NULL AND etat = 'publie'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let (ideaforces,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM governance.idea_force
         WHERE deleted_at IS NULL AND etat = 'publie'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let (total_likes,): (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(nombre_likes), 0)::BIGINT
         FROM governance.factcheck
         WHERE deleted_at IS NULL AND etat = 'publie'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let stats = GouvernanceStats {
        total: factcheck + badhabits + ideaforces,
        factcheck,
        badhabits,
        ideaforces,
        total_likes,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(stats),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/gouvernance/contributions — Liste paginee des contributions
// ──────────────────────────────────────────────────────────────
pub async fn lister_contributions(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<ContributionQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Filtre optionnel par type
    let filtre_type = params.type_contribution.as_deref();

    // Utilisateur courant (optionnel) pour resoudre son etat de reaction par cible
    let utilisateur_id = extraire_utilisateur_id_opt(&req).unwrap_or(Uuid::nil());

    let query = build_contributions_query(filtre_type);

    let rows = sqlx::query_as::<_, ContributionRow>(&query)
        .bind(par_page)
        .bind(offset)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?;

    let total = rows.first().map(|r| r.total_count).unwrap_or(0);
    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    let contributions = rows.iter().map(|r| r.to_response()).collect();

    let reponse = ContributionListeResponse {
        contributions,
        total,
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

/// Construit la requete UNION ALL pour les 3 types de contributions
fn build_contributions_query(filtre_type: Option<&str>) -> String {
    let mut sous_requetes = Vec::new();

    let inclure_factcheck = filtre_type.is_none() || filtre_type == Some("factcheck");
    let inclure_badhabits = filtre_type.is_none() || filtre_type == Some("badhabits");
    let inclure_ideaforces = filtre_type.is_none() || filtre_type == Some("ideaforces");

    if inclure_factcheck {
        sous_requetes.push(
            "SELECT f.id, 'factcheck'::TEXT AS type_contribution,
                    LEFT(f.contenu, 200) AS titre,
                    f.contenu AS description,
                    f.nombre_likes, 0 AS nombre_soutiens,
                    f.etat AS statut,
                    f.cree_par, f.pays_id, f.created_at,
                    NULL::VARCHAR AS region, NULL::VARCHAR AS ville,
                    NULL::TEXT AS categorie, NULL::TEXT AS gravite,
                    f.prejuge_titre, f.prejuge_description, f.prejuge_nombre_likes AS prejuge_likes,
                    f.realite_titre, f.realite_description, f.realite_nombre_likes AS realite_likes,
                    f.nombre_coeur, f.nombre_pouce, f.nombre_rire, f.nombre_jaime_pas
             FROM governance.factcheck f
             WHERE f.deleted_at IS NULL AND f.etat = 'publie'"
                .to_string(),
        );
    }

    if inclure_badhabits {
        sous_requetes.push(
            "SELECT b.id, 'badhabits'::TEXT AS type_contribution,
                    b.titre,
                    b.description_generale AS description,
                    0 AS nombre_likes, b.nombre_soutiens,
                    b.etat AS statut,
                    b.cree_par, b.pays_id, b.created_at,
                    b.region, b.ville_quartier_zone AS ville,
                    b.categorie_probleme::TEXT AS categorie,
                    b.gravite::TEXT AS gravite,
                    NULL::VARCHAR AS prejuge_titre, NULL::TEXT AS prejuge_description, 0 AS prejuge_likes,
                    NULL::VARCHAR AS realite_titre, NULL::TEXT AS realite_description, 0 AS realite_likes,
                    0 AS nombre_coeur, 0 AS nombre_pouce, 0 AS nombre_rire, 0 AS nombre_jaime_pas
             FROM governance.bad_habit b
             WHERE b.deleted_at IS NULL AND b.etat = 'publie'"
                .to_string(),
        );
    }

    if inclure_ideaforces {
        sous_requetes.push(
            "SELECT i.id, 'ideaforces'::TEXT AS type_contribution,
                    i.titre,
                    i.description_generale AS description,
                    0 AS nombre_likes, i.nombre_soutiens,
                    i.etat AS statut,
                    i.cree_par, i.pays_id, i.created_at,
                    i.region, i.ville_quartier_zone AS ville,
                    i.categorie_proposition::TEXT AS categorie,
                    i.urgence::TEXT AS gravite,
                    NULL::VARCHAR AS prejuge_titre, NULL::TEXT AS prejuge_description, 0 AS prejuge_likes,
                    NULL::VARCHAR AS realite_titre, NULL::TEXT AS realite_description, 0 AS realite_likes,
                    0 AS nombre_coeur, 0 AS nombre_pouce, 0 AS nombre_rire, 0 AS nombre_jaime_pas
             FROM governance.idea_force i
             WHERE i.deleted_at IS NULL AND i.etat = 'publie'"
                .to_string(),
        );
    }

    // Si aucun type valide, retourner une requete vide
    if sous_requetes.is_empty() {
        return "SELECT NULL::UUID AS id, ''::TEXT AS type_contribution,
                       ''::TEXT AS titre, ''::TEXT AS description,
                       0 AS nombre_likes, 0 AS nombre_soutiens,
                       ''::TEXT AS statut, NOW() AS created_at,
                       NULL::UUID AS auteur_id,
                       NULL::VARCHAR AS auteur_prenom,
                       NULL::VARCHAR AS auteur_nom,
                       NULL::VARCHAR AS auteur_photo_url,
                       NULL::VARCHAR AS pays_nom,
                       NULL::VARCHAR AS region,
                       NULL::VARCHAR AS ville,
                       NULL::TEXT AS categorie,
                       NULL::TEXT AS gravite,
                       NULL::VARCHAR AS prejuge_titre, NULL::TEXT AS prejuge_description, 0 AS prejuge_likes,
                       NULL::VARCHAR AS realite_titre, NULL::TEXT AS realite_description, 0 AS realite_likes,
                       0 AS nombre_coeur, 0 AS nombre_pouce, 0 AS nombre_rire, 0 AS nombre_jaime_pas,
                       NULL::VARCHAR AS ma_reaction_general,
                       FALSE AS a_like_prejuge, FALSE AS a_like_realite, FALSE AS a_signale,
                       0::BIGINT AS total_count
                WHERE FALSE"
            .to_string();
    }

    let union = sous_requetes.join(" UNION ALL ");

    format!(
        "SELECT c.id, c.type_contribution, c.titre, c.description,
                c.nombre_likes, c.nombre_soutiens, c.statut, c.created_at,
                u.id AS auteur_id,
                u.prenom AS auteur_prenom,
                u.nom AS auteur_nom,
                u.photo_url AS auteur_photo_url,
                p.nom AS pays_nom,
                c.region, c.ville,
                c.categorie, c.gravite,
                c.prejuge_titre, c.prejuge_description, c.prejuge_likes,
                c.realite_titre, c.realite_description, c.realite_likes,
                c.nombre_coeur, c.nombre_pouce, c.nombre_rire, c.nombre_jaime_pas,
                (SELECT r.type_reaction FROM governance.factcheck_reaction r
                  WHERE r.factcheck_id = c.id AND r.utilisateur_id = $3 AND r.cible = 'general') AS ma_reaction_general,
                EXISTS(SELECT 1 FROM governance.factcheck_reaction r
                  WHERE r.factcheck_id = c.id AND r.utilisateur_id = $3 AND r.cible = 'prejuge') AS a_like_prejuge,
                EXISTS(SELECT 1 FROM governance.factcheck_reaction r
                  WHERE r.factcheck_id = c.id AND r.utilisateur_id = $3 AND r.cible = 'realite') AS a_like_realite,
                EXISTS(SELECT 1 FROM governance.factcheck_signalement s
                  WHERE s.factcheck_id = c.id AND s.utilisateur_id = $3) AS a_signale,
                COUNT(*) OVER() AS total_count
         FROM ({}) c
         LEFT JOIN iam.utilisateur u ON c.cree_par = u.id
         LEFT JOIN shared.pays p ON c.pays_id = p.id
         ORDER BY c.created_at DESC
         LIMIT $1 OFFSET $2",
        union
    )
}

// ──────────────────────────────────────────────────────────────
// POST /api/gouvernance/factcheck/{id}/reaction — Toggle d'une reaction
// ──────────────────────────────────────────────────────────────

/// Colonne compteur de la table factcheck pour un type de reaction globale
fn colonne_compteur_general(type_reaction: &str) -> Option<&'static str> {
    match type_reaction {
        "coeur" => Some("nombre_coeur"),
        "pouce" => Some("nombre_pouce"),
        "rire" => Some("nombre_rire"),
        "jaime_pas" => Some("nombre_jaime_pas"),
        _ => None,
    }
}

pub async fn reagir_factcheck(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ReactionFactcheckRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let factcheck_id = chemin.into_inner();

    // Valider la cible et le type de reaction associe
    let cible = body.cible.as_str();
    let type_reaction = match cible {
        "general" => {
            let t = body
                .type_reaction
                .as_deref()
                .ok_or_else(|| ApiErreur::Validation("type_reaction requis pour la cible 'general'".into()))?;
            if colonne_compteur_general(t).is_none() {
                return Err(ApiErreur::Validation(
                    "type_reaction doit etre 'coeur', 'pouce', 'rire' ou 'jaime_pas'".into(),
                ));
            }
            t.to_string()
        }
        "prejuge" | "realite" => "coeur".to_string(),
        _ => {
            return Err(ApiErreur::Validation(
                "cible doit etre 'general', 'prejuge' ou 'realite'".into(),
            ));
        }
    };

    // Verifier que le factcheck existe et est publie
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM governance.factcheck
         WHERE id = $1 AND etat = 'publie' AND deleted_at IS NULL)",
    )
    .bind(factcheck_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Factcheck non trouve".into()));
    }

    // Colonne compteur de la cible (prejuge/realite : compteur dedie)
    let colonne_cible = match cible {
        "prejuge" => "prejuge_nombre_likes",
        "realite" => "realite_nombre_likes",
        _ => colonne_compteur_general(&type_reaction).unwrap(),
    };

    // Reaction existante pour (factcheck, utilisateur, cible)
    let reaction_existante: Option<String> = sqlx::query_scalar(
        "SELECT type_reaction FROM governance.factcheck_reaction
         WHERE factcheck_id = $1 AND utilisateur_id = $2 AND cible = $3",
    )
    .bind(factcheck_id)
    .bind(utilisateur_id)
    .bind(cible)
    .fetch_optional(pool.get_ref())
    .await?;

    match reaction_existante {
        Some(ref ancien) if ancien == &type_reaction => {
            // Meme reaction = toggle off
            sqlx::query(
                "DELETE FROM governance.factcheck_reaction
                 WHERE factcheck_id = $1 AND utilisateur_id = $2 AND cible = $3",
            )
            .bind(factcheck_id)
            .bind(utilisateur_id)
            .bind(cible)
            .execute(pool.get_ref())
            .await?;

            sqlx::query(&format!(
                "UPDATE governance.factcheck SET {0} = GREATEST({0} - 1, 0) WHERE id = $1",
                colonne_cible
            ))
            .bind(factcheck_id)
            .execute(pool.get_ref())
            .await?;
        }
        Some(ref ancien) => {
            // Reaction differente (cible 'general' uniquement) = changer d'emoji
            let ancienne_colonne = colonne_compteur_general(ancien).unwrap_or(colonne_cible);
            sqlx::query(
                "UPDATE governance.factcheck_reaction
                 SET type_reaction = $1, created_at = NOW()
                 WHERE factcheck_id = $2 AND utilisateur_id = $3 AND cible = $4",
            )
            .bind(&type_reaction)
            .bind(factcheck_id)
            .bind(utilisateur_id)
            .bind(cible)
            .execute(pool.get_ref())
            .await?;

            sqlx::query(&format!(
                "UPDATE governance.factcheck
                 SET {0} = {0} + 1, {1} = GREATEST({1} - 1, 0) WHERE id = $1",
                colonne_cible, ancienne_colonne
            ))
            .bind(factcheck_id)
            .execute(pool.get_ref())
            .await?;
        }
        None => {
            // Nouvelle reaction
            sqlx::query(
                "INSERT INTO governance.factcheck_reaction
                 (factcheck_id, utilisateur_id, cible, type_reaction)
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(factcheck_id)
            .bind(utilisateur_id)
            .bind(cible)
            .bind(&type_reaction)
            .execute(pool.get_ref())
            .await?;

            sqlx::query(&format!(
                "UPDATE governance.factcheck SET {0} = {0} + 1 WHERE id = $1",
                colonne_cible
            ))
            .bind(factcheck_id)
            .execute(pool.get_ref())
            .await?;
        }
    }

    // Renvoyer l'etat de reaction a jour pour ce factcheck et cet utilisateur
    let etat = sqlx::query_as::<_, FactcheckReactionEtat>(
        "SELECT f.nombre_coeur, f.nombre_pouce, f.nombre_rire, f.nombre_jaime_pas,
                f.prejuge_nombre_likes, f.realite_nombre_likes,
                (SELECT r.type_reaction FROM governance.factcheck_reaction r
                  WHERE r.factcheck_id = f.id AND r.utilisateur_id = $2 AND r.cible = 'general') AS ma_reaction_general,
                EXISTS(SELECT 1 FROM governance.factcheck_reaction r
                  WHERE r.factcheck_id = f.id AND r.utilisateur_id = $2 AND r.cible = 'prejuge') AS a_like_prejuge,
                EXISTS(SELECT 1 FROM governance.factcheck_reaction r
                  WHERE r.factcheck_id = f.id AND r.utilisateur_id = $2 AND r.cible = 'realite') AS a_like_realite
         FROM governance.factcheck f WHERE f.id = $1",
    )
    .bind(factcheck_id)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(etat),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/pays — Liste publique des pays (pour selecteurs)
// ──────────────────────────────────────────────────────────────

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct PaysPublicRow {
    pub id: Uuid,
    pub nom: String,
    pub code_iso2: Option<String>,
    pub code_iso3: Option<String>,
    pub continent: String,
}

pub async fn lister_pays_public(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let rows = sqlx::query_as::<_, PaysPublicRow>(
        "SELECT id, nom, code_iso2, code_iso3, continent
         FROM shared.pays
         WHERE actif = TRUE
         ORDER BY nom ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rows),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/gouvernance/factcheck/{id}/signalement — Signaler un factcheck
// ──────────────────────────────────────────────────────────────
pub async fn signaler_factcheck(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<SignalementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let factcheck_id = chemin.into_inner();

    // Le factcheck doit exister et ne pas etre supprime
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM governance.factcheck WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(factcheck_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Factcheck non trouve".into()));
    }

    // Insertion idempotente : un seul signalement par utilisateur
    let resultat = sqlx::query(
        "INSERT INTO governance.factcheck_signalement (factcheck_id, utilisateur_id, motif, commentaire)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (factcheck_id, utilisateur_id) DO NOTHING",
    )
    .bind(factcheck_id)
    .bind(utilisateur_id)
    .bind(body.motif.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .bind(body.commentaire.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .execute(pool.get_ref())
    .await?;
    let nouveau_signalement = resultat.rows_affected() > 0;

    // Recompter les signalements distincts et suspendre au-dela du seuil
    let nombre: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM governance.factcheck_signalement WHERE factcheck_id = $1",
    )
    .bind(factcheck_id)
    .fetch_one(pool.get_ref())
    .await?;

    let doit_suspendre = nombre > SEUIL_SIGNALEMENTS_SUSPENSION;

    // Met a jour le compteur ; suspend uniquement (jamais de re-publication auto)
    let etat: String = sqlx::query_scalar(
        "UPDATE governance.factcheck
         SET nombre_signalements = $2,
             etat = CASE WHEN $3 AND etat = 'publie' THEN 'suspendu' ELSE etat END,
             updated_at = NOW()
         WHERE id = $1
         RETURNING etat",
    )
    .bind(factcheck_id)
    .bind(nombre as i32)
    .bind(doit_suspendre)
    .fetch_one(pool.get_ref())
    .await?;

    if nouveau_signalement {
        let ip = audit::extraire_ip(&req);
        let ua = audit::extraire_user_agent(&req);
        audit::log_action(
            pool.get_ref(),
            Some(utilisateur_id),
            "SIGNALEMENT",
            "governance",
            "factcheck",
            Some(factcheck_id),
            None,
            None,
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
    }

    if doit_suspendre && etat == "suspendu" {
        log::warn!(
            "Factcheck {} suspendu automatiquement ({} signalements)",
            factcheck_id, nombre
        );
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SignalementEtat {
            nombre_signalements: nombre as i32,
            suspendu: etat == "suspendu",
            etat,
            deja_signale: !nouveau_signalement,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/gouvernance/factcheck — Publier un factcheck (authentifie)
// ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerFactcheckPublicRequest {
    pub contenu: String,
    pub source_originale: Option<String>,
    pub verdict: Option<String>,
    pub image_couverture_url: Option<String>,
    pub couleur_fond: Option<String>,
    pub pays_id: Option<Uuid>,
    pub prejuge_titre: Option<String>,
    pub prejuge_description: Option<String>,
    pub realite_titre: Option<String>,
    pub realite_description: Option<String>,
}

pub async fn creer_factcheck_public(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerFactcheckPublicRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;

    let contenu = body.contenu.trim();
    if contenu.is_empty() {
        return Err(ApiErreur::Validation("Le contenu est requis".into()));
    }

    if let Some(ref verdict) = body.verdict {
        let verdicts_valides = ["vrai", "faux", "partiellement_vrai", "trompeur", "non_verifie"];
        if !verdicts_valides.contains(&verdict.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Verdict invalide. Valeurs acceptees: {}",
                verdicts_valides.join(", ")
            )));
        }
    }

    let id = Uuid::new_v4();

    // Nettoyage des volets optionnels (chaine vide => NULL)
    let nettoyer = |o: &Option<String>| -> Option<String> {
        o.as_deref().map(str::trim).filter(|s| !s.is_empty()).map(str::to_string)
    };

    sqlx::query(
        "INSERT INTO governance.factcheck
         (id, contenu, source_originale, verdict, image_couverture_url, couleur_fond, pays_id,
          prejuge_titre, prejuge_description, realite_titre, realite_description, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'publie', $12)",
    )
    .bind(id)
    .bind(contenu)
    .bind(body.source_originale.as_deref().map(|s| s.trim()))
    .bind(body.verdict.as_deref())
    .bind(body.image_couverture_url.as_deref().map(|s| s.trim()))
    .bind(body.couleur_fond.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(nettoyer(&body.prejuge_titre))
    .bind(nettoyer(&body.prejuge_description))
    .bind(nettoyer(&body.realite_titre))
    .bind(nettoyer(&body.realite_description))
    .bind(auteur_id)
    .execute(pool.get_ref())
    .await?;

    sqlx::query(
        "UPDATE governance.factcheck SET search_vector =
         to_tsvector('french', COALESCE(contenu, '') || ' ' || COALESCE(source_originale, ''))
         WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Utilisateur {} a publie le factcheck {}", auteur_id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "CREATE",
        "governance",
        "factcheck",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/gouvernance/bad-habits — Publier une mauvaise pratique
// ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerBadHabitPublicRequest {
    pub titre: String,
    pub description_generale: String,
    pub details_problematique: String,
    pub categorie_probleme: String,
    pub categorie_probleme_detail: Option<String>,
    pub gravite: Option<String>,
    pub preuves_temoignages: Option<String>,
    pub solutions_proposees: Option<String>,
    pub publication_anonyme: Option<bool>,
    pub geolocalisation_autorisee: Option<bool>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub pays_id: Option<Uuid>,
    pub region: Option<String>,
    pub ville_quartier_zone: Option<String>,
    pub medias_urls: Option<Vec<String>>,
}

pub async fn creer_bad_habit_public(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerBadHabitPublicRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }
    let description = body.description_generale.trim();
    if description.is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }
    let details = body.details_problematique.trim();
    if details.is_empty() {
        return Err(ApiErreur::Validation("Les details de la problematique sont requis".into()));
    }
    if body.pays_id.is_none() {
        return Err(ApiErreur::Validation("Le pays est requis".into()));
    }

    let categories_valides = [
        "corruption",
        "service_public_defaillant",
        "infrastructure_degradee",
        "acces_services_limite",
        "insalubrite",
        "probleme_securite",
        "autre",
    ];
    if !categories_valides.contains(&body.categorie_probleme.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Categorie invalide. Valeurs acceptees: {}",
            categories_valides.join(", ")
        )));
    }

    if let Some(ref gravite) = body.gravite {
        let gravites_valides = ["faible", "elevee", "critique"];
        if !gravites_valides.contains(&gravite.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Gravite invalide. Valeurs acceptees: {}",
                gravites_valides.join(", ")
            )));
        }
    }

    let gravite = body.gravite.as_deref().unwrap_or("faible");
    let id = Uuid::new_v4();
    let slug = format!("{}-{}", generer_slug(titre), &id.to_string()[..8]);

    sqlx::query(
        "INSERT INTO governance.bad_habit
         (id, titre, slug, description_generale, details_problematique,
          categorie_probleme, categorie_probleme_detail, gravite,
          preuves_temoignages, solutions_proposees,
          publication_anonyme, geolocalisation_autorisee, longitude, latitude,
          pays_id, region, ville_quartier_zone, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8::governance.niveau_gravite, $9, $10,
                 $11, $12, $13, $14, $15, $16, $17, 'publie', $18)",
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(description)
    .bind(details)
    .bind(&body.categorie_probleme)
    .bind(body.categorie_probleme_detail.as_deref().map(|s| s.trim()))
    .bind(gravite)
    .bind(body.preuves_temoignages.as_deref().map(|s| s.trim()))
    .bind(body.solutions_proposees.as_deref().map(|s| s.trim()))
    .bind(body.publication_anonyme.unwrap_or(false))
    .bind(body.geolocalisation_autorisee.unwrap_or(false))
    .bind(body.longitude)
    .bind(body.latitude)
    .bind(body.pays_id)
    .bind(body.region.as_deref().map(|s| s.trim()))
    .bind(body.ville_quartier_zone.as_deref().map(|s| s.trim()))
    .bind(auteur_id)
    .execute(pool.get_ref())
    .await?;

    // Inserer les medias optionnels (URLs)
    if let Some(medias) = &body.medias_urls {
        for (ordre, url) in medias.iter().enumerate() {
            let url_clean = url.trim();
            if url_clean.is_empty() {
                continue;
            }
            sqlx::query(
                "INSERT INTO governance.bad_habit_media (id, bad_habit_id, media_url, ordre)
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(Uuid::new_v4())
            .bind(id)
            .bind(url_clean)
            .bind(ordre as i32)
            .execute(pool.get_ref())
            .await?;
        }
    }

    log::info!("Utilisateur {} a publie la mauvaise pratique {} ({})", auteur_id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "CREATE",
        "governance",
        "mauvaise_pratique",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/gouvernance/idea-forces — Publier une idee force
// ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerIdeaForcePublicRequest {
    pub titre: String,
    pub description_generale: String,
    pub details_proposition: String,
    pub categorie_proposition: String,
    pub categorie_proposition_detail: Option<String>,
    pub urgence: Option<String>,
    pub plan_implementation: Option<String>,
    pub ressources_necessaires: Option<String>,
    pub impact_attendu: Option<String>,
    pub pays_id: Option<Uuid>,
    pub region: Option<String>,
    pub ville_quartier_zone: Option<String>,
    pub medias_urls: Option<Vec<String>>,
}

pub async fn creer_idea_force_public(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerIdeaForcePublicRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let auteur_id = exiger_utilisateur_id(&req)?;

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }
    let description = body.description_generale.trim();
    if description.is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }
    let details = body.details_proposition.trim();
    if details.is_empty() {
        return Err(ApiErreur::Validation("Les details de la proposition sont requis".into()));
    }
    if body.pays_id.is_none() {
        return Err(ApiErreur::Validation("Le pays est requis".into()));
    }

    let categories_valides = [
        "amelioration_gouvernance",
        "education_formation",
        "sante_publique",
        "emploi_jeunes",
        "environnement",
        "transport",
        "autre",
    ];
    if !categories_valides.contains(&body.categorie_proposition.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Categorie invalide. Valeurs acceptees: {}",
            categories_valides.join(", ")
        )));
    }

    if let Some(ref urgence) = body.urgence {
        let urgences_valides = ["faible", "elevee", "critique"];
        if !urgences_valides.contains(&urgence.as_str()) {
            return Err(ApiErreur::Validation(format!(
                "Urgence invalide. Valeurs acceptees: {}",
                urgences_valides.join(", ")
            )));
        }
    }

    let urgence = body.urgence.as_deref().unwrap_or("faible");
    let id = Uuid::new_v4();
    let slug = format!("{}-{}", generer_slug(titre), &id.to_string()[..8]);

    sqlx::query(
        "INSERT INTO governance.idea_force
         (id, titre, slug, description_generale, details_proposition,
          categorie_proposition, categorie_proposition_detail, urgence,
          plan_implementation, ressources_necessaires, impact_attendu,
          pays_id, region, ville_quartier_zone, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8::governance.niveau_gravite, $9, $10,
                 $11, $12, $13, $14, 'publie', $15)",
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(description)
    .bind(details)
    .bind(&body.categorie_proposition)
    .bind(body.categorie_proposition_detail.as_deref().map(|s| s.trim()))
    .bind(urgence)
    .bind(body.plan_implementation.as_deref().map(|s| s.trim()))
    .bind(body.ressources_necessaires.as_deref().map(|s| s.trim()))
    .bind(body.impact_attendu.as_deref().map(|s| s.trim()))
    .bind(body.pays_id)
    .bind(body.region.as_deref().map(|s| s.trim()))
    .bind(body.ville_quartier_zone.as_deref().map(|s| s.trim()))
    .bind(auteur_id)
    .execute(pool.get_ref())
    .await?;

    if let Some(medias) = &body.medias_urls {
        for (ordre, url) in medias.iter().enumerate() {
            let url_clean = url.trim();
            if url_clean.is_empty() {
                continue;
            }
            sqlx::query(
                "INSERT INTO governance.idea_force_media (id, idea_force_id, media_url, ordre)
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(Uuid::new_v4())
            .bind(id)
            .bind(url_clean)
            .bind(ordre as i32)
            .execute(pool.get_ref())
            .await?;
        }
    }

    log::info!("Utilisateur {} a publie l'idee force {} ({})", auteur_id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(auteur_id),
        "CREATE",
        "governance",
        "idee_force",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

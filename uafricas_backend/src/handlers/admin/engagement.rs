//! Endpoints d'administration du système d'engagement (permission `engagement`).
//! Barème paramétrable (règles, paliers, niveaux) + journal global + ajustement manuel.
//! Toute mutation est auditée (`log_action`, schema `engagement`).

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::engagement::{
    ActionDisponible, AjustementRequest, AttribuerBadgeRequest, BadgeAdmin, CategorieAdmin,
    CreerBadgeRequest, CreerCategorieRequest, CreerNiveauRequest, CreerPalierRequest,
    CreerRegleRequest, JournalAdminPage, JournalAdminParams, JournalAdminRow, MiseEnAvantEtat,
    MiseEnAvantRequest, ModifierBadgeRequest, ModifierCategorieRequest, ModifierNiveauRequest,
    ModifierPalierRequest, ModifierRegleRequest, NiveauAdmin, NiveauxRecalculesResponse,
    PalierAdmin, RegleAdmin,
};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ── Catalogue des actions réellement instrumentées par le code (R3) ─────────
//
// Les points ne sont attribués QUE là où le code appelle `attribuer(...)`. Une
// règle créée pour un `type_action` que personne n'émet ne crédite jamais rien.
// Ce catalogue est donc la seule source de vérité de ce qui est branché : il
// vit dans le code, pas en base, parce que le code seul sait ce qu'il émet.
//
// ⚠️ À tenir à jour lors de tout nouveau branchement de `services::engagement`.
//
// Format : (type_action, libellé par défaut, types d'objet concernés, modules d'origine)
type CatalogueAction = (&'static str, &'static str, &'static [&'static str], &'static str);

pub const ACTIONS_INSTRUMENTEES: &[CatalogueAction] = &[
    (
        "contribution_validee",
        "Contribution validée par modération",
        &["codimoi", "video", "bad_habit", "ideaforce"],
        "admin/codimoi_admin, admin/vidafrica, admin/gouvernance",
    ),
    (
        "contribution_mise_en_avant",
        "Contribution mise en avant par l'équipe",
        &["codimoi", "factcheck", "bad_habit", "idea_force", "video"],
        "admin/engagement (mise en avant)",
    ),
    (
        "factcheck_valide",
        "FactCheck jugé correct",
        &["factcheck"],
        "admin/gouvernance",
    ),
    (
        "factcheck_faux",
        "FactCheck jugé faux / abusif",
        &["factcheck"],
        "admin/gouvernance",
    ),
    (
        "popularite_palier",
        "Palier de popularité franchi",
        &[
            "codimoi",
            "factcheck",
            "video",
            "biblio_humaine",
            "chaine_tv",
            "station_radio",
            "programme_tele",
            "programme_radio",
        ],
        "codimoi, gouvernance, vidafrica_contribution, bibliotheques_humaines, media_social",
    ),
    (
        "ajustement_admin",
        "Correction manuelle (administration)",
        &[],
        "admin/engagement (ajustement)",
    ),
    (
        "proposition_media_validee",
        "Proposition de média validée",
        &["chaine_tv", "station_radio", "programme_tele", "programme_radio"],
        "admin/media_proposition",
    ),
    (
        "media_a_la_une",
        "Contenu média mis à la une",
        &["chaine_tv", "station_radio", "programme_tele", "programme_radio"],
        "admin/radio_tele",
    ),
    (
        "animation_support_acceptee",
        "Demande d'animation d'un support acceptée",
        &["chaine_tv", "station_radio"],
        "admin/media_proposition, media_proposition (co-détenteurs)",
    ),
    (
        "partage_externe_5reseaux",
        "Contenu partagé sur plusieurs réseaux sociaux distincts",
        &[],
        "engagement (partages-externes)",
    ),
];

// ── Règles ──────────────────────────────────────────────────────────────────

/// GET /api/admin/engagement/regles
pub async fn lister_regles(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let mut regles = sqlx::query_as::<_, RegleAdmin>(
        "SELECT r.id, r.type_action, r.libelle, r.points, r.reputation_delta,
                r.plafond_journalier, r.plafond_mensuel, r.seuil_declencheur,
                r.categorie_id, c.code AS categorie_code, c.libelle AS categorie_libelle,
                r.actif,
                (SELECT COUNT(*) FROM engagement.mouvement_points m
                  WHERE m.type_action = r.type_action) AS nombre_mouvements
         FROM engagement.regle_points r
         LEFT JOIN engagement.categorie_points c ON c.id = r.categorie_id
         ORDER BY c.ordre NULLS LAST, r.type_action",
    )
    .fetch_all(pool.get_ref())
    .await?;

    // `instrumentee` ne peut pas venir du SQL : seul le code sait ce qu'il émet (R3).
    for regle in &mut regles {
        regle.instrumentee = ACTIONS_INSTRUMENTEES
            .iter()
            .any(|a| a.0 == regle.type_action);
    }

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(regles), error: None }))
}

/// GET /api/admin/engagement/actions-disponibles — l'antidote à la règle orpheline.
///
/// Le catalogue vient du code (`ACTIONS_INSTRUMENTEES`) : l'écran de création le
/// propose en priorité, de sorte que l'administrateur ne crée jamais à son insu
/// une règle pour un `type_action` que personne n'émet.
pub async fn actions_disponibles(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let existantes: Vec<String> =
        sqlx::query_scalar("SELECT type_action FROM engagement.regle_points")
            .fetch_all(pool.get_ref())
            .await?;

    let catalogue: Vec<ActionDisponible> = ACTIONS_INSTRUMENTEES
        .iter()
        .map(|(type_action, libelle_defaut, types_objet, module)| ActionDisponible {
            type_action,
            libelle_defaut,
            types_objet,
            module,
            regle_existante: existantes.iter().any(|e| e == type_action),
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(catalogue), error: None }))
}

/// Valide un identifiant technique (`type_action`, `code`) : c'est une **clé**,
/// pas une phrase — le front s'en sert pour ses icônes et le journal le référence
/// par valeur.
fn valider_cle(valeur: &str, champ: &str, max: usize) -> Result<String, ApiErreur> {
    let cle = valeur.trim().to_string();
    if cle.len() < 3 || cle.len() > max {
        return Err(ApiErreur::Validation(format!(
            "{champ} doit contenir entre 3 et {max} caractères"
        )));
    }
    if !cle.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
        return Err(ApiErreur::Validation(format!(
            "{champ} ne peut contenir que des minuscules non accentuées, des chiffres et des « _ »"
        )));
    }
    Ok(cle)
}

/// POST /api/admin/engagement/regles — création d'une règle de barème.
pub async fn creer_regle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerRegleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let type_action = valider_cle(&body.type_action, "type_action", 50)?;
    let libelle = body.libelle.trim();
    if libelle.is_empty() {
        return Err(ApiErreur::Validation("Le libellé est obligatoire".into()));
    }

    // 409 explicite plutôt qu'une violation de contrainte remontée brute.
    let deja_pris: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM engagement.regle_points WHERE type_action = $1)",
    )
    .bind(&type_action)
    .fetch_one(pool.get_ref())
    .await?;
    if deja_pris {
        return Err(ApiErreur::Conflit(format!(
            "Une règle existe déjà pour l'action « {type_action} » : modifiez-la au lieu d'en créer une seconde"
        )));
    }

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO engagement.regle_points
           (type_action, libelle, points, reputation_delta, plafond_journalier,
            plafond_mensuel, seuil_declencheur, categorie_id, actif)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, COALESCE($9, TRUE))
         RETURNING id",
    )
    .bind(&type_action)
    .bind(libelle)
    .bind(body.points)
    .bind(body.reputation_delta)
    .bind(body.plafond_journalier)
    .bind(body.plafond_mensuel)
    .bind(body.seuil_declencheur)
    .bind(body.categorie_id)
    .bind(body.actif)
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "engagement", "regle_points",
        Some(id), None,
        Some(serde_json::json!({
            "type_action": type_action, "libelle": libelle, "points": body.points,
            "reputation_delta": body.reputation_delta,
            "plafond_journalier": body.plafond_journalier,
            "plafond_mensuel": body.plafond_mensuel,
            "seuil_declencheur": body.seuil_declencheur,
            "categorie_id": body.categorie_id,
        })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/engagement/regles/{id}
///
/// `type_action` est **immuable** : le modifier orphelinerait tous les mouvements
/// passés, qui le référencent par chaîne et non par identifiant.
pub async fn modifier_regle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierRegleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    let avant: Option<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(r) FROM engagement.regle_points r WHERE r.id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    if avant.is_none() {
        return Err(ApiErreur::NonTrouve("Règle introuvable".into()));
    }

    sqlx::query(
        "UPDATE engagement.regle_points SET
            libelle            = COALESCE($2, libelle),
            points             = COALESCE($3, points),
            reputation_delta   = COALESCE($4, reputation_delta),
            plafond_journalier = CASE WHEN $5 THEN $6 ELSE plafond_journalier END,
            plafond_mensuel    = CASE WHEN $7 THEN $8 ELSE plafond_mensuel END,
            seuil_declencheur  = CASE WHEN $9 THEN $10 ELSE seuil_declencheur END,
            categorie_id       = CASE WHEN $11 THEN $12 ELSE categorie_id END,
            actif              = COALESCE($13, actif),
            updated_at         = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(body.libelle.as_deref())
    .bind(body.points)
    .bind(body.reputation_delta)
    .bind(body.plafond_journalier.is_some())
    .bind(body.plafond_journalier.flatten())
    .bind(body.plafond_mensuel.is_some())
    .bind(body.plafond_mensuel.flatten())
    .bind(body.seuil_declencheur.is_some())
    .bind(body.seuil_declencheur.flatten())
    .bind(body.categorie_id.is_some())
    .bind(body.categorie_id.flatten())
    .bind(body.actif)
    .execute(pool.get_ref())
    .await?;

    let apres: Option<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(r) FROM engagement.regle_points r WHERE r.id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "engagement", "regle_points",
        Some(id), avant, apres, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

/// DELETE /api/admin/engagement/regles/{id}
///
/// Refusé si la règle a déjà crédité quelqu'un : l'historique doit rester
/// lisible, donc une règle en service se **désactive** (FR-002). La suppression
/// réelle ne sert qu'à effacer une règle créée par erreur, jamais utilisée.
pub async fn supprimer_regle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    let regle: Option<(String, serde_json::Value)> = sqlx::query_as(
        "SELECT r.type_action, to_jsonb(r) FROM engagement.regle_points r WHERE r.id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (type_action, avant) =
        regle.ok_or_else(|| ApiErreur::NonTrouve("Règle introuvable".into()))?;

    let referencee: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM engagement.mouvement_points WHERE type_action = $1)",
    )
    .bind(&type_action)
    .fetch_one(pool.get_ref())
    .await?;
    if referencee {
        return Err(ApiErreur::Conflit(
            "Cette règle a déjà attribué des points : désactivez-la au lieu de la supprimer".into(),
        ));
    }

    sqlx::query("DELETE FROM engagement.regle_points WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "engagement", "regle_points",
        Some(id), Some(avant), None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Catégories de points ────────────────────────────────────────────────────

/// GET /api/admin/engagement/categories
pub async fn lister_categories(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    // `nombre_regles` dit ce qui est supprimable ; `nombre_mouvements` sert
    // l'avertissement chiffré (ces mouvements basculent en « Autres »).
    let categories = sqlx::query_as::<_, CategorieAdmin>(
        "SELECT c.id, c.code, c.libelle, c.description, c.ordre, c.couleur, c.icone, c.actif,
                (SELECT COUNT(*) FROM engagement.regle_points r WHERE r.categorie_id = c.id)
                    AS nombre_regles,
                (SELECT COUNT(*) FROM engagement.mouvement_points m WHERE m.categorie_id = c.id)
                    AS nombre_mouvements
         FROM engagement.categorie_points c
         ORDER BY c.ordre, c.libelle",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(categories), error: None }))
}

/// POST /api/admin/engagement/categories
pub async fn creer_categorie(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerCategorieRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let code = valider_cle(&body.code, "code", 30)?;
    let libelle = body.libelle.trim();
    if libelle.is_empty() {
        return Err(ApiErreur::Validation("Le libellé est obligatoire".into()));
    }

    let deja_pris: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM engagement.categorie_points WHERE code = $1)",
    )
    .bind(&code)
    .fetch_one(pool.get_ref())
    .await?;
    if deja_pris {
        return Err(ApiErreur::Conflit(format!(
            "Une catégorie porte déjà le code « {code} »"
        )));
    }

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO engagement.categorie_points
           (code, libelle, description, ordre, couleur, icone, actif)
         VALUES ($1, $2, $3, COALESCE($4, 0), $5, $6, COALESCE($7, TRUE))
         RETURNING id",
    )
    .bind(&code)
    .bind(libelle)
    .bind(body.description.as_deref())
    .bind(body.ordre)
    .bind(body.couleur.as_deref())
    .bind(body.icone.as_deref())
    .bind(body.actif)
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "engagement", "categorie_points",
        Some(id), None,
        Some(serde_json::json!({ "code": code, "libelle": libelle, "ordre": body.ordre })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/engagement/categories/{id} — `code` immuable.
pub async fn modifier_categorie(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierCategorieRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    let avant: Option<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(c) FROM engagement.categorie_points c WHERE c.id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let avant = avant.ok_or_else(|| ApiErreur::NonTrouve("Catégorie introuvable".into()))?;

    sqlx::query(
        "UPDATE engagement.categorie_points SET
            libelle     = COALESCE($2, libelle),
            description = CASE WHEN $3 THEN $4 ELSE description END,
            ordre       = COALESCE($5, ordre),
            couleur     = CASE WHEN $6 THEN $7 ELSE couleur END,
            icone       = CASE WHEN $8 THEN $9 ELSE icone END,
            actif       = COALESCE($10, actif),
            updated_at  = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(body.libelle.as_deref().map(str::trim))
    .bind(body.description.is_some())
    .bind(body.description.clone().flatten())
    .bind(body.ordre)
    .bind(body.couleur.is_some())
    .bind(body.couleur.clone().flatten())
    .bind(body.icone.is_some())
    .bind(body.icone.clone().flatten())
    .bind(body.actif)
    .execute(pool.get_ref())
    .await?;

    let apres: Option<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(c) FROM engagement.categorie_points c WHERE c.id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "engagement", "categorie_points",
        Some(id), Some(avant), apres, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

/// DELETE /api/admin/engagement/categories/{id}
///
/// Refusé si une règle la référence (`ON DELETE RESTRICT` en filet). Les
/// mouvements passés ne bloquent pas : leur `categorie_id` passe à NULL
/// (`ON DELETE SET NULL`) et ils basculent sous « Autres ».
pub async fn supprimer_categorie(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    let avant: Option<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(c) FROM engagement.categorie_points c WHERE c.id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let avant = avant.ok_or_else(|| ApiErreur::NonTrouve("Catégorie introuvable".into()))?;

    let nombre_regles: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM engagement.regle_points WHERE categorie_id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if nombre_regles > 0 {
        return Err(ApiErreur::Conflit(format!(
            "{nombre_regles} règle(s) utilisent cette catégorie : réaffectez-les avant de la supprimer, ou désactivez-la"
        )));
    }

    sqlx::query("DELETE FROM engagement.categorie_points WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "engagement", "categorie_points",
        Some(id), Some(avant), None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Paliers de popularité ───────────────────────────────────────────────────

/// GET /api/admin/engagement/paliers
pub async fn lister_paliers(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    // Les globaux d'abord, puis chaque famille : c'est le regroupement attendu
    // par l'écran, qui rappelle que les paliers d'une famille REMPLACENT les
    // globaux pour cette famille (R4).
    let paliers = sqlx::query_as::<_, PalierAdmin>(
        "SELECT id, seuil_likes, points, type_objet, actif
         FROM engagement.palier_popularite
         ORDER BY type_objet NULLS FIRST, seuil_likes",
    )
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(paliers), error: None }))
}

/// Familles de contenus auxquelles un palier de popularité peut être restreint.
/// Littéraux fixes, alignés sur les appelants de `evaluer_popularite`.
const FAMILLES_POPULARITE: &[&str] = &[
    "codimoi",
    "factcheck",
    "video",
    "biblio_humaine",
    "chaine_tv",
    "station_radio",
    "programme_tele",
    "programme_radio",
];

/// Normalise le `type_objet` d'un palier : `None`/chaîne vide = palier global.
fn valider_famille(brut: Option<&str>) -> Result<Option<String>, ApiErreur> {
    match brut.map(str::trim).filter(|s| !s.is_empty()) {
        None => Ok(None),
        Some(f) if FAMILLES_POPULARITE.contains(&f) => Ok(Some(f.to_string())),
        Some(f) => Err(ApiErreur::Validation(format!(
            "Famille de contenus inconnue : « {f} »"
        ))),
    }
}

/// POST /api/admin/engagement/paliers
pub async fn creer_palier(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerPalierRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    if body.seuil_likes <= 0 {
        return Err(ApiErreur::Validation("seuil_likes doit être positif".into()));
    }
    let famille = valider_famille(body.type_objet.as_deref())?;

    // `ON CONFLICT` sur l'index `NULLS NOT DISTINCT` : reposer le même couple
    // (seuil, famille) réactive et réajuste le palier au lieu d'en créer un
    // second, qui créditerait deux fois le même franchissement.
    let palier = sqlx::query_as::<_, PalierAdmin>(
        "INSERT INTO engagement.palier_popularite (seuil_likes, points, type_objet)
         VALUES ($1, $2, $3)
         ON CONFLICT (seuil_likes, type_objet)
           DO UPDATE SET points = EXCLUDED.points, actif = TRUE
         RETURNING id, seuil_likes, points, type_objet, actif",
    )
    .bind(body.seuil_likes)
    .bind(body.points)
    .bind(famille.as_deref())
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "engagement", "palier_popularite",
        Some(palier.id), None,
        Some(serde_json::json!({
            "seuil_likes": body.seuil_likes, "points": body.points, "type_objet": famille,
        })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(palier), error: None }))
}

/// PUT /api/admin/engagement/paliers/{id}
pub async fn modifier_palier(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierPalierRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    let famille = match &body.type_objet {
        Some(v) => Some(valider_famille(v.as_deref())?),
        None => None,
    };

    let res = sqlx::query(
        "UPDATE engagement.palier_popularite
         SET points     = COALESCE($2, points),
             type_objet = CASE WHEN $3 THEN $4 ELSE type_objet END,
             actif      = COALESCE($5, actif)
         WHERE id = $1",
    )
    .bind(id)
    .bind(body.points)
    .bind(famille.is_some())
    .bind(famille.clone().flatten())
    .bind(body.actif)
    .execute(pool.get_ref())
    .await;

    // L'index `(seuil_likes, type_objet) NULLS NOT DISTINCT` peut refuser un
    // déplacement de famille : 409 explicite plutôt qu'une 500.
    if let Err(e) = res {
        if e.as_database_error().is_some_and(|db| db.is_unique_violation()) {
            return Err(ApiErreur::Conflit(
                "Un palier existe déjà pour ce seuil dans cette famille de contenus".into(),
            ));
        }
        return Err(e.into());
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "engagement", "palier_popularite",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

/// DELETE /api/admin/engagement/paliers/{id} — désactivation (référencé par le journal).
pub async fn desactiver_palier(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    sqlx::query("UPDATE engagement.palier_popularite SET actif = FALSE WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "engagement", "palier_popularite",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Niveaux ─────────────────────────────────────────────────────────────────

/// GET /api/admin/engagement/niveaux
pub async fn lister_niveaux(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let niveaux = sqlx::query_as::<_, NiveauAdmin>(
        "SELECT id, code, libelle, seuil_min, ordre, badge_couleur, badge_icone
         FROM engagement.niveau ORDER BY ordre",
    )
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(niveaux), error: None }))
}

/// Recalcule, **dans la transaction de la mutation**, l'ordre des niveaux puis le
/// niveau de tous les comptes (R5).
///
/// Pourquoi c'est indispensable : `compte.niveau_code` n'est recalculé qu'au
/// prochain mouvement du membre. Sans ce recalcul ensembliste, insérer un niveau
/// intermédiaire laisserait des milliers de comptes sur un code périmé jusqu'à
/// leur prochaine action — alors que la spec exige la bascule immédiate, sans
/// opération manuelle membre par membre.
///
/// Une seule requête pour toute la table : `ordre` est réaligné sur `seuil_min`
/// croissant, de sorte que les deux ne puissent jamais se contredire.
async fn recalculer_niveaux(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<u64, sqlx::Error> {
    sqlx::query(
        "UPDATE engagement.niveau n
            SET ordre = r.rang
           FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY seuil_min) AS rang
                   FROM engagement.niveau) r
          WHERE r.id = n.id AND n.ordre <> r.rang::smallint",
    )
    .execute(&mut **tx)
    .await?;

    // `COALESCE(..., 'membre')` : filet identique à `charger_niveau`, au cas où le
    // barème n'aurait plus aucun niveau à seuil 0.
    let res = sqlx::query(
        "UPDATE engagement.compte c
            SET niveau_code = COALESCE((SELECT n.code FROM engagement.niveau n
                                         WHERE n.seuil_min <= c.solde_points
                                         ORDER BY n.seuil_min DESC LIMIT 1), 'membre'),
                updated_at = NOW()
          WHERE c.niveau_code <> COALESCE((SELECT n.code FROM engagement.niveau n
                                            WHERE n.seuil_min <= c.solde_points
                                            ORDER BY n.seuil_min DESC LIMIT 1), 'membre')",
    )
    .execute(&mut **tx)
    .await?;

    Ok(res.rows_affected())
}

/// Charge les niveaux depuis une transaction (réponse des 3 mutations).
async fn niveaux_dans_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<Vec<NiveauAdmin>, sqlx::Error> {
    sqlx::query_as::<_, NiveauAdmin>(
        "SELECT id, code, libelle, seuil_min, ordre, badge_couleur, badge_icone
         FROM engagement.niveau ORDER BY seuil_min",
    )
    .fetch_all(&mut **tx)
    .await
}

/// POST /api/admin/engagement/niveaux
pub async fn creer_niveau(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerNiveauRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let code = valider_cle(&body.code, "code", 30)?;
    let libelle = body.libelle.trim();
    if libelle.is_empty() {
        return Err(ApiErreur::Validation("Le libellé est obligatoire".into()));
    }
    if body.seuil_min < 0 {
        return Err(ApiErreur::Validation("Le seuil ne peut pas être négatif".into()));
    }

    let mut tx = pool.begin().await?;

    let doublon: Option<String> = sqlx::query_scalar(
        "SELECT CASE WHEN code = $1 THEN 'code' ELSE 'seuil' END
         FROM engagement.niveau WHERE code = $1 OR seuil_min = $2 LIMIT 1",
    )
    .bind(&code)
    .bind(body.seuil_min)
    .fetch_optional(&mut *tx)
    .await?;
    if let Some(champ) = doublon {
        return Err(ApiErreur::Conflit(if champ == "code" {
            format!("Un niveau porte déjà le code « {code} »")
        } else {
            format!("Un niveau existe déjà au seuil de {} points", body.seuil_min)
        }));
    }

    // `ordre` est provisoire : `recalculer_niveaux` le réaligne juste après.
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO engagement.niveau (code, libelle, seuil_min, ordre, badge_couleur, badge_icone)
         VALUES ($1, $2, $3, 0, $4, $5)
         RETURNING id",
    )
    .bind(&code)
    .bind(libelle)
    .bind(body.seuil_min)
    .bind(body.badge_couleur.as_deref())
    .bind(body.badge_icone.as_deref())
    .fetch_one(&mut *tx)
    .await?;

    let comptes_recalcules = recalculer_niveaux(&mut tx).await?;
    let niveaux = niveaux_dans_tx(&mut tx).await?;
    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "engagement", "niveau",
        Some(id), None,
        Some(serde_json::json!({
            "code": code, "libelle": libelle, "seuil_min": body.seuil_min,
            "comptes_recalcules": comptes_recalcules,
        })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(NiveauxRecalculesResponse { niveaux, comptes_recalcules }),
        error: None,
    }))
}

/// PUT /api/admin/engagement/niveaux/{id} — `code` immuable, recalcul inclus.
pub async fn modifier_niveau(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierNiveauRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    if body.seuil_min.is_some_and(|s| s < 0) {
        return Err(ApiErreur::Validation("Le seuil ne peut pas être négatif".into()));
    }

    let mut tx = pool.begin().await?;

    let avant: Option<serde_json::Value> =
        sqlx::query_scalar("SELECT to_jsonb(n) FROM engagement.niveau n WHERE n.id = $1")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?;
    let avant = avant.ok_or_else(|| ApiErreur::NonTrouve("Niveau introuvable".into()))?;

    if let Some(seuil) = body.seuil_min {
        let occupe: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM engagement.niveau WHERE seuil_min = $1 AND id <> $2)",
        )
        .bind(seuil)
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;
        if occupe {
            return Err(ApiErreur::Conflit(format!(
                "Un autre niveau occupe déjà le seuil de {seuil} points"
            )));
        }
    }

    sqlx::query(
        "UPDATE engagement.niveau SET
            libelle       = COALESCE($2, libelle),
            seuil_min     = COALESCE($3, seuil_min),
            badge_couleur = COALESCE($4, badge_couleur),
            badge_icone   = COALESCE($5, badge_icone)
         WHERE id = $1",
    )
    .bind(id)
    .bind(body.libelle.as_deref().map(str::trim))
    .bind(body.seuil_min)
    .bind(body.badge_couleur.as_deref())
    .bind(body.badge_icone.as_deref())
    .execute(&mut *tx)
    .await?;

    let comptes_recalcules = recalculer_niveaux(&mut tx).await?;
    let apres: Option<serde_json::Value> =
        sqlx::query_scalar("SELECT to_jsonb(n) FROM engagement.niveau n WHERE n.id = $1")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?;
    let niveaux = niveaux_dans_tx(&mut tx).await?;
    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "engagement", "niveau",
        Some(id), Some(avant), apres, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(NiveauxRecalculesResponse { niveaux, comptes_recalcules }),
        error: None,
    }))
}

/// DELETE /api/admin/engagement/niveaux/{id}
///
/// Deux garde-fous : le **niveau plancher** (`seuil_min = 0`) et le **dernier**
/// niveau restant ne peuvent pas être retirés — sans plancher, aucun compte ne
/// pourrait plus être classé. Les membres portés par un niveau retiré retombent
/// au niveau inférieur, ce que `recalculer_niveaux` applique immédiatement.
pub async fn supprimer_niveau(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    let mut tx = pool.begin().await?;

    let niveau: Option<(i32, serde_json::Value)> = sqlx::query_as(
        "SELECT n.seuil_min, to_jsonb(n) FROM engagement.niveau n WHERE n.id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    let (seuil_min, avant) =
        niveau.ok_or_else(|| ApiErreur::NonTrouve("Niveau introuvable".into()))?;

    if seuil_min == 0 {
        return Err(ApiErreur::Conflit(
            "Le niveau plancher (seuil 0) ne peut pas être retiré : il classe tous les membres sans points".into(),
        ));
    }

    let restants: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM engagement.niveau")
        .fetch_one(&mut *tx)
        .await?;
    if restants <= 1 {
        return Err(ApiErreur::Conflit(
            "Il doit rester au moins un niveau dans le barème".into(),
        ));
    }

    sqlx::query("DELETE FROM engagement.niveau WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    let comptes_recalcules = recalculer_niveaux(&mut tx).await?;
    let niveaux = niveaux_dans_tx(&mut tx).await?;
    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "engagement", "niveau",
        Some(id), Some(avant),
        Some(serde_json::json!({ "comptes_recalcules": comptes_recalcules })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(NiveauxRecalculesResponse { niveaux, comptes_recalcules }),
        error: None,
    }))
}

// ── Journal global ──────────────────────────────────────────────────────────

/// GET /api/admin/engagement/journal
pub async fn lister_journal(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<JournalAdminParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(30).clamp(1, 100);
    let offset = (page - 1) * taille;

    // Filtres optionnels via des paramètres nullables castés (protège contre l'injection).
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM engagement.mouvement_points m
         LEFT JOIN engagement.categorie_points c ON c.id = m.categorie_id
         WHERE ($1::uuid IS NULL OR m.utilisateur_id = $1)
           AND ($2::text IS NULL OR m.type_action = $2)
           AND ($3::timestamptz IS NULL OR m.created_at >= $3::timestamptz)
           AND ($4::timestamptz IS NULL OR m.created_at <= $4::timestamptz)
           AND ($5::text IS NULL OR c.code = $5)",
    )
    .bind(params.utilisateur_id)
    .bind(params.type_action.as_deref())
    .bind(params.depuis.as_deref())
    .bind(params.jusqu_a.as_deref())
    .bind(params.categorie.as_deref())
    .fetch_one(pool.get_ref())
    .await?;

    let elements = sqlx::query_as::<_, JournalAdminRow>(
        "SELECT m.id, m.utilisateur_id,
                (u.prenom || ' ' || u.nom) AS utilisateur_nom,
                m.type_action,
                c.code AS categorie_code, c.libelle AS categorie_libelle,
                m.type_objet, m.objet_id, m.points, m.reputation_delta,
                m.solde_apres, m.plafond_atteint, m.created_at
         FROM engagement.mouvement_points m
         LEFT JOIN iam.utilisateur u ON u.id = m.utilisateur_id
         LEFT JOIN engagement.categorie_points c ON c.id = m.categorie_id
         WHERE ($1::uuid IS NULL OR m.utilisateur_id = $1)
           AND ($2::text IS NULL OR m.type_action = $2)
           AND ($3::timestamptz IS NULL OR m.created_at >= $3::timestamptz)
           AND ($4::timestamptz IS NULL OR m.created_at <= $4::timestamptz)
           AND ($5::text IS NULL OR c.code = $5)
         ORDER BY m.created_at DESC
         LIMIT $6 OFFSET $7",
    )
    .bind(params.utilisateur_id)
    .bind(params.type_action.as_deref())
    .bind(params.depuis.as_deref())
    .bind(params.jusqu_a.as_deref())
    .bind(params.categorie.as_deref())
    .bind(taille)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(JournalAdminPage { elements, total, page, taille }),
        error: None,
    }))
}

/// POST /api/admin/engagement/ajustement — crédit/débit manuel motivé.
pub async fn ajuster_points(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<AjustementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    crate::services::engagement::ajuster(
        pool.get_ref(),
        body.utilisateur_id,
        body.points,
        body.reputation_delta,
    )
    .await;

    let nouveau_solde: Option<i32> =
        sqlx::query_scalar("SELECT solde_points FROM engagement.compte WHERE utilisateur_id = $1")
            .bind(body.utilisateur_id)
            .fetch_optional(pool.get_ref())
            .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "AJUSTEMENT", "engagement", "compte",
        Some(body.utilisateur_id), None,
        Some(serde_json::json!({ "points": body.points, "reputation_delta": body.reputation_delta, "motif": body.motif })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "solde_points": nouveau_solde.unwrap_or(0) })),
        error: None,
    }))
}

// ── Badges ──────────────────────────────────────────────────────────────────

/// Les 5 conditions automatiques reconnues (miroir de l'enum PostgreSQL).
const CONDITIONS_BADGE: &[&str] = &[
    "actions_comptees",
    "points_categorie",
    "solde_total",
    "niveau_atteint",
    "palier_popularite",
];

/// Paramètres d'un badge, après fusion de l'état existant et du patch.
struct ConditionBadge<'a> {
    manuel: bool,
    type_condition: Option<&'a str>,
    parametre_action: Option<&'a str>,
    parametre_categorie_id: Option<Uuid>,
    parametre_niveau_code: Option<&'a str>,
    seuil: Option<i32>,
}

/// Validation applicative **miroir du CHECK SQL `ck_badge_condition`**.
///
/// Le CHECK reste le garant final (l'invariant ne doit pas dépendre de l'API),
/// mais une violation de contrainte remontée telle quelle donnerait à
/// l'administrateur un message PostgreSQL illisible. On refuse donc ici, en
/// français, avec la raison exacte.
fn valider_condition_badge(c: &ConditionBadge) -> Result<(), ApiErreur> {
    if c.manuel {
        if c.type_condition.is_some() {
            return Err(ApiErreur::Validation(
                "Un badge manuel ne peut pas porter de condition automatique : il est attribué à la main".into(),
            ));
        }
        return Ok(());
    }

    let Some(condition) = c.type_condition else {
        return Err(ApiErreur::Validation(
            "Choisissez une condition, ou cochez « badge manuel »".into(),
        ));
    };
    if !CONDITIONS_BADGE.contains(&condition) {
        return Err(ApiErreur::Validation(format!(
            "Condition inconnue : « {condition} »"
        )));
    }

    let seuil_positif = c.seuil.is_some_and(|s| s > 0);
    match condition {
        "actions_comptees" => {
            if c.parametre_action.map(str::trim).is_none_or(str::is_empty) {
                return Err(ApiErreur::Validation(
                    "Précisez l'action à compter (ex. contribution_validee)".into(),
                ));
            }
            if !seuil_positif {
                return Err(ApiErreur::Validation(
                    "Précisez un nombre d'actions strictement positif".into(),
                ));
            }
        }
        "points_categorie" => {
            if c.parametre_categorie_id.is_none() {
                return Err(ApiErreur::Validation("Choisissez la catégorie visée".into()));
            }
            if !seuil_positif {
                return Err(ApiErreur::Validation(
                    "Précisez un nombre de points strictement positif".into(),
                ));
            }
        }
        "solde_total" | "palier_popularite" => {
            if !seuil_positif {
                return Err(ApiErreur::Validation(
                    "Précisez un seuil strictement positif".into(),
                ));
            }
        }
        "niveau_atteint" => {
            if c.parametre_niveau_code.map(str::trim).is_none_or(str::is_empty) {
                return Err(ApiErreur::Validation("Choisissez le niveau visé".into()));
            }
        }
        _ => unreachable!("condition déjà validée contre CONDITIONS_BADGE"),
    }

    Ok(())
}

/// GET /api/admin/engagement/badges
pub async fn lister_badges(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let badges = sqlx::query_as::<_, BadgeAdmin>(
        "SELECT b.id, b.code, b.libelle, b.description, b.couleur, b.icone, b.manuel,
                b.type_condition::text AS type_condition,
                b.parametre_action, b.parametre_categorie_id, b.parametre_niveau_code,
                b.seuil, b.ordre, b.actif,
                (SELECT COUNT(*) FROM engagement.badge_obtenu bo WHERE bo.badge_id = b.id)
                    AS nombre_detenteurs
         FROM engagement.badge b
         ORDER BY b.ordre, b.libelle",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(badges), error: None }))
}

/// POST /api/admin/engagement/badges
pub async fn creer_badge(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerBadgeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let code = valider_cle(&body.code, "code", 40)?;
    let libelle = body.libelle.trim();
    let description = body.description.trim();
    if libelle.is_empty() || description.is_empty() {
        return Err(ApiErreur::Validation(
            "Le libellé et la description (condition en langage clair) sont obligatoires".into(),
        ));
    }

    let type_condition = body.type_condition.as_deref().map(str::trim).filter(|s| !s.is_empty());
    valider_condition_badge(&ConditionBadge {
        manuel: body.manuel,
        type_condition,
        parametre_action: body.parametre_action.as_deref(),
        parametre_categorie_id: body.parametre_categorie_id,
        parametre_niveau_code: body.parametre_niveau_code.as_deref(),
        seuil: body.seuil,
    })?;

    let deja_pris: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM engagement.badge WHERE code = $1)")
            .bind(&code)
            .fetch_one(pool.get_ref())
            .await?;
    if deja_pris {
        return Err(ApiErreur::Conflit(format!(
            "Un badge porte déjà le code « {code} »"
        )));
    }

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO engagement.badge
           (code, libelle, description, couleur, icone, manuel, type_condition,
            parametre_action, parametre_categorie_id, parametre_niveau_code, seuil, ordre, actif)
         VALUES ($1, $2, $3, $4, $5, $6, $7::engagement.type_condition_badge,
                 $8, $9, $10, $11, COALESCE($12, 0), COALESCE($13, TRUE))
         RETURNING id",
    )
    .bind(&code)
    .bind(libelle)
    .bind(description)
    .bind(body.couleur.as_deref())
    .bind(body.icone.as_deref())
    .bind(body.manuel)
    .bind(type_condition)
    .bind(body.parametre_action.as_deref())
    .bind(body.parametre_categorie_id)
    .bind(body.parametre_niveau_code.as_deref())
    .bind(body.seuil)
    .bind(body.ordre)
    .bind(body.actif)
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "engagement", "badge",
        Some(id), None,
        Some(serde_json::json!({
            "code": code, "libelle": libelle, "manuel": body.manuel,
            "type_condition": type_condition, "seuil": body.seuil,
        })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/engagement/badges/{id} — `code` immuable.
pub async fn modifier_badge(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierBadgeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    // Le patch est partiel : la cohérence doit être validée sur l'état FUSIONNÉ,
    // pas sur les seuls champs transmis — sinon on refuserait des patchs valides
    // et on accepterait des combinaisons cassées.
    let actuel = sqlx::query_as::<_, BadgeAdmin>(
        "SELECT b.id, b.code, b.libelle, b.description, b.couleur, b.icone, b.manuel,
                b.type_condition::text AS type_condition,
                b.parametre_action, b.parametre_categorie_id, b.parametre_niveau_code,
                b.seuil, b.ordre, b.actif, 0::bigint AS nombre_detenteurs
         FROM engagement.badge b WHERE b.id = $1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Badge introuvable".into()))?;

    let manuel = body.manuel.unwrap_or(actuel.manuel);
    let type_condition = match &body.type_condition {
        Some(v) => v.as_deref().map(str::trim).filter(|s| !s.is_empty()).map(str::to_string),
        None => actuel.type_condition.clone(),
    };
    let parametre_action = match &body.parametre_action {
        Some(v) => v.clone(),
        None => actuel.parametre_action.clone(),
    };
    let parametre_categorie_id = match body.parametre_categorie_id {
        Some(v) => v,
        None => actuel.parametre_categorie_id,
    };
    let parametre_niveau_code = match &body.parametre_niveau_code {
        Some(v) => v.clone(),
        None => actuel.parametre_niveau_code.clone(),
    };
    let seuil = match body.seuil {
        Some(v) => v,
        None => actuel.seuil,
    };

    // Un badge devenu manuel perd sa condition : la garder violerait le CHECK.
    let type_condition = if manuel { None } else { type_condition };

    valider_condition_badge(&ConditionBadge {
        manuel,
        type_condition: type_condition.as_deref(),
        parametre_action: parametre_action.as_deref(),
        parametre_categorie_id,
        parametre_niveau_code: parametre_niveau_code.as_deref(),
        seuil,
    })?;

    let avant: Option<serde_json::Value> =
        sqlx::query_scalar("SELECT to_jsonb(b) FROM engagement.badge b WHERE b.id = $1")
            .bind(id)
            .fetch_optional(pool.get_ref())
            .await?;

    sqlx::query(
        "UPDATE engagement.badge SET
            libelle                = COALESCE($2, libelle),
            description            = COALESCE($3, description),
            couleur                = CASE WHEN $4 THEN $5 ELSE couleur END,
            icone                  = CASE WHEN $6 THEN $7 ELSE icone END,
            manuel                 = $8,
            type_condition         = $9::engagement.type_condition_badge,
            parametre_action       = $10,
            parametre_categorie_id = $11,
            parametre_niveau_code  = $12,
            seuil                  = $13,
            ordre                  = COALESCE($14, ordre),
            actif                  = COALESCE($15, actif),
            updated_at             = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(body.libelle.as_deref().map(str::trim))
    .bind(body.description.as_deref().map(str::trim))
    .bind(body.couleur.is_some())
    .bind(body.couleur.clone().flatten())
    .bind(body.icone.is_some())
    .bind(body.icone.clone().flatten())
    .bind(manuel)
    .bind(type_condition.as_deref())
    .bind(parametre_action.as_deref())
    .bind(parametre_categorie_id)
    .bind(parametre_niveau_code.as_deref())
    .bind(seuil)
    .bind(body.ordre)
    .bind(body.actif)
    .execute(pool.get_ref())
    .await?;

    let apres: Option<serde_json::Value> =
        sqlx::query_scalar("SELECT to_jsonb(b) FROM engagement.badge b WHERE b.id = $1")
            .bind(id)
            .fetch_optional(pool.get_ref())
            .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "engagement", "badge",
        Some(id), avant, apres, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

/// DELETE /api/admin/engagement/badges/{id}
///
/// Refusé si le badge est détenu : un membre qui l'a obtenu doit le conserver
/// (FR-020). Le retirer du catalogue se fait par `actif = FALSE`, ce qui le
/// masque des « à débloquer » sans le retirer à personne.
pub async fn supprimer_badge(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    let avant: Option<serde_json::Value> =
        sqlx::query_scalar("SELECT to_jsonb(b) FROM engagement.badge b WHERE b.id = $1")
            .bind(id)
            .fetch_optional(pool.get_ref())
            .await?;
    let avant = avant.ok_or_else(|| ApiErreur::NonTrouve("Badge introuvable".into()))?;

    let detenteurs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM engagement.badge_obtenu WHERE badge_id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if detenteurs > 0 {
        return Err(ApiErreur::Conflit(format!(
            "{detenteurs} membre(s) détiennent ce badge : désactivez-le, ils doivent le conserver"
        )));
    }

    sqlx::query("DELETE FROM engagement.badge WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "engagement", "badge",
        Some(id), Some(avant), None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

/// POST /api/admin/engagement/badges/{id}/attribuer — attribution manuelle.
pub async fn attribuer_badge_manuel(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AttribuerBadgeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let badge_id = path.into_inner();

    let badge: Option<String> =
        sqlx::query_scalar("SELECT libelle FROM engagement.badge WHERE id = $1")
            .bind(badge_id)
            .fetch_optional(pool.get_ref())
            .await?;
    let libelle = badge.ok_or_else(|| ApiErreur::NonTrouve("Badge introuvable".into()))?;

    let res = sqlx::query(
        "INSERT INTO engagement.badge_obtenu (utilisateur_id, badge_id, origine, attribue_par)
         VALUES ($1, $2, 'manuel', $3)
         ON CONFLICT (utilisateur_id, badge_id) DO NOTHING",
    )
    .bind(body.utilisateur_id)
    .bind(badge_id)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    let cree = res.rows_affected() == 1;

    // Notification uniquement si la ligne a été créée : réattribuer un badge déjà
    // détenu ne doit rien annoncer au membre.
    if cree {
        crate::models::notification::creer_notification(
            pool.get_ref(),
            body.utilisateur_id,
            crate::models::notification::engagement::BADGE_DEBLOQUE,
            &format!("Nouveau badge débloqué : « {libelle} » !"),
            Some(crate::models::notification::engagement::LIEN_ESPACE),
        )
        .await;
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "ATTRIBUTION_BADGE", "engagement", "badge_obtenu",
        Some(badge_id), None,
        Some(serde_json::json!({
            "utilisateur_id": body.utilisateur_id, "badge": libelle,
            "motif": body.motif, "deja_detenu": !cree,
        })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "attribue": cree })),
        error: None,
    }))
}

/// DELETE /api/admin/engagement/badges/{id}/attribuer/{utilisateur_id} — retrait.
///
/// **Aucune notification** : on n'annonce pas un retrait à un membre, c'est un
/// geste de correction. La trace vit dans l'audit.
pub async fn retirer_badge_manuel(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let (badge_id, utilisateur_id) = path.into_inner();

    let avant: Option<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(bo) FROM engagement.badge_obtenu bo
         WHERE bo.badge_id = $1 AND bo.utilisateur_id = $2",
    )
    .bind(badge_id)
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    sqlx::query(
        "DELETE FROM engagement.badge_obtenu WHERE badge_id = $1 AND utilisateur_id = $2",
    )
    .bind(badge_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "RETRAIT_BADGE", "engagement", "badge_obtenu",
        Some(badge_id), avant,
        Some(serde_json::json!({ "utilisateur_id": utilisateur_id })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Mise en avant d'une contribution (règle `contribution_mise_en_avant`, +5) ─

/// Mappe un `type_objet` vers `(table, colonne auteur)`. Seules les contributions
/// ayant un auteur unique sont éligibles. Les noms sont des littéraux fixes →
/// interpolation SQL sûre.
fn table_et_auteur(type_objet: &str) -> Option<(&'static str, &'static str)> {
    match type_objet {
        "codimoi" => Some(("culture.codimoi", "cree_par")),
        "factcheck" => Some(("governance.factcheck", "cree_par")),
        "bad_habit" => Some(("governance.bad_habit", "cree_par")),
        "idea_force" => Some(("governance.idea_force", "cree_par")),
        "video" => Some(("media_content.piste_sous_titre", "cree_par")),
        _ => None,
    }
}

/// GET /api/admin/engagement/mise-en-avant/{type_objet}/{objet_id}
/// Indique si la contribution est actuellement mise en avant (pour le bouton admin).
pub async fn statut_mise_en_avant(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let (type_objet, objet_id) = path.into_inner();

    let mis_en_avant: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM engagement.mise_en_avant
                       WHERE type_objet = $1 AND objet_id = $2)",
    )
    .bind(&type_objet)
    .bind(objet_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MiseEnAvantEtat { mis_en_avant }),
        error: None,
    }))
}

/// POST /api/admin/engagement/mise-en-avant — met une contribution en avant et
/// crédite son auteur du +5 (idempotent, anti‑auto‑attribution, non‑bloquant).
pub async fn mettre_en_avant(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<MiseEnAvantRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let (table, col_auteur) = table_et_auteur(&body.type_objet)
        .ok_or_else(|| ApiErreur::Validation("type_objet non éligible à la mise en avant".into()))?;

    // Résolution de l'auteur (littéraux fixes → SQL sûr).
    let sql = format!("SELECT {col_auteur} FROM {table} WHERE id = $1");
    let auteur_id: Option<Uuid> = sqlx::query_scalar(&sql)
        .bind(body.objet_id)
        .fetch_optional(pool.get_ref())
        .await?;
    let auteur_id = auteur_id.ok_or_else(|| ApiErreur::NonTrouve("Contribution introuvable".into()))?;

    // Marque la mise en avant (une seule fois par objet).
    let res = sqlx::query(
        "INSERT INTO engagement.mise_en_avant (type_objet, objet_id, auteur_id, mis_par)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (type_objet, objet_id) DO NOTHING",
    )
    .bind(&body.type_objet)
    .bind(body.objet_id)
    .bind(auteur_id)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    // +5 uniquement à la première mise en avant et hors auto‑attribution.
    // La clé d'idempotence rend tout rejeu inoffensif (pas de double crédit même
    // après retrait puis remise en avant — cohérent avec « pas de clawback »).
    if res.rows_affected() > 0 && auteur_id != admin.id {
        crate::services::engagement::attribuer(
            pool.get_ref(),
            auteur_id,
            "contribution_mise_en_avant",
            Some(&body.type_objet),
            Some(body.objet_id),
            &format!("mise_en_avant:{}:{}", body.type_objet, body.objet_id),
        )
        .await;
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "MISE_EN_AVANT", "engagement", "mise_en_avant",
        Some(body.objet_id), None,
        Some(serde_json::json!({ "type_objet": body.type_objet, "auteur_id": auteur_id })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MiseEnAvantEtat { mis_en_avant: true }),
        error: None,
    }))
}

/// DELETE /api/admin/engagement/mise-en-avant/{type_objet}/{objet_id}
/// Retire la mise en avant. Ne reprend PAS les points déjà attribués (pas de clawback).
pub async fn retirer_mise_en_avant(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let (type_objet, objet_id) = path.into_inner();

    sqlx::query("DELETE FROM engagement.mise_en_avant WHERE type_objet = $1 AND objet_id = $2")
        .bind(&type_objet)
        .bind(objet_id)
        .execute(pool.get_ref())
        .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "RETRAIT_MISE_EN_AVANT", "engagement", "mise_en_avant",
        Some(objet_id), None,
        Some(serde_json::json!({ "type_objet": type_objet })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MiseEnAvantEtat { mis_en_avant: false }),
        error: None,
    }))
}

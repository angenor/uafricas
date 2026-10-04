//! Administration du jeu (feature 013).
//! - `GET   /api/admin/jeu/modules`
//! - `PATCH /api/admin/jeu/modules/{code}`
//! - `GET   /api/admin/jeu/epreuves`
//! - `POST  /api/admin/jeu/epreuves`
//! - `GET   /api/admin/jeu/epreuves/{id}`
//! - `PUT   /api/admin/jeu/epreuves/{id}`
//! - `POST  /api/admin/jeu/epreuves/{id}/publier`
//! - `POST  /api/admin/jeu/epreuves/{id}/retirer`
//! - `GET   /api/admin/jeu/epreuves/formes`
//! - `POST  /api/admin/jeu/epreuves/derivation`
//! - `POST  /api/admin/jeu/epreuves/revue`
//! - `GET   /api/admin/jeu/signalements`
//! - `POST  /api/admin/jeu/signalements/{id}/decision`
//! - `GET    /api/admin/jeu/regles`
//! - `PUT    /api/admin/jeu/regles`
//! - `GET    /api/admin/jeu/saisons`
//! - `POST   /api/admin/jeu/saisons`
//! - `PUT    /api/admin/jeu/saisons/{id}`
//! - `POST   /api/admin/jeu/saisons/{id}/clore`
//! - `GET    /api/admin/jeu/joueurs`
//! - `POST   /api/admin/jeu/joueurs/{utilisateur_id}/annuler-gains`
//! - `GET    /api/admin/jeu/defis`
//! - `PUT    /api/admin/jeu/defis/{periodicite}/{date}`
//! - `DELETE /api/admin/jeu/defis/{periodicite}/{date}`
//!
//! Toute route exige la permission `jeu.gerer` ; toute mutation est auditée.
//! Contrat : `specs/013-activites-ludiques/contracts/api-admin.md`.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::jeu_classement::cloturer_saisons_echues;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::jeu::{
    valider_publication, BilanRevue, DecisionSignalementRequest, DefiAdmin, DefisQueryParams,
    DerivationRequest, EpreuveAdmin, EpreuveNettoyee, EpreuveRequest, EpreuveSignalee,
    EpreuvesQueryParams, ModifierModuleRequest, ModuleAdmin, ProgrammerDefiRequest, RefusRevue,
    RevueRequest, SaisonAdmin, SaisonRequest, SignalementAdmin, SignalementRow,
    SignalementsQueryParams, EPREUVE_ADMIN_SELECT,
};
use crate::models::admin::jeu::{AnnulerGainsRequest, BilanAnnulation, JoueurAdmin, JoueursQueryParams};
use crate::models::jeu::ReglesJeu;
use crate::models::notification;
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::jeu::{self as moteur, SERVABLE_SQL};
use crate::services::jeu_derivation::{self, FORMES};
use crate::services::{audit, engagement, image_validation};
use crate::verifier_permission;
use crate::ApiResponse;

/// Seuils de l'aide à la relecture : une épreuve servie assez souvent et que
/// presque tout le monde rate, ou que presque tout le monde réussit, mérite un
/// second regard. Ce sont des aides, pas des règles du jeu : elles ne sont donc
/// pas réglables.
const ANOMALIE_SERVIE_MIN: i32 = 30;
const ANOMALIE_TAUX_BAS: f64 = 0.15;
const ANOMALIE_TAUX_HAUT: f64 = 0.95;

async fn auditer(
    pool: &PgPool,
    req: &HttpRequest,
    admin_id: Uuid,
    action: &str,
    table: &str,
    record_id: Option<Uuid>,
    avant: Option<serde_json::Value>,
    apres: Option<serde_json::Value>,
) {
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool, Some(admin_id), action, "jeu", table, record_id, avant, apres,
        ip.as_deref(), ua.as_deref(),
    )
    .await;
}

async fn charger_epreuve(pool: &PgPool, id: Uuid) -> Result<EpreuveAdmin, ApiErreur> {
    sqlx::query_as::<_, EpreuveAdmin>(&format!("{EPREUVE_ADMIN_SELECT} WHERE e.id = $1"))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .map(EpreuveAdmin::avec_forme_attendue)
        .ok_or_else(|| ApiErreur::NonTrouve("Épreuve introuvable".into()))
}

/// Bascule PARESSEUSE vers `a_revoir` : passe en revue les épreuves dérivées
/// encore `jouable` dont la source a changé depuis la dérivation (FR-012).
///
/// C'est le seul endroit où cet état s'écrit, et il s'écrit à la lecture du
/// vivier par un administrateur, sans tâche de fond. Entre la modification de la
/// source et cet appel, l'épreuve est DÉJÀ non servie : le tirage compare
/// l'empreinte lui-même. Cette bascule ne fait que rendre le fait visible.
async fn basculer_a_revoir(pool: &PgPool) -> Result<u64, ApiErreur> {
    Ok(sqlx::query(
        "UPDATE jeu.epreuve e
            SET etat = 'a_revoir', updated_at = NOW()
           FROM jeu.v_source s
          WHERE e.etat = 'jouable' AND e.origine = 'derivee'
            AND s.type_source = e.type_source AND s.source_id = e.source_id
            AND s.visible AND s.empreinte <> e.source_empreinte",
    )
    .execute(pool)
    .await?
    .rows_affected())
}

// ─── Modules ─────────────────────────────────────────────────────────────────

/// GET /api/admin/jeu/modules : tous les modules, ouverts ou non, avec leur
/// décompte d'épreuves par état.
pub async fn lister_modules(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    basculer_a_revoir(pool.get_ref()).await?;

    let modules = sqlx::query_as::<_, ModuleAdmin>(
        "SELECT m.code, m.libelle, m.route, m.icone, m.ouvert, m.ordre,
                COUNT(e.id) FILTER (WHERE e.etat = 'candidate') AS candidates,
                COUNT(e.id) FILTER (WHERE e.etat = 'jouable')   AS jouables,
                COUNT(e.id) FILTER (WHERE e.etat = 'a_revoir')  AS a_revoir,
                COUNT(e.id) FILTER (WHERE e.etat = 'rejetee')   AS rejetees,
                COUNT(e.id) FILTER (WHERE e.etat = 'retiree')   AS retirees
           FROM jeu.module m
           LEFT JOIN jeu.epreuve e ON e.module_code = m.code
          GROUP BY m.code, m.libelle, m.route, m.icone, m.ouvert, m.ordre
          ORDER BY m.ordre",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(modules), error: None }))
}

/// PATCH /api/admin/jeu/modules/{code} : ouvre ou ferme les activités d'un module.
pub async fn modifier_module(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    body: web::Json<ModifierModuleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let code = path.into_inner();

    let avant: Option<bool> = sqlx::query_scalar("SELECT ouvert FROM jeu.module WHERE code = $1")
        .bind(&code)
        .fetch_optional(pool.get_ref())
        .await?;
    let Some(avant) = avant else {
        return Err(ApiErreur::NonTrouve("Module inconnu".into()));
    };

    sqlx::query("UPDATE jeu.module SET ouvert = $2, updated_at = NOW() WHERE code = $1")
        .bind(&code)
        .bind(body.ouvert)
        .execute(pool.get_ref())
        .await?;

    auditer(
        pool.get_ref(), &req, admin.id,
        if body.ouvert { "MODULE_OUVERT" } else { "MODULE_FERME" },
        "module", None,
        Some(serde_json::json!({ "code": code, "ouvert": avant })),
        Some(serde_json::json!({ "code": code, "ouvert": body.ouvert })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "code": code, "ouvert": body.ouvert })),
        error: None,
    }))
}

// ─── Épreuves ────────────────────────────────────────────────────────────────

/// GET /api/admin/jeu/epreuves : liste paginée et filtrable.
pub async fn lister_epreuves(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<EpreuvesQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    basculer_a_revoir(pool.get_ref()).await?;

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };
    let colonne = pagination.colonne_tri(
        &["created_at", "updated_at", "nombre_servie", "etat", "module_code", "difficulte"],
        "created_at",
    );
    let direction = pagination.direction_tri();

    let recherche = params
        .recherche
        .as_deref()
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(|r| format!("%{}%", r.to_lowercase()));
    let module = params.module.as_deref().filter(|m| !m.is_empty());
    let etat = params.etat.as_deref().filter(|e| !e.is_empty());
    let origine = params.origine.as_deref().filter(|o| !o.is_empty());
    let anomalie = params.anomalie.unwrap_or(false);

    // Les mêmes filtres servent au décompte et à la page : un seul fragment.
    let filtres = format!(
        " WHERE ($1::text IS NULL OR e.module_code = $1)
            AND ($2::text IS NULL OR e.etat = $2)
            AND ($3::text IS NULL OR e.origine = $3)
            AND ($4::smallint IS NULL OR e.difficulte = $4)
            AND ($5::uuid IS NULL OR e.pays_id = $5)
            AND ($6::text IS NULL OR LOWER(e.enonce) LIKE $6)
            AND (NOT $7 OR (e.nombre_servie >= {ANOMALIE_SERVIE_MIN}
                 AND (e.nombre_bonnes::float8 / e.nombre_servie < {ANOMALIE_TAUX_BAS}
                      OR e.nombre_bonnes::float8 / e.nombre_servie > {ANOMALIE_TAUX_HAUT})))"
    );

    let total: i64 =
        sqlx::query_scalar(&format!("SELECT COUNT(*) FROM jeu.epreuve e {filtres}"))
            .bind(module)
            .bind(etat)
            .bind(origine)
            .bind(params.difficulte)
            .bind(params.pays)
            .bind(&recherche)
            .bind(anomalie)
            .fetch_one(pool.get_ref())
            .await?;

    let epreuves = sqlx::query_as::<_, EpreuveAdmin>(&format!(
        "{EPREUVE_ADMIN_SELECT} {filtres}
          ORDER BY e.{colonne} {direction}, e.id
          LIMIT $8 OFFSET $9"
    ))
    .bind(module)
    .bind(etat)
    .bind(origine)
    .bind(params.difficulte)
    .bind(params.pays)
    .bind(&recherche)
    .bind(anomalie)
    .bind(pagination.par_page())
    .bind(pagination.offset())
    .fetch_all(pool.get_ref())
    .await?;
    let epreuves: Vec<EpreuveAdmin> =
        epreuves.into_iter().map(EpreuveAdmin::avec_forme_attendue).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::from_params(epreuves, total, &pagination)),
        error: None,
    }))
}

/// GET /api/admin/jeu/epreuves/{id}
pub async fn obtenir_epreuve(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let epreuve = charger_epreuve(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(epreuve), error: None }))
}

/// Contrôles qui demandent la base : le module existe, et le contenu référencé
/// est encore publié.
/// Carte : un pays désigné par son code ISO2 est résolu en identifiant.
async fn resoudre_pays_iso(pool: &PgPool, mut body: EpreuveRequest) -> Result<EpreuveRequest, ApiErreur> {
    if body.reponse_pays_id.is_none() {
        if let Some(iso) = body.reponse_pays_iso.as_deref().map(str::trim).filter(|i| !i.is_empty()) {
            let id: Option<Uuid> =
                sqlx::query_scalar("SELECT id FROM shared.pays WHERE LOWER(code_iso2) = LOWER($1)")
                    .bind(iso)
                    .fetch_optional(pool)
                    .await?;
            body.reponse_pays_id =
                Some(id.ok_or_else(|| ApiErreur::Validation("Carte : pays inconnu".into()))?);
        }
    }
    Ok(body)
}

async fn verifier_references(pool: &PgPool, epreuve: &EpreuveNettoyee) -> Result<(), ApiErreur> {
    let module_existe: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM jeu.module WHERE code = $1)")
            .bind(&epreuve.module)
            .fetch_one(pool)
            .await?;
    if !module_existe {
        return Err(ApiErreur::Validation("Module inconnu".into()));
    }

    // Carte : le pays attendu est l'un des 55 pays d'Afrique (feature 014).
    if let Some(pays) = epreuve.reponse_pays_id {
        let codes: Vec<String> = crate::constants::afripulse_pays_autorises::PAYS_AFRICAINS_ISO2
            .iter()
            .map(|c| c.to_lowercase())
            .collect();
        let africain: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM shared.pays WHERE id = $1 AND LOWER(code_iso2) = ANY($2))",
        )
        .bind(pays)
        .bind(&codes)
        .fetch_one(pool)
        .await?;
        if !africain {
            return Err(ApiErreur::Validation("Carte : le pays attendu doit être un pays d'Afrique".into()));
        }
    }

    if let (Some(type_source), Some(source_id)) = (&epreuve.type_source, epreuve.source_id) {
        let visible: Option<bool> = sqlx::query_scalar(
            "SELECT visible FROM jeu.v_source WHERE type_source = $1 AND source_id = $2",
        )
        .bind(type_source)
        .bind(source_id)
        .fetch_optional(pool)
        .await?;
        if visible != Some(true) {
            return Err(ApiErreur::Validation(
                "Le contenu référencé est introuvable ou n'est plus publié".into(),
            ));
        }
    }
    Ok(())
}

/// POST /api/admin/jeu/epreuves : saisie d'une épreuve. Elle naît `candidate`
/// (brouillon) ; son auteur la publie ensuite sans passer par la revue.
pub async fn creer_epreuve(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<EpreuveRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");

    let body = resoudre_pays_iso(pool.get_ref(), body.into_inner()).await?;
    let epreuve = body.nettoyer().map_err(ApiErreur::Validation)?;
    verifier_references(pool.get_ref(), &epreuve).await?;

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO jeu.epreuve
            (module_code, enonce, media_type, media_url, propositions, bonne_reponse,
             explication, difficulte, theme, pays_id, origine, type_source, source_id,
             etat, cree_par, type_reponse, solution, appariements, valeurs, reponse_pays_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'saisie', $11, $12, 'candidate', $13,
                 $14, $15, $16, $17, $18)
         RETURNING id",
    )
    .bind(&epreuve.module)
    .bind(&epreuve.enonce)
    .bind(&epreuve.media_type)
    .bind(&epreuve.media_url)
    .bind(&epreuve.propositions)
    .bind(epreuve.bonne_reponse)
    .bind(&epreuve.explication)
    .bind(epreuve.difficulte)
    .bind(&epreuve.theme)
    .bind(epreuve.pays_id)
    .bind(&epreuve.type_source)
    .bind(epreuve.source_id)
    .bind(admin.id)
    .bind(&epreuve.type_reponse)
    .bind(&epreuve.solution)
    .bind(&epreuve.appariements)
    .bind(&epreuve.valeurs)
    .bind(epreuve.reponse_pays_id)
    .fetch_one(pool.get_ref())
    .await?;

    let creee = charger_epreuve(pool.get_ref(), id).await?;
    auditer(
        pool.get_ref(), &req, admin.id, "CREATE", "epreuve", Some(id),
        None, serde_json::to_value(&creee).ok(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse { success: true, data: Some(creee), error: None }))
}

/// PUT /api/admin/jeu/epreuves/{id} : remplace le contenu d'une épreuve.
///
/// Permis sur une épreuve déjà jouable, à condition qu'elle reste publiable.
/// La correction ne touche à aucun gain déjà acquis (FR-084).
pub async fn modifier_epreuve(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<EpreuveRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();

    let avant = charger_epreuve(pool.get_ref(), id).await?;
    if matches!(avant.etat.as_str(), "retiree" | "rejetee") {
        return Err(ApiErreur::Conflit(
            "Une épreuve retirée ou rejetée ne se modifie plus".into(),
        ));
    }

    let body = resoudre_pays_iso(pool.get_ref(), body.into_inner()).await?;
    let epreuve = body.nettoyer().map_err(ApiErreur::Validation)?;
    verifier_references(pool.get_ref(), &epreuve).await?;
    if avant.etat == "jouable" {
        valider_publication(
            &epreuve.type_reponse, &epreuve.propositions, epreuve.explication.as_deref(), "conforme",
        )
            .map_err(ApiErreur::Validation)?;
    }

    sqlx::query(
        "UPDATE jeu.epreuve
            SET module_code = $2, enonce = $3, media_type = $4, media_url = $5,
                propositions = $6, bonne_reponse = $7, explication = $8, difficulte = $9,
                theme = $10, pays_id = $11, type_reponse = $12, solution = $13,
                appariements = $14, valeurs = $15, reponse_pays_id = $16, updated_at = NOW()
          WHERE id = $1",
    )
    .bind(id)
    .bind(&epreuve.module)
    .bind(&epreuve.enonce)
    .bind(&epreuve.media_type)
    .bind(&epreuve.media_url)
    .bind(&epreuve.propositions)
    .bind(epreuve.bonne_reponse)
    .bind(&epreuve.explication)
    .bind(epreuve.difficulte)
    .bind(&epreuve.theme)
    .bind(epreuve.pays_id)
    .bind(&epreuve.type_reponse)
    .bind(&epreuve.solution)
    .bind(&epreuve.appariements)
    .bind(&epreuve.valeurs)
    .bind(epreuve.reponse_pays_id)
    .execute(pool.get_ref())
    .await?;

    // La référence à un contenu ne se change que sur une épreuve saisie : celle
    // d'une épreuve dérivée est sa raison d'être.
    if avant.origine == "saisie" {
        sqlx::query("UPDATE jeu.epreuve SET type_source = $2, source_id = $3 WHERE id = $1")
            .bind(id)
            .bind(&epreuve.type_source)
            .bind(epreuve.source_id)
            .execute(pool.get_ref())
            .await?;
    }

    let apres = charger_epreuve(pool.get_ref(), id).await?;
    auditer(
        pool.get_ref(), &req, admin.id, "UPDATE", "epreuve", Some(id),
        serde_json::to_value(&avant).ok(), serde_json::to_value(&apres).ok(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

/// POST /api/admin/jeu/epreuves/{id}/publier : `candidate` ou `a_revoir` →
/// `jouable`. La validation précède l'écriture et nomme le manque.
pub async fn publier_epreuve(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();

    let avant = charger_epreuve(pool.get_ref(), id).await?;
    if !matches!(avant.etat.as_str(), "candidate" | "a_revoir") {
        return Err(ApiErreur::Conflit(format!(
            "Une épreuve à l'état « {} » ne peut pas être publiée",
            avant.etat
        )));
    }
    valider_publication(
        &avant.type_reponse, &avant.propositions, avant.explication.as_deref(), &avant.source_etat,
    )
        .map_err(ApiErreur::Validation)?;

    // Pour une épreuve dérivée, publier (après revue) vaut acceptation de la
    // source telle qu'elle est aujourd'hui : on réenregistre son empreinte.
    let touchees = sqlx::query(
        "UPDATE jeu.epreuve e
            SET etat = 'jouable', valide_par = $2, valide_at = NOW(), updated_at = NOW(),
                source_empreinte = COALESCE(
                    (SELECT s.empreinte FROM jeu.v_source s
                      WHERE s.type_source = e.type_source AND s.source_id = e.source_id),
                    e.source_empreinte)
          WHERE e.id = $1 AND e.etat IN ('candidate', 'a_revoir')",
    )
    .bind(id)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?
    .rows_affected();
    if touchees == 0 {
        return Err(ApiErreur::Conflit("L'épreuve a changé d'état entre-temps".into()));
    }

    auditer(
        pool.get_ref(), &req, admin.id, "PUBLICATION", "epreuve", Some(id),
        Some(serde_json::json!({ "etat": avant.etat })),
        Some(serde_json::json!({ "etat": "jouable" })),
    )
    .await;

    let apres = charger_epreuve(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

/// POST /api/admin/jeu/epreuves/{id}/retirer : `jouable` → `retiree`. Les
/// réponses déjà données et les scores déjà acquis restent (FR-013).
pub async fn retirer_epreuve(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();

    let touchees = sqlx::query(
        "UPDATE jeu.epreuve SET etat = 'retiree', updated_at = NOW()
          WHERE id = $1 AND etat = 'jouable'",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?
    .rows_affected();

    if touchees == 0 {
        // Distinguer « n'existe pas » de « n'est pas jouable » : 404 ou 409.
        let actuelle = charger_epreuve(pool.get_ref(), id).await?;
        return Err(ApiErreur::Conflit(format!(
            "Seule une épreuve jouable peut être retirée (état actuel : « {} »)",
            actuelle.etat
        )));
    }

    auditer(
        pool.get_ref(), &req, admin.id, "RETRAIT", "epreuve", Some(id),
        Some(serde_json::json!({ "etat": "jouable" })),
        Some(serde_json::json!({ "etat": "retiree" })),
    )
    .await;

    let apres = charger_epreuve(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

// ─── Dérivation et revue ─────────────────────────────────────────────────────

/// GET /api/admin/jeu/epreuves/formes : le catalogue des formes de dérivation,
/// avec ce que chacune pourrait encore produire. Un module sans forme
/// (Afrolang) n'y a aucune ligne : l'écran le dit, il ne propose pas de bouton
/// qui ne ferait rien.
pub async fn lister_formes(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let formes = jeu_derivation::etat_des_formes(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(formes), error: None }))
}

/// POST /api/admin/jeu/epreuves/derivation : propose des candidates à partir du
/// contenu publié d'un module. Rejouable sans doublon ; une seule ligne d'audit
/// pour l'opération, avec son décompte.
pub async fn deriver(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    body: web::Json<DerivationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");

    let module = body.module.trim();
    let formes: Vec<_> = FORMES
        .iter()
        .filter(|f| f.module == module)
        .filter(|f| body.formes.as_ref().is_none_or(|codes| codes.iter().any(|c| c == f.code)))
        .collect();
    if formes.is_empty() {
        return Err(ApiErreur::Validation(
            "Ce module n'a aucune forme de dérivation : ses épreuves se saisissent".into(),
        ));
    }

    let mut par_forme = Vec::with_capacity(formes.len());
    for forme in formes {
        par_forme.push(jeu_derivation::deriver_forme(pool.get_ref(), forme, upload_dir.get_ref()).await?);
    }

    let creees: i64 = par_forme.iter().map(|b| b.creees).sum();
    let deja_proposees: i64 = par_forme.iter().map(|b| b.deja_proposees).sum();
    let sans_distracteurs: i64 = par_forme.iter().map(|b| b.sans_distracteurs).sum();
    let bilan = serde_json::json!({
        "module": module,
        "creees": creees,
        "deja_proposees": deja_proposees,
        "sans_distracteurs": sans_distracteurs,
        "par_forme": par_forme,
    });

    auditer(pool.get_ref(), &req, admin.id, "DERIVATION", "epreuve", None, None, Some(bilan.clone()))
        .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(bilan), error: None }))
}

/// POST /api/admin/jeu/epreuves/revue : accepte ou rejette un lot.
///
/// À l'acceptation, chaque épreuve passe les mêmes validations qu'une
/// publication ; celles qui échouent sont renvoyées avec leur raison, les autres
/// sont acceptées. Une ligne d'audit pour le lot.
pub async fn revue(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<RevueRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");

    if body.ids.is_empty() {
        return Err(ApiErreur::Validation("Aucune épreuve sélectionnée".into()));
    }
    let accepter = match body.decision.as_str() {
        "accepter" => true,
        "rejeter" => false,
        _ => return Err(ApiErreur::Validation("Décision inconnue".into())),
    };
    let motif = body.motif.as_deref().map(str::trim).filter(|m| !m.is_empty());
    if !accepter && motif.is_none() {
        return Err(ApiErreur::Validation("Le motif du rejet est obligatoire".into()));
    }

    let mut bilan = BilanRevue { acceptees: 0, rejetees: 0, refus: Vec::new() };

    for &id in &body.ids {
        let Ok(epreuve) = charger_epreuve(pool.get_ref(), id).await else {
            bilan.refus.push(RefusRevue { id, raison: "Épreuve introuvable".into() });
            continue;
        };
        if !matches!(epreuve.etat.as_str(), "candidate" | "a_revoir") {
            bilan.refus.push(RefusRevue {
                id,
                raison: format!("État « {} » : rien à revoir", epreuve.etat),
            });
            continue;
        }

        if accepter {
            if let Err(raison) = valider_publication(
                &epreuve.type_reponse,
                &epreuve.propositions,
                epreuve.explication.as_deref(),
                &epreuve.source_etat,
            ) {
                bilan.refus.push(RefusRevue { id, raison });
                continue;
            }
            // Accepter vaut acceptation de la source telle qu'elle est
            // aujourd'hui : on réenregistre son empreinte.
            sqlx::query(
                "UPDATE jeu.epreuve e
                    SET etat = 'jouable', valide_par = $2, valide_at = NOW(), updated_at = NOW(),
                        source_empreinte = COALESCE(
                            (SELECT s.empreinte FROM jeu.v_source s
                              WHERE s.type_source = e.type_source AND s.source_id = e.source_id),
                            e.source_empreinte)
                  WHERE e.id = $1 AND e.etat IN ('candidate', 'a_revoir')",
            )
            .bind(id)
            .bind(admin.id)
            .execute(pool.get_ref())
            .await?;
            bilan.acceptees += 1;
        } else {
            sqlx::query(
                "UPDATE jeu.epreuve
                    SET etat = 'rejetee', motif_rejet = $2, valide_par = $3, valide_at = NOW(),
                        updated_at = NOW()
                  WHERE id = $1 AND etat IN ('candidate', 'a_revoir')",
            )
            .bind(id)
            .bind(motif)
            .bind(admin.id)
            .execute(pool.get_ref())
            .await?;
            bilan.rejetees += 1;
        }
    }

    auditer(
        pool.get_ref(), &req, admin.id,
        if accepter { "REVUE_ACCEPTATION" } else { "REVUE_REJET" },
        "epreuve", None, None,
        Some(serde_json::json!({
            "ids": body.ids, "motif": motif,
            "acceptees": bilan.acceptees, "rejetees": bilan.rejetees,
            "refus": bilan.refus.len(),
        })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(bilan), error: None }))
}

// ─── Signalements d'épreuve ──────────────────────────────────────────────────

/// GET /api/admin/jeu/signalements : la file, regroupée par épreuve.
pub async fn lister_signalements(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<SignalementsQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");

    let etat = params.etat.as_deref().filter(|e| !e.is_empty()).unwrap_or("en_attente");

    // Auteur supprimé : on garde le signalement, on efface le nom.
    let lignes = sqlx::query_as::<_, SignalementRow>(
        "SELECT sg.id, sg.epreuve_id, sg.motif, sg.commentaire, sg.etat, sg.created_at,
                u.id AS auteur_id,
                CASE WHEN u.deleted_at IS NULL THEN u.nom ELSE 'Membre' END AS auteur_nom,
                CASE WHEN u.deleted_at IS NULL THEN u.prenom ELSE '' END AS auteur_prenom,
                e.enonce, e.module_code, e.etat AS epreuve_etat, e.propositions, e.bonne_reponse, e.type_reponse
           FROM jeu.signalement_epreuve sg
           JOIN jeu.epreuve e ON e.id = sg.epreuve_id
           JOIN iam.utilisateur u ON u.id = sg.utilisateur_id
          WHERE sg.etat = $1
          ORDER BY e.id, sg.created_at",
    )
    .bind(etat)
    .fetch_all(pool.get_ref())
    .await?;

    let mut file: Vec<EpreuveSignalee> = Vec::new();
    for l in lignes {
        let signalement = SignalementAdmin {
            id: l.id,
            motif: l.motif,
            commentaire: l.commentaire,
            etat: l.etat,
            created_at: l.created_at,
            auteur_id: l.auteur_id,
            auteur: format!("{} {}", l.auteur_prenom, l.auteur_nom).trim().to_string(),
        };
        match file.last_mut() {
            Some(derniere) if derniere.epreuve_id == l.epreuve_id => {
                derniere.signalements.push(signalement)
            }
            _ => file.push(EpreuveSignalee {
                epreuve_id: l.epreuve_id,
                enonce: l.enonce,
                module_code: l.module_code,
                epreuve_etat: l.epreuve_etat,
                propositions: l.propositions,
                bonne_reponse: l.bonne_reponse,
                type_reponse: l.type_reponse,
                signalements: vec![signalement],
            }),
        }
    }
    // Les épreuves les plus signalées d'abord.
    file.sort_by(|a, b| b.signalements.len().cmp(&a.signalements.len()));

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(file), error: None }))
}

/// POST /api/admin/jeu/signalements/{id}/decision
///
/// `confirmer` : le signalement était fondé. Tous les signalements en attente de
/// la même épreuve sont confirmés d'un coup (c'est le même constat), chaque
/// signaleur gagne de la réputation et reçoit une notification. `classer` : ce
/// seul signalement est écarté.
///
/// Aucun gain déjà acquis sur l'épreuve n'est repris (FR-084).
pub async fn decider_signalement(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<DecisionSignalementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();

    let confirmer = match body.decision.as_str() {
        "confirmer" => true,
        "classer" => false,
        _ => return Err(ApiErreur::Validation("Décision inconnue".into())),
    };
    let retirer = confirmer && body.retirer_epreuve.unwrap_or(false);

    let mut tx = pool.begin().await?;

    let cible: Option<(Uuid, String)> = sqlx::query_as(
        "SELECT epreuve_id, etat FROM jeu.signalement_epreuve WHERE id = $1 FOR UPDATE",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    let Some((epreuve_id, etat)) = cible else {
        return Err(ApiErreur::NonTrouve("Signalement introuvable".into()));
    };
    if etat != "en_attente" {
        return Err(ApiErreur::Conflit("Ce signalement a déjà été traité".into()));
    }

    // (signalement, signaleur) effectivement traités par cette décision.
    let traites: Vec<(Uuid, Uuid)> = if confirmer {
        sqlx::query_as(
            "UPDATE jeu.signalement_epreuve
                SET etat = 'confirme', decision_par = $2, decision_at = NOW()
              WHERE epreuve_id = $1 AND etat = 'en_attente'
              RETURNING id, utilisateur_id",
        )
        .bind(epreuve_id)
        .bind(admin.id)
        .fetch_all(&mut *tx)
        .await?
    } else {
        sqlx::query_as(
            "UPDATE jeu.signalement_epreuve
                SET etat = 'classe', decision_par = $2, decision_at = NOW()
              WHERE id = $1 AND etat = 'en_attente'
              RETURNING id, utilisateur_id",
        )
        .bind(id)
        .bind(admin.id)
        .fetch_all(&mut *tx)
        .await?
    };

    let mut epreuve_retiree = false;
    if retirer {
        epreuve_retiree = sqlx::query(
            "UPDATE jeu.epreuve SET etat = 'retiree', updated_at = NOW()
              WHERE id = $1 AND etat IN ('jouable', 'a_revoir')",
        )
        .bind(epreuve_id)
        .execute(&mut *tx)
        .await?
        .rows_affected()
            == 1;
    }

    tx.commit().await?;

    // Après le COMMIT : réputation et notifications ne doivent jamais faire
    // échouer la décision.
    for (signalement_id, signaleur_id) in &traites {
        if confirmer {
            engagement::attribuer(
                pool.get_ref(),
                *signaleur_id,
                "jeu_signalement_confirme",
                Some("signalement_epreuve"),
                Some(*signalement_id),
                &format!("jeu:signalement:{signalement_id}"),
            )
            .await;
        }
        notification::creer_notification(
            pool.get_ref(),
            *signaleur_id,
            notification::jeu::SIGNALEMENT_TRAITE,
            if confirmer {
                "Votre signalement d'épreuve a été confirmé. Merci de votre vigilance."
            } else {
                "Votre signalement d'épreuve a été examiné : l'épreuve est maintenue."
            },
            Some(notification::jeu::LIEN_ESPACE),
        )
        .await;
    }

    auditer(
        pool.get_ref(), &req, admin.id,
        if confirmer { "SIGNALEMENT_CONFIRME" } else { "SIGNALEMENT_CLASSE" },
        "signalement_epreuve", Some(id),
        Some(serde_json::json!({ "etat": "en_attente", "epreuve_id": epreuve_id })),
        Some(serde_json::json!({
            "signalements_traites": traites.len(), "epreuve_retiree": epreuve_retiree,
        })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "signalements_traites": traites.len(), "epreuve_retiree": epreuve_retiree,
        })),
        error: None,
    }))
}

// ─── Défis ───────────────────────────────────────────────────────────────────

/// GET /api/admin/jeu/defis : défis passés avec leur participation, défis à
/// venir programmés. Les 60 plus récents.
pub async fn lister_defis(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<DefisQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");

    let periodicite = params.periodicite.as_deref().filter(|p| !p.is_empty());

    let defis = sqlx::query_as::<_, DefiAdmin>(
        "SELECT d.id, d.periodicite, d.periode_debut, d.module_code, d.titre,
                cardinality(d.epreuve_ids) AS nombre_epreuves, d.origine,
                COUNT(p.id) AS participants,
                COUNT(p.id) FILTER (WHERE p.etat = 'terminee') AS termines,
                d.periode_debut > CASE WHEN d.periodicite = 'semaine'
                    THEN date_trunc('week', NOW() AT TIME ZONE 'UTC')::date
                    ELSE (NOW() AT TIME ZONE 'UTC')::date END AS a_venir
           FROM jeu.defi d
           LEFT JOIN jeu.partie p ON p.defi_id = d.id
          WHERE ($1::text IS NULL OR d.periodicite = $1)
          GROUP BY d.id
          ORDER BY d.periode_debut DESC, d.periodicite
          LIMIT 60",
    )
    .bind(periodicite)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(defis), error: None }))
}

/// Une période n'est programmable que si elle n'a pas commencé : le défi en
/// cours est figé, des membres l'ont peut-être déjà joué.
async fn verifier_periode_a_venir(
    pool: &PgPool,
    periodicite: &str,
    date: chrono::NaiveDate,
) -> Result<(), ApiErreur> {
    use chrono::Datelike;

    if !matches!(periodicite, "jour" | "semaine") {
        return Err(ApiErreur::Validation("La périodicité est « jour » ou « semaine »".into()));
    }
    if periodicite == "semaine" && date.weekday() != chrono::Weekday::Mon {
        return Err(ApiErreur::Validation(
            "Un défi de la semaine commence un lundi".into(),
        ));
    }
    let courante = moteur::periode_courante(pool, periodicite).await?;
    if date <= courante {
        return Err(ApiErreur::Validation(
            "Seule une période à venir peut être programmée : le défi en cours est figé".into(),
        ));
    }
    Ok(())
}

/// PUT /api/admin/jeu/defis/{periodicite}/{date} : programme le défi d'une
/// période à venir. La série compte exactement la taille réglée, et toutes ses
/// épreuves doivent être servables.
pub async fn programmer_defi(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String, chrono::NaiveDate)>,
    body: web::Json<ProgrammerDefiRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let (periodicite, date) = path.into_inner();
    verifier_periode_a_venir(pool.get_ref(), &periodicite, date).await?;

    let regles = moteur::charger_regles(pool.get_ref()).await?;
    let taille = usize::from(
        if periodicite == "semaine" { regles.taille_defi_semaine } else { regles.taille_defi_jour }
            as u16,
    );

    let mut distinctes = body.epreuve_ids.clone();
    distinctes.sort();
    distinctes.dedup();
    if distinctes.len() != body.epreuve_ids.len() {
        return Err(ApiErreur::Validation("Une épreuve figure deux fois dans la série".into()));
    }
    if body.epreuve_ids.len() != taille {
        return Err(ApiErreur::Validation(format!(
            "Ce défi compte exactement {taille} épreuves ({} fournies)",
            body.epreuve_ids.len()
        )));
    }

    let servables: i64 =
        sqlx::query_scalar(&format!("SELECT COUNT(*) {SERVABLE_SQL} AND e.id = ANY($1)"))
            .bind(&body.epreuve_ids)
            .fetch_one(pool.get_ref())
            .await?;
    if servables as usize != taille {
        return Err(ApiErreur::Validation(
            "Certaines épreuves ne sont pas jouables (retirées, ou leur source n'est plus publiée)"
                .into(),
        ));
    }

    let module = body.module.as_deref().map(str::trim).filter(|m| !m.is_empty());
    let titre = body.titre.as_deref().map(str::trim).filter(|t| !t.is_empty());

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO jeu.defi
            (periodicite, periode_debut, module_code, titre, epreuve_ids, origine, cree_par)
         VALUES ($1, $2, $3, $4, $5, 'programme', $6)
         ON CONFLICT (periodicite, periode_debut) DO UPDATE
            SET module_code = EXCLUDED.module_code, titre = EXCLUDED.titre,
                epreuve_ids = EXCLUDED.epreuve_ids, origine = 'programme',
                cree_par = EXCLUDED.cree_par
         RETURNING id",
    )
    .bind(&periodicite)
    .bind(date)
    .bind(module)
    .bind(titre)
    .bind(&body.epreuve_ids)
    .bind(admin.id)
    .fetch_one(pool.get_ref())
    .await?;

    auditer(
        pool.get_ref(), &req, admin.id, "DEFI_PROGRAMME", "defi", Some(id), None,
        Some(serde_json::json!({
            "periodicite": periodicite, "periode_debut": date, "titre": titre,
            "module": module, "epreuve_ids": body.epreuve_ids,
        })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/jeu/defis/{periodicite}/{date} : retire la programmation
/// d'une période à venir. Le défi de cette période redeviendra automatique.
pub async fn deprogrammer_defi(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String, chrono::NaiveDate)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let (periodicite, date) = path.into_inner();
    verifier_periode_a_venir(pool.get_ref(), &periodicite, date).await?;

    let supprime: Option<Uuid> = sqlx::query_scalar(
        "DELETE FROM jeu.defi WHERE periodicite = $1 AND periode_debut = $2 RETURNING id",
    )
    .bind(&periodicite)
    .bind(date)
    .fetch_optional(pool.get_ref())
    .await?;
    let Some(id) = supprime else {
        return Err(ApiErreur::NonTrouve("Aucun défi programmé pour cette période".into()));
    };

    auditer(
        pool.get_ref(), &req, admin.id, "DEFI_DEPROGRAMME", "defi", Some(id),
        Some(serde_json::json!({ "periodicite": periodicite, "periode_debut": date })), None,
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ─── Règles du jeu ───────────────────────────────────────────────────────────

/// GET /api/admin/jeu/regles : le singleton des valeurs réglables.
pub async fn obtenir_regles(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let regles = moteur::charger_regles(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(regles), error: None }))
}

/// Les bornes sont celles des CHECK de `jeu.regles` : rappelées ici pour un
/// message lisible, la base restant le dernier filet.
fn verifier_bornes(r: &ReglesJeu) -> Result<(), String> {
    let entre = |nom: &str, v: i16, min: i16, max: i16| {
        if (min..=max).contains(&v) { Ok(()) } else { Err(format!("{nom} : entre {min} et {max}")) }
    };
    entre("Taille d'une partie", r.taille_partie, 3, 30)?;
    entre("Taille du défi du jour", r.taille_defi_jour, 3, 30)?;
    entre("Taille du défi de la semaine", r.taille_defi_semaine, 3, 30)?;
    entre("Taille d'un duel", r.taille_duel, 3, 30)?;
    entre("Temps par épreuve (secondes)", r.temps_epreuve_s, 5, 120)?;
    entre("Score d'une épreuve facile", r.score_facile, 1, i16::MAX)?;
    entre("Score d'une épreuve moyenne", r.score_moyen, 1, i16::MAX)?;
    entre("Score d'une épreuve difficile", r.score_difficile, 1, i16::MAX)?;
    entre("Prime du défi du jour", r.prime_defi_jour, 0, i16::MAX)?;
    entre("Prime du défi de la semaine", r.prime_defi_semaine, 0, i16::MAX)?;
    entre("Prime de victoire en duel", r.prime_duel_victoire, 0, i16::MAX)?;
    entre("Prime de duel nul", r.prime_duel_nul, 0, i16::MAX)?;
    entre("Délai d'un duel (heures)", r.delai_duel_h, 1, 168)?;
    entre("Délai d'un duel direct (minutes)", r.delai_direct_min, 1, 60)?;
    entre("Délai de grâce en direct (secondes)", r.grace_direct_s, 10, 300)?;
    entre("Pause de révélation (secondes)", r.pause_revelation_s, 2, 15)?;
    entre("Duels comptés par paire et par jour", r.duels_comptes_par_paire_jour, 0, i16::MAX)?;
    entre("Duels comptés par membre et par jour", r.duels_comptes_par_membre_jour, 0, i16::MAX)?;
    entre("Joueurs comptés par pays", r.joueurs_par_pays, 1, 100)?;
    entre("Temps ajouté à l'ordre et aux paires (secondes)", r.majoration_ordre_paires_s, 0, 60)?;
    entre("Prime de participation à un concours", r.prime_concours_participation, 0, i16::MAX)?;
    let podium = &r.prime_concours_podium;
    if podium.len() != 3 || podium[2] < 0 || podium[0] < podium[1] || podium[1] < podium[2] {
        return Err("Primes du podium d'un concours : trois montants positifs, du premier au troisième, sans augmenter".into());
    }
    Ok(())
}

/// PUT /api/admin/jeu/regles : remplacement intégral. Un changement ne vaut que
/// pour l'avenir : les séries déjà composées sont figées et les gains déjà
/// écrits ne sont pas recalculés (FR-077).
pub async fn modifier_regles(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<ReglesJeu>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    verifier_bornes(&body).map_err(ApiErreur::Validation)?;

    let avant = moteur::charger_regles(pool.get_ref()).await?;

    sqlx::query(
        "UPDATE jeu.regles SET
            taille_partie = $1, taille_defi_jour = $2, taille_defi_semaine = $3, taille_duel = $4,
            temps_epreuve_s = $5, score_facile = $6, score_moyen = $7, score_difficile = $8,
            prime_defi_jour = $9, prime_defi_semaine = $10, prime_duel_victoire = $11,
            prime_duel_nul = $12, delai_duel_h = $13, delai_direct_min = $14, grace_direct_s = $15,
            pause_revelation_s = $16, duels_comptes_par_paire_jour = $17,
            duels_comptes_par_membre_jour = $18, joueurs_par_pays = $19,
            majoration_ordre_paires_s = $20, prime_concours_participation = $21,
            prime_concours_podium = $22, updated_at = NOW()
          WHERE id",
    )
    .bind(body.taille_partie)
    .bind(body.taille_defi_jour)
    .bind(body.taille_defi_semaine)
    .bind(body.taille_duel)
    .bind(body.temps_epreuve_s)
    .bind(body.score_facile)
    .bind(body.score_moyen)
    .bind(body.score_difficile)
    .bind(body.prime_defi_jour)
    .bind(body.prime_defi_semaine)
    .bind(body.prime_duel_victoire)
    .bind(body.prime_duel_nul)
    .bind(body.delai_duel_h)
    .bind(body.delai_direct_min)
    .bind(body.grace_direct_s)
    .bind(body.pause_revelation_s)
    .bind(body.duels_comptes_par_paire_jour)
    .bind(body.duels_comptes_par_membre_jour)
    .bind(body.joueurs_par_pays)
    .bind(body.majoration_ordre_paires_s)
    .bind(body.prime_concours_participation)
    .bind(&body.prime_concours_podium)
    .execute(pool.get_ref())
    .await?;

    let apres = moteur::charger_regles(pool.get_ref()).await?;
    auditer(
        pool.get_ref(), &req, admin.id, "REGLES_MODIFIEES", "regles", None,
        serde_json::to_value(&avant).ok(), serde_json::to_value(&apres).ok(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

// ─── Saisons du Championship ─────────────────────────────────────────────────

const SAISON_ADMIN_SELECT: &str = "
    SELECT s.id, s.nom, s.debut_at, s.fin_at, s.cloturee_at,
           CASE WHEN NOW() < s.debut_at THEN 'a_venir'
                WHEN NOW() < s.fin_at THEN 'en_cours'
                ELSE 'close' END AS etat,
           (SELECT COUNT(DISTINCT ss.utilisateur_id) FROM jeu.score_saison ss
             WHERE ss.saison_id = s.id AND ss.score > 0) AS joueurs
      FROM jeu.saison s";

async fn charger_saison(pool: &PgPool, id: Uuid) -> Result<SaisonAdmin, ApiErreur> {
    sqlx::query_as::<_, SaisonAdmin>(&format!("{SAISON_ADMIN_SELECT} WHERE s.id = $1"))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Saison introuvable".into()))
}

/// Traduit la violation de la contrainte d'exclusion en un 409 lisible : c'est
/// la base qui interdit le chevauchement, l'API ne fait que le dire.
fn traduire_erreur_saison(e: sqlx::Error) -> ApiErreur {
    if let sqlx::Error::Database(db) = &e {
        match db.constraint() {
            Some("ex_saison_chevauchement") => {
                return ApiErreur::Conflit(
                    "Cette période chevauche une autre saison : deux saisons ne peuvent pas se recouvrir"
                        .into(),
                )
            }
            Some("ck_saison_dates") => {
                return ApiErreur::Validation("La fin doit être postérieure au début".into())
            }
            _ => {}
        }
    }
    e.into()
}

fn nom_de_saison(nom: &str) -> Result<&str, ApiErreur> {
    let nom = nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de la saison est obligatoire".into()));
    }
    Ok(nom)
}

/// GET /api/admin/jeu/saisons
pub async fn lister_saisons(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    cloturer_saisons_echues(pool.get_ref()).await?;

    let saisons = sqlx::query_as::<_, SaisonAdmin>(&format!(
        "{SAISON_ADMIN_SELECT} ORDER BY s.debut_at DESC"
    ))
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(saisons), error: None }))
}

/// POST /api/admin/jeu/saisons
///
/// Un début dans le passé est ramené à maintenant : les gains acquis avant la
/// création ne portent pas cette saison, la faire « commencer hier » afficherait
/// une période pendant laquelle rien n'a compté.
pub async fn creer_saison(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<SaisonRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let nom = nom_de_saison(&body.nom)?;

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO jeu.saison (nom, debut_at, fin_at, cree_par)
         VALUES ($1, GREATEST($2, NOW()), $3, $4)
         RETURNING id",
    )
    .bind(nom)
    .bind(body.debut_at)
    .bind(body.fin_at)
    .bind(admin.id)
    .fetch_one(pool.get_ref())
    .await
    .map_err(traduire_erreur_saison)?;

    let creee = charger_saison(pool.get_ref(), id).await?;
    auditer(
        pool.get_ref(), &req, admin.id, "CREATE", "saison", Some(id),
        None, serde_json::to_value(&creee).ok(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse { success: true, data: Some(creee), error: None }))
}

/// PUT /api/admin/jeu/saisons/{id}
///
/// Le début d'une saison commencée n'est plus modifiable (des gains la portent
/// déjà), ni rien d'une saison close.
pub async fn modifier_saison(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<SaisonRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();
    let nom = nom_de_saison(&body.nom)?;

    let avant = charger_saison(pool.get_ref(), id).await?;
    match avant.etat.as_str() {
        "close" => {
            return Err(ApiErreur::Validation(
                "Une saison close ne se modifie plus : son classement est archivé".into(),
            ))
        }
        "en_cours" if body.debut_at != avant.debut_at => {
            return Err(ApiErreur::Validation(
                "Le début d'une saison commencée ne peut plus être modifié".into(),
            ))
        }
        "en_cours" if body.fin_at <= chrono::Utc::now() => {
            return Err(ApiErreur::Validation(
                "Pour terminer la saison maintenant, utilisez « Clore »".into(),
            ))
        }
        _ => {}
    }

    // Saison à venir : même règle qu'à la création pour un début dans le passé.
    sqlx::query(
        "UPDATE jeu.saison
            SET nom = $2,
                debut_at = CASE WHEN debut_at <= NOW() THEN debut_at ELSE GREATEST($3, NOW()) END,
                fin_at = $4, updated_at = NOW()
          WHERE id = $1",
    )
    .bind(id)
    .bind(nom)
    .bind(body.debut_at)
    .bind(body.fin_at)
    .execute(pool.get_ref())
    .await
    .map_err(traduire_erreur_saison)?;

    let apres = charger_saison(pool.get_ref(), id).await?;
    auditer(
        pool.get_ref(), &req, admin.id, "UPDATE", "saison", Some(id),
        serde_json::to_value(&avant).ok(), serde_json::to_value(&apres).ok(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

/// POST /api/admin/jeu/saisons/{id}/clore : termine la saison maintenant. Le
/// classement est aussitôt archivé et les distinctions de podium attribuées.
pub async fn clore_saison(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();

    let avant = charger_saison(pool.get_ref(), id).await?;
    if avant.etat != "en_cours" {
        return Err(ApiErreur::Conflit("Seule la saison en cours peut être close".into()));
    }

    sqlx::query("UPDATE jeu.saison SET fin_at = NOW(), updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;
    cloturer_saisons_echues(pool.get_ref()).await?;

    let apres = charger_saison(pool.get_ref(), id).await?;
    auditer(
        pool.get_ref(), &req, admin.id, "SAISON_CLOSE", "saison", Some(id),
        serde_json::to_value(&avant).ok(), serde_json::to_value(&apres).ok(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

// ─── Triche : annulation des gains ───────────────────────────────────────────

/// GET /api/admin/jeu/joueurs : retrouver un joueur par son nom ou son e-mail,
/// avec son score, avant de sanctionner. Les 30 plus actifs si rien n'est cherché.
pub async fn lister_joueurs(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<JoueursQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let recherche = params
        .recherche
        .as_deref()
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(|r| format!("%{}%", r.to_lowercase()));

    let joueurs = sqlx::query_as::<_, JoueurAdmin>(
        "SELECT u.id AS utilisateur_id, u.nom, u.prenom, u.email::text AS email,
                j.score_total,
                COUNT(g.id) FILTER (WHERE g.annule_at IS NULL) AS gains,
                COUNT(g.id) FILTER (WHERE g.annule_at IS NOT NULL) AS gains_annules,
                MAX(g.created_at) AS dernier_gain_at
           FROM jeu.joueur j
           JOIN iam.utilisateur u ON u.id = j.utilisateur_id
           LEFT JOIN jeu.gain g ON g.utilisateur_id = j.utilisateur_id
          WHERE $1::text IS NULL
             OR LOWER(u.nom || ' ' || u.prenom) LIKE $1
             OR LOWER(u.prenom || ' ' || u.nom) LIKE $1
             OR LOWER(u.email::text) LIKE $1
          GROUP BY u.id, u.nom, u.prenom, u.email, j.score_total
          ORDER BY j.score_total DESC
          LIMIT 30",
    )
    .bind(recherche)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(joueurs), error: None }))
}

/// POST /api/admin/jeu/joueurs/{utilisateur_id}/annuler-gains : sanctionne une
/// triche établie (FR-031, FR-072).
///
/// Dans une transaction : les gains visés sont MARQUÉS annulés (le journal ne
/// perd rien, on sait toujours ce qui a été retiré, par qui et pourquoi), puis
/// les agrégats du membre sont RECALCULÉS depuis les gains restants. On ne
/// soustrait pas : soustraire dériverait au moindre écart, recalculer depuis le
/// journal ne peut pas.
///
/// Après le COMMIT : retrait de réputation éventuel, notification, audit.
/// L'opération n'a pas d'inverse dans cette version.
pub async fn annuler_gains(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AnnulerGainsRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let utilisateur_id = path.into_inner();

    let motif = body.motif.trim();
    if motif.is_empty() {
        return Err(ApiErreur::Validation("Le motif de l'annulation est obligatoire".into()));
    }
    let retrait = body.retrait_reputation.unwrap_or(0);
    if retrait < 0 {
        return Err(ApiErreur::Validation("Le retrait de réputation est un nombre positif".into()));
    }

    let mut tx = pool.begin().await?;

    // Verrou sur le joueur : deux annulations simultanées ne recalculent pas
    // chacune sur un état que l'autre est en train de changer.
    let existe: Option<i32> = sqlx::query_scalar(
        "SELECT score_total FROM jeu.joueur WHERE utilisateur_id = $1 FOR UPDATE",
    )
    .bind(utilisateur_id)
    .fetch_optional(&mut *tx)
    .await?;
    if existe.is_none() {
        return Err(ApiErreur::NonTrouve("Ce membre n'a jamais joué".into()));
    }

    let (gains_annules, score_retire): (i64, i64) = sqlx::query_as(
        "WITH annules AS (
            UPDATE jeu.gain
               SET annule_at = NOW(), annule_par = $2, motif_annulation = $3
             WHERE utilisateur_id = $1 AND annule_at IS NULL
               AND ($4::timestamptz IS NULL OR created_at >= $4)
             RETURNING montant
         )
         SELECT COUNT(*), COALESCE(SUM(montant), 0)::bigint FROM annules",
    )
    .bind(utilisateur_id)
    .bind(admin.id)
    .bind(motif)
    .bind(body.depuis)
    .fetch_one(&mut *tx)
    .await?;

    // Recalcul des agrégats depuis le journal restant.
    sqlx::query("DELETE FROM jeu.score_saison WHERE utilisateur_id = $1")
        .bind(utilisateur_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        "INSERT INTO jeu.score_saison (saison_id, utilisateur_id, pays_id, score, atteint_at)
         SELECT saison_id, utilisateur_id, pays_id, SUM(montant)::int, MAX(created_at)
           FROM jeu.gain
          WHERE utilisateur_id = $1 AND annule_at IS NULL AND saison_id IS NOT NULL
          GROUP BY saison_id, utilisateur_id, pays_id",
    )
    .bind(utilisateur_id)
    .execute(&mut *tx)
    .await?;
    let score_total: i32 = sqlx::query_scalar(
        "UPDATE jeu.joueur
            SET score_total = (SELECT COALESCE(SUM(montant), 0)::int FROM jeu.gain
                                WHERE utilisateur_id = $1 AND annule_at IS NULL),
                updated_at = NOW()
          WHERE utilisateur_id = $1
          RETURNING score_total",
    )
    .bind(utilisateur_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    if retrait > 0 {
        crate::services::engagement::ajuster(pool.get_ref(), utilisateur_id, 0, -retrait).await;
    }

    let detail_reputation = if retrait > 0 {
        format!(", et {retrait} de réputation")
    } else {
        String::new()
    };
    notification::creer_notification(
        pool.get_ref(),
        utilisateur_id,
        notification::jeu::GAINS_ANNULES,
        &format!(
            "Des gains d'activités ont été annulés ({score_retire} de score{detail_reputation}). Motif : {motif}"
        ),
        Some(notification::jeu::LIEN_ESPACE),
    )
    .await;

    let bilan = BilanAnnulation { gains_annules, score_retire, score_total, reputation_retiree: retrait };
    auditer(
        pool.get_ref(), &req, admin.id, "GAINS_ANNULES", "gain", Some(utilisateur_id),
        Some(serde_json::json!({ "score_total": existe })),
        Some(serde_json::json!({
            "motif": motif, "depuis": body.depuis, "gains_annules": gains_annules,
            "score_retire": score_retire, "score_total": score_total,
            "reputation_retiree": retrait,
        })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(bilan), error: None }))
}

// ─── Médias d'épreuve (feature 014, research D4) ─────────────────────────────

/// Plafond d'un extrait sonore : environ 30 s à 256 kbit/s. La durée exacte est
/// contrôlée par le navigateur de l'administrateur avant l'envoi ; ce plafond est
/// le garde-fou côté serveur, sans bibliothèque audio.
const TAILLE_MAX_AUDIO_EPREUVE: usize = 1024 * 1024;
/// Lecture bornée d'un fichier déposé : au-delà, `normaliser_photo` refuserait.
const TAILLE_MAX_LECTURE: usize = 15 * 1024 * 1024;

/// Reconnaît un extrait sonore par sa SIGNATURE binaire, jamais par son
/// extension : un PNG renommé en `.mp3` est refusé.
fn format_audio(octets: &[u8]) -> Option<&'static str> {
    let debut = |s: &[u8]| octets.starts_with(s);
    if debut(b"ID3") || (octets.len() > 1 && octets[0] == 0xFF && octets[1] & 0xE0 == 0xE0) {
        Some("mp3")
    } else if debut(b"OggS") {
        Some("ogg")
    } else if octets.len() > 12 && &octets[4..8] == b"ftyp" {
        Some("m4a")
    } else if debut(b"RIFF") && octets.len() > 12 && &octets[8..12] == b"WAVE" {
        Some("wav")
    } else {
        None
    }
}

/// POST /api/admin/jeu/medias (multipart : `type` = image|audio, `fichier`).
///
/// Renvoie le `media_url` à placer dans l'épreuve. Une photo est normalisée et
/// ré-encodée, ce qui en retire les métadonnées EXIF (dont la position GPS).
pub async fn deposer_media(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    mut payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ApiErreur> {
    use futures_util::StreamExt;

    verifier_permission!(admin, "jeu", "gerer");

    let mut type_media: Option<String> = None;
    let mut octets: Vec<u8> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut champ = item.map_err(|e| ApiErreur::Upload(format!("Champ multipart invalide : {e}")))?;
        let nom = champ.content_disposition().and_then(|cd| cd.get_name()).unwrap_or("").to_string();
        let mut contenu: Vec<u8> = Vec::new();
        while let Some(morceau) = champ.next().await {
            let morceau = morceau.map_err(|e| ApiErreur::Upload(format!("Erreur de lecture : {e}")))?;
            if contenu.len() + morceau.len() > TAILLE_MAX_LECTURE {
                return Err(ApiErreur::Validation("Fichier trop lourd : 15 Mo au plus".into()));
            }
            contenu.extend_from_slice(&morceau);
        }
        match nom.as_str() {
            "type" => type_media = Some(String::from_utf8_lossy(&contenu).trim().to_string()),
            "fichier" => octets = contenu,
            _ => {}
        }
    }

    if octets.is_empty() {
        return Err(ApiErreur::Validation("Aucun fichier reçu".into()));
    }

    let (sous_dossier, extension, contenu) = match type_media.as_deref() {
        Some("image") => {
            let (normalisee, format, _, _) =
                image_validation::normaliser_photo(&octets).map_err(|e| ApiErreur::Validation(e.message()))?;
            ("images", format.extension(), normalisee)
        }
        Some("audio") => {
            if octets.len() > TAILLE_MAX_AUDIO_EPREUVE {
                return Err(ApiErreur::Validation(
                    "Extrait sonore trop lourd : 1 Mo au plus (environ 30 secondes)".into(),
                ));
            }
            let ext = format_audio(&octets).ok_or_else(|| {
                ApiErreur::Validation("Format sonore non reconnu : MP3, OGG, M4A ou WAV".into())
            })?;
            ("audios", ext, octets)
        }
        _ => return Err(ApiErreur::Validation("Type de média attendu : image ou audio".into())),
    };

    let nom_stocke = format!("{}.{}", Uuid::new_v4(), extension);
    let dossier = format!("{}/jeu/{}", upload_dir.get_ref(), sous_dossier);
    std::fs::create_dir_all(&dossier)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le répertoire : {e}")))?;
    std::fs::write(format!("{dossier}/{nom_stocke}"), &contenu)
        .map_err(|e| ApiErreur::Upload(format!("Impossible d'écrire le fichier : {e}")))?;

    let media_type = if sous_dossier == "images" { "image" } else { "audio" };
    let media_url = format!("/uploads/jeu/{sous_dossier}/{nom_stocke}");

    auditer(
        pool.get_ref(), &req, admin.id, "MEDIA_EPREUVE_DEPOSE", "epreuve", None, None,
        Some(serde_json::json!({ "media_type": media_type, "media_url": media_url, "octets": contenu.len() })),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "media_type": media_type, "media_url": media_url })),
        error: None,
    }))
}

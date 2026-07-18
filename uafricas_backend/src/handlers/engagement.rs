//! Endpoints publics du système d'engagement (lecture).
//! - `GET /api/engagement/mon-compte` (JWT)
//! - `GET /api/engagement/mon-journal` (JWT, paginé)
//! - `GET /api/engagement/niveau/{utilisateur_id}` (public léger, badge)

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::engagement::{
    CompteResponse, CompteRow, JournalPage, MouvementResponse, NiveauInfo, ProchainNiveau,
};
use crate::ApiResponse;

/// Extrait l'utilisateur connecté depuis le header Authorization (JWT).
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Charge un niveau par son code (défaut « membre » si introuvable).
async fn charger_niveau(pool: &PgPool, code: &str) -> Result<NiveauInfo, ApiErreur> {
    let niveau = sqlx::query_as::<_, NiveauInfo>(
        "SELECT code, libelle, seuil_min, badge_couleur, badge_icone
         FROM engagement.niveau WHERE code = $1",
    )
    .bind(code)
    .fetch_optional(pool)
    .await?;

    Ok(niveau.unwrap_or(NiveauInfo {
        code: "membre".to_string(),
        libelle: "Membre".to_string(),
        seuil_min: 0,
        badge_couleur: Some("gray".to_string()),
        badge_icone: Some("user".to_string()),
    }))
}

/// Prochain niveau au-dessus du solde courant.
async fn charger_prochain_niveau(
    pool: &PgPool,
    solde: i32,
) -> Result<Option<ProchainNiveau>, ApiErreur> {
    let prochain = sqlx::query_as::<_, (String, String, i32)>(
        "SELECT code, libelle, seuil_min FROM engagement.niveau
         WHERE seuil_min > $1 ORDER BY seuil_min ASC LIMIT 1",
    )
    .bind(solde)
    .fetch_optional(pool)
    .await?;

    Ok(prochain.map(|(code, libelle, seuil_min)| ProchainNiveau {
        code,
        libelle,
        seuil_min,
        points_restants: (seuil_min - solde).max(0),
    }))
}

/// GET /api/engagement/mon-compte
pub async fn mon_compte(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let compte = sqlx::query_as::<_, CompteRow>(
        "SELECT solde_points, solde_points_mensuel, reputation, niveau_code, dernier_mouvement_at
         FROM engagement.compte WHERE utilisateur_id = $1",
    )
    .bind(uid)
    .fetch_optional(pool.get_ref())
    .await?;

    let (solde, mensuel, reputation, niveau_code, dernier) = match compte {
        Some(c) => (
            c.solde_points,
            c.solde_points_mensuel,
            c.reputation,
            c.niveau_code,
            c.dernier_mouvement_at,
        ),
        None => (0, 0, 0, "membre".to_string(), None),
    };

    let niveau = charger_niveau(pool.get_ref(), &niveau_code).await?;
    let prochain_niveau = charger_prochain_niveau(pool.get_ref(), solde).await?;

    let reponse = CompteResponse {
        solde_points: solde,
        solde_points_mensuel: mensuel,
        reputation,
        niveau,
        prochain_niveau,
        dernier_mouvement_at: dernier,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

/// Paramètres du journal.
#[derive(serde::Deserialize)]
pub struct JournalParams {
    pub page: Option<i64>,
    pub taille: Option<i64>,
    pub type_action: Option<String>,
}

/// GET /api/engagement/mon-journal
pub async fn mon_journal(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<JournalParams>,
) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * taille;
    let filtre = params.type_action.clone();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM engagement.mouvement_points
         WHERE utilisateur_id = $1 AND ($2::text IS NULL OR type_action = $2)",
    )
    .bind(uid)
    .bind(&filtre)
    .fetch_one(pool.get_ref())
    .await?;

    let elements = sqlx::query_as::<_, MouvementResponse>(
        "SELECT m.id, m.type_action, r.libelle, m.type_objet, m.objet_id, m.points,
                m.reputation_delta, m.solde_apres, m.plafond_atteint, m.created_at
         FROM engagement.mouvement_points m
         LEFT JOIN engagement.regle_points r ON r.type_action = m.type_action
         WHERE m.utilisateur_id = $1 AND ($2::text IS NULL OR m.type_action = $2)
         ORDER BY m.created_at DESC
         LIMIT $3 OFFSET $4",
    )
    .bind(uid)
    .bind(&filtre)
    .bind(taille)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(JournalPage {
            elements,
            total,
            page,
            taille,
        }),
        error: None,
    }))
}

/// GET /api/engagement/niveau/{utilisateur_id} — badge public léger.
pub async fn niveau_utilisateur(
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let uid = path.into_inner();

    let code: Option<String> =
        sqlx::query_scalar("SELECT niveau_code FROM engagement.compte WHERE utilisateur_id = $1")
            .bind(uid)
            .fetch_optional(pool.get_ref())
            .await?;

    let niveau = charger_niveau(pool.get_ref(), code.as_deref().unwrap_or("membre")).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(niveau),
        error: None,
    }))
}

//! Endpoints publics du système d'engagement (lecture).
//! - `GET /api/engagement/mon-compte` (JWT)
//! - `GET /api/engagement/mon-journal` (JWT, paginé, filtrable)
//! - `GET /api/engagement/mes-categories` (JWT, ventilation par catégorie)
//! - `GET /api/engagement/actions-recompensees` (public, barème)
//! - `GET /api/engagement/niveau/{utilisateur_id}` (public léger, badge)

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::engagement::{
    ActionRecompensee, BadgeADebloquerResponse, BadgeCatalogueRow, BadgeObtenuResponse,
    CategorieVentilation, CompteResponse, CompteRow, JournalPage, MesBadgesResponse,
    MouvementResponse, NiveauInfo, ProchainNiveau, VentilationResponse,
};
use crate::ApiResponse;

/// Charge les badges obtenus d'un membre (requête partagée par l'espace membre et
/// le profil public : un seul SQL, donc une seule définition de « badge obtenu »).
async fn charger_badges_obtenus(
    pool: &PgPool,
    utilisateur_id: Uuid,
) -> Result<Vec<BadgeObtenuResponse>, ApiErreur> {
    // Un badge désactivé déjà obtenu RESTE affiché chez son détenteur (FR-020) :
    // pas de filtre `b.actif` ici, contrairement au catalogue « à débloquer ».
    let badges = sqlx::query_as::<_, BadgeObtenuResponse>(
        "SELECT b.code, b.libelle, b.description, b.couleur, b.icone,
                bo.origine::text AS origine, bo.created_at AS obtenu_at
         FROM engagement.badge_obtenu bo
         JOIN engagement.badge b ON b.id = bo.badge_id
         WHERE bo.utilisateur_id = $1
         ORDER BY bo.created_at DESC, b.ordre",
    )
    .bind(utilisateur_id)
    .fetch_all(pool)
    .await?;
    Ok(badges)
}

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

    // Cagnotte et cadeaux reçus (feature 008) : le compte d'engagement est le
    // point d'entrée unique de l'espace membre, il doit porter les deux repères
    // sans imposer un second aller-retour.
    let (montant_cumule, devise): (i32, String) = sqlx::query_as(
        "SELECT COALESCE(c.montant_cumule, 0),
                COALESCE((SELECT devise FROM engagement.parametre_monetisation WHERE id = TRUE), 'XOF')
           FROM (SELECT 1) AS _
           LEFT JOIN engagement.cagnotte c ON c.utilisateur_id = $1",
    )
    .bind(uid)
    .fetch_one(pool.get_ref())
    .await?;

    let cadeaux_recus: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM engagement.transaction_cadeau
          WHERE beneficiaire_id = $1 AND etat = 'abouti'",
    )
    .bind(uid)
    .fetch_one(pool.get_ref())
    .await?;

    let reponse = CompteResponse {
        solde_points: solde,
        solde_points_mensuel: mensuel,
        reputation,
        niveau,
        prochain_niveau,
        dernier_mouvement_at: dernier,
        cagnotte: crate::models::engagement::CagnotteResume {
            montant_cumule,
            devise,
            // Aucun versement dans cette itération (FR-026).
            versement_disponible: false,
        },
        cadeaux_recus,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

/// Paramètres du journal. Les filtres nuls sont neutralisés par cast paramétré
/// (`$n::text IS NULL OR …`) : jamais de concaténation de fragments SQL.
#[derive(serde::Deserialize)]
pub struct JournalParams {
    pub page: Option<i64>,
    pub taille: Option<i64>,
    pub type_action: Option<String>,
    /// Code de catégorie (`engagement.categorie_points.code`).
    pub categorie: Option<String>,
    /// Bornes de période, dates ISO `YYYY-MM-DD`.
    pub depuis: Option<String>,
    pub jusqu_a: Option<String>,
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
    let categorie = params.categorie.clone();
    let depuis = params.depuis.clone();
    let jusqu_a = params.jusqu_a.clone();

    // La condition de catégorie est identique dans les deux requêtes : la
    // catégorie est celle FIGÉE sur le mouvement, jointe pour son code seul.
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM engagement.mouvement_points m
         LEFT JOIN engagement.categorie_points c ON c.id = m.categorie_id
         WHERE m.utilisateur_id = $1
           AND ($2::text IS NULL OR m.type_action = $2)
           AND ($3::text IS NULL OR c.code = $3)
           AND ($4::date IS NULL OR m.created_at >= $4::date)
           AND ($5::date IS NULL OR m.created_at < ($5::date + INTERVAL '1 day'))",
    )
    .bind(uid)
    .bind(&filtre)
    .bind(&categorie)
    .bind(&depuis)
    .bind(&jusqu_a)
    .fetch_one(pool.get_ref())
    .await?;

    let elements = sqlx::query_as::<_, MouvementResponse>(
        "SELECT m.id, m.type_action, r.libelle,
                c.code AS categorie_code, c.libelle AS categorie_libelle,
                m.type_objet, m.objet_id, m.points,
                m.reputation_delta, m.solde_apres, m.plafond_atteint, m.created_at
         FROM engagement.mouvement_points m
         LEFT JOIN engagement.regle_points r ON r.type_action = m.type_action
         LEFT JOIN engagement.categorie_points c ON c.id = m.categorie_id
         WHERE m.utilisateur_id = $1
           AND ($2::text IS NULL OR m.type_action = $2)
           AND ($3::text IS NULL OR c.code = $3)
           AND ($4::date IS NULL OR m.created_at >= $4::date)
           AND ($5::date IS NULL OR m.created_at < ($5::date + INTERVAL '1 day'))
         ORDER BY m.created_at DESC
         LIMIT $6 OFFSET $7",
    )
    .bind(uid)
    .bind(&filtre)
    .bind(&categorie)
    .bind(&depuis)
    .bind(&jusqu_a)
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

/// GET /api/engagement/mes-categories : ventilation des points par catégorie (FR-011).
///
/// Une seule requête d'agrégation sur le journal du membre (R2 : aucun solde
/// persisté par catégorie). `solde_points` (courant) et `total_gagne` (cumul du
/// journal) sont exposés **séparément** : le plancher 0 peut les faire diverger,
/// et c'est précisément l'écart que l'espace membre doit rendre compréhensible.
pub async fn mes_categories(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let solde_points: i32 =
        sqlx::query_scalar("SELECT solde_points FROM engagement.compte WHERE utilisateur_id = $1")
            .bind(uid)
            .fetch_optional(pool.get_ref())
            .await?
            .unwrap_or(0);

    // Les mouvements sans catégorie (antérieurs au rattrapage, règle supprimée)
    // sont regroupés sous « Autres » et placés en fin de liste, aucune ligne
    // n'est masquée, sinon la somme ne se réconcilierait pas avec le journal.
    let categories = sqlx::query_as::<_, CategorieVentilation>(
        "SELECT c.code,
                COALESCE(c.libelle, 'Autres')            AS libelle,
                c.couleur, c.icone,
                COALESCE(c.ordre, 99::smallint)          AS ordre,
                COALESCE(SUM(m.points), 0)::bigint       AS points,
                COUNT(*)::bigint                         AS nombre_mouvements
         FROM engagement.mouvement_points m
         LEFT JOIN engagement.categorie_points c ON c.id = m.categorie_id
         WHERE m.utilisateur_id = $1
         GROUP BY c.code, c.libelle, c.couleur, c.icone, c.ordre
         ORDER BY ordre, libelle",
    )
    .bind(uid)
    .fetch_all(pool.get_ref())
    .await?;

    let total_gagne: i64 = categories.iter().map(|c| c.points).sum();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(VentilationResponse {
            solde_points,
            total_gagne,
            categories,
        }),
        error: None,
    }))
}

/// GET /api/engagement/actions-recompensees, barème public (FR-015, FR-016).
///
/// Source **unique** des libellés, montants, plafonds et seuils du barème côté
/// front : aucune de ces valeurs n'est écrite en dur dans le frontend. Public,
/// car le barème n'est pas une donnée sensible et l'afficher sert l'engagement.
pub async fn actions_recompensees(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let actions = sqlx::query_as::<_, ActionRecompensee>(
        "SELECT r.type_action, r.libelle, r.points, r.reputation_delta,
                c.code AS categorie_code, c.libelle AS categorie_libelle,
                c.icone AS categorie_icone,
                r.plafond_journalier, r.seuil_declencheur
         FROM engagement.regle_points r
         LEFT JOIN engagement.categorie_points c ON c.id = r.categorie_id
         WHERE r.actif = TRUE
         ORDER BY c.ordre NULLS LAST, r.points DESC, r.libelle",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(actions),
        error: None,
    }))
}

/// GET /api/engagement/mes-badges (FR-013, FR-018)
///
/// Effet de bord assumé : `evaluer_badges` est appelée **avant** de répondre.
/// C'est ce qui rattrape les conditions devenues vraies sans mouvement, badge
/// créé par l'administration, seuil abaissé, sans aucune tâche de fond.
/// L'insertion étant `ON CONFLICT DO NOTHING`, l'appel est inoffensif.
pub async fn mes_badges(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    crate::services::engagement::evaluer_badges(pool.get_ref(), uid).await;

    let obtenus = charger_badges_obtenus(pool.get_ref(), uid).await?;

    // Le catalogue « à débloquer » exclut les badges désactivés (retirés du
    // catalogue) et les badges manuels : une distinction éditoriale ne se
    // « débloque » pas, l'annoncer comme un objectif serait mensonger.
    let restants = sqlx::query_as::<_, BadgeCatalogueRow>(
        "SELECT b.id, b.code, b.libelle, b.description, b.couleur, b.icone
         FROM engagement.badge b
         WHERE b.actif = TRUE AND b.manuel = FALSE
           AND NOT EXISTS (SELECT 1 FROM engagement.badge_obtenu bo
                            WHERE bo.badge_id = b.id AND bo.utilisateur_id = $1)
         ORDER BY b.ordre",
    )
    .bind(uid)
    .fetch_all(pool.get_ref())
    .await?;

    let mut a_debloquer = Vec::with_capacity(restants.len());
    for b in restants {
        // La progression vient du même SQL que la condition d'attribution
        // (`services::engagement`), pour que les deux ne puissent pas diverger.
        let progression =
            crate::services::engagement::progression_badge(pool.get_ref(), uid, b.id).await;
        a_debloquer.push(BadgeADebloquerResponse {
            code: b.code,
            libelle: b.libelle,
            description: b.description,
            couleur: b.couleur,
            icone: b.icone,
            progression_actuelle: progression.map(|(actuel, _)| actuel),
            progression_cible: progression.map(|(_, cible)| cible),
        });
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MesBadgesResponse { obtenus, a_debloquer }),
        error: None,
    }))
}

/// GET /api/engagement/badges/{utilisateur_id}, badges **publics** (FR-014).
///
/// Public comme `GET /niveau/{utilisateur_id}` : renvoie uniquement les badges
/// obtenus. **Jamais** de solde, de réputation ni de mouvement, le détail
/// chiffré de l'engagement reste privé.
pub async fn badges_utilisateur(
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let badges = charger_badges_obtenus(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(badges), error: None }))
}

/// Réseaux traçables (miroir de l'enum `engagement.reseau_social`).
/// « Copier le lien » n'en fait pas partie : invérifiable et trivialement
/// répétable, il ne compte pas.
const RESEAUX_TRACABLES: &[&str] =
    &["whatsapp", "facebook", "x", "linkedin", "telegram", "email"];

/// Familles de contenus partageables : littéraux fixes, alignés sur les **valeurs
/// réellement émises** par les 6 composants de partage du frontend (et non sur une
/// nomenclature théorique : une famille mal orthographiée ferait échouer le
/// traçage en silence).
const FAMILLES_PARTAGEABLES: &[&str] = &[
    // Médias radio & télé : `media/MediaPartagerModal`
    "chaine_tv",
    "station_radio",
    "emission_tele",
    "emission_radio",
    "episode_tele",
    "episode_radio",
    // Opportunité Afrique : `PartagerFicheModal` et `PartagerElementModal`
    // (valeurs de `TypeObjetElement`, composables/useOpportuniteAfrique.ts)
    "fiche_pays",
    "secteur_developpement",
    "recette_culinaire",
    "site_touristique",
    "personnalite_connue",
    // Événements : `evenements/EvenementPartage`
    "evenement",
    // Gouvernance : `universite/gouvernance/PartagePublication`
    "idea_force",
    "factcheck",
    "bad_habit",
    // Retrouve-amis : `retrouve-amis/BoutonsPartage`
    "avis_recherche",
    // Familles ajoutées par la feature 008 : depuis que le partage crédite
    // l'auteur, toute famille dotée d'un auteur résolvable doit pouvoir être
    // tracée : sinon le partage externe d'une vidéo ne rapporterait rien alors
    // que son repost interne, lui, crédite.
    "codimoi",
    "video",
    "biblio_humaine",
    "profil",
];

#[derive(serde::Deserialize)]
pub struct PartageExterneRequest {
    pub type_objet: String,
    pub objet_id: Uuid,
    pub reseau: String,
}

/// POST /api/engagement/partages-externes (FR-027)
///
/// L'identité vient **toujours** du JWT, jamais du corps : sinon n'importe qui
/// pourrait créditer n'importe qui.
pub async fn tracer_partage_externe(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<PartageExterneRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let reseau = body.reseau.trim().to_lowercase();
    if !RESEAUX_TRACABLES.contains(&reseau.as_str()) {
        return Err(ApiErreur::Validation(format!(
            "Réseau inconnu : « {reseau} »"
        )));
    }

    let type_objet = body.type_objet.trim();
    if !FAMILLES_PARTAGEABLES.contains(&type_objet) {
        return Err(ApiErreur::Validation(format!(
            "Famille de contenus non partageable : « {type_objet} »"
        )));
    }

    let resultat = crate::services::engagement::enregistrer_partage_externe(
        pool.get_ref(),
        uid,
        type_objet,
        body.objet_id,
        &reseau,
    )
    .await?;

    // Le seuil de 5 réseaux distincts n'existe plus : le partage crédite
    // désormais l'AUTEUR du contenu, une seule fois par partageur et par
    // contenu, tous canaux confondus (feature 008, research R5).
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "enregistre": resultat.enregistre,
            "auteur_credite": resultat.auteur_credite,
        })),
        error: None,
    }))
}

/// GET /api/engagement/niveau/{utilisateur_id}, badge public léger.
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

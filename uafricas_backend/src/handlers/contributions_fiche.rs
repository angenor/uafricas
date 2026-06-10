use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, web};
use futures_util::StreamExt;
use serde_json::json;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::afripulse::{CategorieSiteTouristique, sous_type_appartient_a};
use crate::models::contribution_fiche::{
    CONTRIBUTION_COLONNES, ContributionFicheRow, ContributionListeResponse,
    ContributionQueryParams, CreerContributionBody, TypeObjetContribution, construire_contribution_response, section_est_valide,
};
use crate::services::{audit, image_validation, rate_limit_afripulse};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ══════════════════════════════════════════════════════════════════════════
// Fonctions utilitaires
// ══════════════════════════════════════════════════════════════════════════

/// Extraire l'utilisateur connecte depuis le header Authorization
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Verifier si l'utilisateur a le role admin
async fn verifier_admin(pool: &PgPool, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
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

/// Recuperer la valeur actuelle d'un champ de la fiche pays
async fn obtenir_valeur_actuelle(
    pool: &PgPool,
    fiche_id: Uuid,
    section: &str,
) -> Result<Option<String>, ApiErreur> {
    let valeur: Option<String> = match section {
        "population" => {
            sqlx::query_scalar(
                "SELECT population::text FROM country_profile.fiche_pays WHERE id = $1",
            )
            .bind(fiche_id)
            .fetch_one(pool)
            .await?
        }
        "superficie_km2" => {
            sqlx::query_scalar(
                "SELECT superficie_km2::text FROM country_profile.fiche_pays WHERE id = $1",
            )
            .bind(fiche_id)
            .fetch_one(pool)
            .await?
        }
        "biographie" => {
            sqlx::query_scalar("SELECT biographie FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "contexte" => {
            sqlx::query_scalar("SELECT contexte FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "contexte_historique" => {
            sqlx::query_scalar(
                "SELECT contexte_historique FROM country_profile.fiche_pays WHERE id = $1",
            )
            .bind(fiche_id)
            .fetch_one(pool)
            .await?
        }
        "slogan" => {
            sqlx::query_scalar("SELECT slogan FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "hymne_national" => {
            sqlx::query_scalar(
                "SELECT hymne_national FROM country_profile.fiche_pays WHERE id = $1",
            )
            .bind(fiche_id)
            .fetch_one(pool)
            .await?
        }
        "langue_officielle" => {
            sqlx::query_scalar(
                "SELECT langue_officielle FROM country_profile.fiche_pays WHERE id = $1",
            )
            .bind(fiche_id)
            .fetch_one(pool)
            .await?
        }
        "langues_populaires" => {
            sqlx::query_scalar(
                "SELECT langues_populaires FROM country_profile.fiche_pays WHERE id = $1",
            )
            .bind(fiche_id)
            .fetch_one(pool)
            .await?
        }
        "monnaie" => {
            sqlx::query_scalar("SELECT monnaie FROM country_profile.fiche_pays WHERE id = $1")
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        "fuseau_horaire" => {
            sqlx::query_scalar(
                "SELECT fuseau_horaire FROM country_profile.fiche_pays WHERE id = $1",
            )
            .bind(fiche_id)
            .fetch_one(pool)
            .await?
        }
        // Bloc « À savoir avant de voyager » — colonnes TEXT (liste fermée de littéraux)
        s @ ("voyage_langue_internationale"
        | "voyage_langue_locale"
        | "voyage_infos_visa"
        | "voyage_infos_sanitaires"
        | "voyage_meteo"
        | "voyage_prises_electriques"
        | "voyage_contacts_tourisme"
        | "voyage_recommandations_securite") => {
            // `s` provient d'un motif littéral fermé → interpolation sûre (pas d'injection).
            let requete = format!(
                "SELECT {} FROM country_profile.fiche_pays WHERE id = $1",
                s
            );
            sqlx::query_scalar(&requete)
                .bind(fiche_id)
                .fetch_one(pool)
                .await?
        }
        // Pour les tables liees (groupe_ethnique, site_touristique, etc.), pas de valeur actuelle
        _ => None,
    };
    Ok(valeur)
}

// ══════════════════════════════════════════════════════════════════════════
// Endpoints
// ══════════════════════════════════════════════════════════════════════════

/// POST /api/fiches-pays/{id}/contributions — Soumettre une contribution
///
/// Deux modes :
///   • Legacy (rétrocompatibilité) : body JSON scalaire (section texte +
///     nouvelle_valeur TEXT). Inchangé.
///   • Afripulse : body JSON avec `type_objet_contribution` ≠ "fiche_pays",
///     `section_afripulse`, `nouvelle_valeur_jsonb`, `target_id` optionnel.
///     Applique rate-limit (20 textes/j, 10 photos/j, 5 en attente par pays),
///     snapshot ancienne_valeur_jsonb pour edition/suppression, log audit.
pub async fn soumettre_contribution(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    body: web::Json<CreerContributionBody>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;

    let fiche_id_str = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_id_str)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    // Vérifier que la fiche existe
    let fiche_existe: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM country_profile.fiche_pays WHERE id = $1)")
            .bind(fiche_id)
            .fetch_one(pool.get_ref())
            .await?;
    if !fiche_existe {
        return Err(ApiErreur::NonTrouve("Fiche pays non trouvee".to_string()));
    }

    // ── Détection du mode Afripulse vs legacy ─────────────────────────────
    let type_objet_str = body
        .type_objet_contribution
        .as_deref()
        .unwrap_or("fiche_pays")
        .to_lowercase();

    let mode_afripulse = type_objet_str != "fiche_pays";

    // Type d'action CRUD (aligné enum SQL country_profile.type_contribution)
    let type_contribution = body
        .type_contribution
        .as_deref()
        .unwrap_or(if mode_afripulse {
            "ajout"
        } else {
            "modification"
        })
        .to_lowercase();
    if !["modification", "ajout", "suppression", "edition"].contains(&type_contribution.as_str()) {
        return Err(ApiErreur::Validation(
            "Type de contribution invalide. Valeurs acceptées : modification, edition, ajout, suppression".to_string(),
        ));
    }
    // Alias 'edition' -> 'modification' pour alignement avec l'enum existant
    let type_contribution_sql = if type_contribution == "edition" {
        "modification".to_string()
    } else {
        type_contribution.clone()
    };

    let justification = body.justification.as_deref().map(|s| s.trim().to_string());
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);

    if mode_afripulse {
        return soumettre_contribution_afripulse(
            pool.get_ref(),
            utilisateur_id,
            fiche_id,
            &body,
            &type_objet_str,
            &type_contribution_sql,
            justification.as_deref(),
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
    }

    // ── Chemin legacy (fiche pays scalaire) ───────────────────────────────
    let section = body.section.trim().to_lowercase();
    if !section_est_valide(&section) {
        return Err(ApiErreur::Validation(format!(
            "Section invalide: '{}'. Sections valides: population, biographie, contexte, slogan, etc.",
            section
        )));
    }

    let nouvelle_valeur = body
        .nouvelle_valeur
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_string();
    if nouvelle_valeur.is_empty() {
        return Err(ApiErreur::Validation(
            "La nouvelle valeur ne peut pas etre vide".to_string(),
        ));
    }

    let ancienne_valeur = if type_contribution_sql == "modification" {
        obtenir_valeur_actuelle(pool.get_ref(), fiche_id, &section).await?
    } else {
        None
    };

    let row = sqlx::query_as::<_, ContributionFicheRow>(
        &format!(
            "WITH inserted AS (
                INSERT INTO country_profile.contribution_fiche
                    (fiche_pays_id, cree_par, section, type_contribution, ancienne_valeur, nouvelle_valeur, justification)
                VALUES ($1, $2, $3, $4::country_profile.type_contribution, $5, $6, $7)
                RETURNING *
            )
            SELECT {} FROM inserted cf
            JOIN iam.utilisateur u ON u.id = cf.cree_par",
            CONTRIBUTION_COLONNES
        ),
    )
    .bind(fiche_id)
    .bind(utilisateur_id)
    .bind(&section)
    .bind(&type_contribution_sql)
    .bind(&ancienne_valeur)
    .bind(&nouvelle_valeur)
    .bind(&justification)
    .fetch_one(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "create",
        "country_profile",
        "contribution_fiche",
        Some(row.id),
        None,
        Some(json!({
            "section": &section,
            "type_contribution": &type_contribution_sql,
            "nouvelle_valeur": &nouvelle_valeur,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let response = construire_contribution_response(&row);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

/// Chemin Afripulse : contribution JSONB structurée sur une section enrichie.
#[allow(clippy::too_many_arguments)]
async fn soumettre_contribution_afripulse(
    pool: &PgPool,
    utilisateur_id: Uuid,
    fiche_id: Uuid,
    body: &CreerContributionBody,
    type_objet_str: &str,
    type_contribution_sql: &str,
    justification: Option<&str>,
    ip: Option<&str>,
    ua: Option<&str>,
) -> Result<HttpResponse, ApiErreur> {
    // Parse type_objet vers l'enum
    let type_objet = match type_objet_str {
        "site_touristique" => TypeObjetContribution::SiteTouristique,
        "secteur_developpement" => TypeObjetContribution::SecteurDeveloppement,
        "personnalite_connue" => TypeObjetContribution::PersonnaliteConnue,
        "savoir_pratique" => TypeObjetContribution::SavoirPratique,
        "recommandation_visiteur" => TypeObjetContribution::RecommandationVisiteur,
        "photo_visiteur" => TypeObjetContribution::PhotoVisiteur,
        "fiche_pays" => TypeObjetContribution::FichePays,
        autre => {
            return Err(ApiErreur::Validation(format!(
                "type_objet_contribution invalide : '{}'",
                autre
            )));
        }
    };

    // Rate-limit
    if let Err(e) =
        rate_limit_afripulse::verifier_quotas(pool, utilisateur_id, &type_objet, fiche_id).await
    {
        return Err(match e {
            rate_limit_afripulse::ErreurVerification::LimiteAtteinte(limite) => {
                ApiErreur::LimiteAtteinte(limite.message())
            }
            rate_limit_afripulse::ErreurVerification::Base(err) => {
                ApiErreur::BaseDeDonnees(err.to_string())
            }
        });
    }

    // Payload JSONB obligatoire (sauf suppression où il peut être vide)
    let nouvelle_valeur_jsonb = body
        .nouvelle_valeur_jsonb
        .clone()
        .or_else(|| {
            body.nouvelle_valeur
                .as_ref()
                .map(|s| serde_json::Value::String(s.clone()))
        })
        .unwrap_or(serde_json::Value::Null);

    if type_contribution_sql != "suppression" && nouvelle_valeur_jsonb.is_null() {
        return Err(ApiErreur::Validation(
            "nouvelle_valeur_jsonb requis pour ajout / modification".to_string(),
        ));
    }

    // Validation enrichie des sites touristiques (US1/US2) — ajout & édition.
    if matches!(type_objet, TypeObjetContribution::SiteTouristique)
        && type_contribution_sql != "suppression"
    {
        valider_site_touristique(&nouvelle_valeur_jsonb)?;
    }

    // T059 : si ajout de recommandation et l'utilisateur a deja une reco active,
    // auto-convertir en edition avec target_id = id de la reco active.
    let mut type_contribution_effectif = type_contribution_sql.to_string();
    let mut target_id_effectif = body.target_id;
    if matches!(type_objet, TypeObjetContribution::RecommandationVisiteur)
        && type_contribution_sql == "ajout"
    {
        let reco_active: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM country_profile.recommandation_visiteur
             WHERE utilisateur_id = $1 AND fiche_pays_id = $2
               AND active = TRUE AND deleted_at IS NULL
             LIMIT 1",
        )
        .bind(utilisateur_id)
        .bind(fiche_id)
        .fetch_optional(pool)
        .await?;
        if let Some(reco_id) = reco_active {
            type_contribution_effectif = "modification".to_string();
            target_id_effectif = Some(reco_id);
        }
    }

    // Snapshot ancienne_valeur_jsonb pour edition/suppression
    let ancienne_valeur_jsonb = if type_contribution_effectif != "ajout" {
        if let Some(target_id) = target_id_effectif {
            obtenir_snapshot_afripulse(pool, &type_objet, target_id).await?
        } else {
            return Err(ApiErreur::Validation(
                "target_id requis pour edition / suppression".to_string(),
            ));
        }
    } else {
        None
    };

    // Section Afripulse (optionnelle — utile pour filtres admin)
    let section_afripulse = body
        .section_afripulse
        .as_deref()
        .map(|s| s.trim().to_lowercase());

    // Section texte libre (sert pour rétrocompat ; on reprend type_objet_str)
    let section_texte = section_afripulse
        .clone()
        .unwrap_or_else(|| type_objet_str.to_string());

    // INSERT + SELECT enrichi
    let row = sqlx::query_as::<_, ContributionFicheRow>(&format!(
        "WITH inserted AS (
                INSERT INTO country_profile.contribution_fiche (
                    fiche_pays_id, cree_par, section, type_contribution,
                    ancienne_valeur, nouvelle_valeur, justification,
                    type_objet_contribution, section_afripulse, target_id,
                    nouvelle_valeur_jsonb, ancienne_valeur_jsonb
                ) VALUES (
                    $1, $2, $3, $4::country_profile.type_contribution,
                    NULL, COALESCE($5::text, ''), $6,
                    $7::country_profile.type_objet_contribution,
                    $8::country_profile.section_afripulse,
                    $9, $10, $11
                )
                RETURNING *
            )
            SELECT {} FROM inserted cf JOIN iam.utilisateur u ON u.id = cf.cree_par",
        CONTRIBUTION_COLONNES
    ))
    .bind(fiche_id)
    .bind(utilisateur_id)
    .bind(&section_texte)
    .bind(&type_contribution_effectif)
    .bind(nouvelle_valeur_jsonb.to_string())
    .bind(justification)
    .bind(type_objet_str)
    .bind(section_afripulse.as_deref())
    .bind(target_id_effectif)
    .bind(&nouvelle_valeur_jsonb)
    .bind(&ancienne_valeur_jsonb)
    .fetch_one(pool)
    .await?;

    audit::log_action(
        pool,
        Some(utilisateur_id),
        "create",
        "country_profile",
        "contribution_fiche",
        Some(row.id),
        None,
        Some(json!({
            "type_objet": type_objet_str,
            "section_afripulse": section_afripulse,
            "type_contribution": type_contribution_effectif,
            "target_id": target_id_effectif,
            "nouvelle_valeur_jsonb": &nouvelle_valeur_jsonb,
        })),
        ip,
        ua,
    )
    .await;

    let response = construire_contribution_response(&row);

    Ok(HttpResponse::Accepted().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

/// Récupère un snapshot JSONB de la ligne cible (pour edition/suppression)
/// selon le type d'objet Afripulse.
async fn obtenir_snapshot_afripulse(
    pool: &PgPool,
    type_objet: &TypeObjetContribution,
    target_id: Uuid,
) -> Result<Option<serde_json::Value>, ApiErreur> {
    let table_colonnes = match type_objet {
        TypeObjetContribution::SiteTouristique => (
            "country_profile.site_touristique",
            "jsonb_build_object('id', id, 'nom', nom, 'categorie', categorie::text, 'site_web_url', site_web_url, 'fiche_pays_id', fiche_pays_id)",
        ),
        TypeObjetContribution::SecteurDeveloppement => (
            "country_profile.secteur_developpement",
            "jsonb_build_object('id', id, 'nom', nom, 'fiche_pays_id', fiche_pays_id)",
        ),
        TypeObjetContribution::PersonnaliteConnue => (
            "country_profile.personnalite_connue",
            "jsonb_build_object('id', id, 'nom_complet', nom_complet, 'domaine', domaine::text, 'biographie_courte', biographie_courte, 'annee_naissance', annee_naissance, 'annee_deces', annee_deces, 'portrait_url', portrait_url, 'lien_reference', lien_reference, 'fiche_pays_id', fiche_pays_id)",
        ),
        TypeObjetContribution::SavoirPratique => (
            "country_profile.savoir_pratique",
            "jsonb_build_object('id', id, 'titre', titre, 'categorie', categorie::text, 'explication', explication, 'exemple', exemple, 'fiche_pays_id', fiche_pays_id)",
        ),
        TypeObjetContribution::RecommandationVisiteur => (
            "country_profile.recommandation_visiteur",
            "jsonb_build_object('id', id, 'note', note, 'commentaire', commentaire, 'active', active, 'utilisateur_id', utilisateur_id, 'fiche_pays_id', fiche_pays_id)",
        ),
        TypeObjetContribution::PhotoVisiteur => (
            "country_profile.photo_visiteur",
            "jsonb_build_object('id', id, 'chemin_fichier', chemin_fichier, 'legende', legende, 'format', format, 'taille_octets', taille_octets, 'largeur_px', largeur_px, 'hauteur_px', hauteur_px, 'fiche_pays_id', fiche_pays_id)",
        ),
        TypeObjetContribution::FichePays => return Ok(None),
    };

    let query = format!(
        "SELECT {} FROM {} WHERE id = $1 AND deleted_at IS NULL",
        table_colonnes.1, table_colonnes.0
    );

    let snapshot: Option<serde_json::Value> = sqlx::query_scalar(&query)
        .bind(target_id)
        .fetch_optional(pool)
        .await?
        .flatten();

    Ok(snapshot)
}

/// Valide le payload JSONB d'un site touristique (ajout / édition).
/// Règles (FR-003/FR-005/FR-006) :
///   • Requis : nom, gestionnaire, ville, info_pertinente, latitude, longitude, sous_type.
///   • sous_type cohérent avec la famille `categorie`.
///   • Si `categorie = prive` : au moins un contact (téléphone / courriel / adresse).
fn valider_site_touristique(payload: &serde_json::Value) -> Result<(), ApiErreur> {
    let texte_non_vide = |cle: &str| -> bool {
        payload
            .get(cle)
            .and_then(|v| v.as_str())
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
    };
    let nombre_present =
        |cle: &str| -> bool { payload.get(cle).map(|v| v.is_number()).unwrap_or(false) };

    let mut manquants: Vec<&str> = Vec::new();
    if !texte_non_vide("nom") {
        manquants.push("nom");
    }
    if !texte_non_vide("gestionnaire") {
        manquants.push("gestionnaire");
    }
    if !texte_non_vide("ville") {
        manquants.push("ville");
    }
    if !texte_non_vide("info_pertinente") {
        manquants.push("info pertinente");
    }
    if !texte_non_vide("sous_type") {
        manquants.push("sous-type");
    }
    if !nombre_present("latitude") {
        manquants.push("latitude");
    }
    if !nombre_present("longitude") {
        manquants.push("longitude");
    }
    if !manquants.is_empty() {
        return Err(ApiErreur::Validation(format!(
            "Champs requis manquants pour le site touristique : {}.",
            manquants.join(", ")
        )));
    }

    // Cohérence famille ↔ sous-type (FR-003).
    let categorie_str = payload
        .get("categorie")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_lowercase();
    let categorie = match categorie_str.as_str() {
        "emblematique" => CategorieSiteTouristique::Emblematique,
        "prive" => CategorieSiteTouristique::Prive,
        _ => {
            return Err(ApiErreur::Validation(
                "Famille de site invalide (emblematique ou prive attendu).".to_string(),
            ));
        }
    };
    let sous_type = payload
        .get("sous_type")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_lowercase();
    if !sous_type_appartient_a(&categorie, &sous_type) {
        return Err(ApiErreur::Validation(
            "Sous-type incompatible avec la famille du site.".to_string(),
        ));
    }

    // Contact requis pour un site privé (FR-006).
    if matches!(categorie, CategorieSiteTouristique::Prive) {
        let a_contact = texte_non_vide("contact_telephone")
            || texte_non_vide("contact_courriel")
            || texte_non_vide("contact_adresse");
        if !a_contact {
            return Err(ApiErreur::Validation(
                "Contact gestionnaire requis pour un site privé (téléphone, courriel ou adresse)."
                    .to_string(),
            ));
        }
    }

    // Lien de site web (facultatif) : schéma http/https uniquement (sécurité).
    if let Some(url) = payload.get("site_web_url").and_then(|v| v.as_str()) {
        let url = url.trim();
        if !url.is_empty() && !(url.starts_with("http://") || url.starts_with("https://")) {
            return Err(ApiErreur::Validation(
                "Le lien du site web doit commencer par http:// ou https://.".to_string(),
            ));
        }
    }

    Ok(())
}

/// GET /api/fiches-pays/{id}/contributions — Lister les contributions d'une fiche
pub async fn lister_contributions(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    params: web::Query<ContributionQueryParams>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let fiche_id_str = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_id_str)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Verifier si l'utilisateur est admin (acces a toutes les contributions)
    let est_admin = if let Some(uid) = extraire_utilisateur_id(&req) {
        verifier_admin(pool.get_ref(), uid).await.unwrap_or(false)
    } else {
        false
    };

    // Construction dynamique des conditions
    let mut conditions = vec![
        "cf.fiche_pays_id = $1".to_string(),
        "cf.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 2u32;
    let mut bind_values: Vec<String> = Vec::new();

    if est_admin {
        // Admin peut filtrer par etat
        if let Some(ref etat) = params.etat {
            let etat = etat.trim().to_lowercase();
            if ["en_attente", "approuvee", "rejetee"].contains(&etat.as_str()) {
                conditions.push(format!("cf.etat::text = ${}", bind_index));
                bind_values.push(etat);
                bind_index += 1;
            }
        }
    } else {
        // Public: seulement les approuvees
        conditions.push("cf.etat = 'approuvee'".to_string());
    }

    if let Some(ref section) = params.section {
        let section = section.trim().to_lowercase();
        if section_est_valide(&section) {
            conditions.push(format!("cf.section = ${}", bind_index));
            bind_values.push(section);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM country_profile.contribution_fiche cf WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query).bind(fiche_id);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les contributions
    let select_query = format!(
        "SELECT {} FROM country_profile.contribution_fiche cf
         JOIN iam.utilisateur u ON u.id = cf.cree_par
         WHERE {} ORDER BY cf.created_at DESC LIMIT ${} OFFSET ${}",
        CONTRIBUTION_COLONNES,
        where_clause,
        bind_index,
        bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, ContributionFicheRow>(&select_query).bind(fiche_id);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    let contributions: Vec<_> = rows.iter().map(construire_contribution_response).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ContributionListeResponse {
            contributions,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/fiches-pays/{id}/contributeurs — T066/T068
///
/// Liste agregee des contributeurs ayant au moins 1 contribution approuvee
/// sur la fiche pays. Anonymise les utilisateurs supprimes (`iam.utilisateur.deleted_at IS NOT NULL`) :
/// le libelle devient `{nom: "Contributeur", prenom: "retire", id: null, photo_url: null}`.
/// Inclut `date_derniere_contribution` (MAX(traite_at)) pour l'affichage relatif.
pub async fn lister_contributeurs(
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let fiche_id_str = chemin.into_inner();
    let fiche_id = Uuid::parse_str(&fiche_id_str)
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".to_string()))?;

    #[derive(sqlx::FromRow)]
    struct ContributeurAgrege {
        utilisateur_id: Option<Uuid>,
        nom: String,
        prenom: String,
        photo_url: Option<String>,
        nombre_contributions: i64,
        date_derniere_contribution: Option<chrono::DateTime<chrono::Utc>>,
    }

    let rows = sqlx::query_as::<_, ContributeurAgrege>(
        "SELECT
            CASE WHEN u.deleted_at IS NULL THEN u.id ELSE NULL END AS utilisateur_id,
            CASE WHEN u.deleted_at IS NULL THEN u.nom ELSE 'Contributeur' END AS nom,
            CASE WHEN u.deleted_at IS NULL THEN u.prenom ELSE 'retire' END AS prenom,
            CASE WHEN u.deleted_at IS NULL THEN u.photo_url ELSE NULL END AS photo_url,
            COUNT(*)::bigint AS nombre_contributions,
            MAX(cf.traite_at) AS date_derniere_contribution
         FROM country_profile.contribution_fiche cf
         LEFT JOIN iam.utilisateur u ON u.id = cf.cree_par
         WHERE cf.fiche_pays_id = $1
           AND cf.etat = 'approuvee'::country_profile.etat_contribution
           AND cf.deleted_at IS NULL
         GROUP BY u.id, u.nom, u.prenom, u.photo_url, u.deleted_at
         ORDER BY nombre_contributions DESC, MAX(cf.traite_at) DESC NULLS LAST",
    )
    .bind(fiche_id)
    .fetch_all(pool.get_ref())
    .await?;

    let contributeurs: Vec<serde_json::Value> = rows
        .iter()
        .map(|r| {
            json!({
                "utilisateur_id": r.utilisateur_id,
                "nom": r.nom,
                "prenom": r.prenom,
                "photo_url": r.photo_url,
                "nombre_contributions": r.nombre_contributions,
                "date_derniere_contribution": r.date_derniere_contribution,
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(contributeurs),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// T055-T057 : POST /api/fiches-pays/{id}/contributions/multipart
//
// Variante multipart pour contributions photo_visiteur (galerie photos US4).
// Parse champs texte + fichiers `photos[]` / `legendes[]`, valide chaque photo
// en memoire (taille, format, dimensions), puis ecrit dans
// `./uploads/opportunite-afrique/photos/<uuid>.<ext>` via sanitize-filename.
// Atomique : si UNE photo echoue, tous les fichiers deja ecrits sont nettoyes.
// ══════════════════════════════════════════════════════════════════════════

pub async fn soumettre_contribution_multipart(
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    chemin: web::Path<String>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let fiche_id = Uuid::parse_str(&chemin.into_inner())
        .map_err(|_| ApiErreur::Validation("ID de fiche invalide".into()))?;

    let fiche_existe: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM country_profile.fiche_pays WHERE id = $1)")
            .bind(fiche_id)
            .fetch_one(pool.get_ref())
            .await?;
    if !fiche_existe {
        return Err(ApiErreur::NonTrouve("Fiche pays non trouvee".into()));
    }

    // Parse multipart : champs texte + fichiers + legendes.
    let mut section_afripulse: Option<String> = None;
    let mut type_objet_str: Option<String> = None;
    let mut type_contribution_str: Option<String> = None;
    let mut justification: Option<String> = None;
    let mut photos: Vec<(Vec<u8>, String)> = Vec::new(); // (bytes, nom_original)
    let mut legendes: Vec<String> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|e| ApiErreur::Upload(format!("Erreur lecture multipart: {}", e)))?;
        let cd = field.content_disposition().cloned();
        let nom = cd
            .as_ref()
            .and_then(|c| c.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        // Fichier photo
        if nom == "photos" || nom == "photos[]" {
            let filename = cd
                .as_ref()
                .and_then(|c| c.get_filename().map(|f| sanitize_filename::sanitize(f)))
                .unwrap_or_else(|| format!("{}.bin", Uuid::new_v4()));
            let mut buf: Vec<u8> = Vec::new();
            while let Some(chunk) = field.next().await {
                let data =
                    chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture chunk: {}", e)))?;
                buf.extend_from_slice(&data);
                // Defense en profondeur : aborter si depassement taille max
                if buf.len() > image_validation::TAILLE_MAX_OCTETS + 1 {
                    return Err(ApiErreur::LimiteAtteinte(format!(
                        "Photo trop volumineuse (>{}octets)",
                        image_validation::TAILLE_MAX_OCTETS
                    )));
                }
            }
            photos.push((buf, filename));
        } else if nom == "legendes" || nom == "legendes[]" {
            let mut s = String::new();
            while let Some(chunk) = field.next().await {
                let data =
                    chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture chunk: {}", e)))?;
                s.push_str(&String::from_utf8_lossy(&data));
            }
            legendes.push(s.trim().to_string());
        } else {
            // Champ texte
            let mut s = String::new();
            while let Some(chunk) = field.next().await {
                let data =
                    chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture chunk: {}", e)))?;
                s.push_str(&String::from_utf8_lossy(&data));
            }
            match nom.as_str() {
                "section" | "section_afripulse" => section_afripulse = Some(s.trim().to_string()),
                "type_objet" | "type_objet_contribution" => {
                    type_objet_str = Some(s.trim().to_lowercase())
                }
                "type_contribution" => type_contribution_str = Some(s.trim().to_lowercase()),
                "justification" => justification = Some(s.trim().to_string()),
                _ => {}
            }
        }
    }

    if photos.is_empty() {
        return Err(ApiErreur::Validation(
            "Aucune photo recue (champ `photos`).".into(),
        ));
    }
    if photos.len() > 5 {
        return Err(ApiErreur::Validation(
            "Maximum 5 photos par soumission.".into(),
        ));
    }
    if legendes.len() != photos.len() {
        return Err(ApiErreur::Validation(format!(
            "{} photo(s) mais {} legende(s) — chaque photo doit avoir sa legende.",
            photos.len(),
            legendes.len()
        )));
    }
    for (i, leg) in legendes.iter().enumerate() {
        let n = leg.chars().count();
        if !(1..=500).contains(&n) {
            return Err(ApiErreur::Validation(format!(
                "Legende {} invalide : {} caracteres (1..500 requis).",
                i + 1,
                n
            )));
        }
    }

    let type_objet_str = type_objet_str.unwrap_or_else(|| "photo_visiteur".to_string());
    if type_objet_str != "photo_visiteur" {
        return Err(ApiErreur::Validation(
            "L'endpoint multipart est reserve a type_objet=photo_visiteur.".into(),
        ));
    }
    let type_contribution_str = type_contribution_str.unwrap_or_else(|| "ajout".to_string());

    // Rate-limit (photos_jour + attente par pays)
    if let Err(e) = rate_limit_afripulse::verifier_quotas(
        pool.get_ref(),
        utilisateur_id,
        &TypeObjetContribution::PhotoVisiteur,
        fiche_id,
    )
    .await
    {
        return Err(match e {
            rate_limit_afripulse::ErreurVerification::LimiteAtteinte(limite) => {
                ApiErreur::LimiteAtteinte(limite.message())
            }
            rate_limit_afripulse::ErreurVerification::Base(err) => {
                ApiErreur::BaseDeDonnees(err.to_string())
            }
        });
    }

    // Validation en memoire de toutes les photos — AVANT tout write disque (T056)
    let mut valides: Vec<(Vec<u8>, String, image_validation::DimensionsValides)> = Vec::new();
    for (bytes, filename) in photos.into_iter() {
        match image_validation::valider_photo_contribution(&bytes) {
            Ok(dim) => valides.push((bytes, filename, dim)),
            Err(err) => {
                return Err(ApiErreur::Validation(err.message()));
            }
        }
    }

    // Ecriture disque atomique : si ecriture echoue en cours, cleanup.
    let dossier = format!("{}/opportunite-afrique/photos", upload_dir.get_ref());
    std::fs::create_dir_all(&dossier)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le dossier photos: {}", e)))?;

    let mut pieces_jointes = Vec::<serde_json::Value>::new();
    let mut fichiers_ecrits: Vec<String> = Vec::new();

    for (idx, (bytes, _filename, dim)) in valides.into_iter().enumerate() {
        let ext = dim.format.extension();
        let nom_fichier = format!("{}.{}", Uuid::new_v4(), ext);
        let chemin_complet = format!("{}/{}", dossier, nom_fichier);
        let chemin_relatif = format!("opportunite-afrique/photos/{}", nom_fichier);

        let res = (|| -> std::io::Result<()> {
            let mut f = std::fs::File::create(&chemin_complet)?;
            f.write_all(&bytes)?;
            Ok(())
        })();
        if let Err(e) = res {
            // Rollback : supprimer les fichiers deja ecrits
            for f in &fichiers_ecrits {
                let _ = std::fs::remove_file(f);
            }
            return Err(ApiErreur::Upload(format!("Ecriture disque echouee: {}", e)));
        }
        fichiers_ecrits.push(chemin_complet);

        pieces_jointes.push(json!({
            "chemin_fichier": chemin_relatif,
            "legende": legendes.get(idx).cloned().unwrap_or_default(),
            "format": dim.format.as_str(),
            "taille_octets": dim.taille_octets as i64,
            "largeur": dim.largeur as i64,
            "hauteur": dim.hauteur as i64,
        }));
    }
    let pieces_jointes_json = serde_json::Value::Array(pieces_jointes);

    // INSERT contribution_fiche avec pieces_jointes JSONB
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);

    let insert_res: Result<(Uuid, chrono::DateTime<chrono::Utc>), sqlx::Error> = sqlx::query_as(
        "INSERT INTO country_profile.contribution_fiche (
            fiche_pays_id, cree_par, section, type_contribution,
            nouvelle_valeur, justification,
            type_objet_contribution, section_afripulse,
            pieces_jointes
         ) VALUES (
            $1, $2, 'galerie_photos', $3::country_profile.type_contribution,
            '', $4,
            'photo_visiteur'::country_profile.type_objet_contribution,
            COALESCE($5, 'galerie_photos')::country_profile.section_afripulse,
            $6
         ) RETURNING id, created_at",
    )
    .bind(fiche_id)
    .bind(utilisateur_id)
    .bind(&type_contribution_str)
    .bind(justification.as_deref())
    .bind(section_afripulse.as_deref())
    .bind(&pieces_jointes_json)
    .fetch_one(pool.get_ref())
    .await;

    let (contrib_id, created_at) = match insert_res {
        Ok(row) => row,
        Err(e) => {
            // Rollback disque
            for f in &fichiers_ecrits {
                let _ = std::fs::remove_file(f);
            }
            return Err(ApiErreur::BaseDeDonnees(e.to_string()));
        }
    };

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "create",
        "country_profile",
        "contribution_fiche",
        Some(contrib_id),
        None,
        Some(json!({
            "type_objet": "photo_visiteur",
            "type_contribution": type_contribution_str,
            "nombre_photos": pieces_jointes_json.as_array().map(|a| a.len()).unwrap_or(0),
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Accepted().json(ApiResponse {
        success: true,
        data: Some(json!({
            "id": contrib_id,
            "etat": "en_attente",
            "created_at": created_at,
            "nombre_photos": pieces_jointes_json.as_array().map(|a| a.len()).unwrap_or(0),
        })),
        error: None,
    }))
}

/// POST /api/fiches-pays/contributions/upload-image
///
/// Upload d'une image isolée pour une contribution Afripulse (image de site
/// touristique, portrait de personnalité). Valide en mémoire (JPEG/PNG, ≤ 2 Mo,
/// ≤ 2048 px), stocke sous `opportunite-afrique/images/<uuid>.<ext>` et retourne
/// l'URL relative. Cette URL est ensuite placée dans `nouvelle_valeur_jsonb`
/// (`image_url` / `portrait_url`) lors de la soumission de la contribution.
pub async fn uploader_image_contribution(
    upload_dir: web::Data<String>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    // Réservé aux utilisateurs connectés (comme toute contribution).
    extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    // Lire le champ fichier `image` (un seul attendu), avec garde-fou de taille.
    let mut bytes: Option<Vec<u8>> = None;
    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|e| ApiErreur::Upload(format!("Erreur lecture multipart: {}", e)))?;
        let nom = field
            .content_disposition()
            .and_then(|c| c.get_name().map(|s| s.to_string()))
            .unwrap_or_default();
        if nom == "image" {
            let mut buf: Vec<u8> = Vec::new();
            while let Some(chunk) = field.next().await {
                let data =
                    chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture chunk: {}", e)))?;
                buf.extend_from_slice(&data);
                if buf.len() > image_validation::TAILLE_MAX_OCTETS + 1 {
                    return Err(ApiErreur::LimiteAtteinte(format!(
                        "Image trop volumineuse (>{} octets)",
                        image_validation::TAILLE_MAX_OCTETS
                    )));
                }
            }
            bytes = Some(buf);
        }
    }

    let bytes =
        bytes.ok_or_else(|| ApiErreur::Validation("Aucune image reçue (champ `image`).".into()))?;

    // Validation en mémoire AVANT toute écriture disque.
    let dim = image_validation::valider_photo_contribution(&bytes)
        .map_err(|err| ApiErreur::Validation(err.message()))?;

    let dossier = format!("{}/opportunite-afrique/images", upload_dir.get_ref());
    std::fs::create_dir_all(&dossier)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le dossier images: {}", e)))?;

    let nom_fichier = format!("{}.{}", Uuid::new_v4(), dim.format.extension());
    let chemin_complet = format!("{}/{}", dossier, nom_fichier);
    let chemin_relatif = format!("/uploads/opportunite-afrique/images/{}", nom_fichier);

    let res = (|| -> std::io::Result<()> {
        let mut f = std::fs::File::create(&chemin_complet)?;
        f.write_all(&bytes)?;
        Ok(())
    })();
    if let Err(e) = res {
        return Err(ApiErreur::Upload(format!("Ecriture disque echouee: {}", e)));
    }

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(json!({
            "url": chemin_relatif,
            "format": dim.format.as_str(),
            "largeur": dim.largeur as i64,
            "hauteur": dim.hauteur as i64,
        })),
        error: None,
    }))
}

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::fiche_pays::{
    FichePaysDetailResponse, FichePaysListeResponse, FichePaysQueryParams,
    FichePaysResponse, FichePaysRow, GroupeEthniqueRow, FICHE_PAYS_COLONNES,
    formater_population, formater_superficie, parser_langues, region_depuis_code,
};

#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Compte le nombre de contributions approuvees pour une fiche
async fn compter_contributions(pool: &PgPool, fiche_id: Uuid) -> Result<i64, ApiErreur> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM country_profile.contribution_fiche
         WHERE fiche_pays_id = $1 AND etat = 'approuvee' AND deleted_at IS NULL"
    )
    .bind(fiche_id)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Charge les groupes ethniques d'une fiche
async fn charger_ethnies(pool: &PgPool, fiche_id: Uuid) -> Result<Vec<String>, ApiErreur> {
    let rows = sqlx::query_as::<_, GroupeEthniqueRow>(
        "SELECT id, nom, description FROM country_profile.groupe_ethnique
         WHERE fiche_pays_id = $1 ORDER BY nom"
    )
    .bind(fiche_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.nom).collect())
}

/// Construit un FichePaysResponse a partir d'une row
async fn construire_response(pool: &PgPool, row: &FichePaysRow) -> Result<FichePaysResponse, ApiErreur> {
    let nombre_contributions = compter_contributions(pool, row.id).await?;
    let region = region_depuis_code(row.pays_code.as_deref());

    Ok(FichePaysResponse {
        id: row.id,
        pays_id: row.pays_id,
        code: row.pays_code.clone(),
        nom: row.pays_nom.clone(),
        capitale: row.pays_capitale.clone(),
        image_couverture: row.image_couverture_url.clone(),
        slogan: row.slogan.clone(),
        superficie: formater_superficie(row.superficie_km2.as_deref()),
        population: formater_population(row.population),
        monnaie: row.monnaie.clone(),
        drapeau_url: row.image_drapeau_url.clone(),
        region,
        nombre_contributions,
        updated_at: row.updated_at,
    })
}

/// Construit un FichePaysDetailResponse a partir d'une row.
/// `utilisateur_id` (optionnel) sert à renseigner l'état personnel (réaction, signalement).
async fn construire_detail_response(
    pool: &PgPool,
    row: &FichePaysRow,
    utilisateur_id: Option<Uuid>,
) -> Result<FichePaysDetailResponse, ApiErreur> {
    let nombre_contributions = compter_contributions(pool, row.id).await?;
    let ethnies = charger_ethnies(pool, row.id).await?;
    let (ma_reaction, a_signale) =
        crate::handlers::fiche_pays_social::charger_etat_perso(pool, row.id, utilisateur_id).await?;
    let langues = parser_langues(
        row.langue_officielle.as_deref(),
        row.langues_populaires.as_deref(),
    );
    let region = region_depuis_code(row.pays_code.as_deref());

    Ok(FichePaysDetailResponse {
        id: row.id,
        pays_id: row.pays_id,
        code: row.pays_code.clone(),
        nom: row.pays_nom.clone(),
        capitale: row.pays_capitale.clone(),
        image_couverture: row.image_couverture_url.clone(),
        slogan: row.slogan.clone(),
        superficie: formater_superficie(row.superficie_km2.as_deref()),
        population: formater_population(row.population),
        monnaie: row.monnaie.clone(),
        drapeau_url: row.image_drapeau_url.clone(),
        embleme_url: row.image_embleme_url.clone(),
        devise: row.image_devise_url.clone(),
        hymne_national: row.hymne_national.clone(),
        langue_officielle: row.langue_officielle.clone(),
        langues,
        ethnies,
        region,
        biographie: row.biographie.clone(),
        contexte: row.contexte.clone(),
        fuseau_horaire: row.fuseau_horaire.clone(),
        voyage_langue_internationale: row.voyage_langue_internationale.clone(),
        voyage_langue_locale: row.voyage_langue_locale.clone(),
        voyage_infos_visa: row.voyage_infos_visa.clone(),
        voyage_infos_sanitaires: row.voyage_infos_sanitaires.clone(),
        voyage_meteo: row.voyage_meteo.clone(),
        voyage_prises_electriques: row.voyage_prises_electriques.clone(),
        voyage_contacts_tourisme: row.voyage_contacts_tourisme.clone(),
        voyage_recommandations_securite: row.voyage_recommandations_securite.clone(),
        nombre_contributions,
        nombre_likes: row.nombre_likes,
        nombre_dislikes: row.nombre_dislikes,
        nombre_signalements: row.nombre_signalements,
        bloquee: row.bloquee,
        ma_reaction,
        a_signale,
        updated_at: row.updated_at,
    })
}

// ────────────────────────────────────────────────────────────────
// Endpoints
// ────────────────────────────────────────────────────────────────

/// GET /api/fiches-pays — Liste paginee des fiches pays avec filtres
pub async fn lister_fiches(
    pool: web::Data<PgPool>,
    params: web::Query<FichePaysQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    // Plafond = périmètre Afripulse complet (54 États + Sahara occidental).
    let par_page = params.par_page.unwrap_or(20).clamp(1, 60);
    let offset = (page - 1) * par_page;

    // Construction dynamique du WHERE — exclut toujours les fiches bloquées
    let mut conditions: Vec<String> = vec!["fp.bloquee = FALSE".to_string()];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par recherche (nom du pays, capitale, code ISO)
    if let Some(ref recherche) = params.recherche {
        let terme = recherche.trim();
        if !terme.is_empty() {
            conditions.push(format!(
                "(LOWER(p.nom) LIKE ${idx} OR LOWER(p.capitale) LIKE ${idx} OR LOWER(p.code_iso2) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(format!("%{}%", terme.to_lowercase()));
            bind_index += 1;
        }
    }

    // Filtre par region (via code ISO)
    if let Some(ref region) = params.region {
        let codes = codes_region(region);
        if !codes.is_empty() {
            let placeholders: Vec<String> = codes.iter().enumerate().map(|(i, _)| {
                let idx = bind_index + i as u32;
                format!("${}", idx)
            }).collect();
            conditions.push(format!("p.code_iso2 IN ({})", placeholders.join(", ")));
            for code in &codes {
                bind_values.push(code.to_string());
                bind_index += 1;
            }
        }
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM country_profile.fiche_pays fp
         JOIN shared.pays p ON p.id = fp.pays_id
         {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les fiches
    let select_query = format!(
        "SELECT {} FROM country_profile.fiche_pays fp
         JOIN shared.pays p ON p.id = fp.pays_id
         {} ORDER BY p.nom ASC LIMIT ${} OFFSET ${}",
        FICHE_PAYS_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, FichePaysRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    // Construire les reponses
    let mut fiches = Vec::with_capacity(rows.len());
    for row in &rows {
        fiches.push(construire_response(pool.get_ref(), row).await?);
    }

    let total_pages = if total == 0 { 1 } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(FichePaysListeResponse {
            fiches,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/fiches-pays/{id} — Detail d'une fiche pays (par UUID ou code ISO)
pub async fn obtenir_fiche(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let identifiant = chemin.into_inner();
    let utilisateur_id = crate::handlers::fiche_pays_social::extraire_utilisateur_id(&req);

    // Essayer d'abord par UUID, sinon par code ISO ou slug
    let query = format!(
        "SELECT {} FROM country_profile.fiche_pays fp
         JOIN shared.pays p ON p.id = fp.pays_id
         WHERE fp.id::text = $1 OR LOWER(p.code_iso2) = LOWER($1) OR LOWER(p.nom) = LOWER($1)",
        FICHE_PAYS_COLONNES
    );

    let row = sqlx::query_as::<_, FichePaysRow>(&query)
        .bind(&identifiant)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Fiche pays '{}' non trouvee", identifiant)))?;

    // Fiche bloquée (≥ seuil de signalements) → retirée du public
    if row.bloquee {
        return Err(ApiErreur::NonTrouve(format!(
            "Fiche pays '{}' non disponible",
            identifiant
        )));
    }

    let detail = construire_detail_response(pool.get_ref(), &row, utilisateur_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(detail),
        error: None,
    }))
}

/// GET /api/fiches-pays/regions — Liste des regions disponibles
pub async fn lister_regions(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    // Recuperer tous les codes ISO des fiches existantes
    let codes: Vec<Option<String>> = sqlx::query_scalar(
        "SELECT DISTINCT p.code_iso2 FROM country_profile.fiche_pays fp
         JOIN shared.pays p ON p.id = fp.pays_id
         ORDER BY p.code_iso2"
    )
    .fetch_all(pool.get_ref())
    .await?;

    let mut regions: Vec<String> = codes.iter()
        .map(|c| region_depuis_code(c.as_deref()))
        .collect::<std::collections::HashSet<String>>()
        .into_iter()
        .collect();
    regions.sort();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(regions),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// Utilitaires
// ────────────────────────────────────────────────────────────────

/// Retourne les codes ISO2 correspondant a une region
fn codes_region(region: &str) -> Vec<&str> {
    match region {
        "Afrique de l'Ouest" => vec!["SN", "CI", "NG", "GH", "ML", "BF", "NE", "BJ", "TG", "GN", "SL", "LR", "MR", "GW", "GM", "CV"],
        "Afrique Centrale" => vec!["CM", "CD", "CG", "GA", "GQ", "CF", "TD", "ST"],
        "Afrique de l'Est" => vec!["KE", "ET", "TZ", "UG", "RW", "BI", "SS", "SO", "DJ", "ER", "MG", "MU", "SC", "KM"],
        "Afrique du Nord" => vec!["EG", "MA", "DZ", "TN", "LY", "SD"],
        "Afrique Australe" => vec!["ZA", "BW", "MZ", "ZW", "NA", "SZ", "LS", "ZM", "MW", "AO"],
        _ => vec![],
    }
}

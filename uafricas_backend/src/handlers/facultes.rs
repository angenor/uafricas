use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::faculte::{
    ConditionsAdmissionResponse, EcolePartenaireResponse, EcolePartenaireRow,
    FaculteDetailResponse, FaculteListeResponse, FaculteQueryParams, FaculteResponse,
    FaculteRow, FraisScolariteResponse, ProgrammesResumeResponse, ReferentInfo,
    ReferentResponse, StatsResponse, ECOLE_COLONNES, FACULTE_COLONNES,
};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Charger le nom du pays
async fn charger_nom_pays(pool: &PgPool, pays_id: Uuid) -> Result<String, ApiErreur> {
    sqlx::query_scalar("SELECT nom FROM shared.pays WHERE id = $1")
        .bind(pays_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Pays non trouve".to_string()))
}

/// Charger l'ecole partenaire
async fn charger_ecole(pool: &PgPool, ecole_id: Uuid) -> Result<EcolePartenaireRow, ApiErreur> {
    let query = format!(
        "SELECT {} FROM exchange.ecole_partenaire e WHERE e.id = $1",
        ECOLE_COLONNES
    );
    sqlx::query_as::<_, EcolePartenaireRow>(&query)
        .bind(ecole_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Ecole partenaire non trouvee".to_string()))
}

/// Construire EcolePartenaireResponse depuis la row + pays
async fn construire_ecole_response(
    pool: &PgPool,
    ecole: &EcolePartenaireRow,
) -> Result<EcolePartenaireResponse, ApiErreur> {
    let pays_nom = charger_nom_pays(pool, ecole.pays_id).await?;
    Ok(EcolePartenaireResponse {
        nom: ecole.nom.clone(),
        ville: ecole.ville.clone(),
        pays: pays_nom,
        type_ecole: ecole.type_ecole.clone(),
        site_web: ecole.site_web.clone(),
        email_contact: ecole.email_contact.clone(),
        telephone_contact: ecole.telephone_contact.clone(),
        whatsapp_contact: ecole.whatsapp_contact.clone(),
    })
}

/// Construire un FaculteResponse (listing) a partir d'une row
async fn construire_response_liste(
    pool: &PgPool,
    row: &FaculteRow,
) -> Result<FaculteResponse, ApiErreur> {
    let ecole = charger_ecole(pool, row.ecole_partenaire_id).await?;
    let ecole_response = construire_ecole_response(pool, &ecole).await?;

    Ok(FaculteResponse {
        id: row.id,
        titre: row.titre.clone(),
        acronyme: row.acronyme.clone(),
        description: row.description.clone(),
        image_couverture: row.image_couverture_url.clone(),
        ecole_partenaire: ecole_response,
        domaines_etudes: row.domaines_etudes.clone(),
        accepte_nouveaux_inscrits: row.accepte_nouveaux_inscrits,
        stats: StatsResponse {
            nombre_inscrits_total: row.nombre_inscrits_total,
            nombre_inscrits_annee_en_cours: row.nombre_inscrits_annee,
        },
    })
}

/// Recuperer les domaines uniques de toutes les facultes actives
async fn charger_domaines_uniques(pool: &PgPool) -> Result<Vec<String>, ApiErreur> {
    let domaines: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT UNNEST(domaines_etudes) AS domaine
         FROM exchange.faculte
         WHERE statut = 'active' AND deleted_at IS NULL
         ORDER BY domaine",
    )
    .fetch_all(pool)
    .await?;
    Ok(domaines)
}

// ──────────────────────────────────────────────────────────────
// GET /api/facultes — Lister les facultes avec filtres et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_facultes(
    pool: web::Data<PgPool>,
    params: web::Query<FaculteQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Construire les conditions WHERE
    let mut conditions: Vec<String> = vec![
        "f.statut = 'active'".to_string(),
        "f.deleted_at IS NULL".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par recherche textuelle
    if let Some(ref recherche) = params.recherche {
        if !recherche.is_empty() {
            let terme = format!("%{}%", recherche.to_lowercase());
            conditions.push(format!(
                "(LOWER(f.titre) LIKE ${idx} OR LOWER(f.acronyme) LIKE ${idx} OR LOWER(f.description) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    // Filtre par domaine
    if let Some(ref domaine) = params.domaine {
        if !domaine.is_empty() {
            conditions.push(format!("${} = ANY(f.domaines_etudes)", bind_index));
            bind_values.push(domaine.clone());
            bind_index += 1;
        }
    }

    // Filtre par type d'ecole (publique/privee)
    if let Some(ref type_ecole) = params.type_ecole {
        if !type_ecole.is_empty() {
            // Peut etre une liste separee par virgules : "publique,privee"
            let types: Vec<&str> = type_ecole.split(',').collect();
            if types.len() == 1 {
                conditions.push(format!(
                    "e.type = ${}",
                    bind_index
                ));
                bind_values.push(types[0].to_string());
                bind_index += 1;
            }
        }
    }

    // Filtre inscriptions ouvertes
    if let Some(ref ouvertes) = params.ouvertes {
        if ouvertes == "true" || ouvertes == "1" {
            conditions.push("f.accepte_nouveaux_inscrits = TRUE".to_string());
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*)
         FROM exchange.faculte f
         JOIN exchange.ecole_partenaire e ON e.id = f.ecole_partenaire_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les facultes
    let select_query = format!(
        "SELECT {}
         FROM exchange.faculte f
         JOIN exchange.ecole_partenaire e ON e.id = f.ecole_partenaire_id
         WHERE {} ORDER BY f.titre ASC LIMIT ${} OFFSET ${}",
        FACULTE_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, FaculteRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    // Construire les reponses
    let mut facultes = Vec::with_capacity(rows.len());
    for row in &rows {
        facultes.push(construire_response_liste(pool.get_ref(), row).await?);
    }

    // Charger les domaines uniques pour les filtres
    let domaines = charger_domaines_uniques(pool.get_ref()).await?;

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(FaculteListeResponse {
            facultes,
            domaines,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/universite/stats — Statistiques agregees (bande d'accueil)
// ──────────────────────────────────────────────────────────────
#[derive(serde::Serialize)]
pub struct UniversiteStatsResponse {
    pub nombre_facultes: i64,
    pub nombre_formations: i64,
    pub nombre_inscrits: i64,
    pub nombre_pays: i64,
}

pub async fn stats_universite(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let pool = pool.get_ref();

    // Facultes actives
    let nombre_facultes: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM exchange.faculte
         WHERE statut = 'active' AND deleted_at IS NULL",
    )
    .fetch_one(pool)
    .await?;

    // Formations disponibles (memes etats que le listing public)
    let nombre_formations: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc
         WHERE etat IN ('publie','en_cours','termine') AND deleted_at IS NULL",
    )
    .fetch_one(pool)
    .await?;

    // Apprenants : inscriptions reelles aux formations + effectifs declares des facultes
    let inscrits_moocs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.mooc_inscription i
         JOIN media_content.mooc m ON m.id = i.mooc_id
         WHERE i.statut != 'abandonne' AND m.deleted_at IS NULL",
    )
    .fetch_one(pool)
    .await?;

    let inscrits_facultes: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(nombre_inscrits_total), 0) FROM exchange.faculte
         WHERE statut = 'active' AND deleted_at IS NULL",
    )
    .fetch_one(pool)
    .await?;

    // Territoires representes (facultes via ecole + formations)
    let nombre_pays: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM (
            SELECT e.pays_id AS pays_id
            FROM exchange.faculte f
            JOIN exchange.ecole_partenaire e ON e.id = f.ecole_partenaire_id
            WHERE f.statut = 'active' AND f.deleted_at IS NULL
            UNION
            SELECT m.pays_id AS pays_id
            FROM media_content.mooc m
            WHERE m.pays_id IS NOT NULL
              AND m.etat IN ('publie','en_cours','termine') AND m.deleted_at IS NULL
         ) AS territoires
         WHERE pays_id IS NOT NULL",
    )
    .fetch_one(pool)
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(UniversiteStatsResponse {
            nombre_facultes,
            nombre_formations,
            nombre_inscrits: inscrits_moocs + inscrits_facultes,
            nombre_pays,
        }),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/facultes/{id} — Obtenir le detail d'une faculte
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_faculte(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {}
         FROM exchange.faculte f
         WHERE f.id = $1 AND f.deleted_at IS NULL",
        FACULTE_COLONNES
    );

    let row = sqlx::query_as::<_, FaculteRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Faculte avec id {} non trouvee", id)))?;

    // Charger l'ecole partenaire + pays
    let ecole = charger_ecole(pool.get_ref(), row.ecole_partenaire_id).await?;
    let ecole_response = construire_ecole_response(pool.get_ref(), &ecole).await?;

    // Charger le referent plateforme si present
    let referent = if let Some(ref_id) = row.referent_id {
        let info = sqlx::query_as::<_, ReferentInfo>(
            "SELECT nom, prenom, email FROM iam.utilisateur WHERE id = $1",
        )
        .bind(ref_id)
        .fetch_optional(pool.get_ref())
        .await?;

        info.map(|r| ReferentResponse {
            prenom: r.prenom,
            nom: r.nom,
            role: "Coordinateur INUDA".to_string(),
            email: r.email,
        })
    } else {
        None
    };

    let reponse = FaculteDetailResponse {
        id: row.id,
        titre: row.titre.clone(),
        acronyme: row.acronyme.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        image_couverture: row.image_couverture_url.clone(),
        logo: row.logo_url.clone(),
        ecole_partenaire: ecole_response,
        domaines_etudes: row.domaines_etudes.clone(),
        programmes_resume: ProgrammesResumeResponse {
            licence: row.programmes_licence.clone(),
            master: row.programmes_master.clone(),
            doctorat: row.programmes_doctorat.clone(),
            certificats: row.programmes_certificats.clone(),
        },
        conditions_admission: ConditionsAdmissionResponse {
            diplome_minimum: row.diplome_minimum.clone(),
            langues_enseignement: row.langues_enseignement.clone(),
            frais_scolarite_annuels: FraisScolariteResponse {
                min: row.frais_scolarite_min,
                max: row.frais_scolarite_max,
                bourses_possibles: row.bourses_possibles,
            },
            periodes_inscription: row.periodes_inscription.clone(),
        },
        points_forts: row.points_forts.clone(),
        accepte_nouveaux_inscrits: row.accepte_nouveaux_inscrits,
        statut: row.statut.clone(),
        stats: StatsResponse {
            nombre_inscrits_total: row.nombre_inscrits_total,
            nombre_inscrits_annee_en_cours: row.nombre_inscrits_annee,
        },
        referent_plateforme: referent,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

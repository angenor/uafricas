//! Réactions (like/dislike) et partages communautaires des SOUS-OBJETS afripulse
//! (secteur, recette, site touristique, personnalité). Membres authentifiés
//! pour les mutations. Design générique par (type_objet, objet_id), cf. 11k.
//! Calqué sur `fiche_pays_social`.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::fiche_pays_social::extraire_utilisateur_id;
use crate::models::element_social::{
    table_pour_type, PartageAuteur, PartageElementApercu, PartageElementListeResponse,
    PartageElementRequest, PartageElementResponse, PartageElementRow, PartageQueryParams,
    ReactionElementEtat, ReactionElementRequest, TYPES_ELEMENT_AUTORISES,
};

#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Exige un utilisateur connecté, sinon NonAutorise.
fn exiger_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

/// Valide le type d'objet et parse l'identifiant.
fn valider_cible(type_objet: &str, objet_brut: &str) -> Result<Uuid, ApiErreur> {
    if !TYPES_ELEMENT_AUTORISES.contains(&type_objet) {
        return Err(ApiErreur::Validation(format!(
            "type_objet '{}' non supporté",
            type_objet
        )));
    }
    Uuid::parse_str(objet_brut)
        .map_err(|_| ApiErreur::Validation("Identifiant d'objet invalide".to_string()))
}

/// Vérifie que le sous-objet existe et n'est pas suspendu (ni supprimé).
async fn verifier_element(
    pool: &PgPool,
    type_objet: &str,
    objet_id: Uuid,
) -> Result<(), ApiErreur> {
    let (table, has_deleted) = table_pour_type(type_objet)
        .ok_or_else(|| ApiErreur::Validation("type_objet non supporté".to_string()))?;
    let filtre_del = if has_deleted { " AND deleted_at IS NULL" } else { "" };

    let suspendu: Option<bool> = sqlx::query_scalar(&format!(
        "SELECT suspendu FROM {table} WHERE id = $1{filtre_del}"
    ))
    .bind(objet_id)
    .fetch_optional(pool)
    .await?;

    match suspendu {
        None => Err(ApiErreur::NonTrouve("Élément introuvable".to_string())),
        Some(true) => Err(ApiErreur::NonTrouve("Élément non disponible".to_string())),
        Some(false) => Ok(()),
    }
}

// ────────────────────────────────────────────────────────────────
// POST /api/fiches-pays/elements/{type_objet}/{objet_id}/reaction
// ────────────────────────────────────────────────────────────────

pub async fn reagir_element(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, String)>,
    body: web::Json<ReactionElementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let (type_objet, objet_brut) = chemin.into_inner();
    let objet_id = valider_cible(&type_objet, &objet_brut)?;

    let type_reaction = body.type_reaction.trim();
    if type_reaction != "like" && type_reaction != "dislike" {
        return Err(ApiErreur::Validation(
            "type_reaction doit être 'like' ou 'dislike'".to_string(),
        ));
    }

    verifier_element(pool.get_ref(), &type_objet, objet_id).await?;

    // Réaction existante ?
    let existante: Option<String> = sqlx::query_scalar(
        "SELECT type_reaction FROM country_profile.reaction_element
         WHERE type_objet = $1::country_profile.type_objet_contribution
           AND objet_id = $2 AND utilisateur_id = $3",
    )
    .bind(&type_objet)
    .bind(objet_id)
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let ma_reaction: Option<String> = match existante.as_deref() {
        // Même réaction → toggle off
        Some(actuelle) if actuelle == type_reaction => {
            sqlx::query(
                "DELETE FROM country_profile.reaction_element
                 WHERE type_objet = $1::country_profile.type_objet_contribution
                   AND objet_id = $2 AND utilisateur_id = $3",
            )
            .bind(&type_objet)
            .bind(objet_id)
            .bind(utilisateur_id)
            .execute(pool.get_ref())
            .await?;
            None
        }
        // Réaction différente → bascule like↔dislike
        Some(_) => {
            sqlx::query(
                "UPDATE country_profile.reaction_element SET type_reaction = $4
                 WHERE type_objet = $1::country_profile.type_objet_contribution
                   AND objet_id = $2 AND utilisateur_id = $3",
            )
            .bind(&type_objet)
            .bind(objet_id)
            .bind(utilisateur_id)
            .bind(type_reaction)
            .execute(pool.get_ref())
            .await?;
            Some(type_reaction.to_string())
        }
        // Aucune → insertion
        None => {
            sqlx::query(
                "INSERT INTO country_profile.reaction_element
                    (type_objet, objet_id, utilisateur_id, type_reaction)
                 VALUES ($1::country_profile.type_objet_contribution, $2, $3, $4)",
            )
            .bind(&type_objet)
            .bind(objet_id)
            .bind(utilisateur_id)
            .bind(type_reaction)
            .execute(pool.get_ref())
            .await?;
            Some(type_reaction.to_string())
        }
    };

    let (nombre_likes, nombre_dislikes): (i64, i64) = sqlx::query_as(
        "SELECT
            COUNT(*) FILTER (WHERE type_reaction = 'like'),
            COUNT(*) FILTER (WHERE type_reaction = 'dislike')
         FROM country_profile.reaction_element
         WHERE type_objet = $1::country_profile.type_objet_contribution AND objet_id = $2",
    )
    .bind(&type_objet)
    .bind(objet_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Engagement : 1 point à l'auteur de l'élément par « j'aime » reçu.
    //
    // Le `type_objet` transmis est le **sous-type reçu dans l'URL**, jamais une
    // valeur générique « element » : `reaction_element` est générique par
    // `(type_objet, objet_id)` sur quatre tables distinctes, et sans le sous-type
    // rien ne dirait laquelle interroger pour trouver l'auteur.
    //
    // Seuls `personnalite_connue` et `recette_culinaire` ont un `cree_par` :
    // `site_touristique` et `secteur_developpement` sont des contenus éditoriaux
    // rattachés à une fiche pays, sans aucune colonne d'auteur. Ils ne créditent
    // donc personne : et ce n'est PAS une erreur (FR-008c) : la réaction est
    // enregistrée, le compteur s'incrémente, rien n'échoue.
    if ma_reaction.as_deref() == Some("like") {
        if let Some(auteur_id) =
            crate::services::engagement::resoudre_beneficiaire(pool.get_ref(), &type_objet, objet_id)
                .await
        {
            crate::services::engagement::crediter_jaime(
                pool.get_ref(),
                &type_objet,
                objet_id,
                auteur_id,
                utilisateur_id,
            )
            .await;
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ReactionElementEtat {
            nombre_likes: nombre_likes as i32,
            nombre_dislikes: nombre_dislikes as i32,
            ma_reaction,
        }),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// POST /api/fiches-pays/elements/{type_objet}/{objet_id}/partages
// ────────────────────────────────────────────────────────────────

pub async fn partager_element(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, String)>,
    body: web::Json<PartageElementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let (type_objet, objet_brut) = chemin.into_inner();
    let objet_id = valider_cible(&type_objet, &objet_brut)?;

    verifier_element(pool.get_ref(), &type_objet, objet_id).await?;

    let legende = body
        .legende
        .as_deref()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string());

    if let Some(ref l) = legende {
        if l.chars().count() > 500 {
            return Err(ApiErreur::Validation(
                "La légende ne doit pas dépasser 500 caractères".to_string(),
            ));
        }
    }

    let partage_id: Uuid = sqlx::query_scalar(
        "INSERT INTO country_profile.partage_element (type_objet, objet_id, utilisateur_id, legende)
         VALUES ($1::country_profile.type_objet_contribution, $2, $3, $4) RETURNING id",
    )
    .bind(&type_objet)
    .bind(objet_id)
    .bind(utilisateur_id)
    .bind(&legende)
    .fetch_one(pool.get_ref())
    .await?;

    // Engagement : 1 point à l'auteur de l'élément par partage reçu.
    // Même règle qu'à la réaction : c'est le **sous-type reçu** qui est transmis,
    // et les deux sous-types éditoriaux sans auteur ne créditent personne.
    if let Some(auteur_id) =
        crate::services::engagement::resoudre_beneficiaire(pool.get_ref(), &type_objet, objet_id)
            .await
    {
        crate::services::engagement::crediter_partage(
            pool.get_ref(),
            &type_objet,
            objet_id,
            auteur_id,
            utilisateur_id,
        )
        .await;
    }

    let row = charger_partage(pool.get_ref(), partage_id)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Partage non trouvé".to_string()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(partage_depuis_row(row)),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// GET /api/fiches-pays/elements/partages, mur communautaire (public)
// ────────────────────────────────────────────────────────────────

pub async fn lister_partages_elements(
    pool: web::Data<PgPool>,
    params: web::Query<PartageQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let count_union = TYPES_ELEMENT_AUTORISES
        .iter()
        .map(|t| format!("SELECT pe.id {}", from_where(t)))
        .collect::<Vec<_>>()
        .join(" UNION ALL ");
    let total: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM ({count_union}) s"))
        .fetch_one(pool.get_ref())
        .await?;

    let sql = format!(
        "SELECT * FROM ({}) sub ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        union_select()
    );
    let rows = sqlx::query_as::<_, PartageElementRow>(&sql)
        .bind(par_page)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

    let partages: Vec<PartageElementResponse> = rows.into_iter().map(partage_depuis_row).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PartageElementListeResponse {
            partages,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// Construction dynamique de la requête UNION (4 types de sous-objets)
// ────────────────────────────────────────────────────────────────

/// Clause FROM…JOIN…WHERE commune à un type d'objet donné.
fn from_where(type_objet: &str) -> String {
    // type_objet provient exclusivement de TYPES_ELEMENT_AUTORISES → pas d'injection.
    let (table, has_deleted) = table_pour_type(type_objet).expect("type d'objet supporté");
    let filtre_del = if has_deleted { " AND el.deleted_at IS NULL" } else { "" };
    format!(
        "FROM country_profile.partage_element pe
         JOIN {table} el ON el.id = pe.objet_id{filtre_del}
         JOIN country_profile.fiche_pays fp ON fp.id = el.fiche_pays_id
         JOIN shared.pays p ON p.id = fp.pays_id
         JOIN iam.utilisateur u ON u.id = pe.utilisateur_id
         WHERE pe.deleted_at IS NULL
           AND pe.type_objet = '{type_objet}'::country_profile.type_objet_contribution
           AND el.suspendu = FALSE AND fp.bloquee = FALSE"
    )
}

/// Expressions (titre, image_url) propres à chaque type d'objet.
fn titre_image(type_objet: &str) -> (&'static str, &'static str) {
    match type_objet {
        "secteur_developpement" => ("el.nom", "el.image_url"),
        "recette_culinaire" => ("el.titre", "el.images[1]"),
        "site_touristique" => ("el.nom", "COALESCE(el.image_url, el.images[1])"),
        "personnalite_connue" => ("el.nom_complet", "el.portrait_url"),
        _ => ("''", "NULL"),
    }
}

/// Fragment SELECT complet pour un type.
fn select_fragment(type_objet: &str) -> String {
    let (titre, image) = titre_image(type_objet);
    format!(
        "SELECT pe.id, pe.legende, pe.created_at, pe.type_objet::text AS type_objet,
                pe.objet_id, el.fiche_pays_id, p.nom AS territoire_nom,
                {titre} AS titre, {image} AS image_url,
                u.id AS auteur_id,
                CASE WHEN u.deleted_at IS NULL THEN u.nom ELSE 'Membre' END AS auteur_nom,
                CASE WHEN u.deleted_at IS NULL THEN u.prenom ELSE 'retiré' END AS auteur_prenom,
                CASE WHEN u.deleted_at IS NULL THEN u.photo_url ELSE NULL END AS auteur_photo_url
         {from_where}",
        from_where = from_where(type_objet)
    )
}

/// UNION ALL des 4 types.
fn union_select() -> String {
    TYPES_ELEMENT_AUTORISES
        .iter()
        .map(|t| select_fragment(t))
        .collect::<Vec<_>>()
        .join(" UNION ALL ")
}

/// Recharge un partage enrichi par son id (carte du mur).
async fn charger_partage(pool: &PgPool, partage_id: Uuid) -> Result<Option<PartageElementRow>, ApiErreur> {
    let sql = format!("SELECT * FROM ({}) sub WHERE id = $1", union_select());
    let row = sqlx::query_as::<_, PartageElementRow>(&sql)
        .bind(partage_id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

fn partage_depuis_row(row: PartageElementRow) -> PartageElementResponse {
    PartageElementResponse {
        id: row.id,
        legende: row.legende,
        created_at: row.created_at,
        element: PartageElementApercu {
            type_objet: row.type_objet,
            objet_id: row.objet_id,
            fiche_pays_id: row.fiche_pays_id,
            territoire_nom: row.territoire_nom,
            titre: row.titre,
            image_url: row.image_url,
        },
        auteur: PartageAuteur {
            id: row.auteur_id,
            nom: row.auteur_nom,
            prenom: row.auteur_prenom,
            photo_url: row.auteur_photo_url,
        },
    }
}

//! Équipes éditoriales — lecture groupée, écriture par remplacement, suggestions
//! (feature 010-medias-equipes-vitrine, migration 09t).
//!
//!   GET       /api/medias/equipe/fonctions
//!   GET | PUT /api/medias/{type_porteur}/{porteur_id}/equipe
//!   GET | PUT /api/admin/medias/{type_porteur}/{porteur_id}/equipe
//!
//! Les routes `/api/medias/**` s'adressent à des **MEMBRES** : la garde est
//! `garde_detenteur`, jamais `AdminUtilisateur` — cet extracteur rejetterait
//! tout non-admin. L'erreur inverse a été commise et corrigée en 009.
//!
//! Le public ne passe pas par ces routes : il lit l'équipe dans les payloads de
//! support et de programme, où elle est greffée (D7). `obtenir_equipe` sert à
//! repeupler le formulaire d'édition.

use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_detention::{exiger_utilisateur_id, garde_detenteur};
use crate::handlers::media_emission::contexte_emission;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::media_equipe::{
    est_programme, normaliser_optionnel, table_porteur, type_support_du_porteur,
    valider_type_porteur, EquipeRequest, MembreEquipeRequest, MembreEquipeResponse,
    MembreEquipeRow, MEMBRE_EQUIPE_COLONNES,
};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ═══════════════════════════════════════════════════════════════════════════
// Lecture groupée — sans requête N+1
// ═══════════════════════════════════════════════════════════════════════════

/// Équipes de plusieurs porteurs du **même** discriminant, en une seule requête.
///
/// Le rattachement à un compte est résolu par la jointure et non par la colonne :
/// `u.deleted_at IS NULL` est porté par la **condition de jointure**, si bien
/// qu'un compte fermé donne `compte_id = NULL` et donc un nom en texte simple.
/// Remonter cette condition dans le `WHERE` transformerait la jointure externe
/// en jointure interne et ferait disparaître le membre de l'équipe (FR-014).
pub async fn equipes_par_porteurs(
    pool: &PgPool,
    type_porteur: &str,
    porteur_ids: &[Uuid],
) -> Result<HashMap<Uuid, Vec<MembreEquipeResponse>>, ApiErreur> {
    let mut resultat: HashMap<Uuid, Vec<MembreEquipeResponse>> = HashMap::new();
    if porteur_ids.is_empty() {
        return Ok(resultat);
    }
    valider_type_porteur(type_porteur)?;

    let lignes = sqlx::query_as::<_, MembreEquipeRow>(&format!(
        "SELECT {MEMBRE_EQUIPE_COLONNES}, u.id AS compte_id
           FROM media_content.membre_equipe m
           LEFT JOIN iam.utilisateur u
                  ON u.id = m.utilisateur_id AND u.deleted_at IS NULL
          WHERE m.type_porteur = $1 AND m.porteur_id = ANY($2) AND m.deleted_at IS NULL
          ORDER BY m.ordre ASC, m.created_at ASC"
    ))
    .bind(type_porteur)
    .bind(porteur_ids)
    .fetch_all(pool)
    .await?;

    for ligne in lignes {
        resultat
            .entry(ligne.porteur_id)
            .or_default()
            .push(ligne.to_response());
    }
    Ok(resultat)
}

/// Équipe d'un porteur unique — commodité au-dessus de la lecture groupée.
pub async fn equipe_du_porteur(
    pool: &PgPool,
    type_porteur: &str,
    porteur_id: Uuid,
) -> Result<Vec<MembreEquipeResponse>, ApiErreur> {
    let map = equipes_par_porteurs(pool, type_porteur, &[porteur_id]).await?;
    Ok(map.get(&porteur_id).cloned().unwrap_or_default())
}

// ═══════════════════════════════════════════════════════════════════════════
// Cycle de vie — nettoyage à la suppression du porteur (FR-019)
// ═══════════════════════════════════════════════════════════════════════════

/// Suppression douce de l'équipe d'un porteur, **dans la transaction** de la
/// suppression du porteur lui-même.
///
/// `porteur_id` n'a pas de clé étrangère (prix du polymorphisme) : aucune
/// cascade n'est possible, le nettoyage est explicite. L'oublier ne casse rien
/// de visible — les équipes orphelines restent simplement dans le référentiel de
/// suggestions de fonctions.
pub async fn supprimer_equipe_du_porteur(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    type_porteur: &str,
    porteur_id: Uuid,
) -> Result<(), ApiErreur> {
    sqlx::query(
        "UPDATE media_content.membre_equipe
            SET deleted_at = NOW(), updated_at = NOW()
          WHERE type_porteur = $1 AND porteur_id = $2 AND deleted_at IS NULL",
    )
    .bind(type_porteur)
    .bind(porteur_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

/// Même nettoyage, hors transaction — pour les chemins de suppression qui n'en
/// ouvrent pas.
pub async fn supprimer_equipe_du_porteur_pool(
    pool: &PgPool,
    type_porteur: &str,
    porteur_id: Uuid,
) -> Result<(), ApiErreur> {
    sqlx::query(
        "UPDATE media_content.membre_equipe
            SET deleted_at = NOW(), updated_at = NOW()
          WHERE type_porteur = $1 AND porteur_id = $2 AND deleted_at IS NULL",
    )
    .bind(type_porteur)
    .bind(porteur_id)
    .execute(pool)
    .await?;
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// Écriture — règles communes membre et back-office
// ═══════════════════════════════════════════════════════════════════════════

async fn exiger_porteur_existant(
    pool: &PgPool,
    type_porteur: &str,
    porteur_id: Uuid,
) -> Result<(), ApiErreur> {
    let table = table_porteur(type_porteur)?;
    let existe: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table} WHERE id = $1 AND deleted_at IS NULL)"
    ))
    .bind(porteur_id)
    .fetch_one(pool)
    .await?;

    if existe {
        Ok(())
    } else {
        Err(ApiErreur::NonTrouve(if est_programme(type_porteur) {
            "Programme introuvable".into()
        } else {
            "Support introuvable".into()
        }))
    }
}

/// Remplacement intégral et ordonné de l'équipe d'un porteur (D6).
///
/// `ordre` vaut l'index reçu : réordonner, c'est renvoyer la liste dans le
/// nouvel ordre. La `fonction` est normalisée à l'écriture (`btrim` + espaces
/// internes réduits) sans quoi « Directeur » et « Directeur  » constitueraient
/// deux entrées du référentiel de suggestions (FR-015).
///
/// Chaque `PUT` réattribue les identifiants — aucune table ne référence un
/// membre d'équipe, c'est ce qui autorise le remplacement intégral.
async fn appliquer_equipe(
    pool: &PgPool,
    type_porteur: &str,
    porteur_id: Uuid,
    membres: &[MembreEquipeRequest],
    auteur: Uuid,
) -> Result<Vec<MembreEquipeResponse>, ApiErreur> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "DELETE FROM media_content.membre_equipe
          WHERE type_porteur = $1 AND porteur_id = $2",
    )
    .bind(type_porteur)
    .bind(porteur_id)
    .execute(&mut *tx)
    .await?;

    for (index, membre) in membres.iter().enumerate() {
        sqlx::query(
            "INSERT INTO media_content.membre_equipe
                 (type_porteur, porteur_id, nom, prenom, fonction, territoire,
                  contact, utilisateur_id, ordre, cree_par)
             VALUES ($1, $2, btrim($3),
                     $4,
                     btrim(regexp_replace($5, '\\s+', ' ', 'g')),
                     $6, $7, $8, $9, $10)",
        )
        .bind(type_porteur)
        .bind(porteur_id)
        .bind(&membre.nom)
        .bind(normaliser_optionnel(&membre.prenom))
        .bind(&membre.fonction)
        .bind(normaliser_optionnel(&membre.territoire))
        // Jamais dérivé de iam.utilisateur.email, même quand le rattachement
        // existe : un rattachement ne transforme pas une adresse de compte en
        // donnée publique (D2).
        .bind(normaliser_optionnel(&membre.contact))
        .bind(membre.utilisateur_id)
        .bind(index as i32)
        .bind(auteur)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    equipe_du_porteur(pool, type_porteur, porteur_id).await
}

/// Journalisation de la mutation (Principe VII, FR-018), **après le commit** :
/// une écriture d'audit ne doit jamais faire échouer la transaction métier.
#[allow(clippy::too_many_arguments)]
async fn journaliser_equipe(
    req: &HttpRequest,
    pool: &PgPool,
    auteur: Uuid,
    type_porteur: &str,
    porteur_id: Uuid,
    avant: Vec<MembreEquipeResponse>,
    apres: &[MembreEquipeResponse],
) {
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(auteur),
        "equipe_modifiee",
        "media_content",
        "membre_equipe",
        Some(porteur_id),
        Some(serde_json::json!({
            "type_porteur": type_porteur,
            "membres": serde_json::to_value(&avant).unwrap_or(serde_json::Value::Null),
        })),
        Some(serde_json::json!({
            "type_porteur": type_porteur,
            "membres": serde_json::to_value(apres).unwrap_or(serde_json::Value::Null),
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
}

/// Résout le support porteur des droits et applique `garde_detenteur`.
///
/// Quand le porteur est un **programme**, la détention se lit sur sa chaîne ou
/// sa station : un programme n'a pas de détenteur propre.
async fn garde_ecriture_membre(
    pool: &PgPool,
    type_porteur: &str,
    porteur_id: Uuid,
    moi: Uuid,
) -> Result<(), ApiErreur> {
    let (type_support, support_id) = if est_programme(type_porteur) {
        let (type_support, support_id, _) = contexte_emission(pool, porteur_id).await?;
        (type_support, support_id)
    } else {
        (type_support_du_porteur(type_porteur)?.to_string(), porteur_id)
    };

    garde_detenteur(pool, &type_support, support_id, moi, "co_detenteur").await?;
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// Routes membre
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/medias/{type_porteur}/{porteur_id}/equipe
pub async fn obtenir_equipe(
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (type_porteur, porteur_id) = chemin.into_inner();
    valider_type_porteur(&type_porteur)?;
    exiger_porteur_existant(pool.get_ref(), &type_porteur, porteur_id).await?;

    let membres = equipe_du_porteur(pool.get_ref(), &type_porteur, porteur_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "membres": membres })),
        error: None,
    }))
}

/// PUT /api/medias/{type_porteur}/{porteur_id}/equipe
pub async fn definir_equipe(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<EquipeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_porteur, porteur_id) = chemin.into_inner();
    valider_type_porteur(&type_porteur)?;
    exiger_porteur_existant(pool.get_ref(), &type_porteur, porteur_id).await?;
    garde_ecriture_membre(pool.get_ref(), &type_porteur, porteur_id, moi).await?;

    body.valider()?;
    let avant = equipe_du_porteur(pool.get_ref(), &type_porteur, porteur_id).await?;
    let membres =
        appliquer_equipe(pool.get_ref(), &type_porteur, porteur_id, &body.membres, moi).await?;

    journaliser_equipe(
        &req,
        pool.get_ref(),
        moi,
        &type_porteur,
        porteur_id,
        avant,
        &membres,
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "membres": membres })),
        error: None,
    }))
}

/// GET /api/medias/equipe/fonctions — suggestions du champ « fonction ».
///
/// Portée volontairement **globale** (toutes chaînes, stations et programmes
/// confondus) : une fonction déclarée sur une chaîne doit être proposée sur un
/// programme, sinon le référentiel ne se constituerait jamais.
///
/// Le `DISTINCT ON (cle)` n'est pas décoratif : un simple `SELECT DISTINCT`
/// ferait remonter « Directeur », « directeur » et « directeur  » comme trois
/// entrées (FR-015). La clé ignore casse et espaces ; l'orthographe restituée
/// est **la plus employée**.
pub async fn lister_fonctions(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let fonctions: Vec<String> = sqlx::query_scalar(
        "SELECT fonction FROM (
             SELECT DISTINCT ON (cle) fonction
               FROM (
                 SELECT lower(btrim(regexp_replace(fonction, '\\s+', ' ', 'g'))) AS cle,
                        btrim(regexp_replace(fonction, '\\s+', ' ', 'g'))        AS fonction,
                        COUNT(*)                                                 AS n
                   FROM media_content.membre_equipe
                  WHERE deleted_at IS NULL AND btrim(fonction) <> ''
                  GROUP BY 1, 2
               ) v
              ORDER BY cle, n DESC, fonction ASC
         ) f ORDER BY fonction ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(fonctions),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Routes back-office
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/admin/medias/{type_porteur}/{porteur_id}/equipe
///
/// Sert aussi les porteurs **non publiés** : le back-office prépare une fiche
/// avant sa mise en ligne.
pub async fn admin_obtenir_equipe(
    pool: web::Data<PgPool>,
    admin: AdminUtilisateur,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    obtenir_equipe(pool, chemin).await
}

/// PUT /api/admin/medias/{type_porteur}/{porteur_id}/equipe
///
/// Mêmes règles que le chemin membre — `appliquer_equipe` est partagée, aucune
/// règle n'est réécrite. Seule l'autorité diffère : l'administration n'a pas à
/// détenir le support. **La journalisation, elle, ne diffère pas** : le Principe
/// VII est un MUST et ce `PUT` est une mutation au même titre que l'autre.
pub async fn admin_definir_equipe(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    admin: AdminUtilisateur,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<EquipeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let (type_porteur, porteur_id) = chemin.into_inner();
    valider_type_porteur(&type_porteur)?;
    exiger_porteur_existant(pool.get_ref(), &type_porteur, porteur_id).await?;

    body.valider()?;
    let avant = equipe_du_porteur(pool.get_ref(), &type_porteur, porteur_id).await?;
    let membres = appliquer_equipe(
        pool.get_ref(),
        &type_porteur,
        porteur_id,
        &body.membres,
        admin.id,
    )
    .await?;

    journaliser_equipe(
        &req,
        pool.get_ref(),
        admin.id,
        &type_porteur,
        porteur_id,
        avant,
        &membres,
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "membres": membres })),
        error: None,
    }))
}

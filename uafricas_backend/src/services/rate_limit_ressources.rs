//! Rate-limit des ressources contribuées par utilisateur et par salle
//! (feature 001-ressources-fermeture-session, research.md §4).
//!
//! Règle : ≤ 10 ressources contribuées par utilisateur, par salle, sur une
//! fenêtre glissante de 24 h. Comptage en base (pas de table dédiée — index
//! partiel `idx_afrolang_ressource_contribuee_rate_limit` couvre la requête).

use sqlx::PgPool;
use uuid::Uuid;

/// Compte le nombre de ressources contribuées créées par `auteur_id` dans la
/// salle `salle_id` durant les 24 dernières heures glissantes (soft-delete exclu).
///
/// Le handler appelant compare le résultat à `10` ; si `>=`, renvoyer 429.
pub async fn compter_ressources_recentes(
    db: &PgPool,
    auteur_id: Uuid,
    salle_id: Uuid,
) -> Result<i64, sqlx::Error> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::BIGINT
         FROM afrolang.ressource_contribuee
         WHERE auteur_id = $1
           AND salle_id = $2
           AND deleted_at IS NULL
           AND created_at > NOW() - INTERVAL '24 hours'",
    )
    .bind(auteur_id)
    .bind(salle_id)
    .fetch_one(db)
    .await?;

    Ok(count.0)
}

/// Seuil documenté pour la cohérence des appels.
pub const LIMITE_24H: i64 = 10;

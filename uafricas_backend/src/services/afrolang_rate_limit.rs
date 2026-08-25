//! Rate limit sur la vérification du code secret d'une salle privée Afrolang
//! (R4 : refonte 2026-04 feature 001-afrolang-salles-refonte).
//!
//! Règle : 5 tentatives échouées par minute par couple (utilisateur, salle).
//! Au-delà, verrouillage 5 minutes, message générique « Trop de tentatives,
//! réessayez dans quelques minutes ». Granularité par utilisateur connecté
//! (et non par IP) car l'utilisateur est forcément authentifié (FR-013).

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;

/// Retourne `true` si le couple (salle_privee, utilisateur) est verrouillé,
/// c'est-à-dire a cumulé au moins 5 tentatives échouées dans les 60 dernières
/// secondes ET n'a pas laissé s'écouler 5 minutes depuis la dernière
/// tentative (succès ou échec).
pub async fn est_verrouillee(
    pool: &PgPool,
    salle_privee_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<bool, ApiErreur> {
    // Compter les tentatives échouées de la dernière minute.
    let echecs_recents: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.tentative_code_acces
         WHERE salle_privee_id = $1
           AND utilisateur_id = $2
           AND succes = FALSE
           AND tente_at > NOW() - INTERVAL '1 minute'",
    )
    .bind(salle_privee_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;

    if echecs_recents < 5 {
        return Ok(false);
    }

    // Récupérer la date de la dernière tentative (succès ou échec).
    // Si elle date de moins de 5 minutes → verrou actif.
    let derniere_recente: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.tentative_code_acces
            WHERE salle_privee_id = $1
              AND utilisateur_id = $2
              AND tente_at > NOW() - INTERVAL '5 minutes'
        )",
    )
    .bind(salle_privee_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;

    Ok(derniere_recente)
}

/// Enregistre une tentative de saisie du code (succès ou échec) dans la
/// table `afrolang.tentative_code_acces`. Le `code_acces` en clair n'est
/// jamais stocké : seule la trace (salle, utilisateur, instant, issue, IP,
/// user-agent) est conservée pour l'audit et le rate limit.
pub async fn enregistrer_tentative(
    pool: &PgPool,
    salle_privee_id: Uuid,
    utilisateur_id: Uuid,
    succes: bool,
    ip: Option<&str>,
    user_agent: Option<&str>,
) -> Result<(), ApiErreur> {
    sqlx::query(
        "INSERT INTO afrolang.tentative_code_acces
            (salle_privee_id, utilisateur_id, succes, ip, user_agent)
         VALUES ($1, $2, $3, $4::INET, $5)",
    )
    .bind(salle_privee_id)
    .bind(utilisateur_id)
    .bind(succes)
    .bind(ip)
    .bind(user_agent)
    .execute(pool)
    .await?;
    Ok(())
}

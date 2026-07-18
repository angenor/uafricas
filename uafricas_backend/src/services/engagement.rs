//! Moteur d'engagement (gamification) — attribution de points non-bloquante.
//!
//! Calqué sur `services::audit::log_action` : les fonctions publiques `.await`
//! l'écriture mais **loguent les erreurs sans les propager** (FR-007) — une
//! action métier (validation, like, jugement) ne doit jamais échouer parce que
//! l'attribution de points a échoué.
//!
//! Idempotence structurelle : chaque mouvement porte une `cle_idempotence`
//! `UNIQUE` ; l'insertion en `ON CONFLICT DO NOTHING` rend tout rejeu inoffensif.

use chrono::{Datelike, NaiveDate, Utc};
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

/// Règle de barème active, résolue depuis `engagement.regle_points`.
struct RegleActive {
    points: i32,
    reputation_delta: i32,
    plafond_journalier: Option<i32>,
    plafond_mensuel: Option<i32>,
}

async fn charger_regle(pool: &PgPool, type_action: &str) -> Option<RegleActive> {
    sqlx::query_as::<_, (i32, i32, Option<i32>, Option<i32>)>(
        "SELECT points, reputation_delta, plafond_journalier, plafond_mensuel
         FROM engagement.regle_points
         WHERE type_action = $1 AND actif = TRUE",
    )
    .bind(type_action)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
    .map(|(points, reputation_delta, plafond_journalier, plafond_mensuel)| RegleActive {
        points,
        reputation_delta,
        plafond_journalier,
        plafond_mensuel,
    })
}

/// Code de niveau correspondant à un solde (plus grand `seuil_min <= solde`).
async fn niveau_pour_solde(
    tx: &mut Transaction<'_, Postgres>,
    solde: i32,
) -> Result<String, sqlx::Error> {
    let code: Option<String> = sqlx::query_scalar(
        "SELECT code FROM engagement.niveau
         WHERE seuil_min <= $1 ORDER BY seuil_min DESC LIMIT 1",
    )
    .bind(solde)
    .fetch_optional(&mut **tx)
    .await?;
    Ok(code.unwrap_or_else(|| "membre".to_string()))
}

/// Cœur de l'attribution (transactionnel). Retourne une erreur SQL, jamais
/// propagée aux appelants publics.
///
/// - `montant_override` : force le montant de points (paliers de popularité,
///   ajustement admin) au lieu de celui de la règle.
/// - `reputation_override` : idem pour la réputation.
async fn appliquer(
    pool: &PgPool,
    utilisateur_id: Uuid,
    type_action: &str,
    type_objet: Option<&str>,
    objet_id: Option<Uuid>,
    cle_idempotence: &str,
    montant_override: Option<i32>,
    reputation_override: Option<i32>,
) -> Result<(), sqlx::Error> {
    let regle = charger_regle(pool, type_action).await;

    // Résolution du montant/réputation de base
    let (base_points, base_reputation) = match (&regle, montant_override) {
        (Some(r), None) => (r.points, r.reputation_delta),
        (Some(r), Some(m)) => (m, reputation_override.unwrap_or(r.reputation_delta)),
        (None, Some(m)) => (m, reputation_override.unwrap_or(0)),
        // Règle inconnue ou inactive et aucun montant forcé → aucune attribution.
        (None, None) => return Ok(()),
    };

    let mut tx = pool.begin().await?;

    // Compte créé paresseusement (pas de back-fill au lancement, FR-024).
    sqlx::query(
        "INSERT INTO engagement.compte (utilisateur_id) VALUES ($1)
         ON CONFLICT (utilisateur_id) DO NOTHING",
    )
    .bind(utilisateur_id)
    .execute(&mut *tx)
    .await?;

    let ligne = sqlx::query(
        "SELECT solde_points, solde_points_mensuel, mois_courant, reputation
         FROM engagement.compte WHERE utilisateur_id = $1 FOR UPDATE",
    )
    .bind(utilisateur_id)
    .fetch_one(&mut *tx)
    .await?;

    let solde: i32 = ligne.get("solde_points");
    let mut mensuel: i32 = ligne.get("solde_points_mensuel");
    let mois_courant: NaiveDate = ligne.get("mois_courant");
    let reputation: i32 = ligne.get("reputation");

    // Reset mensuel paresseux (D5)
    let aujourd_hui = Utc::now().date_naive();
    let debut_mois = NaiveDate::from_ymd_opt(aujourd_hui.year(), aujourd_hui.month(), 1)
        .unwrap_or(aujourd_hui);
    if mois_courant != debut_mois {
        mensuel = 0;
    }

    // Écrêtage plafond (uniquement sur les gains positifs, D6)
    let mut points_effectifs = base_points;
    let mut plafond_atteint = false;
    if base_points > 0 {
        if let Some(r) = &regle {
            if let Some(pj) = r.plafond_journalier {
                let deja: i64 = sqlx::query_scalar(
                    "SELECT COALESCE(SUM(points), 0) FROM engagement.mouvement_points
                     WHERE utilisateur_id = $1 AND type_action = $2 AND created_at::date = CURRENT_DATE",
                )
                .bind(utilisateur_id)
                .bind(type_action)
                .fetch_one(&mut *tx)
                .await?;
                let residuel = ((pj as i64) - deja).max(0);
                if (points_effectifs as i64) > residuel {
                    points_effectifs = residuel as i32;
                    plafond_atteint = true;
                }
            }
            if let Some(pm) = r.plafond_mensuel {
                let deja: i64 = sqlx::query_scalar(
                    "SELECT COALESCE(SUM(points), 0) FROM engagement.mouvement_points
                     WHERE utilisateur_id = $1 AND type_action = $2
                       AND date_trunc('month', created_at) = date_trunc('month', NOW())",
                )
                .bind(utilisateur_id)
                .bind(type_action)
                .fetch_one(&mut *tx)
                .await?;
                let residuel = ((pm as i64) - deja).max(0);
                if (points_effectifs as i64) > residuel {
                    points_effectifs = residuel as i32;
                    plafond_atteint = true;
                }
            }
        }
    }

    let nouveau_solde = (solde + points_effectifs).max(0); // plancher 0 (D7)
    let nouveau_mensuel = mensuel + points_effectifs;
    let nouvelle_reputation = reputation + base_reputation;

    // Insertion idempotente du mouvement
    let res = sqlx::query(
        "INSERT INTO engagement.mouvement_points
           (utilisateur_id, type_action, type_objet, objet_id, points,
            reputation_delta, solde_apres, plafond_atteint, cle_idempotence)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         ON CONFLICT (cle_idempotence) DO NOTHING",
    )
    .bind(utilisateur_id)
    .bind(type_action)
    .bind(type_objet)
    .bind(objet_id)
    .bind(points_effectifs)
    .bind(base_reputation)
    .bind(nouveau_solde)
    .bind(plafond_atteint)
    .bind(cle_idempotence)
    .execute(&mut *tx)
    .await?;

    // Déjà attribué → le compte n'est pas modifié (idempotence, FR-008)
    if res.rows_affected() == 0 {
        tx.rollback().await?;
        return Ok(());
    }

    let niveau_code = niveau_pour_solde(&mut tx, nouveau_solde).await?;

    sqlx::query(
        "UPDATE engagement.compte
         SET solde_points = $2, solde_points_mensuel = $3, mois_courant = $4,
             reputation = $5, niveau_code = $6, dernier_mouvement_at = NOW(), updated_at = NOW()
         WHERE utilisateur_id = $1",
    )
    .bind(utilisateur_id)
    .bind(nouveau_solde)
    .bind(nouveau_mensuel)
    .bind(debut_mois)
    .bind(nouvelle_reputation)
    .bind(&niveau_code)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

/// Attribue les points d'une règle à un utilisateur (non-bloquant).
pub async fn attribuer(
    pool: &PgPool,
    utilisateur_id: Uuid,
    type_action: &str,
    type_objet: Option<&str>,
    objet_id: Option<Uuid>,
    cle_idempotence: &str,
) {
    if let Err(e) = appliquer(
        pool, utilisateur_id, type_action, type_objet, objet_id, cle_idempotence, None, None,
    )
    .await
    {
        log::error!("Engagement: échec attribution '{type_action}' pour {utilisateur_id}: {e}");
    }
}

/// Applique un malus (points + réputation, ex. « factcheck faux »). La règle
/// porte déjà des valeurs négatives ; le solde reste plancher 0 (D7).
pub async fn retirer(
    pool: &PgPool,
    utilisateur_id: Uuid,
    type_action: &str,
    type_objet: Option<&str>,
    objet_id: Option<Uuid>,
    cle_idempotence: &str,
) {
    if let Err(e) = appliquer(
        pool, utilisateur_id, type_action, type_objet, objet_id, cle_idempotence, None, None,
    )
    .await
    {
        log::error!("Engagement: échec malus '{type_action}' pour {utilisateur_id}: {e}");
    }
}

/// Évalue les paliers de popularité d'une publication et crédite l'auteur
/// une seule fois par palier franchi (idempotence via la clé) (D3, FR-015/016).
pub async fn evaluer_popularite(
    pool: &PgPool,
    type_objet: &str,
    objet_id: Uuid,
    auteur_id: Uuid,
    likes_count: i64,
) {
    let paliers = sqlx::query_as::<_, (i32, i32)>(
        "SELECT seuil_likes, points FROM engagement.palier_popularite
         WHERE actif = TRUE AND seuil_likes <= $1 ORDER BY seuil_likes",
    )
    .bind(likes_count as i32)
    .fetch_all(pool)
    .await;

    let paliers = match paliers {
        Ok(p) => p,
        Err(e) => {
            log::error!("Engagement: échec lecture paliers popularité: {e}");
            return;
        }
    };

    for (seuil, points) in paliers {
        let cle = format!("popularite:{type_objet}:{objet_id}:{seuil}");
        if let Err(e) = appliquer(
            pool,
            auteur_id,
            "popularite_palier",
            Some(type_objet),
            Some(objet_id),
            &cle,
            Some(points),
            Some(0),
        )
        .await
        {
            log::error!("Engagement: échec palier {seuil} pour {auteur_id}: {e}");
        }
    }
}

/// Ajustement manuel administrateur (crédit/débit motivé). Chaque appel est
/// unique (clé aléatoire) — non plafonné.
pub async fn ajuster(pool: &PgPool, utilisateur_id: Uuid, points: i32, reputation_delta: i32) {
    let cle = format!("ajustement:{}", Uuid::new_v4());
    if let Err(e) = appliquer(
        pool,
        utilisateur_id,
        "ajustement_admin",
        None,
        None,
        &cle,
        Some(points),
        Some(reputation_delta),
    )
    .await
    {
        log::error!("Engagement: échec ajustement admin pour {utilisateur_id}: {e}");
    }
}

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
    /// Catégorie de ventilation, **recopiée** sur le mouvement à l'écriture (R1).
    categorie_id: Option<Uuid>,
}

/// La colonne `regle_points.seuil_declencheur` n'est plus lue par le moteur : son
/// unique consommateur était le bonus « 5 réseaux distincts », supprimé par la
/// feature 008. La colonne subsiste en base et reste réglable en back-office, au
/// cas où une future règle à seuil serait introduite.
async fn charger_regle(pool: &PgPool, type_action: &str) -> Option<RegleActive> {
    sqlx::query_as::<_, (i32, i32, Option<i32>, Option<i32>, Option<Uuid>)>(
        "SELECT points, reputation_delta, plafond_journalier, plafond_mensuel, categorie_id
         FROM engagement.regle_points
         WHERE type_action = $1 AND actif = TRUE",
    )
    .bind(type_action)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
    .map(
        |(points, reputation_delta, plafond_journalier, plafond_mensuel, categorie_id)| {
            RegleActive {
                points,
                reputation_delta,
                plafond_journalier,
                plafond_mensuel,
                categorie_id,
            }
        },
    )
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
        "SELECT solde_points, solde_points_mensuel, mois_courant, reputation, niveau_code
         FROM engagement.compte WHERE utilisateur_id = $1 FOR UPDATE",
    )
    .bind(utilisateur_id)
    .fetch_one(&mut *tx)
    .await?;

    let solde: i32 = ligne.get("solde_points");
    let mut mensuel: i32 = ligne.get("solde_points_mensuel");
    let mois_courant: NaiveDate = ligne.get("mois_courant");
    let reputation: i32 = ligne.get("reputation");
    let niveau_avant: String = ligne.get("niveau_code");

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

    // Insertion idempotente du mouvement. La catégorie est **recopiée** depuis la
    // règle (jamais jointe à la lecture) : re-catégoriser une règle ne déplace
    // aucun point déjà gagné (R1).
    let res = sqlx::query(
        "INSERT INTO engagement.mouvement_points
           (utilisateur_id, type_action, type_objet, objet_id, points,
            reputation_delta, solde_apres, plafond_atteint, cle_idempotence, categorie_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
    .bind(regle.as_ref().and_then(|r| r.categorie_id))
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

    // ── Après le COMMIT seulement ────────────────────────────────────────────
    // Ni la notification de niveau ni l'évaluation des badges ne doivent vivre
    // dans la transaction du mouvement : elles sont accessoires, et une erreur
    // de leur côté ne doit jamais faire échouer l'attribution (FR-028).

    if niveau_code != niveau_avant {
        notifier_niveau(pool, utilisateur_id, &niveau_code).await;
    }

    evaluer_badges(pool, utilisateur_id).await;

    Ok(())
}

/// Notifie une montée (ou une bascule) de niveau. Non-bloquant.
async fn notifier_niveau(pool: &PgPool, utilisateur_id: Uuid, niveau_code: &str) {
    let libelle: Option<String> =
        sqlx::query_scalar("SELECT libelle FROM engagement.niveau WHERE code = $1")
            .bind(niveau_code)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten();

    let Some(libelle) = libelle else { return };

    crate::models::notification::creer_notification(
        pool,
        utilisateur_id,
        crate::models::notification::engagement::NIVEAU_ATTEINT,
        &format!("Félicitations ! Vous avez atteint le statut « {libelle} »."),
        Some(crate::models::notification::engagement::LIEN_ESPACE),
    )
    .await;
}

// ════════════════════════════════════════════════════════════════════════════
// BADGES
// ════════════════════════════════════════════════════════════════════════════

/// Définition d'un badge automatique, telle qu'évaluée par `mesurer_condition`.
struct BadgeAEvaluer {
    id: Uuid,
    libelle: String,
    type_condition: String,
    parametre_action: Option<String>,
    parametre_categorie_id: Option<Uuid>,
    parametre_niveau_code: Option<String>,
    seuil: Option<i32>,
}

/// Résultat de l'évaluation d'une condition de badge.
pub struct MesureBadge {
    /// La condition est-elle satisfaite ?
    pub atteint: bool,
    /// Progression affichable au membre, `(actuel, cible)`.
    ///
    /// `None` quand la condition ne s'exprime pas comme « N sur M » : le niveau
    /// se compare sur un `ordre`, dont afficher « 1 sur 2 » n'apprendrait rien.
    /// L'espace membre n'affiche alors aucune barre (contrat api-membre).
    pub progression: Option<(i64, i64)>,
}

/// Évalue **une** condition de badge pour un membre.
///
/// Source unique de vérité partagée par `evaluer_badges` (attribution) et
/// `GET /mes-badges` (progression affichée) : sans ce partage, les deux
/// finiraient par diverger — un badge affiché « 49/50 » alors qu'il est déjà
/// attribué, ou l'inverse.
async fn mesurer_condition(
    pool: &PgPool,
    utilisateur_id: Uuid,
    badge: &BadgeAEvaluer,
) -> Result<MesureBadge, sqlx::Error> {
    let seuil = badge.seuil.unwrap_or(0) as i64;

    let mesure = match badge.type_condition.as_str() {
        "actions_comptees" => {
            let Some(action) = badge.parametre_action.as_deref() else {
                return Ok(MesureBadge { atteint: false, progression: None });
            };
            let actuel: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM engagement.mouvement_points
                 WHERE utilisateur_id = $1 AND type_action = $2",
            )
            .bind(utilisateur_id)
            .bind(action)
            .fetch_one(pool)
            .await?;
            MesureBadge { atteint: actuel >= seuil, progression: Some((actuel.min(seuil), seuil)) }
        }

        "points_categorie" => {
            let Some(categorie_id) = badge.parametre_categorie_id else {
                return Ok(MesureBadge { atteint: false, progression: None });
            };
            let actuel: i64 = sqlx::query_scalar(
                "SELECT COALESCE(SUM(points), 0)::bigint FROM engagement.mouvement_points
                 WHERE utilisateur_id = $1 AND categorie_id = $2",
            )
            .bind(utilisateur_id)
            .bind(categorie_id)
            .fetch_one(pool)
            .await?;
            MesureBadge {
                atteint: actuel >= seuil,
                progression: Some((actuel.clamp(0, seuil), seuil)),
            }
        }

        "solde_total" => {
            let actuel: i64 = sqlx::query_scalar(
                "SELECT COALESCE(solde_points, 0)::bigint FROM engagement.compte
                 WHERE utilisateur_id = $1",
            )
            .bind(utilisateur_id)
            .fetch_optional(pool)
            .await?
            .unwrap_or(0);
            MesureBadge {
                atteint: actuel >= seuil,
                progression: Some((actuel.clamp(0, seuil), seuil)),
            }
        }

        // Comparaison sur `ordre`, jamais sur le code : le barème des niveaux est
        // paramétrable, et un code ne porte aucune relation d'ordre.
        "niveau_atteint" => {
            let Some(cible) = badge.parametre_niveau_code.as_deref() else {
                return Ok(MesureBadge { atteint: false, progression: None });
            };
            let atteint: bool = sqlx::query_scalar(
                "SELECT EXISTS(
                   SELECT 1 FROM engagement.compte c
                     JOIN engagement.niveau courant ON courant.code = c.niveau_code
                     JOIN engagement.niveau cible   ON cible.code = $2
                    WHERE c.utilisateur_id = $1 AND courant.ordre >= cible.ordre)",
            )
            .bind(utilisateur_id)
            .bind(cible)
            .fetch_one(pool)
            .await?;
            MesureBadge { atteint, progression: None }
        }

        // Le seuil franchi est porté par la clé d'idempotence
        // (`popularite:{type_objet}:{objet_id}:{seuil}`), pas par une colonne.
        // Le garde `~ '^\d+$'` évite qu'une clé malformée fasse échouer le cast.
        "palier_popularite" => {
            let actuel: Option<i64> = sqlx::query_scalar(
                "SELECT MAX(split_part(cle_idempotence, ':', 4)::integer)::bigint
                 FROM engagement.mouvement_points
                 WHERE utilisateur_id = $1 AND type_action = 'popularite_palier'
                   AND split_part(cle_idempotence, ':', 4) ~ '^\\d+$'",
            )
            .bind(utilisateur_id)
            .fetch_one(pool)
            .await?;
            let actuel = actuel.unwrap_or(0);
            MesureBadge { atteint: actuel >= seuil, progression: Some((actuel.min(seuil), seuil)) }
        }

        // Type inconnu (enum élargi sans branche correspondante) : on n'attribue
        // rien plutôt que d'attribuer à tort.
        autre => {
            log::warn!("Engagement: condition de badge non gérée « {autre} »");
            MesureBadge { atteint: false, progression: None }
        }
    };

    Ok(mesure)
}

/// Charge les badges automatiques actifs que ce membre n'a **pas encore** obtenus.
async fn badges_a_evaluer(
    pool: &PgPool,
    utilisateur_id: Uuid,
) -> Result<Vec<BadgeAEvaluer>, sqlx::Error> {
    // `type_condition::text` : l'enum PostgreSQL est lu comme texte, ce qui évite
    // un type Rust dédié pour cinq valeurs consommées par un unique `match`.
    let lignes = sqlx::query(
        "SELECT b.id, b.libelle, b.type_condition::text AS type_condition,
                b.parametre_action, b.parametre_categorie_id, b.parametre_niveau_code, b.seuil
         FROM engagement.badge b
         WHERE b.actif = TRUE AND b.manuel = FALSE AND b.type_condition IS NOT NULL
           AND NOT EXISTS (SELECT 1 FROM engagement.badge_obtenu bo
                            WHERE bo.badge_id = b.id AND bo.utilisateur_id = $1)
         ORDER BY b.ordre",
    )
    .bind(utilisateur_id)
    .fetch_all(pool)
    .await?;

    Ok(lignes
        .into_iter()
        .map(|l| BadgeAEvaluer {
            id: l.get("id"),
            libelle: l.get("libelle"),
            type_condition: l.get("type_condition"),
            parametre_action: l.get("parametre_action"),
            parametre_categorie_id: l.get("parametre_categorie_id"),
            parametre_niveau_code: l.get("parametre_niveau_code"),
            seuil: l.get("seuil"),
        })
        .collect())
}

/// Évalue les badges d'un membre et attribue ceux dont la condition est remplie.
///
/// Appelée à deux endroits, **jamais dans une transaction métier** :
/// 1. après le commit d'un mouvement (depuis `appliquer`, donc pour `attribuer`,
///    `retirer`, les trois `crediter_*` et `ajuster` d'un coup) ;
/// 2. à la lecture de `GET /mes-badges`, ce qui rattrape les conditions devenues
///    vraies autrement qu'à la suite d'un mouvement (badge créé par
///    l'administration, condition assouplie) — sans aucune tâche de fond.
///
/// L'insertion est `ON CONFLICT DO NOTHING` : l'idempotence est structurelle,
/// donc réévaluer est inoffensif. La notification n'est émise que si l'insertion
/// a réellement créé une ligne, ce qui interdit la notification répétée (SC-010).
///
/// Les erreurs sont loguées et **jamais propagées**.
pub async fn evaluer_badges(pool: &PgPool, utilisateur_id: Uuid) {
    let badges = match badges_a_evaluer(pool, utilisateur_id).await {
        Ok(b) => b,
        Err(e) => {
            log::error!("Engagement: échec lecture des badges pour {utilisateur_id}: {e}");
            return;
        }
    };

    for badge in badges {
        match mesurer_condition(pool, utilisateur_id, &badge).await {
            Ok(m) if m.atteint => attribuer_badge(pool, utilisateur_id, &badge).await,
            Ok(_) => {}
            Err(e) => log::error!(
                "Engagement: échec évaluation du badge « {} » pour {utilisateur_id}: {e}",
                badge.libelle
            ),
        }
    }
}

/// Insère un badge obtenu et notifie **uniquement** si la ligne a été créée.
async fn attribuer_badge(pool: &PgPool, utilisateur_id: Uuid, badge: &BadgeAEvaluer) {
    let res = sqlx::query(
        "INSERT INTO engagement.badge_obtenu (utilisateur_id, badge_id, origine)
         VALUES ($1, $2, 'automatique')
         ON CONFLICT (utilisateur_id, badge_id) DO NOTHING",
    )
    .bind(utilisateur_id)
    .bind(badge.id)
    .execute(pool)
    .await;

    match res {
        // `rows_affected() == 1` est la seule garantie qu'il s'agit bien d'une
        // nouveauté : sans ce test, chaque réévaluation renotifierait.
        Ok(r) if r.rows_affected() == 1 => {
            crate::models::notification::creer_notification(
                pool,
                utilisateur_id,
                crate::models::notification::engagement::BADGE_DEBLOQUE,
                &format!("Nouveau badge débloqué : « {} » !", badge.libelle),
                Some(crate::models::notification::engagement::LIEN_ESPACE),
            )
            .await;
        }
        Ok(_) => {}
        Err(e) => log::error!(
            "Engagement: échec attribution du badge « {} » à {utilisateur_id}: {e}",
            badge.libelle
        ),
    }
}

/// Progression d'un membre vers un badge non encore obtenu, pour l'affichage.
/// Réutilise `mesurer_condition` : une seule définition de chaque condition.
pub async fn progression_badge(
    pool: &PgPool,
    utilisateur_id: Uuid,
    badge_id: Uuid,
) -> Option<(i64, i64)> {
    let ligne = sqlx::query(
        "SELECT b.id, b.libelle, b.type_condition::text AS type_condition,
                b.parametre_action, b.parametre_categorie_id, b.parametre_niveau_code, b.seuil
         FROM engagement.badge b WHERE b.id = $1 AND b.type_condition IS NOT NULL",
    )
    .bind(badge_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()?;

    let badge = BadgeAEvaluer {
        id: ligne.get("id"),
        libelle: ligne.get("libelle"),
        type_condition: ligne.get("type_condition"),
        parametre_action: ligne.get("parametre_action"),
        parametre_categorie_id: ligne.get("parametre_categorie_id"),
        parametre_niveau_code: ligne.get("parametre_niveau_code"),
        seuil: ligne.get("seuil"),
    };

    mesurer_condition(pool, utilisateur_id, &badge)
        .await
        .ok()
        .and_then(|m| m.progression)
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

// ════════════════════════════════════════════════════════════════════════════
// RÉSOLUTION DU BÉNÉFICIAIRE (feature 008, research R4)
// ════════════════════════════════════════════════════════════════════════════

/// Propriétaire actif d'un support média, avec repli sur son créateur.
///
/// L'unicité du propriétaire est **structurelle** (`uq_support_un_proprietaire`,
/// migration 09m) : la requête retourne 0 ou 1 ligne, aucune agrégation n'est
/// nécessaire. Le repli sur `cree_par` couvre les supports créés par
/// l'administration avant l'existence de `support_detenteur` — sans lui, ces
/// contenus cesseraient silencieusement de rapporter.
async fn proprietaire_support(
    pool: &PgPool,
    type_support: &str,
    support_id: Uuid,
    table: &str,
) -> Option<Uuid> {
    let proprietaire: Option<Uuid> = sqlx::query_scalar(
        "SELECT utilisateur_id FROM media_content.support_detenteur
          WHERE type_support = $1::media_content.type_support_media
            AND support_id = $2 AND role = 'proprietaire' AND actif = TRUE",
    )
    .bind(type_support)
    .bind(support_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();

    if proprietaire.is_some() {
        return proprietaire;
    }

    // `table` ne vient jamais du client : c'est une constante issue du `match`
    // de `resoudre_beneficiaire`.
    sqlx::query_scalar(&format!("SELECT cree_par FROM {table} WHERE id = $1"))
        .bind(support_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

/// Lit une colonne d'auteur sur une table dont le nom est une **constante**.
async fn auteur_simple(pool: &PgPool, table: &str, objet_id: Uuid) -> Option<Uuid> {
    sqlx::query_scalar(&format!("SELECT cree_par FROM {table} WHERE id = $1"))
        .bind(objet_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

/// Résout le membre à créditer pour un contenu donné (research R4).
///
/// Renvoie `None` — **sans erreur** — dans trois cas parfaitement légitimes :
/// - `site_touristique` et `secteur_developpement` : contenus éditoriaux
///   rattachés à une fiche pays, **sans aucune colonne d'auteur** en base
///   (FR-008c). La réaction et le partage continuent de fonctionner
///   normalement, ils ne créditent simplement personne.
/// - contenu introuvable ;
/// - `type_objet` inconnu.
///
/// `profil` et `biblio_humaine` sont les deux familles où l'`objet_id` **est**
/// déjà l'identifiant du bénéficiaire : le profil se désigne lui-même, et une
/// fiche de bibliothèque humaine est indexée sur son titulaire.
pub async fn resoudre_beneficiaire(
    pool: &PgPool,
    type_objet: &str,
    objet_id: Uuid,
) -> Option<Uuid> {
    match type_objet {
        // ── Supports médias : le PROPRIÉTAIRE, plus le créateur (FR-008a) ────
        "chaine_tv" => {
            proprietaire_support(pool, "chaine_tv", objet_id, "media_content.chaine_tv").await
        }
        "station_radio" => {
            proprietaire_support(pool, "station_radio", objet_id, "media_content.station_radio")
                .await
        }

        // ── Programmes : le propriétaire du SUPPORT PARENT ───────────────────
        // Le rattachement au parent est nullable (`ON DELETE SET NULL`) : un
        // programme orphelin retombe sur son propre créateur.
        "programme_tele" => {
            let chaine_id: Option<Uuid> = sqlx::query_scalar(
                "SELECT chaine_id FROM media_content.programme_tele WHERE id = $1",
            )
            .bind(objet_id)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten()
            .flatten();

            match chaine_id {
                Some(chaine_id) => {
                    proprietaire_support(pool, "chaine_tv", chaine_id, "media_content.chaine_tv")
                        .await
                }
                None => auteur_simple(pool, "media_content.programme_tele", objet_id).await,
            }
        }
        "programme_radio" => {
            let station_id: Option<Uuid> = sqlx::query_scalar(
                "SELECT station_id FROM media_content.programme_radio WHERE id = $1",
            )
            .bind(objet_id)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten()
            .flatten();

            match station_id {
                Some(station_id) => {
                    proprietaire_support(
                        pool,
                        "station_radio",
                        station_id,
                        "media_content.station_radio",
                    )
                    .await
                }
                None => auteur_simple(pool, "media_content.programme_radio", objet_id).await,
            }
        }

        // ── Contenus dotés d'un `cree_par` ───────────────────────────────────
        "codimoi" => auteur_simple(pool, "culture.codimoi", objet_id).await,
        "factcheck" => auteur_simple(pool, "governance.factcheck", objet_id).await,
        "video" => auteur_simple(pool, "media_content.video", objet_id).await,
        "fiche_pays" => auteur_simple(pool, "country_profile.fiche_pays", objet_id).await,
        "personnalite_connue" => {
            auteur_simple(pool, "country_profile.personnalite_connue", objet_id).await
        }
        "recette_culinaire" => {
            auteur_simple(pool, "country_profile.recette_culinaire", objet_id).await
        }

        // ── Familles où l'objet EST le membre ────────────────────────────────
        "profil" | "biblio_humaine" => {
            let existe: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM iam.utilisateur
                                WHERE id = $1 AND deleted_at IS NULL AND etat = 'actif')",
            )
            .bind(objet_id)
            .fetch_one(pool)
            .await
            .unwrap_or(false);
            existe.then_some(objet_id)
        }

        // ── Aucun auteur enregistré, et ce n'est pas une anomalie (FR-008c) ──
        "site_touristique" | "secteur_developpement" => None,

        autre => {
            log::debug!("Engagement: aucune résolution d'auteur pour la famille « {autre} »");
            None
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// LES TROIS SOURCES CANONIQUES (feature 008)
// ════════════════════════════════════════════════════════════════════════════

/// Crédite l'auteur d'un contenu pour un « j'aime » reçu (research R3).
///
/// Clé `jaime:{type_objet}:{objet_id}:{membre_qui_aime_id}` — elle ne porte pas
/// l'état de la réaction, seulement **qui a aimé quoi**. Trois exigences en
/// découlent sans une ligne de code supplémentaire :
/// - retirer puis remettre son j'aime ne crédite qu'une fois (le second INSERT
///   frappe la contrainte `UNIQUE` et ne fait rien) ;
/// - un retrait ne reprend jamais de point (la fonction n'est pas appelée) ;
/// - deux membres distincts créditent séparément.
///
/// N'écrit rien si l'auteur est aussi celui qui aime (FR-009). Non-bloquante.
pub async fn crediter_jaime(
    pool: &PgPool,
    type_objet: &str,
    objet_id: Uuid,
    auteur_id: Uuid,
    membre_qui_aime_id: Uuid,
) {
    if auteur_id == membre_qui_aime_id {
        return;
    }

    let cle = format!("jaime:{type_objet}:{objet_id}:{membre_qui_aime_id}");
    if let Err(e) = appliquer(
        pool,
        auteur_id,
        "jaime_recu",
        Some(type_objet),
        Some(objet_id),
        &cle,
        None,
        None,
    )
    .await
    {
        log::error!("Engagement: échec crédit j'aime sur {type_objet}/{objet_id}: {e}");
    }
}

/// Crédite l'auteur d'un contenu pour un partage reçu (research R5).
///
/// Le **canal n'apparaît pas dans la clé** : `partage:{type_objet}:{objet_id}:{partageur_id}`.
/// Un même partageur ne crédite donc qu'une fois par contenu, qu'il l'ait
/// envoyé sur WhatsApp, Facebook, Telegram ou reposté sur le mur. Aucun
/// `COUNT(DISTINCT …)`, aucune fenêtre de concurrence : l'unicité par contenu et
/// par partageur est structurelle. Les canaux restent tracés dans leur table
/// d'origine et dans `engagement.partage_externe` pour la statistique.
///
/// N'écrit rien si le partageur est l'auteur (FR-014) — le bénéficiaire est
/// désormais l'auteur, plus le partageur, ce qui inverse la sémantique de
/// l'ancien bonus « 5 réseaux ». Non-bloquante.
pub async fn crediter_partage(
    pool: &PgPool,
    type_objet: &str,
    objet_id: Uuid,
    auteur_id: Uuid,
    partageur_id: Uuid,
) {
    if auteur_id == partageur_id {
        return;
    }

    let cle = format!("partage:{type_objet}:{objet_id}:{partageur_id}");
    if let Err(e) = appliquer(
        pool,
        auteur_id,
        "partage_recu",
        Some(type_objet),
        Some(objet_id),
        &cle,
        None,
        None,
    )
    .await
    {
        log::error!("Engagement: échec crédit partage sur {type_objet}/{objet_id}: {e}");
    }
}

/// Crédite le bénéficiaire d'un cadeau virtuel, **après le COMMIT** de la
/// transaction comptable (research R9, R10).
///
/// Le montant vient du **catalogue figé sur la transaction**, jamais de la
/// règle : `cadeau_recu` porte `points = 0` et ne sert que de porte
/// activable/désactivable et de porteuse de catégorie et de plafonds. D'où le
/// `montant_override`, exactement comme l'ajustement administrateur.
///
/// Clé `cadeau:{transaction_id}` : rejouer la confirmation d'un paiement ne
/// crédite jamais deux fois (FR-022). C'est aussi ce motif de clé que cible la
/// purge de fin de phase de test — d'où l'interdiction d'y mettre autre chose.
pub async fn crediter_cadeau(
    pool: &PgPool,
    beneficiaire_id: Uuid,
    transaction_id: Uuid,
    points: i32,
) {
    let cle = format!("cadeau:{transaction_id}");
    if let Err(e) = appliquer(
        pool,
        beneficiaire_id,
        "cadeau_recu",
        Some("cadeau"),
        Some(transaction_id),
        &cle,
        Some(points),
        None,
    )
    .await
    {
        log::error!("Engagement: échec crédit cadeau {transaction_id} pour {beneficiaire_id}: {e}");
    }
}

// ════════════════════════════════════════════════════════════════════════════
// PARTAGE EXTERNE (US5)
// ════════════════════════════════════════════════════════════════════════════

/// Résultat du traçage d'un partage vers un réseau social externe.
pub struct ResultatPartageExterne {
    /// Le partage a-t-il été journalisé ? (`false` = ce réseau était déjà tracé
    /// pour ce couple membre/contenu — la trace ne double jamais.)
    pub enregistre: bool,
    /// L'auteur du contenu vient-il d'être crédité ?
    ///
    /// Vrai au **premier** partage de ce contenu par ce membre, tous canaux
    /// confondus. Purement informatif : le partage n'échoue jamais si le crédit
    /// échoue (FR-034).
    pub auteur_credite: bool,
}

/// Trace un partage vers un réseau externe et crédite **l'auteur du contenu**.
///
/// La sémantique est inversée par rapport au bonus « 5 réseaux distincts »
/// qu'elle remplace : le bénéficiaire n'est plus le partageur mais l'auteur
/// (FR-012). Le seuil, le décompte des réseaux et le drapeau `bonus_attribue`
/// disparaissent avec lui — la clé commune de `crediter_partage` réalise
/// structurellement l'unicité par (contenu, partageur) que le comptage
/// approchait (research R5).
///
/// La table `engagement.partage_externe` survit **en tant que trace
/// statistique** par canal : son unicité (membre, contenu, réseau) garantit
/// qu'un même réseau n'est journalisé qu'une fois, ce qui rend le décompte des
/// canaux exploitable même si le crédit, lui, est unique.
pub async fn enregistrer_partage_externe(
    pool: &PgPool,
    utilisateur_id: Uuid,
    type_objet: &str,
    objet_id: Uuid,
    reseau: &str,
) -> Result<ResultatPartageExterne, sqlx::Error> {
    let insertion = sqlx::query(
        "INSERT INTO engagement.partage_externe (utilisateur_id, type_objet, objet_id, reseau)
         VALUES ($1, $2, $3, $4::engagement.reseau_social)
         ON CONFLICT (utilisateur_id, type_objet, objet_id, reseau) DO NOTHING",
    )
    .bind(utilisateur_id)
    .bind(type_objet)
    .bind(objet_id)
    .bind(reseau)
    .execute(pool)
    .await?;

    // Le crédit est tenté à CHAQUE partage, y compris sur un réseau déjà tracé :
    // c'est la clé d'idempotence qui décide, pas nous. Une lecture préalable
    // rouvrirait une fenêtre de concurrence pour un résultat identique.
    let auteur_id = resoudre_beneficiaire(pool, type_objet, objet_id).await;
    let auteur_credite = match auteur_id {
        Some(auteur_id) if auteur_id != utilisateur_id => {
            let deja: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM engagement.mouvement_points
                                WHERE cle_idempotence = $1)",
            )
            .bind(format!("partage:{type_objet}:{objet_id}:{utilisateur_id}"))
            .fetch_one(pool)
            .await
            .unwrap_or(true);

            crediter_partage(pool, type_objet, objet_id, auteur_id, utilisateur_id).await;
            !deja
        }
        _ => false,
    };

    Ok(ResultatPartageExterne { enregistre: insertion.rows_affected() > 0, auteur_credite })
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

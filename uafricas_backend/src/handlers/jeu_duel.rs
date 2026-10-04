//! Duels entre amis (feature 013), mode différé.
//! - `GET  /api/jeu/duels`
//! - `POST /api/jeu/duels`
//! - `GET  /api/jeu/duels/{id}`
//! - `POST /api/jeu/duels/{id}/accepter`
//! - `POST /api/jeu/duels/{id}/refuser`
//! - `POST /api/jeu/duels/{id}/annuler`
//! - `POST /api/jeu/duels/{id}/jouer`
//! - `GET  /api/jeu/duels/{id}/direct`  (mode direct : état autoritaire, vaut présence)
//! - `POST /api/jeu/duels/{id}/direct/repondre`
//! - `POST /api/jeu/duels/{id}/convertir` (direct expiré → différé)
//!
//! Un duel oppose exactement deux amis sur la même série. Chacun joue quand il
//! veut, dans le délai réglé (48 h par défaut) ; le résultat tombe quand le
//! second a joué, ou à l'échéance (forfait de celui qui n'a pas joué).
//!
//! Le duel n'avance qu'en étant LU : chaque route le résout d'abord
//! (`services::jeu::resoudre_duel`), sous verrou. Les effets (notifications,
//! signaux SSE, réputation) sont produits après le COMMIT.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::jeu::garde_joueur;
use crate::models::amitie::obtenir_membre_light;
use crate::models::jeu::{
    evt_duel, evt_duel_propose, CorrectionManche, DuelResponse, DuelRow, EtatDuelDirect, MesDuels,
    PartieDuel, PartieRow, ProposerDuelRequest, QuotasDuel, ReglesJeu, RepondreDirectRequest,
    DUEL_COLONNES, PARTIE_COLONNES,
};
use crate::models::notification;
use crate::services::jeu::{self as moteur, EffetDuel, SERVABLE_SQL};
use crate::services::messagerie_sse::RegistreSse;
use crate::services::engagement;
use crate::ApiResponse;

/// Duels terminés renvoyés par la liste.
const TERMINES_LISTES: usize = 20;

fn lien_duel(duel_id: Uuid) -> String {
    format!("/activites/duels/{duel_id}")
}

async fn prenom(pool: &PgPool, utilisateur_id: Uuid) -> String {
    sqlx::query_scalar::<_, String>("SELECT prenom FROM iam.utilisateur WHERE id = $1")
        .bind(utilisateur_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or_else(|| "Un ami".to_string())
}

async fn libelle_module(pool: &PgPool, code: &str) -> String {
    sqlx::query_scalar::<_, String>("SELECT libelle FROM jeu.module WHERE code = $1")
        .bind(code)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or_else(|| code.to_string())
}

/// Effets d'après COMMIT d'une résolution : signaux, notifications, réputation.
/// Aucun ne doit faire échouer le duel : les erreurs sont avalées par les
/// services qu'on appelle.
async fn appliquer_effets(pool: &PgPool, sse: &RegistreSse, effets: Vec<EffetDuel>) {
    for effet in effets {
        match effet {
            EffetDuel::Termine { duel } => {
                for joueur in [duel.proposant_id, duel.adversaire_id] {
                    let message = match (duel.issue.as_deref(), duel.vainqueur_id) {
                        (Some("nul"), _) => "Duel terminé : match nul.".to_string(),
                        (Some("forfait"), Some(v)) if v == joueur => {
                            "Duel terminé : vous gagnez par forfait.".to_string()
                        }
                        (Some("forfait"), _) => {
                            "Duel terminé : perdu par forfait, vous n'avez pas joué à temps.".to_string()
                        }
                        (_, Some(v)) if v == joueur => "Duel terminé : victoire !".to_string(),
                        _ => "Duel terminé : défaite, la revanche vous attend.".to_string(),
                    };
                    notification::creer_notification(
                        pool, joueur, notification::jeu::DUEL_TERMINE, &message,
                        Some(&lien_duel(duel.id)),
                    )
                    .await;
                    sse.publier(joueur, &evt_duel("duel_termine", duel.id));
                }
                if let (true, Some(vainqueur)) = (duel.compte, duel.vainqueur_id) {
                    engagement::attribuer(
                        pool,
                        vainqueur,
                        "jeu_duel_gagne",
                        Some("duel"),
                        Some(duel.id),
                        &format!("jeu:duel:{}:{vainqueur}", duel.id),
                    )
                    .await;
                }
            }
            EffetDuel::Expire { duel } => {
                for joueur in [duel.proposant_id, duel.adversaire_id] {
                    notification::creer_notification(
                        pool, joueur, notification::jeu::DUEL_TERMINE,
                        "Un duel a expiré sans être joué.", Some(&lien_duel(duel.id)),
                    )
                    .await;
                    sse.publier(joueur, &evt_duel("duel_termine", duel.id));
                }
            }
            EffetDuel::Annule { duel } => {
                for joueur in [duel.proposant_id, duel.adversaire_id] {
                    sse.publier(joueur, &evt_duel("duel_annule", duel.id));
                }
            }
            EffetDuel::Manche { duel } => {
                for joueur in [duel.proposant_id, duel.adversaire_id] {
                    sse.publier(joueur, &evt_duel("duel_manche", duel.id));
                }
            }
        }
    }
}

/// Résout un duel dans sa propre transaction, puis produit ses effets.
async fn resoudre(
    pool: &PgPool,
    sse: &RegistreSse,
    regles: &ReglesJeu,
    duel_id: Uuid,
) -> Result<DuelRow, ApiErreur> {
    let mut tx = pool.begin().await?;
    let (duel, effets) = moteur::resoudre_duel(&mut tx, duel_id, regles).await?;
    tx.commit().await?;
    appliquer_effets(pool, sse, effets).await;
    Ok(duel)
}

/// Charge un duel dont le membre est l'un des joueurs ; 404 sinon (on ne dit
/// pas qu'il existe).
async fn duel_du_membre(pool: &PgPool, duel_id: Uuid, moi: Uuid) -> Result<(), ApiErreur> {
    let joue: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM jeu.duel
                        WHERE id = $1 AND (proposant_id = $2 OR adversaire_id = $2))",
    )
    .bind(duel_id)
    .bind(moi)
    .fetch_one(pool)
    .await?;
    if !joue {
        return Err(ApiErreur::NonTrouve("Duel introuvable".into()));
    }
    Ok(())
}

/// Le duel vu de `moi`. Le résultat de l'autre joueur n'y figure qu'une fois
/// le duel terminé.
async fn reponse_duel(pool: &PgPool, duel: &DuelRow, moi: Uuid) -> Result<DuelResponse, ApiErreur> {
    let parties = moteur::parties_du_duel(&mut *pool.acquire().await?, duel.id).await?;
    let ma_partie = parties.iter().find(|p| p.utilisateur_id == moi).cloned();
    let sa_partie: Option<PartieDuel> = if duel.etat == "termine" {
        parties.iter().find(|p| p.utilisateur_id != moi).cloned()
    } else {
        None
    };

    let mon_gain: i32 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(montant), 0)::int FROM jeu.gain
          WHERE cle_idempotence = $1 AND annule_at IS NULL",
    )
    .bind(format!("duel:{}:{moi}", duel.id))
    .fetch_one(pool)
    .await?;

    Ok(DuelResponse {
        id: duel.id,
        mode: duel.mode.clone(),
        module: duel.module_code.clone(),
        module_libelle: libelle_module(pool, &duel.module_code).await,
        etat: duel.etat.clone(),
        compte: duel.compte,
        je_propose: duel.proposant_id == moi,
        adversaire: obtenir_membre_light(pool, duel.autre(moi)).await?,
        nombre_epreuves: duel.epreuve_ids.as_ref().map_or(0, Vec::len),
        propose_at: duel.propose_at,
        accepte_at: duel.accepte_at,
        echeance_at: duel.echeance_at,
        issue: duel.issue.clone(),
        vainqueur_id: duel.vainqueur_id,
        termine_at: duel.termine_at,
        ma_partie,
        sa_partie,
        mon_gain,
    })
}

/// Duels COMPTÉS acceptés depuis minuit UTC : pour le membre seul, et pour la
/// paire. Au-delà des plafonds, les duels restent jouables mais amicaux.
async fn duels_comptes_du_jour(
    pool: &PgPool,
    moi: Uuid,
    autre: Option<Uuid>,
) -> Result<(i64, i64), sqlx::Error> {
    sqlx::query_as(
        "SELECT COUNT(*) FILTER (WHERE $1 IN (proposant_id, adversaire_id)),
                COUNT(*) FILTER (WHERE $2::uuid IS NOT NULL
                                   AND LEAST(proposant_id, adversaire_id) = LEAST($1, $2)
                                   AND GREATEST(proposant_id, adversaire_id) = GREATEST($1, $2))
           FROM jeu.duel
          WHERE compte AND accepte_at IS NOT NULL
            AND accepte_at >= (date_trunc('day', NOW() AT TIME ZONE 'UTC') AT TIME ZONE 'UTC')",
    )
    .bind(moi)
    .bind(autre)
    .fetch_one(pool)
    .await
}

/// Un duel accepté maintenant entre ces deux membres serait-il compté ?
async fn sera_compte(pool: &PgPool, regles: &ReglesJeu, a: Uuid, b: Uuid) -> Result<bool, sqlx::Error> {
    let (a_total, paire) = duels_comptes_du_jour(pool, a, Some(b)).await?;
    let (b_total, _) = duels_comptes_du_jour(pool, b, None).await?;
    let plafond_membre = i64::from(regles.duels_comptes_par_membre_jour);
    Ok(paire < i64::from(regles.duels_comptes_par_paire_jour)
        && a_total < plafond_membre
        && b_total < plafond_membre)
}

/// Ami, non bloqué, compte actif : la même règle que pour un appel direct.
async fn verifier_relation(pool: &PgPool, moi: Uuid, autre: Uuid) -> Result<(), ApiErreur> {
    let valide: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM social.amitie
                        WHERE utilisateur_a_id = LEAST($1, $2) AND utilisateur_b_id = GREATEST($1, $2))
            AND NOT EXISTS(SELECT 1 FROM social.blocage
                            WHERE (bloqueur_id = $1 AND bloque_id = $2)
                               OR (bloqueur_id = $2 AND bloque_id = $1))
            AND EXISTS(SELECT 1 FROM iam.utilisateur
                        WHERE id = $2 AND etat = 'actif' AND deleted_at IS NULL)",
    )
    .bind(moi)
    .bind(autre)
    .fetch_one(pool)
    .await?;
    if !valide {
        return Err(ApiErreur::AccesInterdit(
            "On ne peut défier qu'un ami ou une amie".into(),
        ));
    }
    Ok(())
}

/// GET /api/jeu/duels : mes duels, chacun résolu avant d'être listé.
pub async fn lister_duels(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = garde_joueur(pool.get_ref(), &req).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let ids: Vec<(Uuid, String)> = sqlx::query_as(
        "SELECT id, etat FROM jeu.duel
          WHERE proposant_id = $1 OR adversaire_id = $1
          ORDER BY propose_at DESC
          LIMIT 100",
    )
    .bind(moi)
    .fetch_all(pool.get_ref())
    .await?;

    let mut liste = MesDuels {
        a_repondre: Vec::new(),
        a_jouer: Vec::new(),
        en_attente: Vec::new(),
        termines: Vec::new(),
        quotas: QuotasDuel {
            comptes_aujourdhui: duels_comptes_du_jour(pool.get_ref(), moi, None).await?.0,
            plafond: regles.duels_comptes_par_membre_jour,
        },
    };

    for (id, etat) in ids {
        // Résoudre AVANT de lister : un forfait non consulté ne verserait
        // sinon jamais sa prime.
        let duel = if matches!(etat.as_str(), "propose" | "accepte" | "en_cours") {
            resoudre(pool.get_ref(), &sse, &regles, id).await?
        } else {
            sqlx::query_as::<_, DuelRow>(&format!(
                "SELECT {DUEL_COLONNES} FROM jeu.duel WHERE id = $1"
            ))
            .bind(id)
            .fetch_one(pool.get_ref())
            .await?
        };

        if duel.est_termine() {
            if liste.termines.len() < TERMINES_LISTES {
                liste.termines.push(reponse_duel(pool.get_ref(), &duel, moi).await?);
            }
            continue;
        }
        let reponse = reponse_duel(pool.get_ref(), &duel, moi).await?;
        let j_ai_fini = reponse.ma_partie.as_ref().is_some_and(|p| p.etat != "en_cours");
        match duel.etat.as_str() {
            "propose" if duel.adversaire_id == moi => liste.a_repondre.push(reponse),
            "propose" => liste.en_attente.push(reponse),
            _ if j_ai_fini => liste.en_attente.push(reponse),
            _ => liste.a_jouer.push(reponse),
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(liste), error: None }))
}

/// Crée un duel, différé ou direct, après toutes les vérifications. Partagée par
/// la proposition et par la conversion d'un duel direct expiré.
async fn creer_duel(
    pool: &PgPool,
    sse: &RegistreSse,
    regles: &ReglesJeu,
    moi: Uuid,
    adversaire: Uuid,
    module: &str,
    mode: &str,
) -> Result<(DuelRow, bool), ApiErreur> {
    if adversaire == moi {
        return Err(ApiErreur::Validation("On ne se défie pas soi-même".into()));
    }
    // Délai de réponse : 48 h en différé, quelques minutes en direct (les deux
    // doivent être là au même moment).
    let delai_minutes = match mode {
        "differe" => i32::from(regles.delai_duel_h) * 60,
        "direct" => i32::from(regles.delai_direct_min),
        _ => return Err(ApiErreur::Validation("Mode de duel inconnu".into())),
    };
    verifier_relation(pool, moi, adversaire).await?;

    let ouvert: Option<bool> = sqlx::query_scalar("SELECT ouvert FROM jeu.module WHERE code = $1")
        .bind(module)
        .fetch_optional(pool)
        .await?;
    if ouvert != Some(true) {
        return Err(ApiErreur::Conflit("Ce module n'est pas ouvert au jeu".into()));
    }
    let servables = moteur::compter_servables(pool, Some(module), None, None).await?;
    if servables < i64::from(regles.taille_duel) {
        return Err(ApiErreur::Conflit(
            "Ce module n'a pas encore assez d'épreuves pour un duel".into(),
        ));
    }

    // Un seul duel ouvert à la fois entre deux membres sur un module. Les duels
    // en cours sont d'abord résolus : un duel échu ne doit pas bloquer.
    let ouverts: Vec<Uuid> = sqlx::query_scalar(
        "SELECT id FROM jeu.duel
          WHERE module_code = $3 AND etat IN ('propose', 'accepte', 'en_cours')
            AND LEAST(proposant_id, adversaire_id) = LEAST($1, $2)
            AND GREATEST(proposant_id, adversaire_id) = GREATEST($1, $2)",
    )
    .bind(moi)
    .bind(adversaire)
    .bind(module)
    .fetch_all(pool)
    .await?;
    for id in ouverts {
        if !resoudre(pool, sse, regles, id).await?.est_termine() {
            return Err(ApiErreur::Conflit(
                "Un duel est déjà en cours entre vous sur ce module".into(),
            ));
        }
    }

    let compte = sera_compte(pool, regles, moi, adversaire).await?;
    let duel = sqlx::query_as::<_, DuelRow>(&format!(
        "INSERT INTO jeu.duel (proposant_id, adversaire_id, module_code, mode, compte, echeance_at)
         VALUES ($1, $2, $3, $4, $5, NOW() + make_interval(mins => $6))
         RETURNING {DUEL_COLONNES}"
    ))
    .bind(moi)
    .bind(adversaire)
    .bind(module)
    .bind(mode)
    .bind(compte)
    .bind(delai_minutes)
    .fetch_one(pool)
    .await?;

    let module_libelle = libelle_module(pool, module).await;
    let qui = prenom(pool, moi).await;
    notification::creer_notification(
        pool,
        adversaire,
        notification::jeu::DUEL_PROPOSE,
        &if mode == "direct" {
            format!("{qui} vous défie en direct sur {module_libelle}")
        } else {
            format!("{qui} vous défie sur {module_libelle}")
        },
        Some(&lien_duel(duel.id)),
    )
    .await;
    if let Some(proposant) = obtenir_membre_light(pool, moi).await? {
        sse.publier(
            adversaire,
            &evt_duel_propose(duel.id, &duel.mode, module, &proposant, duel.echeance_at),
        );
    }
    Ok((duel, compte))
}

/// POST /api/jeu/duels : défie un ami sur un module, en différé ou en direct.
pub async fn proposer_duel(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    body: web::Json<ProposerDuelRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = garde_joueur(pool.get_ref(), &req).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;
    let (duel, compte) = creer_duel(
        pool.get_ref(), &sse, &regles, moi, body.adversaire_id, body.module.trim(), &body.mode,
    )
    .await?;

    let mut reponse = serde_json::to_value(reponse_duel(pool.get_ref(), &duel, moi).await?)
        .unwrap_or_default();
    // Annoncé AVANT de jouer : un duel amical ne rapporte rien (FR-048).
    reponse["sera_compte"] = serde_json::json!(compte);

    Ok(HttpResponse::Created().json(ApiResponse { success: true, data: Some(reponse), error: None }))
}

/// POST /api/jeu/duels/{id}/convertir : le proposant d'un duel DIRECT resté sans
/// réponse le rouvre en différé, avec le même ami et le même module (FR-046).
pub async fn convertir_duel(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = garde_joueur(pool.get_ref(), &req).await?;
    let id = path.into_inner();
    duel_du_membre(pool.get_ref(), id, moi).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let ancien = resoudre(pool.get_ref(), &sse, &regles, id).await?;
    if ancien.proposant_id != moi {
        return Err(ApiErreur::AccesInterdit("Seul celui qui a proposé le duel peut le rouvrir".into()));
    }
    if ancien.mode != "direct" || ancien.etat != "expire" {
        return Err(ApiErreur::Conflit(
            "Seul un duel direct resté sans réponse se transforme en duel différé".into(),
        ));
    }

    let (duel, compte) = creer_duel(
        pool.get_ref(), &sse, &regles, moi, ancien.adversaire_id, &ancien.module_code, "differe",
    )
    .await?;
    let mut reponse = serde_json::to_value(reponse_duel(pool.get_ref(), &duel, moi).await?)
        .unwrap_or_default();
    reponse["sera_compte"] = serde_json::json!(compte);

    Ok(HttpResponse::Created().json(ApiResponse { success: true, data: Some(reponse), error: None }))
}

/// GET /api/jeu/duels/{id}
pub async fn obtenir_duel(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = garde_joueur(pool.get_ref(), &req).await?;
    let id = path.into_inner();
    duel_du_membre(pool.get_ref(), id, moi).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;
    let duel = resoudre(pool.get_ref(), &sse, &regles, id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse_duel(pool.get_ref(), &duel, moi).await?),
        error: None,
    }))
}

/// Série d'un duel : tirée en priorité parmi les épreuves qu'AUCUN des deux n'a
/// jouées, pour que personne ne parte avec un avantage. Complétée par d'autres
/// si le vivier ne suffit pas.
async fn composer_serie_duel(
    pool: &PgPool,
    module: &str,
    a: Uuid,
    b: Uuid,
    taille: i16,
) -> Result<Vec<Uuid>, sqlx::Error> {
    sqlx::query_scalar(&format!(
        "SELECT e.id {SERVABLE_SQL}
            AND e.module_code = $1
          ORDER BY (EXISTS (SELECT 1 FROM jeu.reponse r
                             WHERE r.epreuve_id = e.id AND r.utilisateur_id IN ($2, $3))),
                   random()
          LIMIT $4"
    ))
    .bind(module)
    .bind(a)
    .bind(b)
    .bind(i64::from(taille))
    .fetch_all(pool)
    .await
}

/// POST /api/jeu/duels/{id}/accepter : l'adversaire accepte. La série est
/// figée MAINTENANT, et le caractère compté ou amical aussi.
pub async fn accepter_duel(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = garde_joueur(pool.get_ref(), &req).await?;
    let id = path.into_inner();
    duel_du_membre(pool.get_ref(), id, moi).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let duel = resoudre(pool.get_ref(), &sse, &regles, id).await?;
    if duel.adversaire_id != moi {
        return Err(ApiErreur::AccesInterdit("Seul le joueur défié peut accepter".into()));
    }
    if duel.etat != "propose" {
        return Err(ApiErreur::Conflit("Ce duel ne peut plus être accepté".into()));
    }

    let serie =
        composer_serie_duel(pool.get_ref(), &duel.module_code, duel.proposant_id, moi, regles.taille_duel)
            .await?;
    if serie.len() < regles.taille_duel as usize {
        return Err(ApiErreur::Conflit(
            "Ce module n'a plus assez d'épreuves pour un duel".into(),
        ));
    }
    let compte = sera_compte(pool.get_ref(), &regles, duel.proposant_id, moi).await?;

    let accepte = sqlx::query_as::<_, DuelRow>(&format!(
        "UPDATE jeu.duel
            SET etat = 'accepte', accepte_at = NOW(), epreuve_ids = $2, compte = $3,
                echeance_at = NOW() + make_interval(mins => $4),
                -- En direct, accepter vaut être là : le joueur arrive sur la salle.
                presence_adversaire_at = CASE WHEN mode = 'direct' THEN NOW() END
          WHERE id = $1 AND etat = 'propose'
          RETURNING {DUEL_COLONNES}"
    ))
    .bind(id)
    .bind(&serie)
    .bind(compte)
    // Différé : le délai pour jouer. Direct : le délai pour que les deux soient
    // présents ; au-delà, le duel tombe.
    .bind(if duel.mode == "direct" {
        i32::from(regles.delai_direct_min)
    } else {
        i32::from(regles.delai_duel_h) * 60
    })
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::Conflit("Ce duel ne peut plus être accepté".into()))?;

    notification::creer_notification(
        pool.get_ref(),
        accepte.proposant_id,
        notification::jeu::DUEL_ACCEPTE,
        &format!("{} a accepté votre duel", prenom(pool.get_ref(), moi).await),
        Some(&lien_duel(id)),
    )
    .await;
    sse.publier(accepte.proposant_id, &evt_duel("duel_accepte", id));

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse_duel(pool.get_ref(), &accepte, moi).await?),
        error: None,
    }))
}

/// Refus par le défié, ou annulation par celui qui a proposé : un duel encore
/// `propose` seulement.
async fn clore_proposition(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    id: Uuid,
    refus: bool,
) -> Result<HttpResponse, ApiErreur> {
    let moi = garde_joueur(pool.get_ref(), &req).await?;
    duel_du_membre(pool.get_ref(), id, moi).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let duel = resoudre(pool.get_ref(), &sse, &regles, id).await?;
    let autorise = if refus { duel.adversaire_id == moi } else { duel.proposant_id == moi };
    if !autorise {
        return Err(ApiErreur::AccesInterdit(if refus {
            "Seul le joueur défié peut refuser".into()
        } else {
            "Seul celui qui a proposé le duel peut l'annuler".into()
        }));
    }

    let nouvel_etat = if refus { "refuse" } else { "annule" };
    let clos = sqlx::query_as::<_, DuelRow>(&format!(
        "UPDATE jeu.duel SET etat = $2, termine_at = NOW()
          WHERE id = $1 AND etat = 'propose'
          RETURNING {DUEL_COLONNES}"
    ))
    .bind(id)
    .bind(nouvel_etat)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::Conflit("Ce duel n'est plus en attente de réponse".into()))?;

    let autre = clos.autre(moi);
    if refus {
        notification::creer_notification(
            pool.get_ref(),
            autre,
            notification::jeu::DUEL_REFUSE,
            &format!("{} a décliné votre duel", prenom(pool.get_ref(), moi).await),
            Some(&lien_duel(id)),
        )
        .await;
        sse.publier(autre, &evt_duel("duel_refuse", id));
    } else {
        sse.publier(autre, &evt_duel("duel_annule", id));
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse_duel(pool.get_ref(), &clos, moi).await?),
        error: None,
    }))
}

/// POST /api/jeu/duels/{id}/refuser
pub async fn refuser_duel(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    clore_proposition(req, pool, sse, path.into_inner(), true).await
}

/// POST /api/jeu/duels/{id}/annuler
pub async fn annuler_duel(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    clore_proposition(req, pool, sse, path.into_inner(), false).await
}

/// POST /api/jeu/duels/{id}/jouer : crée (ou renvoie) la partie du membre. Elle
/// se joue ensuite par les routes des parties, sur le même écran.
pub async fn jouer_duel(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = garde_joueur(pool.get_ref(), &req).await?;
    let id = path.into_inner();
    duel_du_membre(pool.get_ref(), id, moi).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    let duel = resoudre(pool.get_ref(), &sse, &regles, id).await?;
    if duel.mode == "direct" {
        return Err(ApiErreur::Conflit(
            "Un duel direct se joue dans sa salle, au même moment pour les deux".into(),
        ));
    }
    if !matches!(duel.etat.as_str(), "accepte" | "en_cours") {
        return Err(ApiErreur::Conflit("Ce duel ne se joue plus".into()));
    }
    let serie = duel
        .epreuve_ids
        .clone()
        .ok_or_else(|| ApiErreur::Conflit("Ce duel n'a pas encore de série".into()))?;

    sqlx::query(
        "INSERT INTO jeu.joueur (utilisateur_id) VALUES ($1) ON CONFLICT (utilisateur_id) DO NOTHING",
    )
    .bind(moi)
    .execute(pool.get_ref())
    .await?;

    // Deux clics simultanés : le second ne crée rien et relit la partie du premier.
    sqlx::query(
        "INSERT INTO jeu.partie (utilisateur_id, module_code, cadre, duel_id, epreuve_ids)
         VALUES ($1, $2, 'duel', $3, $4)
         ON CONFLICT (duel_id, utilisateur_id) WHERE duel_id IS NOT NULL DO NOTHING",
    )
    .bind(moi)
    .bind(&duel.module_code)
    .bind(id)
    .bind(&serie)
    .execute(pool.get_ref())
    .await?;
    sqlx::query("UPDATE jeu.duel SET etat = 'en_cours' WHERE id = $1 AND etat = 'accepte'")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    let partie = sqlx::query_as::<_, PartieRow>(&format!(
        "SELECT {PARTIE_COLONNES} FROM jeu.partie WHERE duel_id = $1 AND utilisateur_id = $2"
    ))
    .bind(id)
    .bind(moi)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "partie_id": partie.id, "etat": partie.etat })),
        error: None,
    }))
}

/// À appeler APRÈS le COMMIT d'une partie de duel qui vient de se terminer :
/// résout le duel (il se termine si l'autre avait déjà joué) ; sinon, prévient
/// l'autre joueur que c'est à lui.
pub async fn apres_partie_de_duel(pool: &PgPool, sse: &RegistreSse, partie: &PartieRow) {
    let Some(duel_id) = partie.duel_id else {
        return;
    };
    let Ok(regles) = moteur::charger_regles(pool).await else {
        return;
    };
    let Ok(duel) = resoudre(pool, sse, &regles, duel_id).await else {
        return;
    };
    if duel.est_termine() {
        return;
    }

    let autre = duel.autre(partie.utilisateur_id);
    let autre_a_fini: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM jeu.partie
                        WHERE duel_id = $1 AND utilisateur_id = $2 AND etat <> 'en_cours')",
    )
    .bind(duel_id)
    .bind(autre)
    .fetch_one(pool)
    .await
    .unwrap_or(true);
    if !autre_a_fini {
        notification::creer_notification(
            pool,
            autre,
            notification::jeu::DUEL_A_VOUS,
            &format!("{} a joué votre duel : à vous !", prenom(pool, partie.utilisateur_id).await),
            Some(&lien_duel(duel_id)),
        )
        .await;
        sse.publier(autre, &evt_duel("duel_a_vous", duel_id));
    }
}

// ─── Duel direct ─────────────────────────────────────────────────────────────

/// Note la présence du membre dans la salle d'un duel direct. Chaque lecture
/// de l'état en vaut une : le sondage de trois secondes sert aussi de battement.
async fn marquer_presence(pool: &PgPool, duel_id: Uuid, moi: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE jeu.duel
            SET presence_proposant_at = CASE WHEN proposant_id = $2 THEN NOW() ELSE presence_proposant_at END,
                presence_adversaire_at = CASE WHEN adversaire_id = $2 THEN NOW() ELSE presence_adversaire_at END
          WHERE id = $1 AND mode = 'direct' AND etat IN ('accepte', 'en_cours')",
    )
    .bind(duel_id)
    .bind(moi)
    .execute(pool)
    .await?;
    Ok(())
}

/// Graine de l'ordre des propositions d'une manche, pour un joueur : stable
/// d'une relecture à l'autre.
fn graine(duel_id: Uuid, rang: i16, joueur: Uuid) -> u64 {
    (duel_id.as_u128() as u64) ^ (joueur.as_u128() as u64).rotate_left(17) ^ (rang as u64)
}

/// GET /api/jeu/duels/{id}/direct : l'état autoritaire de la salle.
pub async fn etat_direct(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = garde_joueur(pool.get_ref(), &req).await?;
    let id = path.into_inner();
    duel_du_membre(pool.get_ref(), id, moi).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    marquer_presence(pool.get_ref(), id, moi).await?;
    let duel = resoudre(pool.get_ref(), &sse, &regles, id).await?;
    if duel.mode != "direct" {
        return Err(ApiErreur::Conflit("Ce duel n'est pas un duel direct".into()));
    }

    let autre = duel.autre(moi);
    let maintenant = chrono::Utc::now();
    let grace = chrono::Duration::seconds(i64::from(regles.grace_direct_s));
    let presence_autre =
        if duel.proposant_id == autre { duel.presence_proposant_at } else { duel.presence_adversaire_at };

    let mut conn = pool.acquire().await?;
    let parties = moteur::parties_du_duel(&mut conn, id).await?;
    let bonnes = |joueur: Uuid| parties.iter().find(|p| p.utilisateur_id == joueur).map_or(0, |p| p.bonnes);

    let mon_gain: i32 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(montant), 0)::int FROM jeu.gain
          WHERE cle_idempotence = $1 AND annule_at IS NULL",
    )
    .bind(format!("duel:{id}:{moi}"))
    .fetch_one(&mut *conn)
    .await?;

    let mut etat = EtatDuelDirect {
        etat: duel.etat.clone(),
        phase: "attente",
        maintenant,
        rang: duel.rang_courant,
        sur: duel.epreuve_ids.as_ref().map_or(0, Vec::len),
        manche_debut_at: duel.manche_debut_at,
        manche_fin_at: None,
        prochaine_at: None,
        epreuve: None,
        ma_cle: None,
        adversaire_a_repondu: false,
        correction: None,
        mes_bonnes: bonnes(moi),
        ses_bonnes: bonnes(autre),
        adversaire_present: presence_autre.is_some_and(|t| maintenant - t <= grace),
        issue: duel.issue.clone(),
        vainqueur_id: duel.vainqueur_id,
        mon_gain,
    };

    if duel.est_termine() {
        etat.phase = "termine";
    } else if duel.etat == "en_cours" {
        if let Some(manche) = moteur::etat_manche(&mut conn, &duel, &regles).await? {
            etat.manche_fin_at = Some(manche.fin_question);
            if maintenant >= manche.debut {
                let ma = manche.reponses.iter().find(|r| r.0 == moi);
                let sa = manche.reponses.iter().find(|r| r.0 == autre);
                etat.ma_cle = ma.and_then(|r| r.1);
                etat.adversaire_a_repondu = sa.is_some_and(|r| r.2 != "sans_reponse");
                etat.epreuve =
                    Some(moteur::servir_stable(&manche.epreuve, graine(id, manche.rang, moi)));

                if let Some(cloture) = manche.cloture {
                    // Révélation : la correction et les deux réponses, aux deux à la fois.
                    etat.phase = "revelation";
                    etat.prochaine_at = Some(
                        cloture + chrono::Duration::seconds(i64::from(regles.pause_revelation_s)),
                    );
                    etat.correction = Some(CorrectionManche {
                        bonne_cle: manche.epreuve.bonne_reponse,
                        explication: manche.epreuve.explication.clone(),
                        lien: moteur::lien_source(
                            &mut conn,
                            manche.epreuve.type_source.as_deref(),
                            manche.epreuve.source_id,
                        )
                        .await?,
                        ma_cle: etat.ma_cle,
                        sa_cle: sa.and_then(|r| r.1),
                    });
                } else {
                    etat.phase = "question";
                }
            }
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(etat), error: None }))
}

/// POST /api/jeu/duels/{id}/direct/repondre
///
/// Mêmes règles de délai et d'idempotence qu'une partie. NE renvoie PAS la
/// correction : elle n'arrive qu'en phase de révélation, aux deux à la fois.
pub async fn repondre_direct(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    path: web::Path<Uuid>,
    body: web::Json<RepondreDirectRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = garde_joueur(pool.get_ref(), &req).await?;
    let id = path.into_inner();
    duel_du_membre(pool.get_ref(), id, moi).await?;
    let regles = moteur::charger_regles(pool.get_ref()).await?;

    marquer_presence(pool.get_ref(), id, moi).await?;
    let duel = resoudre(pool.get_ref(), &sse, &regles, id).await?;
    if duel.mode != "direct" || duel.etat != "en_cours" {
        return Err(ApiErreur::Conflit("Ce duel n'est pas en cours".into()));
    }
    if body.rang != duel.rang_courant
        || duel.manche_debut_at.is_none_or(|debut| chrono::Utc::now() < debut)
    {
        return Err(ApiErreur::Conflit("Cette manche n'est pas ouverte".into()));
    }

    let mut tx = pool.begin().await?;
    let mut partie = sqlx::query_as::<_, PartieRow>(&format!(
        "SELECT {PARTIE_COLONNES} FROM jeu.partie
          WHERE duel_id = $1 AND utilisateur_id = $2 FOR UPDATE"
    ))
    .bind(id)
    .bind(moi)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| ApiErreur::Conflit("Votre partie de duel n'existe pas".into()))?;
    moteur::repondre(&mut tx, &mut partie, &regles, body.rang, Some(body.cle), None).await?;
    tx.commit().await?;

    // Les deux écrans relisent : celui de l'autre apprend qu'on a répondu, et la
    // manche se clôt si c'était la seconde réponse.
    for joueur in [duel.proposant_id, duel.adversaire_id] {
        sse.publier(joueur, &evt_duel("duel_manche", id));
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "enregistre": true })),
        error: None,
    }))
}

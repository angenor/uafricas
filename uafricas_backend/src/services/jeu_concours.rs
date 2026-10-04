//! Cycle d'un concours (feature 014, famille B) : appel à participation →
//! modération → vote → résultats, résolu À LA LECTURE (research D6).
//!
//! Trois règles tiennent ce fichier :
//!
//! 1. **[`resoudre_concours`] est appelée par toute route qui lit un concours**
//!    (point de conception PC1). L'oublier ne casse rien de visible : le
//!    concours resterait en vote après sa date, et ses récompenses ne
//!    partiraient jamais.
//! 2. **Deux transitions seulement s'écrivent** — l'annulation au seuil du
//!    vote et l'établissement des résultats —, sous `SELECT … FOR UPDATE` de la
//!    ligne du concours, et seulement si l'état est encore `actif`. Leurs effets
//!    (réputation, notifications) partent APRÈS le COMMIT.
//! 3. **Les primes de participation n'ont qu'un chemin** ([`verser_primes_participation`]),
//!    sous la clé `concours:{id}:participation:{pid}`, que le concours soit
//!    annulé ou mené à terme (PC5) : il ne peut pas les verser deux fois.

use chrono::Utc;
use sqlx::{PgConnection, PgExecutor, PgPool};
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::jeu_concours::{ConcoursRow, Phase, CONCOURS_COLONNES};
use crate::models::notification;
use crate::services::engagement;
use crate::services::jeu::{self as moteur};

/// Types de notification des concours.
pub mod notif {
    pub const PARTICIPATION_ACCEPTEE: &str = "jeu.participation_acceptee";
    pub const PARTICIPATION_REJETEE: &str = "jeu.participation_rejetee";
    pub const PARTICIPATION_SUSPENDUE: &str = "jeu.participation_suspendue";
    pub const CONCOURS_RESULTATS: &str = "jeu.concours_resultats";
    pub const CONCOURS_LAUREAT: &str = "jeu.concours_laureat";
    pub const CONCOURS_ANNULE: &str = "jeu.concours_annule";
}

pub fn lien_concours(id: Uuid) -> String {
    format!("/activites/concours/{id}")
}

pub async fn charger_concours<'e, E: PgExecutor<'e>>(ex: E, id: Uuid) -> Result<ConcoursRow, ApiErreur> {
    sqlx::query_as::<_, ConcoursRow>(&format!("SELECT {CONCOURS_COLONNES} FROM jeu.concours c WHERE c.id = $1"))
        .bind(id)
        .fetch_optional(ex)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Concours introuvable".into()))
}

pub async fn compter_publiees<'e, E: PgExecutor<'e>>(ex: E, concours_id: Uuid) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM jeu.participation WHERE concours_id = $1 AND etat = 'publiee'")
        .bind(concours_id)
        .fetch_one(ex)
        .await
}

/// Ce qu'une transition a produit, à publier après le COMMIT.
#[derive(Default)]
pub struct EffetsConcours {
    /// (destinataire, type, message)
    notifications: Vec<(Uuid, &'static str, String)>,
    /// (membre, règle d'engagement, clé d'idempotence, type d'objet, objet)
    reputation: Vec<(Uuid, &'static str, String, &'static str, Uuid)>,
    lien: String,
}

impl EffetsConcours {
    pub async fn publier(self, pool: &PgPool) {
        for (membre, regle, cle, type_objet, objet) in self.reputation {
            engagement::attribuer(pool, membre, regle, Some(type_objet), Some(objet), &cle).await;
        }
        for (membre, type_notif, message) in self.notifications {
            notification::creer_notification(pool, membre, type_notif, &message, Some(&self.lien)).await;
        }
    }
}

/// Verse la prime de participation de chaque participation PUBLIÉE, dans la
/// transaction de l'appelant. Seul chemin de ces primes (PC5) : la clé est la
/// même pour un concours annulé et un concours mené à terme.
pub async fn verser_primes_participation(
    conn: &mut PgConnection,
    concours: &ConcoursRow,
    effets: &mut EffetsConcours,
) -> Result<(), ApiErreur> {
    let regles = moteur::charger_regles(&mut *conn).await?;
    let montant = i32::from(concours.prime_participation.unwrap_or(regles.prime_concours_participation));

    let publiees: Vec<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT id, auteur_id FROM jeu.participation WHERE concours_id = $1 AND etat = 'publiee'",
    )
    .bind(concours.id)
    .fetch_all(&mut *conn)
    .await?;

    for (participation, auteur) in publiees {
        moteur::crediter(
            &mut *conn,
            auteur,
            montant,
            "concours",
            participation,
            None,
            &format!("concours:{}:participation:{participation}", concours.id),
        )
        .await?;
        effets.reputation.push((
            auteur,
            "jeu_concours_participation",
            format!("jeu:concours:{}:participation:{participation}", concours.id),
            "participation",
            participation,
        ));
    }
    Ok(())
}

/// Annule un concours sous verrou. Ne fait rien s'il n'est plus `actif`.
/// Verse les primes de participation déjà dues et prévient les participants
/// (US6, scénario 4).
pub async fn annuler(pool: &PgPool, id: Uuid, motif: &str) -> Result<bool, ApiErreur> {
    let mut tx = pool.begin().await?;
    let concours = sqlx::query_as::<_, ConcoursRow>(&format!(
        "SELECT {CONCOURS_COLONNES} FROM jeu.concours c WHERE c.id = $1 FOR UPDATE"
    ))
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Concours introuvable".into()))?;
    if concours.etat != "actif" {
        return Ok(false);
    }

    sqlx::query(
        "UPDATE jeu.concours SET etat = 'annule', motif_annulation = $2, updated_at = NOW() WHERE id = $1",
    )
    .bind(id)
    .bind(motif)
    .execute(&mut *tx)
    .await?;

    let mut effets = EffetsConcours { lien: lien_concours(id), ..Default::default() };
    verser_primes_participation(&mut tx, &concours, &mut effets).await?;

    // Tous ceux qui ont déposé quelque chose sont prévenus, pas seulement les publiés.
    let participants: Vec<Uuid> = sqlx::query_scalar(
        "SELECT DISTINCT auteur_id FROM jeu.participation
          WHERE concours_id = $1 AND etat IN ('en_attente', 'publiee')",
    )
    .bind(id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;

    for membre in participants {
        effets.notifications.push((
            membre,
            notif::CONCOURS_ANNULE,
            format!("Le concours « {} » est annulé. Motif : {motif}", concours.titre),
        ));
    }
    effets.publier(pool).await;
    Ok(true)
}

/// Lit un concours et fait avancer son cycle si une échéance est passée.
/// À appeler par TOUTE route qui lit un concours (PC1).
pub async fn resoudre_concours(pool: &PgPool, id: Uuid) -> Result<ConcoursRow, ApiErreur> {
    let concours = charger_concours(pool, id).await?;
    if concours.etat != "actif" {
        return Ok(concours);
    }
    let phase = concours.phase(Utc::now());
    if !matches!(phase, Phase::Vote | Phase::Deliberation | Phase::Resultats) {
        return Ok(concours);
    }

    // Seuil du vote : trop peu de participations publiées, le concours est annulé.
    if compter_publiees(pool, id).await? < i64::from(concours.minimum_participations) {
        annuler(pool, id, "participations publiées insuffisantes à l'ouverture du vote").await?;
        return charger_concours(pool, id).await;
    }

    if phase == Phase::Resultats {
        etablir_resultats(pool, id).await?;
        return charger_concours(pool, id).await;
    }
    Ok(concours)
}

// ─── Résultats (research D8, D10) ────────────────────────────────────────────

/// Deux taux à moins de 0,1 point l'un de l'autre sont ex aequo (FR-044).
const ECART_EX_AEQUO: f64 = 0.001;
/// Voix comptées à partir desquelles un votant reçoit la réputation du concours.
const VOIX_POUR_REPUTATION: i64 = 10;

/// Une participation dans le classement, avant ou après son établissement.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LigneClassement {
    pub participation_id: Uuid,
    pub auteur_id: Uuid,
    pub rang: i16,
    pub victoires: i32,
    pub duels: i32,
    pub taux: f64,
    pub sous_seuil: bool,
    pub place_jury: Option<i16>,
}

/// Le classement d'un concours, calculé depuis les voix COMPTÉES. Seule
/// définition du classement : les résultats, les finalistes du jury et le
/// suivi administrateur l'appellent tous (une seconde copie divergerait).
///
/// Ne comptent pas : les voix écartées, celles d'un votant suspendu ou
/// supprimé, et toute confrontation impliquant une participation qui n'est
/// plus publiée (retirée, suspendue) — FR-040, FR-055, FR-056.
pub async fn classement(
    conn: &mut PgConnection,
    concours: &ConcoursRow,
) -> Result<Vec<LigneClassement>, sqlx::Error> {
    let lignes: Vec<(Uuid, Uuid, i32, i32)> = sqlx::query_as(
        "WITH v AS (
             SELECT c.a_id, c.b_id, c.choix_id FROM jeu.confrontation c
               JOIN jeu.participation pa ON pa.id = c.a_id AND pa.etat = 'publiee'
               JOIN jeu.participation pb ON pb.id = c.b_id AND pb.etat = 'publiee'
               JOIN iam.utilisateur u ON u.id = c.votant_id
                AND u.deleted_at IS NULL AND u.etat::text <> 'suspendu'
              WHERE c.concours_id = $1 AND c.comptee),
         d AS (SELECT a_id AS pid, (choix_id = a_id)::int AS gagne FROM v
               UNION ALL
               SELECT b_id, (choix_id = b_id)::int FROM v)
         SELECT p.id, p.auteur_id, COALESCE(SUM(d.gagne), 0)::int, COUNT(d.pid)::int
           FROM jeu.participation p
           JOIN iam.utilisateur ua ON ua.id = p.auteur_id
            AND ua.deleted_at IS NULL AND ua.etat::text <> 'suspendu'
           LEFT JOIN d ON d.pid = p.id
          WHERE p.concours_id = $1 AND p.etat = 'publiee'
          GROUP BY p.id, p.auteur_id, p.created_at
          ORDER BY p.created_at",
    )
    .bind(concours.id)
    .fetch_all(conn)
    .await?;

    let seuil = i32::from(concours.presentations_min);
    let mut classees: Vec<LigneClassement> = lignes
        .into_iter()
        .map(|(participation_id, auteur_id, victoires, duels)| LigneClassement {
            participation_id,
            auteur_id,
            rang: 0,
            victoires,
            duels,
            taux: if duels > 0 { f64::from(victoires) / f64::from(duels) } else { 0.0 },
            sous_seuil: duels < seuil,
            place_jury: None,
        })
        .collect();

    // Le podium du jury, s'il existe, prend la tête dans l'ordre qu'il a fixé.
    let podium = concours.podium_jury.clone().unwrap_or_default();
    for (i, pid) in podium.iter().enumerate() {
        if let Some(l) = classees.iter_mut().find(|l| l.participation_id == *pid) {
            l.place_jury = Some((i + 1) as i16);
        }
    }

    // Jury d'abord ; puis les participations assez présentées, par taux
    // décroissant ; les autres après, hors podium (FR-043).
    classees.sort_by(|a, b| {
        let cle = |l: &LigneClassement| (l.place_jury.is_none(), l.place_jury.unwrap_or(0), l.sous_seuil);
        cle(a).cmp(&cle(b)).then(b.taux.partial_cmp(&a.taux).unwrap_or(std::cmp::Ordering::Equal))
    });

    let mut precedent: Option<(Option<i16>, bool, f64, i16)> = None;
    for (i, l) in classees.iter_mut().enumerate() {
        let rang_brut = (i + 1) as i16;
        l.rang = match precedent {
            // Ex aequo : même groupe (hors jury), même seuil, taux à moins de 0,1 point.
            Some((None, sous, taux, rang))
                if l.place_jury.is_none() && sous == l.sous_seuil && (taux - l.taux).abs() < ECART_EX_AEQUO =>
            {
                rang
            }
            _ => rang_brut,
        };
        precedent = Some((l.place_jury, l.sous_seuil, l.taux, l.rang));
    }
    Ok(classees)
}

/// Établit et FIGE les résultats d'un concours dont le vote est clos, sous
/// verrou de sa ligne ; ne fait rien s'il n'est plus `actif`.
///
/// Dans la transaction : les lignes de résultat, l'état `resultats`, les primes
/// de participation (PC5) et de podium. Après le COMMIT : réputation (lauréats,
/// votants actifs) et notifications.
async fn etablir_resultats(pool: &PgPool, id: Uuid) -> Result<(), ApiErreur> {
    let mut tx = pool.begin().await?;
    let concours = sqlx::query_as::<_, ConcoursRow>(&format!(
        "SELECT {CONCOURS_COLONNES} FROM jeu.concours c WHERE c.id = $1 FOR UPDATE"
    ))
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Concours introuvable".into()))?;
    if concours.etat != "actif" || concours.phase(Utc::now()) != Phase::Resultats {
        return Ok(());
    }

    let lignes = classement(&mut tx, &concours).await?;
    for l in &lignes {
        sqlx::query(
            "INSERT INTO jeu.resultat_concours
                (concours_id, participation_id, rang, victoires, duels, taux, sous_seuil, place_jury)
             VALUES ($1, $2, $3, $4, $5, $6::numeric(5,4), $7, $8)
             ON CONFLICT DO NOTHING",
        )
        .bind(id)
        .bind(l.participation_id)
        .bind(l.rang)
        .bind(l.victoires)
        .bind(l.duels)
        .bind(l.taux)
        .bind(l.sous_seuil)
        .bind(l.place_jury)
        .execute(&mut *tx)
        .await?;
    }
    sqlx::query("UPDATE jeu.concours SET etat = 'resultats', resultats_at = NOW(), updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    let mut effets = EffetsConcours { lien: lien_concours(id), ..Default::default() };
    verser_primes_participation(&mut tx, &concours, &mut effets).await?;

    // Podium : les trois premiers rangs, hors participations sous le seuil. Les
    // ex aequo touchent la prime de leur rang.
    let regles = moteur::charger_regles(&mut *tx).await?;
    let primes = concours.prime_podium.clone().unwrap_or(regles.prime_concours_podium.clone());
    let mut laureats: Vec<(Uuid, i16)> = Vec::new();
    for l in lignes.iter().filter(|l| l.rang <= 3 && (!l.sous_seuil || l.place_jury.is_some())) {
        let montant = i32::from(primes.get((l.rang - 1) as usize).copied().unwrap_or(0));
        moteur::crediter(
            &mut tx,
            l.auteur_id,
            montant,
            "concours",
            l.participation_id,
            None,
            &format!("concours:{id}:podium:{}", l.participation_id),
        )
        .await?;
        effets.reputation.push((
            l.auteur_id,
            "jeu_concours_podium",
            format!("jeu:concours:{id}:podium:{}", l.participation_id),
            "participation",
            l.participation_id,
        ));
        laureats.push((l.auteur_id, l.rang));
    }

    // Réputation des votants assidus : une fois par concours (D10).
    let votants: Vec<Uuid> = sqlx::query_scalar(
        "SELECT votant_id FROM jeu.confrontation WHERE concours_id = $1 AND comptee
          GROUP BY votant_id HAVING COUNT(*) >= $2",
    )
    .bind(id)
    .bind(VOIX_POUR_REPUTATION)
    .fetch_all(&mut *tx)
    .await?;
    for v in votants {
        effets.reputation.push((v, "jeu_concours_vote", format!("jeu:concours:{id}:votant:{v}"), "concours", id));
    }

    let participants: Vec<Uuid> = sqlx::query_scalar(
        "SELECT DISTINCT auteur_id FROM jeu.participation WHERE concours_id = $1 AND etat = 'publiee'",
    )
    .bind(id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;

    for membre in participants {
        if let Some((_, rang)) = laureats.iter().find(|(m, _)| *m == membre) {
            effets.notifications.push((
                membre,
                notif::CONCOURS_LAUREAT,
                format!("Bravo ! Votre photo est {}{} du concours « {} ».", rang, if *rang == 1 { "re" } else { "e" }, concours.titre),
            ));
        } else {
            effets.notifications.push((
                membre,
                notif::CONCOURS_RESULTATS,
                format!("Les résultats du concours « {} » sont publiés.", concours.titre),
            ));
        }
    }
    effets.publier(pool).await;
    Ok(())
}

// ─── Vote à l'aveugle (research D7, D9) ──────────────────────────────────────

/// Seuil en dessous duquel une voix est trop rapide pour avoir regardé les deux
/// photos : elle est enregistrée, mais ne compte pas.
const VOTE_MIN_MS: i64 = 1_000;

/// Une participation telle qu'un votant la voit : sa photo et sa légende, rien
/// qui dise qui l'a prise (FR-039).
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct CoteConfrontation {
    pub id: Uuid,
    pub media_url: String,
    pub legende: Option<String>,
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum Tirage {
    Paire {
        id: Uuid,
        gauche: CoteConfrontation,
        droite: CoteConfrontation,
        votes_exprimes: i64,
        votes_max: Option<i32>,
    },
    Termine {
        termine: bool,
        /// `toutes_vues` ou `plafond`.
        raison: &'static str,
        votes_exprimes: i64,
    },
}

/// Participations qui peuvent entrer dans une paire pour ce votant : publiées,
/// pas les siennes, d'un compte qui n'est pas suspendu (FR-035, FR-056).
const ADMISSIBLES: &str = "
    FROM jeu.participation p
    JOIN iam.utilisateur u ON u.id = p.auteur_id
   WHERE p.concours_id = $1 AND p.etat = 'publiee' AND p.auteur_id <> $2
     AND u.deleted_at IS NULL AND u.etat::text <> 'suspendu'";

async fn votes_exprimes(conn: &mut PgConnection, concours_id: Uuid, votant: Uuid) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT COUNT(*) FROM jeu.confrontation WHERE concours_id = $1 AND votant_id = $2 AND choix_id IS NOT NULL",
    )
    .bind(concours_id)
    .bind(votant)
    .fetch_one(conn)
    .await
}

async fn servir_paire(
    conn: &mut PgConnection,
    concours: &ConcoursRow,
    votant: Uuid,
    confrontation: (Uuid, Uuid, Uuid, bool),
) -> Result<Tirage, ApiErreur> {
    let (id, a, b, gauche_est_a) = confrontation;
    let charger = |pid: Uuid| {
        sqlx::query_as::<_, CoteConfrontation>("SELECT id, media_url, legende FROM jeu.participation WHERE id = $1")
            .bind(pid)
    };
    let cote_a = charger(a).fetch_one(&mut *conn).await?;
    let cote_b = charger(b).fetch_one(&mut *conn).await?;
    let (gauche, droite) = if gauche_est_a { (cote_a, cote_b) } else { (cote_b, cote_a) };
    Ok(Tirage::Paire {
        id,
        gauche,
        droite,
        votes_exprimes: votes_exprimes(conn, concours.id, votant).await?,
        votes_max: concours.votes_max,
    })
}

async fn confrontation_ouverte(
    conn: &mut PgConnection,
    concours_id: Uuid,
    votant: Uuid,
) -> Result<Option<(Uuid, Uuid, Uuid, bool)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, a_id, b_id, gauche_est_a FROM jeu.confrontation
          WHERE concours_id = $1 AND votant_id = $2 AND choix_id IS NULL",
    )
    .bind(concours_id)
    .bind(votant)
    .fetch_optional(conn)
    .await
}

/// Rend la confrontation ouverte du votant, ou en tire une nouvelle (D7) :
/// A est la participation la MOINS présentée qui a encore un partenaire que ce
/// votant n'a jamais vu avec elle ; B, la moins présentée de ces partenaires.
/// Les présentations sont comptées à la présentation, pas au vote : deux
/// votants simultanés ne reçoivent pas tous deux la même « moins vue ».
pub async fn tirer_confrontation(
    pool: &PgPool,
    concours: &ConcoursRow,
    votant: Uuid,
) -> Result<Tirage, ApiErreur> {
    let mut tx = pool.begin().await?;

    // Recharger la page rend la même paire : on ne « passe » pas une paire.
    if let Some(ouverte) = confrontation_ouverte(&mut tx, concours.id, votant).await? {
        let tirage = servir_paire(&mut tx, concours, votant, ouverte).await?;
        tx.commit().await?;
        return Ok(tirage);
    }

    let deja = votes_exprimes(&mut tx, concours.id, votant).await?;
    if concours.votes_max.is_some_and(|max| deja >= i64::from(max)) {
        return Ok(Tirage::Termine { termine: true, raison: "plafond", votes_exprimes: deja });
    }

    // « Encore un partenaire jamais vu avec lui » : paire canonique (a < b).
    let jamais_vue = "NOT EXISTS (SELECT 1 FROM jeu.confrontation c
                                   WHERE c.concours_id = $1 AND c.votant_id = $2
                                     AND c.a_id = LEAST(p.id, q.id) AND c.b_id = GREATEST(p.id, q.id))";
    let a: Option<Uuid> = sqlx::query_scalar(&format!(
        "SELECT p.id {ADMISSIBLES}
            AND EXISTS (SELECT 1 FROM jeu.participation q JOIN iam.utilisateur uq ON uq.id = q.auteur_id
                         WHERE q.concours_id = $1 AND q.etat = 'publiee' AND q.auteur_id <> $2
                           AND q.id <> p.id AND uq.deleted_at IS NULL AND uq.etat::text <> 'suspendu'
                           AND {jamais_vue})
          ORDER BY p.nombre_presentations, random() LIMIT 1"
    ))
    .bind(concours.id)
    .bind(votant)
    .fetch_optional(&mut *tx)
    .await?;
    let Some(a) = a else {
        return Ok(Tirage::Termine { termine: true, raison: "toutes_vues", votes_exprimes: deja });
    };

    // B : on renomme `p` en `q` pour réutiliser la même condition « jamais vue ».
    let b: Uuid = sqlx::query_scalar(&format!(
        "SELECT q.id FROM (SELECT p.* {ADMISSIBLES}) q
           CROSS JOIN (SELECT $3::uuid AS id) p
          WHERE q.id <> p.id AND {jamais_vue}
          ORDER BY q.nombre_presentations, random() LIMIT 1"
    ))
    .bind(concours.id)
    .bind(votant)
    .bind(a)
    .fetch_one(&mut *tx)
    .await?;

    let (petit, grand) = if a < b { (a, b) } else { (b, a) };
    let gauche_est_a: bool = rand::random();
    let inseree: Option<Uuid> = sqlx::query_scalar(
        "INSERT INTO jeu.confrontation (concours_id, votant_id, a_id, b_id, gauche_est_a)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT DO NOTHING RETURNING id",
    )
    .bind(concours.id)
    .bind(votant)
    .bind(petit)
    .bind(grand)
    .bind(gauche_est_a)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(id) = inseree else {
        // Un second onglet a tiré en même temps : on sert la sienne.
        tx.rollback().await?;
        let mut conn = pool.acquire().await?;
        let ouverte = confrontation_ouverte(&mut conn, concours.id, votant)
            .await?
            .ok_or_else(|| ApiErreur::Conflit("Réessayez dans un instant".into()))?;
        return servir_paire(&mut conn, concours, votant, ouverte).await;
    };

    sqlx::query("UPDATE jeu.participation SET nombre_presentations = nombre_presentations + 1 WHERE id IN ($1, $2)")
        .bind(petit)
        .bind(grand)
        .execute(&mut *tx)
        .await?;
    let tirage = servir_paire(&mut tx, concours, votant, (id, petit, grand, gauche_est_a)).await?;
    tx.commit().await?;
    Ok(tirage)
}

/// Enregistre la voix d'un votant (D9), puis tire la confrontation suivante.
///
/// La voix est TOUJOURS acceptée ; `comptee` et `motif_ecart` sont fixés ici
/// et ne sortent jamais vers le votant : le règlement annonce les règles, mais
/// l'écran ne dit pas au fraudeur ce qui l'a trahi.
pub async fn voter(
    pool: &PgPool,
    concours: &ConcoursRow,
    votant: Uuid,
    confrontation_id: Uuid,
    gauche: bool,
) -> Result<Tirage, ApiErreur> {
    let mut tx = pool.begin().await?;
    #[allow(clippy::type_complexity)]
    let ligne: Option<(Uuid, Uuid, bool, Option<Uuid>, chrono::DateTime<Utc>, Uuid, Uuid)> = sqlx::query_as(
        "SELECT c.a_id, c.b_id, c.gauche_est_a, c.choix_id, c.presentee_at, pa.auteur_id, pb.auteur_id
           FROM jeu.confrontation c
           JOIN jeu.participation pa ON pa.id = c.a_id
           JOIN jeu.participation pb ON pb.id = c.b_id
          WHERE c.id = $1 AND c.concours_id = $2 AND c.votant_id = $3
          FOR UPDATE OF c",
    )
    .bind(confrontation_id)
    .bind(concours.id)
    .bind(votant)
    .fetch_optional(&mut *tx)
    .await?;
    let (a, b, gauche_est_a, deja, presentee_at, auteur_a, auteur_b) =
        ligne.ok_or_else(|| ApiErreur::NonTrouve("Confrontation introuvable".into()))?;
    if auteur_a == votant || auteur_b == votant {
        // Impossible par le tirage ; refusé quand même (défense en profondeur).
        return Err(ApiErreur::AccesInterdit("On ne vote pas sur sa propre photo".into()));
    }

    if deja.is_none() {
        let choix = if gauche == gauche_est_a { a } else { b };
        let (cree, verifie): (chrono::DateTime<Utc>, bool) =
            sqlx::query_as("SELECT created_at, email_verifie FROM iam.utilisateur WHERE id = $1")
                .bind(votant)
                .fetch_one(&mut *tx)
                .await?;
        let motif = if (Utc::now() - presentee_at).num_milliseconds() < VOTE_MIN_MS {
            Some("trop_rapide")
        } else if cree > concours.vote_debut {
            Some("compte_recent")
        } else if !verifie {
            Some("non_verifie")
        } else {
            None
        };
        sqlx::query(
            "UPDATE jeu.confrontation
                SET choix_id = $2, vote_at = NOW(), comptee = $3, motif_ecart = $4
              WHERE id = $1 AND choix_id IS NULL",
        )
        .bind(confrontation_id)
        .bind(choix)
        .bind(motif.is_none())
        .bind(motif)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    // Un renvoi du même vote est idempotent : on enchaîne simplement.
    tirer_confrontation(pool, concours, votant).await
}

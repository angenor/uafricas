//! Moteur du jeu (feature 013) : épreuve servable, tirage d'une série, déroulé
//! d'une partie, journal du score.
//!
//! Trois règles tiennent ce fichier :
//!
//! 1. **« Épreuve servable » n'est définie qu'ici** ([`SERVABLE_SQL`]). Tout
//!    tirage (partie libre, défi, duel, partie par pays) passe par ce fragment.
//!    Dupliqué, il divergerait, et une épreuve dont la source est dépubliée
//!    finirait servie par l'un des chemins.
//! 2. **[`crediter`] est le seul point d'écriture du score** : journal `gain`
//!    et ses deux agrégats, dans la transaction de l'appelant. L'idempotence est
//!    celle du journal (`cle_idempotence UNIQUE`), sur le patron de
//!    `services::engagement`.
//! 3. **Le serveur tient la montre.** Le temps d'une épreuve court de
//!    `presentee_at`, écrit ici, jamais d'une valeur venue du client.
//!
//! Le jeu ne crédite AUCUN point d'engagement : seuls des mouvements à 0 point
//! portant de la réputation passent par `services::engagement`, et toujours
//! après le COMMIT.

use chrono::{DateTime, Duration, Utc};
use rand::seq::SliceRandom;
use sqlx::{PgConnection, PgExecutor, PgPool};
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::jeu::{
    Correction, DefiRow, DuelRow, EpreuveRow, EpreuveServie, PartieDuel, PartieRow, PaysCorrection,
    Presentation, PropositionServie, ReglesJeu, ReponseJoueur, Solution, DEFI_COLONNES, DUEL_COLONNES, EPREUVE_COLONNES, PARTIE_COLONNES,
    REGLES_COLONNES,
};
use crate::services::engagement;

/// La SEULE définition d'une épreuve servable : jouable, dans un module ouvert,
/// et dont la source éventuelle est encore publiée et, pour une épreuve
/// dérivée, inchangée depuis la dérivation (FR-011, FR-012). La jointure externe
/// sur `v_source` fait qu'une source supprimée (aucune ligne) donne
/// `visible` NULL, donc non servable.
///
/// S'emploie après un `SELECT …`, et se complète par des `AND …`.
pub const SERVABLE_SQL: &str = "
    FROM jeu.epreuve e
    JOIN jeu.module m ON m.code = e.module_code
    LEFT JOIN jeu.v_source s
           ON s.type_source = e.type_source AND s.source_id = e.source_id
   WHERE e.etat = 'jouable'
     AND m.ouvert
     AND (e.type_source IS NULL
          OR (COALESCE(s.visible, FALSE)
              AND (e.origine = 'saisie' OR s.empreinte = e.source_empreinte)))";

/// Tirage VARIÉ parmi les servables restreintes par `filtre` (une suite de
/// `AND …` sur `e`). Un `ORDER BY random()` nu enchaînait quatre drapeaux, ou
/// posait « capitale du Sénégal ? » puis « Dakar est la capitale de quel
/// pays ? », la seconde donnant la réponse de la première. L'ordre est donc :
///
/// 1. `priorite` d'abord (expression booléenne, FAUX en tête ; `FALSE` si sans
///    objet) ;
/// 2. une seule épreuve par SOURCE avant toute deuxième — pas d'exclusion :
///    « jouer sur ce pays » doit pouvoir remplir une partie avec sa fiche ;
/// 3. dans chaque module, rotation entre les formes (le thème, pour la saisie) ;
/// 4. entre modules, rotation elle aussi.
///
/// L'appelant ajoute le `LIMIT`.
pub fn serie_variee_sql(filtre: &str, priorite: &str) -> String {
    format!(
        "WITH c AS (
             SELECT e.id, e.module_code, COALESCE(e.forme, e.theme, '') AS famille,
                    COALESCE(e.source_id, e.id) AS src, ({priorite}) AS prio
             {SERVABLE_SQL} {filtre}),
         s AS (SELECT c.*, row_number() OVER (PARTITION BY src ORDER BY prio, random()) AS rs FROM c),
         f AS (SELECT s.*, row_number() OVER (PARTITION BY module_code, famille
                                              ORDER BY prio, rs, random()) AS rf FROM s),
         m AS (SELECT f.*, row_number() OVER (PARTITION BY module_code
                                              ORDER BY prio, rs, rf, random()) AS rm FROM f)
         SELECT id FROM m ORDER BY prio, rs, rm, random()"
    )
}

/// Tolérance réseau ajoutée au temps imparti avant de refuser une réponse.
const TOLERANCE_RESEAU_MS: i64 = 2_000;
/// Marge de chargement accordée à une épreuve qui porte un média.
const MARGE_MEDIA_MS: i64 = 5_000;

pub async fn charger_regles<'e, E: PgExecutor<'e>>(ex: E) -> Result<ReglesJeu, sqlx::Error> {
    sqlx::query_as::<_, ReglesJeu>(&format!("SELECT {REGLES_COLONNES} FROM jeu.regles WHERE id"))
        .fetch_one(ex)
        .await
}

/// Nombre d'épreuves servables, éventuellement restreint.
pub async fn compter_servables<'e, E: PgExecutor<'e>>(
    ex: E,
    module: Option<&str>,
    pays_id: Option<Uuid>,
    theme: Option<&str>,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(&format!(
        "SELECT COUNT(*) {SERVABLE_SQL}
            AND ($1::text IS NULL OR e.module_code = $1)
            AND ($2::uuid IS NULL OR e.pays_id = $2)
            AND ($3::text IS NULL OR e.theme = $3)"
    ))
    .bind(module)
    .bind(pays_id)
    .bind(theme)
    .fetch_one(ex)
    .await
}

/// Tire une série en UNE requête. Avec `neuves_seulement`, l'anti-jointure sur
/// `jeu.reponse` écarte toute épreuve à laquelle le membre a déjà répondu, tous
/// cadres confondus (FR-015). Peut renvoyer moins que `taille` : à l'appelant
/// d'en décider.
pub async fn composer_serie(
    conn: &mut PgConnection,
    utilisateur_id: Uuid,
    module: Option<&str>,
    pays_id: Option<Uuid>,
    theme: Option<&str>,
    taille: i16,
    neuves_seulement: bool,
) -> Result<Vec<Uuid>, sqlx::Error> {
    let tirage = serie_variee_sql(
        "AND ($1::text IS NULL OR e.module_code = $1)
         AND ($2::uuid IS NULL OR e.pays_id = $2)
         AND ($3::text IS NULL OR e.theme = $3)
         AND (NOT $4 OR NOT EXISTS (
                 SELECT 1 FROM jeu.reponse r
                  WHERE r.utilisateur_id = $5 AND r.epreuve_id = e.id))",
        "FALSE",
    );
    sqlx::query_scalar(&format!("{tirage} LIMIT $6"))
    .bind(module)
    .bind(pays_id)
    .bind(theme)
    .bind(neuves_seulement)
    .bind(utilisateur_id)
    .bind(i64::from(taille))
    .fetch_all(conn)
    .await
}

/// Écrit un gain de score de jeu et tient ses deux agrégats.
///
/// Le pays de rattachement (`COALESCE(origine, résidence)`) et la saison en
/// cours sont lus ICI et figés sur la ligne : un changement de profil ultérieur
/// ne déplace aucun gain (FR-053, FR-054). Les agrégats ne sont touchés que si
/// la ligne est réellement insérée ; renvoie `false` sur un rejeu (FR-025).
pub async fn crediter(
    conn: &mut PgConnection,
    utilisateur_id: Uuid,
    montant: i32,
    origine: &str,
    reference_id: Uuid,
    module_code: Option<&str>,
    cle_idempotence: &str,
) -> Result<bool, sqlx::Error> {
    if montant <= 0 {
        return Ok(false);
    }

    let insere: Option<(Option<Uuid>, Option<Uuid>)> = sqlx::query_as(
        "INSERT INTO jeu.gain
            (utilisateur_id, montant, origine, reference_id, module_code,
             saison_id, pays_id, cle_idempotence)
         SELECT $1, $2, $3, $4, $5,
                (SELECT id FROM jeu.saison WHERE debut_at <= NOW() AND NOW() < fin_at LIMIT 1),
                (SELECT COALESCE(pays_origine_id, pays_residence_id)
                   FROM iam.utilisateur WHERE id = $1),
                $6
         ON CONFLICT (cle_idempotence) DO NOTHING
         RETURNING saison_id, pays_id",
    )
    .bind(utilisateur_id)
    .bind(montant)
    .bind(origine)
    .bind(reference_id)
    .bind(module_code)
    .bind(cle_idempotence)
    .fetch_optional(&mut *conn)
    .await?;

    let Some((saison_id, pays_id)) = insere else {
        return Ok(false);
    };

    if let Some(saison_id) = saison_id {
        sqlx::query(
            "INSERT INTO jeu.score_saison (saison_id, utilisateur_id, pays_id, score, atteint_at)
             VALUES ($1, $2, $3, $4, NOW())
             ON CONFLICT ON CONSTRAINT uq_score_saison
             DO UPDATE SET score = jeu.score_saison.score + EXCLUDED.score, atteint_at = NOW()",
        )
        .bind(saison_id)
        .bind(utilisateur_id)
        .bind(pays_id)
        .bind(montant)
        .execute(&mut *conn)
        .await?;
    }

    sqlx::query(
        "INSERT INTO jeu.joueur (utilisateur_id, score_total) VALUES ($1, $2)
         ON CONFLICT (utilisateur_id)
         DO UPDATE SET score_total = jeu.joueur.score_total + EXCLUDED.score_total,
                       updated_at = NOW()",
    )
    .bind(utilisateur_id)
    .bind(montant)
    .execute(&mut *conn)
    .await?;

    Ok(true)
}

// ─── Déroulé d'une partie, commun aux cadres libre, entraînement, défi, duel ──

async fn charger_epreuve(conn: &mut PgConnection, id: Uuid) -> Result<EpreuveRow, ApiErreur> {
    sqlx::query_as::<_, EpreuveRow>(&format!(
        "SELECT {EPREUVE_COLONNES} FROM jeu.epreuve e WHERE e.id = $1"
    ))
    .bind(id)
    .fetch_optional(conn)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Épreuve introuvable".into()))
}

/// Temps accordé à une épreuve, en millisecondes, hors tolérance réseau.
/// L'ordre et les paires disposent d'un temps majoré : déplacer quatre
/// éléments demande plus que cliquer une proposition (feature 014, D3).
fn delai_ms(regles: &ReglesJeu, epreuve: &EpreuveRow) -> i64 {
    let mut ms = i64::from(regles.temps_epreuve_s) * 1_000;
    if epreuve.media_type.is_some() {
        ms += MARGE_MEDIA_MS;
    }
    if matches!(epreuve.type_reponse.as_str(), "ordre" | "paires") {
        ms += i64::from(regles.majoration_ordre_paires_s) * 1_000;
    }
    ms
}

/// Chaque texte avec sa clé : son rang d'origine dans le tableau stocké.
fn avec_cles(textes: &[String]) -> Vec<PropositionServie> {
    textes
        .iter()
        .enumerate()
        .map(|(i, texte)| PropositionServie { cle: (i + 1) as i16, texte: texte.clone() })
        .collect()
}

/// Mélange les propositions (FR-030) et, pour les paires, la colonne de droite,
/// indépendamment. Chaque élément garde son rang d'origine comme clé : comme
/// les tableaux sont STOCKÉS au hasard, ce rang ne dit rien de la solution.
fn servir(epreuve: &EpreuveRow) -> EpreuveServie {
    let mut rng = rand::thread_rng();
    let mut propositions = avec_cles(&epreuve.propositions);
    propositions.shuffle(&mut rng);
    let appariements = epreuve.appariements.as_deref().map(|droite| {
        let mut v = avec_cles(droite);
        v.shuffle(&mut rng);
        v
    });

    EpreuveServie {
        id: epreuve.id,
        type_reponse: epreuve.type_reponse.clone(),
        enonce: epreuve.enonce.clone(),
        media_type: epreuve.media_type.clone(),
        media_url: epreuve.media_url.clone(),
        difficulte: epreuve.difficulte,
        propositions,
        appariements,
    }
}

/// Comme [`servir`], mais avec un ordre STABLE pour une graine donnée : en duel
/// direct, l'état est relu toutes les trois secondes, un ordre tiré à chaque
/// lecture ferait danser les propositions sous les yeux du joueur. La colonne
/// de droite des paires est mélangée avec une graine DÉRIVÉE : avec la même,
/// les deux colonnes suivraient la même permutation.
pub fn servir_stable(epreuve: &EpreuveRow, graine: u64) -> EpreuveServie {
    use rand::SeedableRng;
    let mut servie = servir(epreuve);
    servie.propositions.sort_by_key(|p| p.cle);
    servie.propositions.shuffle(&mut rand::rngs::StdRng::seed_from_u64(graine));
    if let Some(droite) = servie.appariements.as_mut() {
        droite.sort_by_key(|p| p.cle);
        droite.shuffle(&mut rand::rngs::StdRng::seed_from_u64(graine ^ 0x9E37_79B9));
    }
    servie
}

/// Vrai si `v` contient exactement les clés 1..=n, chacune une fois.
fn est_permutation(v: &[i16], n: usize) -> bool {
    if v.len() != n {
        return false;
    }
    let mut vues = vec![false; n];
    v.iter().all(|&c| {
        let i = c as usize;
        c >= 1 && i <= n && !std::mem::replace(&mut vues[i - 1], true)
    })
}

/// Corrige une réponse, quel que soit le type de l'épreuve (feature 014, D2).
/// `pays_id` : le pays désigné d'une épreuve « carte », déjà résolu.
///
/// `Ok(None)` : sans réponse. `Err` : une réponse qui n'a pas la forme de
/// l'épreuve (mauvais type, clé inconnue, ordre qui omet ou répète une clé),
/// refusée avant toute écriture. Tout ou rien : il n'y a pas de demi-réponse.
pub fn evaluer(
    epreuve: &EpreuveRow,
    reponse: &ReponseJoueur,
    pays_id: Option<Uuid>,
) -> Result<Option<bool>, ApiErreur> {
    if reponse.est_vide() {
        return Ok(None);
    }
    let formes = [reponse.cle.is_some(), reponse.ordre.is_some(), reponse.paires.is_some(),
                  reponse.pays.is_some()];
    let autre_forme = || ApiErreur::Validation("Cette réponse ne correspond pas au type de l'épreuve".into());
    if formes.iter().filter(|f| **f).count() > 1 {
        return Err(autre_forme());
    }
    let n = epreuve.propositions.len();

    match epreuve.type_reponse.as_str() {
        "choix" => {
            let c = reponse.cle.ok_or_else(autre_forme)?;
            if c < 1 || c as usize > n {
                return Err(ApiErreur::Validation("Proposition inconnue".into()));
            }
            Ok(Some(Some(c) == epreuve.bonne_reponse))
        }
        "ordre" => {
            let ordre = reponse.ordre.as_deref().ok_or_else(autre_forme)?;
            if !est_permutation(ordre, n) {
                return Err(ApiErreur::Validation(
                    "L'ordre doit reprendre chaque élément une fois et une seule".into(),
                ));
            }
            Ok(Some(epreuve.solution.as_deref() == Some(ordre)))
        }
        "paires" => {
            let paires = reponse.paires.as_deref().ok_or_else(autre_forme)?;
            let droite = epreuve.appariements.as_ref().map_or(0, Vec::len);
            if paires.len() != n || !est_permutation(paires, droite) {
                return Err(ApiErreur::Validation(
                    "Chaque élément doit être associé à un correspondant différent".into(),
                ));
            }
            Ok(Some(epreuve.solution.as_deref() == Some(paires)))
        }
        "carte" => {
            reponse.pays.as_ref().ok_or_else(autre_forme)?;
            let id = pays_id
                .ok_or_else(|| ApiErreur::Validation("Pays inconnu : désignez un pays d'Afrique".into()))?;
            Ok(Some(Some(id) == epreuve.reponse_pays_id))
        }
        _ => Err(autre_forme()),
    }
}

/// Résout le code ISO2 d'une réponse « carte » en pays, borné aux 55 pays
/// d'Afrique. `None` si la réponse n'est pas une carte ; erreur si le code est
/// inconnu (le pays d'un autre continent n'est pas une réponse possible).
async fn resoudre_pays_joue(
    conn: &mut PgConnection,
    reponse: &ReponseJoueur,
) -> Result<Option<Uuid>, ApiErreur> {
    let Some(iso) = reponse.pays.as_deref() else {
        return Ok(None);
    };
    let codes: Vec<String> = crate::constants::afripulse_pays_autorises::PAYS_AFRICAINS_ISO2
        .iter()
        .map(|c| c.to_lowercase())
        .collect();
    let id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM shared.pays WHERE LOWER(code_iso2) = LOWER($1) AND LOWER(code_iso2) = ANY($2)",
    )
    .bind(iso.trim())
    .bind(&codes)
    .fetch_optional(conn)
    .await?;
    id.map(Some)
        .ok_or_else(|| ApiErreur::Validation("Pays inconnu : désignez un pays d'Afrique".into()))
}

async fn pays_correction(
    conn: &mut PgConnection,
    id: Option<Uuid>,
) -> Result<Option<PaysCorrection>, sqlx::Error> {
    let Some(id) = id else {
        return Ok(None);
    };
    let ligne: Option<(Option<String>, String)> =
        sqlx::query_as("SELECT LOWER(code_iso2), nom FROM shared.pays WHERE id = $1")
            .bind(id)
            .fetch_optional(conn)
            .await?;
    Ok(ligne.map(|(iso, nom)| PaysCorrection { iso: iso.unwrap_or_default(), nom }))
}

/// La solution complète d'une épreuve, telle que la correction la montre.
pub async fn solution_de(
    conn: &mut PgConnection,
    epreuve: &EpreuveRow,
) -> Result<Solution, sqlx::Error> {
    let valeurs = epreuve.valeurs.as_ref().map(|v| {
        v.iter().enumerate().map(|(i, t)| ((i + 1) as i16, t.clone())).collect()
    });
    Ok(Solution {
        type_reponse: epreuve.type_reponse.clone(),
        bonne_cle: epreuve.bonne_reponse,
        solution: epreuve.solution.clone(),
        valeurs,
        bon_pays: pays_correction(conn, epreuve.reponse_pays_id).await?,
    })
}

/// Ce qui a été joué, reconstruit depuis ce que `jeu.reponse` a enregistré.
pub async fn jouee_depuis(
    conn: &mut PgConnection,
    epreuve: &EpreuveRow,
    cle: Option<i16>,
    detail: Option<Vec<i16>>,
    pays_id: Option<Uuid>,
) -> Result<ReponseJoueur, sqlx::Error> {
    let (ordre, paires) = match epreuve.type_reponse.as_str() {
        "ordre" => (detail, None),
        "paires" => (None, detail),
        _ => (None, None),
    };
    Ok(ReponseJoueur {
        cle,
        ordre,
        paires,
        pays: pays_correction(conn, pays_id).await?.map(|p| p.iso),
    })
}

/// Délai d'une épreuve, exposé pour l'état d'une manche directe.
pub fn delai_epreuve_ms(regles: &ReglesJeu, epreuve: &EpreuveRow) -> i64 {
    delai_ms(regles, epreuve)
}

/// Lien vers le contenu dont l'épreuve est tirée (FR-007). Codimoi et FactCheck
/// n'ont pas de page de détail : le lien mène à la page du module.
pub async fn lien_source(
    conn: &mut PgConnection,
    type_source: Option<&str>,
    source_id: Option<Uuid>,
) -> Result<Option<String>, sqlx::Error> {
    let (Some(type_source), Some(source_id)) = (type_source, source_id) else {
        return Ok(None);
    };

    let (table, segment) = match type_source {
        "codimoi" => return Ok(Some("/codi-moi".to_string())),
        "factcheck" => return Ok(Some("/universite/gouvernance/factcheck".to_string())),
        "fiche_pays" => return Ok(Some(format!("/opportunite-afrique/{source_id}"))),
        "site_touristique" => ("country_profile.site_touristique", "sites"),
        "recette_culinaire" => ("country_profile.recette_culinaire", "recettes"),
        "personnalite_connue" => ("country_profile.personnalite_connue", "personnalites"),
        // Un peuple n'a pas de page à lui : on renvoie à sa fiche pays.
        "groupe_ethnique" => {
            let fiche_id: Option<Uuid> = sqlx::query_scalar(
                "SELECT fiche_pays_id FROM country_profile.groupe_ethnique WHERE id = $1",
            )
            .bind(source_id)
            .fetch_optional(conn)
            .await?;
            return Ok(fiche_id.map(|f| format!("/opportunite-afrique/{f}")));
        }
        _ => return Ok(None),
    };

    let fiche_id: Option<Uuid> =
        sqlx::query_scalar(&format!("SELECT fiche_pays_id FROM {table} WHERE id = $1"))
            .bind(source_id)
            .fetch_optional(conn)
            .await?;

    Ok(fiche_id.map(|f| format!("/opportunite-afrique/{f}/{segment}/{source_id}")))
}

/// Le membre peut-il encore signaler cette épreuve ? Une seule fois par épreuve
/// (FR-082), et pas une épreuve qu'il n'a pas réellement vue.
async fn est_signalable(
    conn: &mut PgConnection,
    epreuve_id: Uuid,
    utilisateur_id: Uuid,
    issue: &str,
) -> Result<bool, sqlx::Error> {
    if issue == "injouable" {
        return Ok(false);
    }
    let deja: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM jeu.signalement_epreuve
                        WHERE epreuve_id = $1 AND utilisateur_id = $2)",
    )
    .bind(epreuve_id)
    .bind(utilisateur_id)
    .fetch_one(conn)
    .await?;
    Ok(!deja)
}

/// Correction d'une réponse déjà enregistrée, ou `None`. Sert l'idempotence :
/// rejouer un envoi renvoie la même correction sans rien écrire.
async fn correction_existante(
    conn: &mut PgConnection,
    partie: &PartieRow,
    rang: i16,
) -> Result<Option<Correction>, ApiErreur> {
    #[allow(clippy::type_complexity)]
    let ligne: Option<(Uuid, Uuid, Option<i16>, Option<Vec<i16>>, Option<Uuid>, String)> =
        sqlx::query_as(
            "SELECT id, epreuve_id, proposition_choisie, reponse_detail, pays_choisi_id, issue
               FROM jeu.reponse WHERE partie_id = $1 AND rang = $2",
        )
        .bind(partie.id)
        .bind(rang)
        .fetch_optional(&mut *conn)
        .await?;

    let Some((reponse_id, epreuve_id, cle_choisie, detail, pays_id, issue)) = ligne else {
        return Ok(None);
    };

    let score_gagne: i32 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(montant), 0)::int FROM jeu.gain
          WHERE cle_idempotence = $1 AND annule_at IS NULL",
    )
    .bind(format!("reponse:{reponse_id}"))
    .fetch_one(&mut *conn)
    .await?;

    let epreuve = charger_epreuve(&mut *conn, epreuve_id).await?;
    let lien =
        lien_source(&mut *conn, epreuve.type_source.as_deref(), epreuve.source_id).await?;

    let signalable =
        est_signalable(&mut *conn, epreuve_id, partie.utilisateur_id, &issue).await?;
    let solution = solution_de(&mut *conn, &epreuve).await?;
    let jouee = jouee_depuis(&mut *conn, &epreuve, cle_choisie, detail, pays_id).await?;

    Ok(Some(Correction {
        rang,
        epreuve_id,
        signalable,
        issue,
        solution,
        cle_choisie,
        jouee,
        explication: epreuve.explication,
        lien,
        score_gagne,
    }))
}

/// Enregistre la réponse à l'épreuve de rang `rang`, tient les compteurs de
/// l'épreuve et de la partie, et crédite le score s'il y a lieu.
///
/// `issue_forcee` : `Some("injouable")` pour un média qui ne se charge pas.
/// Sans elle, l'issue se déduit de `cle` et du temps écoulé : une réponse
/// arrivée hors délai est enregistrée `sans_reponse` (FR-028).
///
/// Ne termine PAS la partie : c'est à l'appelant de le faire.
async fn inscrire_reponse(
    conn: &mut PgConnection,
    partie: &mut PartieRow,
    regles: &ReglesJeu,
    rang: i16,
    reponse: &ReponseJoueur,
    issue_forcee: Option<&str>,
    maintenant: DateTime<Utc>,
) -> Result<Correction, ApiErreur> {
    let epreuve_id = *partie
        .epreuve_ids
        .get((rang - 1).max(0) as usize)
        .ok_or_else(|| ApiErreur::Validation("Rang hors de la série".into()))?;
    let epreuve = charger_epreuve(&mut *conn, epreuve_id).await?;

    // La forme de la réponse est contrôlée AVANT le temps : une réponse mal
    // formée est refusée, pas comptée fausse.
    let pays_joue = resoudre_pays_joue(&mut *conn, reponse).await?;
    let juste = evaluer(&epreuve, reponse, pays_joue)?;

    let limite = delai_ms(regles, &epreuve);
    let ecoule = partie
        .presentee_at
        .map(|p| (maintenant - p).num_milliseconds())
        .unwrap_or(limite);

    // Ce qui est enregistré n'est la réponse jouée que si elle compte.
    let (issue, retenue): (&str, ReponseJoueur) = match (issue_forcee, juste) {
        (Some(forcee), _) => (forcee, ReponseJoueur::default()),
        (None, Some(j)) if ecoule <= limite + TOLERANCE_RESEAU_MS => {
            (if j { "bonne" } else { "mauvaise" }, reponse.clone())
        }
        _ => ("sans_reponse", ReponseJoueur::default()),
    };
    let cle_retenue = retenue.cle;
    let detail_retenu = retenue.ordre.clone().or_else(|| retenue.paires.clone());
    let pays_retenu = if retenue.pays.is_some() { pays_joue } else { None };
    let temps_ms = ecoule.clamp(0, limite) as i32;

    // `ON CONFLICT DO NOTHING` sans cible : couvre `uq_reponse_rang` (rejeu du
    // même envoi) ET `uq_reponse_libre` (épreuve déjà jouée en partie libre).
    let reponse_id: Option<Uuid> = sqlx::query_scalar(
        "INSERT INTO jeu.reponse
            (partie_id, utilisateur_id, epreuve_id, cadre, rang, proposition_choisie, issue, temps_ms,
             reponse_detail, pays_choisi_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         ON CONFLICT DO NOTHING
         RETURNING id",
    )
    .bind(partie.id)
    .bind(partie.utilisateur_id)
    .bind(epreuve.id)
    .bind(&partie.cadre)
    .bind(rang)
    .bind(cle_retenue)
    .bind(issue)
    .bind(temps_ms)
    .bind(&detail_retenu)
    .bind(pays_retenu)
    .fetch_optional(&mut *conn)
    .await?;

    let est_bonne = issue == "bonne";
    let mut score_gagne = 0;

    if let Some(reponse_id) = reponse_id {
        // Une épreuve injouable n'a pas été réellement présentée : elle n'entre
        // pas dans le taux de réussite.
        if issue != "injouable" {
            sqlx::query(
                "UPDATE jeu.epreuve
                    SET nombre_servie = nombre_servie + 1,
                        nombre_bonnes = nombre_bonnes + $2
                  WHERE id = $1",
            )
            .bind(epreuve.id)
            .bind(i32::from(est_bonne))
            .execute(&mut *conn)
            .await?;
        }

        // Seuls la partie libre et le défi rapportent à la réponse. En duel,
        // c'est le résultat du duel qui rapporte ; en entraînement, rien.
        if est_bonne && matches!(partie.cadre.as_str(), "libre" | "defi") {
            let montant = regles.score_pour(epreuve.difficulte);
            let origine = if partie.cadre == "defi" { "defi" } else { "partie" };
            if crediter(
                &mut *conn,
                partie.utilisateur_id,
                montant,
                origine,
                reponse_id,
                Some(&epreuve.module_code),
                &format!("reponse:{reponse_id}"),
            )
            .await?
            {
                score_gagne = montant;
            }
        }

        partie.bonnes += i16::from(est_bonne);
        partie.score_gagne += score_gagne;
        partie.temps_total_ms += temps_ms;
    }

    partie.presentee_at = None;
    sqlx::query(
        "UPDATE jeu.partie
            SET bonnes = $2, score_gagne = $3, temps_total_ms = $4, presentee_at = NULL
          WHERE id = $1",
    )
    .bind(partie.id)
    .bind(partie.bonnes)
    .bind(partie.score_gagne)
    .bind(partie.temps_total_ms)
    .execute(&mut *conn)
    .await?;

    let lien =
        lien_source(&mut *conn, epreuve.type_source.as_deref(), epreuve.source_id).await?;

    let signalable =
        est_signalable(&mut *conn, epreuve.id, partie.utilisateur_id, issue).await?;
    let solution = solution_de(&mut *conn, &epreuve).await?;
    let jouee = jouee_depuis(&mut *conn, &epreuve, cle_retenue, detail_retenu, pays_retenu).await?;

    Ok(Correction {
        rang,
        epreuve_id: epreuve.id,
        issue: issue.to_string(),
        solution,
        cle_choisie: cle_retenue,
        jouee,
        explication: epreuve.explication,
        lien,
        score_gagne,
        signalable,
    })
}

/// Passe la partie à `terminee`. Renvoie `true` si c'est cet appel qui l'a
/// terminée : c'est le signal pour les effets d'après COMMIT.
///
/// Pour un défi, c'est ici que tombent la prime d'achèvement et, pour le défi
/// du jour, la série de jours. Dans la MÊME transaction : une partie terminée
/// sans sa prime n'existe jamais.
async fn terminer(
    conn: &mut PgConnection,
    partie: &mut PartieRow,
    regles: &ReglesJeu,
) -> Result<bool, sqlx::Error> {
    let touchees = sqlx::query(
        "UPDATE jeu.partie SET etat = 'terminee', terminee_at = NOW(), presentee_at = NULL
          WHERE id = $1 AND etat = 'en_cours'",
    )
    .bind(partie.id)
    .execute(&mut *conn)
    .await?
    .rows_affected();
    partie.etat = "terminee".to_string();
    if touchees != 1 {
        return Ok(false);
    }

    if let Some(defi_id) = partie.defi_id {
        cloturer_defi(&mut *conn, partie, defi_id, regles).await?;
    }
    Ok(true)
}

/// Prime d'achèvement et série de jours d'une partie de défi qui se termine.
async fn cloturer_defi(
    conn: &mut PgConnection,
    partie: &mut PartieRow,
    defi_id: Uuid,
    regles: &ReglesJeu,
) -> Result<(), sqlx::Error> {
    let (periodicite, periode_debut): (String, chrono::NaiveDate) =
        sqlx::query_as("SELECT periodicite, periode_debut FROM jeu.defi WHERE id = $1")
            .bind(defi_id)
            .fetch_one(&mut *conn)
            .await?;

    let prime = i32::from(if periodicite == "semaine" {
        regles.prime_defi_semaine
    } else {
        regles.prime_defi_jour
    });
    if crediter(
        &mut *conn,
        partie.utilisateur_id,
        prime,
        "defi",
        defi_id,
        partie.module_code.as_deref(),
        &format!("defi:{defi_id}:{}", partie.utilisateur_id),
    )
    .await?
    {
        partie.score_gagne += prime;
        sqlx::query("UPDATE jeu.partie SET score_gagne = $2 WHERE id = $1")
            .bind(partie.id)
            .bind(partie.score_gagne)
            .execute(&mut *conn)
            .await?;
    }

    // La série se date du JOUR DU DÉFI, pas de l'instant où il est terminé : un
    // défi commencé avant minuit et fini après compte pour le jour où il a été
    // commencé. Terminer un défi plus ancien que le dernier compté ne change
    // rien.
    if periodicite == "jour" {
        sqlx::query(
            "INSERT INTO jeu.joueur (utilisateur_id, serie_jours, serie_dernier_jour)
             VALUES ($1, 1, $2)
             ON CONFLICT (utilisateur_id) DO UPDATE SET
                 serie_jours = CASE
                     WHEN jeu.joueur.serie_dernier_jour >= $2 THEN jeu.joueur.serie_jours
                     WHEN jeu.joueur.serie_dernier_jour = $2 - 1 THEN jeu.joueur.serie_jours + 1
                     ELSE 1 END,
                 serie_dernier_jour = GREATEST(COALESCE(jeu.joueur.serie_dernier_jour, $2), $2),
                 updated_at = NOW()",
        )
        .bind(partie.utilisateur_id)
        .bind(periode_debut)
        .execute(&mut *conn)
        .await?;
    }
    Ok(())
}

/// Présente l'épreuve suivante.
///
/// Si l'épreuve courante est restée sans réponse (temps écoulé, abandon,
/// rechargement), son absence de réponse est ENREGISTRÉE avant d'avancer :
/// recharger la page ne permet pas de revoir une question. Sa correction est
/// jointe en `precedente`.
///
/// Renvoie aussi `true` si la partie vient d'être terminée par cet appel.
pub async fn presenter_suivante(
    conn: &mut PgConnection,
    partie: &mut PartieRow,
    regles: &ReglesJeu,
) -> Result<(Presentation, bool), ApiErreur> {
    let maintenant = Utc::now();
    let sur = partie.epreuve_ids.len();
    let fin = |partie: &PartieRow, precedente| Presentation {
        terminee: true,
        rang: partie.rang_courant,
        sur,
        expire_a: None,
        maintenant,
        epreuve: None,
        precedente,
    };

    if partie.etat != "en_cours" {
        return Ok((fin(partie, None), false));
    }

    let mut precedente = None;
    if partie.presentee_at.is_some() {
        precedente = Some(
            inscrire_reponse(&mut *conn, partie, regles, partie.rang_courant,
                             &ReponseJoueur::default(), None, maintenant)
                .await?,
        );
    }

    if partie.rang_courant as usize >= sur {
        let vient_de_finir = terminer(&mut *conn, partie, regles).await?;
        return Ok((fin(partie, precedente), vient_de_finir));
    }

    partie.rang_courant += 1;
    partie.presentee_at = Some(maintenant);
    sqlx::query("UPDATE jeu.partie SET rang_courant = $2, presentee_at = $3 WHERE id = $1")
        .bind(partie.id)
        .bind(partie.rang_courant)
        .bind(maintenant)
        .execute(&mut *conn)
        .await?;

    let epreuve =
        charger_epreuve(&mut *conn, partie.epreuve_ids[partie.rang_courant as usize - 1]).await?;
    let expire_a = maintenant + Duration::milliseconds(delai_ms(regles, &epreuve));

    Ok((
        Presentation {
            terminee: false,
            rang: partie.rang_courant,
            sur,
            expire_a: Some(expire_a),
            maintenant,
            epreuve: Some(servir(&epreuve)),
            precedente,
        },
        false,
    ))
}

/// Enregistre la réponse du membre à l'épreuve en cours.
///
/// Rejouer le même envoi renvoie la correction déjà enregistrée, sans rien
/// écrire (FR-025). Renvoie aussi `true` si la partie vient d'être terminée.
pub async fn repondre(
    conn: &mut PgConnection,
    partie: &mut PartieRow,
    regles: &ReglesJeu,
    rang: i16,
    reponse: &ReponseJoueur,
    issue_forcee: Option<&str>,
) -> Result<(Correction, bool), ApiErreur> {
    if let Some(deja) = correction_existante(&mut *conn, partie, rang).await? {
        return Ok((deja, false));
    }
    if partie.etat != "en_cours" {
        return Err(ApiErreur::Conflit("Cette partie est terminée".into()));
    }
    if rang != partie.rang_courant || partie.presentee_at.is_none() {
        return Err(ApiErreur::Conflit(
            "Cette épreuve n'est pas l'épreuve en cours".into(),
        ));
    }

    if issue_forcee == Some("injouable") {
        let epreuve =
            charger_epreuve(&mut *conn, partie.epreuve_ids[rang as usize - 1]).await?;
        if epreuve.media_type.is_none() {
            return Err(ApiErreur::Conflit(
                "Cette épreuve n'a pas de média : elle ne peut pas être déclarée injouable".into(),
            ));
        }
    }

    let correction =
        inscrire_reponse(&mut *conn, partie, regles, rang, reponse, issue_forcee, Utc::now()).await?;

    let vient_de_finir = if rang as usize == partie.epreuve_ids.len() {
        terminer(&mut *conn, partie, regles).await?
    } else {
        false
    };

    Ok((correction, vient_de_finir))
}

/// Clôt une partie en l'état. L'épreuve affichée au moment de la clôture est
/// comptée sans réponse : sinon, abandonner serait un moyen de voir une
/// question sans la jouer. Les épreuves non présentées restent neuves.
pub async fn clore(
    conn: &mut PgConnection,
    partie: &mut PartieRow,
    regles: &ReglesJeu,
) -> Result<(), ApiErreur> {
    if partie.etat != "en_cours" {
        return Ok(());
    }
    if partie.presentee_at.is_some() {
        inscrire_reponse(&mut *conn, partie, regles, partie.rang_courant,
                         &ReponseJoueur::default(), None, Utc::now())
            .await?;
    }
    sqlx::query(
        "UPDATE jeu.partie SET etat = 'close', terminee_at = NOW(), presentee_at = NULL
          WHERE id = $1 AND etat = 'en_cours'",
    )
    .bind(partie.id)
    .execute(conn)
    .await?;
    partie.etat = "close".to_string();
    Ok(())
}

/// Effets d'une fin de partie, à appeler APRÈS le COMMIT (le moteur
/// d'engagement prend le pool, pas la transaction, et ne doit jamais faire
/// échouer une partie). Le jalon « première partie » est idempotent par sa clé :
/// il ne s'écrit qu'une fois par membre.
pub async fn apres_fin_de_partie(pool: &PgPool, partie: &PartieRow) {
    engagement::attribuer(
        pool,
        partie.utilisateur_id,
        "jeu_premiere_partie",
        Some("partie"),
        Some(partie.id),
        &format!("jeu:premiere:{}", partie.utilisateur_id),
    )
    .await;

    let Some(defi_id) = partie.defi_id else {
        return;
    };

    engagement::attribuer(
        pool,
        partie.utilisateur_id,
        "jeu_defi_termine",
        Some("defi"),
        Some(defi_id),
        &format!("jeu:defi:{defi_id}:{}", partie.utilisateur_id),
    )
    .await;

    // Jalon « sept jours de suite » : à chaque multiple de sept. La clé porte le
    // jour atteint, donc le jalon ne s'écrit qu'une fois par palier.
    let serie: Option<(i16, Option<chrono::NaiveDate>)> = sqlx::query_as(
        "SELECT j.serie_jours, j.serie_dernier_jour
           FROM jeu.joueur j
           JOIN jeu.defi d ON d.id = $2 AND d.periodicite = 'jour'
                          AND d.periode_debut = j.serie_dernier_jour
          WHERE j.utilisateur_id = $1",
    )
    .bind(partie.utilisateur_id)
    .bind(defi_id)
    .fetch_optional(pool)
    .await
    .unwrap_or(None);

    if let Some((jours, Some(jour))) = serie {
        if jours > 0 && jours % 7 == 0 {
            engagement::attribuer(
                pool,
                partie.utilisateur_id,
                "jeu_serie_7_jours",
                Some("defi"),
                Some(defi_id),
                &format!("jeu:serie:{}:{jour}", partie.utilisateur_id),
            )
            .await;
        }
    }
}

// ─── Défis ───────────────────────────────────────────────────────────────────

/// Fenêtre pendant laquelle une épreuve déjà servie dans un défi n'y revient pas.
const DEFI_SANS_REDITE_JOURS: i32 = 30;

/// Début de la période courante d'une périodicité, en TEMPS UNIVERSEL : un jour
/// par fuseau donnerait à certains membres la série avant les autres.
pub async fn periode_courante(
    pool: &PgPool,
    periodicite: &str,
) -> Result<chrono::NaiveDate, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT CASE WHEN $1 = 'semaine'
                     THEN date_trunc('week', NOW() AT TIME ZONE 'UTC')::date
                     ELSE (NOW() AT TIME ZONE 'UTC')::date END",
    )
    .bind(periodicite)
    .fetch_one(pool)
    .await
}

/// Le défi de la période courante, créé PARESSEUSEMENT.
///
/// Personne n'a à le préparer : s'il n'existe pas (ni programmé par un
/// administrateur, ni déjà composé), le premier lecteur le compose. L'insertion
/// est en `ON CONFLICT DO NOTHING` sur `(periodicite, periode_debut)` puis on
/// RELIT : deux lecteurs simultanés obtiennent la même ligne, celle du premier
/// arrivé. `None` si le vivier ne permet pas de composer la série.
pub async fn defi_courant(
    pool: &PgPool,
    regles: &ReglesJeu,
    periodicite: &str,
) -> Result<Option<DefiRow>, sqlx::Error> {
    let periode = periode_courante(pool, periodicite).await?;
    let requete = format!(
        "SELECT {DEFI_COLONNES} FROM jeu.defi WHERE periodicite = $1 AND periode_debut = $2"
    );
    let lire = || {
        sqlx::query_as::<_, DefiRow>(&requete)
            .bind(periodicite)
            .bind(periode)
            .fetch_optional(pool)
    };

    if let Some(defi) = lire().await? {
        return Ok(Some(defi));
    }

    let taille = i64::from(if periodicite == "semaine" {
        regles.taille_defi_semaine
    } else {
        regles.taille_defi_jour
    });

    // D'abord sans redite ; si le vivier est trop petit pour cela, on accepte
    // les redites plutôt que de laisser la période sans défi.
    let mut serie: Vec<Uuid> = Vec::new();
    for sans_redite in [true, false] {
        let tirage = serie_variee_sql(
            "AND (NOT $1 OR NOT EXISTS (
                     SELECT 1 FROM jeu.defi d
                      WHERE d.periode_debut > CURRENT_DATE - $2::int
                        AND e.id = ANY(d.epreuve_ids)))",
            "FALSE",
        );
        serie = sqlx::query_scalar(&format!("{tirage} LIMIT $3"))
        .bind(sans_redite)
        .bind(DEFI_SANS_REDITE_JOURS)
        .bind(taille)
        .fetch_all(pool)
        .await?;
        if serie.len() as i64 == taille {
            break;
        }
    }
    if (serie.len() as i64) < taille {
        return Ok(None);
    }

    sqlx::query(
        "INSERT INTO jeu.defi (periodicite, periode_debut, epreuve_ids, origine)
         VALUES ($1, $2, $3, 'automatique')
         ON CONFLICT (periodicite, periode_debut) DO NOTHING",
    )
    .bind(periodicite)
    .bind(periode)
    .bind(&serie)
    .execute(pool)
    .await?;

    lire().await
}

// ─── Duels ───────────────────────────────────────────────────────────────────

/// Ce qu'une résolution de duel a changé, à produire APRÈS le COMMIT
/// (notifications, signaux, réputation) : rien de tout cela ne doit pouvoir
/// faire échouer la transaction du duel.
#[derive(Debug, Clone)]
pub enum EffetDuel {
    /// Le duel vient de se terminer ; `vainqueur` est `None` pour un nul.
    Termine { duel: DuelRow },
    /// Proposition restée sans réponse, ou aucun des deux n'a joué.
    Expire { duel: DuelRow },
    /// Amitié rompue, blocage, compte suspendu.
    Annule { duel: DuelRow },
    /// Mode direct : une manche s'ouvre (signal aux deux écrans).
    Manche { duel: DuelRow },
}

/// Les parties des deux joueurs d'un duel.
pub async fn parties_du_duel(
    conn: &mut PgConnection,
    duel_id: Uuid,
) -> Result<Vec<PartieDuel>, sqlx::Error> {
    sqlx::query_as::<_, PartieDuel>(
        "SELECT id, utilisateur_id, etat, bonnes, temps_total_ms
           FROM jeu.partie WHERE duel_id = $1",
    )
    .bind(duel_id)
    .fetch_all(conn)
    .await
}

/// Les deux membres peuvent-ils encore s'affronter ? Amis, sans blocage, et
/// deux comptes actifs.
async fn relation_valide(conn: &mut PgConnection, a: Uuid, b: Uuid) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM social.amitie
                        WHERE utilisateur_a_id = LEAST($1, $2) AND utilisateur_b_id = GREATEST($1, $2))
            AND NOT EXISTS(SELECT 1 FROM social.blocage
                            WHERE (bloqueur_id = $1 AND bloque_id = $2)
                               OR (bloqueur_id = $2 AND bloque_id = $1))
            AND (SELECT COUNT(*) FROM iam.utilisateur
                  WHERE id IN ($1, $2) AND etat = 'actif' AND deleted_at IS NULL) = 2",
    )
    .bind(a)
    .bind(b)
    .fetch_one(conn)
    .await
}

/// Passe le duel d'un état à un autre, seulement s'il est encore dans l'état
/// attendu : deux résolutions concurrentes n'appliquent pas deux fois la même
/// transition.
async fn transition(
    conn: &mut PgConnection,
    duel: &mut DuelRow,
    attendu: &str,
    nouvel_etat: &str,
    issue: Option<&str>,
    vainqueur_id: Option<Uuid>,
) -> Result<bool, sqlx::Error> {
    let touchees = sqlx::query(
        "UPDATE jeu.duel
            SET etat = $3, issue = $4, vainqueur_id = $5,
                termine_at = CASE WHEN $3 IN ('termine', 'annule', 'expire', 'refuse')
                                  THEN NOW() ELSE termine_at END
          WHERE id = $1 AND etat = $2",
    )
    .bind(duel.id)
    .bind(attendu)
    .bind(nouvel_etat)
    .bind(issue)
    .bind(vainqueur_id)
    .execute(conn)
    .await?
    .rows_affected();
    if touchees == 1 {
        duel.etat = nouvel_etat.to_string();
        duel.issue = issue.map(str::to_string);
        duel.vainqueur_id = vainqueur_id;
        duel.termine_at = Some(Utc::now());
    }
    Ok(touchees == 1)
}

/// Résout un duel : applique, d'après l'instant présent et les parties jouées,
/// la transition qui lui revient, et verse les primes s'il se termine.
///
/// Appelée en tête de TOUTE lecture ou écriture d'un duel, sous verrou : c'est
/// la seule façon dont un duel avance, puisque rien ne tourne en tâche de fond.
/// L'oublier sur une route ne casse rien de visible : le duel resterait
/// simplement dans son dernier état, et une prime de forfait ne serait jamais
/// versée.
///
/// Mode différé seulement ; le mode direct a ses propres branches.
pub async fn resoudre_duel(
    conn: &mut PgConnection,
    duel_id: Uuid,
    regles: &ReglesJeu,
) -> Result<(DuelRow, Vec<EffetDuel>), ApiErreur> {
    let mut duel = sqlx::query_as::<_, DuelRow>(&format!(
        "SELECT {DUEL_COLONNES} FROM jeu.duel WHERE id = $1 FOR UPDATE"
    ))
    .bind(duel_id)
    .fetch_one(&mut *conn)
    .await?;
    let mut effets = Vec::new();

    if duel.est_termine() {
        return Ok((duel, effets));
    }

    // Amitié rompue, blocage, compte suspendu : le duel tombe, sans gain.
    if !relation_valide(&mut *conn, duel.proposant_id, duel.adversaire_id).await? {
        let attendu = duel.etat.clone();
        if transition(&mut *conn, &mut duel, &attendu, "annule", None, None).await? {
            effets.push(EffetDuel::Annule { duel: duel.clone() });
        }
        return Ok((duel, effets));
    }

    let echu = Utc::now() >= duel.echeance_at;

    if duel.etat == "propose" {
        if echu && transition(&mut *conn, &mut duel, "propose", "expire", None, None).await? {
            effets.push(EffetDuel::Expire { duel: duel.clone() });
        }
        return Ok((duel, effets));
    }

    if duel.mode == "direct" {
        resoudre_direct(&mut *conn, &mut duel, regles, echu, &mut effets).await?;
        return Ok((duel, effets));
    }

    // Différé, accepte / en_cours : on regarde ce que chacun a joué.
    let mut parties = parties_verrouillees(&mut *conn, duel.id).await?;

    let deux_finies = parties.len() == 2 && parties.iter().all(|p| p.etat != "en_cours");
    if !deux_finies && !echu {
        return Ok((duel, effets));
    }

    // À l'échéance, une partie commencée et pas finie est close en l'état : ce
    // qui a été répondu compte, l'épreuve affichée est comptée sans réponse.
    if echu {
        for partie in parties.iter_mut().filter(|p| p.etat == "en_cours") {
            clore(&mut *conn, partie, regles).await?;
        }
    }

    if let Some(effet) = conclure(&mut *conn, &mut duel, &parties, regles).await? {
        effets.push(effet);
    }
    Ok((duel, effets))
}

async fn parties_verrouillees(
    conn: &mut PgConnection,
    duel_id: Uuid,
) -> Result<Vec<PartieRow>, sqlx::Error> {
    sqlx::query_as::<_, PartieRow>(&format!(
        "SELECT {PARTIE_COLONNES} FROM jeu.partie WHERE duel_id = $1 ORDER BY created_at FOR UPDATE"
    ))
    .bind(duel_id)
    .fetch_all(conn)
    .await
}

/// Termine le duel d'après les parties jouées : aucune (expiré), une seule
/// (forfait de l'autre), deux (plus de bonnes réponses, puis moins de temps,
/// sinon nul). Verse les primes si le duel est compté.
async fn conclure(
    conn: &mut PgConnection,
    duel: &mut DuelRow,
    parties: &[PartieRow],
    regles: &ReglesJeu,
) -> Result<Option<EffetDuel>, ApiErreur> {
    let attendu = duel.etat.clone();
    let (issue, vainqueur) = match parties {
        [] => {
            return Ok(transition(&mut *conn, duel, &attendu, "expire", None, None)
                .await?
                .then(|| EffetDuel::Expire { duel: duel.clone() }));
        }
        [seule] => ("forfait", Some(seule.utilisateur_id)),
        [a, b, ..] => {
            // `Greater` : a fait mieux que b.
            let a_face_a_b =
                a.bonnes.cmp(&b.bonnes).then(b.temps_total_ms.cmp(&a.temps_total_ms));
            match a_face_a_b {
                std::cmp::Ordering::Greater => ("victoire", Some(a.utilisateur_id)),
                std::cmp::Ordering::Less => ("victoire", Some(b.utilisateur_id)),
                std::cmp::Ordering::Equal => ("nul", None),
            }
        }
    };
    terminer_duel(conn, duel, &attendu, issue, vainqueur, regles).await
}

/// Transition vers `termine` et primes. `sans_issue` ne rapporte rien.
async fn terminer_duel(
    conn: &mut PgConnection,
    duel: &mut DuelRow,
    attendu: &str,
    issue: &str,
    vainqueur: Option<Uuid>,
    regles: &ReglesJeu,
) -> Result<Option<EffetDuel>, ApiErreur> {
    if !transition(&mut *conn, duel, attendu, "termine", Some(issue), vainqueur).await? {
        return Ok(None);
    }

    if duel.compte && issue != "sans_issue" {
        let gagnants: Vec<(Uuid, i16)> = match vainqueur {
            Some(v) => vec![(v, regles.prime_duel_victoire)],
            None => vec![
                (duel.proposant_id, regles.prime_duel_nul),
                (duel.adversaire_id, regles.prime_duel_nul),
            ],
        };
        for (utilisateur_id, prime) in gagnants {
            crediter(
                &mut *conn,
                utilisateur_id,
                i32::from(prime),
                "duel",
                duel.id,
                Some(&duel.module_code),
                &format!("duel:{}:{utilisateur_id}", duel.id),
            )
            .await?;
        }
    }

    Ok(Some(EffetDuel::Termine { duel: duel.clone() }))
}

/// Délai avant la première manche d'un duel direct : le temps que le second
/// écran reçoive le signal et relise l'état. Les deux voient le même compte à
/// rebours, calé sur l'horloge du serveur.
const DEPART_DIRECT_MS: i64 = 3_000;

/// Fin de la question d'une manche et instant de sa clôture, s'il est atteint.
pub struct EtatManche {
    pub rang: i16,
    pub epreuve: EpreuveRow,
    pub debut: DateTime<Utc>,
    pub fin_question: DateTime<Utc>,
    /// `Some` quand la manche est close : les deux ont répondu (instant de la
    /// seconde réponse) ou le temps est écoulé (fin de la question).
    pub cloture: Option<DateTime<Utc>>,
    /// Réponse de chaque joueur à cette manche.
    pub reponses: Vec<ReponseManche>,
}

pub struct ReponseManche {
    pub utilisateur_id: Uuid,
    pub jouee: ReponseJoueur,
    pub issue: String,
}

/// Lit l'état de la manche en cours d'un duel direct.
pub async fn etat_manche(
    conn: &mut PgConnection,
    duel: &DuelRow,
    regles: &ReglesJeu,
) -> Result<Option<EtatManche>, ApiErreur> {
    let (Some(debut), Some(serie)) = (duel.manche_debut_at, duel.epreuve_ids.as_ref()) else {
        return Ok(None);
    };
    let rang = duel.rang_courant;
    let Some(epreuve_id) = serie.get((rang - 1).max(0) as usize).copied() else {
        return Ok(None);
    };
    let epreuve = charger_epreuve(&mut *conn, epreuve_id).await?;
    let fin_question = debut + Duration::milliseconds(delai_ms(regles, &epreuve));

    #[allow(clippy::type_complexity)]
    let lignes: Vec<(Uuid, Option<i16>, Option<Vec<i16>>, Option<Uuid>, String, DateTime<Utc>)> =
        sqlx::query_as(
        "SELECT r.utilisateur_id, r.proposition_choisie, r.reponse_detail, r.pays_choisi_id,
                r.issue, r.created_at
           FROM jeu.reponse r
           JOIN jeu.partie p ON p.id = r.partie_id
          WHERE p.duel_id = $1 AND r.rang = $2",
    )
    .bind(duel.id)
    .bind(rang)
    .fetch_all(&mut *conn)
    .await?;

    let maintenant = Utc::now();
    // Une réponse « sans réponse » est écrite à la clôture, pas par le joueur :
    // seules les vraies réponses ferment la manche avant son temps.
    let vraies: Vec<DateTime<Utc>> =
        lignes.iter().filter(|l| l.4 != "sans_reponse").map(|l| l.5).collect();
    let cloture = if vraies.len() >= 2 {
        vraies.iter().max().copied().map(|t| t.min(fin_question))
    } else if maintenant >= fin_question {
        Some(fin_question)
    } else {
        None
    };

    let mut reponses = Vec::with_capacity(lignes.len());
    for (utilisateur_id, cle, detail, pays_id, issue, _) in lignes {
        let jouee = jouee_depuis(&mut *conn, &epreuve, cle, detail, pays_id).await?;
        reponses.push(ReponseManche { utilisateur_id, jouee, issue });
    }

    Ok(Some(EtatManche { rang, epreuve, debut, fin_question, cloture, reponses }))
}

/// Avancement d'un duel DIRECT, résolu à la lecture comme tout le reste.
///
/// Rien ne tourne entre deux lectures : chaque appel rattrape ce qui aurait dû
/// se passer depuis la précédente (manches closes, manches ouvertes, absences),
/// d'où la boucle. Les instants sont DÉDUITS (début de manche, fin de question,
/// clôture, révélation) : deux lecteurs simultanés calculent les mêmes.
async fn resoudre_direct(
    conn: &mut PgConnection,
    duel: &mut DuelRow,
    regles: &ReglesJeu,
    echu: bool,
    effets: &mut Vec<EffetDuel>,
) -> Result<(), ApiErreur> {
    let maintenant = Utc::now();
    let grace = Duration::seconds(i64::from(regles.grace_direct_s));
    let present = |instant: Option<DateTime<Utc>>| instant.is_some_and(|t| maintenant - t <= grace);
    let pause = Duration::seconds(i64::from(regles.pause_revelation_s));

    if duel.etat == "accepte" {
        let deux_presents =
            present(duel.presence_proposant_at) && present(duel.presence_adversaire_at);
        if deux_presents {
            // Départ : les deux parties sont créées, la première manche s'ouvre
            // dans quelques secondes, au même instant pour les deux.
            let serie = duel.epreuve_ids.clone().unwrap_or_default();
            let debut = maintenant + Duration::milliseconds(DEPART_DIRECT_MS);
            for joueur in [duel.proposant_id, duel.adversaire_id] {
                sqlx::query(
                    "INSERT INTO jeu.joueur (utilisateur_id) VALUES ($1)
                     ON CONFLICT (utilisateur_id) DO NOTHING",
                )
                .bind(joueur)
                .execute(&mut *conn)
                .await?;
                sqlx::query(
                    "INSERT INTO jeu.partie
                        (utilisateur_id, module_code, cadre, duel_id, epreuve_ids, rang_courant, presentee_at)
                     VALUES ($1, $2, 'duel', $3, $4, 1, $5)
                     ON CONFLICT (duel_id, utilisateur_id) WHERE duel_id IS NOT NULL DO NOTHING",
                )
                .bind(joueur)
                .bind(&duel.module_code)
                .bind(duel.id)
                .bind(&serie)
                .bind(debut)
                .execute(&mut *conn)
                .await?;
            }
            let ouvert = sqlx::query(
                "UPDATE jeu.duel SET etat = 'en_cours', rang_courant = 1, manche_debut_at = $2
                  WHERE id = $1 AND etat = 'accepte'",
            )
            .bind(duel.id)
            .bind(debut)
            .execute(&mut *conn)
            .await?
            .rows_affected()
                == 1;
            if ouvert {
                duel.etat = "en_cours".into();
                duel.rang_courant = 1;
                duel.manche_debut_at = Some(debut);
                effets.push(EffetDuel::Manche { duel: duel.clone() });
            }
        } else if echu {
            // L'un des deux n'est jamais venu : le duel tombe, sans vainqueur.
            if transition(&mut *conn, duel, "accepte", "expire", None, None).await? {
                effets.push(EffetDuel::Expire { duel: duel.clone() });
            }
        }
        return Ok(());
    }

    if duel.etat != "en_cours" {
        return Ok(());
    }

    loop {
        // Absences : au-delà du délai de grâce, l'absent perd par forfait ; si
        // les deux sont partis, le duel s'arrête sans vainqueur.
        let proposant_la = present(duel.presence_proposant_at);
        let adversaire_la = present(duel.presence_adversaire_at);
        if !proposant_la || !adversaire_la {
            let mut parties = parties_verrouillees(&mut *conn, duel.id).await?;
            for partie in parties.iter_mut().filter(|p| p.etat == "en_cours") {
                clore(&mut *conn, partie, regles).await?;
            }
            let (issue, vainqueur) = match (proposant_la, adversaire_la) {
                (false, false) => ("sans_issue", None),
                (true, _) => ("forfait", Some(duel.proposant_id)),
                (_, true) => ("forfait", Some(duel.adversaire_id)),
            };
            if let Some(effet) =
                terminer_duel(&mut *conn, duel, "en_cours", issue, vainqueur, regles).await?
            {
                effets.push(effet);
            }
            return Ok(());
        }

        let Some(manche) = etat_manche(&mut *conn, duel, regles).await? else {
            return Ok(());
        };
        if maintenant < manche.debut {
            return Ok(()); // compte à rebours
        }
        let Some(cloture) = manche.cloture else {
            return Ok(()); // question ouverte
        };

        // Manche close : celui qui n'a pas répondu est compté sans réponse.
        let mut parties = parties_verrouillees(&mut *conn, duel.id).await?;
        for partie in parties.iter_mut() {
            let a_repondu =
                manche.reponses.iter().any(|r| r.utilisateur_id == partie.utilisateur_id);
            if !a_repondu && partie.etat == "en_cours" && partie.rang_courant == manche.rang {
                repondre(&mut *conn, partie, regles, manche.rang, &ReponseJoueur::default(), None)
                    .await?;
            }
        }

        if maintenant < cloture + pause {
            return Ok(()); // révélation : la correction est montrée aux deux
        }

        let total = duel.epreuve_ids.as_ref().map_or(0, Vec::len);
        if manche.rang as usize >= total {
            let parties = parties_verrouillees(&mut *conn, duel.id).await?;
            if let Some(effet) = conclure(&mut *conn, duel, &parties, regles).await? {
                effets.push(effet);
            }
            return Ok(());
        }

        // Manche suivante : elle commence à la fin de la révélation, instant
        // déduit et donc identique pour les deux écrans.
        let suivante = manche.rang + 1;
        let debut = cloture + pause;
        sqlx::query("UPDATE jeu.duel SET rang_courant = $2, manche_debut_at = $3 WHERE id = $1")
            .bind(duel.id)
            .bind(suivante)
            .bind(debut)
            .execute(&mut *conn)
            .await?;
        sqlx::query(
            "UPDATE jeu.partie SET rang_courant = $2, presentee_at = $3
              WHERE duel_id = $1 AND etat = 'en_cours'",
        )
        .bind(duel.id)
        .bind(suivante)
        .bind(debut)
        .execute(&mut *conn)
        .await?;
        duel.rang_courant = suivante;
        duel.manche_debut_at = Some(debut);
        effets.push(EffetDuel::Manche { duel: duel.clone() });
    }
}

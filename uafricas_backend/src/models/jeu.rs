//! Modèles du jeu (schéma `jeu`, feature 013).
//!
//! Règle de sérialisation qui porte la sécurité : [`EpreuveServie`] et
//! [`Correction`] sont deux types DISTINCTS. La bonne réponse n'existe dans
//! aucun type renvoyé avant la réponse du membre, ce n'est pas un champ qu'on
//! pense à omettre, c'est un champ qui n'existe pas (FR-027).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ─── Règles ──────────────────────────────────────────────────────────────────

/// Le singleton `jeu.regles`. Tout est en `SMALLINT` côté SQL, donc en `i16`.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReglesJeu {
    pub taille_partie: i16,
    pub taille_defi_jour: i16,
    pub taille_defi_semaine: i16,
    pub taille_duel: i16,
    pub temps_epreuve_s: i16,
    pub score_facile: i16,
    pub score_moyen: i16,
    pub score_difficile: i16,
    pub prime_defi_jour: i16,
    pub prime_defi_semaine: i16,
    pub prime_duel_victoire: i16,
    pub prime_duel_nul: i16,
    pub delai_duel_h: i16,
    pub delai_direct_min: i16,
    pub grace_direct_s: i16,
    pub pause_revelation_s: i16,
    pub duels_comptes_par_paire_jour: i16,
    pub duels_comptes_par_membre_jour: i16,
    pub joueurs_par_pays: i16,
}

pub const REGLES_COLONNES: &str = "taille_partie, taille_defi_jour, taille_defi_semaine, taille_duel, \
     temps_epreuve_s, score_facile, score_moyen, score_difficile, prime_defi_jour, \
     prime_defi_semaine, prime_duel_victoire, prime_duel_nul, delai_duel_h, delai_direct_min, \
     grace_direct_s, pause_revelation_s, duels_comptes_par_paire_jour, \
     duels_comptes_par_membre_jour, joueurs_par_pays";

impl ReglesJeu {
    /// Score d'une bonne réponse selon le niveau de difficulté (1 à 3).
    pub fn score_pour(&self, difficulte: i16) -> i32 {
        i32::from(match difficulte {
            3 => self.score_difficile,
            2 => self.score_moyen,
            _ => self.score_facile,
        })
    }
}

// ─── Modules ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct ModuleJeu {
    pub code: String,
    pub libelle: String,
    pub route: String,
    pub icone: Option<String>,
    pub epreuves_jouables: i64,
    /// Calculé contre `regles.taille_partie`, jamais stocké (FR-005).
    #[sqlx(default)]
    pub disponible: bool,
}

// ─── Épreuves ────────────────────────────────────────────────────────────────

/// Ligne complète d'une épreuve. Usage INTERNE : elle porte la bonne réponse et
/// ne doit jamais être sérialisée telle quelle vers un membre.
#[derive(Debug, Clone, FromRow)]
pub struct EpreuveRow {
    pub id: Uuid,
    pub module_code: String,
    pub enonce: String,
    pub media_type: Option<String>,
    pub media_url: Option<String>,
    pub propositions: Vec<String>,
    pub bonne_reponse: i16,
    pub explication: Option<String>,
    pub difficulte: i16,
    pub type_source: Option<String>,
    pub source_id: Option<Uuid>,
}

pub const EPREUVE_COLONNES: &str = "e.id, e.module_code, e.enonce, e.media_type, e.media_url, \
     e.propositions, e.bonne_reponse, e.explication, e.difficulte, e.type_source, e.source_id";

/// Une proposition telle qu'elle est servie : `cle` est son rang d'origine dans
/// l'épreuve. Il ne révèle rien, le client ne sait pas lequel est le bon.
#[derive(Debug, Serialize)]
pub struct PropositionServie {
    pub cle: i16,
    pub texte: String,
}

/// L'épreuve telle qu'un membre la reçoit AVANT de répondre.
#[derive(Debug, Serialize)]
pub struct EpreuveServie {
    pub id: Uuid,
    pub enonce: String,
    pub media_type: Option<String>,
    pub media_url: Option<String>,
    pub difficulte: i16,
    pub propositions: Vec<PropositionServie>,
}

/// Ce qu'un membre reçoit APRÈS sa réponse, ou après l'expiration du temps.
#[derive(Debug, Clone, Serialize)]
pub struct Correction {
    pub rang: i16,
    pub epreuve_id: Uuid,
    /// `bonne`, `mauvaise`, `sans_reponse` ou `injouable`.
    pub issue: String,
    pub bonne_cle: i16,
    pub cle_choisie: Option<i16>,
    pub explication: Option<String>,
    /// Lien vers le contenu dont l'épreuve est tirée, s'il existe (FR-007).
    pub lien: Option<String>,
    pub score_gagne: i32,
    pub signalable: bool,
}

// ─── Parties ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, FromRow)]
pub struct PartieRow {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub module_code: Option<String>,
    pub cadre: String,
    pub defi_id: Option<Uuid>,
    pub duel_id: Option<Uuid>,
    pub epreuve_ids: Vec<Uuid>,
    pub rang_courant: i16,
    pub presentee_at: Option<DateTime<Utc>>,
    pub etat: String,
    pub bonnes: i16,
    pub score_gagne: i32,
    pub temps_total_ms: i32,
    pub created_at: DateTime<Utc>,
}

pub const PARTIE_COLONNES: &str = "id, utilisateur_id, module_code, cadre, defi_id, duel_id, \
     epreuve_ids, rang_courant, presentee_at, etat, bonnes, score_gagne, temps_total_ms, created_at";

#[derive(Debug, Deserialize)]
pub struct CreerPartieRequest {
    pub module: String,
    pub pays_id: Option<Uuid>,
    pub theme: Option<String>,
    /// Doit être demandé explicitement : c'est ce qui garantit que
    /// l'entraînement est annoncé avant de commencer (FR-016).
    pub entrainement: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RepondreRequest {
    pub rang: i16,
    /// `null` : le temps s'est écoulé côté client. L'épreuve est alors
    /// enregistrée sans réponse et sa correction renvoyée, SANS présenter la
    /// suivante : le membre lit la correction avant que l'horloge ne reparte.
    pub cle: Option<i16>,
}

#[derive(Debug, Deserialize)]
pub struct RangRequest {
    pub rang: i16,
}

/// Une ligne du bilan.
#[derive(Debug, Serialize, FromRow)]
pub struct ReponseBilan {
    pub rang: i16,
    pub epreuve_id: Uuid,
    pub issue: String,
}

#[derive(Debug, Serialize)]
pub struct PartieResponse {
    pub id: Uuid,
    pub cadre: String,
    pub module: Option<String>,
    pub defi_id: Option<Uuid>,
    pub duel_id: Option<Uuid>,
    pub nombre_epreuves: usize,
    pub rang_courant: i16,
    pub etat: String,
    pub temps_epreuve_s: i16,
    pub bonnes: i16,
    pub score_gagne: i32,
    /// Renseignés seulement quand la partie est terminée ou close (le bilan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reponses: Option<Vec<ReponseBilan>>,
    /// Rang du membre dans la saison en cours, s'il y en a une et qu'il y a du
    /// score. Bilan seulement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rang_saison: Option<i64>,
}

/// Réponse de `POST /parties/{id}/suivante`.
#[derive(Debug, Serialize)]
pub struct Presentation {
    pub terminee: bool,
    pub rang: i16,
    pub sur: usize,
    /// Instant où le temps imparti s'achève, fixé par le serveur.
    pub expire_a: Option<DateTime<Utc>>,
    /// Heure du serveur : le client y cale son compte à rebours.
    pub maintenant: DateTime<Utc>,
    pub epreuve: Option<EpreuveServie>,
    /// Correction de l'épreuve laissée sans réponse, le cas échéant.
    pub precedente: Option<Correction>,
}

// ─── Signalement d'épreuve ───────────────────────────────────────────────────

pub const MOTIFS_SIGNALEMENT: &[&str] =
    &["reponse_erronee", "enonce_ambigu", "contenu_deplace", "autre"];

#[derive(Debug, Deserialize)]
pub struct SignalerEpreuveRequest {
    pub motif: String,
    pub commentaire: Option<String>,
}

// ─── Défis ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, FromRow)]
pub struct DefiRow {
    pub id: Uuid,
    pub periodicite: String,
    /// Jour UTC, ou lundi UTC de la semaine.
    pub periode_debut: chrono::NaiveDate,
    pub module_code: Option<String>,
    pub titre: Option<String>,
    pub epreuve_ids: Vec<Uuid>,
}

pub const DEFI_COLONNES: &str = "id, periodicite, periode_debut, module_code, titre, epreuve_ids";

impl DefiRow {
    /// Instant où la période s'achève : minuit UTC du lendemain, ou du lundi
    /// suivant.
    pub fn fin_at(&self) -> DateTime<Utc> {
        let jours = if self.periodicite == "semaine" { 7 } else { 1 };
        (self.periode_debut + chrono::Duration::days(jours))
            .and_hms_opt(0, 0, 0)
            .expect("minuit existe")
            .and_utc()
    }
}

/// La partie du membre sur un défi : sa participation.
#[derive(Debug, Serialize, FromRow)]
pub struct MaPartieDefi {
    pub id: Uuid,
    pub etat: String,
    pub bonnes: i16,
    pub score_gagne: i32,
}

#[derive(Debug, Serialize)]
pub struct DefiResponse {
    pub id: Uuid,
    pub periodicite: String,
    pub periode_debut: chrono::NaiveDate,
    pub fin_at: DateTime<Utc>,
    pub nombre_epreuves: usize,
    pub titre: Option<String>,
    pub module: Option<String>,
    /// `None` pour un visiteur, ou pour un membre qui n'a pas encore joué.
    pub ma_partie: Option<MaPartieDefi>,
}

#[derive(Debug, Serialize)]
pub struct DefisCourants {
    /// `None` quand le vivier ne permet pas de composer le défi.
    pub jour: Option<DefiResponse>,
    pub semaine: Option<DefiResponse>,
    /// Série AFFICHÉE : ramenée à 0 dès qu'un jour a été manqué.
    pub serie_jours: i16,
    pub maintenant: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct LigneResultatDefi {
    pub rang: i64,
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub bonnes: i16,
    pub temps_total_ms: i32,
}

#[derive(Debug, Serialize)]
pub struct ResultatsDefi {
    pub defi: DefiResponse,
    pub en_cours: bool,
    pub participants: i64,
    pub podium: Vec<LigneResultatDefi>,
    pub moi: Option<LigneResultatDefi>,
}

// ─── Championship : saisons et classements ───────────────────────────────────

#[derive(Debug, Clone, FromRow)]
pub struct SaisonRow {
    pub id: Uuid,
    pub nom: String,
    pub debut_at: DateTime<Utc>,
    pub fin_at: DateTime<Utc>,
}

pub const SAISON_COLONNES: &str = "id, nom, debut_at, fin_at";

#[derive(Debug, Clone, Serialize)]
pub struct SaisonResponse {
    pub id: Uuid,
    pub nom: String,
    pub debut_at: DateTime<Utc>,
    pub fin_at: DateTime<Utc>,
    /// `a_venir`, `en_cours` ou `close` : DÉDUIT des dates, jamais stocké.
    pub etat: &'static str,
}

impl From<SaisonRow> for SaisonResponse {
    fn from(s: SaisonRow) -> Self {
        let maintenant = Utc::now();
        let etat = if maintenant < s.debut_at {
            "a_venir"
        } else if maintenant < s.fin_at {
            "en_cours"
        } else {
            "close"
        };
        Self { id: s.id, nom: s.nom, debut_at: s.debut_at, fin_at: s.fin_at, etat }
    }
}

#[derive(Debug, Serialize)]
pub struct SaisonsResponse {
    pub courante: Option<SaisonResponse>,
    pub a_venir: Vec<SaisonResponse>,
    pub archives: Vec<SaisonResponse>,
}

/// Une ligne de classement de membres. Rien d'autre du membre n'est exposé
/// (FR-060) : ni e-mail, ni réputation, ni solde de points.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct LigneClassement {
    pub rang: i64,
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub pays: Option<String>,
    pub pays_iso2: Option<String>,
    pub niveau_code: Option<String>,
    pub niveau_libelle: Option<String>,
    pub score: i32,
}

#[derive(Debug, Serialize)]
pub struct MaPosition {
    pub rang: i64,
    pub score: i32,
    /// Le membre et ses voisins immédiats, pour se situer hors du haut de tableau.
    pub voisins: Vec<LigneClassement>,
}

#[derive(Debug, Serialize)]
pub struct ClassementMembres {
    pub elements: Vec<LigneClassement>,
    pub total: i64,
    pub page: i64,
    pub taille: i64,
    pub moi: Option<MaPosition>,
}

#[derive(Debug, Deserialize)]
pub struct ClassementQueryParams {
    pub saison: Option<Uuid>,
    pub pays: Option<Uuid>,
    pub page: Option<i64>,
    pub taille: Option<i64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct LignePays {
    /// `None` pour un pays sans aucun score : il reste listé (FR-067).
    pub rang: Option<i64>,
    pub pays_id: Uuid,
    pub nom: String,
    pub iso2: Option<String>,
    pub score: i32,
    pub joueurs: i64,
}

// ─── Mon jeu ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct MaSaison {
    pub id: Uuid,
    pub nom: String,
    pub score: i32,
    pub rang: Option<i64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PaysRattachement {
    pub id: Uuid,
    pub nom: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ScoreParModule {
    pub code: String,
    pub libelle: String,
    pub score: i32,
}

#[derive(Debug, Serialize)]
pub struct MonJeu {
    pub score_total: i32,
    pub saison: Option<MaSaison>,
    /// `None` : le profil n'a aucun pays. Le membre figure alors au classement de
    /// tous les membres et dans aucun classement de pays (FR-055).
    pub pays_rattachement: Option<PaysRattachement>,
    pub serie_jours: i16,
    pub par_module: Vec<ScoreParModule>,
    pub score_parties: i32,
    pub score_defis: i32,
    pub score_duels: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PartieHistorique {
    pub id: Uuid,
    pub cadre: String,
    pub module_code: Option<String>,
    pub etat: String,
    pub bonnes: i16,
    pub nombre_epreuves: i32,
    pub score_gagne: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct HistoriqueParties {
    pub elements: Vec<PartieHistorique>,
    pub total: i64,
    pub page: i64,
    pub taille: i64,
}

#[derive(Debug, Deserialize)]
pub struct PageQueryParams {
    pub page: Option<i64>,
    pub taille: Option<i64>,
}

// ─── Duels ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, FromRow)]
pub struct DuelRow {
    pub id: Uuid,
    pub proposant_id: Uuid,
    pub adversaire_id: Uuid,
    pub module_code: String,
    pub mode: String,
    pub etat: String,
    pub compte: bool,
    pub epreuve_ids: Option<Vec<Uuid>>,
    pub propose_at: DateTime<Utc>,
    pub accepte_at: Option<DateTime<Utc>>,
    pub echeance_at: DateTime<Utc>,
    pub issue: Option<String>,
    pub vainqueur_id: Option<Uuid>,
    pub termine_at: Option<DateTime<Utc>>,
    /// Mode direct : manche en cours, son instant de début (fixé par le
    /// serveur, le même pour les deux), et le dernier signe de vie de chacun.
    pub rang_courant: i16,
    pub manche_debut_at: Option<DateTime<Utc>>,
    pub presence_proposant_at: Option<DateTime<Utc>>,
    pub presence_adversaire_at: Option<DateTime<Utc>>,
}

pub const DUEL_COLONNES: &str = "id, proposant_id, adversaire_id, module_code, mode, etat, compte, \
     epreuve_ids, propose_at, accepte_at, echeance_at, issue, vainqueur_id, termine_at, \
     rang_courant, manche_debut_at, presence_proposant_at, presence_adversaire_at";

impl DuelRow {
    pub fn est_termine(&self) -> bool {
        matches!(self.etat.as_str(), "termine" | "refuse" | "annule" | "expire")
    }

    /// L'autre joueur, vu de `moi`.
    pub fn autre(&self, moi: Uuid) -> Uuid {
        if self.proposant_id == moi { self.adversaire_id } else { self.proposant_id }
    }

    pub fn joue(&self, utilisateur_id: Uuid) -> bool {
        self.proposant_id == utilisateur_id || self.adversaire_id == utilisateur_id
    }
}

#[derive(Debug, Deserialize)]
pub struct ProposerDuelRequest {
    pub adversaire_id: Uuid,
    pub module: String,
    /// `differe` ou `direct`.
    pub mode: String,
}

/// La partie d'un joueur dans un duel.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PartieDuel {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub etat: String,
    pub bonnes: i16,
    pub temps_total_ms: i32,
}

/// Un duel vu d'UN des deux joueurs.
#[derive(Debug, Serialize)]
pub struct DuelResponse {
    pub id: Uuid,
    pub mode: String,
    pub module: String,
    pub module_libelle: String,
    pub etat: String,
    /// `false` : duel amical, sans gain (un plafond du jour était atteint).
    pub compte: bool,
    pub je_propose: bool,
    pub adversaire: Option<crate::models::amitie::MembreLight>,
    pub nombre_epreuves: usize,
    pub propose_at: DateTime<Utc>,
    pub accepte_at: Option<DateTime<Utc>>,
    pub echeance_at: DateTime<Utc>,
    pub issue: Option<String>,
    pub vainqueur_id: Option<Uuid>,
    pub termine_at: Option<DateTime<Utc>>,
    pub ma_partie: Option<PartieDuel>,
    /// Le résultat de l'autre n'est servi qu'une fois le duel terminé (FR-042) :
    /// avant, on ne joue pas en connaissant le score à battre.
    pub sa_partie: Option<PartieDuel>,
    /// Ce que ce duel a rapporté au membre (prime de victoire ou de nul).
    pub mon_gain: i32,
}

#[derive(Debug, Serialize)]
pub struct QuotasDuel {
    pub comptes_aujourdhui: i64,
    pub plafond: i16,
}

#[derive(Debug, Serialize)]
pub struct MesDuels {
    pub a_repondre: Vec<DuelResponse>,
    pub a_jouer: Vec<DuelResponse>,
    pub en_attente: Vec<DuelResponse>,
    pub termines: Vec<DuelResponse>,
    pub quotas: QuotasDuel,
}

/// Signaux SSE des duels. Un signal dit qu'il faut RELIRE, il ne transporte
/// jamais l'état : le flux n'a pas de tampon et un membre déconnecté le perd.
/// Aucun signal ne porte une épreuve, une réponse ou un score.
pub fn evt_duel(type_evt: &str, duel_id: Uuid) -> serde_json::Value {
    serde_json::json!({ "type": type_evt, "duel_id": duel_id })
}

pub fn evt_duel_propose(
    duel_id: Uuid,
    mode: &str,
    module: &str,
    proposant: &crate::models::amitie::MembreLight,
    expire_a: DateTime<Utc>,
) -> serde_json::Value {
    serde_json::json!({
        "type": "duel_propose", "duel_id": duel_id, "mode": mode, "module": module,
        "proposant": proposant, "expire_a": expire_a,
    })
}

// ─── Fiche d'un pays (carte) ─────────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct ModulePays {
    pub code: String,
    pub libelle: String,
    pub epreuves_jouables: i64,
    /// Assez d'épreuves sur CE pays pour une partie qui ne porte que sur lui.
    #[sqlx(default)]
    pub disponible: bool,
}

#[derive(Debug, Serialize)]
pub struct FichePaysJeu {
    pub pays_id: Uuid,
    pub nom: String,
    pub iso2: Option<String>,
    /// `None` : aucun score encore (FR-067), le pays n'est pas masqué pour autant.
    pub rang: Option<i64>,
    pub score: i32,
    pub joueurs: i64,
    pub meilleurs: Vec<LigneClassement>,
    pub modules: Vec<ModulePays>,
}

// ─── Duel direct : l'état d'une manche ───────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct CorrectionManche {
    pub bonne_cle: i16,
    pub explication: Option<String>,
    pub lien: Option<String>,
    pub ma_cle: Option<i16>,
    pub sa_cle: Option<i16>,
}

/// L'état autoritaire d'un duel direct, tel que le voit l'un des joueurs. Le
/// client le relit à chaque signal et toutes les trois secondes : c'est lui, et
/// non le signal, qui fait foi.
#[derive(Debug, Serialize)]
pub struct EtatDuelDirect {
    pub etat: String,
    /// `attente` (avant le départ), `question`, `revelation`, `termine`.
    pub phase: &'static str,
    /// Heure du serveur : les deux écrans y calent leurs comptes à rebours.
    pub maintenant: DateTime<Utc>,
    pub rang: i16,
    pub sur: usize,
    pub manche_debut_at: Option<DateTime<Utc>>,
    pub manche_fin_at: Option<DateTime<Utc>>,
    /// Début de la manche suivante (fin de la révélation).
    pub prochaine_at: Option<DateTime<Utc>>,
    /// Servie seulement en phase `question` et `revelation`. Jamais la bonne
    /// réponse : elle n'arrive qu'avec `correction`, aux deux à la fois.
    pub epreuve: Option<EpreuveServie>,
    pub ma_cle: Option<i16>,
    /// Booléen seulement : ce que l'autre a répondu n'est jamais servi pendant
    /// la question, sinon l'un pourrait le souffler à l'autre.
    pub adversaire_a_repondu: bool,
    pub correction: Option<CorrectionManche>,
    pub mes_bonnes: i16,
    pub ses_bonnes: i16,
    pub adversaire_present: bool,
    pub issue: Option<String>,
    pub vainqueur_id: Option<Uuid>,
    pub mon_gain: i32,
}

#[derive(Debug, Deserialize)]
pub struct RepondreDirectRequest {
    pub rang: i16,
    pub cle: i16,
}

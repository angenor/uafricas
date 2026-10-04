//! Modèles d'administration du jeu (feature 013).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Une épreuve vue du back-office : elle porte la bonne réponse, ses compteurs
/// et l'état de sa source.
#[derive(Debug, Serialize, FromRow)]
pub struct EpreuveAdmin {
    pub id: Uuid,
    pub module_code: String,
    pub enonce: String,
    pub media_type: Option<String>,
    pub media_url: Option<String>,
    pub propositions: Vec<String>,
    /// Renseignée pour le seul type `choix` (feature 014).
    pub bonne_reponse: Option<i16>,
    pub explication: Option<String>,
    pub difficulte: i16,
    pub theme: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub type_reponse: String,
    /// Tableaux STOCKÉS (ordre aléatoire) : pour relire l'épreuve, voir
    /// `elements_attendus` et `paires_attendues`.
    pub solution: Option<Vec<i16>>,
    pub appariements: Option<Vec<String>>,
    pub valeurs: Option<Vec<String>>,
    pub reponse_pays_id: Option<Uuid>,
    pub reponse_pays_nom: Option<String>,
    pub reponse_pays_iso: Option<String>,
    /// ordre : les éléments remis dans l'ordre ATTENDU, pour l'édition et la revue.
    #[sqlx(skip)]
    pub elements_attendus: Option<Vec<ElementOrdre>>,
    /// paires : chaque élément de gauche avec son correspondant.
    #[sqlx(skip)]
    pub paires_attendues: Option<Vec<PaireSaisie>>,
    pub origine: String,
    pub type_source: Option<String>,
    pub source_id: Option<Uuid>,
    pub forme: Option<String>,
    pub etat: String,
    pub motif_rejet: Option<String>,
    pub nombre_servie: i32,
    pub nombre_bonnes: i32,
    /// `NULL` tant que l'épreuve n'a jamais été servie.
    pub taux_reussite: Option<f64>,
    /// `aucune`, `conforme`, `modifiee` ou `indisponible`.
    pub source_etat: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Sélection commune à la liste et au détail. `source_etat` se lit dans
/// `jeu.v_source` : c'est la même vue qui décide si l'épreuve est servie.
pub const EPREUVE_ADMIN_SELECT: &str = "
    SELECT e.id, e.module_code, e.enonce, e.media_type, e.media_url, e.propositions,
           e.bonne_reponse, e.explication, e.difficulte, e.theme, e.pays_id, p.nom AS pays_nom,
           e.type_reponse, e.solution, e.appariements, e.valeurs, e.reponse_pays_id,
           rp.nom AS reponse_pays_nom, LOWER(rp.code_iso2) AS reponse_pays_iso,
           e.origine, e.type_source, e.source_id, e.forme, e.etat, e.motif_rejet,
           e.nombre_servie, e.nombre_bonnes,
           CASE WHEN e.nombre_servie > 0
                THEN e.nombre_bonnes::float8 / e.nombre_servie END AS taux_reussite,
           CASE WHEN e.type_source IS NULL THEN 'aucune'
                WHEN NOT COALESCE(s.visible, FALSE) THEN 'indisponible'
                WHEN e.origine = 'derivee' AND s.empreinte <> e.source_empreinte THEN 'modifiee'
                ELSE 'conforme' END AS source_etat,
           e.created_at, e.updated_at
      FROM jeu.epreuve e
      LEFT JOIN shared.pays p ON p.id = e.pays_id
      LEFT JOIN shared.pays rp ON rp.id = e.reponse_pays_id
      LEFT JOIN jeu.v_source s
             ON s.type_source = e.type_source AND s.source_id = e.source_id";

#[derive(Debug, Deserialize)]
pub struct EpreuvesQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub module: Option<String>,
    pub etat: Option<String>,
    pub origine: Option<String>,
    pub difficulte: Option<i16>,
    pub pays: Option<Uuid>,
    pub recherche: Option<String>,
    pub anomalie: Option<bool>,
}

/// Corps de la saisie et de la modification d'une épreuve.
#[derive(Debug, Deserialize)]
pub struct EpreuveRequest {
    pub module: String,
    pub enonce: String,
    pub media_type: Option<String>,
    pub media_url: Option<String>,
    /// `choix` (défaut), `carte`, `ordre` ou `paires` (feature 014).
    #[serde(default)]
    pub type_reponse: Option<String>,
    /// choix : les propositions.
    #[serde(default)]
    pub propositions: Vec<String>,
    /// choix : le rang de la bonne proposition.
    #[serde(default)]
    pub bonne_reponse: Option<i16>,
    /// carte : le pays attendu, par son identifiant…
    #[serde(default)]
    pub reponse_pays_id: Option<Uuid>,
    /// … ou par son code ISO2 (la liste des 55 pays du jeu), résolu par le serveur.
    #[serde(default)]
    pub reponse_pays_iso: Option<String>,
    /// ordre : les éléments DANS L'ORDRE ATTENDU.
    #[serde(default)]
    pub elements: Option<Vec<ElementOrdre>>,
    /// paires : chaque élément de gauche avec son correspondant.
    #[serde(default)]
    pub paires: Option<Vec<PaireSaisie>>,
    pub explication: Option<String>,
    pub difficulte: Option<i16>,
    pub theme: Option<String>,
    pub pays_id: Option<Uuid>,
    pub type_source: Option<String>,
    pub source_id: Option<Uuid>,
}

/// Un élément d'une épreuve « ordre », tel que l'administrateur le saisit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementOrdre {
    pub texte: String,
    #[serde(default)]
    pub valeur: Option<String>,
}

/// Une paire d'une épreuve « paires ».
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaireSaisie {
    pub gauche: String,
    pub droite: String,
}

impl EpreuveAdmin {
    /// Remet l'ordre et les paires dans leur forme ATTENDUE : la base les garde
    /// mélangés (les clés servies n'en disent rien), l'administrateur les relit
    /// comme il les a saisis.
    pub fn avec_forme_attendue(mut self) -> Self {
        let Some(solution) = self.solution.as_deref() else {
            return self;
        };
        match self.type_reponse.as_str() {
            "ordre" => {
                self.elements_attendus = Some(
                    solution
                        .iter()
                        .map(|&cle| {
                            let i = (cle - 1) as usize;
                            ElementOrdre {
                                texte: self.propositions.get(i).cloned().unwrap_or_default(),
                                valeur: self.valeurs.as_ref().and_then(|v| v.get(i).cloned()),
                            }
                        })
                        .collect(),
                );
            }
            "paires" => {
                let droite = self.appariements.clone().unwrap_or_default();
                self.paires_attendues = Some(
                    self.propositions
                        .iter()
                        .zip(solution)
                        .map(|(gauche, &cle)| PaireSaisie {
                            gauche: gauche.clone(),
                            droite: droite.get((cle - 1) as usize).cloned().unwrap_or_default(),
                        })
                        .collect(),
                );
            }
            _ => {}
        }
        self
    }
}

/// Mélange `textes` et renvoie, avec le tableau mélangé, la position (rang à
/// partir de 1) où chaque texte d'origine a atterri. Stocker dans l'ordre de
/// saisie ferait des clés servies la solution en clair (PC4).
pub fn melanger(textes: &[String]) -> (Vec<String>, Vec<i16>) {
    use rand::seq::SliceRandom;
    let mut perm: Vec<usize> = (0..textes.len()).collect();
    perm.shuffle(&mut rand::thread_rng());
    let stocke: Vec<String> = perm.iter().map(|&i| textes[i].clone()).collect();
    let mut position = vec![0i16; textes.len()];
    for (k, &i) in perm.iter().enumerate() {
        position[i] = (k + 1) as i16;
    }
    (stocke, position)
}

/// Refuse un texte vide ou un doublon (sans tenir compte de la casse).
fn controler_textes(textes: &[String], quoi: &str) -> Result<(), String> {
    let mut vus: Vec<String> = Vec::new();
    for t in textes {
        if t.is_empty() {
            return Err(format!("{quoi} : aucun ne peut être vide"));
        }
        let cle = t.to_lowercase();
        if vus.contains(&cle) {
            return Err(format!("{quoi} : « {t} » figure deux fois"));
        }
        vus.push(cle);
    }
    Ok(())
}

/// Une épreuve nettoyée, prête à être écrite.
pub struct EpreuveNettoyee {
    pub module: String,
    pub enonce: String,
    pub media_type: Option<String>,
    pub media_url: Option<String>,
    pub type_reponse: String,
    pub propositions: Vec<String>,
    pub bonne_reponse: Option<i16>,
    pub solution: Option<Vec<i16>>,
    pub appariements: Option<Vec<String>>,
    pub valeurs: Option<Vec<String>>,
    pub reponse_pays_id: Option<Uuid>,
    pub explication: Option<String>,
    pub difficulte: i16,
    pub theme: Option<String>,
    pub pays_id: Option<Uuid>,
    pub type_source: Option<String>,
    pub source_id: Option<Uuid>,
}

fn texte_optionnel(valeur: &Option<String>) -> Option<String> {
    valeur.as_deref().map(str::trim).filter(|v| !v.is_empty()).map(str::to_string)
}

impl EpreuveRequest {
    /// Ce qu'un BROUILLON doit déjà respecter : ce sont les contraintes que la
    /// table impose à toute ligne, quel que soit son état.
    pub fn nettoyer(&self) -> Result<EpreuveNettoyee, String> {
        let enonce = self.enonce.trim().to_string();
        if enonce.is_empty() {
            return Err("L'énoncé est obligatoire".into());
        }

        let type_reponse = self.type_reponse.as_deref().map(str::trim).unwrap_or("choix").to_string();
        let mut propositions: Vec<String> = Vec::new();
        let mut bonne_reponse = None;
        let mut solution = None;
        let mut appariements = None;
        let mut valeurs = None;
        let mut reponse_pays_id = None;

        match type_reponse.as_str() {
            "choix" => {
                propositions = self.propositions.iter().map(|p| p.trim().to_string()).collect();
                if propositions.len() < 2 {
                    return Err("Il faut au moins deux propositions".into());
                }
                if propositions.len() > 6 {
                    return Err("Une épreuve porte au plus six propositions".into());
                }
                let rang = self.bonne_reponse.unwrap_or(0);
                if rang < 1 || rang as usize > propositions.len() {
                    return Err("La bonne réponse doit désigner l'une des propositions".into());
                }
                bonne_reponse = Some(rang);
            }
            "carte" => {
                reponse_pays_id =
                    Some(self.reponse_pays_id.ok_or("Carte : désignez le pays attendu")?);
            }
            "ordre" => {
                let elements = self.elements.as_deref().unwrap_or_default();
                if !(3..=6).contains(&elements.len()) {
                    return Err("Ordre : de 3 à 6 éléments".into());
                }
                let textes: Vec<String> = elements.iter().map(|e| e.texte.trim().to_string()).collect();
                controler_textes(&textes, "Ordre, éléments")?;
                let vals: Vec<Option<String>> = elements
                    .iter()
                    .map(|e| e.valeur.as_deref().map(str::trim).filter(|v| !v.is_empty()).map(str::to_string))
                    .collect();
                let nb_vals = vals.iter().filter(|v| v.is_some()).count();
                if nb_vals != 0 && nb_vals != vals.len() {
                    return Err("Ordre : renseignez la valeur de tous les éléments, ou d'aucun".into());
                }
                let (stocke, position) = melanger(&textes);
                // solution[j] = position de stockage de l'élément attendu en j-ième.
                solution = Some(position.clone());
                if nb_vals > 0 {
                    let mut v = vec![String::new(); vals.len()];
                    for (i, val) in vals.into_iter().enumerate() {
                        v[(position[i] - 1) as usize] = val.unwrap_or_default();
                    }
                    valeurs = Some(v);
                }
                propositions = stocke;
            }
            "paires" => {
                let paires = self.paires.as_deref().unwrap_or_default();
                if !(3..=5).contains(&paires.len()) {
                    return Err("Paires : de 3 à 5 paires".into());
                }
                let gauche: Vec<String> = paires.iter().map(|p| p.gauche.trim().to_string()).collect();
                let droite: Vec<String> = paires.iter().map(|p| p.droite.trim().to_string()).collect();
                controler_textes(&gauche, "Paires, colonne de gauche")?;
                controler_textes(&droite, "Paires, colonne de droite")?;
                // La gauche garde l'ordre de saisie ; la droite est mélangée.
                let (stocke, position) = melanger(&droite);
                solution = Some(position);
                appariements = Some(stocke);
                propositions = gauche;
            }
            _ => return Err("Type de réponse inconnu : choix, carte, ordre ou paires".into()),
        }

        let media_type = texte_optionnel(&self.media_type);
        let media_url = texte_optionnel(&self.media_url);
        if let Some(t) = &media_type {
            if t != "image" && t != "audio" {
                return Err("Le média doit être une image ou un extrait sonore".into());
            }
        }
        if media_type.is_some() != media_url.is_some() {
            return Err("Le média annoncé est manquant".into());
        }

        let difficulte = self.difficulte.unwrap_or(1);
        if !(1..=3).contains(&difficulte) {
            return Err("La difficulté va de 1 à 3".into());
        }

        let type_source = texte_optionnel(&self.type_source);
        if type_source.is_some() != self.source_id.is_some() {
            return Err("La référence à un contenu est incomplète".into());
        }

        Ok(EpreuveNettoyee {
            module: self.module.trim().to_string(),
            enonce,
            media_type,
            media_url,
            type_reponse,
            propositions,
            bonne_reponse,
            solution,
            appariements,
            valeurs,
            reponse_pays_id,
            explication: texte_optionnel(&self.explication),
            difficulte,
            theme: texte_optionnel(&self.theme),
            pays_id: self.pays_id,
            type_source,
            source_id: self.source_id,
        })
    }
}

/// Ce qu'une épreuve doit respecter pour devenir JOUABLE. Le message nomme le
/// manque (FR-074) : la contrainte SQL `ck_epreuve_jouable` reste le dernier
/// filet, mais elle ne sait dire que « violation ».
pub fn valider_publication(
    type_reponse: &str,
    propositions: &[String],
    explication: Option<&str>,
    source_etat: &str,
) -> Result<(), String> {
    if explication.map(str::trim).unwrap_or("").is_empty() {
        return Err("L'explication est obligatoire".into());
    }
    if source_etat == "indisponible" {
        return Err("Le contenu référencé n'est plus publié".into());
    }
    // Carte, ordre et paires : leur cohérence est contrôlée à la saisie, et la
    // base la garantit (CHECK) ; le reste vaut pour le choix multiple.
    if type_reponse != "choix" {
        return Ok(());
    }
    if propositions.iter().filter(|p| !p.trim().is_empty()).count() < 2
        || propositions.iter().any(|p| p.trim().is_empty())
    {
        return Err("Il faut au moins deux propositions, et aucune ne peut être vide".into());
    }
    let mut vues: Vec<String> = Vec::new();
    for proposition in propositions {
        let cle = proposition.trim().to_lowercase();
        if vues.contains(&cle) {
            return Err("Deux propositions sont identiques".into());
        }
        vues.push(cle);
    }
    if source_etat == "indisponible" {
        return Err("Le contenu référencé n'est plus publié".into());
    }
    Ok(())
}

/// Un module vu du back-office, avec son décompte d'épreuves par état.
#[derive(Debug, Serialize, FromRow)]
pub struct ModuleAdmin {
    pub code: String,
    pub libelle: String,
    pub route: String,
    pub icone: Option<String>,
    pub ouvert: bool,
    pub ordre: i16,
    pub candidates: i64,
    pub jouables: i64,
    pub a_revoir: i64,
    pub rejetees: i64,
    pub retirees: i64,
}

#[derive(Debug, Deserialize)]
pub struct ModifierModuleRequest {
    pub ouvert: bool,
}

// ─── Dérivation et revue ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct DerivationRequest {
    pub module: String,
    /// Omis : toutes les formes du module.
    pub formes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RevueRequest {
    pub ids: Vec<Uuid>,
    /// `accepter` ou `rejeter`.
    pub decision: String,
    pub motif: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RefusRevue {
    pub id: Uuid,
    pub raison: String,
}

#[derive(Debug, Serialize)]
pub struct BilanRevue {
    pub acceptees: i64,
    pub rejetees: i64,
    pub refus: Vec<RefusRevue>,
}

// ─── Signalements d'épreuve ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SignalementsQueryParams {
    /// `en_attente` (défaut), `confirme` ou `classe`.
    pub etat: Option<String>,
}

/// Une ligne brute : un signalement et l'épreuve qu'il vise.
#[derive(Debug, FromRow)]
pub struct SignalementRow {
    pub id: Uuid,
    pub epreuve_id: Uuid,
    pub motif: String,
    pub commentaire: Option<String>,
    pub etat: String,
    pub created_at: DateTime<Utc>,
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub enonce: String,
    pub module_code: String,
    pub epreuve_etat: String,
    pub propositions: Vec<String>,
    pub bonne_reponse: Option<i16>,
    pub type_reponse: String,
}

#[derive(Debug, Serialize)]
pub struct SignalementAdmin {
    pub id: Uuid,
    pub motif: String,
    pub commentaire: Option<String>,
    pub etat: String,
    pub created_at: DateTime<Utc>,
    pub auteur_id: Uuid,
    pub auteur: String,
}

/// La file est regroupée PAR ÉPREUVE : dix membres qui signalent la même
/// question sont un seul sujet à traiter.
#[derive(Debug, Serialize)]
pub struct EpreuveSignalee {
    pub epreuve_id: Uuid,
    pub enonce: String,
    pub module_code: String,
    pub epreuve_etat: String,
    pub propositions: Vec<String>,
    pub bonne_reponse: Option<i16>,
    pub type_reponse: String,
    pub signalements: Vec<SignalementAdmin>,
}

#[derive(Debug, Deserialize)]
pub struct DecisionSignalementRequest {
    /// `confirmer` ou `classer`.
    pub decision: String,
    pub retirer_epreuve: Option<bool>,
}

// ─── Défis ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct DefisQueryParams {
    /// `jour` ou `semaine` ; omis : les deux.
    pub periodicite: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct DefiAdmin {
    pub id: Uuid,
    pub periodicite: String,
    pub periode_debut: chrono::NaiveDate,
    pub module_code: Option<String>,
    pub titre: Option<String>,
    pub nombre_epreuves: i32,
    pub origine: String,
    /// Parties ouvertes sur ce défi, terminées ou non.
    pub participants: i64,
    pub termines: i64,
    /// La période n'a pas commencé : le défi est encore modifiable.
    pub a_venir: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProgrammerDefiRequest {
    pub titre: Option<String>,
    pub module: Option<String>,
    pub epreuve_ids: Vec<Uuid>,
}

// ─── Saisons ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct SaisonAdmin {
    pub id: Uuid,
    pub nom: String,
    pub debut_at: DateTime<Utc>,
    pub fin_at: DateTime<Utc>,
    pub cloturee_at: Option<DateTime<Utc>>,
    /// `a_venir`, `en_cours` ou `close`, déduit des dates.
    pub etat: String,
    /// Membres ayant du score dans cette saison.
    pub joueurs: i64,
}

#[derive(Debug, Deserialize)]
pub struct SaisonRequest {
    pub nom: String,
    pub debut_at: DateTime<Utc>,
    pub fin_at: DateTime<Utc>,
}

// ─── Triche ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AnnulerGainsRequest {
    pub motif: String,
    /// Omis : tous les gains du membre.
    pub depuis: Option<DateTime<Utc>>,
    /// Réputation retirée en plus ; omis ou 0 : aucune.
    pub retrait_reputation: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct BilanAnnulation {
    pub gains_annules: i64,
    pub score_retire: i64,
    pub score_total: i32,
    pub reputation_retiree: i32,
}

/// Ce qu'un administrateur voit d'un joueur avant de sanctionner.
#[derive(Debug, Serialize, FromRow)]
pub struct JoueurAdmin {
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub score_total: i32,
    pub gains: i64,
    pub gains_annules: i64,
    pub dernier_gain_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct JoueursQueryParams {
    pub recherche: Option<String>,
}

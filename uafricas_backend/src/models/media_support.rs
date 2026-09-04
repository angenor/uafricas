//! Fiche d'un support média : thématiques multiples et couverture territoriale
//! (feature 009-medias-programmes-episodes, US3 et US4, migration 09r).
//!
//! Deux tables de liaison polymorphes par `(type_support, support_id)`, patron
//! maison employé identiquement par `support_detenteur` (09m),
//! `creneau_programmation` (09n) et les quatre tables d'interactions (09k) :
//! une seule table sert la télé et la radio, une seule requête sert les deux
//! filtres.
//!
//! L'exclusivité entre « liste de territoires » et « couverture continentale »
//! est garantie **en base** par le trigger `verifier_couverture_exclusive` ; les
//! validations d'ici ne servent qu'à produire un message lisible avant que la
//! contrainte ne parle (FR-034).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::ApiErreur;

/// Découpe le filtre `?thematique=<uuid>,<uuid>` en identifiants.
///
/// La liste est séparée par des **virgules** et non répétée en plusieurs clés :
/// `web::Query` s'appuie sur `serde_urlencoded`, qui ne sait pas agréger des
/// occurrences répétées dans un `Vec` : il échoue en 400 dès la première
/// valeur, même seule. Parser ici évite d'ajouter `serde_qs` pour un champ.
///
/// Un identifiant mal formé est rejeté explicitement : l'ignorer élargirait
/// silencieusement le filtre, et le visiteur croirait avoir filtré.
pub fn thematiques_demandees(brut: Option<&str>) -> Result<Vec<Uuid>, ApiErreur> {
    let Some(brut) = brut else { return Ok(Vec::new()) };
    let mut ids = Vec::new();
    for morceau in brut.split(',') {
        let morceau = morceau.trim();
        if morceau.is_empty() {
            continue;
        }
        let id = Uuid::parse_str(morceau).map_err(|_| {
            ApiErreur::Validation(format!("Identifiant de thématique invalide : « {morceau} »"))
        })?;
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    Ok(ids)
}

/// Paramètres de `GET .../thematiques` : le référentiel de filtre peut être
/// borné à une origine de publication (09j/09o) et/ou à un groupe (09u) —
/// le `slug` d'un parent de `shared.categorie`, qui bascule du référentiel
/// générique (22 genres de grille, `parent_id IS NULL`) vers un sous-groupe
/// dédié (44 lignes éditoriales d'Africans Télé International).
#[derive(Debug, Deserialize)]
pub struct ThematiquesQueryParams {
    pub origine: Option<String>,
    pub groupe: Option<String>,
}

/// Valide le filtre `origine` : seules « africans » et « territoire » sont
/// reconnues. Même logique que `lister_sections` (`television.rs`) : une
/// valeur hors référentiel est ignorée plutôt que rejetée, un filtre inconnu
/// ne doit pas casser la page.
pub fn origine_validee(brut: Option<&str>) -> Option<String> {
    brut
        .map(str::trim)
        .filter(|o| *o == "africans" || *o == "territoire")
        .map(str::to_string)
}

// ────────────────────────────────────────────────────────────────
// Thématiques (US3)
// ────────────────────────────────────────────────────────────────

/// Thème déclaré par un support, servi tel quel au public.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ThematiquePublique {
    pub id: Uuid,
    pub nom: String,
    /// `true` pour une ligne éditoriale d'Africans Télé International (09u),
    /// `false` pour un genre de grille (09s). Les deux vivent dans la même
    /// table de liaison et arrivaient donc mélangés : sans ce drapeau, une
    /// fiche de chaîne de la plateforme affichait dix-sept pastilles sous un
    /// libellé unique, là où ce sont deux déclarations de nature différente.
    pub est_ligne_editoriale: bool,
}

/// Thème du référentiel `media` réellement déclaré par au moins un support
/// publié, avec son décompte, même principe que `GET /api/experts/specialites`
/// (on ne propose pas un filtre qui ne remonterait rien).
#[derive(Debug, Serialize, FromRow)]
pub struct ThematiqueDecompte {
    pub id: Uuid,
    pub nom: String,
    /// Texte long, affiché en infobulle native (les 44 lignes éditoriales de
    /// 09u en portent une ; les 22 genres de grille de 09s n'en ont pas).
    pub description: Option<String>,
    pub nombre_supports: i64,
}

#[derive(Debug, Deserialize)]
pub struct ThematiquesRequest {
    pub categorie_ids: Vec<Uuid>,
}

impl ThematiquesRequest {
    /// Le vide n'est refusé que sur un support **publié** : les supports hérités
    /// sans correspondance de catégorie doivent rester consultables, l'obligation
    /// ne valant qu'à la prochaine modification (FR-029).
    pub fn valider(&self, etat_support: &str) -> Result<(), ApiErreur> {
        if self.categorie_ids.is_empty() && etat_support == "publie" {
            return Err(ApiErreur::Validation(
                "Déclarez au moins une thématique : un support publié doit être trouvable par thème".into(),
            ));
        }
        Ok(())
    }

    /// Dédoublonne en conservant l'ordre de saisie, l'unicité SQL refuserait
    /// sinon l'insertion en bloc.
    pub fn ids_uniques(&self) -> Vec<Uuid> {
        let mut vus = std::collections::HashSet::new();
        self.categorie_ids
            .iter()
            .copied()
            .filter(|id| vus.insert(*id))
            .collect()
    }
}

// ────────────────────────────────────────────────────────────────
// Couverture territoriale (US4)
// ────────────────────────────────────────────────────────────────

/// Terminologie : l'interface dit « territoire » là où la base dit `pays`, 
/// convention établie du projet.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct TerritoirePublic {
    pub id: Uuid,
    pub nom: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CouverturePublique {
    /// `true` : le support couvre tout le continent ; `territoires` est alors
    /// vide, et le restera, le trigger SQL refuse l'ajout.
    pub couverture_continentale: bool,
    pub territoires: Vec<TerritoirePublic>,
    /// Support thématique (09v) : la couverture continentale n'y est pas un
    /// choix mais une conséquence, et l'écran d'édition n'a alors aucun champ
    /// de couverture à présenter. Sert les formulaires, qui ne lisent que cet
    /// objet là où la fiche complète du support n'est pas chargée.
    pub est_thematique: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TerritoireDecompte {
    pub id: Uuid,
    pub nom: String,
    pub nombre_supports: i64,
}

/// Référentiel de filtre par territoire, avec le marqueur des supports
/// panafricains : ils remontent sur **chaque** territoire (FR-036) et ne
/// pouvaient donc pas être comptés dans les lignes ci-dessus.
#[derive(Debug, Serialize)]
pub struct TerritoiresDisponibles {
    pub territoires: Vec<TerritoireDecompte>,
    pub continentales: i64,
}

#[derive(Debug, Deserialize)]
pub struct CouvertureRequest {
    #[serde(default)]
    pub couverture_continentale: bool,
    #[serde(default)]
    pub pays_ids: Vec<Uuid>,
}

impl CouvertureRequest {
    pub fn valider(&self, etat_support: &str) -> Result<(), ApiErreur> {
        if self.couverture_continentale && !self.pays_ids.is_empty() {
            return Err(ApiErreur::Validation(
                "Couverture continentale et liste de territoires sont exclusives : choisissez l'une ou l'autre".into(),
            ));
        }
        if !self.couverture_continentale && self.pays_ids.is_empty() && etat_support == "publie" {
            return Err(ApiErreur::Validation(
                "Déclarez au moins un territoire, ou cochez la couverture continentale".into(),
            ));
        }
        Ok(())
    }

    pub fn ids_uniques(&self) -> Vec<Uuid> {
        let mut vus = std::collections::HashSet::new();
        self.pays_ids
            .iter()
            .copied()
            .filter(|id| vus.insert(*id))
            .collect()
    }
}

// ────────────────────────────────────────────────────────────────
// Colonnes SQL
// ────────────────────────────────────────────────────────────────

pub const SUPPORT_THEMATIQUE_COLONNES: &str =
    "st.id, st.type_support::text AS type_support, st.support_id, st.categorie_id, st.created_at";

pub const SUPPORT_TERRITOIRE_COLONNES: &str =
    "ste.id, ste.type_support::text AS type_support, ste.support_id, ste.pays_id, ste.created_at";

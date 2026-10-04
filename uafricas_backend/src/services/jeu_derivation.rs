//! Dérivation d'épreuves candidates à partir du contenu publié (feature 013,
//! research D5).
//!
//! Chaque **forme de question** est une entrée du catalogue [`FORMES`] : une
//! requête qui rend les sources éligibles avec leur énoncé et leur bonne
//! réponse, et une façon de trouver les mauvaises réponses.
//!
//! Quatre règles tiennent ce fichier :
//!
//! 1. **Une forme rejetée sur un contenu n'est jamais reproposée.** L'index
//!    unique `(type_source, source_id, forme)` couvre les rejetées, et
//!    l'insertion est en `ON CONFLICT DO NOTHING`.
//! 2. **Pas de question à réponse ambiguë.** Les distracteurs sont des valeurs
//!    DISTINCTES de la bonne réponse (comparaison insensible à la casse) ; une
//!    source pour laquelle on n'en trouve pas trois ne produit rien.
//! 3. **Une candidate n'est jamais jouable.** Elle naît `candidate` ; seule la
//!    revue d'un administrateur la rend servable.
//!
//! 4. **Une forme dont la bonne réponse peut être partagée** (une devise, un
//!    peuple, une langue officielle) écarte des mauvaises réponses tout ce qui
//!    serait AUSSI juste : ses distracteurs sont `Fournis` par sa requête.
//!
//! Afrolang n'a aucune forme : la plateforme n'a pas de référentiel de langues,
//! ses épreuves sont saisies.
//!
//! Ajouter un module au jeu : une branche dans la vue `jeu.v_source`, et une ou
//! plusieurs entrées ici. Aucune règle du jeu n'est touchée.

use std::collections::HashSet;

use rand::seq::SliceRandom;
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::constants::afripulse_pays_autorises::PAYS_AFRICAINS_ISO2;

/// Nombre de mauvaises réponses d'une épreuve dérivée à choix.
const NOMBRE_DISTRACTEURS: usize = 3;

/// D'où viennent les mauvaises réponses d'une forme.
enum Distracteurs {
    /// Les bonnes réponses des AUTRES sources de la même forme.
    MemeColonne,
    /// Les noms des pays d'Afrique : une forme dont la réponse est un pays a
    /// toujours de quoi faire, même avec trois sources.
    PaysAfricains,
    /// Vrai ou faux : deux propositions fixes, aucun tirage.
    VraiOuFaux,
    /// Carte, ordre, paires : pas de mauvaises réponses à tirer.
    Aucun,
    /// Calculées par la requête de la forme, source par source (colonne
    /// `distracteurs`), quand « mauvaise réponse » dépend de la source : un pays
    /// MOINS peuplé, un pays dont la langue officielle n'est PAS celle demandée…
    Fournis,
}

pub struct Forme {
    pub code: &'static str,
    pub module: &'static str,
    /// `choix`, `carte`, `ordre` ou `paires` (feature 014).
    pub type_reponse: &'static str,
    pub type_source: &'static str,
    pub libelle: &'static str,
    /// `image` quand l'énoncé porte un média (le drapeau).
    media_type: Option<&'static str>,
    distracteurs: Distracteurs,
    /// Rend `source_id, enonce, bonne, explication, pays_id, media_url`.
    /// `$1` : les codes ISO2 des pays d'Afrique, en minuscules.
    sql: &'static str,
}

/// Les sources Afripulse passent par la fiche pays : bornées aux 55 pays
/// d'Afrique et aux fiches non bloquées. (Macro et non constante : `concat!`
/// n'accepte que des littéraux.)
macro_rules! fiche {
    ($select:literal, $condition:literal) => {
        concat!(
            "SELECT fp.id AS source_id, ", $select, ", p.id AS pays_id
               FROM country_profile.fiche_pays fp
               JOIN shared.pays p ON p.id = fp.pays_id
              WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1) AND ", $condition
        )
    };
}

/// Sous-objet d'une fiche (site, recette, personnalité) : exclu s'il est
/// supprimé ou suspendu. Un objet suspendu reste affiché sur les pages
/// publiques, mais on ne tire pas de question d'un contenu signalé.
macro_rules! objet_de_fiche {
    ($table:literal, $enonce:literal, $explication:literal) => {
        concat!(
            "SELECT o.id AS source_id, ", $enonce, " AS enonce, p.nom AS bonne, ",
            $explication, " AS explication, NULL::text AS media_url, p.id AS pays_id
               FROM country_profile.", $table, " o
               JOIN country_profile.fiche_pays fp ON fp.id = o.fiche_pays_id
               JOIN shared.pays p ON p.id = fp.pays_id
              WHERE o.deleted_at IS NULL AND o.suspendu = FALSE
                AND fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)"
        )
    };
}

pub const FORMES: &[Forme] = &[
    // ── Afripulse : la fiche pays ────────────────────────────────────────────
    Forme {
        code: "capitale",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Capitale d'un pays",
        media_type: None,
        distracteurs: Distracteurs::MemeColonne,
        sql: fiche!(
            "p.nom || ' : quelle est sa capitale ?' AS enonce, p.capitale AS bonne,
             p.capitale || ' est la capitale de ce pays : ' || p.nom || '.' AS explication,
             NULL::text AS media_url",
            "btrim(COALESCE(p.capitale, '')) <> ''"
        ),
    },
    Forme {
        code: "pays_de_capitale",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Pays d'une capitale",
        media_type: None,
        distracteurs: Distracteurs::PaysAfricains,
        sql: fiche!(
            "p.capitale || ' est la capitale de quel pays ?' AS enonce, p.nom AS bonne,
             p.capitale || ' est la capitale de ce pays : ' || p.nom || '.' AS explication,
             NULL::text AS media_url",
            "btrim(COALESCE(p.capitale, '')) <> ''"
        ),
    },
    Forme {
        code: "monnaie",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Monnaie d'un pays",
        media_type: None,
        distracteurs: Distracteurs::MemeColonne,
        sql: fiche!(
            "p.nom || ' : quelle est sa monnaie ?' AS enonce, fp.monnaie AS bonne,
             'La monnaie de ce pays (' || p.nom || ') : ' || fp.monnaie || '.' AS explication,
             NULL::text AS media_url",
            "btrim(COALESCE(fp.monnaie, '')) <> ''"
        ),
    },
    Forme {
        code: "drapeau",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Pays d'un drapeau",
        media_type: Some("image"),
        distracteurs: Distracteurs::PaysAfricains,
        sql: fiche!(
            "'À quel pays appartient ce drapeau ?' AS enonce, p.nom AS bonne,
             'C''est le drapeau de ce pays : ' || p.nom || '.' AS explication,
             fp.image_drapeau_url AS media_url",
            "btrim(COALESCE(fp.image_drapeau_url, '')) <> ''"
        ),
    },
    // `image_devise_url` contient en pratique le TEXTE de la devise nationale,
    // pas une URL (seeds 20 et 30).
    Forme {
        code: "devise_nationale",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Devise nationale d'un pays",
        media_type: None,
        distracteurs: Distracteurs::MemeColonne,
        sql: fiche!(
            "p.nom || ' : quelle est sa devise nationale ?' AS enonce, fp.image_devise_url AS bonne,
             '« ' || fp.image_devise_url || ' » est la devise de ce pays : ' || p.nom || '.' AS explication,
             NULL::text AS media_url",
            "btrim(COALESCE(fp.image_devise_url, '')) <> ''
             AND fp.image_devise_url NOT ILIKE 'http%' AND fp.image_devise_url NOT LIKE '/%'"
        ),
    },
    // Sens inverse de la précédente. Une devise peut être partagée (« Un peuple,
    // un but, une foi » : Mali et Sénégal) : les pays qui la portent aussi sont
    // exclus, et seuls les pays dont la devise est CONNUE peuvent servir de
    // mauvaise réponse — un pays sans fiche pourrait la partager sans qu'on le sache.
    Forme {
        code: "pays_de_devise",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Pays d'une devise nationale",
        media_type: None,
        distracteurs: Distracteurs::Fournis,
        sql: "SELECT fp.id AS source_id,
                     'Quel pays a pour devise « ' || btrim(fp.image_devise_url) || ' » ?' AS enonce,
                     p.nom AS bonne,
                     '« ' || btrim(fp.image_devise_url) || ' » est la devise de ce pays : ' || p.nom || '.' AS explication,
                     NULL::text AS media_url, p.id AS pays_id,
                     ARRAY(SELECT p2.nom
                             FROM country_profile.fiche_pays f2
                             JOIN shared.pays p2 ON p2.id = f2.pays_id
                            WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                              AND btrim(COALESCE(f2.image_devise_url, '')) <> ''
                              AND lower(btrim(f2.image_devise_url)) <> lower(btrim(fp.image_devise_url))) AS distracteurs
                FROM country_profile.fiche_pays fp
                JOIN shared.pays p ON p.id = fp.pays_id
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)
                 AND btrim(COALESCE(fp.image_devise_url, '')) <> ''
                 AND fp.image_devise_url NOT ILIKE 'http%' AND fp.image_devise_url NOT LIKE '/%'",
    },
    Forme {
        code: "indicatif_telephonique",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Indicatif téléphonique d'un pays",
        media_type: None,
        distracteurs: Distracteurs::MemeColonne,
        sql: fiche!(
            "p.nom || ' : quel est son indicatif téléphonique ?' AS enonce, btrim(p.indicatif_tel) AS bonne,
             'L''indicatif téléphonique de ce pays (' || p.nom || ') est le ' || btrim(p.indicatif_tel) || '.' AS explication,
             NULL::text AS media_url",
            "btrim(COALESCE(p.indicatif_tel, '')) <> ''"
        ),
    },
    // Comparaisons : les trois autres pays sont nettement en dessous (moins de
    // 80 %), pour qu'une donnée approchée de la fiche ne renverse pas la réponse.
    Forme {
        code: "plus_peuple",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Le pays le plus peuplé",
        media_type: None,
        distracteurs: Distracteurs::Fournis,
        sql: "SELECT fp.id AS source_id,
                     'Lequel de ces pays est le plus peuplé ?' AS enonce,
                     p.nom AS bonne,
                     p.nom || ' : ' || jeu.fmt_habitants(fp.population) || '. ' || d.detail || '.' AS explication,
                     NULL::text AS media_url, p.id AS pays_id, d.noms AS distracteurs
                FROM country_profile.fiche_pays fp
                JOIN shared.pays p ON p.id = fp.pays_id
          CROSS JOIN LATERAL (
                     SELECT array_agg(x.nom) AS noms,
                            string_agg(x.nom || ' : ' || jeu.fmt_habitants(x.population), ' ; ') AS detail
                       FROM (SELECT p2.nom, f2.population
                               FROM country_profile.fiche_pays f2
                               JOIN shared.pays p2 ON p2.id = f2.pays_id
                              WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                AND f2.population > 0 AND f2.population < fp.population * 0.8
                              ORDER BY random() LIMIT 3) x) d
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1) AND fp.population > 0",
    },
    Forme {
        code: "plus_vaste",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Le pays le plus vaste",
        media_type: None,
        distracteurs: Distracteurs::Fournis,
        sql: "SELECT fp.id AS source_id,
                     'Lequel de ces pays est le plus vaste ?' AS enonce,
                     p.nom AS bonne,
                     p.nom || ' : ' || jeu.fmt_km2(fp.superficie_km2) || '. ' || d.detail || '.' AS explication,
                     NULL::text AS media_url, p.id AS pays_id, d.noms AS distracteurs
                FROM country_profile.fiche_pays fp
                JOIN shared.pays p ON p.id = fp.pays_id
          CROSS JOIN LATERAL (
                     SELECT array_agg(x.nom) AS noms,
                            string_agg(x.nom || ' : ' || jeu.fmt_km2(x.superficie_km2), ' ; ') AS detail
                       FROM (SELECT p2.nom, f2.superficie_km2
                               FROM country_profile.fiche_pays f2
                               JOIN shared.pays p2 ON p2.id = f2.pays_id
                              WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                AND f2.superficie_km2 > 0 AND f2.superficie_km2 < fp.superficie_km2 * 0.8
                              ORDER BY random() LIMIT 3) x) d
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1) AND fp.superficie_km2 > 0",
    },
    // La première langue officielle de la fiche ; les mauvaises réponses sont des
    // pays dont la liste de langues officielles NE la contient PAS.
    Forme {
        code: "pays_de_langue_officielle",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "fiche_pays",
        libelle: "Pays d'une langue officielle",
        media_type: None,
        distracteurs: Distracteurs::Fournis,
        sql: "SELECT fp.id AS source_id,
                     'Lequel de ces pays a pour langue officielle : ' || lo.langue || ' ?' AS enonce,
                     p.nom AS bonne,
                     'Langues officielles. ' || p.nom || ' : ' || btrim(fp.langue_officielle) || ' ; ' || d.detail || '.' AS explication,
                     NULL::text AS media_url, p.id AS pays_id, d.noms AS distracteurs
                FROM country_profile.fiche_pays fp
                JOIN shared.pays p ON p.id = fp.pays_id
          CROSS JOIN LATERAL (SELECT btrim(split_part(fp.langue_officielle, ',', 1)) AS langue) lo
          CROSS JOIN LATERAL (
                     SELECT array_agg(x.nom) AS noms,
                            string_agg(x.nom || ' : ' || x.langues, ' ; ') AS detail
                       FROM (SELECT p2.nom, btrim(f2.langue_officielle) AS langues
                               FROM country_profile.fiche_pays f2
                               JOIN shared.pays p2 ON p2.id = f2.pays_id
                              WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                AND btrim(COALESCE(f2.langue_officielle, '')) <> ''
                                AND strpos(lower(f2.langue_officielle), lower(lo.langue)) = 0
                              ORDER BY random() LIMIT 3) x) d
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1) AND lo.langue <> ''",
    },
    // ── Afripulse : les objets d'une fiche ───────────────────────────────────
    Forme {
        code: "pays_du_site",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "site_touristique",
        libelle: "Pays d'un site touristique",
        media_type: None,
        distracteurs: Distracteurs::PaysAfricains,
        sql: objet_de_fiche!(
            "site_touristique",
            "'Dans quel pays se trouve ce site : ' || o.nom || ' ?'",
            "COALESCE(NULLIF(btrim(left(o.description, 400)), ''),
                      o.nom || ' se trouve dans ce pays : ' || p.nom || '.')"
        ),
    },
    Forme {
        code: "pays_de_recette",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "recette_culinaire",
        libelle: "Pays d'une recette",
        media_type: None,
        distracteurs: Distracteurs::PaysAfricains,
        sql: objet_de_fiche!(
            "recette_culinaire",
            "'De quel pays vient ce plat : ' || o.titre || ' ?'",
            "COALESCE(NULLIF(btrim(left(o.histoire, 400)), ''),
                      o.titre || ' est un plat de ce pays : ' || p.nom || '.')"
        ),
    },
    Forme {
        code: "pays_de_personnalite",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "personnalite_connue",
        libelle: "Pays d'une personnalité",
        media_type: None,
        distracteurs: Distracteurs::PaysAfricains,
        sql: objet_de_fiche!(
            "personnalite_connue",
            "'De quel pays est ' || o.nom_complet || ' ?'",
            "COALESCE(NULLIF(btrim(left(o.biographie_courte, 400)), ''),
                      o.nom_complet || ' est de ce pays : ' || p.nom || '.')"
        ),
    },
    // Un peuple vit souvent sur plusieurs pays (les Peuls, les Wolof…) : sont
    // écartés tout pays qui déclare un groupe du même nom, ou qui le cite dans
    // ses langues. Seuls les pays dotés d'une fiche servent de mauvaise réponse.
    Forme {
        code: "pays_du_peuple",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "groupe_ethnique",
        libelle: "Pays d'un peuple",
        media_type: None,
        distracteurs: Distracteurs::Fournis,
        sql: "SELECT ge.id AS source_id,
                     'Dans lequel de ces pays vivent notamment les ' || btrim(ge.nom) || ' ?' AS enonce,
                     p.nom AS bonne,
                     COALESCE(NULLIF(btrim(left(ge.description, 400)), ''),
                              'Les ' || btrim(ge.nom) || ' comptent parmi les peuples de ce pays : ' || p.nom || '.') AS explication,
                     NULL::text AS media_url, p.id AS pays_id,
                     ARRAY(SELECT p2.nom
                             FROM country_profile.fiche_pays f2
                             JOIN shared.pays p2 ON p2.id = f2.pays_id
                            WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                              AND f2.id <> fp.id
                              AND NOT EXISTS (SELECT 1 FROM country_profile.groupe_ethnique g2
                                               WHERE g2.fiche_pays_id = f2.id
                                                 AND lower(btrim(g2.nom)) = lower(btrim(ge.nom)))
                              AND strpos(lower(concat_ws(' ', f2.langue_officielle, f2.langues_populaires)),
                                         lower(btrim(ge.nom))) = 0) AS distracteurs
                FROM country_profile.groupe_ethnique ge
                JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
                JOIN shared.pays p ON p.id = fp.pays_id
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)
                 AND length(btrim(ge.nom)) >= 3",
    },
    // `autre` ne donne rien : ce n'est pas une réponse qu'on puisse deviner.
    Forme {
        code: "domaine_de_personnalite",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "personnalite_connue",
        libelle: "Domaine d'une personnalité",
        media_type: None,
        distracteurs: Distracteurs::Fournis,
        sql: "WITH d(code, libelle) AS (VALUES
                     ('politique', 'Politique'), ('artiste_musicien', 'Musique'),
                     ('artiste_autre', 'Lettres et arts (hors musique)'), ('sportif', 'Sport'),
                     ('entrepreneur', 'Entreprise'), ('scientifique', 'Sciences'),
                     ('militaire_historique', 'Histoire militaire'))
              SELECT o.id AS source_id,
                     o.nom_complet || ' : dans quel domaine cette personnalité s''est-elle illustrée ?' AS enonce,
                     d.libelle AS bonne,
                     COALESCE(NULLIF(btrim(left(o.biographie_courte, 400)), ''),
                              o.nom_complet || ' : ' || d.libelle || '.') AS explication,
                     NULL::text AS media_url, p.id AS pays_id,
                     ARRAY(SELECT d2.libelle FROM d d2 WHERE d2.code <> d.code) AS distracteurs
                FROM country_profile.personnalite_connue o
                JOIN d ON d.code = o.domaine::text
                JOIN country_profile.fiche_pays fp ON fp.id = o.fiche_pays_id
                JOIN shared.pays p ON p.id = fp.pays_id
               WHERE o.deleted_at IS NULL AND o.suspendu = FALSE
                 AND fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)",
    },
    // La photo EST l'énoncé : ni le nom du plat ni celui du site n'y figurent.
    // L'image est COPIÉE dans `uploads/jeu/images/` à la dérivation (D4).
    Forme {
        code: "photo_recette",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "recette_culinaire",
        libelle: "Pays d'un plat, d'après sa photo",
        media_type: Some("image"),
        distracteurs: Distracteurs::PaysAfricains,
        sql: "SELECT o.id AS source_id,
                     'De quel pays vient ce plat ?' AS enonce,
                     p.nom AS bonne,
                     o.titre || ', un plat de ce pays : ' || p.nom || '.'
                       || COALESCE(' ' || NULLIF(btrim(left(o.histoire, 300)), ''), '') AS explication,
                     btrim(o.images[1]) AS media_url, p.id AS pays_id
                FROM country_profile.recette_culinaire o
                JOIN country_profile.fiche_pays fp ON fp.id = o.fiche_pays_id
                JOIN shared.pays p ON p.id = fp.pays_id
               WHERE o.deleted_at IS NULL AND o.suspendu = FALSE
                 AND fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)
                 AND btrim(COALESCE(o.images[1], '')) <> ''",
    },
    Forme {
        code: "photo_site",
        module: "afripulse",
        type_reponse: "choix",
        type_source: "site_touristique",
        libelle: "Pays d'un site, d'après sa photo",
        media_type: Some("image"),
        distracteurs: Distracteurs::PaysAfricains,
        sql: "SELECT o.id AS source_id,
                     'Dans quel pays se trouve ce site ?' AS enonce,
                     p.nom AS bonne,
                     o.nom || ' se trouve dans ce pays : ' || p.nom || '.'
                       || COALESCE(' ' || NULLIF(btrim(left(o.description, 300)), ''), '') AS explication,
                     btrim(o.image_url) AS media_url, p.id AS pays_id
                FROM country_profile.site_touristique o
                JOIN country_profile.fiche_pays fp ON fp.id = o.fiche_pays_id
                JOIN shared.pays p ON p.id = fp.pays_id
               WHERE o.deleted_at IS NULL AND o.suspendu = FALSE
                 AND fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)
                 AND btrim(COALESCE(o.image_url, '')) <> ''",
    },
    // ── Codimoi ──────────────────────────────────────────────────────────────
    // Le proverbe sans pays est le cas courant en base (le formulaire ne
    // l'exige pas) : cette forme ne produit que ce que les membres ont renseigné.
    Forme {
        code: "pays_du_proverbe",
        module: "codimoi",
        type_reponse: "choix",
        type_source: "codimoi",
        libelle: "Pays d'un proverbe",
        media_type: None,
        distracteurs: Distracteurs::PaysAfricains,
        sql: "SELECT c.id AS source_id,
                     'De quel pays vient ce proverbe ? « ' || c.contenu || ' »' AS enonce,
                     p.nom AS bonne,
                     COALESCE(NULLIF(btrim(c.explication), ''),
                              'Ce proverbe vient de ce pays : ' || p.nom || '.') AS explication,
                     NULL::text AS media_url, p.id AS pays_id
                FROM culture.codimoi c
                JOIN shared.pays p ON p.id = c.pays_id
               WHERE c.etat = 'publie' AND c.deleted_at IS NULL
                 AND c.type = 'proverbe_adage' AND LOWER(p.code_iso2) = ANY($1)",
    },
    Forme {
        code: "auteur_de_citation",
        module: "codimoi",
        type_reponse: "choix",
        type_source: "codimoi",
        libelle: "Auteur d'une citation",
        media_type: None,
        distracteurs: Distracteurs::MemeColonne,
        sql: "SELECT c.id AS source_id,
                     'Qui a dit : « ' || c.contenu || ' » ?' AS enonce,
                     btrim(c.nom_auteur_originel) AS bonne,
                     COALESCE(NULLIF(btrim(c.explication), ''),
                              'Cette citation est de ' || btrim(c.nom_auteur_originel) || '.') AS explication,
                     NULL::text AS media_url, c.pays_id AS pays_id
                FROM culture.codimoi c
               WHERE c.etat = 'publie' AND c.deleted_at IS NULL
                 AND c.type = 'citation' AND btrim(COALESCE(c.nom_auteur_originel, '')) <> ''
                 AND $1::text[] IS NOT NULL",
    },
    // Les mauvaises réponses sont les explications d'AUTRES proverbes. Une
    // explication trop longue se repérerait d'un coup d'œil : bornée à 160 signes.
    Forme {
        code: "sens_du_proverbe",
        module: "codimoi",
        type_reponse: "choix",
        type_source: "codimoi",
        libelle: "Sens d'un proverbe",
        media_type: None,
        distracteurs: Distracteurs::MemeColonne,
        sql: "SELECT c.id AS source_id,
                     'Que veut dire ce proverbe ? « ' || btrim(c.contenu) || ' »' AS enonce,
                     btrim(c.explication) AS bonne,
                     '« ' || btrim(c.contenu) || ' » : ' || btrim(c.explication) AS explication,
                     NULL::text AS media_url, c.pays_id AS pays_id
                FROM culture.codimoi c
               WHERE c.etat = 'publie' AND c.deleted_at IS NULL
                 AND c.type = 'proverbe_adage'
                 AND length(btrim(COALESCE(c.explication, ''))) BETWEEN 10 AND 160
                 AND $1::text[] IS NOT NULL",
    },
    // Le mot masqué est le plus long (cinq lettres au moins) : c'est presque
    // toujours le mot porteur de sens. Les mauvaises réponses sont les mots
    // masqués des AUTRES proverbes et citations.
    Forme {
        code: "mot_manquant",
        module: "codimoi",
        type_reponse: "choix",
        type_source: "codimoi",
        libelle: "Mot manquant d'un proverbe ou d'une citation",
        media_type: None,
        distracteurs: Distracteurs::MemeColonne,
        sql: "SELECT c.id AS source_id,
                     CASE c.type WHEN 'citation' THEN 'Complétez cette citation : « '
                                 ELSE 'Complétez ce proverbe : « ' END
                       || regexp_replace(btrim(c.contenu), '\\m' || w.mot || '\\M', '______', 'i') || ' »' AS enonce,
                     lower(w.mot) AS bonne,
                     '« ' || btrim(c.contenu) || ' »'
                       || COALESCE(' (' || NULLIF(btrim(c.nom_auteur_originel), '') || ')', '') AS explication,
                     NULL::text AS media_url, c.pays_id AS pays_id
                FROM culture.codimoi c
          CROSS JOIN LATERAL (SELECT m AS mot
                                FROM regexp_split_to_table(c.contenu, '[^[:alpha:]-]+') m
                               WHERE length(m) >= 5
                               ORDER BY length(m) DESC, m
                               LIMIT 1) w
               WHERE c.etat = 'publie' AND c.deleted_at IS NULL
                 AND c.type IN ('proverbe_adage', 'citation')
                 AND $1::text[] IS NOT NULL",
    },
    // ── FactCheck ────────────────────────────────────────────────────────────
    // Seuls les verdicts tranchés donnent une épreuve : un « partiellement
    // vrai » n'a pas de bonne réponse à deux propositions.
    Forme {
        code: "vrai_ou_faux",
        module: "factcheck",
        type_reponse: "choix",
        type_source: "factcheck",
        libelle: "Idée reçue : vrai ou faux",
        media_type: None,
        distracteurs: Distracteurs::VraiOuFaux,
        sql: "SELECT f.id AS source_id,
                     btrim(f.prejuge_titre) AS enonce,
                     CASE f.verdict WHEN 'vrai' THEN 'Vrai' ELSE 'Faux' END AS bonne,
                     COALESCE(NULLIF(btrim(concat_ws(' : ', NULLIF(btrim(f.realite_titre), ''),
                                                           NULLIF(btrim(f.realite_description), ''))), ''),
                              'Cette affirmation a été vérifiée : elle est ' ||
                              CASE f.verdict WHEN 'vrai' THEN 'vraie.' ELSE 'fausse.' END) AS explication,
                     NULL::text AS media_url, f.pays_id AS pays_id
                FROM governance.factcheck f
               WHERE f.etat = 'publie' AND f.deleted_at IS NULL
                 AND f.verdict IN ('vrai', 'faux')
                 AND btrim(COALESCE(f.prejuge_titre, '')) <> ''
                 AND $1::text[] IS NOT NULL",
    },
    // « Laquelle est vraie ? » : une affirmation vérifiée VRAIE parmi trois
    // idées reçues vérifiées FAUSSES. Une seule bonne réponse par construction.
    Forme {
        code: "affirmation_vraie",
        module: "factcheck",
        type_reponse: "choix",
        type_source: "factcheck",
        libelle: "Laquelle de ces affirmations est vraie ?",
        media_type: None,
        distracteurs: Distracteurs::Fournis,
        sql: "SELECT f.id AS source_id,
                     'Laquelle de ces affirmations est vraie ?' AS enonce,
                     btrim(f.prejuge_titre) AS bonne,
                     '« ' || btrim(f.prejuge_titre) || ' » est vrai'
                       || COALESCE(' : ' || NULLIF(btrim(f.realite_description), ''), '.')
                       || ' Les trois autres sont des idées reçues.' AS explication,
                     NULL::text AS media_url, f.pays_id AS pays_id,
                     ARRAY(SELECT btrim(f2.prejuge_titre) FROM governance.factcheck f2
                            WHERE f2.etat = 'publie' AND f2.deleted_at IS NULL AND f2.verdict = 'faux'
                              AND btrim(COALESCE(f2.prejuge_titre, '')) <> '') AS distracteurs
                FROM governance.factcheck f
               WHERE f.etat = 'publie' AND f.deleted_at IS NULL AND f.verdict = 'vrai'
                 AND btrim(COALESCE(f.prejuge_titre, '')) <> ''
                 AND $1::text[] IS NOT NULL",
    },
    // Le miroir : une idée reçue FAUSSE parmi trois affirmations VRAIES.
    Forme {
        code: "idee_recue",
        module: "factcheck",
        type_reponse: "choix",
        type_source: "factcheck",
        libelle: "Laquelle de ces affirmations est une idée reçue ?",
        media_type: None,
        distracteurs: Distracteurs::Fournis,
        sql: "SELECT f.id AS source_id,
                     'Laquelle de ces affirmations est une idée reçue ?' AS enonce,
                     btrim(f.prejuge_titre) AS bonne,
                     '« ' || btrim(f.prejuge_titre) || ' » est faux'
                       || COALESCE(' : ' || NULLIF(btrim(concat_ws('. ', NULLIF(btrim(f.realite_titre), ''),
                                                                         NULLIF(btrim(f.realite_description), ''))), ''), '.')
                       || ' Les trois autres affirmations sont vraies.' AS explication,
                     NULL::text AS media_url, f.pays_id AS pays_id,
                     ARRAY(SELECT btrim(f2.prejuge_titre) FROM governance.factcheck f2
                            WHERE f2.etat = 'publie' AND f2.deleted_at IS NULL AND f2.verdict = 'vrai'
                              AND btrim(COALESCE(f2.prejuge_titre, '')) <> '') AS distracteurs
                FROM governance.factcheck f
               WHERE f.etat = 'publie' AND f.deleted_at IS NULL AND f.verdict = 'faux'
                 AND btrim(COALESCE(f.prejuge_titre, '')) <> ''
                 AND $1::text[] IS NOT NULL",
    },
    // ══ Feature 014 : carte, ordre, paires ════════════════════════════════════
    // Pour l'ordre et les paires, la requête rend les éléments DANS L'ORDRE
    // ATTENDU ; c'est `deriver_forme` qui les mélange avant d'écrire (PC4).
    //
    // Ordre : le pivot et trois autres pays, chacun à moins de 85 % du
    // précédent — deux valeurs trop proches ne se départagent pas sans doute.
    Forme {
        code: "ordre_population",
        module: "afripulse",
        type_reponse: "ordre",
        type_source: "fiche_pays",
        libelle: "Classer des pays par population",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT fp.id AS source_id,
                     'Classez ces pays du plus peuplé au moins peuplé.' AS enonce,
                     p.nom AS bonne,
                     'Populations : ' || p.nom || ' (' || jeu.fmt_habitants(fp.population) || ') ; '
                       || x1.nom || ' (' || jeu.fmt_habitants(x1.v) || ') ; '
                       || x2.nom || ' (' || jeu.fmt_habitants(x2.v) || ') ; '
                       || x3.nom || ' (' || jeu.fmt_habitants(x3.v) || ').' AS explication,
                     NULL::text AS media_url, p.id AS pays_id,
                     ARRAY[p.nom, x1.nom, x2.nom, x3.nom] AS elements,
                     ARRAY[jeu.fmt_habitants(fp.population), jeu.fmt_habitants(x1.v),
                           jeu.fmt_habitants(x2.v), jeu.fmt_habitants(x3.v)] AS valeurs_ordre
                FROM country_profile.fiche_pays fp
                JOIN shared.pays p ON p.id = fp.pays_id
          CROSS JOIN LATERAL (SELECT p2.nom, f2.population AS v FROM country_profile.fiche_pays f2
                                JOIN shared.pays p2 ON p2.id = f2.pays_id
                               WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                 AND f2.population > 0 AND f2.population < fp.population * 0.85
                               ORDER BY random() LIMIT 1) x1
          CROSS JOIN LATERAL (SELECT p2.nom, f2.population AS v FROM country_profile.fiche_pays f2
                                JOIN shared.pays p2 ON p2.id = f2.pays_id
                               WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                 AND f2.population > 0 AND f2.population < x1.v * 0.85
                               ORDER BY random() LIMIT 1) x2
          CROSS JOIN LATERAL (SELECT p2.nom, f2.population AS v FROM country_profile.fiche_pays f2
                                JOIN shared.pays p2 ON p2.id = f2.pays_id
                               WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                 AND f2.population > 0 AND f2.population < x2.v * 0.85
                               ORDER BY random() LIMIT 1) x3
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1) AND fp.population > 0",
    },
    Forme {
        code: "ordre_superficie",
        module: "afripulse",
        type_reponse: "ordre",
        type_source: "fiche_pays",
        libelle: "Classer des pays par superficie",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT fp.id AS source_id,
                     'Classez ces pays du plus vaste au moins vaste.' AS enonce,
                     p.nom AS bonne,
                     'Superficies : ' || p.nom || ' (' || jeu.fmt_km2(fp.superficie_km2) || ') ; '
                       || x1.nom || ' (' || jeu.fmt_km2(x1.v) || ') ; '
                       || x2.nom || ' (' || jeu.fmt_km2(x2.v) || ') ; '
                       || x3.nom || ' (' || jeu.fmt_km2(x3.v) || ').' AS explication,
                     NULL::text AS media_url, p.id AS pays_id,
                     ARRAY[p.nom, x1.nom, x2.nom, x3.nom] AS elements,
                     ARRAY[jeu.fmt_km2(fp.superficie_km2), jeu.fmt_km2(x1.v),
                           jeu.fmt_km2(x2.v), jeu.fmt_km2(x3.v)] AS valeurs_ordre
                FROM country_profile.fiche_pays fp
                JOIN shared.pays p ON p.id = fp.pays_id
          CROSS JOIN LATERAL (SELECT p2.nom, f2.superficie_km2 AS v FROM country_profile.fiche_pays f2
                                JOIN shared.pays p2 ON p2.id = f2.pays_id
                               WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                 AND f2.superficie_km2 > 0 AND f2.superficie_km2 < fp.superficie_km2 * 0.85
                               ORDER BY random() LIMIT 1) x1
          CROSS JOIN LATERAL (SELECT p2.nom, f2.superficie_km2 AS v FROM country_profile.fiche_pays f2
                                JOIN shared.pays p2 ON p2.id = f2.pays_id
                               WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                 AND f2.superficie_km2 > 0 AND f2.superficie_km2 < x1.v * 0.85
                               ORDER BY random() LIMIT 1) x2
          CROSS JOIN LATERAL (SELECT p2.nom, f2.superficie_km2 AS v FROM country_profile.fiche_pays f2
                                JOIN shared.pays p2 ON p2.id = f2.pays_id
                               WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                 AND f2.superficie_km2 > 0 AND f2.superficie_km2 < x2.v * 0.85
                               ORDER BY random() LIMIT 1) x3
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1) AND fp.superficie_km2 > 0",
    },
    // Paires : le pivot et trois autres pays ; `droite[i]` va avec `elements[i]`.
    Forme {
        code: "paires_capitales",
        module: "afripulse",
        type_reponse: "paires",
        type_source: "fiche_pays",
        libelle: "Associer des pays à leur capitale",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT fp.id AS source_id,
                     'Associez chaque pays à sa capitale.' AS enonce,
                     p.nom AS bonne,
                     p.nom || ' : ' || p.capitale || ' ; ' || o.detail || '.' AS explication,
                     NULL::text AS media_url, p.id AS pays_id,
                     ARRAY[p.nom] || o.noms AS elements,
                     ARRAY[p.capitale] || o.capitales AS droite
                FROM country_profile.fiche_pays fp
                JOIN shared.pays p ON p.id = fp.pays_id
          CROSS JOIN LATERAL (
                     SELECT array_agg(x.nom) AS noms, array_agg(x.capitale) AS capitales,
                            string_agg(x.nom || ' : ' || x.capitale, ' ; ') AS detail
                       FROM (SELECT p2.nom, btrim(p2.capitale) AS capitale
                               FROM country_profile.fiche_pays f2
                               JOIN shared.pays p2 ON p2.id = f2.pays_id
                              WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                AND f2.id <> fp.id AND btrim(COALESCE(p2.capitale, '')) <> ''
                                AND lower(btrim(p2.capitale)) <> lower(btrim(p.capitale))
                              ORDER BY random() LIMIT 3) x) o
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)
                 AND btrim(COALESCE(p.capitale, '')) <> '' AND cardinality(o.noms) = 3",
    },
    // Quatre monnaies TOUTES distinctes : deux pays en franc CFA ne sont jamais
    // réunis, la paire serait ambiguë.
    Forme {
        code: "paires_monnaies",
        module: "afripulse",
        type_reponse: "paires",
        type_source: "fiche_pays",
        libelle: "Associer des pays à leur monnaie",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT fp.id AS source_id,
                     'Associez chaque pays à sa monnaie.' AS enonce,
                     p.nom AS bonne,
                     p.nom || ' : ' || btrim(fp.monnaie) || ' ; ' || o.detail || '.' AS explication,
                     NULL::text AS media_url, p.id AS pays_id,
                     ARRAY[p.nom] || o.noms AS elements,
                     ARRAY[btrim(fp.monnaie)] || o.monnaies AS droite
                FROM country_profile.fiche_pays fp
                JOIN shared.pays p ON p.id = fp.pays_id
          CROSS JOIN LATERAL (
                     SELECT array_agg(y.nom) AS noms, array_agg(y.monnaie) AS monnaies,
                            string_agg(y.nom || ' : ' || y.monnaie, ' ; ') AS detail
                       FROM (SELECT * FROM (
                               SELECT DISTINCT ON (lower(btrim(f2.monnaie))) p2.nom, btrim(f2.monnaie) AS monnaie
                                 FROM country_profile.fiche_pays f2
                                 JOIN shared.pays p2 ON p2.id = f2.pays_id
                                WHERE f2.bloquee = FALSE AND LOWER(p2.code_iso2) = ANY($1)
                                  AND f2.id <> fp.id AND btrim(COALESCE(f2.monnaie, '')) <> ''
                                  AND lower(btrim(f2.monnaie)) <> lower(btrim(fp.monnaie))
                                ORDER BY lower(btrim(f2.monnaie)), random()) d
                              ORDER BY random() LIMIT 3) y) o
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)
                 AND btrim(COALESCE(fp.monnaie, '')) <> '' AND cardinality(o.noms) = 3",
    },
    // Carte : la réponse est un pays, et un seul.
    Forme {
        code: "carte_capitale",
        module: "afripulse",
        type_reponse: "carte",
        type_source: "fiche_pays",
        libelle: "Désigner sur la carte le pays d'une capitale",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT fp.id AS source_id,
                     'Désignez sur la carte le pays dont la capitale est ' || btrim(p.capitale) || '.' AS enonce,
                     p.nom AS bonne,
                     btrim(p.capitale) || ' est la capitale de ce pays : ' || p.nom || '.' AS explication,
                     NULL::text AS media_url, p.id AS pays_id, p.id AS reponse_pays_id
                FROM country_profile.fiche_pays fp
                JOIN shared.pays p ON p.id = fp.pays_id
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)
                 AND btrim(COALESCE(p.capitale, '')) <> ''",
    },
    Forme {
        code: "carte_site",
        module: "afripulse",
        type_reponse: "carte",
        type_source: "site_touristique",
        libelle: "Désigner sur la carte le pays d'un site",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT o.id AS source_id,
                     'Désignez sur la carte le pays où se trouve ce site : ' || o.nom || '.' AS enonce,
                     p.nom AS bonne,
                     COALESCE(NULLIF(btrim(left(o.description, 400)), ''),
                              o.nom || ' se trouve dans ce pays : ' || p.nom || '.') AS explication,
                     NULL::text AS media_url, p.id AS pays_id, p.id AS reponse_pays_id
                FROM country_profile.site_touristique o
                JOIN country_profile.fiche_pays fp ON fp.id = o.fiche_pays_id
                JOIN shared.pays p ON p.id = fp.pays_id
               WHERE o.deleted_at IS NULL AND o.suspendu = FALSE
                 AND fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)",
    },
    // Un peuple partagé par plusieurs pays n'a pas de réponse unique : seuls
    // ceux qu'aucun autre pays ne déclare ni ne cite dans ses langues.
    Forme {
        code: "carte_peuple",
        module: "afripulse",
        type_reponse: "carte",
        type_source: "groupe_ethnique",
        libelle: "Désigner sur la carte le pays d'un peuple",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT ge.id AS source_id,
                     'Désignez sur la carte le pays où vivent les ' || btrim(ge.nom) || '.' AS enonce,
                     p.nom AS bonne,
                     COALESCE(NULLIF(btrim(left(ge.description, 400)), ''),
                              'Les ' || btrim(ge.nom) || ' comptent parmi les peuples de ce pays : ' || p.nom || '.') AS explication,
                     NULL::text AS media_url, p.id AS pays_id, p.id AS reponse_pays_id
                FROM country_profile.groupe_ethnique ge
                JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
                JOIN shared.pays p ON p.id = fp.pays_id
               WHERE fp.bloquee = FALSE AND LOWER(p.code_iso2) = ANY($1)
                 AND length(btrim(ge.nom)) >= 3
                 AND NOT EXISTS (
                     SELECT 1 FROM country_profile.fiche_pays f2
                      WHERE f2.id <> fp.id
                        AND (EXISTS (SELECT 1 FROM country_profile.groupe_ethnique g2
                                      WHERE g2.fiche_pays_id = f2.id
                                        AND lower(btrim(g2.nom)) = lower(btrim(ge.nom)))
                             OR strpos(lower(concat_ws(' ', f2.langue_officielle, f2.langues_populaires)),
                                       lower(btrim(ge.nom))) > 0))",
    },
    // Codimoi : quatre proverbes de quatre pays DIFFÉRENTS, sinon deux pays
    // identiques à droite rendraient la paire ambiguë.
    Forme {
        code: "paires_proverbes",
        module: "codimoi",
        type_reponse: "paires",
        type_source: "codimoi",
        libelle: "Associer des proverbes à leur pays",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT c.id AS source_id,
                     'Associez chaque proverbe à son pays.' AS enonce,
                     p.nom AS bonne,
                     '« ' || btrim(c.contenu) || ' » : ' || p.nom || ' ; ' || o.detail || '.' AS explication,
                     NULL::text AS media_url, c.pays_id AS pays_id,
                     ARRAY[btrim(c.contenu)] || o.textes AS elements,
                     ARRAY[p.nom] || o.pays AS droite
                FROM culture.codimoi c
                JOIN shared.pays p ON p.id = c.pays_id
          CROSS JOIN LATERAL (
                     SELECT array_agg(y.contenu) AS textes, array_agg(y.nom) AS pays,
                            string_agg('« ' || y.contenu || ' » : ' || y.nom, ' ; ') AS detail
                       FROM (SELECT * FROM (
                               SELECT DISTINCT ON (c2.pays_id) btrim(c2.contenu) AS contenu, p2.nom
                                 FROM culture.codimoi c2
                                 JOIN shared.pays p2 ON p2.id = c2.pays_id
                                WHERE c2.etat = 'publie' AND c2.deleted_at IS NULL
                                  AND c2.type = 'proverbe_adage' AND c2.pays_id <> c.pays_id
                                  AND LOWER(p2.code_iso2) = ANY($1)
                                ORDER BY c2.pays_id, random()) d
                              ORDER BY random() LIMIT 3) y) o
               WHERE c.etat = 'publie' AND c.deleted_at IS NULL AND c.type = 'proverbe_adage'
                 AND LOWER(p.code_iso2) = ANY($1) AND cardinality(o.textes) = 3",
    },
    Forme {
        code: "paires_citations",
        module: "codimoi",
        type_reponse: "paires",
        type_source: "codimoi",
        libelle: "Associer des citations à leur auteur",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT c.id AS source_id,
                     'Associez chaque citation à son auteur.' AS enonce,
                     btrim(c.nom_auteur_originel) AS bonne,
                     '« ' || btrim(c.contenu) || ' » : ' || btrim(c.nom_auteur_originel) || ' ; ' || o.detail || '.' AS explication,
                     NULL::text AS media_url, c.pays_id AS pays_id,
                     ARRAY[btrim(c.contenu)] || o.textes AS elements,
                     ARRAY[btrim(c.nom_auteur_originel)] || o.auteurs AS droite
                FROM culture.codimoi c
          CROSS JOIN LATERAL (
                     SELECT array_agg(y.contenu) AS textes, array_agg(y.auteur) AS auteurs,
                            string_agg('« ' || y.contenu || ' » : ' || y.auteur, ' ; ') AS detail
                       FROM (SELECT * FROM (
                               SELECT DISTINCT ON (lower(btrim(c2.nom_auteur_originel)))
                                      btrim(c2.contenu) AS contenu, btrim(c2.nom_auteur_originel) AS auteur
                                 FROM culture.codimoi c2
                                WHERE c2.etat = 'publie' AND c2.deleted_at IS NULL AND c2.type = 'citation'
                                  AND btrim(COALESCE(c2.nom_auteur_originel, '')) <> ''
                                  AND lower(btrim(c2.nom_auteur_originel)) <> lower(btrim(c.nom_auteur_originel))
                                ORDER BY lower(btrim(c2.nom_auteur_originel)), random()) d
                              ORDER BY random() LIMIT 3) y) o
               WHERE c.etat = 'publie' AND c.deleted_at IS NULL AND c.type = 'citation'
                 AND btrim(COALESCE(c.nom_auteur_originel, '')) <> ''
                 AND cardinality(o.textes) = 3 AND $1::text[] IS NOT NULL",
    },
    // FactCheck : chaque idée reçue avec SA correction (la réalité établie).
    Forme {
        code: "paires_idees_recues",
        module: "factcheck",
        type_reponse: "paires",
        type_source: "factcheck",
        libelle: "Associer des idées reçues à leur correction",
        media_type: None,
        distracteurs: Distracteurs::Aucun,
        sql: "SELECT f.id AS source_id,
                     'Associez chaque idée reçue à ce qu''il en est réellement.' AS enonce,
                     btrim(f.realite_titre) AS bonne,
                     'Chaque idée reçue et sa réalité : « ' || btrim(f.prejuge_titre) || ' » → '
                       || btrim(f.realite_titre) || ' ; ' || o.detail || '.' AS explication,
                     NULL::text AS media_url, f.pays_id AS pays_id,
                     ARRAY[btrim(f.prejuge_titre)] || o.prejuges AS elements,
                     ARRAY[btrim(f.realite_titre)] || o.realites AS droite
                FROM governance.factcheck f
          CROSS JOIN LATERAL (
                     SELECT array_agg(y.prejuge) AS prejuges, array_agg(y.realite) AS realites,
                            string_agg('« ' || y.prejuge || ' » → ' || y.realite, ' ; ') AS detail
                       FROM (SELECT btrim(f2.prejuge_titre) AS prejuge, btrim(f2.realite_titre) AS realite
                               FROM governance.factcheck f2
                              WHERE f2.etat = 'publie' AND f2.deleted_at IS NULL AND f2.verdict = 'faux'
                                AND f2.id <> f.id
                                AND btrim(COALESCE(f2.prejuge_titre, '')) <> ''
                                AND btrim(COALESCE(f2.realite_titre, '')) <> ''
                              ORDER BY random() LIMIT 3) y) o
               WHERE f.etat = 'publie' AND f.deleted_at IS NULL AND f.verdict = 'faux'
                 AND btrim(COALESCE(f.prejuge_titre, '')) <> ''
                 AND btrim(COALESCE(f.realite_titre, '')) <> ''
                 AND cardinality(o.prejuges) = 3 AND $1::text[] IS NOT NULL",
    },
];

#[derive(FromRow)]
struct SourceEligible {
    source_id: Uuid,
    enonce: String,
    bonne: String,
    explication: Option<String>,
    media_url: Option<String>,
    pays_id: Option<Uuid>,
    /// Pour `Distracteurs::Fournis` seulement ; absente des autres requêtes.
    #[sqlx(default)]
    distracteurs: Option<Vec<String>>,
    /// ordre : les éléments dans l'ordre attendu ; paires : la colonne de gauche.
    #[sqlx(default)]
    elements: Option<Vec<String>>,
    /// ordre : la justification de chaque élément, alignée sur `elements`.
    #[sqlx(default)]
    valeurs_ordre: Option<Vec<String>>,
    /// paires : le correspondant de chaque élément de gauche.
    #[sqlx(default)]
    droite: Option<Vec<String>>,
    /// carte : le pays attendu.
    #[sqlx(default)]
    reponse_pays_id: Option<Uuid>,
}

/// Ce qu'une candidate écrit, selon le type de réponse de sa forme.
#[derive(Default)]
struct Contenu {
    propositions: Vec<String>,
    bonne_reponse: Option<i16>,
    solution: Option<Vec<i16>>,
    appariements: Option<Vec<String>>,
    valeurs: Option<Vec<String>>,
    reponse_pays_id: Option<Uuid>,
}

/// Tout texte distinct, sans tenir compte de la casse : deux éléments égaux
/// rendraient l'ordre ou les paires ambigus.
fn tous_distincts(textes: &[String]) -> bool {
    let mut vus = HashSet::new();
    textes.iter().all(|t| !t.trim().is_empty() && vus.insert(t.trim().to_lowercase()))
}

/// Compose le contenu d'une candidate carte, ordre ou paires. `None` : la
/// source ne donne pas d'épreuve sans ambiguïté.
fn composer_type(forme: &Forme, source: &SourceEligible) -> Option<Contenu> {
    use crate::models::admin::jeu::melanger;
    match forme.type_reponse {
        "carte" => Some(Contenu { reponse_pays_id: Some(source.reponse_pays_id?), ..Default::default() }),
        "ordre" => {
            let elements = source.elements.as_ref()?;
            if !(3..=6).contains(&elements.len()) || !tous_distincts(elements) {
                return None;
            }
            let (stocke, position) = melanger(elements);
            let valeurs = source.valeurs_ordre.as_ref().map(|vals| {
                let mut v = vec![String::new(); vals.len()];
                for (i, val) in vals.iter().enumerate() {
                    v[(position[i] - 1) as usize] = val.clone();
                }
                v
            });
            Some(Contenu { propositions: stocke, solution: Some(position), valeurs, ..Default::default() })
        }
        "paires" => {
            let (gauche, droite) = (source.elements.as_ref()?, source.droite.as_ref()?);
            if !(3..=5).contains(&gauche.len()) || gauche.len() != droite.len()
                || !tous_distincts(gauche) || !tous_distincts(droite)
            {
                return None;
            }
            let (stocke, position) = melanger(droite);
            Some(Contenu {
                propositions: gauche.clone(),
                solution: Some(position),
                appariements: Some(stocke),
                ..Default::default()
            })
        }
        _ => None,
    }
}

#[derive(Debug, Default, Serialize)]
pub struct BilanForme {
    pub forme: &'static str,
    pub libelle: &'static str,
    pub creees: i64,
    pub deja_proposees: i64,
    pub sans_distracteurs: i64,
    /// Sources dont l'image n'a pas pu être copiée (fichier introuvable).
    pub sans_media: i64,
}

/// Décompte d'une forme pour l'écran de revue, sans rien écrire.
#[derive(Debug, Serialize)]
pub struct EtatForme {
    pub forme: &'static str,
    pub module: &'static str,
    pub type_reponse: &'static str,
    pub libelle: &'static str,
    pub sources_eligibles: i64,
    pub deja_proposees: i64,
}

fn codes_pays() -> Vec<String> {
    PAYS_AFRICAINS_ISO2.iter().map(|c| c.to_lowercase()).collect()
}

async fn sources_eligibles(pool: &PgPool, forme: &Forme) -> Result<Vec<SourceEligible>, sqlx::Error> {
    sqlx::query_as::<_, SourceEligible>(forme.sql)
        .bind(codes_pays())
        .fetch_all(pool)
        .await
}

async fn deja_proposees(pool: &PgPool, forme: &Forme) -> Result<HashSet<Uuid>, sqlx::Error> {
    let ids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT source_id FROM jeu.epreuve
          WHERE origine = 'derivee' AND type_source = $1 AND forme = $2",
    )
    .bind(forme.type_source)
    .bind(forme.code)
    .fetch_all(pool)
    .await?;
    Ok(ids.into_iter().collect())
}

/// Formes d'un module, avec ce qu'elles pourraient encore produire.
pub async fn etat_des_formes(pool: &PgPool) -> Result<Vec<EtatForme>, sqlx::Error> {
    let mut etats = Vec::with_capacity(FORMES.len());
    for forme in FORMES {
        let sources = sources_eligibles(pool, forme).await?;
        let deja = deja_proposees(pool, forme).await?;
        etats.push(EtatForme {
            forme: forme.code,
            module: forme.module,
            type_reponse: forme.type_reponse,
            libelle: forme.libelle,
            sources_eligibles: sources.len() as i64,
            deja_proposees: sources.iter().filter(|s| deja.contains(&s.source_id)).count() as i64,
        });
    }
    Ok(etats)
}

/// Copie l'image d'une source vers `uploads/jeu/images/` et renvoie l'URL de la
/// copie : l'épreuve POSSÈDE son média (research D4). Remplacer ensuite la photo
/// d'une recette ne transforme pas une épreuve déjà revue en question sur une
/// autre image.
///
/// Une URL externe (`http…`) est gardée telle quelle : la copier demanderait un
/// client HTTP, donc une dépendance nouvelle. `None` : fichier local introuvable.
fn copier_media(upload_dir: &str, url: &str) -> Option<String> {
    let url = url.trim();
    if url.starts_with("http://") || url.starts_with("https://") {
        return Some(url.to_string());
    }
    let relatif = url.strip_prefix("/uploads/")?;
    if relatif.contains("..") {
        return None;
    }
    let source = std::path::Path::new(upload_dir).join(relatif);
    if !source.is_file() {
        return None;
    }
    let extension = source.extension().and_then(|e| e.to_str()).unwrap_or("jpg").to_lowercase();
    let nom = format!("{}.{}", Uuid::new_v4(), extension);
    let dossier = std::path::Path::new(upload_dir).join("jeu/images");
    std::fs::create_dir_all(&dossier).ok()?;
    std::fs::copy(&source, dossier.join(&nom)).ok()?;
    Some(format!("/uploads/jeu/images/{nom}"))
}

/// Supprime une copie devenue orpheline (insertion refusée par l'unicité).
fn supprimer_copie(upload_dir: &str, url: &str) {
    if let Some(relatif) = url.strip_prefix("/uploads/jeu/images/") {
        let _ = std::fs::remove_file(std::path::Path::new(upload_dir).join("jeu/images").join(relatif));
    }
}

/// Tire trois mauvaises réponses distinctes de la bonne et entre elles
/// (comparaison insensible à la casse). `None` s'il n'y en a pas assez : la
/// source ne produit alors aucune candidate plutôt qu'une épreuve bancale.
fn tirer_distracteurs(reservoir: &[String], bonne: &str) -> Option<Vec<String>> {
    let cle = |v: &str| v.trim().to_lowercase();
    let cle_bonne = cle(bonne);

    let mut vus: HashSet<String> = HashSet::new();
    let mut candidats: Vec<&String> = reservoir
        .iter()
        .filter(|v| {
            let k = cle(v);
            !k.is_empty() && k != cle_bonne && vus.insert(k)
        })
        .collect();

    if candidats.len() < NOMBRE_DISTRACTEURS {
        return None;
    }
    candidats.shuffle(&mut rand::thread_rng());
    Some(candidats.into_iter().take(NOMBRE_DISTRACTEURS).cloned().collect())
}

/// Compose les propositions et le rang de la bonne. L'ordre stocké est tiré au
/// hasard ; il est de toute façon mélangé à chaque présentation.
fn composer_propositions(
    forme: &Forme,
    source: &SourceEligible,
    reservoir: &[String],
) -> Option<(Vec<String>, i16)> {
    if let Distracteurs::VraiOuFaux = forme.distracteurs {
        let rang = if source.bonne == "Vrai" { 1 } else { 2 };
        return Some((vec!["Vrai".to_string(), "Faux".to_string()], rang));
    }

    let reservoir = match forme.distracteurs {
        Distracteurs::Fournis => source.distracteurs.as_deref().unwrap_or_default(),
        _ => reservoir,
    };
    let mut propositions = tirer_distracteurs(reservoir, &source.bonne)?;
    propositions.push(source.bonne.trim().to_string());
    propositions.shuffle(&mut rand::thread_rng());
    let rang = propositions.iter().position(|p| p == source.bonne.trim())? + 1;
    Some((propositions, rang as i16))
}

/// Dérive les candidates d'une forme. Rejouable sans doublon : les sources déjà
/// proposées (acceptées, rejetées ou encore en revue) sont sautées, et
/// l'insertion reste protégée par l'index unique en cas de course.
pub async fn deriver_forme(
    pool: &PgPool,
    forme: &Forme,
    upload_dir: &str,
) -> Result<BilanForme, sqlx::Error> {
    let sources = sources_eligibles(pool, forme).await?;
    let deja = deja_proposees(pool, forme).await?;

    let reservoir: Vec<String> = match forme.distracteurs {
        Distracteurs::MemeColonne => sources.iter().map(|s| s.bonne.clone()).collect(),
        Distracteurs::PaysAfricains => {
            sqlx::query_scalar("SELECT nom FROM shared.pays WHERE LOWER(code_iso2) = ANY($1)")
                .bind(codes_pays())
                .fetch_all(pool)
                .await?
        }
        Distracteurs::VraiOuFaux | Distracteurs::Fournis | Distracteurs::Aucun => Vec::new(),
    };

    let mut bilan = BilanForme { forme: forme.code, libelle: forme.libelle, ..Default::default() };

    for source in &sources {
        if deja.contains(&source.source_id) {
            bilan.deja_proposees += 1;
            continue;
        }
        let contenu = if forme.type_reponse == "choix" {
            composer_propositions(forme, source, &reservoir).map(|(propositions, bonne)| Contenu {
                propositions,
                bonne_reponse: Some(bonne),
                ..Default::default()
            })
        } else {
            composer_type(forme, source)
        };
        let Some(contenu) = contenu else {
            bilan.sans_distracteurs += 1;
            continue;
        };

        // Une forme à image copie son média ; sans fichier, pas de candidate.
        let media_url = match (forme.media_type, source.media_url.as_deref()) {
            (Some(_), Some(url)) => match copier_media(upload_dir, url) {
                Some(copie) => Some(copie),
                None => {
                    bilan.sans_media += 1;
                    continue;
                }
            },
            _ => None,
        };

        // L'empreinte est lue dans la vue au moment de l'insertion : c'est elle
        // que le tirage comparera ensuite pour savoir si la source a changé.
        let inserees = sqlx::query(
            "INSERT INTO jeu.epreuve
                (module_code, enonce, media_type, media_url, propositions, bonne_reponse,
                 explication, difficulte, pays_id, origine, type_source, source_id, forme,
                 source_empreinte, etat, type_reponse, solution, appariements, valeurs,
                 reponse_pays_id)
             SELECT $1, $2, $3, $4, $5, $6, $7, 1, $8, 'derivee', $9, $10, $11,
                    s.empreinte, 'candidate', $12, $13, $14, $15, $16
               FROM jeu.v_source s
              WHERE s.type_source = $9 AND s.source_id = $10 AND s.visible
             ON CONFLICT (type_source, source_id, forme) WHERE origine = 'derivee' DO NOTHING",
        )
        .bind(forme.module)
        .bind(&source.enonce)
        .bind(forme.media_type.filter(|_| media_url.is_some()))
        .bind(media_url.as_deref())
        .bind(&contenu.propositions)
        .bind(contenu.bonne_reponse)
        .bind(&source.explication)
        .bind(source.pays_id)
        .bind(forme.type_source)
        .bind(source.source_id)
        .bind(forme.code)
        .bind(forme.type_reponse)
        .bind(&contenu.solution)
        .bind(&contenu.appariements)
        .bind(&contenu.valeurs)
        .bind(contenu.reponse_pays_id)
        .execute(pool)
        .await?
        .rows_affected();

        if inserees == 1 {
            bilan.creees += 1;
        } else {
            bilan.deja_proposees += 1;
            if let Some(copie) = media_url.as_deref() {
                supprimer_copie(upload_dir, copie);
            }
        }
    }

    Ok(bilan)
}

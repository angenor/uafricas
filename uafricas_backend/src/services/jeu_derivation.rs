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
    /// Calculées par la requête de la forme, source par source (colonne
    /// `distracteurs`), quand « mauvaise réponse » dépend de la source : un pays
    /// MOINS peuplé, un pays dont la langue officielle n'est PAS celle demandée…
    Fournis,
}

pub struct Forme {
    pub code: &'static str,
    pub module: &'static str,
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
    // ── Codimoi ──────────────────────────────────────────────────────────────
    // Le proverbe sans pays est le cas courant en base (le formulaire ne
    // l'exige pas) : cette forme ne produit que ce que les membres ont renseigné.
    Forme {
        code: "pays_du_proverbe",
        module: "codimoi",
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
}

#[derive(Debug, Default, Serialize)]
pub struct BilanForme {
    pub forme: &'static str,
    pub libelle: &'static str,
    pub creees: i64,
    pub deja_proposees: i64,
    pub sans_distracteurs: i64,
}

/// Décompte d'une forme pour l'écran de revue, sans rien écrire.
#[derive(Debug, Serialize)]
pub struct EtatForme {
    pub forme: &'static str,
    pub module: &'static str,
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
            libelle: forme.libelle,
            sources_eligibles: sources.len() as i64,
            deja_proposees: sources.iter().filter(|s| deja.contains(&s.source_id)).count() as i64,
        });
    }
    Ok(etats)
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
pub async fn deriver_forme(pool: &PgPool, forme: &Forme) -> Result<BilanForme, sqlx::Error> {
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
        Distracteurs::VraiOuFaux | Distracteurs::Fournis => Vec::new(),
    };

    let mut bilan = BilanForme { forme: forme.code, libelle: forme.libelle, ..Default::default() };

    for source in &sources {
        if deja.contains(&source.source_id) {
            bilan.deja_proposees += 1;
            continue;
        }
        let Some((propositions, bonne_reponse)) = composer_propositions(forme, source, &reservoir)
        else {
            bilan.sans_distracteurs += 1;
            continue;
        };

        // L'empreinte est lue dans la vue au moment de l'insertion : c'est elle
        // que le tirage comparera ensuite pour savoir si la source a changé.
        let inserees = sqlx::query(
            "INSERT INTO jeu.epreuve
                (module_code, enonce, media_type, media_url, propositions, bonne_reponse,
                 explication, difficulte, pays_id, origine, type_source, source_id, forme,
                 source_empreinte, etat)
             SELECT $1, $2, $3, $4, $5, $6, $7, 1, $8, 'derivee', $9, $10, $11,
                    s.empreinte, 'candidate'
               FROM jeu.v_source s
              WHERE s.type_source = $9 AND s.source_id = $10 AND s.visible
             ON CONFLICT (type_source, source_id, forme) WHERE origine = 'derivee' DO NOTHING",
        )
        .bind(forme.module)
        .bind(&source.enonce)
        .bind(forme.media_type.filter(|_| source.media_url.is_some()))
        .bind(source.media_url.as_deref().filter(|_| forme.media_type.is_some()))
        .bind(&propositions)
        .bind(bonne_reponse)
        .bind(&source.explication)
        .bind(source.pays_id)
        .bind(forme.type_source)
        .bind(source.source_id)
        .bind(forme.code)
        .execute(pool)
        .await?
        .rows_affected();

        if inserees == 1 {
            bilan.creees += 1;
        } else {
            bilan.deja_proposees += 1;
        }
    }

    Ok(bilan)
}

# Research: Matching et Découverte de Parents

**Date**: 2026-03-16
**Feature Branch**: `001-matching-arbres`

## Décision 1 : Algorithme de similarité des noms

**Décision** : pg_trgm (trigram similarity) comme moteur principal, avec une couche de normalisation phonétique appliquée avant comparaison.

**Raisonnement** :
- `pg_trgm` gère les fautes de frappe, les correspondances partielles, et retourne un score 0-1 directement exploitable. L'opérateur `%` permet des lookups via index GIN sans scan séquentiel.
- Une fonction `normaliser_nom()` applique des règles phonétiques africaines avant stockage : `ou→u`, `dy→di`, `ll→l`, suppression du `h` final muet, normalisation des diacritiques.
- Deux colonnes ajoutées : `nom_normalise` et `prenoms_normalise`, indexées via GIN trigram.
- Seuil trigram de 0.3 pour le pré-filtre (large), puis seuil composite de 55% pour les suggestions finales.

**Alternatives évaluées** :
- **Levenshtein** : Sensible à la longueur, moins adapté aux variantes africaines. Utile en complément pour noms courts (< 4 caractères).
- **Soundex/Metaphone** : Conçus pour l'anglais, inadaptés aux phonétiques ouest-africaines. Rejeté.

## Décision 2 : Exécution hybride du matching

**Décision** : Vérification synchrone rapide (nom exact) dans le handler `creer_personne`, puis `tokio::spawn` pour le matching profond en tâche de fond.

**Raisonnement** :
- À l'échelle cible (10 000 personnes), le matching profond via GIN trigram prend < 100ms en SQL. `tokio::spawn` est suffisant : pas besoin de job queue externe.
- Le matching synchrone rapide (nom exact) donne un feedback immédiat à l'utilisateur.
- Le matching profond (fuzzy) s'exécute en background, stocke les résultats en base, l'utilisateur les voit à sa prochaine visite sur la page Découvertes.
- En cas d'erreur du matching, la création de personne n'est pas affectée (fire-and-forget avec log d'erreur).

**Alternatives évaluées** :
- **Tout synchrone** : Bloquerait l'ajout pendant le matching. Mauvaise UX. Rejeté.
- **Job queue externe (Redis/RabbitMQ)** : Complexité opérationnelle injustifiée à cette échelle. Rejeté.
- **Cron périodique** : Mauvais retour utilisateur (feedback différé de plusieurs heures). Rejeté.

## Décision 3 : Formule de scoring

**Décision** : Score composite pondéré sur 5 critères.

| Critère | Poids | Méthode |
|---------|-------|---------|
| Nom de famille | 35% | `similarity(nom_normalise)` via pg_trgm |
| Prénoms | 20% | `similarity(prenoms_normalise)` via pg_trgm |
| Année de naissance | 15% | Gaussienne (sigma=5 ans) |
| Lieu de naissance | 20% | `similarity()` sur lieu normalisé |
| Genre | 10% | Match exact=1.0, différent=0.0, null=0.5 |

**Seuil minimum** : 55% (plus bas que les 60% habituels car les données généalogiques africaines sont souvent parcellaires).

**Raisonnement** :
- Le nom est le critère le plus discriminant (35%).
- Les prénoms sont moins fiables en Afrique (conventions variées, usage du nom du père comme prénom) → 20%.
- Les dates sont souvent approximatives (année seule) → 15% avec fonction gaussienne (±5 ans = 61%).
- Le lieu est un bon discriminant en contexte africain (village/ville d'origine fort marqueur identitaire) → 20%.
- Le genre permet d'éliminer les faux positifs évidents → 10%.

## Décision 4 : Structure de données : nouveau schema vs extension

**Décision** : Étendre le schema `arbre_genealogique` existant avec de nouvelles tables plutôt que créer un nouveau schema.

**Raisonnement** :
- Les correspondances lient des personnes du même schema `arbre_genealogique`. Un schema séparé ajouterait des foreign keys cross-schema inutilement complexes.
- La Décision 8 du research.md de Feature 1 (architecture Personne/Rattachement) a été conçue exactement pour ce cas, le matching compare des rattachements de différents arbres.
- Principe V (YAGNI) de la constitution.

## Décision 5 : Confidentialité et identité anonymisée

**Décision** : Identifier l'autre utilisateur par « Membre #XXXX » (hash court de l'UUID) jusqu'à l'acceptation de la demande de contact.

**Raisonnement** :
- FR-004 et FR-016 exigent l'anonymat avant confirmation mutuelle / acceptation de contact.
- Un hash court (4 derniers caractères de l'UUID) est suffisant pour distinguer les membres sans révéler d'identité.
- Après acceptation de la demande de contact : nom, prénom et email deviennent visibles mutuellement.

## Décision 6 : Pas de nouvelle dépendance Rust

**Décision** : Tout le matching se fait en SQL (pg_trgm, fuzzystrmatch). La normalisation phonétique est une fonction Rust pure (manipulation de chaînes).

**Raisonnement** :
- pg_trgm et les GIN indexes font le gros du travail côté PostgreSQL.
- La normalisation en Rust est simple (lowercase, remplacement de patterns, suppression de diacritiques).
- Pas besoin de crate externe (`strsim` etc.), les fonctions SQL sont suffisantes.
- Si besoin futur de matching côté Rust : `strsim` (Jaro-Winkler) est une option légère.

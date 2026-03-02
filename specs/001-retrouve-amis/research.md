# Research: Retrouve Amis

**Feature**: 001-retrouve-amis | **Date**: 2026-02-27

---

## R1 — Choix du schema PostgreSQL

**Decision** : Créer un nouveau schema `retrouve_amis` (11ème bounded context).

**Rationale** :
- Le domaine "retrouver des amis" est orthogonal aux 10 schemas existants
- Cycle de vie indépendant : les avis de recherche, correspondances et blacklists n'ont de dépendance qu'avec `iam.utilisateur` et `shared.pays`
- Suit le pattern bounded-context établi (chaque domaine métier = 1 schema)
- Facilite une éventuelle extraction en microservice futur

**Alternatives considérées** :
- `iam` — Rejeté : schema déjà riche (utilisateurs, rôles, permissions, organisations, expertise, tokens). Mélange de responsabilités IAM vs social.
- `culture` — Rejeté : les centres culturels et Codi-Moi sont de la culture, pas du social/connexion.
- `exchange` — Rejeté : dédié aux programmes d'échange académiques, pas aux connexions personnelles.

---

## R2 — Algorithme de matching et scoring

**Decision** : Scoring multi-critères basé sur les extensions PostgreSQL existantes (`pg_trgm` + `unaccent`) avec une fonction SQL pure.

**Rationale** :
- `pg_trgm` est déjà chargé (`00_extensions.sql`) mais inutilisé → opportunité d'exploiter l'investissement
- `unaccent` est déjà chargé → essentiel pour les noms africains (Kouamé/Kouame, Ndèye/Ndeye)
- Fonction SQL → scoring calculé côté base, pas besoin de charger les données en mémoire Rust
- Pas de dépendance externe (Elasticsearch, Meilisearch) → YAGNI

**Modèle de scoring** (total 100 points) :

| Critère | Poids | Méthode |
|---------|-------|---------|
| Nom/Prénom | 40 pts | `similarity(unaccent(lower(a)), unaccent(lower(b)))` via pg_trgm |
| Ville | 15 pts | Correspondance exacte (même ville) ou similarité trigramme |
| Pays | 10 pts | Correspondance exacte (même pays_id) |
| École | 20 pts | Similarité trigramme sur le nom d'école |
| Période | 15 pts | Chevauchement temporel : `GREATEST(0, MIN(fin_a, fin_b) - MAX(debut_a, debut_b)) / GREATEST(1, MAX(fin_a, fin_b) - MIN(debut_a, debut_b))` |

**Seuil de notification** : score >= 60 (sur 100).

**Formule nom** (détail) :
```sql
-- Score nom = max entre :
-- 1. Similarité directe nom_recherche ↔ nom_cible
-- 2. Similarité directe nom_recherche ↔ prenom_cible (inversion possible)
-- 3. Combinaison nom+prénom si les deux sont renseignés
GREATEST(
  similarity(unaccent(lower(nom_recherche)), unaccent(lower(cible_nom))),
  similarity(unaccent(lower(nom_recherche)), unaccent(lower(cible_prenom))),
  similarity(
    unaccent(lower(nom_recherche || ' ' || COALESCE(prenom_recherche, ''))),
    unaccent(lower(cible_nom || ' ' || cible_prenom))
  )
) * 40
```

**Alternatives considérées** :
- Elasticsearch/Meilisearch — Rejeté : infra supplémentaire, YAGNI pour le volume initial, PostgreSQL natif suffit
- Soundex/Metaphone — Rejeté : optimisés pour l'anglais, mauvais sur les noms africains francophones
- Levenshtein — Considéré mais trigrammes plus robustes pour les variations orthographiques courantes

---

## R3 — Stratégie de notification

**Decision** : Notifications stockées en base (`retrouve_amis.notification_retrouve`) avec polling côté frontend.

**Rationale** :
- Aucun système de notification n'existe actuellement dans la plateforme
- Le schema.sql prévoit un futur schema `notification` pour WebSocket, mais il n'est pas implémenté
- Polling via composable (`useRetrouvAmis`) toutes les 60 secondes sur la page retrouve-amis est suffisant
- SC-002 demande un délai < 5 minutes → polling 60s satisfait largement cette exigence
- YAGNI : pas de WebSocket pour un MVP

**Mécanisme** :
1. Le handler de création d'avis déclenche le matching synchrone
2. Les correspondances trouvées sont insérées en base
3. Une notification est créée pour chaque utilisateur concerné
4. Le frontend poll les notifications non lues via un composable

**Alternatives considérées** :
- WebSocket/SSE — Rejeté : infrastructure absente, over-engineering pour le MVP
- Job queue (background worker) — Rejeté : complexité supplémentaire, le matching synchrone est rapide pour le volume initial
- Email — À ajouter en Phase 2 via le système SMTP existant (Lettre)

---

## R4 — Déclenchement du matching

**Decision** : Matching synchrone déclenché dans le handler HTTP lors de 3 événements.

**Événements déclencheurs** :
1. **Création d'un avis** → match contre tous les avis actifs + profils trouvables
2. **Modification d'un avis** → supprime les correspondances `en_attente` existantes, relance le match
3. **Activation de `est_trouvable`** → match du profil contre tous les avis actifs

**Rationale** :
- Synchrone dans le handler : simple, pas d'infrastructure de job queue
- Volume initial faible : quelques centaines d'avis max → requête SQL avec pg_trgm est rapide
- La fonction SQL de scoring peut traiter ~1000 comparaisons en < 100ms

**Scalabilité future** (hors scope MVP) :
- Si le volume dépasse 10k avis actifs, migrer vers un job asynchrone (tokio::spawn ou cron PostgreSQL)
- La fonction SQL reste la même, seul le déclenchement change

---

## R5 — Gestion de la confidentialité et anti-abus

**Decision** : Mécanisme en 4 couches.

### Couche 1 — Anonymat des avis
- Les avis de recherche sont strictement privés (FR-003)
- Aucun endpoint public ne liste les avis d'autres utilisateurs
- Seul l'auteur voit ses propres avis

### Couche 2 — Résumé anonymisé
- Les correspondances affichent : initiales (pas le nom complet), ville, période, score %
- Les coordonnées ne sont jamais visibles avant consentement mutuel

### Couche 3 — Consentement mutuel (double opt-in)
- État `en_attente` → A accepte → `acceptee_a` → B accepte → `mutuelle`
- Les coordonnées ne sont partagées qu'à l'état `mutuelle`
- Chaque utilisateur choisit quelles coordonnées partager (email, téléphone, messagerie)

### Couche 4 — Anti-harcèlement
- Refus → blacklist automatique (`retrouve_amis.blacklist`)
- Table symétrique : `CHECK (utilisateur_a_id < utilisateur_b_id)` évite les doublons
- La blacklist empêche toute future correspondance entre ces deux utilisateurs
- Max 10 avis actifs simultanés (vérifié côté handler)
- Signalement d'avis → suspension en attente de modération admin

---

## R6 — Modification de `iam.utilisateur`

**Decision** : Ajouter une seule colonne `est_trouvable BOOLEAN NOT NULL DEFAULT FALSE` sur `iam.utilisateur`.

**Rationale** :
- C'est un attribut du profil utilisateur, pas une entité séparée
- Évite une table 1:1 inutile (`preference_trouvabilite`) → YAGNI
- Le parcours détaillé (écoles, villes passées) est stocké dans `retrouve_amis.parcours_trouvable` (table séparée car 1:N)
- DEFAULT FALSE respecte l'opt-in explicite (FR-009)

**Impact** :
- Migration SQL : `ALTER TABLE iam.utilisateur ADD COLUMN est_trouvable BOOLEAN NOT NULL DEFAULT FALSE;`
- Struct Rust `Utilisateur` : ajout du champ
- Interface TS `Utilisateur` : ajout du champ
- Handler profil : endpoint PATCH pour basculer

---

## R7 — Structure des pages frontend

**Decision** : Section dédiée `retrouve-amis/` dans les pages, avec composants feature-based.

**Pages publiques** :
| Page | Rôle |
|------|------|
| `retrouve-amis/index.vue` | Landing : explication + CTA (créer un avis, activer trouvable) |
| `retrouve-amis/nouveau.vue` | Formulaire multi-étapes de création d'avis |
| `retrouve-amis/mes-recherches.vue` | Liste des avis de l'utilisateur (actifs/clôturés) |
| `retrouve-amis/correspondances.vue` | Liste des correspondances avec filtres |
| `retrouve-amis/correspondances/[id].vue` | Détail d'une correspondance + actions (accepter/refuser) |

**Pages admin** :
| Page | Rôle |
|------|------|
| `admin/retrouve-amis/index.vue` | Liste paginée de tous les avis (AdminDataTable) |
| `admin/retrouve-amis/[id].vue` | Détail d'un avis + actions modération |
| `admin/retrouve-amis/signalements.vue` | Liste des signalements à modérer |

**Pattern UI** :
- Landing page : Tailwind pur (site public, pas de daisyUI)
- Pages admin : daisyUI (conforme Constitution VI)
- Formulaire multi-étapes : composant `AvisRechercheForm.vue` avec étapes progressives
- Pattern existant : Hero/Card/Filters réutilisé

---

## R8 — Stockage et rétention des données

**Decision** : Conservation indéfinie des avis clôturés et correspondances (FR-018).

**Rationale** :
- La spec exige que l'utilisateur puisse consulter son historique complet à tout moment
- Soft deletion (`deleted_at`) sur les avis pour la suppression admin
- Les correspondances sans réponse sont archivées automatiquement après 30 jours (FR-012)
- L'archivage change l'état (`archivee`), ne supprime pas les données

**Nettoyage automatique** :
- Job SQL périodique ou vérification dans le handler de listing : `UPDATE retrouve_amis.correspondance SET etat = 'archivee' WHERE etat = 'en_attente' AND created_at < NOW() - INTERVAL '30 days'`
- Pour le MVP, vérification au moment du listing (lazy archival)

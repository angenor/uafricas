# Research: Partage Public des Avis de Recherche

**Branch**: `002-partage-avis-recherche` | **Date**: 2026-03-02

## R-001: Génération de slug pour les URLs publiques

**Contexte**: FR-002 exige un slug court, lisible et non prédictible pour les URLs publiques (`/retrouve-amis/public/{slug}`).

**Decision**: Utiliser le pattern existant `nom-recherche-{uuid8}`, slug basé sur le nom de la personne recherchée + suffixe UUID v4 tronqué (8 caractères).

**Rationale**:
- Le projet dispose déjà de 17 fonctions `generer_slug()` avec deux patterns : titre-based (modèles) et nom+UUID (auth.rs). Le pattern auth.rs avec UUID tronqué est le plus adapté car il garantit l'unicité sans collision et empêche l'énumération.
- Le slug reste lisible pour le partage social : `keita-fatou-a3f8b2c1` est compréhensible et mémorisable.
- VARCHAR(400) UNIQUE cohérent avec les autres tables du projet.

**Alternatives considérées**:
- nanoid/hashids : Ajouterait une dépendance externe inutile (Principe V YAGNI)
- UUID complet : Trop long, non lisible pour le partage social
- Slug titre seul : Risque de collision entre avis similaires (plusieurs personnes cherchant "Keita Fatou")

## R-002: Open Graph et Twitter Card pour le SSR

**Contexte**: FR-004 exige des balises OG et Twitter Card pour un aperçu riche lors du partage social.

**Decision**: Utiliser `useHead()` / `useSeoMeta()` de Nuxt 4 dans la page `[slug].vue` avec SSR activé (par défaut). Les balises seront rendues côté serveur pour être disponibles aux crawlers sociaux.

**Rationale**:
- Nuxt 4 SSR est déjà activé (configuration par défaut dans `nuxt.config.ts`). Les composables `useHead()` et `useSeoMeta()` sont natifs et gèrent le rendu côté serveur des meta tags.
- Les crawlers de Facebook, WhatsApp, Twitter ne peuvent pas exécuter JavaScript, le SSR est indispensable.
- Pas de dépendance supplémentaire nécessaire.

**Alternatives considérées**:
- Middleware serveur Nitro pour injecter les meta tags : Plus complexe, non nécessaire avec SSR
- Image OG dynamique générée côté serveur : Hors scope initial, image par défaut UAfricas suffisante (mentionné dans les Assumptions)

## R-003: Endpoints publics sans authentification

**Contexte**: Les pages publiques (page d'un avis, listing) doivent être accessibles sans JWT. Le backend actuel exige JWT pour tous les endpoints retrouve-amis.

**Decision**: Créer un nouveau fichier de handlers `retrouve_amis_public.rs` avec des endpoints publics enregistrés hors du scope JWT dans `routes.rs`.

**Rationale**:
- Séparation claire entre endpoints authentifiés et publics, cohérent avec le pattern existant (les routes auth vs protégées sont déjà séparées dans `routes.rs`).
- Les endpoints publics ne renvoient que des données anonymisées (FR-003, FR-008).
- Le fichier séparé évite de surcharger `retrouve_amis.rs` (déjà ~800 lignes).

**Alternatives considérées**:
- Ajouter un middleware conditionnel par route : Plus complexe, mélange auth/public dans le même fichier
- Utiliser un scope séparé dans le même fichier : Moins lisible pour la maintenance

## R-004: Compteur de partages (FR-005)

**Contexte**: Chaque clic sur un bouton de partage doit incrémenter un compteur persistant par avis.

**Decision**: Ajouter une colonne `compteur_partages INTEGER DEFAULT 0` sur `avis_recherche` + un endpoint `POST /api/retrouve-amis/public/{slug}/partage` (sans auth) qui incrémente atomiquement via `UPDATE ... SET compteur_partages = compteur_partages + 1`.

**Rationale**:
- Opération atomique SQL, pas de race condition.
- Pas de table séparée nécessaire : un simple compteur par avis suffit (pas besoin de tracer quel utilisateur a partagé, ni sur quel réseau). Le détail par réseau est hors scope (Principe V YAGNI).
- L'endpoint est public (pas de JWT) car les visiteurs non connectés partagent aussi.

**Alternatives considérées**:
- Table séparée `partage` avec détail (réseau, user, timestamp) : Sur-ingénierie pour le besoin actuel (SC-003 mesure uniquement le total)
- Compteur côté client uniquement (localStorage) : Non fiable, pas persistant, pas mesurable côté serveur
- Analytics externe (Google Analytics events) : Dépendance externe, pas de compteur visible sur la page

## R-005: Réponse publique et intégration au système de correspondances

**Contexte**: FR-006 exige que les réponses à un avis public créent une correspondance dans le système existant.

**Decision**: Créer une table `reponse_publique` qui stocke la réponse + créer automatiquement une `correspondance` de type `type_cible = 'profil'` avec un score fixe de 70 points (indicateur de réponse manuelle, pas de matching algorithmique).

**Rationale**:
- Réutilise le workflow d'acceptation mutuelle existant (en_attente → acceptee_a/b → mutuelle), pas de nouveau code pour la gestion des contacts.
- Le score de 70 est au-dessus du seuil de 60, mais clairement différent des correspondances algorithmiques (qui vont de 60 à 100 avec des détails_score granulaires).
- `details_score` JSONB stockera `{"source": "reponse_publique", "type_reponse": "je_suis_cette_personne"}` pour distinguer les réponses manuelles.
- La table `reponse_publique` garde la trace du type de réponse et du message original.

**Alternatives considérées**:
- Pas de table séparée, uniquement une correspondance : Perdrait le message et le type de réponse
- Nouveau type_cible 'reponse_publique' : Casserait l'enum existant et nécessiterait des modifications partout

## R-006: Demande de retrait avec arbitrage admin

**Contexte**: FR-010 exige un mécanisme de suspension immédiate + arbitrage admin sous 72h.

**Decision**: Créer une table `demande_retrait` avec les champs : id, avis_id, demandeur_id, motif, etat (en_attente/approuvee/rejetee), date_suspension, decision_admin_at. L'avis passe en `suspendu` immédiatement à la création de la demande.

**Rationale**:
- L'état `suspendu` existe déjà dans l'enum `etat_avis`, pas de nouvelle valeur d'enum nécessaire.
- La table séparée est nécessaire pour stocker le motif, le demandeur, et la décision admin (ces infos ne sont pas portées par l'avis lui-même).
- Le délai de 72h est indicatif pour les admins (pas de cron automatique), l'avis reste suspendu jusqu'à décision manuelle (conforme à la clarification 5b de la spec).

**Alternatives considérées**:
- Stocker la demande dans un champ JSONB de l'avis : Perd l'historique si plusieurs demandes, difficile à requêter
- Utiliser la table `signalement` existante avec un motif spécial : Sémantiquement différent (le demandeur est la personne concernée, pas un tiers)

## R-007: Pages publiques : Tailwind CSS v4 pur

**Contexte**: Constitution VI impose Tailwind CSS v4 pur pour le site public (pas de daisyUI).

**Decision**: Les pages publiques (`[slug].vue`, `rechercher.vue`) et leurs composants (`PagePublique.vue`, `BoutonsPartage.vue`, `CarteAvisPublic.vue`, etc.) utiliseront exclusivement les utility classes Tailwind CSS v4, aucune classe daisyUI (`btn`, `card`, `modal`, etc.).

**Rationale**:
- Conformité stricte avec la Constitution VI.
- Les pages publiques existantes du projet (retrouve-amis/index.vue, etc.) utilisent déjà Tailwind CSS v4 pur avec les couleurs custom (`custom-chocolat`, `custom-green`, `custom-gray`).
- Le design doit s'intégrer visuellement avec l'identité UAfricas existante.

**Alternatives considérées**: Aucune : c'est une contrainte constitutionnelle non négociable.

## R-008: Signalement : extension pour les visiteurs de pages publiques

**Contexte**: Le signalement existant exige une correspondance active entre le signaleur et l'avis (anti-spam). Les pages publiques permettent à des utilisateurs connectés sans correspondance de signaler.

**Decision**: Étendre la table `signalement` existante avec une colonne nullable `source` (enum : 'correspondance', 'page_publique') pour distinguer l'origine. Lever la contrainte "correspondance requise" uniquement pour les signalements issus de la page publique (vérification dans le handler).

**Rationale**:
- Réutilise la table et le workflow de modération existants (pas de duplication).
- La distinction `source` permet aux admins de comprendre le contexte du signalement.
- Le rate limit existant (unique par avis+utilisateur) s'applique toujours, pas de spam possible.

**Alternatives considérées**:
- Table séparée `signalement_public` : Duplication de la logique de modération
- Supprimer la contrainte de correspondance pour tous les signalements : Ouvrirait le spam sur les avis non publics

## R-009: Listing public et recherche full-text

**Contexte**: FR-014 exige une page publique de listing avec filtres (pays, ville, école) et recherche full-text.

**Decision**: Créer un endpoint public `GET /api/retrouve-amis/public/rechercher` avec pagination, filtres par pays_id/ville/ecole, et recherche full-text via le `search_vector` TSVECTOR+GIN déjà indexé sur `avis_recherche`.

**Rationale**:
- Le champ `search_vector` et l'index GIN existent déjà sur `avis_recherche`, zéro coût d'infrastructure.
- L'endpoint filtre uniquement les avis avec `est_public = TRUE AND etat = 'actif' AND deleted_at IS NULL`.
- La pagination suit le pattern existant (page, par_page, tri, ordre).

**Alternatives considérées**:
- Elasticsearch/Meilisearch : Sur-ingénierie pour le volume actuel (~500 avis), dépendance externe inutile
- Requête LIKE : Moins performant et moins pertinent que TSVECTOR+GIN pour le français

## R-010: Nouvel état `depublie` vs réutilisation de `est_public = FALSE`

**Contexte**: FR-011 mentionne un état "dépublié" pour les pages publiques. La question est de savoir s'il faut un nouvel état dans l'enum `etat_avis` ou un simple booléen.

**Decision**: Utiliser un booléen `est_public` (TRUE/FALSE) sur `avis_recherche` plutôt qu'un nouvel état d'enum. L'affichage de la page publique dépend de la combinaison `est_public + etat` :
- `est_public = TRUE, etat = actif` → contenu complet
- `est_public = TRUE, etat = cloture` → "personne retrouvée"
- `est_public = TRUE, etat = suspendu` → "avis temporairement retiré"
- `est_public = FALSE` (quelle que soit l'état) → "avis non disponible" (dépublié)

**Rationale**:
- Pas de modification de l'enum `etat_avis` existant, évite les migrations complexes et l'impact sur le code existant.
- La publication/dépublication est orthogonale à l'état de l'avis (un avis peut être actif mais privé, ou suspendu mais publiquement marqué comme tel).
- Plus flexible pour l'avenir.

**Alternatives considérées**:
- Ajouter 'depublie' à l'enum : Casserait la logique existante (actif/cloture/suspendu) qui ne concerne pas la visibilité publique
- Utiliser un état 'public_actif' / 'public_suspendu' : Explosion combinatoire des états

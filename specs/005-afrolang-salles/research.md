# Phase 0 : Research & Decisions

**Feature**: Afrolang : Ajustements salles publiques et privées
**Branch**: `005-afrolang-salles`
**Date**: 2026-04-14

Toutes les « NEEDS CLARIFICATION » du spec ont été levées pendant `/speckit.clarify` (session 2026-04-14). Cette phase consolide les décisions techniques qui en découlent et examine les patterns à réutiliser.

---

## Décision 1 : Référentiel unique des groupes ethniques

- **Décision** : réutiliser la table existante `country_profile.groupe_ethnique` comme source unique de vérité, référencée par FK depuis `afrolang.salle`.
- **Rationale** : évite la duplication, cohérence garantie avec les fiches pays, maintenance centralisée. Décision tranchée en clarification Q1 (option A).
- **Alternatives rejetées** :
  - Créer une table parallèle `afrolang.groupe_ethnique` : doublon coûteux à maintenir, risque de divergence.
  - Solution hybride (référentiel + table de complément) : complexité non justifiée au stade actuel ; l'enrichissement pédagogique (langue cible, alphabet) peut être porté directement par la salle publique.
- **Impact** : ajout d'une colonne `groupe_ethnique_id UUID NOT NULL REFERENCES country_profile.groupe_ethnique(id) ON DELETE RESTRICT` sur `afrolang.salle` ; un index unique partiel garantit au plus une salle active par groupe ethnique (`WHERE actif = TRUE`). La FK `ON DELETE RESTRICT` protège contre la suppression accidentelle d'un groupe référencé.

---

## Décision 2 : Workflow de proposition de salle publique

- **Décision** : nouvelle table `afrolang.proposition_salle` avec enum d'état `en_attente | approuvee | refusee`, handler public pour soumission + suivi, handler admin pour validation/refus avec motif.
- **Rationale** : le pattern existe déjà dans `country_profile.etat_contribution` ; on l'applique localement à Afrolang plutôt que de rattacher au module contributions existant (qui est scopé fiches pays, éviter le couplage hors contexte).
- **Alternatives rejetées** :
  - Créer directement la salle avec `actif=false` et flag `en_attente_validation` : complexifie les requêtes de listing et risque d'exposer des salles non validées par oubli.
  - Stocker la proposition dans le module `contributions_fiche` existant : mauvais bounded context, couplage fort avec le workflow fiche pays.
- **Détection de doublon** : avant insertion, vérifier par `(nom_groupe_ethnique, pays_id)` normalisé (trim + lower + retrait des accents via fonction immutable) contre à la fois les salles existantes et les propositions `en_attente`. Retourne erreur 409 avec pointeur vers la ressource.

---

## Décision 3 : Modération double (Afrolang attitrés + session dynamique)

- **Décision** : 
  - Table d'affectation **many-to-many** `afrolang.salle_moderateur` (une salle peut avoir plusieurs modérateurs Afrolang attitrés, un utilisateur peut être modérateur sur plusieurs salles, cohérent avec FR-008 « un ou plusieurs modérateurs »).
  - Le champ existant `afrolang.salle.moderateur_id` (scalaire) est **déprécié** et supprimé : remplacé par la table d'affectation qui porte aussi la disponibilité et l'horodatage de désignation.
  - Le modérateur **effectif** d'une session reste sur `afrolang.session.moderateur_id`. Les transitions (premier arrivé → transfert manuel → reprise automatique par un Afrolang attitré → réattribution au départ) sont gérées côté serveur via un handler dédié `PUT /sessions/{id}/moderation/transferer` et un déclencheur automatique lors de `rejoindre_session` / `quitter_session`.
  - **Règle de départage quand plusieurs Afrolang attitrés sont présents simultanément** : le modérateur actif est le **premier attitré arrivé dans la session** (`MIN(session_participant.rejoint_at)` parmi les `salle_moderateur.actif=TRUE`). En cas d'arrivée strictement simultanée (timestamps égaux), départage par `salle_moderateur.designe_at` croissant (le plus ancien attitré gagne). Les autres attitrés présents disposent des **mêmes droits fonctionnels** (effacement tableau blanc, modération chat, etc.) mais sans détenir le rôle « actif » au sens `session.moderateur_id`. Cette règle déterministe évite tout conflit (Edge Case « deux attitrés simultanés »).
  - **Déconnexion ≡ quitter** (Edge Case « modérateur perd sa connexion ») : une déconnexion détectée par le SFU (LiveKit) déclenche le même chemin que `quitter_session` (`UPDATE session_participant.quitte_at = NOW()`). À la reconnexion, le membre repasse par `rejoindre_session` qui ré-applique les règles FR-009/FR-011 (reprise automatique du rôle s'il est attitré).
- **Rationale** : la modération effective en session est dérivée d'une règle déterministe serveur, évitant les conditions de course côté client. La table d'affectation sépare « qui est habilité » (statique, géré par l'admin) de « qui modère maintenant » (dynamique, géré par le serveur).
- **Alternatives rejetées** :
  - Garder `salle.moderateur_id` scalaire + table secondaire : double source de vérité, risque de désynchronisation.
  - Client décide qui est modérateur (broadcast « je suis modérateur ») : non sûr, exploitable.

---

## Décision 4 : Enrichissement de `afrolang.salle_privee`

- **Décision** : ajouter sur la table existante les colonnes :
  - `motif` enum `motif_salle_privee` ∈ {`apprentissage_enfants`, `reseautage_adulte`, `echanges_groupe`}, NOT NULL.
  - `declaration_adulte_at` TIMESTAMPTZ NOT NULL (capture horodatée de la déclaration, FR-033).
  - `visibilite` enum `visibilite_salle_privee` ∈ {`fermee`, `visible`}, NOT NULL DEFAULT `fermee`.
  - `archivee_at` TIMESTAMPTZ NULL (archivage par le système, FR-034, séparé de `deleted_at` qui reste le soft-delete classique).
- **Contrainte d'unicité métier** « 1 salle privée active par membre par salle publique » : `CREATE UNIQUE INDEX ... ON salle_privee(salle_id, cree_par) WHERE archivee_at IS NULL AND deleted_at IS NULL`. Cette contrainte est atomique et empêche la condition de course même en cas de doubles clics.
- **Rationale** : enrichissement en place plutôt que table satellite car le nombre de colonnes reste raisonnable et les requêtes de listing bénéficient de l'absence de jointure. L'index unique partiel est le pattern PostgreSQL idiomatique pour une unicité conditionnelle.
- **Alternatives rejetées** :
  - Table `salle_privee_metadata` séparée : complexité d'accès supplémentaire, aucun bénéfice.
  - Vérification applicative de l'unicité : non sûre sous concurrence.

---

## Décision 5 : Adhésions et invitations

- **Décision** : une seule table `afrolang.salle_privee_adhesion` couvre :
  - les **demandes d'adhésion** (émises par un membre vers une salle `visible`),
  - les **invitations directes** (émises par le créateur vers un membre, pour les salles `fermee` ou `visible`),
  - les **abonnés confirmés** (acceptation d'une invitation ou d'une demande).
  - Colonnes : `id`, `salle_privee_id`, `utilisateur_id`, `type` enum `type_adhesion` ∈ {`demande`, `invitation`, `abonne`}, `etat` enum `etat_adhesion` ∈ {`en_attente`, `acceptee`, `refusee`, `groupe_complet`}, `initiateur_id`, `decideur_id`, `created_at`, `decided_at`, `deleted_at`.
  - Contrainte UNIQUE `(salle_privee_id, utilisateur_id)` (un membre ne peut avoir qu'une ligne par salle ; les refus sont conservés pour historique mais le membre peut les réactiver en soumettant à nouveau : voir transition).
- **Rationale** : un seul modèle de flux pour toutes les formes d'adhésion simplifie le code handler et les requêtes de tableau de bord du créateur. Les transitions d'état couvrent tous les scénarios du spec (User Story 5).
- **Alternatives rejetées** :
  - Deux tables séparées (`demandes` et `invitations`) : duplication de schéma et de code, jointures plus complexes pour l'affichage « membres de la salle ».
  - Polymorphisme avec une colonne JSONB : moins requêtable, moins typé.

### Transitions d'état

```
demande      : en_attente → acceptee | refusee | groupe_complet
invitation   : en_attente → acceptee | refusee
(sur acceptee) : type devient abonne (UPDATE du type)
```

Refus automatique « groupe_complet » : calculé au moment de l'insertion de la demande, avant l'appel côté créateur, en comparant `COUNT(abonne)` à `max_participants`.

---

## Décision 6 : Tableau blanc temps réel

- **Décision** : la table `afrolang.tableau_blanc` est conservée pour la **persistance** (snapshot JSONB + version). Le **temps réel** passe par le **canal data** LiveKit déjà configuré (via `livekit-api` côté serveur pour la génération de token, et LiveKit client côté Nuxt). Chaque opération (trait, forme, texte, effacement) est diffusée comme message `DataPacket.Kind.RELIABLE` à tous les participants. Une sauvegarde serveur s'opère :
  - toutes les N secondes (ex. 10 s) pendant la session (throttle côté frontend, appel `PUT /sessions/{id}/tableau-blanc`) ;
  - à la fermeture de la session (`PUT /sessions/{id}/terminer` déclenche une ultime sauvegarde avant transition `etat=terminee`).
- **Rationale** : LiveKit est déjà en place et porte la visio + le data channel. Pas de nouvelle brique (SimplePeer, Yjs, Liveblocks…) à intégrer. Pattern utilisé par Whereby, Meet, etc.
- **Alternatives rejetées** :
  - CRDT (Yjs / Automerge) : puissant mais surdimensionné pour un tableau blanc non concurrent sur la même zone ; ajoute une dépendance non justifiée.
  - WebSocket maison : LiveKit fournit déjà un canal fiable ; redondant.

---

## Décision 7 : Messagerie instantanée écrite

- **Décision** : table `afrolang.message_session` (persistance ≠ signaling), diffusion temps réel via LiveKit data channel. Chaque message est publié par l'émetteur ET inséré en base via `POST /sessions/{id}/messages` pour l'archivage pédagogique (hypothèse explicite du spec). Les autres participants reçoivent le message en temps réel via le canal data ; la BDD sert de fallback / reprise si un participant rejoint la session en retard.
- **Rationale** : pattern hybride « broadcast + persist » classique pour chat de session. Permet de récupérer l'historique lors d'un reconnect.
- **Alternatives rejetées** :
  - Persistance uniquement, affichage par polling : latence inacceptable (FR-029 exige quasi-temps réel).
  - Temps réel uniquement sans persistance : contredit l'hypothèse d'archivage pédagogique.

---

## Décision 8 : Ressources : fichiers internes vs liens externes

- **Décision** : table `afrolang.ressource_salle` avec :
  - `type` enum `type_ressource` ∈ {`fichier`, `lien_externe`}.
  - `fichier_url` VARCHAR NULL (pour `fichier`, stockage local `./uploads/afrolang/ressources/`).
  - `lien_url` VARCHAR NULL (pour `lien_externe`).
  - `etat` enum `etat_ressource` ∈ {`publiee`, `en_attente_validation`, `refusee`}.
  - Règle serveur (CHECK + logique handler) :
    - `type=fichier` ⇒ `etat=publiee` dès l'insertion (FR-028 : fichiers publiables immédiatement) + `fichier_url` NOT NULL.
    - `type=lien_externe` ⇒ `etat=en_attente_validation` à l'insertion + `lien_url` NOT NULL ; bascule en `publiee` uniquement via endpoint admin.
- **Upload** : réutilise le pattern `actix-multipart` + `sanitize-filename` déjà en place pour les autres domaines (vidafrica, livres).
- **Validation d'URL** : `lien_url` validée côté serveur (schéma `http`/`https`, longueur max, absence de caractères de contrôle) avant insertion.
- **Rationale** : cohérent avec le modèle existant de modération (patterns `projets`, `innovations`, `africantives` du spec CLAUDE.md). Respecte la décision de clarification Q4.
- **Alternatives rejetées** :
  - Deux tables séparées (`ressource_fichier`, `ressource_lien`) : duplication, requêtes de listing plus complexes.
  - Modération systématique (fichiers compris) : ralentit l'ajout par les modérateurs eux-mêmes, non demandé par le spec.

---

## Décision 9 : Archivage automatique de la salle privée

- **Décision** : lorsqu'un utilisateur est soft-deleted (`iam.utilisateur.deleted_at IS NOT NULL`) ou désactivé (`iam.utilisateur.etat = 'desactive'`), un handler admin / un job batch parcourt `afrolang.salle_privee WHERE cree_par = ... AND archivee_at IS NULL AND deleted_at IS NULL` et applique `UPDATE ... SET archivee_at = NOW()`. Les participants sont notifiés via `iam.notification` (canal existant, cf. CLAUDE.md). L'opération est instrumentée avec `audit::log_action`.
- **Rationale** : soft archivage non destructif, conforme à la Constitution (Principe III : soft deletion). Les salles archivées restent interrogeables pour l'historique mais plus démarrables.
- **Alternatives rejetées** :
  - Suppression dure (`DELETE`) : perte de données, rupture conversationnelle pour les participants.
  - Transfert automatique de propriété : introduit une question de consentement du repreneur non traitée par le spec ; on préfère l'archivage simple et explicite.

---

## Décision 10 : Notifications

- **Décision** : réutiliser le module `iam.notification` existant (évoqué dans CLAUDE.md et les handlers `notification.rs`). Ajouter les types de notification Afrolang :
  - `afrolang.proposition_validee`, `afrolang.proposition_refusee`.
  - `afrolang.moderation_reprise` (modérateur de session évincé par un attitré).
  - `afrolang.adhesion_demandee`, `afrolang.adhesion_acceptee`, `afrolang.adhesion_refusee`, `afrolang.adhesion_groupe_complet`.
  - `afrolang.invitation_recue`, `afrolang.invitation_refusee`.
  - `afrolang.salle_privee_archivee`.
- **Rationale** : canal unique, pas de silo. Hypothèse documentée du spec.
- **Alternatives rejetées** : canal dédié Afrolang → over-engineering.

---

## Décision 11 : Audit

- **Décision** : chaque mutation admin (validation / refus proposition, désignation / retrait de modérateur Afrolang attitré, validation / refus lien externe, archivage manuel d'une salle privée) utilise `audit::log_action` non-bloquant avec `before / after` JSONB. Conforme au Principe VII de la Constitution et au pattern existant (~100 mutations instrumentées).
- **Rationale** : obligation constitutionnelle ; le service existe déjà.

---

## Décision 12 : Front-office vs Back-office (contrainte UI)

- **Décision** : 
  - Pages publiques sous `/afrolang/**` : Tailwind v4 pur (constitution VI). Les nouveaux composants publics (`SalleChat`, `SalleRessources`, `ProposerSalleModal`, etc.) n'utilisent **aucune** classe daisyUI (`btn`, `card`, `modal`, `drawer`, `alert`, etc.).
  - Pages admin sous `/admin/afrolang/**` : daisyUI v5 autorisé et recommandé. Les composants `ValidationPropositionsList`, `ModerateursAttitresPanel`, `LiensExternesValidation` peuvent utiliser daisyUI.
- **Rationale** : contrainte constitutionnelle stricte. L'auto-memory du projet rappelle ce point (feedback `no_daisyui_arbre` déjà documenté pour un autre feature).

---

## Récapitulatif : Résolution des NEEDS CLARIFICATION

| Zone | Statut |
|------|--------|
| Référentiel des groupes ethniques (Q1) | ✅ Résolu, `country_profile.groupe_ethnique` |
| Créateur quitte la plateforme (Q2) | ✅ Résolu, archivage automatique |
| UX « Créer salle privée » (Q3) | ✅ Résolu, bouton permanent + info-bulle 1ère visite |
| Types de ressources (Q4) | ✅ Résolu, fichier (direct) + lien externe (modéré) |
| Limite salles privées par membre (Q5) | ✅ Résolu, 1 par (membre × salle publique), index unique partiel |

Aucun point de clarification restant.

---

## Patterns réutilisés depuis le monorepo

- **`ApiResponse<T>` wrapper**, **`COLONNES` const**, **`FromRow` structs**, **DTO Request/Response séparés** (pattern général backend UAfricas).
- **Enums PostgreSQL** côté schéma, **enums Rust `#[derive(sqlx::Type)]`** côté modèle.
- **Pagination + filtres** via les helpers communs (`models/pagination.rs`).
- **Soft deletion** (`deleted_at`) + **audit** (`audit::log_action`) systématiques.
- **Upload multipart** : même pipeline que `vidafrica`, `livres`, `codimoi` (sanitize-filename, extension whitelist, stockage sous `./uploads/afrolang/...`).
- **Composable public** `useAfrolang.ts` + **composable admin** `useAdminAfrolangSalles.ts` suivent la convention UAfricas (Public = `useXxx`, Admin = `useAdminXxx` bâti sur `useAdmin`).
- **Routing Nuxt 4** file-based, layouts `default.vue` / `admin`, patterns Hero/Card/Filters/Modal.

---

## Risques techniques identifiés

| Risque | Probabilité | Mitigation |
|--------|-------------|------------|
| Condition de course sur la contrainte « 1 salle privée par (membre × salle publique) » | Faible | Unique index partiel PostgreSQL garantit l'atomicité au niveau BDD (pas de check applicatif seul). |
| Condition de course sur la limite de participants | Moyenne | Transaction SQL + `SELECT ... FOR UPDATE` sur la salle privée avant insertion d'un abonne + comparaison à `max_participants`. Couvert par SC-006. |
| Synchronisation tableau blanc : flicker si plusieurs participants dessinent simultanément | Moyenne | Throttle côté client (16 ms pour les traits) + reconnect reprend depuis le snapshot BDD ; pas de CRDT nécessaire à ce stade. |
| Volume de messages archivés en base | Faible | Pagination + index sur `(session_id, created_at)` ; rotation à étudier post-lancement. |
| Utilisateurs déclarant faussement leur âge adulte | Connu | Traçabilité via `declaration_adulte_at` + notice d'alerte pour motif enfants ; responsabilité couverte au niveau fonctionnel. |
| Salles publiques archivées avec salles privées actives rattachées | Faible | FK `ON DELETE RESTRICT` sur `salle_privee.salle_id` + traitement explicite lors de la désactivation d'une salle publique (notification puis archivage en cascade). |

---

## Statut Phase 0

**Complet** : Toutes les décisions techniques sont prises, aucun NEEDS CLARIFICATION restant. Prêt pour Phase 1.

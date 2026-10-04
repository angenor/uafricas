# Implementation Plan: Activités interactives, ludiques et participatives

**Branch**: `013-activites-ludiques` | **Date**: 2026-10-01 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/013-activites-ludiques/spec.md`

## Summary

Un socle ludique unique, branché sur quatre modules pilotes, livré en cross-stack.

1. **Un nouveau domaine, `jeu`.** Dix tables et une vue dans un schéma dédié (`37_jeu.sql`) : modules, épreuves, parties, réponses, journal des gains et ses deux agrégats, défis, duels, saisons, signalements, règles. Rien n'est modifié dans les schémas existants, hormis des **lignes** ajoutées au barème d'engagement.

2. **Un seul déroulé pour trois cadres.** Présenter une épreuve, recevoir une réponse, corriger : la partie libre, le défi et le duel différé passent par les mêmes cinq routes et le même écran. Seuls changent la composition de la série et ce que le résultat rapporte. Une participation à un défi **est** une partie ; il n'y a pas de table de participation (research D9).

3. **Le score a son propre journal.** `jeu.gain` fait foi (idempotence par clé unique, pays et saison figés à l'écriture) ; `jeu.score_saison` et `jeu.joueur` sont tenus dans la même transaction pour que les classements se lisent en une requête (D2). Le jeu ne crédite **aucun** point d'engagement.

4. **Réputation et distinctions réutilisent le moteur d'engagement sans le modifier** : six règles à `points = 0` portent un `reputation_delta` réglable, et cinq badges s'appuient sur la condition `actions_comptees` qui existe (D3).

5. **La dérivation est un catalogue de formes, gardé par une vue.** Onze formes de question tirent des candidates du contenu publié de trois modules ; `jeu.v_source` dit à chaque tirage si la source est encore publiée et inchangée. Afrolang, qui n'a aucun référentiel de langues, est en saisie pure (D5).

6. **Rien ne tourne en tâche de fond.** Défi du jour, échéances de duel, avancement d'un duel direct, clôture de saison : tout se résout à la lecture, sous verrou (D6).

7. **Le duel direct ne crée aucun canal.** Il publie des signaux sur le flux SSE de la messagerie et s'appuie sur un état serveur autoritaire que le client relit toutes les 3 secondes (D1). C'est le seul morceau de la feature qui synchronise deux écrans ; il est livrable en dernier sans rien changer au reste.

Constat à garder en tête : en dehors d'Afripulse, **le contenu dont on peut tirer des épreuves est presque inexistant dans les seeds** (un proverbe, deux factchecks). Le volume réel dépend de la production et se mesure avant l'ouverture, pas pendant le développement (D14).

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) · TypeScript 5 / Vue 3 SSR / Nuxt 4 (frontend)

**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL, requêtes **runtime** `query_as`), uuid, chrono, serde, rand, futures-util/tokio (SSE) · Pinia, Tailwind CSS v4, daisyUI v5 (back-office), FontAwesome, `@svg-maps/world` (carte, déjà présent). **Aucune dépendance nouvelle, ni côté Cargo ni côté pnpm.**

**Storage**: PostgreSQL 16, **nouveau schéma `jeu`**. Une migration additive : `uafricas_backend/doc/bd/schemas/37_jeu.sql` (dernière en place : `36_social_africanite.sql`), incluse dans `schema.sql` en phase 5 après le bloc `35*`. PostgreSQL 16 requis pour `UNIQUE NULLS NOT DISTINCT` (déjà exigé par `35c`). Médias des épreuves sous `./uploads/`, par les routes d'upload du back-office existantes.

**Testing**: aucun harnais de test n'est configuré sur le projet (contrainte constitutionnelle assumée). Validation manuelle scénarisée : [quickstart.md](./quickstart.md), huit scénarios et une table de critères de sortie.

**Target Platform**: application web SSR (Nuxt 4) servie par un backend Actix-Web **mono-instance** ; PostgreSQL 16 en Docker.

**Project Type**: monorepo web : `uafricas_frontend/` + `uafricas_backend/`.

**Performance Goals**:
- Classements en moins de 2 s à 10 000 membres classés (SC-011) : lecture de `score_saison`, jamais d'agrégation du journal.
- Tirage d'une série en **une** requête (épreuves servables jointes à `v_source`, anti-jointure sur les réponses du membre), sans requête par épreuve.
- Duel direct : moins d'une seconde d'écart d'affichage entre les deux joueurs (SC-010), obtenu par un instant de début fixé côté serveur, pas par la vitesse du signal.

**Constraints**:
- **Ordre de déclaration des routes actix** : tout segment littéral avant le motif paramétré de même profondeur. Cas concrets listés en tête de chaque contrat. Déjà cause de deux 404 à la recette de la feature 009.
- **sqlx vérifié à l'exécution** : une colonne oubliée ne casse pas la compilation. Chaque requête doit être exécutée au moins une fois au recettage.
- **`web::Query` refuse une clé répétée** : toute liste en paramètre passe par des valeurs séparées par des virgules.
- **Le flux SSE n'a pas de tampon** et le registre est en mémoire : aucun écran ne doit dépendre de la réception d'un signal (D1). Corollaire : le duel direct suppose un backend mono-instance, ce qui est le cas en production.
- **Aucun garde commun côté membre** : le JWT ne porte que `sub`. L'état du compte se vérifie en base à chaque route de jeu (D11).
- **`engagement.mouvement_points.cle_idempotence` est unique globalement** : toute clé émise par le jeu porte le bénéficiaire (D3).
- **Principe VI** : pages publiques en Tailwind v4 pur sur le gabarit `africans` (`definePageMeta({ layout: false })` + `<NuxtLayout name="africans">`) ; daisyUI dans `/admin/**` seulement.
- **Principe VII** : les 15 mutations des 23 routes d'administration passent toutes par `audit::log_action`.
- **Aucune tâche de fond** (D6).

**Scale/Scope**: 1 migration SQL + 1 seed local · 2 services Rust neufs, 5 handlers neufs, 2 modules de modèles · **49 routes neuves** (5 publiques, 21 membre, 23 admin) · 3 composables publics + 1 admin · environ 16 composants Vue neufs, 1 déplacé et généralisé · 8 pages publiques + 7 pages d'administration · 6 points de greffe dans l'existant (4 pages de module, plugin SSE, messagerie flottante).

## Constitution Check

*GATE : évalué avant Phase 0, réévalué après Phase 1.*

| Principe | Verdict | Justification |
|---|---|---|
| **I. Français d'abord** | ✅ | Schéma `jeu` ; tables `epreuve`, `partie`, `reponse`, `gain`, `defi`, `duel`, `saison` ; colonnes `enonce`, `propositions`, `bonne_reponse`, `cle_idempotence` ; services `jeu`, `jeu_derivation` ; composants `Jeu…`. « Championship » est le nom du produit donné par le client, employé à l'écran seulement. |
| **II. Monorepo cohérent** | ✅ | SQL → Rust → TypeScript livrés ensemble ; correspondance des types en [data-model.md §15](./data-model.md). |
| **III. SQL source de vérité** | ✅ | `37_jeu.sql` précède tout code. UUID v4, TIMESTAMPTZ, snake_case français. Trois écarts aux conventions, justifiés : `CHECK` plutôt qu'enums (D12, comme `36`) ; pas de `deleted_at` sur des tables qui sont des journaux ou des états (une épreuve se **retire**, elle ne se supprime pas) ; clé naturelle `code` sur `jeu.module`. |
| **IV. Sécurité par défaut** | ✅ | La bonne réponse n'existe dans aucun type sérialisé avant la réponse (`EpreuveServie` ≠ `Correction`) ; temps mesuré par le serveur ; état du compte vérifié en base ; binds paramétrés ; classements publics limités au nom, au pays, au statut et au score. |
| **V. Simplicité (YAGNI)** | ⚠️ justifié | Deux agrégats en plus du journal, et un composant de carte généralisé : voir Complexity Tracking. En contrepartie : aucun trait, aucun dépôt d'abstraction, aucune table de participation, aucun canal temps réel neuf, aucune route d'upload neuve. |
| **VI. Tailwind v4 (daisyUI back-office seul)** | ✅ | Les composants `jeu/` sont en Tailwind pur avec les jetons `af-*` ; daisyUI n'apparaît que sous `/admin/activites/**`. |
| **VII. Audit & traçabilité** | ✅ | Toute mutation d'administration est journalisée ([api-admin.md](./contracts/api-admin.md)). Les mutations des membres (réponses, duels) ne le sont pas : `jeu.reponse` et `jeu.gain` **sont** leur journal, et les y doubler écrirait dix lignes d'audit par partie. |

**Verdict** : aucune violation bloquante. Le schéma neuf est justifié au sens des Contraintes Techniques (« tout nouveau domaine doit être rattaché à un schéma existant ou en créer un nouveau avec justification ») : dix tables dont aucune n'appartient à un domaine existant, et qui référencent quatre schémas sans dépendre d'aucun.

### Réévaluation après conception (Phase 1)

| Principe | Verdict | Ce que la conception a confirmé ou déplacé |
|---|---|---|
| III. SQL source de vérité | ✅ | Les règles qui auraient pu rester en Rust sont en SQL : une seule bonne réponse par construction (D4) ; une participation par défi, un seul défi par période, pas de doublon de dérivation (index uniques) ; pas de chevauchement de saisons (contrainte d'exclusion) ; une épreuve jouable a une explication (CHECK). |
| IV. Sécurité | ✅ | Trois portes fermées en conception : demander la suivante **enregistre** l'absence de réponse (recharger ne permet pas de revoir une question) ; en duel direct la correction n'arrive qu'en phase de révélation, aux deux à la fois ; « épreuve injouable » consomme l'épreuve sans rien rapporter. Limite assumée et écrite : on ne peut pas empêcher de chercher la réponse ailleurs (D8). |
| V. Simplicité | ⚠️ inchangé | La conception a **réduit** la surface : 11 entités dans la spec, 10 tables ; réputation et badges sans une ligne dans le moteur d'engagement ; duel direct sans canal ni dépendance. Les deux déviations du tableau ci-dessous restent les seules. |
| VII. Audit | ✅ | Une ligne d'audit par opération, pas par épreuve : la dérivation de 250 candidates et la revue d'un lot de 30 en écrivent une chacune, avec le décompte. |

**Points de conception à ne pas perdre de vue en Phase 2**

1. **`resoudre_duel` doit être appelée partout où un duel est lu.** L'oublier sur une route ne casse rien de visible : le duel reste simplement dans son dernier état, et une prime de forfait n'est jamais versée. Le quickstart teste la liste **et** le détail.
2. **La définition d'« épreuve servable » ne doit exister qu'une fois** (un fragment SQL partagé par le tirage libre, le défi, le duel et la carte). Dupliquée, elle divergerait et une épreuve dont la source est dépubliée finirait servie par l'un des quatre chemins.
3. **Le déplacement de la carte touche Africonnect** : un seul point d'appel, mais c'est une page en production. Le quickstart en fait un critère de non-régression.
4. **Le badge se fige si sa règle est désactivée** (D3) : à dire dans l'écran de barème, sinon un administrateur désactivera une règle « à 0 point » en la croyant inutile.

## Project Structure

### Documentation (this feature)

```text
specs/013-activites-ludiques/
├── plan.md              # Ce fichier
├── research.md          # Phase 0 : 14 décisions de conception
├── data-model.md        # Phase 1 : 10 tables, 1 vue, transitions, correspondance des types
├── quickstart.md        # Phase 1 : 8 scénarios de recette, critères de sortie, ouverture en production
├── contracts/
│   ├── api-membre.md    # 26 routes publiques et membre
│   ├── api-admin.md     # 23 routes d'administration
│   └── temps-reel.md    # Signaux SSE et notifications
├── checklists/
│   └── requirements.md  # Écrit par /speckit-specify
├── spec.md
└── tasks.md             # Phase 2 (/speckit-tasks, pas créé par /speckit-plan)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/
│   ├── schema.sql                               # + \ir schemas/37_jeu.sql (phase 5, après 35g)
│   ├── schemas/37_jeu.sql                       # NEUF : schéma, 10 tables, vue, règles, badges, permission
│   └── seeds-locaux/95_seed_local_jeu.sql       # NEUF : contenu de recette, hors schemas/, jamais déployé
└── src/
    ├── models/
    │   ├── jeu.rs                               # NEUF : DTO publics, EpreuveServie ≠ Correction, evt_duel_*
    │   ├── admin/jeu.rs                         # NEUF : requêtes et lignes d'administration
    │   └── notification.rs                      # + pub mod jeu { … }
    ├── services/
    │   ├── jeu.rs                               # NEUF : servable, composer_serie, enregistrer_reponse,
    │   │                                        #        crediter, defi_courant, resoudre_duel, cloturer_saison
    │   └── jeu_derivation.rs                    # NEUF : catalogue FORMES, une fonction par forme
    ├── handlers/
    │   ├── jeu.rs                               # NEUF : garde_joueur, modules, mon jeu, parties, signalement
    │   ├── jeu_defi.rs                          # NEUF
    │   ├── jeu_duel.rs                          # NEUF : différé et direct
    │   ├── jeu_classement.rs                    # NEUF : saisons, classements, fiche pays
    │   └── admin/
    │       ├── jeu.rs                           # NEUF : modules, épreuves, revue, règles, saisons, défis,
    │       │                                    #        signalements, annulation de gains
    │       └── engagement.rs                    # + 6 type_action dans ACTIONS_INSTRUMENTEES
    └── routes.rs                                # + scope /jeu, + routes /admin/jeu/*

uafricas_frontend/app/
├── composables/
│   ├── useJeu.ts                                # NEUF : modules, parties, défis, mon jeu, appelAuth (rejeu sur 401)
│   ├── useDuels.ts                              # NEUF : duels, état d'invitation, gererEvenement (SSE)
│   ├── useChampionship.ts                       # NEUF : saisons, classements, fiche pays
│   └── useAdminJeu.ts                           # NEUF : bâti sur useAdmin
├── components/
│   ├── jeu/                                     # NEUF, Tailwind pur
│   │   ├── PanneauActivites.vue                 #   bloc du rail des pages de module
│   │   ├── CarteModule.vue, CarteDefi.vue
│   │   ├── CarteEpreuve.vue, Minuteur.vue, Correction.vue, BilanPartie.vue
│   │   ├── ListeDuels.vue, ProposerDuelModal.vue, InvitationDuelPrompt.vue, DuelDirectSalle.vue
│   │   ├── TableauClassement.vue, FichePaysClassement.vue
│   │   └── ResumeJoueur.vue
│   ├── common/CarteAfriqueValeurs.vue           # DÉPLACÉ depuis retrouve-amis/CarteAfrique.vue, + 3 props
│   ├── admin/jeu/EpreuveFormulaire.vue          # NEUF, daisyUI
│   ├── social/MessagerieFlottante.vue           # + <JeuInvitationDuelPrompt>
│   └── admin/AdminSidebar.vue                   # + section « Activités »
├── pages/
│   ├── activites/
│   │   ├── index.vue                            # hub : défis, modules, mes duels, aperçu du classement
│   │   ├── [module].vue                         # espace d'un module
│   │   ├── partie/[id].vue                      # l'écran de jeu, commun aux trois cadres
│   │   ├── defis/[id].vue                       # résultats d'un défi
│   │   ├── duels/index.vue, duels/[id].vue      # mes duels ; un duel, différé ou direct
│   │   └── championship.vue                     # classements, bascule Classement / Carte
│   ├── mon-compte/activites.vue                 # score, répartition, historique
│   ├── admin/activites/
│   │   ├── epreuves/index.vue, epreuves/[id].vue
│   │   ├── revue.vue, regles.vue, saisons.vue, defis.vue, signalements.vue
│   ├── codi-moi/index.vue                       # + <JeuPanneauActivites module="codimoi"> dans #rail
│   ├── afrolang/index.vue                       # + idem
│   ├── opportunite-afrique/index.vue            # + idem
│   ├── universite/gouvernance/factcheck.vue     # + idem
│   ├── retrouve-amis/index.vue                  # ~ nouveau nom du composant de carte
│   └── notifications.vue                        # + types jeu.*
├── plugins/
│   ├── messagerie.client.ts                     # + branche duel_*
│   └── fontawesome.ts                           # + gamepad, ranking-star
├── utils/
│   ├── navigation-modules.ts                    # + entrée « Activités »
│   └── navigation-compte.ts                     # + « Mes activités »
└── mocks/notifications.ts                       # + types jeu.* (icône, couleur)

CLAUDE.md                                        # + schéma jeu, composables, ligne d'index
```

**Structure Decision**: monorepo web existant, sans dossier de premier niveau neuf. Côté backend, un fichier de handler par famille de routes (le patron « un fichier par domaine » éclaté en cinq parce que le domaine porte 49 routes ; `afrolang.rs`, à plus de 5 000 lignes, montre ce que coûte un fichier unique) et deux services, le moteur et la dérivation, qui sont les seuls endroits où vit de la logique partagée. Côté frontend, un dossier de composants `jeu/` (auto-import `Jeu…`), des pages sous `/activites`, et l'administration sous `/admin/activites`. Le mot affiché au membre est « Activités » ; le nom technique du domaine est `jeu`, plus court et sans ambiguïté avec les « activités » déjà présentes dans le vocabulaire du site.

### Ordre de livraison

Chaque palier est démontrable seul et correspond aux priorités de la spec.

| Palier | Contenu | Histoires |
|---|---|---|
| **1. Socle** | migration, `services/jeu` (servable, série, réponse, gain), parties, back-office des épreuves, panneau dans les 4 modules | US1, US2 (P1) |
| **2. Vivier** | dérivation, revue, vue `v_source`, signalement d'épreuve | US8 (P2) |
| **3. Retour** | défis du jour et de la semaine, série de jours | US3 (P2) |
| **4. Collectif** | saisons, classements, `/mon-compte/activites` | US5 (P2) |
| **5. Social** | duel différé, notifications, plafonds | US4 (P2) |
| **6. Lecture** | carte interactive, réputation et distinctions | US6, US9 (P3) |
| **7. Direct** | duel direct, signaux SSE, invitation | US7 (P3) |

Le palier 6 pourrait remonter : les règles et badges sont dans la migration du palier 1, seuls leurs appels arrivent avec les défis et les duels. Le palier 7 est le seul dont le risque n'est pas maîtrisé d'avance ; il est en dernier pour cette raison.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Deux agrégats (`score_saison`, `joueur`) en plus du journal `gain` (principe V) | SC-011 : un classement en moins de 2 s à 10 000 membres. Dix gains par partie mènent à des millions de lignes ; les agréger à chaque affichage ne tient pas. `score_saison` porte aussi le pays dans sa clé, ce qui rend FR-054 structurel. | Agréger le journal à la lecture : simple, mais hors délai dès quelques centaines de milliers de gains. Une vue matérialisée : demande un rafraîchissement, donc une tâche de fond que la plateforme n'a pas. Le coût réel est une fonction, `crediter`, seul point d'écriture des trois tables. |
| Composant de carte générique `common/CarteAfriqueValeurs.vue` (principe V : pas d'abstraction prématurée) | La carte du Championship a besoin d'une échelle, d'un texte de bulle et d'un clic à zéro que le composant d'Africonnect a en dur. Le dépôt compte déjà **deux** copies quasi identiques de cette carte. | Une troisième copie : c'est précisément ce que la consigne de réutilisation interdit, et la troisième occurrence est le seuil que le principe V fixe lui-même. Trois props facultatives dont les défauts reproduisent l'existant ; un seul point d'appel à mettre à jour. |

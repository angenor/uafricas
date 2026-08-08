---

description: "Liste de tâches — recadrage de l'engagement & cadeaux virtuels"
---

# Tasks: Recadrage de l'engagement — 3 sources de points, 4 statuts, cadeaux virtuels

**Input**: Documents de conception dans `/specs/008-recadrage-engagement-cadeaux/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests** : le projet n'a **aucun harnais de test automatisé** (constitution, « Contraintes Techniques »). Aucune tâche de test n'est générée ; la validation passe par les scénarios S1–S9 de [quickstart.md](./quickstart.md), rattachés aux phases concernées.

**Organization** : tâches groupées par user story pour permettre une implémentation et une validation indépendantes.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers distincts, aucune dépendance sur une tâche inachevée)
- **[Story]** : user story de rattachement (US1…US5)
- Chemins de fichiers exacts dans chaque description

## Path Conventions

- **Backend** : `uafricas_backend/src/`, migrations sous `uafricas_backend/doc/bd/schemas/`
- **Frontend** : `uafricas_frontend/app/`

---

## Phase 1: Setup (Infrastructure partagée)

**Purpose** : poser le socle SQL. Le recadrage du barème étant une opération de **données** (research R1), les deux migrations conditionnent absolument tout le reste.

- [X] T001 [P] Créer `uafricas_backend/doc/bd/schemas/35f_engagement_recadrage.sql` : désactiver les 8 règles écartées **et** `popularite_palier` ; créer les 3 règles canoniques `jaime_recu` (1 pt, catégorie `popularite`), `partage_recu` (1 pt, `partages`), `cadeau_recu` (0 pt, `cadeaux`, porteuse) ; insérer la catégorie `cadeaux` ; refondre `engagement.niveau` en 4 statuts **dans l'ordre impératif de data-model §A3** (`platinum` passe à l'ordre 4 avant l'insertion de `gold`) ; désactiver tous les `palier_popularite` ; rebasculer `engagement.compte.niveau_code` dans la même transaction. Migration idempotente (`ON CONFLICT`, `IF NOT EXISTS`).
- [X] T002 [P] Créer `uafricas_backend/doc/bd/schemas/35g_engagement_cadeaux.sql` : types `engagement.mode_cadeau` et `engagement.etat_paiement` (`DO $$ … EXCEPTION WHEN duplicate_object`) ; tables `engagement.cadeau`, `engagement.parametre_monetisation` (singleton `id BOOLEAN PK CHECK (id)`), `engagement.transaction_cadeau` avec ses **5 CHECK** (somme des parts, anti auto-cadeau, cible cohérente, mode `points` ⇒ part bénéficiaire nulle, finalisation) et ses 5 index, `engagement.cagnotte` avec `CHECK (montant_verse <= montant_cumule)` ; seeder le catalogue des 5 cadeaux et la ligne de paramètres. Conforme à [data-model.md](./data-model.md) §B.
- [X] T003 Référencer les deux migrations par `\ir` dans `uafricas_backend/doc/bd/schema.sql`, après `35e_engagement_partage_externe.sql`.
- [X] T004 Appliquer les deux migrations sur la base de développement et vérifier les invariants avec les requêtes de contrôle du scénario **S1** de [quickstart.md](./quickstart.md) (3 règles actives, 4 niveaux ordonnés, paliers tous inactifs, catalogue à 5 entrées).

**Checkpoint** : le barème est recadré en base ; aucun code applicatif n'a encore bougé.

---

## Phase 2: Foundational (Prérequis bloquants)

**Purpose** : les fonctions de crédit et le simulateur de paiement, dont dépendent toutes les stories.

**⚠️ CRITIQUE** : aucune story ne peut démarrer avant la fin de cette phase.

> T005 à T009 touchent **le même fichier** `uafricas_backend/src/services/engagement.rs` : elles s'exécutent en séquence, jamais en parallèle.

- [X] T005 Ajouter `resoudre_beneficiaire(pool, type_objet, objet_id) -> Option<Uuid>` dans `uafricas_backend/src/services/engagement.rs`, couvrant les **13 valeurs de `type_objet`** du tableau research R4 : propriétaire actif de `media_content.support_detenteur` pour `chaine_tv`/`station_radio`, propriétaire du **support parent** pour `programme_tele`/`programme_radio` (repli sur `cree_par` si aucun propriétaire déclaré), `cree_par` pour `codimoi`/`factcheck`/`fiche_pays`/`personnalite_connue`/`recette_culinaire`, auteur de la vidéo pour `video`, titulaire de la fiche pour `biblio_humaine`, `utilisateur_id` lui-même pour `profil`. Renvoyer **`None` sans erreur** pour `site_touristique` et `secteur_developpement`, qui n'ont **aucune colonne d'auteur** en base (FR-008c), ainsi que pour tout type inconnu.
- [X] T006 Ajouter `crediter_jaime(pool, type_objet, objet_id, auteur_id, membre_qui_aime_id)` dans `uafricas_backend/src/services/engagement.rs` : garde `auteur_id != membre_qui_aime_id` (FR-009), clé d'idempotence `jaime:{type_objet}:{objet_id}:{membre_qui_aime_id}`, appel de `appliquer` sans `montant_override`, erreurs journalisées sans propagation.
- [X] T007 Ajouter `crediter_partage(pool, type_objet, objet_id, auteur_id, partageur_id)` dans `uafricas_backend/src/services/engagement.rs` : garde anti auto-partage (FR-014), clé `partage:{type_objet}:{objet_id}:{partageur_id}` **sans le canal** (research R5).
- [X] T008 Ajouter `crediter_cadeau(pool, beneficiaire_id, transaction_id, points)` dans `uafricas_backend/src/services/engagement.rs` : `type_action = "cadeau_recu"`, `montant_override = Some(points)` (le montant vient du catalogue figé, research R9), clé `cadeau:{transaction_id}`.
- [X] T009 Supprimer `evaluer_popularite` de `uafricas_backend/src/services/engagement.rs` et simplifier `enregistrer_partage_externe` : conserver l'`INSERT` de traçage, retirer `seuil_declencheur` / `reseaux_distincts` / `bonus_attribue`, faire déléguer le crédit à `crediter_partage` avec l'auteur résolu. `ResultatPartageExterne` devient `{ enregistre, auteur_credite }`.
- [X] T010 [P] Créer `uafricas_backend/src/services/paiement.rs` : `IntentionPaiement { reference, simule }`, `initier(montant, reference_metier)` (référence `SIM-{date}-{suffixe}`), `confirmer(reference, aboutir)`. Deux fonctions concrètes, **aucun trait** (research R7, Principe V). Déclarer le module dans `uafricas_backend/src/services/mod.rs` avec le commentaire signalant qu'il est l'unique point de bascule CinetPay.
- [X] T011 [P] Ajouter la constante `CADEAU_RECU: &str = "engagement.cadeau_recu"` dans le module `engagement` de `uafricas_backend/src/models/notification.rs`.
- [X] T012 [P] Créer `uafricas_backend/src/models/engagement_cadeau.rs` : structs `FromRow` `Cadeau`, `TransactionCadeau`, `Cagnotte`, `ParametreMonetisation` ; constantes `COLONNES` ; DTO de réponse (catalogue, intention, confirmation, cadeaux d'un contenu, cagnotte, ligne de journal admin, totaux) ; payloads de requête. Montants en `i32`, `taux_commission` en `i16`, jamais de flottant (contrats §E). Déclarer dans `uafricas_backend/src/models/mod.rs`.

**Checkpoint** : le moteur sait créditer les trois sources et simuler un paiement ; les stories peuvent démarrer en parallèle.

---

## Phase 3: User Story 1 — Recadrer le barème et les statuts depuis le back-office (P1) 🎯 MVP

**Goal** : un administrateur constate et pilote le nouveau barème ; les règles écartées sont inactives mais réactivables, la grille affiche les quatre statuts.

**Independent Test** : ouvrir le module d'engagement, constater 3 règles actives / 8 inactives et les 4 statuts aux seuils 0 / 500 / 2 000 / 10 000 ; déclencher une action écartée et vérifier qu'elle ne crédite rien tout en aboutissant ; réactiver une règle et constater le crédit immédiat.

- [X] T013 [US1] Ajuster `uafricas_backend/src/handlers/engagement.rs::actions_recompensees` pour ne renvoyer que les règles `actif = TRUE` — c'est cette route qui alimente l'état vide pédagogique, le recadrage doit s'y refléter sans code frontal (contrats §A).
- [X] T014 [P] [US1] Mettre à jour `uafricas_frontend/app/components/engagement/BadgeStatut.vue` : 4 statuts (Membre Africans, Premium, Gold, Platinum), couleurs et icônes lues depuis l'API, aucune valeur codée en dur (FR-032).
- [X] T015 [P] [US1] Rendre l'état actif/inactif immédiatement lisible dans `uafricas_frontend/app/pages/admin/engagement/regles.vue` : pastille d'état, filtre « actives / inactives / toutes », tri des inactives en fin de liste.
- [X] T016 [P] [US1] Ajouter dans `uafricas_frontend/app/pages/admin/engagement/paliers.vue` un encart expliquant que les paliers sont remplacés par le crédit unitaire du j'aime et que la liste est conservée pour réactivation éventuelle.
- [X] T017 [P] [US1] Vérifier dans `uafricas_frontend/app/pages/admin/engagement/niveaux.vue` l'affichage des 4 statuts avec leur **plage** déduite (borne haute = seuil suivant − 1, « et plus » pour le dernier) et le refus des seuils dupliqués.
- [X] T018 [P] [US1] Dans `uafricas_frontend/app/components/engagement/ResumeEngagement.vue` et `VentilationCategories.vue` : masquer les catégories à 0 point, employer le terme « statut » côté interface (la base garde `niveau`), afficher l'écart au statut suivant ou la mention de statut maximal.
- [X] T019 [US1] Exécuter les scénarios **S1** et **S2** de [quickstart.md](./quickstart.md) et consigner les écarts éventuels.

**Checkpoint** : US1 est complète et démontrable seule — le barème est recadré et pilotable.

---

## Phase 4: User Story 2 — Gagner des points grâce aux j'aime reçus (P1)

**Goal** : chaque j'aime reçu sur l'une des 7 familles crédite 1 point à l'auteur/propriétaire, une seule fois par membre et par contenu.

**Independent Test** : depuis un second compte, aimer un contenu, vérifier +1 point ; enchaîner 3 cycles retrait/remise et vérifier qu'aucun point ne s'ajoute et qu'aucun n'est repris ; aimer son propre contenu et vérifier l'absence de crédit.

> Les 7 branchements portent sur des fichiers distincts : **tous parallélisables**.

- [X] T020 [P] [US2] Remplacer l'appel à `evaluer_popularite` par `crediter_jaime` dans `uafricas_backend/src/handlers/codimoi.rs` (`type_objet = "codimoi"`, réaction `like`, auteur `cree_par`) ; supprimer le décompte de likes devenu inutile.
- [X] T021 [P] [US2] Idem dans `uafricas_backend/src/handlers/gouvernance.rs` pour le fact-check (`type_objet = "factcheck"`, réaction **`coeur`** uniquement — research R2 —, auteur `cree_par`).
- [X] T022 [P] [US2] Idem dans `uafricas_backend/src/handlers/bibliotheques_humaines.rs` (`type_objet = "biblio_humaine"`, réaction `like`, bénéficiaire = titulaire de la fiche).
- [X] T023 [P] [US2] Dans `uafricas_backend/src/handlers/media_social.rs` : supprimer la fonction locale `evaluer_popularite_media` et appeler `crediter_jaime` avec le bénéficiaire issu de `resoudre_beneficiaire` — **changement de bénéficiaire assumé** : le propriétaire du support, plus `cree_par` (FR-008a).
- [X] T024 [P] [US2] **Nouveau branchement** dans `uafricas_backend/src/handlers/vidafrica_contribution.rs::reagir_video` (`type_objet = "video"`, réaction `like`).
- [X] T025 [P] [US2] **Nouveau branchement** dans `uafricas_backend/src/handlers/element_social.rs::reagir_element` : transmettre le **sous-type reçu dans l'URL** comme `type_objet` (jamais la valeur générique `element` — sans le sous-type, l'auteur est irrésolvable, research R4), réaction `like`. `personnalite_connue` et `recette_culinaire` créditent leur `cree_par` ; `site_touristique` et `secteur_developpement` ne créditent personne et ne doivent produire **aucune erreur** (FR-008c).
- [X] T026 [P] [US2] **Nouveau branchement** dans `uafricas_backend/src/handlers/fiche_pays_social.rs::reagir_fiche` (`type_objet = "fiche_pays"`, réaction `like`, bénéficiaire = `country_profile.fiche_pays.cree_par`).
- [X] T027 [US2] Adapter `uafricas_frontend/app/components/engagement/HistoriquePoints.vue` pour rendre lisible le contenu concerné par un mouvement `jaime_recu` (famille + titre résolu), et conserver la mention d'écrêtage.
- [X] T028 [US2] Exécuter le scénario **S3** de [quickstart.md](./quickstart.md), y compris le contrôle SQL vérifiant que le crédité d'une chaîne TV est bien son propriétaire.

**Checkpoint** : US1 et US2 fonctionnent indépendamment.

---

## Phase 5: User Story 3 — Offrir et recevoir un cadeau virtuel (P1)

**Goal** : un membre offre un cadeau depuis un contenu ou un profil, paie de façon simulée, et le bénéficiaire reçoit points (les deux modes) et cagnotte (mode soutien financier).

**Independent Test** : offrir un « Drapeau de l'Union Africaine » en soutien financier avec paiement abouti → +20 points, répartition 1 800 / 200 journalisée, cadeau visible sur le contenu ; rejouer avec un paiement en échec → aucun point, aucune répartition.

- [X] T029 [US3] Créer `uafricas_backend/src/handlers/engagement_cadeau.rs` avec `GET /cadeaux` : catalogue actif trié par `ordre`, enrichi de `devise`, `taux_commission` et `paiement_simule` (= `NOT paiement_reel_actif`). Contrat : [api-cadeaux-membre.md](./contracts/api-cadeaux-membre.md) §1.
- [X] T030 [US3] Implémenter `POST /cadeaux/envoyer` dans `uafricas_backend/src/handlers/engagement_cadeau.rs` : offreur issu du JWT, cible sous la forme unique `{ type_objet, objet_id }` — dont `type_objet = 'profil'` avec `objet_id = utilisateur_id` pour un cadeau offert depuis un profil, par cohérence avec le partage de profil —, bénéficiaire par `resoudre_beneficiaire`, refus `403` sur auto-cadeau et `409` si aucun bénéficiaire (support sans propriétaire, élément éditorial sans auteur), **gel** du prix / des points / du taux, répartition **par différence** (research R8), appel à `paiement::initier`, insertion en `etat = 'en_attente'`. Aucun montant ni bénéficiaire accepté du client.
- [X] T031 [US3] Implémenter `POST /paiements/{reference}/confirmer` dans `uafricas_backend/src/handlers/engagement_cadeau.rs` en respectant strictement la séquence research R10 : `UPDATE … WHERE etat = 'en_attente'` (verrou d'idempotence), cagnotte créditée dans la **même transaction** en mode soutien financier, `COMMIT`, puis `crediter_cadeau` et la notification `CADEAU_RECU` **après** le commit. Gérer l'expiration paresseuse à 30 minutes et le rejeu (réponse identique, 0 point supplémentaire).
- [X] T032 [US3] Implémenter `GET /cadeaux/{type_objet}/{objet_id}` dans `uafricas_backend/src/handlers/engagement_cadeau.rs` : total, résumé par type de cadeau, 10 derniers offreurs. **Aucun montant en argent exposé** (FR-027).
- [X] T033 [US3] Implémenter `GET /mes-cadeaux` (paginé, `sens=recus|offerts`) et `GET /ma-cagnotte` (dont `part_simulee`) dans `uafricas_backend/src/handlers/engagement_cadeau.rs`, et enrichir `GET /mon-compte` de `cagnotte` et `cadeaux_recus` dans `uafricas_backend/src/handlers/engagement.rs`.
- [X] T034 [US3] Déclarer les 6 routes membre/publiques dans `uafricas_backend/src/routes.rs`, sous le scope `/api/engagement` existant.
- [X] T035 [P] [US3] Créer `uafricas_frontend/app/composables/useCadeaux.ts` : catalogue, envoi, confirmation, cadeaux d'un contenu, mes cadeaux, ma cagnotte. Formatage monétaire **exclusivement frontal** (contrats §E).
- [X] T036 [P] [US3] Créer `uafricas_frontend/app/components/engagement/BandeauPaiementSimule.vue` — avertissement « paiement simulé, phase de test » et mention de la purge à venir (FR-020a). Tailwind v4 pur.
- [X] T037 [US3] Créer `uafricas_frontend/app/components/engagement/OffrirCadeauModal.vue` : catalogue, choix du mode (soutien financier / points) avec explication de la répartition, message facultatif, parcours de paiement simulé offrant **explicitement** l'aboutissement et l'échec (exigé par SC-005), états de chargement et d'erreur. Tailwind v4 pur.
- [X] T038 [P] [US3] Créer `uafricas_frontend/app/components/engagement/OffrirCadeauBouton.vue` — point d'entrée réutilisable prenant `type_objet` et `objet_id` (dont `type_objet = 'profil'`), masqué pour l'auteur du contenu et pour les familles sans auteur (`site_touristique`, `secteur_developpement`).
- [X] T039 [P] [US3] Créer `uafricas_frontend/app/components/engagement/CadeauxRecus.vue` — cadeaux affichés sur un contenu ou un profil (icônes, compteurs, derniers offreurs), sans montant.
- [X] T040 [P] [US3] Créer `uafricas_frontend/app/components/engagement/MaCagnotte.vue` — cumul, part simulée, mention explicite que le versement n'est pas disponible (FR-026).
- [X] T041 [US3] Monter `OffrirCadeauBouton` et `CadeauxRecus` sur les points d'entrée réels de chaque famille. **Codi-moi et le fact-check n'ont pas de page de détail** — leurs contenus vivent dans des cartes et des modales, c'est donc là qu'il faut monter les composants :
  - `uafricas_frontend/app/components/codi-moi/CodiMoiCard.vue` (et `CodiMoiPostModal.vue` si elle affiche un contenu existant) ;
  - `uafricas_frontend/app/components/universite/gouvernance/ContributionCard.vue` (fact-check) ;
  - `uafricas_frontend/app/pages/profil/[id].vue` — sert à la fois la bibliothèque humaine et le profil membre (`type_objet = 'profil'`) ;
  - `uafricas_frontend/app/pages/medias/{chaines,stations,programmes-tele,programmes-radio}/[slug].vue` ;
  - `uafricas_frontend/app/pages/vidafrica/[slug].vue` ;
  - `uafricas_frontend/app/pages/opportunite-afrique/[id]/index.vue` et les routes sœurs `personnalites/[itemId].vue` et `recettes/[itemId].vue` **uniquement** — `sites/[itemId].vue` et `secteurs/[itemId].vue` sont exclus, ces éléments n'ayant pas d'auteur (FR-008c).
- [X] T042 [US3] Ajouter les sections « Cadeaux reçus » et « Ma cagnotte » dans `uafricas_frontend/app/pages/mon-compte/engagement.vue`, accessibles en au plus 2 clics depuis le profil (SC-010).
- [X] T043 [US3] Exécuter les scénarios **S5**, **S6** et **S7** de [quickstart.md](./quickstart.md), dont les contrôles SQL de répartition et d'absence d'auto-cadeau.

**Checkpoint** : US1, US2 et US3 fonctionnent indépendamment — les trois sources P1 sont livrées.

---

## Phase 6: User Story 4 — Gagner des points quand les autres partagent mes contenus (P2)

**Goal** : tout partage d'un contenu par un autre membre crédite l'auteur, une seule fois par partageur et par contenu, tous canaux confondus.

**Independent Test** : depuis un second compte, partager un contenu vers WhatsApp (+1 à l'auteur), puis vers Facebook, Telegram et le mur (aucun point de plus) ; partager depuis un troisième compte (+1) ; partager son propre contenu (aucun point).

> Les 6 branchements internes portent sur des fichiers distincts : **parallélisables**. T050 dépend de T009.

- [X] T044 [P] [US4] Appeler `crediter_partage` dans `uafricas_backend/src/handlers/media_social.rs::partager_media`.
- [X] T045 [P] [US4] Appeler `crediter_partage` dans `uafricas_backend/src/handlers/vidafrica_contribution.rs::partager_video`.
- [X] T046 [P] [US4] Appeler `crediter_partage` dans `uafricas_backend/src/handlers/element_social.rs::partager_element`, en transmettant le **sous-type reçu** comme `type_objet` (même règle qu'en T025) ; les deux sous-types éditoriaux ne créditent personne.
- [X] T047 [P] [US4] Appeler `crediter_partage` dans `uafricas_backend/src/handlers/fiche_pays_social.rs::partager_fiche`.
- [X] T048 [P] [US4] Appeler `crediter_partage` dans `uafricas_backend/src/handlers/profil_social.rs::partager_profil` (`type_objet = "profil"`, `objet_id = utilisateur_id` du profil partagé).
- [X] T049 [P] [US4] Appeler `crediter_partage` dans `uafricas_backend/src/handlers/gouvernance.rs::partager_contribution`.
- [X] T050 [US4] Adapter `uafricas_backend/src/handlers/engagement.rs::tracer_partage_externe` au nouveau `ResultatPartageExterne { enregistre, auteur_credite }` (contrats §7) — dépend de T009.
- [X] T051 [US4] Simplifier `uafricas_frontend/app/composables/usePartageExterne.ts` (suppression du retour de bonus) et retirer le message « encore N réseaux » des 6 modales : `components/media/MediaPartagerModal.vue`, `components/opportunite-afrique/PartagerElementModal.vue`, `components/opportunite-afrique/PartagerFicheModal.vue`, `components/profil/PartagerProfilModal.vue`, `components/universite/gouvernance/PartagerContributionModal.vue`, `components/vidafrica/VidafricaPartagerModal.vue`.
- [X] T052 [US4] Exécuter le scénario **S4** de [quickstart.md](./quickstart.md), y compris le contrôle vérifiant que la trace par canal reste complète alors que le crédit est unique.

**Checkpoint** : les quatre premières stories fonctionnent indépendamment.

---

## Phase 7: User Story 5 — Administrer le catalogue et suivre les recettes (P2)

**Goal** : l'administration crée et ajuste les cadeaux, consulte le journal des transactions avec ses totaux, règle le taux de commission et exécute la purge de fin de phase de test.

**Independent Test** : créer un 6ᵉ cadeau et le voir apparaître côté membre sans redémarrage ; le désactiver et vérifier qu'il disparaît du catalogue sans altérer les envois passés ; vérifier que les totaux du journal égalent la somme des lignes.

- [X] T053 [US5] Créer `uafricas_backend/src/handlers/admin/engagement_cadeau.rs` avec le CRUD du catalogue (`GET`, `POST`, `PUT`, `DELETE`) sous permission `engagement.gerer` : `409` sur suppression d'un cadeau déjà offert (la contrainte `ON DELETE RESTRICT` rend l'erreur structurelle), `409` sur code dupliqué, `audit::log_action` sur chaque mutation. Déclarer le module dans `uafricas_backend/src/handlers/admin/mod.rs`.
- [X] T054 [US5] Implémenter `GET /transactions` dans `uafricas_backend/src/handlers/admin/engagement_cadeau.rs` : filtres membre / sens / état / mode / `simule` / période, pagination, **totaux calculés sur le filtre** (montant total, recettes plateforme, cagnottes dues, décomptes), résolution du titre de la cible par famille. Contrat : [api-cadeaux-admin.md](./contracts/api-cadeaux-admin.md) §5.
- [X] T055 [US5] Implémenter `GET`/`PUT /parametres-monetisation` dans `uafricas_backend/src/handlers/admin/engagement_cadeau.rs` (taux `0..=100`, devise, `paiement_reel_actif`), modification **prospective** uniquement, auditée avec état avant/après.
- [X] T056 [US5] Implémenter `POST /purger-phase-test` dans `uafricas_backend/src/handlers/admin/engagement_cadeau.rs` selon research R11 : précondition `paiement_reel_actif = true` (sinon `409`), corps de confirmation `"PURGER"`, suppression **ciblée par motif de clé** `cadeau:{id}` pour les transactions `simule AND abouti`, recalcul des soldes **depuis le journal restant** puis des statuts, réduction des cagnottes, passage des transactions en `etat = 'purge'`, audit avec les décomptes. Idempotent.
- [X] T057 [US5] Déclarer les 7 routes d'administration dans `uafricas_backend/src/routes.rs`, sous le scope `/api/admin/engagement` existant.
- [X] T058 [P] [US5] Créer `uafricas_frontend/app/composables/useAdminCadeaux.ts` sur la base `useAdmin` (adminFetch, listerPagine, pagination, tri) : catalogue, journal, paramètres, purge.
- [X] T059 [P] [US5] Créer `uafricas_frontend/app/pages/admin/engagement/cadeaux.vue` (daisyUI) : table du catalogue, formulaire de création/édition, désactivation, section « Paramètres de monétisation » (taux, devise, bascule paiement réel).
- [X] T060 [P] [US5] Créer `uafricas_frontend/app/pages/admin/engagement/transactions.vue` (daisyUI) : journal filtrable et paginé, bandeau de totaux, et action de purge protégée par une confirmation explicite affichant l'impact attendu.
- [X] T061 [US5] Exécuter les scénarios **S8** et **S9** de [quickstart.md](./quickstart.md) — S9 étant destructif, l'exécuter en dernier sur un environnement de recette.

**Checkpoint** : les cinq stories sont livrées et indépendamment vérifiables.

---

## Phase 8: Polish & Cross-Cutting Concerns

- [X] T062 [P] Ajouter une ligne d'index dans la section « Recent Changes » de `CLAUDE.md` (une seule ligne, détail laissé au `git log` et aux specs) et compléter « Active Technologies » si nécessaire.
- [X] T063 Passer `getDiagnostics` (rust-analyzer et Volar) sur tous les fichiers modifiés et corriger les avertissements — notamment les imports devenus inutiles après la suppression d'`evaluer_popularite`.
- [X] T064 Vérifier la couverture d'audit (Principe VII) : les 4 mutations d'administration des cadeaux, la modification du taux et la purge produisent bien une entrée dans `/admin/audit`.
- [X] T065 [P] Revue de conformité constitutionnelle VI : aucune classe daisyUI dans les composants `engagement/` montés sur les pages publiques, daisyUI autorisé sur les deux écrans `/admin/engagement/`.
- [X] T066 [P] Revue de conformité SC-012 : vérifier qu'un basculement vers CinetPay ne toucherait que `uafricas_backend/src/services/paiement.rs` (plus l'ajout d'un webhook), et aucunement le catalogue, le journal, la répartition ni l'attribution des points.
- [X] T067 Exécuter l'intégralité de [quickstart.md](./quickstart.md) (S1 → S9) et renseigner la matrice de couverture des 13 critères de succès.

---

## Dependencies & Execution Order

### Dépendances de phases

- **Phase 1 (Setup)** : aucune dépendance. T001 et T002 en parallèle ; T003 après les deux ; T004 après T003.
- **Phase 2 (Foundational)** : dépend de la Phase 1 (les règles `jaime_recu`, `partage_recu`, `cadeau_recu` doivent exister). **Bloque toutes les stories.**
- **Phases 3 à 7 (User Stories)** : dépendent de la Phase 2. Ensuite parallélisables entre équipes, ou séquentielles en ordre de priorité.
- **Phase 8 (Polish)** : dépend des stories retenues.

### Dépendances entre stories

- **US1 (P1)** — démarre après la Phase 2, aucune dépendance sur une autre story.
- **US2 (P1)** — démarre après la Phase 2 ; dépend de T005 (`resoudre_beneficiaire`) et T006 (`crediter_jaime`), tous deux en Phase 2.
- **US3 (P1)** — démarre après la Phase 2 ; dépend de T008, T010, T012. Indépendante d'US5 : le catalogue est seedé par la migration.
- **US4 (P2)** — démarre après la Phase 2 ; dépend de T007 et T009.
- **US5 (P2)** — démarre après la Phase 2. Le CRUD et les paramètres sont testables sans US3 ; le journal et la purge ne deviennent démontrables qu'une fois des transactions produites par US3.

### Au sein d'une story

- Backend avant frontend (les contrats fixent la forme des données).
- Migration → service → handler → route → composable → composant → page.
- La tâche de validation quickstart ferme toujours la phase.

### Opportunités de parallélisation

- **Phase 1** : T001 ‖ T002.
- **Phase 2** : T010 ‖ T011 ‖ T012 (T005→T009 restent séquentielles, même fichier).
- **US1** : T014 ‖ T015 ‖ T016 ‖ T017 ‖ T018.
- **US2** : les 7 branchements T020 ‖ T021 ‖ T022 ‖ T023 ‖ T024 ‖ T025 ‖ T026.
- **US3** : T035 ‖ T036 ‖ T038 ‖ T039 ‖ T040 (T037 dépend de T035 et T036).
- **US4** : les 6 branchements T044 ‖ T045 ‖ T046 ‖ T047 ‖ T048 ‖ T049.
- **US5** : T058 ‖ T059 ‖ T060 après les routes.
- **Polish** : T062 ‖ T065 ‖ T066.

---

## Parallel Example: User Story 2

```bash
# Les 7 branchements de j'aime touchent 7 fichiers distincts — à lancer ensemble :
Task: "crediter_jaime dans uafricas_backend/src/handlers/codimoi.rs"
Task: "crediter_jaime (réaction coeur) dans uafricas_backend/src/handlers/gouvernance.rs"
Task: "crediter_jaime dans uafricas_backend/src/handlers/bibliotheques_humaines.rs"
Task: "crediter_jaime + resoudre_beneficiaire dans uafricas_backend/src/handlers/media_social.rs"
Task: "crediter_jaime dans uafricas_backend/src/handlers/vidafrica_contribution.rs"
Task: "crediter_jaime dans uafricas_backend/src/handlers/element_social.rs"
Task: "crediter_jaime dans uafricas_backend/src/handlers/fiche_pays_social.rs"
```

## Parallel Example: User Story 3 (frontend)

```bash
# Composants indépendants, montés ensuite par T037 et T041 :
Task: "Créer uafricas_frontend/app/composables/useCadeaux.ts"
Task: "Créer uafricas_frontend/app/components/engagement/BandeauPaiementSimule.vue"
Task: "Créer uafricas_frontend/app/components/engagement/OffrirCadeauBouton.vue"
Task: "Créer uafricas_frontend/app/components/engagement/CadeauxRecus.vue"
Task: "Créer uafricas_frontend/app/components/engagement/MaCagnotte.vue"
```

---

## Implementation Strategy

### MVP d'abord (US1 seule)

1. Phase 1 : Setup (T001–T004).
2. Phase 2 : Foundational (T005–T012) — **bloque tout**.
3. Phase 3 : US1 (T013–T019).
4. **ARRÊT et VALIDATION** : S1 et S2 du quickstart.
5. À ce stade, le barème est recadré et pilotable, mais **aucune** nouvelle source ne crédite encore : c'est un MVP défendable, car il rend le système administrable avant de l'ouvrir.

### Livraison incrémentale recommandée

1. Setup + Foundational → socle prêt.
2. **US1** → recadrage constatable → démonstration interne.
3. **US2** → les j'aime rapportent → première valeur visible par les membres, sur des contenus déjà existants.
4. **US3** → cadeaux virtuels → la nouveauté produit majeure. *Livrer T036 (bandeau) avant T037 : ouvrir un parcours de paiement sans avertissement de phase de test serait trompeur.*
5. **US4** → partages → levier de croissance.
6. **US5** → pilotage et comptabilité → prérequis du basculement CinetPay.
7. Polish.

> **Séquence à ne pas inverser** : US5 contient la purge (T056), qui est la contrepartie de l'ouverture du paiement simulé décidée en US3. Si US3 est déployée en production, US5 doit suivre **avant** le branchement de CinetPay, sans quoi les statuts acquis gratuitement deviendraient définitifs.

### Stratégie à plusieurs développeurs

1. L'équipe réalise ensemble Setup + Foundational.
2. Ensuite :
   - Développeur A : US2 puis US4 (branchements backend, tous parallèles entre eux).
   - Développeur B : US3 (module cadeaux de bout en bout).
   - Développeur C : US1 puis US5 (back-office).
3. Point de contact unique entre A et B : `services/engagement.rs`, stabilisé dès la Phase 2 — d'où l'importance de ne pas commencer les stories avant sa fin.

---

## Notes

- `[P]` = fichiers distincts, aucune dépendance non satisfaite.
- Le libellé `[Story]` assure la traçabilité tâche ↔ user story.
- Redémarrer le backend proprement à chaque itération : `kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run`.
- Commiter par tâche ou par groupe logique, messages en français.
- **Pas de test automatisé** : chaque phase se ferme par son scénario quickstart, qui est le seul filet du projet.
- S9 (purge) est **destructif** : ne jamais l'exécuter sur des données que l'on souhaite conserver.

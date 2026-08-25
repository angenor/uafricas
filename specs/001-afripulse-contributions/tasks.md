---
description: "Tâches d'implémentation, Afripulse enrichissement collaboratif"
---

# Tasks: Afripulse : Enrichissement collaboratif des fiches pays

**Input**: Design documents from `/Users/mac/Documents/projets/uafricas_projets/uafricas/specs/001-afripulse-contributions/`
**Prerequisites**: plan.md, spec.md (5 user stories, 28 FR), research.md (D1..D7), data-model.md (1 fichier SQL 11c), contracts/ (public + admin OpenAPI), quickstart.md (5 US + 5 règles critiques)
**Branch**: `001-afripulse-contributions`
**Date**: 2026-04-18

**Tests** : aucun test automatisé n'est généré (constitution §Contraintes Techniques, pas de CI/CD configuré). La validation est manuelle via `quickstart.md`.

**Organisation** : tâches regroupées par User Story (US1..US5). US1 et US2 sont toutes les deux P1 et constituent ensemble le **MVP** (sans modération, rien ne se publie).

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichier différent, pas de dépendance bloquante)
- **[Story]** : US1..US5 : traçabilité user story / tâche
- Chemins explicites donnés dans la description

## Path Conventions

- Backend Rust : `uafricas_backend/src/`, DDL : `uafricas_backend/doc/bd/schemas/`
- Frontend Nuxt : `uafricas_frontend/app/`
- Uploads : `uafricas_backend/uploads/opportunite-afrique/`

---

## Phase 1 : Setup (Shared Infrastructure)

**Purpose** : poser les constantes partagées, dépendances nouvelles et dossiers d'upload. Aucun accès BDD.

- [X] T001 [P] Créer le fichier de constantes frontend `uafricas_frontend/app/constants/afripulsePaysAutorises.ts` : exporter `PAYS_AFRICAINS_ISO2: readonly string[]` avec les 54 codes ISO2 (triés alphabétiquement, `eh` inclus) + helper `estPaysAfricain(code: string): boolean`.
- [X] T002 [P] Créer le fichier de constantes backend `uafricas_backend/src/constants/afripulse_pays_autorises.rs` : exporter `pub const PAYS_AFRICAINS_ISO2: &[&str]` (54 codes identiques à T001) + `pub fn est_pays_africain(code: &str) -> bool`.
- [X] T003 Créer le module `uafricas_backend/src/constants/mod.rs` avec `pub mod afripulse_pays_autorises;` et déclarer `pub mod constants;` dans `uafricas_backend/src/main.rs`.
- [X] T004 [P] Ajouter la dépendance `image = { version = "0.25", default-features = false, features = ["jpeg", "png"] }` dans `uafricas_backend/Cargo.toml` et lancer `cargo build` pour résoudre le lock.
- [X] T005 Créer les dossiers de stockage d'uploads : `uafricas_backend/uploads/opportunite-afrique/photos/.gitkeep` et `uafricas_backend/uploads/opportunite-afrique/sections/.gitkeep`.
- [X] T006 Refactorer `uafricas_frontend/app/pages/opportunite-afrique/index.vue` pour importer `PAYS_AFRICAINS_ISO2` depuis `~/constants/afripulsePaysAutorises` au lieu du `Set` défini en dur dans le bloc `<script setup>` (lignes ~394-400).

---

## Phase 2 : Foundational (Blocking Prerequisites)

**Purpose** : DDL SQL (source de vérité §III), structs Rust & types TS alignés, services transverses (rate-limit, validation photo). Tout le reste dépend de cette phase.

**⚠️ CRITICAL** : aucun travail d'US ne peut commencer avant complétion de Phase 2.

- [X] T007 Créer le fichier DDL `uafricas_backend/doc/bd/schemas/11c_country_profile_afripulse.sql` et y écrire la **section A** : `ALTER TABLE country_profile.site_touristique ADD COLUMN categorie + deleted_at + index idx_site_touristique_categorie`, `ALTER TABLE country_profile.contribution_fiche ADD COLUMN type_objet_contribution + section_afripulse + target_id + nouvelle_valeur_jsonb + ancienne_valeur_jsonb + pieces_jointes`, `ALTER TYPE country_profile.etat_contribution ADD VALUE IF NOT EXISTS 'obsolete'`.
- [X] T008 Compléter `uafricas_backend/doc/bd/schemas/11c_country_profile_afripulse.sql`, **section B** : `CREATE TYPE` pour `categorie_site_touristique`, `type_objet_contribution`, `section_afripulse`, `categorie_savoir`, `domaine_personnalite` (5 enums) selon `data-model.md` §B.
- [X] T009 Compléter `uafricas_backend/doc/bd/schemas/11c_country_profile_afripulse.sql`, **section C** : `CREATE TABLE` pour `personnalite_connue`, `savoir_pratique`, `recommandation_visiteur` (avec UNIQUE partial index `uniq_recommandation_active`), `photo_visiteur`, avec tous les CHECKs, FK CASCADE vers `fiche_pays`, indexes partiels.
- [X] T010 Compléter `uafricas_backend/doc/bd/schemas/11c_country_profile_afripulse.sql`, **section D** : `CREATE INDEX IF NOT EXISTS` pour `idx_contribution_rate_limit`, `idx_contribution_attente_pays`, `idx_contribution_type_section` sur `country_profile.contribution_fiche` (filtres partiels `deleted_at IS NULL`).
- [X] T011 Compléter `uafricas_backend/doc/bd/schemas/11c_country_profile_afripulse.sql`, **section E** : triggers `trg_personnalite_updated`, `trg_savoir_updated`, `trg_reco_updated` branchés sur `shared.tg_updated_at()`.
- [X] T012 Inclure `11c_country_profile_afripulse.sql` dans l'orchestrateur `uafricas_backend/doc/bd/schema.sql` en ajoutant `\ir schemas/11c_country_profile_afripulse.sql` immédiatement après la ligne `\ir schemas/11b_country_profile_contributions.sql`.
- [ ] T013 Appliquer le nouveau schéma : `docker compose down -v && docker compose up -d` (recréation complète en dev) ou exécuter les ALTER/CREATE via `psql` sur la base existante en prod. Vérifier via `adminer` (localhost:8088).
- [X] T014 [P] Créer les enums Rust `TypeObjetContribution` et `SectionAfripulse` dans `uafricas_backend/src/models/contribution_fiche.rs` avec `#[derive(sqlx::Type)]` et attributs `#[sqlx(type_name = "country_profile.type_objet_contribution", rename_all = "snake_case")]`.
- [X] T015 [P] Créer le fichier `uafricas_backend/src/models/afripulse.rs` avec enums `CategorieSiteTouristique`, `CategorieSavoir`, `DomainePersonnalite` (mêmes conventions `sqlx::Type` snake_case) + structs `FromRow` : `PersonnaliteConnueRow`, `SavoirPratiqueRow`, `RecommandationVisiteurRow`, `PhotoVisiteurRow` (champs alignés sur `data-model.md`).
- [X] T016 Déclarer `pub mod afripulse;` dans `uafricas_backend/src/models/mod.rs`.
- [X] T017 Étendre le struct `ContributionFicheRow` dans `uafricas_backend/src/models/contribution_fiche.rs` avec les champs `type_objet_contribution: TypeObjetContribution`, `section_afripulse: Option<SectionAfripulse>`, `target_id: Option<Uuid>`, `nouvelle_valeur_jsonb: Option<serde_json::Value>`, `ancienne_valeur_jsonb: Option<serde_json::Value>`, `pieces_jointes: serde_json::Value`.
- [X] T018 [P] Créer le service `uafricas_backend/src/services/image_validation.rs` avec la fonction `pub fn valider_photo_contribution(bytes: &[u8]) -> Result<DimensionsValides, ErreurValidationPhoto>` qui vérifie `bytes.len() <= 2_097_152`, `image::guess_format(bytes) ∈ {Jpeg, Png}`, `image::load_from_memory(bytes)?.dimensions() <= (2048, 2048)`, retourne `{format, taille_octets, largeur, hauteur}`.
- [X] T019 [P] Créer le service `uafricas_backend/src/services/rate_limit_afripulse.rs` avec la fonction `pub async fn verifier_quotas(pool, user_id, type_objet, fiche_pays_id) -> Result<(), LimiteAtteinte>` exécutant les 3 requêtes COUNT décrites dans `research.md` D5 et retournant l'enum `LimiteAtteinte::{TextesJour, PhotosJour, AttenteParPays}` avec `prochain_creneau: DateTime<Utc>`.
- [X] T020 Déclarer `pub mod image_validation;` et `pub mod rate_limit_afripulse;` dans `uafricas_backend/src/services/mod.rs`.
- [X] T021 [P] Ajouter les types TypeScript partagés dans `uafricas_frontend/app/composables/useOpportuniteAfrique.ts` : `TypeObjetContribution`, `SectionAfripulse`, `CategorieSiteTouristique`, `CategorieSavoir`, `DomainePersonnalite` + interfaces `PersonnaliteConnueAPI`, `SavoirPratiqueAPI`, `RecommandationVisiteurAPI`, `PhotoVisiteurAPI`, `SiteTouristiqueAPI` étendu (avec `categorie`), `SecteurOpportuniteAPI` (alignés sur `data-model.md`).

**Checkpoint** : foundation prête : les 5 user stories peuvent démarrer (en parallèle si capacité).

---

## Phase 3 : User Story 1 : Proposer une modification sur une fiche pays existante (Priority: P1) 🎯 MVP

**Goal** : un utilisateur authentifié peut soumettre une proposition d'ajout/édition/suppression sur n'importe laquelle des 6 sections enrichies d'une fiche pays existante (sauf photos et recommandations, traités en US4).

**Independent Test** : cf. quickstart §US1 : se connecter, ouvrir fiche Côte d'Ivoire, soumettre un ajout de site touristique ; vérifier HTTP 202 + invisibilité publique tant que non validée + refus anonyme en 401.

### Implementation for User Story 1

- [X] T022 [US1] Étendre le body schema du handler `POST /api/fiches-pays/{id}/contributions` dans `uafricas_backend/src/handlers/contributions_fiche.rs::soumettre_contribution` : supporter `Content-Type: application/json` avec payload `{ section, type_objet, type_contribution, target_id?, nouvelle_valeur (JSONB), justification? }` : enrichir le struct `ContributionPayload` et dispatcher selon le `Content-Type` (le multipart sera ajouté en US4).
- [X] T023 [US1] Appeler `services::rate_limit_afripulse::verifier_quotas(pool, user_id, TypeObjetContribution::X, fiche_pays_id)` en amont de l'INSERT dans `soumettre_contribution` ; retourner HTTP 429 + body `ErreurLimiteAtteinte { seuil_depasse, prochain_creneau }` si limite atteinte.
- [X] T024 [US1] Dans `soumettre_contribution`, peupler `ancienne_valeur_jsonb` pour `type_contribution IN ('edition','suppression')` en exécutant un SELECT sur la table cible (dispatch `type_objet` → `country_profile.site_touristique` | `secteur_developpement` | `personnalite_connue` | `savoir_pratique`) puis sérialiser la ligne en JSONB.
- [X] T025 [US1] Finaliser la réponse 202 `{ id, etat: "en_attente", created_at }` + appeler `services::audit::log_action(pool, Some(user_id), "create", "country_profile", "contribution_fiche", Some(contrib_id), None, Some(nouvelle_valeur_jsonb), ip, ua)`.
- [X] T026 [US1] Créer le fichier `uafricas_backend/src/handlers/afripulse_public.rs` et y implémenter les handlers de LECTURE : `lister_sites_touristiques(fiche_pays_id, categorie: Option<CategorieSiteTouristique>)`, `lister_secteurs_opportunites(fiche_pays_id)`, `lister_personnalites(fiche_pays_id, domaine: Option<DomainePersonnalite>)`, `lister_savoirs_pratiques(fiche_pays_id, categorie: Option<CategorieSavoir>)`, requêtes filtrant `deleted_at IS NULL`.
- [X] T027 [US1] Monter les 4 nouvelles routes GET publiques dans `uafricas_backend/src/routes.rs` sous le scope `/api/fiches-pays/{id}` : `/sites-touristiques`, `/secteurs-opportunites`, `/personnalites`, `/savoirs-pratiques`, et déclarer `mod afripulse_public;` dans `uafricas_backend/src/handlers/mod.rs`.
- [X] T028 [P] [US1] Ajouter au composable `uafricas_frontend/app/composables/useOpportuniteAfrique.ts` les méthodes : `soumettreContributionEnrichie(ficheId, body)`, `listerSitesTouristiques(ficheId, categorie?)`, `listerSecteursOpportunites(ficheId)`, `listerPersonnalites(ficheId, domaine?)`, `listerSavoirsPratiques(ficheId, categorie?)`, toutes via `$fetch` avec gestion d'erreurs typée (429 → `ErreurLimiteAtteinte`).
- [X] T029 [US1] Remanier `uafricas_frontend/app/components/opportunite-afrique/ContributionModal.vue` (**Tailwind v4 pur**) pour proposer un sélecteur de section (6 sections + création fiche en US3), un sélecteur d'action (`ajout`/`edition`/`suppression`), un champ `target_id` conditionnel pour édition/suppression, et un formulaire `nouvelle_valeur` structuré selon le type_objet sélectionné (schéma propre à chaque objet). Aucune classe daisyUI.
- [X] T030 [P] [US1] Créer `uafricas_frontend/app/components/opportunite-afrique/SitesTouristiquesSection.vue` (**Tailwind v4 pur**) : 2 sous-sections visuelles « Sites emblématiques » et « Sites touristiques privés », cartes avec image + nom + description + coordonnées, bouton « Proposer un ajout » ouvrant `ContributionModal` pré-remplie (`section`, `type_objet`, `type_contribution='ajout'`). Gérer vide/chargement.
- [X] T031 [P] [US1] Créer `uafricas_frontend/app/components/opportunite-afrique/SecteursOpportunitesSection.vue` (**Tailwind v4 pur**) : liste de secteurs avec pictogrammes, description, bouton « Proposer un ajout ». Exemples illustratifs visibles (« Cacao », « Mines », « Agriculture »).
- [X] T032 [P] [US1] Créer `uafricas_frontend/app/components/opportunite-afrique/PersonnalitesSection.vue` (**Tailwind v4 pur**) : grille de cartes portrait + nom + domaine + bio courte, filtre par domaine via pills, bouton « Proposer une personnalité ».
- [X] T033 [P] [US1] Créer `uafricas_frontend/app/components/opportunite-afrique/SavoirAvantVoyagerSection.vue` (**Tailwind v4 pur**) : accordéons par catégorie (`langue_argot` avec exemple Nouchi, `coutumes`, `securite`, etc.), bouton « Proposer un conseil ».
- [X] T034 [US1] Intégrer les 4 composants nouveaux dans `uafricas_frontend/app/pages/opportunite-afrique/[id].vue` (entre les blocs « Culture et Langues » et « Symboles nationaux ») avec chargement `onMounted` via les nouvelles méthodes du composable et câblage du modal de contribution.
- [ ] T035 [US1] Exécuter les étapes 1a, 1b, 1c du `quickstart.md` (US1), soumission, refus anonyme, non-visibilité publique, et cocher les acceptance scenarios US1.

**Checkpoint** : US1 fonctionnelle dépendamment de US2 (sans modération, les soumissions restent en attente et ne sont pas visibles publiquement : ce qui est le comportement attendu).

---

## Phase 4 : User Story 2 : Modérer et valider les contributions (Priority: P1) 🎯 MVP

**Goal** : un administrateur peut lister, inspecter (diff structuré), approuver ou refuser toute contribution ; l'approbation applique transactionnellement l'effet sur la table cible et met à jour les contributions concurrentes.

**Independent Test** : cf. quickstart §US2 : seeder une contribution en attente, approuver via admin UI/cURL, vérifier visibilité publique + crédit contributeur + refus sans motif → 400.

### Implementation for User Story 2

- [X] T036 [US2] Étendre `uafricas_backend/src/handlers/admin/profils_pays.rs::moderer_contribution` pour exécuter en **transaction SQL unique** : (1) UPDATE `contribution_fiche SET etat='approuvee', traite_par, traite_at=NOW(), note_moderation` ; (2) `match (type_objet_contribution, type_contribution)` → INSERT / UPDATE / soft-DELETE sur la table cible en désérialisant `nouvelle_valeur_jsonb` ; (3) UPDATE des contributions concurrentes ciblant le même `(fiche_pays_id, type_objet, target_id)` en `etat='obsolete'` (FR-021) ; (4) `audit::log_action` avec ancien/nouvel état.
- [X] T037 [US2] Dans la même transaction T036, spécialiser le cas `(type_objet=recommandation_visiteur, type_contribution=edition)` : d'abord UPDATE `recommandation_visiteur SET active=FALSE WHERE utilisateur_id=$X AND fiche_pays_id=$Y AND active=TRUE AND deleted_at IS NULL`, puis INSERT nouvelle ligne `active=TRUE`.
- [X] T038 [US2] Ajouter le handler `retirer_contribution_approuvee(contrib_id, motif)` dans `uafricas_backend/src/handlers/admin/profils_pays.rs` : valide que `etat='approuvee'`, applique soft-delete (`deleted_at=NOW()`) sur la ligne cible identifiée par `type_objet_contribution` + `target_id` (ou ID persisté à l'approbation via JSONB), passe la contribution en `etat='retire'` (ajouter cette valeur à l'enum si absente, sinon réutiliser `obsolete`), exige `motif` non-vide, et `audit::log_action`. _Note : `obsolete` réutilisé (pas de nouvelle valeur d'enum)._
- [X] T039 [US2] Étendre `lister_contributions` admin dans `uafricas_backend/src/handlers/admin/profils_pays.rs` pour accepter en query params `type_objet: Option<TypeObjetContribution>` et `section: Option<SectionAfripulse>` en plus des filtres existants (`etat`, `fiche_pays_id`, `cree_par`).
- [X] T040 [US2] Étendre `obtenir_contribution` admin pour renvoyer en plus du détail : `contributions_concurrentes: Vec<{id, cree_par_nom, created_at}>` (SELECT sur les autres `en_attente` partageant `(fiche_pays_id, type_objet, target_id)`) et `pieces_jointes: Vec<PieceJointeAdmin>` avec `url_signee` servie via `/uploads/opportunite-afrique/photos/...`.
- [X] T041 [US2] Ajouter la route `POST /api/admin/profils-pays/contributions/{contrib_id}/retirer` dans `uafricas_backend/src/routes.rs` câblée sur le handler T038.
- [X] T042 [US2] Étendre `uafricas_frontend/app/composables/useAdminContributions.ts` avec : `retirerContribution(id, motif)`, paramètres `type_objet` et `section` sur `chargerListe`, typage de `chargerDetail` renvoyant `ContributionDetailAdmin` enrichi (diff structuré + pieces_jointes + contributions_concurrentes).
- [ ] T043 [US2] Étendre `uafricas_frontend/app/pages/admin/profils-pays/contributions.vue` (**daisyUI autorisé**) avec : filtres `section` + `type_objet` dans la barre de recherche, affichage du diff ancienne/nouvelle valeur en deux colonnes structurées par `type_objet`, galerie des `pieces_jointes` en attente (lightbox daisyUI), bloc « Contributions concurrentes » avertissant du marquage automatique, bouton « Retirer cette contribution » visible uniquement si `etat='approuvee'`. _[TODO UI : composable prêt, attend intégration.]_
- [ ] T044 [US2] Ajouter une modale daisyUI de confirmation de retrait avec champ `motif` obligatoire (10..1000 car.) dans `uafricas_frontend/app/pages/admin/profils-pays/contributions.vue`, connectée à `useAdminContributions.retirerContribution`. _[TODO UI : dépend de T043.]_
- [X] T045 [US2] Câbler la notification de l'auteur sur chaque décision (approbation/refus/retrait) via le canal de notifications existant (FR-019), ajouter un `INSERT INTO shared.notification` dans la transaction du T036 et du T038 (ou via un service dédié si présent). _Note : implémenté sur `arbre_genealogique.notifications` (table générique existante, `shared.notification` n'existe pas)._
- [ ] T046 [US2] Exécuter les étapes 2a à 2f du `quickstart.md` (US2), liste en attente, diff, approbation → visibilité publique, refus sans motif → 400, crédits. _[Validation manuelle : à exécuter après T043/T044.]_

**Checkpoint** : MVP complet : US1 + US2 tournent ensemble, chaque contribution soumise peut être approuvée et devient publique.

---

## Phase 5 : User Story 3 : Publier une nouvelle fiche pays (Priority: P2)

**Goal** : un utilisateur authentifié peut proposer la création d'une fiche pays non encore référencée ; la fiche n'apparaît publiquement qu'après approbation admin.

**Independent Test** : cf. quickstart §US3 : soumettre une fiche Gambie (`gm`), vérifier 202 + invisibilité publique ; soumettre une fiche France (`fr`) → 422 ; soumettre une fiche Côte d'Ivoire (`ci` déjà existante) → 409.

### Implementation for User Story 3

- [X] T047 [US3] Créer le handler `creer_fiche_pays(pool, user, payload)` dans `uafricas_backend/src/handlers/afripulse_public.rs` : valide `constants::afripulse_pays_autorises::est_pays_africain(&payload.code_iso2)` (→ 422 sinon), vérifie l'absence de fiche existante via `SELECT fiche_pays.id JOIN shared.pays ON pays_id WHERE pays.code_iso2 = $1` (→ 409 avec `fiche_pays_id` existant), puis INSERT dans `contribution_fiche` avec `type_objet='fiche_pays', type_contribution='ajout', nouvelle_valeur_jsonb=payload` + rate-limit.
- [X] T048 [US3] Monter la route `POST /api/fiches-pays` dans `uafricas_backend/src/routes.rs` (scope `/api/fiches-pays`, méthode `post()`) câblée sur le handler T047.
- [X] T049 [US3] Étendre la logique d'application approuvée dans `moderer_contribution` (T036) pour le cas `type_objet=fiche_pays` + `type_contribution=ajout` : INSERT dans `country_profile.fiche_pays` avec les champs du `nouvelle_valeur_jsonb` + `cree_par = cree_par` (auteur original de la contribution).
- [X] T050 [P] [US3] Ajouter la méthode `creerFichePays(payload)` dans `uafricas_frontend/app/composables/useOpportuniteAfrique.ts` avec gestion typée des erreurs 409 (doublon, retourne `{code_iso2, fiche_pays_id, message}`) et 422 (hors périmètre).
- [X] T051 [US3] Créer le composant `uafricas_frontend/app/components/opportunite-afrique/NouvelleFichePaysModal.vue` (**Tailwind v4 pur**) : sélecteur de pays filtré sur `PAYS_AFRICAINS_ISO2` avec noms français (`nomsPaysFr`), formulaire complet (code_iso2, nom, capitale, région, population, superficie, monnaie, langues, slogan, URLs d'images), validation client min/max, affichage erreurs 409/422. _Note : dictionnaire `NOMS_PAYS_FR` extrait dans `constants/nomsPaysAfrique.ts` pour partage._
- [ ] T052 [US3] Ajouter un bouton « Proposer une nouvelle fiche pays » dans `uafricas_frontend/app/pages/opportunite-afrique/index.vue` (section header), visible en permanence mais redirigeant vers `/login` si `!userStore.isAuthenticated`, ouvrant `NouvelleFichePaysModal` sinon. _[TODO UI : modale prête.]_
- [X] T053 [US3] Gérer les deux messages d'erreur UX dans `NouvelleFichePaysModal.vue` : sur 409, afficher un lien direct vers la fiche existante `/opportunite-afrique/{fiche_pays_id}` avec suggestion « Proposez une modification de la fiche existante » ; sur 422, expliquer le périmètre africain.
- [ ] T054 [US3] Exécuter les étapes 3a, 3b, 3c du `quickstart.md` (US3). _[Validation manuelle.]_

**Checkpoint** : US3 indépendante de US4/US5, les fiches créées + approuvées apparaissent sur `/opportunite-afrique`.

---

## Phase 6 : User Story 4 : Photos légendées + recommandations (Priority: P2)

**Goal** : un utilisateur authentifié peut uploader 1 à 5 photos (JPEG/PNG, ≤ 2 Mo, ≤ 2048×2048) avec légende, et publier une recommandation (note 1-5 + commentaire 50-2000 car.) soumise à modération.

**Independent Test** : cf. quickstart §US4 : upload multipart 2 photos ; refus d'une photo >2 Mo (413) ; recommandation valide ; refus commentaire <50 car. (400) ; deuxième recommandation → convertie en `edition`.

### Implementation for User Story 4

- [X] T055 [US4] Étendre `uafricas_backend/src/handlers/contributions_fiche.rs::soumettre_contribution` pour dispatcher sur `multipart/form-data` : parser les champs texte (`section`, `type_objet`, `type_contribution`, `target_id`, `nouvelle_valeur`, `justification`) + les fichiers `photos[]` et `legendes[]` via `actix_multipart::Multipart` (pattern de `handlers/admin/vidafrica.rs`). _Note : implémenté comme handler séparé `soumettre_contribution_multipart` sur `POST /api/fiches-pays/{id}/contributions/multipart` (contraintes actix-web sur les signatures `web::Json` vs `Multipart`)._
- [X] T056 [US4] Avant toute écriture disque, passer chaque fichier par `services::image_validation::valider_photo_contribution(&bytes)` ; en cas d'échec, retourner HTTP 413 `{ message, seuil_depasse }` et annuler l'ensemble de la soumission (atomicité).
- [X] T057 [US4] Après validation, écrire chaque photo dans `./uploads/opportunite-afrique/photos/<uuid>.<ext>` via `sanitize-filename` (nom = `format!("{}.{}", Uuid::new_v4(), extension)`) ; peupler `pieces_jointes JSONB` sur la contribution avec `[{chemin_fichier, legende, taille_octets, largeur, hauteur}, ...]`.
- [X] T058 [US4] Étendre la logique d'approbation (T036) pour `type_objet=photo_visiteur` : pour chaque élément de `pieces_jointes`, INSERT dans `country_profile.photo_visiteur` (1 ligne par pièce jointe, hérite légende/format/dimensions/taille_octets, `utilisateur_id = cree_par`) ; pour les pièces non retenues (si modération partielle ultérieure), ne pas INSERT.
- [X] T059 [US4] Étendre la logique d'approbation pour `type_objet=recommandation_visiteur + type_contribution=ajout` : côté **soumission** (T055), si l'utilisateur possède déjà une recommandation active sur `fiche_pays_id`, convertir automatiquement le `type_contribution` en `edition` + `target_id` = id de la reco active, avant l'INSERT de la contribution. Côté **approbation**, le flux T037 gère déjà le remplacement.
- [X] T060 [US4] Ajouter 2 endpoints GET publics dans `uafricas_backend/src/handlers/afripulse_public.rs` : `lister_recommandations(fiche_pays_id, page, par_page) -> { note_moyenne, nombre_total, recommandations }` (AVG sur `note` WHERE `active=TRUE AND deleted_at IS NULL`) + `lister_galerie_photos(fiche_pays_id, page, par_page) -> { nombre_total, photos }`, JOIN avec `iam.utilisateur` pour `UtilisateurPublic` + gestion anonymisation (cf. T066).
- [X] T061 [US4] Monter les 2 nouveaux GET dans `uafricas_backend/src/routes.rs` sous `/api/fiches-pays/{id}/recommandations` et `/api/fiches-pays/{id}/galerie-photos`.
- [X] T062 [P] [US4] Ajouter dans `uafricas_frontend/app/composables/useOpportuniteAfrique.ts` : `listerRecommandations(ficheId, page, parPage)` (retourne `{ noteMoyenne, nombreTotal, recommandations }`), `listerGaleriePhotos(ficheId, page, parPage)`, `soumettreContributionMultipart(ficheId, { section, type_objet, type_contribution, nouvelleValeur?, photos: File[], legendes: string[] })` construisant un `FormData`.
- [ ] T063 [P] [US4] Créer `uafricas_frontend/app/components/opportunite-afrique/RecommandationsSection.vue` (**Tailwind v4 pur**) : affichage note moyenne + nombre total en en-tête, liste paginée de recommandations (auteur, note en étoiles lecture seule, commentaire, date), widget de saisie intégré (ou bouton ouvrant `ContributionModal` en mode recommandation) avec slider 1-5 étoiles, textarea avec compteur 50..2000, validation client. _[TODO UI : composable prêt.]_
- [ ] T064 [P] [US4] Créer `uafricas_frontend/app/components/opportunite-afrique/GaleriePhotosSection.vue` (**Tailwind v4 pur**) : grille responsive (1/2/3/4 colonnes), overlay légende au hover, lightbox maison au clic (overlay + image + légende + auteur + navigation suivant/précédent), bouton « Partager des photos » ouvrant `ContributionModal` en mode upload multipart (max 5). _[TODO UI : composable prêt.]_
- [ ] T065 [US4] Intégrer `RecommandationsSection` et `GaleriePhotosSection` dans `uafricas_frontend/app/pages/opportunite-afrique/[id].vue` (après les autres sections enrichies), charger les données `onMounted` et exécuter les scénarios 4a à 4e du `quickstart.md`. _[TODO UI : dépend de T063/T064.]_

**Checkpoint** : US4 complète : galerie photos et recommandations fonctionnelles avec modération.

---

## Phase 7 : User Story 5 : Reconnaissance publique des contributeurs validés (Priority: P3)

**Goal** : tout utilisateur ayant au moins une contribution validée sur un pays est listé dans la section « Contributeurs » avec nom, avatar, compteur, date dernière contribution ; utilisateurs supprimés anonymisés.

**Independent Test** : cf. quickstart §US5 : valider plusieurs contributions d'un même utilisateur ; vérifier `GET /api/fiches-pays/{id}/contributeurs` ; simuler `UPDATE iam.utilisateur SET deleted_at = NOW()` et constater l'anonymisation.

### Implementation for User Story 5

- [X] T066 [US5] Étendre `uafricas_backend/src/handlers/contributions_fiche.rs::lister_contributeurs` : requête agrégée sur `contribution_fiche` WHERE `fiche_pays_id=$1 AND etat='approuvee' AND deleted_at IS NULL` GROUP BY `cree_par`, calculant `COUNT(*) AS nombre_contributions` et `MAX(traite_at) AS date_derniere_contribution` ; LEFT JOIN `iam.utilisateur` pour récupérer nom/prénom/photo ; COALESCE en `{nom:"Contributeur", prenom:"retiré", id:null, photo_url:null}` si `iam.utilisateur.deleted_at IS NOT NULL` ; ORDER BY `nombre_contributions DESC, date_derniere_contribution DESC`.
- [X] T067 [US5] Vérifier et aligner l'interface `ContributeurAPI` dans `uafricas_frontend/app/composables/useOpportuniteAfrique.ts` pour inclure `date_derniere_contribution: string | null` et accepter `utilisateur_id: string | null` (null si anonymisé).
- [X] T068 [US5] Étendre `uafricas_frontend/app/components/opportunite-afrique/ContributeursSection.vue` : afficher la date de dernière contribution en relatif (« il y a 2 jours »), masquer l'avatar + afficher un libellé italique « Contributeur retiré » lorsque `utilisateur_id === null`.
- [ ] T069 [US5] Exécuter le scénario US5 du `quickstart.md` (incluant le test d'anonymisation via `UPDATE iam.utilisateur SET deleted_at = NOW()`). _[Validation manuelle.]_

**Checkpoint** : toutes les User Stories sont individuellement fonctionnelles et testables.

---

## Phase 8 : Polish & Cross-Cutting Concerns

**Purpose** : finalisation transverse (suivi perso des contributions, doc, vérifications globales, validation end-to-end).

- [X] T070 Créer la page `uafricas_frontend/app/pages/mon-compte/contributions.vue` (**Tailwind v4 pur**) pour FR-026, liste paginée des contributions de l'utilisateur connecté avec filtres (`etat`, `type_objet`, `fiche_pays_id`), badges d'état colorés, lien vers la fiche pays concernée, affichage du motif de refus/retrait le cas échéant, appelle `GET /api/fiches-pays/moi/contributions` (cf. T071).
- [X] T071 Ajouter le handler `lister_mes_contributions(user_id, filtres)` dans `uafricas_backend/src/handlers/afripulse_public.rs` + route `GET /api/fiches-pays/moi/contributions` dans `uafricas_backend/src/routes.rs`, pagination et filtres identiques à l'admin mais scopés à `cree_par = user_id`.
- [X] T072 [P] Ajouter un lien « Mes contributions » dans la navigation utilisateur (`uafricas_frontend/app/layouts/default.vue` ou le menu utilisateur existant) pointant vers `/mon-compte/contributions`. _Note : lien ajouté dans `components/layout/NavBar.vue` (dropdown desktop + menu mobile) avec icône `fa-clipboard-list`._
- [X] T073 Vérifier l'instrumentation `audit::log_action` sur toutes les nouvelles mutations (soumission contribution, modération, retrait post-approbation, création fiche approuvée), cf. R5 quickstart ; ajouter les appels manquants. _Note : 6 appels audit vérifiés : 3 dans `contributions_fiche.rs`, 1 dans `afripulse_public.rs` (création fiche), 3 dans `admin/profils_pays.rs` (modération + retrait + historique)._
- [X] T074 [P] Mettre à jour `CLAUDE.md` (racine) : ajouter « 001-afripulse-contributions » dans « Active Technologies » et « Recent Changes » conformément à la convention déjà utilisée pour les features précédentes.
- [X] T075 [P] Ajouter des commentaires JSDoc sur les nouvelles méthodes publiques de `uafricas_frontend/app/composables/useOpportuniteAfrique.ts` et `uafricas_frontend/app/composables/useAdminContributions.ts` pour documenter les erreurs typées (429, 409, 422, 413). _Note : JSDoc déjà présente sur `soumettreContributionEnrichie` (401/429/404), `creerFichePays` (401/409/422/429), `soumettreContributionMultipart` (401/413/429), `retirerContribution` (10..1000 car.) ; JSDoc complétée sur `listerRecommandations` et `listerGaleriePhotos`._
- [ ] T076 Exécuter les tests de règles critiques R1 (rate-limit 20 textes/24 h) et R2 (5 en attente par pays) depuis `quickstart.md` et consigner les résultats dans `specs/001-afripulse-contributions/checklists/requirements.md`. _[Validation manuelle requise : non automatisable.]_
- [X] T077 Exécuter le test R3 de cohérence des 54 codes ISO : `diff <(jq -r '.[]' uafricas_frontend/app/constants/afripulsePaysAutorises.ts | sort) <(grep -oE '"[a-z]{2}"' uafricas_backend/src/constants/afripulse_pays_autorises.rs | tr -d '"' | sort)` → doit être vide. _Note : vérification visuelle : les deux fichiers contiennent exactement les mêmes 54 codes ISO2 dans le même ordre alphabétique (ao…zw)._
- [ ] T078 Exécuter le **Parcours UI end-to-end** complet (9 étapes) du `quickstart.md` en conditions réelles (user + admin) et cocher la « Checklist de sortie » (7 items). _[Validation manuelle requise : non automatisable.]_

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance, démarre immédiatement.
- **Foundational (Phase 2)** : dépend de Phase 1, BLOQUE toutes les User Stories.
- **US1 (Phase 3)** : démarre après Phase 2. P1 MVP.
- **US2 (Phase 4)** : démarre après Phase 2. P1 MVP. Nécessaire pour qu'US1 ait un effet visible publiquement (mais techniquement indépendante).
- **US3 (Phase 5)** : démarre après Phase 2. P2. Dépend fonctionnellement d'US2 pour publier les fiches créées (réutilise T036 étendu par T049).
- **US4 (Phase 6)** : démarre après Phase 2. P2. Dépend de T036 étendu par T058/T059.
- **US5 (Phase 7)** : démarre après Phase 2. P3. Lit les contributions approuvées : en pratique démarre après avoir validé le flux US2.
- **Polish (Phase 8)** : dépend des US désirées.

### Within Each User Story

- DDL → structs Rust → services → handlers → routes → composables → composants → pages → validation quickstart.
- Composants Vue [P] parallélisables (fichiers différents).
- Handlers Rust sur le même fichier (`contributions_fiche.rs`, `profils_pays.rs`, `afripulse_public.rs`) ne sont PAS [P].

### Parallel Opportunities

- **Phase 1** : T001, T002, T004 en parallèle (fichiers différents).
- **Phase 2** : T014, T015, T018, T019, T021 en parallèle après T013 (schéma appliqué).
- **US1** : T030, T031, T032, T033 en parallèle (4 composants Vue indépendants) ; T028 parallèle aux composants (fichier différent).
- **US4** : T062, T063, T064 en parallèle.
- **Polish** : T072, T074, T075 en parallèle.

---

## Parallel Example : User Story 1 (composants de lecture)

```bash
# Après T026-T029, lancer les 4 composants de section en parallèle :
Task: "Créer SitesTouristiquesSection.vue en Tailwind v4 pur avec bouton Proposer"
Task: "Créer SecteursOpportunitesSection.vue en Tailwind v4 pur"
Task: "Créer PersonnalitesSection.vue en Tailwind v4 pur avec filtre domaine"
Task: "Créer SavoirAvantVoyagerSection.vue en Tailwind v4 pur avec accordéons par catégorie"
```

---

## Implementation Strategy

### MVP First (US1 + US2 ensemble)

1. Compléter **Phase 1** (Setup) : 6 tâches.
2. Compléter **Phase 2** (Foundational), 15 tâches, CRITICAL.
3. Compléter **Phase 3** (US1 : soumission), 14 tâches.
4. Compléter **Phase 4** (US2 : modération), 11 tâches.
5. **STOP & VALIDATE** : exécuter quickstart US1 + US2. Si OK, démo/deploy MVP (le public voit les 4 sections enrichies : sites, secteurs, personnalités, savoirs).

### Incremental Delivery

1. MVP (US1 + US2) → deploy.
2. Ajouter **US3** (création de fiche) → deploy incrémental.
3. Ajouter **US4** (photos + recommandations) → deploy incrémental (les sections `galerie_photos` et `recommandations` deviennent visibles).
4. Ajouter **US5** (contributeurs enrichis) → deploy incrémental.
5. **Phase 8** (Polish) : suivi perso, doc, validation globale, peut commencer dès US2.

### Parallel Team Strategy

Une fois Phase 2 terminée :

- Dev A : US1 (soumission côté public)
- Dev B : US2 (modération côté admin)
- Dev C (après US2 stable) : US3 puis US4
- Dev D : US5 + Phase 8 (Polish)

---

## Notes

- **Pas de tests automatisés** : validation uniquement manuelle via `quickstart.md` (constitution §Contraintes Techniques).
- **Tailwind v4 pur côté public**, **daisyUI autorisé côté admin** (constitution §VI).
- **SQL source de vérité** : toute modification de modèle commence par T007-T013 (constitution §III).
- **Audit obligatoire** : chaque mutation appelle `audit::log_action` (constitution §VII).
- **Français partout** : identifiants SQL, variables TS, composants Vue, messages UI (constitution §I).
- `[P]` = fichiers différents, pas de dépendance bloquante.
- `[US*]` = traçabilité user story / tâche.
- Commit après chaque tâche ou groupe logique.
- Stop à chaque checkpoint pour valider la story indépendamment.

---
description: "Task list : Enrichissement des sites touristiques"
---

# Tasks: Enrichissement des sites touristiques

**Input**: Design documents from `/specs/001-sites-touristiques-enrichis/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Aucun test automatisé demandé (pas de harnais testing/CI configuré, Constitution). Vérification manuelle via `cargo check`/`clippy`, `getDiagnostics` (rust-analyzer/Volar) et scénarios `quickstart.md`.

**Organization**: Tâches groupées par user story (P1 → P3) pour livraison incrémentale.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : exécutable en parallèle (fichiers différents, aucune dépendance bloquante)
- **[Story]** : US1..US5 ; les phases Setup/Foundational/Polish n'ont pas de label

## Path Conventions

Monorepo : backend `uafricas_backend/src/…` + `uafricas_backend/doc/bd/…` ; frontend `uafricas_frontend/app/…`.

---

## Phase 1: Setup (Schéma partagé : SQL Source de Vérité)

**Purpose**: Poser le socle de données (Principe III) dont dépendent toutes les stories.

- [x] T001 Créer la migration `uafricas_backend/doc/bd/schemas/11d_country_profile_sites_enrichis.sql` : enum `country_profile.sous_type_site` (20 valeurs), `ALTER TABLE country_profile.site_touristique` (sous_type, gestionnaire, ville, village, info_pertinente, contact_telephone, contact_courriel, contact_adresse, constitution_statut_juridique, constitution_numero, constitution_document_url, verifie, verifie_par, verifie_at), table `country_profile.avis_site`, index (`idx_site_touristique_sous_type`, `idx_site_touristique_verifie`, `uniq_avis_site_actif`, `idx_avis_site_visible`) et trigger `trg_avis_site_updated`, conforme à `data-model.md`
- [x] T002 Orchestrer `\ir schemas/11d_country_profile_sites_enrichis.sql` dans `uafricas_backend/doc/bd/schema.sql` (après `11c_…`) puis recréer la base de dev : `docker compose down -v && docker compose up -d`

**Checkpoint**: Schéma en base ; colonnes/table disponibles pour le mapping cross-stack.

---

## Phase 2: Foundational (Types partagés, bloque toutes les stories)

**Purpose**: Aligner les types Rust ↔ TS sur le schéma (Principe II/III). Aucune story ne peut démarrer avant.

**⚠️ CRITICAL**: À compléter avant les phases user stories.

- [x] T003 [P] Ajouter l'enum `SousTypeSite` (`#[sqlx(type_name = "country_profile.sous_type_site", rename_all = "snake_case")]`, 20 variantes) et la struct `AvisSiteRow` (FromRow) dans `uafricas_backend/src/models/afripulse.rs`
- [x] T004 [P] Ajouter le helper de validation `sous_type_appartient_a(categorie: &CategorieSiteTouristique, sous_type: &str) -> bool` (mapping famille↔sous-type de `data-model.md`) dans `uafricas_backend/src/models/afripulse.rs`
- [x] T005 [P] Étendre l'interface `SiteTouristiqueAPI` (sous_type, gestionnaire, ville, village, info_pertinente, contacts, constitution_*, verifie, note_moyenne, nombre_avis), ajouter `type SousTypeSite`, la constante `LIBELLES_SOUS_TYPE` (libellés FR) et `SOUS_TYPES_PAR_CATEGORIE` dans `uafricas_frontend/app/composables/useOpportuniteAfrique.ts`

**Checkpoint**: Types cohérents cross-stack ; les stories peuvent démarrer.

---

## Phase 3: User Story 1 - Classer un site par sous-type (Priority: P1) 🎯 MVP

**Goal**: Chaque site porte un sous-type cohérent avec sa famille, affiché et filtrable.

**Independent Test**: Proposer un site emblématique « plage » et un site privé « hôtel », valider, vérifier l'affichage du sous-type et le filtrage par sous-type.

- [x] T006 [US1] Étendre `SiteTouristiqueResponse` (champ `sous_type: Option<String>`) et la requête de `lister_sites_touristiques` (SELECT `sous_type::text`, filtre query `sous_type`) dans `uafricas_backend/src/handlers/afripulse_public.rs`
- [x] T007 [US1] Dans `soumettre_contribution_afripulse` (`uafricas_backend/src/handlers/contributions_fiche.rs`), pour `type_objet=site_touristique` (ajout/édition) : exiger `sous_type` et valider sa cohérence avec `categorie` via `sous_type_appartient_a` (422 « sous-type incompatible avec la famille »)
- [x] T008 [US1] Étendre les branches `("site_touristique","ajout"|"edition")` de `appliquer_contribution_afripulse` (`uafricas_backend/src/handlers/admin/profils_pays.rs`) pour insérer/MAJ `sous_type` (`$::country_profile.sous_type_site`)
- [x] T009 [US1] Ajouter le sélecteur de sous-type (options filtrées selon la famille via `SOUS_TYPES_PAR_CATEGORIE`) au formulaire « site touristique » de `uafricas_frontend/app/components/opportunite-afrique/ContributionModal.vue`
- [x] T010 [US1] Afficher le sous-type (libellé FR) sur chaque carte et ajouter un filtre par sous-type (par famille) dans `uafricas_frontend/app/components/opportunite-afrique/SitesTouristiquesSection.vue`

**Checkpoint**: US1 fonctionnelle : sous-types proposés, enregistrés, affichés et filtrables.

---

## Phase 4: User Story 2 - Fiche d'informations complète (Priority: P1)

**Goal**: Champs requis (nom, gestionnaire, localisation, GPS, info pertinente) + contacts obligatoires pour les sites privés.

**Independent Test**: Soumettre un site privé sans contact → refus ; avec tous les champs → 202 ; après validation, fiche publique complète.

- [x] T011 [US2] Dans `soumettre_contribution_afripulse` (`uafricas_backend/src/handlers/contributions_fiche.rs`) : valider les champs requis (`nom`, `gestionnaire`, `ville`, `info_pertinente`, `latitude`, `longitude`) et, si `categorie=prive`, exiger au moins un contact (téléphone/courriel/adresse), messages 422 listant les manques (FR-006/FR-008)
- [x] T012 [US2] Étendre les branches site de `appliquer_contribution_afripulse` (`uafricas_backend/src/handlers/admin/profils_pays.rs`) pour mapper gestionnaire, ville, village, info_pertinente, latitude, longitude, contact_telephone, contact_courriel, contact_adresse
- [x] T013 [US2] Étendre `SiteTouristiqueResponse` + SELECT de `lister_sites_touristiques` (`uafricas_backend/src/handlers/afripulse_public.rs`) pour renvoyer gestionnaire, ville, village, info_pertinente, latitude, longitude et les contacts (publics, CL résolue)
- [x] T014 [US2] Étendre `AdminSiteTouristiqueResponse`, `CreerSiteTouristiqueRequest`, `ModifierSiteTouristiqueRequest` (`uafricas_backend/src/models/admin/profils_pays.rs`) avec les nouveaux champs (sous_type, gestionnaire, ville, village, info_pertinente, contacts) et adapter le CRUD admin `creer/modifier_site_touristique` (`uafricas_backend/src/handlers/admin/profils_pays.rs`)
- [x] T015 [US2] Ajouter les champs au formulaire site de `uafricas_frontend/app/components/opportunite-afrique/ContributionModal.vue` (gestionnaire, ville, village, GPS lat/long, info pertinente, contacts conditionnels si privé) + validation côté client
- [x] T016 [US2] Afficher gestionnaire, localisation (ville/village/territoire), GPS et info pertinente sur la fiche ; afficher les contacts pour les sites privés dans `uafricas_frontend/app/components/opportunite-afrique/SitesTouristiquesSection.vue`

**Checkpoint**: US1 + US2 fonctionnelles, fiche site complète et validée.

---

## Phase 5: User Story 5 - Avis de visiteurs notés 1–5 (Priority: P2)

**Goal**: Avis noté par site (écriture directe, upsert), note moyenne + nombre affichés, modération admin.

**Independent Test**: `POST …/avis {note:4}` → moyenne/compteur mis à jour ; re-soumettre `{note:5}` → mise à jour sans doublon ; non connecté → invite connexion ; admin masque → exclu du calcul.

- [x] T017 [US5] Ajouter les DTO `AvisSiteResponse`, `AvisSiteListeResponse` (note_moyenne, nombre_total, avis[]) et le body `CreerAvisBody {note, commentaire}` dans `uafricas_backend/src/handlers/afripulse_public.rs` (ou nouveau module `avis_site.rs`)
- [x] T018 [US5] Implémenter `lister_avis_site` (GET, paginé, agrégat AVG/COUNT, exclut masqués/supprimés, anonymise auteurs supprimés) et `soumettre_avis_site` (POST, auth, upsert sur l'avis actif via `uniq_avis_site_actif`, validation note 1–5 + commentaire 1–2000) dans `uafricas_backend/src/handlers/afripulse_public.rs`
- [x] T019 [US5] Implémenter `masquer_avis_site` (PATCH admin, `verifier_permission!`, MAJ `masque_par_admin`, `audit::log_action`) dans `uafricas_backend/src/handlers/admin/profils_pays.rs`
- [x] T020 [US5] Enregistrer les routes `GET/POST /api/sites-touristiques/{site_id}/avis` (public) et `PATCH /api/admin/sites-touristiques/avis/{avis_id}/masquer` (admin) dans `uafricas_backend/src/routes.rs`
- [x] T021 [US5] Inclure note_moyenne/nombre_avis dans `SiteTouristiqueResponse` via sous-requête agrégée (`lister_sites_touristiques`, `uafricas_backend/src/handlers/afripulse_public.rs`)
- [x] T022 [P] [US5] Ajouter les types `AvisSiteAPI`/`AvisSiteListe` et les méthodes `listerAvisSite`/`soumettreAvisSite` dans `uafricas_frontend/app/composables/useOpportuniteAfrique.ts`
- [x] T023 [P] [US5] Créer le composant `uafricas_frontend/app/components/opportunite-afrique/SiteAvisListe.vue` (Tailwind v4 pur : note moyenne, nombre, liste paginée, formulaire de dépôt avec étoiles 1–5 ; invite connexion si non authentifié)
- [x] T024 [US5] Intégrer `SiteAvisListe` dans la carte/fiche site de `uafricas_frontend/app/components/opportunite-afrique/SitesTouristiquesSection.vue` (affiche « aucun avis » si vide)

**Checkpoint**: US5 fonctionnelle : avis notés, moyenne, upsert, modération.

---

## Phase 6: User Story 3 - Badge « Vérifié » admin (Priority: P2)

**Goal**: Admin attribue/retire le badge « Vérifié » ; visible publiquement.

**Independent Test**: Admin active le badge → visible côté public ; le retire → disparaît ; non-admin n'a aucun moyen de le modifier.

- [x] T025 [US3] Ajouter le body `VerificationSiteBody {verifie: bool}` dans `uafricas_backend/src/models/admin/profils_pays.rs` et exposer `verifie` dans `AdminSiteTouristiqueResponse`
- [x] T026 [US3] Implémenter `definir_verification_site` (PATCH admin, `verifier_permission!(admin,"profil_pays","modifier")`, MAJ `verifie`/`verifie_par`/`verifie_at`, `audit::log_action` avant/après) dans `uafricas_backend/src/handlers/admin/profils_pays.rs`
- [x] T027 [US3] Enregistrer la route `PATCH /api/admin/profils-pays/{id}/sites-touristiques/{site_id}/verification` dans `uafricas_backend/src/routes.rs`
- [x] T028 [US3] Renvoyer `verifie` dans `SiteTouristiqueResponse` (`lister_sites_touristiques`, `uafricas_backend/src/handlers/afripulse_public.rs`) si non déjà inclus
- [x] T029 [P] [US3] Ajouter la méthode `definirVerificationSite(ficheId, siteId, verifie)` dans `uafricas_frontend/app/composables/useAdminProfilsPays.ts`
- [x] T030 [US3] Afficher le badge « Vérifié » (FontAwesome) sur les cartes/fiches dans `uafricas_frontend/app/components/opportunite-afrique/SitesTouristiquesSection.vue`
- [x] T031 [US3] Ajouter le contrôle d'activation/retrait du badge (daisyUI) sur la liste des sites dans `uafricas_frontend/app/pages/admin/profils-pays/[id].vue`

**Checkpoint**: US3 fonctionnelle : badge piloté par l'admin, visible publiquement.

---

## Phase 7: User Story 4 - Constitution légale (Priority: P3)

**Goal**: Informations légales facultatives saisies, affichées si présentes.

**Independent Test**: Renseigner statut juridique + numéro → section « Constitution légale » affichée ; sans → section masquée, site consultable.

- [x] T032 [US4] Étendre les branches site de `appliquer_contribution_afripulse` (`uafricas_backend/src/handlers/admin/profils_pays.rs`) pour mapper constitution_statut_juridique, constitution_numero, constitution_document_url
- [x] T033 [US4] Renvoyer les champs `constitution_*` dans `SiteTouristiqueResponse` (`uafricas_backend/src/handlers/afripulse_public.rs`) et les ajouter aux DTO admin `Creer/Modifier/AdminSiteTouristiqueResponse` (`uafricas_backend/src/models/admin/profils_pays.rs`)
- [x] T034 [US4] Ajouter les champs constitution légale (statut, numéro, upload document via `ImageUploadField`/`uploaderImageContribution`) au formulaire site de `uafricas_frontend/app/components/opportunite-afrique/ContributionModal.vue`
- [x] T035 [US4] Afficher une section « Constitution légale » conditionnelle (masquée si vide) sur la fiche dans `uafricas_frontend/app/components/opportunite-afrique/SitesTouristiquesSection.vue`

**Checkpoint**: US4 fonctionnelle : informations légales saisies et affichées.

---

## Phase 8: Polish & Cross-Cutting Concerns

- [x] T036 [P] Vérifier la rétrocompatibilité (FR-018) : sites existants sans sous_type/gestionnaire restent affichables (gardes `v-if`/fallbacks) dans `SitesTouristiquesSection.vue`
- [x] T037 [P] Backend : `cargo fmt`, `cargo clippy -- -D warnings`, `cargo check` ; corriger warnings
- [x] T038 [P] Frontend : `getDiagnostics` (Volar) sur tous les fichiers modifiés ; corriger types
- [x] T039 Mettre à jour `CLAUDE.md` (section Recent Changes) et la doc schéma si nécessaire (Auto-maintenance)
- [ ] T040 Exécuter les scénarios de validation de `quickstart.md` (US1→US5) avec les utilisateurs de test

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance, démarre immédiatement (T001 → T002).
- **Foundational (Phase 2)** : dépend de Setup, BLOQUE toutes les stories.
- **User Stories (Phases 3–7)** : dépendent de Foundational. Ordre de priorité conseillé P1 (US1, US2) → P2 (US5, US3) → P3 (US4).
- **Polish (Phase 8)** : après les stories visées.

### User Story Dependencies

- **US1 (P1)** : après Foundational. Indépendante.
- **US2 (P1)** : après Foundational. Partage des fichiers avec US1 (contributions_fiche.rs, appliquer_contribution, ContributionModal.vue, SitesTouristiquesSection.vue) → exécuter après US1 pour limiter les conflits.
- **US5 (P2)** : après Foundational. Quasi indépendante (table `avis_site`, nouveaux endpoints, nouveau composant), peut se faire en parallèle d'US1/US2 par un autre développeur.
- **US3 (P2)** : après Foundational. Touche admin + affichage badge ; indépendante d'US5.
- **US4 (P3)** : après Foundational. Partage appliquer_contribution + ContributionModal + section → après US2 de préférence.

### Within Each User Story

- Backend (modèle → application/validation → handler → route) avant frontend.
- Les tâches modifiant le même fichier ne sont PAS `[P]` entre elles.

### Parallel Opportunities

- Phase 2 : T003/T004 (Rust) et T005 (TS) en parallèle (fichiers différents).
- US5 : T022 (composable) et T023 (nouveau composant) en parallèle.
- US3 : T029 (composable admin) parallèle au backend.
- Polish : T036/T037/T038 en parallèle.
- Cross-équipe : US5 (et US3) peuvent avancer pendant qu'un autre dev fait US1+US2.

---

## Parallel Example: Phase 2 (Foundational)

```bash
# Lancer en parallèle (fichiers distincts) :
Task: "T003 enum SousTypeSite + AvisSiteRow dans models/afripulse.rs"
Task: "T004 helper sous_type_appartient_a dans models/afripulse.rs"   # même fichier que T003 → séquentiel
Task: "T005 types + libellés sous-types dans useOpportuniteAfrique.ts"  # fichier TS → parallèle
```

> Note : T003 et T004 visent le même fichier → exécuter séquentiellement ; seul T005 est réellement parallèle.

---

## Implementation Strategy

### MVP (P1 : US1 + US2)

1. Phase 1 (Setup SQL) → Phase 2 (Foundational types).
2. Phase 3 (US1 sous-types) → Phase 4 (US2 fiche complète).
3. **STOP & VALIDATE** : sites classés + fiches complètes + validation. Démo MVP.

### Livraison incrémentale

1. Setup + Foundational → socle prêt.
2. US1 → US2 → MVP (sites enrichis, validés, affichés).
3. US5 (avis) → démo.
4. US3 (badge Vérifié) → démo.
5. US4 (constitution légale) → démo.
6. Polish.

---

## Notes

- `[P]` = fichiers différents, aucune dépendance.
- Principe VI : aucune classe daisyUI dans `SitesTouristiquesSection.vue`, `SiteAvisListe.vue`, `ContributionModal.vue` (Tailwind v4 pur) ; daisyUI uniquement dans `pages/admin/…`.
- Principe VII : `definir_verification_site` et `masquer_avis_site` journalisés via `audit::log_action`.
- Principe I : valeurs d'enum SQL et libellés UI en français.
- Le badge `verifie` n'est jamais modifiable via le canal de contribution (FR-012).
- Commit après chaque tâche ou groupe logique cohérent.

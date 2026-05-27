# Tasks: Ressources contribuées Afrolang & fermeture administrative pour abus

**Branch**: `001-ressources-fermeture-session`
**Inputs** : `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/*`, `quickstart.md`

**Tests** : Aucun framework de test configuré dans le projet (cf. CLAUDE.md « No linting, testing, or CI/CD configured yet »). La validation passe par `quickstart.md` (scénarios manuels). Aucune tâche de test automatisé n'est donc générée.

## Format

`- [ ] [TaskID] [P?] [Story?] Description` — chemins absolus depuis la racine repo.

---

## Phase 1 : Setup (infrastructure partagée)

Aucune initialisation nouvelle n'est requise : le monorepo, Docker, sqlx, Actix, Nuxt et toutes les dépendances sont déjà en place. Création des sous-dossiers nécessaires et préparation du schéma uniquement.

- [X] T001 Créer le sous-dossier d'upload `uafricas_backend/uploads/afrolang/ressources_contribuees/` (et son `.gitkeep`) — `uafricas_backend/uploads/afrolang/ressources_contribuees/.gitkeep`
- [X] T002 Vérifier que `actix-files` sert déjà `/uploads/` (pas de changement attendu) et noter le chemin dans `uafricas_backend/src/main.rs` (ligne `Files::new("/uploads", ...)`)

---

## Phase 2 : Foundational (prérequis bloquants pour toutes les user stories)

**⚠️ CRITIQUE** : aucune US ne peut démarrer tant que le DDL SQL et les modules de base ne sont pas en place.

### SQL Source de Vérité (Principe III)

- [X] T003 Ajouter les 3 enums dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` : `afrolang.type_ressource_contribuee`, `afrolang.statut_accompagnateur`, `afrolang.type_evenement_moderation` (conforme `data-model.md` §1)
- [X] T004 Ajouter l'ALTER TABLE `afrolang.salle` (+6 colonnes désactivation/réactivation + CHECK `ck_salle_desactivation_coherente` + index `idx_afrolang_salle_active`) dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (data-model.md §2)
- [X] T005 [P] Créer la table `afrolang.ressource_contribuee` (incluant CHECK `ck_ressource_contribuee_type` + CHECK `ck_ressource_accompagnateur_pas_soi` + 4 indexes) dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (data-model.md §3)
- [X] T006 [P] Créer la table `afrolang.acces_salle_privee` (+ 2 indexes dont UNIQUE partiel) dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (data-model.md §4)
- [X] T007 [P] Créer la table `afrolang.evenement_moderation_salle` (+ CHECK `ck_moderation_motif_fermeture` + index chrono) dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (data-model.md §5)
- [X] T008 Reset+recompose Docker (`docker compose down -v && docker compose up -d`) pour appliquer le schéma SQL via `docker-init.sh` ; vérifier la création des nouvelles tables via Adminer (`http://localhost:8088`)

### Modèles Rust partagés

- [X] T009 [P] Créer `uafricas_backend/src/models/ressource_contribuee.rs` : enums `TypeRessourceContribuee` + `StatutAccompagnateur` (dérives `sqlx::Type`, `Serialize`, `Deserialize`), struct `RessourceContribuee` (FromRow complet), struct `AuteurLight`, struct `AccompagnateurPublicInfo`, DTOs `RessourceContribueeResponse` et `RessourceContribueeAdminResponse` (data-model.md §6)
- [X] T010 [P] Créer `uafricas_backend/src/models/admin/sessions_moderation.rs` : enum `TypeEvenementModeration`, struct `EvenementModerationSalle` (FromRow), DTOs `EvenementModerationResponse` + `FermetureAdminRequest` + `ReactivationRequest`
- [X] T011 Déclarer les nouveaux modules dans `uafricas_backend/src/models/mod.rs` et `uafricas_backend/src/models/admin/mod.rs` (`pub mod ressource_contribuee; pub mod sessions_moderation;`)
- [X] T012 Étendre `uafricas_backend/src/models/afrolang.rs` : ajouter struct `DesactivationAdminInfo { desactivee_at: DateTime<Utc>, motif: Option<String> }` et propager dans `SalleResponse` / `SalleDetailResponse` / `AdminSalleDetailResponse` un champ `desactivee_admin: Option<DesactivationAdminInfo>` (motif uniquement pour admin)

### Services partagés

- [X] T013 [P] Créer `uafricas_backend/src/services/rate_limit_ressources.rs` exposant `pub async fn compter_ressources_recentes(db: &PgPool, auteur_id: Uuid, salle_id: Uuid) -> Result<i64, sqlx::Error>` (COUNT sur 24h glissantes, conforme research.md §4)
- [X] T014 [P] Créer `uafricas_backend/src/services/youtube_url.rs` exposant `pub fn extraire_id_youtube(url: &str) -> Option<String>` (regex sur `youtube.com/watch?v=`, `youtu.be/`, `embed/`, `shorts/`, valide 11 chars) — research.md §2
- [X] T015 [P] Étendre `uafricas_backend/src/services/livekit_moderation.rs` avec `pub async fn fermer_session_admin(room_name: &str, motif_public: &str) -> Result<(), AppError>` qui diffuse DataPacket RELIABLE `{type:'admin', subtype:'session_fermee', motif_public}` puis appelle `RoomServiceClient::delete_room(room_name)` (research.md §5)
- [X] T016 Déclarer les nouveaux services dans `uafricas_backend/src/services/mod.rs`
- [X] T017 Helper privé `a_acces_salle_privee_actif(db, salle_privee_id, utilisateur_id) -> bool` dans `uafricas_backend/src/handlers/afrolang.rs` (contracts/public-salle-privee-acces.md)

### Notifications afrolang.*

- [X] T018 Recenser les types de notifications existants dans `uafricas_backend/src/services/notifications.rs` (ou équivalent) et ajouter les 6 nouveaux types : `afrolang.accompagnateur.recommandation_recue`, `afrolang.accompagnateur.acceptee`, `afrolang.accompagnateur.refusee`, `afrolang.accompagnateur.retiree`, `afrolang.session.fermee_admin`, `afrolang.salle.desactivee_admin`, `afrolang.salle.reactivee_admin` (constantes + helpers de construction du payload)

**Checkpoint** : le schéma SQL est en place, les modèles compilent, les services partagés sont disponibles → les US peuvent démarrer.

---

## Phase 3 : User Story 1 — Partage de ressources contribuées au niveau salle (P1) 🎯 MVP

**Objectif** : tout utilisateur authentifié peut déposer un document, une vidéo YouTube, un lien ou recommander un accompagnateur ; les ressources sont cumulées au niveau salle ; consentement explicite pour les accompagnateurs.

**Test indépendant** : exécuter les scénarios A, B, C, E de `quickstart.md` (ajout 4 variants, persistance multi-sessions, accès salle privée, rate-limit).

### Backend — public ressources contribuées

- [X] T019 [US1] Étendre `uafricas_backend/src/handlers/afrolang.rs` (handler `verifier_code_acces` existant) : en cas de succès, exécuter `INSERT INTO afrolang.acces_salle_privee ... ON CONFLICT DO NOTHING` + `audit::log_action("CREATE", "afrolang", "acces_salle_privee", ...)` lors de la première validation (contracts/public-salle-privee-acces.md)
- [X] T020 [US1] Étendre `uafricas_backend/src/handlers/afrolang.rs` (handler `modifier_code_acces` existant `PATCH /salles-privees/{id}/code-acces`) : ajouter dans la même transaction `UPDATE acces_salle_privee SET revoque_at = NOW() WHERE salle_privee_id = $1 AND revoque_at IS NULL` + audit `UPDATE`
- [X] T021 [US1] Créer le fichier `uafricas_backend/src/handlers/afrolang_ressources.rs` avec la structure et la déclaration de routage des handlers ci-dessous (squelette + `use` + signature)
- [X] T022 [US1] Implémenter `GET /api/afrolang/salles/{salle_id}/ressources-contribuees` dans `uafricas_backend/src/handlers/afrolang_ressources.rs` : pagination (page/limit/type), JOIN auteur, filtre visibilité accompagnateur (`acceptee` OU acteur), filtre salle privée → 403 si `a_acces_salle_privee_actif()=false` (contracts/public-ressources.md)
- [X] T023 [US1] Implémenter `POST /api/afrolang/salles/{salle_id}/ressources-contribuees` (variant `document`, multipart) dans `uafricas_backend/src/handlers/afrolang_ressources.rs` : validation MIME + extension + taille ≤ 20 Mo, `sanitize_filename`, stockage `./uploads/afrolang/ressources_contribuees/<uuid>/`, rate-limit `services::rate_limit_ressources::compter_ressources_recentes`, contrôle salle non désactivée admin, audit `CREATE`
- [X] T024 [US1] Implémenter `POST /api/afrolang/salles/{salle_id}/ressources-contribuees` (variant `video_youtube`, JSON) dans `uafricas_backend/src/handlers/afrolang_ressources.rs` : `services::youtube_url::extraire_id_youtube`, 400 si invalide, audit
- [X] T025 [US1] Implémenter `POST /api/afrolang/salles/{salle_id}/ressources-contribuees` (variant `lien_web`, JSON) dans `uafricas_backend/src/handlers/afrolang_ressources.rs` : validation URL https, audit
- [X] T026 [US1] Implémenter `POST /api/afrolang/salles/{salle_id}/ressources-contribuees` (variant `accompagnateur`, JSON) dans `uafricas_backend/src/handlers/afrolang_ressources.rs` : vérifier `membre_recommande_id != auteur`, état utilisateur recommandé = `actif`, motif ≥ 20 chars, INSERT avec `statut_accompagnateur='en_attente'`, notification `afrolang.accompagnateur.recommandation_recue` au membre recommandé, audit
- [X] T027 [US1] Implémenter `DELETE /api/afrolang/ressources-contribuees/{id}` dans `uafricas_backend/src/handlers/afrolang_ressources.rs` : autorisation auteur OU admin plateforme, soft-delete `deleted_at + supprime_par`, audit `DELETE`
- [X] T028 [US1] Créer `uafricas_backend/src/handlers/afrolang_accompagnateur.rs` (ou regrouper dans `afrolang_ressources.rs` si simplicité préférée) avec 4 endpoints : `GET /api/afrolang/accompagnateur/recommandations-recues` (paginé filtre statut), `POST /{id}/accepter`, `POST /{id}/refuser` (motif facultatif), `POST /{id}/retirer-consentement` (contracts/public-accompagnateur.md). Chaque mutation : audit `UPDATE` + notification correspondante (`acceptee` / `refusee` / `retiree`) à l'auteur
- [X] T029 [US1] Déclarer les 2 nouveaux modules handlers dans `uafricas_backend/src/handlers/mod.rs`
- [X] T030 [US1] Câbler toutes les nouvelles routes (lecture + écriture + accompagnateur) dans `uafricas_backend/src/routes.rs` (sous le scope `/api/afrolang/...`)
- [X] T031 [US1] Si nécessaire, étendre `uafricas_backend/src/errors.rs` avec les codes erreur métier : `salle_privee_acces_requis`, `salle_desactivee_admin`, `rate_limit_ressources`, `url_youtube_invalide`, `action_non_autorisee`, `statut_incompatible`, `retrait_non_autorise`

### Frontend — composables US1

- [X] T032 [P] [US1] Créer `uafricas_frontend/app/composables/useAfrolangRessources.ts` : types TS (`TypeRessourceContribuee`, `StatutAccompagnateur`, `AuteurLight`, `AccompagnateurPublicInfo`, `RessourceContribueeAPI`, filtres), méthodes `listerRessourcesContribuees(salleId, filtres)`, `ajouterDocument(salleId, formData)`, `ajouterVideoYoutube(salleId, payload)`, `ajouterLienWeb(salleId, payload)`, `recommanderAccompagnateur(salleId, payload)`, `supprimerRessource(id)` (data-model.md §7)
- [X] T033 [P] [US1] Créer `uafricas_frontend/app/composables/useAfrolangAccompagnateur.ts` : méthodes `listerRecommandationsRecues(filtres)`, `accepter(id)`, `refuser(id, motifFacultatif)`, `retirerConsentement(id)` + état réactif `mesRecommandationsEnAttente: Ref<number>` (pour badge NavBar)
- [X] T034 [US1] Étendre `uafricas_frontend/app/composables/useAfrolang.ts` : exposer un computed `salleDesactiveeAdmin: Ref<DesactivationAdminInfoAPI | null>` issu du payload salle, et `peutContribuerRessource: Ref<boolean>` (combinaison `userActif && !salleDesactivee && (sallePublique || aAccesSallePrivee)`)

### Frontend — composants US1 (Tailwind v4 pur — Principe VI)

- [X] T035 [P] [US1] Créer `uafricas_frontend/app/components/afrolang/RessourceContribueeCard.vue` (Tailwind v4) : rendu conditionnel par `type` (document → icon FontAwesome + bouton Télécharger ; video_youtube → iframe embed `https://www.youtube.com/embed/<id>` + vignette ; lien_web → carte avec preview URL ; accompagnateur → mini-profil membre + motif + statut). Bouton suppression visible si `peutSupprimer = (auteur === me || isAdmin)`
- [X] T036 [P] [US1] Créer `uafricas_frontend/app/components/afrolang/RessourceContribueeForm.vue` (Tailwind v4) : modal avec 4 onglets (Document / Vidéo / Lien / Accompagnateur), validations client miroirs des contraintes backend (taille fichier, regex YouTube, motif ≥ 20, recherche autocomplete membre), soumissions via `useAfrolangRessources`
- [X] T037 [US1] Créer `uafricas_frontend/app/components/afrolang/RessourcesContribueesPanel.vue` (Tailwind v4) : section avec header « Ressources contribuées par la communauté », liste paginée `RessourceContribueeCard`, bouton flottant « + Ajouter » (ouvre `RessourceContribueeForm`) désactivé si `!peutContribuerRessource` avec tooltip explicatif. Filtre par type (chips). Distinction visuelle nette d'avec la section « Ressources officielles » modérée (feature 005)
- [X] T038 [P] [US1] Créer `uafricas_frontend/app/components/afrolang/AccompagnateurRecommandationBanner.vue` (Tailwind v4) : bannière compacte affichée dans `pages/mon-compte/recommandations-accompagnateur.vue` pour chaque recommandation en attente avec deux boutons « Accepter » / « Refuser » (modal motif facultatif). Inclus un état vide neutre

### Frontend — pages US1

- [X] T039 [US1] Modifier `uafricas_frontend/app/pages/afrolang/session/[id].vue` : intégrer `<RessourcesContribueesPanel :salle-id="salleId" :session-id="sessionId" />` sous (ou à côté de) la liste de ressources officielles existante, avec libellé clairement différencié
- [ ] T040 [US1] Modifier `uafricas_frontend/app/pages/afrolang/session/privee/[id].vue` : mêmes intégration que T039, en passant la `salle_id` parente issue de la salle privée
- [X] T041 [US1] Créer `uafricas_frontend/app/pages/mon-compte/recommandations-accompagnateur.vue` : liste paginée (filtre par statut) des recommandations reçues, intègre `AccompagnateurRecommandationBanner.vue`, gestion du retrait de consentement post-acceptation
- [X] T042 [US1] Étendre `uafricas_frontend/app/layouts/default.vue` (NavBar) : badge numérique « recommandations en attente » sur le menu utilisateur, lien direct vers `/mon-compte/recommandations-accompagnateur`, alimenté par `useAfrolangAccompagnateur.mesRecommandationsEnAttente`

**Checkpoint US1** : déployable indépendamment. Les 4 variants de ressources fonctionnent, la persistance multi-sessions est garantie, l'accès aux salles privées est mémorisé, le workflow accompagnateur respecte le consentement, le rate-limit s'applique.

---

## Phase 4 : User Story 2 — Fermeture administrative pour abus (P1)

**Objectif** : un admin plateforme peut fermer une session pour abus → désactivation immédiate de la salle hôte ; réactivation réservée aux admins plateforme uniquement ; notifications participants + admins de salle.

**Test indépendant** : exécuter le scénario D de `quickstart.md` (fermeture, éjection, badge, blocage rejointe, blocage réactivation non-admin, réactivation admin).

### Backend — modération admin

- [X] T043 [US2] Créer `uafricas_backend/src/handlers/admin/sessions_moderation.rs` (squelette + `use` + signatures)
- [X] T044 [US2] Implémenter `POST /api/admin/afrolang/sessions/{session_id}/fermer-admin` dans `uafricas_backend/src/handlers/admin/sessions_moderation.rs` : transaction (a) `UPDATE session etat='terminee'`, (b) `UPDATE salle desactivee_admin_*`, (c) `INSERT evenement_moderation_salle`, (d) hors transaction `services::livekit_moderation::fermer_session_admin` puis envoi notifications participants (`session.fermee_admin` sans motif) + admins de salle/créateur (`salle.desactivee_admin` avec motif), (e) 2 lignes audit. 409 si déjà désactivée. Motif 10..1000 chars (contracts/admin-moderation.md)
- [X] T045 [US2] Implémenter `POST /api/admin/afrolang/salles/{salle_id}/reactiver` dans `uafricas_backend/src/handlers/admin/sessions_moderation.rs` : transaction (UPDATE salle + INSERT evenement_moderation), notification `salle.reactivee_admin` aux admins de salle/créateur, 2 lignes audit. 409 si salle non désactivée
- [X] T046 [US2] Étendre les handlers de lecture salle existants (`obtenir_salle` / `lister_salles` dans `uafricas_backend/src/handlers/afrolang.rs`) pour inclure le DTO `desactivee_admin: Option<DesactivationAdminInfo>` (motif=null en public, motif rempli pour appelant admin). Mettre à jour les CTE / json_agg en conséquence
- [X] T047 [US2] Étendre `uafricas_backend/src/handlers/afrolang.rs` (handlers de jointure : `rejoindre_session`, `demarrer_session`, `creer_session`, `verifier_code_acces`) : refuser avec 403 `salle_desactivee_admin` si `salle.desactivee_admin_at IS NOT NULL`
- [X] T048 [US2] Déclarer le module dans `uafricas_backend/src/handlers/admin/mod.rs`
- [X] T049 [US2] Câbler les 2 nouvelles routes admin dans `uafricas_backend/src/routes.rs` sous le scope `/api/admin/afrolang/...` avec le middleware `est_admin_plateforme`
- [X] T050 [US2] Ajouter dans le handler public `DELETE /api/afrolang/ressources-contribuees/{id}` (T027) la vérification additionnelle « admin plateforme = OK » et propager `acteur_admin: bool` dans l'audit (déjà prévu, contracts/admin-moderation.md)

### Frontend — composables US2

- [X] T051 [P] [US2] Méthode `fermerSessionAdmin(sessionId, motif)` ajoutée à `useAdminSessions.ts` (composable existant, pas de nouveau fichier)
- [X] T052 [US2] Étendre `uafricas_frontend/app/composables/useAdminAfrolangSalles.ts` : méthodes `reactiverSalle(salleId, commentaire)`, types `DesactivationAdminInfoAPI`
- [ ] T053 [US2] Étendre `uafricas_frontend/app/composables/useAfrolang.ts` : attacher dans la session active un listener LiveKit DataPacket pour `{type:'admin', subtype:'session_fermee'}` qui (a) déclenche un toast persistant, (b) pousse une notification locale, (c) déclenche la sortie propre de la session

### Frontend — composants US2

- [X] T054 [P] [US2] Créer `uafricas_frontend/app/components/afrolang/SalleDesactiveeBadge.vue` (Tailwind v4 pur) : badge `border-2 border-red-700/40 bg-red-50 text-red-700` avec icône FontAwesome `faBan` et libellé « Désactivée par administration », tooltip motif si admin
- [ ] T055 [P] [US2] Créer `uafricas_frontend/app/components/afrolang/SessionFermeeAdminToast.vue` (Tailwind v4 pur) : toast persistant pleine largeur, contenu « Session fermée par l'administration. Contactez le support si besoin. », bouton fermeture explicite
- [X] T056 [P] [US2] Créer `uafricas_frontend/app/components/admin/afrolang/SessionFermetureModal.vue` (daisyUI v5) : modal avec textarea motif (compteur 10/1000), confirmation, appel `useAdminSessions.fermerSessionAdmin`, toast succès/erreur
- [X] T057 [P] [US2] Créer `uafricas_frontend/app/components/admin/afrolang/SalleReactivationModal.vue` (daisyUI v5) : modal avec textarea commentaire facultatif (0/1000), confirmation, appel `useAdminAfrolangSalles.reactiverSalle`

### Frontend — pages US2

- [ ] T058 [US2] Créer `uafricas_frontend/app/pages/admin/afrolang/sessions/index.vue` (daisyUI) : tableau paginé des sessions en cours/planifiées avec filtres salle/état, colonnes (titre, salle, modérateur, participants, démarrée_at, actions), bouton « Fermer pour abus » par ligne ouvrant `SessionFermetureModal`
- [X] T059 [US2] Modifier `uafricas_frontend/app/pages/admin/salles/[id].vue` : bouton « Réactiver la salle » conditionnel + bandeau « Salle désactivée par administration » avec motif intégrés
- [X] T060 [US2] Badge + bandeau de désactivation intégrés dans `components/afrolang/SalleCard.vue` ; bouton « Démarrer/Rejoindre » désactivé quand `desactivee_admin !== null`
- [ ] T061 [US2] Intégrer `<SessionFermeeAdminToast />` dans `uafricas_frontend/app/pages/afrolang/session/[id].vue` et `uafricas_frontend/app/pages/afrolang/session/privee/[id].vue`, déclenché par le listener LiveKit du T053

**Checkpoint US2** : déployable. La fermeture admin coupe la session en < 5 s, désactive la salle, notifie correctement, et l'IAM admin plateforme est seul autorisé à réactiver.

---

## Phase 5 : User Story 3 — Historique de modération (P2)

**Objectif** : les administrateurs (plateforme + nommés de salle) peuvent consulter l'historique chronologique des fermetures et réactivations d'une salle.

**Test indépendant** : depuis l'onglet « Historique de modération » d'une salle ayant subi ≥ 1 cycle fermeture→réactivation, vérifier la liste chronologique avec auteur, type d'action, motif, horodatage (scénario D étape 9 + scénario F).

### Backend

- [X] T062 [US3] Implémenter `GET /api/admin/afrolang/salles/{salle_id}/historique-moderation` dans `uafricas_backend/src/handlers/admin/sessions_moderation.rs` : pagination, JOIN admin (lookup `iam.utilisateur` pour nom/prénom), tri `created_at DESC` (contracts/admin-moderation.md)
- [X] T063 [US3] Câbler la route dans `uafricas_backend/src/routes.rs`

### Frontend

- [X] T064 [US3] Étendre `uafricas_frontend/app/composables/useAdminAfrolangSalles.ts` : méthode `listerHistoriqueModeration(salleId, page, limit)` + interface `EvenementModerationAPI`
- [X] T065 [US3] Créer `uafricas_frontend/app/components/admin/afrolang/SalleHistoriqueModerationPanel.vue` (daisyUI v5) : tableau chronologique avec badges colorés par `type_action` (`fermeture_admin` rouge, `reactivation_admin` vert), colonnes (date, admin, type, motif tronqué + tooltip plein), pagination
- [X] T066 [US3] Modifier `uafricas_frontend/app/pages/admin/salles/[id].vue` : ajouter un onglet « Historique de modération » embarquant `<SalleHistoriqueModerationPanel :salle-id="id" />`

**Checkpoint US3** : déployable en complément de US2 (gouvernance + transparence).

---

## Phase 6 : Polish & cross-cutting

- [X] T067 [P] Audit vérifié : `admin/sessions_moderation.rs` émet 2 lignes d'audit par fermeture/réactivation (UPDATE salle + CREATE evenement_moderation_salle), avec before/after JSONB conformes
- [X] T068 [P] FontAwesome : `faGavel` ajouté ; `faBan`, `faCircleCheck`, `faCircleXmark` déjà présents
- [ ] T069 [P] Régression visuelle manuelle : exécuter le `quickstart.md` complet (scénarios A à F) sur l'environnement local, capturer un screenshot par scénario, lister tout écart
- [ ] T070 [P] Vérifier qu'aucun composant public n'utilise `daisyUI` (classes `btn`, `modal`, `tabs`, `badge` daisyUI) — Principe VI ; grep ciblé sur `uafricas_frontend/app/components/afrolang/` et les 3 pages publiques modifiées
- [X] T071 Mettre à jour `CLAUDE.md` (section « Recent Changes ») avec un résumé de la feature livrée
- [ ] T072 Pousser la branche `001-ressources-fermeture-session` et préparer la PR avec lien vers `quickstart.md` pour la grille de recette

---

## Dépendances entre user stories

```text
Setup (P1+P2 SQL/modèles/services) → US1 (ressources) ─┐
                                                       ├──> Polish
                                  → US2 (modération) ──┤
                                  → US3 (historique) ──┘  (dépend de US2 pour avoir des données mais peut livrer en parallèle)
```

- **US1 et US2 sont indépendantes** : aucune ne bloque l'autre techniquement. Elles partagent la modification de `afrolang.salle` (T004) et les modèles communs, déjà couverts en Phase 2 Foundational.
- **US3 dépend de US2** uniquement pour avoir des évènements à afficher ; les contrats/code peuvent être développés en parallèle, puis testés une fois US2 livrée.

## Opportunités de parallélisation

| Vague | Tâches parallélisables |
|---|---|
| Foundational SQL | T005, T006, T007 (tables indépendantes dans le même fichier — coordonner les patchs successifs ou un seul commit) |
| Foundational Rust | T009, T010, T013, T014, T015 (fichiers distincts) |
| US1 frontend | T032, T033, T035, T036, T038 (fichiers distincts) |
| US2 frontend | T051, T054, T055, T056, T057 (fichiers distincts) |
| Polish | T067, T068, T069, T070 (lectures/diff indépendantes) |

## Stratégie d'implémentation

**MVP recommandé** : Phase 1 + Phase 2 + Phase 3 (US1 complète). À ce stade, la fonctionnalité « ressources contribuées » est livrable en production. La modération admin (US2) peut suivre dans un second incrément ; l'historique (US3) clôt la feature.

**Découpage de commits suggéré** :
1. **Commit 1** : T001-T018 (SQL + modèles + services partagés) — pas de fonctionnalité visible, vérifier que `cargo build` passe.
2. **Commit 2** : T019-T031 (backend US1) — endpoints publics + extensions salle privée.
3. **Commit 3** : T032-T042 (frontend US1) — UI ressources contribuées + workflow accompagnateur.
4. **Commit 4** : T043-T061 (US2 backend + frontend).
5. **Commit 5** : T062-T066 (US3 historique).
6. **Commit 6** : T067-T072 (polish, audit, CLAUDE.md, PR).

## Critères de validation globale

- [ ] Tous les scénarios `quickstart.md` (A→F) passent à la première exécution.
- [ ] Aucune nouvelle dépendance Cargo / npm ajoutée (vérifier `git diff Cargo.toml package.json pnpm-lock.yaml`).
- [ ] `cargo build --release` sans warning nouveau.
- [ ] Aucune classe daisyUI dans `uafricas_frontend/app/components/afrolang/` (Principe VI).
- [ ] Toutes les mutations possèdent une ligne d'audit (Principe VII).
- [ ] Branche `001-ressources-fermeture-session` poussée + PR ouverte avec lien vers ce `tasks.md` et `quickstart.md`.

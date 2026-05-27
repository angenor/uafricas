# Research & Décisions techniques : Demande pour devenir expert

**Feature**: 001-demande-expertise | **Date**: 2026-05-24

Ce document consolide l'analyse de l'existant et les décisions techniques. Aucun marqueur NEEDS CLARIFICATION ne subsiste (les 3 clarifications de session ont été intégrées à la spec).

## État de l'existant (audit du code)

| Élément | Existe ? | Détail |
|---------|----------|--------|
| Table `iam.expertise` | ✅ | Statut enum `en_attente`/`valide`/`refuse`, `valide_par`, `date_validation`, `deleted_at`, FK `utilisateur_id` |
| Endpoint soumission | ✅ | `POST /api/experts/candidature` (`creer_candidature`) — JWT, bloque doublon via `deja_candidat` |
| Liste publique filtrée | ✅ | `lister_experts` filtre `e.statut='valide' AND e.deleted_at IS NULL AND u.deleted_at IS NULL` |
| Composable soumission | ✅ | `useExperts.creerCandidature()` (POST `/api/experts/candidature`) |
| Mise à jour profil | ✅ | `PUT /api/auth/profil` (`modifier_profil`) + `POST /api/auth/profil/photo` |
| Middleware admin | ✅ | Extracteur `AdminUtilisateur` + macro `verifier_permission!(admin, ressource, action)` |
| Service audit | ✅ | `audit::log_action` non-bloquant |
| SMTP / email | ✅ | `email.rs` (`envoyer_email_verification`) + pattern async `envoyer_*_async` |
| **Endpoints admin de modération expertise** | ❌ | Aucun handler/route admin — à créer |
| **Page formulaire candidature** | ❌ | Aucune page frontend — à créer |
| **Suivi candidature côté membre** | ❌ | Aucun endpoint `GET /moi` ni UI — à créer |
| **Email de décision** | ❌ | À ajouter dans `email.rs` |

## Décisions

### D1 — Re-soumission après refus : index unique partiel + soft-delete

**Décision** : Remplacer la contrainte `utilisateur_id UUID NOT NULL UNIQUE` par `NOT NULL` + un **index unique partiel** `CREATE UNIQUE INDEX ... ON iam.expertise(utilisateur_id) WHERE deleted_at IS NULL`. À la re-soumission, l'ancienne demande `refuse` est soft-deletée (`deleted_at = NOW()`) puis une nouvelle ligne `en_attente` est insérée.

**Rationale** : La contrainte `UNIQUE` totale actuelle bloquerait toute nouvelle insertion même après soft-delete. L'index partiel autorise plusieurs lignes historiques par utilisateur tant qu'une seule est active (`deleted_at IS NULL`). Aligné sur la convention SQL du projet (soft deletion) et sur le choix de clarification (archivage + nouvelle demande).

**Alternatives rejetées** :
- Réécrire la même ligne en `en_attente` : perd l'historique des décisions, contraire à la clarification.
- Garder `UNIQUE` total + UPDATE : impossible de conserver l'historique.

### D2 — Réutilisation des colonnes de décision existantes

**Décision** : Réutiliser `valide_par` (admin ayant traité) et `date_validation` (date de décision, validation OU refus) déjà présentes. Ajouter **une seule** colonne `commentaire_admin TEXT` (obligatoire en cas de refus, NULL sinon).

**Rationale** : Principe V (Simplicité) — migration minimale. Sémantique « date de décision » couvre validation et refus sans colonnes redondantes.

**Alternatives rejetées** : Ajouter `traite_par`/`traite_le` (comme biblio) → doublonnerait les colonnes existantes.

### D3 — Notification par email uniquement (pas de table notification)

**Décision** : Notifier le candidat par email à la validation et au refus, via deux nouvelles fonctions dans `email.rs` (`envoyer_email_expertise_validee`, `envoyer_email_expertise_refusee`) appelées en mode « fire-and-forget » async (comme `envoyer_verification_async`). L'email de refus inclut `commentaire_admin`.

**Rationale** : Clarification de session = email uniquement. Évite d'introduire une table `notification_expertise` (contrairement au pattern biblio in-app), ce qui simplifie le périmètre (Principe V).

**Alternatives rejetées** : Notification in-app (table dédiée) → hors périmètre clarifié.

### D4 — Extension de `ModifierProfilRequest` avec `pays_residence_id`

**Décision** : Ajouter `pays_residence_id: Option<Uuid>` à `ModifierProfilRequest` et au handler `modifier_profil` (SET conditionnel, validation FK `shared.pays`). Le formulaire frontend appelle les endpoints profil existants (`PUT /api/auth/profil` pour fonction + pays, `POST /api/auth/profil/photo` pour la photo) avant/avec la soumission de candidature.

**Rationale** : FR-003a exige la mise à jour de photo/fonction/pays. `fonction` et `ville` sont déjà gérés ; seul `pays_residence_id` manque. La fiche publique expert affiche `u.pays_residence_id` → indispensable. Réutilise les endpoints profil plutôt que de dupliquer la logique dans la candidature (Principe V).

**Alternatives rejetées** : Stocker le pays dans `iam.expertise` → duplication d'une donnée déjà portée par `iam.utilisateur`, contraire au modèle existant (la liste experts joint `shared.pays` via `u.pays_residence_id`).

### D5 — Endpoints admin via `AdminUtilisateur` + permission dédiée

**Décision** : Nouveau handler `handlers/admin/expertise.rs` avec 4 endpoints (liste, détail, valider PATCH, rejeter PATCH) utilisant l'extracteur `AdminUtilisateur` et `verifier_permission!(admin, "expertise", "voir"|"valider")`. Ajouter au seed `15_seed.sql` les permissions `expertise.voir` et `expertise.valider`.

**Rationale** : Cohérent avec tous les autres modules admin (biblio, annonces, projets…). Le rôle `super_admin` a déjà le wildcard `all.all` donc couvre immédiatement les nouvelles permissions ; le seed explicite documente l'intention et permet une attribution fine à d'autres rôles.

**Alternatives rejetées** : Réutiliser une permission générique `utilisateur.*` → moins lisible, mélange les responsabilités.

### D6 — Ajustement de `creer_candidature` (logique de re-soumission)

**Décision** : Modifier le contrôle `deja_candidat` pour ne bloquer (409) que s'il existe une demande active **`en_attente` ou `valide`** (`deleted_at IS NULL`). S'il existe une demande `refuse` active, la soft-deleter dans la même transaction avant d'insérer la nouvelle.

**Rationale** : Implémente FR-006 (une seule demande active) + FR-015 (re-soumission après refus). Transaction pour l'atomicité (soft-delete + insert).

### D7 — Parcours d'authentification du lien

**Décision** : Le lien « Apporter mon expertise » pointe vers `/devenir-expert`. La page applique une garde : si non connecté, redirection vers la connexion avec paramètre de retour (`?redirect=/devenir-expert`) ; après connexion, retour au formulaire.

**Rationale** : FR-002. Réutilise le mécanisme de redirection post-login existant (`useAuth`).

## Bonnes pratiques appliquées

- **sqlx paramétré** partout (Principe IV) — aucune concaténation de SQL utilisateur.
- **Transaction** pour les mutations multi-étapes (re-soumission, validation/refus + audit).
- **Audit** `log_action("VALIDATE"|"REJECT", "iam", "expertise", id)` sur chaque décision (Principe VII).
- **Email async fire-and-forget** : ne bloque pas la réponse HTTP de la décision admin.
- **Tailwind v4 pur** pour `devenir-expert.vue` ; **daisyUI** pour les pages admin (Principe VI).
- **Cohérence des enums** : domaines et situations professionnelles déjà mappés frontend↔DB via `mapper_domaine_db` / `mapper_domaine_frontend`.

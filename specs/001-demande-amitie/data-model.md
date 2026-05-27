# Data Model — Demande d'amitié & messagerie

**Feature**: `001-demande-amitie` | **Schéma**: `social` (nouveau, `schemas/29_social.sql`)

Conventions projet : PK `UUID v4`, `TIMESTAMPTZ`, soft delete `deleted_at` où pertinent, snake_case français, enums PostgreSQL. FK cross-schema vers `iam.utilisateur(id)`.

---

## Enum

### `social.statut_demande_amitie`
`en_attente` | `acceptee` | `refusee` | `annulee`

### `social.type_notification_social`
`demande_recue` | `demande_acceptee`

---

## Entités

### `social.demande_amitie`
Sollicitation orientée d'un membre vers un autre.

| Champ | Type | Contraintes |
|-------|------|-------------|
| id | UUID | PK, default gen_random_uuid() |
| demandeur_id | UUID | FK `iam.utilisateur` NOT NULL |
| destinataire_id | UUID | FK `iam.utilisateur` NOT NULL |
| statut | social.statut_demande_amitie | NOT NULL default `en_attente` |
| created_at | TIMESTAMPTZ | NOT NULL default now() |
| traite_at | TIMESTAMPTZ | NULL (rempli à accept/refus/annul) |
| deleted_at | TIMESTAMPTZ | NULL |

- **CHECK** `ck_demande_pas_soi`: `demandeur_id <> destinataire_id` (FR-002).
- **Index unique partiel** `uq_demande_active`: `(demandeur_id, destinataire_id) WHERE statut = 'en_attente' AND deleted_at IS NULL` — empêche les doublons en attente (FR-003).
- **Index** sur `destinataire_id WHERE statut='en_attente'` (liste reçues), `demandeur_id WHERE statut='en_attente'` (liste envoyées).
- **Index** sur `(demandeur_id, created_at)` pour le rate-limit (FR-014, Décision 6).
- **Transitions**: `en_attente` → `acceptee` (FR-007) | `refusee` (FR-008) | `annulee` (FR-010). État terminal sinon.

### `social.amitie`
Relation mutuelle symétrique. Ordre canonique `utilisateur_a_id < utilisateur_b_id` (Décision 4).

| Champ | Type | Contraintes |
|-------|------|-------------|
| id | UUID | PK |
| utilisateur_a_id | UUID | FK `iam.utilisateur` NOT NULL |
| utilisateur_b_id | UUID | FK `iam.utilisateur` NOT NULL |
| created_at | TIMESTAMPTZ | NOT NULL default now() |

- **CHECK** `ck_amitie_ordre`: `utilisateur_a_id < utilisateur_b_id`.
- **Unique** `(utilisateur_a_id, utilisateur_b_id)` (FR-003).
- Suppression physique au retrait d'ami (FR-012) ou au blocage (FR-013) — pas de soft delete (l'historique relationnel n'est pas requis ; la conversation, elle, est conservée mais verrouillée).

### `social.blocage`
Relation orientée empêchant sollicitation et communication.

| Champ | Type | Contraintes |
|-------|------|-------------|
| id | UUID | PK |
| bloqueur_id | UUID | FK `iam.utilisateur` NOT NULL |
| bloque_id | UUID | FK `iam.utilisateur` NOT NULL |
| created_at | TIMESTAMPTZ | NOT NULL default now() |

- **CHECK** `ck_blocage_pas_soi`: `bloqueur_id <> bloque_id`.
- **Unique** `(bloqueur_id, bloque_id)`.
- **Index** sur `bloque_id` (vérifier « suis-je bloqué par X »).
- Suppression physique au déblocage (FR-013).

### `social.conversation`
Fil privé entre deux amis. Ordre canonique (Décision 4). Conservée même après rupture d'amitié (historique lisible) mais verrouillée pour de nouveaux messages (FR-025).

| Champ | Type | Contraintes |
|-------|------|-------------|
| id | UUID | PK |
| utilisateur_a_id | UUID | FK `iam.utilisateur` NOT NULL |
| utilisateur_b_id | UUID | FK `iam.utilisateur` NOT NULL |
| created_at | TIMESTAMPTZ | NOT NULL default now() |
| dernier_message_at | TIMESTAMPTZ | NULL (tri des conversations) |

- **CHECK** `ck_conversation_ordre`: `utilisateur_a_id < utilisateur_b_id`.
- **Unique** `(utilisateur_a_id, utilisateur_b_id)` — une conversation par paire.
- **Index** sur `dernier_message_at DESC`.

### `social.message`
Message texte dans une conversation.

| Champ | Type | Contraintes |
|-------|------|-------------|
| id | UUID | PK |
| conversation_id | UUID | FK `social.conversation` NOT NULL, ON DELETE CASCADE |
| expediteur_id | UUID | FK `iam.utilisateur` NOT NULL |
| contenu | TEXT | NOT NULL, **CHECK** `char_length(contenu) BETWEEN 1 AND 2000` (FR-027) |
| created_at | TIMESTAMPTZ | NOT NULL default now() |
| lu_at | TIMESTAMPTZ | NULL (FR-024 ; non-NULL = lu par le destinataire) |
| deleted_at | TIMESTAMPTZ | NULL (FR-028 ; soft delete = « message supprimé ») |

- **Index** sur `(conversation_id, created_at DESC)` — pagination de l'historique.
- **Index** sur `(conversation_id, lu_at) WHERE lu_at IS NULL` — comptage des non-lus.

### `social.notification`
Notification relationnelle (Décision 5).

| Champ | Type | Contraintes |
|-------|------|-------------|
| id | UUID | PK |
| destinataire_id | UUID | FK `iam.utilisateur` NOT NULL |
| type | social.type_notification_social | NOT NULL |
| demande_id | UUID | FK `social.demande_amitie` NULL |
| acteur_id | UUID | FK `iam.utilisateur` NULL (qui a déclenché) |
| lu | BOOLEAN | NOT NULL default false |
| created_at | TIMESTAMPTZ | NOT NULL default now() |

- **Index** sur `(destinataire_id, lu)`.

---

## Relations (résumé)

```
iam.utilisateur 1──* social.demande_amitie *──1 iam.utilisateur   (demandeur / destinataire)
iam.utilisateur *──* iam.utilisateur  via social.amitie           (symétrique, canonique)
iam.utilisateur *──* iam.utilisateur  via social.blocage          (orienté)
social.conversation 1──* social.message
iam.utilisateur 1──* social.notification
```

## Règles de validation transverses

- **R1** (FR-001/015): émetteur et destinataire DOIVENT être `etat='actif'` et `deleted_at IS NULL`.
- **R2** (FR-003): refus d'insertion si amitié déjà existante OU demande en attente dans un sens ou l'autre.
- **R3** (FR-009): si une demande inverse `en_attente` existe au moment de l'envoi → la passer `acceptee` et créer l'amitié (auto-acceptation croisée), dans une **transaction**.
- **R4** (FR-013): blocage entre A et B → suppression de l'amitié et de toute demande active entre eux + verrouillage de la conversation (transaction). Toute insertion de message refusée si un blocage existe dans un sens.
- **R5** (FR-022/025): insertion de message autorisée uniquement si une **amitié active** existe entre l'expéditeur et le destinataire de la conversation.
- **R6** (FR-026): toute lecture d'amis/conversations/messages est filtrée sur l'utilisateur authentifié — aucun accès aux relations d'autrui.

## Orchestration & migration

- Ajouter `\ir schemas/29_social.sql` dans `uafricas_backend/doc/bd/schema.sql` (après `26_notifications.sql`).
- Init auto en dev via `docker-init.sh` (recréation volume). En production : migration manuelle SSH+psql (cf. procédure de déploiement).
- Aucune nouvelle permission IAM (pas d'écran admin). Aucune modification de `iam.utilisateur`.

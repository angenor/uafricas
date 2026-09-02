# Phase 1 : Modèle de données

Schéma cible : `media_content` (source de vérité, Principe III). Migration idempotente : `uafricas_backend/doc/bd/schemas/09b_media_content_evenements_streaming.sql`, à inclure dans `doc/bd/schema.sql` (via `\ir schemas/09b_media_content_evenements_streaming.sql`) **après** les tables `media_content` et **avant** `12_audit.sql` / `13_contraintes_inter_schemas.sql` / `14_triggers.sql`.

Aucune modification de la table `media_content.evenement` n'est nécessaire (`format`, `lien_en_ligne`, `cree_par`, `date_heure_debut/fin` existent déjà). Aucun média n'est persisté.

## Entité : `media_content.evenement_session`

Session de direct (instance de diffusion temps réel) rattachée à un événement. Calque allégé de `afrolang.session`. Une seule session `en_cours` par événement (FR-015).

| Colonne | Type | Contraintes | Rôle |
|---------|------|-------------|------|
| `id` | UUID | PK, `DEFAULT uuid_generate_v4()` | Identifiant ; `room_name` LiveKit = `evenement-{id}` |
| `evenement_id` | UUID | NOT NULL, FK → `media_content.evenement(id)` ON DELETE CASCADE | Événement diffusé |
| `etat` | VARCHAR(30) | NOT NULL DEFAULT `'en_cours'`, CHECK ∈ (`'en_cours'`,`'terminee'`) | Cycle de vie persisté |
| `organisateur_id` | UUID | NOT NULL | Ouvreur (= `evenement.cree_par`) ; xref `iam.utilisateur` |
| `demarre_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Instant d'ouverture |
| `termine_at` | TIMESTAMPTZ | NULL | Instant de clôture |
| `duree_secondes` | INT | NULL | Calculé à la clôture |
| `max_participants` | INT | NOT NULL DEFAULT 100 | Capacité (D8 ; ≥ SC-004) |
| `nombre_participants_pic` | INT | NOT NULL DEFAULT 0 | Pic de présents simultanés |
| `arret_securite_at` | TIMESTAMPTZ | NOT NULL | Échéance d'arrêt de sécurité absolu (D6) |
| `noeud_id` | VARCHAR(120) | NULL | Nœud SFU (routage multi-VPS, parité afrolang) |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | |
| `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | |

**Index** :
- `CREATE UNIQUE INDEX uq_evenement_session_active ON media_content.evenement_session(evenement_id) WHERE etat = 'en_cours';` → garantit une seule session active par événement (FR-015).
- `CREATE INDEX idx_evenement_session_evenement ON media_content.evenement_session(evenement_id);`
- `CREATE INDEX idx_evenement_session_etat ON media_content.evenement_session(etat) WHERE etat = 'en_cours';`

> Pas de `deleted_at` : les sessions sont éphémères et conservées comme historique (parité avec `afrolang.session`). L'annulation de l'événement (`evenement.etat='annule'`) force la clôture de la session (FR-016).

## Entité : `media_content.evenement_session_participant`

Présence d'un membre dans une session, avec son rôle (détermine le droit de diffuser) et son signal « lever la main ». Calque de `afrolang.session_participant`.

| Colonne | Type | Contraintes | Rôle |
|---------|------|-------------|------|
| `id` | UUID | PK, `DEFAULT uuid_generate_v4()` | |
| `session_id` | UUID | NOT NULL, FK → `media_content.evenement_session(id)` ON DELETE CASCADE | |
| `utilisateur_id` | UUID | NOT NULL | xref `iam.utilisateur` |
| `role` | VARCHAR(30) | NOT NULL DEFAULT `'spectateur'`, CHECK ∈ (`'organisateur'`,`'intervenant'`,`'spectateur'`) | Détermine `can_publish` |
| `main_levee` | BOOLEAN | NOT NULL DEFAULT FALSE | Demande de parole en cours (FR-022) |
| `main_levee_at` | TIMESTAMPTZ | NULL | Horodatage de la demande (ordre d'affichage) |
| `rejoint_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Dernière entrée |
| `quitte_at` | TIMESTAMPTZ | NULL | Sortie (NULL = présent) ; recompté à la reconnexion |
| `duree_secondes` | INT | NULL | Cumul à la sortie |

**Contraintes / index** :
- `UNIQUE (session_id, utilisateur_id)` → un enregistrement par membre et par session (reconnexion = UPDATE `quitte_at = NULL`, FR-014).
- `CREATE INDEX idx_evenement_session_participant_session ON media_content.evenement_session_participant(session_id);`
- `CREATE INDEX idx_evenement_session_participant_main ON media_content.evenement_session_participant(session_id) WHERE main_levee = TRUE;`

## Règles de validation (issues des exigences)

- **Ouverture** (FR-004) : `evenement.format ∈ {en_ligne, hybride}` (FR-001/019), `evenement.etat = 'publie'`, demandeur = `cree_par`, `NOW() >= date_heure_debut − 15min` (D6), aucune session `en_cours` existante (sinon rejoindre l'existante, FR-015).
- **Jointure** (FR-002/003) : demandeur connecté (401 sinon) ET (`= organisateur` OU `est_inscrit`) ; sinon 403. Refus 409 si présents actifs `>= max_participants` (D8, organisateur exempté).
- **Promotion/rétrogradation/retrait** (FR-009/010) : demandeur = organisateur de la session uniquement.
- **Lever la main** (FR-022) : rôle `spectateur` uniquement ; toggle `main_levee`.
- **Clôture** (FR-011) : demandeur = organisateur ; ou clôture automatique paresseuse si `NOW() > arret_securite_at` (D6) ; ou cascade si `evenement.etat` passe à `annule` (FR-016).
- **Événement annulé** (FR-016) : aucune ouverture/jointure ; session active forcée à `terminee`.

## Transitions d'état

### Session (`etat`)
```
(absente) ──ouvrir (organisateur, fenêtre OK)──▶ en_cours ──clôturer / arrêt sécurité / annulation──▶ terminee
                                                    │
                                                    └─ « en attente de l'organisateur » = état DÉRIVÉ
                                                       (aucune session en_cours + dans la fenêtre), non persisté
```
- `statut_direct` exposé au frontend (dérivé à la lecture, jamais stocké) :
  - `indisponible` : `format` non diffusable, ou `NOW() < debut − 15min`, ou événement `annule`.
  - `en_attente` : dans la fenêtre, aucune session `en_cours`.
  - `en_direct` : une session `en_cours` existe (et `NOW() <= arret_securite_at`).
  - `termine` : la session a été clôturée (ou `arret_securite_at` dépassé).

### Participant (`role`)
```
spectateur ──promouvoir (organisateur)──▶ intervenant
intervenant ──rétrograder (organisateur)──▶ spectateur
{spectateur|intervenant} ──retirer (organisateur)──▶ (quitte_at = NOW(), déconnecté du SFU)
organisateur : rôle fixe (l'ouvreur)
main_levee : spectateur lève/baisse ; remis à FALSE lors d'une promotion
```

## Mapping cross-stack (Principe II)

| SQL (`evenement_session`) | Rust `FromRow` | DTO Response (`Serialize`) | TS (`useEvenements`) |
|---|---|---|---|
| `id` | `id: Uuid` | `session_id` (json inline du token) | `session_id: string` |
| `etat` | `etat: String` | : (dérivé `statut_direct`) | `statut_direct: 'indisponible'\|'en_attente'\|'en_direct'\|'termine'` |
| `role` (participant) | `role: String` | `role` | `role: 'organisateur'\|'intervenant'\|'spectateur'` |
| `main_levee` | `main_levee: bool` | `main_levee` | `main_levee: boolean` |
| (LiveKit) | : | `token`,`room_name`,`livekit_url` | `token`,`room_name`,`livekit_url` |

`room_name` = `format!("evenement-{}", session_id)` ; identité LiveKit = `utilisateur_id.to_string()` ; nom affiché = `"{prenom} {nom}"` (parité afrolang).

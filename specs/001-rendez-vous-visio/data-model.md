# Phase 1 : Data Model : Rendez-vous en visioconférence

Source de vérité = schéma SQL (Principe III). Fichier : `uafricas_backend/doc/bd/schemas/31_social_rendez_vous.sql` (idempotent), intégré à l'orchestrateur `schema.sql`.

## Enum `social.statut_rendez_vous`

| Valeur | Sens |
|--------|------|
| `propose` | Créneau proposé, en attente de réponse de la partie `tour_id`. |
| `accepte` | Créneau figé et accepté ; salle visio possible dans la fenêtre. |
| `refuse` | Proposition refusée (terminal). |
| `annule` | Annulé par l'une des parties (terminal). |

> « expiré » et « terminé/passé » ne sont **pas** des valeurs persistées, dérivées par calcul (cf. research §2).

## Table `social.rendez_vous`

| Colonne | Type | Contraintes | Notes |
|---------|------|-------------|-------|
| `id` | `UUID` | PK, `DEFAULT gen_random_uuid()` | Identifiant ; sert aussi de base au peer-id (haché). |
| `initiateur_id` | `UUID` | `NOT NULL`, FK `iam.utilisateur(id)` ON DELETE CASCADE | Créateur d'origine. |
| `destinataire_id` | `UUID` | `NOT NULL`, FK `iam.utilisateur(id)` ON DELETE CASCADE | Cible d'origine. |
| `sujet` | `VARCHAR(150)` | `NOT NULL`, `CHECK (char_length(sujet) BETWEEN 1 AND 150)` | Titre court obligatoire (FR-006). |
| `description` | `TEXT` | `NULL` | Facultative ; jamais auditée (FR-033). |
| `date_heure` | `TIMESTAMPTZ` | `NOT NULL` | Début prévu (futur à la création/contre-prop). |
| `duree_minutes` | `SMALLINT` | `NOT NULL`, `CHECK (duree_minutes IN (15,30,45,60))` | Créneau prédéfini (FR-007). |
| `statut` | `social.statut_rendez_vous` | `NOT NULL`, `DEFAULT 'propose'` | Machine à états. |
| `tour_id` | `UUID` | `NOT NULL`, FK `iam.utilisateur(id)` ON DELETE CASCADE | Partie devant répondre. |
| `created_at` | `TIMESTAMPTZ` | `NOT NULL`, `DEFAULT NOW()` | |
| `updated_at` | `TIMESTAMPTZ` | `NOT NULL`, `DEFAULT NOW()` | Mis à jour à chaque transition. |
| `deleted_at` | `TIMESTAMPTZ` | `NULL` | Suppression logique (Principe III). |

**Contraintes table** :
- `CONSTRAINT ck_rdv_pas_soi CHECK (initiateur_id <> destinataire_id)` (FR-009).
- `tour_id` doit être l'un des deux participants, garanti applicativement (le SQL ne le contraint pas pour rester simple).

**Index** :
- `idx_rdv_initiateur (initiateur_id) WHERE deleted_at IS NULL`
- `idx_rdv_destinataire (destinataire_id) WHERE deleted_at IS NULL`
- `idx_rdv_tour (tour_id) WHERE statut = 'propose' AND deleted_at IS NULL`, filtre « en attente de ma réponse ».
- `idx_rdv_date (date_heure)` : tri/fenêtres « à venir » / « passés ».

## Transitions d'état (machine)

```text
            proposer (FR-011)
                 │  tour_id = destinataire_id
                 ▼
            ┌──────────┐  accepter (tour_id=moi)        ┌──────────┐
            │ propose  │ ─────────────────────────────► │ accepte  │ (créneau figé)
            │          │                                 └────┬─────┘
            │          │  refuser (tour_id=moi)               │ annuler (l'un OU l'autre)
            │          │ ───────────────────► refuse          ▼
            │          │                                    annule
            │          │  contre-proposer (tour_id=moi)
            │          │   → date/heure/durée maj
            │          │   → tour_id bascule vers l'autre
            │          │   → reste 'propose'
            └────┬─────┘
                 │ annuler (l'un OU l'autre, tant que propose)
                 ▼
              annule
```

Dérivations par calcul (lecture seule, pas de transition) :
- **expiré** : `statut='propose' AND date_heure < NOW()` → classé « passés », aucune action.
- **terminé/passé** : `statut='accepte' AND (date_heure + duree_minutes) < NOW()` → classé « passés », plus de « Rejoindre ».

## Règles de validation (backend, FR-006..FR-010, FR-017, FR-034)

| Règle | Quand | Erreur |
|-------|-------|--------|
| amitié active + pas de blocage | proposer, accepter, refuser, contre-proposer, annuler, salle | `403 AccesInterdit` |
| `destinataire_id <> moi` | proposer | `400 Validation` |
| sujet non vide ≤150, durée ∈ {15,30,45,60} | proposer, contre-proposer | `400 Validation` |
| `date_heure > NOW()` | proposer, contre-proposer | `400 Validation` |
| `statut='propose' AND tour_id=moi` | accepter, refuser, contre-proposer | `409 Conflit` (verrouillage optimiste, FR-035) |
| `statut IN ('propose','accepte') AND (moi ∈ participants)` | annuler | `409 Conflit` |
| `statut='accepte' AND NOW() ∈ fenêtre` | salle (Rejoindre) | `409 Conflit` (hors fenêtre) |

## Filtres de la vue de gestion (FR-019)

Pour l'utilisateur courant `moi` (participant si `initiateur_id=moi OR destinataire_id=moi`, `deleted_at IS NULL`) :

| Filtre | Condition SQL |
|--------|---------------|
| `attente_moi` | `statut='propose' AND tour_id = moi AND date_heure >= NOW()` |
| `attente_autre` | `statut='propose' AND tour_id <> moi AND date_heure >= NOW()` |
| `a_venir` | `statut='accepte' AND (date_heure + duree_minutes*interval) >= NOW()` |
| `passes` | `statut IN ('refuse','annule') OR (statut='propose' AND date_heure < NOW()) OR (statut='accepte' AND (date_heure + duree_minutes*interval) < NOW())` |

## Mapping cross-stack (Principe II)

| SQL | Rust (`models/rendez_vous.rs`) | TS (`useRendezVous.ts`) |
|-----|-------------------------------|--------------------------|
| `id` | `id: Uuid` | `id: string` |
| `sujet` | `sujet: String` | `sujet: string` |
| `description` | `description: Option<String>` | `description?: string` |
| `date_heure` | `date_heure: DateTime<Utc>` | `dateHeure: string` (ISO) |
| `duree_minutes` | `duree_minutes: i16` | `dureeMinutes: number` |
| `statut` | `statut: String` (enum sérialisé) | `statut: 'propose'\|'accepte'\|'refuse'\|'annule'` |
| `tour_id` | `tour_id: Uuid` | `tourId: string` |
| (dérivé) | : | `etatDerive: 'expire'\|'termine'\|null` (calculé) |
| `MembreLight` (autre) | `autre: MembreLight` | `autre: MembreLightAPI` |

Le DTO de réponse (`RendezVousResponse`) inclut l'`autre` membre (MembreLight), `suis_initiateur`, `mon_tour` (bool), et `peut_rejoindre` (bool, fenêtre) calculés côté backend pour simplifier le frontend.

## Notification cloche (réutilisation, research §5)

`arbre_genealogique.notifications` (existant), pas de modification de schéma. Insertion via `creer_notification(pool, destinataire_id, type, message, lien_action)` :

| Événement | `type` | `message` (exemple, sans contenu sensible) |
|-----------|--------|---------------------------------------------|
| proposé | `rdv_propose` | « {Prénom Nom} vous a proposé un rendez-vous. » |
| accepté | `rdv_accepte` | « {Prénom Nom} a accepté votre rendez-vous. » |
| refusé | `rdv_refuse` | « {Prénom Nom} a refusé votre rendez-vous. » |
| contre-proposé | `rdv_contre_propose` | « {Prénom Nom} a proposé un nouveau créneau. » |
| annulé | `rdv_annule` | « {Prénom Nom} a annulé un rendez-vous. » |

`lien_action` = `/?rdv=1` (ou ancre ouvrant le panneau messagerie sur l'onglet rendez-vous).

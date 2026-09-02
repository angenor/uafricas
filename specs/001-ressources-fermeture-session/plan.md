# Implementation Plan: Ressources contribuées Afrolang & fermeture administrative pour abus

**Branch**: `001-ressources-fermeture-session` | **Date**: 2026-05-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-ressources-fermeture-session/spec.md`

## Summary

Cette feature ajoute deux blocs fonctionnels au domaine Afrolang :

1. **Ressources contribuées au niveau salle**, Toute personne authentifiée peut ajouter à la salle hôte d'une session livestream un document (PDF / DOC / DOCX / ODT ≤ 20 Mo), une vidéo YouTube (URL avec extraction d'ID), un lien web, ou recommander un membre comme « accompagnateur » avec consentement explicite a posteriori (workflow `en_attente` → `acceptee` / `refusee` / `retiree`). Les ressources sont rattachées à la **salle** (corpus cumulatif partagé par toutes les sessions de la salle), distinctes de la table modérée `afrolang.ressource_salle` déjà livrée en feature `005-afrolang-salles`. Visibilité : publique pour les salles publiques ; restreinte aux comptes ayant historiquement validé le code d'accès pour les salles privées (nouvelle table `acces_salle_privee` mémorisant les validations).

2. **Fermeture administrative d'une session pour abus**, Un admin plateforme peut interrompre une session en cours en saisissant un motif obligatoire ; cette action désactive la salle hôte jusqu'à réactivation explicite par un autre admin plateforme. Aucun autre rôle (modérateur attitré, admin de salle publique, créateur de salle privée) ne peut lever cette désactivation. Notifications : (i) admins de salle / créateur reçoivent le motif détaillé, (ii) participants présents reçoivent une notification persistante sans motif. Historique de modération exposé aux admins par salle.

**Approche technique** : SQL first (Principe III) → schéma `afrolang` étendu de 3 nouvelles tables (`ressource_contribuee`, `acces_salle_privee`, `evenement_moderation_salle`) + 6 colonnes de désactivation sur `afrolang.salle` + 3 nouveaux enums. Backend Rust modulaire en deux nouveaux handlers (`afrolang_ressources.rs` public, `admin/sessions_moderation.rs` admin) réutilisant les services existants (`audit::log_action`, notifications afrolang, `services/livekit_moderation.rs` de la feature `001-session-moderation`). Frontend : 1 composable public + extensions composables existants + composants Tailwind v4 pur (public) et daisyUI (admin). Aucune nouvelle dépendance.

## Technical Context

**Language/Version**:
- Backend : Rust Edition 2024
- Frontend : TypeScript / Nuxt 4 / Vue 3 SSR

**Primary Dependencies** (toutes déjà présentes, aucune addition) :
- Backend : Actix-Web 4, actix-multipart, sqlx (PostgreSQL async), uuid, chrono, serde, sanitize-filename, lettre, livekit-api (déjà étendu en feature `001-session-moderation` avec `RoomServiceClient::update_participant` et `send_data`), regex (validation URL YouTube)
- Frontend : Pinia, $fetch, FontAwesome, AOS, pas de nouvelle dépendance npm

**Storage** :
- PostgreSQL 16, schema `afrolang` étendu (3 nouvelles tables `ressource_contribuee`, `acces_salle_privee`, `evenement_moderation_salle` + 6 colonnes ALTER sur `afrolang.salle` + 3 enums)
- Stockage local des documents uploadés : `./uploads/afrolang/ressources_contribuees/` (servi via actix-files sur `/uploads/`)

**Testing** : Non configuré dans le projet (cf. CLAUDE.md). Validation manuelle via `quickstart.md`.

**Target Platform** : Linux server (backend Actix sur port 8080) + Nuxt SSR (frontend sur port 3000).

**Project Type** : Web application monorepo (`uafricas_backend/` + `uafricas_frontend/`).

**Performance Goals** :
- Lecture liste ressources d'une salle : < 200 ms p95 (index `(salle_id) WHERE deleted_at IS NULL` + LIMIT 50).
- Téléversement document 5 Mo : succès en < 5 s sur connexion 10 Mbit/s (SC-003).
- Propagation visibilité ressource après dépôt : < 3 s (SC-002), REST classique, pas de WebSocket dédié pour cette feature.
- Fermeture admin → éjection effective via LiveKit : < 5 s (SC-005), réutilise le canal data déjà ouvert pour la modération de session.

**Constraints** :
- Document : taille ≤ 20 Mo, formats `application/pdf`, `application/msword`, `application/vnd.openxmlformats-officedocument.wordprocessingml.document`, `application/vnd.oasis.opendocument.text` (validation MIME + extension).
- Vidéo : whitelist domaines YouTube (`youtube.com`, `www.youtube.com`, `m.youtube.com`, `youtu.be`) + extraction ID 11 caractères par regex.
- Rate limit : ≤ 10 ressources par utilisateur par salle par 24 h glissantes (compté en base, pas de table dédiée).
- Motif fermeture : 10 ≤ longueur ≤ 1000 caractères.
- Motif recommandation accompagnateur : ≥ 20 caractères.

**Scale/Scope** :
- Ordre de grandeur attendu : ~50-200 ressources/salle active après 1 an d'usage.
- Pagination 20 items/page côté listes publiques, 50 côté admin.
- Endpoints ajoutés : ~9 publics + ~4 admin.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Justification |
|---|---|---|
| **I. Français d'Abord** | ✅ PASS | Tous les noms (tables, colonnes, enums, structs, composants, composables, routes) en français snake_case (SQL/Rust) ou camelCase/PascalCase (TS/Vue). Messages d'erreur et libellés UI en français. |
| **II. Monorepo Cohérent** | ✅ PASS | Modifications livrées en un seul PR couvrant SQL + backend Rust + frontend Nuxt avec types cohérents (struct Rust ↔ interface TS ↔ schema SQL). |
| **III. SQL Source de Vérité** | ✅ PASS | DDL écrit en premier dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (ALTER + nouvelles tables) ; structs `FromRow` et interfaces TS dérivées du SQL. |
| **IV. Sécurité par Défaut** | ✅ PASS | Validation MIME + extension + taille ; whitelist URL YouTube via regex ; sanitize-filename pour uploads ; JWT obligatoire sur tous les `POST/DELETE` ; admin gate via helper `est_admin_plateforme` existant ; aucun secret en dur ; le hash bcrypt du code d'accès reste inchangé. |
| **V. Simplicité (YAGNI)** | ✅ PASS | Réutilise audit (`audit::log_action`), notifications, LiveKit data channel, upload local, helper `est_admin_plateforme`. Pas de nouveau pattern. Pas de WebSocket dédié (REST suffit). Pas de table de rate-limit dédiée (COUNT). |
| **VI. Tailwind v4 (daisyUI back-office)** | ✅ PASS | Composants publics (`RessourcesContribueesPanel`, `RessourceContribueeForm`, etc.) en Tailwind v4 pur. Composants admin (`SessionFermetureModal`, `SalleReactivationModal`, `SalleHistoriqueModerationPanel`) en daisyUI v5. |
| **VII. Audit & Traçabilité** | ✅ PASS | Toutes mutations instrumentées : ajout/suppression ressource, acceptation/refus/retrait accompagnateur, validation code d'accès, fermeture admin, réactivation admin. Champ `before/after` JSONB respecté. |

**Verdict** : ✅ Aucune violation. Section « Complexity Tracking » non requise.

### Re-check post Phase 1

Après design détaillé (data-model + contrats), tous les principes restent satisfaits. Notamment :
- Aucune dépendance externe ajoutée (V).
- Le seul élément potentiellement borderline est la cascade `revoque_at` lors d'un changement de code d'accès (FR-001 option C). Implémentation choisie : un simple `UPDATE acces_salle_privee SET revoque_at = NOW() WHERE salle_privee_id = $1 AND revoque_at IS NULL` dans la transaction de `PATCH /salles-privees/{id}/code-acces` existante. Pas de pattern, pas de hook, pas d'event-bus → conforme YAGNI.

## Project Structure

### Documentation (this feature)

```text
specs/001-ressources-fermeture-session/
├── plan.md              # Ce fichier
├── spec.md              # Spec produit (déjà clarifiée)
├── research.md          # Phase 0 : décisions techniques
├── data-model.md        # Phase 1 : schéma SQL + entités
├── quickstart.md        # Phase 1 : checklist de validation manuelle
├── contracts/           # Phase 1 : contrats HTTP des endpoints
│   ├── public-ressources.md
│   ├── public-accompagnateur.md
│   ├── public-salle-privee-acces.md
│   └── admin-moderation.md
└── checklists/
    └── requirements.md  # Validation qualité spec
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 08b_afrolang.sql                          # ÉDITÉ, ALTER + nouvelles tables + enums
└── src/
    ├── handlers/
    │   ├── afrolang_ressources.rs                # NEW, endpoints publics ressources contribuées
    │   ├── afrolang.rs                           # ÉDITÉ, verifier_code persiste acces_salle_privee ; lecture salle privée contrôlée
    │   ├── admin/
    │   │   ├── sessions_moderation.rs            # NEW, fermeture/réactivation admin + historique
    │   │   └── ressources_contribuees.rs         # NEW, retrait admin d'une ressource
    │   └── mod.rs                                # ÉDITÉ, déclaration des nouveaux modules
    ├── models/
    │   ├── afrolang.rs                           # ÉDITÉ, DTOs étendus (badge désactivation)
    │   ├── ressource_contribuee.rs               # NEW, entités + DTOs
    │   └── admin/sessions_moderation.rs          # NEW, historique modération
    ├── services/
    │   ├── livekit_moderation.rs                 # ÉDITÉ, nouvelle fonction fermer_session_admin
    │   └── rate_limit_ressources.rs              # NEW, helper COUNT par user/salle/24h
    ├── routes.rs                                 # ÉDITÉ, câblage nouvelles routes
    └── errors.rs                                 # ÉDITÉ si nouveaux codes erreur métier

uafricas_frontend/
└── app/
    ├── composables/
    │   ├── useAfrolangRessources.ts              # NEW, public ressources contribuées
    │   ├── useAfrolangAccompagnateur.ts          # NEW, acceptation/refus recommandations
    │   ├── useAfrolang.ts                        # ÉDITÉ, flag désactivation salle + écriture autorisée
    │   ├── useAdminAfrolangSessions.ts           # NEW, modération admin sessions
    │   └── useAdminAfrolangSalles.ts             # ÉDITÉ, historique modération + retrait ressource
    ├── components/
    │   ├── afrolang/
    │   │   ├── RessourcesContribueesPanel.vue           # NEW, Tailwind v4 pur
    │   │   ├── RessourceContribueeForm.vue              # NEW, modal d'ajout (4 onglets)
    │   │   ├── RessourceContribueeCard.vue              # NEW, rendu d'un item
    │   │   ├── AccompagnateurRecommandationBanner.vue   # NEW, bannière notif
    │   │   ├── SalleDesactiveeBadge.vue                 # NEW, badge salle désactivée
    │   │   └── SessionFermeeAdminToast.vue              # NEW, toast post-éjection
    │   └── admin/afrolang/
    │       ├── SessionFermetureModal.vue                # NEW, daisyUI
    │       ├── SalleReactivationModal.vue               # NEW, daisyUI
    │       └── SalleHistoriqueModerationPanel.vue       # NEW, daisyUI tableau chronologique
    ├── pages/
    │   ├── afrolang/session/[id].vue                    # ÉDITÉ, intégration panneau + toast
    │   ├── afrolang/session/privee/[id].vue             # ÉDITÉ, idem
    │   ├── mon-compte/recommandations-accompagnateur.vue # NEW, liste + accepter/refuser
    │   └── admin/afrolang/
    │       ├── sessions/index.vue                       # NEW, liste sessions actives + fermer
    │       └── salles/[id].vue                          # ÉDITÉ : onglet historique modération
    └── layouts/default.vue                              # ÉDITÉ, badge "recommandations en attente"
```

**Structure Decision**: Web application monorepo (Option 2). Conformément à la structure backend/frontend existante. Composants publics dans `components/afrolang/` (Tailwind v4 pur), composants admin dans `components/admin/afrolang/` (daisyUI v5), respect du Principe VI.

## Complexity Tracking

> Aucune violation constitutionnelle. Section vide.

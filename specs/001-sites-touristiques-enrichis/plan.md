# Implementation Plan: Enrichissement des sites touristiques

**Branch**: `001-sites-touristiques-enrichis` | **Date**: 2026-05-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-sites-touristiques-enrichis/spec.md`

## Summary

Enrichir les sites touristiques (emblématiques et privés) de la page `/opportunite-afrique/[id]`
en s'appuyant sur l'infrastructure Afripulse existante (contribution communautaire →
validation admin). On étend la table `country_profile.site_touristique` (sous-type, gestionnaire,
localisation textuelle, contacts, constitution légale, badge `verifie`), on ajoute une table
`country_profile.avis_site` (notes 1–5 par site, écriture directe authentifiée + modération admin),
et on propage le tout côté backend (modèles, handlers publics, application de contribution, endpoint
admin de vérification) et frontend (composable, section Vue, modal de contribution, back-office).

## Technical Context

**Language/Version**: Rust Edition 2024 (backend), TypeScript / Nuxt 4 (Vue 3 SSR) (frontend)  
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde, sanitize-filename, image, lettre (backend) ; Pinia, Tailwind CSS v4, FontAwesome (frontend)  
**Storage**: PostgreSQL 16, schéma `country_profile` (source de vérité — Principe III)  
**Testing**: aucun harnais configuré (Principe — pas de testing/CI/CD imposé) ; vérification manuelle via test users + `cargo check` / Volar diagnostics  
**Target Platform**: Serveur Linux (backend port 8082), web SSR (frontend port 3000)  
**Project Type**: Web application (monorepo `uafricas_backend/` + `uafricas_frontend/`)  
**Performance Goals**: lecture page détail fluide ; agrégat note moyenne/site sans N+1 ; pas d'objectif de débit spécifique  
**Constraints**: site public = Tailwind v4 pur (pas de daisyUI — Principe VI) ; UI/code/SQL en français (Principe I) ; mutations auditées (Principe VII) ; pas de secrets en dur (Principe IV)  
**Scale/Scope**: 54 fiches pays max ; quelques dizaines de sites par fiche ; charge faible à modérée

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Conformité du plan |
|----------|--------------------|
| I. Français d'Abord | Colonnes SQL, enums, structs, types TS, labels UI en français. Valeurs d'enum snake_case français (`plage`, `relief_naturel`, `bar_maquis`…). ✅ |
| II. Monorepo Cohérent | Changement cross-stack (SQL → Rust → TS) livré ensemble ; types TS ↔ structs Rust ↔ enums SQL alignés. ✅ |
| III. SQL Source de Vérité | On part du schéma (`11d_…sql` étendant `country_profile`), puis backend, puis frontend. Soft delete `deleted_at`, UUID v4, TIMESTAMPTZ, enums PG. ✅ |
| IV. Sécurité par Défaut | Écritures authentifiées (JWT) ; badge `verifie` réservé admin (`verifier_permission!`) ; requêtes paramétrées sqlx ; validation des entrées (champs requis, contacts privés, note 1–5, longueur commentaire). ✅ |
| V. Simplicité (YAGNI) | Réutilise le workflow de contribution et le pattern `recommandation_visiteur` ; **un seul** enum `sous_type_site` + validation famille↔sous-type en code ; pas de nouvelle abstraction. ✅ |
| VI. Tailwind v4 (public) | Section publique et modal en Tailwind v4 pur ; daisyUI uniquement côté back-office admin. ✅ |
| VII. Audit & Traçabilité | `audit::log_action` sur le toggle `verifie`, la modération d'avis, et (déjà) l'application de contribution. ✅ |

**Résultat** : PASS — aucune violation. Section Complexity Tracking non requise.

## Project Structure

### Documentation (this feature)

```text
specs/001-sites-touristiques-enrichis/
├── plan.md              # Ce fichier (/speckit.plan)
├── research.md          # Phase 0 — décisions techniques
├── data-model.md        # Phase 1 — entités + DDL
├── quickstart.md        # Phase 1 — mise en route & vérification
├── contracts/           # Phase 1 — contrats d'API
│   ├── public-sites-touristiques.md
│   ├── public-avis-site.md
│   └── admin-sites-verification.md
├── checklists/
│   └── requirements.md  # Checklist qualité spec (déjà créé)
└── tasks.md             # Phase 2 (/speckit.tasks — NON créé ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 11d_country_profile_sites_enrichis.sql   # NOUVEAU — ALTER site_touristique + enum + table avis_site
├── src/
│   ├── models/
│   │   ├── afripulse.rs                          # + enum SousTypeSite, + AvisSiteRow
│   │   ├── contribution_fiche.rs                 # (inchangé — réutilisé)
│   │   └── admin/profils_pays.rs                 # + champs sur Admin/Creer/Modifier SiteTouristique, DTO vérification
│   ├── handlers/
│   │   ├── afripulse_public.rs                   # SiteTouristiqueResponse étendu + agrégat avis ; + handlers avis_site
│   │   ├── contributions_fiche.rs                # validation site enrichi (champs requis, contacts privés, sous-type↔famille)
│   │   └── admin/profils_pays.rs                 # appliquer_contribution_afripulse (branche site étendue) + toggle verifie + modération avis
│   └── routes.rs                                 # + routes avis_site (public) + vérification site (admin)

uafricas_frontend/
└── app/
    ├── composables/
    │   ├── useOpportuniteAfrique.ts              # SiteTouristiqueAPI étendu, SousTypeSite + libellés, AvisSiteAPI, méthodes avis
    │   └── useAdminProfilsPays.ts                # (si existant) toggle vérification + modération avis
    ├── components/opportunite-afrique/
    │   ├── SitesTouristiquesSection.vue          # affichage enrichi (sous-type, gestionnaire, localisation, contacts, badge, légal, avis) + filtre sous-type
    │   ├── ContributionModal.vue                 # formulaire site enrichi (champs requis + contacts + légal)
    │   ├── SiteAvisListe.vue                     # NOUVEAU — liste + moyenne + dépôt d'avis (Tailwind v4)
    │   └── ImageUploadField.vue                  # (réutilisé)
    └── pages/
        ├── opportunite-afrique/[id].vue          # (inchangé ou ajustements mineurs de câblage)
        └── admin/contenus/profils-pays/…         # back-office : badge vérification + modération avis (daisyUI)
```

**Structure Decision** : Application web monorepo (Option 2). On modifie les répertoires réels
ci-dessus. Aucune nouvelle couche : on étend les modules afripulse existants conformément au
Principe V.

## Complexity Tracking

> Aucune violation de la Constitution — section non requise.

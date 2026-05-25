# Quickstart — Enrichissement des sites touristiques

## Pré-requis

```bash
docker compose up -d                 # PostgreSQL + Adminer + LiveKit
# Backend (port 8082) — toujours tuer l'ancien process avant
kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run
# Frontend (port 3000)
pnpm dev
```

Utilisateurs de test : `test-admin@test.com` / `Test1234` · `test-user@test.com` / `Test1234`.

## Ordre d'implémentation (Principe III : SQL → backend → frontend)

1. **SQL** — créer `uafricas_backend/doc/bd/schemas/11d_country_profile_sites_enrichis.sql`
   (enum `sous_type_site`, `ALTER site_touristique`, table `avis_site`, index, trigger) et
   l'orchestrer dans `schema.sql` après `11c_…`. Recréer la base de dev :
   `docker compose down -v && docker compose up -d`.
2. **Backend modèles** — étendre `models/afripulse.rs` (`SousTypeSite`, `AvisSiteRow`) et
   `models/admin/profils_pays.rs` (champs sur Admin/Creer/Modifier SiteTouristique, DTO vérification).
3. **Backend handlers** — `afripulse_public.rs` (SiteTouristiqueResponse étendu + agrégat avis +
   handlers `avis_site`), `contributions_fiche.rs` (validation site enrichi),
   `admin/profils_pays.rs` (branches `site_touristique` de `appliquer_contribution_afripulse`
   étendues, toggle `verifie`, masquage d'avis). Ajouter les routes dans `routes.rs`.
4. **Frontend** — `useOpportuniteAfrique.ts` (types + libellés sous-types + méthodes avis),
   `SitesTouristiquesSection.vue` (affichage enrichi + filtre sous-type + badge),
   `SiteAvisListe.vue` (nouveau, Tailwind v4), `ContributionModal.vue` (formulaire site enrichi),
   back-office (toggle vérification + modération avis, daisyUI).

## Vérifications après chaque étape

- Backend : `cargo check` puis `cargo clippy -- -D warnings` ; `getDiagnostics` (rust-analyzer).
- Frontend : `getDiagnostics` (Volar) sur les fichiers modifiés.

## Scénarios de validation (mapping spec)

| Scénario | Action manuelle |
|----------|-----------------|
| US1 — sous-type | Proposer un site emblématique « plage » + un site privé « hôtel » ; vérifier sous-types proposés selon la famille, affichage et filtre. |
| US2 — champs requis | Soumettre un site privé sans contact → refus ; avec tous les champs → 202 ; après validation admin, fiche complète affichée. |
| US3 — badge Vérifié | Admin : `PATCH …/verification {verifie:true}` → badge visible côté public ; `{verifie:false}` → disparaît. |
| US4 — constitution légale | Renseigner statut juridique + numéro → section « Constitution légale » affichée ; sans → section masquée. |
| US5 — avis | `POST …/avis {note:4}` → moyenne + compteur mis à jour ; re-soumettre `{note:5}` → mise à jour sans doublon ; non connecté → invite connexion ; admin masque un avis → disparaît du calcul. |

## Points d'attention

- **Rétrocompatibilité** (FR-018) : les sites existants (sans sous_type/gestionnaire) doivent rester
  affichables — colonnes nullables, gardes côté frontend.
- **Principe VI** : aucune classe daisyUI dans la section publique ni le modal.
- **Principe VII** : toggle `verifie` et masquage d'avis journalisés via `audit::log_action`.
- **Principe I** : valeurs d'enum et libellés en français.

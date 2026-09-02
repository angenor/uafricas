# Implementation Plan: Marché Africain, acheter, vendre, troquer, donner

**Branch**: `001-marche-achat-vente-troc-don` | **Date**: 2026-05-26 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-marche-achat-vente-troc-don/spec.md`

## Summary

Rendre l'espace « Marché Africain » réellement fonctionnel pour les membres : publication d'annonces (vente / troc / don) avec photos et **mise en ligne immédiate**, mise en relation acheteur↔auteur via la **messagerie privée existante** (rattachée à l'annonce), gestion de ses propres annonces (« Mes annonces », modification, conclusion, suppression), et favoris.

L'infrastructure est largement en place : schéma `marketplace` complet (`annonce`, `annonce_pays`, `annonce_media`, `annonce_favori`), endpoints publics en lecture + CRUD admin complet, et page de listing/détail fonctionnelle. Le travail consiste à **ajouter une couche d'endpoints « membre »** (auth JWT) côté backend, **étendre le composable public** et **brancher l'UI** (formulaire de publication, bouton « Contacter », favoris, « Mes annonces »), plus deux ajustements de schéma minimes.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) ; TypeScript / Nuxt 4 (Vue 3 SSR) (frontend)  
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), actix-multipart, `image` crate, service interne `image_validation`, JWT (`jwt.rs`), `audit::log_action`, SSE (`messagerie_sse::RegistreSse`) ; Pinia, Tailwind CSS v4 (pur), FontAwesome (frontend)  
**Storage**: PostgreSQL 16, schémas `marketplace` (source de vérité, Principe III) et `social` (messagerie). Upload photos en local sous `./uploads/marketplace/annonces/` servi par actix-files  
**Testing**: Aucun framework configuré (Contrainte constitution), validation manuelle via quickstart + scénarios d'acceptation  
**Target Platform**: Serveur Linux (backend port 8082 en dev) + SSR Nuxt (port 3000)  
**Project Type**: web (monorepo frontend + backend)  
**Performance Goals**: Annonce visible < 5 s après validation (SC-001) ; listing paginé (≤ 50/page, déjà en place)  
**Constraints**: Public = Tailwind v4 pur, **pas de daisyUI** (Principe VI) ; pas de paiement (mise en relation seule) ; photos JPEG/PNG/WebP, 3 Mo max, 5 max/annonce ; publication immédiate (pas de validation préalable) ; mutations auditées (Principe VII)  
**Scale/Scope**: ~10 nouveaux endpoints backend membre, 1 composable étendu, 1 formulaire + 2 pages frontend, 2 ajustements de schéma

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Conformité | Note |
|----------|-----------|------|
| I. Français d'Abord | ✅ | Code/UI/colonnes/commits en français. Termes existants conservés (`annonce`, `cree_par`, `etat`). |
| II. Monorepo Cohérent | ✅ | Changements cross-stack (SQL → backend → frontend) livrés ensemble ; types TS ↔ structs Rust ↔ SQL alignés. |
| III. SQL Source de Vérité | ✅ | On commence par le schéma (ajout valeur enum `conclue` + colonne `social.conversation.annonce_id`), puis propagation. |
| IV. Sécurité par Défaut | ✅ | JWT requis (émis aux comptes `actif` uniquement → FR-007), contrôle propriétaire (`cree_par`), validation entrées, upload via `image_validation` + `sanitize-filename`, pas de secret en dur, requêtes sqlx paramétrées. |
| V. Simplicité (YAGNI) | ✅ | Réutilisation des helpers messagerie (`obtenir_ou_creer_conversation`), du pattern auth inline (`utilisateur_courant`), des structs admin existantes ; aucun nouveau pattern. Pas de champ de modération superflu (suspendue/supprimee suffisent). |
| VI. Tailwind v4 (daisyUI back-office) | ✅ | Le marché est **public** → formulaire et composants en **Tailwind v4 pur**, aucune classe daisyUI. |
| VII. Audit & Traçabilité | ✅ | Chaque mutation membre (créer/modifier/supprimer/conclure/favori/contact) appelle `audit::log_action`. |

**Verdict** : PASS. Aucune violation. Section Complexity Tracking laissée vide.

Une décision de conception (assouplir le verrou « amitié » de la messagerie pour le contact via annonce) est documentée en Phase 0 ; elle reste minimale et bornée, conforme au Principe V.

## Project Structure

### Documentation (this feature)

```text
specs/001-marche-achat-vente-troc-don/
├── plan.md              # Ce fichier (/speckit.plan)
├── spec.md              # Spécification (déjà créée)
├── research.md          # Phase 0 (ce run)
├── data-model.md        # Phase 1 (ce run)
├── quickstart.md        # Phase 1 (ce run)
├── contracts/           # Phase 1 (ce run)
│   ├── annonces-membre.md
│   ├── favoris.md
│   └── contact-messagerie.md
├── checklists/
│   └── requirements.md  # déjà créé (/speckit.specify)
└── tasks.md             # Phase 2 (/speckit.tasks, non créé ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   ├── 05_marketplace.sql            # MODIF : ajout valeur enum 'conclue'
│   └── 30_social_conversation_annonce.sql  # NOUVEAU : colonne annonce_id nullable sur social.conversation
├── src/
│   ├── handlers/
│   │   ├── annonces.rs               # MODIF : + endpoints membre (créer/modifier/supprimer/conclure/mes-annonces/favoris/contacter)
│   │   └── messagerie.rs             # MODIF : assouplir l'envoi (amitié OU conversation existante)
│   ├── models/
│   │   └── annonce.rs                # MODIF : DTOs membre (CreerAnnonceMembreRequest, MesAnnoncesResponse, FavoriResponse, ContacterRequest)
│   └── routes.rs                     # MODIF : routes membre sous /api/annonces (auth)
└── uploads/marketplace/annonces/     # NOUVEAU dossier d'upload (créé au runtime)

uafricas_frontend/
├── app/
│   ├── composables/
│   │   └── useMarcheAfricain.ts      # MODIF : + fonctions d'écriture (creer/modifier/supprimer/conclure/mesAnnonces/favoris/contacter)
│   ├── components/marche/
│   │   ├── MarcheAnnonceForm.vue     # NOUVEAU : formulaire publication/édition (Tailwind pur, upload photos)
│   │   └── MarcheFavoriBouton.vue    # NOUVEAU : bouton favori réutilisable
│   └── pages/marche-africain/
│       ├── index.vue                 # MODIF : « Publier » → vrai formulaire (garde auth)
│       ├── [id].vue                  # MODIF : « Contacter » → messagerie, favori, libellé « Annonceur »
│       ├── mes-annonces.vue          # NOUVEAU : gestion des annonces du membre
│       └── favoris.vue               # NOUVEAU : liste des favoris
```

**Structure Decision** : monorepo web existant (Principe II). On étend les fichiers de domaine `annonce` déjà présents (un handler/model par domaine) plutôt que d'introduire de nouveaux modules, conformément au Principe V. Le contact réutilise le domaine `messagerie`/`social` existant.

## Complexity Tracking

> Aucune violation de la Constitution, section vide.

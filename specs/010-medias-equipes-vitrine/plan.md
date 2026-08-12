# Implementation Plan: Médias — équipes éditoriales et recentrage des vitrines Télé & Radio

**Branch**: `010-medias-equipes-vitrine` | **Date**: 2026-08-10 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/010-medias-equipes-vitrine/spec.md`

## Summary

Trois chantiers, une seule livraison cross-stack.

1. **Une entité neuve : le membre d'équipe.** Une table polymorphe unique `media_content.membre_equipe`, portée par quatre discriminants (`chaine_tv`, `station_radio`, `emission_tele`, `emission_radio`), décrit une personne par nom, prénom, fonction, territoire, contact, rang, et un lien **facultatif** vers `iam.utilisateur`. Elle est lue en même temps que le support ou le programme (greffée dans les DTO existants, aucune route publique neuve) et écrite par un `PUT` de remplacement intégral, calqué sur `media_support::appliquer_thematiques`.

2. **La vitrine perd ses vidéos.** `GET /television/sections` et `GET /stations-radio/sections` cessent de greffer les aperçus d'épisodes, cessent d'exiger qu'une chaîne ait un épisode publié pour exister, et gagnent l'équipe du support. Les deux composants de section rendent désormais : identité → équipe → bandeau de programmation (conservé, Q3=A) → cartes de programme. Le lecteur inline de `SectionChaine.vue`, les rangées d'épisodes et les barres de réaction sur épisode disparaissent des sections.
   Corollaire découvert à l'analyse : la liste de programmes est aujourd'hui plafonnée à **12 par défaut, 30 au maximum** (`contenus_par_section.unwrap_or(12).clamp(1,30)`), et aucune page ne transmet ce paramètre. Le plafond était sans conséquence tant que la section montrait des épisodes ; il devient une **troncature silencieuse du contenu principal**. Il passe à 30/60, et la section annonce le total dès qu'elle n'affiche pas tout (FR-008) — `total_emissions` est déjà servi, aucun champ neuf n'est requis.

3. **La périodicité passe de 3 à 4 valeurs.** Les clés stockées ne bougent pas (`ponctuelle`, `quotidienne`, `hebdomadaire`) ; seul `mensuelle` s'ajoute, et les **libellés** changent partout pour ceux de la demande. Corollaire non facultatif : `mes_alertes_cadence` calcule aujourd'hui `periode_heures = 24 si quotidienne, sinon 24×7` — un programme mensuel y déclencherait une alerte chaque semaine. La période et l'anticipation deviennent une fonction de la cadence.

Les pages de détail (chaîne, station, programme télé, programme radio) sont retravaillées pour le pliage de la description, le pliage de l'équipe, et l'affichage de la périodicité sans la masquer quand elle vaut « non périodique ». Les deux pages de programme perdent au passage leur ligne héritée « Animation : … · Production : … » (FR-034) : conservée à côté du nouveau bloc d'équipe, elle donnerait deux sources concurrentes pour la même information.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) · TypeScript 5 / Vue 3 SSR / Nuxt 4 (frontend)

**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL, requêtes **runtime** `query_as`), uuid, chrono, serde · Pinia, Tailwind CSS v4, FontAwesome. **Aucune dépendance nouvelle, ni côté Cargo ni côté pnpm.**

**Storage**: PostgreSQL 16, schéma `media_content`. Une migration : `uafricas_backend/doc/bd/schemas/09t_media_content_equipes_periodicite.sql` (la dernière en place est `09s`).

**Testing**: aucun harnais de test n'est configuré sur le projet (contrainte constitutionnelle assumée). La validation est manuelle et scénarisée — voir [quickstart.md](./quickstart.md).

**Target Platform**: application web SSR (Nuxt 4) servie par un backend Actix-Web ; PostgreSQL 16 en Docker.

**Project Type**: monorepo web — `uafricas_frontend/` + `uafricas_backend/`.

**Performance Goals**:
- L'équipe est lue **sans N+1** : une requête par discriminant sur `porteur_id = ANY($1)`, exactement le patron de `media_support::thematiques_par_supports`.
- Le payload de `/sections` **diminue** : jusqu'à 12 épisodes par programme × jusqu'à 12 programmes × 6 chaînes cessent d'être sérialisés. C'est le levier de SC-008.
- Aucune tâche de fond, aucun cache : tout se résout à la lecture, comme la rotation d'épisodes de 09q.

**Constraints**:
- **Ordre de déclaration des routes actix** — contrainte dure de ce projet, déjà cause de deux 404 à la recette de 009. Tout segment littéral neuf doit précéder les motifs `"/{type_support}/{support_id}/…"` (`routes.rs:1150+`) et, côté admin, `/medias/{id}` (`routes.rs:147`) et `/medias/{type_media}/{id}/etat` (`routes.rs:141`).
- **sqlx est vérifié au runtime** : une colonne oubliée ne casse pas la compilation. Toute requête touchée doit être exécutée au moins une fois au recettage.
- **Principe VI** : pages publiques en Tailwind v4 pur. Les composants membres existants (`GestionEpisodes.vue`, `MesSupports.vue`, `GestionCoDetenteurs.vue`) sont **déjà** sans daisyUI — vérifié : 0 occurrence. Le nouveau composant de gestion, monté côté membre **et** côté admin, suit la même règle.
- **Principe VII** : la mutation d'équipe doit passer par `audit::log_action`.

**Scale/Scope**: 1 migration SQL · 3 modules Rust neufs ou étendus + 6 handlers touchés · **5 routes neuves** (3 membre, 2 admin) · **5 composants Vue neufs** · 8 pages/composants publics remaniés · 2 surfaces d'édition (membre `/mon-compte/mes-supports`, admin `/admin/television/[id]`, `/admin/radio/[id]`, `/admin/medias/emissions/[id]`).

## Constitution Check

*GATE : évalué avant Phase 0, réévalué après Phase 1.*

| Principe | Verdict | Justification |
|---|---|---|
| **I. Français d'abord** | ✅ | Table `membre_equipe` ; colonnes `type_porteur, porteur_id, nom, prenom, fonction, territoire, contact, utilisateur_id, ordre` ; composants `EquipeMedia`, `GestionEquipe`, `TexteRepliable`, `ChampCombo` ; messages d'erreur en français. |
| **II. Monorepo cohérent** | ✅ | SQL → Rust → TypeScript livrés ensemble. `MembreEquipeResponse` (Rust) ↔ `MembreEquipeAPI` (TS) ↔ colonnes SQL, un pour un. |
| **III. SQL source de vérité** | ✅ | La migration `09t` précède tout code. UUID v4 PK, `deleted_at`, TIMESTAMPTZ, snake_case français, CHECK plutôt qu'enum (justifié en [research.md](./research.md) D1). |
| **IV. Sécurité par défaut** | ✅ | Écriture gardée par `garde_detenteur(≥ co_detenteur)` côté membre et `verifier_permission!(admin, "media", "modifier")` côté admin ; binds paramétrés partout ; **le contact affiché est saisi à la main, jamais repris de `iam.utilisateur`** — aucun e-mail de compte n'est exposé par le rattachement. |
| **V. Simplicité (YAGNI)** | ⚠️ justifié | Une table pour quatre porteurs plutôt que deux tables ; un `PUT` de remplacement plutôt qu'un CRUD par membre ; aucune page neuve. Deux composants génériques créés — voir Complexity Tracking. |
| **VI. Tailwind v4 (daisyUI back-office seul)** | ✅ | Les 4 composants neufs sont en Tailwind pur. daisyUI n'apparaît que dans les gabarits `/admin/**` qui l'emploient déjà. |
| **VII. Audit & traçabilité** | ✅ | `PUT …/equipe` (membre et admin) journalise via `audit::log_action` sur `media_content.membre_equipe`, avec instantané avant/après en JSONB. |

**Verdict** : aucune violation bloquante. Une déviation mineure au Principe V est documentée et justifiée par le nombre de points d'appel.

### Réévaluation après conception (Phase 1)

| Principe | Verdict | Ce que la conception a confirmé ou déplacé |
|---|---|---|
| III. SQL source de vérité | ✅ | Le modèle a été écrit **en SQL d'abord** ([data-model.md](./data-model.md)), y compris les règles qui auraient pu rester en Rust : `nom` et `fonction` obligatoires sont deux CHECK `btrim(…) <> ''`, un `NOT NULL` laissant passer une chaîne d'espaces. |
| IV. Sécurité | ✅ | Deux points fermés en conception : `contact` n'est jamais dérivé de `iam.utilisateur.email` (D2), et `utilisateur_id` n'est sérialisé que si le compte existe et n'est pas supprimé — le frontend n'a donc jamais à valider un lien. |
| V. Simplicité | ⚠️ inchangé | La conception a **réduit** la surface prévue : aucune route publique neuve (D7), aucune page neuve, un `PUT` de remplacement au lieu de quatre verbes (D6), les clés de périodicité conservées au lieu d'être renommées (D4). Les deux composants génériques restent la seule déviation. |
| VII. Audit | ✅ | Une action `equipe_modifiee` par `PUT`, avec instantané avant/après — un remplacement intégral rend la diff plus lisible qu'un flux d'événements unitaires. |

**Point de conception à ne pas perdre de vue en Phase 2** : `porteur_id` n'a pas de clé étrangère (prix du polymorphisme). Le nettoyage à la suppression du porteur est **explicite dans les handlers**, en quatre endroits ([api-admin.md §4](./contracts/api-admin.md)). L'oublier ne casserait rien de visible — les équipes orphelines resteraient simplement dans le référentiel de suggestions de fonctions. Le quickstart en fait un critère de sortie chiffré.

## Project Structure

### Documentation (this feature)

```text
specs/010-medias-equipes-vitrine/
├── plan.md              # Ce fichier
├── research.md          # Phase 0 — 8 décisions de conception
├── data-model.md        # Phase 1 — modèle de données et contrats de types
├── quickstart.md        # Phase 1 — scénarios de validation manuelle
├── contracts/
│   ├── api-public.md    # Lecture : payloads enrichis, aucune route neuve
│   ├── api-membre.md    # Écriture détenteur : 3 routes
│   └── api-admin.md     # Écriture admin : 2 routes + suggestions
├── checklists/
│   └── requirements.md  # Écrit par /speckit-specify
├── spec.md
└── tasks.md             # Phase 2 — écrit par /speckit-tasks, PAS par /speckit-plan
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 09t_media_content_equipes_periodicite.sql     # NEUF — table membre_equipe + CHECK cadence
├── src/
│   ├── models/
│   │   ├── media_equipe.rs                           # NEUF — MembreEquipeRow/Response/Request, aiguillage
│   │   ├── media_emission.rs                         # MODIFIÉ — CADENCES_AUTORISEES +'mensuelle',
│   │   │                                             #   heures_anticipation_alerte, periode_heures_cadence,
│   │   │                                             #   EmissionResponse.equipe
│   │   ├── television.rs                             # MODIFIÉ — ChaineTvResponse.equipe
│   │   └── station_radio.rs                          # MODIFIÉ — StationRadioResponse.equipe
│   ├── handlers/
│   │   ├── media_equipe.rs                           # NEUF — lecture groupée, PUT membre/admin, suggestions
│   │   ├── media_emission.rs                         # MODIFIÉ — LATERAL ON TRUE, 404 sans épisode levé
│   │   ├── media_programmation.rs                    # MODIFIÉ — période d'alerte fonction de la cadence
│   │   ├── television.rs                             # MODIFIÉ — sections sans aperçus, EXISTS levé, équipe
│   │   ├── stations_radio.rs                         # MODIFIÉ — idem côté radio
│   │   └── admin/radio_tele.rs                       # MODIFIÉ — cadence 'mensuelle' acceptée
│   └── routes.rs                                     # MODIFIÉ — 6 routes, ordre de déclaration critique

uafricas_frontend/app/
├── components/
│   ├── common/
│   │   ├── TexteRepliable.vue                        # NEUF — description tronquée + voir plus/moins
│   │   └── ChampCombo.vue                            # NEUF — saisie libre + suggestions (fonction)
│   ├── media/
│   │   ├── EquipeMedia.vue                           # NEUF — rendu public d'une équipe (+ repli)
│   │   ├── GestionEquipe.vue                         # NEUF — édition, montée membre ET admin
│   │   ├── CarteProgramme.vue                        # NEUF — carte de programme pour la vitrine
│   │   ├── SectionChaine.vue                         # RÉÉCRIT — plus de lecteur ni de vignette
│   │   ├── SectionStation.vue                        # RÉÉCRIT — idem
│   │   └── MesSupports.vue                           # MODIFIÉ — section « Équipe éditoriale »
│   └── (CarteEmission.vue — code mort, remplacé par CarteProgramme)
├── composables/
│   ├── useMediaEquipe.ts                             # NEUF — lecture/écriture équipe + fonctions
│   ├── useMediaEmissions.ts                          # MODIFIÉ — LIBELLES_CADENCE (4 clés, libellés neufs)
│   ├── useTelevision.ts                              # MODIFIÉ — TvChannel.equipe, TvEmission.equipe
│   ├── useStationsRadio.ts                           # MODIFIÉ — idem
│   └── useAdminMediaEmissions.ts                     # MODIFIÉ — CADENCES aligné sur LIBELLES_CADENCE
└── pages/
    ├── medias/chaines/[slug].vue                     # MODIFIÉ — description repliable, équipe, programmes
    ├── medias/stations/[slug].vue                    # MODIFIÉ — idem
    ├── medias/emissions-tele/[slug].vue              # MODIFIÉ — périodicité, équipe, fil d'Ariane réparé
    ├── medias/emissions-radio/[slug].vue             # MODIFIÉ — idem
    └── admin/{television,radio}/[id].vue             # MODIFIÉ — section équipe
        admin/medias/emissions/[id].vue               # MODIFIÉ — section équipe + cadence à 4 valeurs
```

**Structure Decision** : monorepo web existant, aucune arborescence neuve. Le domaine médias occupe déjà `models/media_*.rs` + `handlers/media_*.rs` ; `media_equipe.rs` s'y insère par symétrie stricte avec `media_support.rs`, qui résout le même problème (liaison polymorphe, lecture groupée, écriture par remplacement, doublon membre/admin des handlers). L'admin des programmes n'a pas de module dédié — il vit dans `handlers/admin/radio_tele.rs` ; les handlers admin d'équipe suivent `media_support.rs` et restent dans `media_equipe.rs`, comme `admin_definir_thematiques` reste dans `media_support.rs`.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Deux composants génériques neufs (`TexteRepliable.vue`, `ChampCombo.vue`) alors que le Principe V proscrit l'abstraction prématurée | `TexteRepliable` a **4 points d'appel** — les quatre pages de détail (chaîne, station, programme télé, programme radio). Le repli de la liste d'équipe **n'en fait pas partie** : il relève d'`EquipeMedia`, qui plie des fiches et non du texte. Les sections de vitrine non plus : FR-003 y demande une ellipse, pas une commande de dépliage. `ChampCombo` a **4** points d'appel (champ « fonction » sur les surfaces membre et admin, pour le support et pour le programme). Le seuil des « 3 lignes dupliquées » est franchi dans les deux cas. | Réimplémenter le pliage dans chaque page reproduirait exactement ce que le projet a déjà subi sur `line-clamp`, dupliqué à l'identique dans 5 `<style scoped>` (`CarteContenu`, `SectionChaine`, `SectionStation`, `VedettePleinEcran`, `ListeEpisodes`). Le combo, réimplémenté 4 fois, divergerait sur la normalisation de casse — donc sur FR-015. |
| Une table polymorphe à 4 discriminants plutôt que 2 tables (`support_equipe`, `emission_equipe`) | Les colonnes et toutes les règles sont identiques aux deux niveaux ; le projet applique déjà ce patron trois fois (`support_thematique`, `support_territoire`, les 4 tables d'interactions par `(type_media, media_id)`). Deux tables imposeraient de dupliquer requêtes, DTO, handlers et validations. | Deux tables auraient offert de vraies FK. Le coût — quatre chemins de code au lieu d'un, sur une entité sans référence entrante — dépasse le gain. L'intégrité est portée par le CHECK du discriminant et par un nettoyage explicite à la suppression du porteur (FR-019). |

**Non retenu au titre de cette feature** (dette constatée, hors périmètre, à ne pas traiter en passant) : le N+1 de `diffusion_pour_support` (2 requêtes par chaîne dans `/sections`), le drapeau obsolète `PAGES_DETAIL_DISPONIBLES = false` de `VedettePleinEcran.vue:42`, et `app/pages/medias/radios.vue` encore alimenté par `~/mocks/radios`.

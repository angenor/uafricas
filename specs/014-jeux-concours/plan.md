# Implementation Plan : Jeux variés et concours communautaires

**Branch** : `014-jeux-concours` | **Date** : 2026-10-04 | **Spec** : [spec.md](./spec.md)

**Input** : Feature specification from `/specs/014-jeux-concours/spec.md`

## Summary

La 013 ne sait faire qu'un type de jeu : la question à choix multiple. La 014 ajoute **deux familles**.

**Famille A — quatre façons de répondre**, sur le moteur existant : carte, ordre, paires, et photo ou son en énoncé. Le principe de la **clé opaque** de la 013 s'étend tel quel. Le joueur ne manipule que des rangs dans des tableaux stockés dans un ordre aléatoire, donc la solution ne quitte jamais le serveur avant la réponse. Une fonction `evaluer` unique corrige dans tous les cadres : partie, défi, duel différé, duel direct. Le tirage varié, le score, les défis, les duels et le Championship ne changent pas.

**Famille B — un moteur de concours**, dont la bataille de photos est le premier format. Le cycle est appel → modération → vote → résultats, avec des phases calculées à la lecture et deux transitions écrites sous verrou (annulation et résultats). Le vote se fait à l'aveugle par confrontations de paires : on présente d'abord la participation la moins vue, jamais la sienne, jamais deux fois la même paire, et la confrontation ouverte survit au rechargement. Le classement est la proportion de victoires, avec un seuil de présentations. Les voix suspectes sont enregistrées mais écartées sans rien dire au votant. Les récompenses passent par `crediter`, avec une nouvelle origine `concours`, plus des règles de réputation à 0 point.

Une migration additive (`38_jeu_concours.sql`), aucune dépendance nouvelle.

## Technical Context

**Language/Version** : Rust Edition 2024 (backend) · TypeScript 5 / Vue 3 SSR / Nuxt 4 (frontend)

**Primary Dependencies** : Actix-Web 4, actix-multipart, sqlx (requêtes **runtime**), uuid, chrono, serde, rand, `image` (déjà présent, via `services/image_validation.rs`) · Pinia, Tailwind CSS v4, daisyUI v5 (back-office), FontAwesome. **Aucune dépendance nouvelle** : la durée des extraits sonores est contrôlée par le navigateur de l'administrateur, et le serveur borne le poids (research D4).

**Storage** : PostgreSQL 16, schéma `jeu` de la 013. Migration `uafricas_backend/doc/bd/schemas/38_jeu_concours.sql`, incluse dans `schema.sql` après `37_jeu.sql`. Fichiers sous `uploads/jeu/{images,audios}/` et `uploads/jeu/concours/{id}/`, servis par le `/uploads/` existant.

**Testing** : aucun harnais (contrainte constitutionnelle assumée). Recette scénarisée : [quickstart.md](./quickstart.md), huit scénarios et des critères de sortie.

**Target Platform** : application web SSR servie par un backend Actix-Web **mono-instance**, PostgreSQL 16 en Docker.

**Project Type** : monorepo web, `uafricas_frontend/` + `uafricas_backend/`.

**Performance Goals** :
- Tirage d'une confrontation en deux requêtes indexées, sans pré-calcul (research D7).
- Résultats d'un concours de 200 participations et 50 000 voix établis en moins de 3 s à la première lecture (SC-011) : une agrégation `GROUP BY` sur `jeu.confrontation`, une seule fois.
- Enchaîner 20 votes en moins de 2 minutes (SC-007) : le vote renvoie directement la confrontation suivante.

**Constraints** :
- **La solution ne sort jamais avant la réponse** (FR-008). `EpreuveServie` et `Correction` restent deux types distincts. Les éléments d'un « ordre » et la colonne de droite d'une « paires » sont stockés **au hasard** : leur rang n'apprend rien.
- **Ordre de déclaration des routes actix** : `/concours/mes-participations` avant `/concours/{id}`, et côté admin `/concours/participations…` avant `/concours/{id}…`.
- **sqlx vérifié à l'exécution** : chaque requête neuve passe au moins une fois en recette. Les nouvelles colonnes de `EpreuveRow` sont lues par **toutes** les requêtes qui la remplissent (la constante de colonnes de la 013).
- **`web::Query` refuse une clé répétée** : pas de liste en paramètre de requête.
- **Aucune tâche de fond** : phases et résultats résolus à la lecture (D6). `resoudre_concours` doit être appelée par **toute** route qui lit un concours.
- **Principe VI** : `/activites/concours/**` en Tailwind pur sur le gabarit `africans` ; daisyUI dans `/admin/activites/**` seulement.
- **Principe VII** : toutes les mutations d'administration journalisées ([api-admin.md](./contracts/api-admin.md)).
- **Aucun point d'engagement** n'est crédité par le jeu (SC-013) : pas de J'aime sur les participations (D11), réputation par règles à 0 point (D10).

**Scale/Scope** : 1 migration SQL · 1 service Rust neuf (`jeu_concours`), 2 handlers neufs (`jeu_concours`, `admin/jeu_concours`), extensions de `services/jeu.rs`, `services/jeu_derivation.rs`, `handlers/admin/jeu.rs`, `handlers/jeu_duel.rs` · **~27 routes neuves** (3 publiques, 7 membre, 17 admin) et 5 routes de la 013 dont les corps s'étendent · 1 composable public + 1 admin neufs, 2 étendus · ~12 composants Vue neufs · 3 pages publiques + 3 pages d'administration.

## Constitution Check

*GATE : évalué avant Phase 0, réévalué après Phase 1.*

| Principe | Verdict | Justification |
|---|---|---|
| **I. Français d'abord** | ✅ | Tables `concours`, `participation`, `confrontation`, `resultat_concours`, `signalement_participation` ; colonnes `type_reponse`, `solution`, `appariements`, `valeurs`, `reponse_pays_id`, `motif_ecart` ; service `jeu_concours` ; composants `Jeu…`. |
| **II. Monorepo cohérent** | ✅ | SQL → Rust → TypeScript livrés ensemble ; les formes d'API sont fixées dans [contracts/](./contracts/). |
| **III. SQL source de vérité** | ✅ | `38` précède tout code. Une épreuve incohérente est **impossible en SQL** : solution qui n'est pas une permutation, carte sans pays, choix sans bonne réponse. Une paire n'est jamais représentée, et il y a une seule confrontation ouverte par votant (index uniques). Écarts déjà justifiés par la 013 : CHECK plutôt qu'enums, pas de `deleted_at` sur des journaux. |
| **IV. Sécurité par défaut** | ✅ | Solution jamais sérialisée avant la réponse ; auteur et décomptes jamais servis pendant le vote ; temps de vote mesuré par le serveur ; EXIF supprimées des photos ; type des fichiers reconnu par signature binaire, pas par l'extension ; état du compte vérifié en base (`garde_joueur`). |
| **V. Simplicité (YAGNI)** | ⚠️ justifié | Le moteur de concours est générique (format en CHECK, création réduite à média + légende) alors qu'un seul format est livré : voir Complexity Tracking. Évités : signalement générique, table par type de réponse, temps réel, file de tâches, dépendance audio. |
| **VI. Tailwind v4 / daisyUI back-office** | ✅ | Composants `jeu/` en Tailwind pur avec les jetons `af-*`. |
| **VII. Audit & traçabilité** | ✅ | Création, modification, annulation et suppression de concours ; modération, rétablissement ; voix écartées ou rétablies ; délibération ; dépôt de média ; règles. Les votes des membres ne sont pas audités : `jeu.confrontation` **est** leur journal. |

**Verdict** : aucune violation bloquante.

### Réévaluation après conception (Phase 1)

| Principe | Verdict | Ce que la conception a confirmé ou déplacé |
|---|---|---|
| III | ✅ | Les règles de vote clés sont en SQL (paire unique, une seule ouverte, choix ∈ paire). L'auto-vote est exclu au tirage et revérifié au vote. |
| IV | ✅ | Une porte fermée en conception : la confrontation ouverte est rendue telle quelle au rechargement, donc on ne peut pas « passer » une paire qui déplaît (D7). Le votant n'apprend jamais qu'une voix a été écartée (D9). Galerie pendant le vote en ordre aléatoire, sans auteur. |
| V | ⚠️ inchangé | `evaluer` remplace deux comparaisons dupliquées par une seule fonction : la conception **réduit** la duplication de la 013. |

**Points de conception à ne pas perdre de vue en Phase 2**

1. **`resoudre_concours` partout où un concours est lu** : liste publique, détail, galerie, confrontations, vote, mes participations, liste et suivi admin. L'oublier ne casse rien de visible : le concours resterait en vote après sa date, et les récompenses ne partiraient jamais.
2. **`EpreuveRow` gagne des colonnes** : toutes les requêtes qui la remplissent (constante de colonnes) doivent les lire, sinon `query_as` échoue à l'exécution sur le chemin oublié (tirage, défi, duel, admin).
3. **`servir_stable` (duel direct) doit mélanger aussi `appariements`**, avec une graine dérivée, sans quoi la colonne de droite apparaîtrait dans l'ordre stocké.
4. **Le rang stocké d'un « ordre » doit être tiré au hasard à l'insertion**, à la saisie **comme** à la dérivation. Stocker les éléments dans l'ordre attendu ferait des clés servies la solution en clair. Le quickstart (scénario 1, point 6) le vérifie.
5. **Les primes de participation d'un concours annulé** partent avec l'annulation, sous la même clé que celles d'un concours mené à terme : un concours ne peut pas les verser deux fois.

## Project Structure

### Documentation (this feature)

```text
specs/014-jeux-concours/
├── spec.md
├── plan.md              # ce fichier
├── research.md          # D1–D14
├── data-model.md        # migration 38, entités, transitions
├── quickstart.md        # 8 scénarios + critères de sortie
├── contracts/
│   ├── api-membre.md
│   └── api-admin.md
└── checklists/requirements.md
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/
│   ├── schema.sql                               # + \ir schemas/38_jeu_concours.sql
│   └── schemas/38_jeu_concours.sql              # NEUF
└── src/
    ├── models/jeu.rs                            # ÉTENDU : EpreuveRow, EpreuveServie, Correction, ReponseJoueur
    ├── models/jeu_concours.rs                   # NEUF : Concours, Participation, Confrontation, Phase, DTO
    ├── models/admin/jeu.rs                      # ÉTENDU : saisie par type, règles
    ├── services/jeu.rs                          # ÉTENDU : evaluer, servir (ordre/paires), delai_ms (majoration)
    ├── services/jeu_derivation.rs               # ÉTENDU : type_reponse par forme, 9 formes, copie d'image
    ├── services/jeu_concours.rs                 # NEUF : resoudre_concours, tirer_confrontation, voter, etablir_resultats
    ├── handlers/jeu.rs, jeu_duel.rs             # ÉTENDUS : réponse typée, correction étendue
    ├── handlers/jeu_concours.rs                 # NEUF : routes publiques et membre
    ├── handlers/admin/jeu.rs                    # ÉTENDU : épreuves typées, POST /medias, règles
    ├── handlers/admin/jeu_concours.rs           # NEUF : concours, modération, suivi, jury
    ├── handlers/admin/engagement.rs             # ÉTENDU : ACTIONS_INSTRUMENTEES (+3)
    └── routes.rs                                # + scopes concours (littéraux d'abord)

uafricas_frontend/app/
├── composables/
│   ├── useJeu.ts                                # ÉTENDU : types de réponse et de correction
│   ├── useAdminJeu.ts                           # ÉTENDU : saisie par type, dépôt de média
│   ├── useConcours.ts                           # NEUF
│   └── useAdminConcours.ts                      # NEUF
├── components/jeu/
│   ├── CarteEpreuve.vue                         # ÉTENDU : aiguillage par type_reponse
│   ├── Correction.vue                           # ÉTENDU : solution par type
│   ├── ReponseCarte.vue                         # NEUF : CommonCarteAfriqueValeurs + liste accessible
│   ├── ReponseOrdre.vue                         # NEUF : glisser-déposer + boutons monter/descendre
│   ├── ReponsePaires.vue                        # NEUF : toucher gauche puis droite, clavier
│   ├── DuelDirectSalle.vue                      # ÉTENDU : réponse typée
│   ├── CarteConcours.vue                        # NEUF
│   ├── DeposerParticipation.vue                 # NEUF
│   ├── Confrontation.vue                        # NEUF : deux photos, choix, enchaînement
│   ├── GalerieConcours.vue                      # NEUF : aléatoire et anonyme pendant le vote, classée après
│   ├── PodiumConcours.vue                       # NEUF
│   └── SignalerParticipation.vue                # NEUF
├── components/admin/jeu/
│   ├── EpreuveFormulaire.vue                    # ÉTENDU : champs par type, dépôt de média
│   ├── ChampMediaEpreuve.vue                    # NEUF : dépôt + mesure de durée audio
│   └── ConcoursFormulaire.vue                   # NEUF
├── pages/activites/concours/index.vue           # NEUF
├── pages/activites/concours/[id].vue            # NEUF : vue par phase
├── pages/activites/index.vue                    # ÉTENDU : bloc « Concours en cours »
├── pages/mon-compte/activites.vue               # ÉTENDU : mes participations
├── pages/admin/activites/concours/index.vue     # NEUF : liste + création
├── pages/admin/activites/concours/[id].vue      # NEUF : modération, suivi, jury
├── pages/admin/activites/participations.vue     # NEUF : file de modération transversale
├── pages/admin/activites/regles.vue             # ÉTENDU : 3 champs
├── components/admin/AdminSidebar.vue            # ÉTENDU : Concours, Participations
├── components/jeu/PanneauActivites.vue          # ÉTENDU : concours rattaché au module
└── mocks/notifications.ts, pages/notifications.vue  # ÉTENDUS : 6 types jeu.*
```

**Structure Decision** : prolongement direct de la 013, dans le même schéma et les mêmes dossiers. Le concours obtient son propre service et ses propres handlers parce que son cycle ne partage rien avec la partie (pas de série, pas de bonne réponse). Il réutilise le journal des gains, les règles du jeu, les notifications et l'écran des règles.

### Ordre de livraison

| Palier | Contenu | Histoires | Livrable seul ? |
|---|---|---|---|
| 1 | Migration `38` (partie épreuves), `evaluer`, carte/ordre/paires bout à bout en partie libre | US1 | oui : les nouveaux types sont jouables |
| 2 | Duel direct et défi sur les nouveaux types, temps majoré | US1 | oui |
| 3 | Dépôt de médias, formulaire par type, nouvelles formes de dérivation | US2, US3 | oui : le vivier grossit |
| 4 | Migration `38` (partie concours), programmation, dépôt, modération | US4, US6 | non : sans vote, le concours s'arrête à l'appel |
| 5 | Vote à l'aveugle, résultats, récompenses, galerie | US5, US7 | oui : premier concours complet |
| 6 | Jury, suivi anti-fraude, signalement | US8, US9 | oui |

La migration `38` est écrite en entier dès le palier 1 (une seule migration), mais les paliers 4 et suivants n'en utilisent la partie concours qu'à leur tour.

## Complexity Tracking

| Écart | Pourquoi c'est nécessaire | Alternative plus simple écartée parce que… |
|---|---|---|
| Moteur de concours générique (format en CHECK, création réduite à média + légende) pour un seul format livré | FR-025 et SC exigent que les vagues suivantes (vidéo, traduction, texte, idées, votes citoyens) se branchent **sans refaire le cycle** ; c'est une demande explicite du client | Un moteur « bataille de photos » spécialisé serait à réécrire au premier défi vidéo. La généricité coûte une colonne `format` et l'absence de toute lecture de la création dans le vote et le dépouillement, pas une abstraction. |
| Trois colonnes propres à des types de réponse sur `jeu.epreuve` | Garder les règles de cohérence en SQL (principe III) | Un JSONB unique serait plus court mais incontrôlable par CHECK (research D1). |

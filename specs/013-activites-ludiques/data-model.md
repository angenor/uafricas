# Data Model : Activités interactives, ludiques et participatives

**Feature** : `013-activites-ludiques` · **Migration** : `uafricas_backend/doc/bd/schemas/37_jeu.sql` · **Schéma** : `jeu`

Le SQL est la source de vérité (principe III). Ce document fixe les tables, leurs contraintes et leurs transitions ; les décisions qui les motivent sont dans [research.md](./research.md) (D2, D4, D5, D9, D12).

Conventions : UUID v4 en clé primaire, `TIMESTAMPTZ`, snake_case français, `VARCHAR` + `CHECK` pour les états, migration idempotente (`IF NOT EXISTS`, `DROP CONSTRAINT IF EXISTS` puis `ADD`).

## Vue d'ensemble

```text
jeu.module ──< jeu.epreuve >── (type_source, source_id) ··· jeu.v_source (vue)
                   │
jeu.partie ──< jeu.reponse >──┘
   │  └── defi_id ──> jeu.defi
   │  └── duel_id ──> jeu.duel
   └──────────────< jeu.gain >── saison_id ──> jeu.saison
                        │
                        ├── agrège vers jeu.score_saison (saison × membre × pays)
                        └── agrège vers jeu.joueur       (membre)

jeu.signalement_epreuve ──> jeu.epreuve
jeu.regles (singleton)
```

Dix tables et une vue. Les classements ne sont pas stockés : ils se lisent dans `score_saison`.

---

## 1. `jeu.module`

Un module de la plateforme ouvert au jeu.

| Colonne | Type | Règle |
|---|---|---|
| `code` | `VARCHAR(30)` PK | `[a-z0-9_]`, immuable |
| `libelle` | `VARCHAR(80)` NOT NULL | |
| `route` | `VARCHAR(200)` NOT NULL | page du module, pour le retour |
| `icone` | `VARCHAR(40)` | nom FontAwesome déjà enregistré |
| `ouvert` | `BOOLEAN` NOT NULL DEFAULT TRUE | FR-081 |
| `ordre` | `SMALLINT` NOT NULL DEFAULT 0 | |
| `created_at`, `updated_at` | `TIMESTAMPTZ` | |

Semé : `afrolang`, `codimoi`, `afripulse`, `factcheck`.

Un module est **disponible** s'il est `ouvert` et compte au moins `regles.taille_partie` épreuves servables (FR-005). La disponibilité se calcule, elle ne se stocke pas.

---

## 2. `jeu.epreuve`

| Colonne | Type | Règle |
|---|---|---|
| `id` | UUID PK | |
| `module_code` | `VARCHAR(30)` NOT NULL → `jeu.module` | |
| `enonce` | `TEXT` NOT NULL | `btrim(enonce) <> ''` |
| `media_type` | `VARCHAR(10)` | `IN ('image','audio')` ou NULL |
| `media_url` | `VARCHAR(500)` | non nul si et seulement si `media_type` non nul |
| `propositions` | `TEXT[]` NOT NULL | `cardinality BETWEEN 2 AND 6` |
| `bonne_reponse` | `SMALLINT` NOT NULL | `BETWEEN 1 AND cardinality(propositions)` |
| `explication` | `TEXT` | obligatoire dès `jouable` (voir CHECK) |
| `difficulte` | `SMALLINT` NOT NULL DEFAULT 1 | `BETWEEN 1 AND 3` |
| `theme` | `VARCHAR(80)` | |
| `pays_id` | UUID → `shared.pays` ON DELETE SET NULL | pays dont parle l'épreuve |
| `origine` | `VARCHAR(10)` NOT NULL | `IN ('saisie','derivee')` |
| `type_source` | `VARCHAR(30)` | voir `v_source` |
| `source_id` | UUID | pas de FK (référence polymorphe) |
| `forme` | `VARCHAR(40)` | code de la forme de question |
| `source_empreinte` | `VARCHAR(32)` | empreinte de la source à la dérivation |
| `etat` | `VARCHAR(12)` NOT NULL | `IN ('candidate','jouable','a_revoir','rejetee','retiree')` |
| `motif_rejet` | `TEXT` | |
| `nombre_servie` | `INT` NOT NULL DEFAULT 0 | FR-080 |
| `nombre_bonnes` | `INT` NOT NULL DEFAULT 0 | FR-080 |
| `cree_par` | UUID → `iam.utilisateur` ON DELETE SET NULL | NULL pour une dérivée |
| `valide_par`, `valide_at` | UUID, `TIMESTAMPTZ` | |
| `created_at`, `updated_at` | `TIMESTAMPTZ` | |

**Contraintes**

- `ck_epreuve_source` : `type_source` et `source_id` sont nuls ensemble ou non nuls ensemble.
- `ck_epreuve_derivee` : `origine = 'derivee'` ⇒ `type_source`, `source_id`, `forme` et `source_empreinte` non nuls.
- `ck_epreuve_jouable` : `etat = 'jouable'` ⇒ `btrim(explication) <> ''`. La publication d'une épreuve incomplète est refusée **en SQL** ; l'API nomme le manque avant d'y arriver (FR-074).
- `ck_epreuve_rejet` : `etat = 'rejetee'` ⇒ `btrim(motif_rejet) <> ''`.
- `uq_epreuve_derivee` : index unique `(type_source, source_id, forme) WHERE origine = 'derivee'`. Il couvre les rejetées : une forme rejetée sur un contenu n'est jamais reproposée (FR-075).

**Index** : `(module_code, etat)`, `(pays_id) WHERE etat = 'jouable'`, `(type_source, source_id)`.

**Transitions**

```text
(dérivation) ──> candidate ──accepter──> jouable ──retirer──> retiree
                     │                      │  ▲
                     └──rejeter──> rejetee   │  └──accepter (après revue)
                                            │
(saisie) ──> candidate ──publier──> jouable └──source modifiée──> a_revoir ──rejeter──> rejetee
```

- Une épreuve saisie naît `candidate` (brouillon) ; son auteur administrateur la publie sans revue (FR-009).
- `a_revoir` est écrit paresseusement : la liste de revue du back-office commence par basculer les épreuves `jouable` dont l'empreinte ne correspond plus. Entre la modification de la source et cette bascule, l'épreuve est déjà **non servie**, parce que le tirage passe par `v_source`.
- `retiree` et `rejetee` sont terminaux. Les réponses et les gains sont conservés (FR-013, FR-084).

**Épreuve servable** (la seule définition, reprise par tout tirage) :

```sql
e.etat = 'jouable'
AND m.ouvert
AND (e.type_source IS NULL
     OR (s.visible AND (e.origine = 'saisie' OR s.empreinte = e.source_empreinte)))
-- avec JOIN jeu.module m, LEFT JOIN jeu.v_source s ON (s.type_source, s.source_id) = (e.type_source, e.source_id)
```

---

## 3. `jeu.v_source` (vue)

`CREATE OR REPLACE VIEW`, une branche `UNION ALL` par type de source.

| Colonne | Sens |
|---|---|
| `type_source` | `'fiche_pays'`, `'site_touristique'`, `'recette_culinaire'`, `'personnalite_connue'`, `'codimoi'`, `'factcheck'` |
| `source_id` | identifiant dans la table d'origine |
| `visible` | le contenu est publié et non signalé |
| `empreinte` | `md5` des champs dont on tire des questions |
| `pays_id` | pays du contenu, s'il en a un |

| Branche | `visible` | Champs de l'empreinte |
|---|---|---|
| `fiche_pays` (+ `shared.pays`) | `fp.bloquee = FALSE` | `p.nom`, `p.capitale`, `fp.monnaie`, `fp.image_drapeau_url`, `fp.image_devise_url` |
| `site_touristique` | `deleted_at IS NULL AND suspendu = FALSE` | `nom`, `fiche_pays_id` |
| `recette_culinaire` | idem | `titre`, `fiche_pays_id` |
| `personnalite_connue` | idem | `nom_complet`, `domaine`, `fiche_pays_id` |
| `codimoi` | `etat = 'publie' AND deleted_at IS NULL` | `type`, `contenu`, `pays_id`, `nom_auteur_originel` |
| `factcheck` | `etat = 'publie' AND deleted_at IS NULL` | `prejuge_titre`, `verdict`, `realite_description` |

Les compteurs (`nombre_likes`, `nombre_signalements`) et `updated_at` sont **exclus** de l'empreinte : ils changent sans que le contenu change (research D5). Un contenu supprimé n'a plus de ligne dans la vue ; la jointure externe donne `visible` NULL, donc l'épreuve n'est pas servie.

Le lien vers le contenu (FR-007) n'est pas dans la vue : il est construit côté Rust à partir de `type_source`, à la correction.

---

## 4. `jeu.partie`

| Colonne | Type | Règle |
|---|---|---|
| `id` | UUID PK | |
| `utilisateur_id` | UUID NOT NULL → `iam.utilisateur` ON DELETE CASCADE | |
| `module_code` | `VARCHAR(30)` → `jeu.module` | NULL pour un défi multi-modules |
| `cadre` | `VARCHAR(12)` NOT NULL | `IN ('libre','entrainement','defi','duel')` |
| `defi_id` | UUID → `jeu.defi` | non nul si et seulement si `cadre = 'defi'` |
| `duel_id` | UUID → `jeu.duel` | non nul si et seulement si `cadre = 'duel'` |
| `theme` | `VARCHAR(80)` | restriction demandée |
| `pays_id` | UUID → `shared.pays` | restriction demandée (FR-066) |
| `epreuve_ids` | `UUID[]` NOT NULL | série figée, dans l'ordre |
| `rang_courant` | `SMALLINT` NOT NULL DEFAULT 0 | nombre d'épreuves déjà présentées |
| `presentee_at` | `TIMESTAMPTZ` | début de l'épreuve en cours ; NULL si aucune n'est affichée |
| `etat` | `VARCHAR(10)` NOT NULL DEFAULT 'en_cours' | `IN ('en_cours','terminee','close')` |
| `bonnes` | `SMALLINT` NOT NULL DEFAULT 0 | |
| `score_gagne` | `INT` NOT NULL DEFAULT 0 | |
| `temps_total_ms` | `INT` NOT NULL DEFAULT 0 | départage des duels et des défis |
| `created_at`, `terminee_at` | `TIMESTAMPTZ` | |

**Contraintes**

- `uq_partie_defi` : unique `(utilisateur_id, defi_id) WHERE defi_id IS NOT NULL` → une participation par défi (FR-034). Créer la partie **consomme** le défi.
- `uq_partie_duel` : unique `(duel_id, utilisateur_id) WHERE duel_id IS NOT NULL`.
- `ck_partie_cadre` : cohérence `cadre` ↔ `defi_id` / `duel_id`.

**Transitions**

- `en_cours` → `terminee` : la dernière épreuve a reçu une réponse ou une absence de réponse.
- `en_cours` → `close` : plus de 24 h depuis `created_at` (FR-019), constaté à la lecture. Les épreuves non présentées restent neuves pour le membre.

Index : `(utilisateur_id, created_at DESC)`.

---

## 5. `jeu.reponse`

| Colonne | Type | Règle |
|---|---|---|
| `id` | UUID PK | |
| `partie_id` | UUID NOT NULL → `jeu.partie` ON DELETE CASCADE | |
| `utilisateur_id` | UUID NOT NULL | dénormalisé, pour « jamais répondu » |
| `epreuve_id` | UUID NOT NULL → `jeu.epreuve` | |
| `cadre` | `VARCHAR(12)` NOT NULL | recopié de la partie |
| `rang` | `SMALLINT` NOT NULL | position dans la série, à partir de 1 |
| `proposition_choisie` | `SMALLINT` | NULL = sans réponse |
| `issue` | `VARCHAR(12)` NOT NULL | `IN ('bonne','mauvaise','sans_reponse','injouable')` |
| `temps_ms` | `INT` NOT NULL | plafonné au temps imparti |
| `created_at` | `TIMESTAMPTZ` | |

**Contraintes**

- `uq_reponse_rang` : unique `(partie_id, rang)` → une seule réponse par épreuve d'une partie, quelles que soient les répétitions d'envoi (FR-025).
- `uq_reponse_libre` : unique `(utilisateur_id, epreuve_id) WHERE cadre = 'libre'` → en partie libre, une épreuve ne se joue qu'une fois (FR-029). C'est une ceinture en plus du tirage, qui écarte déjà les épreuves vues.

**« Jamais répondu »** (FR-015) : aucune ligne de `reponse` pour `(utilisateur_id, epreuve_id)`, tous cadres confondus. Index `(utilisateur_id, epreuve_id)`.

L'insertion d'une réponse incrémente `epreuve.nombre_servie` et, si elle est bonne, `nombre_bonnes`, dans la même transaction. L'issue `injouable` n'incrémente rien.

---

## 6. `jeu.gain`

Le journal du score de jeu. Append-only.

| Colonne | Type | Règle |
|---|---|---|
| `id` | UUID PK | |
| `utilisateur_id` | UUID NOT NULL → `iam.utilisateur` ON DELETE CASCADE | |
| `montant` | `INT` NOT NULL | `> 0` |
| `origine` | `VARCHAR(10)` NOT NULL | `IN ('partie','defi','duel')` |
| `reference_id` | UUID NOT NULL | réponse, défi ou duel |
| `module_code` | `VARCHAR(30)` | pour la répartition (FR-026) |
| `saison_id` | UUID → `jeu.saison` | NULL hors saison |
| `pays_id` | UUID → `shared.pays` | **figé à l'écriture** ; NULL si aucun pays |
| `cle_idempotence` | `TEXT` NOT NULL UNIQUE | |
| `annule_at` | `TIMESTAMPTZ` | FR-031 |
| `annule_par` | UUID | |
| `motif_annulation` | `TEXT` | |
| `created_at` | `TIMESTAMPTZ` NOT NULL DEFAULT NOW() | |

**Clés d'idempotence**

| Événement | Clé |
|---|---|
| Bonne réponse comptée (partie libre, défi) | `reponse:{reponse_id}` |
| Prime d'achèvement d'un défi | `defi:{defi_id}:{utilisateur_id}` |
| Prime de duel (victoire, forfait, nul) | `duel:{duel_id}:{utilisateur_id}` |

Les bonnes réponses d'un **duel** ne rapportent pas de gain : seul le résultat du duel en rapporte. Sans cela, un duel vaudrait une partie libre plus une prime, et FR-048 (duels amicaux sans gain) serait contournable.

**Écriture** (`services::jeu::crediter`, dans la transaction de l'appelant) :

1. Lire `COALESCE(pays_origine_id, pays_residence_id)` du membre et la saison telle que `debut_at <= NOW() < fin_at`.
2. `INSERT … ON CONFLICT (cle_idempotence) DO NOTHING`.
3. Si une ligne est insérée : upsert de `score_saison` (si saison) et de `joueur`.

Index : `(utilisateur_id, created_at DESC)`, `(saison_id, pays_id)`.

---

## 7. `jeu.score_saison`

L'agrégat lu par les classements.

| Colonne | Type | Règle |
|---|---|---|
| `saison_id` | UUID NOT NULL → `jeu.saison` ON DELETE CASCADE | |
| `utilisateur_id` | UUID NOT NULL → `iam.utilisateur` ON DELETE CASCADE | |
| `pays_id` | UUID → `shared.pays` | NULL = sans pays |
| `score` | `INT` NOT NULL DEFAULT 0 | `>= 0` |
| `atteint_at` | `TIMESTAMPTZ` NOT NULL | instant du dernier gain, pour l'égalité (FR-058) |

Unicité : `UNIQUE NULLS NOT DISTINCT (saison_id, utilisateur_id, pays_id)` (PostgreSQL 16, déjà employé par `35c`).

Index : `(saison_id, pays_id, score DESC, atteint_at)`.

**Les trois classements** (FR-056), tous filtrés sur `u.etat = 'actif' AND u.deleted_at IS NULL` (FR-061) :

| Classement | Lecture |
|---|---|
| Tous les membres | `SUM(score)`, `MAX(atteint_at)` par `utilisateur_id`, tri `score DESC, atteint_at ASC` |
| Membres d'un pays | lignes `pays_id = $1`, même tri |
| Pays entre eux | par `pays_id` africain : somme des `regles.joueurs_par_pays` meilleurs scores (`ROW_NUMBER() OVER (PARTITION BY pays_id ORDER BY score DESC, atteint_at)`), tri sur cette somme |

Un membre qui a changé de pays en cours de saison a deux lignes : il figure dans deux classements de pays, et une fois au classement de tous les membres avec la somme. C'est la conséquence voulue de FR-054.

---

## 8. `jeu.joueur`

| Colonne | Type | Règle |
|---|---|---|
| `utilisateur_id` | UUID PK → `iam.utilisateur` ON DELETE CASCADE | |
| `score_total` | `INT` NOT NULL DEFAULT 0 | toutes saisons et hors saison |
| `serie_jours` | `SMALLINT` NOT NULL DEFAULT 0 | FR-037 |
| `serie_dernier_jour` | `DATE` | dernier jour UTC d'un défi du jour terminé |
| `created_at`, `updated_at` | `TIMESTAMPTZ` | |

Créé paresseusement au premier gain ou à la première partie (comme `engagement.compte`).

**Série de jours** : à la fin d'un défi du jour de date `d` : si `serie_dernier_jour = d - 1` alors `serie_jours + 1`, sinon `1`. Le défi commencé avant minuit UTC compte pour le jour où il a été commencé (`defi.periode_debut`). La série **affichée** vaut 0 si `serie_dernier_jour < aujourd'hui - 1` : la rupture se constate à la lecture, sans rien écrire.

---

## 9. `jeu.defi`

| Colonne | Type | Règle |
|---|---|---|
| `id` | UUID PK | |
| `periodicite` | `VARCHAR(8)` NOT NULL | `IN ('jour','semaine')` |
| `periode_debut` | `DATE` NOT NULL | jour UTC, ou lundi UTC de la semaine |
| `module_code` | `VARCHAR(30)` → `jeu.module` | thème facultatif |
| `titre` | `VARCHAR(120)` | facultatif, pour un défi programmé |
| `epreuve_ids` | `UUID[]` NOT NULL | série figée |
| `origine` | `VARCHAR(12)` NOT NULL | `IN ('programme','automatique')` |
| `cree_par` | UUID | NULL si automatique |
| `created_at` | `TIMESTAMPTZ` | |

- `uq_defi_periode` : `UNIQUE (periodicite, periode_debut)` → un seul défi par période, identique pour tous (FR-032, FR-035).
- `ck_defi_semaine` : `periodicite = 'semaine'` ⇒ `EXTRACT(ISODOW FROM periode_debut) = 1`.

**Composition automatique** : tirage parmi les épreuves servables des modules ouverts, en écartant celles qui figurent dans un défi des 30 derniers jours. S'il n'y en a pas assez, aucun défi n'est créé et l'écran le dit.

**Programmation** (FR-079) : un administrateur ne peut écrire que les périodes **à venir** ; une période commencée est figée.

---

## 10. `jeu.duel`

| Colonne | Type | Règle |
|---|---|---|
| `id` | UUID PK | |
| `proposant_id`, `adversaire_id` | UUID NOT NULL → `iam.utilisateur` ON DELETE CASCADE | `proposant_id <> adversaire_id` |
| `module_code` | `VARCHAR(30)` NOT NULL → `jeu.module` | |
| `mode` | `VARCHAR(8)` NOT NULL | `IN ('differe','direct')` |
| `etat` | `VARCHAR(10)` NOT NULL DEFAULT 'propose' | `IN ('propose','accepte','en_cours','termine','refuse','annule','expire')` |
| `compte` | `BOOLEAN` NOT NULL DEFAULT TRUE | FALSE = amical, décidé à l'acceptation (FR-048) |
| `epreuve_ids` | `UUID[]` | figée à l'acceptation (FR-041) |
| `propose_at` | `TIMESTAMPTZ` NOT NULL | |
| `accepte_at` | `TIMESTAMPTZ` | |
| `echeance_at` | `TIMESTAMPTZ` NOT NULL | échéance de l'étape en cours |
| `issue` | `VARCHAR(12)` | `IN ('victoire','nul','forfait','sans_issue')` |
| `vainqueur_id` | UUID | non nul si et seulement si `issue IN ('victoire','forfait')` |
| `termine_at` | `TIMESTAMPTZ` | |
| `rang_courant` | `SMALLINT` NOT NULL DEFAULT 0 | **direct** : manche en cours |
| `manche_debut_at` | `TIMESTAMPTZ` | **direct** : début de la manche |
| `presence_proposant_at`, `presence_adversaire_at` | `TIMESTAMPTZ` | **direct** : dernier battement |

**Cycle, commun aux deux modes** (FR-040)

```text
propose ──accepter──> accepte ──(1er joueur joue / les 2 présents)──> en_cours ──> termine
   │ refuser ──> refuse
   │ annuler (proposant) ──> annule
   └ échéance ──> expire        accepte / en_cours ──échéance sans aucun joueur──> expire
                                accepte / en_cours ──amitié rompue, blocage, suspension──> annule
```

**Échéances** (`regles`)

| Étape | Différé | Direct |
|---|---|---|
| Proposition sans réponse | `propose_at + delai_duel_h` (48 h) | `propose_at + delai_direct_min` (5 min) |
| Après acceptation | `accepte_at + delai_duel_h` pour jouer | présence des deux sous `delai_direct_min` |

**Résolution** (`services::jeu::resoudre_duel`, sous `FOR UPDATE`, appelée à chaque lecture ou écriture)

| Constat | Effet |
|---|---|
| `propose` et échéance passée | `expire` |
| Amitié absente, blocage, ou compte non actif, et duel non terminé | `annule`, sans gain (FR-049) |
| Différé, les deux parties terminées | `termine`, vainqueur par FR-047 |
| Différé, échéance passée : les parties commencées et non finies sont closes en l'état (ce qui a été répondu compte) | puis l'une des deux lignes ci-dessous |
| Différé, échéance passée, un seul joueur a une partie | `termine`, `issue = 'forfait'` |
| Différé, échéance passée, aucune partie terminée | `expire` |
| Direct, manche close (deux réponses, ou temps écoulé) | manche suivante, ou `termine` après la dernière |
| Direct, un joueur absent au-delà de `grace_direct_s` | `termine`, `issue = 'forfait'` pour l'autre |
| Direct, les deux absents au-delà du délai de grâce | `termine`, `issue = 'sans_issue'` |

**Vainqueur** (FR-047) : plus de `bonnes` ; à égalité, moins de `temps_total_ms` ; à égalité parfaite, `nul`.

**Gains** (si `compte`) : `prime_duel_victoire` au vainqueur (victoire ou forfait), `prime_duel_nul` à chacun en cas de nul. Rien pour `sans_issue`, `expire`, `annule`.

**Duel compté ou amical** : à l'acceptation, on compte les duels `compte = TRUE` acceptés depuis le début du jour UTC, entre les deux mêmes membres (`duels_comptes_par_paire_jour`) et pour chacun (`duels_comptes_par_membre_jour`). Au-delà, `compte = FALSE`. La proposition renvoie déjà l'information, pour prévenir avant de commencer.

**Conversion** (FR-046) : un duel direct `expire` peut être rouvert par son proposant en différé : nouvelle ligne `mode = 'differe'`, `etat = 'propose'`. L'ancien reste `expire`.

Index : `(proposant_id, etat)`, `(adversaire_id, etat)`.

---

## 11. `jeu.saison`

| Colonne | Type | Règle |
|---|---|---|
| `id` | UUID PK | |
| `nom` | `VARCHAR(120)` NOT NULL | |
| `debut_at`, `fin_at` | `TIMESTAMPTZ` NOT NULL | `fin_at > debut_at` |
| `cloturee_at` | `TIMESTAMPTZ` | distinctions de podium attribuées |
| `cree_par` | UUID | |
| `created_at`, `updated_at` | `TIMESTAMPTZ` | |

- `ex_saison_chevauchement` : `EXCLUDE USING gist (tstzrange(debut_at, fin_at, '[)') WITH &&)`. Deux saisons ne peuvent pas se chevaucher, **en SQL** (FR-078). Les types d'intervalle ont leur classe d'opérateurs GiST native : aucune extension à ajouter.
- L'état (`a_venir`, `en_cours`, `close`) se **déduit** des dates. Rien ne « passe » d'un état à l'autre : FR-062 et SC-013 sont vrais sans traitement.
- La date de début d'une saison commencée n'est plus modifiable (contrôle API, FR-078). « Clore » une saison, c'est ramener `fin_at` à maintenant.

**Podium** : au premier affichage après `fin_at`, la clôture paresseuse (research D6) attribue `jeu_podium_saison` aux trois premiers de tous les membres et au premier de chaque pays africain (FR-063), avec la clé `jeu:podium:{saison_id}:{utilisateur_id}`.

---

## 12. `jeu.signalement_epreuve`

| Colonne | Type | Règle |
|---|---|---|
| `id` | UUID PK | |
| `epreuve_id` | UUID NOT NULL → `jeu.epreuve` ON DELETE CASCADE | |
| `utilisateur_id` | UUID NOT NULL → `iam.utilisateur` ON DELETE CASCADE | |
| `motif` | `VARCHAR(20)` NOT NULL | `IN ('reponse_erronee','enonce_ambigu','contenu_deplace','autre')` |
| `commentaire` | `TEXT` | |
| `etat` | `VARCHAR(12)` NOT NULL DEFAULT 'en_attente' | `IN ('en_attente','confirme','classe')` |
| `decision_par`, `decision_at` | UUID, `TIMESTAMPTZ` | |
| `created_at` | `TIMESTAMPTZ` | |

- `uq_signalement_epreuve` : `UNIQUE (epreuve_id, utilisateur_id)` (FR-082).
- Le signalement n'est accepté que si le membre a une réponse sur cette épreuve.
- Sur une épreuve déjà `retiree` ou `rejetee`, il est inséré directement `classe` (cas limite de la spec).
- `confirme` crédite `jeu_signalement_confirme` au signaleur, clé `jeu:signalement:{signalement_id}`.

---

## 13. `jeu.regles` (singleton)

`id BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (id)`, le patron de `engagement.parametre_monetisation`.

| Colonne | Défaut | Contrainte |
|---|---|---|
| `taille_partie` | 10 | 3 à 30 |
| `taille_defi_jour` | 5 | 3 à 30 |
| `taille_defi_semaine` | 15 | 3 à 30 |
| `taille_duel` | 7 | 3 à 30 |
| `temps_epreuve_s` | 30 | 5 à 120 |
| `score_facile`, `score_moyen`, `score_difficile` | 1, 2, 3 | > 0 |
| `prime_defi_jour`, `prime_defi_semaine` | 5, 15 | ≥ 0 |
| `prime_duel_victoire`, `prime_duel_nul` | 5, 2 | ≥ 0 |
| `delai_duel_h` | 48 | 1 à 168 |
| `delai_direct_min` | 5 | 1 à 60 |
| `grace_direct_s` | 60 | 10 à 300 |
| `pause_revelation_s` | 5 | 2 à 15 |
| `duels_comptes_par_paire_jour` | 3 | ≥ 0 |
| `duels_comptes_par_membre_jour` | 10 | ≥ 0 |
| `joueurs_par_pays` | 10 | 1 à 100 |
| `updated_at` | | |

Un changement ne vaut que pour l'avenir (FR-077) : les séries sont figées à leur création et les gains déjà écrits ne sont pas recalculés.

Les montants de **réputation** ne sont pas ici : ils sont dans `engagement.regle_points` (research D3).

---

## 14. Ajouts au schéma `engagement` (dans la même migration)

- Une catégorie `jeux` dans `engagement.categorie_points`.
- Six règles dans `engagement.regle_points`, toutes à `points = 0`, rattachées à la catégorie `jeux` :

| `type_action` | `reputation_delta` | Déclencheur | Clé d'idempotence |
|---|---|---|---|
| `jeu_premiere_partie` | 0 | première partie terminée | `jeu:premiere:{uid}` |
| `jeu_defi_termine` | 1 | défi terminé | `jeu:defi:{defi_id}:{uid}` |
| `jeu_serie_7_jours` | 0 | série de jours atteignant un multiple de 7 | `jeu:serie:{uid}:{jour}` |
| `jeu_duel_gagne` | 1 | duel compté gagné (victoire ou forfait) | `jeu:duel:{duel_id}:{uid}` |
| `jeu_podium_saison` | 10 | place d'honneur en fin de saison | `jeu:podium:{saison_id}:{uid}` |
| `jeu_signalement_confirme` | 2 | signalement d'épreuve confirmé | `jeu:signalement:{signalement_id}` |

- Cinq badges dans `engagement.badge`, condition `actions_comptees` :

| Code | Action comptée | Seuil |
|---|---|---|
| `jeu_premier_pas` | `jeu_premiere_partie` | 1 |
| `jeu_assidu` | `jeu_serie_7_jours` | 1 |
| `jeu_duelliste` | `jeu_duel_gagne` | 10 |
| `jeu_champion` | `jeu_podium_saison` | 1 |
| `jeu_vigie` | `jeu_signalement_confirme` | 3 |

- Permission `jeu.gerer`, rattachée à `super_admin` et `admin`.

---

## 15. Correspondance des types

| SQL | Rust (`models/jeu.rs`) | TypeScript (`useJeu.ts`) |
|---|---|---|
| `jeu.module` | `ModuleJeu` | `ModuleJeuAPI` |
| `jeu.epreuve` (servie) | `EpreuveServie` : sans `bonne_reponse` ni `explication` | `EpreuveServieAPI` |
| `jeu.epreuve` (corrigée) | `Correction` | `CorrectionAPI` |
| `jeu.epreuve` (admin) | `EpreuveAdmin` (`models/admin/jeu.rs`) | `EpreuveAdminAPI` |
| `jeu.partie` | `PartieResponse` | `PartieAPI` |
| `jeu.defi` | `DefiResponse` | `DefiAPI` |
| `jeu.duel` | `DuelResponse`, `EtatDuelDirect` | `DuelAPI`, `EtatDuelDirectAPI` |
| `jeu.saison` | `SaisonResponse` (état dérivé) | `SaisonAPI` |
| `jeu.score_saison` | `LigneClassement`, `LignePays` | `LigneClassementAPI`, `LignePaysAPI` |
| `jeu.regles` | `ReglesJeu` | `ReglesJeuAPI` |

**Règle de sérialisation qui porte la sécurité** : `EpreuveServie` est un type distinct de `Correction`. La bonne réponse n'existe dans aucun type renvoyé avant la réponse du membre ; ce n'est pas un champ qu'on pense à omettre, c'est un champ qui n'existe pas (FR-027).

# Data Model : Jeux variés et concours communautaires

**Feature** : `014-jeux-concours` · **Migration** : `uafricas_backend/doc/bd/schemas/38_jeu_concours.sql` (additive, rejouable, incluse par `schema.sql` après `37_jeu.sql`)

Tout est dans le schéma `jeu` de la 013. Conventions : UUID v4, `TIMESTAMPTZ`, noms français, CHECK plutôt qu'enum (l'ajout d'une valeur ne demande qu'un `DROP CONSTRAINT … ADD CONSTRAINT`), idempotence par `IF NOT EXISTS` / `ON CONFLICT DO NOTHING` / `DROP CONSTRAINT IF EXISTS`.

---

## 1. Épreuve étendue (`jeu.epreuve`) — research D1

Colonnes ajoutées :

| Colonne | Type | Défaut | Rôle |
|---|---|---|---|
| `type_reponse` | `VARCHAR(10)` | `'choix'` | `choix`, `carte`, `ordre`, `paires` |
| `solution` | `SMALLINT[]` | NULL | ordre : rangs des éléments dans l'ordre attendu ; paires : `solution[i]` = rang dans `appariements` du correspondant de `propositions[i]` |
| `appariements` | `TEXT[]` | NULL | paires : éléments de droite, stockés dans un ordre aléatoire |
| `valeurs` | `TEXT[]` | NULL | ordre : justification de chaque élément, alignée sur `propositions`, montrée à la correction seulement |
| `reponse_pays_id` | `UUID` → `shared.pays` | NULL | carte : le pays attendu |

`bonne_reponse` : `DROP NOT NULL`.

Contraintes (remplacent `ck_epreuve_propositions` et `ck_epreuve_bonne_reponse`) :

```
ck_epreuve_type_reponse   type_reponse IN ('choix','carte','ordre','paires')
ck_epreuve_choix          type_reponse <> 'choix'  OR (cardinality(propositions) BETWEEN 2 AND 6
                                                       AND bonne_reponse BETWEEN 1 AND cardinality(propositions))
ck_epreuve_carte          type_reponse <> 'carte'  OR (reponse_pays_id IS NOT NULL AND cardinality(propositions) = 0)
ck_epreuve_ordre          type_reponse <> 'ordre'  OR (cardinality(propositions) BETWEEN 3 AND 6
                                                       AND cardinality(solution) = cardinality(propositions)
                                                       AND jeu.est_permutation(solution))
ck_epreuve_paires         type_reponse <> 'paires' OR (cardinality(propositions) BETWEEN 3 AND 5
                                                       AND cardinality(appariements) = cardinality(propositions)
                                                       AND cardinality(solution) = cardinality(propositions)
                                                       AND jeu.est_permutation(solution))
ck_epreuve_bonne_si_choix (bonne_reponse IS NOT NULL) = (type_reponse = 'choix')
ck_epreuve_pays_si_carte  (reponse_pays_id IS NOT NULL) = (type_reponse = 'carte')
ck_epreuve_valeurs        valeurs IS NULL OR (type_reponse = 'ordre' AND cardinality(valeurs) = cardinality(propositions))
```

`jeu.est_permutation(smallint[]) RETURNS boolean IMMUTABLE` : vrai si le tableau contient exactement `1..n`, chacun une fois.

Les doublons d'éléments (même texte deux fois dans un ordre ou une paire) sont refusés par l'API (FR-017) : un CHECK sur l'unicité des éléments d'un tableau serait illisible.

## 2. Réponse étendue (`jeu.reponse`) — research D2

| Colonne | Type | Rôle |
|---|---|---|
| `reponse_detail` | `SMALLINT[]` | ordre : les clés dans l'ordre proposé ; paires : clé de droite choisie pour chaque clé de gauche, dans l'ordre des clés de gauche |
| `pays_choisi_id` | `UUID` → `shared.pays` | carte : le pays désigné |

L'issue (`bonne`, `mauvaise`, `sans_reponse`, `injouable`), l'unicité `(partie_id, rang)` et tout ce qui en dépend restent ceux de la 013.

## 3. Règles du jeu (`jeu.regles`) — research D3, D10

| Colonne | Type | Défaut | Borne |
|---|---|---|---|
| `majoration_ordre_paires_s` | `SMALLINT` | 15 | 0 à 60 |
| `prime_concours_participation` | `SMALLINT` | 2 | ≥ 0 |
| `prime_concours_podium` | `SMALLINT[]` | `{30,20,10}` | 3 valeurs ≥ 0, décroissantes |

## 4. Gain (`jeu.gain`)

`ck_gain_origine` élargi : `origine IN ('partie', 'defi', 'duel', 'concours')`. Rien d'autre ne change : `crediter` reste le seul écrivain, et la clé d'idempotence le seul verrou.

## 5. Concours (`jeu.concours`) — research D6, D13

| Colonne | Type | Contrainte / rôle |
|---|---|---|
| `id` | UUID PK | |
| `format` | `VARCHAR(12)` | `photo` (CHECK, extensible : `video`, `traduction`, `texte`, `idee`) |
| `titre` | `VARCHAR(150)` | non vide |
| `theme` | `TEXT` | non vide |
| `reglement` | `TEXT` | non vide |
| `rattachement` | `VARCHAR(30)` | NULL ou `^[a-z_]{3,30}$` (code de module de la plateforme) |
| `image_url` | `VARCHAR(500)` | visuel facultatif du concours |
| `appel_debut`, `vote_debut`, `vote_fin` | `TIMESTAMPTZ` | `appel_debut < vote_debut`, `vote_fin >= vote_debut + 24 h` (FR-022) |
| `participations_max` | `SMALLINT` | défaut 1, 1 à 10 |
| `minimum_participations` | `SMALLINT` | défaut 4, ≥ 2 |
| `votes_max` | `INT` | NULL = sans plafond, sinon ≥ 10 (FR-038) |
| `presentations_min` | `SMALLINT` | défaut 10, ≥ 1 (D8) |
| `jury` | `BOOLEAN` | défaut FALSE |
| `jury_finalistes` | `SMALLINT` | défaut 10, 3 à 30 |
| `jury_delai_jours` | `SMALLINT` | défaut 7, 1 à 30 |
| `prime_participation` | `SMALLINT` | NULL = règle du jeu |
| `prime_podium` | `SMALLINT[]` | NULL = règle du jeu ; sinon 3 valeurs |
| `etat` | `VARCHAR(10)` | `actif`, `annule`, `resultats` |
| `motif_annulation` | `TEXT` | requis si `annule` |
| `podium_jury` | `UUID[]` | participations retenues par le jury, dans l'ordre (≤ 3) |
| `delibere_par` | UUID → `iam.utilisateur` | |
| `delibere_at` | `TIMESTAMPTZ` | |
| `resultats_at` | `TIMESTAMPTZ` | requis si `resultats` |
| `cree_par`, `created_at`, `updated_at` | | |

Index : `(vote_fin)`, `(rattachement, vote_fin)`.

### Phase (calculée, jamais stockée)

```
état annule                                                     → annule
maintenant < appel_debut                                        → a_venir
maintenant < vote_debut                                         → appel
maintenant < vote_fin                                           → vote          ⟵ annulation possible à l'entrée (minimum non atteint)
jury ∧ delibere_at IS NULL ∧ maintenant < vote_fin + délai jury → deliberation
sinon                                                           → resultats     ⟵ établissement à la première lecture
```

### Transitions d'état écrites

```
actif ──(1ʳᵉ lecture en phase vote, publiées < minimum)──▶ annule      [+ primes de participation, notifications]
actif ──(1ʳᵉ lecture en phase resultats)───────────────▶ resultats   [+ resultat_concours, primes, réputation, distinctions, notifications]
```

Chacune sous `SELECT … FOR UPDATE` sur la ligne du concours, et seulement si l'état est encore `actif`. Effets d'engagement et notifications **après le COMMIT**.

## 6. Participation (`jeu.participation`) — research D6, D12

| Colonne | Type | Contrainte / rôle |
|---|---|---|
| `id` | UUID PK | |
| `concours_id` | UUID → `jeu.concours` ON DELETE CASCADE | |
| `auteur_id` | UUID → `iam.utilisateur` ON DELETE CASCADE | |
| `media_type` | `VARCHAR(10)` | `image` (CHECK, extensible) |
| `media_url` | `VARCHAR(500)` | non vide |
| `legende` | `VARCHAR(200)` | facultative |
| `etat` | `VARCHAR(12)` | `en_attente`, `publiee`, `rejetee`, `retiree`, `suspendue` |
| `motif_rejet` | `TEXT` | requis si `rejetee` |
| `modere_par`, `modere_at` | | |
| `nombre_presentations` | `INT` | défaut 0 ; incrémenté à chaque présentation (D7) |
| `nombre_signalements` | `INT` | défaut 0 |
| `created_at`, `updated_at` | | |

Index : `(concours_id, etat)`, `(concours_id, nombre_presentations) WHERE etat = 'publiee'`, `(auteur_id)`.

Le nombre de participations actives d'un membre (états `en_attente` et `publiee`) ne dépasse pas `participations_max`. Le contrôle est fait par l'API, sous verrou de la ligne du concours. Un index unique ne sait pas exprimer « au plus N ».

### Cycle

```
en_attente ──accepter──▶ publiee ──retirer (auteur)──▶ retiree
     │                      └──── >10 signalements ──▶ suspendue ──rétablir (admin)──▶ publiee
     ├──rejeter (motif)──▶ rejetee
     ├──remplacer (auteur) : même ligne, nouveau media_url, reste en_attente
     └──retirer (auteur)──▶ retiree
```

## 7. Confrontation (`jeu.confrontation`) — research D7, D9

| Colonne | Type | Contrainte / rôle |
|---|---|---|
| `id` | UUID PK | |
| `concours_id` | UUID → `jeu.concours` ON DELETE CASCADE | |
| `votant_id` | UUID → `iam.utilisateur` ON DELETE CASCADE | |
| `a_id`, `b_id` | UUID → `jeu.participation` | `a_id < b_id` (paire canonique) |
| `gauche_est_a` | `BOOLEAN` | côté d'affichage, tiré à la présentation |
| `presentee_at` | `TIMESTAMPTZ` | horloge serveur |
| `choix_id` | UUID | NULL tant que non votée ; sinon `a_id` ou `b_id` (CHECK) |
| `vote_at` | `TIMESTAMPTZ` | requis si `choix_id` |
| `comptee` | `BOOLEAN` | fixée au vote |
| `motif_ecart` | `VARCHAR(16)` | NULL si comptée ; sinon `trop_rapide`, `compte_recent`, `non_verifie`, `ecartee_admin` |

Contraintes :
- `UNIQUE (concours_id, votant_id, a_id, b_id)` : une paire n'est jamais représentée à un même votant (FR-036).
- `UNIQUE (concours_id, votant_id) WHERE choix_id IS NULL` : au plus une confrontation ouverte par votant (D7).
- CHECK `choix_id IS NULL OR choix_id IN (a_id, b_id)` ; CHECK `(choix_id IS NULL) = (vote_at IS NULL)` ; CHECK `comptee = (motif_ecart IS NULL)` quand `choix_id` est renseigné.

L'auto-vote (FR-035) est exclu **au tirage** : le votant ne reçoit que des paires sans participation de lui. L'API refuse aussi un vote sur une confrontation dont il serait l'auteur d'un des côtés (défense en profondeur).

## 8. Résultat de concours (`jeu.resultat_concours`) — research D8

| Colonne | Type | Rôle |
|---|---|---|
| `concours_id`, `participation_id` | PK composite | |
| `rang` | `SMALLINT` | ex aequo = même rang |
| `victoires`, `duels` | `INT` | voix comptées seulement |
| `taux` | `NUMERIC(5,4)` | `victoires / duels`, 0 si aucun duel |
| `sous_seuil` | `BOOLEAN` | moins de `presentations_min` duels comptés : classée après, hors podium |
| `place_jury` | `SMALLINT` | 1 à 3 si fixée par le jury |

Écrit une seule fois, à l'établissement ; jamais mis à jour.

## 9. Signalement de participation (`jeu.signalement_participation`) — research D12

`(id, participation_id, utilisateur_id, motif TEXT NOT NULL, created_at)`, `UNIQUE (participation_id, utilisateur_id)`. Le seuil est le même que pour les médias : plus de 10 signalements distincts → `suspendue`.

## 10. Engagement (données ajoutées) — research D10

- `engagement.regle_points` : `jeu_concours_participation` (réputation 1), `jeu_concours_podium` (5), `jeu_concours_vote` (1), toutes à **0 point**, catégorie `jeux`.
- `engagement.badge` : `jeu_laureat` (1 × `jeu_concours_podium`), `jeu_jure_populaire` (5 × `jeu_concours_vote`).
- `ACTIONS_INSTRUMENTEES` (Rust) : les trois codes.

Clés d'idempotence :

| Effet | Clé |
|---|---|
| prime de participation | `concours:{concours_id}:participation:{participation_id}` |
| prime de podium | `concours:{concours_id}:podium:{participation_id}` |
| réputation de participation | `jeu:concours:{concours_id}:participation:{participation_id}` |
| réputation de podium | `jeu:concours:{concours_id}:podium:{participation_id}` |
| réputation de votant | `jeu:concours:{concours_id}:votant:{utilisateur_id}` |

## 11. Notifications (types ajoutés, `arbre_genealogique.notifications`)

`jeu.participation_acceptee`, `jeu.participation_rejetee`, `jeu.participation_suspendue`, `jeu.concours_resultats` (participants), `jeu.concours_laureat` (podium), `jeu.concours_annule`.

## 12. Fichiers

- `uploads/jeu/images/`, `uploads/jeu/audios/` : médias d'épreuve, déposés ou copiés à la dérivation (D4).
- `uploads/jeu/concours/{concours_id}/` : photos de participation, normalisées (EXIF supprimées).

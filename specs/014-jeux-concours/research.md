# Research : Jeux variés et concours communautaires

**Feature** : `014-jeux-concours` · **Date** : 2026-10-04 · **Spec** : [spec.md](./spec.md)

Chaque décision suit le format *Décision / Rationale / Alternatives écartées*. Le socle de référence est celui de la 013 : [../013-activites-ludiques/research.md](../013-activites-ludiques/research.md).

---

## D1. Stocker les nouveaux types de réponse : des colonnes typées, pas du JSONB

**Décision** : `jeu.epreuve` gagne une colonne discriminante `type_reponse` (`choix` par défaut, `carte`, `ordre`, `paires`) et trois colonnes facultatives, chacune propre à un type, tenues par des CHECK :

| Type | `propositions` (existant) | `solution SMALLINT[]` | `appariements TEXT[]` | `reponse_pays_id UUID` | `valeurs TEXT[]` | `bonne_reponse` |
|---|---|---|---|---|---|---|
| `choix` | 2 à 6 propositions | — | — | — | — | rang de la bonne |
| `carte` | vide | — | — | le pays attendu | — | — |
| `ordre` | 3 à 6 éléments, **stockés dans un ordre aléatoire** | les rangs des éléments dans l'ordre attendu | — | — | justification de chaque élément (« 46,0 millions d'habitants ») | — |
| `paires` | 3 à 5 éléments de gauche | `solution[i]` = rang, dans `appariements`, du correspondant de l'élément `i` | 3 à 5 éléments de droite, **stockés dans un ordre aléatoire** | — | — | — |

`bonne_reponse` devient facultative ; un CHECK impose qu'elle soit renseignée **si et seulement si** `type_reponse = 'choix'`. Les CHECK de cohérence (cardinalités, `solution` permutation de 1..n de la bonne longueur, `reponse_pays_id` présent pour `carte` seulement) rendent une épreuve incohérente **impossible en SQL**, l'API nommant le défaut avant d'y arriver (principe III, même posture que `ck_epreuve_jouable`).

**Le principe de la clé opaque, déjà au cœur de la 013, s'étend tel quel.** Le joueur ne reçoit jamais que des **clés** : le rang d'un élément dans le tableau stocké. Comme ce tableau est rangé au hasard à la création, la clé ne dit rien de la solution, exactement comme la clé d'une proposition ne dit pas laquelle est la bonne. L'énoncé servi est ensuite mélangé à chaque présentation (ordre stable en duel direct, via `servir_stable`).

**Rationale** :
- Un JSONB `donnees_reponse` aurait tout accepté : aucun CHECK lisible ne vérifierait qu'un « ordre » a une solution qui est une permutation, et `EpreuveRow` se décoderait en deux temps. Les colonnes typées gardent la règle dans le schéma.
- Le choix multiple, de loin le plus fréquent, ne change pas : `propositions` et `bonne_reponse` gardent leur sens, aucune épreuve existante n'est réécrite (`type_reponse` naît `'choix'` par défaut).
- Une `carte` n'a pas besoin de propositions : les 55 pays sont connus du client (`NOMS_PAYS_FR`, et le composant de carte renvoie un code ISO). Le joueur répond un **code ISO**, résolu en pays côté serveur.

**Alternatives écartées** :
- *Une table par type de réponse* : quatre jointures à chaque tirage, et `SERVABLE_SQL` (la définition unique d'une épreuve servable) se compliquerait pour rien.
- *Ramener l'ordre et les paires à du choix multiple* (« laquelle de ces séquences est la bonne ? ») : c'est exactement l'uniformité que le client reproche.

---

## D2. La réponse du joueur : un type unique, une évaluation unique

**Décision** :
- Côté API, la réponse devient un objet à une seule clé renseignée : `{ cle }` (choix, inchangé), `{ ordre: [cles…] }`, `{ paires: [cle_droite pour chaque cle_gauche, dans l'ordre des clés de gauche] }`, `{ pays: "sn" }`. Un envoi `cle: null` reste le « temps écoulé » de la 013.
- Côté base, `jeu.reponse` gagne `reponse_detail SMALLINT[]` (ordre ou paires, tels que joués) et `pays_choisi_id UUID` (carte). `proposition_choisie` garde son sens pour le choix multiple.
- **Une seule fonction `evaluer(epreuve, reponse) -> bool`** dans `services/jeu.rs`, appelée par `inscrire_reponse` (parties, défis, duel différé) **et** par `repondre_direct` (duel direct). Elle refuse en 400 une réponse qui ne correspond pas au type de l'épreuve, ou qui n'est pas une permutation des clés servies.
- `Correction` et `CorrectionManche` gagnent ce qu'il faut pour afficher la solution complète : `type_reponse`, `solution` (clés dans l'ordre attendu, ou appariement attendu), `valeurs` pour l'ordre, `bon_pays { iso, nom }` pour la carte, et la réponse jouée.

**Rationale** : la 013 a une seule règle de correction (`c == epreuve.bonne_reponse`), dupliquée en deux endroits (partie et duel direct). En faire une fonction partagée garantit que la même réponse donne la même issue dans tous les cadres (FR-011). L'issue (`bonne`/`mauvaise`/`sans_reponse`/`injouable`) et tout ce qui en découle (score, compteurs, primes, vainqueur de manche) restent **inchangés**.

**Alternatives écartées** : un barème partiel pour l'ordre et les paires (spec FR-006 : tout ou rien) ; il casserait aussi l'équivalence « une bonne réponse = un gain » sur laquelle reposent les défis et les duels.

---

## D3. Le temps majoré de l'ordre et des paires

**Décision** : `jeu.regles` gagne `majoration_ordre_paires_s` (défaut 15 s, bornée 0 à 60). `delai_ms` l'ajoute pour `ordre` et `paires`, comme il ajoute déjà `MARGE_MEDIA_MS` pour un média. Elle se règle depuis l'écran des règles du jeu de la 013.

**Rationale** : déplacer quatre éléments demande plus que cliquer une proposition (SC-003 : moins de 30 s pour cinq éléments). Le temps reste calculé **par le serveur** à partir de `presentee_at` ; le duel direct en hérite, puisque son échéance de manche s'appuie sur `delai_epreuve_ms`.

---

## D4. Les médias d'épreuve : déposés par l'administrateur, possédés par l'épreuve

**Décision** :
- Nouvelle route d'administration `POST /api/admin/jeu/medias` (multipart), qui renvoie un `media_url` à placer dans le formulaire d'épreuve. Les photos passent par `image_validation::normaliser_photo` : formats JPEG/PNG/WebP, redimensionnement, ré-encodage. Le ré-encodage **supprime au passage les métadonnées EXIF**, dont la position GPS.
- **Son, sans dépendance nouvelle** : le format est reconnu par sa **signature binaire** (MP3 : ID3 ou synchronisation de trame ; OGG : `OggS` ; M4A : `ftyp` ; WAV : `RIFF…WAVE`), pas par l'extension. Le poids est borné à 1 Mo, soit environ 30 s à 256 kbit/s. La durée exacte est lue **par le navigateur de l'administrateur** (`<audio>.duration`) avant l'envoi, et l'écran refuse au-delà de 30 s. Le plafond de poids côté serveur reste le garde-fou.
- Stockage : `uploads/jeu/images/` et `uploads/jeu/audios/`, noms de fichier en UUID, servis par le `/uploads/` existant.
- **Une épreuve dérivée avec photo COPIE l'image source** dans `uploads/jeu/images/` au moment de la dérivation, au lieu de pointer vers l'image du site ou de la recette.

**Rationale** :
- Copier découple l'épreuve de la source. Remplacer la photo d'une recette ne transforme pas une épreuve déjà revue en question sur une autre image. Supprimer la recette rend l'épreuve non servable par `v_source` (règle de la 013), sans fichier orphelin à surveiller au tirage (FR-016). Et la vue `v_source` **n'a pas à changer** : son empreinte n'a pas à couvrir les images.
- Mesurer exactement la durée d'un MP3 suppose de décoder ses trames, ce qui demande une bibliothèque audio, donc une dépendance nouvelle pour un garde-fou. Un plafond de poids et une mesure côté navigateur suffisent pour des extraits déposés par des administrateurs.

**Alternatives écartées** :
- *`symphonia` ou `ffprobe`* : une dépendance, ou un binaire dans l'image Docker, pour un contrôle d'administration.
- *Garder l'URL saisie à la main (état de la 013)* : c'est précisément le manque relevé par l'histoire 2.

---

## D5. La dérivation des nouveaux types

**Décision** : la struct `Forme` gagne `type_reponse`, et sa requête peut rendre, en plus de `enonce`/`bonne`/`explication`, une `solution`, des `appariements`, des `valeurs` ou un `reponse_pays_id` (colonnes `#[sqlx(default)]`, comme `distracteurs` aujourd'hui). Une épreuve dérivée a toujours **une** source, le **pivot** (la fiche dont part la question). Les autres éléments sont tirés au hasard parmi les sources admissibles, comme les distracteurs des comparaisons livrées dans la 013.

Formes ajoutées :

| Forme | Type | Pivot | Garde-fou d'ambiguïté (FR-019) |
|---|---|---|---|
| `ordre_population` | ordre | fiche pays | 4 pays, écarts d'au moins 15 % entre deux valeurs consécutives |
| `ordre_superficie` | ordre | fiche pays | idem |
| `paires_capitales` | paires | fiche pays | 4 pays aux capitales distinctes |
| `paires_monnaies` | paires | fiche pays | 4 pays aux monnaies **toutes distinctes** (deux pays en franc CFA ne sont jamais réunis) |
| `carte_capitale` | carte | fiche pays | capitale non vide (propre à un pays) |
| `carte_site` | carte | site touristique | — (un site a un pays) |
| `carte_peuple` | carte | groupe ethnique | seulement si aucun autre pays ne déclare ce peuple ni ne le cite dans ses langues |
| `photo_recette` | choix + image | recette | image non vide ; distracteurs : pays d'Afrique |
| `photo_site` | choix + image | site touristique | image non vide ; distracteurs : pays d'Afrique |

Les formes « paires » de Codimoi (`paires_proverbes` : 4 proverbes de 4 pays distincts) et d'Afrolang restent de la **saisie** tant qu'Afrolang n'a pas de référentiel. Les dates d'indépendance ne sont pas un champ publié : les « ordre » chronologiques sont saisis.

**Rationale** : on reprend la mécanique de la 013 (`FORMES`, revue obligatoire, unicité `(type_source, source_id, forme)`), donc rien n'est jouable sans revue (FR-020), et le tirage varié (`serie_variee_sql`) voit les nouvelles formes comme des familles distinctes. FR-012 est ainsi tenu sans ligne de code supplémentaire.

---

## D6. Le cycle d'un concours, résolu à la lecture

**Décision** : un concours porte ses **dates** et un **état écrit** (`actif`, `annule`, `resultats`). Sa **phase** se calcule à chaque lecture :

```
maintenant < appel_debut                                        → a_venir
appel_debut ≤ maintenant < vote_debut                           → appel
vote_debut ≤ maintenant < vote_fin   (et état actif)            → vote       (*)
maintenant ≥ vote_fin, jury prévu, non délibéré, délai non échu → deliberation
maintenant ≥ vote_fin, sinon                                    → resultats  (*)
état annule                                                     → annule
```

(*) Deux transitions **écrivent** : l'annulation, quand on lit pour la première fois la phase de vote et que le minimum de participations publiées n'est pas atteint (FR-033) ; l'**établissement des résultats** (FR-042). Toutes deux s'exécutent sous `SELECT … FOR UPDATE` sur la ligne du concours, ne font rien si l'état n'est plus `actif`, et publient leurs effets (réputation, distinctions, notifications) **après le COMMIT**. C'est le modèle de `cloturer_saisons_echues` et de `resoudre_duel` dans la 013. Une fonction `resoudre_concours(pool, id)` est appelée par **toute** route qui lit un concours.

**Indépendance du format (FR-025)** : `jeu.concours.format` (CHECK, `photo` seul pour l'instant) et `jeu.participation` ne savent que trois choses de la création : un `media_url`, une `legende`, un `media_type`. Un futur défi vidéo ajoute une valeur de format et ses contrôles de dépôt ; une traduction ou un texte ajouteront une colonne `texte`. Le vote, les résultats, les récompenses, le jury et la modération ne lisent jamais la création elle-même.

**Alternatives écartées** : une tâche planifiée qui bascule les phases. La plateforme n'en a aucune (contrainte héritée), et la résolution à la lecture a fait ses preuves sur les saisons et les duels de la 013.

---

## D7. Choisir les paires du vote

**Décision** : quand un votant demande une confrontation :

1. S'il a une confrontation **ouverte** (présentée, non votée) dans ce concours, elle lui est **rendue telle quelle**. Un rechargement de page ne tire donc pas une nouvelle paire, et ne permet pas de « passer » une paire qui déplaît. L'index unique partiel `(concours_id, votant_id) WHERE choix_id IS NULL` garantit qu'il n'en existe qu'une.
2. Sinon, on choisit **A** parmi les participations publiées qui ne sont pas les siennes, celle qui a été **le moins présentée** (`nombre_presentations`, au hasard entre égales), et qui a encore au moins un partenaire que ce votant n'a jamais vu avec elle.
3. On choisit **B** de la même façon parmi les partenaires de A non encore vus par ce votant avec A.
4. La paire est rangée canoniquement (`a_id < b_id`) ; l'unicité `(concours_id, votant_id, a_id, b_id)` interdit de la représenter (FR-036). `nombre_presentations` des deux est incrémenté **à la présentation**, pas au vote. Ainsi, deux votants simultanés ne reçoivent pas tous deux la même participation « la moins vue ».
5. L'ordre gauche/droite d'affichage est tiré au hasard à la présentation et stocké. Le serveur sert les deux participations avec leur seule photo et leur légende, **sans auteur**.

Deux requêtes indexées par concours, quelques centaines de participations au plus : pas de pré-calcul.

**Rationale** : FR-035, FR-036 et FR-037 sont tenus par construction. L'équilibre des présentations (SC-008 : au moins 30 présentations chacune pour 200 participations et 500 votants) vient de « la moins présentée d'abord ». Un tirage uniforme laisserait des participations sous-exposées, au hasard.

**Alternatives écartées** : un tirage purement aléatoire (écart de présentations important sur les petits volumes) ; un calendrier pré-calculé de paires par votant (il ne s'adapte ni aux retraits, ni aux suspensions, ni aux nouveaux votants).

---

## D8. Classer : la proportion de victoires, avec un seuil de présentations

**Décision** : à l'établissement des résultats, pour chaque participation encore publiée :
- `victoires` = confrontations **comptées** qu'elle a gagnées ; `duels` = confrontations comptées votées où elle figurait, **sans** celles qui impliquent une participation retirée ou suspendue ;
- `taux = victoires / duels` ;
- une participation qui a moins de `presentations_min` duels comptés (défaut 10, réglable par concours) est classée **après** toutes les autres, sans accès au podium (FR-043) ;
- tri par `taux` décroissant ; à égalité de taux (à 0,1 point près), départage par le jury s'il est prévu, sinon **ex aequo** (même rang, même récompense, FR-044).

Le tout est figé dans `jeu.resultat_concours` (rang, victoires, duels, taux, place du jury), et n'est plus jamais recalculé.

**Rationale** : la proportion de victoires est **explicable aux membres** (« votre photo a gagné 68 % de ses duels »), ce que la spec exige (FR-045). Le seuil neutralise le « 1 duel, 1 victoire, 100 % ».

**Alternatives écartées** :
- *Elo* : le résultat dépend de l'ordre des votes, donc deux dépouillements des mêmes voix diffèrent.
- *Bradley-Terry* : plus exact, mais itératif et opaque pour un membre.
- *Borne de Wilson* : plus robuste aux petits effectifs, mais un « score » qu'on ne sait pas expliquer simplement. Le seuil de présentations rend le même service, lisiblement.

---

## D9. Écarter les voix suspectes

**Décision** : chaque confrontation votée porte `comptee BOOLEAN` et `motif_ecart`, fixés **au moment du vote** :

| Règle | Motif | Exigence |
|---|---|---|
| vote moins de 1 000 ms après la présentation (horloge serveur) | `trop_rapide` | FR-040 |
| compte créé après `vote_debut` | `compte_recent` | FR-040 |
| adresse non vérifiée | `non_verifie` | FR-040 |

S'y ajoutent deux règles appliquées **au dépouillement**, sans réécrire les lignes : les voix des comptes suspendus à ce moment-là, et celles des confrontations impliquant une participation retirée ou suspendue, ne comptent pas (FR-055, FR-056).

**Signaux pour l'administrateur (FR-041)**, calculés à la lecture de l'écran de suivi, sans stockage :
- plus de 30 votes en une minute ;
- plus de 200 votes dans le concours ;
- préférence systématique : dans au moins 10 confrontations où une même participation figure, le votant la choisit à 90 % ou plus.

L'administrateur peut **écarter** les voix d'un votant (`comptee = false`, `motif_ecart = 'ecartee_admin'`), avec une ligne d'audit. L'opération est réversible tant que les résultats ne sont pas établis.

Le votant n'est **jamais informé** qu'une voix n'a pas compté. Il peut voter, et ses voix sont enregistrées. Le règlement publié annonce les règles (US9 scénario 1), mais l'écran ne dit pas au fraudeur ce qui l'a trahi.

**Rationale** : enregistrer plutôt que refuser garde les preuves pour l'administrateur et ne donne aucun signal exploitable. Fixer `comptee` au vote rend le dépouillement simple et rejouable.

---

## D10. Récompenses, réputation, distinctions

**Décision** :
- **Score de jeu** par `crediter` (le seul écrivain des tables de score), avec une nouvelle origine `concours` (CHECK `ck_gain_origine` élargi). Clés d'idempotence : `concours:{id}:participation:{participation_id}` et `concours:{id}:podium:{participation_id}`. Le pays figé et la saison au moment du versement s'appliquent comme pour tout gain (FR-047).
- Montants : `jeu.regles` gagne `prime_concours_participation` (défaut 2) et `prime_concours_podium SMALLINT[3]` (défaut {30, 20, 10}). Un concours peut surcharger ces montants (colonnes facultatives sur `jeu.concours`).
- **Réputation** par trois règles d'engagement **à 0 point**, sur le modèle de la 013 :

  | Règle | Réputation | Versée |
  |---|---|---|
  | `jeu_concours_participation` | 1 | une fois par participation publiée et non retirée |
  | `jeu_concours_podium` | 5 | une fois par place sur le podium |
  | `jeu_concours_vote` | 1 | une fois par concours, à partir de 10 voix comptées (plafond de fait : FR et Assumptions) |

  Elles s'ajoutent à `ACTIONS_INSTRUMENTEES`.
- **Distinctions** sur la condition `actions_comptees`, déjà employée par la 013 : `jeu_laureat` (1 podium), `jeu_jure_populaire` (votant dans 5 concours).
- Un concours **annulé** verse quand même la récompense de participation à ses participants publiés (US6, scénario 4).

**Rationale** : on n'introduit aucun circuit parallèle. Score, saison, pays figé, Championship et idempotence fonctionnent comme pour une partie gagnée. Les règles à 0 point ne touchent ni les points d'engagement ni le statut (SC-013).

---

## D11. Pas de J'aime sur les participations

**Décision** : les participations n'ont **aucune** réaction. La galerie d'un concours terminé est partageable (lien et Open Graph de la page du concours), comme les autres pages publiques.

**Rationale** : un J'aime crédite des points d'engagement à son destinataire (feature 008). En ajouter sur des photos de concours rouvrirait le circuit que H1 et H2 écartent, et afficherait un décompte pendant le vote, ce que FR-039 interdit.

---

## D12. Modération et signalement

**Décision** :
- Une file de modération dans le back-office, sur `jeu.participation.etat = 'en_attente'`. Accepter publie ; rejeter exige un motif (CHECK, sur le modèle de `ck_epreuve_rejet`). L'auteur est notifié dans les deux cas.
- `jeu.signalement_participation` : une ligne par membre et par participation (unicité), motif obligatoire. Au **11ᵉ** signalement distinct, la participation passe `suspendue` (seuil 10 et comparateur `>`, constante partagée avec `signalement_media`, FR-055). Seul un administrateur la rétablit, et le rétablissement remet le compteur à zéro, comme `changer_etat_media`.

**Rationale** : c'est le schéma qu'appliquent déjà les médias et les contributions Afripulse. On le reproduit plutôt que de bâtir un signalement générique, qui n'existe pas sur la plateforme.

---

## D13. Rattacher un concours à un module

**Décision** : `jeu.concours.rattachement` est un **code de module de la plateforme** (`afroculture`, `afripulse`, `codimoi`…), facultatif, contrôlé par un CHECK de forme (`^[a-z_]{3,30}$`), sans clé étrangère. La page de chaque module interroge `GET /api/jeu/concours?rattachement=<code>&en_cours=true`, et le panneau d'activités de la 013 (`JeuPanneauActivites`) l'affiche s'il y en a un.

**Rationale** : `jeu.module` ne liste que les modules **jouables en quiz** (quatre pilotes). Y ajouter Afroculture pour un concours l'afficherait comme module de quiz « bientôt disponible » sur le hub. Le rattachement d'un concours est une information d'affichage, pas une règle de jeu.

**Alternatives écartées** : une clé étrangère vers `jeu.module`, pour la raison ci-dessus ; un enum PostgreSQL, qu'il faudrait migrer à chaque nouveau module.

---

## D14. Pas de temps réel pour le vote

**Décision** : le vote est un échange question/réponse classique (présenter, voter, présenter). Aucun signal SSE ni DataPacket.

**Rationale** : rien ne se joue à deux en même temps. Le seul « événement » est la fin d'une phase, qui se résout à la lecture (D6) et se notifie par les notifications existantes.

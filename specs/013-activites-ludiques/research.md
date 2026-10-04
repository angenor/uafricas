# Research : Activités interactives, ludiques et participatives

**Feature** : `013-activites-ludiques` · **Date** : 2026-10-01 · **Spec** : [spec.md](./spec.md)

Quatorze décisions de conception. Chacune est ancrée dans le code tel qu'il est sur `main` au 2026-10-01 ; les chemins cités ont été lus, pas supposés.

---

## D1. Duel direct : flux SSE existant + état serveur autoritaire

**Décision** : le duel direct s'appuie sur le flux SSE de la messagerie (`services/messagerie_sse.rs`, `GET /api/messagerie/flux`). Le flux ne transporte que des **signaux** (`duel_*` : « l'état a changé, relis-le »). L'état du duel vit en base et se lit par `GET /api/jeu/duels/{id}/direct`, que le client rappelle à chaque signal **et** toutes les 3 secondes pendant un duel en cours.

**Rationale** :
- `RegistreSse` est indexé **par utilisateur**, ses événements sont du JSON libre discriminé par `type`, et six domaines s'en servent déjà sans toucher à la messagerie (`rendez_vous.rs`, `appels.rs`, `amitie.rs`, `annonces.rs`, `media_detention.rs`, `evenement_streaming.rs`). L'appel direct entre amis, qui a exactement la forme d'une invitation de duel, y passe déjà (`evt_appel_entrant`).
- Le flux **n'a pas de tampon** : un événement publié vers un utilisateur déconnecté est perdu, et le client met 3 s à se reconnecter. Un duel ne peut donc pas reposer sur la livraison des événements. D'où l'état autoritaire en base et le sondage de secours : un signal perdu coûte au plus 3 s, jamais le duel.
- Le sondage sert aussi de **battement de présence** (D6) : pas de mécanisme de présence à part.
- SC-010 (moins d'une seconde d'écart) ne dépend pas de la latence du signal : chaque manche porte un instant de début fixé par le serveur, renvoyé avec l'heure serveur ; les deux clients affichent le même compte à rebours.

**Alternatives écartées** :
- **LiveKit DataPackets** : il faudrait une room par duel, deux tokens, le SDK chargé côté client pour échanger quelques octets, et `send_data` avale ses erreurs (`livekit_moderation.rs:299` renvoie toujours `Ok(())`). Une room vide ne reçoit rien. Aucun gain sur le SSE, un coût de connexion en plus.
- **WebSocket dédié** : un troisième mécanisme temps réel, exclu par la consigne.
- **Sondage seul** : fonctionnerait, mais l'invitation à un duel direct doit surgir sans que le membre soit sur la page du duel, ce que seul le flux permet.

---

## D2. Score : un journal qui fait foi, deux agrégats pour lire vite

**Décision** : trois tables. `jeu.gain` est le journal append-only, une ligne par bonne réponse comptée et par prime, avec clé d'idempotence unique, saison et pays figés à l'écriture. `jeu.score_saison` (une ligne par saison × membre × pays) et `jeu.joueur` (une ligne par membre : total, série de jours) sont tenus dans la **même transaction** que l'insertion du gain.

**Rationale** :
- Le journal est exigé par FR-024 (origine, instant, pays figé) et FR-025 (jamais deux fois). Le patron est celui de `engagement.mouvement_points` : `cle_idempotence TEXT UNIQUE` + `INSERT … ON CONFLICT DO NOTHING`, et on ne touche aux agrégats que si `rows_affected() == 1`.
- SC-011 demande un classement en moins de 2 s à 10 000 membres. Dix gains par partie donnent vite des millions de lignes ; agréger le journal à chaque affichage ne tient pas. `score_saison` ramène le classement à une lecture de quelques dizaines de milliers de lignes.
- Le pays fait partie de la clé de `score_saison` : c'est ce qui rend FR-054 **structurel**. Un membre qui change de pays ouvre une seconde ligne ; l'ancienne ne bouge pas (SC-012).
- Les agrégats sont recalculables depuis le journal : c'est ce que fait l'annulation de gains (FR-031), sur le modèle de `POST /purger-phase-test` qui recalcule les soldes depuis le journal restant.

**Alternatives écartées** :
- **Agrégat seul, sans journal** : pas d'idempotence, pas d'annulation, pas de pays figé.
- **Vue matérialisée** : demande un rafraîchissement, donc une tâche de fond.

---

## D3. Réputation et distinctions : le barème d'engagement, à zéro point

**Décision** : six règles sont ajoutées à `engagement.regle_points` avec `points = 0` et un `reputation_delta` réglable : `jeu_defi_termine`, `jeu_duel_gagne`, `jeu_podium_saison`, `jeu_signalement_confirme`, et deux règles-jalons à réputation nulle, `jeu_premiere_partie` et `jeu_serie_7_jours`. Le jeu appelle `engagement::attribuer(...)` **après le COMMIT**. Cinq badges sont semés avec la condition existante `actions_comptees`.

**Rationale** :
- `engagement::appliquer` insère bien un mouvement à 0 point et y journalise `reputation_delta` ; les plafonds n'écrêtent que les points. FR-023 (aucun point d'engagement) et FR-070 (réputation séparée, hors statut) sont donc tenus **sans modifier le moteur** : le solde ne bouge pas, le niveau n'est pas recalculé à la hausse.
- `actions_comptees` compte les lignes du journal par `type_action`. Un badge « dix duels gagnés » est donc une ligne de seed : `parametre_action = 'jeu_duel_gagne'`, `seuil = 10`. `evaluer_badges` est déjà appelée après chaque mouvement et notifie.
- Les montants de réputation deviennent réglables depuis l'écran de barème qui existe (`/admin/engagement/regles`) : FR-069 sans écran neuf.
- `HistoriquePoints.vue` affiche déjà « réputation +N » sous un montant à 0. Une catégorie `jeux` est ajoutée à `engagement.categorie_points` pour que ces lignes soient filtrables.
- Le retrait de réputation pour triche (FR-072) passe par `engagement::ajuster(pool, uid, 0, -delta)`, qui existe.

**Points de vigilance** :
- `mouvement_points.cle_idempotence` est unique **globalement**. Les clés du jeu portent toujours le bénéficiaire : `jeu:duel:{duel_id}:{uid}`, `jeu:defi:{defi_id}:{uid}`.
- Les six `type_action` doivent entrer dans `ACTIONS_INSTRUMENTEES` (`handlers/admin/engagement.rs:35`), sinon le back-office les affiche « non instrumentée ».
- Une règle désactivée ne produit aucun mouvement : désactiver `jeu_duel_gagne` gèle aussi la progression du badge. C'est cohérent, mais à dire dans l'écran.

**Alternatives écartées** :
- **Une réputation propre au jeu** : deux compteurs de réputation sur un même profil, sans raison.
- **Un badge attribué par le code** : `attribuer_badge` est privée et il n'existe aucune fonction de service pour un badge manuel ; il faudrait en écrire une alors que `actions_comptees` suffit.

---

## D4. L'épreuve : propositions en tableau, bonne réponse par son rang

**Décision** : `jeu.epreuve` porte `propositions TEXT[]` et `bonne_reponse SMALLINT`, avec deux CHECK : `cardinality(propositions) BETWEEN 2 AND 6` et `bonne_reponse BETWEEN 1 AND cardinality(propositions)`. Le serveur sert les propositions **mélangées**, chacune avec son rang d'origine comme clé ; le client renvoie la clé.

**Rationale** :
- « Exactement une bonne réponse » (FR-006) est vrai par construction : il n'y a qu'une colonne pour la dire. Une table fille avec un booléen `correcte` demanderait un index unique partiel et laisserait possible « aucune bonne réponse ».
- Le rang d'origine ne révèle rien : le client ne sait pas lequel est le bon. Le mélange (FR-030) est une permutation à la sérialisation, sans rien stocker.
- Le vrai ou faux est une épreuve à deux propositions : aucune forme à part.

**Alternative écartée** : JSONB. Rien à y gagner, et les CHECK deviennent illisibles.

---

## D5. Dérivation : un catalogue de formes, une vue qui dit si la source tient encore

**Décision** :
1. Chaque **forme de question** est une fonction Rust dans `services/jeu_derivation.rs`, déclarée dans un catalogue `FORMES` (code, module, type de source, libellé). La dérivation est déclenchée par l'administrateur, forme par forme ou module entier.
2. L'unicité `(type_source, source_id, forme)` sur les épreuves dérivées, **rejetées comprises**, interdit de reproposer (FR-075).
3. Une vue `jeu.v_source` expose pour chaque contenu référençable `(type_source, source_id, visible, empreinte, lien)`. Une épreuve n'est servie que si sa source est `visible` et, pour une épreuve dérivée, si `empreinte` est encore celle enregistrée à la dérivation.

**Catalogue de départ** (colonnes vérifiées dans le schéma) :

| Module | Forme | Source | Question | Condition de publication de la source |
|---|---|---|---|---|
| Afripulse | `capitale` | `fiche_pays` | Quelle est la capitale de {pays} ? | `fp.bloquee = FALSE`, pays d'Afrique |
| Afripulse | `pays_de_capitale` | `fiche_pays` | {capitale} est la capitale de quel pays ? | idem |
| Afripulse | `monnaie` | `fiche_pays` | Quelle est la monnaie de {pays} ? | idem, `monnaie` non vide |
| Afripulse | `drapeau` | `fiche_pays` | À quel pays appartient ce drapeau ? (image) | idem, `image_drapeau_url` non vide |
| Afripulse | `devise_nationale` | `fiche_pays` | Quelle est la devise de {pays} ? | idem, `image_devise_url` non vide (la colonne contient le **texte** de la devise) |
| Afripulse | `pays_du_site` | `site_touristique` | Dans quel pays se trouve {nom} ? | `deleted_at IS NULL AND suspendu = FALSE` |
| Afripulse | `pays_de_recette` | `recette_culinaire` | De quel pays vient {titre} ? | idem |
| Afripulse | `pays_de_personnalite` | `personnalite_connue` | De quel pays est {nom_complet} ? | idem |
| Codimoi | `pays_du_proverbe` | `codimoi` | De quel pays vient ce proverbe ? | `etat = 'publie' AND deleted_at IS NULL`, `type = 'proverbe_adage'`, `pays_id` non nul |
| Codimoi | `auteur_de_citation` | `codimoi` | Qui a dit … ? | idem, `type = 'citation'`, `nom_auteur_originel` non vide |
| FactCheck | `vrai_ou_faux` | `factcheck` | {prejuge_titre} : vrai ou faux ? | `etat = 'publie' AND deleted_at IS NULL`, `verdict IN ('vrai','faux')`, `prejuge_titre` non vide |
| Afrolang | aucune | — | saisie manuelle uniquement | — |

**Second catalogue (2026-10-04)**, ajouté après la recette : les 5 formes de la fiche pays faisaient 270 des 288 épreuves Afripulse, et les parties se ressemblaient toutes. Distracteurs `Fournis` = calculés par la requête de la forme, source par source.

| Module | Forme | Source | Question | Mauvaises réponses |
|---|---|---|---|---|
| Afripulse | `pays_de_devise` | `fiche_pays` | Quel pays a pour devise « … » ? | pays dont la devise est CONNUE et DIFFÉRENTE (« Un peuple, un but, une foi » : Mali et Sénégal) |
| Afripulse | `indicatif_telephonique` | `fiche_pays` | {pays} : quel est son indicatif téléphonique ? | même colonne |
| Afripulse | `plus_peuple` | `fiche_pays` | Lequel de ces pays est le plus peuplé ? | 3 pays à moins de 80 % de sa population (une donnée approchée ne renverse pas la réponse) |
| Afripulse | `plus_vaste` | `fiche_pays` | Lequel de ces pays est le plus vaste ? | idem, superficie |
| Afripulse | `pays_de_langue_officielle` | `fiche_pays` | Lequel de ces pays a pour langue officielle : {1ʳᵉ langue} ? | pays dont la liste de langues officielles NE la contient PAS |
| Afripulse | `pays_du_peuple` | `groupe_ethnique` (nouvelle source) | Dans lequel de ces pays vivent notamment les {peuple} ? | pays qui ne déclarent pas ce peuple et ne le citent pas dans leurs langues |
| Afripulse | `domaine_de_personnalite` | `personnalite_connue` | {nom} : dans quel domaine cette personnalité s'est-elle illustrée ? | les autres domaines (`autre` exclu) |
| Codimoi | `sens_du_proverbe` | `codimoi` | Que veut dire ce proverbe ? | explications d'autres proverbes (bornées à 160 signes) |
| Codimoi | `mot_manquant` | `codimoi` | Complétez ce proverbe / cette citation | le plus long mot (≥ 5 lettres) des autres contenus |
| FactCheck | `affirmation_vraie` | `factcheck` (verdict vrai) | Laquelle de ces affirmations est vraie ? | idées reçues vérifiées fausses |
| FactCheck | `idee_recue` | `factcheck` (verdict faux) | Laquelle est une idée reçue ? | affirmations vérifiées vraies |

`langue_officielle`, écartée plus bas pour ambiguïté, devient utilisable en **inversant le sens** : on ne demande plus « la langue de {pays} » (le français est juste pour vingt pays), mais « lequel de ces pays », les autres étant choisis parmi ceux où elle n'est PAS officielle. Même principe pour la devise et le peuple. L'empreinte de `fiche_pays` couvre désormais indicatif, population, superficie et langue officielle, et celle de `codimoi` l'explication ; `37_jeu.sql` réaligne au passage les épreuves dont la source n'a pas bougé, sans quoi le changement de formule les aurait toutes rendues non servables.

Afrolang reçoit un lot de 44 épreuves saisies (`seeds/013_jeu_afrolang_epreuves.sql`) qui naissent `candidate` : rien n'est jouable avant revue.

**Rationale** :
- **Afrolang ne se dérive pas.** Il n'existe aucun référentiel de langues (`GET /afrolang/langues` fait un `SELECT DISTINCT langue_cible` sur les salles), aucun lexique, aucune ressource de type audio. Les quiz linguistiques et jeux audio demandés par le client sont de la saisie, avec un extrait sonore déposé par l'administrateur. C'est cohérent avec l'histoire 2 de la spec.
- **`updated_at` ne peut pas servir à détecter une modification** (FR-012). Les tables sources ont un trigger `updated_at` et des compteurs dénormalisés (`nombre_likes`, `nombre_signalements`) : un simple J'aime sur une fiche pays renverrait toutes ses épreuves en revue. L'empreinte est un `md5` des **seuls champs dont on tire des questions**.
- La vue résout FR-011 et FR-012 **à la lecture**, sans tâche de fond et sans requête par épreuve : le tirage d'une série fait une jointure.
- Les distracteurs sont tirés parmi les valeurs **distinctes** de la même colonne chez les autres sources. Une forme qui n'en trouve pas trois ne produit pas de candidate. Les questions à réponse ambiguë ne sont pas générées : pas de « quel pays utilise le franc CFA », pas de `langue_officielle` (le français est la bonne réponse pour vingt pays).
- Un objet Afripulse `suspendu` reste **visible** sur les pages publiques (aucun handler de liste ne filtre dessus), mais il est exclu du jeu : on ne tire pas de question d'un contenu signalé.

**Limite assumée** : l'empreinte est par source, pas par forme. Corriger la monnaie d'une fiche renvoie aussi en revue la question sur sa capitale. C'est le prix d'une vue simple ; le coût réel est une revue de plus pour l'administrateur.

**Tirage varié.** Un `ORDER BY random()` nu sur un vivier déséquilibré reproduisait le déséquilibre (quatre drapeaux dans une partie), et pouvait poser « capitale du Sénégal ? » puis « Dakar est la capitale de quel pays ? », la seconde donnant la réponse de la première. `serie_variee_sql` (services/jeu.rs), employée par les trois tirages (partie, défi, duel), ordonne : une épreuve par source avant toute deuxième (sans exclusion, pour que « jouer sur ce pays » remplisse une partie avec sa fiche), puis rotation des formes dans chaque module, puis des modules.

**Ajouter un module plus tard** (SC-014) : une branche dans la vue, une ou plusieurs fonctions dans `FORMES`, une ligne dans `jeu.module`. Aucune règle du jeu n'est touchée.

---

## D6. Tout ce qui est périodique se résout à la lecture

**Décision** : aucune tâche de fond. Quatre mécanismes paresseux :

| Sujet | Résolution |
|---|---|
| **Défi du jour, de la semaine** | `INSERT INTO jeu.defi … ON CONFLICT (periodicite, periode_debut) DO NOTHING` puis `SELECT`. Le premier lecteur le crée, tous lisent la même ligne (FR-035). |
| **Échéances d'un duel** | `resoudre_duel(tx, duel_id)` est appelée en tête de toute lecture ou écriture d'un duel, sous `SELECT … FOR UPDATE`. Elle applique expiration, forfait et clôture, puis écrit les gains. Idempotente par les clés de gain et par `UPDATE … WHERE etat = <attendu>`. |
| **Avancement d'un duel direct** | même fonction : tant que la manche courante est close (deux réponses, ou temps écoulé), elle ouvre la suivante. Un joueur seul devant son écran fait donc avancer le duel par son sondage. |
| **Fin de saison** | l'état d'une saison se **déduit** de ses dates. Les distinctions de podium sont attribuées une fois : `UPDATE jeu.saison SET cloturee_at = NOW() WHERE id = $1 AND cloturee_at IS NULL AND fin_at <= NOW()` ; si une ligne est touchée, on attribue. |

**Rationale** : c'est le mécanisme de toute la plateforme (rotation des épisodes, créneau courant, reset mensuel des points, expiration des africanités). Le patron de verrou est celui de `media_programmation.rs` (`FOR UPDATE` sur la ligne parente avant de décider).

**Conséquence à connaître** : un duel que personne ne consulte reste dans son dernier état stocké. La liste « mes duels » résout donc chaque duel non terminé qu'elle affiche ; les classements, eux, ne lisent que des gains déjà écrits. Un forfait non consulté ne rapporte sa prime que lorsque quelqu'un ouvre le duel ou la liste. C'est acceptable : le vainqueur est le premier intéressé à regarder.

---

## D7. Pays de rattachement

**Décision** : à l'écriture d'un gain, `pays_id = COALESCE(u.pays_origine_id, u.pays_residence_id)`, lu dans la même transaction. Le classement des pays et la carte ne portent que les 55 pays d'Afrique ; un membre rattaché à un autre pays figure au classement de tous les membres et au classement de son pays, mais ce pays n'apparaît ni sur la carte ni au classement des pays.

**Rationale** : c'est la lettre de la décision Q1 du client. Les deux colonnes existent, sont facultatives et indexées (`04_iam.sql`).

**Périmètre africain** : la liste figée `PAYS_AFRICAINS_ISO2` (`constants/afripulse_pays_autorises.rs`, miroir frontend `constants/afripulsePaysAutorises.ts`) plutôt que `continent = 'Afrique'`. La colonne `continent` a `'Afrique'` pour valeur par défaut : tout pays inséré sans continent devient africain. La liste est aussi celle que la carte utilise déjà pour filtrer la géométrie.

**Point laissé au client** : un membre dont le pays d'origine n'est pas africain et qui réside en Afrique est classé sous son pays d'origine, donc hors carte. Appliquer le repli sur la résidence dans ce cas serait défendable ; ce n'est pas ce que dit Q1. Signalé, non tranché.

---

## D8. Intégrité : le serveur tient la montre

**Décision** :
- L'épreuve est délivrée par `POST /parties/{id}/suivante`, qui écrit `presentee_at = NOW()`. La réponse est acceptée si elle arrive avant `presentee_at + temps + 2 s` (tolérance réseau fixe).
- La bonne réponse et l'explication ne sont renvoyées que par `POST …/repondre`, ou par `…/suivante` pour une épreuve dont le temps est passé.
- Demander la suivante alors que la courante n'a pas de réponse **enregistre une absence de réponse** : quitter ou recharger ne permet pas de revoir une question.
- Une épreuve à média bénéficie de 5 s de chargement en plus. Si le média ne se charge pas, le client appelle `POST …/injouable` : l'épreuve est marquée vue, ne rapporte rien et **n'est pas comptée comme mauvaise** dans le bilan.

**Rationale** : FR-027 à FR-029. Le cas limite de la spec (« le temps ne commence qu'une fois l'épreuve réellement présentée ») ne peut pas être tenu à la lettre sans rendre la main au client, qui déciderait alors quand le chronomètre démarre. La marge fixe et la sortie « injouable » en donnent l'effet sans ouvrir cette porte : déclarer une épreuve injouable ne fait jamais gagner de score, et l'épreuve est consommée.

**Limite assumée** : rien n'empêche un membre de chercher la réponse ailleurs en trente secondes. Aucune mesure technique raisonnable ne l'empêche ; le temps limité est la seule parade, et c'est celle du genre.

---

## D9. La participation à un défi est une partie

**Décision** : pas de table de participation. Une participation est une ligne de `jeu.partie` avec `cadre = 'defi'` et `defi_id`, protégée par un index unique partiel `(utilisateur_id, defi_id)`. De même, chaque joueur d'un duel a une partie `cadre = 'duel'`, unique par `(duel_id, utilisateur_id)`.

**Rationale** : le déroulé d'un défi ou d'un duel différé **est** celui d'une partie (présenter, répondre, corriger). Les mêmes routes et le même écran servent les trois cadres ; seuls changent la composition de la série et ce que le résultat rapporte. « Une participation par défi » (FR-034) devient une contrainte d'unicité. La spec comptait onze entités ; le modèle en porte dix tables, l'entité « participation » étant une partie.

---

## D10. Carte : un seul composant, généralisé

**Décision** : `components/retrouve-amis/CarteAfrique.vue` est déplacé en `components/common/CarteAfriqueValeurs.vue` et gagne trois props facultatives dont les défauts reproduisent le comportement actuel : `couleur?: (valeur: number) => string`, `libelleBulle?: (valeur: number) => string`, `cliquableAZero?: boolean`. Africonnect est mis à jour sur son unique point d'appel.

**Rationale** : son contrat est déjà le bon (`comptes: Record<iso2, number>` en entrée, `select(iso)` en sortie). Trois choses y sont en dur : l'échelle (`couleurChaleurAvis`), le texte de la bulle (« avis ») et le clic bloqué à zéro, que FR-067 interdit. Les deux cartes existantes sont déjà des copies l'une de l'autre ; en créer une troisième aggraverait la dette. La carte d'Afripulse, couplée au type `FichePaysAPI` et colorée par région, n'est pas touchée.

**Écran étroit** (FR-068) : aucune des deux cartes n'a de gestion tactile du survol. La page Championship offre une bascule Classement / Carte (`AfricansBascule`) ; la liste est la vue par défaut sous 1024 px.

---

## D11. Garde du joueur et jeton expiré

**Décision** :
- Backend : un helper `garde_joueur(pool, req) -> Result<Uuid, ApiErreur>` dans `handlers/jeu.rs` lit le JWT puis **vérifie en base** `etat = 'actif' AND deleted_at IS NULL`. Toutes les routes membres du jeu l'appellent.
- Frontend : `useJeu` enveloppe ses appels authentifiés dans un `appelAuth` local qui, sur 401, appelle une fois `useAuth().refreshAccessToken()` puis rejoue la requête.

**Rationale** :
- Il n'existe aucun extracteur membre (`AuthUtilisateur` n'existe pas) ni garde commun : le JWT ne porte que `sub`, et un jeton émis avant une suspension reste accepté 15 minutes. FR-002 exige donc une lecture en base. Le patron existe en privé dans `afrolang_ressources.rs:73`.
- Il n'existe aucun helper de rejeu sur 401 côté public. L'access token vit 15 minutes en mémoire ; un membre qui laisse l'onglet ouvert puis lance une partie essuierait un 401 muet. Le jeton se lit par `useUserStore().accessToken`, jamais dans `localStorage` (défaut déjà corrigé une fois sur les composables médias).

**Amitié et blocage** : `verifier_relation` (ami et non bloqué) est privée et dupliquée dans `appels.rs` et `rendez_vous.rs`. Le duel en reprend une copie locale, conformément à l'usage du dépôt et au principe V.

---

## D12. Schéma, migration, habilitation

**Décision** :
- Nouveau schéma **`jeu`**, déclaré en tête de `37_jeu.sql` (`CREATE SCHEMA IF NOT EXISTS`), comme `social` (29) et `engagement` (35). `01_schemas.sql` n'est pas touché.
- Inclusion dans `schema.sql` en **phase 5, après le bloc `35*`** : la migration insère une permission et des règles d'engagement, elle a besoin de `15_seed.sql` et de `35c`.
- Permission `jeu.gerer` (`type_ressource = 'jeu'`, `action = 'gerer'`), rattachée **explicitement** aux rôles `super_admin` et `admin`.
- États et cadres en `VARCHAR` + `CHECK`, pas en enums.
- `updated_at` tenu par le code.

**Rationale** :
- Un schéma dédié est justifié au sens de la constitution : dix tables, aucune n'appartient à un domaine existant, et le jeu référence quatre schémas sans dépendre d'aucun.
- `04h_iam_permissions_backoffice.sql` donne au rôle `admin` les permissions existant **au moment où il s'exécute**. Une permission créée après lui n'y va pas seule.
- `CHECK` plutôt qu'enum : c'est le choix argumenté de `36_social_africanite.sql` (un enum se modifie mal) et il simplifie sqlx, qui lit sinon les enums en `::text`.
- `14_triggers.sql` ne boucle que sur onze schémas nommés ; `29`, `35*` et `36` n'ont aucun trigger.

**Une seule migration** : rien n'est supprimé, donc pas de jeu en deux temps au déploiement.

---

## D13. Heure de référence : le temps universel

**Décision** : le jour d'un défi est `(NOW() AT TIME ZONE 'UTC')::date` ; la semaine commence le lundi UTC (`date_trunc('week', …)`).

**Rationale** : la plateforme sert des membres de Dakar (UTC+0) à Maurice (UTC+4) et la diaspora au-delà. Un jour par fuseau donnerait la série du lendemain à certains avant les autres, ce qui casse SC-007. L'écran affiche « se termine dans 5 h 12 », jamais une heure d'horloge.

---

## D14. Le contenu disponible, constat et risque

**Constat** (compté dans les fichiers de seed, aucune base interrogée) :

| Source | Inclus dans `schema.sql` | Seeds locaux |
|---|---|---|
| Fiches pays | 55, avec capitale, monnaie, langue officielle, drapeau, devise | — |
| Sites, recettes, personnalités | 0 | 9, 4, 8 |
| Codimoi | 0 | 4, dont un seul proverbe, **sans pays** |
| FactCheck | 0 | 2 |
| Afrolang | 10 salles « diaspora » | 0 ressource |

**Conséquences** :
- En local, **seul Afripulse** produit des candidates en volume : cinq formes sur 55 fiches, soit environ 250 épreuves. Le recettage de la dérivation se fait donc sur Afripulse ; Codimoi et FactCheck demandent un seed de démonstration.
- **SC-002 (100 épreuves jouables par pilote) dépend du contenu de production**, que cette recherche n'a pas mesuré. À vérifier avant l'ouverture avec `deploy.sh psql` (lecture seule), par exemple : nombre de proverbes publiés portant un pays, nombre de citations portant un auteur, nombre de factchecks publiés à verdict `vrai` ou `faux`.
- Le proverbe sans `pays_id` est un cas courant, pas une exception : le formulaire de publication ne l'exige pas. La forme `pays_du_proverbe` ne produira que ce que les membres ont renseigné.
- Pour Afrolang, les 100 épreuves sont entièrement de la saisie.

**Décision** : un seed local idempotent `seeds-locaux/95_seed_local_jeu.sql` apporte de quoi recetter (proverbes avec pays, citations avec auteur, factchecks tranchés, une saison, quelques épreuves Afrolang saisies). Il reste hors de `schemas/`, donc jamais déployé. Le volume de production est un sujet d'exploitation, consigné dans le quickstart comme critère d'ouverture et non comme tâche de développement.

# Feature Specification: Recadrage de l'engagement, 3 sources de points, 4 statuts, cadeaux virtuels

**Feature Branch**: `008-recadrage-engagement-cadeaux` (répertoire de spec ; aucune branche créée automatiquement)

**Created**: 2026-08-08

**Status**: Draft

**Input**: User description: "nous avons déjà implémenté la fonctionalité 'engagement-points-badges' mais il n'a pas encore été utilisé et nous voulons en profiter pour recadrer les fonctionnalités selon le contenu de tmp.md"

## Contexte et cadrage

Le système d'engagement a été livré en deux itérations (`specs/001-engagement-gamification/` puis `specs/007-engagement-points-badges/`) : compte d'engagement par membre, journal immuable et idempotent des mouvements, barème entièrement paramétrable en back-office, catégories, niveaux, badges, espace membre « Mon engagement ». **Ce socle n'a jamais été mis en service** : aucun membre n'a encore de points, aucun historique réel n'existe.

Cette fenêtre est mise à profit pour **recadrer le produit sur la doctrine d'engagement retenue par la direction du projet Africans** : valoriser la **valeur produite pour les autres** (contenus aimés, contenus relayés) et le **soutien volontaire de la communauté** (cadeaux virtuels), plutôt que le volume d'actions accomplies par le membre lui-même.

**Ce que ce recadrage change par rapport à l'existant :**

| Sujet | Aujourd'hui (livré, non mis en service) | Après recadrage |
|--------|------------------------------------------|-----------------|
| Sources de points | ~10 actions récompensées (contribution validée, fact-check, proposition média, mise à la une, animation acceptée, bonus « 5 réseaux »…) | **3 sources canoniques** : j'aime reçus, partages de ses contenus par autrui, cadeaux virtuels reçus |
| Popularité | Paliers de « j'aime » (100 → 10 pts, 500 → 30 pts, 1 000 → 50 pts) | **Crédit unitaire** : 1 j'aime reçu = 1 point |
| Partages | Bonus au **partageur** après 5 réseaux distincts | Crédit à l'**auteur du contenu**, une fois par partageur (interne et externe) |
| Statuts | 3 niveaux (Membre 0, Membre Premium 200, Influenceur Platinum 1 000) | **4 statuts** : Membre Africans, Premium, Gold, Platinum (0 / 500 / 2 000 / 10 000) |
| Argent | « Aucune conversion argent → points, en aucun cas » (décision 007) | **Décision renversée** : les cadeaux virtuels sont achetés et convertis en points |
| Cadeaux entre membres | Explicitement hors périmètre (décision 007) | **Décision renversée** : cœur de cette itération |

**Explicitement hors périmètre de cette itération :**

- **Encaissement réel** : aucune intégration de prestataire de paiement. L'agrégateur **CinetPay** est prévu ultérieurement ; d'ici là le paiement est **simulé** de bout en bout, sans mouvement d'argent réel.
- **Retrait (payout) de la cagnotte** vers le bénéficiaire : la part de 90 % est **journalisée et consultable**, mais son versement effectif dépend de l'intégration de paiement et sera spécifié avec elle.
- **Impact algorithmique des statuts** sur le classement des fils et les slots « à la une » : maintenu hors périmètre, les statuts restent des distinctions **symboliques et visuelles**, sans avantage de visibilité. C'est la contrepartie qui rend acceptable l'achat de points.
- **Classements publics** (global, par territoire) : toujours hors périmètre.
- **Cadeaux partenaires / catalogue de récompenses dépensables** : les points reçus ne se dépensent pas ; ils ne servent qu'à la progression de statut.
- **Suppression du moteur existant** : le barème reste paramétrable et les règles écartées sont **désactivées, pas supprimées**.

## Clarifications

### Session 2026-08-08

- Q: Sur quelles familles de contenus un « j'aime » reçu doit-il rapporter un point à l'auteur ? → A: Toutes les familles dotées d'un « j'aime » binaire (Codi-moi, contributions de gouvernance, bibliothèques humaines, médias télé/radio, Vidafrica, éléments et fiches Opportunité-Afrique) ; les notations 1–5 et les réactions emoji sont exclues. *(Précision apportée en phase de conception : voir `research.md` R2 : le fact-check n'ayant qu'un système de réaction emoji, sa réaction **cœur** y tient lieu de « j'aime » et crédite ; les trois autres emojis ne créditent pas.)*
- Q: Depuis quels supports un membre doit-il pouvoir offrir un cadeau virtuel ? → A: Les mêmes familles de contenus que le j'aime, **plus** le profil public d'un membre (soutien direct, sans contenu support).
- Q: Sur un support co-détenu (chaîne TV / station radio), qui reçoit les points et l'argent ? → A: Le **détenteur propriétaire** seul ; les co-détenteurs/animateurs ne reçoivent rien au titre du support.
- Q: Comment neutraliser le minage de points permis par le paiement simulé ? → A: Cadeaux **ouverts en production** avec un bandeau « paiement simulé, phase de test » ; les points **et** les cagnottes issus des cadeaux simulés sont **purgés** à la mise en service de l'encaissement réel.
- Q: Quels gestes de partage créditent l'auteur, et à quelle granularité le crédit est-il unique ? → A: **Un seul point par contenu et par partageur**, tous canaux confondus (internes comme externes) : le canal est tracé pour la statistique, mais ne démultiplie pas le crédit.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Recadrer le barème et les statuts depuis le back-office (Priority: P1)

Un administrateur habilité ouvre le module d'engagement et met le nouveau barème en service **sans livraison technique** : il **désactive** les règles écartées (contribution validée, contribution mise en avant, fact-check correct, fact-check abusif, proposition média validée, mise à la une, animation acceptée, bonus « 5 réseaux »), **active** les trois règles canoniques (j'aime reçu, partage reçu, cadeau reçu) avec leurs montants, et **redéfinit la grille des statuts** en quatre paliers : Membre Africans (0), Premium (500), Gold (2 000), Platinum (10 000). Les règles désactivées restent visibles et **réactivables** à tout moment ; leurs libellés et montants sont conservés.

**Why this priority**: C'est le recadrage lui-même, et le préalable de toutes les autres stories : sans grille de statuts et sans règles actives, ni les j'aime, ni les partages, ni les cadeaux ne peuvent créditer quoi que ce soit. Entièrement testable sur le back-office déjà livré.

**Independent Test**: Ouvrir le module d'engagement, constater que seules les trois règles canoniques sont actives, que les huit règles écartées sont présentes à l'état inactif, et que la grille affiche exactement les quatre statuts aux seuils 0 / 500 / 2 000 / 10 000 ; déclencher une action écartée (ex. validation d'une proposition média) et vérifier qu'aucun point n'est attribué et que l'action métier réussit normalement.

**Acceptance Scenarios**:

1. **Given** la plateforme fraîchement mise à jour, **When** un administrateur ouvre la grille des statuts, **Then** il voit quatre statuts, Membre Africans (0 à 499), Premium (500 à 1 999), Gold (2 000 à 9 999), Platinum (10 000 et plus), dans cet ordre et sans seuil dupliqué.
2. **Given** le nouveau barème en service, **When** un administrateur consulte la liste des règles, **Then** les trois règles canoniques (j'aime reçu, partage reçu, cadeau reçu) sont actives et les huit règles écartées apparaissent explicitement **inactives**, avec leur montant d'origine conservé.
3. **Given** une règle écartée et inactive, **When** l'action correspondante se produit dans son parcours métier réel, **Then** aucun point n'est attribué, aucune erreur n'est visible par l'utilisateur et l'action métier aboutit normalement.
4. **Given** un administrateur, **When** il réactive une règle écartée et fixe un montant, **Then** l'action correspondante crédite à nouveau, immédiatement, sans redéploiement.
5. **Given** un administrateur, **When** il modifie le montant du j'aime (par exemple 1 → 2 points), **Then** les j'aime suivants créditent 2 points et les mouvements déjà journalisés restent inchangés.
6. **Given** un administrateur, **When** il tente de définir deux statuts au même seuil ou un seuil qui contredit l'ordre des statuts, **Then** l'enregistrement est refusé avec un message explicite.
7. **Given** un utilisateur sans la permission de gestion de l'engagement, **When** il tente d'accéder au module ou de modifier le barème, **Then** l'accès est refusé avec un message nommant la permission requise.

---

### User Story 2 - Gagner des points grâce aux j'aime reçus sur ses contenus (Priority: P1)

Quand un membre reçoit un « j'aime » sur un contenu dont il est l'auteur ou le détenteur, il est crédité du montant paramétré, **1 point par j'aime** à la mise en service. Le crédit est **unitaire et définitif** : retirer puis remettre son j'aime ne crédite pas une seconde fois, et retirer son j'aime ne reprend pas le point déjà attribué. Le membre qui aime son propre contenu ne crédite personne.

**Why this priority**: C'est la source de points la plus volumineuse et la plus immédiate : elle rend le système vivant dès le premier jour, sur les réactions déjà existantes de la plateforme (contributions, vidéos Vidafrica, médias télé/radio, publications du mur, profils).

**Independent Test**: Avec deux comptes de test, aimer un contenu du premier depuis le second et vérifier le crédit d'exactement 1 point à l'auteur, visible dans son espace « Mon engagement » avec l'action en langage clair ; retirer puis remettre le j'aime et vérifier qu'aucun point supplémentaire n'est attribué et qu'aucun point n'est repris.

**Acceptance Scenarios**:

1. **Given** un membre auteur d'un contenu et un autre membre, **When** ce dernier aime le contenu, **Then** l'auteur est crédité de 1 point, dans la catégorie « Popularité », et le mouvement apparaît dans son historique.
2. **Given** un membre ayant aimé un contenu, **When** il retire son j'aime puis le remet, **Then** aucun nouveau point n'est crédité et le point initial n'est pas repris.
3. **Given** un membre, **When** il aime son propre contenu, **Then** aucun point n'est attribué.
4. **Given** un contenu ayant reçu 350 j'aime de membres distincts, **When** son auteur consulte son engagement, **Then** il totalise 350 points de popularité pour ce contenu, sans aucun effet de palier.
5. **Given** la règle « j'aime reçu » désactivée en back-office, **When** un membre aime un contenu, **Then** aucun point n'est attribué et la réaction fonctionne normalement.
6. **Given** un membre ayant atteint le plafond journalier paramétré pour les j'aime reçus, **When** il reçoit un j'aime supplémentaire le même jour, **Then** le point n'est pas crédité et l'écrêtage est explicitement visible dans son historique.

---

### User Story 3 - Offrir et recevoir un cadeau virtuel (Priority: P1)

Depuis un contenu qu'il apprécie (vidéo Vidafrica, programme télé ou radio, fiche de bibliothèque humaine, publication Codi-moi…) ou depuis le profil d'un membre, un membre choisit un **cadeau virtuel** dans un catalogue de cinq objets, Drapeau de l'Union Africaine (20 points), Badge (10), Chapeau (5), Fleur (3), Épingle de costume (1), et choisit son **mode d'envoi** :

- **Soutien financier** : 90 % du montant revient au bénéficiaire, 10 % restent à Africans au titre des frais de plateforme. Le bénéficiaire reçoit **aussi** les points du cadeau.
- **Cadeau en points** : le bénéficiaire reçoit **uniquement** les points ; Africans conserve 100 % du montant.

Le paiement est **simulé** tant que l'agrégateur n'est pas branché : le membre traverse un parcours de paiement factice qu'il peut voir réussir ou échouer, et rien n'est réellement débité. Le cadeau n'est envoyé, les points ne sont crédités et la répartition n'est journalisée **que si le paiement simulé aboutit**. Le cadeau est visible sur le contenu et dans l'historique des deux membres.

**Why this priority**: C'est la principale nouveauté fonctionnelle du recadrage et la seule source de points reposant sur un engagement délibéré et coûteux de la communauté. Elle est indépendante des j'aime et des partages.

**Independent Test**: Depuis un contenu, offrir un « Drapeau de l'Union Africaine » en mode soutien financier avec un paiement simulé réussi, et vérifier : +20 points au bénéficiaire, ventilation 90 / 10 journalisée, cadeau visible sur le contenu ; puis rejouer avec un paiement simulé en échec et vérifier qu'aucun point n'est crédité et qu'aucune répartition n'est enregistrée.

**Acceptance Scenarios**:

1. **Given** un membre connecté sur un contenu d'un autre membre, **When** il offre une « Fleur » en mode **soutien financier** et que le paiement simulé aboutit, **Then** le bénéficiaire est crédité de 3 points, 90 % du montant lui sont journalisés en cagnotte, 10 % sont journalisés comme frais de plateforme, et les deux membres voient l'opération dans leur historique.
2. **Given** le même membre, **When** il offre une « Fleur » en mode **cadeau en points** et que le paiement simulé aboutit, **Then** le bénéficiaire est crédité de 3 points, **aucun** montant ne lui est journalisé en cagnotte, et 100 % du montant est journalisé au bénéfice de la plateforme.
3. **Given** un membre en cours d'envoi, **When** le paiement simulé échoue ou est abandonné, **Then** aucun point n'est crédité, aucune répartition n'est enregistrée, le cadeau n'apparaît nulle part et le membre peut réessayer.
4. **Given** un membre, **When** il tente de s'offrir un cadeau à lui-même ou d'offrir un cadeau sur son propre contenu, **Then** l'opération est refusée avec un message explicite et aucun point n'est attribué.
5. **Given** un contenu ayant reçu plusieurs cadeaux, **When** un visiteur le consulte, **Then** il voit les cadeaux reçus et leurs offreurs, **sans** aucun montant en argent.
6. **Given** un même parcours de paiement simulé rejoué (rechargement, double validation, retour arrière), **When** il aboutit une seconde fois, **Then** le cadeau n'est envoyé et les points ne sont crédités qu'**une seule fois**.
7. **Given** un membre bénéficiaire, **When** il consulte son espace, **Then** il voit le détail des cadeaux reçus (objet, offreur, date, points) et sa **cagnotte de soutien** cumulée, avec la mention explicite que le versement n'est pas encore disponible.
8. **Given** la règle « cadeau reçu » désactivée en back-office, **When** un cadeau est offert, **Then** aucun point n'est crédité, tandis que la transaction et sa répartition restent journalisées.

---

### User Story 4 - Gagner des points quand les autres partagent mes contenus (Priority: P2)

Quand un membre partage le contenu d'un autre, repartage sur le mur de la plateforme **ou** partage vers un réseau social externe (WhatsApp, Facebook, Telegram, e-mail…) : c'est **l'auteur du contenu** qui est crédité du montant paramétré. Un même partageur ne crédite l'auteur qu'**une seule fois par contenu**, quel que soit le nombre de canaux utilisés : le canal est enregistré pour la statistique, mais ne démultiplie pas le crédit. Partager son propre contenu ne rapporte rien : le recadrage inverse la règle précédente, où le partageur était le bénéficiaire.

**Why this priority**: Levier de croissance explicitement retenu, mais le moins vérifiable (le système constate une intention de partage externe, pas sa publication effective) et le plus exposé à l'abus. Livré après les deux sources sûres, avec un anti-abus strict.

**Independent Test**: Depuis un second compte, partager le contenu d'un membre vers un premier canal et vérifier un crédit unique à l'auteur ; partager le même contenu vers deux autres canaux et vérifier qu'aucun point supplémentaire n'est attribué ; partager son propre contenu et vérifier l'absence totale de crédit.

**Acceptance Scenarios**:

1. **Given** un contenu d'un membre A, **When** un membre B le partage vers WhatsApp, **Then** A est crédité du montant paramétré, dans la catégorie « Partages », et le mouvement nomme le contenu concerné.
2. **Given** le membre B ayant déjà partagé ce contenu vers WhatsApp, **When** il le repartage vers WhatsApp, **Then** aucun point supplémentaire n'est attribué à A.
3. **Given** le membre B ayant déjà partagé ce contenu vers WhatsApp, **When** il le partage vers Facebook puis sur le mur de la plateforme, **Then** aucun point supplémentaire n'est attribué à A, et les deux partages sont néanmoins enregistrés avec leur canal.
4. **Given** un membre A, **When** il partage son propre contenu, **Then** aucun point n'est attribué, ni à lui ni à quiconque.
5. **Given** un membre C n'ayant jamais partagé ce contenu, **When** il le repartage sur le mur de la plateforme, **Then** A est crédité une fois de plus (partageur distinct).
6. **Given** un auteur ayant atteint le plafond journalier paramétré pour les partages reçus, **When** un nouveau partage survient le même jour, **Then** aucun point n'est crédité au-delà du plafond et l'écrêtage est visible dans son historique.
7. **Given** un membre déclenchant un partage, **When** l'enregistrement du partage ou l'attribution des points échoue, **Then** le partage reste utilisable et le parcours n'est pas bloqué.

---

### User Story 5 - Administrer le catalogue de cadeaux et suivre les recettes (Priority: P2)

Un administrateur habilité gère le **catalogue des cadeaux** (nom, apparence, prix, points attribués, ordre d'affichage, actif/inactif) et consulte le **journal des transactions de cadeaux** : offreur, bénéficiaire, contenu concerné, objet offert, mode d'envoi, montant, part bénéficiaire, part plateforme, état du paiement simulé, date. Il dispose de totaux (recettes plateforme, cagnottes dues aux bénéficiaires) et peut filtrer par période, par membre et par état.

**Why this priority**: Sans catalogue paramétrable, chaque ajustement de prix ou de barème exigerait une livraison technique : ce que le produit refuse. Sans journal, la répartition 90 / 10 n'est ni vérifiable ni exploitable au moment de brancher le paiement réel. La story dépend cependant d'un flux de cadeaux existant (US3).

**Independent Test**: Créer un sixième cadeau, le rendre actif, vérifier qu'il apparaît immédiatement côté membre ; le désactiver et vérifier qu'il disparaît du catalogue sans altérer les cadeaux déjà offerts ; réaliser deux envois et vérifier que le journal et les totaux les reflètent exactement.

**Acceptance Scenarios**:

1. **Given** un administrateur, **When** il crée un cadeau doté d'un prix et d'un nombre de points, **Then** il apparaît immédiatement dans le catalogue proposé aux membres, sans redéploiement.
2. **Given** un cadeau déjà offert par le passé, **When** l'administrateur le désactive, **Then** il disparaît du catalogue proposé mais reste affiché et lisible dans les historiques existants.
3. **Given** un administrateur, **When** il modifie le nombre de points d'un cadeau, **Then** les envois suivants créditent le nouveau montant et les mouvements déjà journalisés ne sont pas recalculés.
4. **Given** deux envois aboutis (un par mode), **When** l'administrateur ouvre le journal des transactions, **Then** il retrouve les deux lignes avec leur répartition exacte et des totaux cohérents avec la somme des lignes.
5. **Given** un administrateur, **When** il modifie le catalogue ou le taux de commission, **Then** l'opération est tracée dans la piste d'audit avec son auteur.
6. **Given** un administrateur, **When** il tente de supprimer définitivement un cadeau déjà offert, **Then** l'opération est refusée et seule la désactivation est possible.

---

### Edge Cases

- **J'aime retiré puis remis** par le même membre sur le même contenu : un seul point, jamais repris (le compteur affiché sur le contenu, lui, suit les retraits, l'écart entre compteur de j'aime et points gagnés est normal et assumé).
- **Auteur d'un contenu qui change** (transfert de propriété d'un support média) : les points déjà gagnés restent acquis à l'ancien propriétaire ; les j'aime, partages et cadeaux suivants créditent le nouveau.
- **Support co-détenu** : seul le propriétaire est crédité (FR-008a) ; l'arrivée ou le départ d'un co-détenteur ne modifie ni les points passés, ni la cagnotte.
- **Support sans propriétaire déclaré** (support créé par l'administration avant l'existence des détenteurs) : aucun point n'est attribué et le cadeau est refusé avec un message explicite, plutôt que crédité à un bénéficiaire arbitraire.
- **Contenu supprimé ou dépublié** après avoir rapporté des points : les points restent acquis ; l'historique continue de nommer le contenu même s'il n'est plus consultable.
- **Contenu sans auteur identifiable** (contenu institutionnel publié par l'administration, site touristique ou secteur de développement d'une fiche pays) : aucun point n'est attribué, aucune erreur, et le bouton d'envoi de cadeau n'est pas proposé.
- **Paiement simulé abandonné** (fermeture d'onglet, expiration) : la transaction reste à l'état « en attente » puis expire ; ni cadeau, ni points, ni répartition.
- **Double soumission du paiement simulé** : une seule transaction aboutie, un seul crédit.
- **Cadeau modifié entre le choix et le paiement** (prix ou points changés en back-office) : les valeurs **figées au moment de l'envoi** font foi et sont conservées sur la transaction.
- **Bénéficiaire supprimé** entre l'envoi et l'aboutissement du paiement : la transaction est refusée ou annulée ; aucune cagnotte orpheline.
- **Membre supprimé** : compte d'engagement, mouvements, cadeaux reçus, cagnotte **et transactions** disparaissent avec lui, sans données orphelines. Aucune trace nominative ni anonymisée n'est conservée : la recette de la plateforme reste connue par les états agrégés exportés avant la suppression, et bâtir une archive comptable dédiée serait prématuré tant que l'encaissement n'est pas réel.
- **Statuts modifiés** (ajout, retrait, déplacement d'un seuil) : tous les membres concernés basculent sur le statut correspondant à leur solde, sans opération manuelle et sans perte de points.
- **Solde qui redescend sous un seuil** (ajustement administratif motivé) : le statut redescend ; les badges de succès déjà obtenus restent acquis.
- **Règle canonique désactivée** : l'action métier (j'aime, partage, cadeau) continue de fonctionner ; seuls les points cessent.
- **Panne du moteur de points** : aucun j'aime, partage ou envoi de cadeau n'échoue à cause de l'attribution de points.
- **Anciens mouvements issus des règles écartées** : conservés et lisibles dans l'historique ; leur règle apparaît comme inactive (sans objet aujourd'hui, la fonctionnalité n'ayant jamais été mise en service).
- **Achat massif de cadeaux pour gonfler un statut** : accepté par construction (les points s'achètent), rendu inoffensif par l'absence d'avantage algorithmique attaché aux statuts et par l'interdiction de l'auto-cadeau. **Pendant la phase de paiement simulé**, où l'achat est gratuit, la purge de fin de phase (FR-020b) efface l'avantage ainsi obtenu.
- **Statut acquis grâce à des cadeaux simulés puis purgé** : le membre redescend au statut correspondant à son solde réel, sans perdre les points issus de ses j'aime et partages, et il en est informé.

## Requirements *(mandatory)*

### Recadrage du barème et des statuts

- **FR-001**: Le système DOIT restreindre les sources d'attribution de points actives à trois règles canoniques : **j'aime reçu** sur un contenu dont on est l'auteur/détenteur, **partage reçu** d'un contenu dont on est l'auteur/détenteur, **cadeau virtuel reçu**.
- **FR-002**: Le système DOIT **désactiver sans les supprimer** les règles écartées (contribution validée, contribution mise en avant, fact-check correct, fact-check abusif, proposition média validée, mise à la une, animation de support acceptée, bonus « 5 réseaux distincts »), en conservant leurs libellés et montants, et DOIT permettre leur **réactivation** depuis le back-office sans livraison technique.
- **FR-003**: Le système DOIT créditer **1 point par j'aime reçu**, montant paramétrable, et NE DOIT PLUS appliquer de logique de paliers de popularité pour l'attribution des points.
- **FR-004**: Le système DOIT définir quatre statuts de membre, **Membre Africans** (0 à 499), **Premium** (500 à 1 999), **Gold** (2 000 à 9 999), **Platinum** (10 000 et plus), dérivés du solde total de points, et DOIT refuser toute grille ambiguë (seuil dupliqué ou contredisant l'ordre).
- **FR-005**: Le système DOIT permettre de modifier libellés, seuils et apparence des statuts depuis le back-office, et DOIT rebasculer automatiquement tous les membres concernés sur le statut correspondant à leur solde après modification.
- **FR-006**: Le système DOIT appliquer toute modification du barème ou de la grille de statuts **sans redéploiement**, effective pour les attributions suivantes, sans recalculer les mouvements déjà journalisés.
- **FR-007**: Le système DOIT réserver le paramétrage aux administrateurs porteurs de la permission de gestion de l'engagement et DOIT tracer chaque modification dans la piste d'audit avec son auteur.

### J'aime reçus

- **FR-008**: Le système DOIT créditer l'**auteur/détenteur** d'un contenu à chaque j'aime reçu d'un **autre** membre, sur **toutes les familles de contenus dotées d'un « j'aime » binaire** : Codi-moi, contributions de gouvernance, bibliothèques humaines, médias télé/radio (chaînes, stations, programmes télé, programmes radio), vidéos Vidafrica, fiches pays et éléments Opportunité-Afrique.
- **FR-008c**: Le système NE DOIT PAS attribuer de points lorsqu'un contenu n'a **aucun auteur identifiable**, cas des éléments Opportunité-Afrique de nature éditoriale (sites touristiques, secteurs de développement), rattachés à une fiche pays sans contributeur enregistré. La réaction, le partage et l'affichage fonctionnent normalement ; seul le crédit est sans objet.
- **FR-008a**: Le système DOIT désigner, sur un **support co-détenu** (chaîne TV, station radio), le **détenteur propriétaire** comme unique bénéficiaire des points, des partages reçus et des cadeaux ; les co-détenteurs et animateurs ne reçoivent rien au titre du support.
- **FR-008b**: Le système NE DOIT PAS convertir en points les **notations sur échelle** (avis de sites touristiques, note d'expertise) ni les **réactions emoji autres que le cœur** (fact-check : pouce, rire, « j'aime pas ») : elles n'expriment pas une approbation binaire et resteraient arbitraires à convertir. Sur un fact-check, seule la réaction **cœur**, équivalent du « j'aime » dans cette interface, crédite l'auteur.
- **FR-009**: Le système NE DOIT PAS créditer l'auto-réaction : le j'aime de l'auteur sur son propre contenu ne rapporte rien.
- **FR-010**: Le système DOIT garantir l'unicité du crédit par **(contenu, membre qui aime)** : retirer puis remettre un j'aime ne crédite pas une seconde fois.
- **FR-011**: Le système NE DOIT PAS reprendre un point déjà attribué lorsqu'un j'aime est retiré.

### Partages reçus

- **FR-012**: Le système DOIT créditer l'**auteur/détenteur** du contenu lorsqu'un **autre** membre le partage, qu'il s'agisse d'un repartage interne sur le mur de la plateforme ou d'un partage vers un réseau social externe.
- **FR-013**: Le système DOIT garantir l'unicité du crédit par **(contenu, membre qui partage)**, tous canaux confondus : partager un même contenu vers plusieurs réseaux, ou vers un réseau puis sur le mur, ne crédite l'auteur qu'une seule fois.
- **FR-014**: Le système NE DOIT PAS attribuer de points pour l'auto-partage, ni au partageur, ni à l'auteur.
- **FR-015**: Le système DOIT enregistrer **chaque** partage (partageur, contenu, canal, date), y compris ceux qui ne créditent plus rien, de façon à rendre le comptage vérifiable, l'attribution auditable et la diffusion par canal mesurable.

### Cadeaux virtuels

- **FR-016**: Le système DOIT proposer un **catalogue de cadeaux** paramétrable en back-office (nom, apparence, prix, points attribués, ordre, actif/inactif), initialisé avec : Drapeau de l'Union Africaine (20 points), Badge (10), Chapeau (5), Fleur (3), Épingle de costume (1).
- **FR-017**: Le système DOIT permettre à un membre connecté d'offrir un cadeau **depuis un contenu** appartenant aux mêmes familles que celles éligibles au j'aime (FR-008) **ou depuis le profil public d'un membre**, le bénéficiaire étant l'auteur/détenteur du contenu ou le membre visé.
- **FR-018**: Le système DOIT proposer deux modes d'envoi : **soutien financier** (90 % au bénéficiaire, 10 % de frais de plateforme, taux paramétrable) et **cadeau en points** (100 % du montant à la plateforme).
- **FR-019**: Le système DOIT créditer au bénéficiaire les **points du cadeau dans les deux modes**, selon le barème du catalogue.
- **FR-020**: Le système DOIT **simuler le paiement** de bout en bout, sans mouvement d'argent réel, et DOIT permettre d'observer un aboutissement comme un échec ; le remplacement ultérieur de la simulation par un prestataire réel NE DOIT PAS exiger de refonte du parcours d'envoi ni du journal des transactions.
- **FR-020a**: Le système DOIT signaler **explicitement au membre**, à chaque étape du parcours d'envoi et sur sa cagnotte, que le paiement est **simulé** et que les points et montants correspondants relèvent d'une **phase de test**.
- **FR-020b**: Le système DOIT distinguer durablement les transactions **simulées** des transactions **réelles**, de façon à permettre, à la mise en service de l'encaissement réel, la **purge** des points d'engagement et des cagnottes issus des cadeaux simulés, sans toucher aux points gagnés par les j'aime et les partages.
- **FR-021**: Le système NE DOIT créditer les points, envoyer le cadeau et journaliser la répartition **que si le paiement (simulé) aboutit**.
- **FR-022**: Le système DOIT garantir qu'un même parcours de paiement rejoué ne produit **qu'un seul** cadeau et **qu'un seul** crédit de points.
- **FR-023**: Le système DOIT **interdire l'auto-cadeau** : s'offrir un cadeau à soi-même ou sur son propre contenu est refusé.
- **FR-024**: Le système DOIT **figer sur la transaction** le prix, le nombre de points et le taux de commission en vigueur au moment de l'envoi.
- **FR-025**: Le système DOIT journaliser chaque transaction (offreur, bénéficiaire, contenu, cadeau, mode, montant, part bénéficiaire, part plateforme, état, date) et DOIT tenir pour chaque bénéficiaire une **cagnotte de soutien** cumulée, consultable par lui.
- **FR-026**: Le système DOIT indiquer explicitement au bénéficiaire que le **versement de sa cagnotte n'est pas encore disponible**, tant que l'encaissement réel n'est pas en service.
- **FR-027**: Le système DOIT afficher les cadeaux reçus sur le contenu ou le profil concerné (objet et offreur), **sans jamais exposer de montant en argent** aux visiteurs.
- **FR-028**: Le système DOIT offrir à l'administration un **journal filtrable** des transactions (période, membre, état, mode) avec totaux de recettes plateforme et de cagnottes dues, et DOIT refuser la suppression définitive d'un cadeau déjà offert (désactivation seule).

### Espace membre et profil public

- **FR-029**: Le système DOIT présenter au membre son solde total, son statut courant parmi les quatre, l'écart de points jusqu'au statut suivant (ou la mention du statut maximal), et la ventilation de ses points par catégorie.
- **FR-030**: Le système DOIT présenter au membre l'historique paginé et filtrable de ses mouvements, chaque ligne nommant l'action en langage clair (« j'aime reçu sur … », « partage de … », « cadeau reçu de … »), la date, les points et l'éventuel écrêtage.
- **FR-031**: Le système DOIT afficher sur le **profil public** le statut du membre et ses badges obtenus, sans exposer ses soldes, sa cagnotte ni son historique.
- **FR-032**: Le système DOIT n'afficher au membre que des libellés **paramétrés en back-office** (actions, catégories, statuts, cadeaux), sans barème figé dans l'interface.
- **FR-033**: Le système DOIT **notifier** le membre lorsqu'il change de statut, reçoit un cadeau ou débloque un badge, en réutilisant le mécanisme de notification existant.

### Intégrité du moteur

- **FR-034**: Le système DOIT attribuer les points de manière **non bloquante** : aucun j'aime, partage ou envoi de cadeau ne DOIT échouer parce que l'attribution de points a échoué.
- **FR-035**: Le système DOIT **écrêter** les gains au-delà des plafonds paramétrés et **journaliser l'écrêtage** plutôt que d'ignorer l'action silencieusement.
- **FR-036**: Le système NE DOIT PAS attribuer de points **rétroactivement** : les j'aime, partages et cadeaux antérieurs à la mise en service du nouveau barème ne créditent pas.
- **FR-037**: Le système DOIT conserver intact l'historique des mouvements produits par les règles désormais désactivées, et DOIT continuer de les afficher lisiblement.
- **FR-038**: Le système NE DOIT accorder aux statuts **aucun avantage de visibilité algorithmique** (classement des fils, sélection « à la une ») ; ils restent des distinctions symboliques.

### Key Entities *(include if feature involves data)*

- **Compte d'engagement** *(existant)* : solde total, solde mensuel, réputation, statut dérivé. Relation 1–1 avec un membre.
- **Mouvement de points** *(existant)* : trace immuable d'un gain/perte, membre, action, objet concerné, points signés, catégorie, écrêtage, date.
- **Règle de points** *(existante, à recadrer)* : paramètre d'une action récompensée, identifiant d'action, libellé public, points, plafonds, catégorie, **état actif** (les règles écartées passent inactives).
- **Statut de membre** *(existant, à redéfinir)* : palier de solde, libellé, seuil d'entrée, ordre, apparence. Quatre entrées : Membre Africans, Premium, Gold, Platinum.
- **Cadeau du catalogue** *(nouveau)* : objet offrable, nom, apparence, **prix**, **points attribués**, ordre d'affichage, état actif.
- **Transaction de cadeau** *(nouvelle)* : envoi d'un cadeau, offreur, bénéficiaire, contenu concerné (famille + identifiant) ou membre visé, cadeau, **mode d'envoi**, montant, part bénéficiaire, part plateforme, points crédités, **état du paiement** (en attente / abouti / échoué / expiré), référence de paiement, date. Valeurs figées à l'envoi.
- **Cagnotte de soutien** *(nouvelle)* : cumul des parts revenant à un bénéficiaire, membre, montant cumulé, montant déjà versé (nul tant que le versement n'est pas en service).
- **Partage tracé** *(existant, à étendre)* : membre qui partage, contenu, **canal** (réseau externe ou mur interne), date. Enregistré pour chaque geste ; le **crédit** de points, lui, est unique par (partageur, contenu), tous canaux confondus.
- **Badge / Badge obtenu** *(existants)* : distinctions nominatives conservées en l'état ; leurs conditions restent exprimées sur les données d'engagement.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: À la mise en service, **exactement 3** règles de points sont actives et **100 %** des règles écartées sont présentes à l'état inactif, réactivables en moins d'une minute depuis le back-office.
- **SC-002**: **100 %** des membres voient un statut parmi les quatre définis, cohérent avec leur solde, et un membre à 500 points est classé Premium tandis qu'un membre à 499 reste Membre Africans.
- **SC-003**: Un contenu recevant N j'aime de membres distincts rapporte **exactement N points** à son auteur (au montant paramétré de 1), et rejouer 100 cycles retrait/remise de j'aime produit **0 point supplémentaire**.
- **SC-004**: Un cadeau offert avec paiement simulé abouti crédite le bénéficiaire en **moins de 5 secondes** et fait apparaître la répartition 90 / 10 (ou 0 / 100 selon le mode) dans le journal d'administration.
- **SC-005**: **0** point n'est crédité et **0** répartition n'est enregistrée lorsqu'un paiement simulé échoue, est abandonné ou expire.
- **SC-006**: Rejouer un parcours de paiement (double validation, rechargement, retour arrière) produit **0 doublon** de cadeau et **0 doublon** de points.
- **SC-007**: **0 %** des actions métier (j'aime, partage, envoi de cadeau) échouent à cause du mécanisme de points.
- **SC-008**: **100 %** des paramètres visibles par le membre (montants, libellés d'actions, statuts et seuils, catalogue de cadeaux et barème associé, taux de commission) sont modifiables depuis le back-office ; **aucune** valeur affichée n'est figée dans l'interface.
- **SC-009**: La somme des parts bénéficiaires et des parts plateforme du journal est **égale au centime** à la somme des montants des transactions abouties, sur un jeu de contrôle de 50 transactions.
- **SC-010**: Un membre atteint son solde, son statut, sa distance au statut suivant, ses cadeaux reçus et sa cagnotte en **au plus 2 clics** depuis son profil.
- **SC-011**: **0** point n'est attribué pour une auto-réaction, un auto-partage ou un auto-cadeau, sur **100 %** des tentatives.
- **SC-012**: Le remplacement du paiement simulé par un prestataire réel ne nécessite **aucune modification** du catalogue, du journal des transactions, de la répartition ni de l'attribution des points.
- **SC-013**: **100 %** des transactions produites pendant la phase de test sont identifiables comme simulées, et la purge de fin de phase retire **100 %** des points et cagnottes qui en sont issus tout en laissant **0** point de j'aime ou de partage affecté.

## Assumptions

- **Le socle d'engagement (spécifications 001 et 007) est livré mais jamais mis en service** : aucun membre n'a de points réels. Le recadrage peut donc redéfinir le barème et les statuts sans migrer d'historique significatif.
- Les **règles écartées sont désactivées, pas supprimées** (décision produit du 2026-08-08) : le barème restant paramétrable, un retour en arrière ou une réactivation partielle ne coûte aucune livraison technique.
- Le **mode « soutien financier » crédite aussi les points** du barème (décision produit du 2026-08-08) : le « ne reçoit **que** des points » du mode 2 s'oppose au mode 1, où le bénéficiaire reçoit argent **et** points.
- Les **partages internes et externes** créditent tous deux l'auteur (décision produit du 2026-08-08), le crédit étant unique par (contenu, partageur) quel que soit le canal.
- Le **montant du partage reçu** n'étant pas fixé par le document source, il est initialisé à **1 point**, aligné sur le j'aime, et reste paramétrable.
- Le **prix en argent de chaque cadeau** n'est pas fixé par le document source : il est **paramétrable en back-office**, exprimé dans la devise de référence de la plateforme (FCFA), et initialisé à des valeurs proportionnelles aux points du barème.
- Le **taux de commission** (10 %) est paramétrable ; il s'applique au mode « soutien financier » uniquement.
- Le **paiement simulé** ne met en jeu aucun mouvement d'argent réel. Il est **ouvert en production**, systématiquement signalé comme phase de test, et conçu pour être remplacé par l'agrégateur **CinetPay** sans refonte du parcours. Les points et cagnottes qu'il produit sont **temporaires** : ils seront purgés au basculement vers l'encaissement réel.
- La **purge de fin de phase de test** est une opération d'administration ponctuelle, exécutée une seule fois au basculement ; elle ne porte que sur les points et cagnottes issus de **cadeaux simulés** et laisse intacts les points gagnés par les j'aime et les partages.
- Le **versement (payout) de la cagnotte** aux bénéficiaires est hors périmètre : la cagnotte est cumulée et affichée, jamais versée dans cette itération.
- Les **points ne se dépensent pas** : offrir un cadeau se paie en argent, jamais en points. Il n'existe donc aucun transfert de points entre membres.
- Le **solde de points ne descend pas sous zéro** ; la réputation reste un score distinct n'entrant pas dans le calcul du statut.
- Les **catégories de points** conservées pour la ventilation sont : Popularité (j'aime), Partages, Cadeaux, Ajustements, les autres restent définies mais sans règle active.
- Les **badges de succès** livrés en 007 sont conservés en l'état ; leurs conditions portant sur des actions désormais inactives ne se déclencheront simplement plus, et l'administration peut les redéfinir sur les trois sources canoniques.
- Les écrans membre suivent les conventions des pages publiques de la plateforme ; les écrans d'administration celles du back-office.
- Le **barème n'est pas une donnée sensible** : ce qui rapporte des points et le catalogue de cadeaux sont publics ; les soldes, la cagnotte et l'historique d'un membre restent privés.

## Décisions produit (tranchées)

1. **Trois sources canoniques** de points : j'aime reçus, partages reçus, cadeaux reçus. Les autres règles sont **désactivées, conservées et réactivables**.
2. **Crédit unitaire du j'aime** (1 point, paramétrable) : les paliers de popularité ne pilotent plus l'attribution.
3. **Bénéficiaire du partage = l'auteur du contenu**, sur les canaux internes **et** externes, **une seule fois par partageur et par contenu**, inversion assumée de la règle de la spécification 007.
3 bis. **Bénéficiaire unique sur un support co-détenu** : le détenteur propriétaire, jamais les co-détenteurs.
4. **Quatre statuts** : Membre Africans (0), Premium (500), Gold (2 000), Platinum (10 000).
5. **Argent → points assumé** : renversement de la décision « aucune conversion argent → points » de la spécification 007, rendu acceptable par le maintien du caractère purement symbolique des statuts (aucun avantage algorithmique).
6. **Mode soutien financier = 90 % au bénéficiaire + les points** ; **mode cadeau en points = 100 % à la plateforme + les points**.
7. **Paiement simulé** en attendant CinetPay, **ouvert en production** et signalé comme phase de test ; répartition, journal et attribution de points fonctionnent réellement, mais les points et cagnottes issus des cadeaux simulés sont **purgés au basculement** vers l'encaissement réel.
8. **Aucun payout** dans cette itération : la cagnotte est cumulée et affichée, non versée.
9. **Auto-attribution interdite** sur les trois sources : auto-like, auto-partage, auto-cadeau.
10. **Classements publics** et **impact algorithmique des statuts** : toujours hors périmètre.

# Feature Specification: Récompenses par points, barème 100 % paramétrable & espace « Mon engagement »

**Feature Branch**: `007-engagement-points-badges` (répertoire de spec ; aucune branche créée automatiquement)

**Created**: 2026-07-29

**Status**: Draft

**Input**: User description: "on veut mettre en place un mécanisme de récompense par point aux utilisateurs selon certaines action sur la plateforme, voici le resumé documentations/Systeme_gamification_engagement.md , les points doivent être entièrement paramétrables dans le backoffice et l'utilisateur doit pouvoir consulter ses différentes catégories de points, badges etc. dans son espace profil"

## Contexte et cadrage

Le **socle d'engagement existe déjà** (phase 1 livrée : voir `specs/001-engagement-gamification/` et `documentations/Systeme_gamification_engagement.md`) : chaque membre possède un compte d'engagement (solde global, solde mensuel, réputation, niveau dérivé), chaque gain/perte est journalisé de façon immuable et idempotente, le barème est stocké en base plutôt qu'en dur, et un premier écran de back-office permet d'ajuster les montants existants. Un encart « Mes points » figure déjà dans la page de profil du membre.

**Cette spécification ne recrée rien de tout cela : elle complète le socle sur les trois axes demandés.**

1. **Paramétrage réellement complet en back-office**, aujourd'hui l'administrateur peut modifier les montants et plafonds des règles *déjà existantes*, mais il ne peut ni **créer** une nouvelle action récompensée, ni la **classer dans une catégorie**, ni définir des **paliers de popularité propres à une famille de contenus**, ni **créer ou retirer un niveau**, ni **définir un badge**. Le barème n'est donc paramétrable qu'à moitié.
2. **Espace membre digne du nom** : aujourd'hui un simple encart (solde, réputation, niveau, derniers mouvements) noyé dans la page profil. Le membre ne voit ni la **ventilation de ses points par catégorie**, ni sa **progression vers le niveau suivant**, ni ses **badges**, ni un **historique filtrable**.
3. **Couverture des actions** : plusieurs actions déjà mesurables sur la plateforme ne rapportent rien (proposition de contenu télé/radio validée, mise à la une d'un média, animation de support acceptée, popularité des contenus télé/radio), et les **partages vers des réseaux sociaux externes** ne sont pas tracés du tout.

**Explicitement hors périmètre de cette itération** (chantiers distincts) :

- **Publicité, monétisation, dons** : périmètre commercial séparé. Aucune conversion argent → points, en aucun cas.
- **Impact algorithmique des statuts** sur le classement des fils et les slots « à la une » : reporté (décision de la phase 1 maintenue), les niveaux restent des distinctions visuelles et symboliques.
- **Cadeaux entre utilisateurs** (Gô, Boro, Digbate, Lass, Viemogo) et **cadeaux partenaires** : **exclus** (décision produit du 2026-07-29). Mécanisme de *dépense* de points dont le modèle de coût (transfert intégral vs coût réduit) reste à trancher ; il fera l'objet d'une spécification dédiée avec son propre anti-abus (quota journalier, interdiction de l'auto-cadeau).
- **Classements publics** (global / par application / par territoire) : **exclus** (décision produit du 2026-07-29). Un classement établi sur un barème encore en calibration serait trompeur ; il sera spécifié une fois le barème stabilisé, avec la question du consentement à y figurer.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Consulter mon engagement : points, catégories, niveau, badges (Priority: P1)

Un membre connecté ouvre un espace dédié « Mon engagement » depuis son profil. Il y voit d'un coup d'œil : son **solde total de points**, son **solde du mois en cours**, sa **réputation**, son **niveau actuel** avec le badge correspondant et **combien de points le séparent du niveau suivant**. En dessous, ses points sont **ventilés par catégorie d'activité** (par exemple contributions, popularité, médias, vérification de faits, partages) avec le total gagné dans chacune. L'espace réserve l'emplacement de la section **badges**, qui sera renseignée par l'US3 (les badges nominatifs n'existent pas encore : seuls les badges de niveau existent aujourd'hui). Enfin, l'**historique complet** de ses mouvements est consultable, filtrable par catégorie et par période, paginé, chaque ligne indiquant l'action en langage clair, la date, les points et, le cas échéant, la mention « plafond atteint ».

**Why this priority**: C'est la demande explicite de l'utilisateur et la seule partie **visible** du système. Sans elle, les points restent invisibles et n'engagent personne. Elle est testable immédiatement sur les données déjà produites par le socle existant.

**Independent Test**: Se connecter avec un membre disposant d'un historique de points, ouvrir « Mon engagement », vérifier l'exactitude des soldes, du niveau, de la distance au niveau suivant, de la ventilation par catégorie (dont la somme doit être réconciliable avec le total gagné) et du filtrage de l'historique.

**Acceptance Scenarios**:

1. **Given** un membre ayant 250 points cumulés dont 40 issus de la popularité, **When** il ouvre « Mon engagement », **Then** il voit son solde total, le détail par catégorie incluant « Popularité : 40 », son niveau courant et le nombre de points restants pour atteindre le niveau supérieur.
2. **Given** un membre au niveau le plus élevé, **When** il consulte sa progression, **Then** l'espace indique qu'il a atteint le niveau maximal au lieu d'afficher une progression vide ou erronée.
3. **Given** un membre avec plus de 50 mouvements, **When** il filtre son historique sur la catégorie « Médias » et sur le mois en cours, **Then** seuls les mouvements correspondants s'affichent, paginés, du plus récent au plus ancien.
4. **Given** un membre n'ayant encore aucun point, **When** il ouvre l'espace, **Then** il voit un état vide pédagogique listant les actions qui rapportent des points, sans erreur ni compteur incohérent.
5. **Given** un visiteur consultant le profil public d'un membre, **When** la page se charge, **Then** il voit le **badge de niveau**, mais **pas** le détail des mouvements ni les soldes (données privées). *(Les badges de succès s'ajoutent à cet affichage avec l'US3.)*
6. **Given** un membre dont un gain a été écrêté par un plafond journalier, **When** il consulte son historique, **Then** la ligne concernée signale explicitement l'écrêtage.

---

### User Story 2 - Paramétrer intégralement le barème depuis le back-office (Priority: P1)

Un administrateur habilité ouvre le module d'engagement du back-office et administre **la totalité** du barème sans aucune intervention technique ni redéploiement : il **crée, modifie, active ou désactive** une règle de points (action récompensée, libellé affiché au membre, montant de points, impact réputation, plafond journalier, plafond mensuel, catégorie de rattachement), gère les **catégories de points** utilisées pour la ventilation côté membre, définit les **paliers de popularité** (seuil de « j'aime » → points) globalement ou pour une **famille de contenus** donnée, **crée, réordonne, modifie ou retire un niveau** (libellé, seuil d'entrée, apparence du badge), et consulte le **journal global** filtrable pour investiguer un litige, avec possibilité de **crédit/débit manuel motivé**.

**Why this priority**: L'exigence « entièrement paramétrable » est explicite. Sans création de règles, chaque nouvelle action récompensée exigerait une livraison technique : ce que le produit refuse. C'est aussi le préalable aux stories 3 à 5, qui se contentent alors de brancher des actions sur des règles créées par l'administration.

**Independent Test**: Créer une nouvelle règle avec un montant et un plafond, déclencher l'action correspondante et constater le crédit au bon montant ; désactiver la règle et constater qu'aucun point n'est plus attribué ; créer un niveau intermédiaire et constater que les membres concernés changent de niveau ; vérifier que chaque modification apparaît dans la piste d'audit.

**Acceptance Scenarios**:

1. **Given** un administrateur habilité, **When** il crée une règle « proposition de média validée » à +5 points, plafonnée à 3 par jour et rattachée à la catégorie « Médias », **Then** la règle est immédiatement active et la prochaine action correspondante crédite +5, comptabilisés dans la catégorie « Médias ».
2. **Given** une règle active ayant déjà généré des mouvements, **When** l'administrateur la **désactive**, **Then** aucune attribution nouvelle n'a lieu pour cette action, **et** l'historique des mouvements passés reste intact et lisible.
3. **Given** un administrateur, **When** il modifie le montant d'une règle, **Then** les attributions suivantes utilisent le nouveau montant, les mouvements déjà journalisés ne sont pas recalculés, et la modification est tracée dans l'audit avec son auteur.
4. **Given** un administrateur, **When** il crée un palier de popularité à 2 000 « j'aime » réservé à la famille « contenus télé/radio », **Then** ce palier ne s'applique qu'à cette famille et les autres familles conservent les paliers globaux.
5. **Given** un administrateur, **When** il insère un niveau intermédiaire à 500 points, **Then** les membres situés entre 500 et le seuil suivant basculent sur ce nouveau niveau et son badge dès leur prochaine consultation, sans opération manuelle membre par membre.
6. **Given** un utilisateur **sans** la permission de gestion de l'engagement, **When** il tente d'accéder au module ou d'exécuter une opération de paramétrage, **Then** l'accès est refusé avec un message explicite nommant la permission requise, et le refus est observable dans les **journaux techniques du serveur**. *(La piste d'audit fonctionnelle, elle, ne consigne que les modifications effectivement appliquées : instrumenter les refus supposerait de modifier le contrôle de permission commun à toutes les routes d'administration de la plateforme, hors périmètre.)*
7. **Given** un administrateur, **When** il tente de créer deux règles pour la même action, ou deux niveaux au même seuil, **Then** le système refuse avec un message explicite plutôt que de créer un barème ambigu.
8. **Given** un administrateur, **When** il applique un ajustement manuel motivé à un membre, **Then** le mouvement apparaît dans le journal du membre avec le motif et l'identité de l'administrateur.

---

### User Story 3 - Débloquer des badges et succès (Priority: P2)

Au-delà des badges de niveau, la plateforme distingue les membres par des **badges/succès paramétrables** : l'administration définit chaque badge (nom, description, apparence, condition d'obtention exprimée à partir des données d'engagement, nombre d'occurrences d'une action donnée, total de points dans une catégorie, solde total atteint, niveau atteint, palier de popularité atteint) et le système les attribue **automatiquement** dès que la condition est remplie. Un badge peut aussi être attribué ou retiré **manuellement** par l'administration pour les distinctions éditoriales. Le membre est **notifié** lorsqu'il débloque un badge ou change de niveau, et ses badges sont visibles sur son profil public.

**Why this priority**: La demande cite explicitement les badges parmi ce que le membre doit pouvoir consulter, et les badges nominatifs **n'existent pas** aujourd'hui (seuls les badges de niveau existent). C'est la principale nouveauté fonctionnelle côté membre, mais elle s'appuie sur l'espace membre (US1) et sur le paramétrage (US2) : d'où P2.

**Independent Test**: Définir un badge conditionné à 10 contributions validées, faire atteindre ce seuil à un membre de test, vérifier que le badge apparaît une seule fois dans son espace et sur son profil public, qu'il reçoit une notification, et que la condition réévaluée ne crée pas de doublon.

**Acceptance Scenarios**:

1. **Given** un badge « Conteur » conditionné à 10 contributions validées et un membre en ayant 9, **When** sa 10ᵉ contribution est validée, **Then** le badge lui est attribué une seule fois, horodaté, et il reçoit une notification.
2. **Given** un membre possédant déjà un badge, **When** la condition est réévaluée plusieurs fois, **Then** aucun doublon n'est créé et aucune notification supplémentaire n'est envoyée.
3. **Given** un administrateur, **When** il crée, modifie ou désactive un badge, **Then** le catalogue affiché aux membres reflète le changement, sans retirer les badges déjà obtenus.
4. **Given** un membre, **When** il consulte la section badges, **Then** il voit ses badges obtenus (avec date) **et** les badges encore à débloquer avec leur condition en langage clair et, quand c'est chiffrable, sa progression vers cette condition.
5. **Given** un administrateur, **When** il attribue manuellement un badge éditorial à un membre, **Then** le badge apparaît chez le membre, l'opération est tracée dans l'audit et le membre est notifié.
6. **Given** un membre dont le solde repasse sous le seuil de son niveau après un malus, **When** son niveau est recalculé, **Then** le badge de **niveau** reflète le niveau courant, tandis que les **badges de succès déjà obtenus restent acquis**.

---

### User Story 4 - Récompenser les actions non encore couvertes (Priority: P2)

Des actions déjà mesurables mais aujourd'hui non récompensées créditent désormais leur auteur, selon des règles créées en back-office : **proposition de contenu média (télé/radio) validée**, **contenu média mis à la une**, **demande d'animation ou de co-détention d'un support acceptée**, et **popularité des contenus télé/radio** (les réactions positives des chaînes, stations et programmes alimentent les paliers de popularité).

**Why this priority**: Étend la portée du mécanisme aux domaines les plus récemment livrés, où la contribution communautaire est déjà organisée. Ce sont des branchements sur un moteur existant : forte valeur, mais aucune valeur perceptible sans l'espace membre (US1) ni le paramétrage (US2).

**Independent Test**: Pour chacune des actions, la déclencher de bout en bout dans son parcours réel et vérifier le crédit unique au bon bénéficiaire, la présence du mouvement dans son historique avec la bonne catégorie, et l'absence de crédit en cas de rejeu ou d'auto-attribution.

**Acceptance Scenarios**:

1. **Given** un membre ayant proposé un contenu média en attente, **When** un administrateur valide la proposition, **Then** le membre est crédité une seule fois du montant paramétré et le mouvement apparaît dans la catégorie « Médias ».
2. **Given** un contenu média publié, **When** il est mis « à la une », **Then** son détenteur est crédité une seule fois pour ce contenu, même si la mise à la une est retirée puis reposée.
3. **Given** une demande d'animation de support en attente, **When** elle est acceptée, **Then** le demandeur est crédité du montant paramétré.
4. **Given** un administrateur qui valide sa propre proposition de média, **When** la validation aboutit, **Then** aucun point n'est attribué (interdiction d'auto-attribution).
5. **Given** un contenu média franchissant un palier de popularité, **When** le palier est atteint, **Then** son auteur/détenteur est crédité une seule fois pour ce palier et ce contenu.
6. **Given** une règle média désactivée en back-office, **When** l'action correspondante se produit, **Then** aucun point n'est attribué et l'action métier réussit normalement.

---

### User Story 5 - Récompenser le partage vers les réseaux sociaux externes (Priority: P3)

Un membre partage un contenu de la plateforme vers des réseaux sociaux externes. Lorsqu'il a partagé un même contenu vers **au moins 5 réseaux distincts**, il est crédité d'un bonus paramétrable, dans la limite d'un **plafond journalier** paramétrable. Les partages sont comptés **par réseau distinct** : répéter le même réseau ne compte qu'une fois.

**Why this priority**: Levier de croissance explicitement demandé par le document source, mais c'est la règle la **moins vérifiable** (le système constate une intention de partage, pas sa publication effective) et la seule à exiger un nouveau suivi de données. Livrée en dernier, avec un anti-abus strict.

**Independent Test**: Depuis un contenu, déclencher un partage vers 5 réseaux différents et vérifier le crédit unique du bonus ; répéter le même réseau et vérifier l'absence de crédit ; dépasser le plafond journalier et vérifier l'écrêtage visible dans l'historique.

**Acceptance Scenarios**:

1. **Given** un membre ayant partagé un contenu vers 4 réseaux distincts, **When** il le partage vers un 5ᵉ réseau distinct, **Then** il est crédité du bonus une seule fois pour ce contenu.
2. **Given** un membre ayant déjà obtenu le bonus pour un contenu, **When** il partage ce même contenu vers un 6ᵉ réseau, **Then** aucun bonus supplémentaire n'est attribué pour ce contenu.
3. **Given** un membre répétant 5 fois le partage du même contenu vers le même réseau, **When** les partages sont enregistrés, **Then** aucun bonus n'est attribué (5 réseaux **distincts** requis).
4. **Given** un membre ayant atteint le plafond journalier de bonus de partage, **When** il complète un nouveau contenu à 5 réseaux le même jour, **Then** aucun point n'est crédité au-delà du plafond et l'écrêtage est visible dans son historique.
5. **Given** un membre qui déclenche un partage, **When** l'enregistrement du partage échoue, **Then** l'action de partage reste utilisable (aucun blocage du parcours).

---

### Edge Cases

- **Règle inexistante ou désactivée** au moment où une action se produit : aucune attribution, aucun échec de l'action métier, trace technique consultable.
- **Suppression d'une règle référencée par des mouvements** : refusée ; seule la désactivation est possible, afin que l'historique reste lisible.
- **Changement de catégorie d'une règle** : la ventilation reflète la catégorie **au moment du mouvement** ; une re-catégorisation n'est pas rétroactive.
- **Suppression ou renommage d'une catégorie encore utilisée** : suppression refusée tant que des règles y sont rattachées ; le renommage se répercute sur l'affichage.
- **Niveaux incohérents** (seuil dupliqué, seuil contredisant l'ordre des niveaux) : refusés à l'enregistrement.
- **Retrait d'un niveau intermédiaire** : les membres concernés retombent sur le niveau immédiatement inférieur, sans perdre de points.
- **Condition d'un badge modifiée après attribution** : les membres l'ayant déjà obtenu le conservent ; les nouveaux entrants sont évalués sur la nouvelle condition.
- **Badge désactivé** : il disparaît du catalogue « à débloquer » mais reste affiché chez ceux qui l'ont obtenu.
- **Plafond journalier ou mensuel atteint** : les points au-delà ne sont pas crédités et l'écrêtage est explicitement visible dans l'historique du membre (jamais d'échec silencieux).
- **Auto-attribution** : valider sa propre proposition, mettre à la une son propre contenu, aimer son propre contenu, se partager à soi-même → jamais de points.
- **Actions antérieures à l'activation d'une nouvelle règle** : non récompensées rétroactivement.
- **Somme des catégories ≠ solde courant** (malus, ajustement administratif, plancher à zéro) : l'espace membre distingue clairement « points gagnés par catégorie » et « solde courant », de sorte que l'écart reste compréhensible.
- **Changement de mois** : le solde mensuel repart à zéro sans affecter le solde global ni les cumuls par catégorie.
- **Panne du moteur de points** : aucune action métier (validation, mise à la une, acceptation, réaction, partage) n'échoue à cause de l'attribution de points ou de l'évaluation d'un badge.
- **Membre supprimé** : compte d'engagement, historique et badges disparaissent avec lui, sans laisser de données orphelines.

## Requirements *(mandatory)*

### Functional Requirements

**Paramétrage du barème (back-office)**

- **FR-001**: Le système DOIT permettre à un administrateur habilité de **créer** une règle de points pour une action récompensée, en définissant : identifiant d'action, libellé affiché au membre, montant de points (positif ou négatif), impact sur la réputation, plafond journalier, plafond mensuel, catégorie de rattachement et état actif/inactif.
- **FR-002**: Le système DOIT permettre de **modifier** et **d'activer/désactiver** toute règle existante, et DOIT **refuser la suppression définitive** d'une règle déjà référencée par des mouvements journalisés.
- **FR-003**: Le système DOIT garantir l'**unicité** de l'identifiant d'action d'une règle et refuser toute création en doublon avec un message explicite.
- **FR-004**: Le système DOIT permettre de gérer des **catégories de points** (création, libellé, ordre d'affichage, apparence) servant à ventiler les points côté membre, et DOIT refuser la suppression d'une catégorie encore rattachée à une règle.
- **FR-005**: Le système DOIT permettre de gérer les **paliers de popularité** (seuil de « j'aime » → points) et de restreindre facultativement un palier à une **famille de contenus** ; à défaut de palier spécifique, les paliers globaux s'appliquent.
- **FR-006**: Le système DOIT permettre de **créer, modifier, réordonner et retirer** un niveau (libellé, seuil d'entrée, apparence du badge) en refusant tout barème ambigu.
- **FR-007**: Le système DOIT appliquer toute modification du barème **sans redéploiement**, effective pour les attributions suivantes, sans recalculer les mouvements déjà journalisés.
- **FR-008**: Le système DOIT **réserver** les opérations de paramétrage aux administrateurs porteurs de la permission de gestion de l'engagement, et **tracer dans l'audit** chaque création, modification, activation/désactivation, attribution manuelle de badge et ajustement de points, avec son auteur. La piste d'audit couvre les opérations **appliquées** ; les **refus** de permission relèvent des journaux techniques du serveur.
- **FR-009**: Le système DOIT permettre à un administrateur de consulter le **journal global** des mouvements, filtrable au minimum par membre, type d'action, catégorie et période, et d'appliquer un **crédit/débit manuel motivé** à un membre.

**Espace membre « Mon engagement »**

- **FR-010**: Le système DOIT offrir au membre connecté un **espace dédié** dans son profil présentant : solde total de points, solde du mois en cours, score de réputation, niveau courant avec son badge, et **écart de points jusqu'au niveau suivant** (ou mention explicite du niveau maximal atteint).
- **FR-011**: Le système DOIT présenter la **ventilation des points gagnés par catégorie**, chaque catégorie affichant son libellé et son total, en distinguant clairement ce cumul du **solde courant**.
- **FR-012**: Le système DOIT présenter l'**historique paginé** des mouvements du membre, filtrable par catégorie et par période, chaque ligne indiquant l'action en langage clair, la date, les points signés, l'impact réputation éventuel et la mention d'un éventuel **écrêtage par plafond**.
- **FR-013**: Le système DOIT présenter au membre ses **badges obtenus** (avec date d'obtention) et le **catalogue des badges à débloquer**, avec leur condition en langage clair et, lorsque la condition est chiffrable, sa progression. *(Livré avec l'US3 : sans badges définis, la section reste un emplacement vide.)*
- **FR-014**: Le système DOIT afficher sur le **profil public** d'un membre son **badge de niveau**, dès l'US1, et ses **badges de succès obtenus**, avec l'US3 , sans exposer ses soldes ni son historique, réservés au titulaire et aux administrateurs.
- **FR-015**: Le système DOIT afficher un **état vide pédagogique** listant les actions récompensées lorsqu'un membre n'a encore aucun point.
- **FR-016**: Le système DOIT n'afficher au membre que des libellés **paramétrés en back-office** (actions, catégories, niveaux, badges), sans texte de barème figé dans l'interface.

**Badges et succès**

- **FR-017**: Le système DOIT permettre à un administrateur de **définir un badge** : nom, description, apparence, condition d'obtention automatique choisie parmi un ensemble de conditions mesurables (nombre d'occurrences d'une action donnée, total de points dans une catégorie, solde total atteint, niveau atteint, palier de popularité atteint), et état actif/inactif.
- **FR-018**: Le système DOIT **attribuer automatiquement** un badge dès que sa condition est remplie, **une seule fois par membre**, en horodatant l'obtention.
- **FR-019**: Le système DOIT permettre l'**attribution et le retrait manuels** d'un badge par un administrateur habilité, tracés dans l'audit.
- **FR-020**: Le système DOIT **conserver** les badges déjà obtenus lorsque leur condition est modifiée, lorsque le badge est désactivé ou lorsque le solde du membre diminue ; seul le **badge de niveau** suit le niveau courant à la hausse comme à la baisse.
- **FR-021**: Le système DOIT **notifier** le membre lorsqu'il débloque un badge ou change de niveau, en réutilisant le mécanisme de notification existant de la plateforme.

**Couverture des actions récompensées**

- **FR-022**: Le système DOIT créditer l'auteur d'une **proposition de contenu média (télé/radio) validée** par la modération, du montant paramétré.
- **FR-023**: Le système DOIT créditer le détenteur d'un **contenu média mis à la une**, une seule fois par contenu, du montant paramétré.
- **FR-024**: Le système DOIT créditer le demandeur d'une **demande d'animation / de co-détention de support acceptée**, du montant paramétré.
- **FR-025**: Le système DOIT faire contribuer les **réactions positives des contenus télé/radio** (chaînes, stations, programmes) au franchissement des **paliers de popularité**, chaque palier ne créditant qu'une fois par contenu.
- **FR-026**: Le système DOIT conserver sans régression le fonctionnement des actions **déjà récompensées** (contribution validée, contribution mise en avant, vérification de faits correcte ou abusive, paliers de popularité existants).
- **FR-027**: Le système DOIT enregistrer les **partages d'un contenu vers un réseau social externe** (membre, contenu, réseau, date) et DOIT créditer le bonus paramétré lorsqu'un même contenu a été partagé vers au moins **5 réseaux distincts**, une seule fois par contenu et dans la limite d'un **plafond journalier** paramétrable.

**Intégrité du moteur (garanties à préserver et à étendre)**

- **FR-028**: Le système DOIT attribuer les points de manière **non-bloquante** : aucune action métier déclencheuse ne DOIT échouer parce que l'attribution de points ou l'évaluation d'un badge a échoué.
- **FR-029**: Le système DOIT garantir l'**idempotence** : une même action identifiée (type d'action + objet concerné + palier éventuel) ne peut créditer ou débiter qu'une seule fois, même en cas de rejeu ou d'oscillation d'un compteur.
- **FR-030**: Le système DOIT **interdire l'auto-attribution** de points sur toutes les nouvelles actions couvertes : auto-validation d'une proposition, auto-mise à la une, auto-réaction (le « j'aime » de l'auteur ne compte pas dans ses propres paliers de popularité), et auto-repost interne. En revanche, **partager son propre contenu vers un réseau social externe RAPPORTE des points** au partageur : le bénéficiaire de la règle de partage est celui qui partage, jamais l'auteur du contenu, et promouvoir sa propre contribution à l'extérieur de la plateforme est précisément le comportement recherché.
- **FR-031**: Le système DOIT **écrêter** les gains au-delà des plafonds paramétrés et **journaliser l'écrêtage** plutôt que d'ignorer l'action silencieusement.
- **FR-032**: Le système NE DOIT PAS attribuer de points **rétroactivement** lors de l'activation d'une nouvelle règle : seules les actions postérieures créditent.
- **FR-033**: Le système NE DOIT PAS **reprendre** les points déjà attribués lorsqu'un contenu est ensuite supprimé, dé-publié ou re-jugé ; la seule perte possible provient d'une règle de malus explicite ou d'un ajustement administratif motivé.
- **FR-034**: Le système DOIT rendre l'**attribution des badges** insensible aux réévaluations répétées (aucun doublon de badge, aucune notification répétée).

### Key Entities *(include if feature involves data)*

- **Compte d'engagement** *(existant)* : capital d'engagement d'un membre, solde total, solde mensuel, réputation, niveau courant dérivé, date du dernier mouvement. Relation 1–1 avec un membre.
- **Mouvement de points** *(existant, à enrichir)* : trace immuable d'un gain/perte, membre, action, objet concerné, points signés, impact réputation, solde résultant, indicateur d'écrêtage, **catégorie au moment du mouvement**, date.
- **Règle de points** *(existante, à rendre créable)* : paramètre d'une action récompensée, identifiant d'action, libellé public, points, impact réputation, plafond journalier, plafond mensuel, **catégorie**, état actif.
- **Catégorie de points** *(nouvelle)* : regroupement d'actions servant à la ventilation côté membre, libellé, ordre d'affichage, apparence. Relation 1–N avec les règles.
- **Palier de popularité** *(existant, à enrichir)* : seuil de « j'aime » déclenchant une récompense unique par contenu, seuil, points, **famille de contenus facultative**, état actif.
- **Niveau** *(existant, à rendre créable/supprimable)* : borne de solde définissant un statut, libellé, seuil d'entrée, ordre, apparence du badge.
- **Badge** *(nouveau)* : distinction nominative, nom, description, apparence, type et paramètres de la condition d'obtention, état actif.
- **Badge obtenu** *(nouveau)* : lien membre ↔ badge, date d'obtention, origine (automatique ou attribution éditoriale), administrateur à l'origine le cas échéant. Unique par (membre, badge).
- **Partage externe** *(nouveau)* : trace d'un partage vers un réseau social externe, membre, contenu partagé (famille + identifiant), réseau, date. Unique par (membre, contenu, réseau) pour le comptage des réseaux distincts.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un administrateur peut créer une action récompensée, la doter d'un montant, d'un plafond et d'une catégorie, et constater son application sur une action réelle en **moins de 5 minutes**, **sans aucune intervention technique ni redéploiement**.
- **SC-002**: **100 %** des paramètres du barème visibles par le membre (libellés d'actions, catégories, niveaux, seuils, badges, montants, plafonds) sont modifiables depuis le back-office ; **aucune** valeur affichée n'est figée dans l'interface.
- **SC-003**: Un membre atteint son solde, sa ventilation par catégorie, son niveau, sa distance au niveau suivant et ses badges en **au plus 2 clics** depuis son profil.
- **SC-004**: Une action récompensée est reflétée dans l'espace « Mon engagement » du membre en **moins de 5 secondes** après sa finalisation.
- **SC-005**: **100 %** des mouvements sont traçables (action, objet, montant, catégorie, date) et la ventilation par catégorie est **réconciliable** avec le total gagné, malus et ajustements administratifs inclus, sur un jeu de contrôle.
- **SC-006**: Rejouer une action récompensable (double validation, mise à la une retirée puis reposée, oscillation de « j'aime » autour d'un palier, réévaluation de badge) produit **0 doublon** de points et **0 doublon** de badge.
- **SC-007**: **0 %** des actions métier (validation, mise à la une, acceptation d'engagement, réaction, partage) échouent à cause du mécanisme de points ou de badges.
- **SC-008**: Un membre atteignant un plafond journalier ne reçoit **aucun point au-delà du plafond** ce jour-là, et l'écrêtage est visible dans son historique.
- **SC-009**: **100 %** des tentatives de paramétrage par un utilisateur non habilité sont refusées, et **100 %** des modifications du barème apparaissent dans la piste d'audit avec leur auteur.
- **SC-010**: Un membre est notifié d'un changement de niveau ou d'un badge débloqué dans la **minute** suivant l'événement, et jamais plus d'une fois pour le même badge.
- **SC-011**: Aucune régression sur les actions déjà récompensées avant cette itération : à barème inchangé, montants et comportements restent identiques.

## Assumptions

- **La phase 1 du système d'engagement est en service** : compte d'engagement, journal immuable et idempotent, barème stocké en base, niveaux, encart « Mes points », journal d'administration et ajustement manuel existent déjà. Cette itération les **étend** sans les réécrire.
- Les **montants** cités par le document source (proposition média +5, mise à la une +8, animation acceptée +15, partage vers 5 réseaux +10 plafonné à 3 fois par jour) sont des **valeurs de départ paramétrables**, initialisées puis ajustables par le produit sans livraison technique.
- Les **catégories de points** proposées par défaut sont : Contributions, Popularité, Médias, Vérification de faits, Partages, Ajustements, libellés et périmètre restant paramétrables.
- La **ventilation par catégorie** est un cumul des points **gagnés** par catégorie, dérivé du journal ; les points ne sont pas cloisonnés en soldes séparés dépensables par catégorie, et le niveau continue de dépendre du seul solde global.
- La **réputation** reste un score distinct, non dépensable, n'entrant pas dans le calcul du niveau.
- Le **solde de points ne descend pas sous zéro** (plancher à 0) ; la réputation peut être négative.
- La **popularité** ne compte que les réactions positives et exclut l'auto-réaction.
- Les **conditions de badge** sont choisies parmi un ensemble fermé de types de conditions mesurables : l'administration paramètre les valeurs, pas des expressions libres.
- Les **badges** sont **rétro-évalués une fois à la mise en service** sur l'état courant des comptes (un membre ayant déjà 10 contributions validées obtient le badge correspondant), afin que le catalogue ne paraisse pas vide au lancement ; les **points**, eux, ne sont jamais attribués rétroactivement.
- Les **notifications** d'engagement (niveau atteint, badge débloqué) réutilisent le mécanisme de notification existant de la plateforme ; aucun service de notification dédié n'est créé.
- Le **partage externe** est constaté au moment où le membre déclenche l'action de partage depuis la plateforme ; le système ne vérifie pas la publication effective sur le réseau : d'où le plafond journalier et l'exigence de 5 réseaux **distincts** comme garde-fous. Le **bénéficiaire est le partageur**, y compris lorsqu'il partage son propre contenu (FR-030).
- Le **barème n'est pas une donnée sensible** : la liste des actions récompensées et de leurs montants est consultable publiquement (elle sert l'engagement en montrant ce qui rapporte des points). Seuls les soldes, la réputation et l'historique d'un membre sont privés.
- Le back-office d'engagement reste réservé aux détenteurs de la permission de gestion de l'engagement (aujourd'hui les super-administrateurs).
- Les écrans membre suivent les conventions des pages publiques de la plateforme ; les écrans d'administration celles du back-office.

## Décisions produit (tranchées)

1. **Impact algorithmique des statuts** sur la visibilité des fils : **reporté**, les niveaux restent des distinctions visuelles et symboliques.
2. **Publicité, monétisation, dons** : **hors périmètre**, chantier autonome ; aucune conversion argent → points.
3. **Pas de reprise automatique** des points acquis ; seuls un malus explicite du barème ou un ajustement administratif motivé retirent des points.
4. **Pas de rétroactivité des points** à l'activation d'une nouvelle règle ; rétro-évaluation unique des **badges** au lancement.
5. **Points non cloisonnés** : une seule cagnotte, ventilée par catégorie pour la lecture.
6. **Cadeaux entre utilisateurs** : **hors périmètre** de cette itération (spécification dédiée ultérieure).
7. **Classements publics** : **hors périmètre** de cette itération (à spécifier après stabilisation du barème).

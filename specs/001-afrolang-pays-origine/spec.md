# Feature Specification: Pays d'origine des salles publiques Afrolang

**Feature Branch**: `001-afrolang-pays-origine`
**Created**: 2026-05-10
**Status**: Draft
**Input**: User description: "sur la page uafricas_frontend/app/pages/afrolang/index.vue on voudrait connaître les pays d'origine des cours/salle public. Mais il semble que le modèle de données ne le prenne pas en compte, je veux corriger cela"

## Clarifications

### Session 2026-05-10

- Q: Faut-il pré-remplir les pays d'origine des salles existantes à partir du pays de leur groupe ethnique ? → A: Non, laisser toutes les listes vides au déploiement ; enrichissement 100 % manuel par les admins.
- Q: Le filtre public par pays doit-il être mono ou multi-sélection ? → A: Mono-pays — un seul pays sélectionnable à la fois (radio / select simple). La multi-sélection est explicitement reportée à une itération ultérieure.
- Q: Comment traiter un pays archivé/désactivé dans le référentiel `shared.pays` ? → A: Masqué côté public (réponse API + filtre) ; toujours visible côté admin avec mention « archivé », l'association est conservée pour permettre un éventuel nettoyage ciblé.
- Q: Comment afficher les pays sur une carte de salle quand la liste est longue ? → A: De 1 à 3 pays, drapeau + nom ; au-delà de 3, basculer en mode drapeaux seuls alignés en ligne (sans nom), liste complète accessible au survol/tooltip.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Voir les pays d'origine d'une salle publique sur l'annuaire (Priority: P1)

Un visiteur (anonyme ou connecté) parcourt la page `/afrolang` qui liste les salles publiques d'apprentissage de langues africaines. Pour chaque salle (ex. « Wolof », « Swahili », « Lingala »), il souhaite voir clairement le ou les pays d'origine de la langue afin de choisir une salle qui l'intéresse géographiquement et culturellement.

**Why this priority**: C'est la valeur métier principale. Sans cette information visible, l'utilisateur ne peut pas relier une langue à son territoire d'origine — l'objectif éducatif et culturel d'Afrolang en dépend. Aujourd'hui, le modèle ne permet de rattacher qu'un seul pays implicite (via le groupe ethnique) alors qu'une même langue couvre fréquemment plusieurs pays (le wolof : Sénégal + Gambie + Mauritanie ; le swahili : Tanzanie, Kenya, Ouganda, RDC, etc.).

**Independent Test**: Une administratrice associe deux pays d'origine à la salle « Wolof » ; un visiteur ouvre `/afrolang` et voit la liste des deux pays (drapeau + nom) sur la carte de la salle. Les filtres et le compteur de salles fonctionnent toujours.

**Acceptance Scenarios**:

1. **Given** une salle publique « Wolof » associée aux pays Sénégal, Gambie et Mauritanie, **When** un visiteur charge `/afrolang`, **Then** la carte de la salle affiche les trois pays (nom et drapeau) de manière lisible et non tronquée.
2. **Given** une salle publique « Lingala » associée à un seul pays (RDC), **When** un visiteur consulte la liste, **Then** le pays unique s'affiche avec le même format visuel cohérent.
3. **Given** une salle publique nouvellement créée sans pays d'origine renseigné, **When** un visiteur consulte la liste, **Then** la carte indique explicitement qu'aucun pays d'origine n'est renseigné (ou masque la zone) sans casser la mise en page.

---

### User Story 2 - Filtrer les salles publiques par pays d'origine (Priority: P2)

Un visiteur souhaite ne voir que les salles dont la langue est parlée dans un pays donné (ex. « afficher uniquement les langues du Sénégal »). Il utilise le panneau de filtres latéral de `/afrolang` pour sélectionner un pays et la liste se met à jour.

**Why this priority**: Renforce considérablement la découverte. Filtrer par pays est attendu par les diasporas et les apprenants ciblés géographiquement. Reste P2 car la valeur dépend de l'existence du nouveau lien (US1).

**Independent Test**: Une admin associe le Sénégal aux salles « Wolof » et « Sérère ». Un visiteur sélectionne « Sénégal » dans le filtre et voit uniquement ces deux salles, le compteur de résultats reflète le nombre filtré.

**Acceptance Scenarios**:

1. **Given** la liste complète des salles publiques, **When** le visiteur sélectionne « Sénégal » dans le filtre pays, **Then** seules les salles ayant le Sénégal parmi leurs pays d'origine sont affichées et le compteur correspond.
2. **Given** un filtre pays actif, **When** le visiteur clique « Réinitialiser les filtres », **Then** le filtre pays est effacé et toutes les salles réapparaissent.
3. **Given** un filtre pays combiné à une recherche texte, **When** la requête s'exécute, **Then** les deux critères sont appliqués cumulativement (logique ET).

---

### User Story 3 - Gérer les pays d'origine d'une salle depuis l'administration (Priority: P1)

Une administratrice ouvre la fiche d'édition d'une salle publique dans le back-office Afrolang et associe ou retire un ou plusieurs pays parmi le référentiel des pays existants. Elle enregistre, l'audit est tracé, le rendu public reflète immédiatement le changement.

**Why this priority**: Sans cet outil de gestion, l'information reste invisible et incorrecte. Indispensable conjointement à US1 pour que la fonctionnalité ait un sens dès la première itération.

**Independent Test**: Une admin ouvre la salle « Swahili », sélectionne Tanzanie, Kenya, Ouganda, valide. Elle recharge `/afrolang` (vue publique) et voit les trois pays. Une seconde modification (retrait d'un pays) est reflétée après rechargement et apparaît dans le journal d'audit.

**Acceptance Scenarios**:

1. **Given** une salle publique sans pays associé, **When** l'admin ajoute trois pays et enregistre, **Then** les trois pays sont persistés, le journal d'audit enregistre l'action et l'API publique les renvoie.
2. **Given** une salle avec des pays associés, **When** l'admin retire un pays et enregistre, **Then** la liste publique ne contient plus ce pays et l'audit trace le retrait.
3. **Given** l'admin tente d'associer deux fois le même pays à une salle, **When** elle valide, **Then** le système empêche le doublon (sans erreur bloquante visible) et conserve une seule occurrence.

---

### Edge Cases

- Salle publique existante avant la mise en place de la fonctionnalité : la liste de pays est initialement vide ; l'affichage doit gérer ce cas et l'admin doit pouvoir compléter sans contrainte de migration bloquante.
- Une langue rattachée à un groupe ethnique dont la fiche pays diffère des pays d'origine déclarés (ex. groupe ethnique du Sénégal mais langue parlée aussi en Gambie) : les pays d'origine de la salle sont une donnée distincte du pays du groupe ethnique et priment pour l'affichage langue → pays.
- Pays archivé ou désactivé dans le référentiel : l'association est conservée mais le pays est **masqué de toute réponse publique** (liste sur la carte + valeurs disponibles dans le filtre) ; côté administration, l'entrée reste visible avec une mention « archivé » pour permettre un nettoyage ciblé.
- Beaucoup de pays associés à une même salle (ex. swahili : 6+ pays) : la carte bascule en mode « drapeaux seuls en ligne » dès le 4ᵉ pays, la liste textuelle complète restant accessible au survol/tooltip (cf. FR-004).
- Suppression définitive d'un pays du référentiel : les associations correspondantes doivent être nettoyées automatiquement pour éviter les liens orphelins.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système MUST permettre d'associer zéro, un ou plusieurs pays d'origine à chaque salle publique Afrolang, indépendamment du pays rattaché au groupe ethnique.
- **FR-002**: Le système MUST garantir l'unicité de l'association (un pays donné ne peut être lié qu'une seule fois à la même salle).
- **FR-003**: L'API publique listant les salles MUST renvoyer pour chaque salle la liste de ses pays d'origine **non archivés** (identifiant, nom, code ISO, drapeau si disponible) dans un ordre stable et prévisible (ordre alphabétique du nom localisé). Les associations vers des pays archivés sont filtrées de la réponse publique.
- **FR-004**: La page `/afrolang` MUST afficher les pays d'origine sur chaque carte de salle publique selon la règle suivante : (a) **1 à 3 pays** → drapeau + nom court pour chacun ; (b) **4 pays ou plus** → drapeaux seuls alignés en ligne (sans nom) avec la liste textuelle complète accessible au survol (tooltip / aria-label) ; (c) **liste vide** → la zone est masquée ou affiche une mention discrète « Pays d'origine à compléter », sans casser la mise en page.
- **FR-005**: La page `/afrolang` MUST proposer un filtre « Pays d'origine » dans le panneau latéral, alimenté par la liste réelle des pays associés à au moins une salle publique active.
- **FR-006**: L'API publique MUST accepter un paramètre de filtre par pays d'origine **mono-valué** (un seul pays par requête) et renvoyer uniquement les salles dont la liste de pays d'origine contient ce pays, en respectant la pagination existante. La multi-sélection est hors périmètre de la v1.
- **FR-007**: L'interface d'administration des salles publiques MUST permettre à un administrateur autorisé d'ajouter et de retirer des pays d'origine pour une salle, en sélectionnant parmi le référentiel des pays existants.
- **FR-008**: Toute modification (ajout/retrait) des pays d'origine d'une salle MUST être enregistrée dans le journal d'audit (qui, quand, salle, pays ajoutés, pays retirés).
- **FR-009**: Les salles publiques existantes au moment du déploiement MUST rester fonctionnelles avec une liste de pays d'origine initialement vide ; aucun pré-remplissage automatique (notamment depuis le pays du groupe ethnique) n'est effectué — l'enrichissement est intégralement manuel via l'administration.
- **FR-010**: La suppression (ou désactivation durable) d'un pays dans le référentiel MUST nettoyer automatiquement les associations correspondantes pour éviter les liens orphelins.
- **FR-011**: Le système MUST réserver la modification des pays d'origine d'une salle aux administrateurs disposant des permissions existantes de gestion des salles Afrolang.

### Key Entities

- **Salle publique Afrolang** : canal d'apprentissage d'une langue africaine, déjà rattaché à un groupe ethnique. Acquiert une nouvelle relation « plusieurs-à-plusieurs » avec les pays d'origine.
- **Pays (référentiel)** : entité existante du référentiel partagé (nom, code ISO, drapeau). Aucune modification de structure, simplement référencée par les salles.
- **Association salle ↔ pays d'origine** : relation porteuse de la date de création (audit) et garantissant l'unicité du couple (salle, pays). Ordre d'affichage déduit du nom du pays.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100 % des salles publiques actives affichent au moins un pays d'origine dans les 30 jours suivant le déploiement (objectif d'enrichissement éditorial).
- **SC-002**: Sur la page `/afrolang`, un visiteur identifie le pays d'origine d'une langue en moins de 3 secondes après l'affichage de la carte (test d'utilisabilité).
- **SC-003**: Le filtrage par pays renvoie les résultats attendus dans 100 % des cas testés et le compteur affiché correspond toujours au nombre réel de salles filtrées.
- **SC-004**: Le temps de chargement de la liste `/afrolang` n'augmente pas de plus de 10 % par rapport à la version actuelle, malgré l'ajout des pays d'origine.
- **SC-005**: Une administratrice peut associer ou retirer un ensemble de pays à une salle en moins de 1 minute, sans formation préalable.
- **SC-006**: 0 association orpheline (lien vers un pays inexistant) après suppression d'un pays du référentiel, vérifié par requête de cohérence.

## Assumptions

- Le référentiel de pays existe déjà (`shared.pays`) et expose nom + code ISO + drapeau ; aucun nouveau référentiel n'est créé.
- L'affichage public reste sur la grille de cartes existante (`AfrolangSalleCard`) ; aucune refonte UX n'est demandée au-delà de l'ajout d'un bandeau « Pays d'origine » et d'un filtre.
- L'autorisation administrative s'appuie sur les permissions existantes de gestion des salles publiques Afrolang, sans nouveau rôle dédié.
- Le filtre pays côté public se limite à un pays sélectionné à la fois pour la première itération (multi-sélection envisageable ultérieurement).
- L'ordre d'affichage des pays sur une carte est l'ordre alphabétique du nom localisé en français.
- La donnée « pays d'origine de la salle » prime sur le pays du groupe ethnique pour tout affichage géographique de la salle.

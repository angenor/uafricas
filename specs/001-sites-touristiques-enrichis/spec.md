# Feature Specification: Enrichissement des sites touristiques

**Feature Branch**: `001-sites-touristiques-enrichis`  
**Created**: 2026-05-25  
**Status**: Draft  
**Input**: User description: "Enrichissement des sites touristiques (emblématiques et privés) de la page opportunité-afrique : sous-types détaillés, badge Vérifié, informations complètes (gestionnaire, localisation, GPS, contacts, constitution légale), avis visiteurs note 1-5"

## Contexte

La page de détail d'un territoire (`/opportunite-afrique/[id]`) présente déjà deux familles de
sites touristiques — **emblématiques** et **privés** — alimentées par des contributions
communautaires validées par un administrateur. Chaque site se limite aujourd'hui à un nom, une
description, une image et des coordonnées GPS.

Cette évolution enrichit ces sites pour qu'ils soient mieux qualifiés, plus fiables et plus
utiles aux visiteurs : classification par sous-type, fiche d'informations complète (gestionnaire,
localisation, GPS, contacts pour les sites privés), badge de fiabilité « Vérifié » attribué par
l'administration, informations de constitution légale, et avis notés (1 à 5) laissés par les
visiteurs.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Classer un site par sous-type (Priority: P1)

Un contributeur propose ou modifie un site et choisit un **sous-type** précis qui décrit sa nature,
parmi une liste adaptée à la famille du site. Les visiteurs voient ce sous-type affiché sur chaque
fiche et peuvent distinguer rapidement la nature des sites.

- Sites emblématiques : plage, monument, relief naturel, parc naturel, mosquée, église, pont,
  route, service public, immeuble / édifice, mer ou rivière, site naturel.
- Sites privés : hôtel, plage privée, espace de jeux, agriculture touristique, résidence
  touristique, restaurant, discothèque (boîte de nuit), bar-maquis.

**Why this priority**: La classification est le socle de toute l'évolution : elle structure
l'affichage, le filtrage et la valeur informative. Sans elle, les autres enrichissements n'ont pas
de cadre.

**Independent Test**: Proposer un site emblématique de sous-type « plage » et un site privé de
sous-type « hôtel », les faire valider, puis vérifier que chaque fiche affiche le bon sous-type et
que les listes peuvent être parcourues par sous-type.

**Acceptance Scenarios**:

1. **Given** un contributeur authentifié sur la fiche d'un territoire, **When** il propose un site
   emblématique et sélectionne un sous-type dans la liste emblématique, **Then** la proposition est
   enregistrée avec ce sous-type et soumise à validation.
2. **Given** un contributeur propose un site privé, **When** il ouvre la liste des sous-types,
   **Then** seuls les sous-types privés (hôtel, plage privée, espace de jeux, etc.) lui sont
   proposés.
3. **Given** une fiche de territoire avec des sites validés, **When** un visiteur consulte la
   section sites touristiques, **Then** chaque carte affiche le sous-type du site.
4. **Given** une liste de sites d'une même famille, **When** le visiteur filtre par sous-type,
   **Then** seuls les sites du sous-type choisi sont affichés.

---

### User Story 2 - Renseigner une fiche d'informations complète (Priority: P1)

Un contributeur renseigne, pour chaque site, les informations minimales requises : nom,
gestionnaire, localisation (ville, village, territoire), localisation GPS et information pertinente
à savoir (particularité du site). Pour un site **privé**, il fournit en plus les **contacts du
gestionnaire** (téléphone, courriel, adresse géographique).

**Why this priority**: Ces informations rendent les fiches réellement exploitables par les
visiteurs et constituent le cœur de la valeur ajoutée demandée. Indissociable de la classification
pour un MVP utile.

**Independent Test**: Proposer un site privé en renseignant tous les champs requis plus les
contacts, le valider, et vérifier que la fiche publique affiche l'ensemble des informations ; tenter
une proposition sans champ requis et vérifier le refus avec message clair.

**Acceptance Scenarios**:

1. **Given** un contributeur remplit le formulaire d'un site, **When** il omet le nom, le
   gestionnaire, la localisation ou l'information pertinente, **Then** la soumission est refusée
   avec un message indiquant les champs manquants.
2. **Given** un site privé, **When** le contributeur soumet sans contact (téléphone, courriel ou
   adresse), **Then** la soumission est refusée car au moins un contact du gestionnaire est requis
   pour un site privé.
3. **Given** un site emblématique, **When** le contributeur soumet sans contacts de gestionnaire,
   **Then** la soumission est acceptée (les contacts ne sont pas requis pour un site emblématique).
4. **Given** un site validé, **When** un visiteur ouvre sa fiche, **Then** il voit le nom, le
   gestionnaire, la localisation (ville, village, territoire), la position GPS et l'information
   pertinente ; pour un site privé, il voit aussi les contacts du gestionnaire.

---

### User Story 3 - Marquer un site comme « Vérifié » depuis l'administration (Priority: P2)

Un administrateur peut attribuer (ou retirer) un badge **« Vérifié »** à un site emblématique ou
privé depuis le back-office. Le badge est visible par tous les visiteurs sur la fiche et la carte
du site, signalant que l'information a été contrôlée.

**Why this priority**: Le badge renforce la confiance mais dépend de l'existence des fiches
enrichies (US1/US2). Il peut être livré dans un second temps sans bloquer la valeur de base.

**Independent Test**: Depuis l'administration, marquer un site comme « Vérifié », puis confirmer
que le badge apparaît côté public ; le retirer et confirmer sa disparition.

**Acceptance Scenarios**:

1. **Given** un administrateur dans le back-office, **When** il active le badge « Vérifié » sur un
   site, **Then** la fiche publique de ce site affiche le badge « Vérifié ».
2. **Given** un site marqué « Vérifié », **When** l'administrateur retire le badge, **Then** le
   badge disparaît de la fiche publique.
3. **Given** un visiteur non administrateur, **When** il consulte un site, **Then** il ne dispose
   d'aucun moyen d'attribuer ou retirer le badge.
4. **Given** une liste de sites, **When** un visiteur la consulte, **Then** les sites vérifiés sont
   identifiables (badge visible) et distinguables des sites non vérifiés.

---

### User Story 4 - Partager les informations de constitution légale (Priority: P3)

Un contributeur (ou le gestionnaire) peut renseigner les **informations de constitution légale** du
site (statut / forme juridique, numéro d'enregistrement, et le cas échéant un document
justificatif). Ces informations sont affichées sur la fiche pour appuyer la fiabilité du site.

**Why this priority**: Utile à la crédibilité, surtout pour les sites privés, mais facultatif et
non bloquant pour l'usage courant.

**Independent Test**: Renseigner les informations de constitution légale d'un site privé, valider,
et vérifier leur affichage ; vérifier qu'un site sans ces informations reste valide et affichable.

**Acceptance Scenarios**:

1. **Given** un contributeur édite un site, **When** il renseigne le statut juridique et le numéro
   d'enregistrement, **Then** ces informations sont enregistrées avec la proposition.
2. **Given** un site avec informations légales validées, **When** un visiteur ouvre la fiche,
   **Then** une section « Constitution légale » présente ces informations.
3. **Given** un site sans informations légales, **When** un visiteur ouvre la fiche, **Then** la
   section « Constitution légale » n'est pas affichée et le site reste pleinement consultable.

---

### User Story 5 - Donner et consulter des avis de visiteurs notés (Priority: P2)

Un visiteur authentifié laisse un **avis noté de 1 à 5** (avec commentaire) sur un site donné. La
fiche du site affiche la **note moyenne**, le **nombre d'avis** et la liste des avis. Chaque visiteur
ne peut avoir qu'un avis actif par site et peut le mettre à jour.

**Why this priority**: Le retour des visiteurs augmente fortement la valeur perçue et la confiance,
mais peut être ajouté après les fiches enrichies.

**Independent Test**: Déposer une note 4 avec commentaire sur un site, vérifier que la moyenne et le
compteur se mettent à jour, modifier sa note en 5 et vérifier que la moyenne reflète la mise à jour
sans créer de doublon.

**Acceptance Scenarios**:

1. **Given** un visiteur authentifié sur la fiche d'un site, **When** il attribue une note de 1 à 5
   avec un commentaire, **Then** l'avis est enregistré et compté dans la note moyenne du site.
2. **Given** un visiteur ayant déjà laissé un avis sur un site, **When** il soumet un nouvel avis sur
   le même site, **Then** son avis existant est mis à jour (pas de second avis).
3. **Given** un visiteur non authentifié, **When** il tente de noter un site, **Then** il est invité
   à se connecter.
4. **Given** un site avec plusieurs avis, **When** un visiteur consulte la fiche, **Then** il voit la
   note moyenne (sur 5), le nombre total d'avis et le détail des avis.
5. **Given** une note hors plage (0 ou 6) ou un commentaire vide, **When** le visiteur soumet,
   **Then** l'avis est refusé avec un message explicite.

---

### Edge Cases

- **Sous-type incohérent avec la famille** : un sous-type privé (ex. « hôtel ») soumis pour un site
  classé emblématique doit être refusé (et inversement).
- **Migration des sites existants** : les sites déjà enregistrés sans sous-type, sans gestionnaire
  ni information pertinente doivent rester affichables ; ces champs sont à compléter via une
  modification, sans casser l'affichage existant.
- **Contacts d'un site privé** : si aucun contact n'est fourni à la création d'un site privé, la
  soumission est refusée.
- **Note moyenne sans avis** : un site sans avis affiche « aucun avis » plutôt qu'une moyenne de 0.
- **Badge sur site supprimé** : un site supprimé (soft delete) ne doit plus afficher son badge ni
  accepter d'avis.
- **Modération d'un avis abusif** : un administrateur doit pouvoir masquer un avis inapproprié.

## Requirements *(mandatory)*

### Functional Requirements

#### Classification (sous-types)

- **FR-001**: Le système DOIT permettre de classer chaque site emblématique selon l'un des
  sous-types suivants : plage, monument, relief naturel, parc naturel, mosquée, église, pont, route,
  service public, immeuble / édifice, mer ou rivière, site naturel.
- **FR-002**: Le système DOIT permettre de classer chaque site privé selon l'un des sous-types
  suivants : hôtel, plage privée, espace de jeux, agriculture touristique, résidence touristique,
  restaurant, discothèque (boîte de nuit), bar-maquis.
- **FR-003**: Le système DOIT garantir la cohérence entre la famille du site (emblématique / privé)
  et le sous-type choisi, en refusant un sous-type qui n'appartient pas à la famille.
- **FR-004**: Le système DOIT afficher le sous-type sur chaque fiche et carte de site, et permettre
  aux visiteurs de filtrer les sites d'une famille par sous-type.

#### Informations requises

- **FR-005**: Le système DOIT exiger, pour tout site, les informations minimales suivantes : nom,
  gestionnaire, localisation textuelle (ville, village, territoire), localisation GPS (latitude /
  longitude) et information pertinente à savoir (particularité du site).
- **FR-006**: Le système DOIT exiger, pour tout site **privé**, au moins un contact du gestionnaire
  parmi : numéro de téléphone, adresse de courriel, adresse géographique.
- **FR-007**: Le système NE DOIT PAS exiger les contacts du gestionnaire pour un site emblématique
  (champs facultatifs).
- **FR-008**: Le système DOIT refuser toute proposition ou modification ne respectant pas les champs
  requis, avec un message indiquant les champs manquants.
- **FR-009**: Le système DOIT afficher l'ensemble des informations renseignées sur la fiche publique
  d'un site validé, y compris les contacts du gestionnaire et les informations de constitution
  légale, accessibles à tout visiteur (voir FR-016).

#### Badge « Vérifié »

- **FR-010**: Le système DOIT permettre à un administrateur d'attribuer ou de retirer un badge
  « Vérifié » sur un site emblématique ou privé depuis le back-office.
- **FR-011**: Le système DOIT afficher le badge « Vérifié » sur la fiche et la carte des sites
  vérifiés, visible par tous les visiteurs.
- **FR-012**: Le système NE DOIT PAS permettre à un utilisateur non administrateur d'attribuer ou de
  retirer le badge « Vérifié ».

#### Constitution légale

- **FR-013**: Le système DOIT permettre de renseigner, de façon facultative, les informations de
  constitution légale d'un site (forme / statut juridique, numéro d'enregistrement et, le cas
  échéant, un document justificatif).
- **FR-014**: Le système DOIT afficher les informations de constitution légale sur la fiche
  lorsqu'elles sont présentes, et masquer la section lorsqu'elles sont absentes.

#### Avis visiteurs (note 1–5)

- **FR-015**: Le système DOIT permettre à un visiteur authentifié de laisser un avis noté de 1 à 5
  assorti d'un commentaire sur un site donné, applicable aux sites emblématiques comme privés.
- **FR-015a**: Le système DOIT limiter chaque visiteur à un avis actif par site et permettre la mise
  à jour de cet avis (sans créer de doublon).
- **FR-015b**: Le système DOIT calculer et afficher, pour chaque site, la note moyenne (sur 5) et le
  nombre total d'avis, et afficher « aucun avis » lorsqu'aucun avis n'existe.
- **FR-015c**: Le système DOIT rejeter une note hors de la plage 1–5 ou un commentaire vide, avec un
  message explicite.
- **FR-015d**: Le système DOIT permettre à un administrateur de masquer un avis inapproprié.

#### Visibilité, workflow et données

- **FR-016**: Le système DOIT rendre les contacts du gestionnaire et les informations de
  constitution légale consultables par tout visiteur (publics, sans authentification requise).
- **FR-017**: Le système DOIT faire transiter les ajouts et modifications de sites par le circuit de
  contribution existant (proposition → validation par un administrateur), de façon cohérente avec le
  fonctionnement actuel de la page.
- **FR-018**: Le système DOIT conserver l'affichage des sites existants dépourvus des nouveaux champs
  (rétrocompatibilité), ces champs pouvant être complétés ultérieurement.

### Key Entities *(include if feature involves data)*

- **Site touristique** : représente un lieu emblématique ou privé d'un territoire. Attributs :
  famille (emblématique / privé), sous-type (selon la famille), nom, gestionnaire, localisation
  textuelle (ville, village, territoire), localisation GPS (latitude, longitude), information
  pertinente, contacts du gestionnaire (téléphone, courriel, adresse — requis si privé),
  informations de constitution légale (facultatives), indicateur « Vérifié », image.
- **Avis de visiteur sur un site** : note de 1 à 5 et commentaire laissés par un visiteur
  authentifié sur un site. Un avis actif au plus par couple (visiteur, site). Relié au site et à
  l'auteur. Peut être masqué par un administrateur.
- **Informations de constitution légale** : forme / statut juridique, numéro d'enregistrement,
  document justificatif éventuel — rattachées à un site.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100 % des sites nouvellement proposés portent un sous-type valide cohérent avec leur
  famille.
- **SC-002**: Un visiteur peut identifier la nature (sous-type), la localisation et — pour un site
  privé — un moyen de contact d'un site en moins de 10 secondes de consultation de la fiche.
- **SC-003**: 100 % des sites privés validés disposent d'au moins un contact de gestionnaire
  renseigné.
- **SC-004**: Un administrateur peut attribuer ou retirer le badge « Vérifié » sur un site en moins
  de 30 secondes, et le changement est visible côté public immédiatement après validation.
- **SC-005**: La note moyenne et le nombre d'avis affichés correspondent exactement aux avis actifs,
  et la soumission d'un nouvel avis par un même visiteur ne crée jamais de doublon.
- **SC-006**: Les sites existants avant l'évolution restent affichables à 100 % sans erreur après
  déploiement.

## Assumptions

- Les ajouts/modifications de sites suivent le circuit de contribution communautaire existant
  (proposition par un membre authentifié, validation par un administrateur), comme les autres
  sections enrichies de la page.
- Les avis de visiteurs portent sur un **site** (granularité site), distincts des recommandations
  existantes au niveau du **territoire**.
- Un avis nécessite un compte authentifié ; un même visiteur dispose d'au plus un avis actif par
  site, modifiable.
- Le badge « Vérifié » est un attribut booléen géré exclusivement par l'administration ; il ne
  modifie pas le contenu du site.
- Les informations de constitution légale sont facultatives et n'empêchent jamais la publication
  d'un site.
- La localisation « territoire » correspond au territoire (fiche pays) auquel le site est rattaché ;
  ville et village sont des précisions textuelles saisies par le contributeur.

## Clarifications

### Session 2026-05-25

- **Q : Qui peut consulter les contacts du gestionnaire (téléphone, courriel, adresse) et les
  informations de constitution légale d'un site privé ?**
  → **R : Tous (public).** Ces informations sont consultables par tout visiteur, sans
  authentification requise. (Voir FR-009, FR-016.)

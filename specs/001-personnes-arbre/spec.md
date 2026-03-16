# Feature Specification: Modèle de données des personnes et liens familiaux

**Feature Branch**: `001-personnes-arbre`
**Created**: 2026-03-15
**Status**: Draft
**Input**: User description: "Feature 1 — Modèle de données des personnes et liens familiaux. CRUD des personnes (nom, prénoms, date/lieu de naissance, date/lieu de décès, genre, photo). Relations parent-enfant et conjoint. Chaque personne est rattachée à l'arbre de l'utilisateur qui l'a créée. Point crucial : prévoir dès cette feature que plusieurs utilisateurs peuvent référencer la même personne réelle (un ancêtre commun), car c'est la base du matching. Chaque utilisateur a son propre arbre, mais le modèle doit supporter les futures connexions."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Créer une fiche personne dans son arbre (Priority: P1)

Un utilisateur authentifié souhaite ajouter une personne à son arbre généalogique. Il saisit les informations connues (nom obligatoire, prénoms, genre, date et lieu de naissance, date et lieu de décès, photo). La personne est immédiatement disponible dans son arbre.

**Why this priority**: C'est la brique fondamentale — sans personne, aucune autre fonctionnalité n'est possible. Toute la valeur de la plateforme repose sur la capacité à constituer un arbre.

**Independent Test**: Peut être testée seule en vérifiant qu'un utilisateur peut créer, consulter, modifier et supprimer une personne dans son arbre, sans aucune relation.

**Acceptance Scenarios**:

1. **Given** un utilisateur authentifié sans entrées dans son arbre, **When** il soumet une fiche avec nom, prénoms et genre, **Then** la personne apparaît dans son arbre avec les données saisies et une date de création.
2. **Given** un utilisateur authentifié, **When** il soumet une fiche avec uniquement le nom (champ minimal), **Then** la personne est créée avec les champs facultatifs vides.
3. **Given** un utilisateur authentifié, **When** il tente de créer une personne sans nom, **Then** le système rejette la création avec un message d'erreur explicite.
4. **Given** une personne existante dans son arbre, **When** l'utilisateur modifie son lieu de naissance, **Then** la modification est sauvegardée et visible immédiatement.
5. **Given** une personne sans aucun lien familial, **When** l'utilisateur la supprime, **Then** la personne disparaît de son arbre.

---

### User Story 2 - Créer des liens familiaux entre personnes (Priority: P2)

Un utilisateur souhaite relier deux personnes déjà présentes dans son arbre. Il peut établir un lien parent-enfant (en précisant le rôle : père, mère, ou parent non précisé) ou un lien conjoint entre elles.

**Why this priority**: Les liens donnent du sens aux personnes isolées et constituent la structure de l'arbre. Sans liens, l'arbre n'est qu'une liste — pas un arbre généalogique.

**Independent Test**: Peut être testée en créant deux personnes, en les reliant, puis en vérifiant que la relation est visible et correctement typée depuis les deux côtés.

**Acceptance Scenarios**:

1. **Given** deux personnes dans l'arbre, **When** l'utilisateur crée un lien "père-enfant" (A est père de B), **Then** A apparaît comme père de B et B apparaît comme enfant de A.
2. **Given** deux personnes dans l'arbre, **When** l'utilisateur crée un lien "conjoint" entre elles, **Then** chacune référence l'autre comme conjoint.
3. **Given** deux personnes déjà liées par un lien "père-enfant", **When** l'utilisateur tente de créer un second lien "père-enfant" identique, **Then** le système rejette la duplication.
4. **Given** un lien familial existant, **When** l'utilisateur le supprime, **Then** la relation disparaît des deux côtés.
5. **Given** une relation circulaire potentielle (A parent de B, B parent de A), **When** l'utilisateur tente de la créer, **Then** le système la refuse avec un message explicite.

---

### User Story 3 - Visualiser les personnes et relations de son arbre (Priority: P3)

Un utilisateur souhaite consulter la liste des personnes de son arbre et, pour chaque personne, voir ses liens familiaux directs (parents, enfants, conjoints).

**Why this priority**: La consultation est indispensable pour utiliser l'arbre et préparer l'affichage graphique futur. Elle permet aussi de valider les données saisies.

**Independent Test**: Peut être testée en vérifiant que la liste retourne toutes les personnes de l'utilisateur (et uniquement les siennes), et que le détail d'une personne liste ses relations directes.

**Acceptance Scenarios**:

1. **Given** un arbre avec 5 personnes, **When** l'utilisateur consulte la liste, **Then** il voit exactement ses 5 personnes et aucune personne appartenant à d'autres arbres.
2. **Given** une personne avec 2 parents et 1 enfant, **When** l'utilisateur consulte sa fiche, **Then** les 3 relations directes sont affichées avec leur type.
3. **Given** un utilisateur sans aucune personne dans son arbre, **When** il consulte la liste, **Then** le système affiche un état vide explicite (et non une erreur).

---

### User Story 4 - Référencer une personne réelle partagée entre arbres (Priority: P4)

Dans cette feature, chaque utilisateur crée ses propres fiches indépendamment — il n'y a pas d'interface de recherche cross-users. Le modèle doit cependant être structuré de manière à ce que la déduplication et le rattachement entre fiches représentant la même personne réelle puissent être implémentés dans une feature de matching ultérieure, sans migration de schéma.

**Why this priority**: C'est la contrainte architecturale fondamentale de la plateforme. Si le modèle ne la supporte pas dès le départ, une migration coûteuse sera nécessaire plus tard. Elle n'est pas visible par l'utilisateur dans cette feature, mais conditionne toutes les suivantes.

**Independent Test**: Peut être testée en vérifiant que deux utilisateurs distincts peuvent avoir dans leur arbre respectif une référence à la même fiche personne réelle, chacune avec ses propres métadonnées de rattachement.

**Acceptance Scenarios**:

1. **Given** une personne réelle documentée par l'utilisateur A, **When** l'utilisateur B référence la même personne dans son arbre, **Then** les deux arbres pointent vers la même fiche personne réelle sans conflit.
2. **Given** deux utilisateurs référençant la même personne réelle, **When** l'utilisateur A supprime son entrée dans son arbre, **Then** la personne reste présente dans l'arbre de l'utilisateur B.
3. **Given** deux arbres référençant la même personne réelle, **When** une requête recherche les personnes partagées entre ces deux arbres, **Then** la personne commune est identifiable directement.

---

### Edge Cases

- Que se passe-t-il si l'utilisateur supprime une personne référencée dans d'autres arbres ? Seul le rattachement de son arbre est supprimé ; la Personne réelle et ses rattachements dans les autres arbres sont préservés. Si c'est le dernier rattachement, la Personne réelle est supprimée en cascade (soft delete).
- Comment gérer une personne avec plusieurs conjoints successifs ? Le modèle doit permettre plusieurs liens "conjoint" entre personnes différentes.
- Une personne peut-elle apparaître deux fois dans le même arbre ? Non — chaque utilisateur ne peut référencer une personne réelle qu'une seule fois dans son arbre.
- Que se passe-t-il si on tente de créer un lien entre des personnes appartenant à deux arbres différents ? Seuls les liens au sein d'un même arbre sont autorisés dans cette feature.
- Comment gérer une date de naissance postérieure à la date de décès ? Le système compare les composantes disponibles (ex : si seule l'année est connue, compare les années) ; si la comparaison n'est pas concluante avec les données partielles fournies, la saisie est acceptée sans erreur.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT permettre à un utilisateur authentifié de créer une fiche personne avec au minimum un nom de famille.
- **FR-002**: Une fiche personne DOIT pouvoir contenir les champs suivants : nom de famille (obligatoire), prénoms (facultatif), genre (facultatif : masculin / féminin / autre / non précisé), date de naissance à granularité variable (facultatif : année seule, mois + année, ou date complète JJ/MM/AAAA — chaque composante stockée séparément), lieu de naissance (facultatif), date de décès à granularité variable (facultatif, mêmes règles), lieu de décès (facultatif), photo (facultative).
- **FR-003**: Le système DOIT rejeter toute fiche personne soumise sans nom de famille, avec un message d'erreur explicite.
- **FR-004**: Le système DOIT rejeter une date de décès antérieure à la date de naissance lorsque la comparaison est possible (ex : si seules les années sont fournies, comparer les années ; si les composantes disponibles ne permettent pas de conclure, accepter sans erreur).
- **FR-005**: Un utilisateur DOIT pouvoir modifier les informations d'une fiche personne qu'il a créée. Dans cette feature, chaque utilisateur n'accède et ne modifie que ses propres fiches — aucune édition cross-users n'est exposée.
- **FR-006**: Un utilisateur DOIT pouvoir retirer une personne de son arbre. La suppression du rattachement entraîne en cascade la suppression (soft delete) de la Personne réelle si elle n'est plus rattachée à aucun autre arbre. Les liens familiaux associés au rattachement supprimé sont également nettoyés.
- **FR-007**: Le système DOIT permettre de créer un lien parent-enfant entre deux personnes du même arbre, avec un rôle parental : père, mère, ou parent non précisé.
- **FR-008**: Le système DOIT permettre de créer un lien conjoint entre deux personnes du même arbre.
- **FR-009**: Le système DOIT empêcher la création de liens circulaires dans la hiérarchie parent-enfant (ex : A ancêtre de B et B ancêtre de A).
- **FR-010**: Le système DOIT empêcher la duplication d'un lien identique (même type, mêmes deux personnes).
- **FR-011**: Un utilisateur DOIT pouvoir supprimer un lien familial entre deux personnes de son arbre.
- **FR-012**: Un utilisateur DOIT pouvoir consulter la liste paginée des personnes de son arbre (et uniquement les siennes).
- **FR-013**: Un utilisateur DOIT pouvoir consulter la fiche complète d'une personne de son arbre, incluant ses liens familiaux directs (parents, enfants, conjoints).
- **FR-014**: Le modèle DOIT distinguer la personne réelle (entité potentiellement partageable entre arbres) du rattachement à un arbre spécifique (entité propre à l'utilisateur). Dans cette feature, chaque utilisateur crée ses propres fiches — le mécanisme de déduplication et de rattachement cross-users est réservé à la feature de matching.
- **FR-015**: Le système DOIT garantir qu'un utilisateur ne peut rattacher la même personne réelle qu'une seule fois à son arbre.
- **FR-016**: La suppression d'un rattachement NE DOIT PAS supprimer la Personne réelle si elle est encore rattachée à au moins un autre arbre. Si c'est le dernier rattachement, la Personne réelle est supprimée en cascade (soft delete) avec ses liens familiaux.

### Key Entities

- **Personne réelle** : représente une personne réelle unique, indépendante de tout arbre. Attributs : nom de famille, prénoms, genre, date de naissance, lieu de naissance, date de décès, lieu de décès, photo. Peut être référencée par plusieurs arbres. C'est l'entité partageable qui permettra le matching entre utilisateurs.

- **Arbre généalogique** : appartient à un utilisateur. Sert de conteneur logique pour toutes les personnes documentées par cet utilisateur et leurs relations entre elles.

- **Rattachement (nœud d'arbre)** : lie une Personne réelle à un arbre spécifique. Mémorise qui a ajouté cette personne à cet arbre et quand. C'est le "point de vue" d'un utilisateur sur une personne réelle. Une Personne réelle peut avoir plusieurs Rattachements dans des arbres différents, mais un seul par arbre.

- **Lien familial** : relation typée entre deux Rattachements au sein du même arbre. Types : parent-enfant (avec rôle : père / mère / parent non précisé) ou conjoint. Un lien est bidirectionnel : si A est père de B, B est enfant de A.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un utilisateur peut créer une fiche personne complète (tous champs remplis, avec photo) en moins de 2 minutes.
- **SC-002**: Un utilisateur peut créer un lien familial entre deux personnes existantes en moins de 30 secondes.
- **SC-003**: La consultation de la liste des personnes d'un arbre contenant jusqu'à 500 entrées s'affiche en moins de 1 seconde. Il n'y a pas de limite maximale sur le nombre de personnes par arbre — la pagination gère les volumes supérieurs.
- **SC-004**: Zéro perte de données lors de la suppression d'un rattachement — les liens associés sont nettoyés et les personnes partagées sont préservées.
- **SC-005**: Zéro lien circulaire possible — toute tentative est refusée avant enregistrement.
- **SC-006**: Le modèle permet d'identifier les personnes partagées entre deux arbres distincts par requête directe, sans migration ni modification de structure.
- **SC-007**: 100 % des règles de validation (nom obligatoire, cohérence des dates, unicité du rattachement, absence de doublons de liens) sont appliquées côté serveur, indépendamment de l'interface cliente.

## Assumptions

- **Isolation complète dans cette feature** : chaque utilisateur ne travaille que sur ses propres fiches. Il n'y a pas de partage effectif de fiches entre utilisateurs dans cette feature. La question des droits de modification sur une Personne réelle partagée (propriété, conflits d'édition, notifications) est entièrement déférée à la feature de matching.
- **Upload de photo** : le stockage des photos réutilise l'infrastructure d'upload existante du projet (mêmes contraintes de taille et de format).
- **Pagination** : la liste des personnes d'un arbre est paginée selon les conventions existantes du projet.
- **Suppression logique en cascade** : la suppression d'un rattachement utilise un soft delete. Si c'est le dernier rattachement de la Personne réelle, celle-ci est également soft-deleted en cascade, ainsi que ses liens familiaux.
- **Lien conjoint non daté** : dans cette feature, un lien conjoint ne comporte pas de date de mariage ou de séparation. Ces attributs pourront être ajoutés ultérieurement.
- **Visibilité privée** : toutes les personnes et relations d'un arbre sont privées (visibles uniquement par leur propriétaire). La gestion de la visibilité publique ou du partage est hors scope de cette feature.
- **Arbre unique par utilisateur** : chaque utilisateur possède un seul arbre généalogique. La gestion de plusieurs arbres par utilisateur est hors scope.

## Clarifications

### Session 2026-03-15

- Q: Comment l'utilisateur B ajoute-t-il à son arbre une personne réelle déjà documentée par l'utilisateur A ? → A: Chaque utilisateur crée sa propre fiche indépendamment. La déduplication et le rattachement à une personne réelle commune se font via le matching dans une feature ultérieure. Dans cette feature, le modèle prépare la structure mais n'expose pas le référencement cross-users à l'utilisateur.
- Q: Quel est le régime de propriété et de modification des attributs de la Personne réelle partagée ? → A: Dans cette feature, chaque utilisateur ne travaille que sur ses propres fiches (pas de partage réel). La propriété partagée et les droits d'édition cross-users sont entièrement déférés à la feature de matching.
- Q: Quelle est la granularité acceptée pour saisir une date de naissance ou de décès ? → A: Saisie partielle acceptée — année seule (1850), mois + année (03/1850), ou date complète (15/03/1850). Chaque composante est stockée séparément.
- Q: Que fait le système d'une Personne réelle qui n'est plus rattachée à aucun arbre ? → A: Suppression en cascade (soft delete) — quand le dernier rattachement est supprimé, la Personne réelle est automatiquement marquée supprimée. Pas de données orphelines.
- Q: Faut-il imposer une limite au nombre de personnes par arbre ? → A: Aucune limite imposée — l'arbre est délibérément illimité. SC-003 (500 entrées affichées en moins de 1 seconde) reste la cible de performance de référence ; la pagination gère les grands volumes.

## Dependencies

- Authentification utilisateur fonctionnelle (l'identité de l'utilisateur connecté est nécessaire pour rattacher les personnes à son arbre).
- Infrastructure d'upload de fichiers existante (pour les photos des personnes).
- Architecture multi-schémas PostgreSQL du projet (le nouveau modèle s'intègre dans les schémas bounded-context existants).

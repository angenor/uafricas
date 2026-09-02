# Feature Specification: Ajout de personne ludique

**Feature Branch**: `001-ajout-personne-ludique`  
**Created**: 2026-04-06  
**Status**: Draft  
**Input**: User description: "Le client veut que l'ajout de nouvelles personnes à son arbre généalogique soit plus ludique. Une suite de questions avec textes et animations amusantes et intéressantes, GSAP est déjà installé."

## Clarifications

### Session 2026-04-06

- Q: Mode de présentation du parcours (modal compact, overlay plein écran, ou page dédiée) ? → A: Overlay plein écran : le parcours occupe tout l'écran avec un fond opaque ou semi-transparent, offrant un canvas optimal pour les animations.
- Q: Quelles pages sont concernées (index seule, index + visualisation, ou toutes) ? → A: Les deux pages : index.vue (ajout libre) et visualisation.vue (ajout contextuel parent/enfant/conjoint depuis un noeud). Le parcours ludique s'adapte au contexte d'origine.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Parcours guidé pas-à-pas pour ajouter une personne (Priority: P1)

L'utilisateur clique sur "Ajouter une personne" et au lieu d'un formulaire classique, un overlay plein écran s'ouvre avec un assistant ludique qui lui pose des questions une par une, avec des transitions animées entre chaque étape. Chaque question est formulée de manière chaleureuse et engageante (ex: "Comment s'appelle ce membre de votre famille ?" plutôt que "Nom"). L'utilisateur répond à chaque question, voit une animation de transition vers la question suivante, et peut revenir en arrière à tout moment. À la fin, un récapitulatif animé apparaît avant la validation finale. Ce parcours est disponible depuis la page liste (ajout libre) et depuis la page visualisation (ajout contextuel lié à un noeud existant, parent, enfant ou conjoint).

**Why this priority**: C'est le coeur de la demande : transformer l'expérience d'ajout en parcours ludique. Sans ce parcours étape par étape, la feature n'existe pas.

**Independent Test**: Peut être testé en cliquant sur "Ajouter une personne" et en parcourant toutes les étapes jusqu'à la soumission. Délivre la valeur principale : une expérience d'ajout engageante et amusante.

**Acceptance Scenarios**:

1. **Given** l'utilisateur est sur la page liste de l'arbre généalogique, **When** il clique sur "Ajouter une personne", **Then** un overlay plein écran s'ouvre avec la première question animée (le nom de famille)
2. **Given** l'utilisateur est sur la page visualisation et sélectionne un noeud, **When** il clique sur "Ajouter un parent/enfant/conjoint", **Then** l'overlay plein écran s'ouvre avec un texte d'accroche contextuel mentionnant le lien familial choisi et la personne liée
2. **Given** l'utilisateur a répondu à une question, **When** il passe à la question suivante, **Then** une animation de transition fluide se joue entre les deux étapes
3. **Given** l'utilisateur est à l'étape 3, **When** il clique sur "Retour", **Then** il revient à l'étape précédente avec une animation inverse, ses réponses étant conservées
4. **Given** l'utilisateur a rempli toutes les étapes, **When** il arrive au récapitulatif, **Then** il voit un résumé animé de toutes ses réponses avec un bouton de validation
5. **Given** l'utilisateur est sur le récapitulatif, **When** il valide, **Then** la personne est créée et une animation de célébration se joue

---

### User Story 2 - Textes engageants et contextuels à chaque étape (Priority: P1)

Chaque étape du parcours affiche un texte d'accroche chaleureux, humain et culturellement pertinent. Les formulations sont amicales et encourageantes ("Magnifique ! Et quel est son prénom ?" après avoir saisi le nom). Des emojis ou icônes subtiles accompagnent les textes pour renforcer le côté ludique. Les textes varient légèrement selon le contexte (ex: si la personne est décédée, le ton devient respectueux et bienveillant).

**Why this priority**: Les textes sont indissociables du parcours - c'est ce qui rend l'expérience véritablement ludique et chaleureuse, pas juste un formulaire découpé en étapes.

**Independent Test**: Parcourir chaque étape et vérifier que les textes sont engageants, que le ton s'adapte selon les réponses données, et que l'ensemble est cohérent.

**Acceptance Scenarios**:

1. **Given** l'utilisateur arrive sur la première étape, **When** la question s'affiche, **Then** le texte est chaleureux et invite à saisir le nom de famille (pas un simple label "Nom")
2. **Given** l'utilisateur a indiqué que la personne est décédée, **When** les étapes suivantes s'affichent, **Then** le ton est respectueux et bienveillant
3. **Given** l'utilisateur passe d'une étape à l'autre, **When** chaque question apparaît, **Then** elle est précédée d'un petit texte de transition encourageant qui prend en compte la réponse précédente

---

### User Story 3 - Animations GSAP fluides et performantes (Priority: P2)

Les animations entre les étapes sont fluides, variées et performantes. Elles incluent : apparition/disparition des questions avec slide et fade, micro-animations sur les éléments interactifs (boutons, champs), animation d'entrée de l'assistant, animation de célébration à la fin. Les animations ne bloquent pas l'interaction utilisateur et peuvent être interrompues si l'utilisateur navigue rapidement.

**Why this priority**: Les animations renforcent l'aspect ludique mais le parcours fonctionne même avec des animations minimales. Elles sont néanmoins essentielles pour l'expérience souhaitée.

**Independent Test**: Parcourir le flux complet en vérifiant la fluidité des transitions, l'absence de saccades, et le comportement lors de navigation rapide entre étapes.

**Acceptance Scenarios**:

1. **Given** l'utilisateur passe à l'étape suivante, **When** la transition se joue, **Then** l'ancienne question disparaît (slide + fade out) et la nouvelle apparaît (slide + fade in) de manière fluide
2. **Given** l'utilisateur navigue rapidement entre les étapes, **When** il clique plusieurs fois "Suivant" ou "Retour", **Then** les animations s'interrompent proprement sans artefact visuel
3. **Given** l'utilisateur arrive sur l'écran final de validation, **When** la personne est créée avec succès, **Then** une animation de célébration se joue (confettis, check animé, ou effet similaire)
4. **Given** l'assistant s'ouvre, **When** la première question apparaît, **Then** l'entrée est animée de manière engageante (scale + fade ou effet similaire)

---

### User Story 4 - Indicateur de progression visuel (Priority: P2)

L'utilisateur voit à tout moment où il en est dans le parcours grâce à un indicateur de progression (barre, dots, ou étapes numérotées). Cet indicateur est lui-même animé et se met à jour en temps réel lors de la navigation.

**Why this priority**: L'indicateur de progression rassure l'utilisateur sur la durée du parcours et l'encourage à compléter toutes les étapes.

**Independent Test**: Vérifier que l'indicateur reflète correctement l'étape courante, qu'il s'anime lors des transitions, et qu'il fonctionne avec la navigation avant/arrière.

**Acceptance Scenarios**:

1. **Given** l'utilisateur est à l'étape 2 sur 6, **When** il regarde l'indicateur, **Then** il voit clairement qu'il est à l'étape 2 et combien d'étapes restent
2. **Given** l'utilisateur avance d'une étape, **When** la transition se joue, **Then** l'indicateur s'anime pour refléter la progression
3. **Given** l'utilisateur revient en arrière, **When** la transition se joue, **Then** l'indicateur recule visuellement de manière cohérente

---

### User Story 5 - Compatibilité avec le formulaire existant (Priority: P3)

Le parcours ludique collecte exactement les mêmes informations que le formulaire actuel (nom, prénoms, genre, année de naissance, lieu de naissance, statut vital). Les données sont soumises au même endpoint. Le formulaire classique reste accessible comme alternative (ex: lien "Remplir le formulaire classique") pour les utilisateurs pressés ou en situation d'accessibilité.

**Why this priority**: Assure la compatibilité avec le système existant et offre une alternative pour les utilisateurs qui préfèrent l'ancien mode.

**Independent Test**: Compléter le parcours ludique puis vérifier que les données créées sont identiques à celles du formulaire classique. Vérifier aussi que le lien vers le formulaire classique fonctionne.

**Acceptance Scenarios**:

1. **Given** l'utilisateur complète le parcours ludique, **When** il valide, **Then** la personne créée contient les mêmes champs que via le formulaire classique
2. **Given** l'utilisateur préfère le formulaire classique, **When** il clique sur "Formulaire rapide", **Then** le formulaire actuel s'affiche dans un modal comme avant

---

### Edge Cases

- Que se passe-t-il si l'utilisateur ferme le parcours en cours de route ? Les réponses partielles sont perdues (pas de sauvegarde intermédiaire nécessaire).
- Que se passe-t-il si l'utilisateur ne remplit que le nom (seul champ obligatoire) et saute les étapes optionnelles ? Il doit pouvoir valider avec seulement le nom.
- Comment le parcours se comporte-t-il sur mobile ? Les animations doivent rester fluides et le parcours doit être utilisable en plein écran.
- Que se passe-t-il en cas d'erreur réseau lors de la soumission ? Un message d'erreur clair s'affiche avec possibilité de réessayer sans perdre les données saisies.
- Que se passe-t-il si l'utilisateur entre une année de naissance invalide ? La validation s'affiche à l'étape concernée avant de pouvoir avancer.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT proposer un parcours d'ajout en étapes séquentielles dans un overlay plein écran, chaque étape correspondant à un champ du formulaire (nom, prénoms, genre, année de naissance, lieu de naissance, statut vital)
- **FR-002**: Le système DOIT afficher des textes d'accroche chaleureux et engageants à chaque étape, adaptés au contexte des réponses précédentes
- **FR-003**: Le système DOIT animer les transitions entre étapes avec des animations fluides (slide, fade, scale)
- **FR-004**: L'utilisateur DOIT pouvoir naviguer librement entre les étapes (suivant/retour) sans perdre ses réponses
- **FR-005**: Le système DOIT afficher un indicateur de progression visuel animé indiquant l'étape courante et le nombre total d'étapes
- **FR-006**: Le système DOIT afficher un écran récapitulatif avant la validation finale, montrant toutes les réponses saisies
- **FR-007**: Le système DOIT jouer une animation de célébration après la création réussie d'une personne
- **FR-008**: Le système DOIT permettre de sauter les étapes optionnelles (tout sauf le nom) via un bouton "Passer"
- **FR-009**: Le système DOIT valider les champs à chaque étape avant de permettre la navigation vers l'étape suivante (ex: année de naissance valide)
- **FR-010**: Le système DOIT offrir un lien vers le formulaire classique comme alternative au parcours ludique
- **FR-011**: Le système DOIT interrompre proprement les animations en cours si l'utilisateur navigue rapidement entre les étapes
- **FR-012**: Le système DOIT conserver la même structure de données en sortie que le formulaire actuel pour compatibilité avec l'API existante
- **FR-013**: Le système DOIT être responsive et fonctionnel sur mobile avec des animations adaptées (réduites si nécessaire pour la performance)
- **FR-014**: Le parcours ludique DOIT être accessible depuis la page liste (index.vue, ajout libre) ET depuis la page visualisation (visualisation.vue, ajout contextuel parent/enfant/conjoint)
- **FR-015**: Lorsque le parcours est lancé depuis la visualisation, le système DOIT adapter les textes d'accroche au contexte (type de lien familial et nom de la personne liée)

### Key Entities

- **Étape du parcours**: Représente une question individuelle dans le flux d'ajout. Attributs : ordre, champ associé, texte d'accroche, texte de transition, caractère obligatoire/optionnel
- **Progression**: État courant de l'utilisateur dans le parcours. Attributs : étape courante, réponses collectées, direction de navigation (avant/arrière)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: L'utilisateur peut compléter l'ajout d'une personne (toutes les étapes) en moins de 2 minutes
- **SC-002**: 90% des utilisateurs complètent le parcours jusqu'au bout sans abandonner (vs formulaire classique)
- **SC-003**: Les transitions entre étapes se jouent en moins de 600ms, sans saccade perceptible
- **SC-004**: Le parcours est entièrement utilisable sur écran mobile (largeur 320px minimum)
- **SC-005**: L'utilisateur peut revenir à n'importe quelle étape précédente sans perte de données
- **SC-006**: Le taux de complétion du formulaire d'ajout augmente par rapport au formulaire classique

## Assumptions

- GSAP 3.14.2 est déjà installé et fonctionnel dans le projet
- Les champs collectés restent identiques au formulaire actuel (nom, prénoms, genre, année de naissance, lieu de naissance, statut vital) - aucun nouveau champ n'est ajouté
- L'API backend reste inchangée - seul le frontend est modifié
- Les textes d'accroche sont en français, cohérents avec le ton panafricain et chaleureux de la plateforme UAfricas
- Pas de sauvegarde intermédiaire des réponses partielles (si l'utilisateur ferme, il recommence)
- Le parcours ludique remplace le comportement par défaut du bouton "Ajouter une personne" sur la page liste ET les boutons d'ajout contextuel sur la page visualisation, mais le formulaire classique reste accessible via un lien secondaire

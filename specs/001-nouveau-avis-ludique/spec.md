# Feature Specification: Nouveau avis de recherche ludique et anime

**Feature Branch**: `001-nouveau-avis-ludique`  
**Created**: 2026-04-07  
**Status**: Draft  
**Input**: User description: "On veut que la publication de nouveau avis sur la page nouveau.vue soit plus ludique et animee pour que les utilisateurs prennent du plaisir a le faire. Utiliser GSAP par exemple."

## Clarifications

### Session 2026-04-07

- Q: Quel style visuel pour les animations (fun/cartoon, fluide/moderne, gamifie) ? → A: Fluide/moderne avec touche festive — transitions douces et elegantes, confettis subtils aux couleurs du site (chocolat/vert), rebond leger uniquement sur le succes.
- Q: Duree et intensite de l'effet confettis au succes ? → A: Moyen (3-4s) — confettis qui retombent naturellement avec gravite, puis fade-out.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Navigation fluide et animee entre les etapes du formulaire (Priority: P1)

En tant qu'utilisateur connecte, je veux que chaque transition entre les 6 etapes du formulaire d'avis de recherche soit accompagnee d'animations fluides et engageantes, afin que le processus de saisie soit agreable et non fastidieux.

**Why this priority**: C'est le coeur de l'experience : les transitions entre etapes sont le moment ou l'utilisateur ressent le plus la fluidite (ou la lourdeur) du formulaire. Sans animations de transition, les autres ameliorations n'ont pas d'impact.

**Independent Test**: Peut etre teste en naviguant d'une etape a l'autre du formulaire et en observant les animations de transition (slide, fade, scale).

**Acceptance Scenarios**:

1. **Given** l'utilisateur est a l'etape 1 du formulaire, **When** il clique sur "Suivant" apres avoir rempli les champs requis, **Then** l'etape 1 disparait avec une animation de sortie (slide vers la gauche + fade out) et l'etape 2 apparait avec une animation d'entree (slide depuis la droite + fade in), le tout en moins de 600ms.
2. **Given** l'utilisateur est a l'etape 3, **When** il clique sur "Precedent", **Then** l'etape 3 sort vers la droite et l'etape 2 entre depuis la gauche (animation inversee).
3. **Given** l'utilisateur clique sur un indicateur d'etape deja completee (ex: etape 1 depuis l'etape 4), **Then** la transition s'anime dans la direction logique (retour en arriere = vers la droite).

---

### User Story 2 - Barre de progression animee et motivante (Priority: P1)

En tant qu'utilisateur, je veux voir ma progression de maniere visuelle et animee tout au long du formulaire, afin de savoir ou j'en suis et etre motive a completer le processus.

**Why this priority**: La barre de progression est visible en permanence et donne le sentiment d'avancement. Elle est essentielle pour reduire l'abandon en cours de saisie.

**Independent Test**: Peut etre teste en naviguant entre les etapes et en verifiant que la barre de progression s'anime fluidement a chaque changement d'etape.

**Acceptance Scenarios**:

1. **Given** l'utilisateur passe de l'etape 2 a l'etape 3, **When** la transition se declenche, **Then** la barre de progression s'anime de 33% a 50% avec une animation fluide (pas de saut brusque), et le numero de l'etape active se met en surbrillance avec un effet de pulse.
2. **Given** l'utilisateur complete une etape, **When** il passe a la suivante, **Then** l'indicateur de l'etape completee affiche une icone de validation avec une animation de type "check" (apparition avec un petit rebond).
3. **Given** l'utilisateur revient a une etape precedente, **When** la barre recule, **Then** l'animation de la barre est fluide dans le sens inverse.

---

### User Story 3 - Animations d'apparition des champs de formulaire (Priority: P2)

En tant qu'utilisateur, je veux que les champs de chaque etape apparaissent de maniere echelonnee et animee, afin que le formulaire semble leger et non intimidant.

**Why this priority**: L'apparition echelonnee des champs rend chaque etape moins "lourde" visuellement et guide l'oeil de l'utilisateur vers les champs dans l'ordre logique.

**Independent Test**: Peut etre teste en arrivant sur chaque etape et en observant que les champs apparaissent un par un avec un leger decalage temporel.

**Acceptance Scenarios**:

1. **Given** l'utilisateur arrive a une nouvelle etape, **When** les champs s'affichent, **Then** chaque champ apparait avec un leger decalage (stagger de ~100ms entre chaque champ), avec un effet de fade-in + translation vers le haut.
2. **Given** l'etape contient des champs conditionnels (ex: les reseaux sociaux a l'etape 4), **When** l'utilisateur coche la case "rencontre sur les reseaux sociaux", **Then** les checkboxes de reseaux sociaux apparaissent avec une animation d'expansion fluide.

---

### User Story 4 - Ecran de succes celebratoire (Priority: P2)

En tant qu'utilisateur qui vient de publier son avis, je veux voir une animation de celebration sur l'ecran de succes, afin de ressentir un sentiment d'accomplissement.

**Why this priority**: Le moment de la publication reussie est le point culminant de l'experience. Une celebration visuelle renforce la satisfaction et encourage le partage.

**Independent Test**: Peut etre teste en soumettant un avis avec succes et en observant les animations sur l'ecran de confirmation.

**Acceptance Scenarios**:

1. **Given** l'utilisateur soumet son avis avec succes, **When** l'ecran de succes s'affiche, **Then** l'icone de validation apparait avec une animation de type "bounce in" (rebond leger), le titre s'anime avec un fade-in depuis le bas, et des confettis subtils aux couleurs du site (chocolat/vert) retombent avec gravite pendant 3-4 secondes puis disparaissent en fade-out. Le style general est fluide et moderne, pas cartoon.
2. **Given** des correspondances sont trouvees, **When** le nombre s'affiche, **Then** le chiffre s'anime avec un compteur progressif (de 0 au nombre final).
3. **Given** l'ecran de succes est affiche, **When** les boutons d'action apparaissent, **Then** ils s'animent avec un stagger et un leger effet de rebond.

---

### User Story 5 - Micro-interactions sur les elements interactifs (Priority: P3)

En tant qu'utilisateur, je veux que les boutons, les selections et les interactions avec le formulaire soient accompagnes de micro-animations, afin que chaque action soit satisfaisante.

**Why this priority**: Les micro-interactions sont le "polish" final qui rend l'experience vraiment premium, mais elles ne sont pas indispensables a la fonctionnalite de base.

**Independent Test**: Peut etre teste en interagissant avec les differents elements du formulaire (boutons, champs, upload photo) et en observant les retours visuels animes.

**Acceptance Scenarios**:

1. **Given** l'utilisateur survole le bouton "Suivant", **When** le curseur entre dans la zone du bouton, **Then** le bouton s'anime avec un leger scale-up et un changement de couleur fluide.
2. **Given** l'utilisateur selectionne un type de relation (radio button), **When** il clique sur une option, **Then** l'option selectionnee s'anime avec un effet de highlight/pulse.
3. **Given** l'utilisateur uploade une photo, **When** le fichier est charge, **Then** la preview apparait avec une animation de zoom-in depuis le centre.

---

### Edge Cases

- Que se passe-t-il si l'utilisateur navigue tres rapidement entre les etapes (double-clic rapide) ? Les animations doivent etre interruptibles et ne pas s'accumuler.
- Que se passe-t-il sur un appareil avec des performances limitees ? Les animations doivent respecter la preference `prefers-reduced-motion` du systeme.
- Que se passe-t-il si l'utilisateur redimensionne la fenetre pendant une animation ? L'animation doit s'adapter sans casser la mise en page.
- Que se passe-t-il lors de la soumission en cas d'erreur ? L'erreur doit apparaitre avec une animation d'attention (shake leger) sans perdre les donnees du formulaire.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le systeme DOIT animer les transitions entre les 6 etapes du formulaire avec des animations directionnelles (gauche/droite selon le sens de navigation).
- **FR-002**: Le systeme DOIT afficher une barre de progression animee qui reflue fluidement la progression entre les etapes (1/6 a 6/6).
- **FR-003**: Le systeme DOIT animer l'apparition des champs de formulaire de maniere echelonnee (stagger) a chaque changement d'etape.
- **FR-004**: Le systeme DOIT afficher une animation de celebration sur l'ecran de succes : confettis subtils aux couleurs du site (chocolat #A54A1C / vert #228B22) retombant avec gravite pendant 3-4 secondes puis fade-out. Style fluide et moderne (pas cartoon).
- **FR-005**: Le systeme DOIT animer le compteur de correspondances trouvees (comptage progressif de 0 a N).
- **FR-006**: Le systeme DOIT respecter la preference systeme `prefers-reduced-motion` en desactivant ou reduisant les animations pour les utilisateurs qui l'ont activee.
- **FR-007**: Les animations DOIVENT etre interruptibles : un clic rapide sur "Suivant" pendant une animation en cours doit completer immediatement l'animation actuelle et lancer la suivante.
- **FR-008**: Le systeme DOIT animer l'apparition des champs conditionnels (ex: reseaux sociaux) avec une transition fluide (expansion/collapse).
- **FR-009**: Le systeme DOIT afficher une animation d'attention (shake) sur le message d'erreur en cas d'echec de soumission.
- **FR-010**: Les animations DOIVENT avoir une duree totale inferieure a 800ms par transition pour ne pas ralentir l'utilisateur.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Les utilisateurs completent le formulaire en entier (taux de completion) au moins 20% plus souvent qu'avec le formulaire actuel sans animations.
- **SC-002**: Chaque transition entre etapes se complete visuellement en moins de 800ms.
- **SC-003**: 100% des animations sont desactivees ou reduites lorsque la preference systeme `prefers-reduced-motion` est active.
- **SC-004**: Aucune animation ne bloque ou ne retarde l'interaction utilisateur (le formulaire reste fonctionnel a tout moment pendant les animations).
- **SC-005**: L'experience animee fonctionne de maniere fluide (pas de saccades visibles) sur les appareils mobiles recents (moins de 3 ans).

## Assumptions

- GSAP 3.14.2 est deja installe dans le projet et disponible sans ajout de dependance.
- Le formulaire wizard a 6 etapes existant (composant `AvisRechercheForm.vue`) est la base a enrichir ; la structure fonctionnelle ne change pas.
- Les animations sont uniquement cote frontend (aucune modification backend ou base de donnees necessaire).
- L'effet "confettis" de l'ecran de succes peut etre realise avec GSAP seul (particules animees en canvas ou elements DOM) sans bibliotheque supplementaire.
- Le hero section et la sidebar de la page `nouveau.vue` restent inchanges ; seul le contenu du formulaire et l'ecran de succes sont concernes.

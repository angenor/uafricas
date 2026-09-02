# Feature Specification: Vidafrica : Sous-titrage vidéo multilingue avec surlignage karaoke

**Feature Branch**: `004-vidafrica-sous-titres`  
**Created**: 2026-04-13  
**Status**: Draft  
**Input**: User description: "Développer une fonctionnalité appelée Vidafrica pour sous-titrer des vidéos en plusieurs langues. Les mots des phrases doivent être surlignés dynamiquement façon karaoké. Les sous-titrages seront enregistrés dans la base de données, saisis par l'admin dans le back-office. Format inspiré de WebVTT avec timing mot par mot."

## User Scenarios & Testing *(mandatory)*

### User Story 1 : L'admin saisit des sous-titres pour une vidéo (Priority: P1)

Un administrateur accède au back-office Vidafrica, sélectionne ou crée une vidéo, puis ajoute une piste de sous-titres dans une langue donnée. Pour chaque segment (cue), il saisit le texte et les timestamps de début/fin. Pour définir le timing mot par mot (effet karaoké), il utilise le mode "tap-to-mark" : la vidéo se lance, les mots du segment s'affichent et l'admin tape un bouton à chaque mot au moment où il est prononcé, le système enregistrant automatiquement les timestamps. L'interface lui permet de prévisualiser la vidéo avec les sous-titres synchronisés avant de publier.

**Why this priority**: Sans saisie de sous-titres, aucune autre fonctionnalité n'est possible. C'est le fondement de tout le système Vidafrica.

**Independent Test**: Peut être testé en créant une vidéo, en ajoutant une piste de sous-titres avec plusieurs segments, et en vérifiant que les données sont correctement persistées et récupérables.

**Acceptance Scenarios**:

1. **Given** un admin connecté sur le back-office, **When** il crée une nouvelle vidéo et ajoute une piste de sous-titres en français avec 3 segments incluant le timing mot par mot, **Then** les sous-titres sont enregistrés et apparaissent dans la liste des pistes de la vidéo.
2. **Given** une piste de sous-titres existante, **When** l'admin modifie le texte ou le timing d'un segment, **Then** les modifications sont sauvegardées et reflétées immédiatement.
3. **Given** une piste de sous-titres existante, **When** l'admin supprime un segment, **Then** le segment est retiré et les autres segments restent intacts.
4. **Given** un admin saisissant des sous-titres, **When** il entre un timestamp de fin antérieur au timestamp de début, **Then** le système affiche un message d'erreur et refuse la saisie.

---

### User Story 2 : Le visiteur regarde une vidéo avec sous-titres karaoké (Priority: P1)

Un visiteur (connecté ou non) accède à la page Vidafrica, choisit une vidéo et la regarde avec des sous-titres affichés en bas de l'écran. Les mots sont surlignés un par un au fur et à mesure de la lecture, créant un effet karaoké fluide. Le visiteur peut changer la langue des sous-titres à tout moment pendant la lecture.

**Why this priority**: C'est l'expérience utilisateur principale, la raison d'être de la fonctionnalité. Sans affichage karaoké, la fonctionnalité n'a pas de valeur distincte.

**Independent Test**: Peut être testé en chargeant une vidéo avec des sous-titres pré-saisis, en lançant la lecture et en vérifiant que chaque mot est surligné au bon moment.

**Acceptance Scenarios**:

1. **Given** une vidéo avec des sous-titres en français publiés, **When** le visiteur lance la lecture, **Then** les sous-titres apparaissent synchronisés avec l'audio et chaque mot est surligné individuellement au moment où il est prononcé.
2. **Given** une vidéo en cours de lecture avec sous-titres en français, **When** le visiteur change la langue vers l'anglais, **Then** les sous-titres basculent immédiatement vers la piste anglaise sans interrompre la lecture.
3. **Given** une vidéo avec sous-titres, **When** le visiteur met en pause puis reprend la lecture, **Then** le surlignage karaoké reprend correctement au bon mot.
4. **Given** une vidéo avec sous-titres, **When** le visiteur avance ou recule dans la timeline, **Then** les sous-titres et le surlignage se repositionnent sur le segment correspondant au nouveau timestamp.

---

### User Story 3 : L'admin gère les vidéos Vidafrica (Priority: P2)

Un administrateur peut créer, modifier et supprimer des vidéos dans le back-office Vidafrica. Chaque vidéo possède un titre, une description, un fichier vidéo uploadé, une vignette, et un état de publication. L'admin peut voir la liste de toutes les vidéos avec leurs pistes de sous-titres associées.

**Why this priority**: La gestion des vidéos est nécessaire pour organiser le contenu, mais peut être simplifiée dans un premier temps (un CRUD basique suffit pour le MVP).

**Independent Test**: Peut être testé en créant, modifiant et supprimant des vidéos, puis en vérifiant que la liste reflète les changements.

**Acceptance Scenarios**:

1. **Given** un admin connecté, **When** il crée une vidéo avec titre, description et fichier vidéo uploadé, **Then** la vidéo apparaît dans la liste avec l'état "brouillon".
2. **Given** une vidéo existante avec des sous-titres, **When** l'admin publie la vidéo, **Then** elle devient visible sur la page publique Vidafrica.
3. **Given** une vidéo publiée, **When** l'admin la dépublie, **Then** elle disparaît de la page publique mais reste accessible dans le back-office.

---

### User Story 4 : L'admin prévisualise les sous-titres en temps réel (Priority: P2)

Pendant la saisie des sous-titres, l'admin peut prévisualiser la vidéo avec l'effet karaoké appliqué en temps réel. Cela lui permet d'ajuster finement les timings mot par mot pour une synchronisation parfaite.

**Why this priority**: La prévisualisation est essentielle pour la qualité du résultat final, mais n'est pas bloquante pour la saisie initiale des données.

**Independent Test**: Peut être testé en saisissant des sous-titres, en lançant la prévisualisation et en vérifiant que le surlignage correspond aux timings définis.

**Acceptance Scenarios**:

1. **Given** un admin en train de saisir des sous-titres pour une vidéo, **When** il clique sur "Prévisualiser", **Then** la vidéo se lance avec les sous-titres et l'effet karaoké appliqué selon les timings saisis.
2. **Given** l'admin en mode prévisualisation, **When** il modifie un timing et relance la prévisualisation, **Then** le changement est immédiatement reflété dans l'affichage karaoké.

---

### User Story 5 : Le visiteur navigue dans le catalogue Vidafrica (Priority: P3)

Un visiteur accède à la page Vidafrica et voit un catalogue de vidéos sous-titrées. Il peut filtrer par langue de sous-titres disponible et rechercher par titre ou description. Chaque carte vidéo affiche la vignette, le titre, la durée et les langues disponibles.

**Why this priority**: L'expérience de navigation enrichit l'usage mais n'est pas essentielle pour le MVP, une simple liste suffit initialement.

**Independent Test**: Peut être testé en publiant plusieurs vidéos avec différentes langues et en vérifiant que les filtres et la recherche fonctionnent correctement.

**Acceptance Scenarios**:

1. **Given** 5 vidéos publiées dont 3 avec sous-titres en français, **When** le visiteur filtre par "français", **Then** seules les 3 vidéos avec sous-titres français sont affichées.
2. **Given** des vidéos publiées, **When** le visiteur recherche un mot du titre, **Then** les vidéos correspondantes sont affichées.

---

### Edge Cases

- Que se passe-t-il si une vidéo n'a aucune piste de sous-titres ? Le lecteur vidéo s'affiche normalement sans sous-titres, avec un message indiquant qu'aucun sous-titre n'est disponible.
- Que se passe-t-il si le timing mot par mot n'est pas renseigné pour certains segments ? Le système affiche le segment entier d'un coup (comportement classique), sans effet karaoké pour ces segments spécifiques.
- Que se passe-t-il si le fichier vidéo uploadé est corrompu ou dans un format non supporté ? Le système rejette l'upload avec un message d'erreur indiquant les formats acceptés (MP4, WebM).
- Que se passe-t-il si deux segments ont des timestamps qui se chevauchent ? Le système affiche un avertissement à l'admin lors de la saisie et empêche la publication tant que les chevauchements ne sont pas résolus.
- Que se passe-t-il si le visiteur utilise un navigateur qui ne supporte pas la lecture vidéo ? Le système affiche un message indiquant de mettre à jour le navigateur.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT permettre à un admin de créer une vidéo avec titre, description, fichier vidéo uploadé, vignette et durée.
- **FR-002**: Le système DOIT permettre à un admin d'ajouter plusieurs pistes de sous-titres par vidéo, chacune associée à une langue.
- **FR-003**: Le système DOIT empêcher la création de deux pistes de sous-titres dans la même langue pour une même vidéo.
- **FR-004**: Le système DOIT permettre la saisie de segments de sous-titres avec : texte, timestamp de début, timestamp de fin.
- **FR-005**: Le système DOIT fournir un mode assisté "tap-to-mark" pour la saisie du timing mot par mot : l'admin lance la lecture vidéo, le texte du segment est affiché mot par mot, et l'admin tape un bouton à chaque mot au moment où il est prononcé. Le système enregistre automatiquement le timestamp de chaque frappe.
- **FR-006**: Le système DOIT valider que les timestamps sont cohérents (début < fin, pas de chevauchement entre segments d'une même piste).
- **FR-007**: Le système DOIT stocker les sous-titres et leurs timings dans la base de données (pas dans des fichiers externes).
- **FR-008**: Le système DOIT afficher les sous-titres de manière synchronisée avec la lecture vidéo côté public.
- **FR-009**: Le système DOIT surligner dynamiquement chaque mot au moment correspondant à son timing (effet karaoké) lorsque le timing mot par mot est disponible.
- **FR-010**: Le système DOIT permettre au visiteur de changer la langue des sous-titres pendant la lecture sans interrompre la vidéo.
- **FR-011**: Le système DOIT gérer les états de publication des vidéos (brouillon, publié, archivé).
- **FR-012**: Le système DOIT supporter la pagination dans la liste des vidéos (back-office et page publique).
- **FR-013**: Le système DOIT permettre la recherche de vidéos par titre et description côté public.
- **FR-014**: Le système DOIT permettre le filtrage des vidéos par langue de sous-titres disponible côté public.
- **FR-015**: Le système DOIT permettre à l'admin de réordonner les segments de sous-titres au sein d'une piste.
- **FR-016**: Le système DOIT journaliser les actions admin (création, modification, suppression) via le système d'audit existant.
- **FR-017**: Le système DOIT supporter le soft delete pour les vidéos et les pistes de sous-titres.

### Key Entities

- **Vidéo** : Contenu vidéo avec titre, description, fichier vidéo uploadé (stockage local), vignette, durée, état de publication. Une vidéo peut avoir plusieurs pistes de sous-titres.
- **Piste de sous-titres** : Ensemble de segments de sous-titres associé à une vidéo et une langue. Une seule piste par langue par vidéo.
- **Segment (Cue)** : Unité de sous-titre avec texte, timestamp de début et timestamp de fin. Appartient à une piste. Ordonné séquentiellement.
- **Timing mot** : Timing individuel d'un mot au sein d'un segment, avec position du mot, timestamp de début et timestamp de fin. Permet l'effet karaoké. Optionnel : si absent, le segment s'affiche en bloc.

## Clarifications

### Session 2026-04-13

- Q: Source des vidéos : externe uniquement, upload local, ou les deux ? → A: Upload local uniquement, pas d'URLs externes.
- Q: Méthode de saisie des timings mot par mot ? → A: Mode assisté "tap-to-mark" : l'admin tape un bouton à chaque mot pendant la lecture vidéo.
- Q: Source des langues pour les pistes de sous-titres ? → A: Liste prédéfinie en dur (enum ou table simple avec ~10-15 langues).

## Assumptions

- Les vidéos sont uploadées localement par l'admin et stockées sur le serveur (similaire au système d'upload existant pour les images). Le système ne supporte pas les URLs externes (YouTube, Vimeo, etc.).
- Le format de stockage des timings en base de données s'inspire de WebVTT (timestamps en millisecondes) mais est adapté pour un stockage relationnel avec timing mot par mot.
- L'authentification admin utilise le système JWT existant du projet.
- Les langues disponibles pour les sous-titres sont une liste prédéfinie en dur (enum ou table simple avec ~10-15 langues africaines et internationales : français, anglais, arabe, portugais, swahili, wolof, haoussa, amharique, zoulou, lingala, etc.). Pas de dépendance au référentiel pays existant.
- La précision des timestamps est à la milliseconde pour permettre un surlignage karaoké fluide.
- Le lecteur vidéo supporte les formats courants (MP4, WebM) uploadés localement.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un admin peut créer une vidéo et saisir une piste complète de sous-titres (10 segments avec timing mot par mot) en moins de 15 minutes.
- **SC-002**: Le surlignage karaoké est perceptiblement synchronisé avec l'audio, avec un décalage maximal imperceptible pour l'utilisateur.
- **SC-003**: Le changement de langue des sous-titres pendant la lecture s'effectue en moins de 1 seconde sans interruption de la vidéo.
- **SC-004**: 95% des visiteurs peuvent lancer une vidéo avec sous-titres karaoké au premier essai sans assistance.
- **SC-005**: Le catalogue supporte au moins 100 vidéos publiées avec recherche et filtrage fonctionnels.
- **SC-006**: Le système gère au moins 5 langues de sous-titres par vidéo sans dégradation de l'expérience.

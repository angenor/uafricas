# Feature Specification: Demande pour devenir expert avec validation admin

**Feature Branch**: `001-demande-expertise`  
**Created**: 2026-05-24  
**Status**: Draft  
**Input**: User description: "Je veux que le lien `Apporter mon expertise` de BoutonLateralGauche.vue redirige plutôt vers un formulaire pour compléter et faire sa demande pour être expert de la plateforme. Une fois la demande faite, l'admin devra valider avant qu'il n'apparaisse sur la page `/experts`"

## Clarifications

### Session 2026-05-24

- Q: Comment gérer la re-soumission d'une demande après un refus ? → A: Soft-delete de l'ancienne demande refusée (archivée) puis création d'une nouvelle demande « en attente » (historise les décisions, compatible avec la contrainte d'unicité existante).
- Q: Par quel canal le candidat est-il notifié de la décision (validation/refus) ? → A: Email uniquement.
- Q: Quelle est la portée du formulaire « compléter et faire sa demande » ? → A: Champs d'expertise + édition des informations de profil de base (photo, fonction, pays de résidence) dans le même écran.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Soumettre une demande pour devenir expert (Priority: P1)

Un membre de la plateforme souhaite mettre son expertise au service du continent. Depuis le menu latéral (« Je m'engage » → « Apporter mon expertise »), il accède à un formulaire dédié qui lui permet à la fois de compléter ses informations de profil de base (photo, fonction, pays de résidence) et de renseigner les informations de son profil d'expert (domaine, biographie, années d'expérience, situation professionnelle, portfolio), puis de soumettre sa demande. Sa demande est enregistrée avec le statut « en attente » et n'est pas encore visible publiquement.

**Why this priority**: C'est le cœur de la fonctionnalité demandée. Sans la capacité de soumettre une demande via un formulaire, aucune autre partie n'a de sens. Elle remplace le comportement actuel (le lien renvoie vers la liste publique `/experts`) qui ne permet à personne de candidater.

**Independent Test**: Peut être testée intégralement en cliquant sur « Apporter mon expertise », en remplissant le formulaire et en le soumettant, puis en vérifiant que la demande est bien enregistrée en attente et qu'un message de confirmation s'affiche, sans dépendre de la partie admin.

**Acceptance Scenarios**:

1. **Given** un membre connecté qui n'a pas encore de demande d'expertise, **When** il clique sur « Apporter mon expertise », **Then** il est dirigé vers le formulaire de demande d'expert (et non vers la liste `/experts`).
2. **Given** un membre sur le formulaire, **When** il remplit tous les champs obligatoires et soumet, **Then** sa demande est enregistrée avec le statut « en attente » et un message de confirmation lui indique que sa demande sera examinée par un administrateur.
3. **Given** un visiteur non connecté, **When** il clique sur « Apporter mon expertise », **Then** il est invité à se connecter avant d'accéder au formulaire, puis redirigé vers le formulaire après connexion.
4. **Given** un membre ayant déjà une demande active (en attente ou validée), **When** il accède au formulaire, **Then** le système l'informe qu'une demande existe déjà et affiche son statut, sans créer de doublon.
5. **Given** un membre sur le formulaire, **When** il soumet avec un champ obligatoire manquant ou invalide, **Then** le système affiche un message d'erreur clair indiquant le champ à corriger et n'enregistre pas la demande.

---

### User Story 2 - Valider ou refuser une demande (administrateur) (Priority: P1)

Un administrateur consulte la liste des demandes d'expertise en attente, ouvre le détail d'une demande, et décide de la valider (l'expert devient alors visible sur la page publique `/experts`) ou de la refuser (avec un commentaire expliquant la raison). Le candidat est notifié de la décision.

**Why this priority**: La demande exige explicitement que « l'admin devra valider avant qu'il n'apparaisse sur la page /experts ». Sans cette étape de modération, les demandes resteraient bloquées indéfiniment en attente et la valeur de la fonctionnalité ne serait pas atteinte.

**Independent Test**: Peut être testée en se connectant en tant qu'administrateur, en ouvrant une demande en attente, en la validant, puis en vérifiant que l'expert apparaît sur `/experts` ; et inversement en refusant une demande et en vérifiant qu'elle n'apparaît pas et que le candidat reçoit une notification.

**Acceptance Scenarios**:

1. **Given** un administrateur, **When** il ouvre la page d'administration des demandes d'expertise, **Then** il voit la liste des demandes filtrable par statut (en attente, validée, refusée) avec recherche.
2. **Given** une demande en attente, **When** l'administrateur la valide, **Then** le statut passe à « validée », la date de validation et l'identité du validateur sont enregistrées, l'expert devient visible sur `/experts`, et le candidat reçoit un email d'approbation.
3. **Given** une demande en attente, **When** l'administrateur la refuse en saisissant un commentaire obligatoire, **Then** le statut passe à « refusée », le candidat reçoit un email de refus accompagné du commentaire, et la demande n'apparaît jamais sur `/experts`.
4. **Given** une demande déjà traitée (validée ou refusée), **When** l'administrateur consulte son détail, **Then** il voit la décision, sa date, l'identité du validateur et le commentaire éventuel.

---

### User Story 3 - Suivre le statut de sa demande (Priority: P2)

Un membre ayant soumis une demande peut consulter le statut de celle-ci (en attente, validée, refusée). En cas de refus, il voit le commentaire de l'administrateur et peut soumettre une nouvelle demande corrigée.

**Why this priority**: Améliore l'expérience et la transparence, mais le MVP (soumettre + valider) fonctionne sans cette visibilité côté candidat. Le suivi et la re-soumission ajoutent de la valeur sans être bloquants.

**Independent Test**: Peut être testée en soumettant une demande, en la faisant refuser par un admin, puis en vérifiant que le candidat voit le statut « refusée » avec le commentaire et peut re-soumettre.

**Acceptance Scenarios**:

1. **Given** un membre avec une demande en attente, **When** il consulte l'espace de suivi de sa demande, **Then** il voit le statut « en attente » et la date de soumission.
2. **Given** un membre dont la demande a été refusée, **When** il consulte son suivi, **Then** il voit le statut « refusée », le commentaire de l'administrateur, et un moyen de soumettre une nouvelle demande.
3. **Given** un membre dont la demande a été validée, **When** il consulte son suivi, **Then** il voit le statut « validée » et un lien vers sa fiche publique d'expert.

---

### Edge Cases

- **Demande déjà existante** : un membre ne peut pas avoir plus d'une demande active simultanément ; une nouvelle soumission n'est autorisée qu'après un refus de la précédente.
- **Re-soumission après refus** : une demande refusée est archivée (soft-delete) et une nouvelle demande « en attente » est créée à la re-soumission ; l'historique de la décision précédente est conservé mais n'a pas à rester visible publiquement.
- **Validation concurrente** : si deux administrateurs traitent simultanément la même demande, seule la première décision est appliquée et la seconde reçoit une indication que la demande a déjà été traitée.
- **Champs longs / vides** : la biographie et les autres champs textes doivent respecter des bornes (longueur minimale/maximale) et rejeter les entrées vides ou trop longues.
- **Domaine ou situation invalide** : seules les valeurs prévues (domaines et situations professionnelles autorisés) sont acceptées.
- **Désactivation du compte** : si le compte du membre est désactivé ou supprimé après validation, son profil expert ne doit plus apparaître publiquement.
- **Visiteur non connecté** : l'accès au formulaire nécessite une connexion ; le parcours de connexion doit ramener l'utilisateur au formulaire.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le lien « Apporter mon expertise » du menu latéral MUST diriger vers un formulaire de demande pour devenir expert, et non plus vers la liste publique `/experts`.
- **FR-002**: Le système MUST exiger qu'un utilisateur soit connecté pour accéder au formulaire et soumettre une demande ; un visiteur non connecté MUST être invité à se connecter puis ramené au formulaire.
- **FR-003**: Le formulaire MUST permettre de renseigner au minimum : le domaine d'expertise, une biographie, le nombre d'années d'expérience, la ou les situations professionnelles, et un portfolio (optionnel).
- **FR-003a**: Le formulaire MUST également permettre au membre de compléter ou mettre à jour ses informations de profil de base utilisées sur la fiche publique d'expert (photo, fonction, pays de résidence) au sein du même écran de demande.
- **FR-004**: Le système MUST valider les champs saisis (présence des champs obligatoires, valeurs autorisées pour le domaine et les situations professionnelles, bornes de longueur des textes, années d'expérience positive) et afficher des messages d'erreur clairs en cas d'invalidité.
- **FR-005**: À la soumission, le système MUST enregistrer la demande avec le statut initial « en attente » et la rendre invisible sur la page publique `/experts`.
- **FR-006**: Le système MUST empêcher un membre d'avoir plus d'une demande active (en attente ou validée) à la fois et l'informer si une demande existe déjà.
- **FR-007**: Le système MUST confirmer la soumission au candidat avec un message indiquant que la demande sera examinée par un administrateur.
- **FR-008**: Un administrateur MUST pouvoir consulter la liste des demandes d'expertise, filtrable par statut (en attente, validée, refusée) et avec recherche.
- **FR-009**: Un administrateur MUST pouvoir consulter le détail complet d'une demande (informations du candidat et de l'expertise proposée).
- **FR-010**: Un administrateur MUST pouvoir valider une demande, ce qui rend le profil expert visible sur `/experts` et enregistre la date de validation et l'identité du validateur.
- **FR-011**: Un administrateur MUST pouvoir refuser une demande en saisissant un commentaire obligatoire expliquant la raison.
- **FR-012**: Le système MUST notifier le candidat de la décision (approbation ou refus) par email, l'email de refus incluant le commentaire de l'administrateur.
- **FR-013**: Seules les demandes au statut « validée » dont le compte du membre est actif MUST apparaître sur la page publique `/experts`.
- **FR-014**: Un membre MUST pouvoir consulter le statut de sa demande (en attente, validée, refusée) ainsi que, en cas de refus, le commentaire de l'administrateur.
- **FR-015**: Un membre dont la demande a été refusée MUST pouvoir soumettre une nouvelle demande corrigée ; l'ancienne demande refusée est archivée (conservée à des fins d'historique) et une nouvelle demande « en attente » est créée.
- **FR-016**: Le système MUST empêcher qu'une demande déjà traitée soit traitée une seconde fois (idempotence de la décision en cas d'accès concurrent).
- **FR-017**: Toutes les actions de modération (validation, refus) MUST être tracées dans le journal d'audit avec l'auteur de l'action.

### Key Entities *(include if feature involves data)*

- **Demande d'expertise** : représente la candidature d'un membre pour devenir expert. Attributs clés : membre demandeur, domaine d'expertise, biographie, années d'expérience, situations professionnelles, portfolio (optionnel), statut (en attente / validée / refusée), commentaire administrateur (en cas de refus), administrateur ayant traité la demande, date de décision, indicateur d'archivage (les demandes refusées sont archivées lors d'une re-soumission). Relation : un membre possède au plus une demande active à la fois ; les demandes archivées sont conservées pour l'historique.
- **Membre / Utilisateur** : l'auteur de la demande, déjà existant sur la plateforme (nom, prénom, pays, fonction, photo). C'est ce profil qui, une fois sa demande validée, apparaît comme expert. Ses informations de profil de base (photo, fonction, pays de résidence) peuvent être mises à jour via le formulaire de demande.
- **Email de notification** : message email informant le candidat du résultat (approbation ou refus) de sa demande ; l'email de refus inclut le commentaire de l'administrateur.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100 % des clics sur « Apporter mon expertise » mènent au formulaire de demande (ou à la connexion puis au formulaire), et plus jamais directement à la liste publique.
- **SC-002**: Un membre peut compléter et soumettre une demande complète en moins de 3 minutes.
- **SC-003**: Aucune demande non validée n'apparaît sur la page publique `/experts` (taux de fuite = 0 %).
- **SC-004**: Un administrateur peut localiser une demande en attente et rendre sa décision (validation ou refus) en moins de 1 minute à partir de la liste.
- **SC-005**: 100 % des décisions (validation ou refus) déclenchent l'envoi d'un email au candidat.
- **SC-006**: Après validation, le nouvel expert apparaît sur `/experts` immédiatement (au prochain chargement de la page).

## Assumptions

- **Réutilisation de l'existant** : la plateforme dispose déjà d'une notion d'expertise avec un statut de modération (en attente / validée / refusée), d'un mécanisme de soumission de candidature, et d'un filtrage de la liste publique sur les expertises validées. Cette fonctionnalité s'appuie sur cet existant ; les manques principaux à combler sont le formulaire frontend, la redirection du lien, le suivi côté candidat et l'interface d'administration (validation/refus + notifications).
- **Alignement sur le workflow « Bibliothèque Humaine »** : le comportement de modération (commentaire de refus obligatoire, re-soumission après refus avec archivage de la demande précédente, journalisation d'audit) suit le même modèle que le workflow déjà en place pour les demandes Bibliothèque Humaine, par cohérence d'expérience et d'architecture. Différence : la notification de décision se fait par email (et non par notification interne), conformément à la clarification de session.
- **Domaines et situations professionnelles** : les valeurs proposées dans le formulaire correspondent aux catégories déjà définies sur la plateforme (domaines : agriculture, informatique, électronique, immobilier, mécanique, santé, éducation, finance ; situations : recherche d'emploi, en emploi, consultance, volontariat d'expertise, recherche de nouvelles opportunités).
- **Rôle administrateur** : seuls les utilisateurs disposant des droits d'administration peuvent accéder à l'interface de validation des demandes.
- **Permanence de la décision** : une fois traitée, une demande conserve sa décision ; une nouvelle évaluation passe par une nouvelle demande (re-soumission après refus), pas par la réouverture de l'ancienne.

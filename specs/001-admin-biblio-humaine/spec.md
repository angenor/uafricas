# Feature Specification: Validation Admin des Bibliothèques Humaines

**Feature Branch**: `001-admin-biblio-humaine`  
**Created**: 2026-04-22  
**Status**: Draft  
**Input**: User description: "sur la page bibliotheque/humaine.vue, il y'a un bouton pour devenir une Bibliothèque Humaine. Maintenant une fois que la personne fait cette requête pour devenir une Bibliothèque Humaine, l'admin doit pouvoir le valider dans le backOffice"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Liste des demandes en attente (Priority: P1)

L'administrateur accède au backoffice et consulte la liste de toutes les demandes d'inscription en tant que Bibliothèque Humaine. Il peut voir les demandes en attente de traitement, avec les informations essentielles de chaque candidat (nom, fonction, spécialités, date de soumission).

**Why this priority**: C'est le point d'entrée indispensable au workflow de validation. Sans cette liste, l'admin ne peut pas gérer les demandes.

**Independent Test**: Un utilisateur connecté soumet une demande via le formulaire public → l'admin se connecte au backoffice → la demande apparaît dans la liste avec le statut "en attente".

**Acceptance Scenarios**:

1. **Given** l'admin est connecté au backoffice, **When** il navigue vers la section Bibliothèques Humaines, **Then** il voit la liste de toutes les demandes avec leur statut (en attente, validé, rejeté)
2. **Given** la liste des demandes est affichée, **When** l'admin applique le filtre "en attente", **Then** seules les demandes non encore traitées sont visibles
3. **Given** la liste est vide, **When** aucune demande n'a été soumise, **Then** un message informatif s'affiche

---

### User Story 2 - Validation ou rejet d'une demande (Priority: P1)

L'administrateur ouvre le détail d'une demande pour consulter le profil complet du candidat (fonction, pays, biographie, spécialités choisies). Il peut ensuite approuver la demande (la personne devient Bibliothèque Humaine active) ou la rejeter en laissant un commentaire facultatif.

**Why this priority**: C'est l'action centrale de cette fonctionnalité. Sans la capacité d'approuver ou rejeter, la feature n'a pas de valeur.

**Independent Test**: L'admin ouvre une demande en attente → clique sur "Approuver" → la personne apparaît dans la liste publique des Bibliothèques Humaines sur la page `/bibliotheque/humaine`.

**Acceptance Scenarios**:

1. **Given** une demande est en statut "en attente", **When** l'admin clique sur "Approuver", **Then** la demande passe en statut "validé" et la personne devient une Bibliothèque Humaine visible publiquement
2. **Given** une demande est en statut "en attente", **When** l'admin clique sur "Rejeter" (avec ou sans commentaire), **Then** la demande passe en statut "rejeté" et la personne n'apparaît pas dans la liste publique
3. **Given** une demande a déjà été traitée (validé ou rejeté), **When** l'admin la consulte, **Then** les boutons d'action permettent de changer la décision (re-valider ou re-rejeter)
4. **Given** l'admin approuve une demande, **When** l'action est confirmée, **Then** une confirmation visuelle s'affiche

---

### User Story 3 - Visibilité publique conditionnelle (Priority: P2)

La liste publique des Bibliothèques Humaines (`/bibliotheque/humaine`) n'affiche que les personnes dont la demande a été validée par un administrateur. Les demandes en attente ou rejetées ne sont pas visibles.

**Why this priority**: Sans ce filtrage, des profils non vérifiés apparaîtraient publiquement, ce qui compromet la qualité de la liste.

**Independent Test**: Après validation d'une demande par l'admin, l'utilisateur non connecté accède à `/bibliotheque/humaine` et voit le profil de la personne nouvellement validée.

**Acceptance Scenarios**:

1. **Given** une demande vient d'être soumise (statut "en attente"), **When** un visiteur consulte la page publique, **Then** ce profil n'est pas affiché
2. **Given** une demande est approuvée par l'admin, **When** un visiteur consulte la page publique, **Then** le profil apparaît dans la liste
3. **Given** une bibliothèque humaine est ensuite rejetée (décision réversible), **When** un visiteur consulte la page, **Then** le profil disparaît de la liste publique

---

### User Story 4 - Suivi de demande par le candidat (Priority: P2)

L'utilisateur ayant soumis une demande peut consulter le statut actuel de sa candidature (en attente, validé, rejeté) depuis son profil ou tableau de bord personnel, sans attendre une notification.

**Why this priority**: Évite la frustration de l'utilisateur qui ne sait pas si sa demande est prise en compte, et réduit les soumissions en double.

**Independent Test**: Un utilisateur soumet une demande → accède à son profil → voit un encart "Votre demande Bibliothèque Humaine : En attente de validation".

**Acceptance Scenarios**:

1. **Given** une demande vient d'être soumise, **When** l'utilisateur consulte son profil/tableau de bord, **Then** il voit le statut "en attente" avec la date de soumission
2. **Given** la demande a été approuvée, **When** l'utilisateur consulte son profil, **Then** il voit le statut "validé"
3. **Given** la demande a été rejetée, **When** l'utilisateur consulte son profil, **Then** il voit le statut "rejeté" et le commentaire admin éventuel

---

### User Story 5 - Notification au candidat (Priority: P3)

L'utilisateur ayant soumis une demande reçoit une notification in-app lorsque son dossier a été traité par l'admin, qu'il soit approuvé ou rejeté. En cas de rejet, le commentaire de l'admin est inclus dans la notification.

**Why this priority**: Améliore l'expérience utilisateur mais n'est pas bloquant pour le workflow admin.

**Independent Test**: L'admin approuve une demande → l'utilisateur se connecte et voit une notification indiquant que sa demande a été acceptée.

**Acceptance Scenarios**:

1. **Given** une demande est approuvée, **When** l'admin valide, **Then** l'utilisateur reçoit une notification in-app de succès
2. **Given** une demande est rejetée avec un commentaire, **When** l'admin valide le rejet, **Then** l'utilisateur reçoit une notification incluant le motif de rejet
3. **Given** une demande est rejetée sans commentaire, **When** l'admin valide le rejet, **Then** l'utilisateur reçoit une notification générique de rejet

---

### Edge Cases

- Que se passe-t-il si un utilisateur soumet une demande alors qu'il en a déjà une en attente ou validée ? Le système doit empêcher les doublons.
- Que se passe-t-il si un utilisateur dont la demande est validée supprime son compte ?
- Que se passe-t-il si l'admin tente de valider une demande déjà validée (double clic) ? L'action doit être idempotente.
- Que se passe-t-il si l'utilisateur modifie son profil après validation ? La fiche Bibliothèque Humaine doit-elle se mettre à jour automatiquement ?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT conserver le statut de chaque demande de Bibliothèque Humaine (`en_attente`, `validé`, `rejeté`)
- **FR-002**: Le système DOIT permettre à un administrateur de consulter la liste de toutes les demandes, filtrables par statut
- **FR-003**: L'administrateur DOIT pouvoir consulter le détail complet d'une demande (nom, prénom, fonction, pays, biographie, spécialités, date de soumission)
- **FR-004**: L'administrateur DOIT pouvoir approuver une demande en attente
- **FR-005**: L'administrateur DOIT pouvoir rejeter une demande en attente, avec un commentaire facultatif
- **FR-006**: L'administrateur DOIT pouvoir changer la décision d'une demande déjà traitée (re-valider ou re-rejeter)
- **FR-007**: La liste publique des Bibliothèques Humaines DOIT afficher uniquement les profils dont la demande est en statut `validé`
- **FR-008**: Le système DOIT empêcher un utilisateur d'avoir plusieurs demandes simultanées actives (une seule demande en statut `en_attente` ou `validé` par utilisateur à la fois). Un utilisateur dont la dernière demande est `rejeté` PEUT soumettre une nouvelle demande.
- **FR-009**: L'utilisateur DOIT pouvoir consulter le statut de sa demande (en_attente / validé / rejeté) depuis son profil ou tableau de bord personnel
- **FR-010-bis**: L'utilisateur DOIT être notifié (in-app) lorsque sa demande est approuvée ou rejetée
- **FR-010**: L'interface d'administration DOIT indiquer le nombre de demandes en attente de traitement
- **FR-011**: Chaque décision admin (approbation ou rejet) DOIT être enregistrée via le service d'audit existant (action, identifiant admin, identifiant demande, horodatage)

### Key Entities

- **Demande Bibliothèque Humaine**: Représente une candidature soumise par un utilisateur. Attributs : identifiant candidat, fonction, pays, biographie, spécialités choisies, statut (en_attente/validé/rejeté), date de soumission, date de traitement, commentaire administrateur
- **Bibliothèque Humaine (profil actif)**: Profil public affiché sur la page `/bibliotheque/humaine`. N'existe que si la demande associée est en statut `validé`. Lié à l'identifiant utilisateur du candidat
- **Administrateur**: Utilisateur avec le rôle `admin` (rôle IAM existant) ayant accès au backoffice pour traiter les demandes — aucun nouveau rôle à créer

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un administrateur peut traiter (approuver ou rejeter) une demande en moins de 2 minutes à partir de la liste d'attente
- **SC-002**: 100 % des demandes soumises apparaissent dans le backoffice admin immédiatement après soumission
- **SC-003**: Un profil approuvé est visible sur la page publique dans un délai inférieur à 1 minute après validation
- **SC-004**: Zéro profil non validé n'apparaît dans la liste publique des Bibliothèques Humaines
- **SC-005**: L'utilisateur reçoit une notification de décision dans les 5 minutes suivant le traitement de sa demande

## Clarifications

### Session 2026-04-22

- Q: Quel statut attribuer aux inscriptions déjà existantes avant cette feature ? → A: Aucune inscription existante — le projet n'est pas en production, démarrage à zéro, aucune migration nécessaire.
- Q: Un utilisateur rejeté peut-il resoumettre une nouvelle demande ? → A: Oui, sans délai — un utilisateur rejeté peut resoumettre librement une nouvelle candidature.
- Q: Quel rôle admin peut accéder à la validation des Bibliothèques Humaines ? → A: Tout utilisateur avec le rôle `admin` existant — pas de nouveau rôle à créer.
- Q: L'utilisateur peut-il voir le statut de sa demande dans son espace personnel ? → A: Oui — le statut est visible sur son profil ou tableau de bord.
- Q: Faut-il enregistrer un historique des décisions admin ? → A: Oui — via le service d'audit existant (`audit::log_action`), cohérent avec le reste du projet.

## Assumptions

- L'authentification et la gestion des rôles admin existent déjà dans le système (backoffice UAfricas existant)
- Le formulaire de soumission côté public (`/bibliotheque/humaine`) existe déjà côté frontend (mock) mais le backend n'a pas encore de table `biblio_humaine` — démarrage à zéro, aucune migration de données existantes nécessaire
- La notification "in-app" est suffisante pour P3 ; les notifications email sont hors scope pour cette feature
- La décision de l'admin est réversible (possibilité de re-valider ou re-rejeter après traitement initial)
- Un seul administrateur peut traiter une demande (pas de workflow multi-approbateurs)

# Feature Specification: Collaboration et Partage de l'Arbre

**Feature Branch**: `001-collaboration-partage`
**Created**: 2026-03-16
**Status**: Draft
**Input**: User description: "Feature 6 — Collaboration et partage. Inviter des membres de la famille à co-éditer un arbre. Gestion des permissions (lecture seule vs édition). Paramètres de confidentialité : choisir quelles parties de l'arbre sont visibles pour le matching public. Historique des modifications."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Inviter un membre de la famille à collaborer (Priority: P1)

Le propriétaire de l'arbre peut inviter d'autres utilisateurs de la plateforme à accéder à son arbre. L'invitation se fait par email. Le destinataire reçoit une notification et peut accepter ou refuser. En acceptant, il obtient un accès à l'arbre selon le niveau de permission défini par le propriétaire (lecture seule ou édition).

**Why this priority**: Sans invitation, l'arbre reste un outil individuel. La collaboration familiale est le fondement de la richesse des données — chaque membre peut apporter des informations que les autres ne connaissent pas.

**Independent Test**: Inviter un utilisateur par email → il accepte → il voit l'arbre du propriétaire dans sa liste d'arbres accessibles.

**Acceptance Scenarios**:

1. **Given** le propriétaire de l'arbre, **When** il saisit l'email d'un utilisateur inscrit et clique « Inviter », **Then** le destinataire reçoit une notification d'invitation avec le nom de l'arbre et le niveau de permission proposé.
2. **Given** une invitation en attente, **When** le destinataire accepte, **Then** l'arbre apparaît dans sa liste « Arbres partagés avec moi » et il peut y accéder selon ses permissions.
3. **Given** une invitation en attente, **When** le destinataire refuse, **Then** l'invitation disparaît et le propriétaire est informé du refus.
4. **Given** le propriétaire invite un email non inscrit, **When** l'invitation est envoyée, **Then** un message indique « Cette personne n'est pas encore inscrite. Elle recevra l'invitation quand elle créera son compte. »
5. **Given** le propriétaire, **When** il consulte la page de gestion de son arbre, **Then** il voit la liste des collaborateurs avec leur niveau de permission et la possibilité de modifier ou révoquer l'accès.

---

### User Story 2 - Gérer les permissions de collaboration (Priority: P1)

Le propriétaire définit le niveau de permission pour chaque collaborateur : « Lecture seule » (peut voir l'arbre mais pas le modifier) ou « Édition » (peut ajouter, modifier et supprimer des personnes et des liens). Le propriétaire peut modifier le niveau de permission à tout moment. Seul le propriétaire peut inviter de nouveaux collaborateurs et gérer les permissions.

**Why this priority**: Sans gestion de permissions, soit tout le monde peut tout modifier (dangereux), soit personne ne peut contribuer (inutile). Les permissions sont indissociables de l'invitation.

**Independent Test**: Inviter un utilisateur en « Lecture seule » → il ne peut pas cliquer sur les boutons d'édition. Changer sa permission en « Édition » → il peut maintenant ajouter des personnes.

**Acceptance Scenarios**:

1. **Given** un collaborateur avec permission « Lecture seule » sur un arbre, **When** il consulte la visualisation, **Then** les boutons d'action (ajouter, modifier, supprimer) ne sont PAS affichés dans le panneau contextuel.
2. **Given** un collaborateur avec permission « Édition », **When** il ajoute une personne dans l'arbre, **Then** la personne est créée et visible pour tous les collaborateurs et le propriétaire.
3. **Given** le propriétaire, **When** il change la permission d'un collaborateur de « Lecture seule » à « Édition », **Then** le collaborateur voit immédiatement les boutons d'action lors de sa prochaine consultation.
4. **Given** le propriétaire, **When** il révoque l'accès d'un collaborateur, **Then** l'arbre disparaît de la liste « Arbres partagés » du collaborateur et il ne peut plus y accéder.

---

### User Story 3 - Paramètres de confidentialité pour le matching (Priority: P2)

Le propriétaire peut définir quelles personnes de son arbre sont visibles pour le matching public (Feature 4). Par défaut, toutes les personnes sont éligibles au matching. Le propriétaire peut marquer certaines personnes comme « privées » (exclues du matching) ou rendre tout l'arbre privé.

**Why this priority**: Essentiel pour le respect de la vie privée, mais le comportement par défaut (tout visible) fonctionne pour la majorité des cas.

**Independent Test**: Marquer une personne comme « privée » → vérifier qu'elle n'apparaît plus dans les résultats de recherche publique ni dans les suggestions de matching.

**Acceptance Scenarios**:

1. **Given** le propriétaire dans les paramètres de son arbre, **When** il consulte la section confidentialité, **Then** il voit la liste de toutes les personnes avec un toggle « Visible pour le matching » (activé par défaut).
2. **Given** une personne marquée comme « privée », **When** le matching s'exécute, **Then** cette personne est exclue des comparaisons et n'apparaît dans aucune suggestion.
3. **Given** le propriétaire, **When** il active l'option « Arbre entièrement privé », **Then** aucune personne de son arbre n'est éligible au matching public.
4. **Given** un collaborateur avec permission « Édition », **When** il consulte les paramètres de confidentialité, **Then** il ne peut PAS modifier les paramètres de confidentialité (réservé au propriétaire uniquement).

---

### User Story 4 - Historique des modifications (Priority: P2)

Le propriétaire et les collaborateurs avec permission « Édition » peuvent consulter l'historique des modifications de l'arbre : qui a ajouté/modifié/supprimé quoi et quand. L'historique permet de comprendre l'évolution de l'arbre et de retracer les contributions de chaque collaborateur.

**Why this priority**: Fonctionnalité de traçabilité importante pour la confiance entre collaborateurs, mais non bloquante pour l'utilisation de base.

**Independent Test**: Ajouter une personne avec le compte A, modifier avec le compte B → vérifier que l'historique montre les deux actions avec les bons auteurs.

**Acceptance Scenarios**:

1. **Given** l'arbre a été modifié par 3 collaborateurs, **When** le propriétaire consulte l'historique, **Then** il voit une liste chronologique inversée des actions : ajout/modification/suppression de personnes et liens, avec le nom de l'auteur, la date et les détails du changement.
2. **Given** l'historique affiché, **When** l'utilisateur filtre par collaborateur, **Then** seules les actions de ce collaborateur sont affichées.
3. **Given** une modification récente, **When** l'historique est consulté, **Then** le détail montre les valeurs avant/après (ex : « Prénom changé de 'Ibrahim' à 'Ibrahima' par [Collaborateur] le [Date] »).
4. **Given** un collaborateur avec permission « Lecture seule », **When** il consulte l'historique, **Then** il peut voir les modifications mais ne peut pas les annuler.

---

### User Story 5 - Naviguer entre ses arbres et les arbres partagés (Priority: P1)

L'utilisateur qui a accès à plusieurs arbres (le sien + ceux partagés avec lui) peut naviguer facilement entre eux. La page index de l'arbre généalogique affiche deux sections : « Mon arbre » et « Arbres partagés avec moi ».

**Why this priority**: Indispensable dès que la collaboration est active — l'utilisateur doit pouvoir accéder aux arbres partagés.

**Independent Test**: Se connecter avec un compte qui a accès à 2 arbres partagés → vérifier que les 3 arbres (le sien + 2 partagés) sont listés.

**Acceptance Scenarios**:

1. **Given** un utilisateur avec son propre arbre et 2 arbres partagés, **When** il accède à la page de l'arbre généalogique, **Then** il voit son arbre en premier, suivi d'une section « Arbres partagés avec moi » listant les 2 arbres avec le nom du propriétaire et son niveau de permission.
2. **Given** la liste des arbres, **When** l'utilisateur clique sur un arbre partagé, **Then** il visualise cet arbre dans la page de visualisation, avec les permissions appropriées.
3. **Given** un arbre partagé en lecture seule ouvert, **When** l'utilisateur consulte l'interface, **Then** un bandeau indique « Arbre de [Propriétaire] — Lecture seule ».

---

### Edge Cases

- Que se passe-t-il si le propriétaire supprime son compte ? → Les collaborateurs perdent l'accès. L'arbre est traité selon la politique de suppression de compte de la plateforme (soft delete).
- Que se passe-t-il si deux collaborateurs modifient la même personne en même temps ? → Le dernier à sauvegarder écrase les modifications de l'autre (last-write-wins). L'historique conserve les deux versions.
- Que se passe-t-il si un collaborateur « Édition » supprime une personne que le propriétaire ne voulait pas supprimer ? → Le propriétaire peut consulter l'historique et recréer la personne manuellement. Pas de mécanisme de rollback automatique dans cette version.
- Combien de collaborateurs maximum par arbre ? → Limite de 20 collaborateurs par arbre pour éviter les abus.
- Que se passe-t-il si l'utilisateur invité n'a pas encore de compte ? → L'invitation est mise en attente et sera présentée à l'utilisateur lors de sa première connexion après inscription.
- Un collaborateur peut-il inviter d'autres personnes ? → Non, seul le propriétaire peut inviter et gérer les permissions.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le propriétaire DOIT pouvoir inviter un utilisateur par email à collaborer sur son arbre.
- **FR-002**: Chaque invitation DOIT spécifier un niveau de permission : « Lecture seule » ou « Édition ».
- **FR-003**: Le destinataire DOIT pouvoir accepter ou refuser l'invitation via une notification dans l'interface.
- **FR-004**: Un collaborateur « Lecture seule » DOIT voir l'arbre (visualisation, recherche) mais NE DOIT PAS voir les boutons d'action (ajouter, modifier, supprimer).
- **FR-005**: Un collaborateur « Édition » DOIT pouvoir ajouter, modifier et supprimer des personnes et des liens dans l'arbre partagé.
- **FR-006**: Le propriétaire DOIT pouvoir modifier le niveau de permission ou révoquer l'accès d'un collaborateur à tout moment.
- **FR-007**: Seul le propriétaire DOIT pouvoir inviter de nouveaux collaborateurs et gérer les permissions (pas les collaborateurs eux-mêmes).
- **FR-008**: Le propriétaire DOIT pouvoir marquer des personnes individuelles comme « privées » (exclues du matching public).
- **FR-009**: Le propriétaire DOIT pouvoir rendre son arbre entièrement privé (exclu de tout matching).
- **FR-010**: Les paramètres de confidentialité NE DOIVENT être modifiables que par le propriétaire de l'arbre.
- **FR-011**: Le système DOIT enregistrer un historique de toutes les modifications de l'arbre : action (ajout/modification/suppression), auteur, date, entité concernée, valeurs avant/après.
- **FR-012**: L'historique DOIT être consultable par le propriétaire et les collaborateurs « Édition ». Les collaborateurs « Lecture seule » peuvent consulter l'historique mais pas annuler de modifications.
- **FR-013**: La page d'accueil de l'arbre DOIT afficher « Mon arbre » et « Arbres partagés avec moi » comme deux sections distinctes.
- **FR-014**: Un arbre partagé en lecture seule DOIT afficher un bandeau visuel indiquant le mode et le nom du propriétaire.
- **FR-015**: Le nombre de collaborateurs par arbre DOIT être limité à 20.
- **FR-016**: Si l'email invité ne correspond à aucun compte, l'invitation DOIT être mise en attente et activée automatiquement lors de l'inscription future de cet utilisateur.

### Key Entities

- **Invitation** : Demande d'accès envoyée par le propriétaire à un utilisateur. Attributs : arbre cible, email invité, niveau de permission proposé, statut (en_attente, acceptee, refusee, expiree), date d'envoi. Cycle de vie : en_attente → acceptée (crée un collaborateur) ou refusée.
- **Collaborateur** : Lien entre un utilisateur et un arbre qui ne lui appartient pas. Attributs : utilisateur, arbre, permission (lecture_seule, edition), date d'ajout, invité_par (propriétaire). Pas de soft delete — la révocation supprime le lien.
- **Paramètre de confidentialité** : Configuration par personne ou par arbre. Attributs : personne ou arbre, visible_matching (boolean, défaut true). Quand false, la personne est exclue de l'algorithme de matching (Feature 4).
- **Entrée d'historique** : Enregistrement d'une modification de l'arbre. Attributs : arbre, auteur, action (ajout/modification/suppression), entité (personne ou lien), données avant/après, date. Réutilise potentiellement le système d'audit existant (Feature 1).

## Assumptions

- Chaque utilisateur ne peut être propriétaire que d'un seul arbre (contrainte existante : table `arbres` avec `utilisateur_id UNIQUE`).
- Un collaborateur accède à l'arbre du propriétaire, pas une copie. Toutes les modifications sont partagées en temps réel.
- L'historique des modifications réutilise et étend le système d'audit existant (`shared.audit_log`) qui enregistre déjà les mutations avec before/after JSONB. L'historique sera un filtre/vue sur cette table pour un arbre donné.
- L'invitation par email utilise le système d'email existant (SMTP via lettre, Feature 1). Si l'email n'est pas inscrit, l'invitation est stockée et présentée à la première connexion.
- Les paramètres de confidentialité n'affectent que le matching (Feature 4) et la recherche publique (Feature 5). L'arbre reste toujours visible pour ses collaborateurs, quel que soit le paramètre.
- Le propriétaire est toujours identifié comme le `utilisateur_id` de la table `arbres`. Il ne peut pas transférer la propriété dans cette version.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Le propriétaire peut inviter un collaborateur et celui-ci peut accepter en moins de 2 minutes (de l'envoi à l'accès effectif).
- **SC-002**: 100% des tentatives d'édition par un collaborateur « Lecture seule » sont bloquées (pas de bouton visible, pas de contournement par appel direct).
- **SC-003**: Les personnes marquées « privées » n'apparaissent dans aucun résultat de matching ou de recherche publique — taux d'exclusion : 100%.
- **SC-004**: L'historique des modifications est consultable en moins de 2 secondes pour un arbre avec 500 entrées d'historique.
- **SC-005**: 80% des utilisateurs ayant reçu une invitation la traitent (acceptent ou refusent) dans les 7 jours.
- **SC-006**: Un utilisateur peut naviguer entre son arbre et un arbre partagé en moins de 3 clics.
- **SC-007**: Le bandeau « Lecture seule » est visible en permanence lors de la consultation d'un arbre partagé en lecture seule — aucune confusion possible sur le mode.

# Research: Collaboration et Partage de l'Arbre

**Date**: 2026-03-16
**Feature Branch**: `001-collaboration-partage`

## Décision 1 : Modèle de permissions

**Décision** : Deux niveaux simples : `lecture_seule` et `edition`. Pas de granularité par branche ou par personne.

**Raisonnement** :
- YAGNI : deux niveaux couvrent 95% des cas d'usage familiaux (les enfants en lecture, les adultes en édition).
- Granularité par branche/personne : complexité explosive (ACL par nœud), reportée à une version future si le besoin se confirme.
- Le propriétaire reste le seul à gérer permissions et confidentialité.

## Décision 2 : Stockage des collaborateurs

**Décision** : Nouvelle table `arbre_genealogique.collaborateurs` avec FK vers `arbres` et `iam.utilisateur`.

**Raisonnement** :
- Table dédiée plutôt qu'extension de `arbres` (qui a `utilisateur_id UNIQUE` pour le propriétaire).
- Permet de stocker la permission, la date d'ajout, et l'inviteur.
- Limite de 20 collaborateurs vérifiée par COUNT avant INSERT.

## Décision 3 : Invitations en attente

**Décision** : Table `arbre_genealogique.invitations` séparée des collaborateurs. L'invitation crée un collaborateur uniquement à l'acceptation.

**Raisonnement** :
- Séparation claire entre intention (invitation) et réalité (accès effectif).
- Supporte les invitations à des emails non encore inscrits (stockage de l'email, pas de FK vers utilisateur).
- L'invitation expire après 30 jours.

## Décision 4 : Paramètres de confidentialité

**Décision** : Colonne `visible_matching BOOLEAN DEFAULT TRUE` ajoutée directement sur `arbre_genealogique.personnes`, plus un flag global `arbre_prive BOOLEAN DEFAULT FALSE` sur `arbre_genealogique.arbres`.

**Raisonnement** :
- Pas de table séparée pour les paramètres, un simple boolean par personne est plus simple.
- Le flag global sur `arbres` permet d'exclure tout l'arbre en une opération.
- Le matching (Feature 4) filtre sur `WHERE p.visible_matching = TRUE AND a.arbre_prive = FALSE`.

## Décision 5 : Historique des modifications

**Décision** : Réutiliser la table `shared.audit_log` existante. Ajouter un filtre côté handler pour extraire les entrées d'un arbre donné.

**Raisonnement** :
- L'audit log capture déjà : action, utilisateur, table, record_id, before/after JSONB, IP, user-agent.
- Pas besoin d'une nouvelle table : un endpoint avec filtre `WHERE schema_name = 'arbre_genealogique' AND ...` suffit.
- L'affichage frontend formate les entries en langage lisible.

## Décision 6 : Notification des invitations

**Décision** : Notification in-app (badge/indicateur) + email SMTP pour les invitations. Pas de notification temps réel (WebSocket).

**Raisonnement** :
- Le système d'email existant (lettre, SMTP) permet d'envoyer l'invitation par email.
- Un badge dans l'interface indique les invitations en attente au prochain chargement de page.
- Le temps réel (WebSocket) est surdimensionné pour des invitations qui arrivent rarement.

## Décision 7 : Accès multi-arbres : sélection de l'arbre actif

**Décision** : La page index `/arbre-genealogique` affiche deux sections. L'utilisateur clique sur un arbre pour le charger dans la visualisation. L'endpoint `arbre-complet` accepte un paramètre `arbre_id` optionnel.

**Raisonnement** :
- Actuellement, `arbre-complet` retourne l'arbre de l'utilisateur connecté (via `utilisateur_id`).
- Pour les arbres partagés, l'endpoint doit accepter un `arbre_id` explicite et vérifier que l'utilisateur est propriétaire ou collaborateur.
- La visualisation, l'édition et la recherche fonctionnent ensuite sur l'arbre actif sélectionné.

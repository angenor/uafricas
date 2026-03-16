# Research: Notifications et Suggestions Intelligentes

**Date**: 2026-03-16
**Feature Branch**: `001-notifications-suggestions`

## Décision 1 : Stockage des notifications

**Décision** : Nouvelle table `arbre_genealogique.notifications` avec polling côté client (pas de WebSocket).

**Raisonnement** :
- Table simple : id, destinataire_id, type, message, lien_action, lu, created_at.
- Le polling se fait au chargement de chaque page (un appel GET pour compter les non-lues).
- WebSocket surdimensionné pour des événements qui arrivent quelques fois par jour max.
- YAGNI : le polling est simple à implémenter et suffisant pour l'UX attendue.

## Décision 2 : Création des notifications dans les handlers existants

**Décision** : Injecter des INSERT dans les handlers des Features 4 et 6 (matching, collaboration) au moment des événements.

**Raisonnement** :
- Pattern simple : après chaque action génératrice de notification, un INSERT non-bloquant (comme l'audit existant).
- Pas besoin d'un système d'événements/bus — les points de création sont identifiables (5-6 endroits).
- Exemples : dans `matching_profond` → INSERT notif "Nouveau match", dans `accepter_invitation` → INSERT notif "Invitation acceptée".

## Décision 3 : Suggestions proactives côté client

**Décision** : Calcul entièrement côté client dans un composable `useSuggestions`, basé sur le graphe en mémoire.

**Raisonnement** :
- Les données sont déjà en mémoire (endpoint `arbre-complet`). Analyser le graphe = parcourir les nœuds côté JS.
- Suggestions : `noeud.parents.length < 2` → "Parents manquants", `!noeud.naissance` → "Date manquante".
- Pas de nouvel endpoint — c'est une extension du composable `useLayoutArbre` déjà existant.
- Recalculé à chaque modification de l'arbre.

## Décision 4 : Détection de doublons intra-arbre

**Décision** : Endpoint backend dédié `GET /api/arbre/doublons` qui réutilise pg_trgm + normalisation de Feature 4, mais comparé au sein du même arbre.

**Raisonnement** :
- Le même algorithme de scoring (nom 35%, prénoms 20%, date 15%, lieu 20%, genre 10%) mais appliqué entre les personnes du même arbre.
- Seuil plus élevé que le matching inter-arbres (70% au lieu de 55%) car les doublons intra-arbre sont plus souvent des erreurs.
- Table `arbre_genealogique.doublons_ignores` pour stocker les paires marquées "pas un doublon".

## Décision 5 : Fusion de doublons

**Décision** : Endpoint transactionnel `POST /api/arbre/doublons/fusionner` qui combine deux personnes en une.

**Raisonnement** :
- Transaction : choisir quelle personne garder (A ou B), transférer tous les liens de B vers A, mettre à jour les champs choisis, soft-delete B.
- L'utilisateur choisit champ par champ (nom de A ou B, dates de A ou B, etc.) via un formulaire de fusion côté frontend.
- Après fusion, recharger `arbre-complet` pour mettre à jour le graphe.

## Décision 6 : Badge dans la navbar (composable global)

**Décision** : Composable `useNotifications` global initialisé dans le layout, qui fait un appel `GET /api/notifications/compteur` au montage de chaque page.

**Raisonnement** :
- Un seul appel léger (retourne juste un nombre) à chaque navigation de page.
- Le composable expose un `ref<number>` réactif consommé par le composant cloche dans la navbar.
- Pas de polling périodique — juste au changement de page (Nuxt navigation guards).

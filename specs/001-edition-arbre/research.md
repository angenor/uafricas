# Research: Édition Interactive de l'Arbre Généalogique

**Date**: 2026-03-16
**Feature Branch**: `001-edition-arbre`

## Décision 1 : Menu contextuel dans vue-flow

**Décision** : Intégrer les boutons d'action directement dans le panneau contextuel existant (`PanneauPersonne.vue`) plutôt que d'utiliser un menu contextuel séparé (right-click popup).

**Raisonnement** :
- Le panneau contextuel (mini-fiche) de Feature 2 s'ouvre déjà au clic sur un nœud. Y ajouter des boutons d'action est le flux le plus naturel.
- Pas besoin d'un composant menu contextuel séparé (right-click n'est pas intuitif sur mobile).
- Le panneau latéral a assez d'espace pour les boutons + le formulaire qui le remplace au clic d'une action.

**Alternatives évaluées** :
- **Menu popup flottant** : Nécessite un positionnement complexe par rapport au nœud, problèmes sur mobile. Rejeté.
- **Right-click / long-press** : Non intuitif, surtout sur mobile. Pas de convention établie dans le projet. Rejeté.

## Décision 2 : Formulaire dans le panneau latéral

**Décision** : Le formulaire d'ajout/modification remplace le contenu du panneau latéral existant (desktop) / bottom sheet (mobile). Un bouton « Retour » permet de revenir à la mini-fiche.

**Raisonnement** :
- Clarification de la spec (session 2026-03-16) : l'utilisateur a choisi cette approche.
- Réutilise le composant `PanneauPersonne.vue` avec un état interne (mode: fiche | formulaire-ajout | formulaire-modifier).
- Pas de modal bloquante, l'arbre reste visible pendant l'édition.
- Sur mobile, le bottom sheet existant s'étend pour accueillir le formulaire (scrollable).

**Alternatives évaluées** :
- **Modal overlay** : Bloque la vue de l'arbre, moins contextuel. Rejeté.
- **Nouveau panneau gauche** : Double panneau = trop de chrome, surtout sur mobile. Rejeté.

## Décision 3 : Réutilisation du formulaire existant

**Décision** : Adapter le composant `PersonneForm.vue` existant (Feature 1) pour fonctionner dans le panneau latéral, plutôt que de créer un nouveau formulaire.

**Raisonnement** :
- `PersonneForm.vue` contient déjà toute la logique de validation (nom requis, cohérence dates, genres).
- Il supporte le mode création et modification (via props).
- L'adapter au panneau latéral nécessite principalement un ajustement de taille/espacement.
- Principe V (YAGNI) de la constitution : pas de duplication.

**Alternatives évaluées** :
- **Nouveau composant formulaire** : Duplication de code, deux composants à maintenir. Rejeté.
- **Formulaire simplifié (nom seul)** : Trop limité, l'utilisateur voudra saisir les dates et le genre. Rejeté.

## Décision 4 : Calcul d'incomplétude côté client

**Décision** : Le calcul des branches incomplètes se fait entièrement côté client dans `useLayoutArbre.ts`, basé sur le graphe en mémoire.

**Raisonnement** :
- Les données complètes de l'arbre sont déjà chargées en mémoire (endpoint `arbre-complet`).
- Un parent manquant = `noeud.parents.length < 2`. Simple comptage.
- Pas besoin d'un endpoint dédié.
- Le compteur se met à jour instantanément après chaque ajout/suppression côté client.

**Alternatives évaluées** :
- **Endpoint backend dédié** : Surcharge réseau inutile, les données sont déjà côté client. Rejeté.

## Décision 5 : Pas de nouvel endpoint backend

**Décision** : Aucun nouvel endpoint. Les API CRUD existantes (Feature 1) suffisent.

**Raisonnement** :
- `POST /api/arbre/personnes` crée une personne + rattachement.
- `POST /api/arbre/liens` crée un lien familial (avec cycle detection).
- `PUT /api/arbre/personnes/{id}` modifie une personne.
- `DELETE /api/arbre/personnes/{id}` supprime avec cascade.
- Le frontend chaîne les appels : créer personne → créer lien → recharger arbre-complet.
- Détection de cycles déjà implémentée dans le handler `creer_lien`.

**Alternatives évaluées** :
- **Endpoint composite (créer personne + lien atomiquement)** : Plus propre mais viole YAGNI. Les deux appels séquentiels sont suffisants. Si nécessaire plus tard, un endpoint transactionnel pourra être ajouté.

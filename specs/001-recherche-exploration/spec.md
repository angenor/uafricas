# Feature Specification: Recherche et Exploration de l'Arbre

**Feature Branch**: `001-recherche-exploration`
**Created**: 2026-03-16
**Status**: Draft
**Input**: User description: "Feature 5 — Recherche et exploration. Rechercher une personne par nom/lieu/date dans son propre arbre. Rechercher dans la base publique pour voir si quelqu'un a déjà référencé un de ses ancêtres. Visualiser le chemin de parenté entre deux personnes. Filtrer par branche familiale, par zone géographique, par génération."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Rechercher dans son propre arbre (Priority: P1)

L'utilisateur connecté peut rechercher une personne dans son arbre par nom, prénom, lieu de naissance ou date de naissance. Les résultats s'affichent en temps réel à mesure que l'utilisateur tape. En cliquant sur un résultat, la vue arbre se centre sur la personne trouvée. La recherche fonctionne aussi bien depuis la page de visualisation que depuis une page de recherche dédiée.

**Why this priority**: La recherche dans son propre arbre est le besoin le plus immédiat — dès que l'arbre dépasse 20 personnes, le parcours visuel ne suffit plus.

**Independent Test**: Avec un arbre de 30+ personnes, saisir un nom partiel dans le champ de recherche → vérifier que les résultats pertinents apparaissent en moins de 1 seconde.

**Acceptance Scenarios**:

1. **Given** un arbre contenant "Diallo Ibrahim" et "Diallo Ousmane", **When** l'utilisateur tape "Diallo" dans le champ de recherche, **Then** les deux personnes apparaissent dans les résultats avec leurs dates et lieux.
2. **Given** le champ de recherche vide, **When** l'utilisateur tape "Ségou" (recherche par lieu), **Then** toutes les personnes nées ou décédées à Ségou apparaissent.
3. **Given** un résultat de recherche affiché, **When** l'utilisateur clique sur une personne, **Then** la vue arbre se centre sur cette personne et le panneau contextuel s'ouvre.
4. **Given** une recherche sans résultat, **When** aucune personne ne correspond, **Then** un message « Aucun résultat pour [terme] » s'affiche.
5. **Given** l'utilisateur tape un texte, **When** il efface le champ, **Then** les résultats disparaissent et la vue revient à son état initial.

---

### User Story 2 - Rechercher dans la base publique (Priority: P2)

L'utilisateur peut rechercher si un de ses ancêtres a été référencé dans l'arbre d'un autre utilisateur. La recherche publique utilise les noms normalisés (comme le matching) pour trouver des correspondances même avec des variantes orthographiques. Les résultats montrent des informations anonymisées (pas d'identité du propriétaire de l'arbre) et invitent l'utilisateur à consulter la page Découvertes pour les correspondances automatiques.

**Why this priority**: Complément naturel du matching automatique (Feature 4), mais la recherche manuelle ajoute une dimension exploratoire active.

**Independent Test**: Avec deux comptes ayant des personnes similaires, rechercher un nom depuis le compte A → vérifier que les personnes du compte B apparaissent dans les résultats avec anonymisation.

**Acceptance Scenarios**:

1. **Given** l'utilisateur A recherche "Kouyaté" dans la base publique, **When** l'utilisateur B a une "Kouyaté Fatoumata" dans son arbre, **Then** le résultat montre "Kouyaté Fatoumata" avec dates/lieu mais sans l'identité de B (juste "Membre #XXXX").
2. **Given** un résultat de recherche publique, **When** l'utilisateur veut en savoir plus, **Then** il est invité à aller sur la page Découvertes pour voir les correspondances automatiques ou à attendre que le matching détecte cette paire.
3. **Given** une recherche publique, **When** les résultats incluent des personnes de son propre arbre, **Then** celles-ci sont marquées « Votre arbre » et ne sont pas mélangées avec les résultats externes.

---

### User Story 3 - Visualiser le chemin de parenté entre deux personnes (Priority: P2)

L'utilisateur peut sélectionner deux personnes dans son arbre et voir le chemin de parenté qui les relie, exprimé en langage naturel (ex : « X est le grand-père de Y », « A et B sont cousins au 3ème degré »). Le chemin est aussi mis en surbrillance dans la vue arbre.

**Why this priority**: Fonctionnalité d'exploration avancée très appréciée dans les logiciels de généalogie, mais non bloquante pour l'utilisation de base.

**Independent Test**: Sélectionner un arrière-grand-père et un petit-fils → vérifier que le chemin affiché est correct avec la terminologie familiale appropriée.

**Acceptance Scenarios**:

1. **Given** Ibrahim (grand-père) et Aminata (petite-fille) dans l'arbre, **When** l'utilisateur demande le chemin de parenté entre eux, **Then** le système affiche « Ibrahim est le grand-père de Aminata » avec le chemin Ibrahim → Ousmane → Aminata mis en surbrillance.
2. **Given** deux personnes sans lien familial dans l'arbre, **When** l'utilisateur demande le chemin, **Then** le système affiche « Aucun lien de parenté trouvé entre X et Y ».
3. **Given** deux conjoints, **When** le chemin est demandé, **Then** le système affiche « X est le/la conjoint(e) de Y ».
4. **Given** le chemin de parenté affiché, **When** l'utilisateur consulte la vue arbre, **Then** les nœuds et liens du chemin sont visuellement mis en surbrillance (couleur distincte).

---

### User Story 4 - Filtrer l'arbre par critères (Priority: P3)

L'utilisateur peut filtrer les personnes visibles dans la vue arbre selon plusieurs critères : zone géographique (lieu de naissance), génération (par rapport à une personne de référence), branche familiale (paternelle/maternelle). Les filtres s'appliquent en temps réel et peuvent être combinés.

**Why this priority**: Fonctionnalité de confort pour les grands arbres, mais les modes ascendant/descendant de Feature 2 couvrent déjà partiellement ce besoin.

**Independent Test**: Appliquer un filtre géographique "Mali" → vérifier que seules les personnes nées/décédées au Mali sont visibles.

**Acceptance Scenarios**:

1. **Given** un arbre avec des personnes de différents pays, **When** l'utilisateur filtre par « Mali », **Then** seules les personnes dont le lieu de naissance ou décès contient « Mali » sont affichées.
2. **Given** un arbre affiché, **When** l'utilisateur filtre par génération « ±2 » autour de la personne centrée, **Then** seuls les grands-parents, parents, la personne, ses enfants et petits-enfants sont visibles.
3. **Given** un arbre avec un couple au centre, **When** l'utilisateur filtre par « branche paternelle », **Then** seuls les ancêtres et descendants du côté du père sont visibles.
4. **Given** plusieurs filtres actifs, **When** l'utilisateur désactive tous les filtres, **Then** l'arbre complet est de nouveau affiché.

---

### Edge Cases

- Que se passe-t-il si la recherche par lieu utilise des variantes (« Segou » vs « Ségou ») ? → La recherche normalise les diacritiques pour trouver les correspondances.
- Comment le chemin de parenté gère-t-il les familles recomposées (parent adoptif + parent biologique) ? → Tous les chemins possibles sont affichés, en indiquant le type de lien (père, mère, parent) pour chaque connexion.
- Que se passe-t-il si le chemin de parenté est très long (10+ générations) ? → Le chemin est affiché de façon résumée (« X est un descendant de Y à la 10ème génération »).
- Comment filtrer par « branche paternelle » si la personne a plus de 2 parents ? → Le filtre propose tous les parents comme options de branche.
- Que se passe-t-il si l'utilisateur recherche dans la base publique sans être connecté ? → Redirection vers la page de connexion.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT fournir un champ de recherche permettant de chercher par nom, prénom, lieu ou date dans l'arbre de l'utilisateur.
- **FR-002**: Les résultats de recherche DOIVENT s'afficher en temps réel (debounce de 300ms) à mesure que l'utilisateur tape.
- **FR-003**: Cliquer sur un résultat de recherche DOIT centrer la vue arbre sur la personne correspondante.
- **FR-004**: Le système DOIT proposer une recherche dans la base publique (tous les arbres) avec résultats anonymisés (pas d'identité du propriétaire avant la correspondance confirmée).
- **FR-005**: La recherche publique DOIT utiliser la normalisation phonétique existante (Feature 4) pour trouver les variantes orthographiques des noms.
- **FR-006**: Le système DOIT permettre de sélectionner deux personnes et calculer le chemin de parenté entre elles.
- **FR-007**: Le chemin de parenté DOIT être exprimé en langage naturel français (père, mère, grand-père, cousin, oncle, etc.) et mis en surbrillance dans la vue arbre.
- **FR-008**: Le système DOIT proposer des filtres combinables : zone géographique (lieu), génération (±N autour d'une personne), branche familiale (paternelle/maternelle).
- **FR-009**: Les filtres DOIVENT s'appliquer en temps réel sans rechargement de page.
- **FR-010**: Les résultats de recherche DOIVENT distinguer les personnes de l'arbre de l'utilisateur (marquées « Votre arbre ») de celles provenant d'autres arbres.
- **FR-011**: Si aucun chemin de parenté n'est trouvé entre deux personnes, le système DOIT afficher un message explicite.
- **FR-012**: Le champ de recherche DOIT être accessible depuis la barre d'outils de la page de visualisation.
- **FR-013**: Un seul champ de recherche DOIT être utilisé pour les deux modes (local et public), avec un toggle « Mon arbre / Tous les arbres ». Par défaut, la recherche s'effectue dans l'arbre de l'utilisateur (résultats instantanés côté client). L'utilisateur peut basculer vers la recherche publique d'un clic sur le toggle.

## Clarifications

### Session 2026-03-16

- Q: Comment l'utilisateur bascule-t-il entre recherche locale et publique ? → A: Un seul champ avec toggle "Mon arbre / Tous les arbres" — résultats locaux par défaut, bascule vers public.

### Key Entities

- **Résultat de recherche** : Personne correspondant aux critères saisis. Attributs affichés : nom complet, dates de vie, lieu, indicateur source (mon arbre / autre arbre). Cliquable pour centrer la vue.
- **Chemin de parenté** : Séquence ordonnée de nœuds et liens reliant deux personnes dans l'arbre. Attributs : personne source, personne cible, liste de nœuds intermédiaires, terminologie familiale calculée (« grand-père », « cousin », etc.), longueur en générations.
- **Filtre actif** : Critère de visibilité appliqué à l'arbre. Types : géographique (texte libre sur lieu), générationnel (plage ±N), branche (sélection d'un parent comme racine). Combinables entre eux.

## Assumptions

- La recherche dans son propre arbre est côté client (les données sont déjà chargées via `arbre-complet`). Pas de nouvel endpoint pour la recherche locale.
- La recherche publique nécessite un nouvel endpoint côté serveur qui utilise les colonnes normalisées et pg_trgm existants (Feature 4).
- Le calcul du chemin de parenté se fait côté client par un algorithme BFS/DFS sur le graphe en mémoire. Pour les chemins longs (>10 nœuds), un résumé est affiché.
- La terminologie familiale est déterminée par un algorithme qui compte les générations ascendantes et descendantes entre les deux personnes (méthode de Knuth : LCA — Lowest Common Ancestor).
- Les filtres sont des transformations côté client du graphe existant, pas de nouveaux endpoints.
- La « branche paternelle » est définie comme : tous les ancêtres et descendants accessibles en passant uniquement par le père (ou le premier parent masculin) de la personne centrée.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Les résultats de recherche dans son propre arbre apparaissent en moins de 500ms pour un arbre de 200 personnes.
- **SC-002**: La recherche publique retourne des résultats en moins de 2 secondes pour une base de 10 000 personnes.
- **SC-003**: Le chemin de parenté entre deux personnes est calculé et affiché en moins de 1 seconde pour un arbre de 200 personnes.
- **SC-004**: 90% des utilisateurs trouvent une personne spécifique dans leur arbre en moins de 10 secondes grâce à la recherche.
- **SC-005**: La terminologie familiale affichée est correcte dans 95% des cas pour les relations jusqu'au 4ème degré (cousin, oncle, grand-oncle, etc.).
- **SC-006**: Les filtres réduisent le nombre de nœuds visibles d'au moins 40% en moyenne quand appliqués sur un arbre de 50+ personnes.

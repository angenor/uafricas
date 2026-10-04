# Specification Quality Checklist : Activités interactives, ludiques et participatives

**Purpose** : valider la complétude et la qualité de la spécification avant le passage au plan
**Created** : 2026-10-01
**Feature** : [spec.md](../spec.md)

## Content Quality

- [x] Aucun détail d'implémentation (langages, frameworks, API)
- [x] Centrée sur la valeur pour le membre et le besoin produit
- [x] Rédigée pour un lecteur non technique
- [x] Toutes les sections obligatoires sont remplies

## Requirement Completeness

- [x] Aucun marqueur de clarification ne subsiste, les trois questions avaient été tranchées par le client avant la réécriture
- [x] Les exigences sont testables et sans ambiguïté
- [x] Les critères de succès sont mesurables
- [x] Les critères de succès sont indépendants de la technique
- [x] Tous les scénarios d'acceptation sont définis
- [x] Les cas limites sont identifiés
- [x] Le périmètre est borné (sections *Couverture du document client* et *Assumptions*, « Périmètre exclu »)
- [x] Dépendances et hypothèses identifiées

## Feature Readiness

- [x] Chaque exigence fonctionnelle a des critères d'acceptation clairs
- [x] Les histoires couvrent les parcours principaux (jouer, alimenter, défi, duel différé, Championship, carte, duel direct, dérivation, réputation)
- [x] La feature satisfait les résultats mesurables des critères de succès
- [x] Aucun détail d'implémentation ne fuit dans la spec, les valeurs chiffrées (48 heures, tailles de série) sont des règles produit, réglables

## Notes

Cette spec est une **réécriture** : la première version, validée, n'avait pas été commitée et a été perdue avec la machine. Les numéros d'exigences ne correspondent donc pas à ceux de l'original (qui comptait 74 exigences ; celle-ci en compte 85, pour 9 histoires, 15 critères de succès et 11 entités).

Les trois décisions du client sont consignées dans la section *Clarifications* et répercutées dans les exigences :

- **Q1 → pays d'origine, repli sur la résidence, figé à l'acquisition.** FR-053 à FR-055. Conséquence assumée : un membre qui change de pays en cours de saison peut figurer dans deux classements de pays.
- **Q2 → mixte, dérivation puis revue.** FR-008, FR-009, FR-075, FR-076 et l'histoire 8. Deux règles en découlent que la décision ne disait pas : une épreuve dérivée suit le sort de son contenu source (FR-011, FR-012), et une candidate rejetée n'est jamais reproposée (FR-075).
- **Q3 → duel différé et duel direct.** Histoires 4 (P2) et 7 (P3), un seul cycle d'états pour les deux modes (FR-040).

Trois points appellent une vigilance au plan, sans bloquer :

1. **Le jeu ne crédite pas de points d'engagement** (FR-023). C'est une hypothèse, pas une décision du client : elle découle du recadrage du système d'engagement sur trois sources. À faire confirmer avant d'engager le plan, car l'inverser change le modèle de score.
2. **Le duel direct est le seul élément qui exige une synchronisation entre deux écrans.** Le plan doit choisir entre les deux moyens temps réel déjà en service. S'il déborde, il se livre après les autres histoires sans rien casser.
3. **SC-002 engage un volume de contenu** (100 épreuves jouables par pilote à la livraison). Il dépend du travail de revue de l'administration, pas seulement du développement.

Spec prête pour `/speckit-plan`.

# Specification Quality Checklist : Africanité, publications éphémères

**Purpose** : valider la complétude et la qualité de la spécification avant le passage au plan
**Created** : 2026-08-25
**Feature** : [spec.md](../spec.md)

## Content Quality

- [x] Aucun détail d'implémentation (langages, frameworks, API)
- [x] Centrée sur la valeur pour le membre et le besoin produit
- [x] Rédigée pour un lecteur non technique
- [x] Toutes les sections obligatoires sont remplies

## Requirement Completeness

- [x] Aucun marqueur de clarification ne subsiste, les trois questions ont été tranchées le 2026-08-25
- [x] Les exigences sont testables et sans ambiguïté
- [x] Les critères de succès sont mesurables
- [x] Les critères de succès sont indépendants de la technique
- [x] Tous les scénarios d'acceptation sont définis
- [x] Les cas limites sont identifiés
- [x] Le périmètre est borné (section Assumptions, « Périmètre exclu »)
- [x] Dépendances et hypothèses identifiées

## Feature Readiness

- [x] Chaque exigence fonctionnelle a des critères d'acceptation clairs
- [x] Les histoires couvrent les parcours principaux (publier, regarder, retirer, signaler, mesurer)
- [x] La feature satisfait les résultats mesurables des critères de succès
- [x] Aucun détail d'implémentation ne fuit dans la spec, les limites de fichier restent énoncées comme des règles produit, sans valeur ni format imposés

## Notes

Les trois décisions prises le 2026-08-25 sont consignées dans la section *Clarifications* de la spec et répercutées dans les exigences, pas seulement enregistrées.

- **Q1 → ami(e)s uniquement.** A produit FR-006a (amitié constatée *à l'instant de la lecture*) et FR-006b (le membre sans ami(e)s n'a pas une rangée vide, il a une sortie).
- **Q2 → détruite, sauf si signalée.** C'est la décision la plus structurante : elle introduit **trois états** (active, échue, détruite) là où la spec n'en connaissait que deux, et le gel de destruction par un signalement en attente (FR-018a à FR-018d). FR-018d ferme le trou qui restait : un signalement portant sur une africanité déjà détruite est accepté quand même, sinon un abus tardif ne laisserait aucune trace.
- **Q3 → trois formes.** Le texte sur fond coloré ne dépose aucun fichier (FR-001b) ; les trois formes ne se combinent pas (FR-001a). C'est la décision qui élargit le plus le périmètre : la vidéo introduit une contrainte de durée que la plateforme ne manipule nulle part ailleurs.

Deux points appellent une vigilance au plan, sans bloquer :

1. Le gel de destruction crée un état où un média échu survit sur le stockage. Il faut pouvoir le constater et le purger une fois la modération statuée, sinon la rétention dérive silencieusement.
2. La vidéo courte est le seul élément de cette feature sans précédent dans le dépôt. Si le plan la fait déborder, elle peut être livrée après les deux autres formes sans casser les histoires P1.

Spec prête pour `/speckit-plan`.

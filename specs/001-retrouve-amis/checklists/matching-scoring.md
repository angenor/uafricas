# Matching & Scoring Requirements Quality Checklist: Retrouve Amis

**Purpose**: Valider la complétude, clarté et cohérence des exigences liées à l'algorithme de matching et scoring avant implémentation
**Created**: 2026-02-28
**Feature**: [spec.md](../spec.md)
**Audience**: Auteur (pré-implémentation)
**Depth**: Standard

---

## Complétude des exigences de scoring

- [ ] CHK001 — Les pondérations des 5 critères de scoring (nom 40pts, école 20pts, ville 15pts, période 15pts, pays 10pts) sont-elles justifiées par des données utilisateur ou une hypothèse documentée ? [Completeness, Research §R2]
- [ ] CHK002 — Le comportement de la fonction de scoring est-il défini lorsqu'un seul critère est renseigné sur l'avis source (ex: uniquement le nom) ? Le score max possible est-il documenté dans ce cas ? [Completeness, Gap]
- [ ] CHK003 — Les exigences définissent-elles le comportement du scoring lorsqu'un critère est NULL d'un côté seulement (avis source renseigné, cible non renseigné ou inversement) ? [Completeness, Spec §FR-004]
- [ ] CHK004 — Le score de correspondance pour la branche avis↔profil trouvable est-il spécifié avec la même granularité que la branche avis↔avis, incluant le mapping des champs parcours ? [Completeness, Research §R2]
- [ ] CHK005 — Les exigences précisent-elles comment le "meilleur score parmi les entrées de parcours" est calculé lorsqu'un profil trouvable a plusieurs écoles et villes ? Est-ce le max par critère ou un agrégat ? [Clarity, Research §R2]

## Clarté de l'algorithme de matching

- [ ] CHK006 — La formule de scoring du nom (GREATEST de 3 variantes : directe, inversée nom/prénom, combinée) est-elle spécifiée de manière mesurable pour le cas où `prenom_recherche` est NULL ? [Clarity, Data-model §calculer_correspondances]
- [ ] CHK007 — Le seuil de 60% est-il défini sur une échelle absolue (60 points sur 100) ou relative ? La spec FR-004 dit "score minimum de 60%" mais la recherche R2 parle de "score >= 60 (sur 100)" — cette ambiguïté est-elle résolue ? [Ambiguity, Spec §FR-004 vs Research §R2]
- [ ] CHK008 — Le calcul de chevauchement de période est-il spécifié pour les cas limites : une seule année renseignée (début sans fin), périodes identiques, périodes qui ne se chevauchent pas du tout ? [Clarity, Research §R2]
- [ ] CHK009 — La méthode de similarité pour les villes est-elle clairement définie : "correspondance exacte (même ville) ou similarité trigramme" — quel est le seuil de similarité trigramme utilisé pour les villes ? [Ambiguity, Research §R2]
- [ ] CHK010 — Le terme "variantes orthographiques courantes" pour les noms africains est-il quantifié ? Des exemples représentatifs au-delà de Kouamé/Kouame et Ndèye/Ndeye sont-ils documentés pour calibrer les attentes de `pg_trgm` ? [Clarity, Assumption §3]

## Cohérence entre documents

- [ ] CHK011 — Les poids de scoring dans research.md (R2) sont-ils cohérents avec ceux dans data-model.md (fonction SQL pseudo-code) et dans tasks.md (T002) ? [Consistency, Research §R2 vs Data-model §calculer_correspondances vs Tasks §T002]
- [ ] CHK012 — La spec FR-003 mentionne "recoupement automatique" mais le plan mentionne "matching synchrone dans le handler HTTP" — les exigences fonctionnelles sont-elles cohérentes avec les contraintes de performance SC-002 (< 5 min) pour tous les cas de volume ? [Consistency, Spec §FR-003 vs Plan §R4]
- [ ] CHK013 — Les critères d'exclusion du matching (auto-correspondance, blacklist, correspondances actives) sont-ils documentés de manière identique dans la spec (FR-015, FR-017), le data-model (contraintes SQL) et les tasks (T002) ? [Consistency]
- [ ] CHK014 — La structure JSONB `details_score` est-elle définie (clés, types de valeurs) dans le data-model ou les contrats API, ou reste-t-elle implicite ? [Consistency, Data-model §correspondance vs Contracts]

## Couverture des scénarios

- [ ] CHK015 — Les exigences définissent-elles le comportement du matching lors de la modification d'un avis (T015 mentionne "suppression correspondances en_attente + relance") : que se passe-t-il pour les correspondances aux états `acceptee_a` ou `acceptee_b` ? [Coverage, Tasks §T015]
- [ ] CHK016 — Le scénario de désactivation de `est_trouvable` est-il spécifié pour les correspondances en état `acceptee_a` ou `acceptee_b` (pas seulement `en_attente`) ? La spec FR-009 dit "correspondances en cours basées uniquement sur son profil sont annulées" mais quels états exacts sont concernés ? [Coverage, Spec §FR-009]
- [ ] CHK017 — Les exigences couvrent-elles le scénario de correspondance bidirectionnelle avis↔avis (A cherche B ET B cherche A) ? Deux correspondances sont-elles créées ou une seule ? Comment éviter les doublons symétriques ? [Coverage, Edge Case]
- [ ] CHK018 — Le comportement est-il défini quand un avis est suspendu par un admin alors qu'il a des correspondances `en_attente` ou partiellement acceptées ? Les correspondances sont-elles annulées ou conservées ? [Coverage, Edge Case, Spec §FR-016]
- [ ] CHK019 — Les exigences couvrent-elles le cas où le score de matching change après une modification d'avis (ex: passe sous les 60% pour une correspondance déjà `en_attente`) ? [Coverage, Edge Case]

## Exigences non-fonctionnelles du matching

- [ ] CHK020 — Le SC-002 exige un délai < 5 min pour la détection — les exigences définissent-elles un seuil de volume au-delà duquel cette garantie n'est plus tenable avec l'approche synchrone ? [Measurability, Spec §SC-002]
- [ ] CHK021 — Les exigences de performance de la fonction SQL de scoring sont-elles spécifiées ? Le plan mentionne "~1000 comparaisons en < 100ms" — ce chiffre est-il une exigence formelle ou une estimation ? [Clarity, Research §R4]
- [ ] CHK022 — La qualité du matching (SC-004 : taux de faux positifs < 20%) est-elle mesurable avec la formule actuelle ? Des critères de validation ou un jeu de test de référence sont-ils définis ? [Measurability, Spec §SC-004]
- [ ] CHK023 — Les exigences définissent-elles un plan de recalibrage si le seuil de 60% s'avère trop bas (trop de faux positifs) ou trop haut (aucune correspondance trouvée) ? [Gap, Recovery Flow]

## Dépendances et hypothèses

- [ ] CHK024 — L'hypothèse que les extensions PostgreSQL `pg_trgm` et `unaccent` sont déjà chargées (`00_extensions.sql`) est-elle vérifiée et documentée comme prérequis explicite ? [Dependency, Research §R2]
- [ ] CHK025 — L'hypothèse que `pg_trgm` avec `unaccent` gère correctement les noms africains francophones est-elle validée ? Des tests de similarité sur des noms représentatifs (Wolof, Bambara, Fon, Swahili) sont-ils envisagés ? [Assumption, Spec §Assumptions §3]
- [ ] CHK026 — Le comportement de `pg_trgm.similarity()` pour des chaînes très courtes (1-3 caractères, ex: surnoms "Mo", "Di") est-il documenté ? Les scores peuvent être biaisés pour les chaînes courtes. [Assumption, Gap]

---

## Notes

- 26 items couvrant les dimensions clés de qualité des exigences de matching et scoring
- Focus sur la clarté de la formule de scoring, la couverture des cas limites, et la cohérence inter-documents
- Les items CHK007, CHK009, CHK017, CHK019 identifient des ambiguïtés potentielles à résoudre avant implémentation
- Références croisées : spec.md (FR-003/004/009/015/017, SC-002/004), research.md (R2/R4), data-model.md, tasks.md (T002/T015)

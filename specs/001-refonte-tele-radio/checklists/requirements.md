# Specification Quality Checklist: Refonte des pages Télé et Radio Africans

**Purpose**: Validate specification completeness and quality before proceeding to planning
**Created**: 2026-07-19
**Feature**: [spec.md](../spec.md)

## Content Quality

- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

## Requirement Completeness

- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Success criteria are technology-agnostic (no implementation details)
- [x] All acceptance scenarios are defined
- [x] Edge cases are identified
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

## Feature Readiness

- [x] All functional requirements have clear acceptance criteria
- [x] User scenarios cover primary flows
- [x] Feature meets measurable outcomes defined in Success Criteria
- [x] No implementation details leak into specification

## Notes

- Items marked incomplete require spec updates before `/speckit.clarify` or `/speckit.plan`

### Itération 1 — 2026-07-19

Un seul critère en échec : **3 marqueurs `[NEEDS CLARIFICATION]` subsistent**, tous portant sur des décisions
qui relèvent du commanditaire et pour lesquelles aucun défaut raisonnable ne s'impose :

| Marqueur | Exigence | Sujet |
|----------|----------|-------|
| 1 | FR-014 | Répartition des stations entre Radio Africans et Radio Nationales |
| 2 | FR-036 | Périmètre des contributeurs autorisés à soumettre chaîne et contenus |
| 3 | FR-022 | Forme visuelle des sections (plein écran enchaîné ou blocs empilés) |

Tous les autres points de vigilance relevés à la rédaction ont été traités sans arbitrage externe et
consignés en **Hypothèses** (H-001 à H-010) plutôt qu'en marqueurs :

- périmètre du lot → résolu par la priorisation P1/P2/P3, le MVP étant limité aux deux stories P1 (H-001) ;
- conservation du direct radio → conservé, le flux devenant un contenu parmi d'autres dans la section (FR-016, H-009) ;
- espace communautés → l'espace de publications existant, sans en créer un nouveau (H-003) ;
- seuil de signalement → aligné sur les contributions comparables de la plateforme (H-007) ;
- vedette codée en dur de la page Télé → provisoire assumé, remplacé par la mise en avant pilotable (H-006, FR-010).

### Itération 2 — 2026-07-19 — ✅ tous critères satisfaits

Les 3 arbitrages ont été rendus par le commanditaire et intégrés à la spécification ; aucun marqueur ne subsiste.

| Sujet | Décision | Traduction dans la spec |
|-------|----------|--------------------------|
| Répartition des deux pages Radio | Par **origine de publication** : Radio Africans = publications propres à la plateforme, décidées par ses créateurs ; Radio Nationales = stations et contenus rattachés à un territoire africain. Rattachements disjoints. | FR-014 réécrit ; US2 scénarios 1-3 ; D-001 ; H-011 (la page Télé, non dédoublée, n'est pas concernée) |
| Contributeurs autorisés | **Tout membre connecté**, sans accréditation préalable, avec validation administrative systématique. La bannière Radio Africans reste éditoriale. | FR-036 réécrit, adossé à FR-031/FR-032 |
| Forme des sections | **Blocs empilés de hauteur naturelle** : bandeau du contenu mis en évidence + rangée horizontale défilante des autres contenus. Seule la vedette générale de la page Télé est plein écran. | FR-022 réécrit, référencé depuis FR-005 |

**Verdict** : spécification prête pour `/speckit.clarify` (facultatif) ou `/speckit.plan`.

### Itération 3 — 2026-07-19 — session `/speckit.clarify`

5 questions posées et intégrées (voir `## Clarifications` dans la spec). Aucune n'a invalidé un critère du
checklist ; toutes ont converti des zones **Partielles** de la taxonomie en exigences testables.

| # | Zone taxonomique | Décision | Impact spec |
|---|------------------|----------|-------------|
| 1 | Compliance / droits | Aucune décharge demandée au contributeur | FR-033, H-012, 1 edge case |
| 2 | Rôles & permissions | Co-détention à plusieurs | FR-037/045/047/055, entité Co-détention, US5, 2 edge cases |
| 3 | Cycle de vie / sécurité | Métadonnées libres, remplacement du média revalidé | FR-032, US4 scénarios 9-10, entité Programme |
| 4 | Intégration externe | Téléversement et lien externe, sans restriction | FR-056, SC-001, SC-010, H-009, 1 edge case |
| 5 | Parcours & UX | Barre de lecture persistante en bas d'écran | FR-013, FR-017, US2 scénario 5, SC-006 |

**Restent Outstanding, faible impact** : fiabilité/disponibilité (non critique pour des pages de contenu) ;
terminologie « programme / émission / contenu », que le back-office emploie différemment côté radio et côté
télé — à unifier lors du plan. **Deferred au plan** : limitation de débit, formats d'import/export, versionnement
de protocole.

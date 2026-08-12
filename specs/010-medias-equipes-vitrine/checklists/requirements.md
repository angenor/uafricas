# Specification Quality Checklist: Médias — équipes éditoriales et recentrage des vitrines Télé & Radio

**Purpose**: Valider la complétude et la qualité de la spécification avant le passage à la planification
**Created**: 2026-08-10
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

- **Itération 1 (2026-08-10)** : 3 marqueurs `[NEEDS CLARIFICATION]` posés, présentés sous forme de questions Q1–Q3.
- **Itération 2 (2026-08-10)** : les 3 marqueurs sont levés par les arbitrages du commanditaire — tous les items passent.
  - **Q1 → A / FR-040** : référentiel de périodicité fermé à **4 valeurs** (non périodique, journalier, hebdomadaire, mensuel). « Mensuel » est la seule valeur neuve ; les trois cadences existantes se transposent terme à terme (FR-043).
  - **Q2 → B / FR-013, FR-014** : un membre d'équipe est une fiche descriptive, **rattachable facultativement** à un compte UAfricas ; le rattachement ne fait que rendre le nom cliquable vers le profil public et ne confère aucun droit.
  - **Q3 → A / FR-002** : le bandeau textuel « en cours de diffusion / à suivre » **reste** dans la section de vitrine, l'exigence « aucun média lisible » ne visant que les lecteurs et vignettes.
- Zones d'ombre tranchées par hypothèse documentée plutôt que par question : visibilité publique du contact d'un membre, seuils de troncature, conservation du contenu vedette en tête de l'espace Télé, conservation des champs « info animateur » / « info producteur » en base.
- La parité Radio est portée par une exigence unique (FR-060) plutôt que dupliquée sur chaque item, ce qui garde les exigences vérifiables sans les doubler.
- Effet de bord relevé et consigné en hypothèse : les réactions et le partage aujourd'hui attachés à l'épisode mis en avant dans la section perdent leur cible avec le retrait des vidéos, et sont reportés sur les pages de détail.
- **Itération 3 (2026-08-10, après `/speckit-analyze`)** : deux exigences ajoutées, issues de trouvailles de sévérité HAUTE sur le code existant.
  - **FR-008** — la liste de programmes d'une section est aujourd'hui plafonnée à 12 par défaut (30 au maximum) et aucune page ne transmet ce paramètre. Le plafond bornait un aperçu d'épisodes ; il borne désormais le contenu principal de la section. La troncature silencieuse est explicitement proscrite : annoncer le total et mener au reste. SC-008 et le cas limite « plus de 30 programmes » sont reformulés en conséquence.
  - **FR-034** — les deux pages de programme affichent une ligne héritée « Animation : … · Production : … » issue de `info_animateur`/`info_producteur`. Conservée à côté du nouveau bloc d'équipe, elle offrirait deux sources concurrentes pour la même information. L'affichage public est retiré ; les colonnes restent en base et lisibles en back-office, sous un libellé « hérité ».
  - La distinction FR-003 (ellipse en vitrine) / FR-021 (dépliage sur les pages de détail) est désormais explicite, et le composant `TexteRepliable` porte un mode pour chacune.

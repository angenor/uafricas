# Research : Pays d'origine des salles publiques Afrolang

**Phase 0** : Résolution des inconnues techniques avant design.

## R1 : Modèle de relation N-N : table de jointure ou tableau natif ?

**Decision** : Table de jointure dédiée `afrolang.salle_pays_origine(salle_id, pays_id, created_at)` avec PK composite `(salle_id, pays_id)` + FK ON DELETE CASCADE des deux côtés.

**Rationale** :
- Pattern **déjà éprouvé** dans le monorepo : `marketplace.annonce_pays` (référence à 100 % similaire). Réutiliser garantit cohérence et zéro courbe d'apprentissage.
- PK composite ⇒ unicité **gratuite** (FR-002), pas besoin d'index supplémentaire.
- FK `ON DELETE CASCADE` côté `shared.pays(id)` ⇒ FR-010 satisfait sans trigger ni job de nettoyage.
- Permet `EXISTS (SELECT 1 FROM ... WHERE pays_id = $X)` performant pour le filtre public (FR-006).
- Auditable : table physique → entrées dans `audit_log` claires.

**Alternatives considered** :
- Colonne `pays_origine_ids UUID[]` directement sur `afrolang.salle` : plus compact mais (a) pas d'intégrité référentielle vers `shared.pays`, (b) impossible d'utiliser les FK CASCADE pour FR-010, (c) requêtes de filtre nécessitent `pays_origine_ids @> ARRAY[$X]` moins lisible et moins indexable, (d) rompt l'homogénéité avec `annonce_pays`. **Rejeté**.
- Table avec colonne `ordre` explicite : pas d'intérêt, l'ordre d'affichage est dérivé du nom (`ORDER BY p.nom`). YAGNI. **Rejeté**.

## R2 : Comment renvoyer la liste de pays dans le payload public sans N+1 ?

**Decision** : `array_agg` corrélé dans le SELECT principal de `lister_salles`, désérialisé en `Vec<PaysOrigineLight>` via `sqlx::types::Json` ou directement `serde_json::Value` puis cast.

```sql
SELECT
  ...,
  COALESCE(
    (SELECT json_agg(json_build_object(
       'id', p.id,
       'nom', p.nom,
       'code_iso2', p.code_iso2
     ) ORDER BY p.nom)
     FROM afrolang.salle_pays_origine spo
     JOIN shared.pays p ON p.id = spo.pays_id
     WHERE spo.salle_id = s.id AND p.actif = TRUE),
    '[]'::json
  ) AS pays_origine
FROM afrolang.salle s
WHERE ...
```

**Rationale** :
- Une seule requête, pas de N+1 (SC-004 : ≤ 110 % du temps actuel).
- Filtre `p.actif = TRUE` directement dans la sous-requête ⇒ Q3 (pays archivés masqués côté public) appliqué naturellement.
- `ORDER BY p.nom` ⇒ ordre stable alphabétique (FR-003).
- `COALESCE … '[]'::json` ⇒ jamais `NULL`, toujours un tableau vide ⇒ FR-009 satisfait sans condition spéciale côté Rust.

**Alternatives considered** :
- 2ᵉ requête après `lister_salles` pour récupérer les pays par `WHERE salle_id = ANY($1)` puis grouper côté Rust : plus de code Rust, deux allers-retours réseau. **Rejeté** : la version `json_agg` est l'approche la plus propre pour PostgreSQL.

## R3 : Filtre public `?pays_id=` (mono-valué)

**Decision** : Ajouter dans `SalleFiltres` (Rust) un `Option<Uuid> pays_id`. Quand renseigné, ajouter à `conditions` :

```sql
EXISTS (
  SELECT 1 FROM afrolang.salle_pays_origine spo
  JOIN shared.pays p ON p.id = spo.pays_id
  WHERE spo.salle_id = s.id AND spo.pays_id = $X AND p.actif = TRUE
)
```

**Rationale** :
- `EXISTS` indexable via la PK composite `(salle_id, pays_id)` ⇒ scan ciblé.
- Inclusion du `p.actif = TRUE` ⇒ filtre par un pays archivé renvoie 0 résultat (cohérent avec Q3).
- Mono-valué (Q2) ⇒ pas de parsing CSV, query string simple.

**Alternatives considered** :
- `JOIN` direct au lieu de `EXISTS` : risque de doublons si plusieurs pays matchent (impossible ici en mono mais fragile). **Rejeté**.

## R4 : Endpoints admin : où placer les routes ?

**Decision** : Sous `/api/admin/afrolang/salles/{id}/pays` (POST) et `/api/admin/afrolang/salles/{id}/pays/{pays_id}` (DELETE). Handlers dans `src/handlers/admin/salles.rs` (fichier existant pour les salles publiques admin).

**Rationale** :
- Cohérent avec `/api/admin/annonces/{id}/pays` et `/api/admin/annonces/{id}/pays/{pays_id}`.
- Permission existante `verifier_permission!(admin, "afrolang", "modifier")` (déjà utilisée pour `modifier_salle`) ⇒ Principe IV satisfait sans nouveau rôle (Q1 confirme : pas de pré-remplissage, mais permission inchangée).
- Aucune route GET dédiée : la liste des pays d'origine pour la vue admin est déjà dans `obtenir_salle` une fois le champ `pays_origine` ajouté à `SalleDetailResponse`.

**Alternatives considered** :
- Endpoint PUT « bulk » (`PUT /pays` avec un tableau d'UUID qui remplace) : plus simple côté UI mais moins clair côté audit (granularité « N ajouts + M retraits » plutôt qu'un diff). **Rejeté** par alignement strict avec `annonce_pays` (Principe V).

## R5 : UI carte : règle d'affichage 1-3 vs 4+

**Decision** : Composable de calcul dans `SalleCard.vue` :

```ts
const paysAffiches = computed(() => salle.pays_origine ?? [])
const modeCompact = computed(() => paysAffiches.value.length >= 4)
const tooltipPays = computed(() => paysAffiches.value.map(p => p.nom).join(', '))
```

Rendu :
- `paysAffiches.length === 0` ⇒ ne rien afficher (pas d'espace réservé).
- `1..3` ⇒ chips horizontales : drapeau (emoji ISO2 ou `<font-awesome-icon>` placeholder) + nom court.
- `≥ 4` ⇒ rangée de drapeaux uniquement, `:title="tooltipPays"` + `aria-label` pour accessibilité.

**Rationale** :
- Pas de dépendance externe pour les drapeaux : utilisation de l'emoji régional ISO 3166-1 alpha-2 dérivé du `code_iso2` (`String.fromCodePoint(0x1F1E6 + code.charCodeAt(0)-65, ...)`). Cohérent avec d'autres pages publiques du projet (Tailwind v4 pur, Principe VI).
- Repli gracieux (FR-004) : si `code_iso2` manquant, afficher le nom seul.

**Alternatives considered** :
- Bibliothèque `country-flag-icons` ou `flag-icon-css` : ajoute 100+ Ko de SVG/CSS pour un usage marginal. **Rejeté** (Principe V, pas de nouvelle dépendance).
- Tooltip via daisyUI `tooltip` class : interdit côté public (Principe VI). Utilisation de l'attribut HTML natif `title=` + classe Tailwind si besoin d'un style custom plus tard.

## R6 : Migration des salles existantes (Q1 : aucune)

**Decision** : Aucun script de seed, aucun `INSERT` dans la migration. La table `afrolang.salle_pays_origine` est créée vide. Documentation en tête du DDL : « Enrichissement éditorial 100 % manuel (cf. spec.md Q1). »

**Rationale** :
- Q1 explicite. Pré-remplir depuis `groupe_ethnique → fiche_pays → pays` ferait une supposition que l'utilisateur a explicitement rejetée.
- Réduit le risque de migration : pas de transaction longue, pas d'effet de bord sur un schéma `country_profile` étranger.

**Alternatives considered** : Aucune : décision prise au moment du `/speckit.clarify`.

## R7 : Audit : granularité

**Decision** : 1 entrée d'audit par appel API (1 ajout = 1 ligne, 1 retrait = 1 ligne). Action = `CREATE` ou `DELETE`, table = `salle_pays_origine`, entity_id = `salle_id`. Pas de payload before/after (le couple `(salle_id, pays_id)` est dans l'URL et trivialement reconstructible depuis l'audit log via les autres champs).

**Rationale** : Identique à `marketplace.annonce_pays` (cf. `ajouter_pays_annonce` lignes 666-677 de `handlers/admin/annonces.rs`). Principe V (homogénéité) + Principe VII (audit présent).

## Synthèse

Toutes les décisions techniques s'appuient sur des patterns **déjà en production** dans le monorepo (`annonce_pays`). Aucune NEEDS CLARIFICATION résiduelle. Aucune nouvelle dépendance. Surface de code minimale : 1 fichier SQL touché, 3 fichiers Rust touchés (1 handler public, 1 handler admin, 1 modèle), 1 fichier de routes, 5 fichiers frontend touchés.

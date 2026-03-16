# Research: Modèle de données des personnes et liens familiaux

**Branch**: `001-personnes-arbre` | **Date**: 2026-03-15

## Décision 1 — Schema PostgreSQL bounded-context

**Décision**: Créer un nouveau schema `arbre_genealogique` (fichier `23_arbre_genealogique.sql`).

**Rationale**: Aucun des 10 schemas existants ne couvre la généalogie :
- `iam` → identité & accès (utilisateurs, rôles)
- `retrouve_amis` → recherche de personnes perdues (avis, correspondances)
- `country_profile` → fiches pays
- Les autres → marchés, innovations, culture, médias, gouvernance, échanges, langues

La généalogie est un bounded context distinct avec ses propres entités (Personne réelle, Arbre, Rattachement, Lien familial) et ses propres règles métier (cycle detection, cascade soft delete). Créer un schema dédié est conforme au Principe III (SQL source de vérité) et à l'architecture Monolith-First du projet.

**Alternatives considérées**:
- Attacher à `retrouve_amis` : rejeté — le contexte "retrouve amis" concerne la recherche de personnes perdues, pas la modélisation d'arbres. Mélanger les deux compliquerait les futurs microservices.
- Attacher à `iam` : rejeté — `iam` gère l'identité système (authentification, rôles), pas les données biographiques ancestrales.

---

## Décision 2 — Représentation des dates à granularité variable

**Décision**: 3 colonnes SMALLINT nullable par date (`naissance_annee`, `naissance_mois`, `naissance_jour` / `deces_annee`, `deces_mois`, `deces_jour`).

**Rationale**: Pour les données généalogiques ancestrales, la date complète est rarement connue (ex: arrière-grand-père né "vers 1850"). Trois options évaluées :

- **Option A — Colonne TEXT** (`"1850"`, `"03/1850"`, `"15/03/1850"`) : flexible mais impossible à valider, comparer ou indexer efficacement.
- **Option B — 3 colonnes SMALLINT** ← Choisie : permet de valider chaque composante (mois 1-12, jour 1-31), de comparer les dates partielles (ex: comparer les années pour FR-004), et d'indexer. Compatible avec la convention SQL du projet.
- **Option C — Colonne DATE + colonne précision** (`precision = 'annee'|'mois'|'jour'`) : élégante mais introduit une redondance (la précision est déductible des colonnes non-NULL) et complique les queries.

**Contrainte de validation** (FR-004) : quand `deces_annee IS NOT NULL AND naissance_annee IS NOT NULL`, vérifier `deces_annee >= naissance_annee`. Si les années sont égales et les mois sont connus, comparer les mois. Si toujours égaux et jours connus, comparer les jours. Sinon, ne pas bloquer.

---

## Décision 3 — Détection des cycles dans la hiérarchie parent-enfant

**Décision**: Requête récursive CTE PostgreSQL avant insertion d'un lien parent-enfant.

**Rationale**: FR-009 interdit les cycles (A ancêtre de B et B ancêtre de A). La détection naïve (vérifier uniquement le lien direct) ne couvre pas les cycles sur N générations. La Recursive CTE est la solution standard PostgreSQL pour traverser des graphes :

```sql
WITH RECURSIVE ancetres AS (
    SELECT rattachement_a_id FROM arbre_genealogique.liens_familiaux
    WHERE rattachement_b_id = $nouveau_enfant_id
      AND type_lien IN ('pere', 'mere', 'parent')
      AND deleted_at IS NULL
    UNION ALL
    SELECT lf.rattachement_a_id FROM arbre_genealogique.liens_familiaux lf
    INNER JOIN ancetres a ON lf.rattachement_b_id = a.rattachement_a_id
    WHERE lf.type_lien IN ('pere', 'mere', 'parent') AND lf.deleted_at IS NULL
)
SELECT COUNT(*) FROM ancetres WHERE rattachement_a_id = $nouveau_parent_id;
```

Si count > 0 → cycle détecté → retourner 422 Unprocessable Entity.

**Alternatives considérées**:
- Trigger PostgreSQL pour la détection : possible mais rend l'erreur difficile à traduire en message UX clair côté Rust. La logique en Rust est plus lisible et testable.
- Limite de profondeur (ex: max 50 générations) : rejeté — arbitraire, et la CTE s'arrête naturellement quand il n'y a plus de parent.

---

## Décision 4 — Soft delete en cascade pour la Personne orpheline

**Décision**: Logique applicative en Rust (dans le handler de suppression de rattachement), pas de trigger PostgreSQL.

**Rationale**: Quand un rattachement est supprimé et qu'aucun autre rattachement actif ne référence la Personne réelle, celle-ci doit être soft-deleted en cascade (avec ses liens familiaux). Deux approches :

- **Trigger PostgreSQL** : exécuté automatiquement, mais obscurcit la logique dans la couche SQL. Difficile à tester et à déboguer. La convention du projet préfère la logique dans les handlers.
- **Logique Rust** ← Choisie : sequence dans le handler `supprimer_personne` :
  1. Marquer le rattachement `deleted_at = NOW()`
  2. Supprimer (soft) les `liens_familiaux` impliquant ce rattachement
  3. Vérifier si d'autres rattachements actifs existent pour la même `personne_id`
  4. Si aucun → marquer la Personne réelle `deleted_at = NOW()`
  5. Tout dans une transaction sqlx pour garantir l'atomicité

**Note**: Le tout dans une transaction `BEGIN/COMMIT` — en cas d'erreur, rollback automatique.

---

## Décision 5 — Création automatique de l'arbre utilisateur

**Décision**: L'arbre de l'utilisateur est créé automatiquement (INSERT OR IGNORE / ON CONFLICT DO NOTHING) lors du premier appel à `POST /api/arbre/personnes`.

**Rationale**: Exposer un endpoint séparé `POST /api/arbre` obligerait le frontend à gérer une étape supplémentaire (créer l'arbre avant d'y ajouter une personne). L'utilisateur ne devrait pas avoir à "initialiser" son arbre manuellement — c'est un détail d'implémentation transparent.

**Implémentation** : Dans le handler `creer_personne` :
```sql
INSERT INTO arbre_genealogique.arbres (utilisateur_id)
VALUES ($utilisateur_id)
ON CONFLICT (utilisateur_id) DO NOTHING
RETURNING id;
```
Puis utiliser l'`arbre_id` (existant ou nouvellement créé) pour créer le `rattachement`.

---

## Décision 6 — Structure du Lien familial : directionnel vs symétrique

**Décision**: Lien directionnel avec `rattachement_source_id` (parent) → `rattachement_cible_id` (enfant) pour les liens parent-enfant. Lien symétrique stocké une seule fois pour les conjoints (avec convention `rattachement_a_id < rattachement_b_id` par UUID).

**Rationale**: Deux approches pour les liens bidirectionnels :
- **Deux lignes** (une dans chaque sens) : simplifie les queries mais double les données, complique les contraintes d'unicité.
- **Une ligne directionnelle** ← Choisie : une seule ligne `(parent → enfant)`. Le sens inverse est recalculé en lecture ("qui sont les enfants de A ?" = tous les `liens_familiaux` où `rattachement_source_id = A`). Pour les conjoints (symétrique), convention : `min(a, b)` en source et `max(a, b)` en cible pour garantir l'unicité.

**Pour les queries de lecture** (détail d'une personne) : récupérer les liens dans les deux sens avec un `OR` :
```sql
WHERE (rattachement_source_id = $id OR rattachement_cible_id = $id) AND deleted_at IS NULL
```

---

## Décision 7 — Nommage des colonnes de direction du lien

**Décision**: `rattachement_source_id` (le parent / le premier conjoint) et `rattachement_cible_id` (l'enfant / le second conjoint).

**Rationale**: Évite l'ambiguïté des noms `a` et `b` utilisés dans `retrouve_amis.blacklist`. Pour les liens parent-enfant, "source" = le parent qui donne et "cible" = l'enfant qui reçoit, ce qui est sémantiquement clair. Pour les conjoints, la convention min/max par UUID garantit l'unicité sans dépendre de l'ordre de saisie.

---

## Décision 8 — Requête SQL pour le matching inter-arbres (fondation future)

**Décision**: La séparation `Personne réelle / Rattachement` permet d'identifier les personnes partagées entre deux arbres avec une requête CTE simple, **sans aucune migration de schéma**.

**Requête type pour trouver les ancêtres communs entre deux utilisateurs** :

```sql
WITH personnes_communes AS (
    SELECT personne_id
    FROM arbre_genealogique.rattachements
    WHERE arbre_id IN ($arbre_a_id, $arbre_b_id)
      AND deleted_at IS NULL
    GROUP BY personne_id
    HAVING COUNT(DISTINCT arbre_id) = 2
)
SELECT p.id, p.nom, p.prenoms, p.naissance_annee, p.naissance_lieu
FROM arbre_genealogique.personnes p
JOIN personnes_communes pc ON p.id = pc.personne_id
WHERE p.deleted_at IS NULL;
```

**Exemple concret** : l'utilisateur A et l'utilisateur B ont tous les deux ajouté "Ibrahim Diallo, né en 1850 à Ségou" dans leur arbre respectif (deux `rattachements` distincts sur la même `personne.id`). La requête ci-dessus retourne "Ibrahim Diallo" comme ancêtre commun potentiel.

**Pourquoi aucune migration ne sera nécessaire** : le schéma actuel contient déjà :
1. `rattachements.arbre_id` — identifie l'arbre propriétaire du rattachement
2. `rattachements.personne_id` — référence vers la Personne réelle partageable
3. `UNIQUE(arbre_id, personne_id)` — garantit qu'un utilisateur ne peut rattacher la même personne réelle qu'une seule fois à son arbre

La future feature de matching n'aura besoin que d'ajouter une table `correspondances_arbres` pour stocker les connexions confirmées entre arbres, sans toucher aux tables existantes.

**Rationale**: Cette décision architecturale (inspirée de la clarification Q1 et Q2 dans `spec.md`) maintient la séparation des responsabilités : cette feature gère les arbres individuels, la future feature de matching exploitera les données existantes.

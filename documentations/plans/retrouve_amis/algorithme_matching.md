# Retrouve Amis : Algorithme de matching

## Vue d'ensemble

L'algorithme de matching est le coeur de la fonctionnalité. Il croise les informations de plusieurs sources pour détecter des correspondances entre :

1. **Avis ↔ Avis** : deux personnes se cherchent mutuellement.
2. **Avis ↔ Profil** : un avis correspond au profil d'un utilisateur inscrit (trouvable).

Le matching produit un **score de 0 à 100** avec un détail des critères qui ont matché.

---

## Sources de données pour le matching

### Source 1 : Critères d'un avis de recherche
Chaque avis contient N critères typés avec un poids :

```
Exemple : Avis de Jean qui cherche Paul :
  - nom: "Kamga" (poids: 5)
  - prenom: "Paul" (poids: 5)
  - ville: "Douala" (poids: 4)
  - ecole: "Lycée Joss" (poids: 3)
  - annee_debut: "1998" (poids: 2)
  - annee_fin: "2005" (poids: 2)
  - quartier: "Akwa" (poids: 1)
```

### Source 2 : Critères d'un autre avis (matching croisé)
Si Paul cherche aussi Jean, ses critères seront croisés :

```
Avis de Paul qui cherche Jean :
  - nom: "Mbarga" (poids: 5)
  - prenom: "Jean" (poids: 5)
  - ville: "Douala" (poids: 4)
  - ecole: "Lycee Joss" (poids: 3)
  - annee_fin: "2004" (poids: 2)
```

### Source 3 : Profil trouvable d'un utilisateur
Un utilisateur inscrit qui a activé "être trouvable" :

```
Préférences de Paul (utilisateur inscrit) :
  - nom actuel: "Kamga" (depuis iam.utilisateur)
  - prenom actuel: "Paul" (depuis iam.utilisateur)
  - anciens_noms: ["Kamga"]
  - anciennes_villes: ["Douala", "Yaoundé"]
  - anciennes_ecoles: ["Lycée Joss", "Université de Douala"]
  - periode: 1995 - 2010
```

---

## Algorithme de scoring

### Étape 1 : Sélection des candidats

Pour un avis A donné, on sélectionne les candidats potentiels :

**Candidats avis :**
```sql
-- Trouver les autres avis actifs qui partagent au moins un critère
SELECT DISTINCT b.avis_id
FROM retrouve_amis.critere_recherche a
JOIN retrouve_amis.critere_recherche b
  ON a.type_critere = b.type_critere
  AND a.avis_id != b.avis_id
  AND (
    -- Match exact sur valeur normalisée
    a.valeur_normalisee = b.valeur_normalisee
    OR
    -- Match fuzzy (similarité > 0.6)
    similarity(a.valeur_normalisee, b.valeur_normalisee) > 0.6
    OR
    -- Match full-text
    a.tsv @@ plainto_tsquery('french', b.valeur)
  )
WHERE a.avis_id = :avis_a_id
  AND b.avis_id IN (
    SELECT id FROM retrouve_amis.avis_recherche
    WHERE etat = 'actif' AND deleted_at IS NULL AND id != :avis_a_id
  )
```

**Candidats profils :**
```sql
-- Trouver les utilisateurs trouvables qui correspondent
SELECT p.utilisateur_id
FROM retrouve_amis.preference_trouvabilite p
JOIN iam.utilisateur u ON u.id = p.utilisateur_id
WHERE p.est_trouvable = true
  AND u.deleted_at IS NULL
  AND u.etat = 'actif'
  AND (
    -- Le nom/prénom de l'utilisateur matche un critère 'nom' ou 'prenom'
    EXISTS (
      SELECT 1 FROM retrouve_amis.critere_recherche c
      WHERE c.avis_id = :avis_a_id
        AND c.type_critere IN ('nom', 'prenom')
        AND (
          similarity(c.valeur_normalisee, lower(u.nom)) > 0.6
          OR similarity(c.valeur_normalisee, lower(u.prenom)) > 0.6
          OR c.valeur_normalisee = ANY(
            SELECT lower(unnest(p.anciens_noms))
          )
        )
    )
    -- Ou une ville/école correspond
    OR EXISTS (
      SELECT 1 FROM retrouve_amis.critere_recherche c
      WHERE c.avis_id = :avis_a_id
        AND c.type_critere IN ('ville', 'ecole')
        AND (
          c.valeur_normalisee = ANY(
            SELECT lower(unnest(p.anciennes_villes))
          )
          OR c.valeur_normalisee = ANY(
            SELECT lower(unnest(p.anciennes_ecoles))
          )
        )
    )
  )
```

**Prérequis PostgreSQL :** Extension `pg_trgm` pour la fonction `similarity()`.

```sql
CREATE EXTENSION IF NOT EXISTS pg_trgm;
```

### Étape 2 : Calcul du score par candidat

Pour chaque candidat, on calcule un score composite :

```
Score = Σ (score_critère × poids_critère) / Σ poids_critère × 100
```

#### Scores par type de critère

| Type | Méthode de comparaison | Score max |
|------|------------------------|-----------|
| `nom` | Similarité trigram + Soundex | 100 |
| `prenom` | Similarité trigram + Soundex | 100 |
| `surnom` | Similarité trigram | 100 |
| `ville` | Match exact normalisé | 100 (exact) / 70 (fuzzy) |
| `quartier` | Match exact normalisé | 100 (exact) / 60 (fuzzy) |
| `pays` | Match exact (UUID pays) | 100 |
| `ecole` | Similarité trigram | 100 (exact) / 80 (fuzzy > 0.8) / 60 (fuzzy > 0.6) |
| `universite` | Similarité trigram | 100 / 80 / 60 |
| `entreprise` | Similarité trigram | 100 / 80 / 60 |
| `annee_debut` | Proximité (±2 ans = 100, ±5 ans = 70, ±10 ans = 40) | 100 |
| `annee_fin` | Proximité (même logique) | 100 |
| `tranche_age` | Chevauchement de tranches | 100 (même) / 70 (adjacent) |
| `description` | Full-text search | 80 max |
| `anecdote` | Non comparée (usage post-match) | 0 |

#### Algorithme détaillé

```rust
/// Calcule le score de correspondance entre deux ensembles de critères
fn calculer_score(
    criteres_a: &[CritereRechercheRow],
    criteres_b: &[CritereRechercheRow],  // ou données du profil
) -> (i32, Vec<CritereCommun>) {
    let mut total_poids = 0;
    let mut total_score_pondere = 0.0;
    let mut criteres_communs = Vec::new();

    for ca in criteres_a {
        // Ignorer les anecdotes (pas pour le matching)
        if ca.type_critere == "anecdote" { continue; }

        // Chercher le meilleur match dans les critères B
        let mut meilleur_score = 0;
        let mut meilleure_valeur_b = String::new();

        for cb in criteres_b {
            if ca.type_critere != cb.type_critere { continue; }

            let score = match ca.type_critere.as_str() {
                "nom" | "prenom" | "surnom" => {
                    score_nom(&ca.valeur_normalisee, &cb.valeur_normalisee)
                }
                "ville" | "quartier" => {
                    score_lieu(&ca.valeur_normalisee, &cb.valeur_normalisee)
                }
                "pays" => {
                    if ca.pays_id == cb.pays_id { 100 } else { 0 }
                }
                "ecole" | "universite" | "entreprise" => {
                    score_institution(&ca.valeur_normalisee, &cb.valeur_normalisee)
                }
                "annee_debut" | "annee_fin" => {
                    score_annee(&ca.valeur, &cb.valeur)
                }
                "tranche_age" => {
                    score_tranche_age(&ca.valeur, &cb.valeur)
                }
                "description" => {
                    score_texte_libre(&ca.valeur, &cb.valeur)
                }
                _ => 0,
            };

            if score > meilleur_score {
                meilleur_score = score;
                meilleure_valeur_b = cb.valeur.clone();
            }
        }

        if meilleur_score > 0 {
            total_poids += ca.poids;
            total_score_pondere += (meilleur_score as f64) * (ca.poids as f64);
            criteres_communs.push(CritereCommun {
                type_critere: ca.type_critere.clone(),
                valeur_a: ca.valeur.clone(),
                valeur_b: meilleure_valeur_b,
                score: meilleur_score,
            });
        }
    }

    let score_global = if total_poids > 0 {
        (total_score_pondere / (total_poids as f64 * 100.0) * 100.0) as i32
    } else {
        0
    };

    (score_global, criteres_communs)
}
```

### Étape 3 : Fonctions de scoring détaillées

#### Noms et prénoms

```rust
fn score_nom(a: &str, b: &str) -> i32 {
    // 1. Match exact
    if a == b { return 100; }

    // 2. Similarité trigram (pg_trgm)
    let sim = trigram_similarity(a, b);
    if sim > 0.85 { return 95; }
    if sim > 0.7  { return 80; }
    if sim > 0.5  { return 60; }

    // 3. Soundex (phonétique) : utile pour les noms africains
    //    avec des orthographes variées
    if soundex_fr(a) == soundex_fr(b) { return 70; }

    // 4. Contenance (un nom contient l'autre)
    if a.contains(b) || b.contains(a) { return 50; }

    0
}
```

#### Soundex francophone

L'algorithme Soundex classique est conçu pour l'anglais. Pour les noms africains et francophones, on utilise une adaptation :

```rust
fn soundex_fr(nom: &str) -> String {
    // Adaptation du Soundex pour le français et les langues africaines
    // Règles spécifiques :
    // - "ou" → "u"  (Douala vs Duala)
    // - "ph" → "f"
    // - "th" → "t"
    // - "ch" → "s" ou "k" selon contexte
    // - Voyelles nasales : "an" = "en", "on" = "om", "in" = "im"
    // - Doublement de consonnes ignoré : "nn" = "n", "mm" = "m"
    // - "gn" → "n"
    // - "qu" → "k"
    //
    // Exemples de correspondances attendues :
    // - "Kamga" ↔ "Camga" → même code
    // - "Nkoulou" ↔ "Nkullu" → même code
    // - "Tchamba" ↔ "Chamba" → même code
    // - "Douala" ↔ "Duala" → même code
}
```

#### Lieux (villes, quartiers)

```rust
fn score_lieu(a: &str, b: &str) -> i32 {
    if a == b { return 100; }

    let sim = trigram_similarity(a, b);
    if sim > 0.8 { return 90; }
    if sim > 0.6 { return 70; }

    // Gestion des variantes connues
    // Ex: "N'Djamena" ↔ "Ndjamena" ↔ "N Djamena"
    let a_clean = a.replace(['\'', '-', ' '], "");
    let b_clean = b.replace(['\'', '-', ' '], "");
    if a_clean == b_clean { return 95; }

    0
}
```

#### Années

```rust
fn score_annee(a: &str, b: &str) -> i32 {
    let ya: i32 = a.parse().unwrap_or(0);
    let yb: i32 = b.parse().unwrap_or(0);
    if ya == 0 || yb == 0 { return 0; }

    let diff = (ya - yb).abs();
    match diff {
        0 => 100,
        1..=2 => 90,
        3..=5 => 70,
        6..=10 => 40,
        _ => 0,
    }
}
```

#### Institutions (écoles, universités, entreprises)

```rust
fn score_institution(a: &str, b: &str) -> i32 {
    if a == b { return 100; }

    // Retirer les mots courants non discriminants
    let mots_ignores = ["lycee", "college", "ecole", "universite",
                         "de", "du", "des", "la", "le", "les", "d"];

    let tokens_a: Vec<&str> = a.split_whitespace()
        .filter(|w| !mots_ignores.contains(w))
        .collect();
    let tokens_b: Vec<&str> = b.split_whitespace()
        .filter(|w| !mots_ignores.contains(w))
        .collect();

    // Intersection des tokens significatifs
    let communs = tokens_a.iter()
        .filter(|t| tokens_b.contains(t))
        .count();

    let max_tokens = tokens_a.len().max(tokens_b.len());
    if max_tokens == 0 { return 0; }

    let ratio = (communs as f64) / (max_tokens as f64);
    (ratio * 100.0) as i32
}
```

---

## Seuils de décision

| Score | Interprétation | Action |
|-------|----------------|--------|
| 80-100 | Très probable | Correspondance créée, notifiée en priorité |
| 60-79 | Probable | Correspondance créée, notifiée normalement |
| 40-59 | Possible | Correspondance créée, affichée mais pas notifiée |
| 0-39 | Peu probable | Correspondance NON créée |

**Seuil minimum :** 40 points. En dessous, pas de correspondance créée.

**Condition supplémentaire :** Au moins 2 critères doivent avoir matché pour créer une correspondance (évite les faux positifs sur un seul critère commun comme "ville = Douala").

---

## Matching croisé (avis ↔ avis)

Le cas le plus intéressant : A cherche B ET B cherche A.

```
Avis de Jean (cherche Paul) :          Avis de Paul (cherche Jean) :
  - nom: "Kamga"                          - nom: "Mbarga"
  - prenom: "Paul"                        - prenom: "Jean"
  - ville: "Douala"                       - ville: "Douala"
  - ecole: "Lycée Joss"                   - ecole: "Lycee Joss"
```

Le matching croisé compare :
1. Les critères de l'avis de Jean avec ceux de l'avis de Paul **ET** avec l'identité du déposant de l'avis de Paul.
2. Réciproquement.

```
Scoring croisé :
  - Le nom cherché par Jean ("Kamga") → correspond au nom réel de Paul ?
    → Vérifié contre iam.utilisateur si Paul est trouvable
  - Le prénom cherché par Jean ("Paul") → correspond au prénom de Paul ?
  - La ville est commune aux deux avis → +score
  - L'école est commune → +score

  Score croisé = max(score_A→B, score_B→A)
```

---

## Exécution du matching

### Déclenchement

| Événement | Action |
|-----------|--------|
| Création d'un avis | Matching immédiat (async, tokio::spawn) |
| Modification de critères | Re-matching (async) |
| Activation de trouvabilité | Matching contre tous les avis actifs |
| Job périodique (toutes les heures) | Re-matching des avis actifs sans correspondance |

### Performance

Pour éviter des requêtes coûteuses :

1. **Pré-filtre SQL** : Ne considérer que les candidats partageant au moins un critère (via index sur `type_critere + valeur_normalisee`).
2. **Calcul en Rust** : Le scoring détaillé est fait côté applicatif (pas en SQL).
3. **Limit** : Max 50 candidats par exécution (triés par nombre de critères communs).
4. **Cache** : Ne pas recalculer si aucun critère n'a changé depuis le dernier matching.

### Structure du job

```rust
// src/services/matching.rs

pub async fn executer_matching(pool: &PgPool, avis_id: Uuid) -> Result<MatchingResult, ApiErreur> {
    let debut = std::time::Instant::now();

    // 1. Charger les critères de l'avis
    let criteres = charger_criteres(pool, avis_id).await?;

    // 2. Trouver les candidats (avis + profils)
    let candidats_avis = trouver_candidats_avis(pool, avis_id, &criteres).await?;
    let candidats_profils = trouver_candidats_profils(pool, avis_id, &criteres).await?;

    // 3. Calculer les scores
    let mut correspondances = Vec::new();

    for candidat in candidats_avis {
        let (score, details) = calculer_score(&criteres, &candidat.criteres);
        if score >= 40 && details.len() >= 2 {
            correspondances.push((candidat, score, details));
        }
    }

    for profil in candidats_profils {
        let criteres_profil = convertir_profil_en_criteres(&profil);
        let (score, details) = calculer_score(&criteres, &criteres_profil);
        if score >= 40 && details.len() >= 2 {
            correspondances.push((profil, score, details));
        }
    }

    // 4. Créer les correspondances en base
    let mut creees = 0;
    for (candidat, score, details) in &correspondances {
        // Vérifier qu'elle n'existe pas déjà
        if !correspondance_existe(pool, avis_id, candidat.id).await? {
            creer_correspondance(pool, avis_id, candidat, *score, details).await?;
            creees += 1;
        }
    }

    // 5. Logger dans journal_matching
    let duree = debut.elapsed().as_millis() as i32;
    log_matching(pool, avis_id, candidats_avis.len() + candidats_profils.len(), creees, duree).await?;

    Ok(MatchingResult { candidats: correspondances.len(), creees })
}
```

---

## Extensions futures (Phase 3)

### Matching par réseau social
Si A cherche B et B a des contacts communs avec C qui est inscrit :
```
A → cherche B
C → connaît B (indiqué dans son profil)
Résultat : Notification à A que C pourrait connaître B
```

### Matching géographique
Croisement avec les informations de `country_profile` :
- Villes connues par pays
- Quartiers référencés
- Écoles/universités du pays

### Apprentissage des succès
Analyser les correspondances validées (score, types de critères) pour affiner les poids et seuils de matching.

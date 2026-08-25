# Data Model : Phase 1 : schéma `engagement`

Source de vérité = SQL (Principe III). Nouveau schéma bounded-context **`engagement`**. Migration idempotente unique : `uafricas_backend/doc/bd/schemas/NN_engagement.sql` (prochain numéro libre, ≈ `33`), branchée dans l'orchestrateur `schema.sql` via `\ir`. Conventions : UUID v4 PK, TIMESTAMPTZ, snake_case français, enums PostgreSQL, `IF NOT EXISTS` / blocs `DO` conditionnels pour l'idempotence.

> **Soft-delete** : volontairement **absent** de `compte` (entité 1‑1 vivant avec l'utilisateur) et de `mouvement_points` (journal **append-only**, immuable, jamais supprimé). Les tables de config (`regle_points`, `palier_popularite`, `niveau`) utilisent un drapeau `actif` plutôt que `deleted_at` (une règle désactivée reste référencée par l'historique). Écart au Principe III justifié : aucune de ces entités n'est un contenu utilisateur supprimable.

---

## Entités

### 1. `engagement.compte` : compte d'engagement (1‑1 utilisateur)

| Colonne | Type | Contraintes | Rôle |
|---------|------|-------------|------|
| `utilisateur_id` | UUID | **PK**, FK → `iam.utilisateur(id)` ON DELETE CASCADE | Titulaire |
| `solde_points` | INTEGER | NOT NULL DEFAULT 0, `CHECK (solde_points >= 0)` | Solde global (plancher 0, D7) |
| `solde_points_mensuel` | INTEGER | NOT NULL DEFAULT 0 | Solde du mois courant |
| `mois_courant` | DATE | NOT NULL DEFAULT `date_trunc('month', now())` | Mois de référence du solde mensuel (reset paresseux D5) |
| `reputation` | INTEGER | NOT NULL DEFAULT 0 | Score de confiance (signé, indépendant, D7) |
| `niveau_code` | VARCHAR(30) | NOT NULL DEFAULT `'membre'` | Niveau dérivé dénormalisé (D4) |
| `dernier_mouvement_at` | TIMESTAMPTZ | NULL | Date du dernier mouvement |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT now() | |

- Le compte est **créé paresseusement** au premier mouvement (`INSERT ... ON CONFLICT (utilisateur_id) DO ...`), pas besoin de back-fill au lancement (non-rétroactif, FR-024).

### 2. `engagement.mouvement_points` : journal (append-only)

| Colonne | Type | Contraintes | Rôle |
|---------|------|-------------|------|
| `id` | UUID | PK DEFAULT uuid_generate_v4() | |
| `utilisateur_id` | UUID | NOT NULL, FK → `iam.utilisateur(id)` ON DELETE CASCADE | Bénéficiaire du mouvement |
| `type_action` | VARCHAR(50) | NOT NULL | Réfère `regle_points.type_action` |
| `type_objet` | VARCHAR(40) | NULL | `codimoi` / `factcheck` / `video` / `ideaforce` / `bad_habit` / `biblio_humaine` / `fiche_pays` |
| `objet_id` | UUID | NULL | Identifiant de l'objet source |
| `points` | INTEGER | NOT NULL | Delta réellement appliqué au solde (signé, après plancher/écrêtage) |
| `reputation_delta` | INTEGER | NOT NULL DEFAULT 0 | Delta appliqué à la réputation |
| `solde_apres` | INTEGER | NOT NULL | Snapshot du solde global après application |
| `plafond_atteint` | BOOLEAN | NOT NULL DEFAULT false | Vrai si le gain a été écrêté (D6) |
| `cle_idempotence` | TEXT | **UNIQUE** | Empêche le doublon (D2) |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT now() | |

Index : `idx_mouvement_utilisateur (utilisateur_id, created_at DESC)`, `idx_mouvement_type_action (type_action, created_at)` (pour le calcul des plafonds).

**Règles** :
- Insertion en `ON CONFLICT (cle_idempotence) DO NOTHING` : si aucune ligne insérée, le compte n'est pas modifié.
- Ligne **immuable** : jamais d'UPDATE/DELETE. Une correction admin est un **nouveau** mouvement (`type_action = 'ajustement_admin'`).

### 3. `engagement.regle_points` : barème paramétrable

| Colonne | Type | Contraintes | Rôle |
|---------|------|-------------|------|
| `id` | UUID | PK | |
| `type_action` | VARCHAR(50) | **UNIQUE** NOT NULL | Clé métier |
| `libelle` | VARCHAR(150) | NOT NULL | Libellé admin |
| `points` | INTEGER | NOT NULL | Montant (signé : négatif pour un malus) |
| `reputation_delta` | INTEGER | NOT NULL DEFAULT 0 | Impact réputation |
| `plafond_journalier` | INTEGER | NULL | Max de points/jour pour ce type (NULL = illimité) |
| `plafond_mensuel` | INTEGER | NULL | Max de points/mois pour ce type |
| `actif` | BOOLEAN | NOT NULL DEFAULT true | |
| `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT now() | |

**Seed initial (valeurs indicatives, paramétrables)** :

| type_action | libelle | points | reputation_delta | plafonds |
|-------------|---------|:------:|:----------------:|----------|
| `contribution_validee` | Contribution validée par modération | +2 | 0 |, |
| `contribution_mise_en_avant` | Contribution mise en avant par l'équipe | +5 | 0 |, |
| `factcheck_valide` | FactCheck jugé correct | +3 | +1 |, |
| `factcheck_faux` | FactCheck jugé faux/abusif | −2 | −3 |, |
| `popularite_palier` | Palier de popularité franchi | *(voir palier)* | 0 |, |
| `ajustement_admin` | Correction manuelle admin | *(variable)* | 0 |, |

> `popularite_palier` : le montant réel provient de `palier_popularite.points` ; la règle sert de libellé/traçabilité commune. `ajustement_admin` : montant saisi par l'admin, non plafonné.

### 4. `engagement.palier_popularite` : paliers de « j'aime »

| Colonne | Type | Contraintes |
|---------|------|-------------|
| `id` | UUID | PK |
| `seuil_likes` | INTEGER | **UNIQUE** NOT NULL, `CHECK (seuil_likes > 0)` |
| `points` | INTEGER | NOT NULL |
| `actif` | BOOLEAN | NOT NULL DEFAULT true |

**Seed** : `(100, +10)`, `(500, +30)`, `(1000, +50)`.

### 5. `engagement.niveau` : seuils de statut

| Colonne | Type | Contraintes |
|---------|------|-------------|
| `id` | UUID | PK |
| `code` | VARCHAR(30) | **UNIQUE** NOT NULL (`membre` / `premium` / `platinum`) |
| `libelle` | VARCHAR(80) | NOT NULL |
| `seuil_min` | INTEGER | NOT NULL (solde minimal pour entrer dans le niveau) |
| `ordre` | SMALLINT | NOT NULL (tri croissant) |
| `badge_couleur` | VARCHAR(20) | NULL (indice UI) |
| `badge_icone` | VARCHAR(40) | NULL (nom FontAwesome) |

**Seed** : `(membre, « Membre », 0, 1)`, `(premium, « Membre Premium », 200, 2)`, `(platinum, « Influenceur Platinum », 1000, 3)`.

Le niveau d'un solde = ligne active de plus grand `seuil_min` tel que `seuil_min <= solde_points` (`recalculer_niveau`, D4).

---

## Contrat du service (`src/services/engagement.rs`)

Toutes non-bloquantes (erreurs loguées, jamais propagées, D1) :

```
attribuer(pool, utilisateur_id, type_action, type_objet, objet_id, cle_idempotence)
    → charge la règle active ; garde anti-auto-attribution assurée par l'appelant ;
      applique reset mensuel (D5) ; calcule l'écrêtage plafond (D6) ;
      INSERT mouvement ON CONFLICT DO NOTHING ; met à jour compte (solde, mensuel, réputation, dernier_mouvement) ;
      recalcule niveau_code (D4).

retirer(pool, utilisateur_id, type_action, type_objet, objet_id, cle_idempotence)
    → cas d'un malus (factcheck_faux) : points via GREATEST(0, solde+points_negatifs), réputation_delta appliqué sans plancher.

evaluer_popularite(pool, type_objet, objet_id, auteur_id, likes_count)
    → pour chaque palier_popularite actif tel que seuil_likes <= likes_count :
        attribuer(auteur_id, "popularite_palier", type_objet, objet_id,
                  cle = "popularite:{type_objet}:{objet_id}:{seuil_likes}", montant = palier.points)

recalculer_niveau(solde_points) -> niveau_code   // pur, testable
```

L'appelant fournit toujours `auteur_id` = auteur du contenu et garantit `auteur_id != acteur` pour les likes (anti-auto-attribution, FR-009). Pour les contributions/factcheck, le bénéficiaire est l'auteur ; le modérateur qui déclenche n'est jamais crédité.

---

## Intégration (call-sites à ajouter : non-bloquants)

Voir `research.md §D9` pour le tableau complet. Résumé des fichiers backend touchés (ajout d'un appel en fin de mutation réussie) :

- `handlers/admin/codimoi_admin.rs` : validation Codimoi.
- `handlers/admin/gouvernance.rs` : validation Ideaforce, BadHabit ; jugement FactCheck (validé/faux) ; mise en avant.
- `handlers/admin/vidafrica.rs` : `changer_etat_piste` (publie).
- Handlers de réaction « like » (Codimoi, FactCheck, biblio humaine, VidAfrica, fiche pays), appel `evaluer_popularite` après ajout d'un like.
- `routes.rs` : enregistrement des scopes `/api/engagement` (public) et `/api/admin/engagement` (admin).
- Référentiel IAM : seed de la permission admin `engagement`.

Aucune table existante n'est modifiée ; seuls des appels sont ajoutés. Le badge de statut à afficher « sous les contenus » (FR-019) est obtenu côté frontend via l'endpoint public `GET /api/engagement/niveau/{utilisateur_id}` (ou enrichissement ultérieur des réponses de contenu, décision d'implémentation, hors périmètre du modèle de données).

---

## Machine à états (niveau)

```
solde_points ∈ [0, 199]      → niveau_code = "membre"
solde_points ∈ [200, 999]    → niveau_code = "premium"
solde_points ≥ 1000          → niveau_code = "platinum"   (seuils = engagement.niveau, paramétrables)
```

Transitions déclenchées automatiquement à chaque mutation de `solde_points` (montée et descente), sans action utilisateur (FR-020). Aucun effet transactionnel autre que l'affichage du badge en Phase 1 (visibilité algorithmique reportée, FR-021).

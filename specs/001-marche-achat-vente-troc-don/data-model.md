# Phase 1 — Modèle de données : Marché Africain

Source de vérité : schéma PostgreSQL (Principe III). Les tables `marketplace.*` existent déjà ; cette feature introduit **deux ajustements** et **réutilise** le reste. Les DTOs Rust et interfaces TS doivent refléter fidèlement ce schéma.

## Changements de schéma

### C1 — Nouvelle valeur d'enum `etat_annonce` (`05_marketplace.sql`)

```sql
-- Ajout de l'état de conclusion (FR-018, clarification Q5)
ALTER TYPE marketplace.etat_annonce ADD VALUE IF NOT EXISTS 'conclue';
```

> Note : pour une base déjà initialisée, exécuter cet `ALTER TYPE` en migration manuelle (SSH+psql) avant déploiement. En dev (init auto via `docker-init.sh`), éditer directement la définition de l'enum dans `05_marketplace.sql` :
> `'brouillon', 'publiee', 'en_attente', 'expiree', 'suspendue', 'supprimee', 'conclue'`.

### C2 — Colonne `annonce_id` sur `social.conversation` (nouveau `30_social_conversation_annonce.sql`)

```sql
-- Contexte marketplace d'une conversation initiée depuis une annonce (D2).
-- Nullable, informationnelle, NE participe PAS à l'unicité de la paire.
ALTER TABLE social.conversation
    ADD COLUMN annonce_id UUID REFERENCES marketplace.annonce(id) ON DELETE SET NULL;

CREATE INDEX idx_conversation_annonce ON social.conversation(annonce_id)
    WHERE annonce_id IS NOT NULL;
```

## Entités (existantes — rappel des champs utilisés)

### `marketplace.annonce`
| Champ | Type | Règle pour le parcours membre |
|-------|------|-------------------------------|
| `id` | UUID PK | — |
| `titre` | VARCHAR(350) NOT NULL | obligatoire, 3..350 car. |
| `slug` | VARCHAR(400) UNIQUE | généré depuis le titre (réutiliser la logique admin) |
| `description` | TEXT NOT NULL | obligatoire, ≥ 10 car. |
| `type_operation` | enum | **membre limité à `vente` / `troc` / `don`** (FR-001) |
| `categorie_id` | UUID → `shared.categorie` | obligatoire (FR-002) |
| `condition_article` | enum | défaut `non_applicable` |
| `prix` | DECIMAL(15,2) | obligatoire si `vente`, sinon NULL/0 (FR-003) |
| `devise` | VARCHAR(5) | défaut `XOF` ; pertinent si `vente` |
| `prix_negociable` | BOOLEAN | défaut FALSE |
| `ville` / `adresse` | VARCHAR/TEXT | facultatifs |
| `longitude` / `latitude` | DECIMAL(10,7) | facultatifs |
| `type_contact` | enum | **forcé à `messagerie_plateforme`** (D2) |
| `contact_info` | VARCHAR(300) | non utilisé pour le membre (contact via messagerie) |
| `quantite` | INT | défaut 1, ≥ 1 |
| `etat` | enum | **forcé `publiee`** à la création membre (D4) ; transitions ci-dessous |
| `nombre_vues` | INT | incrémenté au détail (existant) |
| `cree_par` | UUID → `iam.utilisateur` | **= utilisateur courant** (jamais fourni par le client) |
| `expire_at` | TIMESTAMPTZ | **NULL** (pas d'expiration auto — D4) |
| `created_at` / `updated_at` / `deleted_at` | TIMESTAMPTZ | soft delete |

**Transitions d'état (membre)** :
```
(création) ───────────────▶ publiee
publiee ──[conclure]──────▶ conclue        (FR-018, retiré du public)
publiee/conclue ─[suppr.]─▶ supprimee + deleted_at   (FR-019, soft delete)
publiee ──[admin]─────────▶ suspendue / supprimee     (FR-023, modération)
```
Le membre ne peut agir que sur **ses** annonces (`cree_par = courant`, FR-020). Une annonce `conclue` ou `supprimee` n'apparaît plus dans le listing public (FR-010).

### `marketplace.annonce_media`
- ≤ 5 lignes par annonce (Q1). `media_url` pointe vers `/uploads/marketplace/annonces/<uuid>.<ext>`. `est_principale` : exactement une à TRUE. `ordre` : 0-based. `type_mime` ∈ {image/jpeg, image/png, image/webp}.

### `marketplace.annonce_pays`
- Paire (annonce_id, pays_id). Territoires ciblés (FR-002). Au moins 0..n.

### `marketplace.annonce_favori`
- PK (utilisateur_id, annonce_id). Ajout/retrait idempotent (`ON CONFLICT DO NOTHING` / `DELETE`). FR-021/FR-022.

### `social.conversation` (étendue)
- `+ annonce_id` (C2). Réutilise `obtenir_ou_creer_conversation` ; à la création depuis un contact d'annonce, renseigner `annonce_id` (COALESCE : ne pas écraser une valeur existante).

### `social.message` (réutilisée)
- `contenu` 1..2000 car. (contrainte existante). Le message initial de contact est inséré par l'endpoint `contacter`.

## DTOs backend (`src/models/annonce.rs` — à ajouter)

```rust
// Création par un membre (multipart : champs + fichiers photos séparés)
pub struct CreerAnnonceMembreRequest {
    pub titre: String,
    pub description: String,
    pub type_operation: String,      // "vente" | "troc" | "don" (validé)
    pub categorie_id: Uuid,
    pub condition_article: Option<String>,
    pub prix: Option<f64>,           // requis si vente
    pub devise: Option<String>,
    pub prix_negociable: Option<bool>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub quantite: Option<i32>,
    pub pays_ids: Vec<Uuid>,         // territoires ciblés
}

// Modification (mêmes champs, tous optionnels sauf garde propriétaire)
pub struct ModifierAnnonceMembreRequest { /* idem, champs Option */ }

// Contact
pub struct ContacterAuteurRequest { pub message: String } // 1..2000

// Réponses : réutiliser AnnonceResponse / AnnonceDetailResponse existants.
// Mes annonces : variante incluant `etat` et compteur photos.
```

## Interfaces frontend (`useMarcheAfricain.ts` — à ajouter)

```ts
// Aligner sur AnnonceAPI / AnnonceDetailAPI existants.
interface CreerAnnonceForm {
  titre: string
  description: string
  typeEchange: TypeEchange          // 'Vente' | 'Troc' | 'Don' → mappé en db
  categorie: Categorie
  conditionArticle?: string
  prix?: number | null
  devise?: Devise
  prixNegociable?: boolean
  ville?: string
  adresse?: string
  longitude?: number | null
  latitude?: number | null
  quantite?: number
  paysIds: string[]
  photos: File[]                    // ≤ 5, ≤ 3 Mo, jpeg/png/webp
}
```

## Règles de validation (frontière)

- **Titre** 3..350, **description** ≥ 10 car.
- **type_operation** ∈ {vente, troc, don} (rejet sinon).
- **prix** : requis et > 0 si `vente` ; ignoré/0 si `don`/`troc`.
- **categorie_id** doit exister dans `shared.categorie`.
- **pays_ids** doivent exister dans `shared.pays`.
- **photos** : 1..5, chacune ≤ 3 Mo, MIME ∈ {image/jpeg, image/png, image/webp} (via `image_validation`).
- **contacter** : annonce `publiee`, `cree_par <> courant`, pas de blocage réciproque, `message` 1..2000.
- **modifier/supprimer/conclure** : `cree_par = courant` sinon `403`.

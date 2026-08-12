# Phase 1 — Modèle de données

**Feature**: 010-medias-equipes-vitrine · **Date**: 2026-08-10
**Migration**: `uafricas_backend/doc/bd/schemas/09t_media_content_equipes_periodicite.sql`

Le schéma SQL est la source de vérité (Principe III) : la migration précède le code Rust, qui précède le TypeScript.

---

## 1. Table neuve — `media_content.membre_equipe`

```sql
CREATE TABLE IF NOT EXISTS media_content.membre_equipe (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_porteur   VARCHAR(20)  NOT NULL,
    porteur_id     UUID         NOT NULL,
    nom            VARCHAR(150) NOT NULL,
    prenom         VARCHAR(150),
    fonction       VARCHAR(120) NOT NULL,
    territoire     VARCHAR(150),
    contact        VARCHAR(250),
    utilisateur_id UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    ordre          INT          NOT NULL DEFAULT 0,
    cree_par       UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE RESTRICT,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMPTZ
);
```

### Contraintes

```sql
ALTER TABLE media_content.membre_equipe
    ADD CONSTRAINT ck_membre_equipe_type_porteur
        CHECK (type_porteur IN ('chaine_tv', 'station_radio', 'emission_tele', 'emission_radio'));

-- FR-012 : nom et fonction obligatoires, et « obligatoire » veut dire non vide,
-- pas « non NULL » — une chaîne d'espaces passerait le NOT NULL.
ALTER TABLE media_content.membre_equipe
    ADD CONSTRAINT ck_membre_equipe_nom_non_vide      CHECK (btrim(nom) <> '');
ALTER TABLE media_content.membre_equipe
    ADD CONSTRAINT ck_membre_equipe_fonction_non_vide CHECK (btrim(fonction) <> '');
```

**Pas d'unicité sur `(porteur_id, nom, prenom)`** : le cas limite « homonymes dans une même équipe » de la spec l'exige explicitement (deux personnes de même nom se distinguent par leur fonction). **Pas d'unicité sur `(porteur_id, utilisateur_id)`** non plus : rien n'interdit qu'un compte figure deux fois dans une même équipe sous deux fonctions.

### Index

```sql
-- Lecture groupée (equipes_par_porteurs) : le seul accès chaud.
CREATE INDEX IF NOT EXISTS idx_membre_equipe_porteur
    ON media_content.membre_equipe (type_porteur, porteur_id, ordre)
    WHERE deleted_at IS NULL;

-- Route de suggestions des fonctions.
CREATE INDEX IF NOT EXISTS idx_membre_equipe_fonction
    ON media_content.membre_equipe (fonction)
    WHERE deleted_at IS NULL;
```

### Commentaires de colonne (obligatoires, cf. 09q/09r)

- `type_porteur` — « Support (chaine_tv, station_radio) ou programme (emission_tele, emission_radio). Volontairement PAS l'enum type_support_media (09m), qui ne couvre que les supports, ni le CHECK des interactions (09k), qui inclut les épisodes. »
- `utilisateur_id` — « Rattachement FACULTATIF à un compte (FR-013). ON DELETE SET NULL : la fiche survit à la fermeture du compte, sans lien mort. Ne confère AUCUN droit — les droits vivent dans support_detenteur (09m). »
- `contact` — « Coordonnée professionnelle SAISIE par le gestionnaire. N'est JAMAIS dérivée de iam.utilisateur.email, même quand utilisateur_id est renseigné. »
- `ordre` — « Rang d'affichage public (FR-016), réécrit à chaque PUT depuis l'index reçu. »

### Cycle de vie

| Événement | Effet |
|---|---|
| `PUT …/equipe` | `DELETE` des lignes du porteur puis `INSERT` de la liste reçue, dans une transaction. `ordre` = index. |
| Suppression douce d'un programme (`emission_*.deleted_at`) | Suppression douce de son équipe : `UPDATE membre_equipe SET deleted_at = NOW() WHERE type_porteur = … AND porteur_id = …` — FR-019. À poser dans `media_emission::supprimer_emission` **et** `admin::radio_tele::supprimer_emission_admin`. |
| Suppression douce d'un support | Même traitement, dans `admin::radio_tele::supprimer_{chaine_tv,station_radio}`. |
| Fermeture d'un compte rattaché | `ON DELETE SET NULL` (suppression dure) ; la suppression douce est absorbée à la lecture par `AND u.deleted_at IS NULL` dans la jointure. |

> La suppression du porteur n'est pas une CASCADE : `porteur_id` n'a pas de FK (c'est le prix du polymorphisme). Le nettoyage est **explicite dans le handler**, exactement comme `support_thematique` (09r). L'oublier laisserait des équipes orphelines invisibles mais présentes dans les suggestions de fonction.

---

## 2. Modification — périodicité des programmes

```sql
ALTER TABLE media_content.emission_tele  DROP CONSTRAINT IF EXISTS ck_emission_tele_cadence;
ALTER TABLE media_content.emission_tele
    ADD CONSTRAINT ck_emission_tele_cadence
        CHECK (cadence IN ('quotidienne', 'hebdomadaire', 'mensuelle', 'ponctuelle'));

ALTER TABLE media_content.emission_radio DROP CONSTRAINT IF EXISTS ck_emission_radio_cadence;
ALTER TABLE media_content.emission_radio
    ADD CONSTRAINT ck_emission_radio_cadence
        CHECK (cadence IN ('quotidienne', 'hebdomadaire', 'mensuelle', 'ponctuelle'));
```

**Aucun `UPDATE` de données.** Les clés existantes sont conservées ; seuls les libellés d'affichage changent (D4). `DEFAULT 'ponctuelle'` reste en place et satisfait FR-042.

---

## 3. Types Rust — `src/models/media_equipe.rs`

```rust
pub const TYPES_PORTEUR: [&str; 4] =
    ["chaine_tv", "station_radio", "emission_tele", "emission_radio"];

pub const MEMBRE_EQUIPE_COLONNES: &str = "\
    m.id, m.type_porteur, m.porteur_id, m.nom, m.prenom, m.fonction, \
    m.territoire, m.contact, m.utilisateur_id, m.ordre, m.created_at, m.updated_at";

#[derive(sqlx::FromRow)]
pub struct MembreEquipeRow {
    pub id: Uuid,
    pub type_porteur: String,
    pub porteur_id: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub fonction: String,
    pub territoire: Option<String>,
    pub contact: Option<String>,
    pub utilisateur_id: Option<Uuid>,
    pub ordre: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Jointure LEFT sur iam.utilisateur AND u.deleted_at IS NULL.
    /// `None` quand le compte n'existe plus : le nom s'affiche alors en texte simple.
    #[sqlx(default)]
    pub profil_slug_ou_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct MembreEquipeResponse {
    pub id: Uuid,
    pub nom: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub prenom: Option<String>,
    pub fonction: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub territoire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub contact: Option<String>,
    /// Renseigné SEULEMENT si le compte existe et n'est pas supprimé (FR-014).
    #[serde(skip_serializing_if = "Option::is_none")] pub utilisateur_id: Option<Uuid>,
    pub ordre: i32,
}

#[derive(Deserialize)]
pub struct MembreEquipeRequest {
    pub nom: String,
    pub prenom: Option<String>,
    pub fonction: String,
    pub territoire: Option<String>,
    pub contact: Option<String>,
    pub utilisateur_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct EquipeRequest { pub membres: Vec<MembreEquipeRequest> }
```

### Règles de validation (`EquipeRequest::valider`)

| Règle | Message |
|---|---|
| `nom` non vide après `btrim` | « Le nom d'un membre de l'équipe est obligatoire » |
| `fonction` non vide après `btrim` | « La fonction d'un membre de l'équipe est obligatoire » |
| `membres.len() <= 60` | « Une équipe ne peut compter plus de 60 personnes » |
| Normalisation appliquée à `fonction` avant insertion | `btrim` + `regexp_replace('\s+', ' ')` — voir D3 |

Une liste **vide est valide** : c'est ainsi qu'on supprime toute l'équipe.

### Aiguillage (symétrie stricte avec `media_emission::table_emission`)

```rust
pub fn table_porteur(type_porteur: &str) -> Result<&'static str, ApiErreur>
// chaine_tv → media_content.chaine_tv | station_radio → media_content.station_radio
// emission_tele → media_content.emission_tele | emission_radio → media_content.emission_radio

pub fn type_support_du_porteur(type_porteur: &str) -> Result<&'static str, ApiErreur>
// chaine_tv|emission_tele → "chaine_tv" ; station_radio|emission_radio → "station_radio"
// Sert à résoudre la garde de détention quand le porteur est un programme.
```

---

## 4. DTO existants enrichis

| Struct | Fichier | Ajout |
|---|---|---|
| `ChaineTvResponse` | `models/television.rs:63` | `#[serde(skip_serializing_if = "Vec::is_empty")] pub equipe: Vec<MembreEquipeResponse>` |
| `StationRadioResponse` | `models/station_radio.rs` | idem |
| `EmissionResponse` | `models/media_emission.rs:188` | idem |

Ces trois champs suivent la convention de `thematiques` (09r) : omis du JSON quand l'équipe est vide, ce qui réalise FR-007 côté contrat autant que côté rendu.

---

## 5. Types TypeScript — `app/composables/useMediaEquipe.ts`

```ts
export type TypePorteurEquipe =
  | 'chaine_tv' | 'station_radio' | 'emission_tele' | 'emission_radio'

export interface MembreEquipeAPI {
  id: string
  nom: string
  prenom?: string
  fonction: string
  territoire?: string
  contact?: string
  utilisateur_id?: string   // absent = fiche non rattachée → nom en texte simple
  ordre: number
}

/** Ce que le formulaire manipule : pas d'`id` (le PUT remplace tout), pas d'`ordre`
 *  (c'est l'index dans le tableau). */
export interface MembreEquipeForm {
  nom: string
  prenom: string
  fonction: string
  territoire: string
  contact: string
  utilisateur_id: string | null
}
```

Ajouts aux types existants : `TvChannel.equipe: MembreEquipeAPI[]`, `TvEmission.equipe: MembreEquipeAPI[]`, et leurs pendants radio — repli `[]` dans les mappeurs, jamais `undefined`, pour que les gabarits n'aient pas à tester deux formes.

---

## 6. Périodicité côté code

### Rust — `models/media_emission.rs`

```rust
pub const CADENCES_AUTORISEES: [&str; 4] =
    ["quotidienne", "hebdomadaire", "mensuelle", "ponctuelle"];

/// Longueur du cycle. `None` pour 'ponctuelle' : aucune échéance à tenir.
pub fn periode_heures_cadence(cadence: &str) -> Option<i64> {
    match cadence {
        "quotidienne"  => Some(24),
        "hebdomadaire" => Some(24 * 7),
        "mensuelle"    => Some(24 * 30),
        _              => None,
    }
}

/// Marge d'anticipation de l'alerte (FR-024 de la feature 009).
pub fn heures_anticipation_alerte(cadence: &str) -> Option<i64> {
    match cadence {
        "quotidienne"  => Some(6),
        "hebdomadaire" => Some(48),
        "mensuelle"    => Some(24 * 7),
        _              => None,
    }
}
```

`handlers/media_programmation.rs:829` cesse de calculer `if cadence == "quotidienne" { 24 } else { 24 * 7 }` et appelle `periode_heures_cadence`. Sans cette reprise, un programme mensuel serait signalé en retard au bout d'une semaine.

Le message d'erreur de `valider_cadence` mentionne les quatre valeurs.

### TypeScript — `app/composables/useMediaEmissions.ts`

```ts
export const LIBELLES_CADENCE: Record<string, string> = {
  ponctuelle:   'Non périodique',
  quotidienne:  'Journalier',
  hebdomadaire: 'Hebdomadaire',
  mensuelle:    'Mensuel',
}
/** Ordre d'affichage dans les sélecteurs — « non périodique » en tête (défaut, FR-042). */
export const CADENCES_ORDONNEES = ['ponctuelle', 'quotidienne', 'hebdomadaire', 'mensuelle'] as const
```

`useAdminMediaEmissions.ts` supprime son `CADENCES`/`libelleCadence` propre et réexporte ceux-ci : FR-041 exige un libellé identique à la saisie et en public, ce que deux tables séparées ne garantissent pas dans la durée.

---

## 7. Conformité aux conventions du projet

| Convention | Respect |
|---|---|
| UUID v4 en PK | `gen_random_uuid()` — comme 09m/09q/09r |
| Suppression douce | `deleted_at TIMESTAMPTZ`, filtré dans tous les index partiels et toutes les lectures |
| TIMESTAMPTZ | `created_at`, `updated_at`, `deleted_at` |
| snake_case français | `type_porteur`, `porteur_id`, `territoire`, `cree_par` |
| Migration idempotente | `CREATE TABLE IF NOT EXISTS`, `CREATE INDEX IF NOT EXISTS`, `DROP CONSTRAINT IF EXISTS` puis `ADD` |
| `COLONNES` const + `FromRow` + DTO `Response` séparé | `MEMBRE_EQUIPE_COLONNES` / `MembreEquipeRow` / `MembreEquipeResponse` |
| Enregistrement dans l'orchestrateur | Ajouter `\ir schemas/09t_media_content_equipes_periodicite.sql` à `doc/bd/schema.sql` |

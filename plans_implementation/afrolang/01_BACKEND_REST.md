# Phase 1 — Backend REST API

> **Statut** : `TERMINE`
> **Progression** : 12/12 taches
> **Bloque par** : Rien (premiere phase)
> **Debloque** : [Phase 2 — Frontend UI](./02_FRONTEND_UI.md)

---

## Contexte inter-phases

```
✅ = termine    🔄 = en cours    ⬜ = a faire    🔒 = bloque

[🔄] Phase 1 — Backend REST        ◄── VOUS ETES ICI
[🔒] Phase 2 — Frontend UI          (attend Phase 1 complete)
[🔒] Phase 3 — WebRTC Signaling     (attend Phase 2 complete)
[🔒] Phase 4 — Tableau blanc        (attend Phase 3 complete)
```

**Ce que cette phase produit pour la Phase 2 :**
- Tous les endpoints REST fonctionnels et testes
- Le composable `useAfrolang.ts` (Phase 2) appellera ces endpoints via `$fetch`
- Contrats API (types de retour JSON) que le frontend consommera

---

## Progression

- [x] **1.1** Creer `src/models/afrolang.rs` — Constantes SQL
- [x] **1.2** Creer `src/models/afrolang.rs` — Structs FromRow
- [x] **1.3** Creer `src/models/afrolang.rs` — DTOs Response (Serialize)
- [x] **1.4** Creer `src/models/afrolang.rs` — Structs de requete (Deserialize)
- [x] **1.5** Creer `src/handlers/afrolang.rs` — Handlers salles publiques (5 endpoints)
- [x] **1.6** Creer `src/handlers/afrolang.rs` — Handlers salles privees (5 endpoints)
- [x] **1.7** Creer `src/handlers/afrolang.rs` — Handlers sessions (7 endpoints)
- [x] **1.8** Creer `src/handlers/afrolang.rs` — Handlers utilitaires (stats + langues)
- [x] **1.9** Modifier `src/handlers/mod.rs` — Ajouter `pub mod afrolang;`
- [x] **1.10** Modifier `src/models/mod.rs` — Ajouter `pub mod afrolang;`
- [x] **1.11** Modifier `src/routes.rs` — Ajouter le scope `/afrolang`
- [x] **1.12** Tests manuels curl — Valider tous les endpoints

---

## 1.1–1.4 — Fichier `src/models/afrolang.rs`

### 1.1 — Constantes SQL

```rust
// Colonnes pour la liste des salles publiques
pub const SALLE_COLONNES: &str = "s.id, s.titre, s.slug, s.description, s.image_couverture_url, s.langue_cible, s.moderateur_id, s.actif, s.cree_par, s.created_at, s.updated_at";

// Colonnes pour la liste des salles privees
pub const SALLE_PRIVEE_COLONNES: &str = "sp.id, sp.salle_id, sp.titre, sp.description, sp.image_couverture_url, sp.max_participants, sp.actif, sp.cree_par, sp.created_at, sp.updated_at";

// Colonnes pour les sessions
pub const SESSION_COLONNES: &str = "ses.id, ses.salle_privee_id, ses.titre, ses.etat::TEXT, ses.moderateur_id, ses.date_debut_prevue, ses.demarre_at, ses.termine_at, ses.duree_secondes, ses.max_participants, ses.nombre_participants_pic, ses.tableau_blanc_actif, ses.noeud_id, ses.cree_par, ses.created_at, ses.updated_at";
```

### 1.2 — Structs FromRow

```rust
#[derive(Debug, sqlx::FromRow)]
pub struct SalleRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub langue_cible: Option<String>,
    pub moderateur_id: Option<Uuid>,
    pub actif: bool,
    pub cree_par: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SallePriveeRow {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub max_participants: Option<i32>,
    pub actif: bool,
    pub cree_par: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    // JOINs
    pub createur_nom: Option<String>,
    pub createur_prenom: Option<String>,
    pub createur_photo: Option<String>,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SessionRow {
    pub id: Uuid,
    pub salle_privee_id: Uuid,
    pub titre: Option<String>,
    pub etat: String,
    pub moderateur_id: Option<Uuid>,
    pub date_debut_prevue: Option<chrono::DateTime<chrono::Utc>>,
    pub demarre_at: Option<chrono::DateTime<chrono::Utc>>,
    pub termine_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duree_secondes: Option<i32>,
    pub max_participants: Option<i32>,
    pub nombre_participants_pic: Option<i32>,
    pub tableau_blanc_actif: bool,
    pub noeud_id: Option<String>,
    pub cree_par: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SessionParticipantRow {
    pub id: Uuid,
    pub session_id: Uuid,
    pub utilisateur_id: Uuid,
    pub role_session: String,
    pub rejoint_at: chrono::DateTime<chrono::Utc>,
    pub quitte_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duree_secondes: Option<i32>,
}
```

### 1.3 — DTOs Response (Serialize)

```rust
#[derive(Debug, Serialize)]
pub struct SalleResponse { ... }          // Salle publique pour listing

#[derive(Debug, Serialize)]
pub struct SalleDetailResponse { ... }    // Avec salles privees associees + stats

#[derive(Debug, Serialize)]
pub struct SallePriveeResponse { ... }    // Salle privee pour listing

#[derive(Debug, Serialize)]
pub struct SallePriveeDetailResponse { ... } // Avec sessions associees

#[derive(Debug, Serialize)]
pub struct SessionResponse { ... }        // Session pour listing

#[derive(Debug, Serialize)]
pub struct SessionDetailResponse { ... }  // Avec participants + whiteboard status

#[derive(Debug, Serialize)]
pub struct AfrolangStatsResponse {        // Statistiques globales
    pub total_salles: i64,
    pub total_salles_privees: i64,
    pub sessions_en_cours: i64,
    pub sessions_terminees: i64,
    pub total_participants_uniques: i64,
}
```

### 1.4 — Structs de requete (Deserialize)

```rust
#[derive(Debug, Deserialize)]
pub struct SalleFiltres {
    pub recherche: Option<String>,
    pub langue: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SallePriveeFiltres {
    pub recherche: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SessionFiltres {
    pub etat: Option<String>,   // planifiee, en_cours, terminee, annulee
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreerSallePriveeRequest {
    pub titre: String,
    pub description: Option<String>,
    pub code_acces: Option<String>,
    pub max_participants: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreerSessionRequest {
    pub titre: Option<String>,
    pub date_debut_prevue: Option<String>,
    pub max_participants: Option<i32>,
    pub tableau_blanc_actif: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RejoindreRequest {
    pub code_acces: Option<String>,  // requis pour salle privee
}
```

---

## 1.5 — Handlers salles publiques (admin only pour creation)

| Methode | Route | Handler | Auth | Description |
|---------|-------|---------|------|-------------|
| `GET` | `/api/afrolang/salles` | `lister_salles` | Non | Liste paginee des salles publiques actives |
| `GET` | `/api/afrolang/salles/{id}` | `obtenir_salle` | Non | Detail d'une salle avec ses salles privees |
| `POST` | `/api/afrolang/salles` | `creer_salle` | JWT + Admin | Creation multipart (image + metadonnees) |
| `PUT` | `/api/afrolang/salles/{id}` | `modifier_salle` | JWT + Admin | Modification (titre, description, moderateur) |
| `DELETE` | `/api/afrolang/salles/{id}` | `supprimer_salle` | JWT + Admin | Soft delete (`actif = false`) |

> **Important pour Phase 2** : Le `GET /salles` retourne `nombre_salles_privees` et `sessions_en_cours` (sous-requetes COUNT) pour les `SalleCard.vue`.

---

## 1.6 — Handlers salles privees (tout utilisateur authentifie)

| Methode | Route | Handler | Auth | Description |
|---------|-------|---------|------|-------------|
| `GET` | `/api/afrolang/salles/{salle_id}/privees` | `lister_salles_privees` | Non | Salles privees d'une salle publique |
| `GET` | `/api/afrolang/salles-privees/{id}` | `obtenir_salle_privee` | Non | Detail d'une salle privee avec sessions |
| `POST` | `/api/afrolang/salles/{salle_id}/privees` | `creer_salle_privee` | JWT | Creer une salle privee liee a une salle publique |
| `PUT` | `/api/afrolang/salles-privees/{id}` | `modifier_salle_privee` | JWT (createur) | Modifier sa salle privee |
| `DELETE` | `/api/afrolang/salles-privees/{id}` | `supprimer_salle_privee` | JWT (createur) | Soft delete |

> **Important pour Phase 2** : Le `GET /salles-privees/{id}` retourne `est_protegee` (bool, ne PAS renvoyer `code_acces` en clair) et `session_en_cours` (bool) pour les `SallePriveeCard.vue`.

---

## 1.7 — Handlers sessions WebRTC

| Methode | Route | Handler | Auth | Description |
|---------|-------|---------|------|-------------|
| `GET` | `/api/afrolang/salles-privees/{sp_id}/sessions` | `lister_sessions` | Non | Sessions d'une salle privee |
| `GET` | `/api/afrolang/sessions/{id}` | `obtenir_session` | Non | Detail avec participants |
| `POST` | `/api/afrolang/salles-privees/{sp_id}/sessions` | `creer_session` | JWT (moderateur) | Planifier une session |
| `PUT` | `/api/afrolang/sessions/{id}/demarrer` | `demarrer_session` | JWT (moderateur) | Passer etat → en_cours |
| `PUT` | `/api/afrolang/sessions/{id}/terminer` | `terminer_session` | JWT (moderateur) | Passer etat → terminee |
| `POST` | `/api/afrolang/sessions/{id}/rejoindre` | `rejoindre_session` | JWT | Rejoindre (verifie code_acces si salle privee) |
| `POST` | `/api/afrolang/sessions/{id}/quitter` | `quitter_session` | JWT | Quitter (met a jour quitte_at + duree) |

> **Important pour Phase 3** : Les handlers `demarrer`/`terminer`/`rejoindre`/`quitter` seront reutilises par le flow WebRTC. La Phase 3 ajoutera `generer_token_session` en complement.

### Logique metier cle

#### Verification d'acces salle privee
```
Si salle_privee.code_acces IS NOT NULL:
  → Le request body doit contenir le bon code_acces
  → Sinon → 403 Forbidden
```

#### Demarrage de session
```
1. Verifier que l'utilisateur est le moderateur (cree_par de la salle_privee)
2. Verifier que la session est en etat 'planifiee'
3. UPDATE etat = 'en_cours', demarre_at = NOW()
4. Creer le participant avec role_session = 'moderateur'
```

#### Rejoindre une session
```
1. Verifier que la session est en etat 'en_cours'
2. Verifier le code_acces de la salle privee (si defini)
3. Verifier max_participants non atteint
4. INSERT session_participant avec role_session = 'participant'
5. UPDATE nombre_participants_pic si necessaire
```

---

## 1.8 — Handlers utilitaires

| Methode | Route | Handler | Auth | Description |
|---------|-------|---------|------|-------------|
| `GET` | `/api/afrolang/stats` | `obtenir_stats` | Non | Statistiques globales Afrolang |
| `GET` | `/api/afrolang/langues` | `lister_langues` | Non | Liste des langues disponibles (depuis salles) |

> **Important pour Phase 2** : `obtenir_stats` alimente `AfrolangStats.vue` et `lister_langues` alimente le filtre dans `SalleFilters.vue`.

---

## 1.9–1.10 — Enregistrement des modules

### `src/handlers/mod.rs`
```rust
pub mod afrolang;
```

### `src/models/mod.rs`
```rust
pub mod afrolang;
```

---

## 1.11 — Routes (`src/routes.rs`)

```rust
.service(
    web::scope("/afrolang")
        // Salles publiques
        .route("/salles", web::get().to(afrolang::lister_salles))
        .route("/salles", web::post().to(afrolang::creer_salle))
        .route("/salles/{id}", web::get().to(afrolang::obtenir_salle))
        .route("/salles/{id}", web::put().to(afrolang::modifier_salle))
        .route("/salles/{id}", web::delete().to(afrolang::supprimer_salle))
        // Salles privees (creation sous une salle publique)
        .route("/salles/{salle_id}/privees", web::get().to(afrolang::lister_salles_privees))
        .route("/salles/{salle_id}/privees", web::post().to(afrolang::creer_salle_privee))
        // Salles privees (CRUD direct)
        .route("/salles-privees/{id}", web::get().to(afrolang::obtenir_salle_privee))
        .route("/salles-privees/{id}", web::put().to(afrolang::modifier_salle_privee))
        .route("/salles-privees/{id}", web::delete().to(afrolang::supprimer_salle_privee))
        // Sessions
        .route("/salles-privees/{sp_id}/sessions", web::get().to(afrolang::lister_sessions))
        .route("/salles-privees/{sp_id}/sessions", web::post().to(afrolang::creer_session))
        .route("/sessions/{id}", web::get().to(afrolang::obtenir_session))
        .route("/sessions/{id}/demarrer", web::put().to(afrolang::demarrer_session))
        .route("/sessions/{id}/terminer", web::put().to(afrolang::terminer_session))
        .route("/sessions/{id}/rejoindre", web::post().to(afrolang::rejoindre_session))
        .route("/sessions/{id}/quitter", web::post().to(afrolang::quitter_session))
        // Utilitaires
        .route("/stats", web::get().to(afrolang::obtenir_stats))
        .route("/langues", web::get().to(afrolang::lister_langues))
)
```

---

## 1.12 — Tests manuels curl

Valider **chaque endpoint** avant de passer a la Phase 2 :

- [ ] Creer une salle publique (admin) → `POST /api/afrolang/salles`
- [ ] Lister les salles → `GET /api/afrolang/salles`
- [ ] Detail d'une salle → `GET /api/afrolang/salles/{id}`
- [ ] Creer une salle privee → `POST /api/afrolang/salles/{id}/privees`
- [ ] Lister les salles privees → `GET /api/afrolang/salles/{id}/privees`
- [ ] Detail salle privee → `GET /api/afrolang/salles-privees/{id}`
- [ ] Planifier une session → `POST /api/afrolang/salles-privees/{id}/sessions`
- [ ] Demarrer la session → `PUT /api/afrolang/sessions/{id}/demarrer`
- [ ] Rejoindre la session → `POST /api/afrolang/sessions/{id}/rejoindre`
- [ ] Quitter la session → `POST /api/afrolang/sessions/{id}/quitter`
- [ ] Terminer la session → `PUT /api/afrolang/sessions/{id}/terminer`
- [ ] Stats → `GET /api/afrolang/stats`
- [ ] Langues → `GET /api/afrolang/langues`

---

## Recapitulatif fichiers

### Fichiers a creer
- `src/models/afrolang.rs` (~200 lignes)
- `src/handlers/afrolang.rs` (~600-800 lignes)

### Fichiers a modifier
- `src/models/mod.rs` — Ajouter `pub mod afrolang;`
- `src/handlers/mod.rs` — Ajouter `pub mod afrolang;`
- `src/routes.rs` — Ajouter le scope `/afrolang`

### Dependances cargo additionnelles
- Aucune — tout est deja disponible (actix-web, sqlx, uuid, chrono, serde)

### Prerequis
- Le schema SQL `08b_afrolang.sql` doit etre applique a la base (deja dans l'orchestrateur `schema.sql`)
- Les contraintes inter-schemas (`13_contraintes_inter_schemas.sql`) doivent etre en place

---

## Critere de completion Phase 1

> **La Phase 2 peut commencer UNIQUEMENT quand :**
> - [x] Tous les 12 points de la progression sont coches
> - [x] Tous les tests curl de la section 1.12 passent
> - [x] Le backend compile sans erreur (`cargo build`)
> - [x] Les endpoints retournent les bons formats JSON (verifies manuellement)
>
> Quand c'est fait → mettre le statut a `TERMINE` dans [00_OVERVIEW.md](./00_OVERVIEW.md) et debloquer Phase 2.

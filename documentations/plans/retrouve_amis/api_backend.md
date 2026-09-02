# Retrouve Amis : Plan API Backend

## Structure des fichiers

```
uafricas_backend/src/
├── handlers/
│   ├── retrouve_amis.rs              # Handlers publics
│   └── admin/
│       └── retrouve_amis.rs          # Handlers admin
├── models/
│   ├── retrouve_amis.rs              # Modèles publics
│   └── admin/
│       └── retrouve_amis.rs          # Modèles admin
└── routes.rs                         # + nouvelles routes
```

---

## Endpoints API

### Routes publiques (utilisateur connecté)

| Méthode | Route | Handler | Description |
|---------|-------|---------|-------------|
| GET | `/api/retrouve-amis` | `lister_avis` | Liste publique des avis (critères visibles, identité cachée) |
| GET | `/api/retrouve-amis/{id}` | `obtenir_avis` | Détail d'un avis (sans identité du chercheur si anonyme) |
| POST | `/api/retrouve-amis` | `creer_avis` | Créer un avis de recherche + critères |
| PUT | `/api/retrouve-amis/{id}` | `modifier_avis` | Modifier son avis |
| DELETE | `/api/retrouve-amis/{id}` | `supprimer_avis` | Supprimer son avis (soft delete) |
| PATCH | `/api/retrouve-amis/{id}/etat` | `changer_etat_avis` | Changer l'état (actif, en_pause, resolu) |
| GET | `/api/retrouve-amis/mes-recherches` | `mes_recherches` | Mes avis de recherche |
| POST | `/api/retrouve-amis/rechercher` | `rechercher` | Recherche libre (retourne avis correspondants) |

### Routes correspondances

| Méthode | Route | Handler | Description |
|---------|-------|---------|-------------|
| GET | `/api/retrouve-amis/correspondances` | `mes_correspondances` | Mes correspondances |
| GET | `/api/retrouve-amis/correspondances/{id}` | `obtenir_correspondance` | Détail d'une correspondance |
| POST | `/api/retrouve-amis/correspondances/{id}/confirmer` | `confirmer_correspondance` | Confirmer (double opt-in) |
| POST | `/api/retrouve-amis/correspondances/{id}/rejeter` | `rejeter_correspondance` | Rejeter une correspondance |

### Routes messagerie

| Méthode | Route | Handler | Description |
|---------|-------|---------|-------------|
| GET | `/api/retrouve-amis/correspondances/{id}/messages` | `lister_messages` | Messages d'une correspondance |
| POST | `/api/retrouve-amis/correspondances/{id}/messages` | `envoyer_message` | Envoyer un message |
| POST | `/api/retrouve-amis/correspondances/{id}/messages/contact` | `partager_contact` | Partager un numéro/email |
| PATCH | `/api/retrouve-amis/correspondances/{id}/messages/lire` | `marquer_lu` | Marquer messages comme lus |

### Routes préférences de trouvabilité

| Méthode | Route | Handler | Description |
|---------|-------|---------|-------------|
| GET | `/api/retrouve-amis/preferences` | `obtenir_preferences` | Mes préférences |
| PUT | `/api/retrouve-amis/preferences` | `modifier_preferences` | Modifier mes préférences |

### Routes signalement

| Méthode | Route | Handler | Description |
|---------|-------|---------|-------------|
| POST | `/api/retrouve-amis/signaler` | `signaler` | Signaler un avis ou correspondance |

### Routes publiques (sans authentification)

| Méthode | Route | Handler | Description |
|---------|-------|---------|-------------|
| POST | `/api/retrouve-amis/recherche-anonyme` | `recherche_anonyme` | Recherche sans compte (résultats limités) |
| GET | `/api/retrouve-amis/stats` | `statistiques` | Stats publiques (nombre d'avis, retrouvailles...) |

### Routes admin

| Méthode | Route | Handler | Description |
|---------|-------|---------|-------------|
| GET | `/api/admin/retrouve-amis` | `lister_avis` | Liste tous les avis (avec identité) |
| GET | `/api/admin/retrouve-amis/{id}` | `obtenir_avis` | Détail complet |
| PATCH | `/api/admin/retrouve-amis/{id}/etat` | `moderer_avis` | Modérer (masquer, réactiver) |
| DELETE | `/api/admin/retrouve-amis/{id}` | `supprimer_avis` | Suppression admin |
| GET | `/api/admin/retrouve-amis/signalements` | `lister_signalements` | Signalements non traités |
| GET | `/api/admin/retrouve-amis/signalements/{id}` | `obtenir_signalement` | Détail signalement |
| PATCH | `/api/admin/retrouve-amis/signalements/{id}` | `traiter_signalement` | Traiter un signalement |
| GET | `/api/admin/retrouve-amis/stats` | `statistiques_admin` | Statistiques détaillées |

---

## Modèles Rust

### `src/models/retrouve_amis.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ============================================================
// Colonnes SQL
// ============================================================

pub const AVIS_COLONNES: &str = "a.id, a.auteur_id, a.titre, a.description, \
    a.relation, a.est_anonyme, a.etat::TEXT, a.nombre_vues, \
    a.created_at, a.updated_at, a.derniere_activite, a.expire_le";

pub const CRITERE_COLONNES: &str = "c.id, c.avis_id, c.type_critere::TEXT, \
    c.valeur, c.valeur_normalisee, c.poids, c.pays_id, c.created_at";

pub const CORRESPONDANCE_COLONNES: &str = "co.id, co.avis_a_id, co.avis_b_id, \
    co.utilisateur_b_id, co.score, co.detail_score, co.etat::TEXT, \
    co.confirme_a_le, co.confirme_b_le, co.rejete_par, co.rejete_le, \
    co.created_at, co.updated_at, co.expire_le";

pub const MESSAGE_COLONNES: &str = "m.id, m.correspondance_id, m.auteur_id, \
    m.contenu, m.type_contact::TEXT, m.valeur_contact_chiffree, \
    m.nom_contact, m.lu, m.lu_le, m.created_at";

// ============================================================
// FromRow : Rows BD
// ============================================================

#[derive(Debug, FromRow)]
pub struct AvisRechercheRow {
    pub id: Uuid,
    pub auteur_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub relation: Option<String>,
    pub est_anonyme: bool,
    pub etat: String,
    pub nombre_vues: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub derniere_activite: DateTime<Utc>,
    pub expire_le: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct CritereRechercheRow {
    pub id: Uuid,
    pub avis_id: Uuid,
    pub type_critere: String,
    pub valeur: String,
    pub valeur_normalisee: String,
    pub poids: i32,
    pub pays_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct CorrespondanceRow {
    pub id: Uuid,
    pub avis_a_id: Uuid,
    pub avis_b_id: Option<Uuid>,
    pub utilisateur_b_id: Option<Uuid>,
    pub score: i32,
    pub detail_score: serde_json::Value,
    pub etat: String,
    pub confirme_a_le: Option<DateTime<Utc>>,
    pub confirme_b_le: Option<DateTime<Utc>>,
    pub rejete_par: Option<Uuid>,
    pub rejete_le: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expire_le: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct MessageCorrespondanceRow {
    pub id: Uuid,
    pub correspondance_id: Uuid,
    pub auteur_id: Uuid,
    pub contenu: String,
    pub type_contact: Option<String>,
    pub valeur_contact_chiffree: Option<String>,
    pub nom_contact: Option<String>,
    pub lu: bool,
    pub lu_le: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ============================================================
// Response DTOs : Sérialisation frontend
// ============================================================

#[derive(Debug, Serialize)]
pub struct AvisRechercheResponse {
    pub id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub relation: Option<String>,
    pub est_anonyme: bool,
    pub etat: String,
    pub nombre_vues: i32,
    pub criteres: Vec<CritereResponse>,
    pub nombre_correspondances: i64,
    pub created_at: DateTime<Utc>,
    // auteur_id n'est PAS inclus si l'avis est anonyme (sécurité)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auteur: Option<AuteurResponse>,
}

#[derive(Debug, Serialize)]
pub struct AuteurResponse {
    pub id: Uuid,
    pub prenom: String,
    pub nom: String,
}

#[derive(Debug, Serialize)]
pub struct CritereResponse {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_critere: String,
    pub valeur: String,
    pub poids: i32,
    pub pays_nom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CorrespondanceResponse {
    pub id: Uuid,
    pub score: i32,
    pub etat: String,
    pub detail_score: serde_json::Value,
    pub avis_titre: String,           // Titre de l'avis matché
    pub criteres_communs: Vec<CritereCommun>,
    pub confirme_a: bool,
    pub confirme_b: bool,
    pub created_at: DateTime<Utc>,
    pub expire_le: DateTime<Utc>,
    // Info sur l'autre partie (limité tant que pas validé)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autre_partie: Option<AuteurResponse>,
}

#[derive(Debug, Serialize)]
pub struct CritereCommun {
    #[serde(rename = "type")]
    pub type_critere: String,
    pub valeur_a: String,
    pub valeur_b: String,
    pub score: i32,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub contenu: String,
    pub est_contact: bool,       // true si c'est un partage de contact
    pub type_contact: Option<String>,
    pub valeur_contact: Option<String>,  // Déchiffrée pour le destinataire
    pub nom_contact: Option<String>,
    pub lu: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AvisListeResponse {
    pub avis: Vec<AvisRechercheResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_avis_actifs: i64,
    pub total_retrouvailles: i64,   // Correspondances validées
    pub total_utilisateurs_trouvables: i64,
}

// ============================================================
// Query Params : Filtres de recherche
// ============================================================

#[derive(Debug, Deserialize)]
pub struct AvisQueryParams {
    pub recherche: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub ecole: Option<String>,
    pub annee_debut: Option<i32>,
    pub annee_fin: Option<i32>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RechercheForm {
    pub criteres: Vec<CritereForm>,
}

#[derive(Debug, Deserialize)]
pub struct CritereForm {
    pub type_critere: String,
    pub valeur: String,
    pub poids: Option<i32>,
    pub pays_id: Option<Uuid>,
}

// ============================================================
// Create/Update DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct CreerAvisForm {
    pub titre: String,
    pub description: Option<String>,
    pub relation: Option<String>,
    pub est_anonyme: Option<bool>,
    pub criteres: Vec<CritereForm>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierAvisForm {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub relation: Option<String>,
    pub est_anonyme: Option<bool>,
    pub criteres: Option<Vec<CritereForm>>,
}

#[derive(Debug, Deserialize)]
pub struct EnvoyerMessageForm {
    pub contenu: String,
}

#[derive(Debug, Deserialize)]
pub struct PartagerContactForm {
    pub type_contact: String,       // "telephone", "email", "telephone_proche"
    pub valeur: String,             // Le numéro ou email
    pub nom_contact: Option<String>, // Nom du proche si applicable
    pub message: Option<String>,     // Message accompagnant le partage
}

#[derive(Debug, Deserialize)]
pub struct PreferencesForm {
    pub est_trouvable: bool,
    pub anciens_noms: Option<Vec<String>>,
    pub anciennes_villes: Option<Vec<String>>,
    pub anciennes_ecoles: Option<Vec<String>>,
    pub anciennes_entreprises: Option<Vec<String>>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SignalementForm {
    pub avis_id: Option<Uuid>,
    pub correspondance_id: Option<Uuid>,
    pub motif: String,
    pub description: Option<String>,
}
```

---

## Handlers : Logique métier clé

### `creer_avis` : Création d'un avis

```
1. Valider les données (titre non vide, au moins 2 critères)
2. Vérifier le rate limiting (max 5 avis actifs par utilisateur)
3. INSERT dans avis_recherche
4. INSERT des critères (avec normalisation automatique via trigger)
5. Lancer le matching en arrière-plan (tokio::spawn)
6. Retourner l'avis créé
7. Audit: log_action("creer", "retrouve_amis.avis_recherche", ...)
```

### `confirmer_correspondance` : Double opt-in

```
1. Charger la correspondance
2. Vérifier que l'utilisateur est bien partie A ou B
3. Si partie A : UPDATE etat = 'confirmee_a', confirme_a_le = NOW()
4. Si partie B : UPDATE etat = 'confirmee_b', confirme_b_le = NOW()
5. Si les deux ont confirmé : UPDATE etat = 'validee'
   → La messagerie est maintenant ouverte
6. Audit
```

### `partager_contact` : Partage sécurisé de coordonnées

```
1. Vérifier que la correspondance est 'validee'
2. Vérifier que l'utilisateur est partie A ou B
3. Chiffrer la valeur du contact (AES-256-GCM côté serveur)
4. INSERT message avec type_contact et valeur chiffrée
5. Audit (sans logger la valeur en clair)
```

### `recherche_anonyme` : Recherche sans compte

```
1. Rate limiting strict (3 recherches / IP / heure)
2. Accepter les critères de recherche
3. Exécuter le matching (version simplifiée)
4. Retourner les résultats SANS identité des chercheurs
5. Afficher un CTA "Créez un compte pour entrer en contact"
```

---

## Configuration des routes

Ajout dans `src/routes.rs` :

```rust
// === Retrouve Amis (public) ===
.service(web::scope("/retrouve-amis")
    // Sans auth
    .route("/recherche-anonyme", web::post().to(retrouve_amis::recherche_anonyme))
    .route("/stats", web::get().to(retrouve_amis::statistiques))
    // Avec auth
    .route("", web::get().to(retrouve_amis::lister_avis))
    .route("/{id}", web::get().to(retrouve_amis::obtenir_avis))
    .route("", web::post().to(retrouve_amis::creer_avis))
    .route("/{id}", web::put().to(retrouve_amis::modifier_avis))
    .route("/{id}", web::delete().to(retrouve_amis::supprimer_avis))
    .route("/{id}/etat", web::patch().to(retrouve_amis::changer_etat_avis))
    .route("/mes-recherches", web::get().to(retrouve_amis::mes_recherches))
    .route("/rechercher", web::post().to(retrouve_amis::rechercher))
    // Correspondances
    .route("/correspondances", web::get().to(retrouve_amis::mes_correspondances))
    .route("/correspondances/{id}", web::get().to(retrouve_amis::obtenir_correspondance))
    .route("/correspondances/{id}/confirmer", web::post().to(retrouve_amis::confirmer_correspondance))
    .route("/correspondances/{id}/rejeter", web::post().to(retrouve_amis::rejeter_correspondance))
    // Messages
    .route("/correspondances/{id}/messages", web::get().to(retrouve_amis::lister_messages))
    .route("/correspondances/{id}/messages", web::post().to(retrouve_amis::envoyer_message))
    .route("/correspondances/{id}/messages/contact", web::post().to(retrouve_amis::partager_contact))
    .route("/correspondances/{id}/messages/lire", web::patch().to(retrouve_amis::marquer_lu))
    // Préférences
    .route("/preferences", web::get().to(retrouve_amis::obtenir_preferences))
    .route("/preferences", web::put().to(retrouve_amis::modifier_preferences))
    // Signalement
    .route("/signaler", web::post().to(retrouve_amis::signaler))
)

// === Admin Retrouve Amis ===
// Dans le scope /api/admin existant :
.route("/retrouve-amis", web::get().to(admin::retrouve_amis::lister_avis))
.route("/retrouve-amis/{id}", web::get().to(admin::retrouve_amis::obtenir_avis))
.route("/retrouve-amis/{id}/etat", web::patch().to(admin::retrouve_amis::moderer_avis))
.route("/retrouve-amis/{id}", web::delete().to(admin::retrouve_amis::supprimer_avis))
.route("/retrouve-amis/signalements", web::get().to(admin::retrouve_amis::lister_signalements))
.route("/retrouve-amis/signalements/{id}", web::get().to(admin::retrouve_amis::obtenir_signalement))
.route("/retrouve-amis/signalements/{id}", web::patch().to(admin::retrouve_amis::traiter_signalement))
.route("/retrouve-amis/stats", web::get().to(admin::retrouve_amis::statistiques_admin))
```

---

## Service de matching

Le matching sera implémenté comme un service séparé dans `src/services/matching.rs` :

```rust
/// Exécute le matching pour un avis donné
/// Appelé :
///   - À la création d'un avis (matching immédiat)
///   - Périodiquement via un job (toutes les heures pour les avis actifs)
pub async fn executer_matching(pool: &PgPool, avis_id: Uuid) -> Result<i32, ApiErreur> {
    // 1. Charger les critères de l'avis
    // 2. Chercher les correspondances potentielles :
    //    a) Autres avis actifs avec critères similaires
    //    b) Profils trouvables avec données correspondantes
    // 3. Calculer le score pour chaque candidat
    // 4. Créer les correspondances au-dessus du seuil (score >= 40)
    // 5. Logger dans journal_matching
    // 6. Retourner le nombre de correspondances créées
}
```

Voir [algorithme_matching.md](./algorithme_matching.md) pour le détail de l'algorithme.

---

## Audit

Toutes les mutations seront instrumentées avec `services::audit::log_action` :

| Action | Table | Contexte |
|--------|-------|----------|
| `creer` | `retrouve_amis.avis_recherche` | Création d'avis |
| `modifier` | `retrouve_amis.avis_recherche` | Modification d'avis |
| `supprimer` | `retrouve_amis.avis_recherche` | Suppression d'avis |
| `changer_etat` | `retrouve_amis.avis_recherche` | Changement d'état |
| `confirmer` | `retrouve_amis.correspondance` | Confirmation correspondance |
| `rejeter` | `retrouve_amis.correspondance` | Rejet correspondance |
| `envoyer_message` | `retrouve_amis.message_correspondance` | Envoi message |
| `partager_contact` | `retrouve_amis.message_correspondance` | Partage contact (sans valeur) |
| `signaler` | `retrouve_amis.signalement` | Signalement |
| `moderer` | `retrouve_amis.avis_recherche` | Action admin |
| `traiter_signalement` | `retrouve_amis.signalement` | Traitement admin |

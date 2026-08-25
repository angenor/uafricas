# Quickstart : Arbre généalogique

**Branch**: `001-personnes-arbre` | **Date**: 2026-03-15

Ce guide explique comment démarrer le développement de cette feature depuis zéro.

## Prérequis

- Docker en cours d'exécution (`docker compose up -d`)
- PostgreSQL accessible sur `localhost:5432`
- Rust toolchain installée (Edition 2024)
- pnpm installé

## Ordre d'implémentation

Suivre cet ordre strict (SQL → Backend → Frontend) conformément au Principe III de la constitution.

```
1. SQL (schema + tables)
2. Backend (models → handlers → routes)
3. Frontend (mock → composable → pages → composants)
```

---

## Étape 1 : Appliquer le schéma SQL

Créer le fichier `uafricas_backend/doc/bd/schemas/23_arbre_genealogique.sql` avec le contenu défini dans [data-model.md](./data-model.md).

Puis ajouter l'import dans `uafricas_backend/doc/bd/schema.sql` :
```sql
\ir schemas/23_arbre_genealogique.sql
```

Réinitialiser la base :
```bash
docker compose down -v
docker compose up -d
# Attendre ~5s que PostgreSQL soit prêt
```

Vérifier dans Adminer (`http://localhost:8088`) que le schema `arbre_genealogique` contient bien les 4 tables.

---

## Étape 2 : Modèles Rust

Créer `uafricas_backend/src/models/arbre_genealogique.rs` avec les structs définies dans [data-model.md](./data-model.md).

Ajouter le module dans `uafricas_backend/src/models/mod.rs` :
```rust
pub mod arbre_genealogique;
```

Vérifier la compilation :
```bash
cd uafricas_backend
cargo check
```

---

## Étape 3 : Handler Rust

Créer `uafricas_backend/src/handlers/arbre_genealogique.rs`.

Les 7 fonctions à implémenter (dans l'ordre de dépendance) :

```rust
// 1. Lecture seule (pas de JWT complexe, bon pour commencer)
pub async fn lister_personnes(pool, params, claims) -> Result<HttpResponse, ApiErreur>
pub async fn obtenir_personne(pool, id, claims) -> Result<HttpResponse, ApiErreur>

// 2. Mutations (nécessitent la transaction sqlx)
pub async fn creer_personne(pool, body, claims) -> Result<HttpResponse, ApiErreur>
pub async fn modifier_personne(pool, id, body, claims) -> Result<HttpResponse, ApiErreur>
pub async fn supprimer_personne(pool, id, claims) -> Result<HttpResponse, ApiErreur>

// 3. Liens
pub async fn creer_lien(pool, body, claims) -> Result<HttpResponse, ApiErreur>
pub async fn supprimer_lien(pool, id, claims) -> Result<HttpResponse, ApiErreur>
```

Ajouter le module dans `uafricas_backend/src/handlers/mod.rs` :
```rust
pub mod arbre_genealogique;
```

---

## Étape 4 : Routes

Dans `uafricas_backend/src/routes.rs`, ajouter dans `configurer_routes()` :
```rust
use crate::handlers::arbre_genealogique;

// Dans cfg.service(web::scope("/api") ...
.service(
    web::scope("/arbre")
        .route("/personnes", web::get().to(arbre_genealogique::lister_personnes))
        .route("/personnes", web::post().to(arbre_genealogique::creer_personne))
        .route("/personnes/{id}", web::get().to(arbre_genealogique::obtenir_personne))
        .route("/personnes/{id}", web::put().to(arbre_genealogique::modifier_personne))
        .route("/personnes/{id}", web::delete().to(arbre_genealogique::supprimer_personne))
        .route("/personnes/{id}/photo", web::post().to(arbre_genealogique::uploader_photo))
        .route("/liens", web::post().to(arbre_genealogique::creer_lien))
        .route("/liens/{id}", web::delete().to(arbre_genealogique::supprimer_lien))
)
```

Relancer le backend :
```bash
kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run
```

Tester rapidement :
```bash
# Créer une personne (avec token JWT valide)
curl -X POST http://localhost:8080/api/arbre/personnes \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"nom": "Diallo"}'
```

---

## Étape 5 : Mock frontend

Créer `uafricas_frontend/app/mocks/arbre-genealogique.ts` avec le contenu défini dans [data-model.md](./data-model.md).

---

## Étape 6 : Composable

Créer `uafricas_frontend/app/composables/useArbreGenealogique.ts` :

```typescript
import type { PersonneListe, PersonneDetail, CreerPersonneForm, ModifierPersonneForm, CreerLienForm } from '~/mocks/arbre-genealogique'

export const useArbreGenealogique = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const { accessToken } = useUserStore()

  const enTete = () => ({ Authorization: `Bearer ${accessToken}` })

  const listerPersonnes = async (params?: { page?: number; par_page?: number; recherche?: string }) => {
    return $fetch<ApiResponse<PersonneListe>>(`${apiBase}/api/arbre/personnes`, {
      headers: enTete(),
      query: params,
    })
  }

  const obtenirPersonne = async (id: string) => {
    return $fetch<ApiResponse<PersonneDetail>>(`${apiBase}/api/arbre/personnes/${id}`, {
      headers: enTete(),
    })
  }

  const creerPersonne = async (form: CreerPersonneForm) => {
    return $fetch<ApiResponse<PersonneDetail>>(`${apiBase}/api/arbre/personnes`, {
      method: 'POST',
      headers: enTete(),
      body: form,
    })
  }

  const modifierPersonne = async (id: string, form: ModifierPersonneForm) => {
    return $fetch<ApiResponse<PersonneDetail>>(`${apiBase}/api/arbre/personnes/${id}`, {
      method: 'PUT',
      headers: enTete(),
      body: form,
    })
  }

  const supprimerPersonne = async (id: string) => {
    return $fetch<ApiResponse<{ message: string }>>(`${apiBase}/api/arbre/personnes/${id}`, {
      method: 'DELETE',
      headers: enTete(),
    })
  }

  const creerLien = async (form: CreerLienForm) => {
    return $fetch<ApiResponse<unknown>>(`${apiBase}/api/arbre/liens`, {
      method: 'POST',
      headers: enTete(),
      body: form,
    })
  }

  const supprimerLien = async (id: string) => {
    return $fetch<ApiResponse<{ message: string }>>(`${apiBase}/api/arbre/liens/${id}`, {
      method: 'DELETE',
      headers: enTete(),
    })
  }

  return { listerPersonnes, obtenirPersonne, creerPersonne, modifierPersonne, supprimerPersonne, creerLien, supprimerLien }
}
```

---

## Étape 7 : Pages et composants

Créer les pages :
- `app/pages/arbre-genealogique/index.vue`, liste paginée avec recherche
- `app/pages/arbre-genealogique/[id].vue`, fiche détail avec liens

Créer les composants :
- `app/components/arbre-genealogique/PersonneCard.vue`
- `app/components/arbre-genealogique/PersonneForm.vue`
- `app/components/arbre-genealogique/LienFamilialForm.vue`

**Rappel Tailwind** : ces pages sont publiques (accessibles avec auth utilisateur standard). Utiliser Tailwind CSS v4 pur : pas de classes daisyUI (`btn`, `card`, `modal`, etc.).

---

## Points d'attention

### Audit obligatoire
Toutes les mutations backend doivent appeler `audit::log_action` :
```rust
use crate::services::audit;
audit::log_action(&pool, "creer_personne", claims.utilisateur_id, "arbre_genealogique.personnes", &id, None, Some(&json_apres), &ip, &user_agent).await;
```

### Détection de cycle
Avant d'insérer un lien `pere|mere|parent`, exécuter la requête CTE décrite dans [research.md](./research.md) (Décision 3). Retourner `422` si un cycle est détecté.

### Transaction atomique pour la suppression
Dans `supprimer_personne`, utiliser `sqlx::transaction` :
1. Soft delete du rattachement
2. Soft delete des liens du rattachement
3. Count des rattachements actifs restants pour la personne
4. Si 0 → soft delete de la Personne réelle

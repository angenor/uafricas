# Contrat — Avis de visiteur sur un site (note 1–5)

Écriture directe (publication immédiate, D2). Enveloppe `{ success, data, error }`.

## GET /api/sites-touristiques/{site_id}/avis

Liste paginée des avis visibles d'un site + agrégats. **Public**.

**Query** : `page` (≥1, défaut 1), `par_page` (1..50, défaut 10).

**200** :
```jsonc
{
  "note_moyenne": 4.3,        // null si aucun avis
  "nombre_total": 12,
  "avis": [
    {
      "id": "uuid",
      "utilisateur": { "id": "uuid|null", "nom": "Koffi", "prenom": "Awa", "photo_url": null },
      "note": 5,
      "commentaire": "Superbe endroit, personnel accueillant.",
      "created_at": "2026-05-21T09:00:00Z"
    }
  ]
}
```
Exclut `deleted_at IS NOT NULL` et `masque_par_admin = TRUE`. Auteurs supprimés anonymisés
(`utilisateur.id = null`, nom « Contributeur »).

---

## POST /api/sites-touristiques/{site_id}/avis

Dépose ou met à jour l'avis de l'utilisateur connecté (upsert sur l'avis actif). **Auth requise**.

**Body** :
```jsonc
{ "note": 4, "commentaire": "Très bon accueil." }
```

**Règles** :
- `note` ∈ [1,5], `commentaire` non vide (1..2000) — sinon **422**.
- Un avis actif au plus par (utilisateur, site) : si déjà présent → mise à jour (pas de doublon,
  FR-015a). Réponse **201** (création) ou **200** (mise à jour) avec l'avis.
- Site inexistant/supprimé → **404**.

**Erreurs** : 401, 404, 422.

---

## (Admin) PATCH /api/admin/sites-touristiques/avis/{avis_id}/masquer

Masque/affiche un avis inapproprié (FR-015d). **Admin** (`verifier_permission!`). Audité.

**Body** : `{ "masque": true }`  → **200** `{ id, masque }`.

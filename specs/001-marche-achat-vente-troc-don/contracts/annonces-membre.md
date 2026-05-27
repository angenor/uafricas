# Contrat API — Annonces membre

Toutes les routes ci-dessous sont sous `/api/annonces` et **exigent un JWT valide** (`Authorization: Bearer <token>`). Un JWT n'est émis qu'aux comptes `actif` → FR-007 satisfait. Réponses au format `ApiResponse<T>` (`{ success, data, error }`). Mutations auditées (Principe VII).

> **Ordre de routage** : déclarer `GET /annonces/mes-annonces` et `GET /annonces/favoris` AVANT `GET /annonces/{id}` (D8).

## POST /api/annonces — Publier une annonce (FR-001..FR-007)

- **Body** : `multipart/form-data`
  - champs texte : `titre`, `description`, `type_operation` (`vente|troc|don`), `categorie_id`, `condition_article?`, `prix?`, `devise?`, `prix_negociable?`, `ville?`, `adresse?`, `longitude?`, `latitude?`, `quantite?`, `pays_ids?` (CSV ou répété)
  - fichiers : `photos[]` (1..5, ≤ 3 Mo, image/jpeg|png|webp)
- **Règles** : `cree_par = courant` (forcé) ; `type_contact='messagerie_plateforme'` ; `etat='publiee'` immédiat ; au moins 1 photo, 1ʳᵉ = principale ; `prix` requis si `vente`.
- **201** : `{ success: true, data: AnnonceDetailResponse }`
- **400** : champ manquant / photo invalide / type_operation hors {vente,troc,don} / prix manquant en vente
- **401** : non authentifié

## GET /api/annonces/mes-annonces — Mes annonces (FR-016)

- **Query** : `page?`, `par_page?` (≤ 50), `etat?` (filtre optionnel)
- **Règle** : seulement les annonces où `cree_par = courant` (tous états sauf jamais masqué à l'auteur), `deleted_at IS NULL` (les supprimées exclues).
- **200** : `{ success: true, data: { annonces: [...], total, page, par_page, total_pages } }` — chaque item inclut `etat`.

## PUT /api/annonces/{id} — Modifier (FR-017, FR-020)

- **Body** : `multipart/form-data` (mêmes champs qu'en création, tous optionnels) ; gestion photos via endpoints dédiés ci-dessous ou champs `photos[]` ajoutées.
- **Règles** : `403` si `cree_par <> courant` ; `404` si introuvable/supprimée ; régénérer `slug` si `titre` change.
- **200** : `{ success: true, data: AnnonceDetailResponse }`

## PATCH /api/annonces/{id}/conclure — Marquer conclue (FR-018)

- **Body** : aucun (ou `{}`)
- **Règles** : `403` si non-propriétaire ; passe `etat` de `publiee` → `conclue` ; retirée du listing public, conservée dans « Mes annonces ».
- **200** : `{ success: true, data: { id, etat: "conclue" } }`

## DELETE /api/annonces/{id} — Supprimer (FR-019, FR-020)

- **Règles** : `403` si non-propriétaire ; soft delete (`etat='supprimee'`, `deleted_at=NOW()`).
- **200** : `{ success: true, data: null }`

## POST /api/annonces/{id}/medias — Ajouter des photos (propriétaire)

- **Body** : `multipart/form-data` `photos[]` ; refus si total > 5.
- **200** : `{ success: true, data: [AnnonceMediaResponse] }`

## DELETE /api/annonces/{id}/medias/{media_id} — Retirer une photo (propriétaire)

- **Règles** : si la photo principale est retirée, promouvoir la suivante en principale.
- **200** : `{ success: true, data: null }`

## Codes d'erreur communs

| Code | Cas |
|------|-----|
| 400 | Validation (champ, photo, type, prix) |
| 401 | JWT absent/invalide |
| 403 | Action sur une annonce d'autrui |
| 404 | Annonce introuvable / supprimée |
| 413 | Photo > 3 Mo (ou rejet `image_validation`) |

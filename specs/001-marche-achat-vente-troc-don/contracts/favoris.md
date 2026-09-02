# Contrat API : Favoris (FR-021, FR-022)

Routes sous `/api/annonces`, **JWT requis**. Table `marketplace.annonce_favori` (PK composite `utilisateur_id + annonce_id`). Mutations auditées.

## POST /api/annonces/{id}/favori : Ajouter aux favoris

- **Règles** : idempotent (`INSERT ... ON CONFLICT DO NOTHING`) ; `404` si annonce introuvable/non publiée.
- **200/201** : `{ success: true, data: { annonce_id, favori: true } }`
- **401** : non authentifié

## DELETE /api/annonces/{id}/favori : Retirer des favoris

- **Règles** : idempotent (`DELETE` silencieux si absent).
- **200** : `{ success: true, data: { annonce_id, favori: false } }`

## GET /api/annonces/favoris : Mes favoris

- **Query** : `page?`, `par_page?` (≤ 50)
- **Règle** : annonces favorites du membre courant **encore publiées** (les conclues/supprimées exclues du rendu, mais le lien favori peut subsister).
- **200** : `{ success: true, data: { annonces: [AnnonceResponse], total, page, par_page, total_pages } }`

> Optionnel UX : le listing public (`GET /api/annonces`) et le détail (`GET /api/annonces/{id}`) peuvent, si un JWT est présent, renvoyer un booléen `est_favori` pour l'état du bouton. Sinon le frontend charge `/favoris` pour connaître l'état. À trancher en implémentation (Principe V : commencer simple, sans `est_favori` côté listing).

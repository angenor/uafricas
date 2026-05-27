# Contrat — Sites touristiques (public + contribution)

Toutes les réponses suivent l'enveloppe `{ success, data, error }`.

## GET /api/fiches-pays/{id}/sites-touristiques

Liste les sites validés d'une fiche. Filtres optionnels.

**Query** : `categorie` (`emblematique` | `prive`), `sous_type` (une des 20 valeurs).

**200** — `data: SiteTouristique[]`
```jsonc
{
  "id": "uuid",
  "fiche_pays_id": "uuid",
  "nom": "Plage de Grand-Bassam",
  "categorie": "emblematique",
  "sous_type": "plage",
  "description": "…",
  "info_pertinente": "Accès libre, baignade surveillée le week-end.",
  "image_url": "/uploads/opportunite-afrique/images/….jpg",
  "gestionnaire": "Mairie de Grand-Bassam",
  "ville": "Grand-Bassam",
  "village": null,
  "latitude": 5.1962,
  "longitude": -3.7388,
  // Contacts (publics — CL résolue) ; renseignés surtout pour les sites privés
  "contact_telephone": null,
  "contact_courriel": null,
  "contact_adresse": null,
  // Constitution légale (facultatif)
  "constitution_statut_juridique": null,
  "constitution_numero": null,
  "constitution_document_url": null,
  // Fiabilité
  "verifie": true,
  // Agrégats avis
  "note_moyenne": 4.3,
  "nombre_avis": 12,
  "created_at": "2026-05-20T10:00:00Z"
}
```

**Notes** : filtre `categorie`/`sous_type` ignoré si valeur invalide (comme l'existant). Toujours
`deleted_at IS NULL`.

---

## POST /api/fiches-pays/{id}/contributions  (mode Afripulse — site touristique)

Propose un ajout/édition/suppression de site (workflow existant, validation admin requise).
**Auth requise** (JWT Bearer). Réponse **202** (en attente).

**Body (ajout)** :
```jsonc
{
  "type_objet_contribution": "site_touristique",
  "section_afripulse": "sites_emblematiques",   // ou "sites_prives"
  "type_contribution": "ajout",                  // | "edition" | "suppression"
  "target_id": null,                              // requis pour edition/suppression
  "nouvelle_valeur_jsonb": {
    "categorie": "prive",
    "sous_type": "hotel",
    "nom": "Hôtel Ivoire",
    "description": "…",
    "info_pertinente": "Parking gratuit, piscine.",
    "image_url": "/uploads/opportunite-afrique/images/….jpg",
    "gestionnaire": "Groupe X",
    "ville": "Abidjan",
    "village": null,
    "latitude": 5.32,
    "longitude": -4.01,
    "contact_telephone": "+225 01 02 03 04 05",
    "contact_courriel": "contact@hotel.ci",
    "contact_adresse": "Boulevard …, Cocody",
    "constitution_statut_juridique": "SARL",
    "constitution_numero": "CI-ABJ-2020-B-12345",
    "constitution_document_url": "/uploads/opportunite-afrique/images/….png"
  },
  "justification": "Ajout d'un hôtel de référence."
}
```

**Validation (422 sinon)** :
- Requis : `nom`, `gestionnaire`, `ville`, `info_pertinente`, `latitude`, `longitude`, `sous_type`.
- `sous_type` cohérent avec `categorie` (sinon 422 « sous-type incompatible avec la famille »).
- Si `categorie = "prive"` : au moins un de `contact_telephone` / `contact_courriel` /
  `contact_adresse` (sinon 422 « contact gestionnaire requis pour un site privé »).
- `edition`/`suppression` : `target_id` requis (422 sinon).

**Erreurs** : 401 (non auth), 404 (fiche inexistante), 422 (validation), 429 (rate-limit).

**Application** : à l'approbation admin, `appliquer_contribution_afripulse` insère/MAJ la ligne
`site_touristique` avec les nouveaux champs (le badge `verifie` n'est PAS modifiable par ce canal).

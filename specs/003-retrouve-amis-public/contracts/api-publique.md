# Contrat API Publique: 003-retrouve-amis-public

**Date**: 2026-03-15 | **Branch**: `003-retrouve-amis-public`

## Endpoints modifiés

### POST `/api/retrouve-amis/avis` (multipart/form-data)

**Avant** : JSON body avec `CreerAvisRecherche`
**Après** : `multipart/form-data` (pour supporter l'upload photo)

**Champs du formulaire** :

| Champ | Type | Obligatoire | Validation |
|-------|------|-------------|------------|
| `nom_recherche` | string | Oui | 1-100 chars, non vide |
| `prenom_recherche` | string | Non | max 100 chars |
| `surnom` | string | Non | max 100 chars |
| `est_anonyme` | boolean | Non | default: false |
| `genre_recherche` | string | Non | "homme" ou "femme" |
| `type_relation` | string | Non | enum: amis_enfance, amis_ecole, collegue, connaissance, frere_soeur, parent |
| `comment_connu` | string | Non | max 500 chars |
| `localite_rencontre` | string | Non | max 200 chars |
| `ecole_rencontre` | string | Non | max 250 chars |
| `ville_rencontre` | string | Non | max 200 chars |
| `jamais_rencontre` | boolean | Non | default: false |
| `photo` | file | Non | JPEG, PNG, WebP, max 5 Mo |
| `description_physique` | string | Non | texte libre |
| `description` | string | Non | texte libre (contexte général) |
| `partage_coordonnees` | boolean | Non | default: false |
| `coordonnees_email` | string | Non* | email valide |
| `coordonnees_telephone` | string | Non* | max 50 chars |
| `coordonnees_whatsapp` | string | Non* | max 50 chars |

*Au moins un requis si `partage_coordonnees = true`

**Validation supplémentaire** :
- Au moins un critère en plus du nom : `type_relation` OU `localite_rencontre` OU `ecole_rencontre` OU `ville_rencontre` OU `jamais_rencontre`
- Max 10 avis actifs par utilisateur
- Publication automatique (est_public = true, slug auto-généré)

**Réponse** (201) :
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "etat": "actif",
    "slug": "string",
    "correspondances_trouvees": 0
  }
}
```

**Champs anciens conservés mais optionnels** : `ecole`, `ville`, `pays_id`, `periode_debut`, `periode_fin` — restent dans le schéma pour rétrocompatibilité avec les avis existants mais ne sont plus dans le formulaire principal.

---

### PUT `/api/retrouve-amis/avis/{id}` (multipart/form-data)

Mêmes champs que POST. Passe aussi en multipart pour l'upload photo.

**Comportements** :
- Si `photo` est fourni : remplace la photo existante (supprime l'ancienne)
- Si `photo` est absent : la photo existante est conservée
- Relance le recoupement automatique

---

### GET `/api/retrouve-amis/public/rechercher`

**Paramètres de requête ajoutés** :

| Param | Type | Description |
|-------|------|-------------|
| `type_relation` | string | Filtre par type de relation (enum) |

**Paramètres existants conservés** : `recherche`, `pays_id`, `ville`, `ecole`, `tri`, `ordre`, `page`, `par_page`

**Réponse** — champs ajoutés dans chaque élément :

```json
{
  "success": true,
  "data": {
    "avis": [
      {
        "id": "uuid",
        "slug": "string",
        "nom_recherche": "string",
        "prenom_recherche": "string | null",
        "genre_recherche": "homme | femme | null",
        "type_relation": "amis_enfance | ... | null",
        "localite_rencontre": "string | null",
        "ecole_rencontre": "string | null",
        "ville_rencontre": "string | null",
        "photo_url": "string | null",
        "description_physique": "string | null",
        "auteur_anonyme": true,
        "auteur_pseudonyme": "Amadou D. | null",
        "compteur_partages": 0,
        "created_at": "ISO 8601"
      }
    ],
    "pagination": {
      "page": 1,
      "par_page": 12,
      "total": 100,
      "pages": 9
    }
  }
}
```

**Note** : `auteur_pseudonyme` est `null` si `est_anonyme = true`.

---

### GET `/api/retrouve-amis/public/{slug}`

**Réponse** — champs ajoutés :

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "slug": "string",
    "nom_recherche": "string",
    "prenom_recherche": "string | null",
    "surnom": "string | null",
    "genre_recherche": "homme | femme | null",
    "type_relation": "amis_enfance | ... | null",
    "comment_connu": "string | null",
    "localite_rencontre": "string | null",
    "ecole_rencontre": "string | null",
    "ville_rencontre": "string | null",
    "jamais_rencontre": false,
    "photo_url": "string | null",
    "description_physique": "string | null",
    "description": "string | null",
    "auteur_anonyme": true,
    "auteur_pseudonyme": "Amadou D. | null",
    "compteur_partages": 0,
    "created_at": "ISO 8601",
    "etat": "actif"
  }
}
```

**Champs JAMAIS inclus** : `coordonnees_email`, `coordonnees_telephone`, `coordonnees_whatsapp`, `auteur_id`, email/téléphone de l'auteur.

## Endpoint supprimé

### ~~PATCH `/api/retrouve-amis/avis/{id}/publier`~~

Supprimé — la publication est automatique à la création. Le champ `est_public` est toujours `true` par défaut.

## Endpoints inchangés

Tous les autres endpoints restent identiques :
- Correspondances (lister, détail, accepter, refuser)
- Notifications (lister, marquer lu, tout marquer lu)
- Tableau de bord
- Signalements (avis, public)
- Demandes de retrait
- Profil trouvable + parcours
- Partage (incrémenter compteur)
- Réponse publique
- Admin (tous les endpoints)

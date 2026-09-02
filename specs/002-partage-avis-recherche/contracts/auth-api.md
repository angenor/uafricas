# Contrats API Authentifiés (JWT requis)

**Branch**: `002-partage-avis-recherche` | **Date**: 2026-03-02

Ces endpoints nécessitent un JWT valide dans l'en-tête `Authorization: Bearer {token}`.

---

## PATCH /api/retrouve-amis/avis/{id}/publier

**Description**: Activer ou désactiver la visibilité publique d'un avis.

**Autorisation**: Auteur de l'avis uniquement.

**Corps**:
```json
{
  "est_public": true
}
```

**Réponse 200** (activation):
```json
{
  "succes": true,
  "donnees": {
    "id": "uuid",
    "est_public": true,
    "slug": "keita-fatou-a3f8b2c1",
    "date_publication_publique": "2026-03-01T10:00:00Z"
  }
}
```

**Réponse 200** (désactivation):
```json
{
  "succes": true,
  "donnees": {
    "id": "uuid",
    "est_public": false,
    "slug": "keita-fatou-a3f8b2c1"
  }
}
```

**Règles**:
- Seuls les avis avec `etat = 'actif'` peuvent être rendus publics
- Le slug est généré à la première activation (et conservé lors des toggles suivants)
- `date_publication_publique` est set une seule fois à la première activation
- Audité via `audit::log_action`

**Erreurs**:
- 403 : L'utilisateur n'est pas l'auteur
- 400 : L'avis n'est pas actif (`etat != 'actif'`)
- 404 : Avis non trouvé

---

## POST /api/retrouve-amis/public/{slug}/repondre

**Description**: Répondre à un avis public (créer une correspondance).

**Autorisation**: Utilisateur connecté (pas l'auteur de l'avis).

**Corps**:
```json
{
  "type_reponse": "je_suis_cette_personne",
  "message": "Bonjour, je pense être la personne que vous recherchez. Nous étions ensemble au Lycée de Bamako entre 2000 et 2005."
}
```

**Valeurs de `type_reponse`**: `je_suis_cette_personne`, `je_la_connais`, `jai_des_informations`

**Réponse 201**:
```json
{
  "succes": true,
  "donnees": {
    "id": "uuid",
    "correspondance_id": "uuid",
    "message": "Votre réponse a été envoyée. L'auteur de l'avis sera notifié."
  }
}
```

**Règles**:
- L'avis doit être public ET actif (`est_public = TRUE AND etat = 'actif'`)
- Le répondeur ne doit pas être l'auteur de l'avis
- Le répondeur ne doit pas avoir déjà répondu à cet avis (UNIQUE constraint)
- Le répondeur ne doit pas être dans la blacklist de l'auteur
- Le répondeur ne doit pas dépasser 10 réponses par jour (tous avis confondus)
- Crée automatiquement une `correspondance` (type_cible='profil', score=70, details_score={"source":"reponse_publique"})
- Crée une notification `reponse_publique` pour l'auteur
- Audité via `audit::log_action`

**Erreurs**:
- 401 : Non connecté
- 403 : L'utilisateur est l'auteur de l'avis
- 404 : Avis non trouvé ou non public/actif
- 409 : Déjà répondu à cet avis
- 429 : Limite de 10 réponses/jour atteinte

---

## POST /api/retrouve-amis/public/{slug}/signaler

**Description**: Signaler un avis public (connexion requise).

**Autorisation**: Utilisateur connecté.

**Corps**:
```json
{
  "motif": "contenu_abusif",
  "description": "Cet avis contient des informations fausses sur la personne recherchée."
}
```

**Valeurs de `motif`**: `contenu_abusif`, `usurpation_identite`, `harcelement`, `autre`

**Réponse 201**:
```json
{
  "succes": true,
  "donnees": {
    "id": "uuid"
  }
}
```

**Règles**:
- L'avis doit être public ET actif
- Le signaleur ne peut pas être l'auteur de l'avis
- Un seul signalement par utilisateur par avis (UNIQUE existant)
- `source` = `'page_publique'` (vs `'correspondance'` pour les signalements existants)
- Si 3 signalements distincts atteints → suspension automatique de l'avis (`etat = 'suspendu'`)
- Audité via `audit::log_action`

**Erreurs**:
- 401 : Non connecté
- 404 : Avis non trouvé ou non public/actif
- 409 : Déjà signalé cet avis

---

## POST /api/retrouve-amis/public/{slug}/demande-retrait

**Description**: Demander le retrait d'un avis par la personne qui s'y reconnaît.

**Autorisation**: Utilisateur connecté (pas l'auteur de l'avis).

**Corps**:
```json
{
  "motif": "Je suis la personne recherchée dans cet avis et je ne souhaite pas être retrouvée."
}
```

**Réponse 201**:
```json
{
  "succes": true,
  "donnees": {
    "id": "uuid",
    "message": "L'avis a été immédiatement suspendu. Un administrateur examinera votre demande sous 72h."
  }
}
```

**Règles**:
- L'avis doit être public (`est_public = TRUE`)
- Le demandeur ne peut pas être l'auteur de l'avis
- Une seule demande par utilisateur par avis (UNIQUE constraint)
- **Effet immédiat** : L'avis passe en `etat = 'suspendu'`
- Crée une notification `demande_retrait` pour l'auteur
- Crée une notification `demande_retrait` pour les administrateurs
- Audité via `audit::log_action`

**Erreurs**:
- 401 : Non connecté
- 403 : L'utilisateur est l'auteur de l'avis
- 404 : Avis non trouvé ou non public
- 409 : Demande de retrait déjà soumise pour cet avis

---

## PATCH /api/admin/retrouve-amis/demandes-retrait/{id}/statuer

**Description**: Admin statue sur une demande de retrait.

**Autorisation**: Admin avec permission `retrouve_amis/modifier`.

**Corps**:
```json
{
  "decision": "approuvee",
  "commentaire": "La personne a fourni des preuves suffisantes de son identité."
}
```

**Valeurs de `decision`**: `approuvee`, `rejetee`

**Réponse 200**:
```json
{
  "succes": true,
  "donnees": {
    "id": "uuid",
    "etat": "approuvee",
    "avis_id": "uuid",
    "avis_etat": "suspendu",
    "avis_est_public": false
  }
}
```

**Règles**:
- La demande doit être en état `en_attente`
- Si `approuvee` : l'avis reste `suspendu` ET passe `est_public = FALSE` (retrait définitif)
- Si `rejetee` : l'avis revient à `etat = 'actif'` (réactivation) et `est_public = TRUE` (republication)
- Met à jour `decide_par`, `decision_at`, `commentaire_admin`
- Notification à l'auteur et au demandeur du résultat
- Audité via `audit::log_action`

**Erreurs**:
- 403 : Permission insuffisante
- 404 : Demande non trouvée
- 409 : Demande déjà traitée

---

## GET /api/admin/retrouve-amis/demandes-retrait

**Description**: Lister les demandes de retrait (admin).

**Autorisation**: Admin avec permission `retrouve_amis/voir`.

**Paramètres query**:
| Param | Type | Défaut | Description |
|-------|------|--------|-------------|
| `page` | u32 | 1 | Numéro de page |
| `par_page` | u32 | 20 | Résultats par page |
| `etat` | String | : | Filtrer par état (en_attente, approuvee, rejetee) |
| `tri_par` | String | "created_at" | Champ de tri |
| `tri_dir` | String | "desc" | Direction (asc, desc) |

**Réponse 200**:
```json
{
  "succes": true,
  "donnees": {
    "demandes": [
      {
        "id": "uuid",
        "avis_id": "uuid",
        "nom_recherche": "Keita Fatou",
        "demandeur": { "id": "uuid", "prenom": "Fatou", "nom": "K." },
        "auteur": { "id": "uuid", "prenom": "Amadou", "nom": "D." },
        "motif": "Je suis cette personne...",
        "etat": "en_attente",
        "date_suspension": "2026-03-01T10:00:00Z",
        "created_at": "2026-03-01T10:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "par_page": 20,
      "total": 3,
      "pages": 1
    }
  }
}
```

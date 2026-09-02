# Contrat d'API : Cadeaux virtuels (membre & public)

**Base** : `/api/engagement` · **Enveloppe** : `ApiResponse<T>` (`{ succes, message, donnees }`) comme le reste de la plateforme.

**Règle transversale de sécurité** : ni le montant, ni les points, ni le taux, ni le bénéficiaire ne sont acceptés du client. L'offreur vient du JWT, le prix et les points du catalogue, le bénéficiaire de `resoudre_beneficiaire` (research R4). Un client ne peut exprimer que **quel cadeau** et **sur quoi**.

---

## 1. `GET /api/engagement/cadeaux` : catalogue

**Auth** : publique.

**Réponse `200`**

```json
{
  "succes": true,
  "donnees": {
    "devise": "XOF",
    "taux_commission": 10,
    "paiement_simule": true,
    "cadeaux": [
      { "id": "uuid", "code": "drapeau_ua", "libelle": "Drapeau de l'Union Africaine",
        "description": null, "icone": "flag", "couleur": "amber",
        "prix": 2000, "points": 20, "ordre": 1 }
    ]
  }
}
```

Seuls les cadeaux `actif = TRUE` sont renvoyés, triés par `ordre` puis `points` décroissants. `paiement_simule` pilote l'affichage du bandeau d'avertissement (FR-020a) ; il vaut `NOT parametre_monetisation.paiement_reel_actif`.

---

## 2. `POST /api/engagement/cadeaux/envoyer`, créer une intention

**Auth** : JWT membre requis.

**Corps**

```json
{
  "cadeau_id": "uuid",
  "mode": "soutien_financier",
  "cible": { "type_objet": "video", "objet_id": "uuid" },
  "message": "Merci pour ce documentaire."
}
```

`cible` a **une seule forme** : `{ "type_objet": "<famille>", "objet_id": "<uuid>" }`.

Familles admises (identiques à celles éligibles au j'aime, FR-008) :

| `type_objet` | Cible |
|--------------|-------|
| `codimoi`, `factcheck`, `biblio_humaine`, `video`, `fiche_pays` | Le contenu |
| `chaine_tv`, `station_radio`, `programme_tele`, `programme_radio` | Le support ou le programme ; bénéficiaire = **propriétaire** |
| `personnalite_connue`, `recette_culinaire` | Élément Opportunité-Afrique doté d'un auteur |
| `profil` | Cadeau offert depuis un profil public ; `objet_id` = identifiant du membre visé |

`site_touristique` et `secteur_developpement` sont **refusés** (`409`) : ces éléments éditoriaux n'ont pas d'auteur enregistré (FR-008c). Le bouton d'envoi n'y est d'ailleurs pas proposé.

**Réponse `201`**

```json
{
  "succes": true,
  "donnees": {
    "transaction_id": "uuid",
    "reference_paiement": "SIM-2026-08-08-XXXXXXXX",
    "etat": "en_attente",
    "montant": 2000,
    "points": 20,
    "part_beneficiaire": 1800,
    "part_plateforme": 200,
    "beneficiaire": { "id": "uuid", "nom_affiche": "Awa D." },
    "simule": true,
    "expire_at": "2026-08-08T13:05:00Z"
  }
}
```

**Erreurs**

| Code | Cas |
|------|-----|
| `400` | `cible` malformée, famille inconnue, mode invalide |
| `401` | Non authentifié |
| `403` | Auto-cadeau (bénéficiaire résolu = offreur), message explicite (FR-023) |
| `404` | Cadeau inactif ou inexistant, contenu inexistant |
| `409` | Aucun bénéficiaire résolvable, support sans propriétaire déclaré, ou élément éditorial sans auteur (`site_touristique`, `secteur_developpement`) |

Aucun point n'est crédité, aucune cagnotte n'est touchée à cette étape.

---

## 3. `POST /api/engagement/paiements/{reference}/confirmer`, issue du paiement

**Auth** : JWT membre requis ; la transaction doit appartenir à l'appelant.

> Cette route **remplace le retour du prestataire**. À l'arrivée de CinetPay, elle devient un webhook signé et le corps `aboutir` disparaît ; le reste du contrat est inchangé (SC-012).

**Corps** : `{ "aboutir": true }`

**Réponse `200`**

```json
{
  "succes": true,
  "message": "Cadeau envoyé.",
  "donnees": {
    "transaction_id": "uuid",
    "etat": "abouti",
    "points_credites": 20,
    "beneficiaire": { "id": "uuid", "nom_affiche": "Awa D." }
  }
}
```

**Comportements imposés**

- Rejeu d'une confirmation déjà aboutie : `200` avec le **même** contenu, **0 point supplémentaire** (FR-022). L'idempotence est portée par `UPDATE … WHERE etat = 'en_attente'` puis par la clé `cadeau:{transaction_id}`.
- `aboutir = false` → `etat = 'echoue'`, aucun point, aucune cagnotte (FR-021).
- Intention de plus de 30 minutes → `etat = 'expire'`, `409`.
- Règle `cadeau_recu` désactivée → transaction et répartition journalisées, `points_credites = 0` (FR-020 scénario 8).
- Bénéficiaire supprimé entre l'envoi et la confirmation → `409`, transaction passée en `echoue`.

**Erreurs** : `401`, `403` (transaction d'un autre membre), `404` (référence inconnue), `409` (état incompatible ou expiré).

---

## 4. `GET /api/engagement/cadeaux/{type_objet}/{objet_id}`, cadeaux reçus par un contenu

**Auth** : publique.

```json
{
  "succes": true,
  "donnees": {
    "total": 7,
    "resume": [ { "code": "fleur", "libelle": "Fleur", "icone": "seedling", "nombre": 4 } ],
    "derniers": [
      { "offreur": { "id": "uuid", "nom_affiche": "Kofi A." },
        "cadeau": { "code": "drapeau_ua", "libelle": "Drapeau de l'Union Africaine", "icone": "flag" },
        "message": "Merci !", "created_at": "2026-08-08T12:00:00Z" }
    ]
  }
}
```

**Aucun montant en argent n'est exposé** (FR-027). Seules les transactions `etat = 'abouti'` sont comptées. `derniers` est borné à 10.

---

## 5. `GET /api/engagement/mes-cadeaux` : mes cadeaux reçus et offerts

**Auth** : JWT membre requis. **Pagination** : `?page=&taille=&sens=recus|offerts`.

Chaque ligne : cadeau, contrepartie (offreur ou bénéficiaire), contenu concerné (famille, identifiant, titre résolu), points, date, `simule`. Le **montant** n'est exposé que sur le sens `offerts` (l'offreur a le droit de savoir ce qu'il a dépensé) et sur la cagnotte du bénéficiaire ; il n'est jamais exposé sur les cadeaux reçus ligne à ligne.

---

## 6. `GET /api/engagement/ma-cagnotte` : solde de soutien

**Auth** : JWT membre requis.

```json
{
  "succes": true,
  "donnees": {
    "montant_cumule": 12600,
    "montant_verse": 0,
    "devise": "XOF",
    "versement_disponible": false,
    "part_simulee": 12600
  }
}
```

`versement_disponible` est toujours `false` dans cette itération (FR-026). `part_simulee` indique la fraction issue de transactions `simule = TRUE`, c'est-à-dire ce que la purge retirera, l'information doit être visible du membre pour que la purge ne soit pas une surprise.

---

## 7. `POST /api/engagement/partages-externes`, traçage (route existante, contrat modifié)

Le corps est inchangé (`{ type_objet, objet_id, reseau }`). **La réponse change** :

```diff
- { "reseaux_distincts": 4, "seuil": 5, "bonus_attribue": false }
+ { "enregistre": true, "auteur_credite": true }
```

`auteur_credite` vaut `true` lorsque c'est le **premier** partage de ce contenu par ce membre, tous canaux confondus (research R5). Le champ est informatif : le partage n'échoue jamais si le crédit échoue (FR-034).

---

## Notifications émises

| Événement | Type | Message |
|-----------|------|---------|
| Cadeau reçu | `engagement.cadeau_recu` *(nouveau)* | « {Offreur} vous a offert {cadeau} (+{points} points). » → `/mon-compte/engagement` |
| Changement de statut | `engagement.niveau_atteint` *(existant)* | inchangé |
| Badge débloqué | `engagement.badge_debloque` *(existant)* | inchangé |

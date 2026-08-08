# Contrat d'API — Cadeaux virtuels (back-office)

**Base** : `/api/admin/engagement` · **Permission requise** : `engagement.gerer` (existante, migration `35`) · **Audit** : `audit::log_action` sur toute mutation (Principe VII).

---

## 1. `GET /cadeaux` — catalogue complet

Renvoie **tous** les cadeaux, actifs et inactifs, avec pour chacun le nombre de transactions abouties (`nombre_envois`) et le montant total collecté. Ce décompte conditionne l'affichage du bouton de suppression côté interface.

---

## 2. `POST /cadeaux` — créer

```json
{ "code": "tam_tam", "libelle": "Tam-tam", "description": null,
  "icone": "drum", "couleur": "chocolat", "prix": 5000, "points": 50, "ordre": 0, "actif": true }
```

**Réponse `201`** : le cadeau créé. Immédiatement visible par les membres si `actif` (FR-016, SC-001).

**Erreurs** : `400` (`prix <= 0`, `points <= 0`), `409` (`code` déjà pris), `403` (permission manquante).

---

## 3. `PUT /cadeaux/{id}` — modifier

Tous les champs de la création sont modifiables, sauf `code` (clé stable). Une modification de `prix` ou de `points` **n'affecte aucune transaction passée** : elles portent leurs propres valeurs figées (FR-024, scénario 3 de l'US5).

---

## 4. `DELETE /cadeaux/{id}` — désactiver ou supprimer

- Aucun envoi abouti → suppression réelle, `204`.
- Au moins un envoi → **`409`** avec un message explicite ; seule la désactivation (`PUT … actif = false`) est possible (FR-028). La contrainte `ON DELETE RESTRICT` de `transaction_cadeau.cadeau_id` rend l'erreur structurelle : même une requête mal écrite ne peut pas casser l'historique.

---

## 5. `GET /transactions` — journal comptable

**Filtres** : `?membre_id=&sens=offreur|beneficiaire&etat=&mode=&simule=&debut=&fin=&page=&taille=`

```json
{
  "succes": true,
  "donnees": {
    "elements": [
      { "id": "uuid", "created_at": "2026-08-08T12:00:00Z", "finalise_at": "2026-08-08T12:00:14Z",
        "offreur": { "id": "uuid", "nom_affiche": "Kofi A." },
        "beneficiaire": { "id": "uuid", "nom_affiche": "Awa D." },
        "cible": { "type_objet": "video", "objet_id": "uuid", "titre": "Sur les traces du Djoliba" },
        "cadeau": { "code": "drapeau_ua", "libelle": "Drapeau de l'Union Africaine" },
        "mode": "soutien_financier", "montant": 2000, "points": 20, "taux_commission": 10,
        "part_beneficiaire": 1800, "part_plateforme": 200,
        "etat": "abouti", "simule": true, "reference_paiement": "SIM-…" }
    ],
    "pagination": { "page": 1, "taille": 25, "total": 312 },
    "totaux": {
      "montant_total": 624000, "recettes_plateforme": 62400,
      "cagnottes_dues": 561600, "nombre_abouti": 312, "nombre_simule": 312
    }
  }
}
```

Les `totaux` sont calculés **sur le filtre courant**, pas sur la page, et ne comptent que `etat = 'abouti'`. Invariant vérifiable en recette : `recettes_plateforme + cagnottes_dues = montant_total` (SC-009).

`cible.titre` est résolu par famille, comme le fait déjà `admin/media_proposition` pour la file de modération ; il vaut `null` pour un cadeau offert depuis un profil.

---

## 6. `GET` / `PUT /parametres-monetisation`

```json
{ "taux_commission": 10, "devise": "XOF", "paiement_reel_actif": false }
```

- `taux_commission` : `0..=100`. La modification est **prospective** : les transactions passées conservent leur taux figé (FR-024).
- `paiement_reel_actif` : passe à `true` le jour du branchement CinetPay. Bascule l'affichage du bandeau « phase de test » côté membre et conditionne l'accès à la purge.
- Toute modification est auditée avec l'état avant/après.

---

## 7. `POST /purger-phase-test` — purge de fin de phase de test

**Corps** : `{ "confirmation": "PURGER" }` — garde-fou explicite contre le déclenchement accidentel.

**Préconditions** : `paiement_reel_actif = true`. Sinon `409` : purger avant le basculement rouvrirait immédiatement la porte au minage.

**Effets** (une seule transaction, research R11) :

1. Suppression des `mouvement_points` de clé `cadeau:{id}` pour toute transaction `simule = TRUE AND etat = 'abouti'`.
2. Recalcul de `compte.solde_points` et `solde_points_mensuel` **depuis le journal restant**, puis de `niveau_code`.
3. Réduction des cagnottes du montant des `part_beneficiaire` purgées.
4. Passage de ces transactions à `etat = 'purge'` (jamais de suppression).
5. Entrée d'audit avec les décomptes.

**Réponse `200`**

```json
{
  "succes": true,
  "donnees": {
    "transactions_purgees": 312,
    "mouvements_supprimes": 312,
    "comptes_recalcules": 87,
    "montant_cagnottes_annule": 561600
  }
}
```

**Idempotence** : un second appel ne trouve plus de transaction `simule AND abouti` et renvoie des décomptes à zéro, sans erreur.

**Garantie exigée par SC-013** : aucun mouvement de `type_action` `jaime_recu` ou `partage_recu` ne doit être supprimé. La suppression est ciblée par le **motif de clé**, pas par une plage de dates.

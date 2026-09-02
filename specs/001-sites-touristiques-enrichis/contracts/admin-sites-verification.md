# Contrat : Vérification d'un site (badge « Vérifié »)

Réservé à l'administration (FR-010/FR-012). Enveloppe `{ success, data, error }`.

## PATCH /api/admin/profils-pays/{id}/sites-touristiques/{site_id}/verification

Active ou retire le badge « Vérifié » d'un site emblématique ou privé.

**Permission** : `verifier_permission!(admin, "profil_pays", "modifier")`.

**Body** :
```jsonc
{ "verifie": true }
```

**Effet** :
- `UPDATE country_profile.site_touristique SET verifie = $verifie,
   verifie_par = $admin_id, verifie_at = (verifie ? NOW() : NULL) WHERE id = $site_id AND deleted_at IS NULL`.
- `audit::log_action` (action `update`, table `site_touristique`, avant/après `verifie`).

**200** : `{ "id": "uuid", "verifie": true, "verifie_at": "2026-05-25T…Z" }`

**Erreurs** : 401/403 (non admin), 404 (site inexistant ou supprimé).

---

## Effet côté lecture publique

`GET /api/fiches-pays/{id}/sites-touristiques` renvoie `verifie` (booléen) pour chaque site →
le frontend affiche le badge « Vérifié » sur la carte/fiche (FR-011). Les visiteurs ne disposent
d'aucun endpoint pour modifier `verifie` (FR-012).

---

## Note d'intégration admin

Les sites peuvent aussi être créés/modifiés directement par l'admin via les routes existantes
(`POST/PUT /api/admin/profils-pays/{id}/sites-touristiques[/{site_id}]`). Ces DTO
(`CreerSiteTouristiqueRequest`, `ModifierSiteTouristiqueRequest`) sont étendus avec les nouveaux
champs (sous_type, gestionnaire, ville, village, info_pertinente, contacts, constitution_*) pour
permettre l'édition admin complète. Le champ `verifie` reste géré uniquement par l'endpoint de
vérification ci-dessus.

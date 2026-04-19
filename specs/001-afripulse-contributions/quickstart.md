# Quickstart — Afripulse Enrichissement collaboratif

**Feature**: Afripulse — Enrichissement collaboratif des fiches pays
**Branch**: `001-afripulse-contributions`
**Date**: 2026-04-18

Ce document fournit un scénario reproductible pour valider manuellement les 5 User Stories et les règles critiques (rate-limit, périmètre ISO, unicité recommandation, retrait post-approbation). Il tient lieu de **test d'acceptation** tant qu'aucune suite de tests automatisée n'est configurée.

Pré-requis :
- Backend lancé : `kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run`
- Frontend lancé : `pnpm dev` (port 3000)
- PostgreSQL disponible via `docker compose up -d` ; schéma `11c_country_profile_afripulse.sql` appliqué.
- Comptes de test existants (cf. `CLAUDE.md`) :
  - `admin@test.com` / `Test1234` (rôle admin)
  - `user2@test.com` / `Test1234` (rôle utilisateur standard)

Convention dans ce document :
- `$TOKEN_USER` : JWT de `user2@test.com`
- `$TOKEN_ADMIN` : JWT de `admin@test.com`
- `$FICHE_ID` : UUID d'une fiche pays existante (ex. Côte d'Ivoire)
- `$CONTRIB_ID` : UUID d'une contribution retournée par `POST /contributions`

---

## Préparation

```bash
# Authentifier un utilisateur standard
export TOKEN_USER=$(
  curl -s -X POST http://localhost:8080/api/auth/connexion \
    -H "Content-Type: application/json" \
    -d '{"email":"user2@test.com","mot_de_passe":"Test1234"}' \
    | jq -r '.access_token'
)

# Authentifier l'admin
export TOKEN_ADMIN=$(
  curl -s -X POST http://localhost:8080/api/auth/connexion \
    -H "Content-Type: application/json" \
    -d '{"email":"admin@test.com","mot_de_passe":"Test1234"}' \
    | jq -r '.access_token'
)

# Récupérer l'ID d'une fiche pays existante (Côte d'Ivoire)
export FICHE_ID=$(
  curl -s "http://localhost:8080/api/fiches-pays?recherche=Ivoire" \
    | jq -r '.fiches[0].id'
)

echo "FICHE_ID=$FICHE_ID"
```

---

## US1 — Proposer une modification sur une fiche pays existante (P1)

### 1a. Ajout d'un site touristique emblématique (texte seul)

```bash
curl -i -X POST "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributions" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -H "Content-Type: application/json" \
  -d '{
    "section": "sites_emblematiques",
    "type_objet": "site_touristique",
    "type_contribution": "ajout",
    "target_id": null,
    "nouvelle_valeur": {
      "nom": "Basilique Notre-Dame de la Paix",
      "description": "Plus grand édifice chrétien au monde, inspiré de la basilique Saint-Pierre.",
      "longitude": -5.2767,
      "latitude": 6.8395,
      "categorie": "emblematique",
      "region_id": null
    },
    "justification": "Site UNESCO absent de la fiche."
  }'
```

**Attendu** : HTTP 202, body `{ "id": "…", "etat": "en_attente", "created_at": "…" }`.

### 1b. Non-authentifié refusé

```bash
curl -i -X POST "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributions" \
  -H "Content-Type: application/json" \
  -d '{"section":"sites_emblematiques","type_objet":"site_touristique","type_contribution":"ajout","nouvelle_valeur":{}}'
```

**Attendu** : HTTP 401.

### 1c. La contribution n'apparaît pas sur la fiche publique

```bash
curl -s "http://localhost:8080/api/fiches-pays/$FICHE_ID/sites-touristiques?categorie=emblematique" \
  | jq '.[] | select(.nom == "Basilique Notre-Dame de la Paix")'
```

**Attendu** : sortie vide (donnée non visible tant que non approuvée).

---

## US2 — Modérer et valider les contributions (P1)

### 2a. Lister les contributions en attente

```bash
curl -s "http://localhost:8080/api/admin/profils-pays/contributions?etat=en_attente" \
  -H "Authorization: Bearer $TOKEN_ADMIN" | jq '.items[0]'
export CONTRIB_ID=$(
  curl -s "http://localhost:8080/api/admin/profils-pays/contributions?etat=en_attente" \
    -H "Authorization: Bearer $TOKEN_ADMIN" | jq -r '.items[0].id'
)
```

### 2b. Obtenir le détail avec diff structuré

```bash
curl -s "http://localhost:8080/api/admin/profils-pays/contributions/$CONTRIB_ID" \
  -H "Authorization: Bearer $TOKEN_ADMIN" | jq
```

**Attendu** : objet contenant `ancienne_valeur: null`, `nouvelle_valeur: {…}`, `pieces_jointes: []`, `contributions_concurrentes: []`.

### 2c. Approuver la contribution

```bash
curl -i -X PATCH "http://localhost:8080/api/admin/profils-pays/contributions/$CONTRIB_ID/etat" \
  -H "Authorization: Bearer $TOKEN_ADMIN" \
  -H "Content-Type: application/json" \
  -d '{"etat":"approuvee","note_moderation":"Source UNESCO vérifiée."}'
```

**Attendu** : HTTP 200, `etat = approuvee`.

### 2d. La donnée apparaît maintenant publiquement

```bash
curl -s "http://localhost:8080/api/fiches-pays/$FICHE_ID/sites-touristiques?categorie=emblematique" \
  | jq '.[] | select(.nom == "Basilique Notre-Dame de la Paix")'
```

**Attendu** : objet retourné avec les valeurs soumises.

### 2e. L'utilisateur apparaît dans les contributeurs

```bash
curl -s "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributeurs" | jq
```

**Attendu** : `user2@test.com` présent avec `nombre_contributions >= 1`.

### 2f. Refus avec motif obligatoire

```bash
# Soumettre une deuxième contribution
curl -s -X POST "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributions" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -H "Content-Type: application/json" \
  -d '{"section":"personnalites","type_objet":"personnalite_connue","type_contribution":"ajout",
       "nouvelle_valeur":{"nom_complet":"Test X","domaine":"autre","biographie_courte":"lorem ipsum"}}'

# Refuser sans motif → erreur
curl -i -X PATCH "http://localhost:8080/api/admin/profils-pays/contributions/<NEW_ID>/etat" \
  -H "Authorization: Bearer $TOKEN_ADMIN" \
  -H "Content-Type: application/json" \
  -d '{"etat":"rejetee"}'
```

**Attendu** : HTTP 400, message « `note_moderation` obligatoire pour un refus ».

---

## US3 — Publier une nouvelle fiche pays (P2)

### 3a. Création d'une fiche pour un pays non encore fiché (ex. Gambie `gm`)

```bash
curl -i -X POST "http://localhost:8080/api/fiches-pays" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -H "Content-Type: application/json" \
  -d '{
    "code_iso2": "gm",
    "nom": "Gambie",
    "capitale": "Banjul",
    "region": "Afrique de l'\''Ouest",
    "population": "2 500 000",
    "superficie_km2": 11295,
    "monnaie": "Dalasi gambien",
    "langues_populaires": "anglais, wolof, mandinka",
    "slogan": "The Smiling Coast of Africa"
  }'
```

**Attendu** : HTTP 202, contribution `fiche_pays` + `ajout` créée.

### 3b. Refus si code ISO hors périmètre (ex. `fr`)

```bash
curl -i -X POST "http://localhost:8080/api/fiches-pays" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -H "Content-Type: application/json" \
  -d '{"code_iso2":"fr","nom":"France","capitale":"Paris"}'
```

**Attendu** : HTTP 422, message « Code ISO hors périmètre africain ».

### 3c. Refus si fiche déjà existante (ex. `ci`)

```bash
curl -i -X POST "http://localhost:8080/api/fiches-pays" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -H "Content-Type: application/json" \
  -d '{"code_iso2":"ci","nom":"Côte d'\''Ivoire","capitale":"Yamoussoukro"}'
```

**Attendu** : HTTP 409, body contient `fiche_pays_id` existant + suggestion.

---

## US4 — Partager photos légendées + recommandations (P2)

### 4a. Upload de photos légendées (multipart)

```bash
# Préparer 2 fichiers de test (JPEG < 2 Mo, < 2048×2048)
curl -i -X POST "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributions" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -F "section=galerie_photos" \
  -F "type_objet=photo_visiteur" \
  -F "type_contribution=ajout" \
  -F "photos[]=@./test-assets/photo1.jpg" \
  -F "photos[]=@./test-assets/photo2.jpg" \
  -F "legendes[]=Coucher de soleil sur la lagune Ébrié" \
  -F "legendes[]=Marché de Cocody, matin"
```

**Attendu** : HTTP 202 + contribution avec `pieces_jointes` array de 2.

### 4b. Refus d'une photo > 2 Mo

```bash
curl -i -X POST "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributions" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -F "section=galerie_photos" -F "type_objet=photo_visiteur" -F "type_contribution=ajout" \
  -F "photos[]=@./test-assets/photo-too-big.jpg" \
  -F "legendes[]=Trop lourd"
```

**Attendu** : HTTP 413, message indiquant la limite 2 Mo dépassée.

### 4c. Soumission d'une recommandation (note + commentaire)

```bash
curl -i -X POST "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributions" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -H "Content-Type: application/json" \
  -d '{
    "section": "recommandations",
    "type_objet": "recommandation_visiteur",
    "type_contribution": "ajout",
    "nouvelle_valeur": {
      "note": 5,
      "commentaire": "Accueil chaleureux, gastronomie exceptionnelle, paysages variés. Un voyage qui marque durablement."
    }
  }'
```

**Attendu** : HTTP 202.

### 4d. Refus si commentaire < 50 caractères

```bash
curl -i -X POST "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributions" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -H "Content-Type: application/json" \
  -d '{"section":"recommandations","type_objet":"recommandation_visiteur","type_contribution":"ajout",
       "nouvelle_valeur":{"note":4,"commentaire":"Sympa."}}'
```

**Attendu** : HTTP 400, message « commentaire doit faire 50..2000 caractères ».

### 4e. Unicité recommandation — deuxième soumission = édition

Après approbation admin de la recommandation précédente (étape 4c), l'utilisateur soumet une nouvelle note :

```bash
curl -i -X POST "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributions" \
  -H "Authorization: Bearer $TOKEN_USER" \
  -H "Content-Type: application/json" \
  -d '{"section":"recommandations","type_objet":"recommandation_visiteur","type_contribution":"edition",
       "target_id":"<ID_DE_LA_RECO_EXISTANTE>",
       "nouvelle_valeur":{"note":4,"commentaire":"Après un second voyage, note ajustée. La pluie d'\''octobre complique les déplacements mais l'\''accueil reste au top."}}'
```

**Attendu** : HTTP 202, contribution `edition`. Après approbation : l'ancienne recommandation passe en `active = FALSE` et la nouvelle devient active.

---

## US5 — Reconnaissance publique des contributeurs validés (P3)

Après avoir validé plusieurs contributions de `user2@test.com` sur `$FICHE_ID`, vérifier :

```bash
curl -s "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributeurs" | jq
```

**Attendu** : `user2@test.com` listé avec `nombre_contributions` reflétant le total approuvé, `photo_url` si défini, date de dernière contribution validée.

Test d'anonymisation :
```sql
-- Dans psql (Adminer localhost:8088)
UPDATE iam.utilisateur SET deleted_at = NOW() WHERE email = 'user2@test.com';
```
Puis recharger la fiche côté UI : le contributeur doit s'afficher comme « Contributeur retiré ».

---

## Tests de règles critiques (non couverts par les US)

### R1 — Rate-limit « 20 textes / 24 h »

Boucle bash simulant 21 soumissions textuelles :

```bash
for i in $(seq 1 21); do
  status=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
    "http://localhost:8080/api/fiches-pays/$FICHE_ID/contributions" \
    -H "Authorization: Bearer $TOKEN_USER" \
    -H "Content-Type: application/json" \
    -d '{"section":"secteurs_opportunites","type_objet":"secteur_developpement","type_contribution":"ajout",
         "nouvelle_valeur":{"nom":"Secteur test '"$i"'","description":"test"}}')
  echo "$i → HTTP $status"
done
```

**Attendu** : les 20 premières → 202 ; la 21ᵉ → 429 avec body `seuil_depasse: "20_contributions_textuelles_24h"`.

### R2 — Rate-limit « 5 en attente par pays »

Après 5 contributions `en_attente` sur `$FICHE_ID`, la 6ᵉ soumission retourne 429 avec `seuil_depasse: "5_contributions_en_attente_par_pays"`. Dès qu'une des 5 est approuvée/refusée, une nouvelle soumission redevient possible.

### R3 — Cohérence des 54 codes ISO

```bash
# Nombre d'entrées dans les deux sources doit être strictement égal à 54
jq '.length' uafricas_frontend/app/constants/afripulsePaysAutorises.ts || echo "grep manuel"
grep -oE '"[a-z]{2}"' uafricas_backend/src/constants/afripulse_pays_autorises.rs | sort -u | wc -l
```

**Attendu** : `54` pour les deux sorties, et `diff` des deux listes (triées) doit être vide.

### R4 — Retrait d'une contribution approuvée (FR-028)

```bash
# Après approbation d'une contribution
curl -i -X POST "http://localhost:8080/api/admin/profils-pays/contributions/$APPROUVEE_ID/retirer" \
  -H "Authorization: Bearer $TOKEN_ADMIN" \
  -H "Content-Type: application/json" \
  -d '{"motif":"Contenu ultérieurement jugé inapproprié — propos discriminatoires signalés."}'
```

**Attendu** : HTTP 200. La ligne correspondante dans la table cible est soft-deletée (`deleted_at IS NOT NULL`), mais les autres contributions validées du même auteur sur la même fiche restent créditées.

### R5 — Audit trail

```sql
-- Dans psql
SELECT action, table_name, user_id, created_at
FROM shared.journal_audit
WHERE schema_name = 'country_profile'
ORDER BY created_at DESC
LIMIT 20;
```

**Attendu** : chaque mutation ci-dessus est représentée par une ligne (`create`, `update`, `delete`) avec `before`/`after` JSONB renseignés.

---

## Parcours UI end-to-end (≤ 3 min — SC-004)

1. Ouvrir `http://localhost:3000/opportunite-afrique` en utilisateur connecté.
2. Cliquer sur la Côte d'Ivoire → `/opportunite-afrique/<id>`.
3. Scroll vers la section « Personnalités connues » → cliquer « Proposer un ajout ».
4. Remplir le formulaire (nom, domaine, biographie ≥ 100 car.) → attacher un portrait JPEG.
5. Soumettre → un toast confirme la soumission en attente.
6. Sur le même écran, ouvrir la section « Recommandations » → noter 5⭐ + commentaire ≥ 50 car. → soumettre.
7. Se déconnecter, se reconnecter en admin.
8. Aller sur `/admin/profils-pays/contributions?etat=en_attente` → voir les 2 soumissions, approuver les deux en < 2 min (SC-006).
9. Se déconnecter, revenir sur `/opportunite-afrique/<id>` en anonyme → les 2 nouvelles données sont publiques, l'utilisateur apparaît comme contributeur.

---

## Checklist de sortie

- [ ] Backend démarre sans erreur après `cargo run`.
- [ ] Migration `11c_country_profile_afripulse.sql` appliquée (3 nouveaux enums, 4 nouvelles tables, ALTER sur `site_touristique` et `contribution_fiche`).
- [ ] Les 5 User Stories passent leurs scénarios `curl` + UI.
- [ ] Les 5 règles critiques R1..R5 passent.
- [ ] `shared.journal_audit` contient les entrées attendues pour chaque mutation.
- [ ] Aucune contribution non validée n'est visible via un endpoint public (SC-003).
- [ ] Les composants publics Vue n'utilisent aucune classe daisyUI (recherche manuelle dans `app/components/opportunite-afrique/` — aucune occurrence de `btn`, `card`, `modal`, `alert` en classe daisyUI).

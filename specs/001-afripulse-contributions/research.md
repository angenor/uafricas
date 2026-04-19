# Research — Afripulse Enrichissement collaboratif

**Feature**: Afripulse — Enrichissement collaboratif des fiches pays
**Branch**: `001-afripulse-contributions`
**Date**: 2026-04-18

Ce document consolide les décisions techniques prises en phase 0, avant le design détaillé (Phase 1). Pour chaque décision sont listés : le choix retenu, la justification et les alternatives écartées avec leurs raisons.

---

## D1. Réutiliser `country_profile.contribution_fiche` plutôt que créer un pipeline parallèle

- **Décision** : réutiliser la table `country_profile.contribution_fiche` existante ainsi que les handlers publics (`POST /api/fiches-pays/{id}/contributions`, `PUT .../contributions/{id}/valider|rejeter`) et admin (`GET /api/admin/profils-pays/contributions*`, `PATCH .../etat`) déjà en production comme pipeline unique pour toutes les contributions Afripulse (texte, photo, recommandation, création de fiche, ajout/édition/suppression d'élément de section).
- **Rationale** :
  - Principe V (Simplicité) et Principe VII (Audit) : le pipeline existant est instrumenté pour `audit::log_action`, possède un frontend admin opérationnel (`/admin/profils-pays/contributions.vue`) et un composable `useAdminContributions.ts` — toute duplication violerait YAGNI.
  - Cohérence UX : un seul bouton « Proposer une modification » côté visiteur, une seule file d'attente côté admin.
  - Transactionnalité : l'approbation d'une contribution peut appliquer son effet (insert/update/delete sur la table cible) dans la même transaction que la mise à jour de `etat = 'approuvee'`, garantissant l'absence d'états intermédiaires.
- **Alternatives écartées** :
  - Nouveau pipeline `contribution_afripulse` dédié : rejeté — duplication du code de modération et risque d'incohérence d'audit.
  - Un pipeline par type d'objet (4 tables) : rejeté — multiplication des handlers admin, complexification de la queue de modération globale.

## D2. Étendre les enums `type_contribution` et ajouter `type_objet_contribution` + `section_contribution`

- **Décision** : conserver l'enum existant `country_profile.type_contribution` (valeurs `modification`, `ajout`, `suppression`) mais ajouter deux nouveaux enums :
  - `country_profile.type_objet_contribution` : `fiche_pays`, `site_touristique`, `secteur_developpement`, `personnalite_connue`, `savoir_pratique`, `recommandation_visiteur`, `photo_visiteur`. Identifie QUOI est modifié.
  - `country_profile.section_afripulse` : `sites_emblematiques`, `sites_prives`, `secteurs_opportunites`, `personnalites`, `savoir_avant_voyager`, `recommandations`, `galerie_photos`. Identifie la section UI de rattachement.
  - Ajouter la valeur `obsolete` à `country_profile.etat_contribution` (pour les contributions concurrentes automatiquement marquées après approbation d'une proposition sur le même champ — cf. FR-021).
- **Rationale** :
  - L'enum `type_contribution` actuel (`modification`/`ajout`/`suppression`) décrit déjà parfaitement l'action CRUD, aligné sur la décision Q1 du `/speckit.clarify` (B — ajout+édition+suppression).
  - Séparer « action » (type_contribution) et « objet ciblé » (type_objet_contribution) évite la multiplication combinatoire et facilite la logique d'application (match sur type_objet + type_contribution).
  - La valeur `obsolete` permet d'archiver une contribution surplantée sans perdre l'info pour l'audit (principe VII).
- **Alternatives écartées** :
  - Utiliser `section VARCHAR(100)` comme aujourd'hui (texte libre) : rejeté — non auto-documentant, aucune contrainte à l'insertion, rend les filtres admin fragiles.
  - Fusionner section + objet en un seul enum : rejeté — l'objet `photo_visiteur` peut appartenir à la section `galerie_photos` mais aussi à une section `sites_emblematiques` (illustration d'un site), la séparation est nécessaire.
  - Nouvel enum `type_contribution_v2` : rejeté — inutile, l'actuel est cohérent.

## D3. Application transactionnelle des contributions approuvées

- **Décision** : le handler admin de modération (`PATCH /api/admin/profils-pays/contributions/{id}/etat`) exécute une transaction SQL unique qui :
  1. met à jour `contribution_fiche` : `etat = 'approuvee'`, `traite_par`, `traite_at`, `note_moderation` ;
  2. selon `type_objet_contribution` + `type_contribution` + `target_id` + `nouvelle_valeur` (JSONB), applique l'effet sur la table cible :
     - `ajout` → `INSERT INTO <table_cible>` avec les valeurs de `nouvelle_valeur` ;
     - `edition` → `UPDATE <table_cible> SET … WHERE id = target_id` ;
     - `suppression` → `UPDATE <table_cible> SET deleted_at = NOW() WHERE id = target_id` (soft delete) ;
  3. marque toutes les autres contributions `en_attente` ciblant le même `(fiche_pays_id, type_objet, target_id)` en `etat = 'obsolete'` (FR-021) ;
  4. appelle `audit::log_action` avec l'état avant/après.
- **Rationale** :
  - Atomicité : une approbation ne peut jamais laisser la DB dans un état partiel (ex. contribution marquée approuvée mais table cible non modifiée).
  - Pas d'event bus ni de workers asynchrones : principe V — tout en SQL synchrone.
  - Le `target_id` n'est peuplé que pour `edition`/`suppression` ; pour `ajout`, il reste NULL et le INSERT crée la nouvelle ligne.
  - La valeur `nouvelle_valeur` est stockée en JSONB côté `contribution_fiche` pour supporter les formes variables (simple string pour un champ texte, objet complet pour un site touristique).
- **Alternatives écartées** :
  - Application différée via une table d'outbox : rejeté — YAGNI, complexité injustifiée.
  - Application côté composable frontend (2 requêtes séquentielles) : rejeté — impossible à sécuriser, violation Principe IV.
  - Trigger PostgreSQL sur `UPDATE contribution_fiche.etat` : rejeté — déplace la logique métier dans la DB, difficile à tester et à faire évoluer.

## D4. Validation photo côté serveur via crate `image` 0.25

- **Décision** : dépendance `image = "0.25"` (sans features lourdes ; on active uniquement `jpeg` et `png`) pour décoder les pièces jointes EN MÉMOIRE avant écriture disque. Le service `src/services/image_validation.rs` expose une fonction `valider_photo_contribution(bytes) -> Result<ImageDimensions, ErreurValidation>` qui vérifie :
  - magic bytes (rejet si autre que JPEG ou PNG) — se fie à `image::guess_format` ;
  - taille ≤ 2 Mo (check sur longueur du buffer avant décodage) ;
  - dimensions ≤ 2048×2048 (via `image::load_from_memory` et méthode `dimensions()`).
- **Rationale** :
  - Principe IV (Sécurité) : se fier à l'extension du nom de fichier ou au `Content-Type` déclaré par le client est trivialement contournable.
  - `image` 0.25 est déjà utilisée par de nombreux projets Actix ; son décodage partiel suffit à lire les dimensions d'une JPEG sans charger tous les pixels.
  - Refus AVANT `sanitize-filename` et avant écriture disque garantit qu'aucun fichier malveillant ne touche `./uploads/`.
- **Alternatives écartées** :
  - `imagesize` crate : plus léger mais pas de sécurité sur la validité structurelle du fichier (accepte certains containers corrompus).
  - Shell out à `identify` (ImageMagick) : rejeté — dépendance externe non présente sur le VPS, attack surface élargie.
  - Validation uniquement côté client : rejeté — contournable par appel direct à l'API.

## D5. Rate-limiting en temps réel par requêtes SQL (pas de table compteur)

- **Décision** : le service `src/services/rate_limit_afripulse.rs` expose `verifier_quotas(pool, user_id, type_objet, fiche_pays_id) -> Result<(), LimiteAtteinte>` qui exécute 3 requêtes COUNT paramétrées sur `contribution_fiche` :
  1. `COUNT(*) WHERE cree_par = $1 AND type_objet_contribution <> 'photo_visiteur' AND created_at > NOW() - INTERVAL '24 hours' AND deleted_at IS NULL` → limite 20 (textuelles).
  2. `COUNT(*) WHERE cree_par = $1 AND type_objet_contribution = 'photo_visiteur' AND created_at > NOW() - INTERVAL '24 hours' AND deleted_at IS NULL` → limite 10 (photos).
  3. `COUNT(*) WHERE cree_par = $1 AND fiche_pays_id = $2 AND etat = 'en_attente' AND deleted_at IS NULL` → limite 5 (en attente par pays).
- **Rationale** :
  - Volumétrie cible modeste (≤ 10 000 contributions/an → ~30 / jour globalement) : les COUNTs sur un index `(cree_par, created_at)` s'exécutent en quelques ms.
  - Pas de table `compteur_rate_limit` : principe V (YAGNI) — toute l'information nécessaire vit déjà dans `contribution_fiche`.
  - Cohérence : pas de divergence possible entre compteur et vérité SQL.
- **Alternatives écartées** :
  - Table compteur avec reset horaire : rejeté — nécessite un cron, et redondant.
  - Redis sliding window : rejeté — ajout d'une dépendance infrastructurelle pour 1000 utilisateurs.
  - Enforcement au niveau nginx : rejeté — perd l'isolation par utilisateur authentifié, et la règle « 5 en attente par pays » est métier, pas infrastructurelle.
- **Index SQL à ajouter** : `CREATE INDEX idx_contribution_rate_limit ON country_profile.contribution_fiche (cree_par, created_at) WHERE deleted_at IS NULL;` et `CREATE INDEX idx_contribution_attente_pays ON country_profile.contribution_fiche (cree_par, fiche_pays_id, etat) WHERE etat = 'en_attente' AND deleted_at IS NULL;`

## D6. Source unique des 54 codes ISO africains

- **Décision** :
  - Créer `uafricas_frontend/app/constants/afripulsePaysAutorises.ts` exportant `export const PAYS_AFRICAINS_ISO2: readonly string[] = ['dz', 'ao', ..., 'eh']` (54 entrées, triées alphabétiquement) + un helper `estPaysAfricain(code: string): boolean`.
  - Créer `uafricas_backend/src/constants/afripulse_pays_autorises.rs` exportant `pub const PAYS_AFRICAINS_ISO2: &[&str] = &["dz", "ao", ..., "eh"];` + fonction `pub fn est_pays_africain(code: &str) -> bool`.
  - Consommer côté frontend dans `NouvelleFichePaysModal.vue` (sélecteur filtré) et `index.vue` (carte SVG).
  - Consommer côté backend dans `afripulse_public.rs::creer_fiche_pays` (validation du champ `code_iso2`).
  - Refactor de `/opportunite-afrique/index.vue` pour importer depuis `app/constants/afripulsePaysAutorises.ts` au lieu du `Set` en dur.
- **Rationale** :
  - FR-027 impose une source unique partagée.
  - Éviter la divergence silencieuse : test de cohérence inclus dans `quickstart.md` (script bash qui compare les deux fichiers).
- **Alternatives écartées** :
  - Table SQL `shared.pays_autorises_afripulse` : rejeté — déjà couvert par `shared.pays` (colonne `code_iso2` + flag potentiel). Ajouter un flag `est_africain` à `shared.pays` est envisageable mais hors scope de ce plan ; on le laisse en mention dans `data-model.md` comme évolution future.
  - Endpoint `GET /api/meta/pays-africains` chargé au runtime frontend : rejeté — latence inutile pour une liste figée.

## D7. Stratégie multipart unique pour le POST de contribution

- **Décision** : l'endpoint `POST /api/fiches-pays/{id}/contributions` accepte deux `Content-Type` :
  - `application/json` : contribution texte seule (création élément de section texte, édition, suppression, recommandation) — payload JSON avec `section`, `type_contribution`, `type_objet`, `target_id?`, `nouvelle_valeur` (objet ou string), `justification?`.
  - `multipart/form-data` : contribution avec pièces jointes photo — champs texte identiques à JSON + fichiers `photos[]` (1 à 5) et `legendes[]` (même cardinalité, correspondance par index). La pièce jointe principale sert pour la section `galerie_photos` ; pour un site touristique, la première photo est utilisée comme `image_url` du site.
- **Rationale** :
  - Principe V (Simplicité) : un seul endpoint, une seule modération par soumission, un seul enregistrement dans `contribution_fiche`.
  - La distinction JSON vs multipart est triviale côté Actix (match sur `Content-Type`).
  - Limite 5 photos alignée sur FR-012 (contrainte serveur).
- **Alternatives écartées** :
  - Endpoint séparé pour photos (`POST /api/fiches-pays/{id}/photos-contribution`) : rejeté — double pipeline de modération et risque d'incohérence entre la photo et sa légende.
  - Upload séparé avec token pré-signé : rejeté — infrastructure cloud (S3) non présente, stockage local suffit.
- **Modélisation stockage des pièces jointes** : champ `pieces_jointes JSONB` sur `contribution_fiche`, de forme `[{"chemin_fichier": "uploads/opportunite-afrique/photos/<uuid>.jpg", "legende": "…", "taille_octets": 1234567, "largeur": 1600, "hauteur": 1200}, …]`. Au moment de l'approbation, le handler copie (ou promeut) ces entrées vers la table `photo_visiteur` si `type_objet = 'photo_visiteur'`.

---

## Récapitulatif des décisions

| Décision | Choix | Impact |
|---|---|---|
| D1 | Réutiliser `contribution_fiche` | Aucun nouveau pipeline |
| D2 | 2 nouveaux enums + valeur `obsolete` | +3 enums PostgreSQL |
| D3 | Application transactionnelle dans handler modération | +1 bloc logique dans `profils_pays.rs::moderer_contribution` |
| D4 | Validation photos via crate `image` | +1 dépendance Cargo, +1 service |
| D5 | Rate-limit SQL temps réel | +2 index, +1 service |
| D6 | Constantes jumelles TS + Rust | +2 fichiers constants |
| D7 | Endpoint unique JSON/multipart | Extension de l'endpoint existant |

Aucune `[NEEDS CLARIFICATION]` — toutes les décisions sont tranchées. Prêt pour la Phase 1.

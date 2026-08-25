# Research: 003-retrouve-amis-public

**Date**: 2026-03-15 | **Branch**: `003-retrouve-amis-public`

## R1 : Adaptation du schéma SQL existant

**Decision**: Ajouter de nouvelles colonnes à `retrouve_amis.avis_recherche` et modifier `est_public` pour être `TRUE` par défaut. Ne pas créer de nouvelle table.

**Rationale**: La table `avis_recherche` existe déjà avec 6 tables liées. Les nouveaux champs du formulaire (anonymat, genre, type de relation, lieux de rencontre, photo, coordonnées) s'ajoutent naturellement comme colonnes supplémentaires. Changer `est_public DEFAULT TRUE` aligne le schéma avec le modèle "tout public par défaut".

**Alternatives considered**:
- Créer une nouvelle table `avis_recherche_v2` : rejeté car duplication massive et rupture des FK existantes (correspondance, signalement, notification, reponse_publique, demande_retrait).
- Garder `est_public DEFAULT FALSE` et forcer à TRUE côté handler : rejeté car incohérence entre schéma et comportement.

## R2 : Nouveaux champs du formulaire

**Decision**: Ajouter les colonnes suivantes à `retrouve_amis.avis_recherche` :

| Colonne | Type | Nullable | Default | Notes |
|---------|------|----------|---------|-------|
| `est_anonyme` | BOOLEAN | NOT NULL | FALSE | Anonymat de l'auteur |
| `genre_recherche` | `genre_personne` (enum) | NULL |, | homme/femme |
| `type_relation` | `type_relation_recherche` (enum) | NULL |, | Nouveau enum |
| `comment_connu` | VARCHAR(500) | NULL |, | "Comment la personne vous connaît" |
| `localite_rencontre` | VARCHAR(200) | NULL |, | Lieu de rencontre : localité |
| `ecole_rencontre` | VARCHAR(250) | NULL |, | Lieu de rencontre : école |
| `ville_rencontre` | VARCHAR(200) | NULL |, | Lieu de rencontre : ville |
| `jamais_rencontre` | BOOLEAN | NOT NULL | FALSE | Jamais rencontré |
| `photo_url` | VARCHAR(500) | NULL |, | Chemin fichier photo uploadé |
| `description_physique` | TEXT | NULL |, | Description physique |
| `partage_coordonnees` | BOOLEAN | NOT NULL | FALSE | Souhaite partager ses coordonnées |
| `coordonnees_email` | VARCHAR(250) | NULL |, | Email de contact (non public) |
| `coordonnees_telephone` | VARCHAR(50) | NULL |, | Téléphone de contact (non public) |
| `coordonnees_whatsapp` | VARCHAR(50) | NULL |, | WhatsApp de contact (non public) |

**Nouveaux enums PostgreSQL** :
- `genre_personne` : `'homme'`, `'femme'`
- `type_relation_recherche` : `'amis_enfance'`, `'amis_ecole'`, `'collegue'`, `'connaissance'`, `'frere_soeur'`, `'parent'`

**Rationale**: Champs structurés alignés avec les 10 questions du formulaire client. Les lieux de rencontre sont 3 colonnes séparées (cumulables) + un booléen "jamais rencontré". Les coordonnées sont des champs séparés (email, tel, WhatsApp) pour permettre la validation.

**Alternatives considered**:
- JSONB pour les lieux de rencontre : rejeté car empêche les index et la recherche full-text.
- JSONB pour les coordonnées : rejeté car la spec 001 utilise déjà `coordonnees_a/b` en JSONB dans `correspondance`, garder la cohérence mais les champs source doivent être structurés pour validation.

## R3 : Modification de `est_public`

**Decision**: Changer `est_public DEFAULT FALSE` en `DEFAULT TRUE` et supprimer l'endpoint `publier_avis` (`PATCH /avis/{id}/publier`). La génération du slug se fait automatiquement à la création.

**Rationale**: Dans le modèle "tout public par défaut", il n'y a plus de notion de publication optionnelle. Tous les avis sont publics dès leur création.

**Alternatives considered**:
- Supprimer la colonne `est_public` : rejeté car elle reste utile pour les avis suspendus (masqués du listing public).

## R4 : Adaptation du formulaire frontend

**Decision**: Réécrire le composant `AvisRechercheForm.vue` avec 6 étapes au lieu de 5. Le composant existant est remplacé entièrement.

**Rationale**: Les étapes changent complètement :
- Ancien : Identité → Éducation → Localisation → Période → Récapitulatif
- Nouveau : Préférences → Identité → Relation → Lieu de rencontre → Photo/Description → Récapitulatif

Les champs `ecole`, `ville`, `pays_id`, `periode_debut`, `periode_fin` de l'ancien formulaire sont remplacés par les nouveaux champs (ecole_rencontre, ville_rencontre, localite_rencontre, etc.).

## R5 : Page `/retrouve-amis` (index)

**Decision**: Transformer la page index pour afficher les avis publics en premier plan (accessible sans connexion), avec le dashboard utilisateur en section secondaire pour les connectés.

**Rationale**: La page actuelle montre uniquement un CTA + dashboard. Le nouveau modèle exige que les avis soient visibles par tous. La page `rechercher.vue` existante contient déjà la logique de listing public, fusionner cette logique dans `index.vue`.

**Alternatives considered**:
- Garder `index.vue` et `rechercher.vue` séparées : rejeté car redondant avec le modèle "tout public". La page index DOIT montrer les avis.
- Rediriger `/retrouve-amis` vers `/retrouve-amis/rechercher` : rejeté car l'index doit rester le point d'entrée avec hero + listing.

## R6 : Upload de photo

**Decision**: Utiliser le mécanisme d'upload existant (`./uploads/`) avec un nouveau sous-dossier `./uploads/retrouve-amis/`. L'endpoint de création d'avis passe en multipart au lieu de JSON.

**Rationale**: Le backend utilise déjà actix-multipart pour les uploads (couvertures de livres, médias). Le même pattern s'applique.

## R7 : Mise à jour du search_vector (TSVECTOR)

**Decision**: Étendre le trigger/calcul du `search_vector` pour inclure les nouveaux champs : `comment_connu`, `localite_rencontre`, `ecole_rencontre`, `ville_rencontre`, `description_physique`.

**Rationale**: La recherche full-text (FR-007) doit couvrir tous les champs textuels pertinents.

## R8 : Sécurité des coordonnées

**Decision**: Les colonnes `coordonnees_email`, `coordonnees_telephone`, `coordonnees_whatsapp` ne sont JAMAIS incluses dans les réponses publiques (GET `/retrouve-amis/public/*`). Elles sont uniquement copiées dans `correspondance.coordonnees_a/b` lors de l'acceptation mutuelle.

**Rationale**: Conformité avec FR-016 et le principe de sécurité par défaut (Constitution IV).

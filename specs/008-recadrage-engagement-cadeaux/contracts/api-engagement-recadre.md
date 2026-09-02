# Contrat d'API : Impacts du recadrage sur l'engagement existant

Ce document recense ce qui **change** dans les 14 routes d'engagement déjà livrées, et les 11 points de branchement du crédit. Aucune route existante n'est supprimée.

---

## A. Routes membre existantes : contrats modifiés

| Route | Changement |
|-------|------------|
| `GET /api/engagement/mon-compte` | Ajoute `cagnotte: { montant_cumule, versement_disponible }` et `cadeaux_recus: <nombre>`. `niveau` renvoie désormais l'un des 4 statuts. |
| `GET /api/engagement/mes-categories` | Une catégorie `cadeaux` apparaît. Les catégories sans mouvement (`contributions`, `medias`, `verification`) restent renvoyées à 0, l'espace membre les masque plutôt que de les afficher vides. |
| `GET /api/engagement/mon-journal` | Aucun changement de forme. Les libellés viennent des règles, donc du recadrage : « J'aime reçu sur un contenu », « Contenu partagé par un membre », « Cadeau virtuel reçu ». |
| `GET /api/engagement/actions-recompensees` | Ne renvoie plus que les 3 règles actives (+ `ajustement_admin`). C'est cette route qui alimente l'état vide pédagogique (FR-015) : le recadrage s'y reflète **sans une ligne de code frontal**. |
| `GET /api/engagement/niveau/{utilisateur_id}` | Aucun changement de forme ; 4 valeurs possibles au lieu de 3. |
| `POST /api/engagement/partages-externes` | **Réponse modifiée** : voir `api-cadeaux-membre.md` §7. |

## B. Routes d'administration existantes : inchangées

`regles`, `categories`, `paliers`, `niveaux`, `badges`, `journal`, `ajustement`, `mise-en-avant` conservent leur contrat. Elles suffisent à piloter tout le recadrage du barème : c'est précisément ce que FR-006 exige.

Deux écrans changent d'usage sans changer d'API :
- **Paliers** : la liste est désormais entièrement inactive. L'écran affiche un encart expliquant que les paliers sont remplacés par le crédit unitaire.
- **Règles** : 8 lignes inactives, 3 actives. L'écran doit rendre l'état `actif` immédiatement lisible, ce qui n'était pas critique jusqu'ici.

---

## C. Points de branchement du crédit

### C1. J'aime : 7 branchements (FR-008)

Chaque handler de réaction appelle, **après** l'écriture de la réaction et **uniquement** pour une réaction positive :

```
services::engagement::crediter_jaime(pool, type_objet, objet_id, auteur_id, membre_qui_aime_id)
```

| Handler | Route | `type_objet` | Réaction créditante | Auteur | Statut |
|---------|-------|--------------|---------------------|--------|--------|
| `codimoi.rs` | `POST /api/codimoi/{id}/reaction` | `codimoi` | `like` | `cree_par` | Remplace `evaluer_popularite` |
| `gouvernance.rs` | `POST /api/gouvernance/factcheck/{id}/reaction` | `factcheck` | `coeur` (research R2) | `cree_par` | Remplace `evaluer_popularite` |
| `bibliotheques_humaines.rs` | `POST /api/bibliotheques-humaines/{id}/reaction` | `biblio_humaine` | `like` | titulaire de la fiche | Remplace `evaluer_popularite` |
| `media_social.rs` | `POST /api/medias/{type_media}/{media_id}/reaction` | `chaine_tv` / `station_radio` / `programme_tele` / `programme_radio` | `like` | **propriétaire** (R4) | Remplace `evaluer_popularite_media` ; changement de bénéficiaire |
| `vidafrica_contribution.rs` | `POST /api/vidafrica/videos/{id}/reaction` | `video` | `like` | auteur de la vidéo | **Nouveau** |
| `element_social.rs` | `POST /api/opportunite-afrique/elements/{type_objet}/{objet_id}/reaction` | **le sous-type reçu** : `personnalite_connue`, `recette_culinaire` | `like` | `cree_par` de l'élément | **Nouveau** |
| `element_social.rs` | idem | `site_touristique`, `secteur_developpement` | `like` | **aucun**, pas de colonne d'auteur (FR-008c) | Aucun crédit |
| `fiche_pays_social.rs` | `POST /api/opportunite-afrique/{id}/reaction` | `fiche_pays` | `like` | `country_profile.fiche_pays.cree_par` | **Nouveau** |

**Contrat de la fonction** : ne crédite pas si `auteur_id == membre_qui_aime_id` (FR-009) ; n'est jamais appelée sur un retrait de réaction ; ne fait rien lorsque `resoudre_beneficiaire` ne renvoie personne (FR-008c) ; ne propage aucune erreur (FR-034).

> **`type_objet` porte toujours la table réelle.** Pour les éléments Opportunité-Afrique, il n'existe pas de valeur `element` : `element_social` étant générique sur `country_profile.type_objet_contribution`, le sous-type reçu dans l'URL est transmis tel quel. Sans lui, aucune résolution d'auteur n'est possible (research R4).

### C2. Partage : 7 branchements, une seule clé (FR-012, research R5)

```
services::engagement::crediter_partage(pool, type_objet, objet_id, auteur_id, partageur_id)
```

| Handler | Route | Canal |
|---------|-------|-------|
| `media_social.rs` | `POST /api/medias/{type_media}/{media_id}/partages` | interne (mur) |
| `vidafrica_contribution.rs` | `POST /api/vidafrica/videos/{id}/partage` | interne |
| `element_social.rs` | `POST /api/opportunite-afrique/elements/{type_objet}/{objet_id}/partages` | interne |
| `fiche_pays_social.rs` | `POST /api/opportunite-afrique/{id}/partages` | interne |
| `profil_social.rs` | `POST /api/profils/{id}/partages` | interne |
| `gouvernance.rs` | `POST /api/gouvernance/partages` | interne |
| `engagement.rs` | `POST /api/engagement/partages-externes` | externe (6 réseaux) |

Le partage d'un **profil** crédite le membre dont le profil est partagé (`type_objet = 'profil'`, `objet_id = utilisateur_id`).

**Contrat** : ne crédite pas si `auteur_id == partageur_id` (FR-014) ; la clé étant commune, le 2ᵉ à 7ᵉ geste d'un même partageur sur un même contenu n'écrit rien.

### C3. Cadeau : 1 branchement (FR-019)

`handlers/engagement_cadeau.rs`, après le COMMIT de la confirmation :

```
services::engagement::crediter_cadeau(pool, beneficiaire_id, transaction_id, points)
```

---

## D. Points de suppression de code

| Élément | Action | Motif |
|---------|--------|-------|
| `services::engagement::evaluer_popularite` | **Supprimée** | Sémantique incompatible avec le crédit unitaire (research R3) |
| `handlers/media_social.rs::evaluer_popularite_media` | **Supprimée** | Remplacée par `crediter_jaime` + `resoudre_beneficiaire` |
| `ResultatPartageExterne { seuil, bonus_attribue }` | **Simplifié** | Le seuil de 5 réseaux n'existe plus (research R5) |
| `usePartageExterne` : retour de bonus | **Simplifié** | Plus de message « encore N réseaux pour votre bonus » |
| Branchements de `attribuer` sur les 8 actions écartées | **Conservés** | Neutralisés par `actif = FALSE` (research R1) |

---

## E. Vérification de cohérence cross-stack (Principe II)

| Champ | SQL | Rust | TypeScript |
|-------|-----|------|------------|
| Montants (`prix`, `montant`, `part_*`, `cagnotte`) | `INTEGER` | `i32` | `number` (entier, jamais formaté côté API) |
| `mode` | `engagement.mode_cadeau` | `String` validée à la désérialisation | union `'soutien_financier' \| 'points'` |
| `etat` | `engagement.etat_paiement` | `String` | union des 5 valeurs |
| `taux_commission` | `SMALLINT` | `i16` | `number` |
| `simule` | `BOOLEAN` | `bool` | `boolean` |

Le formatage monétaire (séparateurs, devise) est **exclusivement frontal**. Aucune API ne renvoie de montant formaté : deux représentations d'un même montant finiraient par diverger.

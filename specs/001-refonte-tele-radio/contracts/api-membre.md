# Contrat — API membre (JWT requis)

**Feature** : `001-refonte-tele-radio`
Authentification : `Authorization: Bearer <access_token>` → `extraire_utilisateur_id(&req)`.
**Ne jamais utiliser l'extracteur `AdminUtilisateur`** sur ces routes : il rejette tout non-admin
(`middleware/admin.rs:100-105`), or les contributeurs et co-détenteurs sont des membres ordinaires.

---

## 1. Interactions communautaires (US3)

| Méthode | Chemin | Corps | Effet |
|---|---|---|---|
| POST | `/api/medias/{type_media}/{media_id}/reaction` | `{ "type_reaction": "like" \| "dislike" \| null }` | `ON CONFLICT … DO UPDATE` ; `null` retire la réaction. Une seule retenue par membre (FR-023) |
| POST | `/api/medias/{type_media}/{media_id}/commentaires` | `{ "contenu": "…" }` (1–2000) | crée le commentaire (FR-024) |
| DELETE | `/api/medias/commentaires/{id}` | — | soft delete, **auteur uniquement** |
| POST | `/api/medias/{type_media}/{media_id}/partages` | `{ "legende": "…" }` (≤ 500, facultative) | publie sur le mur communautaire (FR-025) |
| POST | `/api/medias/{type_media}/{media_id}/signalement` | `{ "motif": "…", "description": "…" }` (≤ 1000) | idempotent par `UNIQUE (type_media, media_id, signale_par)` (FR-049) |

**Réponse du signalement** : `{ "nombre_signalements": 11, "suspendu": true }` — le passage à
`etat = 'suspendu'` au-delà de 10 signalements distincts est **immédiat et sans intervention** (FR-050,
SC-009).

---

## 2. Soumission et suivi (US4)

| Méthode | Chemin | Notes |
|---|---|---|
| POST | `/api/medias/propositions` | **multipart** — champs texte + fichiers média et image |
| GET | `/api/medias/propositions/moi?statut=&type_objet=&page=` | comble le trou fonctionnel de vidafrica, qui n'offre aucun suivi (FR-034) |
| PATCH | `/api/medias/propositions/{id}/retirer` | auteur uniquement, et seulement si `statut = 'en_attente'` |

**Corps de la soumission** :

```jsonc
{
  "type_objet": "chaine_tv | station_radio | programme_tele | programme_radio | animation_programme | idee_contenu",
  "target_id": "uuid | null",          // requis pour animation_programme et idee_contenu
  "justification": "…",                 // obligatoire
  "donnees": {                          // payload de l'objet, validé selon type_objet
    "nom": "…", "description": "…",
    "role_partie_prenante": "journaliste",       // chaîne/station — FR-029
    "role_partie_prenante_autre": null,          // obligatoire si role = 'autre'
    "theme_phare_id": "uuid",                    // contenu — FR-030
    "theme_phare_autre": null,                   // obligatoire si theme_phare_id absent
    "origine_publication": "territoire",         // station : forcé côté serveur (voir ci-dessous)
    "video_url": "https://youtu.be/… | null"     // ou fichier téléversé — FR-056
  }
}
```

**Trois garde-fous serveur, non négociables** :

1. **`origine_publication` est forcée à `'territoire'`** pour toute soumission membre. La bannière Radio
   Africans est une décision éditoriale de la plateforme (FR-036) : la valeur du client est ignorée, jamais
   simplement validée.
2. **`statut` naît toujours à `'en_attente'`** et n'est pas pilotable par le client. Rien n'est public avant
   validation (FR-031).
3. **« Autre » sans précision → 400.** Doublé par un `CHECK` SQL, la validation applicative fournissant le
   message en français.

**Aucune décharge de droits n'est demandée** (H-012) — pas de champ `decharge_droits`, contrairement à
vidafrica. L'examen de licéité relève de l'administrateur (FR-033).

---

## 3. Gestion de son support (co-détenteurs, US4 / US5)

Garde commune : `garde_detenteur(pool, type_support, support_id, moi, roles_admis)` →
**403** si le membre n'est pas co-détenteur actif au rôle requis.

| Méthode | Chemin | Rôle minimal |
|---|---|---|
| PATCH | `/api/medias/contenus/{type_media}/{id}/metadonnees` | `co_detenteur` |
| PUT | `/api/medias/contenus/{type_media}/{id}/media` | `co_detenteur` |
| GET | `/api/medias/{type_support}/{support_id}/detenteurs` | `programmateur` |
| POST | `/api/medias/{type_support}/{support_id}/invitations` | `proprietaire` |
| DELETE | `/api/medias/{type_support}/{support_id}/detenteurs/{utilisateur_id}` | `proprietaire` |
| GET | `/api/medias/invitations/moi` | — (destinataire) |
| PATCH | `/api/medias/invitations/{id}/accepter` · `/refuser` | — (destinataire) |
| POST | `/api/medias/{type_support}/{support_id}/contacter` | — (tout membre, FR-046) |

**Distinction structurante entre les deux routes d'édition de contenu** (FR-032, clarification du
2026-07-19) :

- `PATCH …/metadonnees` — titre, description, image, thème phare. **Publié immédiatement**, `etat` inchangé.
- `PUT …/media` — remplace le fichier vidéo ou audio. Bascule `etat` en `'en_attente'` et ouvre une
  `proposition_media` de type modification (`target_id` renseigné). **Le contenu cesse d'être diffusé
  jusqu'à revalidation.**

**Mise en relation** : `POST …/contacter` duplique `contacter_auteur` (`handlers/annonces.rs:893`) et
`obtenir_ou_creer_conversation_annonce` (`:146-166`). Aucun endpoint générique « ouvrir une conversation »
n'existe : la messagerie n'autorise l'envoi que si amitié active **ou** conversation préexistante
(`handlers/messagerie.rs:291-302`), donc seul un handler métier peut créer le canal. Respecter
`paire_canonique` (contrainte `ck_conversation_ordre : a_id < b_id`).

---

## 4. Grille de programmation (US5)

| Méthode | Chemin | Rôle minimal |
|---|---|---|
| POST | `/api/medias/{type_support}/{support_id}/creneaux` | `programmateur` |
| PUT | `/api/medias/creneaux/{id}` | `programmateur` |
| DELETE | `/api/medias/creneaux/{id}` | `programmateur` |

```jsonc
{
  "contenu_id": "uuid",
  "recurrence": "quotidien | hebdomadaire",
  "jour_semaine": 3,                    // 0=dimanche…6=samedi ; null si quotidien
  "heure_debut": "20:30",
  "duree_minutes": 45,
  "fuseau": "Africa/Abidjan"            // défaut si omis
}
```

**Séquence obligatoire côté serveur** (FR-040, edge case « co-détenteurs en concurrence ») :

1. `BEGIN`
2. `SELECT id FROM media_content.chaine_tv WHERE id = $1 FOR UPDATE` — verrou sur le **support parent**,
   qui sérialise toutes les modifications de sa grille, y compris les insertions concurrentes
3. recherche de chevauchement sur les créneaux actifs du support
4. si conflit → `409` détaillant le créneau en cause, **sans** écrire
5. `INSERT` / `UPDATE`, puis `audit::log_action` (FR-055)
6. `COMMIT`

**Contenu non hébergé** : un créneau pointant vers un média externe est accepté (FR-056), avec la réserve
que le démarrage à l'heure exacte dépend du lecteur tiers (SC-010).

---

## 5. Codes d'erreur

| Code | Cas |
|---|---|
| 400 | « Autre » sans précision ; `jour_semaine` incohérent avec `recurrence` ; créneau franchissant minuit |
| 401 | jeton absent ou expiré — la lecture reste publique, seule la participation exige un compte (FR-027) |
| 403 | non co-détenteur, ou rôle insuffisant ; suppression d'un commentaire dont on n'est pas l'auteur |
| 404 | objet inexistant, supprimé, ou suspendu |
| 409 | chevauchement de créneaux ; membre déjà co-détenteur actif |
| 422 | `type_objet` incompatible avec `target_id` (ex. `animation_programme` sans cible) |

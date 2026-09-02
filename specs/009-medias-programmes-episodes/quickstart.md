# Quickstart : validation de bout en bout

**Feature**: 009-medias-programmes-episodes

Ce guide vérifie que la feature fonctionne réellement. Il ne contient pas de code d'implémentation :
les détails de schéma sont dans [data-model.md](./data-model.md), les formes de requête et de réponse
dans [contracts/](./contracts/).

---

## Prérequis

```bash
# Base + LiveKit
docker compose up -d

# Backend (toujours tuer l'ancien processus : le port 8082 reste occupé sinon)
kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run

# Frontend
cd uafricas_frontend && pnpm dev        # http://localhost:3000
```

Comptes de test : `test-admin@test.com` / `Test1234` · `test-user@test.com` / `Test1234`.

---

## Étape 0 : Appliquer et vérifier la migration

```bash
psql "postgresql://uafricas@localhost:5432/africans_db" \
     -f uafricas_backend/doc/bd/schemas/09q_media_content_emissions_episodes.sql
```

La migration est idempotente : la rejouer ne doit produire aucune erreur.

**Rapport de reprise** : appeler `POST /api/admin/medias/rapport-reprise`
([contrat](./contracts/api-admin.md#5-purge-de-reprise)) et vérifier :

| Compteur | Attendu |
|----------|---------|
| `episodes_tele` = `emissions_tele` | ✅ un épisode par émission après reprise |
| `episodes_sans_emission` | **0** : sinon la reprise est incomplète |
| `creneaux_orphelins` | **0** |
| `slugs_en_collision` | **0** |
| `interactions_reportees.*` | égal au décompte avant migration |
| `chaines_sans_thematique` | non nul attendu, liste le travail éditorial restant |

Contre-vérification directe : les tables `media_content.programme_tele` et `programme_radio` **ne
doivent plus exister**. Toute requête les visant doit échouer bruyamment : c'est la garantie qui rend le
portage vérifiable (research.md R1).

```sql
SELECT to_regclass('media_content.programme_tele');   -- attendu : NULL
SELECT to_regclass('media_content.episode_tele');     -- attendu : non NULL
```

---

## Scénario 1 : Regrouper les vidéos sous un programme (US1)

1. Se connecter en `test-user`, détenteur d'une chaîne (au besoin, se l'attribuer depuis
   `/admin/medias` → détenteurs).
2. Aller sur `/mon-compte/mes-supports` → sa chaîne → **Créer un programme** « Débats africains »,
   cadence *hebdomadaire*, **sans joindre de fichier**.
   - ✅ L'enregistrement passe (FR-003) : aucune vidéo n'est réclamée.
3. Ajouter trois épisodes avec leurs vidéos.
   - ✅ Chacun affiche l'état **En attente de validation** (FR-040).
   - ✅ Aucun n'apparaît sur `/medias/tele` ni sur la page publique de la chaîne.
4. Se connecter en `test-admin` → `/admin/medias/moderation-episodes`.
   - ✅ Les trois épisodes sont en file, avec leur ancienneté et leur émission.
5. Valider les trois.
   - ✅ `test-user` reçoit trois notifications de validation (FR-041).
6. Revenir sur `/medias/tele`, ouvrir la chaîne.
   - ✅ **Un seul bloc** « Débats africains », annonçant 3 épisodes, pas trois vignettes séparées
     (US1 §3).
   - ✅ Ouvrir un épisode : la page nomme son programme et sa chaîne, et propose les autres épisodes
     (US1 §4).

**Vérification de non-régression des adresses (FR-056, SC-001)** : relever avant migration une URL
`/medias/programmes-tele/<slug>` d'un contenu existant, et la rouvrir après.
- ✅ Elle résout, et affiche désormais la page **épisode** de ce contenu.

**Vérification de l'émission vide (US1 §6)** : créer un programme sans épisode.
- ✅ Absent de l'espace public, présent et modifiable dans `/mon-compte/mes-supports` et
  `/admin/medias/emissions`.

---

## Scénario 2 : Rotation quotidienne et hebdomadaire (US2)

1. Sur la chaîne, programmer « Débats africains » : **chaque samedi à 18h00**, durée 60 min, fuseau
   `Africa/Abidjan`, `date_effet` = le samedi de la semaine en cours.
2. Appeler `GET /api/medias/chaine_tv/{id}/diffusion` un samedi entre 18h00 et 19h00.
   - ✅ `emission.titre` = « Débats africains », `episode` = le **1er** de l'ordre,
     `rang_occurrence` = 0, `est_rediffusion` = false.
3. **Déterminisme (FR-017, SC-006)** : rappeler trois fois dans la même plage.
   - ✅ Le même `episode.id` à chaque appel.
4. **Avancer d'une occurrence sans attendre une semaine** : reculer `date_effet` de 7 jours
   (`PUT /api/medias/creneaux/{id}`).
   - ✅ `rang_occurrence` = 1, l'épisode est le **2e** de l'ordre.
5. Reculer `date_effet` de 21 jours au total (rang 3, pour 3 épisodes).
   - ✅ `rang_occurrence` = 3, l'épisode revient au **1er**, `est_rediffusion` = **true** (FR-020).
   - ✅ L'interface publique affiche la mention « Rediffusion ».
6. **Ajout en cours de cycle (FR-019, US2 §4)** : ajouter et faire valider un 4e épisode pendant que
   l'occurrence courante est active.
   - ✅ L'épisode annoncé pour l'occurrence en cours **ne change pas**.
   - ✅ Il prend `ordre = 3` et entre au cycle suivant.
7. **Retrait en cours de cycle (US2 §6)** : supprimer un épisode.
   - ✅ La diffusion continue, sans trou ni doublon dans le même cycle.
8. **Émission programmée vide (FR-021, US2 §5)** : programmer un créneau sur un programme sans épisode
   publié.
   - ✅ `diffusion_en_cours` = `null`, le créneau **n'apparaît pas** dans la grille publique.
   - ✅ Il apparaît dans `GET …/grille?vue=detenteur` avec `alerte: "aucun_episode_publie"`.
9. **Chevauchement (FR-022)** : programmer un second programme le samedi de 18h30 à 19h30.
   - ✅ `409`, plage en conflit indiquée, grille existante inchangée.
10. **Franchissement de minuit (FR-023)** : créneau 23h30, durée 60 min.
    - ✅ `400`.
11. **Fuseau (FR-026)** : la grille publique indique explicitement `Africa/Abidjan`.

**Alerte de cadence (FR-024)** : appeler `GET /api/medias/mes-alertes-cadence` deux jours avant
l'échéance hebdomadaire.
- ✅ `niveau: "approche"`. Après l'échéance sans nouvel épisode : `niveau: "depassee"`.
- ✅ Si un épisode est en file, `episodes_en_attente ≥ 1`, l'alerte reste informative, pas
  accusatrice.

---

## Scénario 3 : Thématiques multiples (US3)

1. `/admin/medias` → une chaîne → sélectionner **trois** thématiques → enregistrer.
   - ✅ Les trois s'affichent sur `/medias/chaines/<slug>`.
2. Retirer toutes les thématiques et enregistrer une chaîne publiée.
   - ✅ Refus explicite : au moins une thématique requise (FR-029).
3. Sur `/medias/tele`, filtrer sur chacune des trois.
   - ✅ La chaîne remonte à chaque fois, **une seule fois** par résultat (FR-030).
4. `GET /api/television/thematiques`.
   - ✅ Ne liste que les thèmes réellement déclarés, avec leur décompte.
5. Répéter sur une station radio.
   - ✅ Comportement identique (US3 §5).
6. Vérifier qu'une chaîne héritée sans correspondance de catégorie reste **consultable** malgré
   l'absence de thématique (edge case), et que sa première modification en réclame une.

---

## Scénario 4 : Couverture territoriale (US4)

1. Chaîne A : sélectionner quatre territoires. Chaîne B : cocher **toute l'Afrique**.
   - ✅ Sur B, la sélection individuelle est neutralisée (FR-034).
2. Tenter d'ajouter un territoire à B **par l'API** (`PUT …/couverture` avec les deux renseignés).
   - ✅ `400` côté API, et le trigger SQL refuse l'écriture directe :

   ```sql
   INSERT INTO media_content.support_territoire (type_support, support_id, pays_id)
   VALUES ('chaine_tv', '<id de B>', '<un pays>');
   -- attendu : ERROR … Couverture continentale déclarée …
   ```

3. Filtrer `/medias/tele` sur un territoire couvert par A.
   - ✅ A **et** B remontent (FR-036).
4. Filtrer sur un territoire non couvert par A.
   - ✅ A ne remonte pas, B remonte toujours.
5. Enregistrer une chaîne publiée sans territoire ni couverture continentale.
   - ✅ Refus (FR-035).

---

## Scénario 5 : Interactions aux deux niveaux (US5)

1. En `test-user`, commenter un **épisode**.
   - ✅ Le commentaire est sur l'épisode, **absent** du fil du programme (US5 §1).
2. Réagir au **programme**.
   - ✅ Le compteur du programme s'incrémente, aucun compteur d'épisode ne bouge (US5 §2).
3. Ouvrir la page du programme.
   - ✅ Ses compteurs et ceux de ses épisodes sont présentés **distinctement**, jamais additionnés
     (FR-048).
4. Partager l'un puis l'autre depuis `/publications`.
   - ✅ Chaque partage mène à la bonne cible (FR-049).
5. Déplacer un épisode vers un autre programme
   (`PATCH /api/medias/episodes/{id}/emission`).
   - ✅ Réactions, commentaires, partages et signalements l'ont suivi intégralement (FR-009, US5 §4).
6. **Suspension par niveau (FR-050)** : porter un épisode à 11 signalements distincts.
   - ✅ L'épisode passe `suspendu` ; son **programme reste publié**.
   - ✅ Suspendre le programme retire ses épisodes du public sans les supprimer.
7. Rétablir depuis `/admin/medias/signalements`.
   - ✅ `nombre_signalements` est remis à 0, les lignes de signalement sont conservées.

**Continuité (FR-051)** : vérifier qu'un commentaire déposé **avant** la migration est toujours attaché
à l'épisode issu du contenu qu'il visait.

```sql
SELECT type_media, count(*) FROM media_content.media_commentaire GROUP BY 1;
-- attendu : plus aucune ligne 'programme_tele' ni 'programme_radio'
```

---

## Scénario 6 : Rejet et resoumission

1. En `test-user`, soumettre un épisode.
2. En `test-admin`, le rejeter avec un motif de moins de 10 caractères.
   - ✅ `400`.
3. Le rejeter avec un motif complet.
   - ✅ L'auteur est notifié **avec le motif** (SC-008).
   - ✅ L'épisode n'entre jamais dans la rotation.
4. En `test-user`, corriger l'épisode et l'enregistrer.
   - ✅ Il repasse `en_attente`, `motif_rejet` est effacé (FR-041).

Vérification SQL de l'invariant de rejet :

```sql
UPDATE media_content.episode_tele SET etat = 'rejete', motif_rejet = NULL WHERE id = '<id>';
-- attendu : ERROR … ck_episode_tele_rejet_motive
```

---

## Scénario 7 : Volume et performance (SC-009, SC-010)

1. Injecter un support de 50 programmes et 500 épisodes publiés.
2. Ouvrir sa page publique.
   - ✅ Première vue en moins de 2 s ; la liste d'épisodes d'un programme est paginée (24 par page).
   - ✅ Les sections ne déclenchent **aucune requête N+1**, vérifier avec `RUST_LOG=sqlx=debug` que
     le nombre de requêtes ne croît pas avec le nombre de programmes.
3. Filtrer par thématique puis par territoire.
   - ✅ Résultats en moins d'1 s perçue.
4. Appeler `…/diffusion` sur ce support.
   - ✅ Toujours **2 requêtes** : la rotation est une jointure latérale, pas un aller-retour
     supplémentaire (research.md R3).

---

## Vérifications transverses

**Audit (FR-045, SC-012)**, après avoir joué les scénarios 1 à 6, ouvrir `/admin/audit` :
- ✅ Chaque création/modification d'émission, d'épisode, d'ordre, de thématique, de couverture, de
  créneau et chaque décision de modération y figure, avec son auteur et l'état avant/après.

**Diagnostics** : `cargo check` côté backend, `getDiagnostics` (Volar) sur chaque fichier Vue modifié.

**Constitution** :
- ✅ Aucune classe daisyUI sur les pages `/medias/**` (principe VI) :
  `grep -rn "\bbtn\b\|\bcard\b\|\bmodal\b" uafricas_frontend/app/pages/medias/` ne doit rien remonter
  d'utilisé comme classe daisyUI.
- ✅ Aucun fichier ni dossier créé avec accent ou caractère spécial.
- ✅ Libellés, colonnes et identifiants en français.

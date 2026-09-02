# Contrat : API back-office

**Feature** : `001-refonte-tele-radio`
Extracteur : `AdminUtilisateur` (exige le rôle `admin` ou `super_admin`).
Garde de permission : **`verifier_permission!(admin, "media", <action>)`**.

> **Piège à ne pas reproduire** : `"media"` couvre radio et télé (`admin/radio_tele.rs`, 21 occurrences) ;
> `"media_content"` couvre vidafrica. `"programme"` désigne les programmes d'échange, pas les programmes
> radio/TV. Utiliser `"media"` : voir R15.

> **Prérequis de migration** : `15_seed.sql` ne déclare **aucune** permission `media`. Sans le seed ajouté
> par `09j`, seul `super_admin` (wildcard `all.all`) franchira ces gardes et la file de modération sera
> inaccessible aux administrateurs ordinaires.

---

## 1. File de modération des propositions (US4)

| Méthode | Chemin | Permission |
|---|---|---|
| GET | `/api/admin/medias/propositions?statut=&type_objet=&auteur=&page=&par_page=&tri_par=&tri_dir=` | `media.voir` |
| GET | `/api/admin/medias/propositions/{id}` | `media.voir` |
| PATCH | `/api/admin/medias/propositions/{id}/valider` | `media.modifier` |
| PATCH | `/api/admin/medias/propositions/{id}/rejeter` | `media.modifier` |

**Valider** : corps `{ "commentaire": "…" }` (facultatif). Séquence atomique, calquée sur
`admin/propositions_salle.rs:204-390` :

```
BEGIN
  SELECT … FOR UPDATE                       -- refuse de re-trancher une proposition déjà décidée
  INSERT INTO <table cible> … RETURNING id  -- selon type_objet
  INSERT INTO support_detenteur (role='proprietaire')   -- si chaîne ou station
  UPDATE proposition_media SET statut='validee', decideur, decide_at=NOW(), objet_id_cree
  INSERT INTO arbre_genealogique.notifications …        -- DANS la transaction
COMMIT
audit::log_action(…)                        -- après commit, non bloquant
```

La notification est émise **dans la transaction** (style `admin/profils_pays.rs:2545-2559`), non en
fire-and-forget : une décision de publication ne doit pas pouvoir être commitée sans que l'auteur en soit
averti (FR-034).

**Rejeter** : corps `{ "commentaire": "…" }` **obligatoire, ≥ 10 caractères** (FR-033). Garde applicative
doublée du `CHECK ck_prop_media_rejet_commente`. Aucun objet n'est créé ; l'auteur voit le motif dans
`/api/medias/propositions/moi`.

**Examen de licéité** : aucune décharge de droits n'ayant été recueillie (H-012), l'écran de modération doit
présenter en évidence la source du média et l'auteur déclaré, l'administrateur étant seul à se prononcer sur
les droits d'auteur et l'autorisation de rediffusion.

---

## 2. Mise en avant et qualification éditoriale (US1, US2)

| Méthode | Chemin | Effet |
|---|---|---|
| PATCH | `/api/admin/television/programmes-tele/{id}/vedette-globale` | `{ "a_la_une_globale": true }`, bascule l'ancienne vedette à `false` **dans la même transaction** (FR-001) |
| PATCH | `/api/admin/stations-radio/{id}/origine` | `{ "origine_publication": "africans" \| "territoire" }`, détermine la page d'affichage (FR-014) |

**Concurrence** : l'exclusivité de `a_la_une` est aujourd'hui gérée par deux requêtes séparées sur le pool
(`admin/radio_tele.rs:1256-1265`, `:1392-1407`). Avec l'index unique global de `09j`, la seconde échouerait
en cas de concurrence : envelopper les deux requêtes dans une transaction est **obligatoire**, et le même
correctif s'applique à l'exclusivité par chaîne et par station déjà en place.

**CRUD existants enrichis** (`/api/admin/stations-radio`, `/chaines-tv`, `/programmes-radio`,
`/programmes-tele`) : ajout de `origine_publication`, `role_partie_prenante` + `_autre`, `theme_phare_id` +
`_autre`, `a_la_une_globale`. Filtres de liste étendus à `origine` et `theme_phare_id`.

---

## 3. Modération des contenus et signalements (US7)

| Méthode | Chemin | Effet |
|---|---|---|
| GET | `/api/admin/medias/signalements?type_media=&suspendu=&page=` | file des contenus signalés, triée par `nombre_signalements DESC` |
| PATCH | `/api/admin/medias/{type_media}/{id}/etat` | `{ "etat": "publie" \| "suspendu" \| "supprime" }` |

**Rétablissement** (FR-051) : passer un contenu suspendu à `'publie'` remet `nombre_signalements = 0`.
Sans cette remise à zéro, le contenu serait resuspendu au premier signalement suivant, le seuil restant
franchi. Les signalements individuels sont conservés pour l'historique.

**Retrait de l'antenne** (FR-033) : `'suspendu'` retire le contenu de toutes les pages publiques sans le
supprimer : il reste consultable et réactivable en back-office. Un contenu retiré alors qu'il est vedette
ou programmé fait basculer la page sur son repli à la requête suivante.

---

## 4. Co-détention (US5)

| Méthode | Chemin | Permission |
|---|---|---|
| GET | `/api/admin/medias/{type_support}/{support_id}/detenteurs` | `media.voir` |
| POST | `/api/admin/medias/{type_support}/{support_id}/detenteurs` | `media.modifier` |
| DELETE | `/api/admin/medias/{type_support}/{support_id}/detenteurs/{utilisateur_id}` | `media.modifier` |

Logique d'ajout à trois branches, reprise telle quelle de `admin/moderateurs_afrolang.rs:59-190` :

| Situation | Résultat |
|---|---|
| ligne existante, `actif = TRUE` | **400**, « Ce membre est déjà co-détenteur de ce support » |
| ligne existante, `actif = FALSE` | `UPDATE … SET actif = TRUE, retire_at = NULL, role, designe_par, designe_at = NOW()` |
| aucune ligne | `INSERT` |

Retrait : `UPDATE … SET actif = FALSE, retire_at = NOW() WHERE … AND actif = TRUE`, puis contrôle de
`rows_affected() == 0` → **404**. L'historique n'est jamais effacé.

Un support sans co-détenteur actif reste diffusé et administrable, sa grille demeure modifiable par un
administrateur (edge case « dernier co-détenteur retiré »).

---

## 5. Audit

Conformément au Principe VII et à FR-055, `audit::log_action` est appelé sur **toute** mutation introduite :

| Action | `table_name` |
|---|---|
| `VALIDATION` / `REJET` | `proposition_media` |
| `MISE_EN_AVANT_GLOBALE` | `programme_tele` |
| `CHANGEMENT_ORIGINE` | `station_radio` |
| `SIGNALEMENT` / `SIGNALEMENT_SUSPENSION` / `RETABLISSEMENT` | table du contenu visé |
| `CREATE` / `DELETE` | `support_detenteur` |
| `CREATE` / `UPDATE` / `DELETE` | `creneau_programmation` |

**Amélioration attendue sur l'existant** : `admin/radio_tele.rs` passe systématiquement `ancien_etat` et
`nouvel_etat` à `None` (ex. `:1473-1474`), rendant l'audit inexploitable pour un diff. Les nouvelles
mutations doivent renseigner les deux instantanés JSONB.

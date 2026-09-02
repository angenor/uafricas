# Quickstart : validation de la feature 010

**Feature**: 010-medias-equipes-vitrine · **Date**: 2026-08-10

Le projet n'a **aucun harnais de test** (contrainte constitutionnelle assumée). La validation est manuelle et scénarisée. Chaque scénario cite l'exigence qu'il prouve.

---

## Prérequis

```bash
# Base + services
docker compose up -d                 # PostgreSQL 5432, Adminer 8088, LiveKit

# Migration de la feature (BD déjà initialisée)
psql "postgresql://uafricas@localhost:5432/africans_db" \
     -f uafricas_backend/doc/bd/schemas/09t_media_content_equipes_periodicite.sql

# Backend : toujours tuer l'ancien processus d'abord
kill $(lsof -i :8082 -t) 2>/dev/null; cd uafricas_backend && RUST_LOG=info cargo run

# Frontend
cd uafricas_frontend && pnpm dev      # http://localhost:3000
```

**Comptes** : `test-admin@test.com` / `Test1234` (admin) · `test-user@test.com` / `Test1234` (membre).

**Jeu de données** : la démonstration de 009 (`3 chaînes, 4 programmes, 13 épisodes`) suffit, à condition d'y ajouter les cas creux du §1.

### Contrôle de la migration

```sql
-- La table existe, avec ses 3 contraintes
\d media_content.membre_equipe

-- La périodicité accepte 4 valeurs et n'a perdu aucune déclaration
SELECT conname, pg_get_constraintdef(oid)
  FROM pg_constraint WHERE conname LIKE 'ck_emission_%_cadence';
SELECT cadence, COUNT(*) FROM media_content.emission_tele  GROUP BY 1
UNION ALL
SELECT cadence, COUNT(*) FROM media_content.emission_radio GROUP BY 1;
-- ⇒ aucune ligne ne doit avoir changé de cadence (FR-043)
```

---

## §1 : Fabriquer les cas creux (indispensable)

La feature se juge autant sur ce qu'elle affiche que sur ce qu'elle **n'affiche pas**. Avant tout scénario, créer en back-office :

| Cas | Comment |
|---|---|
| Une chaîne **sans aucun programme** | `/admin/television/create`, état `publié` |
| Un programme **sans aucun épisode publié** | `/admin/medias/emissions`, état `publié`, aucun épisode |
| Une chaîne **à description longue** (> 900 caractères) | coller un pavé |
| Un programme **à description longue** (> 400 caractères) | idem |
| Un programme **mensuel** | `UPDATE media_content.emission_tele SET cadence = 'mensuelle' WHERE id = '…'`, le CHECK l'accepte dès la migration, mais le sélecteur ne propose « Mensuel » qu'après la Phase 7 |
| Une chaîne portant **plus de 30 programmes** | duplication SQL d'un programme existant, pour éprouver le plafond de section et le lien « Voir les N programmes » (FR-008) |

Ces cinq objets couvrent à eux seuls SC-006, SC-007, FR-005 et les cas limites de la spec.

---

## §2 : Vitrine recentrée (US1)

Ouvrir `/medias/tele` **déconnecté**, puis répéter sur `/medias/radio/africans` et `/medias/radio/nationales` (FR-060).

| # | Vérification | Exigence |
|---|---|---|
| 2.1 | Aucune section ne contient de lecteur, de vignette d'épisode ni de rangée d'épisodes | FR-002, SC-001 |
| 2.2 | Chaque section montre : nom de la chaîne → extrait de description → équipe → bandeau de programmation → cartes de programme, dans cet ordre | FR-001, Q3 |
| 2.3 | La description longue est coupée et se termine par des points de suspension | FR-003 |
| 2.4 | Chaque carte de programme porte couverture + nom + description tronquée par des points de suspension | FR-004 |
| 2.5 | Le programme **sans épisode publié** est bien listé | FR-005 |
| 2.6 | La chaîne **sans programme** affiche son identité et son équipe, et signale l'absence de programmes | Cas limite |
| 2.7 | Une chaîne sans équipe n'affiche **aucun cadre vide** | FR-007, SC-007 |
| 2.8 | Un clic sur le nom de la chaîne mène à sa page ; un clic sur une carte mène au programme | FR-006, SC-002 |
| 2.9 | Le bandeau « en cours de diffusion / à suivre » est présent, en texte, sans lecteur | FR-002, SC-011 |
| 2.10 | Une chaîne de 30 programmes les affiche **tous** | FR-008, SC-008 |
| 2.11 | Au-delà du plafond, la section annonce le total et propose « Voir les N programmes » vers la page de la chaîne, jamais de disparition silencieuse | FR-008 |
| 2.12 | La description tronquée de la section **n'offre pas** de commande « voir plus » : en vitrine, FR-003 demande une ellipse, le dépliage est réservé aux pages de détail | FR-003 vs FR-021 |

> **Attendu contre-intuitif** : le nombre de chaînes affichées **augmente** par rapport à avant la feature, celles dépourvues d'épisode publié étaient filtrées. Noter le décompte avant/après. Voir [research.md D5](./research.md).

**Contrôle de charge (SC-008)** : sur une chaîne portant 30 programmes, le défilement de la page reste fluide et le payload de `/api/television/sections` ne contient plus aucun `episodes_apercu` :

```bash
curl -s 'http://localhost:8082/api/television/sections?par_page=6' | grep -c episodes_apercu   # ⇒ 0
```

---

## §3 : Déclarer une équipe (US2)

Connecté comme **détenteur** : `/mon-compte/mes-supports` → déplier un support → section **« Équipe éditoriale »** (à ne pas confondre avec « Gestion des accès », l'ancien panneau des co-détenteurs).

| # | Action | Attendu | Exigence |
|---|---|---|---|
| 3.1 | Ajouter 3 personnes, dont une avec une fonction inédite (« Concepteur sonore ») ; enregistrer | Sauvegarde en moins de 2 minutes, sans quitter la fiche | FR-010, SC-003 |
| 3.2 | Recharger la page | Les 3 personnes reviennent dans l'ordre déclaré | FR-016 |
| 3.3 | Rouvrir le champ « fonction » sur un **autre** support | « Concepteur sonore » est proposé | FR-015, SC-004 |
| 3.4 | Saisir « directeur », puis ailleurs « Directeur », puis « directeur  » (espaces) | La liste de suggestions ne montre **qu'une** entrée | FR-015 |
| 3.5 | Retirer une personne et remonter une autre en tête ; enregistrer | La vitrine reflète le nouvel ordre, sans la personne retirée | FR-016 |
| 3.6 | Déclarer l'équipe d'un **programme** du même support | Les deux équipes coexistent, sans recopie ni écrasement | FR-011 |
| 3.7 | Saisir une personne **sans** territoire ni contact | Aucun libellé vide à l'affichage public | FR-012, FR-007 |
| 3.8 | Rattacher une personne à un compte UAfricas existant | Son nom mène au profil public ; les autres restent en texte simple | FR-014, SC-010 |
| 3.9 | Enregistrer une équipe dont **aucun** membre n'est rattaché | Accepté sans réserve | FR-013, SC-010 |
| 3.10 | Vider entièrement l'équipe (liste vide) puis enregistrer | Le bloc « équipe » disparaît des pages publiques | FR-007 |

**Contrôle des droits (FR-017)** : déconnecté, ou connecté avec un compte qui ne détient pas le support, aucune commande d'édition n'est offerte, et l'appel direct est refusé :

```bash
curl -X PUT 'http://localhost:8082/api/medias/chaine_tv/<id>/equipe' \
     -H 'Content-Type: application/json' -d '{"membres":[]}'          # ⇒ 401
curl -X PUT … -H "Authorization: Bearer <jeton d'un non-détenteur>"    # ⇒ 403
```

**Contrôle d'audit (FR-018)** : après un enregistrement, `/admin/audit` contient une entrée `equipe_modifiee` sur `media_content.membre_equipe`, avec instantané avant/après.

**Contrôle de sécurité** : dans le JSON servi, le `contact` d'un membre rattaché est **celui saisi**, jamais l'e-mail de son compte.

---

## §4 : Page de détail d'une chaîne / station (US3)

`/medias/chaines/<slug>` puis `/medias/stations/<slug>`.

| # | Vérification | Exigence |
|---|---|---|
| 4.1 | Chaîne à description **courte** : texte entier, **aucun** bouton « voir plus » | FR-022 |
| 4.2 | Chaîne à description **longue** : extrait + « voir plus » ; le clic déplie sans rechargement, et le bouton propose de replier | FR-021, SC-005 |
| 4.3 | Équipe présentée personne par personne : nom, prénom, fonction, territoire, contact | FR-023 |
| 4.4 | Équipe de 11 personnes : sous-ensemble affiché + « voir plus » qui révèle les 11 | FR-024, SC-005 |
| 4.5 | Chaque programme affiche sa périodicité, son nom, sa description, son équipe propre et ses vidéos | FR-025 |
| 4.6 | **Aucune image de couverture de programme** sur cette page | FR-026 |
| 4.7 | Les vidéos listées sont consultables depuis cette page | FR-027 |
| 4.8 | Un programme mensuel affiche « Mensuel » ; un programme non périodique affiche « Non périodique » (et non un blanc) | FR-041, FR-044, US5-3 |

---

## §5 : Page de détail d'un programme (US4)

`/medias/emissions-tele/<slug>` puis `/medias/emissions-radio/<slug>`.

| # | Vérification | Exigence |
|---|---|---|
| 5.1 | La page affiche périodicité, nom, **image de couverture**, description, équipe, vidéos | FR-030 |
| 5.2 | La couverture est bien présente ici, alors qu'elle est absente de la page chaîne | FR-031 |
| 5.3 | Programme sans couverture : mise en page cohérente, pas d'emplacement vide signalé | Cas limite |
| 5.4 | L'équipe affichée est **celle du programme**, jamais celle de la chaîne en repli | FR-032 |
| 5.5 | Programme **sans épisode publié** : page consultable, message explicite d'absence de vidéo (et non un 404) | FR-033 |
| 5.6 | Le fil d'Ariane s'affiche (il était mort : `CommonFilAriane` n'existe pas) | Réparation D8 |
| 5.7 | La ligne héritée « Animation : … · Production : … » **a disparu** de la page : l'équipe est la seule source sur les personnes | FR-034 |
| 5.8 | Les deux champs hérités restent lisibles en back-office, sous un libellé « Champs hérités, reporter dans l'équipe » | [research.md D5 ter](./research.md) |

```bash
# Contrôle du 404 levé
curl -s 'http://localhost:8082/api/television/emissions/slug/<slug-sans-episode>' | head -c 200
# ⇒ 200 avec "nombre_episodes":0, et non 404
```

---

## §6 : Périodicité (US5)

| # | Vérification | Exigence |
|---|---|---|
| 6.1 | Le sélecteur d'un programme propose **exactement 4** valeurs : Non périodique, Journalier, Hebdomadaire, Mensuel | FR-040, US5-1 |
| 6.2 | Les libellés sont **identiques** en back-office, dans l'espace membre et en public | FR-041 |
| 6.3 | Un programme créé avant la feature conserve sa périodicité | FR-043, SC-006 |
| 6.4 | Un nouveau programme naît « Non périodique » | FR-042 |
| 6.5 | 100 % des programmes affichent une périodicité intelligible | SC-006 |

**Contrôle de l'alerte de cadence** : le piège de cette feature. Sur `GET /api/medias/mes-alertes-cadence`, un programme **mensuel** dont le dernier épisode date de 10 jours **ne doit pas** être signalé en retard ; à 31 jours, il doit l'être :

```sql
-- Vieillir artificiellement le dernier épisode d'un programme mensuel
UPDATE media_content.episode_tele SET valide_at = NOW() - INTERVAL '10 days'
 WHERE emission_id = '<id-programme-mensuel>';
```

```bash
curl -s -H "Authorization: Bearer <jeton détenteur>" \
     'http://localhost:8082/api/medias/mes-alertes-cadence'      # ⇒ pas d'alerte
# puis rejouer avec INTERVAL '31 days'                            # ⇒ alerte "depassee"
```

Sans la reprise de `periode_heures_cadence`, l'alerte se déclencherait dès le 8ᵉ jour.

---

## §7 : Parité Radio (FR-060) et non-régression

| # | Vérification |
|---|---|
| 7.1 | Les §2 à §6 rejoués sur l'espace Radio donnent les mêmes résultats, « station » et « audio » substitués |
| 7.2 | `/medias/tele` : la vedette plein écran conserve **sa** vidéo (elle n'est pas une section) |
| 7.3 | Signalement, partage, proposition d'idée et demande d'animation restent accessibles là où ils l'étaient |
| 7.4 | Les adresses déjà indexées résolvent : `/medias/chaines/<slug>`, `/medias/stations/<slug>`, `/medias/emissions-{tele,radio}/<slug>`, `/medias/programmes-{tele,radio}/<slug>` (page d'épisode), SC-009 |
| 7.5 | La grille de programmation, les thématiques et la couverture d'un support sont inchangées |
| 7.6 | Supprimer un programme portant une équipe : son équipe disparaît, celle de la chaîne est intacte, FR-019 |

```sql
-- Aucune équipe orpheline après suppression d'un porteur (FR-019)
SELECT COUNT(*) FROM media_content.membre_equipe m
 WHERE m.deleted_at IS NULL
   AND m.type_porteur = 'emission_tele'
   AND NOT EXISTS (SELECT 1 FROM media_content.emission_tele e
                    WHERE e.id = m.porteur_id AND e.deleted_at IS NULL);
-- ⇒ 0
```

---

## Critères de sortie

La feature est recevable quand :

- les §2 à §7 passent intégralement sur **les deux familles** (télé et radio) ;
- la requête d'équipes orphelines du §7 renvoie `0` ;
- `curl … /sections | grep -c episodes_apercu` renvoie `0` ;
- aucune page ne montre de cadre ni de libellé vide sur les six cas creux du §1 ;
- aucun programme ne disparaît en silence d'une section (§2.10 et §2.11) ;
- la ligne héritée « Animation / Production » n'apparaît plus sur aucune page publique ;
- le décompte de chaînes avant/après est noté et assumé (D5) ;
- `RUST_LOG=info cargo run` ne journalise aucune erreur SQL au parcours complet, **sqlx est vérifié au runtime**, une colonne oubliée compile sans broncher.

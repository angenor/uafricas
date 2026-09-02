# Quickstart : Refonte des pages Télé et Radio Africans

**Feature** : `001-refonte-tele-radio` | **Branche** : `001-refonte-tele-radio`

Ce document sert à démarrer le développement et à **valider** chaque lot. Le projet n'ayant aucun harnais de
test automatisé, la validation se fait par parcours manuels, chacun mappé à ses exigences.

---

## 0. Mise en route

```bash
# Infrastructure
docker compose up -d                       # PostgreSQL 5432 · Adminer 8088 · LiveKit

# Backend : toujours tuer l'ancien processus avant de relancer
kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run

# Frontend
pnpm dev                                   # http://localhost:3000
```

**Comptes de test** : `test-admin@test.com` / `Test1234`, `test-user@test.com` / `Test1234`.

### Appliquer les migrations

Aucun runner de migration n'existe : elles sont jouées **manuellement**, d'où l'exigence d'idempotence.

```bash
# Base fraîche : schema.sql orchestre tout via \ir
docker compose down -v && docker compose up -d

# Base existante : jouer les fichiers du lot, dans l'ordre
psql -h localhost -U uafricas -d africans_db \
     -f uafricas_backend/doc/bd/schemas/09j_media_content_editorial.sql
```

Chaque nouveau fichier **doit** être déclaré dans `uafricas_backend/doc/bd/schema.sql` par un `\ir`, placé
après la ligne de `09i`. Une migration oubliée passe inaperçue en développement (base déjà à jour) et casse
toute initialisation fraîche.

### Jeu de données de test

Un script installe tout le nécessaire aux parcours ci-dessous, sur les trois lots :

```bash
# Installation : idempotent, rejouable sans doublon
psql -h localhost -U uafricas -d africans_db \
     -f uafricas_backend/doc/bd/seed_test_medias.sql

# Retrait : ne touche qu'aux lignes marquées, les données existantes survivent
psql -h localhost -U uafricas -d africans_db \
     -f uafricas_backend/doc/bd/seed_test_medias_purge.sql
```

Il crée, sous des identifiants tous préfixés `dddd0000-` (marqueur exploité par la purge) :

- **4 stations radio** : 2 en `origine_publication = 'africans'`, 2 en `'territoire'`, avec leurs émissions,
  de quoi vérifier qu'aucune station n'apparaît sur les deux pages (FR-012, FR-014)
- **2 chaînes TV** et 4 émissions, dont une portant `a_la_une_globale = TRUE` (la vedette plein écran)
- **2 créneaux de programmation contigus** sur `Africans Doc` : l'un en cours, l'autre démarrant **3 minutes
  plus tard**. La résolution étant paresseuse, il suffit de recharger la page à l'échéance pour voir la
  bascule « En ce moment » → « À suivre » (FR-038, FR-042)
- **de la co-détention** : `test-admin` propriétaire, `test-user` co-détenteur puis programmateur : ce dernier
  accède donc à la grille depuis `/mon-compte/mes-supports`
- **4 propositions en attente** couvrant les quatre cas : chaîne, émission radio, **idée de contenu** (ne crée
  aucun objet) et **demande d'animation** (l'acceptation ajoute un co-détenteur, FR-045)
- **1 invitation de co-détention en attente**, visible sur `/mon-compte/invitations-medias`

Le script préserve les données déjà présentes : il ne retire la vedette générale existante que pour respecter
l'index unique partiel qui n'en autorise qu'une.

**Deux cas restent à créer à la main** depuis `/admin/television`, le seed ne pouvant les couvrir :

- une émission dont `video_url` pointe vers un **fichier téléversé**, le seed n'utilise que des liens
  YouTube, or les deux chemins de lecture doivent être exercés (FR-056) ;
- vérifier qu'une chaîne **sans aucun programme publié** ne génère aucune section (FR-008), la chaîne
  « Chaine Vide » du jeu initial joue ce rôle si elle est présente.

**Franchissement du seuil de signalement (US7)** : la suspension exige 11 comptes distincts, or seuls les deux
comptes de test ont un mot de passe connu. Poser le premier signalement depuis l'interface, puis compléter :

```sql
INSERT INTO media_content.signalement_media (type_media, media_id, signale_par, motif)
SELECT 'chaine_tv', '<id-de-la-chaine>', u.id, 'violence'
  FROM iam.utilisateur u WHERE u.deleted_at IS NULL LIMIT 10
ON CONFLICT DO NOTHING;
```

Un nouveau signalement depuis l'interface déclenche alors le recompte et la bascule, le compteur
dénormalisé n'étant mis à jour que par le handler, l'insertion SQL seule ne suspend rien.

---

## Lot 1 : Consultation (US1, US2) · MVP

### Ordre d'implémentation conseillé

1. Migration `09j` (+ `\ir` dans `schema.sql`)
2. Modèles Rust `FromRow` et DTO, puis handlers publics `sections` / `vedette` / `programmes-radio`
3. Composables `useTelevision` / `useStationsRadio` étendus, puis `useLecteurMedia` (`useState`)
4. Composants : `LecteurMedia` → `VedettePleinEcran` → `RangeeContenus` → `SectionChaine`/`SectionStation`
   → `BarreLecturePersistante`
5. Montage de la barre dans `layouts/default.vue`, hors du `<slot/>`
6. Remaniement des trois pages

### Parcours de validation

| # | Parcours | Exigences |
|---|---|---|
| 1 | Ouvrir `/medias/tele` : la vedette occupe **toute** la fenêtre et démarre, son coupé | FR-002, FR-003, SC-001 |
| 2 | Réduire à 375 px de large : même vedette plein écran, aucun défilement horizontal | FR-011, SC-013 |
| 3 | Un seul geste de défilement atteint la première section de chaîne | FR-009, SC-002 |
| 4 | Chaque section montre nom, territoire, catégorie, contenu mis en évidence, rangée défilante | FR-004, FR-005, FR-022 |
| 5 | La chaîne sans programme publié n'apparaît pas | FR-008 |
| 6 | Lancer un contenu depuis une rangée : lecture sur place, position de défilement conservée | FR-006, SC-003 |
| 7 | Dépublier la vedette, recharger : repli sur le contenu publié le plus récent, jamais d'écran vide | FR-007 |
| 8 | Ouvrir un contenu YouTube puis un contenu téléversé : les deux se lisent | FR-056 |
| 9 | `/medias/radio/africans` n'affiche **que** les stations `origine_publication = 'africans'` | FR-014, SC-005 |
| 10 | `/medias/radio/nationales` n'affiche que `'territoire'` ; aucune station sur les deux pages | FR-014, SC-005 |
| 11 | Lancer une écoute, défiler, changer de section, **naviguer vers `/medias/tele`** : le son continue, la barre reste visible | FR-017, SC-006 |
| 12 | Lancer un second contenu : le premier s'arrête, jamais deux flux | FR-018 |
| 13 | Une station avec `stream_url` propose son direct dans sa section | FR-016 |
| 14 | Tabulation seule : atteindre et actionner pause, son, changement de contenu | FR-053, SC-012 |
| 15 | Onglet Réseau : les médias des sections hors écran ne sont **pas** chargés | FR-054, SC-011 |

### Pièges vérifiés en Phase 0

- Retirer `videoProvisoireEmbed` (`tele.vue:139`) **expose** un bug latent : `programmeActif.videoUrl` peut
  contenir une URL YouTube `watch` injectée dans `<video :src>`, qui ne sait pas la lire. Le routage par
  `youtubeEmbedUrl()` (`useEvenements.ts:285`) doit être en place **avant** ce retrait.
- Supprimer le `v-if="!isMobile"` (`tele.vue:216`) qui prive aujourd'hui les mobiles de tout hero.
- Remplacer `loading loading-spinner` (daisyUI) par `animate-spin rounded-full border-b-2`, déjà utilisé en
  `tele.vue:392` : Principe VI.
- La barre en `fixed bottom-0` recouvrira le FAB messagerie (`bottom-6 right-6`, z-50) et l'invite d'appel
  (`bottom-24 right-6`, z-[75]) : les décaler quand la barre est active, et rester sous z-[75].
- Utiliser `h-[100svh]`, non `100vh`, qui déborde sous la barre d'URL mobile.
- La NavBar est `absolute` (pas `fixed`) : elle défile. Réserver son gabarit par `top-24` / `pt-24`.

---

## Lot 2 : Participation (US3, US4)

### Ordre d'implémentation

1. Migration `09k` (interactions) puis `09l` (propositions)
2. **Pages de détail SSR d'abord** : prérequis du partage : sans URL propre, pas d'aperçu social
3. Handlers `media_social.rs`, puis `media_proposition.rs` et son pendant admin
4. **Fermer la faille** : `stations_radio.rs:263`, `television.rs:207` et `:428` n'insèrent plus `'publie'`
5. Brancher la 8ᵉ source du mur `/publications`

### Parcours de validation

| # | Parcours | Exigences |
|---|---|---|
| 1 | Réagir, changer d'avis, retirer : le compteur ne double jamais | FR-023 |
| 2 | Commenter, recharger : le commentaire persiste avec auteur et date | FR-024 |
| 3 | Partager avec légende : la publication apparaît sur `/publications` | FR-025 |
| 4 | Coller l'URL de détail dans un validateur d'aperçu social : titre, description et image remontent | FR-026, R12 |
| 5 | Déconnecté : compteurs et commentaires visibles ; toute action invite à se connecter et **revient au contenu** | FR-027 |
| 6 | Soumettre une chaîne depuis un compte membre : invisible sur les pages publiques | FR-031, SC-007 |
| 7 | `POST /api/stations-radio` en direct (curl, membre) : ne crée **plus** de station publiée | FR-032 |
| 8 | Valider en back-office : la station apparaît, l'auteur devient `proprietaire` et reçoit une notification | FR-033, FR-037 |
| 9 | Rejeter sans motif → refus ; avec motif → l'auteur voit le motif dans son suivi | FR-033, FR-034 |
| 10 | Rôle ou thème « Autre » sans précision → refus | FR-029, FR-030 |
| 11 | Modifier le titre d'un contenu publié : visible aussitôt, sans revalidation | FR-032 |
| 12 | Remplacer son fichier média : repasse en `en_attente` et quitte l'antenne | FR-032 |
| 13 | Ouvrir un partage vers un contenu retiré : message explicite, pas le contenu | FR-028 |

---

## Lot 3 : Programmation, engagement, modération (US5, US6, US7)

### Ordre d'implémentation

1. Migration `09m` (co-détention) puis `09n` (grille)
2. `garde_detenteur` **avant** tout endpoint de co-détenteur
3. Résolution paresseuse du créneau courant, branchée sur les endpoints `sections`
4. Signalement, en dernier : il s'appuie sur `nombre_signalements` posé en `09k`

### Parcours de validation

| # | Parcours | Exigences |
|---|---|---|
| 1 | Planifier un contenu quotidien à 2 minutes d'ici ; à l'échéance, la section le diffuse | FR-037, FR-038, SC-010 |
| 2 | La section affiche « en ce moment » et le créneau suivant | FR-039 |
| 3 | Planifier un créneau chevauchant : refus **409** décrivant le conflit | FR-040 |
| 4 | Retirer le contenu d'un créneau actif : bascule sur le contenu mis en évidence | FR-041, FR-043 |
| 5 | Consulter depuis un autre fuseau : horaire cohérent, référentiel explicite | FR-042 |
| 6 | Deux navigateurs, deux co-détenteurs, enregistrement simultané : aucun écrasement silencieux | edge case |
| 7 | Inviter un co-détenteur, accepter : il peut programmer, pas révoquer | FR-037 |
| 8 | Retirer le dernier co-détenteur : le support reste diffusé et administrable | edge case |
| 9 | Demander l'animation d'un programme ; l'acceptation ajoute aux co-détenteurs | FR-045 |
| 10 | Rechercher un réalisateur par territoire et spécialité, puis le contacter | FR-046 |
| 11 | Signaler depuis 11 comptes distincts : le contenu quitte les pages publiques sans intervention | FR-049, FR-050, SC-009 |
| 12 | Re-signaler depuis un compte ayant déjà signalé : le compteur n'augmente pas | FR-049 |
| 13 | Rétablir en back-office : le compteur repart à zéro | FR-051 |
| 14 | Les règles de contenu interdit sont lisibles sur les trois pages | FR-048 |

---

## Vérifications transverses avant livraison

```bash
# Aucune classe daisyUI sur les pages publiques du périmètre (Principe VI)
grep -rnE '\b(btn|card|modal|loading|badge|navbar|drawer)\b' \
     uafricas_frontend/app/pages/medias/ uafricas_frontend/app/components/media/

# Aucun résidu Tailwind v3
grep -rn 'bg-gradient-to-' uafricas_frontend/app/pages/medias/ uafricas_frontend/app/components/media/

# Toute migration du lot est déclarée dans l'orchestrateur
grep -n '09[j-n]' uafricas_backend/doc/bd/schema.sql
```

- Lancer `getDiagnostics` (rust-analyzer, Volar) après chaque modification de fichier.
- Vérifier que `audit::log_action` est appelé sur **chaque** mutation nouvelle, avec `ancien_etat` et
  `nouvel_etat` renseignés : l'existant les passe à `None`, ne pas reproduire ce défaut.
- Ajouter une ligne dans « Recent Changes » de `CLAUDE.md`, citant l'indice de migration.

## Déploiement

```bash
./deploy.sh update
# puis, par SSH, jouer chaque migration du lot dans l'ordre sur la base de production
```

Les migrations étant idempotentes, un rejeu est sans effet. **Reprise de données spécifique** : après
`09j`, toutes les stations existantes sont qualifiées `origine_publication = 'territoire'` et apparaissent
donc sur Radio Nationales. Basculer manuellement celles relevant d'Africans :

```sql
UPDATE media_content.station_radio
   SET origine_publication = 'africans'
 WHERE id IN ( … );   -- liste arrêtée avec le commanditaire
```

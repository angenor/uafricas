# Phase 1 — Modèle de données

**Feature**: `008-recadrage-engagement-cadeaux` | **Schéma**: `engagement` | **Migrations**: `35f_engagement_recadrage.sql`, `35g_engagement_cadeaux.sql`

Conventions du projet respectées : UUID v4 en clé primaire, `TIMESTAMPTZ`, `snake_case` français, enums PostgreSQL, migrations idempotentes (`IF NOT EXISTS`, `DO $$ … EXCEPTION WHEN duplicate_object`).

---

## Partie A — Entités modifiées (migration `35f`)

### A1. `engagement.regle_points` — recadrage du barème

Aucun changement de structure. Trois opérations de données :

**Désactivation des 8 règles écartées** (FR-002) :

```
UPDATE engagement.regle_points SET actif = FALSE, updated_at = NOW()
 WHERE type_action IN ('contribution_validee', 'contribution_mise_en_avant',
                       'factcheck_valide', 'factcheck_faux',
                       'proposition_media_validee', 'media_a_la_une',
                       'animation_support_acceptee', 'partage_externe_5reseaux',
                       'popularite_palier');
```

`ajustement_admin` **reste active** : c'est l'outil de correction de l'administration (FR déjà livré), pas une source de points communautaire.

**Création des 3 règles canoniques** :

| `type_action` | `libelle` | `points` | `reputation_delta` | `plafond_journalier` | `categorie` | Rôle |
|---------------|-----------|---------:|-------------------:|---------------------:|-------------|------|
| `jaime_recu` | J'aime reçu sur un contenu | 1 | 0 | `NULL` | `popularite` | Montant réel |
| `partage_recu` | Contenu partagé par un membre | 1 | 0 | `NULL` | `partages` | Montant réel |
| `cadeau_recu` | Cadeau virtuel reçu | 0 | 0 | `NULL` | `cadeaux` | **Porte** : le montant vient du catalogue figé sur la transaction (R9) |

> Les plafonds sont livrés à `NULL` (illimité) et restent réglables en back-office. Rappel de la spécification 007 : **les plafonds s'expriment en points, pas en occurrences**.

### A2. `engagement.categorie_points` — catégorie « Cadeaux »

```
INSERT INTO engagement.categorie_points (code, libelle, description, ordre, couleur, icone)
VALUES ('cadeaux', 'Cadeaux', 'Points reçus grâce aux cadeaux virtuels offerts par la communauté.', 4, 'amber', 'gift')
ON CONFLICT (code) DO NOTHING;
```

Les catégories `popularite`, `partages` et `ajustements` existent déjà (migration `35c`). Les catégories devenues sans règle active (`contributions`, `medias`, `verification`) sont **conservées** : leur suppression est interdite tant qu'une règle y est rattachée (FR-004 de la spécification 007), et elles redeviendront utiles si une règle écartée est réactivée.

### A3. `engagement.niveau` — quatre statuts (FR-004, R6)

Ordre d'exécution **impératif** (l'ordre 3 doit se libérer avant l'insertion de `gold`) :

```
UPDATE engagement.niveau SET libelle = 'Membre Africans', seuil_min = 0,     ordre = 1 WHERE code = 'membre';
UPDATE engagement.niveau SET libelle = 'Premium',         seuil_min = 500,   ordre = 2 WHERE code = 'premium';
UPDATE engagement.niveau SET libelle = 'Platinum',        seuil_min = 10000, ordre = 4 WHERE code = 'platinum';
INSERT INTO engagement.niveau (code, libelle, seuil_min, ordre, badge_couleur, badge_icone)
VALUES ('gold', 'Gold', 2000, 3, 'yellow', 'medal')
ON CONFLICT (code) DO UPDATE SET libelle = EXCLUDED.libelle, seuil_min = EXCLUDED.seuil_min, ordre = EXCLUDED.ordre;
```

| Code | Libellé | `seuil_min` | `ordre` | Plage effective |
|------|---------|------------:|--------:|-----------------|
| `membre` | Membre Africans | 0 | 1 | 0 – 499 |
| `premium` | Premium | 500 | 2 | 500 – 1 999 |
| `gold` | Gold | 2 000 | 3 | 2 000 – 9 999 |
| `platinum` | Platinum | 10 000 | 4 | 10 000 et plus |

**Invariants existants conservés** : `seuil_min` unique, `code` unique. La borne haute n'est jamais stockée : elle se déduit du seuil suivant, ce qui rend une grille incohérente impossible à exprimer.

**Rebascule des comptes**, dans la même transaction que les `UPDATE` ci-dessus (FR-005) :

```
UPDATE engagement.compte c
   SET niveau_code = (SELECT n.code FROM engagement.niveau n
                       WHERE n.seuil_min <= c.solde_points
                       ORDER BY n.seuil_min DESC LIMIT 1),
       updated_at = NOW();
```

### A4. `engagement.palier_popularite` — neutralisé (FR-003, R3)

```
UPDATE engagement.palier_popularite SET actif = FALSE;
```

La table et les écrans d'administration sont conservés : le mécanisme n'est plus alimenté mais reste réactivable, comme les règles écartées. `services::engagement::evaluer_popularite` est en revanche **supprimée du code** — sa sémantique (crédit par palier de contenu) est incompatible avec le crédit unitaire par membre.

---

## Partie B — Entités nouvelles (migration `35g`)

### B1. Types énumérés

```
CREATE TYPE engagement.mode_cadeau  AS ENUM ('soutien_financier', 'points');
CREATE TYPE engagement.etat_paiement AS ENUM ('en_attente', 'abouti', 'echoue', 'expire', 'purge');
```

`purge` n'est pas un état de paiement à proprement parler : c'est la marque laissée par la purge de fin de phase de test (R11), qui conserve la ligne pour l'historique tout en signalant que ses effets ont été annulés.

### B2. `engagement.cadeau` — catalogue

| Colonne | Type | Contraintes | Rôle |
|---------|------|-------------|------|
| `id` | UUID | PK, `uuid_generate_v4()` | |
| `code` | VARCHAR(40) | `NOT NULL UNIQUE` | Clé stable, immuable après création |
| `libelle` | VARCHAR(80) | `NOT NULL` | Affiché au membre |
| `description` | TEXT | | Facultative |
| `icone` | VARCHAR(40) | | Nom FontAwesome |
| `couleur` | VARCHAR(20) | | Jeton de couleur front |
| `prix` | INTEGER | `NOT NULL CHECK (prix > 0)` | Unité entière de la devise (FCFA) |
| `points` | INTEGER | `NOT NULL CHECK (points > 0)` | Points crédités au bénéficiaire |
| `ordre` | SMALLINT | `NOT NULL DEFAULT 0` | Ordre d'affichage |
| `actif` | BOOLEAN | `NOT NULL DEFAULT TRUE` | Désactivation, jamais suppression (FR-028) |
| `created_at` / `updated_at` | TIMESTAMPTZ | `NOT NULL DEFAULT NOW()` | |

**Catalogue initial** (FR-016 ; prix proportionnels aux points, paramétrables) :

| `code` | Libellé | `points` | `prix` (FCFA) | `icone` |
|--------|---------|---------:|--------------:|---------|
| `drapeau_ua` | Drapeau de l'Union Africaine | 20 | 2 000 | `flag` |
| `badge` | Badge | 10 | 1 000 | `certificate` |
| `chapeau` | Chapeau | 5 | 500 | `hat-cowboy` |
| `fleur` | Fleur | 3 | 300 | `seedling` |
| `epingle` | Épingle de costume | 1 | 100 | `thumbtack` |

> Le rapport prix/points est constant (100 FCFA le point) à la mise en service. Rien ne l'impose : l'administration peut le rompre pour valoriser un cadeau symbolique.

### B3. `engagement.parametre_monetisation` — singleton

| Colonne | Type | Contraintes |
|---------|------|-------------|
| `id` | BOOLEAN | `PRIMARY KEY DEFAULT TRUE CHECK (id)` — ligne unique par construction |
| `taux_commission` | SMALLINT | `NOT NULL DEFAULT 10 CHECK (taux_commission BETWEEN 0 AND 100)` |
| `devise` | VARCHAR(3) | `NOT NULL DEFAULT 'XOF'` |
| `paiement_reel_actif` | BOOLEAN | `NOT NULL DEFAULT FALSE` — bascule CinetPay ; informe le bandeau et la purge |
| `updated_at` | TIMESTAMPTZ | `NOT NULL DEFAULT NOW()` |

L'astuce `id BOOLEAN PRIMARY KEY CHECK (id)` rend la **seconde ligne impossible en SQL** : il ne peut exister qu'un seul paramétrage, sans code de garde applicatif.

### B4. `engagement.transaction_cadeau` — journal comptable

| Colonne | Type | Contraintes | Rôle |
|---------|------|-------------|------|
| `id` | UUID | PK | |
| `offreur_id` | UUID | `NOT NULL REFERENCES iam.utilisateur(id) ON DELETE SET NULL` → voir note | Toujours issu du JWT |
| `beneficiaire_id` | UUID | `NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE` | Résolu **serveur** (R4) |
| `cadeau_id` | UUID | `NOT NULL REFERENCES engagement.cadeau(id) ON DELETE RESTRICT` | Rend la suppression d'un cadeau offert impossible (FR-028) |
| `type_objet` | VARCHAR(40) | `NOT NULL` | Famille du contenu, ou `'profil'` pour un cadeau offert depuis un profil |
| `objet_id` | UUID | `NOT NULL` | Identifiant du contenu, ou `utilisateur_id` du bénéficiaire si `type_objet = 'profil'` |
| `mode` | `engagement.mode_cadeau` | `NOT NULL` | |
| `montant` | INTEGER | `NOT NULL CHECK (montant > 0)` | Prix **figé** (FR-024) |
| `points` | INTEGER | `NOT NULL CHECK (points > 0)` | Points **figés** (FR-024) |
| `taux_commission` | SMALLINT | `NOT NULL` | Taux **figé** (FR-024) |
| `part_beneficiaire` | INTEGER | `NOT NULL CHECK (part_beneficiaire >= 0)` | |
| `part_plateforme` | INTEGER | `NOT NULL CHECK (part_plateforme >= 0)` | |
| `etat` | `engagement.etat_paiement` | `NOT NULL DEFAULT 'en_attente'` | |
| `simule` | BOOLEAN | `NOT NULL DEFAULT TRUE` | Rend la purge exacte (R7) |
| `reference_paiement` | TEXT | `NOT NULL UNIQUE` | Référence rendue par le prestataire (ou le simulateur) |
| `message` | VARCHAR(280) | | Mot facultatif de l'offreur |
| `created_at` | TIMESTAMPTZ | `NOT NULL DEFAULT NOW()` | |
| `finalise_at` | TIMESTAMPTZ | | Renseigné à l'aboutissement, l'échec ou l'expiration |

**Contraintes structurelles — les invariants métier deviennent impossibles à violer en SQL :**

```
CHECK (part_beneficiaire + part_plateforme = montant)             -- SC-009, exact par construction
CHECK (offreur_id <> beneficiaire_id)                             -- FR-023, auto-cadeau impossible
CHECK (type_objet <> 'profil' OR objet_id = beneficiaire_id)      -- cadeau au profil : la cible EST le bénéficiaire
CHECK (mode <> 'points' OR part_beneficiaire = 0)                  -- FR-018, mode points = 100 % plateforme
CHECK (etat <> 'en_attente' OR finalise_at IS NULL)                -- pas de finalisation sans état final
```

> **Pourquoi `type_objet = 'profil'` plutôt qu'une paire nullable** : le partage d'un profil emploie déjà `type_objet = 'profil'`, `objet_id = utilisateur_id` (contrats §C2). Deux représentations du même objet dans un même schéma finiraient par diverger dans les requêtes d'agrégation. Le `CHECK` ci-dessus rend en outre impossible un cadeau « au profil » pointant vers quelqu'un d'autre que son bénéficiaire.

**Index** : `(beneficiaire_id, etat)`, `(offreur_id)`, `(type_objet, objet_id) WHERE etat = 'abouti'` (affichage des cadeaux d'un contenu), `(created_at DESC)` (journal admin), `(simule) WHERE etat = 'abouti'` (purge).

**Note sur `offreur_id`** : `ON DELETE SET NULL` exigerait une colonne nullable, ce qui contredirait `NOT NULL`. La colonne est donc `NOT NULL REFERENCES … ON DELETE CASCADE` comme le reste du schéma `engagement`. Une transaction dont l'offreur disparaît disparaît avec lui — aucune archive comptable n'est bâtie, la recette restant connue par les états agrégés exportés avant la suppression. Le cas limite correspondant de `spec.md` a été aligné sur cette décision ; construire une table d'archive avant que l'encaissement ne soit réel serait prématuré (Principe V).

### B5. `engagement.cagnotte` — solde de soutien du bénéficiaire

| Colonne | Type | Contraintes |
|---------|------|-------------|
| `utilisateur_id` | UUID | `PRIMARY KEY REFERENCES iam.utilisateur(id) ON DELETE CASCADE` |
| `montant_cumule` | INTEGER | `NOT NULL DEFAULT 0 CHECK (montant_cumule >= 0)` |
| `montant_verse` | INTEGER | `NOT NULL DEFAULT 0 CHECK (montant_verse >= 0)` |
| `updated_at` | TIMESTAMPTZ | `NOT NULL DEFAULT NOW()` |

```
CHECK (montant_verse <= montant_cumule)
```

`montant_verse` reste à 0 pendant toute cette itération (FR-026, aucun payout). La colonne existe dès maintenant pour que l'arrivée du versement n'exige pas de migration de la table la plus sensible.

**Cohérence vérifiable** (SC-009) — invariant que la recette doit pouvoir contrôler :

```
montant_cumule = COALESCE((SELECT SUM(part_beneficiaire) FROM engagement.transaction_cadeau
                            WHERE beneficiaire_id = c.utilisateur_id AND etat = 'abouti'), 0)
```

---

## Partie C — Transitions d'état d'une transaction

```
                 initier()                confirmer(aboutir = true)
    (néant) ──────────────► en_attente ──────────────────────────► abouti
                                │                                     │
                                │ confirmer(aboutir = false)          │ purge de fin de phase
                                ├────────────────────► echoue        │ (simule = TRUE uniquement)
                                │                                     ▼
                                │ délai dépassé                     purge
                                └────────────────────► expire
```

| Transition | Déclencheur | Effets |
|------------|-------------|--------|
| → `en_attente` | `POST /cadeaux/envoyer` | Création de la ligne, prix/points/taux figés, référence obtenue du simulateur. **Aucun point, aucune cagnotte.** |
| `en_attente` → `abouti` | `POST /paiements/{reference}/confirmer` avec succès | Cagnotte créditée (mode `soutien_financier` uniquement) **dans la transaction SQL** ; points crédités et notification émise **après le COMMIT** (R10). |
| `en_attente` → `echoue` | Confirmation en échec | Aucun effet, la ligne reste pour l'analyse. Le membre peut réémettre un envoi. |
| `en_attente` → `expire` | Lecture d'une intention dont `created_at` dépasse 30 minutes | Résolution **paresseuse**, aucune tâche de fond — même motif que les créneaux de programmation média. |
| `abouti` → `purge` | `POST /admin/engagement/purger-phase-test` | Mouvements de points supprimés, soldes et niveaux recalculés depuis le journal, cagnottes réduites d'autant. |

**États terminaux** : `abouti` (hors purge), `echoue`, `expire`, `purge`. Aucun retour en arrière n'est prévu ; un litige se règle par l'ajustement manuel motivé déjà livré.

---

## Partie D — Récapitulatif des clés d'idempotence

L'ensemble du recadrage repose sur trois clés, et sur elles seules. Aucune vérification en lecture-puis-écriture n'est nécessaire.

| Source | Clé `mouvement_points.cle_idempotence` | Ce que la clé garantit |
|--------|----------------------------------------|------------------------|
| J'aime reçu | `jaime:{type_objet}:{objet_id}:{membre_qui_aime_id}` | Un membre ne crédite qu'une fois par contenu, quels que soient les retraits/remises (FR-010, FR-011) |
| Partage reçu | `partage:{type_objet}:{objet_id}:{partageur_id}` | Un partageur ne crédite qu'une fois par contenu, tous canaux confondus (FR-013) |
| Cadeau reçu | `cadeau:{transaction_id}` | Une transaction ne crédite qu'une fois, quel que soit le nombre de confirmations rejouées (FR-022) |

Clés **héritées** encore présentes dans le journal (règles désactivées) : `popularite:*`, `partage5:*`, et les clés des actions de contribution. Elles ne sont plus produites mais restent lisibles (FR-037).

---

## Partie E — Impact sur les entités existantes non modifiées structurellement

| Entité | Impact |
|--------|--------|
| `engagement.compte` | Aucun changement de colonne. `niveau_code` rebasculé par `35f`, puis maintenu par le moteur. |
| `engagement.mouvement_points` | Aucun changement de colonne. Trois nouveaux `type_action`, trois nouveaux motifs de clé. |
| `engagement.badge` | Aucun changement. Les badges paramétrés sur `actions_comptees` d'une action désactivée cessent de progresser — l'administration peut les repointer sur `jaime_recu`, `partage_recu` ou `cadeau_recu` sans migration. |
| `engagement.partage_externe` | Aucun changement de structure. La table cesse d'alimenter un seuil et devient une **trace statistique** par canal (FR-015). |
| `media_content.support_detenteur` | Aucun changement. Lu en lecture seule par `resoudre_beneficiaire` (R4). |

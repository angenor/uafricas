# Research — Refonte salles Afrolang

**Branch** : `001-afrolang-salles-refonte`
**Date** : 2026-04-15

Ce document résout les inconnues techniques restantes (issues des Assumptions A3/A5 de la spec et des contraintes de la Constitution).

---

## R1 — Stratégie de migration SQL : table rase legacy

**Decision** : modification in-place du fichier `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` avec `DROP TABLE` / `DROP COLUMN` pour les artefacts legacy. Pas de fichier de migration séparé : le projet n'a pas encore d'outil de migration versionné (cf. CLAUDE.md, init via `docker-init.sh`) et le produit n'est pas en production (décision Q2).

**Tables / colonnes supprimées** :

- `DROP TABLE afrolang.salle_privee_adhesion CASCADE` (mécanisme adhésion/invitation abandonné)
- `DROP TABLE afrolang.proposition_salle CASCADE` (création de salles publiques par utilisateurs abandonnée — admin-only désormais)
- `DROP TYPE afrolang.type_adhesion`, `DROP TYPE afrolang.etat_adhesion`, `DROP TYPE afrolang.etat_proposition`
- Sur `afrolang.salle_privee` : `DROP COLUMN motif`, `DROP COLUMN declaration_adulte_at`, `DROP COLUMN visibilite`, `DROP COLUMN code_acces` (remplacé par `code_acces_hash`)
- `DROP TYPE afrolang.motif_salle_privee`, `DROP TYPE afrolang.visibilite_salle_privee`

**Tables conservées telles quelles** : `afrolang.salle`, `afrolang.session` (XOR salle publique / salle privée toujours valide), `afrolang.session_participant`, `afrolang.tableau_blanc`, `afrolang.ressource_salle`, `afrolang.message_session`.

**Sort de `afrolang.salle_moderateur`** : conservée si elle servait aux salles publiques (transfert de modération en cours de session) ; supprimée uniquement si dédiée aux salles privées. Vérification : la table `salle_moderateur` référence `salle_id` (publique) → **conservée**.

**Rationale** : produit non encore en production (Q2 — A4 spec). Suppression dure plutôt que soft-deprecation pour réduire la dette immédiatement. Cohérent avec Principe V (Simplicité).

**Alternatives écartées** :

- Migration douce (génération de code auto + email aux auteurs legacy) — surcoût pour zéro utilisateur réel.
- Soft delete via flag `deprecated_at` — laisse du code mort, contraire au Principe V.
- Fichier de migration versionné (Flyway-like) — outil non en place, hors scope.

---

## R2 — Format du code secret

**Decision** :

- **Longueur** : 4 à 16 caractères (saisie clavier, conviviale à transmettre à l'oral).
- **Charset** : alphanumérique + symboles courants `[A-Za-z0-9!@#$%&*?-]`. Pas d'espaces, pas d'unicode étendu (évite confusions oral/clavier mobile).
- **Sensibilité à la casse** : oui (cohérent avec la pratique mots de passe ; bcrypt n'opère pas de normalisation).
- **Validation** : regex côté backend `^[A-Za-z0-9!@#$%&*?-]{4,16}$` ; validation HTML5 + Vue côté frontend pour feedback immédiat.

**Rationale** : équilibre entre sécurité (longueur ≥ 4 + entropie raisonnable + rate limit) et UX (assez court pour transmettre à l'oral, sans caractères ambigus comme les espaces). Pas d'exigence de complexité (majuscule + chiffre + symbole) car la protection principale repose sur la non-divulgation et le rate limit.

**Alternatives écartées** :

- Code purement numérique 6 chiffres (style OTP) : moins flexible pour l'auteur et trop similaire à un code SMS, source de confusion.
- Code généré par le système : retire le contrôle à l'auteur (qui veut souvent un code mémorable).
- Longueur min 8 + complexité forte : trop contraignant pour un échange en cercle restreint (le secret est court car oralement partagé).

---

## R3 — Stockage du code secret : bcrypt cost 10

**Decision** : `code_acces_hash CHAR(60) NOT NULL` (format bcrypt standard 60 caractères). Hash bcrypt cost **10** (et non 12 comme les mots de passe — voir rationale).

**Implémentation** :

- Crate `bcrypt` (déjà en dépendances pour les mots de passe utilisateurs).
- Fonction utilitaire dans `src/handlers/afrolang.rs` : `hasher_code_acces(code: &str) -> String` et `verifier_code_acces(code: &str, hash: &str) -> bool`.
- Hashage à la création et à la modification ; vérification à chaque tentative d'accès.

**Rationale du cost 10** :

- Cost 10 = ~100 ms de calcul (vs ~400 ms pour cost 12). Acceptable pour SC-006 (≤ 2 s).
- Le code secret est par nature à entropie faible (4-16 caractères, charset restreint) ; un cost 12 ne change pas drastiquement la résistance brute-force qui est principalement maîtrisée par le rate limit (R4).
- Cost 10 est le standard OWASP minimum recommandé pour bcrypt et reste robuste contre les attaques offline en cas de fuite BDD.
- Différence assumée avec cost 12 des mots de passe : les mots de passe sont haute entropie & rares à vérifier ; les codes salles sont basse entropie & vérifiés à chaque entrée.

**Alternatives écartées** :

- Stockage clair : violation directe Principe IV.
- SHA-256 simple : pas de salt, vulnérable aux rainbow tables.
- Argon2id : non présent dans les dépendances actuelles, ajout injustifié vu que bcrypt est suffisant et déjà utilisé.
- bcrypt cost 12 : surcoût latence pour bénéfice marginal sur secret faible-entropie.

---

## R4 — Rate limit sur vérification du code secret

**Decision** : 5 tentatives échouées par minute par couple (utilisateur connecté, salle privée). Au-delà : verrouillage 5 minutes, message « Trop de tentatives, réessayez dans quelques minutes ».

**Implémentation** :

- Table `afrolang.tentative_code_acces` (in-memory n'est pas viable car Actix-Web peut tourner multi-instance) :

```sql
CREATE TABLE afrolang.tentative_code_acces (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id UUID        NOT NULL REFERENCES afrolang.salle_privee(id) ON DELETE CASCADE,
    utilisateur_id  UUID        NOT NULL,
    tente_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    succes          BOOLEAN     NOT NULL DEFAULT FALSE
);
CREATE INDEX idx_afrolang_tentative_lookup
    ON afrolang.tentative_code_acces(salle_privee_id, utilisateur_id, tente_at DESC);
```

- À chaque tentative : compter les échecs des 60 dernières secondes pour le couple. Si ≥ 5 et dernière tentative < 5 min, refuser sans vérifier le hash. Sinon, vérifier et insérer une ligne (succès ou échec).
- Job de purge périodique non requis pour l'instant : la table est petite (purge manuelle après quelques mois si besoin).

**Rationale** :

- 5 tentatives/min = balance UX (un utilisateur honnête se trompe rarement plus de 2-3 fois) vs sécurité (limite massivement le brute-force).
- Granularité par utilisateur (et non par IP) car l'utilisateur est forcément connecté (FR-013) : l'identité authentifiée est plus fiable que l'IP (NAT, mobile).
- Stockage BDD plutôt que mémoire : compatible multi-instance et persistant entre redémarrages.

**Alternatives écartées** :

- Rate limit par IP : contournable via NAT, pénalise les utilisateurs derrière proxy partagé.
- Verrouillage permanent (style 3 essais Apple) : trop punitif pour un usage social ; demander à l'auteur de débloquer ajouterait de la friction.
- Pas de rate limit (juste hash bcrypt cost 12) : insuffisant pour codes 4 caractères (≤ 14M combinaisons → cassable hors-ligne).

---

## R5 — Cycle de vie d'une session live

**Decision** : la table `afrolang.session` reste structurellement inchangée. Une salle privée durable PEUT avoir 0..N sessions historiques + 0..1 session active (état `en_cours`). Démarrer une nouvelle session dans une salle privée existante = créer une nouvelle ligne `afrolang.session` avec `salle_privee_id` renseigné et `etat='en_cours'`.

**Règle d'unicité runtime** : au plus une session `en_cours` par `salle_privee_id` (vérification au démarrage côté handler ; pas de contrainte SQL UNIQUE car index partiel sur enum dynamique est complexe et la vérif handler est suffisante).

**Indépendance salle privée ↔ salle publique** (cf. Q5 spec) : aucune contrainte FK conditionnée à l'état de la salle publique. Démarrage autorisé même si `afrolang.salle.actif=true` mais aucune session publique en cours.

**Modérateur effectif d'une session privée** : `moderateur_id = salle_privee.cree_par` (auteur), simplification — pas de transfert de modération sur salles privées (cohérent avec absence du concept dans la refonte).

**Alternatives écartées** :

- Fusionner `salle_privee` et `session` (« une salle = une session, jetable ») : contredit Q1 (objet durable).
- Index partiel UNIQUE sur `etat='en_cours'` : SQL valide mais oblige à supprimer/marquer la ligne pour redémarrer ; vérif handler plus simple.

---

## R6 — Flux frontend pour la création / accès salle privée

**Decision** : réutilisation maximale des composants existants `SallePriveeCreateModal.vue` et `SallePriveeJoinModal.vue`, refactorisés pour n'exposer que :

- **CreateModal** : champs `titre` (text, required, 5-350) + `code_acces` (text, required, 4-16 chars regex) + `description` (textarea, optional, 0-1000) → POST `/api/afrolang/salles-privees`.
- **JoinModal** : champ unique `code_acces` (text, required) → POST `/api/afrolang/salles-privees/{id}/verifier-code` puis si succès, navigation vers `/afrolang/session/...` avec param indiquant la salle privée à démarrer/rejoindre.

**Composable** `useAfrolang.ts` : conserve les helpers existants pour salles publiques, **supprime** `listerAdhesions`, `accepterAdhesion`, `proposer*`, `*Moderateur*` (legacy), **ajoute** `creerSallePrivee`, `verifierCodeAcces`, `modifierCodeAcces`, `archiverSallePriveeParAuteur`.

**Bouton « Démarrer / Rejoindre »** sur `SalleCard` : libellé dynamique selon `salle.session_en_cours`. Action = navigation directe vers `/afrolang/session/{salle.id}` (la page session sait démarrer ou rejoindre selon l'état). Pas de modale de confirmation.

**Widget « Canal privé » (dropdown existant)** : aucun changement structurel, branchement de l'action « Rejoindre » sur l'ouverture de `SallePriveeJoinModal` (au lieu de l'ancien parcours adhésion). Branchement de l'action « Créer ma salle privée » (visible si l'utilisateur n'en a pas pour cette salle publique) sur `SallePriveeCreateModal`.

**Rationale** : minimise le diff frontend (Principe V), respecte Tailwind v4 pur (Principe VI — composants publics).

**Alternatives écartées** :

- Refonte UI complète des modales : surcoût pour zéro valeur ajoutée produit.
- Page intermédiaire de saisie code : contredit FR-007 (modale uniquement).

---

## Récapitulatif décisionnel

| Décision | Référence spec | Référence research |
|---|---|---|
| Table rase BDD legacy | A4 (Q2) | R1 |
| Code secret 4-16 chars alphanum + symboles | A5 | R2 |
| Hash bcrypt cost 10 | FR-015 + Principe IV | R3 |
| Rate limit 5 tentatives/min/user/salle | A3 | R4 |
| Session reste éphémère, salle privée durable | Q1 + Q5 | R5 |
| Réutilisation modales existantes | FR-007, FR-008 | R6 |

Toutes les inconnues techniques restantes sont résolues. Phase 1 peut démarrer.

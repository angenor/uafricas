# Phase 0 : Research

**Feature**: 001-ressources-fermeture-session
**Date**: 2026-05-24

Toutes les ambiguïtés produit ont été levées en phase `/speckit.clarify` (5 questions, cf. section `## Clarifications` de `spec.md`). Cette phase 0 ne lève donc que les **décisions techniques** restantes.

---

## Décision 1 : Cohabitation avec la table `ressource_salle` existante

- **Décision** : créer une **nouvelle table** `afrolang.ressource_contribuee`, en parallèle de `afrolang.ressource_salle` (livrée en feature `005-afrolang-salles`).
- **Rationale** : la table existante est **modérée** (workflow `en_attente_validation → publiee/refusee` pour les liens externes, ajout réservé aux modérateurs/admins de salle). La nouvelle table est **libre d'ajout par tout utilisateur authentifié**, publiée immédiatement, avec un cycle de vie différent (en particulier le sous-type `accompagnateur` avec consentement explicite). Fusionner les deux exigerait un schéma à colonnes polymorphes et un `etat` à 7+ valeurs ambiguës, au prix de la clarté de modération côté admin. Cohérent avec Principe V (« 3 lignes dupliquées valent mieux qu'une abstraction non justifiée »).
- **Alternatives rejetées** :
  - Étendre `ressource_salle` avec un `etat='contribuee'` et un nouveau type `accompagnateur` → produit une table fourre-tout, casse la sémantique audit existante, force la migration des handlers existants.
  - Une vue UNION ALL pour le frontend → masque la différence sémantique, complique les agrégations et les `DELETE` cibles.

## Décision 2 : Périmètre de validation YouTube

- **Décision** : whitelist regex unique acceptant `https?://(www\.|m\.)?(youtube\.com|youtu\.be)/...` avec extraction de l'**ID 11 caractères** depuis `watch?v=`, `youtu.be/`, `embed/` et `shorts/`. ID stocké séparément (`video_id_youtube`) pour reconstruire un embed canonique côté frontend (`https://www.youtube.com/embed/<id>`).
- **Rationale** : Q2 de `/speckit.clarify` a fixé YouTube uniquement. Stocker l'ID en plus de l'URL d'origine permet (a) un rendu d'embed stable indépendant du format d'URL collé par l'utilisateur, (b) une vignette `https://i.ytimg.com/vi/<id>/hqdefault.jpg` sans appel API. Aucune dépendance externe.
- **Alternatives rejetées** :
  - oEmbed / API YouTube Data → introduit clé API à gérer + quotas, non nécessaire pour un simple embed.
  - Stocker uniquement l'URL → contraint le frontend à re-parser à chaque rendu.

## Décision 3 : Mémorisation de l'accès aux ressources des salles privées (FR-001 option C)

- **Décision** : nouvelle table `afrolang.acces_salle_privee (salle_privee_id, utilisateur_id, valide_at, revoque_at)` avec **UNIQUE partial index** `(salle_privee_id, utilisateur_id) WHERE revoque_at IS NULL`. À chaque succès de `POST /salles-privees/{id}/verifier-code`, on fait `INSERT ... ON CONFLICT DO NOTHING` (idempotent). À chaque `PATCH /salles-privees/{id}/code-acces` (changement de code), on fait `UPDATE acces_salle_privee SET revoque_at = NOW() WHERE salle_privee_id = $1 AND revoque_at IS NULL` dans la transaction.
- **Rationale** : persistance simple, requête de contrôle d'accès en O(1) via index, révocation atomique. Cohérent avec FR-001 (option C de la clarification) qui exige une mémorisation indépendante du JWT 4 h et révocable au changement de code.
- **Alternatives rejetées** :
  - Stocker la liste des `acces` dans un JSONB sur `salle_privee` → croissance non bornée, requêtes plus coûteuses.
  - Ré-utiliser la table `tentative_code_acces` (rate-limit) → mélange deux responsabilités (audit vs autorisation), risque de purge accidentelle.

## Décision 4 : Rate limit ressources contribuées

- **Décision** : compter à la volée `SELECT COUNT(*) FROM ressource_contribuee WHERE auteur_id = $1 AND salle_id = $2 AND created_at > NOW() - INTERVAL '24 hours' AND deleted_at IS NULL`. Bloquer à ≥ 10.
- **Rationale** : volume attendu très faible par utilisateur, COUNT borné par l'index `(salle_id, auteur_id, created_at)`. Pas besoin de table dédiée (YAGNI Principe V). La feature `001-afripulse-contributions` a déjà adopté ce pattern (service `rate_limit_afripulse`).
- **Alternatives rejetées** :
  - Redis token bucket → introduit une nouvelle dépendance d'infrastructure.
  - Table dédiée `rate_limit_ressource` → réplique l'information déjà présente, complexifie la suppression.

## Décision 5 : Fermeture LiveKit lors de la fermeture admin

- **Décision** : étendre `services/livekit_moderation.rs` avec une fonction `fermer_session_admin(room_name, motif_court)` qui (1) diffuse un DataPacket RELIABLE `{type:'admin', subtype:'session_fermee', motif_public:'fermeture par administration'}` à toute la room via `RoomServiceClient::send_data`, puis (2) appelle `RoomServiceClient::delete_room(room_name)` pour forcer la déconnexion des participants côté SFU.
- **Rationale** : le crate `livekit-api` est déjà importé et étendu en `001-session-moderation`. `delete_room` éjecte tous les participants ; le data packet préalable garantit que le toast `SessionFermeeAdminToast.vue` se déclenche côté frontend avant la déconnexion réseau. Pas de WebSocket dédié, pas de pull côté frontend.
- **Alternatives rejetées** :
  - `RemoveParticipant` boucle sur chaque participant → N appels au lieu d'un, latence cumulée.
  - Compter sur la simple mise à jour BDD `etat='terminee'` et laisser les frontends détecter par polling → pas conforme à SC-005 (< 5 s).

## Décision 6 : Notifications participants éjectés (Q5)

- **Décision** : à la fermeture admin, requêter `SELECT DISTINCT utilisateur_id FROM afrolang.session_participant WHERE session_id = $1 AND quitte_at IS NULL` pour obtenir la liste des participants présents au moment précis de la coupure. Pour chacun, insérer une notification de type `afrolang.session.fermee_admin` (sans motif détaillé) via le mécanisme `services/notifications.rs` existant. Pour les admins de salle / créateur, type distinct `afrolang.salle.desactivee_admin` avec motif détaillé inclus.
- **Rationale** : la table `session_participant` est déjà alimentée par la jointure LiveKit ; la requête est O(N) avec N ~ 10-50 typique. Deux types de notifications distincts permettent un rendu différencié et un filtrage côté audit.
- **Alternatives rejetées** :
  - Notifier tous les contributeurs de ressources → bruit (B rejetée en Q5).
  - Notifier toute personne ayant rejoint dans les 30 derniers jours → volume potentiellement très élevé, spam (D rejetée en Q5).

## Décision 7 : Helper `est_admin_plateforme`

- **Décision** : réutiliser le helper d'autorisation existant (`handlers/admin/mod.rs` ou équivalent, déjà appelé dans tous les handlers `/api/admin/*`). Aucun helper additionnel n'est nécessaire pour la fermeture/réactivation puisque ces endpoints sont sous `/api/admin/afrolang/sessions-moderation/...`.
- **Rationale** : Principe V (YAGNI) et II (cohérence monorepo).
- **Alternatives rejetées** : créer un sous-rôle « modérateur abus » dédié → out-of-scope, non demandé par la spec.

## Décision 8 : Workflow accompagnateur sans expiration automatique

- **Décision** : pas de tâche planifiée (cron / `tokio::spawn` périodique) pour expirer les recommandations `en_attente` au bout de N jours. Elles restent en attente indéfiniment jusqu'à action de la personne recommandée ou suppression par l'auteur (FR-006).
- **Rationale** : la spec ne demande pas d'expiration. Pas d'infrastructure de scheduling dans le projet ; en introduire une pour un seul cas d'usage serait disproportionné (YAGNI).
- **Alternatives rejetées** : tokio interval avec purge à J+30 → introduit du code de fond, surveillance, risque de purge en cas d'inactivité légitime de la personne recommandée.

## Décision 9 : Limites stockage upload

- **Décision** : sous-dossier dédié `./uploads/afrolang/ressources_contribuees/<uuid-ressource>/<nom_sanitize>.<ext>`. Validation MIME via le content-type multipart + double-check de l'extension (whitelist `.pdf`, `.doc`, `.docx`, `.odt`). Limite 20 Mo enforced via `actix_multipart::Multipart` configuration et taille calculée à la lecture du stream (fail-fast).
- **Rationale** : aligné sur le pattern existant `uploads/couvertures/`, `uploads/documents/`, `uploads/videos/`, `uploads/afrolang/ressources/` (modération salle). Pas de stockage cloud → conforme contrainte constitutionnelle « Stockage local sans migration approuvée ».
- **Alternatives rejetées** : S3/MinIO → out-of-scope.

## Décision 10 : Représentation TS / DTO

- **Décision** : type discriminé côté TS `Ressource = DocumentRes | VideoRes | LienRes | AccompagnateurRes` avec champ `type` discriminant (string union literal). Côté Rust, struct unique `RessourceContribuee` avec champs `Option<...>` correspondant aux colonnes nullables, et un enum `TypeRessourceContribuee` pour le discriminant. DTOs séparés `RessourceContribueeResponse` (lecture publique) et `RessourceContribueeAdminResponse` (lecture admin incluant motifs internes).
- **Rationale** : conforme au pattern existant du projet (`ApiResponse<T>`, DTOs Response séparés). Le discriminé TS facilite le rendu conditionnel dans `RessourceContribueeCard.vue`.
- **Alternatives rejetées** : une table par sous-type → multiplie les jointures, casse la simplicité YAGNI.

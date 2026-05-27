# Phase 0 — Recherche & décisions techniques

Feature : Événements en streaming direct (LiveKit). Toutes les questions ouvertes de la spec ont été résolues lors de `/speckit.clarify` ; ce document fige les décisions d'architecture qui en découlent, ancrées sur le code existant.

## D1 — Infrastructure de streaming : réutiliser LiveKit (afrolang)

- **Décision** : réutiliser LiveKit, déjà câblé côté backend (`config.rs::LivekitConfig`, deps `livekit-api` 0.4 / `livekit-protocol` 0.7, service `services/livekit_moderation.rs`, env `LIVEKIT_URL`/`LIVEKIT_API_KEY`/`LIVEKIT_API_SECRET`) et côté frontend (`livekit-client` ^2.17.1 déjà installé).
- **Rationale** : Principe V (YAGNI) — aucune nouvelle infra ; le pattern de génération de token afrolang (`AccessToken::with_api_key().with_identity().with_name().with_grants(VideoGrants{…}).to_jwt()`) est directement transposable. L'URL et le token LiveKit sont fournis **par le backend** dans la réponse (afrolang ne met aucune config LiveKit côté frontend) → rien à ajouter dans `nuxt.config.ts`.
- **Alternatives rejetées** : PeerJS/WebRTC P2P (utilisé pour rendez-vous 1-à-1) ne passe pas à l'échelle pour une diffusion 1→N à ≥100 spectateurs ; service externe (Zoom/YouTube) contredit l'exigence « directement sur la plateforme ».

## D2 — Modèle webinaire : `can_publish` scopé par rôle

- **Décision** : le token LiveKit porte des grants différenciés par rôle :
  - **organisateur / intervenant** : `room_join: true, can_publish: true, can_subscribe: true, can_publish_data: true`
  - **spectateur** : `room_join: true, can_publish: false, can_subscribe: true, can_publish_data: true`
- **Rationale** : c'est la différence clé avec afrolang (qui donne `can_publish: true` à tous, sans mode spectateur). `can_subscribe: true` permet de regarder ; `can_publish: false` empêche la diffusion média non sollicitée. `can_publish_data: true` pour **tous** afin que chat, réactions et lever-la-main circulent en DataPackets sans backend (cf. D4). Le SFU LiveKit applique ces droits côté serveur (sécurité — Principe IV).
- **Alternatives rejetées** : gating applicatif côté client uniquement (contournable) ; mode interactif type afrolang (rejeté en clarification — inadapté aux grandes audiences).

## D3 — Promotion / rétrogradation / retrait : via API serveur LiveKit

- **Décision** : la promotion d'un spectateur en intervenant met à jour son `can_publish` côté SFU via `RoomClient.update_participant` (clé API serveur requise) + met à jour `role` en base + diffuse un DataPacket `{type:'moderation', subtype:'role_update'}`. Le retrait (« kick ») utilise `RoomClient.remove_participant`.
- **Implémentation** : étendre `services/livekit_moderation.rs` avec deux fonctions (le service actuel ne gère que `can_publish_data`, `delete_room` et `send_data`) :
  - `update_participant_can_publish(cfg, room_name, identity, autorise: bool)` — `ParticipantPermission { can_subscribe: true, can_publish: autorise, can_publish_data: true }`
  - `retirer_participant(cfg, room_name, identity)` — `RoomClient.remove_participant`
- **Rationale** : changer une permission LiveKit exige l'API key serveur → doit passer par le backend (le client ne peut pas s'auto-promouvoir, Principe IV). On **n'altère pas** les fonctions afrolang existantes (isolation, pas de régression).
- **Alternatives rejetées** : régénérer un token et reconnecter le participant (UX cassante, coupure vidéo) ; réutiliser `update_participant_can_publish_data` tel quel (ne touche pas `can_publish`).

## D4 — Chat / réactions / lever-la-main : DataPackets LiveKit éphémères

- **Décision** : chat texte et réactions circulent **uniquement** en DataPackets LiveKit client-à-client (aucun stockage, aucun endpoint). Le « lever la main » envoie un DataPacket **et** appelle un endpoint backend léger qui pose un flag `main_levee` sur le participant, afin que l'organisateur dispose d'une liste fiable même s'il a manqué le DataPacket.
- **Rationale** : la spec impose chat/réactions **éphémères** (FR-007/008, non archivés — cohérent avec l'absence d'enregistrement). Les DataPackets respectent le délai < 2 s (SC-006) sans charge serveur. afrolang utilise déjà ce canal (`{type:'reaction',emoji}`, `{type:'moderation',subtype:…}`) — pattern réutilisé. La persistance du seul `main_levee` (FR-022) garantit la fiabilité de la liste de demandes côté organisateur.
- **Alternatives rejetées** : chat persistant en base (contredit « éphémère », ajoute table + endpoints inutiles — YAGNI) ; lever-la-main purement DataPacket (risque de perte si l'organisateur rejoint après le signal).

## D5 — Cycle de vie de la session : une session active par événement, états dérivés

- **Décision** : table `media_content.evenement_session` 1-à-N avec l'événement, mais **une seule session `en_cours` à la fois** garantie par un index unique partiel `WHERE etat = 'en_cours'` (FR-015). États persistés : `en_cours`, `terminee`. L'état « en attente de l'organisateur » est **dérivé à la lecture** (aucune session active + on est dans la fenêtre) — pas de valeur persistée, cohérent avec la dérivation du `statut` événement existante (`calculer_statut`) et des rendez-vous.
- **Rationale** : Principe V — éviter une machine à états superflue. L'ouverture crée directement une session `en_cours` ; la clôture la passe `terminee` et calcule `duree_secondes`. Le pattern calque `afrolang.session` (`demarre_at`/`termine_at`/`duree_secondes`/`noeud_id`/`nombre_participants_pic`).
- **Alternatives rejetées** : enum à 4 états type afrolang (`planifiee`/`annulee` inutiles ici — l'annulation passe par `evenement.etat='annule'`).

## D6 — Fenêtre temporelle & arrêt de sécurité sans cron

- **Décision** :
  - Ouverture autorisée à partir de **15 min avant** `date_heure_debut`.
  - Une fois ouverte, la session reste joignable tant que l'organisateur ne clôture pas (même au-delà de `date_heure_fin`) — clarification « continue jusqu'à clôture ».
  - **Arrêt de sécurité absolu** : `arret_securite_at` = `date_heure_fin + 2h` (ou `date_heure_debut + duree_defaut + 2h` à défaut de fin), **stocké à l'ouverture**. Appliqué **paresseusement** : à chaque lecture/jointure, si `NOW() > arret_securite_at` et session `en_cours`, la session est clôturée (UPDATE `terminee` + best-effort `delete_room`).
- **Rationale** : la constitution interdit implicitement la sur-ingénierie ; le projet n'a pas de cron et dérive déjà « expiré/terminé » par calcul (rendez-vous, événements). L'application paresseuse suffit (un direct fantôme consomme l'infra LiveKit mais le SFU recycle les rooms inactives ; la clôture logique se fait au prochain accès).
- **Alternatives rejetées** : tâche planifiée/cron (nouvelle dépendance opérationnelle, hors pattern projet) ; fermeture dure à `date_heure_fin` (rejeté en clarification — empêche les dépassements légitimes).

## D7 — Éligibilité & accès

- **Décision** : à l'ouverture comme à la jointure, revérification serveur :
  - **organisateur** = `evenement.cree_par` (seul habilité à ouvrir/clôturer/modérer) ;
  - **spectateur/intervenant** = utilisateur **inscrit** (`evenement_inscription.statut != 'annule'`) via le helper `est_inscrit` existant ;
  - non-inscrit → 403 avec message « Inscrivez-vous d'abord » ; non connecté → 401.
- **Rationale** : clarification « inscrits + organisateur » ; réutilise la logique d'inscription et l'auth JWT in-handler (`extraire_utilisateur_id`) déjà présentes dans `handlers/evenements.rs` (Principe IV/V).
- **Alternatives rejetées** : accès public anonyme (rejeté en clarification) ; co-animateurs pré-désignés (rejeté — promotion à la volée uniquement).

## D8 — Capacité

- **Décision** : `evenement_session.max_participants` (défaut 100, cohérent SC-004). À la jointure, compter les participants actifs (`quitte_at IS NULL`) ; si `>= max_participants` → **refus** HTTP 409 avec message « Capacité atteinte, réessayez plus tard » (clarification — pas de file d'attente). L'organisateur n'est pas compté dans la limite (toujours admis).
- **Rationale** : clarification « refuser avec message ». Compteur simple en base, pas d'état d'attente.

## D9 — Notifications & temps réel

- **Décision** : à l'ouverture du direct, notifier chaque inscrit via `models::notification::creer_notification(pool, inscrit_id, "evenement_direct_demarre", message, Some(lien))` (cloche persistante) + pousser un SSE `{type:"event_stream_demarre", evenement_id}` à chaque inscrit via `RegistreSse::publier`. Le frontend dispatch via une nouvelle branche `evt.type.startsWith('event_stream_')` dans `plugins/messagerie.client.ts` + `compteurNonLues()`.
- **Rationale** : réutilise le système cloche unifié (`arbre_genealogique.notifications`) et le flux SSE messagerie existants (pattern `rdv_*`). Lien d'action = `/evenements/{id}`.
- **Alternatives rejetées** : email (hors périmètre MVP, plus lourd) ; nouveau canal SSE dédié (le flux messagerie unique suffit — Principe V).

## D10 — Repli en cas d'indisponibilité du streaming (FR-023)

- **Décision** : le frontend capture toute erreur d'obtention de token ou de `room.connect(...)` et affiche un message clair + bouton « Réessayer » ; si `evenement.lien_en_ligne` est renseigné, il est proposé en repli (« Rejoindre via le lien externe »). Côté backend, l'échec d'appel LiveKit (modération) reste **non bloquant** et journalisé (pattern existant `livekit_moderation`), Postgres restant la source de vérité.
- **Rationale** : `lien_en_ligne` existe déjà sur l'événement ; dégradation gracieuse à coût nul. Cohérent avec l'esprit « repli » des rendez-vous.

## Synthèse des points de réutilisation

| Besoin | Réutilisé tel quel | À étendre / créer |
|--------|--------------------|-------------------|
| Token LiveKit | Pattern `AccessToken` (afrolang) | grants scopés par rôle (nouveau handler) |
| Modération SFU | `fermer_session_admin`, `publier_evenement_moderation` | + `update_participant_can_publish`, `retirer_participant` |
| Auth & éligibilité | `extraire_utilisateur_id`, `est_inscrit`, `cree_par` | revérif à chaque action |
| Persistance session | DDL `afrolang.session`/`session_participant` (calque) | tables `evenement_session(_participant)` |
| Notifications | `creer_notification`, `RegistreSse::publier` | types `evenement_direct_demarre` / `event_stream_demarre` |
| Audit | `audit::log_action` | actions ouvrir/clôturer/promouvoir/… |
| Frontend salle | patterns `livekit-client` d'`AfrolangRoom.vue` | composants `evenements/` rôle-aware (non couplés afrolang) |
| Composable | structure `useEvenements` / `useRendezVous` | fonctions direct + état `useState` |

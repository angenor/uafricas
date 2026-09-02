# Phase 0 : Research : Rendez-vous en visioconférence entre amis

Toutes les décisions techniques de la spec étaient déjà cadrées ; ce document les consolide en s'appuyant sur l'exploration du code social existant et lève les derniers points (peer-id, intégration cloche, glare WebRTC). Aucun `NEEDS CLARIFICATION` ne subsiste.

## Décision 1 : Schéma de données : nouvelle table `social.rendez_vous`

- **Décision** : ajouter `social.rendez_vous` + enum `social.statut_rendez_vous` dans un nouveau fichier `schemas/31_social_rendez_vous.sql`, intégré à l'orchestrateur `schema.sql` (`\ir schemas/31_social_rendez_vous.sql`, juste après la ligne `30_social_conversation_annonce.sql`). Migration idempotente (`CREATE … IF NOT EXISTS`, `DO $$ … EXCEPTION WHEN duplicate_object` pour l'enum).
- **Rationale** : Principe III (SQL source de vérité) + bounded-context `social` déjà hôte de l'amitié/messagerie. Numéro 31 = suivant logique après 29/30.
- **Alternatives rejetées** : réutiliser `social.demande_amitie` (sémantique incompatible) ; nouveau schéma `meeting` (Principe V, un domaine social suffit).

## Décision 2 : Statuts persistés vs dérivés

- **Décision** : enum persisté = `('propose', 'accepte', 'refuse', 'annule')`. Les notions **« expiré »** (propose dont la date est passée) et **« terminé/passé »** (accepte dont la fenêtre est écoulée) sont **dérivées par calcul** (statut + `date_heure`/durée + `NOW()`), sans valeur d'énumération ni tâche planifiée.
- **Rationale** : clarifications Q1 + Q2 ; Principe V (pas de job de fond). Le filtre « passés » de la vue de gestion combine : refusés, annulés, proposés expirés, acceptés dont la fenêtre est écoulée.
- **Alternatives rejetées** : statut `expire`/`termine` persisté via cron (complexité injustifiée).

## Décision 3 : « Tour » de réponse (négociation)

- **Décision** : colonne `tour_id` (UUID, référence un des deux participants) = la partie qui doit répondre. À la création, `tour_id = destinataire_id`. À chaque contre-proposition, `tour_id` bascule vers l'autre. Une réponse (accepter/refuser/contre-proposer) n'est autorisée que si `tour_id = utilisateur_courant` ET `statut = 'propose'`.
- **Rationale** : modélise directement FR-015/FR-017 ; évite un champ « dernier acteur » ambigu.
- **Alternatives rejetées** : déduire le tour du nombre de contre-propositions (fragile, non explicite).

## Décision 4 : Identité des parties & affichage

- **Décision** : conserver `initiateur_id` (créateur d'origine) et `destinataire_id` (cible d'origine) figés ; l'« autre membre » affiché = celui des deux ≠ utilisateur courant, résolu via `obtenir_membre_light` (réutilisé de `models/amitie.rs`). Pas d'ordre canonique requis (les deux ids sont explicites).
- **Rationale** : `MembreLight` (id, nom, prenom, slug, photo_url, fonction, pays) couvre exactement FR-020. Réutilisation directe (Principe II/V).
- **Alternatives rejetées** : ordre canonique a<b (utile pour l'unicité de paire, inutile ici car plusieurs RDV par paire autorisés).

## Décision 5 : Notifications cloche : système unifié `arbre_genealogique.notifications`

- **Décision** : utiliser le **système de cloche unifié existant** (`arbre_genealogique.notifications`, type `VARCHAR` libre) via `models::notification::creer_notification(pool, destinataire_id, type, message, lien_action)`. Types : `rdv_propose`, `rdv_accepte`, `rdv_refuse`, `rdv_contre_propose`, `rdv_annule`. `message` = libellé générique en français avec le nom de l'acteur (PAS le sujet/description, prudence). `lien_action` = ancre ouvrant le panneau messagerie sur l'onglet rendez-vous.
- **Rationale** : la cloche réellement affichée (`ClocheNotifications.vue` → `useNotifications` → `/api/notifications`) lit `arbre_genealogique.notifications`. C'est LE canal cloche unifié. `social.notification` est spécifique amitié (FK `demande_id`, enum fermé), l'étendre forcerait une migration d'enum et resterait invisible de la cloche. Réutiliser le canal unifié = moins de surface, conforme à l'intention de la spec (« réutiliser le système de notifications existant »).
- **Alternatives rejetées** : étendre l'enum `social.type_notification_social` + ajouter `rendez_vous_id` à `social.notification` (canal non branché à la cloche → notifications invisibles).

## Décision 6 : Temps réel SSE & rafraîchissement cloche

- **Décision** : pousser des événements typés via `RegistreSse.publier(utilisateur_id, &evt)` avec la forme `{ "type": "rdv_propose" | "rdv_accepte" | "rdv_refuse" | "rdv_contre_propose" | "rdv_annule", "rendez_vous_id": "<uuid>" }` (sans contenu sensible). Étendre `plugins/messagerie.client.ts` pour : (a) transmettre l'événement à `useRendezVous().gererEvenement(evt)` (rafraîchit la liste si le panneau est ouvert) et (b) sur tout `type` commençant par `rdv_`, appeler `useNotifications().compteurNonLues()` pour rafraîchir le badge de la cloche.
- **Rationale** : réutilise le flux SSE unique `/api/messagerie/flux?token=` (un seul EventSource global). Comble le fait que la cloche n'est aujourd'hui pas rafraîchie par SSE (constat exploration) avec un changement minimal et localisé.
- **Alternatives rejetées** : second EventSource dédié aux RDV (doublon de connexion, Principe V) ; polling périodique de la cloche (latence, charge).

## Décision 7 : Visioconférence P2P : PeerJS + peer-id déterministe

- **Décision** :
  - Bibliothèque **PeerJS** (`pnpm add peerjs`), client uniquement. Hôte de signalisation par défaut = cloud public PeerJS (`0.peerjs.com`), surchargé par `runtimeConfig.public` (`peerjsHost`, `peerjsPort`, `peerjsPath`, `peerjsSecure`).
  - **Peer-id déterministe** calculé côté **backend** et renvoyé par l'endpoint `/salle` : `mon_peer_id` et `pair_peer_id` = `format!("uafr-{}", hex(sha256(rendez_vous_id || participant_id)))` (tronqué). Chaque côté connaît donc l'id à appeler sans échange via le backend applicatif (la signalisation transite par le cloud PeerJS).
  - **Anti-glare (qui appelle qui)** : `suis_appelant = (utilisateur_courant_id < autre_id)` (comparaison d'UUID). Le plus petit UUID initie l'appel (`peer.call`), l'autre attend (`peer.on('call')`). Évite la double initiation.
  - **ICE** : liste `iceServers` depuis `runtimeConfig.public.iceServers` (défaut : STUN Google `stun:stun.l.google.com:19302` + secours). Aucun TURN dans ce lot.
- **Rationale** : conforme aux décisions de la spec. Le hachage évite d'exposer en clair les UUID dans l'id de pair tout en restant déterministe. L'UUID du rendez-vous (inconnu des non-participants, renvoyé seulement par l'API authentifiée) sert de secret d'accès.
- **Limite documentée** : sans TURN, ~15 % des connexions derrière NAT symétrique échoueront → état d'échec explicite + repli messagerie privée (FR-029). La liste ICE reste configurable pour brancher un TURN ultérieurement.
- **Alternatives rejetées** : LiveKit (déjà présent pour afrolang, mais SFU = média transitant par un serveur, contraire à l'objectif P2P/0-bande-passante) ; `simple-peer` + signalisation maison via SSE backend (plus de code de signalisation, Principe V).

## Décision 8 : Fenêtre d'ouverture de la salle

- **Décision** : bouton « Rejoindre » actif si `statut = 'accepte'` ET `NOW() ∈ [date_heure − 5 min, date_heure + duree_minutes + 15 min]`. Calcul partagé : le backend renforce la fenêtre dans `/salle` (rejette hors fenêtre), le frontend la calcule aussi pour l'état d'activation du bouton.
- **Rationale** : FR-024 ; valeurs par défaut de la spec (Assumptions). Double contrôle (UI + serveur) = sécurité par défaut.
- **Alternatives rejetées** : fenêtre purement frontend (contournable).

## Décision 9 : Concurrence (verrouillage optimiste)

- **Décision** : chaque mutation s'exécute dans une transaction avec une mise à jour conditionnelle : `UPDATE social.rendez_vous SET … WHERE id = $1 AND statut = 'propose' AND tour_id = $moi AND deleted_at IS NULL` (ou `statut = 'accepte'` pour l'annulation). Si `rows_affected = 0`, renvoyer un conflit clair (« ce rendez-vous a déjà été modifié »).
- **Rationale** : clarification Q4 (verrouillage optimiste) ; pas de verrou pessimiste (Principe V).
- **Alternatives rejetées** : `SELECT … FOR UPDATE` systématique (surdimensionné pour la volumétrie).

## Décision 10 : Validations & audit

- **Décision** : validations backend = sujet `1..150` car, durée ∈ {15,30,45,60}, `date_heure > NOW()` (à la création et à la contre-proposition), pas soi-même, amitié active + absence de blocage (helpers `amitie_existe` / `blocage_existe` réutilisés). Audit `log_action` (schema `social`, table `rendez_vous`) sur chaque mutation, `nouvel_etat` = identifiants/statut **sans** sujet ni description (FR-033).
- **Rationale** : FR-006..FR-010, FR-033, FR-034 ; réutilisation des helpers existants.
- **Alternatives rejetées** : validations uniquement frontend (Principe IV).

## Dépendances à ajouter

| Côté | Dépendance | Commande / action |
|------|-----------|-------------------|
| Frontend | `peerjs` | `pnpm add peerjs` (dans `uafricas_frontend/`) |
| Frontend | variables runtime | `nuxt.config.ts` → `runtimeConfig.public` : `peerjsHost/Port/Path/Secure`, `iceServers` |
| Backend | `sha2` | déjà présent (utilisé par JWT/refresh), réutilisé pour le peer-id |

Aucune nouvelle dépendance backend (sha2, uuid, chrono, serde, sqlx déjà présents).

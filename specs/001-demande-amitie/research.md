# Research : Demande d'amitié & messagerie temps réel

**Feature**: `001-demande-amitie` | **Date**: 2026-05-24

Ce document résout les inconnues techniques de la Technical Context et fige les décisions structurantes avant la conception détaillée.

---

## Décision 1 : Schéma de données dédié `social`

**Decision**: Créer un **nouveau schéma PostgreSQL `social`** (fichier `schemas/29_social.sql`) regroupant les relations sociales et la messagerie : `demande_amitie`, `amitie`, `blocage`, `conversation`, `message`, `notification`.

**Rationale**:
- Le projet suit une architecture **bounded-context** (10+ schémas : `iam`, `afrolang`, `retrouve_amis`, `arbre_genealogique`…). L'amitié et la messagerie privée forment un domaine cohérent et autonome.
- `iam` est réservé à l'identité/accès (utilisateurs, rôles, permissions), y mêler amitié/messagerie le polluerait.
- `retrouve_amis` traite du **matching** (correspondances algorithmiques avis↔profil), concept distinct de l'amitié explicite ; ne pas réutiliser pour éviter la confusion sémantique.
- Le schéma `exchange` est dédié aux échanges du marché, sans rapport.

**Alternatives rejetées**:
- *Rattacher à `iam`* : viole la séparation des responsabilités ; `iam` ne porte aucune donnée relationnelle inter-utilisateurs aujourd'hui.
- *Réutiliser `retrouve_amis`* : couplerait deux domaines aux cycles de vie différents.

**Conformité Principe III** : le schéma SQL est écrit en premier ; structs Rust et types TS en découleront.

---

## Décision 2 : Transport temps réel : SSE (Server-Sent Events)

**Decision**: Diffusion serveur→client via **SSE** sur un endpoint `GET /api/messagerie/flux`, alimenté par un **registre de connexions en mémoire** (mono-instance). L'**envoi** d'un message se fait par un `POST` REST classique qui persiste en base puis pousse l'évènement aux connexions SSE concernées.

**Rationale**:
- Le besoin est un **push unidirectionnel** serveur→client (recevoir un message, un signal de non-lu) ; l'envoi reste une mutation REST normale. SSE couvre exactement ce besoin, plus simple qu'un WebSocket bidirectionnel (Principe V, YAGNI).
- Aucune nouvelle dépendance : `actix-web` sait renvoyer un flux (`HttpResponse::streaming`), `tokio` (full) et `futures-util` sont déjà présents.
- Le déploiement de production est **mono-backend** (`docker-compose.prod.yml`) : un registre en mémoire `HashMap<utilisateur_id, Vec<Sender>>` suffit pour le fan-out.
- Satisfait SC-008 (< 2 s) : push immédiat à la persistance.

**Alternatives rejetées**:
- *LiveKit (data channels)* : orienté **salles** audio/vidéo éphémères (utilisé par Afrolang). Inadapté à une messagerie 1-1 **persistante et globale** : imposerait une room par paire et ne gère pas la persistance ni l'historique.
- *WebSocket (actix-ws)* : bidirectionnel non nécessaire ; ajoute une dépendance et de la complexité de protocole.
- *Polling REST* : plus simple encore mais dégrade la latence et multiplie les requêtes ; SSE est un meilleur compromis sans surcoût d'infra.

**Contrainte documentée (scale)**: le registre en mémoire suppose **une seule instance backend**. Une montée en charge multi-instance nécessiterait un bus (PostgreSQL `LISTEN/NOTIFY` ou Redis pub/sub), **hors périmètre** actuel, à réévaluer si scaling horizontal.

---

## Décision 3 : Authentification du flux SSE

**Decision**: L'endpoint SSE accepte l'**access token JWT en paramètre de requête** (`/api/messagerie/flux?token=<jwt>`), validé par le même mécanisme que l'en-tête `Authorization`. La reconnexion (token expiré à 15 min) est gérée côté client en rafraîchissant le token puis en rouvrant le flux.

**Rationale**:
- L'API navigateur `EventSource` ne permet pas d'en-têtes personnalisés ; le token en query est le pattern usuel.
- L'access token est **court (15 min)** et l'endpoint est en lecture seule, exposition limitée. Recommandation : ne pas journaliser les query strings de cet endpoint.

**Alternatives rejetées**:
- *Jeton de flux éphémère dédié* (comme le jeton LiveKit d'Afrolang) : plus sûr mais ajoute un endpoint et de la complexité ; non justifié pour ce périmètre (Principe V).

---

## Décision 4 : Ordre canonique des paires

**Decision**: Pour `amitie` et `conversation`, stocker la paire d'utilisateurs en **ordre canonique** (`utilisateur_a_id < utilisateur_b_id`) avec une contrainte d'unicité `(utilisateur_a_id, utilisateur_b_id)`. Pour `blocage`, conserver l'orientation (`bloqueur_id`, `bloque_id`).

**Rationale**: l'amitié et la conversation sont **symétriques** ; l'ordre canonique garantit une unique ligne par paire et simplifie les requêtes d'existence. Le blocage est **orienté** (A bloque B ≠ B bloque A).

---

## Décision 5 : Notifications relationnelles

**Decision**: Table `social.notification` (type `demande_recue` | `demande_acceptee`, `lu` booléen), suivant le pattern per-domaine déjà établi (`retrouve_amis.notification_retrouve`, `iam.notification_biblio_humaine`). Exposée via des endpoints de liste + marquage lu.

**Rationale**: le projet n'a pas de centre de notifications **unifié** ; chaque domaine porte sa table. Reproduire ce pattern « réutilise le mécanisme existant » au sens de la convention, sans introduire d'infrastructure nouvelle.

**Portée (clarification Q5)**: seuls les évènements **de relation** (demande reçue, demande acceptée) créent une notification. Les **nouveaux messages** ne créent **pas** de notification persistante : ils sont signalés par l'indicateur du bouton flottant uniquement.

---

## Décision 6 : Limite anti-spam (FR-014)

**Decision**: Plafonner à **30 demandes d'amitié envoyées par 24 h glissantes** par demandeur, vérifié par un `COUNT` sur `social.demande_amitie` (created_at > now() - 24h). Dépassement → refus `429`.

**Rationale**: valeur raisonnable couvrant un usage normal tout en bloquant le harcèlement ; pas de table dédiée nécessaire (Principe V). Seuil ajustable par constante.

---

## Décision 7 : État global frontend & cycle SSE

**Decision**: Domaine exposé par **deux composables** (`useAmis`, `useMessagerie`) + un **plugin client** (`messagerie.client.ts`) qui ouvre le flux SSE après authentification et alimente un état global `useState` (compteur de non-lus, conversations). Pas de nouveau store Pinia.

**Rationale**: respecte la convention « un composable par domaine » du projet ; `useState` Nuxt fournit un état global SSR-safe suffisant. Le plugin `.client` garantit que `EventSource` ne tourne que côté navigateur.

---

## Décision 8 : UI 100 % Tailwind v4 pur (Principe VI)

**Decision**: Toute l'UI de cette feature est **publique / espace membre** (annuaire `/profil`, fiche `/profil/{id}`, bouton flottant global, fenêtre de chat, page `/mon-compte/amis`). **Aucune** surface back-office admin n'est ajoutée → **Tailwind CSS v4 pur, sans daisyUI**.

**Rationale**: le Principe VI réserve daisyUI au back-office admin. La modération éventuelle des blocages reste côté membre (auto-service), pas d'écran admin requis. L'audit (Principe VII) est assuré côté backend sans UI dédiée.

---

## Décision 9 : Audit (Principe VII) : toutes mutations, métadonnées seules pour les messages

**Decision**: Auditer via `audit::log_action` **toutes** les mutations, y compris l'envoi et la suppression de message. Pour les **messages**, l'audit ne journalise que des **métadonnées** (id message, id conversation, id expéditeur, longueur du contenu), **jamais le contenu textuel**. Les mutations relationnelles (demande, amitié, blocage) sont auditées normalement avec leur état before/after.

**Rationale**: respecte pleinement le Principe VII (« toute mutation DOIT être auditée ») tout en évitant d'exposer le contenu des conversations privées dans le journal d'audit. La traçabilité de l'action (qui a envoyé/supprimé quoi, quand) est conservée sans atteinte à la confidentialité ni duplication du contenu (déjà persisté dans `social.message`).

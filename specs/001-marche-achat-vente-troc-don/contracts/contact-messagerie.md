# Contrat API — Contact via messagerie (FR-011..FR-014, D2)

Met en relation un membre intéressé et l'auteur d'une annonce **via la messagerie privée existante** (`social.conversation` / `social.message`, SSE temps réel). JWT requis. Mutation auditée.

## POST /api/annonces/{id}/contacter — Contacter l'auteur

- **Body** : `application/json` `{ "message": "<1..2000 car.>" }`
- **Comportement** :
  1. `utilisateur_courant` = acheteur ; `auteur` = `annonce.cree_par`.
  2. Vérifs : annonce `etat='publiee'` (sinon `404`) ; `acheteur <> auteur` (sinon `400` « on ne contacte pas sa propre annonce », FR-013) ; pas de blocage réciproque (sinon `403`).
  3. `obtenir_ou_creer_conversation(acheteur, auteur)` **sans exiger l'amitié** (chemin marketplace, D2) ; si création, renseigner `conversation.annonce_id = {id}` (COALESCE).
  4. Insérer un `social.message` (`expediteur_id=acheteur`, `contenu=message`).
  5. Mettre à jour `conversation.dernier_message_at` ; pousser l'évènement SSE (`evt_message`) ; créer/incrémenter les non-lus de l'auteur ; notifier l'auteur (FR-012).
- **200/201** : `{ success: true, data: { conversation_id, message: MessageResponse } }`
- **400** : message vide/trop long, ou contact de sa propre annonce
- **401** : non authentifié
- **403** : blocage réciproque
- **404** : annonce introuvable / non publiée

## Modification associée — `envoyer_message` (handlers/messagerie.rs)

Assouplir la règle d'envoi (D2) :

> Envoi autorisé si **(amitié active) OU (une conversation existe déjà entre les deux)**, et **aucun blocage**.

Ainsi les échanges de suivi sur une conversation née d'un contact d'annonce sont permis, sans ouvrir la messagerie à des inconnus (la conversation n'a pu naître que d'une amitié ou d'un contact d'annonce publiée réelle).

## Contexte UI

Le détail de conversation peut afficher « À propos de l'annonce : <titre> » via `conversation.annonce_id` (jointure `marketplace.annonce`). Côté `[id].vue`, le bouton « Contacter » :
1. exige l'authentification (sinon redirection `/login`),
2. n'est pas affiché sur sa propre annonce (FR-013),
3. après succès, redirige vers la messagerie sur la conversation retournée.

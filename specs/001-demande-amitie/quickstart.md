# Quickstart : Demande d'amitié & messagerie

**Feature**: `001-demande-amitie`

Guide de mise en route pour développer et valider la fonctionnalité localement.

## Prérequis

```bash
docker compose up -d                 # PostgreSQL + Adminer + LiveKit
```

> Le schéma `social` est créé par `schemas/29_social.sql` via l'orchestrateur. En dev, recréer le volume pour réinitialiser : `docker compose down -v && docker compose up -d`.

## Lancer le projet

```bash
# Backend (port 8082) : tuer l'ancien process d'abord
kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run   # dans uafricas_backend/

# Frontend (port 3000)
pnpm dev                                                         # dans uafricas_frontend/
```

## Comptes de test

| Email | Mot de passe |
|-------|--------------|
| test-admin@test.com | Test1234 |
| test-user@test.com | Test1234 |

Pour tester l'amitié il faut **deux comptes** : utiliser ces deux-là (ou en créer un second).

## Parcours de validation (manuel)

### US1 : Envoyer une demande
1. Connecté en `test-user`, aller sur `/profil`, ouvrir la fiche de `test-admin`.
2. Cliquer « Demander en ami » → le bouton passe à « Demande envoyée ».
3. Vérifier en base : `SELECT * FROM social.demande_amitie;` (statut `en_attente`).
4. Déconnecté : le bouton invite à se connecter (`/login?redirect=...`).

### US2 : Répondre
1. Connecté en `test-admin`, ouvrir `/mon-compte/amis` (ou via le bouton flottant / notifications).
2. Accepter la demande → l'amitié apparaît des deux côtés ; `test-user` reçoit une notification `demande_acceptee` en temps réel.
3. Refuser une autre demande → elle disparaît, l'émetteur n'est pas notifié.
4. Demande croisée : A→B en attente, puis B→A → amitié directe (pas de 2ᵉ demande).

### US3 : Chat temps réel
1. Avec deux navigateurs (un par compte, amis), cliquer le **bouton flottant** présent sur n'importe quelle page.
2. La fenêtre liste les amis ; sélectionner l'ami, envoyer un message.
3. Vérifier la réception **quasi instantanée** (< 2 s) côté destinataire sans rechargement (SSE).
4. Fenêtre fermée → le badge du bouton flottant signale les non-lus ; ouverture → marqués lus.
5. Supprimer un de ses messages → « message supprimé » des deux côtés.
6. Un non-ami n'apparaît pas dans la liste et n'est pas joignable.

### US4 : Gestion & blocage
1. `/mon-compte/amis` : onglets amis / demandes reçues / envoyées / bloqués.
2. Annuler une demande envoyée en attente.
3. Retirer un ami → disparaît du chat des deux côtés ; la conversation devient verrouillée.
4. Bloquer un membre → demandes/amitié rompues, messagerie inaccessible. Débloquer le rétablit (sans recréer l'amitié).

## Vérifications transverses
- **Confidentialité** (FR-026) : la liste d'amis d'un autre membre n'est exposée par aucun endpoint.
- **Rate-limit** (FR-014) : au-delà de 30 demandes/24 h → `429`.
- **Audit** (Principe VII) : `social/demande_amitie`, `social/amitie`, `social/blocage` apparaissent dans le journal d'audit ; les messages n'y figurent pas (Décision 9).
- **SSE auth** : `/api/messagerie/flux` sans `token` valide → `401`.

## Points d'attention
- Le push temps réel suppose **une seule instance backend** (registre en mémoire). OK en dev et en prod mono-backend actuel.
- UI 100 % **Tailwind v4 pur** (pas de daisyUI), pages publiques + espace membre (Principe VI).

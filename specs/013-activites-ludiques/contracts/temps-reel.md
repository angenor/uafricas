# Contrat : signaux temps réel et notifications du jeu

Le jeu n'ouvre aucun canal neuf. Il publie sur le flux SSE existant (`GET /api/messagerie/flux?token=<jwt>`, `services/messagerie_sse.rs`) et écrit dans la table de notifications générique (`arbre_genealogique.notifications`, via `models::notification::creer_notification`).

## 1. Principe

**Un signal dit qu'il faut relire, il ne transporte pas l'état.** Le flux n'a pas de tampon : un événement publié vers un membre déconnecté est perdu. Aucun écran ne doit donc dépendre de la réception d'un signal pour être juste. Chaque signal porte l'identifiant du duel ; le client relit `GET /api/jeu/duels/{id}` ou `…/direct`.

Filet : pendant un duel direct en cours, le client relit `…/direct` toutes les 3 secondes, signal ou pas.

## 2. Signaux SSE

Publiés par `sse.publier(utilisateur_id, &evt)`, construits par des fonctions `evt_duel_*` dans `models/jeu.rs`, sur le modèle de `models/appel.rs`.

| `type` | Destinataire | Émis quand | Charge |
|---|---|---|---|
| `duel_propose` | adversaire | `POST /duels` | `duel_id`, `mode`, `module`, `proposant` (membre léger), `expire_a` |
| `duel_accepte` | proposant | `…/accepter` | `duel_id`, `mode` |
| `duel_refuse` | proposant | `…/refuser` | `duel_id` |
| `duel_annule` | l'autre joueur | `…/annuler`, ou annulation par résolution | `duel_id` |
| `duel_a_vous` | l'autre joueur | différé : un joueur a terminé sa partie | `duel_id` |
| `duel_manche` | les deux | direct : une réponse est enregistrée, une manche s'ouvre ou se clôt | `duel_id`, `rang` |
| `duel_termine` | les deux | le duel passe `termine` ou `expire` | `duel_id` |

`proposant` réutilise `MembreLight` (`models/amitie.rs`) : `id`, `nom`, `prenom`, `slug`, `photoUrl`, `fonction`, `pays`.

**Aucun signal ne porte une épreuve, une réponse ou un score.**

## 3. Côté client

| Fichier | Changement |
|---|---|
| `app/plugins/messagerie.client.ts` | une branche `else if (evt.type.startsWith('duel_'))` → `useDuels().gererEvenement(evt)`, **avant** le `else` final. Le dispatch n'a pas de cas par défaut : sans cette branche, les signaux seraient ignorés en silence |
| `app/composables/useDuels.ts` | état `useState('duel:invitation')`, `gererEvenement`, calqué sur `useAppels.ts` |
| `app/components/social/MessagerieFlottante.vue` | monte `<JeuInvitationDuelPrompt>` à côté de `<SocialAppelEntrantPrompt>` |

`MessagerieFlottante` est monté dans les gabarits `africans`, `default` et `africans-cinema`, sous `ClientOnly` et pour un membre connecté. Il ne l'est pas dans `admin` ni `auth` : un administrateur dans le back-office ne voit pas surgir l'invitation, il la trouve dans ses notifications.

**Comportement de l'invitation**

- `duel_propose` en mode `direct` : fenêtre d'invitation immédiate, avec le temps restant. Accepter mène à `/activites/duels/{id}`.
- `duel_propose` en mode `differe` : pas de fenêtre. Le compteur de la cloche est rafraîchi ; la notification suffit.
- Une invitation directe reçue pendant qu'un duel direct est déjà en cours n'ouvre pas de fenêtre (la notification reste).

## 4. Notifications

Constantes dans un sous-module `pub mod jeu` de `models/notification.rs`. Le `type` est un `VARCHAR(80)` libre.

| `type` | Destinataire | Message | `lien_action` |
|---|---|---|---|
| `jeu.duel_propose` | adversaire | « {Prénom} vous défie sur {module} » | `/activites/duels/{id}` |
| `jeu.duel_accepte` | proposant | « {Prénom} a accepté votre duel » | idem |
| `jeu.duel_refuse` | proposant | « {Prénom} a décliné votre duel » | idem |
| `jeu.duel_a_vous` | l'autre joueur | « {Prénom} a joué, à vous » | idem |
| `jeu.duel_termine` | les deux | « Duel terminé : victoire / défaite / nul / forfait » | idem |
| `jeu.signalement_traite` | signaleur | « Votre signalement a été confirmé / classé » | `/mon-compte/activites` |
| `jeu.gains_annules` | membre | « Des gains ont été annulés : {motif} » | `/mon-compte/activites` |

Les distinctions et la réputation passent par les notifications **existantes** du moteur d'engagement (`engagement.badge_debloque`) : rien à ajouter.

Un duel direct n'émet **pas** de notification `duel_a_vous` (les deux joueurs sont devant l'écran).

**Côté client**, quatre endroits pour chaque type nouveau : l'union `TypeNotification`, `iconeNotification` et `couleurNotification` dans `app/mocks/notifications.ts` (malgré son nom, c'est le code de production), et la table `TYPES` de `app/pages/notifications.vue`. Un type oublié ne casse rien : il s'affiche avec la cloche générique.

## 5. Ce qui n'est pas temps réel

Les classements, la carte, les défis et le duel différé ne reçoivent aucun signal : ils se lisent à l'ouverture de la page. Un classement qui se mettrait à jour seul n'apporte rien et coûterait une diffusion à tous les membres.

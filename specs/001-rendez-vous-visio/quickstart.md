# Quickstart — Rendez-vous en visioconférence entre amis

## Pré-requis

- Stack dev lancée : `docker compose up -d` (PostgreSQL + Adminer + LiveKit).
- Deux comptes de test **amis** entre eux. Sinon, lier d'amitié via le panneau messagerie (onglet « Membres ») ou les comptes seed.
- Comptes : `test-admin@test.com` / `Test1234` et `test-user@test.com` / `Test1234`.

## Mise en place (ordre Principe III : SQL → backend → frontend)

1. **Schéma SQL** — créer `uafricas_backend/doc/bd/schemas/31_social_rendez_vous.sql` (enum + table + index, idempotent) puis ajouter `\ir schemas/31_social_rendez_vous.sql` dans `doc/bd/schema.sql` après la ligne `30_social_conversation_annonce.sql`.
   - Réinitialiser la BDD dev : `docker compose down -v && docker compose up -d` (le `docker-init.sh` rejoue le schéma), **ou** appliquer la migration à chaud :
     `docker compose exec -T postgres psql -U uafricas -d africans_db < uafricas_backend/doc/bd/schemas/31_social_rendez_vous.sql`

2. **Backend** — créer `src/models/rendez_vous.rs` + `src/handlers/rendez_vous.rs`, déclarer les modules (`models/mod.rs`, `handlers/mod.rs`), ajouter le scope `/rendez-vous` dans `src/routes.rs`. Relancer proprement :
   `kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run`

3. **Frontend** — `cd uafricas_frontend && pnpm add peerjs`. Ajouter dans `nuxt.config.ts` :
   ```ts
   runtimeConfig: {
     public: {
       peerjsHost: '', // vide = cloud public 0.peerjs.com
       peerjsPort: 443,
       peerjsPath: '/',
       peerjsSecure: true,
       iceServers: [{ urls: 'stun:stun.l.google.com:19302' }],
     }
   }
   ```
   Surcharge via env : `NUXT_PUBLIC_PEERJS_HOST`, `NUXT_PUBLIC_ICE_SERVERS`, etc. Créer le composable + composants, modifier `MessagerieFlottante.vue`, `plugins/messagerie.client.ts`, `pages/profil/[id].vue`. Lancer : `pnpm dev` (port 3000).

## Parcours de validation manuelle (scénarios d'acceptation)

### US1 — Proposer (P1)
1. Se connecter (A), ouvrir `/profil/<id de B>` (amis). Vérifier le bouton **« Proposer un rendez-vous »** (absent si non-amis).
2. Remplir sujet + date future + durée 30, envoyer → toast succès. B reçoit la cloche (+1) et un événement temps réel < 5 s.
3. Cas d'échec : date passée / sujet vide / durée absente / soi-même → message d'erreur, aucun RDV créé.

### US2 — Répondre (P2)
4. Côté B : panneau messagerie → onglet **Rendez-vous** → filtre « en attente de ma réponse ». Accepter → statut `accepté`, A notifié.
5. Refuser un autre RDV → `refusé`, A notifié.
6. Contre-proposer → reste `proposé`, l'initiative bascule vers A (chez A : « en attente de ma réponse »), A notifié.
7. Tenter une action quand ce n'est pas son tour, ou contre-proposer un `accepté` → rejet `409`.

### US3 — Gérer (P3)
8. Vérifier les 4 filtres (attente moi / attente autre / à venir / passés) et l'affichage MembreLight (photo, nom, fonction, pays) + sujet/date/durée/statut.
9. Annuler un RDV accepté (depuis l'un OU l'autre) → `annulé`, l'autre notifié.
10. Cliquer le lien **messagerie** d'un RDV → la conversation privée avec ce membre s'ouvre.

### US4 — Visio (P4)
11. Sur un RDV `accepté`, vérifier que **« Rejoindre »** est inactif hors fenêtre, actif dès `−5 min`.
12. Les deux membres rejoignent (deux navigateurs/onglets) → flux local + distant visibles ; tester micro/caméra/quitter.
13. L'un quitte → l'autre voit « l'autre a quitté ».
14. Forcer un échec (bloquer la caméra / réseau) → message clair + bouton « Ouvrir la messagerie » (repli).
15. Après la fenêtre, le RDV bascule en « passés » (terminé par calcul), plus de bouton « Rejoindre ».

## Vérifications transverses
- **Audit** : `GET` admin audit ou Adminer `shared.audit_log` → entrées `social/rendez_vous` pour chaque mutation, **sans** sujet ni description (FR-033).
- **P2P** : pendant un appel, vérifier (DevTools `chrome://webrtc-internals`) qu'aucun média ne transite par le backend (SC-005).
- **Cloche** : badge mis à jour en temps réel à chaque événement (proposé/accepté/refusé/contre/annulé).

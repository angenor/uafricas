# Quickstart : Validation manuelle du direct d'événement

Pas de framework de test configuré (constitution). Validation manuelle de bout en bout avec 2 comptes.

## Pré-requis

```bash
# 1. Infra (PostgreSQL + Adminer + LiveKit)
docker compose up -d

# 2. Backend (tuer l'ancien process puis relancer)
kill $(lsof -i :8082 -t) 2>/dev/null; cd uafricas_backend && RUST_LOG=info cargo run

# 3. Frontend
cd uafricas_frontend && pnpm dev   # http://localhost:3000
```

Comptes de test : `test-admin@test.com` / `Test1234` (organisateur), `test-user@test.com` / `Test1234` (spectateur inscrit). Variables LiveKit dev par défaut (`devkey`/`secret`, `ws://localhost:7880`).

## Pré-condition : un événement diffusable

1. Connecté en organisateur, créer (ou via admin) un événement **format « En ligne »** ou **« Hybride »**, `etat='publie'`, `date_heure_debut` ≈ maintenant.
2. Connecter le second compte, ouvrir `/evenements/{id}`, cliquer **« S'inscrire »** → inscription confirmée.

## Scénario A : Organisateur ouvre et diffuse (US2)

1. Organisateur sur `/evenements/{id}` pendant la fenêtre (≥ 15 min avant le début) → bouton **« Ouvrir le direct »** visible (`statut_direct = en_attente`, `peut_ouvrir = true`).
2. Cliquer → redirige vers `/evenements/{id}/direct` ; `POST …/rejoindre` crée la session (`etat='en_cours'`), renvoie un token `role:'organisateur'`.
3. La caméra et le micro de l'organisateur s'activent et se diffusent. Tester **partage d'écran**.
4. **Vérifier** : `evenement_session` contient 1 ligne `en_cours` ; tentative d'ouvrir une 2ᵉ fois → rejoint la même session (index unique partiel, FR-015).

## Scénario B : Inscrit assiste au direct (US1)

1. Compte inscrit sur `/evenements/{id}` → bouton **« Rejoindre le direct »** actif (`statut_direct = en_direct`).
2. Cliquer → `/evenements/{id}/direct`, `POST …/rejoindre` renvoie `role:'spectateur'` (token `can_publish:false`).
3. **Vérifier** : la vidéo/audio de l'organisateur est visible/audible ; aucun bouton caméra/micro pour le spectateur ; latence < 5 s (SC-002).

### Cas négatifs
- Compte **non inscrit** sur la page → pas de bouton direct, invite à s'inscrire (403 si appel direct).
- **Non connecté** → invite à se connecter (401).
- Événement **présentiel** → aucun bouton direct (FR-019).

## Scénario C : Chat & réactions (US3)

1. Les deux comptes dans le direct. Le spectateur envoie un message dans le chat → visible chez l'organisateur < 2 s (SC-006), avec son nom.
2. Le spectateur envoie une réaction (emoji) → overlay éphémère chez l'organisateur.
3. **Vérifier** : à la clôture puis réouverture, **aucun** message de chat n'est conservé (éphémère, FR-007).

## Scénario D : Lever la main & promotion (US4)

1. Le spectateur clique **« Lever la main »** → l'organisateur voit la demande (liste `demandes_parole` + DataPacket).
2. L'organisateur **promeut** → le spectateur devient `intervenant`, ses contrôles caméra/micro s'activent, sa diffusion apparaît.
3. L'organisateur **rétrograde** → sa diffusion média cesse, redevient spectateur.
4. L'organisateur **retire** un participant → ce dernier est déconnecté et revient sur `/evenements/{id}`.
5. **Vérifier en base** : `evenement_session_participant.role` reflète les transitions ; `main_levee` repasse à `FALSE` après promotion.

## Scénario E : Capacité, fenêtre, repli (FR-020/012/023)

1. **Capacité** : poser `max_participants` bas (ex. 1) ; un 2ᵉ spectateur reçoit **409 « Capacité atteinte »** (pas de file d'attente).
2. **Fenêtre** : avant `debut − 15min` → bouton inactif, `statut_direct = indisponible`.
3. **Dépassement / arrêt sécurité** : laisser tourner au-delà de `arret_securite_at` (ou forcer la valeur en base) puis recharger `GET …/direct` → session passée `terminee` automatiquement (D6).
4. **Repli streaming KO** : arrêter le conteneur LiveKit, tenter de rejoindre → message d'erreur + **« Réessayer »** ; si `lien_en_ligne` renseigné, lien externe proposé (FR-023).

## Scénario F : Notifications & clôture (FR-011/017)

1. À l'ouverture (Scénario A), **vérifier** que l'inscrit reçoit une notification cloche « Le direct de … a commencé » + (page ouverte) un rafraîchissement SSE.
2. L'organisateur **clôture** → tous les participants sont prévenus (`session_fermee`) et la salle se ferme ; `evenement_session.etat='terminee'`, `duree_secondes` calculé.

## Vérifications transverses

- **Audit** : `shared.audit_log` contient `OUVRIR`/`PROMOUVOIR`/`RETROGRADER`/`RETIRER`/`CLOTURER` sur `media_content.evenement_session`, sans contenu de chat.
- **Annulation** (FR-016) : passer l'événement à `etat='annule'` pendant un direct → la prochaine lecture force la clôture, jointure impossible.
- **Cohérence cross-stack** : types TS (`statut_direct`, `role`) ↔ DTO Rust ↔ colonnes SQL identiques.

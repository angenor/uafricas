# Phase 1 — Quickstart: Migration tldraw → Excalidraw

**Feature** : `006-afrolang-excalidraw`
**Date** : 2026-04-24

Procédure opérationnelle pour implémenter, builder et valider la migration. Destinée au développeur qui exécutera les tâches générées par `/speckit.tasks`.

---

## Pré-requis

- Node.js + pnpm installés (version compatible avec Vite 6 et React 19).
- Branche courante : `006-afrolang-excalidraw`.
- PostgreSQL + LiveKit lancés localement (`docker compose up -d` depuis la racine du monorepo).
- Backend Rust actif : `cd uafricas_backend && RUST_LOG=info cargo run`.
- Frontend Nuxt lancé : `cd uafricas_frontend && pnpm dev`.
- Deux navigateurs (ou profils distincts) pour tester la collaboration.
- Comptes de test documentés dans `CLAUDE.md` (section « Test Users »).

## Étape 1 — Mise à jour des dépendances du projet iframe

```bash
cd whiteboard
# Retrait de tldraw, ajout d'Excalidraw
pnpm remove tldraw
pnpm add @excalidraw/excalidraw
# Vérification : aucune occurrence tldraw dans le lockfile
grep -c tldraw pnpm-lock.yaml   # doit retourner 0
```

## Étape 2 — Refonte de `whiteboard/src/App.tsx`

Appliquer le pattern décrit dans `contracts/postmessage.md` :

- Monter `<Excalidraw langCode="fr-FR" onChange={...} excalidrawAPI={(api) => { apiRef.current = api; postReady() }} />`.
- Implémenter le débouncing 80 ms de `onChange` pour émettre `excalidraw-operation`.
- Implémenter les listeners `window.addEventListener('message', ...)` pour `apply-operation`, `load-snapshot`, `get-snapshot`, `clear`.
- Implémenter la garde images (2 Mo + JPEG/PNG) avec émission `excalidraw-image-rejected`.
- Implémenter `estSnapshotExcalidrawValide` (cf. data-model.md).
- Conserver `main.tsx` et `index.html` tels quels.

## Étape 3 — Adaptation de `AfrolangWhiteboard.vue`

Sans toucher à la signature des props (`sessionId`, `estModerateur`, `room`) :

- Remplacer les handlers des anciens messages tldraw par les nouveaux types.
- Brancher le `setInterval` 30 s de `get-snapshot` conditionné à `props.estModerateur`.
- Ajouter un `watch(() => props.room?.state, ...)` pour détecter Connected ↔ Disconnected et déclencher la resync (appel `obtenirTableauBlanc` + `load-snapshot`).
- Ajouter la garde de broadcast : n'envoyer sur LiveKit que si `room?.state === 'connected'`.
- Ajouter un toast (Tailwind v4 pur, sans daisyUI — principe VI de la constitution) pour `excalidraw-image-rejected`.
- Ajouter le bouton « Effacer tout » avec `v-if="estModerateur"` déclenchant le pattern `clear`.

## Étape 4 — Build et copie de l'actif statique

```bash
cd whiteboard
pnpm build     # génère whiteboard/dist/

# Remplacement atomique de l'actif servi par Nuxt
rm -rf ../uafricas_frontend/public/whiteboard
cp -r dist ../uafricas_frontend/public/whiteboard

# Vérifications rapides côté bundle
cd ../uafricas_frontend
grep -lr "tldraw" public/whiteboard || echo "AC-5 OK: aucun résidu tldraw"
grep -l "tl-watermark\|No tldraw license key provided" public/whiteboard/assets/*.js || echo "AC-6 OK: aucun marqueur anti-tamper"
```

## Étape 5 — Validation locale (dev)

1. Ouvrir deux profils navigateur distincts sur `http://localhost:3000`.
2. Connecter le compte modérateur dans l'un, un compte participant dans l'autre (cf. `CLAUDE.md § Test Users`).
3. Rejoindre la même session Afrolang (publique ou privée).
4. Ouvrir le tableau blanc des deux côtés.
5. Dessiner une forme depuis le modérateur → vérifier apparition < 500 ms côté participant (AC-2, SC-002).
6. Inverser : dessiner depuis le participant → vérifier apparition côté modérateur.
7. Tester « Effacer tout » → les deux tableaux doivent se vider immédiatement (AC-4).
8. Tester persistance : dessiner, attendre > 30 s, fermer le tableau, rouvrir → contenu restauré (AC-3).
9. Tester image valide (JPEG < 2 Mo) : elle s'affiche et se diffuse.
10. Tester image invalide (PDF ou JPEG > 2 Mo) : refus local + toast, rien de diffusé.
11. Tester reconnexion : couper le wifi 10 s, le rétablir → le tableau se resynchronise automatiquement sur le dernier snapshot serveur (FR-016).

## Étape 6 — Validation production

Après merge et déploiement via `./deploy.sh update` :

1. Ouvrir `https://www.africans-world.org`, rejoindre une session Afrolang.
2. Activer le tableau blanc et laisser la fenêtre ouverte au moins 15 minutes.
3. Vérifier visuellement que la barre d'outils reste présente et fonctionnelle (AC-1, SC-001).
4. Inspecter la console navigateur → absence totale d'erreurs (SC-004).
5. Exécuter côté VPS : `grep -c "tl-watermark\|No tldraw license" /opt/uafricas/frontend_static/whiteboard/assets/*.js` → doit retourner 0 (AC-6).

## Étape 7 — Nettoyage final

```bash
# Depuis la racine du monorepo
grep -rn "tldraw" whiteboard/src uafricas_frontend/app uafricas_frontend/public/whiteboard 2>/dev/null | grep -v ".lock\|node_modules"
# Aucune sortie attendue (AC-5)
```

Puis commit conventionnel :

```bash
git add whiteboard/package.json whiteboard/pnpm-lock.yaml whiteboard/src/App.tsx \
        uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue \
        uafricas_frontend/public/whiteboard
git commit -m "feat(afrolang-whiteboard): migration tldraw → Excalidraw pour résoudre la désactivation UI en prod"
```

## Points de vigilance

- **Ne pas toucher** : `AfrolangRoom.vue`, `pages/afrolang/session/[id].vue`, `pages/afrolang/session/privee/[id].vue`, `useAfrolang.ts`, tout le `uafricas_backend/`, le schéma SQL. Tout changement dans ces fichiers viole la spec (FR-012, FR-013).
- **Bundle size** : si Vite affiche un warning sur la taille du chunk, laisser tel quel — l'iframe est chargée à la demande, l'impact sur l'ouverture de la page Afrolang est nul (NFR-7).
- **CSP / iframe** : vérifier qu'en prod `nginx/nginx.conf` autorise bien le chargement de `/whiteboard/` ; pas de modification prévue, mais à surveiller si la page refuse d'embarquer l'iframe.

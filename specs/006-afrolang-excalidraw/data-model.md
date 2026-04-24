# Phase 1 — Data Model: Snapshot Excalidraw persisté

**Feature** : `006-afrolang-excalidraw`
**Date** : 2026-04-24

Ce document décrit la forme du contenu sérialisé dans la colonne JSONB existante `afrolang.tableau_blanc_session.donnees`. **Aucune modification du schéma SQL n'est requise** (FR-013, NFR-3) : la colonne reste générique et opaque côté backend Rust.

---

## Entités

### 1. `TableauBlancData` (enveloppe déjà existante — inchangée)

```ts
interface TableauBlancData {
  donnees: Record<string, unknown>  // conteneur JSONB opaque — voir SnapshotExcalidraw ci-dessous
  version: number                   // entier incrémental géré côté serveur
}
```

Cette enveloppe est renvoyée par `GET /api/afrolang/sessions/:id/tableau-blanc` et consommée par `obtenirTableauBlanc` / `sauvegarderTableauBlanc` dans `useAfrolang.ts`. **Signature inchangée**.

---

### 2. `SnapshotExcalidraw` (nouveau contenu de `donnees`)

```ts
interface SnapshotExcalidraw {
  type: 'excalidraw'              // marqueur explicite pour détection sans ambiguïté
  version: 1                      // version du format interne, incrémentée en cas d'évolution
  elements: ReadonlyArray<ExcalidrawElement>
  appState: Partial<AppState>     // restreint aux champs non volatils (voir ci-dessous)
  files?: BinaryFiles             // dictionnaire d'images encodées — voir ci-dessous
}
```

**Champs principaux** :

| Champ | Type | Description | Contrainte |
|-------|------|-------------|-----------|
| `type` | `'excalidraw'` | Discriminant pour rejeter les snapshots legacy | Toujours `'excalidraw'` à l'écriture |
| `version` | `number` | Version de format applicatif | `1` pour cette itération |
| `elements` | tableau | Scène graphique (formes, textes, flèches, dessins libres, images) | Tableau obligatoire, vide autorisé |
| `appState` | objet | État d'affichage filtré (zoom, background, nom de fichier) | Les champs volatils sont supprimés (voir filtre ci-dessous) |
| `files` | objet | Images encodées en data URL, indexées par `fileId` | Optionnel, absent si la scène n'a aucune image |

**Filtre des champs volatils d'`appState`** (supprimés avant persistance pour limiter la taille JSONB et éviter les faux conflits multi-navigateurs) :

- `collaborators` (carte des pairs — reconstruite à chaque connexion)
- `selectedElementIds`, `selectedGroupIds` (sélection locale)
- `editingElement`, `draggingElement`, `resizingElement` (états d'édition en cours)
- `cursorButton`, `scrolledOutside` (curseur viewport)
- `contextMenu`, `openPopup`, `openMenu`, `openDialog`, `openSidebar` (UI ouverte)
- `toast`, `errorMessage` (messages transitoires)
- `showHyperlinkPopup`, `showWelcomeScreen`

### 3. `ExcalidrawElement` (référence externe)

Forme canonique d'un élément Excalidraw (rectangle, ellipse, diamond, arrow, line, freedraw, text, image, frame). Identifié par `id: string`, positionné par `x`/`y`, dimensionné par `width`/`height`, typé par `type`. Les types complets sont fournis par `@excalidraw/excalidraw/types`. **Aucune validation structurelle additionnelle côté UAfricas** : la librairie garantit la cohérence de ses propres objets.

### 4. `BinaryFiles` (ressources images)

```ts
type BinaryFiles = Record<string, {
  mimeType: 'image/jpeg' | 'image/png'
  id: string
  dataURL: string   // data:image/jpeg;base64,... encodage standard Excalidraw
  created: number   // timestamp ms
}>
```

**Contraintes de validation côté client avant insertion** (FR-001a) :

- `mimeType` restreint à `image/jpeg` et `image/png` (les autres types natifs d'Excalidraw comme `image/svg+xml`, `image/webp` sont refusés).
- Taille binaire décodée ≤ 2 Mo (2 097 152 octets).
- En cas d'échec de validation, l'image n'est ni ajoutée à la scène, ni diffusée, ni persistée ; un toast localisé informe l'utilisateur.

---

## Lecture défensive des snapshots legacy

### Règle de détection (Décision 4 de research.md)

Fonction utilitaire `estSnapshotExcalidrawValide(donnees: unknown): boolean` appliquée côté iframe React juste avant `excalidrawAPI.updateScene()` :

```
Retour `false` si donnees est :
  - null / undefined / primitive
  - un objet avec une clé "document", "store" ou "records"          (marqueurs tldraw)
  - un objet sans tableau "elements"
Retour `true` si donnees est :
  - un objet avec elements: Array
  (les champs type/version peuvent être absents dans d'anciens snapshots Excalidraw — tolérés)
```

Si `false`, initialiser Excalidraw avec `{ elements: [], appState: {}, files: {} }` sans lever d'erreur utilisateur (FR-009).

### Écriture

Tous les snapshots écrits par cette feature incluent le marqueur `type: 'excalidraw'` + `version: 1`. Les futures évolutions incrémenteront `version` et devront gérer explicitement la rétro-compatibilité.

---

## Cycle de vie

| Transition | Déclencheur | Conséquence |
|------------|-------------|-------------|
| Création | Premier `PUT` sur une session sans ligne existante | Backend existant crée la ligne, version = 1 |
| Mise à jour | `PUT` périodique par un modérateur (30 s) | Backend existant écrase `donnees`, incrémente `version` |
| Effacement | `DELETE` (modérateur) | Backend existant supprime la ligne ; prochain `GET` renvoie l'état par défaut `{ donnees: {}, version: 0 }` |
| Ouverture d'une session inexistante | Premier `GET` avant toute écriture | Backend existant retourne `{ donnees: {}, version: 0 }` → iframe ouverte sur scène vide |

Aucune de ces transitions ne requiert de changement de handler Rust ; elles sont toutes déjà implémentées et auditées (principe VII de la constitution).

---

## Volumétrie et scale

- Taille typique d'une scène pédagogique modérée (sans images) : 5 à 50 Ko JSON non minifié.
- Taille maximale attendue avec images (FR-001a = max 2 Mo × quelques images dans la même scène) : ~10 Mo. PostgreSQL JSONB supporte sans difficulté, mais au-delà de quelques Mo la performance de diffusion sur DataChannel LiveKit dégrade — en pratique, l'usage pédagogique reste en deçà.
- Un seul `TableauBlancData` par session Afrolang (relation 1-à-1 avec `afrolang.session`).

---

## Non-goals

- **Pas d'historique / undo persistant** : chaque snapshot écrase le précédent. L'undo local d'Excalidraw (stack en mémoire) est conservé côté client mais non persisté.
- **Pas de snapshots horodatés multiples** : une seule ligne par session dans la table existante.
- **Pas de migration automatique depuis tldraw** : cf. Décision 4 de research.md (lecture défensive → état vide).

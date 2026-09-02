# Phase 2 : Frontend UI

> **Statut** : `TERMINE`
> **Progression** : 14/14 taches
> **Bloque par** : [Phase 1 : Backend REST](./01_BACKEND_REST.md) (tous les endpoints doivent etre operationnels)
> **Debloque** : [Phase 3 : WebRTC Signaling](./03_WEBRTC_SIGNALING.md)

---

## Contexte inter-phases

```
✅ = termine    🔄 = en cours    ⬜ = a faire    🔒 = bloque

[✅] Phase 1 : Backend REST         (terminee)
[✅] Phase 2 : Frontend UI          ◄── TERMINE
[⬜] Phase 3 : WebRTC Signaling     (debloquee)
[🔒] Phase 4 : Tableau blanc        (attend Phase 3 complete)
```

**Ce que la Phase 1 fournit a cette phase :**
- 19 endpoints REST fonctionnels (`/api/afrolang/...`)
- Formats JSON valides pour tous les DTOs (SalleResponse, SallePriveeResponse, SessionResponse, etc.)
- Logique metier validee (acces code, demarrage session, rejoindre/quitter)

**Ce que cette phase produit pour la Phase 3 :**
- Page `afrolang/session/[id].vue` (placeholder) qui sera transformee en salle de visioconference
- Composable `useAfrolang.ts` avec toutes les fonctions API que Phase 3 enrichira avec `genererTokenSession()`
- Navigation complete : liste salles → detail → salle privee → session

---

## Progression

- [x] **2.1** Creer `app/composables/useAfrolang.ts`, Interfaces TypeScript
- [x] **2.2** Creer `app/composables/useAfrolang.ts`, Fonctions API + constantes
- [x] **2.3** Creer `app/components/afrolang/AfrolangHero.vue`
- [x] **2.4** Creer `app/components/afrolang/AfrolangStats.vue`
- [x] **2.5** Creer `app/components/afrolang/SalleCard.vue`
- [x] **2.6** Creer `app/components/afrolang/SalleFilters.vue` + `SalleFiltersMobile.vue`
- [x] **2.7** Creer `app/components/afrolang/SallePriveeCard.vue`
- [x] **2.8** Creer `app/components/afrolang/SallePriveeCreateModal.vue`
- [x] **2.9** Creer `app/components/afrolang/SallePriveeJoinModal.vue`
- [x] **2.10** Creer `app/components/afrolang/SessionCard.vue` + `SessionTimeline.vue` + `ParticipantBadge.vue`
- [x] **2.11** Creer `app/pages/afrolang/index.vue`, Liste salles publiques
- [x] **2.12** Creer `app/pages/afrolang/[id].vue`, Detail salle publique
- [x] **2.13** Creer `app/pages/afrolang/salle-privee/[id].vue`, Detail salle privee
- [x] **2.14** Creer `app/pages/afrolang/session/[id].vue`, Page session (placeholder pour Phase 3)

---

## 2.1–2.2 : Composable `app/composables/useAfrolang.ts`

### 2.1 : Interfaces TypeScript

> **Basees sur les DTOs de la Phase 1** (SalleResponse, SallePriveeResponse, SessionResponse, etc.)

```typescript
// ── Types ──────────────────────────────────────────────────────────────
export type EtatSession = 'planifiee' | 'en_cours' | 'terminee' | 'annulee'
export type RoleSession = 'moderateur' | 'participant' | 'observateur'

// ── Utilisateur resume (JOIN depuis Phase 1) ───────────────────────────
export interface AfrolangUser {
  uid: string
  nom: string
  prenom: string | null
  photo_url: string | null
}

// ── Salle publique ─────────────────────────────────────────────────────
export interface SalleAPI {
  id: string
  titre: string
  slug: string | null
  description: string | null
  image_couverture_url: string | null
  langue_cible: string | null
  moderateur: AfrolangUser | null
  actif: boolean
  nombre_salles_privees: number    // ← sous-requete COUNT dans Phase 1
  sessions_en_cours: number        // ← sous-requete COUNT dans Phase 1
  created_at: string
}

export interface SalleDetailAPI extends SalleAPI {
  salles_privees: SallePriveeAPI[]
}

// ── Salle privee ───────────────────────────────────────────────────────
export interface SallePriveeAPI {
  id: string
  salle_id: string
  titre: string
  description: string | null
  image_couverture_url: string | null
  max_participants: number
  est_protegee: boolean            // ← Phase 1 renvoie true/false (jamais le code en clair)
  actif: boolean
  createur: AfrolangUser
  salle_titre: string
  salle_langue: string | null
  session_en_cours: boolean
  nombre_sessions: number
  created_at: string
}

export interface SallePriveeDetailAPI extends SallePriveeAPI {
  sessions: SessionAPI[]
}

// ── Session ────────────────────────────────────────────────────────────
export interface SessionAPI {
  id: string
  salle_privee_id: string
  titre: string | null
  etat: EtatSession
  moderateur: AfrolangUser | null
  date_debut_prevue: string | null
  demarre_at: string | null
  termine_at: string | null
  duree_secondes: number | null
  max_participants: number
  nombre_participants_pic: number
  nombre_participants_actuel: number
  tableau_blanc_actif: boolean
  created_at: string
}

export interface SessionDetailAPI extends SessionAPI {
  participants: ParticipantAPI[]
  salle_privee_titre: string
  salle_publique_titre: string
}

// ── Participant ────────────────────────────────────────────────────────
export interface ParticipantAPI {
  id: string
  utilisateur: AfrolangUser
  role_session: RoleSession
  rejoint_at: string
  quitte_at: string | null
  duree_secondes: number | null
}

// ── Stats (depuis Phase 1 : GET /api/afrolang/stats) ──────────────────
export interface AfrolangStats {
  total_salles: number
  total_salles_privees: number
  sessions_en_cours: number
  sessions_terminees: number
  total_participants_uniques: number
}

// ── Filtres ────────────────────────────────────────────────────────────
export interface SalleFiltres {
  recherche?: string
  langue?: string
  page?: number
  par_page?: number
}

export interface SallePriveeFiltres {
  recherche?: string
  page?: number
  par_page?: number
}

// ── Listes paginee ────────────────────────────────────────────────────
export interface SalleListeAPI {
  salles: SalleAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

export interface SallePriveeListeAPI {
  salles_privees: SallePriveeAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

// ── Formulaire creation salle privee ───────────────────────────────────
export interface CreerSallePriveeForm {
  titre: string
  description: string
  code_acces: string
  max_participants: number
}
```

### 2.2 : Fonctions API + constantes

```typescript
// ── Fonctions API (appellent les endpoints Phase 1) ────────────────────

// Salles publiques
export async function listerSalles(filtres: SalleFiltres): Promise<SalleListeAPI>
export async function obtenirSalle(id: string): Promise<SalleDetailAPI>

// Salles privees
export async function listerSallesPrivees(salleId: string, filtres: SallePriveeFiltres): Promise<SallePriveeListeAPI>
export async function obtenirSallePrivee(id: string): Promise<SallePriveeDetailAPI>
export async function creerSallePrivee(salleId: string, form: CreerSallePriveeForm): Promise<SallePriveeAPI>

// Sessions
export async function obtenirSession(id: string): Promise<SessionDetailAPI>
export async function creerSession(sallePriveeId: string, form: CreerSessionForm): Promise<SessionAPI>
export async function demarrerSession(sessionId: string): Promise<void>
export async function terminerSession(sessionId: string): Promise<void>
export async function rejoindreSession(sessionId: string, codeAcces?: string): Promise<void>
export async function quitterSession(sessionId: string): Promise<void>

// Utilitaires
export async function obtenirStats(): Promise<AfrolangStats>
export async function listerLangues(): Promise<string[]>

// ── NOTE Phase 3 : La fonction suivante sera ajoutee ici ──────────────
// export async function genererTokenSession(sessionId: string, codeAcces?: string): Promise<TokenResponse>
```

```typescript
// ── Constantes ─────────────────────────────────────────────────────────
export const ETATS_SESSION = [
  { value: '', label: 'Tous les etats' },
  { value: 'planifiee', label: 'Planifiee' },
  { value: 'en_cours', label: 'En cours' },
  { value: 'terminee', label: 'Terminee' },
  { value: 'annulee', label: 'Annulee' },
]

export function getEtatInfo(etat: EtatSession): { label: string; couleur: string; icone: string } {
  // planifiee → { label: 'Planifiee', couleur: 'badge-info', icone: 'clock' }
  // en_cours  → { label: 'En direct', couleur: 'badge-success', icone: 'video' }
  // terminee  → { label: 'Terminee', couleur: 'badge-neutral', icone: 'check' }
  // annulee   → { label: 'Annulee', couleur: 'badge-error', icone: 'xmark' }
}

export function formatDuree(secondes: number): string {
  // → "1h 23min" ou "45min" ou "2h"
}
```

---

## 2.3–2.10 : Composants `app/components/afrolang/`

### Structure des composants

```
app/components/afrolang/
├── AfrolangHero.vue              # 2.3, Banniere hero
├── AfrolangStats.vue             # 2.4, Compteurs animes
├── SalleCard.vue                 # 2.5, Card salle publique
├── SalleFilters.vue              # 2.6, Filtres desktop
├── SalleFiltersMobile.vue        # 2.6, Filtres mobile
├── SallePriveeCard.vue           # 2.7, Card salle privee
├── SallePriveeCreateModal.vue    # 2.8, Modal creation salle privee
├── SallePriveeJoinModal.vue      # 2.9, Modal saisie code d'acces
├── SessionCard.vue               # 2.10, Card session
├── SessionTimeline.vue           # 2.10, Timeline sessions
└── ParticipantBadge.vue          # 2.10, Avatar + role d'un participant
```

> **Composants Phase 3** (seront ajoutes plus tard dans ce meme dossier) :
> `AfrolangRoom.vue`, `AfrolangVideoGrid.vue`, `AfrolangControls.vue`, `AfrolangParticipantTile.vue`, `AfrolangSidebar.vue`
>
> **Composants Phase 4** (seront ajoutes encore apres) :
> `AfrolangWhiteboard.vue`

### 2.3 : `AfrolangHero.vue`
- Banniere hero de la page principale
- Titre "Salles Afrolang", sous-titre, bouton CTA
- Pattern identique aux autres Hero du projet

### 2.4 : `AfrolangStats.vue`
- Props: `stats: AfrolangStats` (depuis `GET /api/afrolang/stats`, Phase 1, tache 1.8)
- Affiche 4-5 compteurs avec icones
- Animation compteur au scroll (via AOS)

### 2.5 : `SalleCard.vue`
- Props: `salle: SalleAPI` (depuis `GET /api/afrolang/salles`, Phase 1, tache 1.5)
- Affiche: image couverture, titre, langue_cible (badge), nombre de salles privees, sessions en cours (indicateur "En direct" clignotant), moderateur (avatar + nom)
- Action: `NuxtLink` vers `/afrolang/{id}`

### 2.6 : `SalleFilters.vue` + `SalleFiltersMobile.vue`
- Filtres: recherche (texte) + langue (select, depuis `GET /api/afrolang/langues`, Phase 1, tache 1.8)
- Emit: `@filtre-change` avec les filtres actualises

### 2.7 : `SallePriveeCard.vue`
- Props: `sallePrivee: SallePriveeAPI` (depuis `GET /api/afrolang/salles/{id}/privees`, Phase 1, tache 1.6)
- Affiche: titre, createur, cadenas si `est_protegee`, badge "En direct" si `session_en_cours`, max participants
- Actions: bouton "Rejoindre" ou "Voir les sessions"

### 2.8 : `SallePriveeCreateModal.vue`
- Props: `salleId: string`, `visible: boolean`
- Appelle `creerSallePrivee()` (→ `POST /api/afrolang/salles/{id}/privees`, Phase 1, tache 1.6)
- Formulaire: titre, description, code_acces (optionnel), max_participants

### 2.9 : `SallePriveeJoinModal.vue`
- Props: `sessionId: string`, `visible: boolean`
- Appelle `rejoindreSession()` (→ `POST /api/afrolang/sessions/{id}/rejoindre`, Phase 1, tache 1.7)
- En Phase 3 : cette modal appellera plutot `genererTokenSession()` pour obtenir le token LiveKit

### 2.10 : `SessionCard.vue` + `SessionTimeline.vue` + `ParticipantBadge.vue`
- Utilisent les donnees de `GET /api/afrolang/sessions/{id}` (Phase 1, tache 1.7)
- `SessionCard` : titre, etat badge, date, duree, participants
- `SessionTimeline` : vue chronologique des sessions d'une salle privee
- `ParticipantBadge` : avatar + nom + role

---

## 2.11–2.14 : Pages `app/pages/afrolang/`

### Arborescence

```
app/pages/afrolang/
├── index.vue                         # 2.11, Liste des salles publiques
├── [id].vue                          # 2.12, Detail salle publique
├── salle-privee/
│   └── [id].vue                      # 2.13, Detail salle privee
└── session/
    └── [id].vue                      # 2.14, Placeholder session (transforme en Phase 3)
```

### 2.11 : `afrolang/index.vue`

```
┌────────────────────────────────────────────────┐
│  AfrolangHero (2.3)                            │
├────────────────────────────────────────────────┤
│  Breadcrumb: Accueil > Afrolang                │
├────────────────────────────────────────────────┤
│  AfrolangStats (2.4)                           │
├────────────────────────────────────────────────┤
│  SalleFilters (2.6)                            │
├────────────────────────────────────────────────┤
│  Grid : SalleCard (2.5) x N                   │
├────────────────────────────────────────────────┤
│  Pagination                                    │
└────────────────────────────────────────────────┘
```

### 2.12 : `afrolang/[id].vue`

```
┌────────────────────────────────────────────────┐
│  Breadcrumb: Accueil > Afrolang > {titre}      │
├────────────────────────────────────────────────┤
│  Image couverture + Titre + Langue + Moderateur│
│  Description                                   │
├────────────────────────────────────────────────┤
│  [+ Creer une salle privee] (JWT)              │
│  SallePriveeCreateModal (2.8)                  │
├────────────────────────────────────────────────┤
│  Grid : SallePriveeCard (2.7) x N             │
├────────────────────────────────────────────────┤
│  Pagination                                    │
└────────────────────────────────────────────────┘
```

### 2.13 : `afrolang/salle-privee/[id].vue`

```
┌────────────────────────────────────────────────┐
│  Breadcrumb: Accueil > Afrolang > {salle}      │
│              > {salle_privee}                   │
├────────────────────────────────────────────────┤
│  Titre + Createur + Cadenas + Description      │
├────────────────────────────────────────────────┤
│  Actions moderateur: [+ Planifier une session] │
├────────────────────────────────────────────────┤
│  SessionTimeline (2.10)                        │
│  Liste SessionCard (2.10)                      │
└────────────────────────────────────────────────┘
```

### 2.14 : `afrolang/session/[id].vue` (PLACEHOLDER, sera transforme en Phase 3)

```
┌────────────────────────────────────────────────┐
│  Titre session + Etat (badge)                  │
│  Salle privee + Salle publique (liens)         │
│  Moderateur + Date prevue + Duree              │
├────────────────────────────────────────────────┤
│  Si "en_cours":                                │
│  [Rejoindre la session] → SallePriveeJoinModal │
├────────────────────────────────────────────────┤
│  Liste des participants (ParticipantBadge)     │
└────────────────────────────────────────────────┘
```

> **NOTE Phase 3** : Cette page sera enrichie avec `AfrolangRoom.vue` (visioconference WebRTC).
> Le bouton "Rejoindre" appellera `genererTokenSession()` au lieu de `rejoindreSession()`.

---

## Icones FontAwesome a ajouter

Ajouter dans `app/plugins/fontawesome.ts` (verifier celles deja presentes) :

```typescript
import {
  faVideo,           // session video
  faLock,            // salle protegee
  faLockOpen,        // salle ouverte
  faChalkboard,      // tableau blanc (Phase 4)
  faLanguage,        // langues
  faSignal,          // en direct
  faDoorOpen,        // rejoindre
  faCircle,          // indicateur live
} from '@fortawesome/free-solid-svg-icons'
```

---

## Recapitulatif fichiers

### Fichiers a creer (16)
| Fichier | Tache |
|---------|-------|
| `app/composables/useAfrolang.ts` | 2.1–2.2 |
| `app/components/afrolang/AfrolangHero.vue` | 2.3 |
| `app/components/afrolang/AfrolangStats.vue` | 2.4 |
| `app/components/afrolang/SalleCard.vue` | 2.5 |
| `app/components/afrolang/SalleFilters.vue` | 2.6 |
| `app/components/afrolang/SalleFiltersMobile.vue` | 2.6 |
| `app/components/afrolang/SallePriveeCard.vue` | 2.7 |
| `app/components/afrolang/SallePriveeCreateModal.vue` | 2.8 |
| `app/components/afrolang/SallePriveeJoinModal.vue` | 2.9 |
| `app/components/afrolang/SessionCard.vue` | 2.10 |
| `app/components/afrolang/SessionTimeline.vue` | 2.10 |
| `app/components/afrolang/ParticipantBadge.vue` | 2.10 |
| `app/pages/afrolang/index.vue` | 2.11 |
| `app/pages/afrolang/[id].vue` | 2.12 |
| `app/pages/afrolang/salle-privee/[id].vue` | 2.13 |
| `app/pages/afrolang/session/[id].vue` | 2.14 |

### Fichiers a modifier (1)
| Fichier | Modification |
|---------|-------------|
| `app/plugins/fontawesome.ts` | Ajouter les icones manquantes |

---

## Critere de completion Phase 2

> **La Phase 3 peut commencer UNIQUEMENT quand :**
> - [ ] Tous les 14 points de la progression sont coches
> - [ ] Navigation complete fonctionnelle : `/afrolang` → `/afrolang/{id}` → `/afrolang/salle-privee/{id}` → `/afrolang/session/{id}`
> - [ ] Les pages chargent les donnees depuis l'API backend (Phase 1)
> - [ ] La creation de salle privee fonctionne (formulaire + API)
> - [ ] La page `session/[id].vue` existe en placeholder (prete a recevoir AfrolangRoom en Phase 3)
> - [ ] `pnpm dev` compile sans erreur
>
> Quand c'est fait → mettre le statut a `TERMINE` dans [00_OVERVIEW.md](./00_OVERVIEW.md) et debloquer Phase 3.

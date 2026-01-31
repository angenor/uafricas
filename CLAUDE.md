# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Context

UAfricas frontend migration from Vue 3 + Vite to Nuxt 4.

**Source:** `../uafricas_old_version_vuejs/`
**Target:** `uafricas_frontend/`

## Commands

### New Frontend (uafricas_frontend/)
```bash
pnpm install                   # Install dependencies
pnpm dev                       # Dev server (port 3000)
pnpm build                     # Production build
pnpm generate                  # Static site generation
```

### Old Frontend (../uafricas_old_version_vuejs/)
```bash
npm install                    # Install dependencies
npm run dev                    # Dev server (Vite)
npm run build                  # Production build
```

## Architecture

### Target Structure (Nuxt 4)
```
uafricas_frontend/
├── app/
│   └── app.vue
├── pages/                     # File-based routing
├── components/
├── composables/
└── nuxt.config.ts
```

### Source Structure (Old Vue 3)
```
uafricas_old_version_vuejs/src/
├── views/                     # 233 page components (→ pages/ in Nuxt)
│   ├── Admin/                 # Complete admin dashboard
│   ├── Auth/                  # Login, registration
│   ├── CentreCulturel/        # Cultural center (Experts, Afrolang, CodiMoi, Marché)
│   ├── Conference/            # CLOM/MOOC courses
│   ├── Forum/                 # Discussion forums
│   ├── Media/                 # TV, Radio, Africantives
│   ├── Projet/                # Project management
│   ├── Universite/            # INUDA university system
│   └── UserProfils/           # User profiles
├── components/                # 43 reusable components
├── composables/               # 16 composables (useAuth, useNotifications, etc.)
├── services/                  # 9 services (authService, roleService, etc.)
├── stores/                    # Pinia stores (userInfos, codiMoi)
└── firebase-init.js           # Firebase configuration
```

## Tech Stack

**New Frontend:** Nuxt 4.2, Vue 3, TypeScript, Tailwind CSS, pnpm
**Old Frontend:** Vue 3, Vite, Vue Router 4, Pinia, Tailwind CSS 3, npm

## Old Frontend - Key Dependencies

- **Firebase:** Auth, Firestore, Storage (project: `africans-4e67a`)
- **Real-time:** PeerJS, Agora RTC, WebRTC
- **UI:** HeadlessUI, Heroicons, Lucide, FontAwesome 6
- **Charts:** Chart.js, ApexCharts
- **Other:** tldraw (whiteboard), v-calendar, papaparse, date-fns, gsap, aos

## Old Frontend - State Management

Two Pinia stores:
- `useUserStore` - User auth, permissions, preferences (persisted to sessionStorage)
- `useCodiMoiStore` - Posts, filtering, reactions for CodiMoi feature

## Old Frontend - Key Composables

- `useAuth()` - Firebase authentication
- `useNotifications()` - Toast system
- `useBreadcrumbs()` - Navigation breadcrumbs
- `useFirestore()` - Generic Firestore operations
- `useProjets()`, `useOpportunites()` - Business logic
- `useCalls()` - PeerJS/WebRTC video calls

## Migration Notes

- `views/` → Nuxt `pages/` with file-based routing
- Route guards → Nuxt middleware
- Manual router → Automatic from pages structure
- Composables and services can be reused with minimal changes
- Pinia stores work with Nuxt 4 (use auto-imports)

## Conventions from Old Project

- Always use `BreadcrumbNav.vue` for new pages
- Reference `@bank/modele/modele_firebase.md` before creating Firestore collections
- Follow existing authentication patterns with Firebase Auth

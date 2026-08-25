# Afrolang - Plan d'implémentation

> **Mode** : Chronologique strict. Chaque phase doit être terminee avant de passer a la suivante.
> **Suivi** : Cocher les cases `[ ]` → `[x]` au fur et a mesure de l'avancement.

---

## Progression globale

| Phase | Fichier | Statut | Progression |
|-------|---------|--------|-------------|
| **Phase 1** | [01_BACKEND_REST.md](./01_BACKEND_REST.md) | `TERMINE` | 12/12 |
| **Phase 2** | [02_FRONTEND_UI.md](./02_FRONTEND_UI.md) | `TERMINE` | 14/14 |
| **Phase 3** | [03_WEBRTC_SIGNALING.md](./03_WEBRTC_SIGNALING.md) | `TERMINE` | 11/11 |
| **Phase 4** | [04_TABLEAU_BLANC.md](./04_TABLEAU_BLANC.md) | `TERMINE` | 9/9 |

**Progression totale : 46/46 taches : FEATURE COMPLET**

---

## Vision

Les **salles Afrolang** sont des espaces de visioconference WebRTC dedies a l'apprentissage des langues africaines. Chaque "salle" est un webinaire interactif avec support de tableau blanc collaboratif.

## Architecture fonctionnelle

```
Salle publique (admin)         Salle privee (utilisateur)
┌─────────────────────┐       ┌──────────────────────────┐
│  Canal Wolof        │◄──────│  Cours Wolof debutant    │
│  Canal Swahili      │       │  (code_acces requis)     │
│  Canal Lingala      │       │                          │
│  ...                │       │  → Session 1 (terminee)  │
│                     │       │  → Session 2 (en cours)  │
│  cree par admin     │       │  → Session 3 (planifiee) │
│  moderateur designe │       │  cree par n'importe qui  │
└─────────────────────┘       │  moderateur = createur   │
                              └──────────────────────────┘
                                        │
                              ┌─────────┴─────────┐
                              │  Session WebRTC    │
                              │  + Tableau blanc   │
                              │  (ephemere)        │
                              └────────────────────┘
```

## Hierarchie des entites

1. **Salle publique** (`afrolang.salle`), Canal par langue, cree par admin
2. **Salle privee** (`afrolang.salle_privee`), Sous-salle liee a une salle publique, creee par un utilisateur, acces par code
3. **Session** (`afrolang.session`) : Conference WebRTC ephemere dans une salle privee
4. **Participant** (`afrolang.session_participant`), Tracking de presence par session
5. **Tableau blanc** (`afrolang.tableau_blanc`), Un par session, snapshot JSONB

## Ordre chronologique strict

```
Phase 1 (Backend REST)
  │  Livrable : API REST fonctionnelle, testee avec curl
  │
  └──► Phase 2 (Frontend UI)
         │  Prerequis : Tous les endpoints Phase 1 operationnels
         │  Livrable : Pages navigables, connectees a l'API
         │
         └──► Phase 3 (WebRTC)
                │  Prerequis : Page session/[id].vue (Phase 2) + endpoints sessions (Phase 1)
                │  Livrable : Visioconference fonctionnelle avec LiveKit
                │
                └──► Phase 4 (Tableau blanc)
                       │  Prerequis : AfrolangRoom.vue + DataChannels (Phase 3)
                       │  Livrable : Tableau blanc collaboratif en temps reel
```

## Stack technique additionnelle requise

| Composant | Technologie recommandee | Phase |
|-----------|------------------------|-------|
| SFU (Selective Forwarding Unit) | **LiveKit** (self-hosted) | Phase 3 |
| Signaling WebSocket | **Actix-Web WebSocket** (integre) | Phase 3 |
| Tableau blanc | **tldraw** (React) via iframe | Phase 4 |
| TURN/STUN | **coturn** (self-hosted) | Phase 3 (prod) |

## Schema SQL existant

Le schema est deja defini dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` avec :
- 5 tables (`salle`, `salle_privee`, `session`, `session_participant`, `tableau_blanc`)
- 1 enum (`etat_session`: planifiee, en_cours, terminee, annulee)
- Index optimises (etat session, noeud VPS, date planifiee)
- Contraintes inter-schemas vers `iam.utilisateur`

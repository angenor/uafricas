# Synthèse : Système d'engagement / gamification AFRICANS

> Phase 1 (fondation + barème vérifiable) livrée et validée end-to-end.
> Spec complète : `specs/001-engagement-gamification/`.

## 1. Vision globale (5 sous-systèmes)

Le cahier des charges décrit 5 briques de tailles très différentes. Seule la fondation + barème est implémentée ; les autres sont des chantiers futurs.

| # | Sous-système | Statut | Ampleur |
|---|---|---|---|
| **A** | Moteur de points + compte + niveaux + badges | ✅ **Livré (Phase 1)** | Moyenne |
| **B** | Barème (contributions, factcheck, popularité likes) | ✅ **Livré** (partiel, voir §5) | Grande |
| **C** | Statuts Membre/Premium/Platinum + visibilité | ✅ badges / ⏸️ visibilité algo reportée | Moyenne |
| **D** | Récompenses (cadeaux users Gô/Boro… + partenaires touristiques) | ⛔ Non commencé | Moyenne |
| **E** | Monétisation (publicité payante + dons/paiements) | ⛔ Non commencé | Très grande |

## 2. Architecture livrée (Phase 1)

Nouveau schéma bounded-context **`engagement`** (migration idempotente `35_engagement.sql`).

```
Backend (Rust/Actix)                          Frontend (Nuxt)
├─ doc/bd/schemas/35_engagement.sql           ├─ composables/useEngagement.ts (public)
├─ services/engagement.rs   ← le moteur       ├─ composables/useAdminEngagement.ts
├─ models/engagement.rs (public DTO)          ├─ components/engagement/
├─ models/admin/engagement.rs                 │   ├─ MesPointsPanel.vue (Tailwind pur)
├─ handlers/engagement.rs (public)            │   └─ BadgeStatut.vue (réutilisable)
├─ handlers/admin/engagement.rs (barème)      ├─ pages/admin/engagement/{regles,journal}.vue
└─ routes.rs (+ appels dans 8 handlers        ├─ pages/mon-compte/profil.vue (onglet Mes points)
   de modération/réaction existants)          └─ NavBar + AdminSidebar (entrées)
```

**5 tables** : `compte` (1‑1 utilisateur), `mouvement_points` (journal append-only, idempotent), `regle_points`, `palier_popularite`, `niveau`.

## 3. Détail d'implémentation

### Le moteur (`services/engagement.rs`)

Calqué sur `services/audit.rs` : **non-bloquant** : une validation / like / jugement ne peut pas échouer à cause des points.

| Propriété | Mécanisme |
|---|---|
| Idempotence | `cle_idempotence TEXT UNIQUE` + `INSERT … ON CONFLICT DO NOTHING` |
| Plancher 0 | `GREATEST(0, solde + delta)` |
| Réputation séparée | compteur distinct, hors calcul de niveau |
| Plafonds anti-abus | journalier/mensuel par règle, avec écrêtage tracé |
| Reset mensuel | paresseux (pas de cron), au 1er mouvement du mois |
| Niveau dérivé | recalculé et dénormalisé à chaque mutation |

Fonctions : `attribuer` · `retirer` · `evaluer_popularite` · `ajuster` · recalcul de niveau.

### Barème câblé (déclencheurs ajoutés aux mutations existantes)

| Règle | Points | Déclenché depuis |
|---|---|---|
| `contribution_validee` | +2 | Codimoi, VidAfrica (piste), BadHabit, IdeaForce → passage `publie` |
| `contribution_mise_en_avant` | +5 | **(non câblé, pas de colonne « vedette »)** |
| `factcheck_valide` | +3 / +1 rép. | FactCheck → `publie` |
| `factcheck_faux` | −2 / −3 rép. | FactCheck → `suspendu` |
| `popularite_palier` | 100/500/1000 likes → +10/+30/+50 | likes Codimoi, FactCheck, Biblio, Vidéo |
| `ajustement_admin` | variable | back-office |

Tous **anti-auto-attribution** (auteur ≠ modérateur / likeur).

### Endpoints

- **Public** (JWT) : `GET /api/engagement/mon-compte`, `/mon-journal`, `/niveau/{id}` (badge léger).
- **Admin** (permission `engagement.gerer`, **audités**) : CRUD `regles`/`paliers`/`niveaux`, `journal` filtrable, `ajustement` manuel.

### UI

- **Public** (Tailwind pur) : onglet « Mes points » (solde global/mensuel, statut, réputation, progression, historique) + entrée menu NavBar (`?onglet=mes-points`) + badge sous le profil public.
- **Admin** (daisyUI) : écran Barème (édition à chaud, sans redéploiement) + Journal + ajustement manuel.

## 4. Validation effectuée (runtime réel)

| Vérification | Résultat |
|---|---|
| Compilation backend (conteneur Rust, edition 2024) | ✅ 0 erreur |
| Migration + seed en base | ✅ 5 tables, 6 règles / 3 paliers / 3 niveaux, permission |
| Boucle : ajustement +250 → niveau **premium** auto | ✅ |
| US1 : validation Codimoi → auteur +2 | ✅ |
| Idempotence : re-validation → pas de doublon | ✅ |
| Gardes JWT / permission (401) | ✅ |
| UI « Mes points » (badge Premium + historique) | ✅ |

## 5. Reste à faire

### Dans le périmètre Phase 1 (2 points en attente)

- **Mise en avant (+5)** : aucune colonne « vedette » n'existe → décision produit + migration requises avant câblage.
- **Popularité fiche pays** : écartée (contenu collaboratif sans auteur unique).

### Phases futures (non commencées)

- **Partages réseaux sociaux externes** (+10 par 5 partages, plafond 30/j) → tracking de clics de boutons.
- **Quiz / jeux / concours** → infrastructure ludique inexistante à construire d'abord.
- **Cadeaux entre utilisateurs** (Gô 20 / Boro 50 / Digbate 100 / Lass 300 / Viemogo 500) → choisir modèle A (transfert) ou B (symbolique).
- **Cadeaux partenaires touristiques** → module admin dédié.
- **Visibilité algorithmique** Premium/Platinum (ranking, « à la une »).
- **Monétisation** : publicité payante + module de dons (Mobile Money / CB / PayPal) → passerelle de paiement à intégrer (le plus lourd).

## 6. Décisions produit

**Tranchées** : visibilité algo reportée · lancement non rétroactif · pas de clawback (sauf factcheck faux) · points ↔ réputation séparés · seuils paramétrables (Membre 0 / Premium 200 / Platinum 1000).

**À confirmer** : mapping FactCheck (retenu : `publie`=correct / `suspendu`=faux ; alternative = lier au champ `verdict`) · calibration fine du barème · modèle des cadeaux entre utilisateurs.

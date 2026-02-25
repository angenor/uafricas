# Retrouve Amis — Plan Frontend

## Structure des fichiers

```
uafricas_frontend/app/
├── pages/
│   ├── retrouve-amis/
│   │   ├── index.vue                    # Page d'accueil + recherche rapide
│   │   ├── nouveau.vue                  # Formulaire multi-étapes (créer un avis)
│   │   ├── mes-recherches.vue           # Mes avis de recherche
│   │   ├── correspondances.vue          # Liste de mes correspondances
│   │   ├── correspondances/
│   │   │   └── [id].vue                 # Détail correspondance + messagerie
│   │   └── parametres.vue               # Préférences de trouvabilité
│   └── admin/
│       └── retrouve-amis/
│           ├── index.vue                # Admin : liste des avis
│           └── signalements.vue         # Admin : modération signalements
├── components/
│   └── retrouve-amis/
│       ├── RetrouvAmisHero.vue          # Hero section page d'accueil
│       ├── RetrouvAmisStats.vue         # Statistiques publiques
│       ├── RetrouvAmisSearchForm.vue    # Formulaire de recherche rapide
│       ├── RetrouvAmisAvisCard.vue      # Carte d'un avis (résultat)
│       ├── RetrouvAmisAvisForm.vue      # Formulaire multi-étapes
│       ├── RetrouvAmisStepIndicator.vue # Indicateur d'étape du formulaire
│       ├── RetrouvAmisCritereInput.vue  # Input dynamique pour un critère
│       ├── RetrouvAmisCorrespondanceCard.vue  # Carte d'une correspondance
│       ├── RetrouvAmisScoreBadge.vue    # Badge de score (couleur selon niveau)
│       ├── RetrouvAmisMessageBubble.vue # Bulle de message (messagerie)
│       ├── RetrouvAmisContactShare.vue  # Modal partage de contact
│       ├── RetrouvAmisSignalForm.vue    # Formulaire de signalement
│       └── RetrouvAmisPreferences.vue   # Formulaire de préférences
└── composables/
    ├── useRetrouvAmis.ts                # API publique
    └── useAdminRetrouvAmis.ts           # API admin
```

---

## Pages

### 1. Page d'accueil — `retrouve-amis/index.vue`

```
┌──────────────────────────────────────────────────┐
│                    HERO SECTION                    │
│                                                    │
│          Retrouvez vos amis perdus de vue          │
│     Grâce au recoupement d'informations, UAfricas  │
│     vous aide à retrouver ceux que vous avez       │
│     perdus de vue.                                 │
│                                                    │
│  [🔍 Rechercher quelqu'un]  [📝 Déposer un avis]  │
│                                                    │
├──────────────────────────────────────────────────┤
│                  STATISTIQUES                      │
│                                                    │
│   🔍 342 recherches     ✅ 47 retrouvailles        │
│   actives               réussies                   │
│                                                    │
├──────────────────────────────────────────────────┤
│              COMMENT ÇA MARCHE ?                   │
│                                                    │
│  1. Décrivez la personne    2. On croise les       │
│     que vous cherchez          informations        │
│                                                    │
│  3. Correspondance trouvée  4. Échangez en         │
│     → notification             toute sécurité      │
│                                                    │
├──────────────────────────────────────────────────┤
│            RECHERCHE RAPIDE                        │
│                                                    │
│  Nom: [____________]  Ville: [____________]        │
│  École: [__________]  Pays:  [▼ Sélectionner]     │
│  Période: [19__] à [20__]                          │
│                                                    │
│              [Rechercher]                           │
│                                                    │
├──────────────────────────────────────────────────┤
│          RÉSULTATS (si recherche)                   │
│                                                    │
│  ┌─────────────────┐  ┌─────────────────┐         │
│  │ Avis #1         │  │ Avis #2         │         │
│  │ Score: 78%      │  │ Score: 65%      │         │
│  │ Ville: Douala   │  │ Ville: Douala   │         │
│  │ École: Lycée... │  │ Quartier: ...   │         │
│  │ [Voir détails]  │  │ [Voir détails]  │         │
│  └─────────────────┘  └─────────────────┘         │
│                                                    │
│  💡 Créez un compte pour entrer en contact         │
│     avec les personnes qui correspondent           │
│                                                    │
└──────────────────────────────────────────────────┘
```

**Comportement :**
- La recherche rapide fonctionne sans compte (résultats anonymisés).
- Avec compte : bouton "Voir détails" mène à la correspondance.
- Sans compte : CTA vers inscription.

### 2. Formulaire multi-étapes — `retrouve-amis/nouveau.vue`

```
┌──────────────────────────────────────────────────┐
│  Étape 1/5 ─── ● ─── ○ ─── ○ ─── ○ ─── ○       │
│                                                    │
│  QUI CHERCHEZ-VOUS ?                               │
│                                                    │
│  Nom de famille: [__________________]              │
│  Prénom:         [__________________]              │
│  Surnom:         [__________________] (optionnel)  │
│                                                    │
│  💡 Plus vous donnez d'informations, plus les      │
│     chances de retrouver cette personne sont       │
│     élevées.                                       │
│                                                    │
│                          [Suivant →]               │
├──────────────────────────────────────────────────┤
│  Étape 2/5 ─── ● ─── ● ─── ○ ─── ○ ─── ○       │
│                                                    │
│  OÙ L'AVEZ-VOUS CONNU(E) ?                        │
│                                                    │
│  Pays:      [▼ Cameroun        ]                   │
│  Ville:     [__________________]                   │
│  Quartier:  [__________________] (optionnel)       │
│  École:     [__________________] (optionnel)       │
│  Université:[__________________] (optionnel)       │
│  Entreprise:[__________________] (optionnel)       │
│                                                    │
│  [← Précédent]              [Suivant →]            │
├──────────────────────────────────────────────────┤
│  Étape 3/5 ─── ● ─── ● ─── ● ─── ○ ─── ○       │
│                                                    │
│  QUELLE PÉRIODE ?                                  │
│                                                    │
│  Années approximatives: [1998] à [2005]            │
│  Tranche d'âge à l'époque: [▼ 15-20 ans]          │
│  Votre relation: [▼ Camarade de classe]            │
│                                                    │
│  [← Précédent]              [Suivant →]            │
├──────────────────────────────────────────────────┤
│  Étape 4/5 ─── ● ─── ● ─── ● ─── ● ─── ○       │
│                                                    │
│  DÉTAILS SUPPLÉMENTAIRES (optionnel)               │
│                                                    │
│  Description physique ou traits distinctifs:        │
│  [                                        ]        │
│  [                                        ]        │
│                                                    │
│  Anecdote partagée (pour vérification):            │
│  [                                        ]        │
│  [                                        ]        │
│                                                    │
│  💡 L'anecdote ne sera révélée qu'après            │
│     confirmation mutuelle.                         │
│                                                    │
│  [← Précédent]              [Suivant →]            │
├──────────────────────────────────────────────────┤
│  Étape 5/5 ─── ● ─── ● ─── ● ─── ● ─── ●       │
│                                                    │
│  VOS PRÉFÉRENCES                                   │
│                                                    │
│  ☑ Rester anonyme (votre identité ne sera          │
│    pas révélée avant confirmation mutuelle)         │
│                                                    │
│  Titre de votre avis:                              │
│  [Je cherche mon ami d'enfance de Douala  ]        │
│                                                    │
│  [← Précédent]     [📝 Déposer mon avis]          │
│                                                    │
│  🔒 Vos informations sont protégées.               │
│     Seul le système accède à votre identité.       │
│                                                    │
└──────────────────────────────────────────────────┘
```

**Comportement :**
- Chaque étape est un composant enfant.
- Validation côté client avant passage à l'étape suivante.
- Au moins 2 critères obligatoires (nom + un autre).
- L'étape 5 génère un titre par défaut si non renseigné.
- Soumission via `useRetrouvAmis().creerAvis()`.

### 3. Mes recherches — `retrouve-amis/mes-recherches.vue`

```
┌──────────────────────────────────────────────────┐
│  MES RECHERCHES                [+ Nouveau]         │
│                                                    │
│  Filtres: [Tous ▼] [Actifs] [En pause] [Résolus]  │
│                                                    │
│  ┌────────────────────────────────────────────┐   │
│  │ 🔍 Je cherche mon ami d'enfance de Douala  │   │
│  │ État: ● Actif                               │   │
│  │ Critères: Kamga, Douala, Lycée Joss, 1998  │   │
│  │ Correspondances: 2 potentielles             │   │
│  │ Créé le: 15 jan 2026                        │   │
│  │                                              │   │
│  │ [Voir correspondances] [Modifier] [⏸ Pause] │   │
│  └────────────────────────────────────────────┘   │
│                                                    │
│  ┌────────────────────────────────────────────┐   │
│  │ 🔍 Recherche de ma cousine de Yaoundé      │   │
│  │ État: ● Résolu ✅                           │   │
│  │ Retrouvée le: 3 fév 2026                   │   │
│  └────────────────────────────────────────────┘   │
│                                                    │
└──────────────────────────────────────────────────┘
```

### 4. Correspondances — `retrouve-amis/correspondances.vue`

```
┌──────────────────────────────────────────────────┐
│  MES CORRESPONDANCES                               │
│                                                    │
│  Filtres: [Toutes] [En attente] [Validées]         │
│                                                    │
│  ┌────────────────────────────────────────────┐   │
│  │ ✅ Correspondance validée — Score: 92%      │   │
│  │ Avis: "Je cherche mon ami de Douala"        │   │
│  │ Critères communs: Nom, Ville, École         │   │
│  │ 💬 3 messages non lus                       │   │
│  │ [Ouvrir la conversation →]                  │   │
│  └────────────────────────────────────────────┘   │
│                                                    │
│  ┌────────────────────────────────────────────┐   │
│  │ ⏳ En attente de confirmation — Score: 71%   │   │
│  │ Avis: "Recherche ami d'enfance"             │   │
│  │ Critères communs: Ville, Période            │   │
│  │ Vous avez confirmé ✓ — En attente de l'autre│   │
│  └────────────────────────────────────────────┘   │
│                                                    │
│  ┌────────────────────────────────────────────┐   │
│  │ 🔔 Nouvelle correspondance — Score: 65%     │   │
│  │ Critères communs: Nom (similaire), Ville    │   │
│  │ [✅ Confirmer]  [❌ Rejeter]                 │   │
│  └────────────────────────────────────────────┘   │
│                                                    │
└──────────────────────────────────────────────────┘
```

### 5. Messagerie — `retrouve-amis/correspondances/[id].vue`

```
┌──────────────────────────────────────────────────┐
│  ← Retour    Correspondance avec [Prénom N.]       │
│              Score: 92% | Validée le 5 fév 2026    │
│                                                    │
│  ┌─── CRITÈRES COMMUNS ──────────────────────┐    │
│  │ ✓ Nom: Kamga ↔ Kamga (100%)               │    │
│  │ ✓ Ville: Douala ↔ Douala (100%)           │    │
│  │ ✓ École: Lycée Joss ↔ Lycee Joss (95%)   │    │
│  │ ✓ Période: 1998-2005 ↔ 1997-2004 (90%)   │    │
│  └────────────────────────────────────────────┘   │
│                                                    │
│  ─────────── MESSAGES ───────────                  │
│                                                    │
│       Bonjour ! Je pense que nous étions           │
│       dans la même classe en terminale.            │
│                                        15:32 ✓    │
│                                                    │
│  Oui ! Tu te souviens du prof de maths             │
│  M. Nkoulou ? 😄                                  │
│  15:45                                             │
│                                                    │
│       Bien sûr ! C'est bien toi alors !            │
│       Je suis content de te retrouver.             │
│                                        15:48 ✓    │
│                                                    │
│  ─────────── ZONE DE SAISIE ───────────           │
│                                                    │
│  [Message...                            ] [📤]     │
│                                                    │
│  [📱 Partager mon numéro]  [📞 Partager un        │
│                               contact proche]      │
│                                                    │
│  🔒 Les contacts partagés sont chiffrés et         │
│     visibles uniquement par votre correspondant.   │
│                                                    │
└──────────────────────────────────────────────────┘
```

**Modal de partage de contact :**

```
┌────────────────────────────────────┐
│  PARTAGER UN CONTACT               │
│                                    │
│  Type: ○ Mon téléphone             │
│        ○ Mon email                 │
│        ○ Téléphone d'un proche     │
│                                    │
│  Numéro/Email: [+237 6__ __ __ __]│
│  Nom du proche: [_____________]    │
│  (si contact d'un proche)          │
│                                    │
│  Message: [On peut s'appeler      ]│
│           [ce weekend !           ]│
│                                    │
│  ⚠️  Ce contact sera visible       │
│     uniquement par votre           │
│     correspondant.                 │
│                                    │
│  [Annuler]      [📤 Partager]      │
│                                    │
└────────────────────────────────────┘
```

### 6. Préférences — `retrouve-amis/parametres.vue`

```
┌──────────────────────────────────────────────────┐
│  PRÉFÉRENCES DE TROUVABILITÉ                       │
│                                                    │
│  ☑ Accepter d'être trouvé(e)                       │
│                                                    │
│  💡 En activant cette option, les personnes qui    │
│     vous cherchent pourront vous trouver grâce     │
│     aux informations ci-dessous. Votre identité    │
│     ne sera révélée qu'après confirmation          │
│     mutuelle.                                      │
│                                                    │
│  Anciens noms/prénoms:                             │
│  [+ Ajouter]  [Kamga] [x]  [Marie] [x]            │
│                                                    │
│  Villes où vous avez vécu:                         │
│  [+ Ajouter]  [Douala] [x]  [Yaoundé] [x]         │
│                                                    │
│  Écoles fréquentées:                               │
│  [+ Ajouter]  [Lycée Joss] [x]                    │
│                                                    │
│  Entreprises:                                      │
│  [+ Ajouter]                                       │
│                                                    │
│  Période: de [1995] à [2010]                       │
│                                                    │
│  [💾 Enregistrer]                                  │
│                                                    │
│  🔒 Ces informations sont utilisées uniquement     │
│     pour le matching. Elles ne sont jamais         │
│     affichées publiquement.                        │
│                                                    │
└──────────────────────────────────────────────────┘
```

---

## Composable — `useRetrouvAmis.ts`

```typescript
export interface AvisRecherche {
  id: string
  titre: string
  description?: string
  relation?: string
  est_anonyme: boolean
  etat: string
  nombre_vues: number
  criteres: Critere[]
  nombre_correspondances: number
  auteur?: { id: string; prenom: string; nom: string }
  created_at: string
}

export interface Critere {
  id: string
  type: string       // 'nom', 'ville', 'ecole', etc.
  valeur: string
  poids: number
  pays_nom?: string
}

export interface Correspondance {
  id: string
  score: number
  etat: string
  avis_titre: string
  criteres_communs: CritereCommun[]
  confirme_a: boolean
  confirme_b: boolean
  autre_partie?: { id: string; prenom: string; nom: string }
  created_at: string
  expire_le: string
}

export interface CritereCommun {
  type: string
  valeur_a: string
  valeur_b: string
  score: number
}

export interface Message {
  id: string
  auteur_id: string
  auteur_nom: string
  contenu: string
  est_contact: boolean
  type_contact?: string
  valeur_contact?: string
  nom_contact?: string
  lu: boolean
  created_at: string
}

export const TYPES_CRITERE = [
  { value: 'nom', label: 'Nom de famille' },
  { value: 'prenom', label: 'Prénom' },
  { value: 'surnom', label: 'Surnom' },
  { value: 'ecole', label: 'École' },
  { value: 'universite', label: 'Université' },
  { value: 'entreprise', label: 'Entreprise' },
  { value: 'quartier', label: 'Quartier' },
  { value: 'ville', label: 'Ville' },
  { value: 'pays', label: 'Pays' },
  { value: 'annee_debut', label: 'Année de début' },
  { value: 'annee_fin', label: 'Année de fin' },
  { value: 'tranche_age', label: 'Tranche d\'âge' },
  { value: 'description', label: 'Description' },
  { value: 'anecdote', label: 'Anecdote partagée' },
]

export const RELATIONS = [
  'Ami d\'enfance', 'Camarade de classe', 'Voisin',
  'Collègue', 'Cousin/Cousine', 'Ancien professeur',
  'Membre de famille éloigné', 'Autre'
]

export const useRetrouvAmis = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  // --- Avis de recherche ---
  const listerAvis = async (filtres: AvisQueryParams): Promise<AvisListeResponse>
  const obtenirAvis = async (id: string): Promise<AvisRecherche>
  const creerAvis = async (form: CreerAvisForm): Promise<AvisRecherche>
  const modifierAvis = async (id: string, form: ModifierAvisForm): Promise<AvisRecherche>
  const supprimerAvis = async (id: string): Promise<void>
  const changerEtatAvis = async (id: string, etat: string): Promise<void>
  const mesRecherches = async (): Promise<AvisRecherche[]>

  // --- Recherche ---
  const rechercher = async (criteres: CritereForm[]): Promise<AvisListeResponse>
  const rechercheAnonyme = async (criteres: CritereForm[]): Promise<AvisListeResponse>
  const statistiques = async (): Promise<StatsResponse>

  // --- Correspondances ---
  const mesCorrespondances = async (): Promise<Correspondance[]>
  const obtenirCorrespondance = async (id: string): Promise<Correspondance>
  const confirmerCorrespondance = async (id: string): Promise<void>
  const rejeterCorrespondance = async (id: string): Promise<void>

  // --- Messages ---
  const listerMessages = async (correspondanceId: string): Promise<Message[]>
  const envoyerMessage = async (correspondanceId: string, contenu: string): Promise<Message>
  const partagerContact = async (correspondanceId: string, form: PartagerContactForm): Promise<Message>
  const marquerLu = async (correspondanceId: string): Promise<void>

  // --- Préférences ---
  const obtenirPreferences = async (): Promise<Preferences>
  const modifierPreferences = async (form: PreferencesForm): Promise<void>

  // --- Signalement ---
  const signaler = async (form: SignalementForm): Promise<void>

  // --- Helpers ---
  const scoreLabel = (score: number): string => {
    if (score >= 80) return 'Très probable'
    if (score >= 60) return 'Probable'
    if (score >= 40) return 'Possible'
    return 'Faible'
  }

  const scoreCouleur = (score: number): string => {
    if (score >= 80) return 'badge-success'
    if (score >= 60) return 'badge-info'
    if (score >= 40) return 'badge-warning'
    return 'badge-error'
  }

  const etatLabel = (etat: string): string => {
    const labels: Record<string, string> = {
      'brouillon': 'Brouillon',
      'actif': 'Actif',
      'en_pause': 'En pause',
      'resolu': 'Résolu',
      'expire': 'Expiré',
      'modere': 'Modéré',
    }
    return labels[etat] || etat
  }

  return {
    listerAvis, obtenirAvis, creerAvis, modifierAvis,
    supprimerAvis, changerEtatAvis, mesRecherches,
    rechercher, rechercheAnonyme, statistiques,
    mesCorrespondances, obtenirCorrespondance,
    confirmerCorrespondance, rejeterCorrespondance,
    listerMessages, envoyerMessage, partagerContact, marquerLu,
    obtenirPreferences, modifierPreferences,
    signaler,
    scoreLabel, scoreCouleur, etatLabel,
  }
}
```

---

## Navigation

Ajout dans la NavBar existante (`app/components/layout/NavBar.vue`) :

```
Menu principal → "Retrouve Amis" → /retrouve-amis
```

Ajout dans le menu utilisateur (si connecté) :

```
Mon profil → "Mes recherches" → /retrouve-amis/mes-recherches
           → "Mes correspondances" → /retrouve-amis/correspondances
           → "Préférences trouvabilité" → /retrouve-amis/parametres
```

Ajout dans le sidebar admin :

```
Administration → "Retrouve Amis"
              → "Avis de recherche" → /admin/retrouve-amis
              → "Signalements" → /admin/retrouve-amis/signalements
```

---

## Composants daisyUI utilisés

| Composant | Usage |
|-----------|-------|
| `card` | Cartes d'avis et correspondances |
| `badge` | Score, état, nombre de messages |
| `steps` | Indicateur d'étape du formulaire |
| `modal` | Partage de contact, signalement, confirmation |
| `chat` / `chat-bubble` | Messagerie entre correspondants |
| `form-control` / `input` / `select` / `textarea` | Formulaires |
| `btn` | Boutons d'action |
| `alert` | Messages d'information/avertissement |
| `tabs` | Filtres (Tous / Actifs / En pause / Résolus) |
| `stat` | Statistiques publiques |
| `tooltip` | Explications contextuelles |
| `progress` | Barre de score de correspondance |
| `indicator` | Badge de notification (messages non lus) |

---

## Responsive

| Breakpoint | Disposition |
|------------|-------------|
| Mobile (`< 640px`) | 1 colonne, formulaire en pleine largeur, messagerie adaptée |
| Tablette (`640-1024px`) | 2 colonnes pour les cartes, sidebar réduit |
| Desktop (`> 1024px`) | 3 colonnes pour les cartes, messagerie en split-view |

---

## Animations

- **AOS** (déjà intégré) : fade-in des cartes de résultats au scroll.
- **Transition** : slide entre les étapes du formulaire multi-étapes.
- **Badge score** : animation pulse quand nouveau match détecté.

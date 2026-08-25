# Retrouve Amis : Plan d'implémentation

## Vue d'ensemble

**Retrouve Amis** est une fonctionnalité de la plateforme UAfricas permettant de retrouver des amis perdus de vue grâce au recoupement d'informations. Un utilisateur dépose une "avis de recherche" décrivant la personne qu'il cherche (nom, école, ville, période, etc.), et le système croise ces informations avec :

1. **Les autres avis de recherche** : quelqu'un cherche peut-être aussi cette personne, ou la personne perdue cherche aussi l'utilisateur.
2. **Les profils des utilisateurs inscrits** (uniquement ceux ayant consenti à être trouvables).

3. etc...

Quand un recoupement produit une correspondance suffisante, les deux parties sont notifiées et peuvent décider de partager leurs coordonnées de manière sécurisée.

---

## Principes fondamentaux

### 1. Anonymat par défaut
- Un utilisateur peut chercher **sans révéler son identité** aux autres.
- L'avis de recherche n'affiche jamais le nom du chercheur publiquement.
- Seul le système connaît l'identité du chercheur (pour empêcher les abus).

### 2. Consentement mutuel
- Aucune information de contact n'est partagée sans l'accord **des deux parties**.
- Le système utilise un mécanisme de "double opt-in" : les deux personnes doivent confirmer avant tout échange de coordonnées.

### 3. Sécurité des données
- Les informations sensibles (téléphone, email de contact) sont chiffrées au repos.
- Les données de recherche sont automatiquement supprimées après 12 mois sans activité.
- Limitation du nombre de recherches pour éviter le scraping.

### 4. Protection contre les abus
- Vérification d'email obligatoire (compte actif) pour créer un avis.
- Système de signalement intégré.
- Modération admin des avis signalés.
- Rate limiting sur les recherches.

---

## Architecture technique

### Nouveau schema PostgreSQL
Un nouveau schema `retrouve_amis` sera ajouté aux 10 schemas existants, dans la logique bounded-context du projet.

**Tables principales :**

| Table | Description |
|-------|-------------|
| `avis_recherche` | Avis de recherche déposé par un utilisateur |
| `critere_recherche` | Critères de recherche (nom, école, ville, période...) |
| `correspondance` | Correspondances trouvées par l'algorithme |
| `message_correspondance` | Messages échangés entre correspondants |
| `signalement` | Signalements d'abus |
| `preference_trouvabilite` | Préférences utilisateur pour être trouvable |

### Fichiers à créer/modifier

#### Backend (Rust)
| Fichier | Action |
|---------|--------|
| `doc/bd/schemas/16_retrouve_amis.sql` | Nouveau, schema SQL |
| `src/models/retrouve_amis.rs` | Nouveau, modèles Rust |
| `src/models/admin/retrouve_amis.rs` | Nouveau, modèles admin |
| `src/handlers/retrouve_amis.rs` | Nouveau, handlers publics |
| `src/handlers/admin/retrouve_amis.rs` | Nouveau, handlers admin |
| `src/routes.rs` | Modifier : ajouter routes |
| `src/models/mod.rs` | Modifier : déclarer module |
| `src/handlers/mod.rs` | Modifier : déclarer module |

#### Frontend (Nuxt 4)
| Fichier | Action |
|---------|--------|
| `app/pages/retrouve-amis/index.vue` | Page d'accueil de la fonctionnalité |
| `app/pages/retrouve-amis/nouveau.vue` | Formulaire de dépôt d'avis |
| `app/pages/retrouve-amis/mes-recherches.vue` | Mes avis de recherche |
| `app/pages/retrouve-amis/correspondances.vue` | Mes correspondances |
| `app/pages/retrouve-amis/correspondances/[id].vue` | Détail d'une correspondance + messagerie |
| `app/pages/retrouve-amis/parametres.vue` | Paramètres de trouvabilité |
| `app/pages/admin/retrouve-amis/index.vue` | Administration des avis |
| `app/pages/admin/retrouve-amis/signalements.vue` | Modération des signalements |
| `app/components/retrouve-amis/` | Composants (Hero, Card, Filters, etc.) |
| `app/composables/useRetrouvAmis.ts` | Composable public |
| `app/composables/useAdminRetrouvAmis.ts` | Composable admin |

---

## Flux utilisateur

### Flux 1 : Déposer un avis de recherche

```
[Utilisateur connecté]
    │
    ▼
[Page "Retrouve Amis" : description + CTA]
    │
    ▼
[Formulaire multi-étapes]
    ├── Étape 1 : Qui cherchez-vous ? (nom, prénom, surnom)
    ├── Étape 2 : Contexte (école, entreprise, quartier, ville, pays)
    ├── Étape 3 : Période (années approximatives, tranche d'âge)
    ├── Étape 4 : Détails supplémentaires (description physique, anecdote)
    └── Étape 5 : Vos préférences (anonymat, mode de contact)
    │
    ▼
[Avis créé : état "actif"]
    │
    ▼
[Algorithme de matching lancé en arrière-plan]
    │
    ├── Match trouvé → Notification aux deux parties
    └── Pas de match → Recherche périodique continue
```

### Flux 2 : Correspondance trouvée

```
[Notification : "Correspondance potentielle trouvée"]
    │
    ▼
[Page correspondance : affiche le score + critères communs]
    │
    ├── [Utilisateur A confirme : "Oui, c'est bien moi / la personne"]
    │       │
    │       ▼
    │   [En attente de confirmation de B]
    │
    └── [Utilisateur B confirme aussi]
            │
            ▼
        [Double opt-in validé]
            │
            ▼
        [Messagerie sécurisée ouverte]
            │
            ├── Échange de messages texte
            ├── Option : partager son numéro de téléphone
            └── Option : partager le contact d'un proche/intermédiaire
```

### Flux 3 : Recherche anonyme (non connecté)

```
[Visiteur non connecté]
    │
    ▼
[Page "Retrouve Amis" : formulaire de recherche rapide]
    │
    ▼
[Résultats : "X avis correspondent à vos critères"]
    │
    ├── Affiche les critères communs (sans identité du chercheur)
    └── CTA : "Créez un compte pour entrer en contact"
    │
    ▼
[Inscription / Connexion]
    │
    ▼
[Retour au résultat → possibilité de confirmer]
```

---

## Découpage en phases

### Phase 1 : MVP (prioritaire)
- Schema BD + migrations
- CRUD avis de recherche (backend + frontend)
- Algorithme de matching basique (correspondance exacte + fuzzy sur noms)
- Page de résultats avec score de correspondance
- Système de double opt-in
- Messagerie simple entre correspondants

### Phase 2 : Enrichissement
- Recherche anonyme (visiteurs non connectés)
- Préférences de trouvabilité (profil utilisateur)
- Notifications en temps réel (intégration existante ou polling)
- Administration et modération
- Système de signalement

### Phase 3 : Intelligence
- Algorithme de matching avancé (phonétique, translittération)
- Suggestions proactives basées sur les profils
- Matching croisé multi-avis (A cherche B, B cherche C, C connaît A)
- Statistiques et tableau de bord admin

---

## Documents détaillés

| Document | Contenu |
|----------|---------|
| [schema_bd.sql](./schema_bd.sql) | Schema PostgreSQL complet |
| [api_backend.md](./api_backend.md) | Endpoints API, modèles Rust, handlers |
| [frontend.md](./frontend.md) | Pages, composants, composables, maquettes |
| [securite.md](./securite.md) | Sécurité, confidentialité, RGPD, anti-abus |
| [algorithme_matching.md](./algorithme_matching.md) | Algorithme de recoupement et scoring |

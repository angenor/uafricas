# Research: Validation Admin des Bibliothèques Humaines

**Date**: 2026-04-22 | **Feature**: 001-admin-biblio-humaine

## Découvertes sur le code existant

### Architecture actuelle : Bibliothèques Humaines

Le backend possède déjà un module complet pour les bibliothèques humaines :

| Fichier | Rôle |
|---------|------|
| `src/handlers/bibliotheques_humaines.rs` | 4 endpoints publics (lister, détail, inscription, spécialités) |
| `src/models/bibliotheque_humaine.rs` | BiblioHumaineRow, BiblioHumaineResponse, InscriptionBiblioBody |
| `src/routes.rs` | Scope `/api/bibliotheques-humaines` |
| `app/composables/useBibliothequeHumaine.ts` | Composable frontend complet (lister, obtenir, inscrire, spécialités) |
| `app/pages/bibliotheque/humaine.vue` | Page publique avec formulaire d'inscription |

### Modèle de données actuel

**Problème clé** : L'inscription (`inscrire_biblio`) met directement `iam.utilisateur.bibliotheque_humain = TRUE` sans état intermédiaire. Aucune table de demande n'existe.

```sql
-- Champ actuel dans iam.utilisateur
bibliotheque_humain BOOLEAN NOT NULL DEFAULT FALSE

-- Index partiel existant
CREATE INDEX idx_utilisateur_biblio_humain ON iam.utilisateur(bibliotheque_humain)
    WHERE bibliotheque_humain = TRUE;

-- Tables connexes existantes
iam.specialite_bibliotheque (id, nom, slug)
iam.utilisateur_specialite (utilisateur_id, specialite_id)
```

### Pattern admin existant

Les modules admin suivent tous le même pattern :

| Composant | Implémentation |
|-----------|----------------|
| Composable | `useAdmin` base + `useAdminXxx` (ex: `useAdminCandidatures`) |
| Liste admin | `AdminDataTable` + `AdminFilters` + `AdminPageHeader` |
| Détail admin | Page `[id].vue` avec changement de statut via modal |
| Backend handler | `src/handlers/admin/xxx.rs` avec `verifier_permission!` |
| Backend model | `src/models/admin/xxx.rs` avec `COLONNES` const + `FromRow` |
| Route | Scope `/api/admin/xxx` dans `routes.rs` |

### Système de notifications

Un système de notifications existe (`src/handlers/notification.rs`) mais il utilise la table `arbre_genealogique.notifications`, spécifique à ce module. Pour la P3 (notifications), une solution simple sera ajoutée dans le schema `iam`.

---

## Décisions techniques

### D1 : Nouvelle table `iam.demande_biblio_humaine`

**Décision** : Créer une table dédiée pour les demandes, séparée du flag `bibliotheque_humain`.

**Justification** :
- Conserve l'historique des demandes (y compris rejetées)
- Autorise la réinscription après rejet (nouvelle demande)
- Permet le workflow en_attente → validé/rejeté sans modifier le profil utilisateur prématurément

**Alternative rejetée** : Ajouter un champ `statut_biblio` à `iam.utilisateur`, ne supporte pas l'historique ni les demandes multiples successives.

### D2 : Modification de `inscrire_biblio`

**Décision** : L'endpoint `POST /api/bibliotheques-humaines/inscription` crée maintenant une `demande_biblio_humaine` (statut `en_attente`) au lieu de directement passer `bibliotheque_humain = TRUE`.

**Impact** : Le profil utilisateur n'est modifié qu'au moment de la validation admin. La réponse retourne la demande créée (pas le profil biblio humaine).

**Contrainte** : Un utilisateur ne peut soumettre une nouvelle demande que si sa dernière est `rejete` (une seule demande `en_attente` ou `valide` à la fois).

### D3 : Validation applique les changements de profil en transaction

**Décision** : Quand l'admin valide, le handler exécute en transaction atomique :
1. `statut → valide` dans `demande_biblio_humaine`
2. `iam.utilisateur` : `bibliotheque_humain = TRUE`, `fonction`, `biographie`, `pays_origine_id`
3. Insertion des spécialités dans `iam.utilisateur_specialite`
4. `audit::log_action` non-bloquant

### D4 : Endpoint consultation demande propre

**Décision** : `GET /api/bibliotheques-humaines/moi/demande` (JWT requis) retourne la demande active de l'utilisateur connecté (statut + commentaire admin si rejeté).

### D5 : Notifications P3 simples

**Décision** : Créer `iam.notification_biblio_humaine` pour les notifications P3. Non bloquant pour P1/P2.

---

## Artefacts à créer

### Backend
| Fichier | Action |
|---------|--------|
| `uafricas_backend/doc/bd/schemas/04b_iam_biblio_demande.sql` | CRÉER, DDL nouvelle table + enum |
| `src/models/admin/biblio_humaine.rs` | CRÉER, modèles admin |
| `src/handlers/admin/bibliotheques_humaines.rs` | CRÉER, 4 handlers admin |
| `src/handlers/bibliotheques_humaines.rs` | MODIFIER, `inscrire_biblio` crée une demande |
| `src/models/bibliotheque_humaine.rs` | MODIFIER, ajouter types demande |
| `src/models/admin/mod.rs` | MODIFIER, déclarer `biblio_humaine` |
| `src/handlers/admin/mod.rs` | MODIFIER, déclarer `bibliotheques_humaines` |
| `src/routes.rs` | MODIFIER : routes admin + `moi/demande` |

### Frontend
| Fichier | Action |
|---------|--------|
| `app/composables/useBibliothequeHumaine.ts` | MODIFIER, ajouter `obtenirMaDemande` |
| `app/composables/useAdminBibliosHumaines.ts` | CRÉER, composable admin |
| `app/pages/admin/bibliotheques-humaines/index.vue` | CRÉER, liste des demandes |
| `app/pages/admin/bibliotheques-humaines/[id].vue` | CRÉER, détail + actions |
| `app/pages/profil.vue` | MODIFIER : afficher statut demande si en attente |

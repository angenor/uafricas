# Quickstart : Collaboration et Partage

## Prérequis

- Features 1-5 déployées
- 2 comptes utilisateurs (admin@test.com + user2@test.com)
- Arbre avec des personnes sur le compte admin

## Fichiers à créer/modifier

### Backend (~15 fichiers)

| Fichier | Action |
|---------|--------|
| `doc/bd/schemas/25_collaboration.sql` | Créer — tables invitations + collaborateurs + colonnes confidentialité |
| `src/models/collaboration.rs` | Créer — structs + DTOs |
| `src/handlers/collaboration.rs` | Créer — 12 handlers |
| `src/handlers/mod.rs` | Modifier — +pub mod collaboration |
| `src/models/mod.rs` | Modifier — +pub mod collaboration |
| `src/routes.rs` | Modifier — +12 routes dans scope /arbre |
| `src/handlers/arbre_genealogique.rs` | Modifier — vérification accès collaborateur sur arbre-complet |
| `src/services/matching.rs` | Modifier — filtre visible_matching et arbre_prive |

### Frontend (~8 fichiers)

| Fichier | Action |
|---------|--------|
| `app/pages/arbre-genealogique/index.vue` | Modifier — sections Mon arbre + Arbres partagés |
| `app/pages/arbre-genealogique/gestion.vue` | Créer — page gestion collaborateurs + confidentialité + historique |
| `app/composables/useCollaboration.ts` | Créer — API wrapper 12 endpoints |
| `app/components/arbre-genealogique/CarteInvitation.vue` | Créer — carte invitation (accepter/refuser) |
| `app/components/arbre-genealogique/BandeauLectureSeule.vue` | Créer — bandeau visuel lecture seule |
| `app/pages/arbre-genealogique/visualisation.vue` | Modifier — bandeau + masquer actions si lecture seule |
| `app/mocks/collaboration.ts` | Créer — types TS + mocks |

## Scénario de vérification

1. Se connecter admin@test.com → inviter user2@test.com en "Lecture seule"
2. Se connecter user2@test.com → accepter l'invitation
3. Vérifier que l'arbre d'admin apparaît dans "Arbres partagés" de user2
4. Ouvrir l'arbre partagé → vérifier le bandeau "Lecture seule" + pas de boutons d'action
5. Admin change la permission en "Édition" → user2 peut maintenant éditer
6. Admin marque une personne comme "privée" → vérifier qu'elle disparaît du matching
7. Consulter l'historique → vérifier les actions des deux utilisateurs

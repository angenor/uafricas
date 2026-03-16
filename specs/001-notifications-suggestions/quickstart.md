# Quickstart : Notifications et Suggestions Intelligentes

## Prérequis
- Features 1-6 déployées
- 2 comptes test avec arbres + collaboration active
- Extension pg_trgm activée

## Fichiers à créer/modifier

### Backend (~10 fichiers)
| Fichier | Action |
|---------|--------|
| `doc/bd/schemas/26_notifications.sql` | Créer — tables notifications + doublons_ignores |
| `src/models/notification.rs` | Créer |
| `src/handlers/notification.rs` | Créer — 7 handlers |
| `src/handlers/matching.rs` | Modifier — INSERT notification après matching |
| `src/handlers/collaboration.rs` | Modifier — INSERT notification après invitation/modification |
| `src/routes.rs` | Modifier — +7 routes |

### Frontend (~6 fichiers)
| Fichier | Action |
|---------|--------|
| `app/composables/useNotifications.ts` | Créer — compteur + liste + actions |
| `app/composables/useSuggestions.ts` | Créer — suggestions proactives côté client |
| `app/components/layout/ClocheNotifications.vue` | Créer — cloche + panneau déroulant |
| `app/components/arbre-genealogique/PanneauDoublons.vue` | Créer — détection + fusion |
| `app/pages/notifications.vue` | Créer — page complète |
| `app/layouts/default.vue` | Modifier — ajouter ClocheNotifications |

## Scénario de vérification
1. Ajouter une personne similaire à un autre arbre → notification matching
2. Vérifier la cloche dans la navbar → badge "+1"
3. Cliquer la cloche → panneau avec la notification
4. Consulter les suggestions → "Parents manquants de [X]"
5. Cliquer suggestion → redirigé vers l'arbre avec formulaire ouvert
6. Ajouter un doublon dans son arbre → détection affichée
7. Ignorer un doublon → ne réapparaît plus
8. Fusionner un doublon → une seule personne avec tous les liens

# 11 — Audit & Logs

> **Phase** : 4 — Modules transversaux
> **Section sidebar** : Audit & Logs
> **Icône** : faClockRotateLeft
> **Statut global** : [ ] Non démarré

---

## Dépendances

### Fichiers SQL requis
- `schemas/12_audit.sql` → `shared.audit_log`
- `schemas/04_iam.sql` → `utilisateur` (FK nullable = actions système)

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** — Utilisateurs (auteurs des actions auditées)
- **Tous les plans 02 à 10** — L'audit enregistre les actions de toutes les rubriques. Il est logique de l'implémenter après que les premiers modules CRUD sont en place pour avoir des données à auditer.

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** — Stats activité récente (dernières actions auditées)

### Backend existant
- [ ] Aucun handler audit — **À CRÉER**
- [ ] Aucun mécanisme d'enregistrement automatique des actions — **À CRÉER**

---

## Backend

### B11.1 — Service d'audit (enregistrement)
- [ ] Créer un module `src/services/audit.rs` — service d'enregistrement des événements d'audit
  - Fonction `log_action(pool, user_id, action, table_name, record_id, old_data, new_data, ip, user_agent)`
  - Actions : CREATE, UPDATE, DELETE, LOGIN
  - `old_data` / `new_data` : sérialisation JSONB avant/après
- [ ] Intégrer l'appel d'audit dans les handlers admin existants (après chaque CREATE/UPDATE/DELETE)
- **Fichiers** : `src/services/audit.rs`, `src/services/mod.rs`

### B11.2 — Endpoints de consultation
- [ ] `GET /api/admin/audit` — liste paginée + filtres :
  - Par action (CREATE/UPDATE/DELETE/LOGIN)
  - Par utilisateur (qui a fait l'action)
  - Par table cible (quelle entité)
  - Par date range
  - Par adresse IP
  - Recherche full-text sur les données before/after
- [ ] `GET /api/admin/audit/:id` — détail d'un événement :
  - Utilisateur + rôle au moment de l'action
  - Action effectuée
  - Table + record_id ciblé
  - Snapshot before/after (JSONB) avec diff visuel
  - IP + User Agent
  - Timestamp précis
- **Fichiers** : `src/handlers/admin/audit.rs`, `src/models/admin/audit.rs`

---

## Frontend

### Pages
- [ ] `app/pages/admin/audit/index.vue` — journal d'audit :
  - DataTable chronologique (plus récent en premier)
  - Filtres : action, utilisateur (autocomplete), table, date range
  - Colonnes : date, utilisateur, action, table, résumé
  - Code couleur par action (CREATE=vert, UPDATE=bleu, DELETE=rouge, LOGIN=gris)
- [ ] `app/pages/admin/audit/[id].vue` — détail événement :
  - En-tête : utilisateur, action, date, IP, User Agent
  - Section before/after : diff visuel côte à côte (JSON pretty-print avec highlighting des changements)
  - Lien vers l'entité concernée (si elle existe encore)

### Composables
- [ ] `app/composables/useAdminAudit.ts` — API client audit + filtres

### Composants spécifiques
- [ ] `app/components/admin/AdminJsonDiff.vue` — composant de diff JSON visuel (before/after avec highlighting)

---

## Critères de validation
- [ ] Le service d'audit enregistre automatiquement les actions CRUD admin
- [ ] Les filtres avancés fonctionnent (action, utilisateur, table, date, IP)
- [ ] Le diff JSON before/after est lisible et met en surbrillance les changements
- [ ] Les liens vers les entités concernées sont fonctionnels
- [ ] Les actions LOGIN sont enregistrées

---

## Notes
- L'audit est transversal : il dépend de tous les autres modules (pour avoir des actions à enregistrer) mais il peut être implémenté dès que quelques modules CRUD sont en place.
- Le composant `AdminJsonDiff` est spécifique à cette rubrique mais pourrait être réutilisé dans les contributions de profils pays (plan 10).
- L'enregistrement côté backend doit être non-bloquant (ne pas ralentir les opérations CRUD si l'écriture audit échoue).
- Le `user_id` est nullable dans `audit_log` pour les actions système (migrations, tâches cron, etc.).

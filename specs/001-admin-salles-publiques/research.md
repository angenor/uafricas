# Phase 0 — Research : Administrateurs de salle publique & propositions

**Feature** : 001-admin-salles-publiques
**Date** : 2026-05-10

Aucun marqueur `[NEEDS CLARIFICATION]` n'est resté dans la spec ; cette phase documente les **décisions techniques structurantes** prises sur la base de l'existant et de la constitution.

---

## Décision 1 — Faut-il rouvrir l'historique de la feature 005 (`proposition_salle` supprimée) ?

**Décision** : NON. Reconcevoir `afrolang.proposition_salle` *from scratch*, table simple et autonome, indépendante de l'ancienne implémentation supprimée par la refonte.

**Rationale** :
- L'ancienne table avait été conçue pour un workflow plus large (parrains, votes communautaires, modération multi-étages) jamais activé.
- Repartir d'une table minimale aligne avec le Principe V (Simplicité / YAGNI).
- Aucune donnée historique à migrer (la table avait été supprimée).

**Alternatives rejetées** :
- *Restaurer l'ancien schéma* : vivement déconseillé — contenait des colonnes `parrain_*`, `votes`, `etat_communautaire` qui dépassent largement la spec actuelle.
- *Réutiliser une table générique de demandes* : aucune table générique n'existe ; en créer une serait de la sur-ingénierie.

---

## Décision 2 — `salle_moderateur` vs nouvelle table `salle_administrateur`

**Décision** : Créer **une table distincte** `afrolang.salle_administrateur` ; ne **pas** réutiliser `afrolang.salle_moderateur`.

**Rationale** :
- FR-018 exige une distinction sémantique stricte entre rôles (« administrateur de salle » vs « administrateur de la plateforme » vs « modérateur attitré »).
- `salle_moderateur` porte une sémantique opérationnelle existante (modération de chat/contenu, validation de ressources) qui n'est *pas* la sémantique du nouveau rôle (FR-019 : capacités à définir plus tard).
- Mélanger les deux via une colonne `role` introduirait :
  - Des conditions `WHERE role = 'moderateur'` partout dans le code existant → risque élevé de régression sur la feature 005.
  - Une ambiguïté pour les futures capacités du rôle administrateur de salle.
- Tables séparées = découplage propre, futures capacités branchées sur une seule table sans condition.

**Alternatives rejetées** :
- *Colonne `role` sur `salle_moderateur`* : risque de régression sur les handlers existants qui présupposent que toutes les lignes sont des modérateurs.
- *Vue SQL `salle_administrateur` au-dessus de `salle_moderateur`* : sur-ingénierie, ne résout pas le problème de la distinction sémantique.

---

## Décision 3 — Atomicité validation proposition → création de salle

**Décision** : Validation effectuée dans une **transaction sqlx unique** :
1. `SELECT ... FOR UPDATE` sur la proposition (verrou ligne).
2. Vérifier statut = `en_attente`.
3. Vérifier qu'aucune `afrolang.salle` active n'existe déjà pour ce `groupe_ethnique_id` (l'index unique partiel `idx_afrolang_salle_groupe_unique` le garantit, mais on lève une 409 explicite avant).
4. `INSERT INTO afrolang.salle (...)` à partir des données de la proposition.
5. `INSERT INTO afrolang.salle_pays_origine` pour chaque pays d'origine listé dans la proposition (réutilise feature 001-afrolang-pays-origine).
6. `UPDATE afrolang.proposition_salle SET statut='validee', salle_id_creee=..., decideur=..., decide_at=NOW()`.
7. `audit::log_action('VALIDATE', 'afrolang', 'proposition_salle', proposition_id)`.
8. `audit::log_action('CREATE', 'afrolang', 'salle', salle_id)`.
9. Notification auteur (best effort, non bloquante).

**Rationale** : Empêche toute fenêtre où une salle serait créée sans que la proposition soit marquée validée (ou inversement). L'audit et la notification sortent de la transaction (best effort, conformes au pattern existant).

**Alternatives rejetées** :
- *Étape par étape sans transaction* : crée des incohérences si l'une des étapes échoue.
- *Création différée par worker* : sur-ingénierie, le volume (< 10/jour) ne justifie aucune file asynchrone.

---

## Décision 4 — Suspension automatique en cascade (FR-021, FR-022, SC-008)

**Décision** : Implémenter via **handlers existants étendus**, pas via trigger SQL.

- Quand un admin archive/désactive une salle publique : le handler existant qui désactive `afrolang.salle.actif=FALSE` est étendu pour `UPDATE afrolang.salle_administrateur SET actif=FALSE, suspendu_at=NOW(), motif_suspension='salle_archivee' WHERE salle_id=$1 AND actif=TRUE`.
- Quand un admin désactive un compte utilisateur : le handler `iam` existant est étendu pour `UPDATE afrolang.salle_administrateur SET actif=FALSE, suspendu_at=NOW(), motif_suspension='compte_desactive' WHERE utilisateur_id=$1 AND actif=TRUE`.

**Rationale** :
- Conforme Principe VII (audit) : un trigger SQL ne pourrait pas appeler `audit::log_action` (qui vit dans Rust).
- Plus lisible et testable manuellement (handler unique = une transaction, un audit groupé).
- Le coût (deux UPDATE supplémentaires dans des handlers admin déjà rares) est négligeable.

**Alternatives rejetées** :
- *Trigger AFTER UPDATE* : casse la traçabilité audit (pas d'IP, pas de user-agent).
- *Job cron de nettoyage* : latence incompatible avec SC-008 (60 s).

---

## Décision 5 — Notification : canal et délai

**Décision** : Réutiliser le mécanisme de notification existant — pour cette feature, **in-app uniquement** au minimum (insertion dans la table de notifications existante), e-mail SMTP best-effort si déjà branché pour les décisions de modération similaires (Bibliothèque Humaine notamment, cf. mémoire `001-admin-biblio-humaine`).

**Rationale** :
- SC-004 exige une notification < 60 s après la décision : un INSERT in-app + envoi SMTP non bloquant satisfait ce SLA.
- Pas de nouveau canal, pas de WebSocket dédié — Principe V (YAGNI).

**Alternatives rejetées** :
- *Notification temps réel via LiveKit data channel* : hors périmètre, salle pas forcément ouverte au moment de la décision.
- *Pousser sur un nouveau topic SSE* : aucune infra SSE en place, sur-ingénierie.

---

## Décision 6 — Anti-spam (edge case « tentatives répétées »)

**Décision** : Compteur léger côté backend lors de `POST /api/afrolang/propositions` :
- `SELECT COUNT(*) FROM afrolang.proposition_salle WHERE auteur_id=$1 AND statut='rejetee' AND decide_at > NOW() - INTERVAL '7 days'`
- Si ≥ 5 → 429 Too Many Requests, message « trop de propositions rejetées récentes, réessayez après le {decide_at + 7j} ».

**Rationale** : Aligné sur le pattern `services/afrolang_rate_limit.rs` existant (rate-limit code accès) ; pas de nouvelle table.

**Alternatives rejetées** :
- *Table dédiée d'historique anti-spam* : aucun gain, l'information est déjà dans `proposition_salle`.

---

## Décision 7 — Visibilité publique des administrateurs de salle

**Décision** : `GET /api/afrolang/salles/{id}` (endpoint public existant) est étendu d'un champ `administrateurs: Array<{ utilisateur_id, nom, prenom, photo_url, nomme_at }>` peuplé par `json_agg` filtré sur `actif=TRUE`.

**Rationale** :
- Conforme FR-017 (visibilité publique).
- Pas de nouvel endpoint — Principe V.
- `json_agg` est déjà utilisé dans `lister_salles` pour `pays_origine` (feature 001-afrolang-pays-origine), même pattern.

**Alternatives rejetées** :
- *Endpoint séparé `/salles/{id}/administrateurs`* : 1 round-trip HTTP supplémentaire pour rien.

---

## Décision 8 — Modèle d'autorisation pour pouvoirs futurs (FR-019)

**Décision** : Exposer un helper Rust **`est_administrateur_salle(pool, salle_id, user_id) -> Result<bool>`** dans `src/handlers/afrolang.rs`. Aucune capacité branchée à ce stade : le helper renvoie un booléen, prêt à être appelé par les futurs handlers qui implémenteront les pouvoirs.

**Rationale** :
- Point d'autorisation unique → migration future = brancher des appels au helper, sans toucher au schéma.
- Conforme FR-019 (« exposer le rôle via un point d'autorisation centralisé permettant d'y attacher des capacités plus tard sans rupture de compatibilité »).

**Alternatives rejetées** :
- *Système RBAC complet (table `permission`, `role_permission`)* : sur-ingénierie tant que les pouvoirs ne sont pas définis.

---

## Synthèse

| Sujet | Choix |
|-------|-------|
| Table proposition | Nouvelle, minimale, schéma `afrolang` |
| Rôle admin de salle | Table dédiée `salle_administrateur` (séparée de `salle_moderateur`) |
| Validation proposition | Transaction sqlx atomique |
| Cascade suspension | Handlers Rust existants étendus, pas de trigger SQL |
| Notification | Réutilise l'existant (in-app + SMTP best-effort) |
| Anti-spam | Compteur ad-hoc sur la table existante, 5 rejets / 7 jours |
| Visibilité admins | Champ `administrateurs[]` dans le DTO existant `SalleResponse` |
| Pouvoirs futurs | Helper `est_administrateur_salle(...)`, capacités à venir |

Aucune `NEEDS CLARIFICATION` non résolue.

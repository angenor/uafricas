# Research : Phase 0 : Système d'engagement (Phase 1)

Toutes les inconnues techniques sont résolues ci-dessous. Aucune ne reste marquée « NEEDS CLARIFICATION » (les 3 décisions produit ont été tranchées dans la spec : visibilité reportée, non-rétroactif, pas de clawback).

---

## D1 : Attribution non-bloquante : modèle du service

**Décision** : créer `src/services/engagement.rs` calqué **à l'identique** sur `src/services/audit.rs::log_action` : fonctions `async` prenant `&PgPool`, exécutant l'écriture, et **loguant l'erreur sans la propager** (`if let Err(e) = result { log::error!(...) }`). L'appelant (handler de modération ou de réaction) `.await` l'appel mais son résultat n'affecte jamais la réponse HTTP de l'action métier.

**Rationale** : c'est le pattern déjà éprouvé (~100 mutations auditées via `log_action`), zéro nouvelle abstraction (Principe V), et il satisfait FR-007 / SC-003 (l'action métier ne peut pas échouer à cause des points).

**Alternatives rejetées** :
- *Tâche détachée `tokio::spawn`* : complexifie la gestion d'erreur et le pool ; non justifié tant que l'écriture est brève. Rejeté (YAGNI).
- *File d'événements / outbox* : sur-ingénierie pour la Phase 1. Un job de **réconciliation** (re-scan des actions non créditées) est noté comme évolution future si des pertes sont observées.

---

## D2 : Idempotence de l'attribution

**Décision** : chaque mouvement porte une **clé d'idempotence** textuelle `cle_idempotence` avec contrainte `UNIQUE`. L'insertion se fait en `INSERT ... ON CONFLICT (cle_idempotence) DO NOTHING`. Convention de clé :
- contribution validée : `contribution_validee:{type_objet}:{objet_id}`
- mise en avant : `contribution_mise_en_avant:{type_objet}:{objet_id}`
- factcheck validé / faux : `factcheck_valide:{factcheck_id}` / `factcheck_faux:{factcheck_id}`
- palier de popularité : `popularite:{type_objet}:{objet_id}:{seuil_likes}`

**Rationale** : rend l'idempotence **structurelle** (garantie par la base, pas par la logique applicative) → satisfait FR-008 et SC-004 (zéro doublon) même en cas de rejeu (double validation, oscillation de likes autour d'un palier). Évite une table `palier_attribué` séparée : l'existence de la ligne journal `popularite:…:{seuil}` **est** la preuve que le palier a été récompensé (Principe V).

**Alternatives rejetées** :
- *Table dédiée `palier_popularite_attribue`* : redondante avec le journal. Rejetée.
- *Idempotence applicative (SELECT avant INSERT)* : sujette aux courses concurrentes. Rejetée au profit de la contrainte `UNIQUE`.

---

## D3 : Agrégation unifiée des « j'aime » (popularité)

**Constat** : les réactions sont éclatées sur des tables hétérogènes de forme uniforme, `id, <objet>_id, utilisateur_id, type_reaction ('like'|'dislike'), created_at, UNIQUE(<objet>_id, utilisateur_id)` : `culture.codimoi_reaction`, `governance.factcheck_reaction`, `iam.biblio_reaction`, `media_content.video_reaction`, réactions fiches pays (`country_profile`). **L'auteur n'est pas sur la table de réaction** mais sur la table de contenu.

**Décision** : ne PAS créer de vue matérialisée globale. À la place, **le déclencheur est local** : chaque handler qui enregistre un « like » appelle, en fin de traitement (non-bloquant), `engagement::evaluer_popularite(pool, type_objet, objet_id, auteur_id, likes_count)`. Le handler connaît déjà `objet_id`, sait résoudre l'auteur du contenu, et peut compter les likes (`COUNT(*) WHERE type_reaction='like'`, en excluant l'auto-like). Le service compare le compte aux `palier_popularite` actifs et crédite chaque palier franchi non encore récompensé (via la clé d'idempotence D2).

**Rationale** : chaque domaine reste responsable de sa table de réaction ; pas de couplage inverse ni de balayage périodique. La référence unifiée `(type_objet, objet_id)`, déjà utilisée par `country_profile.signalement_contribution`, sert de vocabulaire commun côté journal. Satisfait FR-015/016/017.

**`type_objet` : valeurs Phase 1** : `codimoi`, `factcheck`, `biblio_humaine`, `video`, `fiche_pays` (extensible). L'exclusion de l'auto-like se fait dans le `COUNT` (`AND utilisateur_id <> auteur_id`) ou en amont (un membre ne peut de toute façon pas atteindre un palier via son seul vote).

**Alternatives rejetées** :
- *Vue/matérialisée `engagement.publication` unifiant tous les contenus* : gros couplage transverse, maintenance lourde, contraire à YAGNI. Rejetée.
- *Trigger SQL sur chaque table de réaction* : logique métier (résolution auteur, plafonds, niveaux) mal placée en base ; difficile à auditer. Rejetée au profit du service Rust.

---

## D4 : Niveau : dérivé mais dénormalisé

**Décision** : le niveau est **calculé** à partir des seuils configurables (`engagement.niveau`) mais **stocké dénormalisé** sur `engagement.compte.niveau_code`, recalculé à chaque mutation de solde par `engagement::recalculer_niveau`.

**Rationale** : permet le tri/filtre et l'affichage du badge (profil, sous contenus) sans recalcul à la lecture, tout en gardant les seuils paramétrables. Satisfait FR-002/020 et SC-006.

**Alternatives rejetées** : *calcul pur à la lecture* → recalcul répété et jointure systématique sur `niveau` à chaque affichage de badge. Rejeté pour la perf d'affichage.

---

## D5 : Solde mensuel : réinitialisation paresseuse

**Décision** : `engagement.compte` porte `mois_courant DATE` (1er jour du mois de référence du `solde_mensuel`). À chaque attribution, si `mois_courant <> date_trunc('month', now())`, on remet `solde_mensuel = 0` et on met à jour `mois_courant` **avant** d'appliquer le nouveau mouvement. Pas de cron (cohérent avec les patterns « cloturer_si_necessaire » du projet).

**Rationale** : satisfait FR-004 sans tâche planifiée ; simple, robuste aux redémarrages.

---

## D6 : Plafonds anti-abus (écrêtage)

**Décision** : `engagement.regle_points` porte `plafond_journalier` et `plafond_mensuel` (nullable = illimité). Avant d'appliquer un gain, le service somme les points **déjà crédités pour ce `type_action`** sur la fenêtre (jour / mois) depuis le journal, calcule le crédit résiduel disponible, et **écrête**. Un mouvement écrêté est **tracé** (points effectivement crédités, éventuellement 0), jamais ignoré silencieusement.

**Rationale** : satisfait FR-010 et SC-007. Les malus (factcheck faux) ne sont pas plafonnés.

---

## D7 : Points ≥ 0 ; réputation indépendante

**Décision** : `solde_points` a un **plancher à 0** (un malus ne rend jamais le solde négatif : `GREATEST(0, solde - malus)`). La `reputation` est un **entier signé indépendant** du solde et n'entre pas dans le calcul de niveau (Phase 1). Le journal enregistre `points` (le delta réellement appliqué après plancher) et `reputation_delta` séparément.

**Rationale** : cohérent avec les *Assumptions* de la spec ; évite les incohérences d'affichage (solde négatif) tout en gardant la réputation comme signal de confiance distinct (FR-003/014).

---

## D8 : Sécurité & permission admin

**Décision** : endpoints publics protégés par JWT (`utilisateur_courant`) ; endpoints d'administration protégés par une **nouvelle permission `engagement`** (référentiel IAM existant, comme `mooc`, `media`, `gouvernance`…). La modification de barème et l'ajustement manuel de points sont **audités** via `log_action` (schema `engagement`).

**Rationale** : aligné sur le modèle d'autorisation existant et le Principe VII. Le badge d'un membre est exposé par un endpoint public **léger** (niveau + code seulement) ; le **journal détaillé reste privé** (titulaire ou admin), conformément à FR-019.

---

## D9 : Points d'intégration (déclencheurs) dans le code existant

Les appels au service sont **ajoutés** aux mutations déjà en place (aucune nouvelle route pour déclencher les gains) :

| Action récompensable | Handler existant à instrumenter | Appel service |
|----------------------|--------------------------------|---------------|
| Contribution Codimoi validée | modération Codimoi (`handlers/admin/codimoi_admin.rs`) | `attribuer(auteur, "contribution_validee", "codimoi", id, …)` |
| Piste VidAfrica publiée | `handlers/admin/vidafrica.rs::changer_etat_piste` (etat=`publie`) | `attribuer(auteur, "contribution_validee", "video", piste_id, …)` |
| Idée Ideaforces validée | modération Ideaforces (`handlers/admin/gouvernance.rs`) | `attribuer(auteur, "contribution_validee", "ideaforce", id, …)` |
| Bonne pratique BadGoodHabit validée | modération BadHabit (`handlers/admin/gouvernance.rs`) | `attribuer(auteur, "contribution_validee", "bad_habit", id, …)` |
| Contribution « mise en avant » | flag vedette/à la une correspondant | `attribuer(auteur, "contribution_mise_en_avant", …)` |
| FactCheck jugé correct/validé | modération FactCheck (`handlers/admin/gouvernance.rs`) | `attribuer(auteur, "factcheck_valide", "factcheck", id, …)` |
| FactCheck jugé faux | idem | `retirer(auteur, "factcheck_faux", "factcheck", id, …)` (points + réputation) |
| Like sur contenu | handlers de réaction (codimoi, factcheck, biblio, vidafrica, fiche) | `evaluer_popularite(type_objet, objet_id, auteur, likes)` |

Les noms de fonctions/flags exacts sont confirmés à l'implémentation (`/speckit.tasks`). Le contrat du service est stable ; seuls les call-sites varient.

**Note** : le mapping validation ↔ « contribution validée » dépend de l'état de modération de chaque domaine (ex. FactCheck `verdict IN ('vrai','faux',…)` + statut de publication). La règle retenue : on crédite quand la contribution **passe à l'état publié/validé** par un modérateur, une seule fois (idempotence D2).

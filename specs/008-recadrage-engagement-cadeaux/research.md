# Phase 0 — Research : recadrage de l'engagement & cadeaux virtuels

**Feature**: `008-recadrage-engagement-cadeaux` | **Date**: 2026-08-08

Ce document tranche les décisions techniques laissées ouvertes par la spécification et par la conception. Chaque entrée suit le format *Décision / Justification / Alternatives écartées*.

---

## R1 — Le recadrage du barème est une migration de données, pas une suppression de code

**Décision.** Les 8 règles écartées (`contribution_validee`, `contribution_mise_en_avant`, `factcheck_valide`, `factcheck_faux`, `proposition_media_validee`, `media_a_la_une`, `animation_support_acceptee`, `partage_externe_5reseaux`) sont passées à `actif = FALSE` par la migration `35f`. **Les appels correspondants dans les handlers restent en place.**

**Justification.** `services::engagement::appliquer` commence par `charger_regle(… WHERE actif = TRUE)` et retourne sans rien écrire quand la règle est absente ou inactive. Un appel resté en place sur une règle désactivée ne coûte donc qu'une requête indexée et **ne crédite rien** — c'est exactement le comportement demandé par FR-002 et par le scénario 3 de l'US1. Retirer les appels aurait rendu la réactivation impossible sans livraison technique, ce que FR-002 interdit explicitement.

**Alternatives écartées.**
- *Supprimer les appels et les règles* : contredit la réponse de clarification (« désactivées, réactivables ») et exigerait une livraison pour tout retour arrière.
- *Mettre les montants à 0 en gardant `actif = TRUE`* : produirait des mouvements à 0 point qui pollueraient l'historique et les décomptes de badges `actions_comptees`.

**Exception.** Le bonus `partage_externe_5reseaux` est le seul dont le **code** change, parce que sa sémantique est inversée : le bénéficiaire n'est plus le partageur mais l'auteur (FR-012). `enregistrer_partage_externe` conserve sa trace mais délègue le crédit à `crediter_partage` (voir R5).

---

## R2 — Réactions du fact-check : contradiction de la spécification, résolue

**Problème.** La clarification Q1 retient « les contributions de gouvernance » parmi les familles créditées, **et** exclut « les réactions emoji du fact-check ». Or l'unique système de réaction du schéma `governance` est `factcheck_reaction`, devenu emoji (`coeur`, `pouce`, `rire`, `jaime_pas`) avec la migration `10f` ; c'est `coeur` que le branchement existant traite comme un « j'aime ». Les deux membres de la réponse se contredisent.

**Décision.** La réaction **`coeur`** d'un fact-check est traitée comme le « j'aime » de cette famille et **crédite** son auteur. Les trois autres emojis (`pouce`, `rire`, `jaime_pas`) ne créditent rien. L'exclusion de FR-008b vise donc les emojis **autres que le cœur**, ainsi que les notations sur échelle (`avis_site`, `note_expertise`), qui restent hors barème.

**Justification.** Le cœur est présenté à l'utilisateur comme la marque d'approbation de la fiche, et c'est déjà la sémantique retenue par le code livré (`gouvernance.rs`, décompte `type_reaction = 'coeur'` avec exclusion de l'auto-réaction). Retirer le fact-check du barème aurait retiré une famille que la réponse de clarification demandait explicitement de conserver ; créditer les quatre emojis aurait supposé une règle de conversion arbitraire que la même réponse rejette.

**Conséquence.** La formulation de FR-008/FR-008b est ajustée dans `spec.md` pour lever la contradiction. Point signalé à l'utilisateur dans le rapport de fin de commande.

**Alternatives écartées.**
- *Exclure le fact-check* : perte d'une famille explicitement retenue, et régression par rapport au comportement déjà livré.
- *Poser une 6ᵉ question de clarification* : le quota de `/speckit-clarify` était atteint, et la résolution est déductible du code existant.

---

## R3 — `evaluer_popularite` est remplacée par `crediter_jaime`, à l'unité

**Décision.** Une nouvelle fonction publique du service remplace les paliers :

```
crediter_jaime(pool, type_objet, objet_id, auteur_id, membre_qui_aime_id)
    → clé d'idempotence : "jaime:{type_objet}:{objet_id}:{membre_qui_aime_id}"
```

`evaluer_popularite` est supprimée, la règle `popularite_palier` désactivée et tous les `palier_popularite` passés à `actif = FALSE`.

**Justification.** Trois exigences tombent d'elles-mêmes avec cette clé :
- **FR-010** (retrait puis remise ne crédite qu'une fois) : la clé ne contient pas l'état de la réaction, seulement qui a aimé quoi. Le second `INSERT` frappe la contrainte `UNIQUE` et ne fait rien.
- **FR-011** (pas de reprise au retrait) : le retrait n'appelle simplement pas le service.
- **FR-009** (pas d'auto-like) : garde `auteur_id != membre_qui_aime_id` en tête de fonction.

Le modèle par paliers exigeait au contraire un **recomptage complet** des j'aime à chaque réaction (`COUNT(*)` sur toute la table) pour détecter un franchissement ; le crédit unitaire ne lit plus rien. C'est à la fois plus simple et moins coûteux.

**Alternatives écartées.**
- *Conserver les paliers avec un palier à 1 j'aime* : un palier ne se déclenche qu'une fois par contenu, jamais par membre — il crédite 1 point pour le 1ᵉʳ j'aime et plus rien ensuite. Inutilisable.
- *Recalculer le solde comme `COUNT` de j'aime* : casserait le journal immuable, les plafonds et la réconciliation par catégorie (SC-005).

---

## R4 — Bénéficiaire sur un support média : le propriétaire, avec repli sur le créateur

**Décision.** Une fonction `resoudre_beneficiaire(pool, type_objet, objet_id)` centralise la résolution :

| `type_objet` | Table | Bénéficiaire |
|--------------|-------|--------------|
| `chaine_tv`, `station_radio` | `media_content.{chaine_tv, station_radio}` | `support_detenteur.utilisateur_id` où `role = 'proprietaire' AND actif = TRUE` ; à défaut, `cree_par` du support |
| `programme_tele`, `programme_radio` | `media_content.{programme_tele, programme_radio}` | propriétaire du **support parent**, même règle de repli |
| `codimoi` | `culture.codimoi` | `cree_par` |
| `factcheck` | `governance.factcheck` | `cree_par` |
| `video` | `media_content.video` | auteur de la vidéo |
| `fiche_pays` | `country_profile.fiche_pays` | `cree_par` |
| `biblio_humaine` | `iam.biblio_*` | le membre titulaire de la fiche |
| `personnalite_connue` | `country_profile.personnalite_connue` | `cree_par` |
| `recette_culinaire` | `country_profile.recette_culinaire` | `cree_par` |
| `site_touristique`, `secteur_developpement` | `country_profile.{site_touristique, secteur_developpement}` | **aucun** — voir ci-dessous |
| Aucun bénéficiaire résolu | — | **aucun crédit**, aucune erreur (cadeau : refus explicite) |

**Justification — supports médias.** FR-008a impose le propriétaire unique. Le schéma `09m` garantit déjà structurellement l'unicité par `idx … WHERE role = 'proprietaire' AND actif = TRUE` : aucune agrégation n'est nécessaire, la requête retourne 0 ou 1 ligne. Le repli sur `cree_par` couvre les supports créés par l'administration avant l'existence de `support_detenteur`, sans quoi ces contenus cesseraient silencieusement de rapporter.

**Justification — éléments Opportunité-Afrique.** Le « j'aime » d'un élément n'est **pas** une famille unique : `element_social` est générique par `(type_objet, objet_id)` sur l'enum `country_profile.type_objet_contribution`, et `TYPES_ELEMENT_AUTORISES` en compte **quatre**, chacun dans sa propre table (`table_pour_type`). Une valeur `type_objet = "element"` serait donc **irrésolvable** : rien ne dirait quelle table interroger. Le `type_objet` porte par conséquent le sous-type réel.

Or seuls **deux** de ces quatre sous-types portent un auteur : `personnalite_connue` (migration `11c`) et `recette_culinaire` (migration `11i`) ont `cree_par NOT NULL`. `site_touristique` et `secteur_developpement`, créés par `11_country_profile.sql`, sont des contenus **éditoriaux rattachés à une fiche pays, sans colonne d'auteur**. Ils tombent donc dans le cas limite déjà prévu par la spécification (« contenu sans auteur identifiable ») et ne créditent personne — la réaction et le partage continuant de fonctionner normalement (FR-008c).

**Alternatives écartées.**
- *Garder `cree_par` seul* (comportement actuel de `media_social`) : viole FR-008a dès qu'un support change de mains.
- *Répartir entre co-détenteurs* : écarté par la clarification, et introduirait des arrondis sur l'argent.
- *Traiter `element` comme une famille unique* : impossible — la résolution de l'auteur exige la table, donc le sous-type.
- *Remonter l'auteur d'un site touristique via `contribution_fiche.target_id`* : `target_id` désigne la cible d'une **modification**, pas le créateur d'un objet ; l'archéologie serait fragile, non garantie par une contrainte, et ferait dépendre un crédit d'une table de workflow. Écarté au profit du cas limite explicite.

---

## R5 — Un seul crédit de partage, garanti par la clé, sans aucun comptage

**Décision.** Les 6 handlers de partage interne et le traçage de partage externe appellent tous :

```
crediter_partage(pool, type_objet, objet_id, auteur_id, partageur_id)
    → clé d'idempotence : "partage:{type_objet}:{objet_id}:{partageur_id}"
```

Le **canal n'apparaît pas dans la clé**. Il reste enregistré dans sa table d'origine (`partage_media`, `partage_video`, `partage_element`, `partage_fiche`, `partage_profil`, `partage_contribution`) et dans `engagement.partage_externe` pour la statistique (FR-015).

**Justification.** FR-013 demande l'unicité par `(contenu, partageur)` tous canaux confondus. La clé la réalise **structurellement** : le premier geste crédite, tous les suivants — autre réseau, mur interne, répétition — retombent sur le `ON CONFLICT DO NOTHING`. Aucun `COUNT(DISTINCT …)`, aucune fenêtre de concurrence, aucun code de comptage à maintenir. C'est le même mécanisme que R3, appliqué à une autre dimension.

**Conséquence sur `enregistrer_partage_externe`.** La fonction perd sa logique de seuil (`seuil_declencheur`, `reseaux_distincts`, `bonus_attribue`) et son commentaire « le bénéficiaire est le partageur ». Elle conserve l'`INSERT` de traçage puis appelle `crediter_partage` avec l'auteur résolu. Le DTO de réponse perd `seuil` et `bonus_attribue` — impact frontal documenté dans `contracts/api-engagement-recadre.md`.

**Alternatives écartées.**
- *Clé incluant le canal* : correspondait à l'option A de la clarification, écartée par l'utilisateur.
- *Compter les partages distincts avant de créditer* : réintroduit une lecture-puis-écriture non atomique pour un résultat identique.

---

## R6 — Refonte des niveaux : réutilisation des codes existants, jamais de suppression

**Décision.** La migration `35f` fait évoluer `engagement.niveau` ainsi :

| Code | Avant | Après |
|------|-------|-------|
| `membre` | « Membre », seuil 0 | « Membre Africans », seuil 0 |
| `premium` | « Membre Premium », seuil 200 | « Premium », seuil **500** |
| `gold` | *(inexistant)* | « Gold », seuil **2 000**, ordre 3 |
| `platinum` | « Influenceur Platinum », seuil 1 000, ordre 3 | « Platinum », seuil **10 000**, ordre 4 |

Puis un `UPDATE engagement.compte SET niveau_code = (plus grand seuil ≤ solde)` rebascule tous les comptes **dans la même transaction**.

**Justification.** Les codes sont des clés stables référencées par `compte.niveau_code` et par `badge.parametre_niveau_code`. Les supprimer/recréer romprait ces références. Réutiliser `premium` et `platinum` en déplaçant leur seuil et en insérant `gold` préserve l'intégrité sans migration de données référentielles. L'ordre de mise à jour importe : `platinum` doit passer à l'ordre 4 **avant** l'insertion de `gold` à l'ordre 3, sinon la contrainte d'ordre est momentanément violée.

**Alternatives écartées.**
- *Créer 4 codes neufs et supprimer les anciens* : `ON DELETE` non défini côté `compte.niveau_code` (simple `VARCHAR`), et les badges paramétrés sur un niveau perdraient leur cible.
- *Laisser l'administrateur saisir la grille à la main* : FR-004 exige que les quatre statuts existent à la mise en service ; s'en remettre à une saisie manuelle rendrait SC-002 non vérifiable au déploiement.

---

## R7 — `services::paiement` : deux fonctions concrètes, aucun trait

**Décision.** Un module `src/services/paiement.rs` expose :

```
initier(montant, reference_metier) -> Result<IntentionPaiement>   // simule : renvoie une référence + état en_attente
confirmer(reference, aboutir: bool) -> Result<EtatPaiement>       // simule : abouti | echoue
```

`IntentionPaiement` porte `reference` et `simule: bool`. À l'arrivée de CinetPay, **seul le corps de ces deux fonctions change** (plus l'ajout d'un handler de webhook), et `simule` passe à `false`.

**Justification.** SC-012 exige que le remplacement ne touche ni le catalogue, ni le journal, ni la répartition, ni l'attribution des points. Isoler l'appel dans un module suffit à le garantir. Le Principe V interdit d'introduire un trait `PrestatairePaiement` avec une seule implémentation : ce serait une abstraction sans second usage, et le jour où CinetPay arrive, un trait ne ferait pas gagner une ligne.

Le drapeau `simule` est stocké **sur la transaction**, pas déduit de la configuration : c'est lui qui rend la purge de fin de phase (FR-020b) possible et exacte, même si des transactions réelles et simulées cohabitent le jour du basculement.

**Alternatives écartées.**
- *Trait `PrestatairePaiement` + implémentation `Simulateur`* : sur-ingénierie explicitement proscrite (Principe V).
- *Variable d'environnement `PAIEMENT_SIMULE`* : la purge doit distinguer les transactions **au cas par cas**, pas selon l'état de la configuration au moment de la lecture.

---

## R8 — Répartition monétaire : entiers, part plateforme calculée par différence

**Décision.** Les montants sont des `INTEGER` en **unité entière de la devise** (FCFA, sans subdivision). La répartition s'écrit :

```
part_beneficiaire = (montant * (100 - taux_commission)) / 100     -- division entière
part_plateforme   = montant - part_beneficiaire                   -- par différence
```

garanties par un `CHECK (part_beneficiaire + part_plateforme = montant)`.

**Justification.** SC-009 exige une somme exacte. Calculer les deux parts indépendamment produirait une perte d'arrondi (10 % de 1 001 → 100 et 900, somme 1 000 ≠ 1 001). Le calcul par différence est exact par construction, et le `CHECK` rend l'invariant **impossible à violer en SQL**, y compris par une écriture manuelle en base. L'arrondi favorise la plateforme d'au plus 1 unité, ce qui est le sens conventionnel des frais.

Le mode `points` s'exprime dans le même schéma sans cas particulier : `part_beneficiaire = 0`, `part_plateforme = montant`.

**Alternatives écartées.**
- *`NUMERIC(12,2)`* : la devise de référence n'a pas de centimes ; introduirait des flottants côté TS et un risque de dérive de sérialisation entre Rust et TypeScript.
- *Stocker uniquement le taux et recalculer à la lecture* : une modification ultérieure du taux réécrirait rétroactivement l'histoire comptable, ce que FR-024 interdit.

---

## R9 — Le crédit du cadeau réutilise le moteur, avec `montant_override`

**Décision.** Une fonction `crediter_cadeau(pool, beneficiaire_id, transaction_id, points)` appelle le cœur existant `appliquer(…)` avec :
- `type_action = "cadeau_recu"` (règle créée par `35f`, catégorie « Cadeaux ») ;
- `montant_override = Some(points)` — les points viennent du **catalogue figé sur la transaction**, pas de la règle ;
- clé d'idempotence `"cadeau:{transaction_id}"`.

La règle `cadeau_recu` porte `points = 0` : elle sert de porte (activable/désactivable, FR-020 scénario 8) et de porteuse de catégorie et de plafonds, pas de montant.

**Justification.** C'est exactement le mécanisme déjà utilisé par `evaluer_popularite` et `ajuster` : `appliquer` accepte un `montant_override` précisément pour les cas où le montant n'est pas dans la règle. Aucun nouveau chemin d'écriture n'est créé, donc les plafonds, le plancher à 0, le recalcul de niveau, la notification et l'évaluation des badges s'appliquent gratuitement au cadeau. La clé sur `transaction_id` rend le rejeu de confirmation (FR-022) inoffensif.

**Point d'attention.** `appliquer` étant privée, `crediter_cadeau` doit vivre dans `services::engagement.rs`, pas dans le handler des cadeaux. La séparation transaction/points reste nette : le handler commet la transaction, puis appelle le service.

---

## R10 — Ordre d'écriture du cadeau : la transaction d'abord, les points après le COMMIT

**Décision.** Séquence d'un envoi abouti :

1. `UPDATE transaction_cadeau SET etat = 'abouti', finalise_at = NOW() WHERE id = $1 AND etat = 'en_attente'` — si `rows_affected = 0`, on s'arrête (rejeu ou état incompatible).
2. `INSERT`/`UPDATE` de la cagnotte du bénéficiaire, **dans la même transaction** (uniquement en mode `soutien_financier`).
3. `COMMIT`.
4. **Après le COMMIT** : `crediter_cadeau(…)` puis la notification « cadeau reçu ».

**Justification.** L'`UPDATE` conditionné sur `etat = 'en_attente'` est le verrou d'idempotence de la confirmation : deux requêtes concurrentes ne peuvent pas toutes deux passer l'étape 1. La comptabilité (transaction + cagnotte) doit être atomique — une cagnotte créditée sans transaction aboutie serait de l'argent inventé. Les points, eux, sont accessoires et non bloquants (FR-034) : les mettre dans la transaction comptable ferait échouer un envoi payé à cause d'une erreur du moteur de points. C'est le motif déjà appliqué par `appliquer`, qui notifie et évalue les badges après son propre `COMMIT`.

**Alternatives écartées.**
- *Tout dans une transaction* : viole FR-034 et SC-007.
- *Points d'abord* : un échec de la comptabilité laisserait des points sans contrepartie, impossible à rattraper proprement.

---

## R11 — Purge de fin de phase de test : une route d'administration, jamais un script

**Décision.** `POST /api/admin/engagement/purger-phase-test` exécute, en une transaction :
1. sélection des `transaction_cadeau` où `simule = TRUE AND etat = 'abouti'` ;
2. suppression des `mouvement_points` dont la `cle_idempotence` vaut `cadeau:{id}` pour ces transactions ;
3. recalcul intégral de `compte.solde_points` et `solde_points_mensuel` à partir du journal restant, puis de `niveau_code` ;
4. remise à zéro des cagnottes issues de ces transactions ;
5. marquage des transactions purgées (`etat = 'purge'`), jamais leur suppression ;
6. `audit::log_action` avec le décompte des lignes touchées.

**Justification.** FR-020b et SC-013 exigent que la purge retire 100 % des points issus de cadeaux simulés **sans toucher** aux points de j'aime et de partage. Le recalcul du solde depuis le journal est la seule méthode qui garantisse la cohérence : soustraire les points supprimés du solde courant dériverait dès qu'un plafond a écrêté un mouvement. Passer par une route auditée plutôt qu'un script SQL manuel respecte le Principe VII et rend l'opération reproductible en recette.

La suppression de lignes de `mouvement_points` est la **seule** entorse à l'immuabilité du journal ; elle est bornée à une opération unique, tracée, et justifiée par le fait que ces points n'auraient jamais dû exister hors phase de test.

**Alternatives écartées.**
- *Migration SQL exécutée à la main via SSH* : non auditée, non reproductible, contraire au Principe VII.
- *Neutraliser par un mouvement de malus* : laisserait des statuts atteints puis perdus dans l'historique et fausserait la ventilation par catégorie plutôt que de la corriger.

---

## Synthèse des dépendances externes

| Dépendance | Statut | Note |
|------------|--------|------|
| CinetPay | **Différée** | Aucun SDK, aucune clé, aucun appel réseau dans cette itération. Point de bascule : `services/paiement.rs` (R7). |
| Notifications | Réutilisées | `models::notification::engagement` gagne `CADEAU_RECU` ; mécanisme inchangé. |
| Audit | Réutilisé | `audit::log_action` sur les mutations d'administration et la purge. |
| Aucune nouvelle dépendance Cargo ni pnpm | ✅ | Confirmé : la simulation, la répartition et l'affichage n'exigent aucune bibliothèque. |

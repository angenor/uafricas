# Système d'engagement AFRICANS : Résumé opérationnel pour les équipes

> Développement de la section **6. Résumé opérationnel** du document source, mis en regard de ce qui existe déjà sur la plateforme.
> **Point de départ essentiel : le socle d'engagement est déjà construit.** Ne pas repartir de zéro, étendre l'existant.

---

## 0. Ce qui existe déjà (à réutiliser, pas à recréer)

| Brique | Emplacement | État |
|--------|-------------|------|
| Schéma points | `doc/bd/schemas/35_engagement.sql` + `35b_..._mise_en_avant.sql` | ✅ Livré |
| Service Rust | `src/services/engagement.rs` (`attribuer`, `retirer`, `evaluer_popularite`, `ajuster`) | ✅ Câblé |
| Tables clés | `engagement.compte` (solde, solde mensuel, réputation, niveau), `mouvement_points` (journal append-only + `cle_idempotence`), `regle_points` (barème paramétrable + plafonds), `palier_popularite`, `niveau`, `mise_en_avant` | ✅ |
| Barème seedé | `contribution_validee +2`, `factcheck_valide +3` (rép +1), `factcheck_faux −2` (rép −3), `popularite_palier`, `ajustement_admin` | ✅ |
| Points d'appel | codimoi, biblio humaine, vidafrica, gouvernance (+ handlers admin correspondants) | ✅ |
| Permission | `engagement.gerer` (super_admin) | ✅ |
| Spec de référence | `specs/001-engagement-gamification/` | ✅ |

**Conséquence :** la table `points_rules` demandée au point 6.2 = `engagement.regle_points`, déjà en place. `user_points`/`points_log` = `engagement.compte`/`mouvement_points`.

---

## 1. Actions donnant des points : figer la liste (point 6.1)

Statut par action du barème du document :

- **Déjà branché :** contribution validée (Codimoi, VidAfrica, Ideaforces, BadGoodHabit, fiches pays), fact-check correct / abusif, popularité par paliers de likes.
- **À brancher (règles à seeder dans `regle_points`) :**
  - Partages externes : `+10 pts` / 5 partages sur réseaux distincts, plafond 3×/jour. → **nécessite un nouveau log** (voir §5-a).
  - Contribution « mise en avant » : `+5 pts` (réutiliser `engagement.mise_en_avant`).
  - **Télé / radio (refonte lot 1→3)** : proposition de média validée (`admin/media_proposition::valider`) → `+5` ; média mis « à la une » → `+8` ; demande d'animation/co-détention acceptée (`support_detenteur` créé via `appliquer_acceptation_engagement`) → `+15`.
  - **Popularité télé/radio** : réactions / commentaires / partages des 4 types de médias (`media_reaction`, `media_commentaire`, `partage_media`) alimentent déjà les compteurs `evaluer_popularite()` : il suffit d'ajouter des `palier_popularite` pour ces objets.
- **Décision produit attendue :** valeurs et plafonds définitifs (le doc les donne comme indicatifs). Aucun hard-coding : tout passe par `regle_points`.

### Barème proposé (à seeder dans `engagement.regle_points`)

Valeurs concrètes recommandées : cohérentes avec le doc source et le barème déjà seedé. Colonnes : `type_action` (clé), points, réputation, plafond.

| `type_action` | Points | Réput. | Plafond | État |
|---------------|:------:|:------:|---------|------|
| `partage_externe_5reseaux` | +10 | : | **3×/jour** (30 pts/j) | à créer (§5-a) |
| `popularite_palier` : 100 likes | +10 |, | 1×/palier/objet | ✅ existe |
| `popularite_palier` : 500 likes | +30 |, | 1×/palier/objet | ✅ existe |
| `popularite_palier` : 1 000 likes | +50 |, | 1×/palier/objet | ✅ existe |
| `contribution_validee` | +2 | : | : | ✅ branché |
| `contribution_mise_en_avant` | +5 | +1 |, | à brancher |
| `factcheck_valide` | +3 | +1 | : | ✅ branché |
| `factcheck_faux` | −2 | −3 | : | ✅ branché |
| `proposition_media_validee` | +5 | +1 |, | **à brancher (télé/radio)** |
| `media_a_la_une` | +8 | +1 | 1×/média | **à brancher (télé/radio)** |
| `animation_support_acceptee` | +15 | +2 |, | **à brancher (télé/radio)** |
| `ajustement_admin` | libre | libre | permission `engagement.gerer` | ✅ existe |

**Seuils de niveaux (`engagement.niveau`)**, proposition de départ, ajustable :

| Niveau | Seuil (pts cumulés) | Avantage principal |
|--------|:-------------------:|--------------------|
| Membre | 0 – 199 | Standard |
| Premium | 200 | Badge + légère priorisation des publications |
| Influenceur Gold | 1 000 | Mise en avant régulière (« à la une ») |
| Influenceur Diamant | 3 000 | + invitations privilégiées (Africalive/Télé/Radio) |

**Cadeaux entre utilisateurs** : coût (Modèle A = transfert intégral / Modèle B = coût réduit + bonus symbolique) :

| Cadeau | Valeur | Modèle A (émetteur −) | Modèle B (émetteur −) |
|--------|:------:|:---------------------:|:---------------------:|
| Gô | 20 | −20 | −5 |
| Boro | 50 | −50 | −10 |
| Digbate | 100 | −100 | −20 |
| Lass | 300 | −300 | −40 |
| Viemogo | 500 | −500 | −60 |

> Le receveur gagne toujours la valeur nominale. Garde-fous : quota/jour, pas d'auto-cadeau, anti-abus via `cle_idempotence`.

---

## 2. Écrans à concevoir (point 6.3)

| Écran | Réutilise | À créer |
|-------|-----------|---------|
| « Mes points / statut / badges » (profil) | `engagement.compte` + `niveau` (badge_couleur/icône) | Page front `/mon-compte/engagement` + endpoint lecture |
| « Classements » (global / par app / par pays) | `compte.solde_points` (+ mensuel) | Endpoint agrégé + page publique |
| Admin cadeaux partenaires | pattern admin country_profile / afripulse | **Module neuf** (voir §4) |
| Admin engagement (barème, ajustements) | permission `engagement.gerer` déjà là | Vérifier/compléter l'UI admin |

## 3. Statuts & visibilité

- 3 niveaux seedés (`membre` / `premium` / `platinum`) via `engagement.niveau`. **Décider les seuils exacts** (200 / 1000 = exemples) et l'**impact algorithmique** sur les fils (poids ranking, slots « à la une », s'appuyer sur `engagement.mise_en_avant`).
- **Badges nominatifs / succès débloquables** au-delà des 3 niveaux : **n'existent pas** → table à créer si retenu.

## 4. Cadeaux entre utilisateurs (point 6.4)

- **Décision requise :** Modèle A (transfert de points) vs B (cadeau symbolique + bonus) vs mixte.
- Le journal `mouvement_points` + idempotence supporte déjà le transfert (deux mouvements liés). À ajouter : catalogue (Gô 20 / Boro 50 / Digbate 100 / Lass 300 / Viemogo 500), garde-fous anti-abus (quota/jour, pas d'auto-cadeau).

## 5. Vrais manques à créer (rien de réutilisable en l'état)

- **(a) Log de partage externe par réseau social.** Les tables `partage_*` existantes = reposts internes (pas de colonne réseau, pas de tracking Facebook/X/WhatsApp/LinkedIn). → nouvelle table `engagement.partage_externe (user_id, contenu, reseau, date)` + comptage « 5 réseaux distincts » côté service.
- **(b) Table de badges/succès** nominatifs (si §3 retenu).
- **(c) Notifications d'engagement** : pas de service unifié (tables par domaine : `social.notification`, etc.). Point-gagné / niveau-atteint / cadeau-reçu à raccrocher au domaine `social` ou à un futur service central (`specs/001-notifications-suggestions/`).

## 6. Publicité & monétisation (points 6.5, hors socle engagement)

- **Non commencé.** Périmètre séparé : compte annonceur, campagnes (budget/période/ciblage/visuels), tableau de bord (impressions, clics, CTR), formats (bannières, natif, encarts jeux).
- **Règle produit à figer :** limites UX (fréquence, emplacements) pour ne pas dégrader l'expérience ludique.
- **Financement volontaire :** reconnaissance **non monétaire** uniquement (badge « Supporter/Ambassadeur », page « Merci »). **Ne jamais convertir argent → points d'engagement.**

---

## Synthèse des décisions attendues du produit

1. Valeurs/plafonds définitifs du barème (à seeder dans `regle_points`).
2. Seuils exacts des niveaux + impact algorithmique sur la visibilité.
3. Modèle de cadeaux entre utilisateurs (A / B / mixte) + anti-abus.
4. Créer ou non des badges/succès nominatifs.
5. Règles publicité (formats, emplacements, limites UX), chantier distinct.

## Priorisation d'implémentation suggérée

1. **Compléter le socle existant** : brancher partages externes (nouvelle table §5-a) + mise-en-avant `+5`.
2. **Exposer côté membre** : page « Mes points/statut/badges » + classements.
3. **Cadeaux entre utilisateurs** (après décision modèle).
4. **Module admin cadeaux partenaires**.
5. **Publicité / monétisation** (chantier autonome).

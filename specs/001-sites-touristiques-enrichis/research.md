# Phase 0 — Research & Décisions techniques

Feature : Enrichissement des sites touristiques · Branche `001-sites-touristiques-enrichis`

Aucun marqueur `NEEDS CLARIFICATION` ne subsiste (visibilité résolue : contacts/légal = publics).
Les décisions ci-dessous tranchent les choix d'implémentation à partir du code afripulse existant.

---

## D1 — Représentation des sous-types : un seul enum PostgreSQL

**Décision** : créer **un seul** enum `country_profile.sous_type_site` regroupant les 20 valeurs
(12 emblématiques + 8 privées), et valider la cohérence famille↔sous-type côté code (Rust + TS).

Valeurs (snake_case français, Principe I) :
- Emblématiques : `plage`, `monument`, `relief_naturel`, `parc_naturel`, `mosquee`, `eglise`,
  `pont`, `route`, `service_public`, `immeuble_edifice`, `mer_riviere`, `site_naturel`.
- Privés : `hotel`, `plage_privee`, `espace_jeux`, `agriculture_touristique`,
  `residence_touristique`, `restaurant`, `discotheque`, `bar_maquis`.

**Rationale** : un enum unique reste simple à faire évoluer (Principe V), évite deux types SQL à
joindre, et la table de correspondance famille↔sous-type vit en un seul endroit (fonction de
validation). La colonne `categorie` (emblematique/prive) existe déjà et reste la source de la
famille ; `sous_type` la précise.

**Alternatives rejetées** :
- *Deux enums distincts* (`sous_type_emblematique`, `sous_type_prive`) : double les types SQL et la
  logique de mapping côté Rust/TS sans gain ; FR-003 (cohérence) reste à valider de toute façon.
- *Colonne texte libre* : perd la contrainte d'intégrité et le filtrage fiable (FR-004).

---

## D2 — Avis par site : écriture directe (pas le workflow de contribution)

**Décision** : nouvelle table `country_profile.avis_site` alimentée par un **endpoint direct
authentifié** (publication immédiate), distincte du circuit de contribution. Un avis actif au plus
par couple (utilisateur, site) via index unique partiel. Modération admin = masquage (soft).

**Rationale** : US5 scénario 1 exige que l'avis soit « enregistré et compté dans la note moyenne »
dès la soumission → publication immédiate. Le workflow de contribution (validation admin préalable)
contredirait ce comportement. Le pattern d'unicité reprend `recommandation_visiteur`
(`uniq_recommandation_active`), éprouvé. FR-015d (masquage admin) couvre l'abus a posteriori.

**Alternatives rejetées** :
- *Router les avis via `contribution_fiche`* (comme `recommandation_visiteur`) : impose une
  validation admin avant affichage, incompatible avec la publication immédiate attendue, et alourdit
  la file de modération.
- *Réutiliser `recommandation_visiteur`* : cette table est rattachée à la fiche **pays**, pas au
  **site** ; granularité incompatible (hypothèse de la spec).

---

## D3 — Champs enrichis du site : extension de `site_touristique` via contribution JSONB

**Décision** : ajouter les colonnes à `country_profile.site_touristique` (sous_type, gestionnaire,
ville, village, info_pertinente, contact_telephone, contact_courriel, contact_adresse,
constitution_statut_juridique, constitution_numero, constitution_document_url, verifie, verifie_par,
verifie_at). Les ajouts/éditions passent par le payload `nouvelle_valeur_jsonb` existant et sont
appliqués par `appliquer_contribution_afripulse` (branches `site_touristique` étendues).

**Rationale** : Principe III (SQL SoT) + réutilisation du workflow de contribution déjà câblé
(`soumettre_contribution_afripulse` → `moderer_contribution` → `appliquer_contribution_afripulse`).
`latitude`/`longitude` existent déjà (GPS). La famille (`categorie`) existe déjà.

**Validation** (dans `soumettre_contribution_afripulse`, à l'ajout/édition d'un site) :
- Champs requis (FR-005) : `nom`, `gestionnaire`, `ville`, `info_pertinente`, `latitude`,
  `longitude` (territoire = `fiche_pays_id` déjà porté par la route).
- Si `categorie = prive` : au moins un parmi `contact_telephone`, `contact_courriel`,
  `contact_adresse` (FR-006).
- `sous_type` cohérent avec `categorie` (FR-003) via fonction `sous_type_appartient_a(categorie)`.

**Alternatives rejetées** :
- *Table séparée pour les contacts / la constitution légale* : sur-normalisation non justifiée
  (relation 1–1, Principe V).

---

## D4 — Badge « Vérifié » : toggle admin direct + audit

**Décision** : endpoint admin `PATCH /api/admin/profils-pays/{id}/sites-touristiques/{site_id}/verification`
(body `{ verifie: bool }`), protégé par `verifier_permission!(admin, "profil_pays", "modifier")`,
qui met à jour `verifie`, `verifie_par`, `verifie_at` et journalise via `audit::log_action`
(Principe VII). La lecture publique renvoie `verifie` pour l'affichage du badge.

**Rationale** : le badge est un attribut admin pur (hypothèse spec) ; pas besoin de passer par une
contribution. Cohérent avec le CRUD admin existant des sites (`modifier_site_touristique`).

**Alternatives rejetées** :
- *Champ modifiable via contribution communautaire* : violerait FR-012 (réservé admin).

---

## D5 — Rate-limit et upload

**Décision** : réutiliser `rate_limit_afripulse::verifier_quotas` pour les contributions de sites
(déjà appliqué au `type_objet = site_touristique`). Les avis (D2) utilisent un garde simple
(1 avis actif/site déjà garanti par l'index unique ; ajout d'un contrôle de fréquence léger si
nécessaire — non bloquant). Le document de constitution légale réutilise
`uploader_image_contribution` / le stockage `./uploads/opportunite-afrique/` (Principe — upload local).

**Rationale** : éviter toute nouvelle infrastructure (Principe V) ; cohérence avec l'existant.

---

## D6 — Frontend : Tailwind v4 pur côté public

**Décision** : la section publique (`SitesTouristiquesSection.vue`), le nouveau composant
`SiteAvisListe.vue` et les ajouts au `ContributionModal.vue` utilisent exclusivement les utilitaires
Tailwind v4 (Principe VI). Le back-office (badge vérification + modération d'avis) peut utiliser
daisyUI. Icônes via FontAwesome (plugin global).

**Rationale** : conformité stricte au Principe VI ; le code existant de la section est déjà en
Tailwind pur.

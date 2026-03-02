-- ============================================================================
-- PLATEFORME AFRICANS-WORLD
-- Schéma de Base de Données PostgreSQL
-- Architecture : Monolith-First, Microservice-Ready
-- ============================================================================
--
-- PRINCIPE D'ARCHITECTURE
-- ───────────────────────
-- Chaque SCHEMA PostgreSQL représente un bounded context (futur microservice).
-- En phase monolithe, les FK inter-schemas garantissent l'intégrité.
-- Lors du découpage, ces FK (regroupées dans 13_contraintes_inter_schemas.sql)
-- seront remplacées par des appels API ; chaque schema deviendra une base
-- indépendante.
--
-- SCHEMAS (BOUNDED CONTEXTS)
-- ──────────────────────────
--   shared          → Référentiels partagés (pays, domaines, catégories, tags, médias)
--   iam             → Identité & Accès (utilisateurs, rôles, permissions, organisations)
--   marketplace     → Marché Africain (annonces)
--   exchange        → Programmes d'échange
--   innovation      → Innovations, projets, africantives
--   culture         → Centres culturels, Codi-Moi
--   afrolang        → Visioconférence WebRTC (apprentissage langues africaines)
--   media_content   → Radio/Télé, livres, événements, MOOC
--   governance      → Gouvernance citoyenne (FactCheck, BadHabits, IdeaForces)
--   country_profile → Fiches pays
--   retrouve_amis   → Retrouver des amis perdus de vue
--
-- CONVENTIONS
-- ───────────
--   • PK        : UUID v4 générée côté base (uuid_generate_v4)
--   • Temps     : TIMESTAMPTZ partout (fuseaux horaires)
--   • Soft del. : colonne deleted_at nullable
--   • Audit     : created_at / updated_at sur toute table mutable
--   • Nommage   : snake_case, vocabulaire français aligné sur le domaine métier
--   • [xref]    : commentaire signalant une référence inter-schemas
--                  (→ deviendra un appel API lors du split microservice)
--
-- STRUCTURE DES FICHIERS
-- ──────────────────────
--   schemas/
--   ├── 00_extensions.sql                  Extensions PostgreSQL
--   ├── 01_schemas.sql                     Création des schemas
--   ├── 02_fonctions.sql                   Fonctions utilitaires
--   ├── 03_shared.sql                      Référentiels partagés
--   ├── 04_iam.sql                         Identité & Accès
--   ├── 05_marketplace.sql                 Marché Africain
--   ├── 06_exchange.sql                    Programmes d'échange, Écoles partenaires, Facultés INUDA
--   ├── 07_innovation.sql                  Innovations & Projets
--   ├── 08_culture.sql                     Culture, Codi-Moi
--   ├── 08b_afrolang.sql                   Afrolang — Visioconférence WebRTC
--   ├── 09_media_content.sql               Radio/Télé, Livres, Events, MOOC
--   ├── 10_governance.sql                  Gouvernance Citoyenne
--   ├── 11_country_profile.sql             Fiches pays
--   ├── 12_audit.sql                       Journal d'audit
--   ├── 13_contraintes_inter_schemas.sql   FK cross-boundary (à supprimer lors du split)
--   ├── 14_triggers.sql                    Triggers updated_at
--   └── 15_seed.sql                        Données de référence
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- PHASE 1 : Infrastructure
-- ════════════════════════════════════════════════════════════════════════════

\ir schemas/00_extensions.sql
\ir schemas/01_schemas.sql
\ir schemas/02_fonctions.sql


-- ════════════════════════════════════════════════════════════════════════════
-- PHASE 2 : Tables par bounded context (ordre respectant les dépendances)
-- ════════════════════════════════════════════════════════════════════════════

\ir schemas/03_shared.sql
\ir schemas/04_iam.sql
\ir schemas/04b_iam_expertise.sql
\ir schemas/04c_iam_verification_email.sql
\ir schemas/05_marketplace.sql
\ir schemas/06_exchange.sql
\ir schemas/07_innovation.sql
\ir schemas/08_culture.sql
\ir schemas/08b_afrolang.sql
\ir schemas/09_media_content.sql
\ir schemas/10_governance.sql
\ir schemas/11_country_profile.sql
\ir schemas/11b_country_profile_contributions.sql
\ir schemas/16_retrouve_amis.sql


-- ════════════════════════════════════════════════════════════════════════════
-- PHASE 3 : Transversal
-- ════════════════════════════════════════════════════════════════════════════

\ir schemas/12_audit.sql


-- ════════════════════════════════════════════════════════════════════════════
-- PHASE 4 : Contraintes inter-schemas
-- ────────────────────────────────────
-- Ces FK seront SUPPRIMÉES lors du découpage en microservices.
-- Chaque service conservera l'UUID et résoudra la référence via API.
-- ════════════════════════════════════════════════════════════════════════════

\ir schemas/13_contraintes_inter_schemas.sql


-- ════════════════════════════════════════════════════════════════════════════
-- PHASE 5 : Triggers & Données initiales
-- ════════════════════════════════════════════════════════════════════════════

\ir schemas/14_triggers.sql
\ir schemas/15_seed.sql
\ir schemas/16_seed_centres_culturels.sql
\ir schemas/17_seed_evenements.sql
\ir schemas/18_seed_sabbatiques.sql
\ir schemas/19_seed_moocs.sql
\ir schemas/20_seed_fiches_pays.sql
\ir schemas/21_seed_facultes.sql
\ir schemas/22_seed_experts.sql


-- ════════════════════════════════════════════════════════════════════════════
-- FIN DU SCHÉMA
-- ════════════════════════════════════════════════════════════════════════════
--
-- NOTES POUR LE FUTUR
-- ───────────────────
-- 1. PostGIS : remplacer les colonnes longitude/latitude par des colonnes
--    GEOMETRY(Point, 4326) pour les requêtes géospatiales avancées.
--
-- 2. Messagerie interne : ajouter un schema « messaging » quand le module
--    de messagerie plateforme sera spécifié (conversations, messages, pièces
--    jointes, statut lu/non-lu).
--
-- 3. Notifications : ajouter un schema « notification » pour les alertes
--    temps réel (WebSocket) et les emails différés.
--
-- 4. Full-text search : les colonnes search_vector doivent être alimentées
--    via des triggers applicatifs utilisant to_tsvector('french', ...).
--
-- 5. Partitionnement : si le volume de données le justifie, partitionner
--    shared.audit_log par mois (RANGE sur created_at).
--
-- 6. Découpage microservices : supprimer le fichier
--    13_contraintes_inter_schemas.sql, chaque service gardera les UUID
--    et résoudra via API / événements.
-- ════════════════════════════════════════════════════════════════════════════

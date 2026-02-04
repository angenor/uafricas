-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Création des schemas (bounded contexts)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Chaque schema représente un bounded context (futur microservice).
--   shared          → Référentiels partagés (pays, domaines, catégories, tags, médias)
--   iam             → Identité & Accès (utilisateurs, rôles, permissions, organisations)
--   marketplace     → Marché Africain (annonces)
--   exchange        → Programmes d'échange
--   innovation      → Innovations, projets, africantives
--   culture         → Centres culturels, Afrolang, Codi-Moi
--   media_content   → Radio/Télé, livres, événements, MOOC
--   governance      → Gouvernance citoyenne (FactCheck, BadHabits, IdeaForces)
--   country_profile → Fiches pays
-- ════════════════════════════════════════════════════════════════════════════

CREATE SCHEMA IF NOT EXISTS shared;
CREATE SCHEMA IF NOT EXISTS iam;
CREATE SCHEMA IF NOT EXISTS marketplace;
CREATE SCHEMA IF NOT EXISTS exchange;
CREATE SCHEMA IF NOT EXISTS innovation;
CREATE SCHEMA IF NOT EXISTS culture;
CREATE SCHEMA IF NOT EXISTS media_content;
CREATE SCHEMA IF NOT EXISTS governance;
CREATE SCHEMA IF NOT EXISTS country_profile;

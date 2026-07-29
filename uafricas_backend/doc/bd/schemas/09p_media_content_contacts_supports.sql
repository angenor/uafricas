-- ============================================================================
-- 09p — media_content : contacts publics des supports (chaînes et stations)
-- ----------------------------------------------------------------------------
-- Une chaîne ou une station est portée par une équipe réelle, qu'un visiteur
-- doit pouvoir joindre hors plateforme : annonceurs, partenaires, auditeurs.
-- La messagerie interne (`ContacterSupportModal`, US6) ne remplace pas ces
-- coordonnées — elle exige un compte et ne joint que les co-détenteurs inscrits.
--
-- Les colonnes reprennent le nommage de `09c_media_content_evenement_contact`
-- (`contact_email`, `contact_telephone`, `contact_site_web`), auquel s'ajoutent
-- WhatsApp — premier canal de contact d'un média africain — et l'adresse du
-- siège.
--
-- Toutes facultatives : l'existant n'en porte aucune, et rien n'oblige une
-- chaîne à publier ses coordonnées. Aucun CHECK de format : un numéro
-- international, une extension, un compte WhatsApp business n'ont pas de forme
-- canonique, et un CHECK trop strict rejetterait des saisies légitimes. La
-- normalisation utile — préfixer un site web dépourvu de schéma — est faite à
-- l'écriture côté Rust (`services::contacts_media`).
--
-- Migration idempotente : ADD COLUMN IF NOT EXISTS.
-- ============================================================================

ALTER TABLE media_content.chaine_tv
    ADD COLUMN IF NOT EXISTS contact_email     VARCHAR(320),
    ADD COLUMN IF NOT EXISTS contact_telephone VARCHAR(50),
    ADD COLUMN IF NOT EXISTS contact_whatsapp  VARCHAR(50),
    ADD COLUMN IF NOT EXISTS contact_site_web  VARCHAR(500),
    ADD COLUMN IF NOT EXISTS contact_adresse   VARCHAR(300);

ALTER TABLE media_content.station_radio
    ADD COLUMN IF NOT EXISTS contact_email     VARCHAR(320),
    ADD COLUMN IF NOT EXISTS contact_telephone VARCHAR(50),
    ADD COLUMN IF NOT EXISTS contact_whatsapp  VARCHAR(50),
    ADD COLUMN IF NOT EXISTS contact_site_web  VARCHAR(500),
    ADD COLUMN IF NOT EXISTS contact_adresse   VARCHAR(300);

COMMENT ON COLUMN media_content.chaine_tv.contact_email IS
    'Coordonnées publiques de l''équipe, affichées sur /medias/chaines/{slug} une fois la chaîne publiée.';
COMMENT ON COLUMN media_content.station_radio.contact_email IS
    'Coordonnées publiques de l''équipe, affichées sur /medias/stations/{slug} une fois la station publiée.';

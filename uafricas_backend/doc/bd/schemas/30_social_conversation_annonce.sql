-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Marché Africain : contexte annonce d'une conversation (D2)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Une conversation peut naître d'un contact d'annonce (feature
-- 001-marche-achat-vente-troc-don). On rattache la conversation à l'annonce
-- d'origine pour afficher « À propos de l'annonce : <titre> » côté messagerie.
--
-- La colonne est NULLABLE, purement informationnelle, et NE participe PAS à
-- l'unicité de la paire (uq_conversation_paire). Une seule conversation par
-- paire est conservée : si une même paire échange ensuite sur une autre
-- annonce, annonce_id garde l'origine (limitation acceptée, D2).
--
-- En base déjà initialisée, exécuter ce fichier en migration manuelle (SSH+psql).
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE social.conversation
    ADD COLUMN IF NOT EXISTS annonce_id UUID REFERENCES marketplace.annonce(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_conversation_annonce
    ON social.conversation(annonce_id)
    WHERE annonce_id IS NOT NULL;

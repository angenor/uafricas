-- Afrolang : groupe ethnique « Autre » en texte libre.
-- Une proposition / salle peut cibler SOIT un groupe référencé (groupe_ethnique_id),
-- SOIT un nom libre (groupe_ethnique_libre) lorsque le groupe n'existe pas encore.
ALTER TABLE afrolang.proposition_salle ALTER COLUMN groupe_ethnique_id DROP NOT NULL;
ALTER TABLE afrolang.proposition_salle ADD COLUMN IF NOT EXISTS groupe_ethnique_libre VARCHAR(250);
ALTER TABLE afrolang.salle ALTER COLUMN groupe_ethnique_id DROP NOT NULL;
ALTER TABLE afrolang.salle ADD COLUMN IF NOT EXISTS groupe_ethnique_libre VARCHAR(250);

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'chk_proposition_groupe_present') THEN
    ALTER TABLE afrolang.proposition_salle
      ADD CONSTRAINT chk_proposition_groupe_present
      CHECK (groupe_ethnique_id IS NOT NULL OR groupe_ethnique_libre IS NOT NULL);
  END IF;
  IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'chk_salle_groupe_present') THEN
    ALTER TABLE afrolang.salle
      ADD CONSTRAINT chk_salle_groupe_present
      CHECK (groupe_ethnique_id IS NOT NULL OR groupe_ethnique_libre IS NOT NULL);
  END IF;
END $$;

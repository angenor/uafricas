-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Contraintes inter-schemas (FK cross-boundary)
-- ════════════════════════════════════════════════════════════════════════════
-- Ces contraintes seront SUPPRIMÉES lors du découpage en microservices.
-- Chaque service conservera l'UUID et résoudra la référence via API.
-- ════════════════════════════════════════════════════════════════════════════


-- ── shared.media ────────────────────────────────────────────────────────
ALTER TABLE shared.media
    ADD CONSTRAINT fk_media_uploaded_by
    FOREIGN KEY (uploaded_by) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

-- ── iam → shared ────────────────────────────────────────────────────────
ALTER TABLE iam.utilisateur
    ADD CONSTRAINT fk_utilisateur_pays_origine
    FOREIGN KEY (pays_origine_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE iam.utilisateur
    ADD CONSTRAINT fk_utilisateur_pays_residence
    FOREIGN KEY (pays_residence_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE iam.organisation
    ADD CONSTRAINT fk_organisation_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

-- ── marketplace → shared & iam ──────────────────────────────────────────
ALTER TABLE marketplace.annonce
    ADD CONSTRAINT fk_annonce_categorie
    FOREIGN KEY (categorie_id) REFERENCES shared.categorie(id) ON DELETE SET NULL;

ALTER TABLE marketplace.annonce
    ADD CONSTRAINT fk_annonce_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE marketplace.annonce_pays
    ADD CONSTRAINT fk_annonce_pays_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE CASCADE;

ALTER TABLE marketplace.annonce_favori
    ADD CONSTRAINT fk_annonce_favori_utilisateur
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

-- ── exchange → shared & iam ─────────────────────────────────────────────
ALTER TABLE exchange.programme
    ADD CONSTRAINT fk_programme_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE RESTRICT;

ALTER TABLE exchange.programme
    ADD CONSTRAINT fk_programme_domaine
    FOREIGN KEY (domaine_id) REFERENCES shared.domaine_secteur(id) ON DELETE SET NULL;

ALTER TABLE exchange.programme
    ADD CONSTRAINT fk_programme_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE exchange.programme
    ADD CONSTRAINT fk_programme_valide_par
    FOREIGN KEY (valide_par) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

ALTER TABLE exchange.candidature
    ADD CONSTRAINT fk_candidature_candidat
    FOREIGN KEY (candidat_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE exchange.candidature
    ADD CONSTRAINT fk_candidature_traite_par
    FOREIGN KEY (traite_par) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

-- ── innovation → shared & iam ───────────────────────────────────────────
ALTER TABLE innovation.innovation
    ADD CONSTRAINT fk_innovation_domaine
    FOREIGN KEY (domaine_id) REFERENCES shared.domaine_secteur(id) ON DELETE SET NULL;

ALTER TABLE innovation.innovation
    ADD CONSTRAINT fk_innovation_organisation
    FOREIGN KEY (organisation_id) REFERENCES iam.organisation(id) ON DELETE SET NULL;

ALTER TABLE innovation.innovation
    ADD CONSTRAINT fk_innovation_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE innovation.innovation
    ADD CONSTRAINT fk_innovation_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE innovation.projet
    ADD CONSTRAINT fk_projet_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE innovation.projet
    ADD CONSTRAINT fk_projet_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE innovation.projet
    ADD CONSTRAINT fk_projet_traite_par
    FOREIGN KEY (traite_par) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

ALTER TABLE innovation.africantive
    ADD CONSTRAINT fk_africantive_domaine
    FOREIGN KEY (domaine_id) REFERENCES shared.domaine_secteur(id) ON DELETE SET NULL;

ALTER TABLE innovation.africantive
    ADD CONSTRAINT fk_africantive_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE innovation.africantive
    ADD CONSTRAINT fk_africantive_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

-- ── culture → shared & iam ──────────────────────────────────────────────
ALTER TABLE culture.centre_culturel
    ADD CONSTRAINT fk_centre_culturel_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE culture.centre_culturel
    ADD CONSTRAINT fk_centre_culturel_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE culture.programmation_centre
    ADD CONSTRAINT fk_prog_centre_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

-- ── afrolang → iam ─────────────────────────────────────────────────────
ALTER TABLE afrolang.salle
    ADD CONSTRAINT fk_afrolang_salle_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE afrolang.salle
    ADD CONSTRAINT fk_afrolang_salle_moderateur
    FOREIGN KEY (moderateur_id) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

ALTER TABLE afrolang.salle_privee
    ADD CONSTRAINT fk_afrolang_privee_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE afrolang.session
    ADD CONSTRAINT fk_afrolang_session_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE afrolang.session
    ADD CONSTRAINT fk_afrolang_session_moderateur
    FOREIGN KEY (moderateur_id) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

ALTER TABLE afrolang.session_participant
    ADD CONSTRAINT fk_afrolang_participant_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE culture.codimoi
    ADD CONSTRAINT fk_codimoi_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE culture.codimoi
    ADD CONSTRAINT fk_codimoi_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE culture.codimoi_tag
    ADD CONSTRAINT fk_codimoi_tag_tag
    FOREIGN KEY (tag_id) REFERENCES shared.tag(id) ON DELETE CASCADE;

ALTER TABLE culture.codimoi_commentaire
    ADD CONSTRAINT fk_codimoi_comm_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE culture.codimoi_reaction
    ADD CONSTRAINT fk_codimoi_react_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

-- ── media_content → shared & iam ────────────────────────────────────────
ALTER TABLE media_content.programme_radio_tele
    ADD CONSTRAINT fk_radio_tele_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE media_content.programme_radio_tele
    ADD CONSTRAINT fk_radio_tele_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE media_content.evenement
    ADD CONSTRAINT fk_evenement_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE media_content.evenement
    ADD CONSTRAINT fk_evenement_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE media_content.evenement_inscription
    ADD CONSTRAINT fk_evt_inscription_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE media_content.mooc
    ADD CONSTRAINT fk_mooc_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE media_content.mooc
    ADD CONSTRAINT fk_mooc_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE media_content.mooc_inscription
    ADD CONSTRAINT fk_mooc_inscription_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE media_content.livre
    ADD CONSTRAINT fk_livre_categorie
    FOREIGN KEY (categorie_id) REFERENCES shared.categorie(id) ON DELETE SET NULL;

ALTER TABLE media_content.livre
    ADD CONSTRAINT fk_livre_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE media_content.livre_tag
    ADD CONSTRAINT fk_livre_tag_tag
    FOREIGN KEY (tag_id) REFERENCES shared.tag(id) ON DELETE CASCADE;

-- ── governance → shared & iam ───────────────────────────────────────────
ALTER TABLE governance.factcheck
    ADD CONSTRAINT fk_factcheck_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE governance.factcheck
    ADD CONSTRAINT fk_factcheck_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE governance.factcheck_commentaire
    ADD CONSTRAINT fk_fc_comm_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE governance.factcheck_reaction
    ADD CONSTRAINT fk_fc_react_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE governance.bad_habit
    ADD CONSTRAINT fk_bad_habit_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE RESTRICT;

ALTER TABLE governance.bad_habit
    ADD CONSTRAINT fk_bad_habit_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE governance.idea_force
    ADD CONSTRAINT fk_idea_force_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE RESTRICT;

ALTER TABLE governance.idea_force
    ADD CONSTRAINT fk_idea_force_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

-- ── country_profile → shared & iam ──────────────────────────────────────
ALTER TABLE country_profile.fiche_pays
    ADD CONSTRAINT fk_fiche_pays_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE RESTRICT;

ALTER TABLE country_profile.fiche_pays
    ADD CONSTRAINT fk_fiche_pays_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE country_profile.lien_interethnique
    ADD CONSTRAINT fk_lien_interethno_pays_lie
    FOREIGN KEY (pays_lie_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

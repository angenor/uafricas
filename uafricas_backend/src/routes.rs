use actix_web::web;

use crate::handlers::{admin, africantives, afripulse_public, afrolang, afrolang_ressources, amitie, annonces, appels, arbre_genealogique, auth, bibliotheques_humaines, centres_culturels, codimoi, collaboration, contribution_signalement, contributions_fiche, element_social, engagement, evenements, evenement_streaming, experts, facultes, fiches_pays, fiche_pays_social, gouvernance, livres, matching, membres, messagerie, moocs, notification, profil_social, projets, rendez_vous, retrouve_amis, retrouve_amis_public, sabbatiques, session_signalement, stations_radio, television, vidafrica, vidafrica_contribution};

/// Configure toutes les routes de l'API
pub fn configurer_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(crate::health_check))
            .route("/", web::get().to(crate::index))
            // Routes d'authentification
            .service(
                web::scope("/auth")
                    .route("/inscription", web::post().to(auth::inscription))
                    .route("/connexion", web::post().to(auth::connexion))
                    .route("/deconnexion", web::post().to(auth::deconnexion))
                    .route("/moi", web::get().to(auth::moi))
                    .route("/rafraichir", web::post().to(auth::rafraichir_token))
                    .route("/verifier-email", web::post().to(auth::verifier_email))
                    .route("/renvoyer-verification", web::post().to(auth::renvoyer_verification))
                    .route("/profil", web::put().to(auth::modifier_profil))
                    .route("/profil/photo", web::post().to(auth::uploader_photo_profil))
                    .route("/changer-mot-de-passe", web::post().to(auth::changer_mot_de_passe))
                    // Profil trouvable (Retrouve Amis)
                    .route("/profil/trouvable", web::patch().to(retrouve_amis::basculer_trouvable))
                    .route("/profil/parcours", web::get().to(retrouve_amis::lister_parcours))
                    .route("/profil/parcours", web::post().to(retrouve_amis::ajouter_parcours))
                    .route("/profil/parcours/{id}", web::put().to(retrouve_amis::modifier_parcours))
                    .route("/profil/parcours/{id}", web::delete().to(retrouve_amis::supprimer_parcours)),
            )
            // Routes d'administration
            .service(
                web::scope("/admin")
                    .route("/me", web::get().to(admin::admin_me))
                    // Utilisateurs
                    .route("/utilisateurs", web::get().to(admin::utilisateurs::lister_utilisateurs))
                    .route("/utilisateurs", web::post().to(admin::utilisateurs::creer_utilisateur))
                    .route("/utilisateurs/{id}", web::get().to(admin::utilisateurs::obtenir_utilisateur))
                    .route("/utilisateurs/{id}", web::put().to(admin::utilisateurs::modifier_utilisateur))
                    .route("/utilisateurs/{id}", web::delete().to(admin::utilisateurs::supprimer_utilisateur))
                    .route("/utilisateurs/{id}/etat", web::patch().to(admin::utilisateurs::changer_etat_utilisateur))
                    .route("/utilisateurs/{id}/compte-verifie-admin", web::patch().to(admin::utilisateurs::valider_compte_admin))
                    .route("/utilisateurs/{id}/roles", web::post().to(admin::utilisateurs::assigner_role))
                    .route("/utilisateurs/{id}/roles/{role_id}", web::delete().to(admin::utilisateurs::retirer_role))
                    .route("/utilisateurs/{id}/specialites", web::post().to(admin::utilisateurs::assigner_specialite))
                    .route("/utilisateurs/{id}/specialites/{spec_id}", web::delete().to(admin::utilisateurs::retirer_specialite))
                    .route("/utilisateurs/{id}/permissions", web::post().to(admin::utilisateurs::ajouter_permission_specifique))
                    .route("/utilisateurs/{id}/permissions/{perm_id}", web::delete().to(admin::utilisateurs::retirer_permission_specifique))
                    // Organisations
                    .route("/organisations", web::get().to(admin::organisations::lister_organisations))
                    .route("/organisations", web::post().to(admin::organisations::creer_organisation))
                    .route("/organisations/{id}", web::get().to(admin::organisations::obtenir_organisation))
                    .route("/organisations/{id}", web::put().to(admin::organisations::modifier_organisation))
                    .route("/organisations/{id}", web::delete().to(admin::organisations::supprimer_organisation))
                    // Partenariats
                    .route("/partenariats", web::get().to(admin::partenariats::lister_partenariats))
                    .route("/partenariats", web::post().to(admin::partenariats::creer_partenariat))
                    .route("/partenariats/{id}", web::get().to(admin::partenariats::obtenir_partenariat))
                    .route("/partenariats/{id}", web::put().to(admin::partenariats::modifier_partenariat))
                    .route("/partenariats/{id}", web::delete().to(admin::partenariats::supprimer_partenariat))
                    // Roles & Permissions
                    .route("/roles", web::get().to(admin::roles::lister_roles))
                    .route("/roles", web::post().to(admin::roles::creer_role))
                    .route("/roles/{id}", web::get().to(admin::roles::obtenir_role))
                    .route("/roles/{id}", web::put().to(admin::roles::modifier_role))
                    .route("/roles/{id}", web::delete().to(admin::roles::supprimer_role))
                    .route("/roles/{id}/permissions", web::post().to(admin::roles::assigner_permissions))
                    .route("/roles/{id}/permissions/{perm_id}", web::delete().to(admin::roles::retirer_permission))
                    .route("/permissions", web::get().to(admin::roles::lister_permissions))
                    // Referentiels - Pays
                    .route("/pays", web::get().to(admin::pays::lister_pays))
                    .route("/pays", web::post().to(admin::pays::creer_pays))
                    .route("/pays/{id}", web::get().to(admin::pays::obtenir_pays))
                    .route("/pays/{id}", web::put().to(admin::pays::modifier_pays))
                    .route("/pays/{id}", web::delete().to(admin::pays::supprimer_pays))
                    // Referentiels - Domaines
                    .route("/domaines", web::get().to(admin::domaines::lister_domaines))
                    .route("/domaines", web::post().to(admin::domaines::creer_domaine))
                    .route("/domaines/{id}", web::get().to(admin::domaines::obtenir_domaine))
                    .route("/domaines/{id}", web::put().to(admin::domaines::modifier_domaine))
                    .route("/domaines/{id}", web::delete().to(admin::domaines::supprimer_domaine))
                    // Referentiels - Categories
                    .route("/categories", web::get().to(admin::categories::lister_categories))
                    .route("/categories", web::post().to(admin::categories::creer_categorie))
                    .route("/categories/{id}", web::get().to(admin::categories::obtenir_categorie))
                    .route("/categories/{id}", web::put().to(admin::categories::modifier_categorie))
                    .route("/categories/{id}", web::delete().to(admin::categories::supprimer_categorie))
                    // Referentiels - Tags
                    .route("/tags", web::get().to(admin::tags::lister_tags))
                    .route("/tags", web::post().to(admin::tags::creer_tag))
                    .route("/tags/{id}", web::get().to(admin::tags::obtenir_tag))
                    .route("/tags/{id}", web::put().to(admin::tags::modifier_tag))
                    .route("/tags/{id}", web::delete().to(admin::tags::supprimer_tag))
                    // Referentiels - Medias
                    .route("/medias", web::get().to(admin::medias::lister_medias))
                    .route("/medias/{id}", web::get().to(admin::medias::obtenir_media))
                    .route("/medias/{id}", web::delete().to(admin::medias::supprimer_media))
                    // Referentiels - Specialites
                    .route("/specialites", web::get().to(admin::specialites::lister_specialites))
                    .route("/specialites", web::post().to(admin::specialites::creer_specialite))
                    .route("/specialites/{id}", web::get().to(admin::specialites::obtenir_specialite))
                    .route("/specialites/{id}", web::put().to(admin::specialites::modifier_specialite))
                    .route("/specialites/{id}", web::delete().to(admin::specialites::supprimer_specialite))
                    // Programmes d'echange
                    .route("/programmes", web::get().to(admin::programmes::lister_programmes))
                    .route("/programmes", web::post().to(admin::programmes::creer_programme))
                    .route("/programmes/{id}", web::get().to(admin::programmes::obtenir_programme))
                    .route("/programmes/{id}", web::put().to(admin::programmes::modifier_programme))
                    .route("/programmes/{id}", web::delete().to(admin::programmes::supprimer_programme))
                    .route("/programmes/{id}/etat", web::patch().to(admin::programmes::changer_etat_programme))
                    .route("/programmes/{id}/candidatures", web::get().to(admin::programmes::lister_candidatures_programme))
                    // Ecoles partenaires (INUDA)
                    .route("/ecoles-partenaires", web::get().to(admin::facultes::lister_ecoles))
                    .route("/ecoles-partenaires", web::post().to(admin::facultes::creer_ecole))
                    .route("/ecoles-partenaires/{id}", web::get().to(admin::facultes::obtenir_ecole))
                    .route("/ecoles-partenaires/{id}", web::put().to(admin::facultes::modifier_ecole))
                    .route("/ecoles-partenaires/{id}", web::delete().to(admin::facultes::supprimer_ecole))
                    // Facultes (INUDA)
                    .route("/facultes", web::get().to(admin::facultes::lister_facultes))
                    .route("/facultes", web::post().to(admin::facultes::creer_faculte))
                    .route("/facultes/{id}", web::get().to(admin::facultes::obtenir_faculte))
                    .route("/facultes/{id}", web::put().to(admin::facultes::modifier_faculte))
                    .route("/facultes/{id}", web::delete().to(admin::facultes::supprimer_faculte))
                    // Candidatures
                    .route("/candidatures", web::get().to(admin::candidatures::lister_candidatures))
                    .route("/candidatures/{id}", web::get().to(admin::candidatures::obtenir_candidature))
                    .route("/candidatures/{id}/etat", web::patch().to(admin::candidatures::changer_statut_candidature))
                    // Marche Africain - Annonces
                    .route("/annonces", web::get().to(admin::annonces::lister_annonces))
                    .route("/annonces", web::post().to(admin::annonces::creer_annonce))
                    .route("/annonces/{id}", web::get().to(admin::annonces::obtenir_annonce))
                    .route("/annonces/{id}", web::put().to(admin::annonces::modifier_annonce))
                    .route("/annonces/{id}", web::delete().to(admin::annonces::supprimer_annonce))
                    .route("/annonces/{id}/etat", web::patch().to(admin::annonces::changer_etat_annonce))
                    .route("/annonces/{id}/pays", web::post().to(admin::annonces::ajouter_pays_annonce))
                    .route("/annonces/{id}/pays/{pays_id}", web::delete().to(admin::annonces::retirer_pays_annonce))
                    .route("/annonces/{id}/medias", web::post().to(admin::annonces::ajouter_media_annonce))
                    .route("/annonces/{id}/medias/{media_id}", web::delete().to(admin::annonces::retirer_media_annonce))
                    .route("/annonces/{id}/medias/ordre", web::put().to(admin::annonces::reordonner_medias_annonce))
                    // Marche Africain - Favoris
                    .route("/annonces-favoris", web::get().to(admin::annonces_favoris::lister_favoris))
                    .route("/annonces-favoris/stats", web::get().to(admin::annonces_favoris::stats_favoris))
                    // Innovation - Innovations
                    .route("/innovations", web::get().to(admin::innovations::lister_innovations))
                    .route("/innovations", web::post().to(admin::innovations::creer_innovation))
                    .route("/innovations/{id}", web::get().to(admin::innovations::obtenir_innovation))
                    .route("/innovations/{id}", web::put().to(admin::innovations::modifier_innovation))
                    .route("/innovations/{id}", web::delete().to(admin::innovations::supprimer_innovation))
                    .route("/innovations/{id}/etat", web::patch().to(admin::innovations::changer_etat_innovation))
                    .route("/innovations/{id}/medias", web::post().to(admin::innovations::ajouter_media_innovation))
                    .route("/innovations/{id}/medias/{media_id}", web::delete().to(admin::innovations::retirer_media_innovation))
                    // Innovation - Projets
                    .route("/projets", web::get().to(admin::projets_admin::lister_projets))
                    .route("/projets", web::post().to(admin::projets_admin::creer_projet))
                    .route("/projets/{id}", web::get().to(admin::projets_admin::obtenir_projet))
                    .route("/projets/{id}", web::put().to(admin::projets_admin::modifier_projet))
                    .route("/projets/{id}", web::delete().to(admin::projets_admin::supprimer_projet))
                    .route("/projets/{id}/etat", web::patch().to(admin::projets_admin::changer_etat_projet))
                    .route("/projets/{id}/documents", web::post().to(admin::projets_admin::ajouter_document_projet))
                    .route("/projets/{id}/documents/{doc_id}", web::delete().to(admin::projets_admin::retirer_document_projet))
                    // Innovation - Africantives
                    .route("/africantives", web::get().to(admin::africantives_admin::lister_africantives))
                    .route("/africantives", web::post().to(admin::africantives_admin::creer_africantive))
                    .route("/africantives/{id}", web::get().to(admin::africantives_admin::obtenir_africantive))
                    .route("/africantives/{id}", web::put().to(admin::africantives_admin::modifier_africantive))
                    .route("/africantives/{id}", web::delete().to(admin::africantives_admin::supprimer_africantive))
                    .route("/africantives/{id}/etat", web::patch().to(admin::africantives_admin::changer_etat_africantive))
                    // Culture - Centres culturels
                    .route("/centres-culturels", web::get().to(admin::centres_culturels::lister_centres))
                    .route("/centres-culturels", web::post().to(admin::centres_culturels::creer_centre))
                    .route("/centres-culturels/{id}", web::get().to(admin::centres_culturels::obtenir_centre))
                    .route("/centres-culturels/{id}", web::put().to(admin::centres_culturels::modifier_centre))
                    .route("/centres-culturels/{id}", web::delete().to(admin::centres_culturels::supprimer_centre))
                    .route("/centres-culturels/{id}/membres", web::get().to(admin::centres_culturels::lister_membres))
                    .route("/centres-culturels/{id}/membres", web::post().to(admin::centres_culturels::ajouter_membre))
                    .route("/centres-culturels/{id}/membres/{membre_id}", web::put().to(admin::centres_culturels::modifier_membre))
                    .route("/centres-culturels/{id}/membres/{membre_id}", web::delete().to(admin::centres_culturels::retirer_membre))
                    // Culture - Programmations
                    .route("/programmations", web::get().to(admin::programmations::lister_programmations))
                    .route("/programmations", web::post().to(admin::programmations::creer_programmation))
                    .route("/programmations/{id}", web::get().to(admin::programmations::obtenir_programmation))
                    .route("/programmations/{id}", web::put().to(admin::programmations::modifier_programmation))
                    .route("/programmations/{id}", web::delete().to(admin::programmations::supprimer_programmation))
                    .route("/programmations/{id}/inscriptions", web::get().to(admin::programmations::lister_inscriptions))
                    // Culture - Codi-Moi
                    .route("/codimoi", web::get().to(admin::codimoi_admin::lister_codimoi))
                    .route("/codimoi", web::post().to(admin::codimoi_admin::creer_codimoi))
                    .route("/codimoi/{id}", web::get().to(admin::codimoi_admin::obtenir_codimoi))
                    .route("/codimoi/{id}", web::put().to(admin::codimoi_admin::modifier_codimoi))
                    .route("/codimoi/{id}", web::delete().to(admin::codimoi_admin::supprimer_codimoi))
                    .route("/codimoi/{id}/tags", web::post().to(admin::codimoi_admin::ajouter_tag))
                    .route("/codimoi/{id}/tags/{tag_id}", web::delete().to(admin::codimoi_admin::retirer_tag))
                    .route("/codimoi/{id}/commentaires", web::get().to(admin::codimoi_admin::lister_commentaires))
                    .route("/codimoi/{id}/commentaires/{commentaire_id}", web::delete().to(admin::codimoi_admin::supprimer_commentaire))
                    .route("/codimoi/{id}/reactions", web::get().to(admin::codimoi_admin::obtenir_reactions))
                    // Engagement / gamification (barème + journal)
                    .route("/engagement/regles", web::get().to(admin::engagement::lister_regles))
                    .route("/engagement/regles/{id}", web::put().to(admin::engagement::modifier_regle))
                    .route("/engagement/paliers", web::get().to(admin::engagement::lister_paliers))
                    .route("/engagement/paliers", web::post().to(admin::engagement::creer_palier))
                    .route("/engagement/paliers/{id}", web::put().to(admin::engagement::modifier_palier))
                    .route("/engagement/paliers/{id}", web::delete().to(admin::engagement::desactiver_palier))
                    .route("/engagement/niveaux", web::get().to(admin::engagement::lister_niveaux))
                    .route("/engagement/niveaux/{id}", web::put().to(admin::engagement::modifier_niveau))
                    .route("/engagement/journal", web::get().to(admin::engagement::lister_journal))
                    .route("/engagement/ajustement", web::post().to(admin::engagement::ajuster_points))
                    .route("/engagement/mise-en-avant", web::post().to(admin::engagement::mettre_en_avant))
                    .route("/engagement/mise-en-avant/{type_objet}/{objet_id}", web::get().to(admin::engagement::statut_mise_en_avant))
                    .route("/engagement/mise-en-avant/{type_objet}/{objet_id}", web::delete().to(admin::engagement::retirer_mise_en_avant))
                    // AfroLang - Salles publiques
                    .route("/salles", web::get().to(admin::salles::lister_salles))
                    .route("/salles", web::post().to(admin::salles::creer_salle))
                    .route("/salles/{id}", web::get().to(admin::salles::obtenir_salle))
                    .route("/salles/{id}", web::put().to(admin::salles::modifier_salle))
                    .route("/salles/{id}", web::delete().to(admin::salles::supprimer_salle))
                    // AfroLang - Salles privees (supervision)
                    .route("/salles-privees", web::get().to(admin::salles_privees::lister_salles_privees))
                    .route("/salles-privees/{id}", web::get().to(admin::salles_privees::obtenir_salle_privee))
                    // AfroLang - Sessions
                    .route("/sessions", web::get().to(admin::sessions_afrolang::lister_sessions))
                    .route("/sessions/{id}", web::get().to(admin::sessions_afrolang::obtenir_session))
                    .route("/sessions/{id}/participants", web::get().to(admin::sessions_afrolang::lister_participants))
                    .route("/sessions/{id}/tableau-blanc", web::get().to(admin::sessions_afrolang::obtenir_tableau_blanc))
                    // AfroLang - Modérateurs attitrés (feature 005, US3) — salles publiques uniquement
                    .route("/afrolang/salles/{salle_id}/moderateurs", web::get().to(admin::moderateurs_afrolang::lister_moderateurs_attitres))
                    .route("/afrolang/salles/{salle_id}/moderateurs", web::post().to(admin::moderateurs_afrolang::designer_moderateur))
                    .route("/afrolang/salles/{salle_id}/moderateurs/{utilisateur_id}", web::delete().to(admin::moderateurs_afrolang::retirer_moderateur))
                    // AfroLang - Ressources & liens externes (feature 005, US6)
                    .route("/afrolang/ressources/en-attente", web::get().to(admin::sessions_afrolang::lister_liens_en_attente))
                    .route("/afrolang/ressources/{id}/publier", web::post().to(admin::sessions_afrolang::publier_lien))
                    .route("/afrolang/ressources/{id}/refuser", web::post().to(admin::sessions_afrolang::refuser_lien))
                    // AfroLang - Archivage salles privées (feature 005, Phase 9)
                    .route("/afrolang/salles-privees/archiver-batch-utilisateur", web::post().to(admin::sessions_afrolang::archiver_batch_utilisateur))
                    .route("/afrolang/salles-privees/{id}/archiver", web::post().to(admin::sessions_afrolang::archiver_salle_privee))
                    .route("/afrolang/salles/{id}/desactiver", web::post().to(admin::sessions_afrolang::desactiver_salle_publique_avec_cascade))
                    // AfroLang - Modération admin (feature 001-ressources-fermeture-session)
                    .route("/afrolang/sessions/{session_id}/fermer-admin", web::post().to(admin::sessions_moderation::fermer_session_admin))
                    .route("/afrolang/salles/{salle_id}/reactiver", web::post().to(admin::sessions_moderation::reactiver_salle))
                    .route("/afrolang/salles/{salle_id}/historique-moderation", web::get().to(admin::sessions_moderation::historique_moderation))
                    // AfroLang - Pays d'origine (feature 001-afrolang-pays-origine)
                    .route("/afrolang/salles/{id}/pays", web::post().to(admin::salles::ajouter_pays_origine_salle))
                    .route("/afrolang/salles/{id}/pays/{pays_id}", web::delete().to(admin::salles::retirer_pays_origine_salle))
                    // AfroLang - Propositions communautaires (feature 001-admin-salles-publiques, US2)
                    .route("/afrolang/propositions", web::get().to(admin::propositions_salle::lister_propositions_admin))
                    .route("/afrolang/propositions/{id}", web::get().to(admin::propositions_salle::obtenir_proposition_admin))
                    .route("/afrolang/propositions/{id}/valider", web::patch().to(admin::propositions_salle::valider_proposition))
                    .route("/afrolang/propositions/{id}/rejeter", web::patch().to(admin::propositions_salle::rejeter_proposition))
                    // AfroLang - Administrateurs de salle publique (feature 001-admin-salles-publiques, US3)
                    .route("/afrolang/salles/{salle_id}/administrateurs", web::get().to(admin::salles::lister_administrateurs_salle))
                    .route("/afrolang/salles/{salle_id}/administrateurs", web::post().to(admin::salles::nommer_administrateur_salle))
                    .route("/afrolang/salles/{salle_id}/administrateurs/{utilisateur_id}", web::delete().to(admin::salles::revoquer_administrateur_salle))
                    // Gouvernance - FactCheck
                    .route("/factcheck", web::get().to(admin::gouvernance::lister_factchecks))
                    .route("/factcheck", web::post().to(admin::gouvernance::creer_factcheck))
                    .route("/factcheck/{id}", web::get().to(admin::gouvernance::obtenir_factcheck))
                    .route("/factcheck/{id}", web::put().to(admin::gouvernance::modifier_factcheck))
                    .route("/factcheck/{id}", web::delete().to(admin::gouvernance::supprimer_factcheck))
                    .route("/factcheck/{id}/commentaires", web::get().to(admin::gouvernance::lister_commentaires_factcheck))
                    .route("/factcheck/{id}/commentaires/{commentaire_id}", web::delete().to(admin::gouvernance::supprimer_commentaire_factcheck))
                    .route("/factcheck/{id}/reactions", web::get().to(admin::gouvernance::obtenir_reactions_factcheck))
                    // Gouvernance - Mauvaises pratiques
                    .route("/bad-habits", web::get().to(admin::gouvernance::lister_bad_habits))
                    .route("/bad-habits", web::post().to(admin::gouvernance::creer_bad_habit))
                    .route("/bad-habits/{id}", web::get().to(admin::gouvernance::obtenir_bad_habit))
                    .route("/bad-habits/{id}", web::put().to(admin::gouvernance::modifier_bad_habit))
                    .route("/bad-habits/{id}", web::delete().to(admin::gouvernance::supprimer_bad_habit))
                    .route("/bad-habits/{id}/medias", web::get().to(admin::gouvernance::lister_medias_bad_habit))
                    .route("/bad-habits/{id}/medias", web::post().to(admin::gouvernance::ajouter_media_bad_habit))
                    .route("/bad-habits/{id}/medias/{media_id}", web::delete().to(admin::gouvernance::retirer_media_bad_habit))
                    // Gouvernance - Idees forces
                    .route("/idea-forces", web::get().to(admin::gouvernance::lister_idea_forces))
                    .route("/idea-forces", web::post().to(admin::gouvernance::creer_idea_force))
                    .route("/idea-forces/{id}", web::get().to(admin::gouvernance::obtenir_idea_force))
                    .route("/idea-forces/{id}", web::put().to(admin::gouvernance::modifier_idea_force))
                    .route("/idea-forces/{id}", web::delete().to(admin::gouvernance::supprimer_idea_force))
                    .route("/idea-forces/{id}/medias", web::get().to(admin::gouvernance::lister_medias_idea_force))
                    .route("/idea-forces/{id}/medias", web::post().to(admin::gouvernance::ajouter_media_idea_force))
                    .route("/idea-forces/{id}/medias/{media_id}", web::delete().to(admin::gouvernance::retirer_media_idea_force))
                    // Medias & Contenus - Stations Radio
                    .route("/stations-radio", web::get().to(admin::radio_tele::lister_stations_radio))
                    .route("/stations-radio", web::post().to(admin::radio_tele::creer_station_radio))
                    .route("/stations-radio/{id}", web::get().to(admin::radio_tele::obtenir_station_radio))
                    .route("/stations-radio/{id}", web::put().to(admin::radio_tele::modifier_station_radio))
                    .route("/stations-radio/{id}", web::delete().to(admin::radio_tele::supprimer_station_radio))
                    // Medias & Contenus - Chaines TV
                    .route("/chaines-tv", web::get().to(admin::radio_tele::lister_chaines_tv))
                    .route("/chaines-tv", web::post().to(admin::radio_tele::creer_chaine_tv))
                    .route("/chaines-tv/{id}", web::get().to(admin::radio_tele::obtenir_chaine_tv))
                    .route("/chaines-tv/{id}", web::put().to(admin::radio_tele::modifier_chaine_tv))
                    .route("/chaines-tv/{id}", web::delete().to(admin::radio_tele::supprimer_chaine_tv))
                    // Medias & Contenus - Programmes Radio/Tele
                    .route("/programmes-radio", web::get().to(admin::radio_tele::lister_programmes_radio))
                    .route("/programmes-radio", web::post().to(admin::radio_tele::creer_programme_radio))
                    .route("/programmes-radio/{id}", web::get().to(admin::radio_tele::obtenir_programme_radio))
                    .route("/programmes-radio/{id}", web::put().to(admin::radio_tele::modifier_programme_radio))
                    .route("/programmes-radio/{id}", web::delete().to(admin::radio_tele::supprimer_programme_radio))

                    .route("/programmes-tele", web::get().to(admin::radio_tele::lister_programmes_tele))
                    .route("/programmes-tele", web::post().to(admin::radio_tele::creer_programme_tele))
                    .route("/programmes-tele/{id}", web::get().to(admin::radio_tele::obtenir_programme_tele))
                    .route("/programmes-tele/{id}", web::put().to(admin::radio_tele::modifier_programme_tele))
                    .route("/programmes-tele/{id}", web::delete().to(admin::radio_tele::supprimer_programme_tele))
                    .route("/medias/upload", web::post().to(admin::radio_tele::uploader_media))
                    // Medias & Contenus - Evenements
                    .route("/evenements", web::get().to(admin::evenements::lister_evenements))
                    .route("/evenements", web::post().to(admin::evenements::creer_evenement))
                    .route("/evenements/{id}", web::get().to(admin::evenements::obtenir_evenement))
                    .route("/evenements/{id}", web::put().to(admin::evenements::modifier_evenement))
                    .route("/evenements/{id}", web::delete().to(admin::evenements::supprimer_evenement))
                    .route("/evenements/{id}/etat", web::patch().to(admin::evenements::changer_etat_evenement))
                    .route("/evenements/{id}/inscriptions", web::get().to(admin::evenements::lister_inscriptions))
                    .route("/evenements/{id}/inscriptions/{insc_id}/statut", web::patch().to(admin::evenements::changer_statut_inscription))
                    .route("/evenements/{id}/inscriptions/stats", web::get().to(admin::evenements::stats_inscriptions))
                    // Medias & Contenus - MOOC
                    .route("/mooc", web::get().to(admin::mooc::lister_moocs))
                    .route("/mooc", web::post().to(admin::mooc::creer_mooc))
                    // Programme (chapitres/leçons) — /mooc/upload AVANT /mooc/{id} pour ne pas être capturé par {id}
                    .route("/mooc/upload", web::post().to(admin::formation_contenu::uploader_fichier_formation))
                    .route("/mooc/{id}/chapitres", web::get().to(admin::formation_contenu::lister_contenu))
                    .route("/mooc/{id}/chapitres", web::post().to(admin::formation_contenu::creer_chapitre))
                    .route("/mooc/{id}/chapitres/reordonner", web::put().to(admin::formation_contenu::reordonner_chapitres))
                    .route("/mooc/{id}", web::get().to(admin::mooc::obtenir_mooc))
                    .route("/mooc/{id}", web::put().to(admin::mooc::modifier_mooc))
                    .route("/mooc/{id}", web::delete().to(admin::mooc::supprimer_mooc))
                    .route("/mooc/{id}/etat", web::patch().to(admin::mooc::changer_etat_mooc))
                    .route("/mooc/{id}/inscriptions", web::get().to(admin::mooc::lister_inscriptions))
                    .route("/mooc/{id}/inscriptions/stats", web::get().to(admin::mooc::stats_inscriptions))
                    // Programme (chapitres/leçons) — édition directe par id
                    .route("/chapitres/{chapitre_id}", web::put().to(admin::formation_contenu::modifier_chapitre))
                    .route("/chapitres/{chapitre_id}", web::delete().to(admin::formation_contenu::supprimer_chapitre))
                    .route("/chapitres/{chapitre_id}/lecons", web::post().to(admin::formation_contenu::creer_lecon))
                    .route("/chapitres/{chapitre_id}/lecons/reordonner", web::put().to(admin::formation_contenu::reordonner_lecons))
                    .route("/lecons/{lecon_id}", web::put().to(admin::formation_contenu::modifier_lecon))
                    .route("/lecons/{lecon_id}", web::delete().to(admin::formation_contenu::supprimer_lecon))
                    // Medias & Contenus - Livres (Bibliotheque)
                    .route("/livres", web::get().to(admin::livres::lister_livres))
                    .route("/livres", web::post().to(admin::livres::creer_livre))
                    .route("/livres/{id}", web::get().to(admin::livres::obtenir_livre))
                    .route("/livres/{id}", web::put().to(admin::livres::modifier_livre))
                    .route("/livres/{id}", web::delete().to(admin::livres::supprimer_livre))
                    .route("/livres/{id}/etat", web::patch().to(admin::livres::changer_etat_livre))
                    .route("/livres/{id}/tags", web::post().to(admin::livres::ajouter_tag))
                    .route("/livres/{id}/tags/{tag_id}", web::delete().to(admin::livres::retirer_tag))
                    // Dashboard
                    .route("/dashboard/stats", web::get().to(admin::dashboard::obtenir_stats))
                    .route("/dashboard/activite-recente", web::get().to(admin::dashboard::lister_activite_recente))
                    .route("/dashboard/tendances", web::get().to(admin::dashboard::obtenir_tendances))
                    // Audit & Logs
                    .route("/audit", web::get().to(admin::audit::lister_audit))
                    .route("/audit/{id}", web::get().to(admin::audit::obtenir_audit))
                    // Profils Pays - CRUD principal
                    .route("/profils-pays", web::get().to(admin::profils_pays::lister_fiches_pays))
                    .route("/profils-pays", web::post().to(admin::profils_pays::creer_fiche_pays))
                    // Profils Pays - Contributions (routes statiques avant parametrees)
                    .route("/profils-pays/contributions", web::get().to(admin::profils_pays::lister_contributions))
                    .route("/profils-pays/contributions/{contrib_id}", web::get().to(admin::profils_pays::obtenir_contribution))
                    .route("/profils-pays/contributions/{contrib_id}/etat", web::patch().to(admin::profils_pays::moderer_contribution))
                    .route("/profils-pays/contributions/{contrib_id}/retirer", web::post().to(admin::profils_pays::retirer_contribution_approuvee))
                    // Contributions suspendues par signalement communautaire (réactivation)
                    .route("/profils-pays/contributions-suspendues", web::get().to(admin::profils_pays::lister_contributions_suspendues))
                    .route("/profils-pays/contributions-suspendues/{type_objet}/{objet_id}/reactiver", web::post().to(admin::profils_pays::reactiver_contribution))
                    // Profils Pays - Routes parametrees
                    .route("/profils-pays/{id}", web::get().to(admin::profils_pays::obtenir_fiche_pays))
                    .route("/profils-pays/{id}", web::put().to(admin::profils_pays::modifier_fiche_pays))
                    .route("/profils-pays/{id}", web::delete().to(admin::profils_pays::supprimer_fiche_pays))
                    .route("/profils-pays/{id}/debloquer", web::patch().to(admin::profils_pays::debloquer_fiche_pays))
                    // Profils Pays - Regions
                    .route("/profils-pays/{id}/regions", web::get().to(admin::profils_pays::lister_regions))
                    .route("/profils-pays/{id}/regions", web::post().to(admin::profils_pays::creer_region))
                    .route("/profils-pays/{id}/regions/{region_id}", web::put().to(admin::profils_pays::modifier_region))
                    .route("/profils-pays/{id}/regions/{region_id}", web::delete().to(admin::profils_pays::supprimer_region))
                    // Profils Pays - Groupes ethniques
                    .route("/profils-pays/{id}/groupes-ethniques", web::get().to(admin::profils_pays::lister_groupes_ethniques))
                    .route("/profils-pays/{id}/groupes-ethniques", web::post().to(admin::profils_pays::creer_groupe_ethnique))
                    .route("/profils-pays/{id}/groupes-ethniques/{ge_id}", web::put().to(admin::profils_pays::modifier_groupe_ethnique))
                    .route("/profils-pays/{id}/groupes-ethniques/{ge_id}", web::delete().to(admin::profils_pays::supprimer_groupe_ethnique))
                    // Profils Pays - Alliances interethniques
                    .route("/profils-pays/{id}/alliances", web::get().to(admin::profils_pays::lister_alliances))
                    .route("/profils-pays/{id}/alliances", web::post().to(admin::profils_pays::creer_alliance))
                    .route("/profils-pays/{id}/alliances/{alliance_id}", web::put().to(admin::profils_pays::modifier_alliance))
                    .route("/profils-pays/{id}/alliances/{alliance_id}", web::delete().to(admin::profils_pays::supprimer_alliance))
                    // Profils Pays - Contes & Histoires
                    .route("/profils-pays/{id}/contes", web::get().to(admin::profils_pays::lister_contes))
                    .route("/profils-pays/{id}/contes", web::post().to(admin::profils_pays::creer_conte))
                    .route("/profils-pays/{id}/contes/{conte_id}", web::put().to(admin::profils_pays::modifier_conte))
                    .route("/profils-pays/{id}/contes/{conte_id}", web::delete().to(admin::profils_pays::supprimer_conte))
                    // Profils Pays - Sites touristiques
                    .route("/profils-pays/{id}/sites-touristiques", web::get().to(admin::profils_pays::lister_sites_touristiques))
                    .route("/profils-pays/{id}/sites-touristiques", web::post().to(admin::profils_pays::creer_site_touristique))
                    .route("/profils-pays/{id}/sites-touristiques/{site_id}", web::put().to(admin::profils_pays::modifier_site_touristique))
                    .route("/profils-pays/{id}/sites-touristiques/{site_id}", web::delete().to(admin::profils_pays::supprimer_site_touristique))
                    .route("/profils-pays/{id}/sites-touristiques/{site_id}/verification", web::patch().to(admin::profils_pays::definir_verification_site))
                    // Moderation des avis de site (US5)
                    .route("/sites-touristiques/avis/{avis_id}/masquer", web::patch().to(admin::profils_pays::masquer_avis_site))
                    // Profils Pays - Secteurs de developpement
                    .route("/profils-pays/{id}/secteurs", web::get().to(admin::profils_pays::lister_secteurs))
                    .route("/profils-pays/{id}/secteurs", web::post().to(admin::profils_pays::creer_secteur))
                    .route("/profils-pays/{id}/secteurs/{secteur_id}", web::put().to(admin::profils_pays::modifier_secteur))
                    .route("/profils-pays/{id}/secteurs/{secteur_id}", web::delete().to(admin::profils_pays::supprimer_secteur))
                    // Profils Pays - Saisons
                    .route("/profils-pays/{id}/saisons", web::get().to(admin::profils_pays::lister_saisons))
                    .route("/profils-pays/{id}/saisons", web::post().to(admin::profils_pays::creer_saison))
                    .route("/profils-pays/{id}/saisons/{saison_id}", web::put().to(admin::profils_pays::modifier_saison))
                    .route("/profils-pays/{id}/saisons/{saison_id}", web::delete().to(admin::profils_pays::supprimer_saison))
                    // Profils Pays - Liens interethniques
                    .route("/profils-pays/{id}/liens-interethniques", web::get().to(admin::profils_pays::lister_liens_interethniques))
                    .route("/profils-pays/{id}/liens-interethniques", web::post().to(admin::profils_pays::creer_lien_interethnique))
                    .route("/profils-pays/{id}/liens-interethniques/{lien_id}", web::put().to(admin::profils_pays::modifier_lien_interethnique))
                    .route("/profils-pays/{id}/liens-interethniques/{lien_id}", web::delete().to(admin::profils_pays::supprimer_lien_interethnique))
                    // Retrouve Amis - Avis de recherche
                    .route("/retrouve-amis/avis", web::get().to(admin::retrouve_amis::lister_avis_admin))
                    .route("/retrouve-amis/avis/{id}", web::get().to(admin::retrouve_amis::detail_avis_admin))
                    .route("/retrouve-amis/avis/{id}/etat", web::patch().to(admin::retrouve_amis::changer_etat_avis))
                    // Retrouve Amis - Signalements
                    .route("/retrouve-amis/signalements", web::get().to(admin::retrouve_amis::lister_signalements))
                    .route("/retrouve-amis/signalements/{id}", web::get().to(admin::retrouve_amis::detail_signalement))
                    .route("/retrouve-amis/signalements/{id}/moderer", web::patch().to(admin::retrouve_amis::moderer_signalement))
                    // Retrouve Amis - Statistiques
                    .route("/retrouve-amis/statistiques", web::get().to(admin::retrouve_amis::statistiques))
                    // Retrouve Amis - Demandes de retrait
                    .route("/retrouve-amis/demandes-retrait", web::get().to(admin::retrouve_amis::lister_demandes_retrait))
                    .route("/retrouve-amis/demandes-retrait/{id}/statuer", web::patch().to(admin::retrouve_amis::statuer_demande_retrait))
                    // Vidafrica - Vidéos
                    .route("/vidafrica/videos", web::get().to(admin::vidafrica::lister_videos))
                    .route("/vidafrica/videos", web::post().to(admin::vidafrica::creer_video))
                    .route("/vidafrica/videos/{id}", web::get().to(admin::vidafrica::obtenir_video))
                    .route("/vidafrica/videos/{id}", web::put().to(admin::vidafrica::modifier_video))
                    .route("/vidafrica/videos/{id}", web::delete().to(admin::vidafrica::supprimer_video))
                    .route("/vidafrica/videos/{id}/etat", web::patch().to(admin::vidafrica::changer_etat_video))
                    // Vidafrica - Pistes de sous-titres
                    .route("/vidafrica/videos/{video_id}/pistes", web::get().to(admin::vidafrica::lister_pistes))
                    .route("/vidafrica/videos/{video_id}/pistes", web::post().to(admin::vidafrica::creer_piste))
                    .route("/vidafrica/pistes/{id}/etat", web::patch().to(admin::vidafrica::changer_etat_piste))
                    .route("/vidafrica/pistes/{id}", web::delete().to(admin::vidafrica::supprimer_piste))
                    // Vidafrica - Segments
                    .route("/vidafrica/pistes/{piste_id}/segments", web::get().to(admin::vidafrica::lister_segments))
                    .route("/vidafrica/pistes/{piste_id}/segments", web::post().to(admin::vidafrica::creer_segment))
                    .route("/vidafrica/pistes/{piste_id}/segments/reordonner", web::put().to(admin::vidafrica::reordonner_segments))
                    .route("/vidafrica/segments/{id}", web::put().to(admin::vidafrica::modifier_segment))
                    .route("/vidafrica/segments/{id}", web::delete().to(admin::vidafrica::supprimer_segment))
                    // Vidafrica - Timings mot
                    .route("/vidafrica/segments/{segment_id}/timings-mot", web::post().to(admin::vidafrica::enregistrer_timings_mot))
                    .route("/vidafrica/segments/{segment_id}/timings-mot", web::delete().to(admin::vidafrica::supprimer_timings_mot))
                    // Bibliotheques Humaines admin (US1 + US2)
                    .route("/bibliotheques-humaines", web::get().to(admin::bibliotheques_humaines::lister_demandes))
                    .route("/bibliotheques-humaines/{id}", web::get().to(admin::bibliotheques_humaines::obtenir_demande))
                    .route("/bibliotheques-humaines/{id}/valider", web::patch().to(admin::bibliotheques_humaines::valider_demande))
                    .route("/bibliotheques-humaines/{id}/rejeter", web::patch().to(admin::bibliotheques_humaines::rejeter_demande))
                    // Demandes d'expertise admin (US2)
                    .route("/experts", web::get().to(admin::expertise::lister_demandes))
                    .route("/experts/{id}", web::get().to(admin::expertise::obtenir_demande))
                    .route("/experts/{id}/valider", web::patch().to(admin::expertise::valider_demande))
                    .route("/experts/{id}/rejeter", web::patch().to(admin::expertise::rejeter_demande)),
            )
            // Routes Retrouve Amis
            .service(
                web::scope("/retrouve-amis")
                    // Pays (liste publique sans auth)
                    .route("/pays", web::get().to(retrouve_amis::lister_pays))
                    // Pages publiques (sans auth)
                    .route("/public/rechercher", web::get().to(retrouve_amis_public::rechercher_avis_publics))
                    .route("/public/{slug}", web::get().to(retrouve_amis_public::detail_avis_public))
                    .route("/public/{slug}/partage", web::post().to(retrouve_amis_public::incrementer_partage))
                    // Signalement, demande de retrait et reponse depuis la page publique (avec auth)
                    .route("/public/{slug}/signaler", web::post().to(retrouve_amis::signaler_avis_public))
                    .route("/public/{slug}/demande-retrait", web::post().to(retrouve_amis::demander_retrait))
                    .route("/public/{slug}/repondre", web::post().to(retrouve_amis::repondre_avis_public))
                    // Avis de recherche
                    .route("/avis", web::post().to(retrouve_amis::creer_avis))
                    .route("/avis", web::get().to(retrouve_amis::lister_avis))
                    .route("/avis/{id}", web::get().to(retrouve_amis::detail_avis))
                    .route("/avis/{id}", web::put().to(retrouve_amis::modifier_avis))
                    .route("/avis/{id}/cloturer", web::patch().to(retrouve_amis::cloturer_avis))
                    .route("/avis/{id}", web::delete().to(retrouve_amis::supprimer_avis))
                    // Route publier_avis supprimee (003) — publication automatique a la creation
                    .route("/avis/{id}/signaler", web::post().to(retrouve_amis::signaler_avis))
                    // Correspondances
                    .route("/correspondances", web::get().to(retrouve_amis::lister_correspondances))
                    .route("/correspondances/{id}", web::get().to(retrouve_amis::detail_correspondance))
                    .route("/correspondances/{id}/accepter", web::post().to(retrouve_amis::accepter_correspondance))
                    .route("/correspondances/{id}/refuser", web::post().to(retrouve_amis::refuser_correspondance))
                    // Notifications
                    .route("/notifications", web::get().to(retrouve_amis::lister_notifications))
                    .route("/notifications/{id}/lire", web::patch().to(retrouve_amis::marquer_lu))
                    .route("/notifications/tout-lire", web::patch().to(retrouve_amis::tout_marquer_lu))
                    // Tableau de bord
                    .route("/tableau-de-bord", web::get().to(retrouve_amis::tableau_de_bord)),
            )
            // Routes des livres
            .service(
                web::scope("/livres")
                    .route("", web::get().to(livres::lister_livres))
                    .route("", web::post().to(livres::creer_livre))
                    .route("/{id}", web::get().to(livres::obtenir_livre))
                    .route("/{id}", web::delete().to(livres::supprimer_livre)),
            )
            // Routes des bibliotheques humaines
            .service(
                web::scope("/bibliotheques-humaines")
                    .route("", web::get().to(bibliotheques_humaines::lister_biblios))
                    .route("/specialites", web::get().to(bibliotheques_humaines::lister_specialites))
                    .route("/inscription", web::post().to(bibliotheques_humaines::inscrire_biblio))
                    .route("/moi/demande", web::get().to(bibliotheques_humaines::ma_demande))
                    .route("/{id}", web::get().to(bibliotheques_humaines::obtenir_biblio))
                    .route("/{id}/reaction", web::post().to(bibliotheques_humaines::reagir_biblio))
                    .route("/{id}/commentaires", web::get().to(bibliotheques_humaines::lister_commentaires_biblio))
                    .route("/{id}/commentaires", web::post().to(bibliotheques_humaines::creer_commentaire_biblio))
                    .route("/{id}/commentaires/{commentaire_id}", web::delete().to(bibliotheques_humaines::supprimer_commentaire_biblio))
                    .route("/{id}/recommandation", web::post().to(bibliotheques_humaines::recommander_biblio))
                    .route("/{id}/recommandation", web::delete().to(bibliotheques_humaines::retirer_recommandation_biblio))
                    .route("/{id}/recommandations", web::get().to(bibliotheques_humaines::lister_recommandations_biblio)),
            )
            // Routes des centres culturels
            .service(
                web::scope("/centres-culturels")
                    .route("", web::get().to(centres_culturels::lister_centres))
                    .route("/{id}", web::get().to(centres_culturels::obtenir_centre))
                    .route("/{centre_id}/programmations/{id}", web::get().to(centres_culturels::obtenir_programmation))
                    .route("/{centre_id}/programmations/{id}/inscription", web::post().to(centres_culturels::inscrire_programmation))
                    .route("/{centre_id}/programmations/{id}/inscription", web::delete().to(centres_culturels::desinscrire_programmation)),
            )
            // Routes de gouvernance citoyenne
            .service(
                web::scope("/gouvernance")
                    .route("/stats", web::get().to(gouvernance::obtenir_stats))
                    .route("/contributions", web::get().to(gouvernance::lister_contributions))
                    .route("/partages", web::get().to(gouvernance::lister_partages_contributions))
                    .route("/partages", web::post().to(gouvernance::partager_contribution))
                    .route("/factcheck", web::post().to(gouvernance::creer_factcheck_public))
                    .route("/factcheck/upload-preuve", web::post().to(gouvernance::uploader_preuve_factcheck))
                    .route("/factcheck/{id}/reaction", web::post().to(gouvernance::reagir_factcheck))
                    .route("/factcheck/{id}/signalement", web::post().to(gouvernance::signaler_factcheck))
                    .route("/bad-habits", web::post().to(gouvernance::creer_bad_habit_public))
                    .route("/idea-forces", web::post().to(gouvernance::creer_idea_force_public)),
            )
            // Liste publique des pays pour selecteurs
            .route("/pays", web::get().to(gouvernance::lister_pays_public))
            // Routes Codi-Moi
            .service(
                web::scope("/codimoi")
                    .route("", web::get().to(codimoi::lister_posts))
                    .route("", web::post().to(codimoi::creer_post))
                    .route("/{id}", web::get().to(codimoi::obtenir_post))
                    .route("/{id}/reaction", web::post().to(codimoi::reagir))
                    .route("/{id}/commentaires", web::get().to(codimoi::lister_commentaires))
                    .route("/{id}/commentaires", web::post().to(codimoi::creer_commentaire)),
            )
            // Routes Engagement / gamification (lecture publique)
            .service(
                web::scope("/engagement")
                    .route("/mon-compte", web::get().to(engagement::mon_compte))
                    .route("/mon-journal", web::get().to(engagement::mon_journal))
                    .route("/niveau/{utilisateur_id}", web::get().to(engagement::niveau_utilisateur)),
            )
            // Routes des annonces (Marche Africain)
            .service(
                web::scope("/annonces")
                    .route("", web::get().to(annonces::lister_annonces))
                    // Publication membre (auth via JWT dans le handler)
                    .route("", web::post().to(annonces::creer_annonce_membre))
                    // Routes statiques AVANT la route dynamique /{id} (D8)
                    .route("/categories", web::get().to(annonces::lister_categories_annonce))
                    .route("/secteurs", web::get().to(annonces::lister_secteurs_annonce))
                    .route("/mes-annonces", web::get().to(annonces::mes_annonces))
                    .route("/favoris", web::get().to(annonces::mes_favoris))
                    .route("/{id}", web::get().to(annonces::obtenir_annonce))
                    .route("/{id}", web::put().to(annonces::modifier_annonce_membre))
                    .route("/{id}", web::delete().to(annonces::supprimer_annonce_membre))
                    .route("/{id}/conclure", web::patch().to(annonces::conclure_annonce))
                    .route("/{id}/contacter", web::post().to(annonces::contacter_auteur))
                    .route("/{id}/favori", web::post().to(annonces::ajouter_favori))
                    .route("/{id}/favori", web::delete().to(annonces::retirer_favori))
                    .route("/{id}/medias", web::post().to(annonces::ajouter_medias_membre))
                    .route("/{id}/medias/{media_id}", web::delete().to(annonces::supprimer_media_membre)),
            )
            // Routes des evenements
            .service(
                web::scope("/evenements")
                    .route("", web::get().to(evenements::lister_evenements))
                    .route("", web::post().to(evenements::creer_evenement))
                    // Gestion par l'organisateur (membre) — declare avant /{id}
                    .route("/mes-evenements", web::get().to(evenements::lister_mes_evenements))
                    .route("/{id}", web::get().to(evenements::obtenir_evenement))
                    .route("/{id}", web::put().to(evenements::modifier_mon_evenement))
                    .route("/{id}", web::delete().to(evenements::supprimer_mon_evenement))
                    .route("/{id}/couverture", web::post().to(evenements::changer_couverture_mon_evenement))
                    .route("/{id}/inscrits", web::get().to(evenements::lister_inscrits_mon_evenement))
                    .route("/{id}/inscription", web::post().to(evenements::inscrire_evenement))
                    // Direct en streaming (feature 001-evenements-streaming)
                    .route("/{id}/direct", web::get().to(evenement_streaming::etat_direct))
                    .route("/{id}/direct/rejoindre", web::post().to(evenement_streaming::rejoindre))
                    .route("/{id}/direct/quitter", web::post().to(evenement_streaming::quitter))
                    .route("/{id}/direct/cloturer", web::post().to(evenement_streaming::cloturer))
                    .route("/{id}/direct/lever-main", web::post().to(evenement_streaming::lever_main))
                    .route("/{id}/direct/participants/{utilisateur_id}/promouvoir", web::post().to(evenement_streaming::promouvoir))
                    .route("/{id}/direct/participants/{utilisateur_id}/retrograder", web::post().to(evenement_streaming::retrograder))
                    .route("/{id}/direct/participants/{utilisateur_id}/retirer", web::post().to(evenement_streaming::retirer)),
            )
            // Routes des projets (financer un projet)
            .service(
                web::scope("/projets")
                    .route("", web::get().to(projets::lister_projets))
                    .route("", web::post().to(projets::creer_projet))
                    .route("/statistiques", web::get().to(projets::obtenir_statistiques))
                    .route("/{id}", web::get().to(projets::obtenir_projet)),
            )
            // Routes des experts
            .service(
                web::scope("/experts")
                    .route("", web::get().to(experts::lister_experts))
                    .route("/cv", web::post().to(experts::uploader_cv))
                    .route("/candidature", web::post().to(experts::creer_candidature))
                    .route("/moi", web::get().to(experts::ma_candidature))
                    .route("/{id}", web::get().to(experts::obtenir_expert))
                    .route("/{id}/note", web::post().to(experts::noter_expert)),
            )
            // Routes annuaire des membres (profils publics)
            .service(
                web::scope("/utilisateurs")
                    .route("", web::get().to(membres::lister_membres))
                    // Mur communautaire des profils partagés (avant /{id})
                    .route("/partages", web::get().to(profil_social::lister_partages_profils))
                    .route("/{id}", web::get().to(membres::obtenir_membre))
                    .route("/{id}/partages", web::post().to(profil_social::partager_profil))
                    .route("/{id}/signalement", web::post().to(profil_social::signaler_profil)),
            )
            // Routes des amitiés & relations sociales (US1 + US2 + US4)
            .service(
                web::scope("/amities")
                    // Demandes
                    .route("/demandes", web::post().to(amitie::creer_demande))
                    .route("/demandes/recues", web::get().to(amitie::lister_demandes_recues))
                    .route("/demandes/envoyees", web::get().to(amitie::lister_demandes_envoyees))
                    .route("/demandes/{id}/accepter", web::post().to(amitie::accepter_demande))
                    .route("/demandes/{id}/refuser", web::post().to(amitie::refuser_demande))
                    // Annulation d'une demande émise (US4, FR-010)
                    .route("/demandes/{id}", web::delete().to(amitie::annuler_demande))
                    // États de relation
                    .route("/etat/{utilisateur_id}", web::get().to(amitie::etat_relation))
                    .route("/etats", web::post().to(amitie::etats_relation_lot))
                    // Notifications relationnelles
                    .route("/notifications", web::get().to(amitie::lister_notifications))
                    .route("/notifications/tout-lu", web::patch().to(amitie::marquer_tout_lu))
                    .route("/notifications/{id}/lu", web::patch().to(amitie::marquer_notification_lue))
                    // Liste des amis & retrait (US4, FR-011/FR-012)
                    .route("", web::get().to(amitie::lister_amis))
                    .route("/{utilisateur_id}", web::delete().to(amitie::retirer_ami)),
            )
            // Routes de blocage (US4, FR-013)
            .service(
                web::scope("/blocages")
                    .route("", web::get().to(amitie::lister_blocages))
                    .route("", web::post().to(amitie::bloquer))
                    .route("/{utilisateur_id}", web::delete().to(amitie::debloquer)),
            )
            // Routes de la messagerie privée temps réel (US3)
            .service(
                web::scope("/messagerie")
                    .route("/flux", web::get().to(messagerie::flux))
                    .route("/non-lus", web::get().to(messagerie::compteur_non_lus))
                    .route("/conversations", web::get().to(messagerie::lister_conversations))
                    .route("/conversations/{ami_id}/messages", web::get().to(messagerie::lister_messages))
                    .route("/conversations/{ami_id}/messages", web::post().to(messagerie::envoyer_message))
                    .route("/conversations/{ami_id}/lu", web::post().to(messagerie::marquer_conversation_lue))
                    .route("/messages/{id}", web::delete().to(messagerie::supprimer_message)),
            )
            // Routes des rendez-vous en visioconférence (feature 001-rendez-vous-visio)
            .service(
                web::scope("/rendez-vous")
                    .route("", web::post().to(rendez_vous::proposer))
                    .route("", web::get().to(rendez_vous::lister))
                    .route("/{id}", web::get().to(rendez_vous::detail))
                    .route("/{id}/salle", web::get().to(rendez_vous::salle))
                    .route("/{id}/accepter", web::post().to(rendez_vous::accepter))
                    .route("/{id}/refuser", web::post().to(rendez_vous::refuser))
                    .route("/{id}/contre-proposer", web::post().to(rendez_vous::contre_proposer))
                    .route("/{id}/annuler", web::post().to(rendez_vous::annuler)),
            )
            // Routes des appels directs en visioconférence (sans rendez-vous)
            .service(
                web::scope("/appels")
                    .route("", web::post().to(appels::appeler))
                    .route("/{id}/salle", web::get().to(appels::salle))
                    .route("/{id}/refuser", web::post().to(appels::refuser))
                    .route("/{id}/annuler", web::post().to(appels::annuler)),
            )
            // Routes des formations (MOOC/CLOM)
            .service(
                web::scope("/moocs")
                    .route("", web::get().to(moocs::lister_moocs))
                    .route("/{id}", web::get().to(moocs::obtenir_mooc))
                    .route("/{id}/contenu", web::get().to(moocs::obtenir_contenu_mooc))
                    .route("/{id}/inscription", web::post().to(moocs::inscrire_mooc))
                    .route(
                        "/{id}/lecons/{lecon_id}/completion",
                        web::post().to(moocs::marquer_lecon_terminee),
                    )
                    .route(
                        "/{id}/lecons/{lecon_id}/completion",
                        web::delete().to(moocs::annuler_lecon_terminee),
                    ),
            )
            // Routes des programmes sabbatiques
            .service(
                web::scope("/sabbatiques")
                    .route("", web::get().to(sabbatiques::lister_programmes))
                    .route("", web::post().to(sabbatiques::creer_programme))
                    .route(
                        "/mes-programmes",
                        web::get().to(sabbatiques::lister_mes_programmes),
                    )
                    .route("/{id}", web::get().to(sabbatiques::obtenir_programme))
                    .route(
                        "/{id}/candidatures",
                        web::get().to(sabbatiques::lister_candidatures),
                    )
                    .route(
                        "/{id}/candidatures",
                        web::post().to(sabbatiques::candidater),
                    )
                    .route(
                        "/{id}/candidatures/{candidature_id}/retenir",
                        web::post().to(sabbatiques::selectionner_candidat),
                    ),
            )
            // Routes des facultes INUDA
            .service(
                web::scope("/facultes")
                    .route("", web::get().to(facultes::lister_facultes))
                    .route("/{id}", web::get().to(facultes::obtenir_faculte)),
            )
            // Statistiques agregees Muniversa (bande d'accueil /universite)
            .service(
                web::scope("/universite")
                    .route("/stats", web::get().to(facultes::stats_universite)),
            )
            // Routes des stations radio
            .service(
                web::scope("/stations-radio")
                    .route("", web::get().to(stations_radio::lister_stations))
                    .route("", web::post().to(stations_radio::creer_station))
                    .route("/pays", web::get().to(stations_radio::lister_pays_stations))
                    .route("/genres", web::get().to(stations_radio::lister_genres_stations))
                    .route("/{id}", web::get().to(stations_radio::obtenir_station)),
            )
            // Routes des fiches pays (Opportunites en Afrique)
            .service(
                web::scope("/fiches-pays")
                    .route("", web::get().to(fiches_pays::lister_fiches))
                    // US3 — creation d'une nouvelle fiche pays (soumission)
                    .route("", web::post().to(afripulse_public::creer_fiche_pays))
                    .route("/regions", web::get().to(fiches_pays::lister_regions))
                    // Mur communautaire : fiches partagées (public, paginé)
                    .route("/partages", web::get().to(fiche_pays_social::lister_partages))
                    // T071 — mes contributions (utilisateur connecte)
                    .route("/moi/contributions", web::get().to(afripulse_public::lister_mes_contributions))
                    // Upload d'image isolée pour une contribution (site, personnalité)
                    .route("/contributions/upload-image", web::post().to(contributions_fiche::uploader_image_contribution))
                    // Signalement d'une contribution individuelle (suspension au seuil)
                    .route("/contributions/{type_objet}/{objet_id}/signalement", web::post().to(contribution_signalement::signaler_contribution))
                    // La modération des contributions (valider/rejeter) se fait via
                    // /api/admin/profils-pays/contributions/{id}/etat (admin::profils_pays::moderer_contribution).
                    // Routes parametrees
                    .route("/{id}", web::get().to(fiches_pays::obtenir_fiche))
                    .route("/{id}/contributions", web::get().to(contributions_fiche::lister_contributions))
                    .route("/{id}/contributions", web::post().to(contributions_fiche::soumettre_contribution))
                    .route("/{id}/contributions/multipart", web::post().to(contributions_fiche::soumettre_contribution_multipart))
                    .route("/{id}/contributeurs", web::get().to(contributions_fiche::lister_contributeurs))
                    // Réactions, signalement (blocage au seuil) et partage communautaire
                    .route("/{id}/reaction", web::post().to(fiche_pays_social::reagir_fiche))
                    .route("/{id}/signalement", web::post().to(fiche_pays_social::signaler_fiche))
                    .route("/{id}/partages", web::post().to(fiche_pays_social::partager_fiche))
                    // Sous-objets afripulse — réactions like/dislike & partage communautaire.
                    // Route littérale (mur) AVANT les routes paramétrées.
                    .route("/elements/partages", web::get().to(element_social::lister_partages_elements))
                    .route("/elements/{type_objet}/{objet_id}/reaction", web::post().to(element_social::reagir_element))
                    .route("/elements/{type_objet}/{objet_id}/partages", web::post().to(element_social::partager_element))
                    // Sections Afripulse enrichies (US1)
                    .route("/{id}/sites-touristiques", web::get().to(afripulse_public::lister_sites_touristiques))
                    .route("/{id}/sites-touristiques/{site_id}", web::get().to(afripulse_public::obtenir_site_touristique))
                    .route("/{id}/secteurs-opportunites", web::get().to(afripulse_public::lister_secteurs_opportunites))
                    .route("/{id}/secteurs-opportunites/{secteur_id}", web::get().to(afripulse_public::obtenir_secteur_opportunite))
                    .route("/{id}/recettes-culinaires", web::get().to(afripulse_public::lister_recettes_culinaires))
                    .route("/{id}/recettes-culinaires/{recette_id}", web::get().to(afripulse_public::obtenir_recette_culinaire))
                    .route("/{id}/personnalites", web::get().to(afripulse_public::lister_personnalites))
                    .route("/{id}/personnalites/{personnalite_id}", web::get().to(afripulse_public::obtenir_personnalite))
                    .route("/{id}/savoirs-pratiques", web::get().to(afripulse_public::lister_savoirs_pratiques))
                    // US4 — galerie photos + recommandations
                    .route("/{id}/recommandations", web::get().to(afripulse_public::lister_recommandations))
                    .route("/{id}/galerie-photos", web::get().to(afripulse_public::lister_galerie_photos)),
            )
            // Avis de visiteurs sur un site touristique (US5 - ecriture directe)
            .service(
                web::scope("/sites-touristiques")
                    .route("/{site_id}/avis", web::get().to(afripulse_public::lister_avis_site))
                    .route("/{site_id}/avis", web::post().to(afripulse_public::soumettre_avis_site)),
            )
            // Routes des africantives (initiatives africaines)
            .service(
                web::scope("/africantives")
                    .route("", web::get().to(africantives::lister_africantives))
                    .route("", web::post().to(africantives::creer_africantive))
                    .route("/domaines", web::get().to(africantives::lister_domaines))
                    .route("/pays", web::get().to(africantives::lister_pays))
                    .route("/{id}", web::get().to(africantives::obtenir_africantive)),
            )
            // Routes Afrolang (visioconference WebRTC)
            .service(
                web::scope("/afrolang")
                    // Annuaire des groupes ethniques (utilisé côté admin uniquement)
                    .route("/groupes-ethniques", web::get().to(afrolang::lister_groupes_ethniques))
                    // Salles publiques (création restreinte aux admins)
                    .route("/salles", web::get().to(afrolang::lister_salles))
                    .route("/salles", web::post().to(afrolang::creer_salle))
                    .route("/salles/{id}", web::get().to(afrolang::obtenir_salle))
                    .route("/salles/{id}", web::put().to(afrolang::modifier_salle))
                    .route("/salles/{id}", web::delete().to(afrolang::supprimer_salle))
                    // Salles privées (refonte : code secret)
                    .route("/salles/{salle_id}/salles-privees", web::get().to(afrolang::lister_salles_privees_par_salle_publique))
                    .route("/salles-privees", web::post().to(afrolang::creer_salle_privee_publique))
                    .route("/salles-privees/{id}/verifier-code", web::post().to(afrolang::verifier_code_acces_salle_privee))
                    .route("/salles-privees/{id}/sessions/demarrer-ou-rejoindre", web::post().to(afrolang::demarrer_ou_rejoindre_session_salle_privee))
                    .route("/salles-privees/{id}/code-acces", web::patch().to(afrolang::modifier_code_acces_salle_privee))
                    .route("/salles-privees/{id}/archiver", web::post().to(afrolang::archiver_salle_privee_par_auteur))
                    .route("/salles-privees/{id}/max-participants", web::patch().to(afrolang::modifier_max_participants_salle_privee))
                    .route("/salles-privees/{id}", web::get().to(afrolang::obtenir_salle_privee))
                    .route("/salles-privees/{id}", web::put().to(afrolang::modifier_salle_privee))
                    .route("/salles-privees/{id}", web::delete().to(afrolang::supprimer_salle_privee))
                    // Ressources contribuées communauté (feature 001-ressources-fermeture-session, US1)
                    .route("/salles/{salle_id}/ressources-contribuees", web::get().to(afrolang_ressources::lister_ressources_contribuees))
                    .route("/salles/{salle_id}/ressources-contribuees", web::post().to(afrolang_ressources::ajouter_ressource_contribuee))
                    .route("/ressources-contribuees/{id}", web::delete().to(afrolang_ressources::supprimer_ressource_contribuee))
                    // Workflow accompagnateur
                    .route("/accompagnateur/recommandations-recues", web::get().to(afrolang_ressources::lister_recommandations_recues))
                    .route("/ressources-contribuees/{id}/accepter", web::post().to(afrolang_ressources::accepter_recommandation))
                    .route("/ressources-contribuees/{id}/refuser", web::post().to(afrolang_ressources::refuser_recommandation))
                    .route("/ressources-contribuees/{id}/retirer-consentement", web::post().to(afrolang_ressources::retirer_consentement))
                    // Ressources de salle publique (feature 005, US6)
                    .route("/salles/{salle_id}/ressources", web::get().to(afrolang::lister_ressources))
                    .route("/salles/{salle_id}/ressources/fichier", web::post().to(afrolang::uploader_ressource_fichier))
                    .route("/salles/{salle_id}/ressources/lien", web::post().to(afrolang::soumettre_lien_externe))
                    .route("/ressources/{id}", web::delete().to(afrolang::supprimer_ressource))
                    // Messagerie de session (feature 005, US6)
                    .route("/sessions/{id}/messages", web::get().to(afrolang::lister_messages_session))
                    .route("/sessions/{id}/messages", web::post().to(afrolang::envoyer_message_session))
                    // Propositions communautaires (feature 001-admin-salles-publiques, US1)
                    .route("/propositions", web::post().to(afrolang::soumettre_proposition))
                    .route("/propositions/moi", web::get().to(afrolang::lister_mes_propositions))
                    .route("/propositions/{id}/retirer", web::patch().to(afrolang::retirer_ma_proposition))
                    .route("/pays-disponibles", web::get().to(afrolang::lister_pays_disponibles))
                    .route("/territoires", web::get().to(afrolang::lister_territoires))
                    // Sessions - salles publiques (feature 005 Option A)
                    .route("/salles/{salle_id}/sessions", web::get().to(afrolang::lister_sessions_salle_publique))
                    .route("/salles/{salle_id}/sessions", web::post().to(afrolang::creer_session_salle_publique))
                    // US1 refonte : démarrer/rejoindre en 1 appel pour streaming direct
                    .route("/salles/{salle_id}/sessions/demarrer-ou-rejoindre", web::post().to(afrolang::demarrer_ou_rejoindre_session_salle_publique))
                    // Sessions - salles privées
                    .route("/salles-privees/{sp_id}/sessions", web::get().to(afrolang::lister_sessions))
                    .route("/salles-privees/{sp_id}/sessions", web::post().to(afrolang::creer_session))
                    .route("/sessions/{id}", web::get().to(afrolang::obtenir_session))
                    .route("/sessions/{id}/demarrer", web::put().to(afrolang::demarrer_session))
                    .route("/sessions/{id}/terminer", web::put().to(afrolang::terminer_session))
                    .route("/sessions/{id}/rejoindre", web::post().to(afrolang::rejoindre_session))
                    .route("/sessions/{id}/quitter", web::post().to(afrolang::quitter_session))
                    .route("/sessions/{id}/fermer-pour-abus", web::post().to(afrolang::fermer_session_pour_abus))
                    // Signalement communautaire de la salle hôte (suspension au seuil)
                    .route("/sessions/{id}/signalement", web::post().to(session_signalement::signaler_session))
                    // Transfert de modération (feature 005, US3)
                    .route("/sessions/{id}/moderation/transferer", web::put().to(afrolang::transferer_moderation_session))
                    // Passation de modération multi-modérateurs (refonte 2026-06)
                    .route("/sessions/{id}/passation/accepter", web::post().to(afrolang::accepter_passation))
                    .route("/sessions/{id}/passation/finaliser", web::post().to(afrolang::finaliser_passation))
                    // Phase 3 : Token LiveKit
                    .route("/sessions/{id}/token", web::post().to(afrolang::generer_token_session))
                    // Phase 4 : Tableau blanc
                    .route("/sessions/{id}/tableau-blanc", web::get().to(afrolang::obtenir_tableau_blanc))
                    .route("/sessions/{id}/tableau-blanc", web::put().to(afrolang::sauvegarder_tableau_blanc))
                    .route("/sessions/{id}/tableau-blanc", web::delete().to(afrolang::effacer_tableau_blanc))
                    // Feature 001-session-moderation : permissions tableau blanc
                    .route(
                        "/sessions/{id}/permissions-tableau-blanc",
                        web::get().to(afrolang::lister_permissions_tableau_blanc),
                    )
                    .route(
                        "/sessions/{id}/permissions-tableau-blanc",
                        web::post().to(afrolang::accorder_permission_tableau_blanc),
                    )
                    .route(
                        "/sessions/{id}/permissions-tableau-blanc/{user_id}",
                        web::delete().to(afrolang::retirer_permission_tableau_blanc),
                    )
                    // Feature 001-session-moderation : spotlight (mise en évidence)
                    .route(
                        "/sessions/{id}/spotlight",
                        web::post().to(afrolang::mettre_en_evidence),
                    )
                    .route(
                        "/sessions/{id}/spotlight",
                        web::delete().to(afrolang::retirer_mise_en_evidence),
                    )
                    // Utilitaires
                    .route("/stats", web::get().to(afrolang::obtenir_stats))
                    .route("/langues", web::get().to(afrolang::lister_langues)),
            )
            // Routes Notifications
            .service(
                web::scope("/notifications")
                    .route("/compteur", web::get().to(notification::compteur_notifications))
                    .route("", web::get().to(notification::lister_notifications))
                    .route("/{id}/lire", web::post().to(notification::marquer_lue))
                    .route("/tout-lire", web::post().to(notification::tout_marquer_lu)),
            )
            // Routes Arbre généalogique
            .service(
                web::scope("/arbre")
                    .route("/arbre-complet", web::get().to(arbre_genealogique::obtenir_arbre_complet))
                    .route("/personnes", web::get().to(arbre_genealogique::lister_personnes))
                    .route("/personnes", web::post().to(arbre_genealogique::creer_personne))
                    .route("/personnes/{id}", web::get().to(arbre_genealogique::obtenir_personne))
                    .route("/personnes/{id}", web::put().to(arbre_genealogique::modifier_personne))
                    .route("/personnes/{id}", web::delete().to(arbre_genealogique::supprimer_personne))
                    .route("/personnes/{id}/photo", web::post().to(arbre_genealogique::uploader_photo))
                    .route("/liens", web::post().to(arbre_genealogique::creer_lien))
                    .route("/liens/{id}", web::delete().to(arbre_genealogique::supprimer_lien))
                    // Routes Matching / Découvertes
                    .route("/decouvertes", web::get().to(matching::lister_decouvertes))
                    .route("/decouvertes/{id}/confirmer", web::post().to(matching::confirmer_suggestion))
                    .route("/decouvertes/{id}/rejeter", web::post().to(matching::rejeter_suggestion))
                    .route("/decouvertes/{id}/branches", web::get().to(matching::obtenir_branches))
                    .route("/decouvertes/{id}/demande-contact", web::post().to(matching::demander_contact))
                    .route("/demandes-contact/{id}/accepter", web::post().to(matching::accepter_demande))
                    .route("/demandes-contact/{id}/refuser", web::post().to(matching::refuser_demande))
                    .route("/recherche-publique", web::get().to(matching::recherche_publique))
                    // Routes Collaboration
                    .route("/mes-arbres", web::get().to(collaboration::mes_arbres))
                    .route("/invitations", web::post().to(collaboration::creer_invitation))
                    .route("/invitations", web::get().to(collaboration::lister_invitations_recues))
                    .route("/invitations/{id}/accepter", web::post().to(collaboration::accepter_invitation))
                    .route("/invitations/{id}/refuser", web::post().to(collaboration::refuser_invitation))
                    .route("/{arbre_id}/collaborateurs", web::get().to(collaboration::lister_collaborateurs))
                    .route("/collaborateurs/{id}", web::put().to(collaboration::modifier_permission))
                    .route("/collaborateurs/{id}", web::delete().to(collaboration::revoquer_collaborateur))
                    .route("/{arbre_id}/confidentialite", web::put().to(collaboration::modifier_confidentialite_arbre))
                    .route("/personnes/{id}/confidentialite", web::put().to(collaboration::modifier_confidentialite_personne))
                    .route("/{arbre_id}/historique", web::get().to(collaboration::obtenir_historique))
                    // Routes Doublons
                    .route("/doublons", web::get().to(notification::detecter_doublons))
                    .route("/doublons/ignorer", web::post().to(notification::ignorer_doublon))
                    .route("/doublons/fusionner", web::post().to(notification::fusionner_doublons)),
            )
            // Routes Vidafrica (public)
            .service(
                web::scope("/vidafrica")
                    // Lecture publique
                    .route("/videos", web::get().to(vidafrica::lister_videos_publiques))
                    // Proposition d'une vidéo par un membre (auth JWT dans le handler)
                    .route("/videos", web::post().to(vidafrica_contribution::proposer_video))
                    .route("/langues-sous-titres", web::get().to(vidafrica::lister_langues_disponibles))
                    // Mur des partages de vidéos (route littérale AVANT /videos/{slug})
                    .route("/videos/partages", web::get().to(vidafrica::lister_partages_videos))
                    // Réaction & partage d'une vidéo (auth JWT dans le handler)
                    .route("/videos/{id}/reaction", web::post().to(vidafrica_contribution::reagir_video))
                    .route("/videos/{id}/partage", web::post().to(vidafrica_contribution::partager_video))
                    // Contribution membre — sous-titres (routes spécifiques AVANT /videos/{slug})
                    .route("/videos/{video_id}/mes-pistes", web::get().to(vidafrica_contribution::mes_pistes))
                    .route("/videos/{video_id}/pistes", web::post().to(vidafrica_contribution::creer_piste_membre))
                    .route("/videos/{video_id}/sous-titres/{langue}", web::get().to(vidafrica::obtenir_sous_titres))
                    .route("/videos/{slug}", web::get().to(vidafrica::obtenir_video_publique))
                    .route("/pistes/{id}", web::delete().to(vidafrica_contribution::supprimer_piste_membre))
                    .route("/pistes/{piste_id}/segments", web::get().to(vidafrica_contribution::lister_segments_membre))
                    .route("/pistes/{piste_id}/segments", web::post().to(vidafrica_contribution::creer_segment_membre))
                    .route("/segments/{id}", web::put().to(vidafrica_contribution::modifier_segment_membre))
                    .route("/segments/{id}", web::delete().to(vidafrica_contribution::supprimer_segment_membre))
                    .route("/segments/{segment_id}/timings-mot", web::post().to(vidafrica_contribution::enregistrer_timings_mot_membre))
                    .route("/segments/{segment_id}/timings-mot", web::delete().to(vidafrica_contribution::supprimer_timings_mot_membre)),
            )
            // Routes de la télévision
            .service(
                web::scope("/television")
                    .route("/chaines", web::get().to(television::lister_chaines))
                    .route("/chaines", web::post().to(television::creer_chaine))
                    .route("/chaines/{id}", web::get().to(television::obtenir_chaine))
                    .route("/programmes-vedettes", web::get().to(television::lister_programmes_vedettes))
                    .route("/programmes-vedettes", web::post().to(television::creer_programme_vedette))
                    .route("/programmes-vedettes/{id}", web::get().to(television::obtenir_programme_vedette))
                    .route("/pays", web::get().to(television::lister_pays_television))
                    .route("/categories", web::get().to(television::lister_categories_television))
                    .route("/stats", web::get().to(television::obtenir_stats_television)),
            ),
    );
}

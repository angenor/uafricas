use actix_web::web;

use crate::handlers::{admin, africantives, afrolang, annonces, auth, bibliotheques_humaines, centres_culturels, codimoi, contributions_fiche, evenements, experts, facultes, fiches_pays, gouvernance, livres, moocs, projets, sabbatiques, stations_radio, television};

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
                    .route("/renvoyer-verification", web::post().to(auth::renvoyer_verification)),
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
                    .route("/codimoi/{id}/reactions", web::get().to(admin::codimoi_admin::obtenir_reactions)),
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
                    .route("/{id}", web::get().to(bibliotheques_humaines::obtenir_biblio)),
            )
            // Routes des centres culturels
            .service(
                web::scope("/centres-culturels")
                    .route("", web::get().to(centres_culturels::lister_centres))
                    .route("/{id}", web::get().to(centres_culturels::obtenir_centre))
                    .route("/{centre_id}/programmations/{id}", web::get().to(centres_culturels::obtenir_programmation)),
            )
            // Routes de gouvernance citoyenne
            .service(
                web::scope("/gouvernance")
                    .route("/stats", web::get().to(gouvernance::obtenir_stats))
                    .route("/contributions", web::get().to(gouvernance::lister_contributions)),
            )
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
            // Routes des annonces (Marche Africain)
            .service(
                web::scope("/annonces")
                    .route("", web::get().to(annonces::lister_annonces))
                    .route("/{id}", web::get().to(annonces::obtenir_annonce)),
            )
            // Routes des evenements
            .service(
                web::scope("/evenements")
                    .route("", web::get().to(evenements::lister_evenements))
                    .route("", web::post().to(evenements::creer_evenement))
                    .route("/{id}", web::get().to(evenements::obtenir_evenement))
                    .route("/{id}/inscription", web::post().to(evenements::inscrire_evenement)),
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
                    .route("/candidature", web::post().to(experts::creer_candidature))
                    .route("/{id}", web::get().to(experts::obtenir_expert)),
            )
            // Routes des formations (MOOC/CLOM)
            .service(
                web::scope("/moocs")
                    .route("", web::get().to(moocs::lister_moocs))
                    .route("/{id}", web::get().to(moocs::obtenir_mooc))
                    .route("/{id}/inscription", web::post().to(moocs::inscrire_mooc)),
            )
            // Routes des programmes sabbatiques
            .service(
                web::scope("/sabbatiques")
                    .route("", web::get().to(sabbatiques::lister_programmes))
                    .route("", web::post().to(sabbatiques::creer_programme))
                    .route("/{id}", web::get().to(sabbatiques::obtenir_programme)),
            )
            // Routes des facultes INUDA
            .service(
                web::scope("/facultes")
                    .route("", web::get().to(facultes::lister_facultes))
                    .route("/{id}", web::get().to(facultes::obtenir_faculte)),
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
                    .route("/regions", web::get().to(fiches_pays::lister_regions))
                    // Contributions (routes statiques avant parametrees)
                    .route("/contributions/{id}/valider", web::put().to(contributions_fiche::valider_contribution))
                    .route("/contributions/{id}/rejeter", web::put().to(contributions_fiche::rejeter_contribution))
                    // Routes parametrees
                    .route("/{id}", web::get().to(fiches_pays::obtenir_fiche))
                    .route("/{id}/contributions", web::get().to(contributions_fiche::lister_contributions))
                    .route("/{id}/contributions", web::post().to(contributions_fiche::soumettre_contribution))
                    .route("/{id}/contributeurs", web::get().to(contributions_fiche::lister_contributeurs)),
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
                    // Salles publiques
                    .route("/salles", web::get().to(afrolang::lister_salles))
                    .route("/salles", web::post().to(afrolang::creer_salle))
                    .route("/salles/{id}", web::get().to(afrolang::obtenir_salle))
                    .route("/salles/{id}", web::put().to(afrolang::modifier_salle))
                    .route("/salles/{id}", web::delete().to(afrolang::supprimer_salle))
                    // Salles privees (sous une salle publique)
                    .route("/salles/{salle_id}/privees", web::get().to(afrolang::lister_salles_privees))
                    .route("/salles/{salle_id}/privees", web::post().to(afrolang::creer_salle_privee))
                    // Salles privees (CRUD direct)
                    .route("/salles-privees/{id}", web::get().to(afrolang::obtenir_salle_privee))
                    .route("/salles-privees/{id}", web::put().to(afrolang::modifier_salle_privee))
                    .route("/salles-privees/{id}", web::delete().to(afrolang::supprimer_salle_privee))
                    // Sessions
                    .route("/salles-privees/{sp_id}/sessions", web::get().to(afrolang::lister_sessions))
                    .route("/salles-privees/{sp_id}/sessions", web::post().to(afrolang::creer_session))
                    .route("/sessions/{id}", web::get().to(afrolang::obtenir_session))
                    .route("/sessions/{id}/demarrer", web::put().to(afrolang::demarrer_session))
                    .route("/sessions/{id}/terminer", web::put().to(afrolang::terminer_session))
                    .route("/sessions/{id}/rejoindre", web::post().to(afrolang::rejoindre_session))
                    .route("/sessions/{id}/quitter", web::post().to(afrolang::quitter_session))
                    // Phase 3 : Token LiveKit
                    .route("/sessions/{id}/token", web::post().to(afrolang::generer_token_session))
                    // Phase 4 : Tableau blanc
                    .route("/sessions/{id}/tableau-blanc", web::get().to(afrolang::obtenir_tableau_blanc))
                    .route("/sessions/{id}/tableau-blanc", web::put().to(afrolang::sauvegarder_tableau_blanc))
                    .route("/sessions/{id}/tableau-blanc", web::delete().to(afrolang::effacer_tableau_blanc))
                    // Utilitaires
                    .route("/stats", web::get().to(afrolang::obtenir_stats))
                    .route("/langues", web::get().to(afrolang::lister_langues)),
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

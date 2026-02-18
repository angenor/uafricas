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
                    .route("/me", web::get().to(admin::admin_me)),
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

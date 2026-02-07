use actix_web::web;

use crate::handlers::{annonces, auth, centres_culturels, codimoi, evenements, facultes, fiches_pays, gouvernance, livres, moocs, sabbatiques};

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
                    .route("/rafraichir", web::post().to(auth::rafraichir_token)),
            )
            // Routes des livres
            .service(
                web::scope("/livres")
                    .route("", web::get().to(livres::lister_livres))
                    .route("", web::post().to(livres::creer_livre))
                    .route("/{id}", web::get().to(livres::obtenir_livre))
                    .route("/{id}", web::delete().to(livres::supprimer_livre)),
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
            // Routes des fiches pays (Opportunites en Afrique)
            .service(
                web::scope("/fiches-pays")
                    .route("", web::get().to(fiches_pays::lister_fiches))
                    .route("/regions", web::get().to(fiches_pays::lister_regions))
                    .route("/{id}", web::get().to(fiches_pays::obtenir_fiche)),
            ),
    );
}

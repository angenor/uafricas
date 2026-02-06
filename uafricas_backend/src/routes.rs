use actix_web::web;

use crate::handlers::{auth, centres_culturels, gouvernance, livres};

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
            ),
    );
}

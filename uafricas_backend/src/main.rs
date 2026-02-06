use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod config;
mod errors;
mod handlers;
mod models;
mod routes;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    message: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        message: "UAfricas Backend is running",
    })
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    log::info!("Demarrage du serveur UAfricas Backend...");

    let app_config = config::AppConfig::from_env();

    // Initialiser le pool de connexions PostgreSQL
    let pool = config::init_db_pool(&app_config.database_url).await;
    log::info!("Connexion a la base de donnees etablie");

    // Creer les repertoires d'upload si necessaire
    std::fs::create_dir_all(format!("{}/couvertures", &app_config.upload_dir))
        .expect("Impossible de creer le repertoire uploads/couvertures");
    std::fs::create_dir_all(format!("{}/documents", &app_config.upload_dir))
        .expect("Impossible de creer le repertoire uploads/documents");
    log::info!("Repertoires d'upload prets: {}", &app_config.upload_dir);

    let upload_dir = app_config.upload_dir.clone();
    let frontend_url = app_config.frontend_url.clone();
    let upload_dir_static = app_config.upload_dir.clone();

    log::info!(
        "Serveur en ecoute sur http://{}:{}",
        app_config.host,
        app_config.port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(upload_dir.clone()))
            .app_data(web::PayloadConfig::new(50 * 1024 * 1024))
            .configure(routes::configurer_routes)
            .service(Files::new("/uploads", &upload_dir_static))
    })
    .bind((app_config.host.as_str(), app_config.port))?
    .run()
    .await
}

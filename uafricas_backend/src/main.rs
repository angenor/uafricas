use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::http::header::{ContentDisposition, DispositionType, DispositionParam};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod config;
pub mod constants;
mod email;
mod errors;
mod handlers;
mod jwt;
mod middleware;
mod models;
mod routes;
pub mod services;

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
        message: "AfricanS Backend is running",
    })
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    })
}

/// Handler qui sert les fichiers uploades avec Content-Disposition: inline
/// Permet l'affichage des PDF dans un iframe au lieu du telechargement
async fn servir_fichier_inline(
    req: HttpRequest,
    upload_dir: web::Data<String>,
) -> actix_web::Result<NamedFile> {
    // match_info decode automatiquement les caracteres percent-encoded
    let chemin_relatif = req.match_info().query("chemin");
    let chemin_complet = format!("{}/{}", upload_dir.get_ref(), chemin_relatif);

    let nom_fichier = chemin_relatif
        .rsplit('/')
        .next()
        .unwrap_or("fichier")
        .to_string();

    let fichier = NamedFile::open(&chemin_complet)?
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![DispositionParam::Filename(nom_fichier)],
        });

    Ok(fichier)
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
    std::fs::create_dir_all(format!("{}/marketplace/annonces", &app_config.upload_dir))
        .expect("Impossible de creer le repertoire uploads/marketplace/annonces");
    log::info!("Repertoires d'upload prets: {}", &app_config.upload_dir);

    let upload_dir = app_config.upload_dir.clone();
    let frontend_url = app_config.frontend_url.clone();
    let jwt_config = app_config.jwt_config();
    let livekit_config = app_config.livekit_config();
    let smtp_config = app_config.smtp_config();

    // Registre des connexions SSE de la messagerie (partagé entre workers, mono-instance)
    let registre_sse = services::messagerie_sse::RegistreSse::new();
    // Registre des appels directs en cours (éphémère, mono-instance)
    let registre_appels = services::appels::RegistreAppels::new();

    log::info!(
        "Serveur en ecoute sur http://{}:{}",
        app_config.host,
        app_config.port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::HeaderName::from_static("x-afrolang-acces-jeton"),
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(upload_dir.clone()))
            .app_data(web::Data::new(jwt_config.clone()))
            .app_data(web::Data::new(livekit_config.clone()))
            .app_data(web::Data::new(smtp_config.clone()))
            .app_data(web::Data::new(registre_sse.clone()))
            .app_data(web::Data::new(registre_appels.clone()))
            .app_data(web::PayloadConfig::new(50 * 1024 * 1024))
            .configure(routes::configurer_routes)
            // Servir les fichiers uploades avec Content-Disposition: inline
            .route("/uploads/{chemin:.*}", web::get().to(servir_fichier_inline))
    })
    .bind((app_config.host.as_str(), app_config.port))?
    .run()
    .await
}

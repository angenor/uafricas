use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    message: &'static str,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        message: "UAfricas Backend is running",
    })
}

async fn index() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    })
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(index)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Starting UAfricas Backend server...");

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    log::info!("Server running at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

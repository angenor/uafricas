use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

/// Configuration de l'application
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub upload_dir: String,
    pub frontend_url: String,
}

impl AppConfig {
    /// Charge la configuration depuis les variables d'environnement
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .expect("PORT doit etre un nombre valide"),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL doit etre definie dans .env"),
            upload_dir: env::var("UPLOAD_DIR")
                .unwrap_or_else(|_| "./uploads".to_string()),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        }
    }
}

/// Initialise le pool de connexions PostgreSQL
pub async fn init_db_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Impossible de se connecter a la base de donnees")
}

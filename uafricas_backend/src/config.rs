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
    pub jwt_secret: String,
    pub jwt_expiration_minutes: i64,
    pub refresh_expiration_days: i64,
    pub livekit_url: String,
    pub livekit_api_key: String,
    pub livekit_api_secret: String,
}

/// Configuration JWT partagee via web::Data
#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_minutes: i64,
    pub refresh_expiration_days: i64,
}

/// Configuration LiveKit partagee via web::Data
#[derive(Clone)]
pub struct LivekitConfig {
    pub url: String,
    pub api_key: String,
    pub api_secret: String,
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
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET doit etre definie dans .env"),
            jwt_expiration_minutes: env::var("JWT_EXPIRATION_MINUTES")
                .unwrap_or_else(|_| "15".to_string())
                .parse::<i64>()
                .expect("JWT_EXPIRATION_MINUTES doit etre un nombre"),
            refresh_expiration_days: env::var("REFRESH_EXPIRATION_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse::<i64>()
                .expect("REFRESH_EXPIRATION_DAYS doit etre un nombre"),
            livekit_url: env::var("LIVEKIT_URL")
                .unwrap_or_else(|_| "ws://localhost:7880".to_string()),
            livekit_api_key: env::var("LIVEKIT_API_KEY")
                .unwrap_or_else(|_| "devkey".to_string()),
            livekit_api_secret: env::var("LIVEKIT_API_SECRET")
                .unwrap_or_else(|_| "secret".to_string()),
        }
    }

    /// Creer la configuration JWT a partir de AppConfig
    pub fn jwt_config(&self) -> JwtConfig {
        JwtConfig {
            secret: self.jwt_secret.clone(),
            expiration_minutes: self.jwt_expiration_minutes,
            refresh_expiration_days: self.refresh_expiration_days,
        }
    }

    /// Creer la configuration LiveKit a partir de AppConfig
    pub fn livekit_config(&self) -> LivekitConfig {
        LivekitConfig {
            url: self.livekit_url.clone(),
            api_key: self.livekit_api_key.clone(),
            api_secret: self.livekit_api_secret.clone(),
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

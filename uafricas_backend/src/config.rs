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
    pub livekit_api_url: String,
    pub livekit_api_key: String,
    pub livekit_api_secret: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_email: String,
    pub smtp_from_name: String,
    pub email_verification_expiration_hours: i64,
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
    /// URL client (WebSocket, publique), renvoyee au frontend pour la connexion SFU.
    pub url: String,
    /// URL de l'API serveur LiveKit (HTTP, souvent interne). En prod derriere un
    /// reverse-proxy, elle DOIT pointer directement sur le conteneur LiveKit
    /// (ex. `http://uafricas_livekit:7880`) car le client Twirp ecrase le chemin
    /// de l'URL (le prefixe `/livekit-ws` du proxy est perdu). Repli sur `url`.
    pub api_url: String,
    pub api_key: String,
    pub api_secret: String,
}

/// Configuration SMTP partagee via web::Data
#[derive(Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
    pub frontend_url: String,
    pub verification_expiration_hours: i64,
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
            // URL de l'API serveur : `LIVEKIT_API_URL` si definie, sinon repli sur
            // `LIVEKIT_URL` (comportement historique, valable en dev ou WS==API).
            livekit_api_url: env::var("LIVEKIT_API_URL")
                .or_else(|_| env::var("LIVEKIT_URL"))
                .unwrap_or_else(|_| "ws://localhost:7880".to_string()),
            livekit_api_key: env::var("LIVEKIT_API_KEY")
                .unwrap_or_else(|_| "devkey".to_string()),
            livekit_api_secret: env::var("LIVEKIT_API_SECRET")
                .unwrap_or_else(|_| "secret".to_string()),
            smtp_host: env::var("SMTP_HOST")
                .expect("SMTP_HOST doit etre definie dans .env"),
            smtp_port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse::<u16>()
                .expect("SMTP_PORT doit etre un nombre"),
            smtp_username: env::var("SMTP_USERNAME")
                .expect("SMTP_USERNAME doit etre definie dans .env"),
            smtp_password: env::var("SMTP_PASSWORD")
                .expect("SMTP_PASSWORD doit etre definie dans .env"),
            smtp_from_email: env::var("SMTP_FROM_EMAIL")
                .unwrap_or_else(|_| env::var("SMTP_USERNAME").unwrap_or_default()),
            smtp_from_name: env::var("SMTP_FROM_NAME")
                .unwrap_or_else(|_| "AfricanS".to_string()),
            email_verification_expiration_hours: env::var("EMAIL_VERIFICATION_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse::<i64>()
                .expect("EMAIL_VERIFICATION_EXPIRATION_HOURS doit etre un nombre"),
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
            api_url: self.livekit_api_url.clone(),
            api_key: self.livekit_api_key.clone(),
            api_secret: self.livekit_api_secret.clone(),
        }
    }

    /// Creer la configuration SMTP a partir de AppConfig
    pub fn smtp_config(&self) -> SmtpConfig {
        SmtpConfig {
            host: self.smtp_host.clone(),
            port: self.smtp_port,
            username: self.smtp_username.clone(),
            password: self.smtp_password.clone(),
            from_email: self.smtp_from_email.clone(),
            from_name: self.smtp_from_name.clone(),
            frontend_url: self.frontend_url.clone(),
            verification_expiration_hours: self.email_verification_expiration_hours,
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

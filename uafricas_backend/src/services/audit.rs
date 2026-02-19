use sqlx::PgPool;
use uuid::Uuid;

/// Enregistre une action dans le journal d'audit (non-bloquant : les erreurs sont loguees sans bloquer l'appelant)
pub async fn log_action(
    pool: &PgPool,
    utilisateur_id: Option<Uuid>,
    action: &str,
    schema_name: &str,
    table_name: &str,
    record_id: Option<Uuid>,
    ancien_etat: Option<serde_json::Value>,
    nouvel_etat: Option<serde_json::Value>,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) {
    let result = sqlx::query(
        "INSERT INTO shared.audit_log
         (utilisateur_id, action, schema_name, table_name, record_id, ancien_etat, nouvel_etat, ip_address, user_agent)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8::INET, $9)"
    )
    .bind(utilisateur_id)
    .bind(action)
    .bind(schema_name)
    .bind(table_name)
    .bind(record_id)
    .bind(ancien_etat)
    .bind(nouvel_etat)
    .bind(ip_address)
    .bind(user_agent)
    .execute(pool)
    .await;

    if let Err(e) = result {
        log::error!("Erreur lors de l'enregistrement audit: {}", e);
    }
}

/// Extrait l'adresse IP depuis les headers de la requete Actix
pub fn extraire_ip(req: &actix_web::HttpRequest) -> Option<String> {
    req.connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string())
}

/// Extrait le User-Agent depuis les headers de la requete
pub fn extraire_user_agent(req: &actix_web::HttpRequest) -> Option<String> {
    req.headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

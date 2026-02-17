use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::config::SmtpConfig;
use crate::errors::ApiErreur;

/// Envoyer un email de verification a l'utilisateur
pub async fn envoyer_email_verification(
    config: &SmtpConfig,
    destinataire_email: &str,
    destinataire_prenom: &str,
    lien_verification: &str,
) -> Result<(), ApiErreur> {
    let corps_html = construire_html_verification(destinataire_prenom, lien_verification);

    let expediteur = format!("{} <{}>", config.from_name, config.from_email)
        .parse()
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Adresse expediteur invalide: {}", e)))?;

    let destinataire = destinataire_email
        .parse()
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Adresse destinataire invalide: {}", e)))?;

    let email = Message::builder()
        .from(expediteur)
        .to(destinataire)
        .subject("Verifiez votre adresse email - UAfricas")
        .header(ContentType::TEXT_HTML)
        .body(corps_html)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur construction email: {}", e)))?;

    let creds = Credentials::new(config.username.clone(), config.password.clone());

    // Port 587 = STARTTLS
    let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur connexion SMTP: {}", e)))?
        .credentials(creds)
        .port(config.port)
        .build();

    transport.send(email).await.map_err(|e| {
        log::error!("Erreur envoi email a {}: {}", destinataire_email, e);
        ApiErreur::BaseDeDonnees(format!("Erreur envoi email: {}", e))
    })?;

    log::info!("Email de verification envoye a {}", destinataire_email);
    Ok(())
}

/// Construire le HTML du mail de verification
fn construire_html_verification(prenom: &str, lien: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"></head>
<body style="font-family: 'Open Sans', Arial, sans-serif; background-color: #f5f5f5; margin: 0; padding: 20px;">
  <div style="max-width: 600px; margin: 0 auto; background: white; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 10px rgba(0,0,0,0.1);">
    <div style="background: linear-gradient(135deg, #1f2937, #111827); padding: 30px; text-align: center;">
      <h1 style="color: white; margin: 0; font-size: 24px; font-family: 'Oswald', sans-serif;">UAfricas</h1>
      <p style="color: #9ca3af; margin: 8px 0 0; font-size: 14px;">Verification de votre adresse email</p>
    </div>
    <div style="padding: 30px;">
      <p style="color: #374151; font-size: 16px;">Bonjour <strong>{prenom}</strong>,</p>
      <p style="color: #4b5563; font-size: 15px; line-height: 1.6;">
        Merci de vous etre inscrit sur <strong>UAfricas</strong> ! Pour activer votre compte et acceder a la plateforme, veuillez cliquer sur le bouton ci-dessous :
      </p>
      <div style="text-align: center; margin: 30px 0;">
        <a href="{lien}" style="display: inline-block; background: linear-gradient(to right, #A54A1C, #228B22); color: white; text-decoration: none; padding: 14px 40px; border-radius: 8px; font-weight: 600; font-size: 16px;">
          Verifier mon email
        </a>
      </div>
      <p style="color: #6b7280; font-size: 13px;">Si le bouton ne fonctionne pas, copiez-collez ce lien dans votre navigateur :</p>
      <p style="color: #228B22; font-size: 12px; word-break: break-all; background: #f0fdf4; padding: 10px; border-radius: 6px;">{lien}</p>
      <hr style="border: none; border-top: 1px solid #e5e7eb; margin: 25px 0;" />
      <p style="color: #9ca3af; font-size: 12px; text-align: center;">
        Ce lien expire dans 24 heures.<br/>
        Si vous n'avez pas cree de compte sur UAfricas, ignorez ce message.
      </p>
    </div>
    <div style="background: #f9fafb; padding: 15px; text-align: center; border-top: 1px solid #e5e7eb;">
      <p style="color: #9ca3af; font-size: 11px; margin: 0;">UAfricas - Plateforme panafricaine pour le developpement durable</p>
    </div>
  </div>
</body>
</html>"#,
        prenom = prenom,
        lien = lien
    )
}

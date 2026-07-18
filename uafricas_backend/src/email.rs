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
        .subject("Verifiez votre adresse email - AfricanS")
        .header(ContentType::TEXT_HTML)
        .body(corps_html)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur construction email: {}", e)))?;

    let creds = Credentials::new(config.username.clone(), config.password.clone());

    // Port 587 = STARTTLS (hostname reel LWS : mail77.lwspanel.com)
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

/// Helper interne : construire le transport SMTP et envoyer un message HTML
async fn envoyer_html(
    config: &SmtpConfig,
    destinataire_email: &str,
    sujet: &str,
    corps_html: String,
) -> Result<(), ApiErreur> {
    let expediteur = format!("{} <{}>", config.from_name, config.from_email)
        .parse()
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Adresse expediteur invalide: {}", e)))?;

    let destinataire = destinataire_email
        .parse()
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Adresse destinataire invalide: {}", e)))?;

    let email = Message::builder()
        .from(expediteur)
        .to(destinataire)
        .subject(sujet)
        .header(ContentType::TEXT_HTML)
        .body(corps_html)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur construction email: {}", e)))?;

    let creds = Credentials::new(config.username.clone(), config.password.clone());

    let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur connexion SMTP: {}", e)))?
        .credentials(creds)
        .port(config.port)
        .build();

    transport.send(email).await.map_err(|e| {
        log::error!("Erreur envoi email a {}: {}", destinataire_email, e);
        ApiErreur::BaseDeDonnees(format!("Erreur envoi email: {}", e))
    })?;

    Ok(())
}

/// Echapper le HTML d'un contenu fourni par l'utilisateur (commentaire admin)
fn echapper_html(texte: &str) -> String {
    texte
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Notifier le candidat de la validation de sa demande d'expertise
pub async fn envoyer_email_expertise_validee(
    config: &SmtpConfig,
    destinataire_email: &str,
    destinataire_prenom: &str,
    lien_fiche: &str,
) -> Result<(), ApiErreur> {
    let corps_html = construire_html_expertise_validee(destinataire_prenom, lien_fiche);
    envoyer_html(
        config,
        destinataire_email,
        "Votre demande d'expertise a ete approuvee - AfricanS",
        corps_html,
    )
    .await?;
    log::info!("Email d'approbation d'expertise envoye a {}", destinataire_email);
    Ok(())
}

/// Notifier le candidat du refus de sa demande d'expertise (avec motif)
pub async fn envoyer_email_expertise_refusee(
    config: &SmtpConfig,
    destinataire_email: &str,
    destinataire_prenom: &str,
    commentaire: &str,
) -> Result<(), ApiErreur> {
    let corps_html = construire_html_expertise_refusee(destinataire_prenom, commentaire);
    envoyer_html(
        config,
        destinataire_email,
        "Votre demande d'expertise - AfricanS",
        corps_html,
    )
    .await?;
    log::info!("Email de refus d'expertise envoye a {}", destinataire_email);
    Ok(())
}

/// HTML du mail d'approbation d'une demande d'expertise
fn construire_html_expertise_validee(prenom: &str, lien_fiche: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"></head>
<body style="font-family: 'Open Sans', Arial, sans-serif; background-color: #f5f5f5; margin: 0; padding: 20px;">
  <div style="max-width: 600px; margin: 0 auto; background: white; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 10px rgba(0,0,0,0.1);">
    <div style="background: linear-gradient(135deg, #228B22, #1a6b1a); padding: 30px; text-align: center;">
      <h1 style="color: white; margin: 0; font-size: 24px; font-family: 'Oswald', sans-serif;">AfricanS</h1>
      <p style="color: #d1fae5; margin: 8px 0 0; font-size: 14px;">Votre demande d'expertise a ete approuvee</p>
    </div>
    <div style="padding: 30px;">
      <p style="color: #374151; font-size: 16px;">Bonjour <strong>{prenom}</strong>,</p>
      <p style="color: #4b5563; font-size: 15px; line-height: 1.6;">
        Felicitations ! Votre demande pour devenir expert sur <strong>AfricanS</strong> a ete <strong>approuvee</strong>.
        Votre profil d'expert est desormais visible publiquement.
      </p>
      <div style="text-align: center; margin: 30px 0;">
        <a href="{lien}" style="display: inline-block; background: linear-gradient(to right, #A54A1C, #228B22); color: white; text-decoration: none; padding: 14px 40px; border-radius: 8px; font-weight: 600; font-size: 16px;">
          Voir ma fiche d'expert
        </a>
      </div>
      <hr style="border: none; border-top: 1px solid #e5e7eb; margin: 25px 0;" />
      <p style="color: #9ca3af; font-size: 12px; text-align: center;">
        Merci de mettre votre expertise au service du continent.
      </p>
    </div>
    <div style="background: #f9fafb; padding: 15px; text-align: center; border-top: 1px solid #e5e7eb;">
      <p style="color: #9ca3af; font-size: 11px; margin: 0;">AfricanS - Plateforme panafricaine pour le developpement durable</p>
    </div>
  </div>
</body>
</html>"#,
        prenom = prenom,
        lien = lien_fiche
    )
}

/// HTML du mail de refus d'une demande d'expertise (inclut le motif)
fn construire_html_expertise_refusee(prenom: &str, commentaire: &str) -> String {
    let motif = echapper_html(commentaire);
    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"></head>
<body style="font-family: 'Open Sans', Arial, sans-serif; background-color: #f5f5f5; margin: 0; padding: 20px;">
  <div style="max-width: 600px; margin: 0 auto; background: white; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 10px rgba(0,0,0,0.1);">
    <div style="background: linear-gradient(135deg, #1f2937, #111827); padding: 30px; text-align: center;">
      <h1 style="color: white; margin: 0; font-size: 24px; font-family: 'Oswald', sans-serif;">AfricanS</h1>
      <p style="color: #9ca3af; margin: 8px 0 0; font-size: 14px;">Suivi de votre demande d'expertise</p>
    </div>
    <div style="padding: 30px;">
      <p style="color: #374151; font-size: 16px;">Bonjour <strong>{prenom}</strong>,</p>
      <p style="color: #4b5563; font-size: 15px; line-height: 1.6;">
        Apres examen, votre demande pour devenir expert sur <strong>AfricanS</strong> n'a pas pu etre approuvee en l'etat.
      </p>
      <div style="background: #fef2f2; border-left: 4px solid #ef4444; padding: 14px 16px; border-radius: 6px; margin: 20px 0;">
        <p style="color: #991b1b; font-size: 13px; font-weight: 600; margin: 0 0 6px;">Motif</p>
        <p style="color: #4b5563; font-size: 14px; margin: 0; line-height: 1.5;">{motif}</p>
      </div>
      <p style="color: #4b5563; font-size: 15px; line-height: 1.6;">
        Vous pouvez corriger votre dossier et soumettre une nouvelle demande depuis votre espace personnel.
      </p>
      <hr style="border: none; border-top: 1px solid #e5e7eb; margin: 25px 0;" />
      <p style="color: #9ca3af; font-size: 12px; text-align: center;">
        Merci de votre interet pour la plateforme AfricanS.
      </p>
    </div>
    <div style="background: #f9fafb; padding: 15px; text-align: center; border-top: 1px solid #e5e7eb;">
      <p style="color: #9ca3af; font-size: 11px; margin: 0;">AfricanS - Plateforme panafricaine pour le developpement durable</p>
    </div>
  </div>
</body>
</html>"#,
        prenom = prenom,
        motif = motif
    )
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
      <h1 style="color: white; margin: 0; font-size: 24px; font-family: 'Oswald', sans-serif;">AfricanS</h1>
      <p style="color: #9ca3af; margin: 8px 0 0; font-size: 14px;">Verification de votre adresse email</p>
    </div>
    <div style="padding: 30px;">
      <p style="color: #374151; font-size: 16px;">Bonjour <strong>{prenom}</strong>,</p>
      <p style="color: #4b5563; font-size: 15px; line-height: 1.6;">
        Merci de vous etre inscrit sur <strong>AfricanS</strong> ! Pour activer votre compte et acceder a la plateforme, veuillez cliquer sur le bouton ci-dessous :
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
        Si vous n'avez pas cree de compte sur AfricanS, ignorez ce message.
      </p>
    </div>
    <div style="background: #f9fafb; padding: 15px; text-align: center; border-top: 1px solid #e5e7eb;">
      <p style="color: #9ca3af; font-size: 11px; margin: 0;">AfricanS - Plateforme panafricaine pour le developpement durable</p>
    </div>
  </div>
</body>
</html>"#,
        prenom = prenom,
        lien = lien
    )
}

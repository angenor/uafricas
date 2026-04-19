//! Rate-limit anti-spam Afripulse (research.md §D5).
//!
//! Bornes appliquées par utilisateur :
//!   • 20 contributions textuelles / 24 h glissantes
//!   • 10 contributions photo / 24 h glissantes
//!   • 5 contributions `en_attente` simultanées sur un même pays
//!
//! Calcul en temps réel via 3 requêtes COUNT paramétrées (indexées par
//! idx_contribution_rate_limit et idx_contribution_attente_pays — cf. 11c).

use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::contribution_fiche::TypeObjetContribution;

pub const LIMITE_TEXTES_PAR_JOUR: i64 = 20;
pub const LIMITE_PHOTOS_PAR_JOUR: i64 = 10;
pub const LIMITE_ATTENTE_PAR_PAYS: i64 = 5;

#[derive(Debug, Clone)]
pub enum LimiteAtteinte {
    TextesJour { compteur: i64, limite: i64, prochain_creneau: DateTime<Utc> },
    PhotosJour { compteur: i64, limite: i64, prochain_creneau: DateTime<Utc> },
    AttenteParPays { compteur: i64, limite: i64 },
}

impl LimiteAtteinte {
    pub fn message(&self) -> String {
        match self {
            Self::TextesJour { limite, .. } => format!(
                "Limite atteinte : {} contributions textuelles / 24 h.",
                limite
            ),
            Self::PhotosJour { limite, .. } => format!(
                "Limite atteinte : {} contributions photo / 24 h.",
                limite
            ),
            Self::AttenteParPays { limite, .. } => format!(
                "Limite atteinte : {} contributions en attente maximum sur ce pays.",
                limite
            ),
        }
    }

    pub fn prochain_creneau(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::TextesJour { prochain_creneau, .. } => Some(*prochain_creneau),
            Self::PhotosJour { prochain_creneau, .. } => Some(*prochain_creneau),
            Self::AttenteParPays { .. } => None,
        }
    }

    pub fn seuil_depasse(&self) -> &'static str {
        match self {
            Self::TextesJour { .. } => "textes_jour",
            Self::PhotosJour { .. } => "photos_jour",
            Self::AttenteParPays { .. } => "attente_par_pays",
        }
    }
}

#[derive(Debug)]
pub enum ErreurVerification {
    LimiteAtteinte(LimiteAtteinte),
    Base(sqlx::Error),
}

impl From<sqlx::Error> for ErreurVerification {
    fn from(e: sqlx::Error) -> Self {
        ErreurVerification::Base(e)
    }
}

/// Vérifie les 3 quotas avant acceptation d'une contribution. Retour Ok
/// signifie que l'appelant peut INSERT.
pub async fn verifier_quotas(
    pool: &PgPool,
    utilisateur_id: Uuid,
    type_objet: &TypeObjetContribution,
    fiche_pays_id: Uuid,
) -> Result<(), ErreurVerification> {
    let est_photo = matches!(type_objet, TypeObjetContribution::PhotoVisiteur);

    if est_photo {
        let compteur_photos: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::bigint FROM country_profile.contribution_fiche
             WHERE cree_par = $1
               AND type_objet_contribution = 'photo_visiteur'::country_profile.type_objet_contribution
               AND created_at > NOW() - INTERVAL '24 hours'
               AND deleted_at IS NULL",
        )
        .bind(utilisateur_id)
        .fetch_one(pool)
        .await?;

        if compteur_photos >= LIMITE_PHOTOS_PAR_JOUR {
            return Err(ErreurVerification::LimiteAtteinte(
                LimiteAtteinte::PhotosJour {
                    compteur: compteur_photos,
                    limite: LIMITE_PHOTOS_PAR_JOUR,
                    prochain_creneau: Utc::now() + Duration::hours(24),
                },
            ));
        }
    } else {
        let compteur_textes: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::bigint FROM country_profile.contribution_fiche
             WHERE cree_par = $1
               AND type_objet_contribution <> 'photo_visiteur'::country_profile.type_objet_contribution
               AND created_at > NOW() - INTERVAL '24 hours'
               AND deleted_at IS NULL",
        )
        .bind(utilisateur_id)
        .fetch_one(pool)
        .await?;

        if compteur_textes >= LIMITE_TEXTES_PAR_JOUR {
            return Err(ErreurVerification::LimiteAtteinte(
                LimiteAtteinte::TextesJour {
                    compteur: compteur_textes,
                    limite: LIMITE_TEXTES_PAR_JOUR,
                    prochain_creneau: Utc::now() + Duration::hours(24),
                },
            ));
        }
    }

    let compteur_attente: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM country_profile.contribution_fiche
         WHERE cree_par = $1
           AND fiche_pays_id = $2
           AND etat = 'en_attente'::country_profile.etat_contribution
           AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .bind(fiche_pays_id)
    .fetch_one(pool)
    .await?;

    if compteur_attente >= LIMITE_ATTENTE_PAR_PAYS {
        return Err(ErreurVerification::LimiteAtteinte(
            LimiteAtteinte::AttenteParPays {
                compteur: compteur_attente,
                limite: LIMITE_ATTENTE_PAR_PAYS,
            },
        ));
    }

    Ok(())
}

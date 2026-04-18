//! Validation des photos de contribution visiteur (galerie Afripulse).
//!
//! Défense en profondeur (research.md §D4) :
//!   • taille ≤ 2 Mo (2 097 152 octets)
//!   • format JPEG ou PNG exclusivement (magic bytes via `image::guess_format`)
//!   • dimensions ≤ 2048 × 2048 px
//!
//! La validation se fait EN MÉMOIRE, AVANT écriture disque — un fichier
//! invalide ne touche jamais `./uploads/`.

use image::GenericImageView;

pub const TAILLE_MAX_OCTETS: usize = 2_097_152; // 2 Mo
pub const DIMENSION_MAX: u32 = 2048;

/// Dimensions et métadonnées validées d'une photo acceptée.
#[derive(Debug, Clone)]
pub struct DimensionsValides {
    pub format: FormatPhoto,
    pub taille_octets: usize,
    pub largeur: u32,
    pub hauteur: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatPhoto {
    Jpeg,
    Png,
}

impl FormatPhoto {
    pub fn as_str(&self) -> &'static str {
        match self {
            FormatPhoto::Jpeg => "jpeg",
            FormatPhoto::Png => "png",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            FormatPhoto::Jpeg => "jpg",
            FormatPhoto::Png => "png",
        }
    }
}

/// Cause de rejet d'une photo soumise.
#[derive(Debug, Clone)]
pub enum ErreurValidationPhoto {
    TailleTropGrande { taille_octets: usize, max_octets: usize },
    FormatNonAutorise { format_detecte: Option<String> },
    DimensionsTropGrandes { largeur: u32, hauteur: u32, max: u32 },
    DecodageImpossible,
}

impl ErreurValidationPhoto {
    pub fn message(&self) -> String {
        match self {
            Self::TailleTropGrande { taille_octets, max_octets } => format!(
                "Photo trop volumineuse : {} octets (max {}).",
                taille_octets, max_octets
            ),
            Self::FormatNonAutorise { format_detecte } => match format_detecte {
                Some(f) => format!("Format photo non autorisé : {} (JPEG ou PNG uniquement).", f),
                None => "Format photo non reconnu (JPEG ou PNG uniquement).".to_string(),
            },
            Self::DimensionsTropGrandes { largeur, hauteur, max } => format!(
                "Dimensions photo trop grandes : {}×{} (max {}×{}).",
                largeur, hauteur, max, max
            ),
            Self::DecodageImpossible => "Photo illisible ou corrompue.".to_string(),
        }
    }
}

/// Valide une photo de contribution : taille, format magic bytes, dimensions.
pub fn valider_photo_contribution(bytes: &[u8]) -> Result<DimensionsValides, ErreurValidationPhoto> {
    if bytes.len() > TAILLE_MAX_OCTETS {
        return Err(ErreurValidationPhoto::TailleTropGrande {
            taille_octets: bytes.len(),
            max_octets: TAILLE_MAX_OCTETS,
        });
    }

    let format_detecte = image::guess_format(bytes)
        .map_err(|_| ErreurValidationPhoto::FormatNonAutorise { format_detecte: None })?;

    let format = match format_detecte {
        image::ImageFormat::Jpeg => FormatPhoto::Jpeg,
        image::ImageFormat::Png => FormatPhoto::Png,
        autre => {
            return Err(ErreurValidationPhoto::FormatNonAutorise {
                format_detecte: Some(format!("{:?}", autre).to_lowercase()),
            });
        }
    };

    let image = image::load_from_memory(bytes)
        .map_err(|_| ErreurValidationPhoto::DecodageImpossible)?;

    let (largeur, hauteur) = image.dimensions();
    if largeur > DIMENSION_MAX || hauteur > DIMENSION_MAX {
        return Err(ErreurValidationPhoto::DimensionsTropGrandes {
            largeur,
            hauteur,
            max: DIMENSION_MAX,
        });
    }

    Ok(DimensionsValides {
        format,
        taille_octets: bytes.len(),
        largeur,
        hauteur,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejette_buffer_trop_grand() {
        let gros = vec![0u8; TAILLE_MAX_OCTETS + 1];
        assert!(matches!(
            valider_photo_contribution(&gros),
            Err(ErreurValidationPhoto::TailleTropGrande { .. })
        ));
    }

    #[test]
    fn rejette_format_inconnu() {
        assert!(matches!(
            valider_photo_contribution(b"pas une image"),
            Err(ErreurValidationPhoto::FormatNonAutorise { .. })
        ));
    }
}

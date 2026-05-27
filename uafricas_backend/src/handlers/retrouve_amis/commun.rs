//! Helpers partagés du module Retrouve Amis (extraction JWT, résumés anonymisés, upload photos, slug).

use actix_web::{web, HttpRequest};
use futures_util::StreamExt;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;

/// Extraire l'ID utilisateur depuis le token JWT dans le header Authorization
pub(super) fn extraire_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiErreur::NonAutorise("Token manquant".into()))?;

    let token = jwt::extraire_token_du_header(header)?;
    let jwt_config = req
        .app_data::<web::Data<crate::config::JwtConfig>>()
        .ok_or_else(|| ApiErreur::BaseDeDonnees("Configuration JWT manquante".into()))?;
    let claims = jwt::valider_token(token, &jwt_config.secret)?;
    claims
        .sub
        .parse::<Uuid>()
        .map_err(|_| ApiErreur::NonAutorise("ID utilisateur invalide".into()))
}


/// Construire les initiales pour le résumé anonymisé
pub(super) fn construire_initiales(nom: Option<&str>, prenom: Option<&str>) -> String {
    let n = nom
        .and_then(|s| s.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());
    let p = prenom
        .and_then(|s| s.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "".to_string());
    if p.is_empty() {
        format!("{}.", n)
    } else {
        format!("{}.{}.", p, n)
    }
}

/// Construire la période pour le résumé anonymisé
pub(super) fn construire_periode(debut: Option<i32>, fin: Option<i32>) -> Option<String> {
    match (debut, fin) {
        (Some(d), Some(f)) => Some(format!("{}-{}", d, f)),
        (Some(d), None) => Some(format!("{}-...", d)),
        (None, Some(f)) => Some(format!("...-{}", f)),
        (None, None) => None,
    }
}

/// Construire la liste des critères communs à partir des détails du score
pub(super) fn construire_criteres_communs(details: &serde_json::Value) -> Vec<String> {
    let mut criteres = Vec::new();
    if let Some(obj) = details.as_object() {
        for (cle, valeur) in obj {
            if let Some(v) = valeur.as_f64() {
                if v > 0.0 {
                    criteres.push(cle.clone());
                }
            }
        }
    }
    criteres
}



/// Lire le contenu texte d'un champ multipart avis
pub(super) async fn lire_champ_texte_avis(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut contenu = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture champ: {}", e)))?;
        contenu.extend_from_slice(&data);
    }
    String::from_utf8(contenu)
        .map_err(|e| ApiErreur::Upload(format!("Encodage UTF-8 invalide: {}", e)))
}

/// Lire un champ texte optionnel (retourne None si vide)
pub(super) async fn lire_champ_option(field: &mut actix_multipart::Field) -> Result<Option<String>, ApiErreur> {
    let val = lire_champ_texte_avis(field).await?;
    if val.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(val))
    }
}

/// Sauvegarder une photo uploadee avec limite de taille
/// Verifie que les premiers octets correspondent a un format image autorise (JPEG, PNG, WebP).
pub(super) fn valider_magic_bytes(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }
    // JPEG : FF D8 FF
    if data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF {
        return true;
    }
    // PNG : 89 50 4E 47
    if data[0] == 0x89 && data[1] == 0x50 && data[2] == 0x4E && data[3] == 0x47 {
        return true;
    }
    // WebP : RIFF....WEBP
    if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
        return true;
    }
    false
}

pub(super) async fn sauvegarder_photo_avis(
    field: &mut actix_multipart::Field,
    chemin: &str,
    taille_max: usize,
) -> Result<(), ApiErreur> {
    if let Some(parent) = std::path::Path::new(chemin).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le repertoire: {}", e)))?;
    }

    let mut fichier = std::fs::File::create(chemin)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le fichier: {}", e)))?;

    let mut taille_totale: usize = 0;
    let mut premier_chunk = true;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
        taille_totale += data.len();
        if taille_totale > taille_max {
            let _ = std::fs::remove_file(chemin);
            return Err(ApiErreur::Validation(
                "La photo depasse la taille maximale de 5 Mo".into(),
            ));
        }
        // Valider les magic bytes sur le premier chunk
        if premier_chunk {
            if !valider_magic_bytes(&data) {
                let _ = std::fs::remove_file(chemin);
                return Err(ApiErreur::Validation(
                    "Le contenu du fichier ne correspond pas a un format image valide (JPEG, PNG, WebP)".into(),
                ));
            }
            premier_chunk = false;
        }
        fichier
            .write_all(&data)
            .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture fichier: {}", e)))?;
    }

    Ok(())
}



/// Generer un slug URL-safe a partir du nom et prenom recherches
pub(super) fn generer_slug_avis(nom: &str, prenom: Option<&str>) -> String {
    let base = if let Some(p) = prenom {
        format!("{}-{}", nom, p)
    } else {
        nom.to_string()
    };

    // Normaliser : minuscules, remplacer espaces et caracteres speciaux par des tirets
    let slug: String = base
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else if c == ' ' || c == '_' {
                '-'
            } else {
                // Caracteres accentues courants
                match c {
                    'é' | 'è' | 'ê' | 'ë' => 'e',
                    'à' | 'â' | 'ä' => 'a',
                    'ù' | 'û' | 'ü' => 'u',
                    'î' | 'ï' => 'i',
                    'ô' | 'ö' => 'o',
                    'ç' => 'c',
                    'ñ' => 'n',
                    _ => '-',
                }
            }
        })
        .collect();

    // Supprimer les tirets multiples et les tirets en debut/fin
    let slug = slug
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    // Ajouter un suffixe UUID8 pour garantir l'unicite
    let uuid_suffix = &Uuid::new_v4().to_string()[..8];
    format!("{}-{}", slug, uuid_suffix)
}

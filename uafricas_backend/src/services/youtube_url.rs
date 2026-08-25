//! Extraction de l'identifiant vidéo YouTube depuis une URL
//! (feature 001-ressources-fermeture-session, research.md §2).
//!
//! Domaines whitelistés : `youtube.com`, `www.youtube.com`, `m.youtube.com`,
//! `youtu.be`. Formats supportés :
//!   - `https://www.youtube.com/watch?v=<ID>` (+ paramètres)
//!   - `https://youtu.be/<ID>`
//!   - `https://www.youtube.com/embed/<ID>`
//!   - `https://www.youtube.com/shorts/<ID>`
//!
//! L'ID YouTube est strictement 11 caractères dans le jeu `[A-Za-z0-9_-]`.
//! Implémenté en parsing manuel (pas de dépendance `regex`).

const HOTES_YOUTUBE: &[&str] = &[
    "youtube.com",
    "www.youtube.com",
    "m.youtube.com",
    "youtu.be",
];

const LONGUEUR_ID: usize = 11;

fn est_caractere_id_valide(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

fn valider_id(candidat: &str) -> Option<String> {
    if candidat.len() == LONGUEUR_ID && candidat.chars().all(est_caractere_id_valide) {
        Some(candidat.to_string())
    } else {
        None
    }
}

/// Extrait l'identifiant vidéo YouTube depuis `url`. Retourne `None` si
/// l'URL n'est pas une URL YouTube reconnue ou si l'ID est invalide.
pub fn extraire_id_youtube(url: &str) -> Option<String> {
    let url = url.trim();
    let sans_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;

    let (hote, reste) = match sans_scheme.find('/') {
        Some(pos) => sans_scheme.split_at(pos),
        None => (sans_scheme, ""),
    };
    let reste = reste.trim_start_matches('/');

    let (hote_pur, _) = hote.split_once(':').unwrap_or((hote, ""));
    if !HOTES_YOUTUBE.iter().any(|h| h.eq_ignore_ascii_case(hote_pur)) {
        return None;
    }

    if hote_pur.eq_ignore_ascii_case("youtu.be") {
        // youtu.be/<ID>[?params][#frag]
        let segment = reste
            .split(|c| c == '?' || c == '#' || c == '/')
            .next()
            .unwrap_or("");
        return valider_id(segment);
    }

    // youtube.com : distinguer /watch, /embed/<ID>, /shorts/<ID>
    let (chemin, query) = reste.split_once('?').unwrap_or((reste, ""));
    let mut segments = chemin.split('/').filter(|s| !s.is_empty());

    match segments.next() {
        Some("watch") => {
            // Cherche le paramètre v=<ID>
            for paire in query.split('&') {
                if let Some(v) = paire.strip_prefix("v=") {
                    let id = v.split('&').next().unwrap_or("");
                    return valider_id(id);
                }
            }
            None
        }
        Some("embed") | Some("shorts") => {
            let id = segments.next().unwrap_or("");
            let id = id.split(|c| c == '?' || c == '#').next().unwrap_or("");
            valider_id(id)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrait_id_watch() {
        assert_eq!(
            extraire_id_youtube("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn extrait_id_short_link() {
        assert_eq!(
            extraire_id_youtube("https://youtu.be/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn extrait_id_embed() {
        assert_eq!(
            extraire_id_youtube("https://www.youtube.com/embed/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn extrait_id_shorts() {
        assert_eq!(
            extraire_id_youtube("https://www.youtube.com/shorts/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn extrait_id_avec_query_supplementaire() {
        assert_eq!(
            extraire_id_youtube("https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=42s"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn rejette_hote_non_youtube() {
        assert!(extraire_id_youtube("https://vimeo.com/dQw4w9WgXcQ").is_none());
        assert!(extraire_id_youtube("https://evil.com/watch?v=dQw4w9WgXcQ").is_none());
    }

    #[test]
    fn rejette_id_mal_forme() {
        assert!(extraire_id_youtube("https://youtu.be/trop_court").is_none());
        assert!(extraire_id_youtube("https://youtu.be/dQw4w9WgXcQXXX").is_none());
        assert!(extraire_id_youtube("https://www.youtube.com/watch?v=").is_none());
    }
}

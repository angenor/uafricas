//! Coordonnées publiques des supports médias (chaînes TV, stations radio).
//!
//! Trois chemins d'écriture les alimentent, création admin, modification
//! admin, validation d'une proposition de membre, et tous doivent normaliser
//! de la même façon, sans quoi la même chaîne s'afficherait différemment selon
//! qui l'a saisie.

use serde::{Deserialize, Serialize};

/// Bloc de coordonnées tel qu'il voyage vers le client.
///
/// Regroupé plutôt qu'aplati sur chaque DTO de support : le front n'a qu'un
/// objet à tester pour décider d'afficher, ou non, le bloc « Contacts », et
/// les deux supports parlent le même langage. Défini ici, aux côtés des
/// helpers de normalisation, plutôt qu'en double dans `models::television` et
/// `models::station_radio`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContactsSupport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_web: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adresse: Option<String>,
}

impl ContactsSupport {
    /// `None` quand aucune coordonnée n'est renseignée : le client distingue
    /// ainsi « pas de contacts » d'un objet vide, sans inspecter cinq champs.
    pub fn depuis(
        email: Option<&str>,
        telephone: Option<&str>,
        whatsapp: Option<&str>,
        site_web: Option<&str>,
        adresse: Option<&str>,
    ) -> Option<Self> {
        let contacts = ContactsSupport {
            email: texte_non_vide(email),
            telephone: texte_non_vide(telephone),
            whatsapp: texte_non_vide(whatsapp),
            site_web: texte_non_vide(site_web),
            adresse: texte_non_vide(adresse),
        };
        if contacts.est_vide() {
            None
        } else {
            Some(contacts)
        }
    }

    pub fn est_vide(&self) -> bool {
        self.email.is_none()
            && self.telephone.is_none()
            && self.whatsapp.is_none()
            && self.site_web.is_none()
            && self.adresse.is_none()
    }
}

/// Trim, puis `None` si le résultat est vide.
///
/// Sans ce filtrage, un champ laissé vide par le formulaire arriverait en base
/// sous la forme d'une chaîne vide : indiscernable d'un contact renseigné pour
/// `Option::is_some`, et donc affiché comme un lien vide côté public.
pub fn texte_non_vide(valeur: Option<&str>) -> Option<String> {
    valeur
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Idem, en garantissant qu'une URL de site web porte son schéma.
///
/// `<a href="www.chaine.tv">` est un lien RELATIF : le navigateur y verrait
/// `/medias/chaines/www.chaine.tv` et servirait une page introuvable. Or
/// « www.… » est exactement la forme qu'un contributeur saisit spontanément.
/// On préfixe donc `https://` à l'écriture, une fois pour toutes, plutôt que de
/// rattraper le coup à chaque affichage.
pub fn normaliser_url(valeur: Option<&str>) -> Option<String> {
    texte_non_vide(valeur).map(|url| {
        let minuscule = url.to_lowercase();
        if minuscule.starts_with("http://") || minuscule.starts_with("https://") {
            url
        } else {
            format!("https://{}", url)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vide_ou_espaces_donne_none() {
        assert_eq!(texte_non_vide(Some("   ")), None);
        assert_eq!(texte_non_vide(None), None);
        assert_eq!(texte_non_vide(Some(" a@b.c ")), Some("a@b.c".to_string()));
    }

    #[test]
    fn url_sans_schema_est_prefixee() {
        assert_eq!(
            normaliser_url(Some("www.chaine.tv")),
            Some("https://www.chaine.tv".to_string())
        );
        assert_eq!(
            normaliser_url(Some("HTTP://chaine.tv")),
            Some("HTTP://chaine.tv".to_string())
        );
        assert_eq!(
            normaliser_url(Some("https://chaine.tv")),
            Some("https://chaine.tv".to_string())
        );
    }
}

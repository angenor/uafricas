//! Périmètre Afripulse : liste figée des codes ISO2 des pays africains autorisés
//! à posséder une fiche pays sur `/opportunite-afrique`.
//!
//! Source unique partagée avec le frontend
//! (`uafricas_frontend/app/constants/afripulsePaysAutorises.ts`).
//! Toute modification doit être propagée dans les deux fichiers (cf. T077).
//!
//! Triés alphabétiquement, minuscules. Inclut `eh` (Sahara occidental).

pub const PAYS_AFRICAINS_ISO2: &[&str] = &[
    "ao", "bf", "bi", "bj", "bw", "cd", "cf", "cg", "ci", "cm",
    "cv", "dj", "dz", "eg", "eh", "er", "et", "ga", "gh", "gm",
    "gn", "gq", "gw", "ke", "km", "lr", "ls", "ly", "ma", "mg",
    "ml", "mr", "mu", "mw", "mz", "na", "ne", "ng", "rw", "sc",
    "sd", "sl", "sn", "so", "ss", "st", "sz", "td", "tg", "tn",
    "tz", "ug", "za", "zm", "zw",
];

/// Indique si un code ISO2 (insensible à la casse) appartient au périmètre
/// Afripulse. Retourne `false` pour toute chaîne vide, malformée ou hors périmètre.
pub fn est_pays_africain(code: &str) -> bool {
    if code.is_empty() {
        return false;
    }
    let normalise = code.trim().to_ascii_lowercase();
    PAYS_AFRICAINS_ISO2.iter().any(|c| *c == normalise)
}

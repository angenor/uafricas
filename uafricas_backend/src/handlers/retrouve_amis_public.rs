use crate::models::retrouve_amis::*;

// ══════════════════════════════════════════════════════════════
// Handlers publics — Avis de recherche (sans authentification)
// ══════════════════════════════════════════════════════════════
//
// Ces handlers sont enregistres HORS du scope JWT dans routes.rs.
// Ils ne necessitent pas de token d'authentification.
//
// Endpoints :
// - GET  /api/retrouve-amis/public/{slug}          → detail_avis_public
// - GET  /api/retrouve-amis/public/rechercher       → rechercher_avis_publics
// - POST /api/retrouve-amis/public/{slug}/partage   → incrementer_partage

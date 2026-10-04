pub mod afrolang_rate_limit;
pub mod appels;
pub mod audit;
pub mod contacts_media;
pub mod engagement;
pub mod livekit_moderation;
pub mod image_validation;
/// Moteur des activités ludiques (feature 013) : épreuve servable, série, partie, score.
pub mod jeu;
/// Dérivation d'épreuves candidates depuis le contenu publié (feature 013).
pub mod jeu_derivation;
pub mod matching;
pub mod messagerie_sse;
/// Prestataire de paiement : **unique point de bascule vers CinetPay** (SC-012).
/// Le basculement vers l'encaissement réel ne doit toucher que ce fichier.
pub mod paiement;
pub mod rate_limit_afripulse;
pub mod rate_limit_ressources;
pub mod youtube_url;

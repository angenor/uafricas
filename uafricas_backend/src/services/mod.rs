pub mod afrolang_rate_limit;
pub mod appels;
pub mod audit;
pub mod contacts_media;
pub mod engagement;
pub mod livekit_moderation;
pub mod image_validation;
pub mod matching;
pub mod messagerie_sse;
/// Prestataire de paiement — **unique point de bascule vers CinetPay** (SC-012).
/// Le basculement vers l'encaissement réel ne doit toucher que ce fichier.
pub mod paiement;
pub mod rate_limit_afripulse;
pub mod rate_limit_ressources;
pub mod youtube_url;

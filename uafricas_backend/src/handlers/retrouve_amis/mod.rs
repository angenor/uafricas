// ════════════════════════════════════════════════════════════════════════════
// Handlers publics : Retrouve Amis
// ════════════════════════════════════════════════════════════════════════════
//
// Endpoints publics pour la fonctionnalité "Retrouve Amis", découpés par domaine :
// - `pays`            : liste publique des pays
// - `avis`            : CRUD des avis de recherche
// - `correspondances` : correspondances (listing, détail, accepter, refuser) + signalement
// - `notifications`   : notifications utilisateur
// - `trouvable`       : tableau de bord, profil trouvable, parcours
// - `avis_public`     : publication, signalement public, retrait, réponse publique
// - `commun`          : helpers partagés (JWT, résumés anonymisés, upload, slug)
//
// Les handlers sont re-exportés à plat (`pub use`) pour préserver les chemins
// `retrouve_amis::<handler>` utilisés dans `routes.rs`.
// ════════════════════════════════════════════════════════════════════════════

mod commun;

pub mod avis;
pub mod avis_public;
pub mod correspondances;
pub mod notifications;
pub mod pays;
pub mod trouvable;

pub use avis::*;
pub use avis_public::*;
pub use correspondances::*;
pub use notifications::*;
pub use pays::*;
pub use trouvable::*;

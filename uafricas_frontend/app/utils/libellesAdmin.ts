// Libellés humains pour les codes techniques du back-office (audit #21, #22, #31).
// Auto-importé par Nuxt (app/utils/). Fallback `prettifier` : tout code inconnu
// devient lisible (underscores → espaces, capitalisation) au lieu d'un code brut.

export function prettifierCode(code: string | null | undefined): string {
  if (!code) return ''
  const s = code.replace(/_/g, ' ').trim().toLowerCase()
  return s.charAt(0).toUpperCase() + s.slice(1)
}

// ── Actions du journal d'audit / flux d'activité (#21, #31) ──────────────────
const LIBELLES_ACTION: Record<string, string> = {
  CREATE: 'Création',
  UPDATE: 'Modification',
  DELETE: 'Suppression',
  LOGIN: 'Connexion',
  LOGOUT: 'Déconnexion',
  SIGNALEMENT: 'Signalement',
  SIGNALEMENT_SUSPENSION: 'Suspension (seuil de signalements)',
  REACTIVATION: 'Réactivation',
  AJUSTEMENT: 'Ajustement de points',
  MISE_EN_AVANT: 'Mise en avant',
  RETRAIT_MISE_EN_AVANT: 'Retrait de mise en avant',
  OUVRIR: 'Ouverture',
  CLOTURER: 'Clôture',
  PROMOUVOIR: 'Promotion',
  RETROGRADER: 'Rétrogradation',
  RETIRER: 'Retrait',
  rejoindre_session_salle_publique: 'A rejoint une salle publique',
  demarrer_session_salle_publique: 'A démarré une salle publique',
  creer_personne: "Création d'une personne",
  creer_lien: "Création d'un lien",
  creer_pub: 'Publication',
}

export function libelleAction(code: string | null | undefined): string {
  if (!code) return ''
  return LIBELLES_ACTION[code] ?? prettifierCode(code)
}

// ── Ressources (types) affichées dans l'écran des permissions (#22) ──────────
const LIBELLES_RESSOURCE: Record<string, string> = {
  '*': 'Toutes les ressources',
  all: 'Toutes les ressources',
  afrolang: 'AfroLang (salles)',
  annonce: 'Annonces (Marché africain)',
  bad_habit: 'Mauvaises pratiques',
  centre_culturel: 'Centres culturels',
  engagement: 'Engagement (points)',
  evenement: 'Événements',
  expertise: 'Expertises',
  fiche_pays: 'Profils pays',
  profil_pays: 'Profils pays',
  factcheck: 'FactCheck',
  idea_force: 'Idées forces',
  innovation: 'Innovations',
  livre: 'Bibliothèque',
  mooc: 'Formations',
  programme: "Programmes d'échange",
  projet: 'Projets',
  radio_tele: 'Radio & Télévision',
  media_content: 'Médias & Contenus',
  utilisateur: 'Utilisateurs',
}

export function libelleRessource(code: string | null | undefined): string {
  if (!code) return ''
  return LIBELLES_RESSOURCE[code] ?? prettifierCode(code)
}

// ── Verbes d'action des permissions (#22/#24 affichage) ──────────────────────
const LIBELLES_PERMISSION_ACTION: Record<string, string> = {
  '*': 'Tout',
  all: 'Tout',
  creer: 'Créer',
  creer_pub: 'Créer (public)',
  creer_publique: 'Créer (public)',
  modifier: 'Modifier',
  supprimer: 'Supprimer',
  voir: 'Consulter',
  valider: 'Valider',
  moderer: 'Modérer',
  gerer: 'Gérer',
  approuver: 'Approuver',
  suspendre: 'Suspendre',
  bloquer: 'Bloquer',
}

export function libellePermissionAction(code: string | null | undefined): string {
  if (!code) return ''
  return LIBELLES_PERMISSION_ACTION[code] ?? prettifierCode(code)
}

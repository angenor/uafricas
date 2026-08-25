/**
 * Navigation latérale de la refonte (maquette « Africans, Design »).
 *
 * La maquette introduit une arborescence à DEUX niveaux que le routage actuel
 * ne porte pas : des univers (Africarise, Opafrica, …) contenant des modules
 * (Codimoi, Afrolang, Afripulse, …). Les fils d'Ariane de la maquette en
 * donnent quatre de façon certaine :
 *
 *   Africarise / Codimoi · Africarise / Afrolang
 *   Africarise / Afripulse · Africarise / Afroculture · Africamood / Vidafrica
 *
 * Les autres rattachements ne sont écrits nulle part. Les entrées marquées
 * `aValider: true` portent donc une cible déduite, pas confirmée : c'est le
 * seul fichier à corriger quand l'arbitrage sera rendu.
 */

export interface EntreeNav {
  /** Libellé exact de la maquette. */
  libelle: string
  /** Icône FontAwesome (déjà enregistrée dans le plugin). */
  icone: string
  /** Route cible. `null` = aucune route existante identifiée. */
  vers: string | null
  /** true = rattachement déduit, à faire confirmer. */
  aValider?: boolean
}

export const NAV_AFRICANS: EntreeNav[] = [
  { libelle: "Fil d'actualité", icone: 'fa-solid fa-home', vers: '/publications' },
  { libelle: 'Africarise', icone: 'fa-solid fa-book-open', vers: '/codi-moi', aValider: true },
  { libelle: 'Opafrica', icone: 'fa-solid fa-id-card', vers: '/retrouve-amis', aValider: true },
  { libelle: 'Novagouv', icone: 'fa-solid fa-circle', vers: null, aValider: true },
  // Renommé Muniversa par le commit 1daf61f ; la maquette dit encore « Mindshiftlab ».
  { libelle: 'Muniversa', icone: 'fa-solid fa-shield-halved', vers: '/universite', aValider: true },
  { libelle: 'Africantives', icone: 'fa-solid fa-arrows-rotate', vers: '/africantives' },
  { libelle: 'Africamood', icone: 'fa-solid fa-photo-film', vers: '/vidafrica', aValider: true },
  { libelle: 'Explorez', icone: 'fa-solid fa-paper-plane', vers: null, aValider: true },
  { libelle: 'Messages', icone: 'fa-solid fa-envelope', vers: '/mon-compte', aValider: true },
  { libelle: 'Communauté', icone: 'fa-solid fa-user', vers: '/profil', aValider: true },
  { libelle: 'Application African', icone: 'fa-solid fa-star', vers: null, aValider: true },
]

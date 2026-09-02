/**
 * Les pages de l'espace « Mon compte » : la SOURCE UNIQUE.
 *
 * Elles étaient listées à deux endroits qui ne se connaissaient pas, le menu
 * déroulant de la barre supérieure et rien d'autre : chaque page de compte se
 * trouvait par la barre d'adresse ou par un lien perdu dans une autre page.
 * Le rail des huit pages en a besoin à son tour, et deux copies auraient
 * divergé à la première page ajoutée sans que rien ne le signale.
 *
 * `dansLeMenu` distingue les deux emplois : la barre supérieure offre un
 * raccourci vers l'essentiel, le rail donne l'espace entier. Un menu déroulant
 * de huit entrées surplombant chaque page ne serait plus un raccourci.
 */
export interface EntreeCompte {
  libelle: string
  vers: string
  icone: string
  /** Retenue par le menu déroulant de la barre supérieure. */
  dansLeMenu?: boolean
}

export const NAV_COMPTE: EntreeCompte[] = [
  { libelle: 'Mon profil', vers: '/mon-compte/profil', icone: 'fa-solid fa-user', dansLeMenu: true },
  { libelle: 'Mon engagement', vers: '/mon-compte/engagement', icone: 'fa-solid fa-medal', dansLeMenu: true },
  { libelle: 'Mes ami(e)s', vers: '/mon-compte/amis', icone: 'fa-solid fa-user-check', dansLeMenu: true },
  { libelle: 'Mes contributions', vers: '/mon-compte/contributions', icone: 'fa-solid fa-clipboard-list', dansLeMenu: true },
  { libelle: 'Mes supports médias', vers: '/mon-compte/mes-supports', icone: 'fa-solid fa-tv' },
  { libelle: 'Invitations médias', vers: '/mon-compte/invitations-medias', icone: 'fa-solid fa-envelope' },
  { libelle: 'Propositions médias', vers: '/mon-compte/propositions-medias', icone: 'fa-solid fa-paper-plane' },
  { libelle: 'Recommandations', vers: '/mon-compte/recommandations-accompagnateur', icone: 'fa-solid fa-handshake' },
]

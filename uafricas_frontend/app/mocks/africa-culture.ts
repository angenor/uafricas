// Données mock pour la page AfricaCulture

export interface CultureCard {
  id: number
  title: string
  description: string
  image: string
  link: string
  borderColor: 'green' | 'chocolat'
}

export interface AfricaCultureHero {
  title: string
  backgroundClass: string
}

export interface AfricaCulturePageData {
  hero: AfricaCultureHero
  cards: CultureCard[]
}

export const africaCulturePageData: AfricaCulturePageData = {
  hero: {
    title: 'AfricaCulture',
    backgroundClass: 'bg-font-centre-culturel',
  },
  cards: [
    {
      id: 1,
      title: 'Promotion des Valeurs Africaines',
      description: 'Valorisation du patrimoine culturel africain et afro-descendant',
      image: '/images/centre-culturel.jpg',
      link: '/promotion-valeur',
      borderColor: 'green',
    },
    {
      id: 2,
      title: 'Expertise de la Diaspora',
      description: 'Mise en valeur des compétences et talents de la diaspora africaine',
      image: '/images/diaspora1.jpg',
      link: '/experts',
      borderColor: 'chocolat',
    },
    {
      id: 3,
      title: 'Opportunités en Afrique',
      description: "Fiches d'informations détaillées sur les opportunités africaines",
      image: '/images/fiche-opportunite.jpg',
      link: '/opportunite-afrique',
      borderColor: 'green',
    },
    {
      id: 4,
      title: 'Échanges Sabbatiques',
      description: "Programme d'échanges d'expériences et de formation continue",
      image: '/images/sabbatique.jpg',
      link: '/echanges-sabbatiques',
      borderColor: 'chocolat',
    },
  ],
}

// Helper pour obtenir la classe de bordure Tailwind
export const getBorderColorClass = (color: 'green' | 'chocolat'): string => {
  return color === 'green' ? 'border-custom-green' : 'border-custom-chocolat'
}

// Helper pour obtenir la classe de hover text
export const getHoverTextClass = (color: 'green' | 'chocolat'): string => {
  return color === 'green' ? 'group-hover:text-custom-green' : 'group-hover:text-custom-chocolat'
}

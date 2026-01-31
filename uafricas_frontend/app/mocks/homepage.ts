// Données mock pour la page d'accueil

export interface BonnePratique {
  id: number
  order: number
  banniere: string
  durree: number
  temp: number
  lien: string
  titre: string
}

export const bonnePratiques: BonnePratique[] = [
  {
    id: 1,
    order: 0,
    banniere: '/images/photos/negociatrices.png',
    durree: 20,
    temp: 0,
    lien: 'https://www.youtube.com/watch?v=Ti0xERkKsHw',
    titre: 'Les femmes au premier plan dans la lutte contre le changement climatique',
  },
  {
    id: 2,
    order: 0,
    banniere: '/images/photos/nexus.png',
    durree: 18,
    temp: 38,
    lien: 'https://www.youtube.com/watch?v=96ceaA-9rR4',
    titre: 'NEXUS CREDEL',
  },
  {
    id: 3,
    order: 0,
    banniere: '/images/photos/halieutiques.png',
    durree: 15,
    temp: 53,
    lien: 'https://www.youtube.com/watch?v=GFcs_yXwiLw',
    titre: 'Des transformatrices de produits halieutiques face au changement climatique, à Cayar, Sénégal',
  },
  {
    id: 4,
    order: 0,
    banniere: '/images/photos/WOMAN_MOBILITY.jpg',
    durree: 11,
    temp: 65,
    lien: 'https://www.youtube.com/watch?v=vG1y0VMqMt4',
    titre: "WOMAN MOBILITY - Une initiative de l'ONG MyDREAM FOR AFRICA",
  },
]

export interface Partenaire {
  id: number
  nom: string
  logo: string
}

export const partenaires: Partenaire[] = [
  {
    id: 1,
    nom: 'Google',
    logo: 'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQUCT6hUiHrgX3azTtY5ib7YKpqj7IdRVQ27g&s',
  },
  {
    id: 2,
    nom: 'Tesla',
    logo: 'https://upload.wikimedia.org/wikipedia/commons/thumb/e/e8/Tesla_logo.png/1200px-Tesla_logo.png',
  },
  {
    id: 3,
    nom: 'Amazon',
    logo: 'https://upload.wikimedia.org/wikipedia/commons/a/a9/Amazon_logo.svg',
  },
]

export interface Responsable {
  nom: string
  role: string
  photo: string
}

export const responsable: Responsable = {
  nom: 'Koné Mamadou',
  role: 'Responsable de la direction de UAfricaS',
  photo: '/images/profil1.jpg',
}

// Navigation mock (pour NavBar sans authentification)
export interface NavItem {
  label: string
  to: string
  children?: NavItem[]
}

export const navigationItems: NavItem[] = [
  { label: 'Actions', to: '/actions' },
  { label: 'AfricaCulture', to: '/africa-culture' },
  {
    label: "Lib. d'Afrique",
    to: '/bibliotheques',
    children: [
      { label: 'Biblio Numérique', to: '/bibliotheque/numerique' },
      { label: 'Biblio Humaine', to: '/bibliotheque/humaine' },
    ],
  },
  {
    label: 'Africa Univers',
    to: '/universite',
    children: [
      { label: 'Gouvernance', to: '/gouvernance' },
      { label: 'INUDA', to: '/INUDA' },
    ],
  },
  {
    label: 'Africamood',
    to: '/medias',
    children: [
      { label: 'Télé', to: '/tele' },
      { label: 'Radio', to: '/radios' },
      { label: 'Africantives', to: '/africantives' },
    ],
  },
]

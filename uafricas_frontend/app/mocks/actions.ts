// Données mock pour la page Actions

export interface ActionCard {
  id: number
  title: string
  description: string
  image: string
  icon: string
  link: string
}

export const actionCards: ActionCard[] = [
  {
    id: 1,
    title: 'Culture et développement',
    description:
      'Valoriser et promouvoir la richesse culturelle africaine comme moteur de développement.',
    image: '/images/culturel_danse.jpg',
    icon: 'fa-solid fa-masks-theater',
    link: '/africa-culture',
  },
  {
    id: 2,
    title: 'Environnement et climat',
    description:
      "Mettre en œuvre des solutions durables pour la préservation de l'environnement africain.",
    image: '/images/diaspora1.jpg',
    icon: 'fa-solid fa-leaf',
    link: '#',
  },
  {
    id: 3,
    title: 'Éducation et formation',
    description:
      "Renforcer les capacités par l'accès à une éducation de qualité pour tous.",
    image: 'https://citinewsroom.com/wp-content/uploads/2021/01/KNUST.jpg',
    icon: 'fa-solid fa-graduation-cap',
    link: '/universite',
  },
  {
    id: 4,
    title: 'Intégration et marché africain',
    description:
      "Favoriser la coopération économique et l'unité des marchés à travers le continent.",
    image:
      'https://www.barlamane.com/fr/wp-content/uploads/2019/06/Zone-africaine-de-libre-%C3%A9change-ZLECAF-Le-Maroc-ratifie%E2%80%A6.jpg',
    icon: 'fa-solid fa-handshake',
    link: '/promotion-valeur',
  },
]

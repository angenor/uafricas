// Donnees mock pour la page Universite

export interface UniversiteCategory {
  id: number
  title: string
  description: string
  image: string
  link: string
  tags: string[]
  badge: string
  badgeColor: 'green' | 'chocolat'
}

export interface UniversiteHero {
  title: string
  subtitle: string
  backgroundImage: string
}

export interface UniversitePageData {
  hero: UniversiteHero
  categories: UniversiteCategory[]
}

export const universitePageData: UniversitePageData = {
  hero: {
    title: 'Université',
    subtitle: 'Explorez nos programmes de formation et initiatives de gouvernance citoyenne',
    backgroundImage: 'https://images.unsplash.com/photo-1507842217343-583bb7270b66?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80',
  },
  categories: [
    {
      id: 1,
      title: 'Gouvernance',
      description: 'Participez activement à la gouvernance citoyenne : vérification des faits, signalement des mauvaises pratiques et partage d\'idées constructives.',
      image: 'https://www.psychologue.net/pro/psicologos_fr/articles/cf/cd/0/38379/shutterstock-692127361.jpg',
      link: '/universite/gouvernance',
      tags: ['Factcheck', 'Badhabits', 'Ideaforces', 'Quiz'],
      badge: 'Gouvernance',
      badgeColor: 'green',
    },
    {
      id: 2,
      title: 'INUDA',
      description: 'Institut universitaire pour le développement de l\'Afrique - Formation, ateliers et programmes académiques.',
      image: '/images/mr_nkrumah.png',
      link: '/universite/inuda',
      tags: ['Concertations-Ateliers', 'Clom-Moc'],
      badge: 'Formation',
      badgeColor: 'chocolat',
    },
  ],
}

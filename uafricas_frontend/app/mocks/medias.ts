// Données mock pour la page Médias

export interface MediaCard {
  id: number
  title: string
  description: string
  image: string
  badge: string
  badgeColor: 'green' | 'chocolat'
  link: string
}

export interface MediaStat {
  value: string
  label: string
}

export const mediaCards: MediaCard[] = [
  {
    id: 1,
    title: 'Télévision Africaine',
    description:
      'Découvrez les chaînes de télévision africaines en direct. Actualités, divertissement et culture à portée de clic.',
    image: '/images/banners/regarde-tele.jpg',
    badge: 'Télé',
    badgeColor: 'green',
    link: '/medias/tele',
  },
  {
    id: 2,
    title: 'Radios Africaines',
    description:
      'Écoutez les meilleures radios africaines en streaming. Musique, informations et programmes variés 24h/24.',
    image: '/images/banners/radio-national.jpg',
    badge: 'Radio',
    badgeColor: 'chocolat',
    link: '/medias/radios',
  },
]

export const mediaStats: MediaStat[] = [
  { value: '150+', label: 'Médias Disponibles' },
  { value: '54', label: 'Territoires Couverts' },
  { value: '24/7', label: 'En Continue' },
  { value: 'HD+', label: 'Qualité Audio & Vidéo' },
]

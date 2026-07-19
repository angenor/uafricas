// Contenu éditorial de la page hub `/medias/radios`.
//
// Les données de stations et d'émissions vivent désormais dans les composables
// (`useStationsRadio`, `useTelevision`), qui font foi : les interfaces
// `RadioStation` / `TvChannel` / `TvProgram` qui doublonnaient ici ont été
// retirées, ainsi que les jeux de démonstration qu'elles typaient, tous sans
// consommateur.

export interface RadioCategory {
  id: number
  title: string
  description: string
  image: string
  badge: string
  badgeColor: 'green' | 'chocolat'
  link: string
}

export interface RadioStats {
  value: string
  label: string
}

// Catégories de radios pour la page hub
export const radioCategories: RadioCategory[] = [
  {
    id: 1,
    title: 'Radios Africans',
    description: 'Découvrez les meilleures stations de radio africaines internationales diffusant de la musique et des programmes culturels.',
    image: '/images/banners/radio-africans.avif',
    badge: 'International',
    badgeColor: 'green',
    link: '/medias/radio/africans'
  },
  {
    id: 2,
    title: 'Radios Nationales',
    description: 'Écoutez les radios nationales de chaque territoire africain avec des programmes locaux et des informations régionales.',
    image: '/images/banners/radio-national.jpg',
    badge: 'Local',
    badgeColor: 'chocolat',
    link: '/medias/radio/nationales'
  }
]

// Statistiques pour la page hub radios
export const radioStats: RadioStats[] = [
  { value: '150+', label: 'Stations Radio' },
  { value: '54', label: 'Territoires Africains' },
  { value: '24/7', label: 'Diffusion Continue' },
  { value: 'HD', label: 'Qualité Audio' }
]

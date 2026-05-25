// Types et interfaces pour la télévision
export interface TvChannel {
  id: string
  name: string
  description: string
  streamUrl: string
  cover: string
  category: string
  country: string
  language: string
  isLive: boolean
}

export interface TvProgram {
  id: number
  order: number
  banniere: string
  duree: number
  temp: number
  lien: string
  titre: string
}

export interface TvStats {
  value: string
  label: string
}

// Statistiques pour la page télévision
export const teleStats: TvStats[] = [
  { value: '50+', label: 'Chaînes TV' },
  { value: '54', label: 'Territoires Africains' },
  { value: '24/7', label: 'Diffusion Continue' },
  { value: 'HD+', label: 'Qualité Vidéo' }
]

// Programmes de démonstration (timeline vidéo)
export const telePrograms: TvProgram[] = [
  {
    id: 1,
    order: 1,
    banniere: 'https://cdn.pixabay.com/photo/2020/04/21/13/32/africa-5072833_1280.jpg',
    duree: 20,
    temp: 20,
    lien: 'https://www.youtube.com/watch?v=example1',
    titre: 'Documentaire: Les femmes africaines au premier plan'
  },
  {
    id: 2,
    order: 2,
    banniere: 'https://cdn.pixabay.com/photo/2017/08/30/12/45/girl-2696947_1280.jpg',
    duree: 18,
    temp: 38,
    lien: 'https://www.youtube.com/watch?v=example2',
    titre: 'Culture et traditions africaines'
  },
  {
    id: 3,
    order: 3,
    banniere: 'https://cdn.pixabay.com/photo/2019/06/20/16/10/traditional-dance-4287442_1280.jpg',
    duree: 15,
    temp: 53,
    lien: 'https://www.youtube.com/watch?v=example3',
    titre: 'Danses traditionnelles d\'Afrique'
  },
  {
    id: 4,
    order: 4,
    banniere: 'https://cdn.pixabay.com/photo/2021/01/04/10/45/massai-5887817_1280.jpg',
    duree: 12,
    temp: 65,
    lien: 'https://www.youtube.com/watch?v=example4',
    titre: 'Les Maasaï: Un peuple millénaire'
  }
]

// Chaînes TV de démonstration
export const tvChannels: TvChannel[] = [
  {
    id: 'rti-1',
    name: 'RTI 1',
    description: 'Radiodiffusion Télévision Ivoirienne - Chaîne nationale',
    streamUrl: 'https://example.com/rti1.m3u8',
    cover: 'https://cdn.pixabay.com/photo/2020/04/21/13/32/africa-5072833_1280.jpg',
    category: 'Généraliste',
    country: 'Côte d\'Ivoire',
    language: 'Français',
    isLive: true
  },
  {
    id: 'rts-1',
    name: 'RTS 1',
    description: 'Radiodiffusion Télévision Sénégalaise',
    streamUrl: 'https://example.com/rts1.m3u8',
    cover: 'https://cdn.pixabay.com/photo/2023/03/12/18/11/african-7848586_1280.jpg',
    category: 'Généraliste',
    country: 'Sénégal',
    language: 'Français',
    isLive: true
  },
  {
    id: 'nta',
    name: 'NTA',
    description: 'Nigerian Television Authority',
    streamUrl: 'https://example.com/nta.m3u8',
    cover: 'https://cdn.pixabay.com/photo/2020/01/31/07/53/disc-jockey-4807566_1280.jpg',
    category: 'Généraliste',
    country: 'Nigeria',
    language: 'Anglais',
    isLive: true
  },
  {
    id: 'crtv',
    name: 'CRTV',
    description: 'Cameroon Radio Television',
    streamUrl: 'https://example.com/crtv.m3u8',
    cover: 'https://cdn.pixabay.com/photo/2017/08/06/12/52/woman-2592247_1280.jpg',
    category: 'Généraliste',
    country: 'Cameroun',
    language: 'Français',
    isLive: true
  },
  {
    id: 'rtnc',
    name: 'RTNC',
    description: 'Radio-Télévision Nationale Congolaise',
    streamUrl: 'https://example.com/rtnc.m3u8',
    cover: 'https://cdn.pixabay.com/photo/2019/06/20/16/10/traditional-dance-4287442_1280.jpg',
    category: 'Généraliste',
    country: 'RDC',
    language: 'Français',
    isLive: true
  },
  {
    id: 'ghbc',
    name: 'GBC',
    description: 'Ghana Broadcasting Corporation',
    streamUrl: 'https://example.com/gbc.m3u8',
    cover: 'https://cdn.pixabay.com/photo/2020/04/21/13/32/africa-5072833_1280.jpg',
    category: 'Généraliste',
    country: 'Ghana',
    language: 'Anglais',
    isLive: true
  },
  {
    id: 'ktn',
    name: 'KTN',
    description: 'Kenya Television Network',
    streamUrl: 'https://example.com/ktn.m3u8',
    cover: 'https://cdn.pixabay.com/photo/2021/01/04/10/45/massai-5887817_1280.jpg',
    category: 'Généraliste',
    country: 'Kenya',
    language: 'Anglais',
    isLive: true
  },
  {
    id: 'ortm',
    name: 'ORTM',
    description: 'Office de Radiodiffusion Télévision du Mali',
    streamUrl: 'https://example.com/ortm.m3u8',
    cover: 'https://cdn.pixabay.com/photo/2017/08/30/12/45/girl-2696947_1280.jpg',
    category: 'Généraliste',
    country: 'Mali',
    language: 'Français',
    isLive: true
  }
]

// Fonctions utilitaires
export const getTvChannelById = (id: string): TvChannel | undefined => {
  return tvChannels.find(channel => channel.id === id)
}

export const getTvChannelsByCountry = (country: string): TvChannel[] => {
  if (country === 'Tous les territoires') return tvChannels
  return tvChannels.filter(channel => channel.country === country)
}

export const getTvChannelsByCategory = (category: string): TvChannel[] => {
  if (category === 'Toutes les catégories') return tvChannels
  return tvChannels.filter(channel => channel.category === category)
}

export const getUniqueTvCountries = (): string[] => {
  const countries = [...new Set(tvChannels.map(channel => channel.country))]
  return countries.sort()
}

export const getUniqueTvCategories = (): string[] => {
  const categories = [...new Set(tvChannels.map(channel => channel.category))]
  return categories.sort()
}

// URL vidéo de couverture par défaut
export const defaultCoverVideoUrl = 'https://firebasestorage.googleapis.com/v0/b/epavillon-12137.appspot.com/o/video_couverture2.mp4?alt=media&token=4a790687-2f5a-45c6-a01f-03889105879d'

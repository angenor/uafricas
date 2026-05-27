// Types et interfaces pour les radios
export interface RadioStation {
  id: string
  name: string
  description: string
  streamUrl: string
  cover: string
  genre: string
  genresList: string[]
  location: string
  country: string
  programType: 'Nationales' | 'Local' | 'International'
}

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

export interface ProgramForm {
  nom: string
  description: string
  imageCouverture: string
  telephone: string
  emailProducteur: string
  pays: string
  urlVideo: string
  urlAudio: string
  siteWeb: string
  langue: string
  langueLocale: string
  programType: string
  typeMedia: 'Radio' | 'Télévision' | 'Mixte'
  genres: string[]
  loading: boolean
  submitted: boolean
  error: boolean
  errorMessage: string
}

// Liste des genres musicaux disponibles (41 genres)
export const availableGenres: string[] = [
  'Afrobeats',
  'Afropop',
  'Hip-Hop',
  'Rap',
  'R&B',
  'Soukous',
  'Rumba',
  'Makossa',
  'Bikutsi',
  'Highlife',
  'Hiplife',
  'Mbalax',
  'Coupé-Décalé',
  'Ndombolo',
  'Genge',
  'Benga',
  'Jazz africain',
  'Musique traditionnelle',
  'Musique mandingue',
  'Gospel',
  'Reggae',
  'Dancehall',
  'Zouk',
  'Kompa',
  'Bongo Flava',
  'Amapiano',
  'Kwaito',
  'Actualité',
  'Talk Show',
  'Débats',
  'Sport',
  'Culture',
  'Éducation',
  'Musique du monde',
  'Électronique',
  'Pop internationale',
  'Rock',
  'Blues',
  'Soul',
  'Funk',
  'Variété'
]

// Liste des pays supportés (26 pays)
export const availableCountries: string[] = [
  'Côte d\'Ivoire',
  'Sénégal',
  'Nigeria',
  'Ghana',
  'Mali',
  'Burkina Faso',
  'RDC',
  'Cameroun',
  'Kenya',
  'Bénin',
  'Togo',
  'Mauritanie',
  'Maroc',
  'Algérie',
  'Tunisie',
  'Égypte',
  'Éthiopie',
  'Afrique du Sud',
  'Angola',
  'Tanzanie',
  'Ouganda',
  'Zambie',
  'Zimbabwe',
  'France',
  'Canada',
  'États-Unis'
]

// Catégories de radios pour la page hub
export const radioCategories: RadioCategory[] = [
  {
    id: 1,
    title: 'Radios Africans',
    description: 'Découvrez les meilleures stations de radio africaines internationales diffusant de la musique et des programmes culturels.',
    image: '/images/banners/radio-africans.avif',
    badge: 'International',
    badgeColor: 'green',
    link: '/radio/africans'
  },
  {
    id: 2,
    title: 'Radios Nationales',
    description: 'Écoutez les radios nationales de chaque territoire africain avec des programmes locaux et des informations régionales.',
    image: '/images/banners/radio-national.jpg',
    badge: 'Local',
    badgeColor: 'chocolat',
    link: '/radio/nationales'
  }
]

// Statistiques pour la page hub radios
export const radioStats: RadioStats[] = [
  { value: '150+', label: 'Stations Radio' },
  { value: '54', label: 'Territoires Africains' },
  { value: '24/7', label: 'Diffusion Continue' },
  { value: 'HD', label: 'Qualité Audio' }
]

// 13 stations radio de démonstration
export const radioStations: RadioStation[] = [
  {
    id: 'africa-n1',
    name: 'Africa N°1',
    description: 'La radio panafricaine par excellence, diffusant de la musique africaine moderne et traditionnelle.',
    streamUrl: 'https://african1paris.ice.infomaniak.ch/african1paris-128.mp3',
    cover: 'https://i.pinimg.com/originals/84/bc/28/84bc28d77e23db447a893e9cb0f72ef1.jpg',
    genre: 'Afropop, Soukous',
    genresList: ['Afropop', 'Soukous'],
    location: 'Paris, France',
    country: 'France',
    programType: 'Nationales'
  },
  {
    id: 'trace-fm',
    name: 'Trace FM',
    description: 'Une radio dédiée aux hits urbains africains et aux nouvelles tendances musicales.',
    streamUrl: 'https://stream.rcs.revma.com/ypqt40u0x1zuv',
    cover: 'https://image.winudf.com/v2/image1/Y29tLnRyYWNldHYuYWZyaWNhX2ljb25fMTU2MDI2NzA4OF8wMTk/icon.png?w=&fakeurl=1',
    genre: 'Afrobeats, Hip-Hop',
    genresList: ['Afrobeats', 'Hip-Hop'],
    location: 'Abidjan, Côte d\'Ivoire',
    country: 'Côte d\'Ivoire',
    programType: 'Nationales'
  },
  {
    id: 'radio-okapi',
    name: 'Radio Okapi',
    description: 'Radio de référence en République Démocratique du Congo avec une programmation variée.',
    streamUrl: 'https://stream.zeno.fm/1kb0991khf9uv',
    cover: 'https://radio-okapi-lpa.monsite-orange.fr/file/49ee284df7ddd4badf44eaa71800fd1f.jpg',
    genre: 'Actualité, Soukous',
    genresList: ['Actualité', 'Soukous'],
    location: 'Kinshasa, RDC',
    country: 'RDC',
    programType: 'Local'
  },
  {
    id: 'zikeli-africa',
    name: 'Zikeli Africa',
    description: 'Une immersion dans la musique africaine contemporaine avec des artistes émergents.',
    streamUrl: 'https://stream.zeno.fm/mh7sqa4u4rhvv',
    cover: 'https://cdn.pixabay.com/photo/2020/01/31/07/53/disc-jockey-4807566_1280.jpg',
    genre: 'Afrobeats, Highlife',
    genresList: ['Afrobeats', 'Highlife'],
    location: 'Lagos, Nigeria',
    country: 'Nigeria',
    programType: 'Nationales'
  },
  {
    id: 'rfi-afrique',
    name: 'RFI Afrique',
    description: 'Radio France Internationale pour l\'Afrique, offrant des actualités et une diversité musicale.',
    streamUrl: 'https://rfiafrique64k.ice.infomaniak.ch/rfiafrique-64.mp3',
    cover: 'https://upload.wikimedia.org/wikipedia/fr/thumb/e/ef/RFI_logo_2013.svg/1200px-RFI_logo_2013.svg.png',
    genre: 'Actualité, Variété',
    genresList: ['Actualité', 'Variété'],
    location: 'Paris, France',
    country: 'France',
    programType: 'International'
  },
  {
    id: 'radio-nouakchott',
    name: 'Radio Nouakchott',
    description: 'Radio mauritanienne diffusant de la musique traditionnelle et moderne d\'Afrique de l\'Ouest.',
    streamUrl: 'https://stream.zeno.fm/1kb0991khf9uv',
    cover: 'https://cdn.pixabay.com/photo/2017/10/03/12/02/desert-2812655_1280.jpg',
    genre: 'Musique traditionnelle',
    genresList: ['Musique traditionnelle'],
    location: 'Nouakchott, Mauritanie',
    country: 'Mauritanie',
    programType: 'Local'
  },
  {
    id: 'hits-africa',
    name: 'Hits Africa',
    description: 'Les meilleurs hits afrobeats et afropop du continent africain.',
    streamUrl: 'https://stream.zenolive.com/w89xvpn5v2quv',
    cover: 'https://cdn.pixabay.com/photo/2015/04/01/08/38/drums-702551_1280.jpg',
    genre: 'Afrobeats, Coupé-Décalé',
    genresList: ['Afrobeats', 'Coupé-Décalé'],
    location: 'Dakar, Sénégal',
    country: 'Sénégal',
    programType: 'Nationales'
  },
  {
    id: 'radio-senegal',
    name: 'Radio Sénégal',
    description: 'La radio nationale sénégalaise proposant une programmation musicale riche et variée.',
    streamUrl: 'https://stream.zeno.fm/h7n8ug96ptzuv',
    cover: 'https://cdn.pixabay.com/photo/2023/03/12/18/11/african-7848586_1280.jpg',
    genre: 'Mbalax, Afropop',
    genresList: ['Mbalax', 'Afropop'],
    location: 'Dakar, Sénégal',
    country: 'Sénégal',
    programType: 'Local'
  },
  {
    id: 'cameroon-radio',
    name: 'Cameroon Radio',
    description: 'Une station dédiée à la musique et à la culture camerounaises, du makossa au bikutsi.',
    streamUrl: 'https://stream.zeno.fm/yd1c24mbytzuv',
    cover: 'https://cdn.pixabay.com/photo/2017/08/06/12/52/woman-2592247_1280.jpg',
    genre: 'Makossa, Bikutsi',
    genresList: ['Makossa', 'Bikutsi'],
    location: 'Douala, Cameroun',
    country: 'Cameroun',
    programType: 'Local'
  },
  {
    id: 'kenya-beats',
    name: 'Kenya Beats',
    description: 'Les dernières tendances musicales et artistes du Kenya et de l\'Afrique de l\'Est.',
    streamUrl: 'https://stream.zeno.fm/eyuiqmwyqtzuv',
    cover: 'https://cdn.pixabay.com/photo/2021/01/04/10/45/massai-5887817_1280.jpg',
    genre: 'Genge, Benga',
    genresList: ['Genge', 'Benga'],
    location: 'Nairobi, Kenya',
    country: 'Kenya',
    programType: 'Local'
  },
  {
    id: 'ghana-gold',
    name: 'Ghana Gold',
    description: 'Célébration de la richesse musicale du Ghana avec du highlife classique et contemporain.',
    streamUrl: 'https://stream.zeno.fm/66zgtrmbytzuv',
    cover: 'https://cdn.pixabay.com/photo/2020/04/21/13/32/africa-5072833_1280.jpg',
    genre: 'Highlife, Hiplife',
    genresList: ['Highlife', 'Hiplife'],
    location: 'Accra, Ghana',
    country: 'Ghana',
    programType: 'Local'
  },
  {
    id: 'mali-musique',
    name: 'Mali Musique',
    description: 'Une immersion dans les sons authentiques maliens avec des artistes traditionnels et modernes.',
    streamUrl: 'https://stream.zeno.fm/2xgfm76bytzuv',
    cover: 'https://cdn.pixabay.com/photo/2017/08/30/12/45/girl-2696947_1280.jpg',
    genre: 'Musique mandingue, Jazz africain',
    genresList: ['Musique mandingue', 'Jazz africain'],
    location: 'Bamako, Mali',
    country: 'Mali',
    programType: 'Nationales'
  },
  {
    id: 'congo-rumba',
    name: 'Congo Rumba',
    description: 'Dédié à la rumba congolaise et aux sonorités congolaises qui ont influencé la musique mondiale.',
    streamUrl: 'https://stream.zeno.fm/3xgfm76bytzuv',
    cover: 'https://cdn.pixabay.com/photo/2019/06/20/16/10/traditional-dance-4287442_1280.jpg',
    genre: 'Rumba, Soukous',
    genresList: ['Rumba', 'Soukous'],
    location: 'Kinshasa, RDC',
    country: 'RDC',
    programType: 'Nationales'
  }
]

// Fonctions utilitaires
export const getRadioById = (id: string): RadioStation | undefined => {
  return radioStations.find(station => station.id === id)
}

export const getRadiosByCountry = (country: string): RadioStation[] => {
  if (country === 'Tous les territoires') return radioStations
  return radioStations.filter(station => station.country === country)
}

export const getRadiosByGenre = (genre: string): RadioStation[] => {
  if (genre === 'Tous les genres') return radioStations
  return radioStations.filter(station => station.genresList.includes(genre))
}

export const getRadiosByType = (type: string): RadioStation[] => {
  if (type === 'Tous les types') return radioStations
  return radioStations.filter(station => station.programType === type)
}

export const filterRadios = (
  type: string = 'Tous les types',
  country: string = 'Tous les territoires',
  genre: string = 'Tous les genres'
): RadioStation[] => {
  let filtered = [...radioStations]

  if (type !== 'Tous les types') {
    filtered = filtered.filter(station => station.programType === type)
  }

  if (country !== 'Tous les territoires') {
    filtered = filtered.filter(station => station.country === country)
  }

  if (genre !== 'Tous les genres') {
    filtered = filtered.filter(station => station.genresList.includes(genre))
  }

  return filtered
}

export const getUniqueCountriesFromStations = (): string[] => {
  const countries = [...new Set(radioStations.map(station => station.country))]
  return countries.sort()
}

export const getUniqueGenresFromStations = (): string[] => {
  const allGenres = radioStations.flatMap(station => station.genresList)
  const uniqueGenres = [...new Set(allGenres)]
  return uniqueGenres.sort()
}

// Fonction pour créer un formulaire vide
export const createEmptyProgramForm = (): ProgramForm => ({
  nom: '',
  description: '',
  imageCouverture: '',
  telephone: '',
  emailProducteur: '',
  pays: '',
  urlVideo: '',
  urlAudio: '',
  siteWeb: '',
  langue: 'Français',
  langueLocale: '',
  programType: 'Local',
  typeMedia: 'Radio',
  genres: [],
  loading: false,
  submitted: false,
  error: false,
  errorMessage: ''
})

// Donnees mock pour les opportunites en Afrique (fiches pays)

export type Region =
  | 'Afrique Centrale'
  | 'Afrique de l\'Ouest'
  | 'Afrique de l\'Est'
  | 'Afrique du Nord'
  | 'Afrique Australe'

export interface FichePays {
  id: string
  code: string                    // Code ISO 3166
  nom: string
  imageCouverture: string
  slogan?: string
  superficie: string
  population: string
  capitale: string
  monnaie: string
  drapeauURL: string
  emblemeURL?: string
  devise?: string
  langues: string[]
  ethnies: string[]
  region: Region
  derniereValidation: Date
  contributeursPrincipaux: string[]
  nombreContributions: number
}

export interface FichePaysStats {
  totalPays: number
  regions: Region[]
  derniereMiseAJour: Date | null
}

// Donnees mock des pays africains
export const paysAfricainsMock: FichePays[] = [
  {
    id: 'senegal',
    code: 'SN',
    nom: 'Sénégal',
    imageCouverture: 'https://images.unsplash.com/photo-1589556264800-08ae9e129a8c?w=800',
    slogan: 'Un Peuple, Un But, Une Foi',
    superficie: '196 722 km²',
    population: '17,7 millions',
    capitale: 'Dakar',
    monnaie: 'Franc CFA (XOF)',
    drapeauURL: 'https://flagcdn.com/w160/sn.png',
    devise: 'Un Peuple, Un But, Une Foi',
    langues: ['Français', 'Wolof', 'Sérère', 'Diola', 'Mandingue', 'Peul'],
    ethnies: ['Wolof', 'Peul', 'Sérère', 'Diola', 'Mandingue', 'Soninké'],
    region: 'Afrique de l\'Ouest',
    derniereValidation: new Date('2025-01-15'),
    contributeursPrincipaux: ['Amadou Diallo', 'Fatou Ndiaye'],
    nombreContributions: 45
  },
  {
    id: 'cote-divoire',
    code: 'CI',
    nom: 'Côte d\'Ivoire',
    imageCouverture: 'https://images.unsplash.com/photo-1591117207239-788bf8de6c3b?w=800',
    slogan: 'Union, Discipline, Travail',
    superficie: '322 463 km²',
    population: '29,4 millions',
    capitale: 'Yamoussoukro',
    monnaie: 'Franc CFA (XOF)',
    drapeauURL: 'https://flagcdn.com/w160/ci.png',
    devise: 'Union, Discipline, Travail',
    langues: ['Français', 'Dioula', 'Baoulé', 'Bété', 'Sénoufo'],
    ethnies: ['Baoulé', 'Bété', 'Sénoufo', 'Malinké', 'Dan', 'Lobi'],
    region: 'Afrique de l\'Ouest',
    derniereValidation: new Date('2025-01-10'),
    contributeursPrincipaux: ['Kouamé Yao', 'Adjoua Koné'],
    nombreContributions: 38
  },
  {
    id: 'cameroun',
    code: 'CM',
    nom: 'Cameroun',
    imageCouverture: 'https://images.unsplash.com/photo-1596005554384-d293674c91d7?w=800',
    slogan: 'Paix, Travail, Patrie',
    superficie: '475 442 km²',
    population: '28,6 millions',
    capitale: 'Yaoundé',
    monnaie: 'Franc CFA (XAF)',
    drapeauURL: 'https://flagcdn.com/w160/cm.png',
    devise: 'Paix, Travail, Patrie',
    langues: ['Français', 'Anglais', 'Fulfulde', 'Ewondo', 'Bamiléké'],
    ethnies: ['Bamiléké', 'Béti', 'Fulani', 'Kirdi', 'Bassa', 'Douala'],
    region: 'Afrique Centrale',
    derniereValidation: new Date('2025-01-12'),
    contributeursPrincipaux: ['Jean-Pierre Mbarga', 'Marie Tabi'],
    nombreContributions: 52
  },
  {
    id: 'rdc',
    code: 'CD',
    nom: 'République Démocratique du Congo',
    imageCouverture: 'https://images.unsplash.com/photo-1580746738099-78d6833b3471?w=800',
    slogan: 'Justice, Paix, Travail',
    superficie: '2 345 409 km²',
    population: '99,0 millions',
    capitale: 'Kinshasa',
    monnaie: 'Franc congolais (CDF)',
    drapeauURL: 'https://flagcdn.com/w160/cd.png',
    devise: 'Justice, Paix, Travail',
    langues: ['Français', 'Lingala', 'Swahili', 'Kikongo', 'Tshiluba'],
    ethnies: ['Luba', 'Kongo', 'Mongo', 'Mangbetu-Azande', 'Lunda'],
    region: 'Afrique Centrale',
    derniereValidation: new Date('2025-01-08'),
    contributeursPrincipaux: ['Patrick Mukendi', 'Grace Mbuyi'],
    nombreContributions: 67
  },
  {
    id: 'kenya',
    code: 'KE',
    nom: 'Kenya',
    imageCouverture: 'https://images.unsplash.com/photo-1489392191049-fc10c97e64b6?w=800',
    slogan: 'Harambee (Travaillons ensemble)',
    superficie: '580 367 km²',
    population: '54,0 millions',
    capitale: 'Nairobi',
    monnaie: 'Shilling kényan (KES)',
    drapeauURL: 'https://flagcdn.com/w160/ke.png',
    devise: 'Harambee',
    langues: ['Swahili', 'Anglais', 'Kikuyu', 'Luo', 'Kamba'],
    ethnies: ['Kikuyu', 'Luhya', 'Kalenjin', 'Luo', 'Kamba', 'Kisii', 'Meru'],
    region: 'Afrique de l\'Est',
    derniereValidation: new Date('2025-01-14'),
    contributeursPrincipaux: ['James Ochieng', 'Faith Wanjiku'],
    nombreContributions: 41
  },
  {
    id: 'ethiopie',
    code: 'ET',
    nom: 'Éthiopie',
    imageCouverture: 'https://images.unsplash.com/photo-1523805009345-7448845a9e53?w=800',
    slogan: 'L\'Éthiopie d\'abord',
    superficie: '1 104 300 km²',
    population: '126,5 millions',
    capitale: 'Addis-Abeba',
    monnaie: 'Birr éthiopien (ETB)',
    drapeauURL: 'https://flagcdn.com/w160/et.png',
    devise: 'L\'Éthiopie d\'abord',
    langues: ['Amharique', 'Oromo', 'Tigrinya', 'Somali', 'Afar'],
    ethnies: ['Oromo', 'Amhara', 'Somali', 'Tigréens', 'Sidama', 'Gurage'],
    region: 'Afrique de l\'Est',
    derniereValidation: new Date('2025-01-05'),
    contributeursPrincipaux: ['Abebe Tadesse', 'Meron Bekele'],
    nombreContributions: 33
  },
  {
    id: 'egypte',
    code: 'EG',
    nom: 'Égypte',
    imageCouverture: 'https://images.unsplash.com/photo-1539650116574-8efeb43e2750?w=800',
    slogan: 'Liberté, Socialisme, Unité',
    superficie: '1 001 449 km²',
    population: '109,3 millions',
    capitale: 'Le Caire',
    monnaie: 'Livre égyptienne (EGP)',
    drapeauURL: 'https://flagcdn.com/w160/eg.png',
    devise: 'Liberté, Socialisme, Unité',
    langues: ['Arabe', 'Arabe égyptien'],
    ethnies: ['Égyptiens', 'Bédouins', 'Nubiens', 'Berbères'],
    region: 'Afrique du Nord',
    derniereValidation: new Date('2025-01-11'),
    contributeursPrincipaux: ['Ahmed Hassan', 'Fatma El-Sayed'],
    nombreContributions: 58
  },
  {
    id: 'maroc',
    code: 'MA',
    nom: 'Maroc',
    imageCouverture: 'https://images.unsplash.com/photo-1493246507139-91e8fad9978e?w=800',
    slogan: 'Dieu, la Patrie, le Roi',
    superficie: '446 550 km²',
    population: '37,8 millions',
    capitale: 'Rabat',
    monnaie: 'Dirham marocain (MAD)',
    drapeauURL: 'https://flagcdn.com/w160/ma.png',
    devise: 'Dieu, la Patrie, le Roi',
    langues: ['Arabe', 'Amazigh', 'Français'],
    ethnies: ['Arabes', 'Berbères', 'Gnawa', 'Haratin'],
    region: 'Afrique du Nord',
    derniereValidation: new Date('2025-01-13'),
    contributeursPrincipaux: ['Youssef Benali', 'Amina Tazi'],
    nombreContributions: 49
  },
  {
    id: 'afrique-du-sud',
    code: 'ZA',
    nom: 'Afrique du Sud',
    imageCouverture: 'https://images.unsplash.com/photo-1580060839134-75a5edca2e99?w=800',
    slogan: '!ke e: /xarra //ke (L\'unité dans la diversité)',
    superficie: '1 221 037 km²',
    population: '60,0 millions',
    capitale: 'Pretoria',
    monnaie: 'Rand sud-africain (ZAR)',
    drapeauURL: 'https://flagcdn.com/w160/za.png',
    devise: '!ke e: /xarra //ke',
    langues: ['Zoulou', 'Xhosa', 'Afrikaans', 'Anglais', 'Pedi', 'Tswana', 'Sotho', 'Tsonga', 'Swati', 'Venda', 'Ndebele'],
    ethnies: ['Zoulou', 'Xhosa', 'Métis', 'Blancs', 'Pedi', 'Sotho', 'Tswana'],
    region: 'Afrique Australe',
    derniereValidation: new Date('2025-01-09'),
    contributeursPrincipaux: ['Sipho Ndlovu', 'Thandi Molefe'],
    nombreContributions: 72
  },
  {
    id: 'nigeria',
    code: 'NG',
    nom: 'Nigeria',
    imageCouverture: 'https://images.unsplash.com/photo-1618828665011-0abd973f7bb8?w=800',
    slogan: 'Unité et Foi, Paix et Progrès',
    superficie: '923 768 km²',
    population: '223,8 millions',
    capitale: 'Abuja',
    monnaie: 'Naira nigérian (NGN)',
    drapeauURL: 'https://flagcdn.com/w160/ng.png',
    devise: 'Unité et Foi, Paix et Progrès',
    langues: ['Anglais', 'Haoussa', 'Yoruba', 'Igbo', 'Fulani'],
    ethnies: ['Haoussa', 'Yoruba', 'Igbo', 'Fulani', 'Ijaw', 'Kanuri', 'Ibibio', 'Tiv'],
    region: 'Afrique de l\'Ouest',
    derniereValidation: new Date('2025-01-16'),
    contributeursPrincipaux: ['Chukwuemeka Okonkwo', 'Aisha Mohammed'],
    nombreContributions: 85
  },
  {
    id: 'ghana',
    code: 'GH',
    nom: 'Ghana',
    imageCouverture: 'https://images.unsplash.com/photo-1577948000111-9c970dfe3743?w=800',
    slogan: 'Liberté et Justice',
    superficie: '238 535 km²',
    population: '33,5 millions',
    capitale: 'Accra',
    monnaie: 'Cedi ghanéen (GHS)',
    drapeauURL: 'https://flagcdn.com/w160/gh.png',
    devise: 'Liberté et Justice',
    langues: ['Anglais', 'Twi', 'Fante', 'Ga', 'Ewe', 'Dagbani'],
    ethnies: ['Akan', 'Mole-Dagbani', 'Ewe', 'Ga-Dangme', 'Gurma', 'Guan'],
    region: 'Afrique de l\'Ouest',
    derniereValidation: new Date('2025-01-07'),
    contributeursPrincipaux: ['Kwame Asante', 'Abena Mensah'],
    nombreContributions: 36
  },
  {
    id: 'tanzanie',
    code: 'TZ',
    nom: 'Tanzanie',
    imageCouverture: 'https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?w=800',
    slogan: 'Uhuru na Umoja (Liberté et Unité)',
    superficie: '945 087 km²',
    population: '65,5 millions',
    capitale: 'Dodoma',
    monnaie: 'Shilling tanzanien (TZS)',
    drapeauURL: 'https://flagcdn.com/w160/tz.png',
    devise: 'Uhuru na Umoja',
    langues: ['Swahili', 'Anglais', 'Sukuma', 'Chagga', 'Haya'],
    ethnies: ['Sukuma', 'Chagga', 'Haya', 'Makonde', 'Nyamwezi', 'Zanzibar'],
    region: 'Afrique de l\'Est',
    derniereValidation: new Date('2025-01-06'),
    contributeursPrincipaux: ['Juma Mwalimu', 'Rehema Mushi'],
    nombreContributions: 29
  }
]

// Fonction pour obtenir tous les pays
export const getAllPays = (): FichePays[] => {
  return [...paysAfricainsMock]
}

// Fonction pour obtenir un pays par son ID
export const getPaysById = (id: string): FichePays | undefined => {
  return paysAfricainsMock.find(p => p.id === id || p.code.toLowerCase() === id.toLowerCase())
}

// Fonction pour obtenir les pays par region
export const getPaysByRegion = (region: Region): FichePays[] => {
  return paysAfricainsMock.filter(p => p.region === region)
}

// Fonction pour rechercher des pays par nom
export const searchPays = (term: string): FichePays[] => {
  const searchLower = term.toLowerCase().trim()
  if (!searchLower) return getAllPays()

  return paysAfricainsMock.filter(p =>
    p.nom.toLowerCase().includes(searchLower) ||
    p.capitale.toLowerCase().includes(searchLower) ||
    p.code.toLowerCase().includes(searchLower)
  )
}

// Fonction pour obtenir les regions uniques
export const getRegionsUniques = (): Region[] => {
  const regions = new Set<Region>()
  paysAfricainsMock.forEach(p => regions.add(p.region))
  return Array.from(regions).sort()
}

// Fonction pour obtenir les statistiques
export const getStats = (): FichePaysStats => {
  const regions = getRegionsUniques()
  const dates = paysAfricainsMock
    .map(p => p.derniereValidation)
    .filter(d => d)
    .sort((a, b) => b.getTime() - a.getTime())

  return {
    totalPays: paysAfricainsMock.length,
    regions,
    derniereMiseAJour: dates.length > 0 ? dates[0] : null
  }
}

// Fonction pour formater une date en francais
export const formatDate = (date: Date): string => {
  return date.toLocaleDateString('fr-FR', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
}

// Fonction pour formater une date courte
export const formatDateShort = (date: Date): string => {
  return date.toLocaleDateString('fr-FR', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

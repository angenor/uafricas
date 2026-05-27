// Données mock pour le Marché Africain

export type TypeEchange = 'Vente' | 'Troc' | 'Don'
export type Categorie = 'Agriculture' | 'Informatique' | 'Immobilier' | 'Voitures' | 'Electronique' | 'Formation'
export type Devise = 'XOF' | 'EUR' | 'NGN' | 'USD'

export interface AnnonceUser {
  uid: string
  email: string
  nom: string
  prenom: string
}

export interface Annonce {
  id: string
  titre: string
  description: string
  type_echange: TypeEchange
  categorie: Categorie
  prix: number
  devise: Devise
  pays: string
  ville: string
  tel: string
  photo_url: string
  minQty?: number
  user: AnnonceUser
  created_at: Date
  updated_at?: Date
}

export interface FiltresAnnonce {
  categorie: Categorie | 'Tout'
  typesEchange: TypeEchange[]
  prixMin: number | null
  prixMax: number | null
  recherche: string
  tri: 'recent' | 'price-asc' | 'price-desc'
}

// Constantes pour les selects
export const CATEGORIES: { key: Categorie | 'Tout'; label: string }[] = [
  { key: 'Tout', label: 'Toutes les catégories' },
  { key: 'Agriculture', label: 'Agriculture' },
  { key: 'Informatique', label: 'Informatique' },
  { key: 'Immobilier', label: 'Immobilier' },
  { key: 'Voitures', label: 'Voitures' },
  { key: 'Electronique', label: 'Électronique' },
  { key: 'Formation', label: 'Formation' }
]

export const TYPES_ECHANGE: { value: TypeEchange; label: string; color: string }[] = [
  { value: 'Vente', label: 'Vente', color: 'bg-white/90 text-gray-700' },
  { value: 'Troc', label: 'Troc', color: 'bg-purple-100/90 text-purple-700' },
  { value: 'Don', label: 'Don', color: 'bg-blue-100/90 text-blue-700' }
]

export const DEVISES: { value: Devise; label: string; symbol: string }[] = [
  { value: 'XOF', label: 'Franc CFA', symbol: 'FCFA' },
  { value: 'EUR', label: 'Euro', symbol: '€' },
  { value: 'NGN', label: 'Naira', symbol: '₦' },
  { value: 'USD', label: 'Dollar US', symbol: '$' }
]

export const PAYS_AFRICAINS: { value: string; label: string }[] = [
  { value: '', label: 'Tous les territoires' },
  { value: 'Afrique du Sud', label: 'Afrique du Sud' },
  { value: 'Burkina Faso', label: 'Burkina Faso' },
  { value: 'Cap-Vert', label: 'Cap-Vert' },
  { value: 'Comores', label: 'Comores' },
  { value: 'Côte d\'Ivoire', label: 'Côte d\'Ivoire' },
  { value: 'Éthiopie', label: 'Éthiopie' },
  { value: 'Gabon', label: 'Gabon' },
  { value: 'Gambie', label: 'Gambie' },
  { value: 'Ghana', label: 'Ghana' },
  { value: 'Mali', label: 'Mali' },
  { value: 'Mauritanie', label: 'Mauritanie' },
  { value: 'Namibie', label: 'Namibie' },
  { value: 'Niger', label: 'Niger' },
  { value: 'Nigeria', label: 'Nigeria' },
  { value: 'Sénégal', label: 'Sénégal' }
]

// Données mock (15 annonces variées)
export const annoncesMock: Annonce[] = [
  // Agriculture
  {
    id: 'ann-1',
    titre: 'Tracteur John Deere 5075E - Excellent état',
    description: 'Tracteur agricole John Deere 5075E de 2020, 75 CV, seulement 850 heures de fonctionnement. Parfait pour exploitations moyennes. Entretien régulier effectué chez concessionnaire agréé. Livraison possible dans la région.',
    type_echange: 'Vente',
    categorie: 'Agriculture',
    prix: 15000000,
    devise: 'XOF',
    pays: 'Sénégal',
    ville: 'Thiès',
    tel: '+221 77 123 45 67',
    photo_url: 'https://images.unsplash.com/photo-1530267981375-f0de937f5f13?w=800',
    user: { uid: 'u1', email: 'mamadou.diallo@email.sn', nom: 'Diallo', prenom: 'Mamadou' },
    created_at: new Date('2026-01-15'),
    updated_at: new Date('2026-01-20')
  },
  {
    id: 'ann-2',
    titre: 'Semences de maïs hybride certifiées - 500kg',
    description: 'Lot de semences de maïs hybride certifiées, variété adaptée au climat sahélien. Rendement prouvé de 6-8 tonnes/hectare. Conditionnement en sacs de 25kg. Certificat phytosanitaire fourni.',
    type_echange: 'Vente',
    categorie: 'Agriculture',
    prix: 175000,
    devise: 'XOF',
    pays: 'Mali',
    ville: 'Sikasso',
    tel: '+223 66 789 01 23',
    photo_url: 'https://images.unsplash.com/photo-1574323347407-f5e1ad6d020b?w=800',
    minQty: 100,
    user: { uid: 'u2', email: 'oumar.traore@email.ml', nom: 'Traoré', prenom: 'Oumar' },
    created_at: new Date('2026-01-10'),
    updated_at: new Date('2026-01-12')
  },
  {
    id: 'ann-3',
    titre: 'Engrais NPK 15-15-15 - 2 tonnes disponibles',
    description: 'Engrais composé NPK 15-15-15 de qualité supérieure. Idéal pour cultures céréalières et maraîchères. Prix négociable pour grande quantité. Livraison possible sur Abidjan et environs.',
    type_echange: 'Vente',
    categorie: 'Agriculture',
    prix: 450000,
    devise: 'XOF',
    pays: 'Côte d\'Ivoire',
    ville: 'Abidjan',
    tel: '+225 07 456 78 90',
    photo_url: 'https://images.unsplash.com/photo-1416879595882-3373a0480b5b?w=800',
    user: { uid: 'u3', email: 'koffi.yao@email.ci', nom: 'Yao', prenom: 'Koffi' },
    created_at: new Date('2026-01-08')
  },
  // Informatique
  {
    id: 'ann-4',
    titre: 'MacBook Pro M2 14" - Comme neuf',
    description: 'MacBook Pro 14 pouces avec puce M2 Pro, 16Go RAM, 512Go SSD. Acheté en septembre 2025, sous garantie Apple Care+ jusqu\'en 2027. Batterie à 98%. Vendu avec chargeur original et housse.',
    type_echange: 'Vente',
    categorie: 'Informatique',
    prix: 1200,
    devise: 'EUR',
    pays: 'Sénégal',
    ville: 'Dakar',
    tel: '+221 78 234 56 78',
    photo_url: 'https://images.unsplash.com/photo-1517336714731-489689fd1ca8?w=800',
    user: { uid: 'u4', email: 'fatou.ndiaye@email.sn', nom: 'Ndiaye', prenom: 'Fatou' },
    created_at: new Date('2026-01-18'),
    updated_at: new Date('2026-01-19')
  },
  {
    id: 'ann-5',
    titre: 'Imprimante HP LaserJet Pro + Stock de toners',
    description: 'Imprimante laser HP LaserJet Pro M404dn, impression recto-verso automatique. Vendue avec 5 toners compatibles neufs. Parfaite pour bureau ou PME. Compteur: 12000 pages.',
    type_echange: 'Troc',
    categorie: 'Informatique',
    prix: 0,
    devise: 'XOF',
    pays: 'Ghana',
    ville: 'Accra',
    tel: '+233 24 567 8901',
    photo_url: 'https://images.unsplash.com/photo-1612815154858-60aa4c59eaa6?w=800',
    user: { uid: 'u5', email: 'kwame.asante@email.gh', nom: 'Asante', prenom: 'Kwame' },
    created_at: new Date('2026-01-05')
  },
  {
    id: 'ann-6',
    titre: 'Serveur Dell PowerEdge R740 - Configuration pro',
    description: 'Serveur rack Dell PowerEdge R740, 2x Xeon Gold 6248, 256Go RAM DDR4, 4x SSD 960Go en RAID 10. Idéal pour entreprise ou datacenter. Installation et configuration incluses sur Abuja.',
    type_echange: 'Vente',
    categorie: 'Informatique',
    prix: 3500,
    devise: 'USD',
    pays: 'Nigeria',
    ville: 'Abuja',
    tel: '+234 803 456 7890',
    photo_url: 'https://images.unsplash.com/photo-1558494949-ef010cbdcc31?w=800',
    user: { uid: 'u6', email: 'chidi.okafor@email.ng', nom: 'Okafor', prenom: 'Chidi' },
    created_at: new Date('2026-01-12')
  },
  // Immobilier
  {
    id: 'ann-7',
    titre: 'Terrain constructible 500m² - Zone résidentielle',
    description: 'Terrain viabilisé de 500m² dans quartier résidentiel calme. Eau, électricité et assainissement en bordure. Titre foncier disponible. Proche écoles et commerces. Négociable.',
    type_echange: 'Vente',
    categorie: 'Immobilier',
    prix: 25000000,
    devise: 'XOF',
    pays: 'Burkina Faso',
    ville: 'Ouagadougou',
    tel: '+226 70 123 456',
    photo_url: 'https://images.unsplash.com/photo-1500382017468-9049fed747ef?w=800',
    user: { uid: 'u7', email: 'abdoul.kabore@email.bf', nom: 'Kaboré', prenom: 'Abdoul' },
    created_at: new Date('2026-01-02')
  },
  {
    id: 'ann-8',
    titre: 'Appartement F3 meublé - Location longue durée',
    description: 'Bel appartement F3 entièrement meublé et équipé. Cuisine américaine, salon spacieux, 2 chambres climatisées. Immeuble sécurisé avec gardien. Loyer mensuel, charges comprises.',
    type_echange: 'Vente',
    categorie: 'Immobilier',
    prix: 350000,
    devise: 'XOF',
    pays: 'Côte d\'Ivoire',
    ville: 'Abidjan',
    tel: '+225 05 678 90 12',
    photo_url: 'https://images.unsplash.com/photo-1502672260266-1c1ef2d93688?w=800',
    user: { uid: 'u8', email: 'marie.kouame@email.ci', nom: 'Kouamé', prenom: 'Marie' },
    created_at: new Date('2026-01-14')
  },
  // Voitures
  {
    id: 'ann-9',
    titre: 'Toyota Hilux 2019 - Double cabine 4x4',
    description: 'Toyota Hilux double cabine 4x4, diesel 2.4L, 85000 km. Carnet d\'entretien complet chez Toyota. Climatisation, vitres électriques, Bluetooth. Véhicule très propre, pas de choc.',
    type_echange: 'Vente',
    categorie: 'Voitures',
    prix: 18500000,
    devise: 'XOF',
    pays: 'Sénégal',
    ville: 'Dakar',
    tel: '+221 77 890 12 34',
    photo_url: 'https://images.unsplash.com/photo-1559416523-140ddc3d238c?w=800',
    user: { uid: 'u9', email: 'ibrahima.ba@email.sn', nom: 'Ba', prenom: 'Ibrahima' },
    created_at: new Date('2026-01-16')
  },
  {
    id: 'ann-10',
    titre: 'Peugeot 308 2020 - Essence automatique',
    description: 'Peugeot 308 GT Line, essence 1.2L PureTech, boîte automatique 8 vitesses. 45000 km, première main. Full options: toit panoramique, caméra 360°, sièges cuir chauffants.',
    type_echange: 'Troc',
    categorie: 'Voitures',
    prix: 0,
    devise: 'EUR',
    pays: 'Gabon',
    ville: 'Libreville',
    tel: '+241 06 789 0123',
    photo_url: 'https://images.unsplash.com/photo-1549317661-bd32c8ce0db2?w=800',
    user: { uid: 'u10', email: 'jean.mbou@email.ga', nom: 'Mbou', prenom: 'Jean' },
    created_at: new Date('2026-01-11')
  },
  // Électronique
  {
    id: 'ann-11',
    titre: 'iPhone 15 Pro Max 256Go - Neuf sous blister',
    description: 'iPhone 15 Pro Max 256Go couleur Titane Naturel. Neuf, jamais ouvert, sous blister d\'origine Apple. Facture et garantie internationale. Livraison gratuite sur Dakar.',
    type_echange: 'Vente',
    categorie: 'Electronique',
    prix: 850000,
    devise: 'XOF',
    pays: 'Sénégal',
    ville: 'Dakar',
    tel: '+221 76 345 67 89',
    photo_url: 'https://images.unsplash.com/photo-1695048133142-1a20484d2569?w=800',
    user: { uid: 'u11', email: 'moussa.fall@email.sn', nom: 'Fall', prenom: 'Moussa' },
    created_at: new Date('2026-01-20')
  },
  {
    id: 'ann-12',
    titre: 'TV Samsung 65" QLED 4K - Smart TV',
    description: 'Téléviseur Samsung QLED 65 pouces, résolution 4K UHD, Smart TV Tizen. Acheté il y a 6 mois, parfait état. Télécommande originale et pied inclus. Cause déménagement.',
    type_echange: 'Vente',
    categorie: 'Electronique',
    prix: 450000,
    devise: 'XOF',
    pays: 'Mali',
    ville: 'Bamako',
    tel: '+223 76 012 3456',
    photo_url: 'https://images.unsplash.com/photo-1593359677879-a4bb92f829d1?w=800',
    user: { uid: 'u12', email: 'amadou.coulibaly@email.ml', nom: 'Coulibaly', prenom: 'Amadou' },
    created_at: new Date('2026-01-17')
  },
  // Formation
  {
    id: 'ann-13',
    titre: 'Formation Développement Web Full-Stack - 6 mois',
    description: 'Formation intensive en développement web: HTML/CSS, JavaScript, React, Node.js, bases de données. 6 mois de formation + 2 mois de stage en entreprise. Certification reconnue. Places limitées.',
    type_echange: 'Vente',
    categorie: 'Formation',
    prix: 1500000,
    devise: 'XOF',
    pays: 'Côte d\'Ivoire',
    ville: 'Abidjan',
    tel: '+225 07 890 12 34',
    photo_url: 'https://images.unsplash.com/photo-1517694712202-14dd9538aa97?w=800',
    user: { uid: 'u13', email: 'techacademy@email.ci', nom: 'Tech Academy', prenom: 'Abidjan' },
    created_at: new Date('2026-01-03')
  },
  {
    id: 'ann-14',
    titre: 'Coaching Business - Accompagnement création entreprise',
    description: 'Accompagnement personnalisé pour créer votre entreprise: business plan, étude de marché, recherche de financement, aspects juridiques. 3 mois de coaching individuel avec expert certifié.',
    type_echange: 'Vente',
    categorie: 'Formation',
    prix: 500,
    devise: 'EUR',
    pays: 'Ghana',
    ville: 'Accra',
    tel: '+233 20 123 4567',
    photo_url: 'https://images.unsplash.com/photo-1552664730-d307ca884978?w=800',
    user: { uid: 'u14', email: 'sarah.mensah@email.gh', nom: 'Mensah', prenom: 'Sarah' },
    created_at: new Date('2026-01-07')
  },
  // Don
  {
    id: 'ann-15',
    titre: 'Livres scolaires primaire - Don pour école',
    description: 'Collection de manuels scolaires pour classes de CP à CM2: français, mathématiques, sciences. Environ 200 livres en bon état. Don réservé aux écoles ou associations éducatives.',
    type_echange: 'Don',
    categorie: 'Formation',
    prix: 0,
    devise: 'XOF',
    pays: 'Burkina Faso',
    ville: 'Bobo-Dioulasso',
    tel: '+226 76 543 210',
    photo_url: 'https://images.unsplash.com/photo-1524995997946-a1c2e315a42f?w=800',
    user: { uid: 'u15', email: 'association.savoir@email.bf', nom: 'Association', prenom: 'Savoir Plus' },
    created_at: new Date('2026-01-09')
  }
]

// Fonctions utilitaires
export const getAllAnnonces = (): Annonce[] => {
  return [...annoncesMock].sort((a, b) => b.created_at.getTime() - a.created_at.getTime())
}

export const getAnnonceById = (id: string): Annonce | undefined => {
  return annoncesMock.find(a => a.id === id)
}

export const rechercherAnnonces = (filtres: Partial<FiltresAnnonce>): Annonce[] => {
  let result = [...annoncesMock]

  // Filtre par catégorie
  if (filtres.categorie && filtres.categorie !== 'Tout') {
    result = result.filter(a => a.categorie === filtres.categorie)
  }

  // Filtre par types d'échange
  if (filtres.typesEchange && filtres.typesEchange.length > 0) {
    result = result.filter(a => filtres.typesEchange!.includes(a.type_echange))
  }

  // Filtre par prix min
  if (filtres.prixMin !== null && filtres.prixMin !== undefined) {
    result = result.filter(a => a.prix >= filtres.prixMin!)
  }

  // Filtre par prix max
  if (filtres.prixMax !== null && filtres.prixMax !== undefined) {
    result = result.filter(a => a.prix <= filtres.prixMax!)
  }

  // Recherche textuelle
  if (filtres.recherche && filtres.recherche.trim() !== '') {
    const searchLower = filtres.recherche.toLowerCase()
    result = result.filter(a =>
      a.titre.toLowerCase().includes(searchLower) ||
      a.description.toLowerCase().includes(searchLower) ||
      a.pays.toLowerCase().includes(searchLower) ||
      a.ville.toLowerCase().includes(searchLower)
    )
  }

  // Tri
  if (filtres.tri === 'recent' || !filtres.tri) {
    result.sort((a, b) => b.created_at.getTime() - a.created_at.getTime())
  } else if (filtres.tri === 'price-asc') {
    result.sort((a, b) => a.prix - b.prix)
  } else if (filtres.tri === 'price-desc') {
    result.sort((a, b) => b.prix - a.prix)
  }

  return result
}

export const getAnnoncesByCategorie = (categorie: Categorie): Annonce[] => {
  return annoncesMock.filter(a => a.categorie === categorie)
}

export const getCountByType = (type: TypeEchange, annonces: Annonce[]): number => {
  return annonces.filter(a => a.type_echange === type).length
}

// Formatage
export const formatDate = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  }).format(new Date(date))
}

export const formatDateCourte = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(new Date(date))
}

export const formatPrix = (prix: number, devise: Devise): string => {
  if (prix === 0) return 'Gratuit'

  const deviseInfo = DEVISES.find(d => d.value === devise)
  const symbol = deviseInfo?.symbol || devise

  if (devise === 'XOF' || devise === 'NGN') {
    return `${prix.toLocaleString('fr-FR')} ${symbol}`
  } else {
    return `${symbol}${prix.toLocaleString('fr-FR')}`
  }
}

export const getTypeEchangeColor = (type: TypeEchange): string => {
  const typeInfo = TYPES_ECHANGE.find(t => t.value === type)
  return typeInfo?.color || 'bg-gray-100 text-gray-700'
}

export const getCategorieLabel = (categorie: Categorie): string => {
  const cat = CATEGORIES.find(c => c.key === categorie)
  return cat?.label || categorie
}

// Stats
export const getStatsAnnonces = () => {
  const total = annoncesMock.length
  const ventes = getCountByType('Vente', annoncesMock)
  const trocs = getCountByType('Troc', annoncesMock)
  const dons = getCountByType('Don', annoncesMock)

  return {
    total,
    ventes,
    trocs,
    dons
  }
}

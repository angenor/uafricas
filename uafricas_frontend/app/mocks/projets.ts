// Donnees mock pour la page Financer un Projet

// Types
export type StatutProjet = 'valide' | 'en_cours' | 'termine' | 'soumis' | 'brouillon' | 'rejete'
export type DeviseProjet = 'XOF' | 'XAF' | 'EUR' | 'USD'

// Interfaces
export interface Country {
  value: string
  label: string
}

export interface UserInfoProjet {
  nom: string
  prenom: string
  photoURL?: string
}

export interface Projet {
  id: string
  titre: string
  description: string
  resume?: string
  organisation: string
  pays: string
  ville?: string
  coutTotal: number
  devise: DeviseProjet
  duree: string
  dateDebutSouhaitee?: Date
  statut: StatutProjet
  objectifs: string[]
  imageCouverture?: string
  userId?: string
  userInfo?: UserInfoProjet
  dateCreation: Date
  dateMiseAJour?: Date
}

export interface FiltresProjet {
  pays: string
  budgetMax: string
  duree: string
  recherche: string
  sortBy: 'dateCreation' | 'titre' | 'coutTotal' | 'coutTotal-desc'
}

export interface StatistiquesProjet {
  total: number
  valides: number
  enCours: number
  termines: number
}

// Liste des pays africains
export const paysAfricains: Country[] = [
  { value: '', label: 'Tous les pays' },
  { value: 'Senegal', label: 'Sénégal' },
  { value: 'Cote_d_Ivoire', label: "Côte d'Ivoire" },
  { value: 'Cameroun', label: 'Cameroun' },
  { value: 'Mali', label: 'Mali' },
  { value: 'Burkina_Faso', label: 'Burkina Faso' },
  { value: 'Benin', label: 'Bénin' },
  { value: 'Togo', label: 'Togo' },
  { value: 'Niger', label: 'Niger' },
  { value: 'Guinee', label: 'Guinée' },
  { value: 'Congo', label: 'Congo' },
  { value: 'Congo_democratique', label: 'RD Congo' },
  { value: 'Gabon', label: 'Gabon' },
  { value: 'Rwanda', label: 'Rwanda' },
  { value: 'Maroc', label: 'Maroc' },
  { value: 'Tunisie', label: 'Tunisie' },
  { value: 'Nigeria', label: 'Nigeria' },
  { value: 'Ghana', label: 'Ghana' },
  { value: 'Kenya', label: 'Kenya' },
]

// Tranches de budget
export const budgets: { value: string; label: string }[] = [
  { value: '', label: 'Tous les budgets' },
  { value: '1000000', label: 'Moins de 1 million XOF' },
  { value: '5000000', label: 'Moins de 5 millions XOF' },
  { value: '10000000', label: 'Moins de 10 millions XOF' },
  { value: '50000000', label: 'Moins de 50 millions XOF' },
  { value: '100000000', label: 'Moins de 100 millions XOF' },
]

// Durees de projet
export const durees: { value: string; label: string }[] = [
  { value: '', label: 'Toutes les durées' },
  { value: 'court', label: 'Court terme (< 6 mois)' },
  { value: 'moyen', label: 'Moyen terme (6-24 mois)' },
  { value: 'long', label: 'Long terme (> 24 mois)' },
]

// Options de tri
export const sortOptions: { value: string; label: string }[] = [
  { value: 'dateCreation', label: 'Plus récents' },
  { value: 'titre', label: 'Alphabétique (A-Z)' },
  { value: 'coutTotal', label: 'Budget croissant' },
  { value: 'coutTotal-desc', label: 'Budget décroissant' },
]

// Labels des statuts
export const statutsLabels: Record<StatutProjet, { label: string; color: string }> = {
  valide: { label: 'Validé', color: 'bg-green-100 text-green-800' },
  en_cours: { label: 'En cours', color: 'bg-blue-100 text-blue-800' },
  termine: { label: 'Terminé', color: 'bg-gray-100 text-gray-800' },
  soumis: { label: 'Soumis', color: 'bg-yellow-100 text-yellow-800' },
  brouillon: { label: 'Brouillon', color: 'bg-orange-100 text-orange-800' },
  rejete: { label: 'Rejeté', color: 'bg-red-100 text-red-800' },
}

// Labels des devises
export const devisesLabels: Record<DeviseProjet, string> = {
  XOF: 'XOF (FCFA)',
  XAF: 'XAF (FCFA)',
  EUR: 'EUR (€)',
  USD: 'USD ($)',
}

// Projets mock
export const projetsMock: Projet[] = [
  {
    id: '1',
    titre: 'Ferme Agro-écologique Moderne',
    description: "Création d'une ferme agro-écologique de 50 hectares pour promouvoir l'agriculture durable et la sécurité alimentaire. Le projet inclut l'installation de systèmes d'irrigation goutte-à-goutte, la formation des agriculteurs locaux et la mise en place de circuits courts de distribution.",
    resume: "Ferme durable de 50 hectares avec irrigation moderne et formation agricole.",
    organisation: 'Coopérative Agricole du Sahel',
    pays: 'Senegal',
    ville: 'Thiès',
    coutTotal: 45000000,
    devise: 'XOF',
    duree: '24 mois',
    dateDebutSouhaitee: new Date('2024-06-01'),
    statut: 'valide',
    objectifs: [
      'Cultiver 50 hectares de terres arables',
      'Former 200 agriculteurs aux techniques durables',
      'Créer 50 emplois permanents',
      'Produire 500 tonnes de légumes par an',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1500382017468-9049fed747ef?w=800&h=600&fit=crop',
    userId: 'user1',
    userInfo: {
      nom: 'Diallo',
      prenom: 'Amadou',
      photoURL: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2024-01-15'),
    dateMiseAJour: new Date('2024-03-20'),
  },
  {
    id: '2',
    titre: 'Centre de Formation Numérique',
    description: "Construction et équipement d'un centre de formation aux métiers du numérique destiné aux jeunes déscolarisés. Le centre proposera des formations en développement web, design graphique et marketing digital.",
    resume: "Centre de formation numérique pour jeunes avec équipements modernes.",
    organisation: 'Tech for Africa',
    pays: 'Cote_d_Ivoire',
    ville: 'Abidjan',
    coutTotal: 75000000,
    devise: 'XOF',
    duree: '18 mois',
    dateDebutSouhaitee: new Date('2024-04-01'),
    statut: 'valide',
    objectifs: [
      'Construire un bâtiment de 500m²',
      'Équiper 3 salles informatiques de 30 postes',
      'Former 500 jeunes en 3 ans',
      'Atteindre 70% d\'insertion professionnelle',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1517245386807-bb43f82c33c4?w=800&h=600&fit=crop',
    userId: 'user2',
    userInfo: {
      nom: 'Kouassi',
      prenom: 'Marie',
      photoURL: 'https://images.unsplash.com/photo-1531123897727-8f129e1688ce?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2024-02-10'),
    dateMiseAJour: new Date('2024-03-15'),
  },
  {
    id: '3',
    titre: 'Clinique Mobile Rurale',
    description: "Mise en place d'une clinique mobile pour desservir les zones rurales isolées. Le projet comprend l'achat d'un véhicule médicalisé, l'équipement médical et le recrutement d'une équipe de santé.",
    resume: "Clinique mobile pour soins de santé dans les zones rurales isolées.",
    organisation: 'Santé Pour Tous Mali',
    pays: 'Mali',
    ville: 'Ségou',
    coutTotal: 35000000,
    devise: 'XOF',
    duree: '12 mois',
    dateDebutSouhaitee: new Date('2024-05-01'),
    statut: 'en_cours',
    objectifs: [
      'Acquérir un véhicule médicalisé équipé',
      'Desservir 20 villages par mois',
      'Réaliser 5000 consultations par an',
      'Former 10 agents de santé communautaires',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1584982751601-97dcc096659c?w=800&h=600&fit=crop',
    userId: 'user3',
    userInfo: {
      nom: 'Traoré',
      prenom: 'Ibrahim',
      photoURL: 'https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2024-01-20'),
    dateMiseAJour: new Date('2024-04-10'),
  },
  {
    id: '4',
    titre: 'Usine de Transformation de Mangues',
    description: "Installation d'une unité de transformation de mangues pour produire des jus, confitures et mangues séchées. Le projet vise à réduire les pertes post-récolte et à créer de la valeur ajoutée locale.",
    resume: "Usine de transformation de mangues en jus, confitures et fruits séchés.",
    organisation: 'Agro-Industrie Burkina',
    pays: 'Burkina_Faso',
    ville: 'Bobo-Dioulasso',
    coutTotal: 120000000,
    devise: 'XOF',
    duree: '30 mois',
    dateDebutSouhaitee: new Date('2024-07-01'),
    statut: 'valide',
    objectifs: [
      'Construire une usine de 1000m²',
      'Transformer 500 tonnes de mangues par saison',
      'Créer 80 emplois dont 60% pour les femmes',
      'Exporter 30% de la production',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1553279768-865429fa0078?w=800&h=600&fit=crop',
    userId: 'user4',
    userInfo: {
      nom: 'Ouédraogo',
      prenom: 'Fatou',
      photoURL: 'https://images.unsplash.com/photo-1489424731084-a5d8b219a5bb?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2024-03-05'),
    dateMiseAJour: new Date('2024-04-01'),
  },
  {
    id: '5',
    titre: 'École Primaire Écologique',
    description: "Construction d'une école primaire utilisant des matériaux locaux et des énergies renouvelables. L'école sera équipée de panneaux solaires et d'un système de récupération d'eau de pluie.",
    resume: "École primaire écologique avec panneaux solaires et récupération d'eau.",
    organisation: 'Éducation Verte Cameroun',
    pays: 'Cameroun',
    ville: 'Bafoussam',
    coutTotal: 55000000,
    devise: 'XOF',
    duree: '18 mois',
    dateDebutSouhaitee: new Date('2024-09-01'),
    statut: 'valide',
    objectifs: [
      'Construire 6 salles de classe',
      'Accueillir 300 élèves',
      'Installer 20 panneaux solaires',
      'Créer un jardin pédagogique',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1580582932707-520aed937b7b?w=800&h=600&fit=crop',
    userId: 'user5',
    userInfo: {
      nom: 'Nguema',
      prenom: 'Paul',
      photoURL: 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2024-02-28'),
    dateMiseAJour: new Date('2024-03-25'),
  },
  {
    id: '6',
    titre: 'Centrale Solaire Communautaire',
    description: "Installation d'une centrale solaire de 100 kWc pour électrifier 5 villages. Le projet inclut la formation de techniciens locaux pour la maintenance et la gestion communautaire.",
    resume: "Centrale solaire de 100 kWc pour électrifier 5 villages.",
    organisation: 'Énergie Rurale Niger',
    pays: 'Niger',
    ville: 'Maradi',
    coutTotal: 85000000,
    devise: 'XOF',
    duree: '15 mois',
    dateDebutSouhaitee: new Date('2024-06-15'),
    statut: 'en_cours',
    objectifs: [
      'Installer 100 kWc de panneaux solaires',
      'Électrifier 500 foyers',
      'Former 10 techniciens locaux',
      'Réduire les émissions de CO2 de 200 tonnes/an',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1509391366360-2e959784a276?w=800&h=600&fit=crop',
    userId: 'user6',
    userInfo: {
      nom: 'Sani',
      prenom: 'Moussa',
      photoURL: 'https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2024-01-08'),
    dateMiseAJour: new Date('2024-04-05'),
  },
  {
    id: '7',
    titre: 'Pisciculture Durable',
    description: "Création d'une ferme piscicole intégrée combinant élevage de tilapias et culture maraîchère. Le système aquaponique permet une production durable et écologique.",
    resume: "Ferme piscicole aquaponique combinant tilapias et maraîchage.",
    organisation: 'Aqua-Farm Bénin',
    pays: 'Benin',
    ville: 'Porto-Novo',
    coutTotal: 28000000,
    devise: 'XOF',
    duree: '12 mois',
    dateDebutSouhaitee: new Date('2024-04-15'),
    statut: 'termine',
    objectifs: [
      'Installer 10 bassins de 50m³',
      'Produire 5 tonnes de tilapia par an',
      'Cultiver 1 hectare de légumes',
      'Former 15 jeunes entrepreneurs',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1544551763-46a013bb70d5?w=800&h=600&fit=crop',
    userId: 'user7',
    userInfo: {
      nom: 'Adéyèmi',
      prenom: 'Kokou',
      photoURL: 'https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2023-06-10'),
    dateMiseAJour: new Date('2024-02-20'),
  },
  {
    id: '8',
    titre: 'Bibliothèque Numérique Mobile',
    description: "Création d'une bibliothèque numérique itinérante équipée de tablettes et d'une connexion satellite pour promouvoir la lecture et l'accès au savoir dans les zones reculées.",
    resume: "Bibliothèque numérique itinérante avec tablettes et connexion satellite.",
    organisation: 'Lire pour Grandir',
    pays: 'Togo',
    ville: 'Kara',
    coutTotal: 18000000,
    devise: 'XOF',
    duree: '10 mois',
    dateDebutSouhaitee: new Date('2024-08-01'),
    statut: 'valide',
    objectifs: [
      'Équiper un véhicule avec 50 tablettes',
      'Visiter 30 villages par mois',
      'Toucher 10000 bénéficiaires par an',
      'Créer 100 clubs de lecture',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1481627834876-b7833e8f5570?w=800&h=600&fit=crop',
    userId: 'user8',
    userInfo: {
      nom: 'Mensah',
      prenom: 'Akossiwa',
      photoURL: 'https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2024-03-12'),
    dateMiseAJour: new Date('2024-04-08'),
  },
  {
    id: '9',
    titre: 'Centre Artisanal Féminin',
    description: "Création d'un centre de formation et de production artisanale pour les femmes. Le centre proposera des formations en tissage, poterie et couture avec un espace de vente.",
    resume: "Centre de formation artisanale pour femmes avec espace de vente.",
    organisation: 'Femmes Créatives Ghana',
    pays: 'Ghana',
    ville: 'Kumasi',
    coutTotal: 32000000,
    devise: 'XOF',
    duree: '14 mois',
    dateDebutSouhaitee: new Date('2024-05-15'),
    statut: 'en_cours',
    objectifs: [
      'Construire un atelier de 300m²',
      'Former 150 femmes par an',
      'Créer 40 micro-entreprises',
      'Générer 50 millions XOF de chiffre d\'affaires annuel',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1559827260-dc66d52bef19?w=800&h=600&fit=crop',
    userId: 'user9',
    userInfo: {
      nom: 'Asante',
      prenom: 'Ama',
      photoURL: 'https://images.unsplash.com/photo-1580489944761-15a19d654956?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2024-02-05'),
    dateMiseAJour: new Date('2024-03-30'),
  },
  {
    id: '10',
    titre: 'Application Mobile de Télémédecine',
    description: "Développement d'une application mobile de télémédecine permettant aux patients des zones rurales de consulter des médecins à distance et d'accéder à des conseils de santé.",
    resume: "Application mobile de télémédecine pour consultations à distance.",
    organisation: 'E-Health Kenya',
    pays: 'Kenya',
    ville: 'Nairobi',
    coutTotal: 42000000,
    devise: 'XOF',
    duree: '12 mois',
    dateDebutSouhaitee: new Date('2024-03-01'),
    statut: 'valide',
    objectifs: [
      'Développer une application iOS et Android',
      'Recruter 50 médecins partenaires',
      'Réaliser 20000 téléconsultations la première année',
      'Couvrir 5 comtés ruraux',
    ],
    imageCouverture: 'https://images.unsplash.com/photo-1576091160399-112ba8d25d1d?w=800&h=600&fit=crop',
    userId: 'user10',
    userInfo: {
      nom: 'Mwangi',
      prenom: 'Grace',
      photoURL: 'https://images.unsplash.com/photo-1573496359142-b8d87734a5a2?w=100&h=100&fit=crop',
    },
    dateCreation: new Date('2024-01-25'),
    dateMiseAJour: new Date('2024-04-12'),
  },
]

// Fonctions utilitaires
export const getProjetById = (id: string): Projet | undefined => {
  return projetsMock.find((projet) => projet.id === id)
}

export const getPaysLabel = (value: string): string => {
  const pays = paysAfricains.find((p) => p.value === value)
  return pays?.label || value
}

export const getStatutInfo = (statut: StatutProjet): { label: string; color: string } => {
  return statutsLabels[statut] || { label: statut, color: 'bg-gray-100 text-gray-800' }
}

export const formatCurrency = (amount: number, devise: DeviseProjet = 'XOF'): string => {
  const symbols: Record<DeviseProjet, string> = {
    XOF: 'FCFA',
    XAF: 'FCFA',
    EUR: '€',
    USD: '$',
  }

  return new Intl.NumberFormat('fr-FR', {
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(amount) + ' ' + symbols[devise]
}

export const formatDate = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  }).format(date)
}

export const getInitiales = (nom?: string, prenom?: string): string => {
  const n = nom?.charAt(0).toUpperCase() || ''
  const p = prenom?.charAt(0).toUpperCase() || ''
  return p + n
}

export const getDureeCategory = (duree: string): 'court' | 'moyen' | 'long' => {
  const mois = parseInt(duree)
  if (isNaN(mois)) {
    if (duree.includes('mois')) {
      const num = parseInt(duree.split(' ')[0])
      if (num < 6) return 'court'
      if (num <= 24) return 'moyen'
      return 'long'
    }
    return 'moyen'
  }
  if (mois < 6) return 'court'
  if (mois <= 24) return 'moyen'
  return 'long'
}

export const getStatistiques = (): StatistiquesProjet => {
  return {
    total: projetsMock.length,
    valides: projetsMock.filter((p) => p.statut === 'valide').length,
    enCours: projetsMock.filter((p) => p.statut === 'en_cours').length,
    termines: projetsMock.filter((p) => p.statut === 'termine').length,
  }
}

export const rechercherProjets = (filtres: Partial<FiltresProjet>): Projet[] => {
  let result = [...projetsMock]

  // Filtre par recherche textuelle
  if (filtres.recherche) {
    const search = filtres.recherche.toLowerCase()
    result = result.filter(
      (p) =>
        p.titre.toLowerCase().includes(search) ||
        p.description.toLowerCase().includes(search) ||
        p.organisation.toLowerCase().includes(search)
    )
  }

  // Filtre par pays
  if (filtres.pays) {
    result = result.filter((p) => p.pays === filtres.pays)
  }

  // Filtre par budget max
  if (filtres.budgetMax) {
    const max = parseInt(filtres.budgetMax)
    result = result.filter((p) => p.coutTotal <= max)
  }

  // Filtre par durée
  if (filtres.duree) {
    result = result.filter((p) => getDureeCategory(p.duree) === filtres.duree)
  }

  // Tri
  if (filtres.sortBy) {
    switch (filtres.sortBy) {
      case 'dateCreation':
        result.sort((a, b) => b.dateCreation.getTime() - a.dateCreation.getTime())
        break
      case 'titre':
        result.sort((a, b) => a.titre.localeCompare(b.titre))
        break
      case 'coutTotal':
        result.sort((a, b) => a.coutTotal - b.coutTotal)
        break
      case 'coutTotal-desc':
        result.sort((a, b) => b.coutTotal - a.coutTotal)
        break
    }
  }

  return result
}

// Donnees mock pour la page Experts

// Interfaces
export interface Country {
  value: string
  label: string
}

export interface Profile {
  id: string
  label: string
  icon: string
  color: string
}

export interface ExpertExpertise {
  domaine: string
  biographie: string
  nbAnneesExperience: number
  rating: number
  portfolio?: string
  statut: 'valide' | 'en_attente' | 'refuse'
}

export interface Expert {
  id: string
  nom: string
  prenom: string
  photoURL: string
  pays: string
  ville?: string
  email: string
  expertiseInfo: ExpertExpertise
  situationProfessionnelle: string[]
  dateInscription: Date
  dateDerniereMiseAJour?: Date
}

// Categories d'expertise
export const categories: string[] = [
  'Tout',
  'Agriculture',
  'Informatique',
  'Électronique',
  'Immobilier',
  'Mécanique',
  'Santé',
  'Éducation',
  'Finance',
]

// Profils professionnels
export const profiles: Profile[] = [
  {
    id: 'tous',
    label: 'Tous les profils',
    icon: 'fas fa-users',
    color: 'gray',
  },
  {
    id: 'recherche_emploi',
    label: "En recherche d'emploi",
    icon: 'fas fa-search',
    color: 'red',
  },
  {
    id: 'en_emploi',
    label: 'En Emploi',
    icon: 'fas fa-briefcase',
    color: 'green',
  },
  {
    id: 'consultance',
    label: 'Consultant',
    icon: 'fas fa-user-tie',
    color: 'blue',
  },
  {
    id: 'volontariat_expertise',
    label: "Volontariat (partage d'expertise)",
    icon: 'fas fa-heart',
    color: 'purple',
  },
  {
    id: 'recherche_nouvelles_opportunites',
    label: 'En Emploi mais recherche de nouvelles opportunités',
    icon: 'fas fa-exchange-alt',
    color: 'orange',
  },
]

// Liste des pays
export const countries: Country[] = [
  { value: '', label: 'Tous les territoires' },
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
  { value: 'Algerie', label: 'Algérie' },
  { value: 'Nigeria', label: 'Nigeria' },
  { value: 'Ghana', label: 'Ghana' },
  { value: 'Kenya', label: 'Kenya' },
  { value: 'Afrique_du_sud', label: 'Afrique du Sud' },
  { value: 'France', label: 'France' },
  { value: 'Belgique', label: 'Belgique' },
  { value: 'Canada', label: 'Canada' },
  { value: 'Etats_Unis', label: 'États-Unis' },
]

// Experts mock
export const expertsMock: Expert[] = [
  {
    id: '1',
    nom: 'Diallo',
    prenom: 'Mamadou',
    photoURL: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=400&fit=crop',
    pays: 'Senegal',
    ville: 'Dakar',
    email: 'mamadou.diallo@example.com',
    expertiseInfo: {
      domaine: 'Informatique',
      biographie: "Expert en développement web et mobile avec plus de 10 ans d'expérience. Spécialisé dans les technologies modernes comme React, Vue.js et Node.js. Passionné par l'innovation technologique en Afrique.",
      nbAnneesExperience: 12,
      rating: 4.8,
      statut: 'valide',
    },
    situationProfessionnelle: ['consultance', 'volontariat_expertise'],
    dateInscription: new Date('2023-01-15'),
    dateDerniereMiseAJour: new Date('2024-11-20'),
  },
  {
    id: '2',
    nom: 'Kouassi',
    prenom: 'Awa',
    photoURL: 'https://images.unsplash.com/photo-1531123897727-8f129e1688ce?w=400&h=400&fit=crop',
    pays: 'Cote_d_Ivoire',
    ville: 'Abidjan',
    email: 'awa.kouassi@example.com',
    expertiseInfo: {
      domaine: 'Finance',
      biographie: "Experte en finance et gestion d'entreprise. Accompagne les startups africaines dans leur développement financier et leur levée de fonds. MBA de HEC Paris.",
      nbAnneesExperience: 8,
      rating: 4.9,
      statut: 'valide',
    },
    situationProfessionnelle: ['en_emploi', 'recherche_nouvelles_opportunites'],
    dateInscription: new Date('2023-03-20'),
    dateDerniereMiseAJour: new Date('2024-12-01'),
  },
  {
    id: '3',
    nom: 'Ndiaye',
    prenom: 'Ibrahima',
    photoURL: 'https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400&h=400&fit=crop',
    pays: 'Senegal',
    ville: 'Saint-Louis',
    email: 'ibrahima.ndiaye@example.com',
    expertiseInfo: {
      domaine: 'Agriculture',
      biographie: "Ingénieur agronome spécialisé dans l'agriculture durable et l'agroécologie. Travaille sur des projets de sécurité alimentaire en Afrique de l'Ouest depuis 15 ans.",
      nbAnneesExperience: 15,
      rating: 4.7,
      statut: 'valide',
    },
    situationProfessionnelle: ['en_emploi', 'volontariat_expertise'],
    dateInscription: new Date('2022-06-10'),
    dateDerniereMiseAJour: new Date('2024-10-15'),
  },
  {
    id: '4',
    nom: 'Kamara',
    prenom: 'Fatoumata',
    photoURL: 'https://images.unsplash.com/photo-1489424731084-a5d8b219a5bb?w=400&h=400&fit=crop',
    pays: 'Mali',
    ville: 'Bamako',
    email: 'fatoumata.kamara@example.com',
    expertiseInfo: {
      domaine: 'Santé',
      biographie: "Médecin spécialiste en santé publique. Coordonne des programmes de vaccination et de lutte contre les maladies tropicales. Docteur de l'Université de Genève.",
      nbAnneesExperience: 10,
      rating: 4.9,
      statut: 'valide',
    },
    situationProfessionnelle: ['en_emploi'],
    dateInscription: new Date('2023-02-28'),
    dateDerniereMiseAJour: new Date('2024-11-30'),
  },
  {
    id: '5',
    nom: 'Ouedraogo',
    prenom: 'Pascal',
    photoURL: 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400&h=400&fit=crop',
    pays: 'Burkina_Faso',
    ville: 'Ouagadougou',
    email: 'pascal.ouedraogo@example.com',
    expertiseInfo: {
      domaine: 'Éducation',
      biographie: "Expert en pédagogie et formation professionnelle. Développe des programmes éducatifs innovants adaptés au contexte africain. Ancien directeur d'école.",
      nbAnneesExperience: 20,
      rating: 4.6,
      statut: 'valide',
    },
    situationProfessionnelle: ['consultance', 'recherche_nouvelles_opportunites'],
    dateInscription: new Date('2022-09-01'),
    dateDerniereMiseAJour: new Date('2024-08-20'),
  },
  {
    id: '6',
    nom: 'Mensah',
    prenom: 'Kofi',
    photoURL: 'https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=400&h=400&fit=crop',
    pays: 'Ghana',
    ville: 'Accra',
    email: 'kofi.mensah@example.com',
    expertiseInfo: {
      domaine: 'Électronique',
      biographie: "Ingénieur en électronique et systèmes embarqués. Spécialisé dans les solutions IoT pour l'agriculture intelligente. Fondateur d'une startup tech à Accra.",
      nbAnneesExperience: 7,
      rating: 4.5,
      statut: 'valide',
    },
    situationProfessionnelle: ['en_emploi', 'consultance'],
    dateInscription: new Date('2023-05-15'),
    dateDerniereMiseAJour: new Date('2024-12-05'),
  },
  {
    id: '7',
    nom: 'Bah',
    prenom: 'Mariama',
    photoURL: 'https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400&h=400&fit=crop',
    pays: 'Guinee',
    ville: 'Conakry',
    email: 'mariama.bah@example.com',
    expertiseInfo: {
      domaine: 'Immobilier',
      biographie: "Experte en développement immobilier et urbanisme durable. Conseille les promoteurs et les collectivités sur les projets de construction écologique en Afrique.",
      nbAnneesExperience: 9,
      rating: 4.7,
      statut: 'valide',
    },
    situationProfessionnelle: ['recherche_emploi'],
    dateInscription: new Date('2024-01-10'),
    dateDerniereMiseAJour: new Date('2024-11-25'),
  },
  {
    id: '8',
    nom: 'Toure',
    prenom: 'Amadou',
    photoURL: 'https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400&h=400&fit=crop',
    pays: 'Mali',
    ville: 'Bamako',
    email: 'amadou.toure@example.com',
    expertiseInfo: {
      domaine: 'Mécanique',
      biographie: "Ingénieur mécanique spécialisé dans les énergies renouvelables. Conçoit des solutions solaires et éoliennes adaptées au contexte africain.",
      nbAnneesExperience: 11,
      rating: 4.8,
      statut: 'valide',
    },
    situationProfessionnelle: ['en_emploi', 'volontariat_expertise'],
    dateInscription: new Date('2022-11-20'),
    dateDerniereMiseAJour: new Date('2024-09-30'),
  },
  {
    id: '9',
    nom: 'Ndongo',
    prenom: 'Aicha',
    photoURL: 'https://images.unsplash.com/photo-1580489944761-15a19d654956?w=400&h=400&fit=crop',
    pays: 'Cameroun',
    ville: 'Douala',
    email: 'aicha.ndongo@example.com',
    expertiseInfo: {
      domaine: 'Informatique',
      biographie: "Data scientist et experte en intelligence artificielle. Développe des solutions IA pour résoudre des problématiques africaines. PhD en Machine Learning.",
      nbAnneesExperience: 6,
      rating: 4.9,
      statut: 'valide',
    },
    situationProfessionnelle: ['consultance', 'recherche_nouvelles_opportunites'],
    dateInscription: new Date('2023-07-05'),
    dateDerniereMiseAJour: new Date('2024-12-10'),
  },
  {
    id: '10',
    nom: 'Kone',
    prenom: 'Seydou',
    photoURL: 'https://images.unsplash.com/photo-1531427186611-ecfd6d936c79?w=400&h=400&fit=crop',
    pays: 'Cote_d_Ivoire',
    ville: 'Yamoussoukro',
    email: 'seydou.kone@example.com',
    expertiseInfo: {
      domaine: 'Agriculture',
      biographie: "Spécialiste en cultures vivrières et transformation agroalimentaire. Accompagne les coopératives agricoles dans leur modernisation et leur accès aux marchés.",
      nbAnneesExperience: 14,
      rating: 4.6,
      statut: 'valide',
    },
    situationProfessionnelle: ['volontariat_expertise'],
    dateInscription: new Date('2022-04-12'),
    dateDerniereMiseAJour: new Date('2024-07-18'),
  },
  {
    id: '11',
    nom: 'Diop',
    prenom: 'Ousmane',
    photoURL: 'https://images.unsplash.com/photo-1560250097-0b93528c311a?w=400&h=400&fit=crop',
    pays: 'Senegal',
    ville: 'Dakar',
    email: 'ousmane.diop@example.com',
    expertiseInfo: {
      domaine: 'Finance',
      biographie: "Expert en microfinance et inclusion financière. Travaille avec des institutions de microfinance pour développer des produits adaptés aux populations rurales.",
      nbAnneesExperience: 13,
      rating: 4.7,
      statut: 'valide',
    },
    situationProfessionnelle: ['en_emploi'],
    dateInscription: new Date('2022-08-25'),
    dateDerniereMiseAJour: new Date('2024-10-05'),
  },
  {
    id: '12',
    nom: 'Sanogo',
    prenom: 'Aminata',
    photoURL: 'https://images.unsplash.com/photo-1573496359142-b8d87734a5a2?w=400&h=400&fit=crop',
    pays: 'Burkina_Faso',
    ville: 'Bobo-Dioulasso',
    email: 'aminata.sanogo@example.com',
    expertiseInfo: {
      domaine: 'Santé',
      biographie: "Pharmacienne et experte en médecine traditionnelle africaine. Travaille sur la valorisation des plantes médicinales et leur intégration dans les systèmes de santé.",
      nbAnneesExperience: 8,
      rating: 4.8,
      statut: 'valide',
    },
    situationProfessionnelle: ['recherche_emploi', 'consultance'],
    dateInscription: new Date('2023-09-18'),
    dateDerniereMiseAJour: new Date('2024-11-12'),
  },
]

// Fonctions utilitaires
export const getExpertById = (id: string): Expert | undefined => {
  return expertsMock.find((expert) => expert.id === id)
}

export const getCountryLabel = (value: string): string => {
  const country = countries.find((c) => c.value === value)
  return country?.label || value
}

export const getProfileLabel = (id: string): string => {
  const profile = profiles.find((p) => p.id === id)
  return profile?.label || id
}

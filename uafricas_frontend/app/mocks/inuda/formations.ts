// Donnees mock pour les formations INUDA

export interface DureeEstimee {
  heures: number
  semaines?: number
}

export interface Tarification {
  gratuit: boolean
  prix: number
  prixReduit?: { montant: number; conditions: string }[]
}

export interface Capacite {
  maximum: number | null
  inscritsActuels: number
}

export interface Modalites {
  langue: string
  niveauRequis: 'debutant' | 'intermediaire' | 'avance' | 'tous_niveaux'
  certificationDisponible: boolean
  prerequis: string[]
  modulesCours?: ModuleCours[]
}

export interface ModuleCours {
  id: string
  titre: string
  description: string
  duree: number // en minutes
  ordre: number
}

export interface Objectifs {
  generaux: string[]
  specifiques: string[]
  competencesAcquises: string[]
}

export interface Formation {
  id: string
  titre: string
  resume: string
  description: string
  type: 'mooc' | 'clom' | 'atelier' | 'concertation'
  statut: 'brouillon' | 'programme' | 'inscriptions_ouvertes' | 'complet' | 'en_cours' | 'termine' | 'annule' | 'archive'
  dateDebut: Date
  dateFin: Date
  formateurPrenom: string
  formateurNom: string
  formateurPhotoURL?: string
  formateurBio?: string
  formateurQualifications?: string[]
  dureeEstimee: DureeEstimee
  modalites: Modalites
  tarification: Tarification
  capacite: Capacite
  objectifs?: Objectifs
  sessions?: ModuleCours[]
  stats?: {
    inscriptions: number
  }
  tags?: string[]
}

export const formationsMock: Formation[] = [
  {
    id: 'form-1',
    titre: 'Introduction à l\'économie africaine',
    resume: 'Comprendre les fondamentaux de l\'économie du continent africain et ses perspectives de développement.',
    description: 'Ce MOOC offre une vue d\'ensemble complète de l\'économie africaine, de ses défis et opportunités. Vous découvrirez les principales économies du continent, les secteurs porteurs et les enjeux de l\'intégration régionale.',
    type: 'mooc',
    statut: 'inscriptions_ouvertes',
    dateDebut: new Date('2025-02-15'),
    dateFin: new Date('2025-04-15'),
    formateurPrenom: 'Fatou',
    formateurNom: 'Dieng',
    formateurPhotoURL: 'https://images.unsplash.com/photo-1573497019940-1c28c88b4f3e?w=200',
    formateurBio: 'Économiste spécialisée dans le développement africain, ancienne conseillère à la BAD.',
    formateurQualifications: ['PhD Économie - Université de Dakar', 'Consultante BAD'],
    dureeEstimee: { heures: 24, semaines: 8 },
    modalites: {
      langue: 'fr',
      niveauRequis: 'debutant',
      certificationDisponible: true,
      prerequis: ['Aucun prérequis particulier'],
      modulesCours: [
        { id: 'm1', titre: 'Panorama économique africain', description: 'Vue d\'ensemble des économies africaines', duree: 180, ordre: 1 },
        { id: 'm2', titre: 'Les moteurs de croissance', description: 'Secteurs clés et dynamiques', duree: 180, ordre: 2 },
        { id: 'm3', titre: 'Intégration régionale', description: 'ZLECAF et coopération économique', duree: 180, ordre: 3 }
      ]
    },
    tarification: { gratuit: true, prix: 0 },
    capacite: { maximum: 500, inscritsActuels: 234 },
    objectifs: {
      generaux: ['Comprendre l\'économie africaine'],
      specifiques: ['Analyser les indicateurs économiques', 'Identifier les opportunités'],
      competencesAcquises: ['Analyse économique', 'Vision panafricaine']
    },
    stats: { inscriptions: 234 },
    tags: ['économie', 'développement', 'Afrique']
  },
  {
    id: 'form-2',
    titre: 'Leadership et gouvernance en Afrique',
    resume: 'Développez vos compétences en leadership adaptées au contexte africain.',
    description: 'Un programme intensif pour développer un leadership authentique et efficace, ancré dans les valeurs africaines tout en intégrant les meilleures pratiques internationales.',
    type: 'clom',
    statut: 'inscriptions_ouvertes',
    dateDebut: new Date('2025-03-01'),
    dateFin: new Date('2025-05-30'),
    formateurPrenom: 'Kwame',
    formateurNom: 'Asante',
    formateurPhotoURL: 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=200',
    formateurBio: 'Expert en leadership organisationnel, auteur de plusieurs ouvrages sur le management africain.',
    dureeEstimee: { heures: 40, semaines: 12 },
    modalites: {
      langue: 'fr',
      niveauRequis: 'intermediaire',
      certificationDisponible: true,
      prerequis: ['Expérience professionnelle de 3 ans minimum']
    },
    tarification: {
      gratuit: false,
      prix: 75000,
      prixReduit: [{ montant: 50000, conditions: 'Étudiants et jeunes diplômés' }]
    },
    capacite: { maximum: 50, inscritsActuels: 38 },
    stats: { inscriptions: 38 },
    tags: ['leadership', 'management', 'gouvernance']
  },
  {
    id: 'form-3',
    titre: 'Atelier pratique : Entrepreneuriat social',
    resume: 'Lancez votre projet d\'entreprise sociale avec un accompagnement personnalisé.',
    description: 'Un atelier pratique de 3 jours pour transformer votre idée en projet d\'entreprise sociale viable. Coaching individuel et collectif inclus.',
    type: 'atelier',
    statut: 'inscriptions_ouvertes',
    dateDebut: new Date('2025-02-20'),
    dateFin: new Date('2025-02-22'),
    formateurPrenom: 'Aminata',
    formateurNom: 'Konaré',
    formateurPhotoURL: 'https://images.unsplash.com/photo-1580489944761-15a19d654956?w=200',
    formateurBio: 'Fondatrice de 3 entreprises sociales primées en Afrique de l\'Ouest.',
    dureeEstimee: { heures: 18 },
    modalites: {
      langue: 'fr',
      niveauRequis: 'tous_niveaux',
      certificationDisponible: false,
      prerequis: ['Avoir une idée de projet']
    },
    tarification: { gratuit: false, prix: 25000 },
    capacite: { maximum: 25, inscritsActuels: 22 },
    stats: { inscriptions: 22 },
    tags: ['entrepreneuriat', 'social', 'impact']
  },
  {
    id: 'form-4',
    titre: 'Concertation : L\'avenir de l\'éducation en Afrique',
    resume: 'Participez au débat sur les réformes éducatives nécessaires pour le continent.',
    description: 'Une série de discussions en ligne réunissant experts, éducateurs et décideurs pour repenser l\'éducation africaine.',
    type: 'concertation',
    statut: 'programme',
    dateDebut: new Date('2025-04-10'),
    dateFin: new Date('2025-04-12'),
    formateurPrenom: 'Ibrahim',
    formateurNom: 'Maïga',
    formateurPhotoURL: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=200',
    formateurBio: 'Ancien ministre de l\'Éducation, expert UNESCO.',
    dureeEstimee: { heures: 6 },
    modalites: {
      langue: 'fr',
      niveauRequis: 'tous_niveaux',
      certificationDisponible: false,
      prerequis: []
    },
    tarification: { gratuit: true, prix: 0 },
    capacite: { maximum: 200, inscritsActuels: 0 },
    stats: { inscriptions: 0 },
    tags: ['éducation', 'politique', 'réforme']
  },
  {
    id: 'form-5',
    titre: 'Développement web moderne',
    resume: 'Apprenez à créer des applications web avec les technologies actuelles.',
    description: 'Formation complète au développement web : HTML, CSS, JavaScript, Vue.js, Node.js. Projets pratiques et déploiement inclus.',
    type: 'mooc',
    statut: 'en_cours',
    dateDebut: new Date('2025-01-10'),
    dateFin: new Date('2025-03-10'),
    formateurPrenom: 'Ousmane',
    formateurNom: 'Traoré',
    formateurPhotoURL: 'https://images.unsplash.com/photo-1560250097-0b93528c311a?w=200',
    formateurBio: 'Développeur senior et formateur, 10 ans d\'expérience.',
    dureeEstimee: { heures: 60, semaines: 10 },
    modalites: {
      langue: 'fr',
      niveauRequis: 'debutant',
      certificationDisponible: true,
      prerequis: ['Connaissances de base en informatique']
    },
    tarification: { gratuit: false, prix: 50000 },
    capacite: { maximum: 100, inscritsActuels: 100 },
    stats: { inscriptions: 100 },
    tags: ['informatique', 'web', 'programmation']
  },
  {
    id: 'form-6',
    titre: 'Agriculture durable et agroécologie',
    resume: 'Techniques agricoles respectueuses de l\'environnement adaptées au climat africain.',
    description: 'Découvrez les méthodes d\'agriculture durable, la permaculture et l\'agroforesterie pour une production alimentaire résiliente.',
    type: 'mooc',
    statut: 'inscriptions_ouvertes',
    dateDebut: new Date('2025-03-15'),
    dateFin: new Date('2025-05-15'),
    formateurPrenom: 'Mariama',
    formateurNom: 'Bah',
    formateurPhotoURL: 'https://images.unsplash.com/photo-1594744803329-e58b31de8bf5?w=200',
    formateurBio: 'Ingénieure agronome, spécialiste en agroécologie tropicale.',
    dureeEstimee: { heures: 30, semaines: 8 },
    modalites: {
      langue: 'fr',
      niveauRequis: 'tous_niveaux',
      certificationDisponible: true,
      prerequis: []
    },
    tarification: { gratuit: true, prix: 0 },
    capacite: { maximum: 300, inscritsActuels: 156 },
    stats: { inscriptions: 156 },
    tags: ['agriculture', 'environnement', 'développement durable']
  }
]

// Fonction pour obtenir les formations ouvertes aux inscriptions
export const getFormationsOuvertes = (): Formation[] => {
  return formationsMock.filter(f => f.statut === 'inscriptions_ouvertes')
}

// Fonction pour obtenir une formation par son ID
export const getFormationById = (id: string): Formation | undefined => {
  return formationsMock.find(f => f.id === id)
}

// Fonction pour rechercher des formations avec filtres
export const rechercherFormations = (
  recherche: string = '',
  filtres: {
    types?: string[]
    statuts?: string[]
    gratuit?: boolean | null
  } = {}
): Formation[] => {
  return formationsMock.filter(formation => {
    // Filtre par recherche textuelle
    if (recherche) {
      const searchLower = recherche.toLowerCase()
      const matchTitre = formation.titre.toLowerCase().includes(searchLower)
      const matchDescription = formation.description.toLowerCase().includes(searchLower)
      const matchFormateur = `${formation.formateurPrenom} ${formation.formateurNom}`.toLowerCase().includes(searchLower)
      if (!matchTitre && !matchDescription && !matchFormateur) {
        return false
      }
    }

    // Filtre par type
    if (filtres.types && filtres.types.length > 0) {
      if (!filtres.types.includes(formation.type)) {
        return false
      }
    }

    // Filtre par statut
    if (filtres.statuts && filtres.statuts.length > 0) {
      if (!filtres.statuts.includes(formation.statut)) {
        return false
      }
    }

    // Filtre par gratuité
    if (filtres.gratuit !== null && filtres.gratuit !== undefined) {
      if (formation.tarification.gratuit !== filtres.gratuit) {
        return false
      }
    }

    return true
  })
}

// Fonction de formatage de date
export const formatDateFormation = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  }).format(date)
}

// Donnees mock pour les contributions citoyennes (Gouvernance)

export interface ContributionAuteur {
  id: string
  prenom: string
  nom: string
  photoURL?: string
}

export interface ContributionStats {
  vues: number
  vuesUniques: number
  likes: number
  commentaires: number
  partages: number
  soutiens?: number
  validations?: number
  confirmations?: number
}

export interface ContributionLocalisation {
  pays: string
  region?: string
  ville?: string
  quartier?: string
  latitude?: number
  longitude?: number
}

export interface Prejudice {
  titre: string
  description: string
}

export interface ContributionProblematique {
  categorie: string
  gravite?: 'faible' | 'moyenne' | 'grave' | 'critique'
  urgence?: string
}

export type TypePratique = 'mauvaise' | 'bonne'

export interface ContributionBonnePratique {
  categorie: string
  impact?: 'faible' | 'moyen' | 'fort' | 'exemplaire'
  reproductibilite?: string
}

export interface ContributionMedias {
  photos: string[]
  videos: string[]
  documents: string[]
}

export interface ContributionCitoyenne {
  id: string
  type: 'factcheck' | 'badhabits' | 'ideaforces'
  statut: 'brouillon' | 'publie' | 'archive'
  titre: string
  description: string
  auteur: ContributionAuteur
  localisation: ContributionLocalisation
  dateCreation: Date
  dateMiseAJour?: Date
  typePratique?: TypePratique
  problematique?: ContributionProblematique
  bonnePratique?: ContributionBonnePratique
  factcheck?: {
    prejuge: Prejudice
    contrePrejuge: Prejudice
  }
  proposition?: {
    objectif: string
    moyens: string[]
    beneficiaires: string[]
    impact: string
  }
  medias?: ContributionMedias
  sources?: { titre: string; url: string }[]
  stats: ContributionStats
  tags?: string[]
  verified?: boolean
}

export const contributionsMock: ContributionCitoyenne[] = [
  // FactCheck
  {
    id: 'contrib-1',
    type: 'factcheck',
    statut: 'publie',
    titre: 'L\'Afrique n\'a pas d\'histoire écrite avant la colonisation',
    description: 'Ce préjugé répandu ignore les nombreuses civilisations africaines qui avaient développé leurs propres systèmes d\'écriture et de documentation historique.',
    auteur: {
      id: 'user-1',
      prenom: 'Moussa',
      nom: 'Keita',
      photoURL: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=100'
    },
    localisation: {
      pays: 'Mali',
      ville: 'Bamako'
    },
    dateCreation: new Date('2025-01-10'),
    factcheck: {
      prejuge: {
        titre: 'L\'Afrique sans histoire écrite',
        description: 'Beaucoup croient que l\'Afrique n\'avait pas d\'histoire documentée avant l\'arrivée des Européens.'
      },
      contrePrejuge: {
        titre: 'Les civilisations lettrées africaines',
        description: 'L\'Empire du Mali, l\'Éthiopie avec le Ge\'ez, l\'Égypte avec les hiéroglyphes, et bien d\'autres civilisations avaient des systèmes d\'écriture sophistiqués.'
      }
    },
    sources: [
      { titre: 'UNESCO - Histoire générale de l\'Afrique', url: 'https://unesco.org' },
      { titre: 'Manuscrits de Tombouctou', url: 'https://example.com' }
    ],
    stats: {
      vues: 1250,
      vuesUniques: 890,
      likes: 234,
      commentaires: 45,
      partages: 78,
      validations: 156
    },
    tags: ['histoire', 'écriture', 'civilisation'],
    verified: true
  },
  {
    id: 'contrib-2',
    type: 'factcheck',
    statut: 'publie',
    titre: 'Les Africains ne savent pas gérer les entreprises',
    description: 'Ce stéréotype ignore les nombreux entrepreneurs africains qui ont bâti des empires économiques prospères.',
    auteur: {
      id: 'user-2',
      prenom: 'Awa',
      nom: 'Diallo',
      photoURL: 'https://images.unsplash.com/photo-1531123897727-8f129e1688ce?w=100'
    },
    localisation: {
      pays: 'Sénégal',
      ville: 'Dakar'
    },
    dateCreation: new Date('2025-01-15'),
    factcheck: {
      prejuge: {
        titre: 'Incapacité entrepreneuriale africaine',
        description: 'Préjugé selon lequel les Africains ne peuvent pas créer et gérer des entreprises prospères.'
      },
      contrePrejuge: {
        titre: 'Success stories africaines',
        description: 'De Dangote à Patrice Motsepe, l\'Afrique compte de nombreux milliardaires et entrepreneurs à succès dans divers secteurs.'
      }
    },
    stats: {
      vues: 890,
      vuesUniques: 650,
      likes: 189,
      commentaires: 32,
      partages: 56,
      validations: 98
    },
    tags: ['entrepreneuriat', 'économie', 'business'],
    verified: true
  },

  // BadHabits
  {
    id: 'contrib-3',
    type: 'badhabits',
    statut: 'publie',
    titre: 'Corruption dans les administrations publiques',
    description: 'Signalement des pratiques de corruption qui entravent le développement et l\'accès aux services publics.',
    auteur: {
      id: 'user-3',
      prenom: 'Ibrahim',
      nom: 'Touré'
    },
    localisation: {
      pays: 'Côte d\'Ivoire',
      ville: 'Abidjan',
      region: 'Lagunes'
    },
    dateCreation: new Date('2025-01-12'),
    typePratique: 'mauvaise',
    problematique: {
      categorie: 'Corruption',
      gravite: 'grave',
      urgence: 'Haute'
    },
    medias: {
      photos: [],
      videos: [],
      documents: []
    },
    stats: {
      vues: 567,
      vuesUniques: 432,
      likes: 89,
      commentaires: 23,
      partages: 34,
      soutiens: 156
    },
    tags: ['corruption', 'administration', 'gouvernance']
  },
  {
    id: 'contrib-4',
    type: 'badhabits',
    statut: 'publie',
    titre: 'Gestion des déchets urbains',
    description: 'Les quartiers périphériques manquent cruellement de services de collecte des ordures, créant des problèmes sanitaires.',
    auteur: {
      id: 'user-4',
      prenom: 'Fatima',
      nom: 'Ouedraogo',
      photoURL: 'https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100'
    },
    localisation: {
      pays: 'Burkina Faso',
      ville: 'Ouagadougou',
      quartier: 'Tampouy'
    },
    dateCreation: new Date('2025-01-18'),
    typePratique: 'mauvaise',
    problematique: {
      categorie: 'Environnement',
      gravite: 'moyenne',
      urgence: 'Moyenne'
    },
    stats: {
      vues: 345,
      vuesUniques: 289,
      likes: 67,
      commentaires: 15,
      partages: 23,
      soutiens: 89
    },
    tags: ['environnement', 'déchets', 'santé publique']
  },
  {
    id: 'contrib-7',
    type: 'badhabits',
    statut: 'publie',
    titre: 'Brigades citoyennes de salubrité à Kigali',
    description: 'Chaque dernier samedi du mois, les habitants nettoient collectivement leur quartier. Résultat : Kigali est l\'une des villes les plus propres d\'Afrique.',
    auteur: {
      id: 'user-7',
      prenom: 'Aline',
      nom: 'Mukamana',
      photoURL: 'https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=100'
    },
    localisation: {
      pays: 'Rwanda',
      ville: 'Kigali'
    },
    dateCreation: new Date('2025-02-02'),
    typePratique: 'bonne',
    bonnePratique: {
      categorie: 'Civisme',
      impact: 'exemplaire',
      reproductibilite: 'Adaptable à toute commune avec une organisation de quartier.'
    },
    stats: {
      vues: 712,
      vuesUniques: 540,
      likes: 298,
      commentaires: 42,
      partages: 88,
      soutiens: 321
    },
    tags: ['civisme', 'propreté', 'communauté']
  },
  {
    id: 'contrib-8',
    type: 'badhabits',
    statut: 'publie',
    titre: 'Guichet unique 100% digital au ministère des transports',
    description: 'Démarches administratives dématérialisées : permis, cartes grises, vignettes. Délai réduit de 3 semaines à 48h.',
    auteur: {
      id: 'user-8',
      prenom: 'Ismaël',
      nom: 'Traoré'
    },
    localisation: {
      pays: 'Togo',
      ville: 'Lomé'
    },
    dateCreation: new Date('2025-02-08'),
    typePratique: 'bonne',
    bonnePratique: {
      categorie: 'Service public',
      impact: 'fort',
      reproductibilite: 'Nécessite un investissement initial mais ROI rapide.'
    },
    stats: {
      vues: 489,
      vuesUniques: 398,
      likes: 176,
      commentaires: 29,
      partages: 54,
      soutiens: 203
    },
    tags: ['digitalisation', 'administration', 'efficacité']
  },
  {
    id: 'contrib-9',
    type: 'badhabits',
    statut: 'publie',
    titre: 'Cantine scolaire bio financée par la coopérative agricole',
    description: 'Les agriculteurs locaux fournissent gratuitement une partie de leur production pour alimenter les cantines scolaires du village.',
    auteur: {
      id: 'user-9',
      prenom: 'Mariam',
      nom: 'Sow',
      photoURL: 'https://images.unsplash.com/photo-1580489944761-15a19d654956?w=100'
    },
    localisation: {
      pays: 'Sénégal',
      ville: 'Kaolack',
      region: 'Kaolack'
    },
    dateCreation: new Date('2025-02-14'),
    typePratique: 'bonne',
    bonnePratique: {
      categorie: 'Solidarité',
      impact: 'fort',
      reproductibilite: 'Modèle duplicable dans toute zone rurale agricole.'
    },
    stats: {
      vues: 356,
      vuesUniques: 287,
      likes: 134,
      commentaires: 21,
      partages: 41,
      soutiens: 189
    },
    tags: ['solidarité', 'éducation', 'agriculture']
  },

  // IdeaForces
  {
    id: 'contrib-5',
    type: 'ideaforces',
    statut: 'publie',
    titre: 'Création d\'incubateurs dans les universités africaines',
    description: 'Proposition de créer des espaces d\'incubation dans chaque université pour transformer les projets étudiants en entreprises viables.',
    auteur: {
      id: 'user-5',
      prenom: 'Amadou',
      nom: 'Diop',
      photoURL: 'https://images.unsplash.com/photo-1560250097-0b93528c311a?w=100'
    },
    localisation: {
      pays: 'Sénégal',
      ville: 'Saint-Louis'
    },
    dateCreation: new Date('2025-01-08'),
    proposition: {
      objectif: 'Favoriser l\'entrepreneuriat étudiant',
      moyens: ['Espaces de coworking', 'Mentorat', 'Financement amorçage'],
      beneficiaires: ['Étudiants', 'Jeunes diplômés', 'Chercheurs'],
      impact: 'Création de 1000 startups par an dans la région'
    },
    stats: {
      vues: 789,
      vuesUniques: 567,
      likes: 234,
      commentaires: 56,
      partages: 89,
      soutiens: 345
    },
    tags: ['entrepreneuriat', 'éducation', 'innovation']
  },
  {
    id: 'contrib-6',
    type: 'ideaforces',
    statut: 'publie',
    titre: 'Application mobile pour l\'agriculture locale',
    description: 'Développer une application connectant directement agriculteurs et consommateurs pour éliminer les intermédiaires.',
    auteur: {
      id: 'user-6',
      prenom: 'Kofi',
      nom: 'Mensah'
    },
    localisation: {
      pays: 'Ghana',
      ville: 'Accra'
    },
    dateCreation: new Date('2025-01-20'),
    proposition: {
      objectif: 'Circuit court alimentaire',
      moyens: ['Application mobile', 'Système de paiement mobile', 'Logistique locale'],
      beneficiaires: ['Agriculteurs', 'Consommateurs urbains'],
      impact: 'Augmentation de 40% des revenus agricoles'
    },
    stats: {
      vues: 456,
      vuesUniques: 378,
      likes: 145,
      commentaires: 34,
      partages: 67,
      soutiens: 234
    },
    tags: ['agriculture', 'technologie', 'économie locale']
  }
]

// Fonctions utilitaires

export const getContributionsPubliees = (): ContributionCitoyenne[] => {
  return contributionsMock.filter(c => c.statut === 'publie')
}

export const getContributionsByType = (type: 'factcheck' | 'badhabits' | 'ideaforces'): ContributionCitoyenne[] => {
  return contributionsMock.filter(c => c.type === type && c.statut === 'publie')
}

export const getContributionById = (id: string): ContributionCitoyenne | undefined => {
  return contributionsMock.find(c => c.id === id)
}

export const getStatsGouvernance = () => {
  const publiees = getContributionsPubliees()
  return {
    total: publiees.length,
    factcheck: publiees.filter(c => c.type === 'factcheck').length,
    badhabits: publiees.filter(c => c.type === 'badhabits').length,
    ideaforces: publiees.filter(c => c.type === 'ideaforces').length,
    totalLikes: publiees.reduce((acc, c) => acc + c.stats.likes, 0)
  }
}

// Liste des pays africains pour les filtres
export const paysAfricains = [
  'Afrique du Sud', 'Algérie', 'Angola', 'Bénin', 'Botswana', 'Burkina Faso',
  'Burundi', 'Cameroun', 'Cap-Vert', 'Centrafrique', 'Comores', 'Congo',
  'Côte d\'Ivoire', 'Djibouti', 'Égypte', 'Érythrée', 'Éthiopie', 'Gabon',
  'Gambie', 'Ghana', 'Guinée', 'Guinée équatoriale', 'Guinée-Bissau', 'Kenya',
  'Lesotho', 'Liberia', 'Libye', 'Madagascar', 'Malawi', 'Mali', 'Maroc',
  'Maurice', 'Mauritanie', 'Mozambique', 'Namibie', 'Niger', 'Nigeria',
  'Ouganda', 'RD Congo', 'Rwanda', 'Sao Tomé-et-Príncipe', 'Sénégal',
  'Seychelles', 'Sierra Leone', 'Somalie', 'Soudan', 'Soudan du Sud',
  'Swaziland', 'Tanzanie', 'Tchad', 'Togo', 'Tunisie', 'Zambie', 'Zimbabwe'
]

export const categoriesProblematiques = [
  'Corruption',
  'Environnement',
  'Éducation',
  'Santé',
  'Infrastructure',
  'Sécurité',
  'Économie',
  'Droits humains',
  'Justice',
  'Autre'
]

export const categoriesBonnesPratiques = [
  'Civisme',
  'Service public',
  'Solidarité',
  'Innovation sociale',
  'Initiative citoyenne',
  'Leadership exemplaire',
  'Transparence',
  'Environnement',
  'Éducation',
  'Santé',
  'Autre'
]

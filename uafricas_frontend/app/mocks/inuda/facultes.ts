// Donnees mock pour les facultes INUDA

export interface EcolePartenaire {
  nom: string
  ville: string
  pays: string
  type: 'publique' | 'privee'
  siteWeb?: string
  emailContact: string
  telephoneContact?: string
  whatsappContact?: string
}

export interface ProgrammesResume {
  licence: string[]
  master: string[]
  doctorat: string[]
  certificats: string[]
}

export interface ConditionsAdmission {
  diplomeMinimum: string
  languesEnseignement: string[]
  fraisScolariteAnnuels: {
    min: number
    max: number
    boursesPossibles: boolean
  }
  periodesInscription: string
}

export interface ReferentPlateforme {
  prenom: string
  nom: string
  role: string
  email: string
}

export interface Faculte {
  id: string
  titre: string
  acronyme: string
  description: string
  imageCouverture?: string
  logo?: string
  ecolePartenaire: EcolePartenaire
  domainesEtudes: string[]
  programmesResume: ProgrammesResume
  conditionsAdmission: ConditionsAdmission
  pointsForts: string[]
  accepteNouveauxInscrits: boolean
  statut: 'active' | 'inactive'
  stats: {
    nombreInscritsTotal: number
    nombreInscritsAnneeEnCours: number
  }
  referentPlateforme?: ReferentPlateforme
}

export const facultesMock: Faculte[] = [
  {
    id: 'fac-1',
    titre: 'Faculté des Sciences Économiques et de Gestion',
    acronyme: 'FASEG',
    description: 'La FASEG forme les futurs leaders économiques africains avec des programmes innovants en économie, finance et management adaptés aux réalités du continent.',
    imageCouverture: 'https://images.unsplash.com/photo-1454165804606-c3d57bc86b40?w=800',
    logo: '',
    ecolePartenaire: {
      nom: 'Université Cheikh Anta Diop',
      ville: 'Dakar',
      pays: 'Sénégal',
      type: 'publique',
      siteWeb: 'https://ucad.sn',
      emailContact: 'faseg@ucad.sn',
      telephoneContact: '+221 33 825 00 00'
    },
    domainesEtudes: ['Économie', 'Finance', 'Management', 'Comptabilité', 'Marketing'],
    programmesResume: {
      licence: ['Économie', 'Gestion des entreprises', 'Finance'],
      master: ['MBA', 'Finance internationale', 'Économie du développement'],
      doctorat: ['Sciences économiques'],
      certificats: ['Gestion de projet', 'Analyse financière']
    },
    conditionsAdmission: {
      diplomeMinimum: 'Baccalauréat',
      languesEnseignement: ['Français', 'Anglais'],
      fraisScolariteAnnuels: {
        min: 150000,
        max: 500000,
        boursesPossibles: true
      },
      periodesInscription: 'Juillet - Septembre'
    },
    pointsForts: [
      'Partenariats internationaux',
      'Stages en entreprise garantis',
      'Formation pratique orientée marché',
      'Incubateur de startups'
    ],
    accepteNouveauxInscrits: true,
    statut: 'active',
    stats: {
      nombreInscritsTotal: 1250,
      nombreInscritsAnneeEnCours: 320
    },
    referentPlateforme: {
      prenom: 'Amadou',
      nom: 'Diallo',
      role: 'Coordinateur INUDA',
      email: 'a.diallo@inuda.org'
    }
  },
  {
    id: 'fac-2',
    titre: 'Institut Polytechnique de Technologie',
    acronyme: 'IPT',
    description: 'L\'IPT est une école d\'ingénieurs de premier plan formant des experts en technologie, informatique et télécommunications pour l\'Afrique de demain.',
    imageCouverture: 'https://images.unsplash.com/photo-1581092918056-0c4c3acd3789?w=800',
    logo: '',
    ecolePartenaire: {
      nom: 'Institut National Polytechnique',
      ville: 'Yamoussoukro',
      pays: 'Côte d\'Ivoire',
      type: 'publique',
      siteWeb: 'https://inphb.ci',
      emailContact: 'contact@inphb.ci',
      telephoneContact: '+225 30 64 01 00'
    },
    domainesEtudes: ['Informatique', 'Génie civil', 'Électronique', 'Télécommunications', 'Énergie'],
    programmesResume: {
      licence: ['Informatique', 'Génie électrique', 'Génie civil'],
      master: ['Ingénierie logicielle', 'Cybersécurité', 'Énergies renouvelables'],
      doctorat: ['Sciences de l\'ingénieur'],
      certificats: ['Développement web', 'Administration système', 'IoT']
    },
    conditionsAdmission: {
      diplomeMinimum: 'Baccalauréat scientifique',
      languesEnseignement: ['Français'],
      fraisScolariteAnnuels: {
        min: 200000,
        max: 800000,
        boursesPossibles: true
      },
      periodesInscription: 'Juin - Août'
    },
    pointsForts: [
      'Laboratoires équipés dernière génération',
      'Double diplôme avec universités européennes',
      'Taux d\'insertion professionnelle de 95%',
      'Recherche appliquée'
    ],
    accepteNouveauxInscrits: true,
    statut: 'active',
    stats: {
      nombreInscritsTotal: 890,
      nombreInscritsAnneeEnCours: 215
    }
  },
  {
    id: 'fac-3',
    titre: 'École Supérieure de Droit et Sciences Politiques',
    acronyme: 'ESDSP',
    description: 'L\'ESDSP forme les juristes et politologues de demain avec une approche comparative du droit africain et international.',
    imageCouverture: 'https://images.unsplash.com/photo-1589829545856-d10d557cf95f?w=800',
    logo: '',
    ecolePartenaire: {
      nom: 'Université de Ouagadougou',
      ville: 'Ouagadougou',
      pays: 'Burkina Faso',
      type: 'publique',
      emailContact: 'esdsp@univ-ouaga.bf'
    },
    domainesEtudes: ['Droit', 'Sciences politiques', 'Relations internationales', 'Administration publique'],
    programmesResume: {
      licence: ['Droit privé', 'Droit public', 'Sciences politiques'],
      master: ['Droit des affaires', 'Droit international', 'Gouvernance publique'],
      doctorat: ['Droit', 'Sciences politiques'],
      certificats: ['Médiation', 'Droit OHADA']
    },
    conditionsAdmission: {
      diplomeMinimum: 'Baccalauréat',
      languesEnseignement: ['Français'],
      fraisScolariteAnnuels: {
        min: 100000,
        max: 350000,
        boursesPossibles: true
      },
      periodesInscription: 'Août - Octobre'
    },
    pointsForts: [
      'Clinique juridique pour les étudiants',
      'Moot courts internationaux',
      'Partenariats avec organisations internationales'
    ],
    accepteNouveauxInscrits: false,
    statut: 'active',
    stats: {
      nombreInscritsTotal: 670,
      nombreInscritsAnneeEnCours: 0
    }
  },
  {
    id: 'fac-4',
    titre: 'Faculté des Sciences de la Santé',
    acronyme: 'FSS',
    description: 'La FSS forme les professionnels de santé de qualité pour répondre aux défis sanitaires du continent africain.',
    imageCouverture: 'https://images.unsplash.com/photo-1576091160399-112ba8d25d1d?w=800',
    logo: '',
    ecolePartenaire: {
      nom: 'Université de Lomé',
      ville: 'Lomé',
      pays: 'Togo',
      type: 'publique',
      emailContact: 'fss@univ-lome.tg',
      telephoneContact: '+228 22 21 35 00'
    },
    domainesEtudes: ['Médecine', 'Pharmacie', 'Sciences infirmières', 'Santé publique'],
    programmesResume: {
      licence: ['Sciences infirmières', 'Sage-femme'],
      master: ['Santé publique', 'Épidémiologie'],
      doctorat: ['Médecine', 'Pharmacie'],
      certificats: ['Urgences médicales', 'Gestion hospitalière']
    },
    conditionsAdmission: {
      diplomeMinimum: 'Baccalauréat scientifique',
      languesEnseignement: ['Français'],
      fraisScolariteAnnuels: {
        min: 300000,
        max: 1500000,
        boursesPossibles: true
      },
      periodesInscription: 'Mai - Juillet'
    },
    pointsForts: [
      'CHU universitaire intégré',
      'Équipements médicaux modernes',
      'Programmes de spécialisation'
    ],
    accepteNouveauxInscrits: true,
    statut: 'active',
    stats: {
      nombreInscritsTotal: 1100,
      nombreInscritsAnneeEnCours: 180
    }
  },
  {
    id: 'fac-5',
    titre: 'École des Arts et de la Culture Africaine',
    acronyme: 'EACA',
    description: 'L\'EACA célèbre et perpétue les arts africains tout en formant les créateurs et managers culturels de demain.',
    imageCouverture: 'https://images.unsplash.com/photo-1578926288207-a90a5366759d?w=800',
    logo: '',
    ecolePartenaire: {
      nom: 'Institut National des Arts',
      ville: 'Bamako',
      pays: 'Mali',
      type: 'publique',
      emailContact: 'eaca@ina-mali.ml'
    },
    domainesEtudes: ['Arts visuels', 'Musique', 'Danse', 'Cinéma', 'Management culturel'],
    programmesResume: {
      licence: ['Arts plastiques', 'Musique', 'Arts du spectacle'],
      master: ['Direction artistique', 'Production cinématographique', 'Management culturel'],
      doctorat: [],
      certificats: ['Photographie', 'Design graphique', 'Sound design']
    },
    conditionsAdmission: {
      diplomeMinimum: 'Baccalauréat ou équivalent artistique',
      languesEnseignement: ['Français', 'Bambara'],
      fraisScolariteAnnuels: {
        min: 80000,
        max: 250000,
        boursesPossibles: true
      },
      periodesInscription: 'Septembre - Novembre'
    },
    pointsForts: [
      'Studios et ateliers professionnels',
      'Festivals étudiants annuels',
      'Résidences d\'artistes'
    ],
    accepteNouveauxInscrits: true,
    statut: 'active',
    stats: {
      nombreInscritsTotal: 340,
      nombreInscritsAnneeEnCours: 85
    }
  }
]

// Fonction pour obtenir toutes les facultés actives
export const getFacultesActives = (): Faculte[] => {
  return facultesMock.filter(f => f.statut === 'active')
}

// Fonction pour obtenir une faculté par son ID
export const getFaculteById = (id: string): Faculte | undefined => {
  return facultesMock.find(f => f.id === id)
}

// Fonction pour obtenir les domaines uniques
export const getDomainesUniques = (): string[] => {
  const domaines = new Set<string>()
  facultesMock.forEach(f => {
    f.domainesEtudes.forEach(d => domaines.add(d))
  })
  return Array.from(domaines).sort()
}

// Stats globales
export const getStatsInuda = () => {
  const facultesActives = getFacultesActives()
  return {
    nombreFacultes: facultesActives.length,
    nombreInscritsTotal: facultesActives.reduce((acc, f) => acc + f.stats.nombreInscritsTotal, 0),
    nombreFormationsOuvertes: 12, // Mock
    nombrePays: 15
  }
}

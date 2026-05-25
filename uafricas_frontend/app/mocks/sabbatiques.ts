// Données mock pour les programmes d'échanges sabbatiques

export type TypeProgramme = 'interafricain' | 'hors_afrique'
export type StatutProgramme = 'ouvert' | 'en_cours' | 'termine' | 'complet'
export type Domaine = 'education' | 'infrastructures' | 'sante' | 'eau' | 'developpement_local'
export type DureeProgramme = '1_semaine' | '2_semaines' | '3_semaines' | '6_semaines' | '1_mois' | '2_mois' | '6_mois' | '1_an'
export type PriseEnCharge = 'billet_avion' | 'hebergement' | 'frais_subsistance'

export interface ProgrammeSabbatique {
  id: string
  titre: string
  description: string
  couvertureUrl: string
  documentUrl?: string
  pays: string
  ville?: string
  priseEnCharge: PriseEnCharge[]
  dureeProgramme: DureeProgramme
  domaine: Domaine
  dateHeureDebut: Date
  dateHeureFin: Date
  interafricain: boolean
  statut: StatutProgramme
  organisateurNom?: string
  organisateurEmail?: string
  createdAt: Date
  updatedAt: Date
}

export interface FiltresSabbatique {
  type: 'tous' | TypeProgramme
  pays: string
  domaine: string
  recherche: string
}

// Constantes pour les selects
export const DOMAINES: { value: Domaine | ''; label: string }[] = [
  { value: '', label: 'Tous les domaines' },
  { value: 'education', label: 'Éducation' },
  { value: 'infrastructures', label: 'Infrastructures' },
  { value: 'sante', label: 'Santé' },
  { value: 'eau', label: 'Eau' },
  { value: 'developpement_local', label: 'Développement local' }
]

export const DUREES: { value: DureeProgramme; label: string }[] = [
  { value: '1_semaine', label: '1 semaine' },
  { value: '2_semaines', label: '2 semaines' },
  { value: '3_semaines', label: '3 semaines' },
  { value: '6_semaines', label: '6 semaines' },
  { value: '1_mois', label: '1 mois' },
  { value: '2_mois', label: '2 mois' },
  { value: '6_mois', label: '6 mois' },
  { value: '1_an', label: '1 an' }
]

export const PAYS_AFRICAINS: { value: string; label: string }[] = [
  { value: '', label: 'Tous les territoires' },
  { value: 'Afrique du Sud', label: 'Afrique du Sud' },
  { value: 'Algérie', label: 'Algérie' },
  { value: 'Bénin', label: 'Bénin' },
  { value: 'Burkina Faso', label: 'Burkina Faso' },
  { value: 'Cameroun', label: 'Cameroun' },
  { value: 'Cap-Vert', label: 'Cap-Vert' },
  { value: 'Comores', label: 'Comores' },
  { value: 'Côte d\'Ivoire', label: 'Côte d\'Ivoire' },
  { value: 'Égypte', label: 'Égypte' },
  { value: 'Éthiopie', label: 'Éthiopie' },
  { value: 'Gabon', label: 'Gabon' },
  { value: 'Gambie', label: 'Gambie' },
  { value: 'Ghana', label: 'Ghana' },
  { value: 'Guinée', label: 'Guinée' },
  { value: 'Kenya', label: 'Kenya' },
  { value: 'Madagascar', label: 'Madagascar' },
  { value: 'Mali', label: 'Mali' },
  { value: 'Maroc', label: 'Maroc' },
  { value: 'Maurice', label: 'Maurice' },
  { value: 'Nigeria', label: 'Nigeria' },
  { value: 'Rwanda', label: 'Rwanda' },
  { value: 'Sénégal', label: 'Sénégal' },
  { value: 'Tanzanie', label: 'Tanzanie' },
  { value: 'Togo', label: 'Togo' },
  { value: 'Tunisie', label: 'Tunisie' }
]

export const PRISES_EN_CHARGE: { value: PriseEnCharge; label: string }[] = [
  { value: 'billet_avion', label: 'Billet d\'avion' },
  { value: 'hebergement', label: 'Hébergement' },
  { value: 'frais_subsistance', label: 'Frais de subsistance' }
]

export const TYPES_PROGRAMME: { value: 'tous' | TypeProgramme; label: string }[] = [
  { value: 'tous', label: 'Tous les programmes' },
  { value: 'interafricain', label: 'Interafricain' },
  { value: 'hors_afrique', label: 'Hors Afrique vers Afrique' }
]

// Données mock
export const sabbatiquesMock: ProgrammeSabbatique[] = [
  {
    id: 'sab-1',
    titre: 'Programme d\'échange en ingénierie agricole',
    description: 'Venez partager votre expertise en techniques agricoles modernes avec les agriculteurs locaux du Sénégal. Ce programme de 6 mois vous permettra de former des coopératives agricoles aux méthodes durables et à l\'utilisation efficace des ressources en eau. Vous travaillerez avec des équipes locales pour améliorer les rendements tout en préservant l\'environnement.',
    couvertureUrl: 'https://images.unsplash.com/photo-1574943320219-553eb213f72d?w=800',
    pays: 'Sénégal',
    ville: 'Thiès',
    priseEnCharge: ['billet_avion', 'hebergement', 'frais_subsistance'],
    dureeProgramme: '6_mois',
    domaine: 'eau',
    dateHeureDebut: new Date('2026-03-15T09:00:00'),
    dateHeureFin: new Date('2026-09-15T18:00:00'),
    interafricain: true,
    statut: 'ouvert',
    organisateurNom: 'Coopérative Agricole de Thiès',
    organisateurEmail: 'contact@coop-thies.sn',
    createdAt: new Date('2025-12-01'),
    updatedAt: new Date('2025-12-15')
  },
  {
    id: 'sab-2',
    titre: 'Mission santé communautaire en zone rurale',
    description: 'Programme de formation des agents de santé communautaires en Côte d\'Ivoire. Vous participerez à la mise en place de centres de santé de proximité et formerez les personnels locaux aux premiers soins et à la prévention des maladies. Une expérience enrichissante au service des populations rurales.',
    couvertureUrl: 'https://images.unsplash.com/photo-1576091160550-2173dba999ef?w=800',
    pays: 'Côte d\'Ivoire',
    ville: 'Bouaké',
    priseEnCharge: ['hebergement', 'frais_subsistance'],
    dureeProgramme: '3_semaines',
    domaine: 'sante',
    dateHeureDebut: new Date('2026-04-01T08:00:00'),
    dateHeureFin: new Date('2026-04-22T17:00:00'),
    interafricain: true,
    statut: 'ouvert',
    organisateurNom: 'ONG Santé Pour Tous',
    organisateurEmail: 'info@santépourtous.ci',
    createdAt: new Date('2025-11-20'),
    updatedAt: new Date('2025-12-10')
  },
  {
    id: 'sab-3',
    titre: 'Développement de programmes éducatifs au Ghana',
    description: 'Rejoignez notre équipe pour développer des programmes éducatifs innovants dans les écoles primaires du Ghana. Vous travaillerez sur l\'intégration des technologies numériques dans l\'enseignement et formerez les enseignants locaux aux nouvelles méthodes pédagogiques.',
    couvertureUrl: 'https://images.unsplash.com/photo-1503676260728-1c00da094a0b?w=800',
    pays: 'Ghana',
    ville: 'Accra',
    priseEnCharge: ['billet_avion', 'hebergement'],
    dureeProgramme: '2_mois',
    domaine: 'education',
    dateHeureDebut: new Date('2026-05-10T09:00:00'),
    dateHeureFin: new Date('2026-07-10T18:00:00'),
    interafricain: true,
    statut: 'ouvert',
    organisateurNom: 'Ghana Education Initiative',
    organisateurEmail: 'programs@ghedu.org',
    createdAt: new Date('2025-10-15'),
    updatedAt: new Date('2025-11-30')
  },
  {
    id: 'sab-4',
    titre: 'Construction d\'infrastructures scolaires au Mali',
    description: 'Participez à un projet de construction et rénovation d\'écoles dans la région de Ségou. Ce programme combine travail manuel et transfert de compétences en techniques de construction durable. Vous travaillerez avec des artisans locaux et contribuerez à améliorer l\'accès à l\'éducation.',
    couvertureUrl: 'https://images.unsplash.com/photo-1541888946425-d81bb19240f5?w=800',
    pays: 'Mali',
    ville: 'Ségou',
    priseEnCharge: ['hebergement', 'frais_subsistance'],
    dureeProgramme: '1_mois',
    domaine: 'infrastructures',
    dateHeureDebut: new Date('2026-06-01T07:00:00'),
    dateHeureFin: new Date('2026-07-01T18:00:00'),
    interafricain: true,
    statut: 'ouvert',
    organisateurNom: 'Association Bâtisseurs d\'Avenir',
    organisateurEmail: 'contact@batisseurs-avenir.ml',
    createdAt: new Date('2025-09-01'),
    updatedAt: new Date('2025-12-01')
  },
  {
    id: 'sab-5',
    titre: 'Expertise en gestion municipale - France vers Sénégal',
    description: 'Programme d\'échange destiné aux experts français en gestion municipale souhaitant partager leur expérience avec les collectivités locales sénégalaises. Vous accompagnerez les équipes municipales dans la modernisation de leurs services publics et la gestion des ressources.',
    couvertureUrl: 'https://images.unsplash.com/photo-1517245386807-bb43f82c33c4?w=800',
    pays: 'Sénégal',
    ville: 'Saint-Louis',
    priseEnCharge: ['billet_avion', 'hebergement', 'frais_subsistance'],
    dureeProgramme: '6_mois',
    domaine: 'developpement_local',
    dateHeureDebut: new Date('2026-02-01T09:00:00'),
    dateHeureFin: new Date('2026-08-01T18:00:00'),
    interafricain: false,
    statut: 'ouvert',
    organisateurNom: 'Mairie de Saint-Louis',
    organisateurEmail: 'cooperation@mairie-stlouis.sn',
    createdAt: new Date('2025-08-15'),
    updatedAt: new Date('2025-11-20')
  },
  {
    id: 'sab-6',
    titre: 'Formation en énergies renouvelables - USA vers Kenya',
    description: 'Programme d\'échange pour ingénieurs américains spécialisés en énergies renouvelables. Vous formerez des techniciens kényans à l\'installation et la maintenance de systèmes solaires dans les zones rurales non raccordées au réseau électrique.',
    couvertureUrl: 'https://images.unsplash.com/photo-1509391366360-2e959784a276?w=800',
    pays: 'Kenya',
    ville: 'Nairobi',
    priseEnCharge: ['billet_avion', 'hebergement'],
    dureeProgramme: '3_semaines',
    domaine: 'infrastructures',
    dateHeureDebut: new Date('2026-03-10T08:00:00'),
    dateHeureFin: new Date('2026-03-31T17:00:00'),
    interafricain: false,
    statut: 'ouvert',
    organisateurNom: 'Kenya Green Energy Foundation',
    organisateurEmail: 'programs@kgef.org',
    createdAt: new Date('2025-10-01'),
    updatedAt: new Date('2025-12-05')
  },
  {
    id: 'sab-7',
    titre: 'Échange médical Canada-Rwanda',
    description: 'Programme d\'échange pour médecins et infirmiers canadiens souhaitant contribuer au renforcement du système de santé rwandais. Vous travaillerez dans des hôpitaux régionaux et participerez à la formation continue du personnel médical local.',
    couvertureUrl: 'https://images.unsplash.com/photo-1551601651-2a8555f1a136?w=800',
    pays: 'Rwanda',
    ville: 'Kigali',
    priseEnCharge: ['hebergement', 'frais_subsistance'],
    dureeProgramme: '2_mois',
    domaine: 'sante',
    dateHeureDebut: new Date('2026-04-15T08:00:00'),
    dateHeureFin: new Date('2026-06-15T17:00:00'),
    interafricain: false,
    statut: 'en_cours',
    organisateurNom: 'Rwanda Health Partnership',
    organisateurEmail: 'exchange@rhp.rw',
    createdAt: new Date('2025-07-20'),
    updatedAt: new Date('2025-10-15')
  }
]

// Fonctions utilitaires
export const getSabbatiqueById = (id: string): ProgrammeSabbatique | undefined => {
  return sabbatiquesMock.find(s => s.id === id)
}

export const getSabbatiquesInterafricains = (): ProgrammeSabbatique[] => {
  return sabbatiquesMock.filter(s => s.interafricain)
}

export const getSabbatiquesHorsAfrique = (): ProgrammeSabbatique[] => {
  return sabbatiquesMock.filter(s => !s.interafricain)
}

export const rechercherSabbatiques = (filtres: Partial<FiltresSabbatique>): ProgrammeSabbatique[] => {
  return sabbatiquesMock.filter(programme => {
    // Filtre par type
    if (filtres.type && filtres.type !== 'tous') {
      const estInterafricain = filtres.type === 'interafricain'
      if (programme.interafricain !== estInterafricain) {
        return false
      }
    }

    // Filtre par pays
    if (filtres.pays && filtres.pays !== '') {
      if (programme.pays !== filtres.pays) {
        return false
      }
    }

    // Filtre par domaine
    if (filtres.domaine && filtres.domaine !== '') {
      if (programme.domaine !== filtres.domaine) {
        return false
      }
    }

    // Filtre par recherche textuelle
    if (filtres.recherche && filtres.recherche.trim() !== '') {
      const searchLower = filtres.recherche.toLowerCase()
      const matchTitre = programme.titre.toLowerCase().includes(searchLower)
      const matchDescription = programme.description.toLowerCase().includes(searchLower)
      const matchPays = programme.pays.toLowerCase().includes(searchLower)
      if (!matchTitre && !matchDescription && !matchPays) {
        return false
      }
    }

    return true
  })
}

// Formatage de date
export const formatDateSabbatique = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    weekday: 'long',
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  }).format(new Date(date))
}

export const formatDateCourteSabbatique = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(new Date(date))
}

export const formatHeureSabbatique = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(date))
}

export const getDureeLabel = (duree: DureeProgramme): string => {
  const found = DUREES.find(d => d.value === duree)
  return found ? found.label : duree
}

export const getDomaineLabel = (domaine: Domaine): string => {
  const found = DOMAINES.find(d => d.value === domaine)
  return found ? found.label : domaine
}

export const getPriseEnChargeLabels = (prises: PriseEnCharge[]): string[] => {
  return prises.map(p => {
    const found = PRISES_EN_CHARGE.find(pc => pc.value === p)
    return found ? found.label : p
  })
}

// Stats
export const getStatsSabbatiques = () => {
  const total = sabbatiquesMock.length
  const interafricain = getSabbatiquesInterafricains().length
  const horsAfrique = getSabbatiquesHorsAfrique().length
  const ouverts = sabbatiquesMock.filter(s => s.statut === 'ouvert').length

  return {
    total,
    interafricain,
    horsAfrique,
    ouverts
  }
}

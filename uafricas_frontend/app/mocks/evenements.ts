// Donnees mock pour les evenements

export type TypeEvenement = 'En ligne' | 'En présentiel' | 'Hybride'
export type StatutEvenement = 'a_venir' | 'en_cours' | 'termine' | 'annule'

export interface UserInfo {
  uid: string
  email: string
  nom: string
  prenom: string
  photo_url: string | null
}

export interface Inscription {
  user_uid: string
  email: string
  nom: string
  prenom: string
  photo_url: string | null
  created_at: Date
  updated_at: Date
}

export interface Evenement {
  id: string
  titre: string
  description: string
  type: TypeEvenement
  pays: string
  ville: string
  date_heure_debut: Date
  date_heure_fin: Date
  couverture_url: string
  statut: StatutEvenement
  user: UserInfo
  inscriptions?: Inscription[]
  created_at: Date
  updated_at: Date
}

// Constantes pour les filtres
export const TYPES_EVENEMENT: { value: TypeEvenement | ''; label: string }[] = [
  { value: '', label: 'Tous les types' },
  { value: 'En ligne', label: 'En ligne' },
  { value: 'En présentiel', label: 'En présentiel' },
  { value: 'Hybride', label: 'Hybride' }
]

export const ANNEES = ['2025', '2026', '2027', '2028']

export const PAYS_AFRICAINS = [
  'Afrique du Sud',
  'Algérie',
  'Bénin',
  'Burkina Faso',
  'Cameroun',
  'Cap-Vert',
  'Comores',
  'Côte d\'Ivoire',
  'Égypte',
  'Éthiopie',
  'Gabon',
  'Gambie',
  'Ghana',
  'Guinée',
  'Kenya',
  'Madagascar',
  'Mali',
  'Maroc',
  'Maurice',
  'Mauritanie',
  'Namibie',
  'Niger',
  'Nigeria',
  'RDC',
  'Rwanda',
  'Sénégal',
  'Tanzanie',
  'Togo',
  'Tunisie'
]

// Donnees mock des evenements
export const evenementsMock: Evenement[] = [
  {
    id: 'evt-001',
    titre: 'ForAfrica - Forum des Valeurs Africaines',
    description: 'Forum annuel de discussion sur les valeurs africaines traditionnelles et leur pertinence dans le monde moderne. Échanges entre experts, chercheurs et acteurs culturels.',
    type: 'En présentiel',
    pays: 'Mali',
    ville: 'Bamako',
    date_heure_debut: new Date('2025-03-15T09:00:00'),
    date_heure_fin: new Date('2025-03-15T17:00:00'),
    couverture_url: 'https://images.unsplash.com/photo-1540575467063-178a50c2df87?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-001',
      email: 'amadou.diallo@example.com',
      nom: 'Diallo',
      prenom: 'Amadou',
      photo_url: 'https://randomuser.me/api/portraits/men/1.jpg'
    },
    created_at: new Date('2025-01-10'),
    updated_at: new Date('2025-01-10')
  },
  {
    id: 'evt-002',
    titre: 'Webinaire: Langues africaines à l\'ère numérique',
    description: 'Découvrez comment les technologies numériques peuvent contribuer à la préservation et à la promotion des langues africaines. Session interactive avec démonstrations.',
    type: 'En ligne',
    pays: 'Sénégal',
    ville: 'Dakar',
    date_heure_debut: new Date('2025-04-20T14:00:00'),
    date_heure_fin: new Date('2025-04-20T16:30:00'),
    couverture_url: 'https://images.unsplash.com/photo-1591115765373-5207764f72e7?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-002',
      email: 'fatou.ndiaye@example.com',
      nom: 'Ndiaye',
      prenom: 'Fatou',
      photo_url: 'https://randomuser.me/api/portraits/women/2.jpg'
    },
    created_at: new Date('2025-01-15'),
    updated_at: new Date('2025-01-15')
  },
  {
    id: 'evt-003',
    titre: 'Atelier de musique traditionnelle Mandingue',
    description: 'Initiation aux instruments traditionnels mandingues: kora, balafon et djembé. Atelier pratique animé par des griots de renommée internationale.',
    type: 'En présentiel',
    pays: 'Côte d\'Ivoire',
    ville: 'Abidjan',
    date_heure_debut: new Date('2025-05-10T10:00:00'),
    date_heure_fin: new Date('2025-05-10T18:00:00'),
    couverture_url: 'https://images.unsplash.com/photo-1516450360452-9312f5e86fc7?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-003',
      email: 'kouame.yao@example.com',
      nom: 'Yao',
      prenom: 'Kouamé',
      photo_url: 'https://randomuser.me/api/portraits/men/3.jpg'
    },
    created_at: new Date('2025-01-20'),
    updated_at: new Date('2025-01-20')
  },
  {
    id: 'evt-004',
    titre: 'Conférence: Économie circulaire en Afrique',
    description: 'Table ronde sur les modèles économiques durables inspirés des pratiques traditionnelles africaines. Intervenants de 10 pays africains.',
    type: 'Hybride',
    pays: 'Kenya',
    ville: 'Nairobi',
    date_heure_debut: new Date('2025-06-05T08:00:00'),
    date_heure_fin: new Date('2025-06-06T17:00:00'),
    couverture_url: 'https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-004',
      email: 'james.ochieng@example.com',
      nom: 'Ochieng',
      prenom: 'James',
      photo_url: 'https://randomuser.me/api/portraits/men/4.jpg'
    },
    created_at: new Date('2025-01-25'),
    updated_at: new Date('2025-01-25')
  },
  {
    id: 'evt-005',
    titre: 'Festival de la Gastronomie Africaine',
    description: 'Célébration des saveurs du continent avec des chefs venus de toute l\'Afrique. Dégustations, ateliers culinaires et échanges culturels.',
    type: 'En présentiel',
    pays: 'Cameroun',
    ville: 'Douala',
    date_heure_debut: new Date('2025-07-20T11:00:00'),
    date_heure_fin: new Date('2025-07-22T20:00:00'),
    couverture_url: 'https://images.unsplash.com/photo-1504674900247-0877df9cc836?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-005',
      email: 'marie.tabi@example.com',
      nom: 'Tabi',
      prenom: 'Marie',
      photo_url: 'https://randomuser.me/api/portraits/women/5.jpg'
    },
    created_at: new Date('2025-01-28'),
    updated_at: new Date('2025-01-28')
  },
  {
    id: 'evt-006',
    titre: 'Hackathon: Solutions IA pour l\'Afrique',
    description: 'Compétition de développement de solutions d\'intelligence artificielle répondant aux défis africains. Prix et mentorat pour les gagnants.',
    type: 'Hybride',
    pays: 'Nigeria',
    ville: 'Lagos',
    date_heure_debut: new Date('2025-08-15T09:00:00'),
    date_heure_fin: new Date('2025-08-17T18:00:00'),
    couverture_url: 'https://images.unsplash.com/photo-1531482615713-2afd69097998?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-006',
      email: 'chukwuemeka@example.com',
      nom: 'Okonkwo',
      prenom: 'Chukwuemeka',
      photo_url: 'https://randomuser.me/api/portraits/men/6.jpg'
    },
    created_at: new Date('2025-01-30'),
    updated_at: new Date('2025-01-30')
  },
  {
    id: 'evt-007',
    titre: 'Séminaire: Leadership féminin en Afrique',
    description: 'Rencontre inspirante avec des femmes leaders africaines. Partage d\'expériences et réseautage pour la nouvelle génération.',
    type: 'En ligne',
    pays: 'Ghana',
    ville: 'Accra',
    date_heure_debut: new Date('2025-09-08T15:00:00'),
    date_heure_fin: new Date('2025-09-08T18:00:00'),
    couverture_url: 'https://images.unsplash.com/photo-1573164713988-8665fc963095?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-007',
      email: 'abena.mensah@example.com',
      nom: 'Mensah',
      prenom: 'Abena',
      photo_url: 'https://randomuser.me/api/portraits/women/7.jpg'
    },
    created_at: new Date('2025-02-01'),
    updated_at: new Date('2025-02-01')
  },
  {
    id: 'evt-008',
    titre: 'Exposition: Art contemporain africain',
    description: 'Vernissage et exposition d\'artistes africains contemporains explorant l\'identité et la modernité à travers leurs œuvres.',
    type: 'En présentiel',
    pays: 'Afrique du Sud',
    ville: 'Johannesburg',
    date_heure_debut: new Date('2025-10-12T18:00:00'),
    date_heure_fin: new Date('2025-10-30T20:00:00'),
    couverture_url: 'https://images.unsplash.com/photo-1544967082-d9d25d867d66?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-008',
      email: 'sipho.ndlovu@example.com',
      nom: 'Ndlovu',
      prenom: 'Sipho',
      photo_url: 'https://randomuser.me/api/portraits/men/8.jpg'
    },
    created_at: new Date('2025-02-05'),
    updated_at: new Date('2025-02-05')
  },
  {
    id: 'evt-009',
    titre: 'Formation: Entrepreneuriat social',
    description: 'Programme intensif de formation pour entrepreneurs sociaux africains. Méthodologie, financement et mise en réseau.',
    type: 'Hybride',
    pays: 'Burkina Faso',
    ville: 'Ouagadougou',
    date_heure_debut: new Date('2026-01-15T08:00:00'),
    date_heure_fin: new Date('2026-01-20T17:00:00'),
    couverture_url: 'https://images.unsplash.com/photo-1552664730-d307ca884978?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-009',
      email: 'ousmane.ouedraogo@example.com',
      nom: 'Ouédraogo',
      prenom: 'Ousmane',
      photo_url: 'https://randomuser.me/api/portraits/men/9.jpg'
    },
    created_at: new Date('2025-02-10'),
    updated_at: new Date('2025-02-10')
  },
  {
    id: 'evt-010',
    titre: 'Colloque: Médecine traditionnelle africaine',
    description: 'Échanges entre praticiens de la médecine traditionnelle et chercheurs modernes sur l\'intégration des savoirs ancestraux.',
    type: 'En présentiel',
    pays: 'RDC',
    ville: 'Kinshasa',
    date_heure_debut: new Date('2026-02-20T09:00:00'),
    date_heure_fin: new Date('2026-02-22T16:00:00'),
    couverture_url: 'https://images.unsplash.com/photo-1576091160550-2173dba999ef?w=800',
    statut: 'a_venir',
    user: {
      uid: 'user-010',
      email: 'grace.mbuyi@example.com',
      nom: 'Mbuyi',
      prenom: 'Grace',
      photo_url: 'https://randomuser.me/api/portraits/women/10.jpg'
    },
    created_at: new Date('2025-02-15'),
    updated_at: new Date('2025-02-15')
  }
]

// Fonctions utilitaires

export const getAllEvenements = (): Evenement[] => {
  return [...evenementsMock]
}

export const getEvenementById = (id: string): Evenement | undefined => {
  return evenementsMock.find(e => e.id === id)
}

export const filterEvenements = (
  annee?: string,
  type?: TypeEvenement | '',
  pays?: string
): Evenement[] => {
  return evenementsMock.filter(event => {
    const eventYear = event.date_heure_debut.getFullYear().toString()
    const yearMatch = !annee || eventYear === annee
    const typeMatch = !type || event.type === type
    const paysMatch = !pays || event.pays === pays
    return yearMatch && typeMatch && paysMatch
  })
}

export const getEvenementsByStatut = (statut: StatutEvenement): Evenement[] => {
  return evenementsMock.filter(e => e.statut === statut)
}

export const getUpcomingEvenements = (): Evenement[] => {
  const now = new Date()
  return evenementsMock
    .filter(e => e.date_heure_debut > now)
    .sort((a, b) => a.date_heure_debut.getTime() - b.date_heure_debut.getTime())
}

export const searchEvenements = (term: string): Evenement[] => {
  const searchLower = term.toLowerCase().trim()
  if (!searchLower) return getAllEvenements()
  return evenementsMock.filter(e =>
    e.titre.toLowerCase().includes(searchLower) ||
    e.description.toLowerCase().includes(searchLower) ||
    e.pays.toLowerCase().includes(searchLower) ||
    e.ville.toLowerCase().includes(searchLower)
  )
}

export const formatDate = (date: Date): string => {
  return date.toLocaleDateString('fr-FR', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
}

export const formatDateShort = (date: Date): string => {
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  })
}

export const getHeure = (date: Date): string => {
  return date.toLocaleTimeString('fr-FR', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  })
}

export const getStats = () => {
  const now = new Date()
  return {
    total: evenementsMock.length,
    aVenir: evenementsMock.filter(e => e.date_heure_debut > now).length,
    enLigne: evenementsMock.filter(e => e.type === 'En ligne').length,
    presentiel: evenementsMock.filter(e => e.type === 'En présentiel').length,
    hybride: evenementsMock.filter(e => e.type === 'Hybride').length
  }
}

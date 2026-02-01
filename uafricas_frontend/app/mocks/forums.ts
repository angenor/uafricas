// Donnees mock pour les forums de discussion

export type TypeForum = 'Proverbe' | 'Citation' | 'Bonne pratique' | 'Histoire'

export interface UserInfo {
  uid: string
  email: string
  nom: string
  prenom: string
  photo_url: string | null
}

export interface ProverbeData {
  libellet: string | null
  explication: string | null
  auteur: string | null
  thematique: string | null
  pays: string
  village?: string
  photo_url: string | null
}

export interface CitationData {
  libellet: string | null
  auteur: string | null
  pays: string
  explication: string | null
}

export interface BonnePratiqueData {
  pays: string
  domaine: string
  description: string | null
  photo_url: string | null
}

export interface HistoireData {
  description: string | null
  pays: string
  source: string | null
  photo_url: string | null
}

export interface Forum {
  id: string
  type: TypeForum
  couleurSelected: string
  user: UserInfo
  proverbe: ProverbeData
  citation: CitationData
  bonne_pratique: BonnePratiqueData
  histoire: HistoireData
  statut: number
  likes: number
  vues: number
  created_at: Date
  updated_at: Date
}

export interface ForumComment {
  id: string
  forumId: string
  content: string
  user: UserInfo
  created_at: Date
  likes: number
}

// Constantes
export const TYPES_FORUM: { value: TypeForum | ''; label: string }[] = [
  { value: '', label: 'Tous les types' },
  { value: 'Proverbe', label: 'Proverbes' },
  { value: 'Citation', label: 'Citations' },
  { value: 'Bonne pratique', label: 'Bonnes pratiques' },
  { value: 'Histoire', label: 'Histoires' }
]

export const COULEURS_FOND = [
  'bg-red-600',
  'bg-blue-600',
  'bg-green-600',
  'bg-purple-600',
  'bg-orange-600',
  'bg-teal-600',
  'bg-indigo-600',
  'bg-pink-600'
]

export const PAYS = [
  'Sénégal', 'Mali', 'Côte d\'Ivoire', 'Burkina Faso', 'Ghana',
  'Nigeria', 'Cameroun', 'RDC', 'Kenya', 'Afrique du Sud'
]

export const ETHNIES = [
  'Wolof', 'Peul', 'Sérère', 'Bambara', 'Mossi', 'Akan',
  'Yoruba', 'Igbo', 'Bamiléké', 'Zoulou', 'Kikuyu'
]

export const THEMATIQUES = [
  'Sagesse', 'Famille', 'Travail', 'Amour', 'Justice',
  'Éducation', 'Solidarité', 'Courage', 'Patience', 'Respect'
]

// Donnees mock des forums
export const forumsMock: Forum[] = [
  {
    id: 'forum-001',
    type: 'Proverbe',
    couleurSelected: 'bg-red-600',
    user: {
      uid: 'user-001',
      email: 'amadou.diallo@example.com',
      nom: 'Diallo',
      prenom: 'Amadou',
      photo_url: 'https://randomuser.me/api/portraits/men/1.jpg'
    },
    proverbe: {
      libellet: 'Quand on ne sait pas où l\'on va, il faut savoir d\'où l\'on vient.',
      explication: 'Ce proverbe africain nous rappelle l\'importance de connaître nos racines et notre histoire pour mieux appréhender notre avenir. Sans la connaissance de notre passé, nous risquons de nous perdre.',
      auteur: null,
      thematique: 'Sagesse',
      pays: 'Sénégal',
      village: 'Thiès',
      photo_url: 'https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?w=800'
    },
    citation: { libellet: null, auteur: null, pays: '', explication: null },
    bonne_pratique: { pays: '', domaine: '', description: null, photo_url: null },
    histoire: { description: null, pays: '', source: null, photo_url: null },
    statut: 1,
    likes: 45,
    vues: 230,
    created_at: new Date('2025-01-10'),
    updated_at: new Date('2025-01-10')
  },
  {
    id: 'forum-002',
    type: 'Citation',
    couleurSelected: 'bg-blue-600',
    user: {
      uid: 'user-002',
      email: 'fatou.ndiaye@example.com',
      nom: 'Ndiaye',
      prenom: 'Fatou',
      photo_url: 'https://randomuser.me/api/portraits/women/2.jpg'
    },
    proverbe: { libellet: null, explication: null, auteur: null, thematique: null, pays: '', photo_url: null },
    citation: {
      libellet: 'L\'éducation est l\'arme la plus puissante pour changer le monde.',
      auteur: 'Nelson Mandela',
      pays: 'Afrique du Sud',
      explication: 'Cette citation de Mandela souligne le pouvoir transformateur de l\'éducation dans la lutte contre l\'ignorance et l\'injustice.'
    },
    bonne_pratique: { pays: '', domaine: '', description: null, photo_url: null },
    histoire: { description: null, pays: '', source: null, photo_url: null },
    statut: 1,
    likes: 78,
    vues: 412,
    created_at: new Date('2025-01-12'),
    updated_at: new Date('2025-01-12')
  },
  {
    id: 'forum-003',
    type: 'Bonne pratique',
    couleurSelected: 'bg-green-600',
    user: {
      uid: 'user-003',
      email: 'kouame.yao@example.com',
      nom: 'Yao',
      prenom: 'Kouamé',
      photo_url: 'https://randomuser.me/api/portraits/men/3.jpg'
    },
    proverbe: { libellet: null, explication: null, auteur: null, thematique: null, pays: '', photo_url: null },
    citation: { libellet: null, auteur: null, pays: '', explication: null },
    bonne_pratique: {
      pays: 'Côte d\'Ivoire',
      domaine: 'Agriculture',
      description: 'Le système de la "tontine agricole" permet aux agriculteurs de mutualiser leurs ressources pour acheter des semences et du matériel. Chaque membre contribue selon ses moyens et bénéficie à tour de rôle.',
      photo_url: 'https://images.unsplash.com/photo-1500651230702-0e2d8a49d4ad?w=800'
    },
    histoire: { description: null, pays: '', source: null, photo_url: null },
    statut: 1,
    likes: 34,
    vues: 156,
    created_at: new Date('2025-01-15'),
    updated_at: new Date('2025-01-15')
  },
  {
    id: 'forum-004',
    type: 'Histoire',
    couleurSelected: 'bg-purple-600',
    user: {
      uid: 'user-004',
      email: 'james.ochieng@example.com',
      nom: 'Ochieng',
      prenom: 'James',
      photo_url: 'https://randomuser.me/api/portraits/men/4.jpg'
    },
    proverbe: { libellet: null, explication: null, auteur: null, thematique: null, pays: '', photo_url: null },
    citation: { libellet: null, auteur: null, pays: '', explication: null },
    bonne_pratique: { pays: '', domaine: '', description: null, photo_url: null },
    histoire: {
      description: 'L\'Empire du Mali, fondé au XIIIe siècle par Soundiata Keïta, fut l\'un des plus grands empires d\'Afrique de l\'Ouest. Sa richesse légendaire atteignit son apogée sous Mansa Moussa, considéré comme l\'homme le plus riche de l\'histoire.',
      pays: 'Mali',
      source: 'Tradition orale et chroniques arabes',
      photo_url: 'https://images.unsplash.com/photo-1489749798305-4fea3ae63d43?w=800'
    },
    statut: 1,
    likes: 92,
    vues: 534,
    created_at: new Date('2025-01-18'),
    updated_at: new Date('2025-01-18')
  },
  {
    id: 'forum-005',
    type: 'Proverbe',
    couleurSelected: 'bg-orange-600',
    user: {
      uid: 'user-005',
      email: 'marie.tabi@example.com',
      nom: 'Tabi',
      prenom: 'Marie',
      photo_url: 'https://randomuser.me/api/portraits/women/5.jpg'
    },
    proverbe: {
      libellet: 'L\'enfant qui pose des questions ne s\'égare pas en chemin.',
      explication: 'Ce proverbe nous enseigne l\'importance de la curiosité et de l\'humilité. Poser des questions est signe de sagesse, pas de faiblesse.',
      auteur: null,
      thematique: 'Éducation',
      pays: 'Cameroun',
      photo_url: 'https://images.unsplash.com/photo-1503676260728-1c00da094a0b?w=800'
    },
    citation: { libellet: null, auteur: null, pays: '', explication: null },
    bonne_pratique: { pays: '', domaine: '', description: null, photo_url: null },
    histoire: { description: null, pays: '', source: null, photo_url: null },
    statut: 1,
    likes: 56,
    vues: 289,
    created_at: new Date('2025-01-20'),
    updated_at: new Date('2025-01-20')
  },
  {
    id: 'forum-006',
    type: 'Citation',
    couleurSelected: 'bg-teal-600',
    user: {
      uid: 'user-006',
      email: 'chukwuemeka@example.com',
      nom: 'Okonkwo',
      prenom: 'Chukwuemeka',
      photo_url: 'https://randomuser.me/api/portraits/men/6.jpg'
    },
    proverbe: { libellet: null, explication: null, auteur: null, thematique: null, pays: '', photo_url: null },
    citation: {
      libellet: 'Je ne suis pas africain parce que je suis né en Afrique, mais parce que l\'Afrique est née en moi.',
      auteur: 'Kwame Nkrumah',
      pays: 'Ghana',
      explication: 'Cette citation puissante de Nkrumah exprime l\'idée que l\'identité africaine va au-delà de la géographie.'
    },
    bonne_pratique: { pays: '', domaine: '', description: null, photo_url: null },
    histoire: { description: null, pays: '', source: null, photo_url: null },
    statut: 1,
    likes: 123,
    vues: 678,
    created_at: new Date('2025-01-22'),
    updated_at: new Date('2025-01-22')
  },
  {
    id: 'forum-007',
    type: 'Bonne pratique',
    couleurSelected: 'bg-indigo-600',
    user: {
      uid: 'user-007',
      email: 'abena.mensah@example.com',
      nom: 'Mensah',
      prenom: 'Abena',
      photo_url: 'https://randomuser.me/api/portraits/women/7.jpg'
    },
    proverbe: { libellet: null, explication: null, auteur: null, thematique: null, pays: '', photo_url: null },
    citation: { libellet: null, auteur: null, pays: '', explication: null },
    bonne_pratique: {
      pays: 'Ghana',
      domaine: 'Solidarité',
      description: 'Le système "Susu" est une forme traditionnelle d\'épargne collective. Les participants versent régulièrement une somme fixe, et chacun reçoit la totalité des contributions à tour de rôle.',
      photo_url: 'https://images.unsplash.com/photo-1532629345422-7515f3d16bb6?w=800'
    },
    histoire: { description: null, pays: '', source: null, photo_url: null },
    statut: 1,
    likes: 67,
    vues: 345,
    created_at: new Date('2025-01-25'),
    updated_at: new Date('2025-01-25')
  },
  {
    id: 'forum-008',
    type: 'Proverbe',
    couleurSelected: 'bg-pink-600',
    user: {
      uid: 'user-008',
      email: 'sipho.ndlovu@example.com',
      nom: 'Ndlovu',
      prenom: 'Sipho',
      photo_url: 'https://randomuser.me/api/portraits/men/8.jpg'
    },
    proverbe: {
      libellet: 'Umuntu ngumuntu ngabantu - Je suis parce que nous sommes.',
      explication: 'Cette philosophie Ubuntu sud-africaine exprime l\'interconnexion de tous les êtres humains. Notre humanité se définit par notre relation aux autres.',
      auteur: null,
      thematique: 'Solidarité',
      pays: 'Afrique du Sud',
      photo_url: 'https://images.unsplash.com/photo-1529156069898-49953e39b3ac?w=800'
    },
    citation: { libellet: null, auteur: null, pays: '', explication: null },
    bonne_pratique: { pays: '', domaine: '', description: null, photo_url: null },
    histoire: { description: null, pays: '', source: null, photo_url: null },
    statut: 1,
    likes: 156,
    vues: 823,
    created_at: new Date('2025-01-28'),
    updated_at: new Date('2025-01-28')
  }
]

// Commentaires mock
export const commentsMock: ForumComment[] = [
  {
    id: 'comment-001',
    forumId: 'forum-001',
    content: 'Ce proverbe m\'a été transmis par ma grand-mère. Il guide ma vie depuis toujours.',
    user: {
      uid: 'user-010',
      email: 'moussa@example.com',
      nom: 'Traoré',
      prenom: 'Moussa',
      photo_url: 'https://randomuser.me/api/portraits/men/10.jpg'
    },
    created_at: new Date('2025-01-11'),
    likes: 12
  },
  {
    id: 'comment-002',
    forumId: 'forum-001',
    content: 'Magnifique sagesse de nos ancêtres. Merci pour ce partage !',
    user: {
      uid: 'user-011',
      email: 'awa@example.com',
      nom: 'Diop',
      prenom: 'Awa',
      photo_url: 'https://randomuser.me/api/portraits/women/11.jpg'
    },
    created_at: new Date('2025-01-12'),
    likes: 8
  },
  {
    id: 'comment-003',
    forumId: 'forum-002',
    content: 'Mandela était vraiment un visionnaire. Cette citation reste d\'actualité.',
    user: {
      uid: 'user-012',
      email: 'jean@example.com',
      nom: 'Mbeki',
      prenom: 'Jean',
      photo_url: 'https://randomuser.me/api/portraits/men/12.jpg'
    },
    created_at: new Date('2025-01-13'),
    likes: 23
  },
  {
    id: 'comment-004',
    forumId: 'forum-004',
    content: 'L\'histoire de l\'Empire du Mali devrait être enseignée dans toutes les écoles africaines.',
    user: {
      uid: 'user-013',
      email: 'ibrahima@example.com',
      nom: 'Keïta',
      prenom: 'Ibrahima',
      photo_url: 'https://randomuser.me/api/portraits/men/13.jpg'
    },
    created_at: new Date('2025-01-19'),
    likes: 45
  },
  {
    id: 'comment-005',
    forumId: 'forum-008',
    content: 'Ubuntu est une philosophie de vie extraordinaire. Elle nous rappelle notre humanité commune.',
    user: {
      uid: 'user-014',
      email: 'grace@example.com',
      nom: 'Muthoni',
      prenom: 'Grace',
      photo_url: 'https://randomuser.me/api/portraits/women/14.jpg'
    },
    created_at: new Date('2025-01-29'),
    likes: 34
  }
]

// Fonctions utilitaires

export const getAllForums = (): Forum[] => {
  return [...forumsMock]
}

export const getForumById = (id: string): Forum | undefined => {
  return forumsMock.find(f => f.id === id)
}

export const getForumsByType = (type: TypeForum): Forum[] => {
  return forumsMock.filter(f => f.type === type)
}

export const getForumComments = (forumId: string): ForumComment[] => {
  return commentsMock.filter(c => c.forumId === forumId)
}

export const getCommentsCount = (forumId: string): number => {
  return getForumComments(forumId).length
}

export const filterForums = (
  type?: TypeForum | '',
  pays?: string,
  ethnie?: string
): Forum[] => {
  return forumsMock.filter(forum => {
    const typeMatch = !type || forum.type === type

    let paysMatch = true
    if (pays) {
      if (forum.type === 'Proverbe') paysMatch = forum.proverbe.pays === pays
      else if (forum.type === 'Citation') paysMatch = forum.citation.pays === pays
      else if (forum.type === 'Bonne pratique') paysMatch = forum.bonne_pratique.pays === pays
      else if (forum.type === 'Histoire') paysMatch = forum.histoire.pays === pays
    }

    return typeMatch && paysMatch
  })
}

export const searchForums = (term: string): Forum[] => {
  const searchLower = term.toLowerCase().trim()
  if (!searchLower) return getAllForums()

  return forumsMock.filter(forum => {
    const proverbeMatch = forum.proverbe.libellet?.toLowerCase().includes(searchLower) ||
                          forum.proverbe.explication?.toLowerCase().includes(searchLower)
    const citationMatch = forum.citation.libellet?.toLowerCase().includes(searchLower) ||
                          forum.citation.auteur?.toLowerCase().includes(searchLower)
    const pratiqueMatch = forum.bonne_pratique.description?.toLowerCase().includes(searchLower)
    const histoireMatch = forum.histoire.description?.toLowerCase().includes(searchLower)

    return proverbeMatch || citationMatch || pratiqueMatch || histoireMatch
  })
}

export const formatDate = (date: Date): string => {
  const now = new Date()
  const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60))

  if (diffInMinutes < 1) return 'À l\'instant'
  if (diffInMinutes < 60) return `Il y a ${diffInMinutes} min`
  if (diffInMinutes < 1440) return `Il y a ${Math.floor(diffInMinutes / 60)} h`

  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: date.getFullYear() !== now.getFullYear() ? 'numeric' : undefined
  })
}

export const getStats = () => {
  return {
    total: forumsMock.length,
    proverbes: forumsMock.filter(f => f.type === 'Proverbe').length,
    citations: forumsMock.filter(f => f.type === 'Citation').length,
    bonnesPratiques: forumsMock.filter(f => f.type === 'Bonne pratique').length,
    histoires: forumsMock.filter(f => f.type === 'Histoire').length,
    totalLikes: forumsMock.reduce((sum, f) => sum + f.likes, 0),
    totalVues: forumsMock.reduce((sum, f) => sum + f.vues, 0)
  }
}

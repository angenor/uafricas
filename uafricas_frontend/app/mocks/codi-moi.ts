// Donnees mock pour CodiMoi - Codification des valeurs africaines

export type CategoriePost = 'proverbe_adage' | 'citation' | 'ressource_historique' | 'bonne_pratique'
export type UserReaction = 'like' | 'dislike' | null

export interface UserInfo {
  uid: string
  email: string
  nom: string
  prenom: string
  photoURL: string | null
}

export interface PostStats {
  likes: number
  dislikes: number
  commentaires: number
  partages: number
  vues: number
}

export interface CodiMoiPost {
  id: string
  categorie: CategoriePost
  contenu: string
  nomAuteur: string | null
  explication: string | null
  couleurFond: string | null
  imageArrierePlan: string | null
  imageCouverture: string | null
  pays: string
  groupeEthnique: string | null
  hashtags: string[]
  userId: string
  userInfo: UserInfo
  dateCreation: Date
  dateModification: Date
  stats: PostStats
  userReaction: UserReaction
}

export interface CodiMoiComment {
  id: string
  postId: string
  contenu: string
  userId: string
  userInfo: UserInfo
  dateCreation: Date
  likes: number
}

// Constantes
export const CATEGORIES_POST: { value: CategoriePost | ''; label: string; icon: string }[] = [
  { value: '', label: 'Tout', icon: 'fa-solid fa-globe' },
  { value: 'proverbe_adage', label: 'Proverbes & Adages', icon: 'fa-solid fa-quote-left' },
  { value: 'citation', label: 'Citations', icon: 'fa-solid fa-quote-right' },
  { value: 'ressource_historique', label: 'Histoire', icon: 'fa-solid fa-landmark' },
  { value: 'bonne_pratique', label: 'Bonnes pratiques', icon: 'fa-solid fa-lightbulb' }
]

export const COULEURS_FOND = [
  '#2D5A27', // Vert forêt
  '#8B4513', // Marron
  '#1E3A5F', // Bleu marine
  '#6B2C5B', // Bordeaux
  '#B8860B', // Or foncé
  '#2F4F4F', // Gris ardoise
  '#800020', // Bourgogne
  '#004D40'  // Vert émeraude foncé
]

export const PAYS_AFRICAINS = [
  'Sénégal', 'Mali', 'Côte d\'Ivoire', 'Burkina Faso', 'Ghana',
  'Nigeria', 'Cameroun', 'RDC', 'Kenya', 'Afrique du Sud',
  'Éthiopie', 'Tanzanie', 'Maroc', 'Algérie', 'Égypte'
]

export const GROUPES_ETHNIQUES = [
  'Wolof', 'Peul', 'Sérère', 'Bambara', 'Mossi', 'Akan',
  'Yoruba', 'Igbo', 'Bamiléké', 'Zoulou', 'Kikuyu', 'Oromo',
  'Amazigh', 'Haoussa', 'Mandingue', 'Dioula'
]

// Donnees mock des posts
export const codiMoiPostsMock: CodiMoiPost[] = [
  {
    id: 'post-001',
    categorie: 'proverbe_adage',
    contenu: 'Quand on ne sait pas où l\'on va, il faut savoir d\'où l\'on vient.',
    nomAuteur: null,
    explication: 'Ce proverbe africain nous rappelle l\'importance de connaître nos racines et notre histoire pour mieux appréhender notre avenir.',
    couleurFond: '#2D5A27',
    imageArrierePlan: 'https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?w=800',
    imageCouverture: null,
    pays: 'Sénégal',
    groupeEthnique: 'Wolof',
    hashtags: ['sagesse', 'racines', 'identité'],
    userId: 'user-001',
    userInfo: {
      uid: 'user-001',
      email: 'amadou.diallo@example.com',
      nom: 'Diallo',
      prenom: 'Amadou',
      photoURL: 'https://randomuser.me/api/portraits/men/1.jpg'
    },
    dateCreation: new Date('2025-01-10'),
    dateModification: new Date('2025-01-10'),
    stats: { likes: 156, dislikes: 3, commentaires: 24, partages: 45, vues: 1234 },
    userReaction: null
  },
  {
    id: 'post-002',
    categorie: 'citation',
    contenu: 'L\'éducation est l\'arme la plus puissante pour changer le monde.',
    nomAuteur: 'Nelson Mandela',
    explication: 'Cette citation de Mandela souligne le pouvoir transformateur de l\'éducation dans la lutte contre l\'ignorance et l\'injustice.',
    couleurFond: '#1E3A5F',
    imageArrierePlan: null,
    imageCouverture: null,
    pays: 'Afrique du Sud',
    groupeEthnique: 'Xhosa',
    hashtags: ['mandela', 'éducation', 'changement'],
    userId: 'user-002',
    userInfo: {
      uid: 'user-002',
      email: 'fatou.ndiaye@example.com',
      nom: 'Ndiaye',
      prenom: 'Fatou',
      photoURL: 'https://randomuser.me/api/portraits/women/2.jpg'
    },
    dateCreation: new Date('2025-01-12'),
    dateModification: new Date('2025-01-12'),
    stats: { likes: 289, dislikes: 2, commentaires: 56, partages: 123, vues: 2456 },
    userReaction: 'like'
  },
  {
    id: 'post-003',
    categorie: 'bonne_pratique',
    contenu: 'La tontine : un système d\'épargne collective ancestral',
    nomAuteur: null,
    explication: 'La tontine est un système d\'épargne rotative où les membres contribuent régulièrement et reçoivent à tour de rôle la totalité des contributions. Ce système renforce la solidarité communautaire et permet l\'accès au crédit sans intérêt.',
    couleurFond: null,
    imageArrierePlan: null,
    imageCouverture: 'https://images.unsplash.com/photo-1532629345422-7515f3d16bb6?w=800',
    pays: 'Côte d\'Ivoire',
    groupeEthnique: 'Akan',
    hashtags: ['tontine', 'épargne', 'solidarité', 'économie'],
    userId: 'user-003',
    userInfo: {
      uid: 'user-003',
      email: 'kouame.yao@example.com',
      nom: 'Yao',
      prenom: 'Kouamé',
      photoURL: 'https://randomuser.me/api/portraits/men/3.jpg'
    },
    dateCreation: new Date('2025-01-15'),
    dateModification: new Date('2025-01-15'),
    stats: { likes: 98, dislikes: 1, commentaires: 18, partages: 34, vues: 876 },
    userReaction: null
  },
  {
    id: 'post-004',
    categorie: 'ressource_historique',
    contenu: 'L\'Empire du Mali : l\'âge d\'or de l\'Afrique de l\'Ouest',
    nomAuteur: null,
    explication: 'Fondé au XIIIe siècle par Soundiata Keïta, l\'Empire du Mali fut l\'un des plus grands et des plus riches empires de l\'histoire africaine. Sous Mansa Moussa, il atteignit son apogée, contrôlant les routes commerciales transsahariennes et devenant un centre majeur d\'érudition islamique.',
    couleurFond: null,
    imageArrierePlan: null,
    imageCouverture: 'https://images.unsplash.com/photo-1489749798305-4fea3ae63d43?w=800',
    pays: 'Mali',
    groupeEthnique: 'Mandingue',
    hashtags: ['histoire', 'empire', 'mali', 'mansamoussa'],
    userId: 'user-004',
    userInfo: {
      uid: 'user-004',
      email: 'ibrahima.keita@example.com',
      nom: 'Keïta',
      prenom: 'Ibrahima',
      photoURL: 'https://randomuser.me/api/portraits/men/4.jpg'
    },
    dateCreation: new Date('2025-01-18'),
    dateModification: new Date('2025-01-18'),
    stats: { likes: 234, dislikes: 0, commentaires: 67, partages: 89, vues: 3456 },
    userReaction: 'like'
  },
  {
    id: 'post-005',
    categorie: 'proverbe_adage',
    contenu: 'L\'enfant qui pose des questions ne s\'égare pas en chemin.',
    nomAuteur: null,
    explication: 'Ce proverbe nous enseigne l\'importance de la curiosité et de l\'humilité. Poser des questions est signe de sagesse, pas de faiblesse.',
    couleurFond: '#6B2C5B',
    imageArrierePlan: 'https://images.unsplash.com/photo-1503676260728-1c00da094a0b?w=800',
    imageCouverture: null,
    pays: 'Cameroun',
    groupeEthnique: 'Bamiléké',
    hashtags: ['curiosité', 'apprentissage', 'sagesse'],
    userId: 'user-005',
    userInfo: {
      uid: 'user-005',
      email: 'marie.tabi@example.com',
      nom: 'Tabi',
      prenom: 'Marie',
      photoURL: 'https://randomuser.me/api/portraits/women/5.jpg'
    },
    dateCreation: new Date('2025-01-20'),
    dateModification: new Date('2025-01-20'),
    stats: { likes: 145, dislikes: 2, commentaires: 32, partages: 56, vues: 1567 },
    userReaction: null
  },
  {
    id: 'post-006',
    categorie: 'citation',
    contenu: 'Je ne suis pas africain parce que je suis né en Afrique, mais parce que l\'Afrique est née en moi.',
    nomAuteur: 'Kwame Nkrumah',
    explication: 'Cette citation puissante de Nkrumah exprime l\'idée que l\'identité africaine va au-delà de la géographie pour devenir une essence, une appartenance spirituelle et culturelle.',
    couleurFond: '#B8860B',
    imageArrierePlan: null,
    imageCouverture: null,
    pays: 'Ghana',
    groupeEthnique: 'Akan',
    hashtags: ['nkrumah', 'panafricanisme', 'identité'],
    userId: 'user-006',
    userInfo: {
      uid: 'user-006',
      email: 'abena.mensah@example.com',
      nom: 'Mensah',
      prenom: 'Abena',
      photoURL: 'https://randomuser.me/api/portraits/women/6.jpg'
    },
    dateCreation: new Date('2025-01-22'),
    dateModification: new Date('2025-01-22'),
    stats: { likes: 312, dislikes: 4, commentaires: 78, partages: 156, vues: 4567 },
    userReaction: null
  },
  {
    id: 'post-007',
    categorie: 'proverbe_adage',
    contenu: 'Umuntu ngumuntu ngabantu - Je suis parce que nous sommes.',
    nomAuteur: null,
    explication: 'Cette philosophie Ubuntu sud-africaine exprime l\'interconnexion de tous les êtres humains. Notre humanité se définit par notre relation aux autres. Elle est au cœur de la vision africaine de la communauté.',
    couleurFond: '#800020',
    imageArrierePlan: 'https://images.unsplash.com/photo-1529156069898-49953e39b3ac?w=800',
    imageCouverture: null,
    pays: 'Afrique du Sud',
    groupeEthnique: 'Zoulou',
    hashtags: ['ubuntu', 'humanité', 'solidarité', 'communauté'],
    userId: 'user-007',
    userInfo: {
      uid: 'user-007',
      email: 'sipho.ndlovu@example.com',
      nom: 'Ndlovu',
      prenom: 'Sipho',
      photoURL: 'https://randomuser.me/api/portraits/men/7.jpg'
    },
    dateCreation: new Date('2025-01-25'),
    dateModification: new Date('2025-01-25'),
    stats: { likes: 456, dislikes: 1, commentaires: 123, partages: 234, vues: 6789 },
    userReaction: 'like'
  },
  {
    id: 'post-008',
    categorie: 'bonne_pratique',
    contenu: 'Le palabre africain : la démocratie participative ancestrale',
    nomAuteur: null,
    explication: 'L\'arbre à palabre est l\'espace traditionnel où se règlent les conflits et se prennent les décisions communautaires. Tous peuvent s\'exprimer jusqu\'à l\'obtention d\'un consensus. C\'est une forme de démocratie délibérative qui privilégie l\'harmonie sociale.',
    couleurFond: null,
    imageArrierePlan: null,
    imageCouverture: 'https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?w=800',
    pays: 'Burkina Faso',
    groupeEthnique: 'Mossi',
    hashtags: ['palabre', 'démocratie', 'consensus', 'tradition'],
    userId: 'user-008',
    userInfo: {
      uid: 'user-008',
      email: 'ousmane.ouedraogo@example.com',
      nom: 'Ouédraogo',
      prenom: 'Ousmane',
      photoURL: 'https://randomuser.me/api/portraits/men/8.jpg'
    },
    dateCreation: new Date('2025-01-28'),
    dateModification: new Date('2025-01-28'),
    stats: { likes: 178, dislikes: 0, commentaires: 45, partages: 67, vues: 2345 },
    userReaction: null
  },
  {
    id: 'post-009',
    categorie: 'ressource_historique',
    contenu: 'Tombouctou : la perle du désert et centre du savoir',
    nomAuteur: null,
    explication: 'Au XVe et XVIe siècle, Tombouctou était l\'un des plus grands centres intellectuels du monde. L\'université de Sankoré accueillait 25 000 étudiants. Ses bibliothèques contenaient des centaines de milliers de manuscrits sur la théologie, les sciences et la philosophie.',
    couleurFond: null,
    imageArrierePlan: null,
    imageCouverture: 'https://images.unsplash.com/photo-1547471080-7cc2caa01a7e?w=800',
    pays: 'Mali',
    groupeEthnique: 'Songhaï',
    hashtags: ['tombouctou', 'savoir', 'université', 'manuscrits'],
    userId: 'user-009',
    userInfo: {
      uid: 'user-009',
      email: 'aliou.toure@example.com',
      nom: 'Touré',
      prenom: 'Aliou',
      photoURL: 'https://randomuser.me/api/portraits/men/9.jpg'
    },
    dateCreation: new Date('2025-01-30'),
    dateModification: new Date('2025-01-30'),
    stats: { likes: 267, dislikes: 0, commentaires: 89, partages: 112, vues: 4123 },
    userReaction: null
  },
  {
    id: 'post-010',
    categorie: 'citation',
    contenu: 'Jusqu\'à ce que les lions aient leurs propres historiens, les histoires de chasse glorifieront toujours le chasseur.',
    nomAuteur: 'Chinua Achebe',
    explication: 'Achebe nous rappelle l\'importance de raconter notre propre histoire. Tant que nous ne prenons pas en main notre récit, il sera façonné par d\'autres selon leurs intérêts.',
    couleurFond: '#004D40',
    imageArrierePlan: null,
    imageCouverture: null,
    pays: 'Nigeria',
    groupeEthnique: 'Igbo',
    hashtags: ['achebe', 'histoire', 'récit', 'décolonisation'],
    userId: 'user-010',
    userInfo: {
      uid: 'user-010',
      email: 'chukwuemeka@example.com',
      nom: 'Okonkwo',
      prenom: 'Chukwuemeka',
      photoURL: 'https://randomuser.me/api/portraits/men/10.jpg'
    },
    dateCreation: new Date('2025-02-01'),
    dateModification: new Date('2025-02-01'),
    stats: { likes: 389, dislikes: 5, commentaires: 94, partages: 178, vues: 5678 },
    userReaction: null
  }
]

// Commentaires mock
export const codiMoiCommentsMock: CodiMoiComment[] = [
  {
    id: 'cmt-001',
    postId: 'post-001',
    contenu: 'Ce proverbe m\'a été transmis par ma grand-mère. Il guide ma vie depuis toujours.',
    userId: 'user-011',
    userInfo: {
      uid: 'user-011',
      email: 'moussa@example.com',
      nom: 'Traoré',
      prenom: 'Moussa',
      photoURL: 'https://randomuser.me/api/portraits/men/11.jpg'
    },
    dateCreation: new Date('2025-01-11'),
    likes: 23
  },
  {
    id: 'cmt-002',
    postId: 'post-001',
    contenu: 'Magnifique sagesse de nos ancêtres. Merci pour ce partage !',
    userId: 'user-012',
    userInfo: {
      uid: 'user-012',
      email: 'awa@example.com',
      nom: 'Diop',
      prenom: 'Awa',
      photoURL: 'https://randomuser.me/api/portraits/women/12.jpg'
    },
    dateCreation: new Date('2025-01-12'),
    likes: 15
  },
  {
    id: 'cmt-003',
    postId: 'post-002',
    contenu: 'Mandela était vraiment un visionnaire. Cette citation reste d\'une actualité brûlante.',
    userId: 'user-013',
    userInfo: {
      uid: 'user-013',
      email: 'thandi@example.com',
      nom: 'Molefe',
      prenom: 'Thandi',
      photoURL: 'https://randomuser.me/api/portraits/women/13.jpg'
    },
    dateCreation: new Date('2025-01-13'),
    likes: 45
  },
  {
    id: 'cmt-004',
    postId: 'post-004',
    contenu: 'L\'histoire de l\'Empire du Mali devrait être enseignée dans toutes les écoles africaines.',
    userId: 'user-014',
    userInfo: {
      uid: 'user-014',
      email: 'sekou@example.com',
      nom: 'Konaté',
      prenom: 'Sékou',
      photoURL: 'https://randomuser.me/api/portraits/men/14.jpg'
    },
    dateCreation: new Date('2025-01-19'),
    likes: 67
  },
  {
    id: 'cmt-005',
    postId: 'post-007',
    contenu: 'Ubuntu est une philosophie de vie extraordinaire. Elle nous rappelle notre humanité commune.',
    userId: 'user-015',
    userInfo: {
      uid: 'user-015',
      email: 'grace@example.com',
      nom: 'Muthoni',
      prenom: 'Grace',
      photoURL: 'https://randomuser.me/api/portraits/women/15.jpg'
    },
    dateCreation: new Date('2025-01-26'),
    likes: 56
  }
]

// Fonctions utilitaires

export const getAllPosts = (): CodiMoiPost[] => {
  return [...codiMoiPostsMock]
}

export const getPostById = (id: string): CodiMoiPost | undefined => {
  return codiMoiPostsMock.find(p => p.id === id)
}

export const getPostsByCategory = (categorie: CategoriePost): CodiMoiPost[] => {
  return codiMoiPostsMock.filter(p => p.categorie === categorie)
}

export const getPostComments = (postId: string): CodiMoiComment[] => {
  return codiMoiCommentsMock.filter(c => c.postId === postId)
}

export const filterPosts = (
  categorie?: CategoriePost | '',
  pays?: string,
  groupeEthnique?: string
): CodiMoiPost[] => {
  return codiMoiPostsMock.filter(post => {
    const categorieMatch = !categorie || post.categorie === categorie
    const paysMatch = !pays || post.pays === pays
    const ethnieMatch = !groupeEthnique || post.groupeEthnique === groupeEthnique
    return categorieMatch && paysMatch && ethnieMatch
  })
}

export const searchPosts = (term: string): CodiMoiPost[] => {
  const searchLower = term.toLowerCase().trim()
  if (!searchLower) return getAllPosts()

  return codiMoiPostsMock.filter(post =>
    post.contenu.toLowerCase().includes(searchLower) ||
    post.explication?.toLowerCase().includes(searchLower) ||
    post.nomAuteur?.toLowerCase().includes(searchLower) ||
    post.pays.toLowerCase().includes(searchLower) ||
    post.hashtags.some(h => h.toLowerCase().includes(searchLower))
  )
}

export const getPopularPosts = (limit: number = 5): CodiMoiPost[] => {
  return [...codiMoiPostsMock]
    .sort((a, b) => b.stats.likes - a.stats.likes)
    .slice(0, limit)
}

export const getRecentPosts = (limit: number = 10): CodiMoiPost[] => {
  return [...codiMoiPostsMock]
    .sort((a, b) => b.dateCreation.getTime() - a.dateCreation.getTime())
    .slice(0, limit)
}

export const formatDate = (date: Date): string => {
  const now = new Date()
  const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60))

  if (diffInMinutes < 1) return 'À l\'instant'
  if (diffInMinutes < 60) return `Il y a ${diffInMinutes} min`
  if (diffInMinutes < 1440) return `Il y a ${Math.floor(diffInMinutes / 60)} h`
  if (diffInMinutes < 10080) return `Il y a ${Math.floor(diffInMinutes / 1440)} j`

  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: date.getFullYear() !== now.getFullYear() ? 'numeric' : undefined
  })
}

export const getStats = () => {
  return {
    totalPosts: codiMoiPostsMock.length,
    proverbesAdages: codiMoiPostsMock.filter(p => p.categorie === 'proverbe_adage').length,
    citations: codiMoiPostsMock.filter(p => p.categorie === 'citation').length,
    ressourcesHistoriques: codiMoiPostsMock.filter(p => p.categorie === 'ressource_historique').length,
    bonnesPratiques: codiMoiPostsMock.filter(p => p.categorie === 'bonne_pratique').length,
    totalLikes: codiMoiPostsMock.reduce((sum, p) => sum + p.stats.likes, 0),
    totalCommentaires: codiMoiPostsMock.reduce((sum, p) => sum + p.stats.commentaires, 0),
    totalVues: codiMoiPostsMock.reduce((sum, p) => sum + p.stats.vues, 0)
  }
}

export const getCategoryStyle = (categorie: CategoriePost): string => {
  const styles: Record<CategoriePost, string> = {
    'proverbe_adage': 'bg-purple-100 text-purple-800',
    'citation': 'bg-blue-100 text-blue-800',
    'ressource_historique': 'bg-orange-100 text-orange-800',
    'bonne_pratique': 'bg-green-100 text-green-800'
  }
  return styles[categorie] || 'bg-gray-100 text-gray-800'
}

export const getCategoryLabel = (categorie: CategoriePost): string => {
  const labels: Record<CategoriePost, string> = {
    'proverbe_adage': 'Proverbe/Adage',
    'citation': 'Citation',
    'ressource_historique': 'Histoire',
    'bonne_pratique': 'Bonne pratique'
  }
  return labels[categorie] || categorie
}

// Composable pour les appels API Codi-Moi
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type CategoriePost = 'proverbe_adage' | 'citation' | 'ressource_historique' | 'bonne_pratique'
export type UserReaction = 'like' | 'dislike' | null

export interface CodiMoiAuteur {
  id: string
  nom: string
  prenom: string | null
}

/** DTO correspondant a CodiMoiResponse du backend */
export interface CodiMoiPostAPI {
  id: string
  type: string
  contenu: string
  explication: string | null
  nom_auteur_originel: string | null
  pays: string | null
  groupe_ethnique: string | null
  couleur_fond: string | null
  image_couverture_url: string | null
  image_arriere_plan_url: string | null
  nombre_likes: number
  nombre_dislikes: number
  nombre_commentaires: number
  hashtags: string[]
  auteur: CodiMoiAuteur
  user_reaction: UserReaction
  created_at: string
}

/** DTO correspondant a CodiMoiListeResponse du backend */
export interface CodiMoiListeAPI {
  posts: CodiMoiPostAPI[]
  total: number
  page: number
  par_page: number
}

/** DTO pour un commentaire */
export interface CommentaireAPI {
  id: string
  contenu: string
  parent_id: string | null
  nombre_likes: number
  auteur: CodiMoiAuteur
  created_at: string
}

/** DTO pour la liste des commentaires */
export interface CommentaireListeAPI {
  commentaires: CommentaireAPI[]
  total: number
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface CodiMoiFiltres {
  recherche?: string
  type?: string
  pays?: string
  page?: number
  par_page?: number
}

/** Corps de la requete de creation */
export interface CreerCodiMoiPayload {
  type: string
  contenu: string
  explication?: string
  nom_auteur_originel?: string
  pays?: string
  groupe_ethnique?: string
  couleur_fond?: string
  hashtags?: string[]
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const CATEGORIES_POST: { value: CategoriePost | ''; label: string; icon: string }[] = [
  { value: '', label: 'Tout', icon: 'fa-solid fa-globe' },
  { value: 'proverbe_adage', label: 'Proverbes & Adages', icon: 'fa-solid fa-quote-left' },
  { value: 'citation', label: 'Citations', icon: 'fa-solid fa-quote-right' },
  { value: 'ressource_historique', label: 'Histoire', icon: 'fa-solid fa-landmark' },
  { value: 'bonne_pratique', label: 'Bonnes pratiques', icon: 'fa-solid fa-lightbulb' },
]

export const COULEURS_FOND = [
  '#2D5A27', '#8B4513', '#1E3A5F', '#6B2C5B',
  '#B8860B', '#2F4F4F', '#800020', '#004D40',
]

export const PAYS_AFRICAINS = [
  'Sénégal', 'Mali', 'Côte d\'Ivoire', 'Burkina Faso', 'Ghana',
  'Nigeria', 'Cameroun', 'RDC', 'Kenya', 'Afrique du Sud',
  'Éthiopie', 'Tanzanie', 'Maroc', 'Algérie', 'Égypte',
]

export const GROUPES_ETHNIQUES = [
  'Wolof', 'Peul', 'Sérère', 'Bambara', 'Mossi', 'Akan',
  'Yoruba', 'Igbo', 'Bamiléké', 'Zoulou', 'Kikuyu', 'Oromo',
  'Amazigh', 'Haoussa', 'Mandingue', 'Dioula',
]

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

export const formatDateRelative = (dateStr: string): string => {
  const date = new Date(dateStr)
  const now = new Date()
  const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60))

  if (diffInMinutes < 1) return 'À l\'instant'
  if (diffInMinutes < 60) return `Il y a ${diffInMinutes} min`
  if (diffInMinutes < 1440) return `Il y a ${Math.floor(diffInMinutes / 60)} h`
  if (diffInMinutes < 10080) return `Il y a ${Math.floor(diffInMinutes / 1440)} j`

  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: date.getFullYear() !== now.getFullYear() ? 'numeric' : undefined,
  })
}

export const getCategoryStyle = (categorie: CategoriePost): string => {
  const styles: Record<CategoriePost, string> = {
    proverbe_adage: 'bg-purple-100 text-purple-800',
    citation: 'bg-blue-100 text-blue-800',
    ressource_historique: 'bg-orange-100 text-orange-800',
    bonne_pratique: 'bg-green-100 text-green-800',
  }
  return styles[categorie] || 'bg-gray-100 text-gray-800'
}

export const getCategoryLabel = (categorie: CategoriePost): string => {
  const labels: Record<CategoriePost, string> = {
    proverbe_adage: 'Proverbe/Adage',
    citation: 'Citation',
    ressource_historique: 'Histoire',
    bonne_pratique: 'Bonne pratique',
  }
  return labels[categorie] || categorie
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useCodiMoi = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** Headers d'authentification si l'utilisateur est connecté */
  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  /**
   * Lister les posts avec filtres et pagination
   */
  const listerPosts = async (filtres: CodiMoiFiltres = {}): Promise<CodiMoiListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.type) params.set('type', filtres.type)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/codimoi${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<CodiMoiListeAPI>>(url, {
        headers: authHeaders(),
      })

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des posts')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerPosts:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir un post par son ID
   */
  const obtenirPost = async (id: string): Promise<CodiMoiPostAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<CodiMoiPostAPI>>(
        `${apiBase}/api/codimoi/${id}`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Post non trouve')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirPost:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Creer un nouveau post
   */
  const creerPost = async (payload: CreerCodiMoiPayload): Promise<CodiMoiPostAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<CodiMoiPostAPI>>(
        `${apiBase}/api/codimoi`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: payload,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation du post')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerPost:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Reagir a un post (like/dislike toggle)
   */
  const reagir = async (postId: string, typeReaction: 'like' | 'dislike'): Promise<CodiMoiPostAPI | null> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<CodiMoiPostAPI>>(
        `${apiBase}/api/codimoi/${postId}/reaction`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: { type_reaction: typeReaction },
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la reaction')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur reagir:', e)
      return null
    }
  }

  /**
   * Lister les commentaires d'un post
   */
  const listerCommentaires = async (postId: string): Promise<CommentaireListeAPI | null> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<CommentaireListeAPI>>(
        `${apiBase}/api/codimoi/${postId}/commentaires`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des commentaires')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerCommentaires:', e)
      return null
    }
  }

  /**
   * Creer un commentaire
   */
  const creerCommentaire = async (postId: string, contenu: string, parentId?: string): Promise<CommentaireAPI | null> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<CommentaireAPI>>(
        `${apiBase}/api/codimoi/${postId}/commentaires`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: { contenu, parent_id: parentId || null },
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation du commentaire')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerCommentaire:', e)
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerPosts,
    obtenirPost,
    creerPost,
    reagir,
    listerCommentaires,
    creerCommentaire,
  }
}

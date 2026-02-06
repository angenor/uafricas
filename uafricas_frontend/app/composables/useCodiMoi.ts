// Composable pour les appels API Codi-Moi

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
  hashtags: string[]
  auteur: {
    id: string
    nom: string
    prenom: string | null
  }
  created_at: string
}

/** DTO correspondant a CodiMoiListeResponse du backend */
export interface CodiMoiListeAPI {
  posts: CodiMoiPostAPI[]
  total: number
  page: number
  par_page: number
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

export const useCodiMoi = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

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

      const reponse = await $fetch<ApiResponse<CodiMoiListeAPI>>(url)

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

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerPosts,
    obtenirPost,
    creerPost,
  }
}

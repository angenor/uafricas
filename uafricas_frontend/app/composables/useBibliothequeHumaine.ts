// Composable pour les appels API Bibliotheques Humaines
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

/** DTO correspondant a BiblioHumaineResponse du backend */
export interface BiblioHumaineAPI {
  id: string
  userId: string
  nom: string
  prenom: string
  photoUrl: string | null
  fonction: string
  pays: string
  specialite: string
  specialites: string[]
  biographie: string
  ville: string | null
  dateInscription: string
}

/** Reponse paginee */
export interface BiblioHumaineListeAPI {
  bibliotheques: BiblioHumaineAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Specialite */
export interface SpecialiteAPI {
  id: string
  nom: string
  slug: string
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface BiblioHumaineFiltres {
  recherche?: string
  specialite?: string
  pays?: string
  page?: number
  par_page?: number
}

/** Body pour l'inscription */
export interface InscriptionBiblioBody {
  specialites: string[]
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useBibliothequeHumaine = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** Headers d'authentification si l'utilisateur est connecte */
  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  /**
   * Lister les bibliotheques humaines avec filtres et pagination
   */
  const listerBiblios = async (filtres: BiblioHumaineFiltres = {}): Promise<BiblioHumaineListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.specialite && filtres.specialite !== 'Tous') params.set('specialite', filtres.specialite)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/bibliotheques-humaines${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<BiblioHumaineListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des bibliotheques humaines')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerBiblios:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir une bibliotheque humaine par son ID
   */
  const obtenirBiblio = async (id: string): Promise<BiblioHumaineAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<BiblioHumaineAPI>>(
        `${apiBase}/api/bibliotheques-humaines/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Bibliotheque humaine non trouvee')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirBiblio:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * S'inscrire comme bibliotheque humaine (JWT requis)
   */
  const inscrireBiblioHumaine = async (body: InscriptionBiblioBody): Promise<BiblioHumaineAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<BiblioHumaineAPI>>(
        `${apiBase}/api/bibliotheques-humaines/inscription`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...authHeaders(),
          },
          body,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l\'inscription')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur inscrireBiblioHumaine:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Lister les specialites disponibles
   */
  const listerSpecialites = async (): Promise<SpecialiteAPI[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<SpecialiteAPI[]>>(
        `${apiBase}/api/bibliotheques-humaines/specialites`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des specialites')
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerSpecialites:', e)
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerBiblios,
    obtenirBiblio,
    inscrireBiblioHumaine,
    listerSpecialites,
  }
}

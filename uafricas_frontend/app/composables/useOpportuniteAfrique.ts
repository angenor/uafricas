// Composable pour les appels API Fiches Pays (Opportunites en Afrique)

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type Region =
  | 'Afrique Centrale'
  | 'Afrique de l\'Ouest'
  | 'Afrique de l\'Est'
  | 'Afrique du Nord'
  | 'Afrique Australe'

/** DTO correspondant a FichePaysResponse du backend */
export interface FichePaysAPI {
  id: string
  pays_id: string
  code: string | null
  nom: string
  capitale: string | null
  image_couverture: string | null
  slogan: string | null
  superficie: string | null
  population: string | null
  monnaie: string | null
  drapeau_url: string | null
  region: string
  nombre_contributions: number
  updated_at: string
}

/** DTO correspondant a FichePaysDetailResponse du backend */
export interface FichePaysDetailAPI extends FichePaysAPI {
  embleme_url: string | null
  devise: string | null
  hymne_national: string | null
  langue_officielle: string | null
  langues: string[]
  ethnies: string[]
  biographie: string | null
  contexte: string | null
  fuseau_horaire: string | null
}

/** Reponse paginee */
export interface FichePaysListeAPI {
  fiches: FichePaysAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface FichePaysFiltres {
  recherche?: string
  region?: string
  page?: number
  par_page?: number
}

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/** Formater une date ISO en francais long (ex: "15 janvier 2025") */
export const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

/** Formater une date ISO en francais court (ex: "15 janv. 2025") */
export const formatDateShort = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  })
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useOpportuniteAfrique = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /**
   * Lister les fiches pays avec filtres et pagination
   */
  const listerFiches = async (filtres: FichePaysFiltres = {}): Promise<FichePaysListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.region) params.set('region', filtres.region)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/fiches-pays${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<FichePaysListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des fiches pays')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerFiches:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir une fiche pays par son ID (UUID, code ISO ou nom)
   */
  const obtenirFiche = async (id: string): Promise<FichePaysDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<FichePaysDetailAPI>>(
        `${apiBase}/api/fiches-pays/${encodeURIComponent(id)}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Fiche pays non trouvee')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirFiche:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Lister les regions disponibles
   */
  const listerRegions = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/fiches-pays/regions`,
      )

      if (!reponse.success || !reponse.data) {
        return null
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerRegions:', e)
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerFiches,
    obtenirFiche,
    listerRegions,
  }
}

// Composable pour les appels API de la télévision

/** Interface correspondant au DTO ChaineTvResponse du backend */
export interface ChaineTvAPI {
  id: string
  nom: string
  slug: string | null
  description: string | null
  stream_url: string
  image_couverture_url: string | null
  categorie: string
  pays: string | null
  langue: string
  est_en_direct: boolean
  created_at: string
}

/** Interface correspondant au DTO ChaineTvListeResponse du backend */
export interface ChaineTvListeAPI {
  chaines: ChaineTvAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Interface correspondant au DTO ProgrammeTeleResponse du backend */
export interface ProgrammeTeleAPI {
  id: string
  nom_emission: string
  slug: string | null
  description: string
  image_couverture_url: string | null
  video_url: string | null
  info_animateur: string | null
  info_producteur: string | null
  pays: string | null
  est_international: boolean
  langue: string
  created_at: string
}

/** Interface correspondant au DTO ProgrammeTeleListeResponse du backend */
export interface ProgrammeTeleListeAPI {
  programmes: ProgrammeTeleAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Interface correspondant au DTO TelevisionStats du backend */
export interface TelevisionStatsAPI {
  nombre_chaines: number
  nombre_pays: number
  nombre_programmes: number
  nombre_chaines_en_direct: number
}

/** Interface adaptée au format attendu par les composants frontend (chaîne) */
export interface TvChannel {
  id: string
  name: string
  description: string
  streamUrl: string
  cover: string
  category: string
  country: string
  language: string
  isLive: boolean
}

/** Interface adaptée au format attendu par les composants frontend (programme) */
export interface TvProgram {
  id: string
  title: string
  description: string
  banner: string
  videoUrl: string
  animator: string
  producer: string
  country: string
  language: string
}

/** Stats frontend */
export interface TvStat {
  value: string
  label: string
}

/** Réponse API standardisée */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Paramètres de filtre pour le listing des chaînes */
export interface ChaineTvFiltres {
  recherche?: string
  categorie?: string
  pays?: string
  page?: number
  par_page?: number
}

/** Paramètres de filtre pour le listing des programmes */
export interface ProgrammeTeleFiltres {
  recherche?: string
  pays?: string
  page?: number
  par_page?: number
}

/** Formulaire de création de chaîne */
export interface CreerChaineTvForm {
  nom: string
  description?: string
  stream_url: string
  categorie?: string
  pays?: string
  langue?: string
}

/** Formulaire de création de programme vedette */
export interface CreerProgrammeVedetteForm {
  nom_emission: string
  description: string
  video_url: string
  image_couverture_url?: string
  info_animateur?: string
  info_producteur?: string
  pays?: string
  est_international?: boolean
  langue?: string
}

// ── Mapping API → Frontend ────────────────────────────────────────────

function resoudreUrl(url: string | null, apiBase: string, fallback: string): string {
  if (!url) return fallback
  if (url.startsWith('http://') || url.startsWith('https://')) return url
  return `${apiBase}${url}`
}

function mapperChaineApiVersTv(chaine: ChaineTvAPI, apiBase: string): TvChannel {
  return {
    id: chaine.id,
    name: chaine.nom,
    description: chaine.description || '',
    streamUrl: chaine.stream_url,
    cover: resoudreUrl(chaine.image_couverture_url, apiBase, '/images/tv-default.jpg'),
    category: chaine.categorie,
    country: chaine.pays || '',
    language: chaine.langue,
    isLive: chaine.est_en_direct,
  }
}

function mapperProgrammeApiVersTv(programme: ProgrammeTeleAPI, apiBase: string): TvProgram {
  return {
    id: programme.id,
    title: programme.nom_emission,
    description: programme.description,
    banner: resoudreUrl(programme.image_couverture_url, apiBase, '/images/tv-programme-default.jpg'),
    videoUrl: programme.video_url || '',
    animator: programme.info_animateur || '',
    producer: programme.info_producteur || '',
    country: programme.pays || '',
    language: programme.langue,
  }
}

function mapperStatsApiVersFrontend(stats: TelevisionStatsAPI): TvStat[] {
  return [
    { value: `${stats.nombre_chaines}+`, label: 'Chaînes TV' },
    { value: `${stats.nombre_pays}`, label: 'Territoires Africains' },
    { value: '24/7', label: 'Diffusion Continue' },
    { value: 'HD+', label: 'Qualité Vidéo' },
  ]
}

export const useTelevision = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** Headers d'authentification */
  const authHeaders = (): Record<string, string> => {
    if (import.meta.client) {
      const token = localStorage.getItem('accessToken')
      if (token) return { Authorization: `Bearer ${token}` }
    }
    return {}
  }

  /**
   * Récupérer la liste des chaînes TV avec filtres et pagination
   */
  const listerChaines = async (filtres: ChaineTvFiltres = {}): Promise<{ chaines: TvChannel[]; total: number } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.categorie && filtres.categorie !== 'Toutes les catégories') params.set('categorie', filtres.categorie)
      if (filtres.pays && filtres.pays !== 'Tous les territoires') params.set('pays', filtres.pays)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/television/chaines${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ChaineTvListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des chaînes TV')
      }

      return {
        chaines: reponse.data.chaines.map(c => mapperChaineApiVersTv(c, apiBase)),
        total: reponse.data.total,
      }
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur listerChaines:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Récupérer une chaîne par ID
   */
  const obtenirChaine = async (id: string): Promise<TvChannel | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ChaineTvAPI>>(
        `${apiBase}/api/television/chaines/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Chaîne non trouvée')
      }

      return mapperChaineApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur obtenirChaine:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Récupérer la liste des programmes vedettes (TV à la une)
   */
  const listerProgrammesVedettes = async (filtres: ProgrammeTeleFiltres = {}): Promise<{ programmes: TvProgram[]; total: number } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.pays && filtres.pays !== 'Tous les territoires') params.set('pays', filtres.pays)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/television/programmes-vedettes${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ProgrammeTeleListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des programmes')
      }

      return {
        programmes: reponse.data.programmes.map(p => mapperProgrammeApiVersTv(p, apiBase)),
        total: reponse.data.total,
      }
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur listerProgrammesVedettes:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Récupérer un programme vedette par ID
   */
  const obtenirProgrammeVedette = async (id: string): Promise<TvProgram | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ProgrammeTeleAPI>>(
        `${apiBase}/api/television/programmes-vedettes/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Programme non trouvé')
      }

      return mapperProgrammeApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur obtenirProgrammeVedette:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Récupérer la liste des pays disponibles
   */
  const listerPays = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/television/pays`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des pays')
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerPays:', e)
      return null
    }
  }

  /**
   * Récupérer la liste des catégories disponibles
   */
  const listerCategories = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/television/categories`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des catégories')
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerCategories:', e)
      return null
    }
  }

  /**
   * Récupérer les statistiques TV
   */
  const obtenirStats = async (): Promise<TvStat[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<TelevisionStatsAPI>>(
        `${apiBase}/api/television/stats`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des statistiques')
      }

      return mapperStatsApiVersFrontend(reponse.data)
    }
    catch (e: any) {
      console.error('Erreur obtenirStats:', e)
      return null
    }
  }

  /**
   * Créer une nouvelle chaîne TV (authentification requise)
   */
  const creerChaine = async (form: CreerChaineTvForm): Promise<TvChannel | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ChaineTvAPI>>(
        `${apiBase}/api/television/chaines`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...authHeaders(),
          },
          body: form,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la création de la chaîne')
      }

      return mapperChaineApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur creerChaine:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Créer un nouveau programme vedette (authentification requise)
   */
  const creerProgrammeVedette = async (form: CreerProgrammeVedetteForm): Promise<TvProgram | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ProgrammeTeleAPI>>(
        `${apiBase}/api/television/programmes-vedettes`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...authHeaders(),
          },
          body: form,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la création du programme')
      }

      return mapperProgrammeApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur creerProgrammeVedette:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerChaines,
    obtenirChaine,
    listerProgrammesVedettes,
    obtenirProgrammeVedette,
    listerPays,
    listerCategories,
    obtenirStats,
    creerChaine,
    creerProgrammeVedette,
  }
}

// Composable pour les appels API des stations radio

/** Interface correspondant au DTO StationRadioResponse du backend */
export interface StationRadioAPI {
  id: string
  nom: string
  slug: string | null
  description: string | null
  stream_url: string
  image_couverture_url: string | null
  genre: string | null
  genres_liste: string[]
  pays: string | null
  ville: string | null
  type_station: string
  created_at: string
}

/** Interface correspondant au DTO StationRadioListeResponse du backend */
export interface StationRadioListeAPI {
  stations: StationRadioAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Interface adaptee au format attendu par les composants frontend */
export interface RadioStation {
  id: string
  name: string
  description: string
  streamUrl: string
  cover: string
  genre: string
  genresList: string[]
  location: string
  country: string
  programType: 'Nationales' | 'Local' | 'International'
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface StationRadioFiltres {
  recherche?: string
  type_station?: string
  pays?: string
  genre?: string
  page?: number
  par_page?: number
}

/** Formulaire de creation de station */
export interface CreerStationForm {
  nom: string
  description?: string
  stream_url: string
  genre?: string
  genres_liste?: string[]
  pays?: string
  ville?: string
  type_station?: string
}

// ── Mapping API → Frontend ────────────────────────────────────────────

function mapperStationApiVersRadio(station: StationRadioAPI, apiBase: string): RadioStation {
  const location = [station.ville, station.pays].filter(Boolean).join(', ')
  return {
    id: station.id,
    name: station.nom,
    description: station.description || '',
    streamUrl: station.stream_url,
    cover: station.image_couverture_url
      ? `${apiBase}${station.image_couverture_url}`
      : '/images/radio-default.jpg',
    genre: station.genre || '',
    genresList: station.genres_liste || [],
    location,
    country: station.pays || '',
    programType: station.type_station as RadioStation['programType'],
  }
}

export const useStationsRadio = () => {
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
   * Recuperer la liste des stations radio avec filtres et pagination
   */
  const listerStations = async (filtres: StationRadioFiltres = {}): Promise<{ stations: RadioStation[]; total: number } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.type_station && filtres.type_station !== 'Tous les types') params.set('type_station', filtres.type_station)
      if (filtres.pays && filtres.pays !== 'Tous les territoires') params.set('pays', filtres.pays)
      if (filtres.genre && filtres.genre !== 'Tous les genres') params.set('genre', filtres.genre)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/stations-radio${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<StationRadioListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des stations')
      }

      return {
        stations: reponse.data.stations.map(s => mapperStationApiVersRadio(s, apiBase)),
        total: reponse.data.total,
      }
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerStations:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Recuperer une station par ID
   */
  const obtenirStation = async (id: string): Promise<RadioStation | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<StationRadioAPI>>(
        `${apiBase}/api/stations-radio/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Station non trouvée')
      }

      return mapperStationApiVersRadio(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirStation:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Recuperer la liste des pays disponibles
   */
  const listerPays = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/stations-radio/pays`,
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
   * Recuperer la liste des genres disponibles
   */
  const listerGenres = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/stations-radio/genres`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des genres')
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerGenres:', e)
      return null
    }
  }

  /**
   * Creer une nouvelle station radio (authentification requise)
   */
  const creerStation = async (form: CreerStationForm): Promise<RadioStation | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<StationRadioAPI>>(
        `${apiBase}/api/stations-radio`,
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
        throw new Error(reponse.error || 'Erreur lors de la création de la station')
      }

      return mapperStationApiVersRadio(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerStation:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerStations,
    obtenirStation,
    listerPays,
    listerGenres,
    creerStation,
  }
}

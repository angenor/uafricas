// Composable pour les appels API des centres culturels

/** Interface correspondant au DTO CentreCulturelResponse du backend */
export interface CentreCulturelAPI {
  id: string
  nom: string
  slug: string | null
  description: string | null
  image_couverture_url: string | null
  ville: string | null
  adresse: string | null
  latitude: number | null
  longitude: number | null
  nombre_programmations: number
  created_at: string
  updated_at: string
}

/** Interface correspondant au DTO MembreCentreResponse du backend */
export interface MembreCentreAPI {
  nom: string
  prenom: string | null
  email: string
  telephone: string | null
  role: string
  role_label: string
}

/** Interface correspondant au DTO ProgrammationResponse du backend */
export interface ProgrammationAPI {
  id: string
  centre_culturel_id: string
  titre: string
  description: string | null
  lieu: string | null
  mode: string
  lien_en_ligne: string | null
  date_heure_debut: string
  date_heure_fin: string | null
  nombre_places: number | null
  statut: string
  created_at: string
}

/** Interface correspondant au DTO CentreCulturelDetailResponse du backend */
export interface CentreCulturelDetailAPI {
  id: string
  nom: string
  slug: string | null
  description: string | null
  image_couverture_url: string | null
  ville: string | null
  adresse: string | null
  latitude: number | null
  longitude: number | null
  membres: MembreCentreAPI[]
  programmations: ProgrammationAPI[]
  created_at: string
  updated_at: string
}

/** Interface correspondant au DTO ProgrammationDetailResponse du backend */
export interface ProgrammationDetailAPI {
  programmation: ProgrammationAPI
  centre: {
    id: string
    nom: string
    slug: string | null
  }
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface CentreCulturelFiltres {
  recherche?: string
}

/** Generer un lien Google Maps a partir des coordonnees */
function genererLienGoogleMaps(latitude: number | null, longitude: number | null): string | null {
  if (latitude && longitude) {
    return `https://www.google.com/maps?q=${latitude},${longitude}`
  }
  return null
}

/** Transformer les URLs relatives en URLs absolues */
function mapperUrlsCentre(centre: CentreCulturelAPI, apiBase: string): CentreCulturelAPI {
  return {
    ...centre,
    image_couverture_url: centre.image_couverture_url
      ? `${apiBase}${centre.image_couverture_url}`
      : null,
  }
}

/** Transformer les URLs relatives en URLs absolues pour le detail */
function mapperUrlsCentreDetail(centre: CentreCulturelDetailAPI, apiBase: string): CentreCulturelDetailAPI {
  return {
    ...centre,
    image_couverture_url: centre.image_couverture_url
      ? `${apiBase}${centre.image_couverture_url}`
      : null,
  }
}

/** Formater une date en francais (ex: "jeudi 15 janvier 2026") */
export function formatDateFrancais(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

/** Formater une date courte en francais (ex: "15 janvier 2026") */
export function formatDateCourteFrancais(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

/** Formater l'heure en francais (ex: "14:30") */
export function formatHeureFrancais(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleTimeString('fr-FR', {
    hour: '2-digit',
    minute: '2-digit',
  })
}

/** Obtenir le label du mode d'evenement */
export function getModeLabel(mode: string): string {
  switch (mode) {
    case 'presentiel':
      return 'Exposition en présentiel'
    case 'en-ligne':
      return 'En ligne'
    case 'hybride':
      return 'Hybride'
    default:
      return mode
  }
}

export const useCentresCulturels = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /**
   * Recuperer la liste des centres culturels
   */
  const listerCentres = async (filtres: CentreCulturelFiltres = {}): Promise<CentreCulturelAPI[] | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)

      const queryString = params.toString()
      const url = `${apiBase}/api/centres-culturels${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<CentreCulturelAPI[]>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des centres culturels')
      }

      return reponse.data.map(c => mapperUrlsCentre(c, apiBase))
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerCentres:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Recuperer un centre culturel avec ses programmations
   */
  const obtenirCentre = async (id: string): Promise<CentreCulturelDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<CentreCulturelDetailAPI>>(
        `${apiBase}/api/centres-culturels/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Centre culturel non trouve')
      }

      return mapperUrlsCentreDetail(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirCentre:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Recuperer le detail d'une programmation
   */
  const obtenirProgrammation = async (centreId: string, programmationId: string): Promise<ProgrammationDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ProgrammationDetailAPI>>(
        `${apiBase}/api/centres-culturels/${centreId}/programmations/${programmationId}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Programmation non trouvee')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirProgrammation:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerCentres,
    obtenirCentre,
    obtenirProgrammation,
    // Utilitaires exportes
    genererLienGoogleMaps,
    formatDateFrancais,
    formatDateCourteFrancais,
    formatHeureFrancais,
    getModeLabel,
  }
}

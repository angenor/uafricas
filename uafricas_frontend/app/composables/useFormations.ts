// Composable pour les appels API Formations (MOOC/CLOM)
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces (conformes au schema media_content.mooc)
// ──────────────────────────────────────────────────────────────

export type TypeFormation = 'mooc' | 'clom' | 'atelier' | 'concertation' | 'grand_public'
export type StatutFormation = 'inscriptions_ouvertes' | 'complet' | 'en_cours' | 'termine' | 'annule' | 'programme' | 'suspendu'

export interface FormateurInfo {
  uid: string
  nom: string
  prenom: string | null
  email: string
  photo_url: string | null
}

/** DTO correspondant a MoocResponse du backend */
export interface FormationAPI {
  id: string
  titre: string
  description: string
  type: string | null
  langue: string
  date_heure_debut: string
  date_heure_fin: string | null
  couverture_url: string | null
  statut: StatutFormation
  nombre_places: number | null
  nombre_inscrits: number
  formateur: FormateurInfo
  created_at: string
  updated_at: string
}

/** DTO pour le detail d'une formation */
export interface FormationDetailAPI extends FormationAPI {
  slug: string | null
  pays: string | null
  ville: string | null
  format: string
  lien_en_ligne: string | null
  prerequis: string | null
  est_inscrit: boolean
}

/** DTO des statistiques agregees de la page d'accueil universite */
export interface UniversiteStatsAPI {
  nombre_facultes: number
  nombre_formations: number
  nombre_inscrits: number
  nombre_pays: number
}

/** Reponse paginee */
export interface FormationListeAPI {
  formations: FormationAPI[]
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
export interface FormationFiltres {
  recherche?: string
  type?: string
  page?: number
  par_page?: number
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const TYPES_FORMATION: { value: string; label: string }[] = [
  { value: 'mooc', label: 'MOOC' },
  { value: 'clom', label: 'CLOM' },
  { value: 'atelier', label: 'Atelier' },
  { value: 'concertation', label: 'Concertation' },
  { value: 'grand_public', label: 'Formations grand public' },
]

export const STATUTS_FORMATION: { value: string; label: string }[] = [
  { value: 'inscriptions_ouvertes', label: 'Inscriptions ouvertes' },
  { value: 'programme', label: 'Programmé' },
  { value: 'en_cours', label: 'En cours' },
  { value: 'termine', label: 'Terminé' },
]

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/** Label du type de formation */
export const getTypeLabel = (type: string | null): string => {
  const labels: Record<string, string> = {
    mooc: 'MOOC',
    clom: 'CLOM',
    atelier: 'Atelier',
    concertation: 'Concertation',
    grand_public: 'Formations grand public',
  }
  return type ? (labels[type] || type.toUpperCase()) : 'Formation'
}

/** Classes CSS du badge type */
export const getTypeClasses = (type: string | null): string => {
  const classes: Record<string, string> = {
    mooc: 'bg-blue-100 text-blue-800',
    clom: 'bg-purple-100 text-purple-800',
    atelier: 'bg-green-100 text-green-800',
    concertation: 'bg-orange-100 text-orange-800',
    grand_public: 'bg-custom-chocolat/10 text-custom-chocolat',
  }
  return type ? (classes[type] || 'bg-gray-100 text-gray-800') : 'bg-gray-100 text-gray-800'
}

/** Label du statut */
export const getStatutLabel = (statut: string): string => {
  const labels: Record<string, string> = {
    inscriptions_ouvertes: 'Inscriptions ouvertes',
    complet: 'Complet',
    en_cours: 'En cours',
    termine: 'Terminé',
    annule: 'Annulé',
    programme: 'Programmé',
    suspendu: 'Suspendu',
  }
  return labels[statut] || statut
}

/** Couleur du point de statut */
export const getStatutColor = (statut: string): string => {
  const colors: Record<string, string> = {
    inscriptions_ouvertes: 'bg-green-500',
    complet: 'bg-red-500',
    en_cours: 'bg-blue-500',
    termine: 'bg-gray-500',
    annule: 'bg-red-500',
    programme: 'bg-yellow-500',
  }
  return colors[statut] || 'bg-gray-500'
}

/** Gradient du hero par type */
export const getTypeGradient = (type: string | null): string => {
  const gradients: Record<string, string> = {
    mooc: 'from-blue-600 to-blue-800',
    clom: 'from-purple-600 to-purple-800',
    atelier: 'from-green-600 to-green-800',
    concertation: 'from-orange-600 to-orange-800',
    grand_public: 'from-custom-chocolat to-amber-800',
  }
  return type ? (gradients[type] || 'from-gray-600 to-gray-800') : 'from-gray-600 to-gray-800'
}

/** Classes du badge statut */
export const getStatutBadgeClass = (statut: string): string => {
  const classes: Record<string, string> = {
    inscriptions_ouvertes: 'px-3 py-1 bg-green-500 text-white rounded-full text-sm font-medium',
    en_cours: 'px-3 py-1 bg-blue-500 text-white rounded-full text-sm font-medium',
    termine: 'px-3 py-1 bg-gray-500 text-white rounded-full text-sm font-medium',
    complet: 'px-3 py-1 bg-red-500 text-white rounded-full text-sm font-medium',
  }
  return classes[statut] || 'px-3 py-1 bg-gray-500 text-white rounded-full text-sm font-medium'
}

/** Peut s'inscrire a une formation */
export const peutSInscrire = (formation: FormationAPI): boolean => {
  return formation.statut === 'inscriptions_ouvertes' &&
         (!formation.nombre_places || formation.nombre_inscrits < formation.nombre_places)
}

/** Label du bouton d'action */
export const getActionLabel = (formation: FormationAPI): string => {
  if (formation.statut === 'complet') return 'Complet'
  if (formation.statut === 'termine') return 'Terminé'
  if (formation.statut === 'en_cours') return 'En cours'
  if (formation.statut === 'inscriptions_ouvertes') return "S'inscrire"
  return 'Prochainement'
}

/** Formater une date ISO en francais */
export const formatDateFormation = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
}

/** Formater une date ISO en format court */
export const formatDateCourt = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  })
}

/** Mapper format DB vers label frontend */
export const mapperFormatFrontend = (format: string): string => {
  const map: Record<string, string> = {
    en_ligne: 'En ligne',
    presentiel: 'En présentiel',
    hybride: 'Hybride',
  }
  return map[format] || format
}

/** Transformer une URL relative (`/uploads/...`) en URL absolue */
const mapperUrlImage = (url: string | null | undefined, apiBase: string): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useFormations = () => {
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
   * Lister les formations avec filtres et pagination
   */
  const listerFormations = async (filtres: FormationFiltres = {}): Promise<FormationListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.type) params.set('type', filtres.type)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/moocs${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<FormationListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des formations')
      }

      // Le backend renvoie des URLs de couverture relatives (`/uploads/...`) : les rendre absolues
      reponse.data.formations = reponse.data.formations.map(f => ({
        ...f,
        couverture_url: mapperUrlImage(f.couverture_url, apiBase),
      }))

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerFormations:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir une formation par son ID
   */
  const obtenirFormation = async (id: string): Promise<FormationDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<FormationDetailAPI>>(
        `${apiBase}/api/moocs/${id}`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Formation non trouvee')
      }

      return {
        ...reponse.data,
        couverture_url: mapperUrlImage(reponse.data.couverture_url, apiBase),
      }
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirFormation:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * S'inscrire a une formation
   */
  const inscrireFormation = async (formationId: string): Promise<boolean> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/moocs/${formationId}/inscription`,
        {
          method: 'POST',
          headers: authHeaders(),
        },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors de l\'inscription')
      }

      return true
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur inscrireFormation:', e)
      return false
    }
  }

  /**
   * Recuperer les statistiques agregees de la page d'accueil universite
   */
  const obtenirStatsUniversite = async (): Promise<UniversiteStatsAPI | null> => {
    try {
      const reponse = await $fetch<ApiResponse<UniversiteStatsAPI>>(
        `${apiBase}/api/universite/stats`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des statistiques')
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur obtenirStatsUniversite:', e)
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerFormations,
    obtenirFormation,
    inscrireFormation,
    obtenirStatsUniversite,
  }
}

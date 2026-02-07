// Composable pour les appels API Evenements
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type TypeEvenement = 'En ligne' | 'En présentiel' | 'Hybride'
export type StatutEvenement = 'a_venir' | 'en_cours' | 'termine' | 'annule'

export interface EvenementOrganisateur {
  uid: string
  nom: string
  prenom: string | null
  email: string
  photo_url: string | null
}

/** DTO correspondant a EvenementResponse du backend */
export interface EvenementAPI {
  id: string
  titre: string
  description: string
  type: string
  pays: string | null
  ville: string | null
  date_heure_debut: string
  date_heure_fin: string | null
  couverture_url: string | null
  statut: StatutEvenement
  nombre_places: number | null
  nombre_inscrits: number
  user: EvenementOrganisateur
  created_at: string
  updated_at: string
}

/** DTO pour le detail d'un evenement */
export interface EvenementDetailAPI extends EvenementAPI {
  slug: string | null
  adresse: string | null
  lien_en_ligne: string | null
  est_inscrit: boolean
}

/** Reponse paginee */
export interface EvenementListeAPI {
  evenements: EvenementAPI[]
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
export interface EvenementFiltres {
  recherche?: string
  format?: string
  pays?: string
  annee?: number
  page?: number
  par_page?: number
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const TYPES_EVENEMENT: { value: string; label: string }[] = [
  { value: '', label: 'Tous les types' },
  { value: 'en_ligne', label: 'En ligne' },
  { value: 'presentiel', label: 'En présentiel' },
  { value: 'hybride', label: 'Hybride' },
]

export const ANNEES = ['2025', '2026', '2027', '2028']

export const PAYS_AFRICAINS = [
  'Afrique du Sud',
  'Algérie',
  'Bénin',
  'Burkina Faso',
  'Cameroun',
  'Cap-Vert',
  'Comores',
  'Côte d\'Ivoire',
  'Égypte',
  'Éthiopie',
  'Gabon',
  'Gambie',
  'Ghana',
  'Guinée',
  'Kenya',
  'Madagascar',
  'Mali',
  'Maroc',
  'Maurice',
  'Mauritanie',
  'Namibie',
  'Niger',
  'Nigeria',
  'RDC',
  'Rwanda',
  'Sénégal',
  'Tanzanie',
  'Togo',
  'Tunisie',
]

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/** Mapper format DB vers label frontend */
export const mapperFormatFrontend = (format: string): string => {
  const map: Record<string, string> = {
    en_ligne: 'En ligne',
    presentiel: 'En présentiel',
    hybride: 'Hybride',
  }
  return map[format] || format
}

/** Mapper format frontend vers valeur DB */
export const mapperFormatDb = (type: string): string => {
  const map: Record<string, string> = {
    'En ligne': 'en_ligne',
    'En présentiel': 'presentiel',
    'Hybride': 'hybride',
  }
  return map[type] || type.toLowerCase().replace(/ /g, '_')
}

/** Formater une date ISO en francais complet (ex: "mardi 15 janvier 2025") */
export const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

/** Formater une date ISO en format court (ex: "15 janvier 2025") */
export const formatDateShort = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
}

/** Extraire l'heure d'une date ISO (ex: "14:30") */
export const getHeure = (dateStr: string | null): string => {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  return date.toLocaleTimeString('fr-FR', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
  })
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useEvenements = () => {
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
   * Lister les evenements avec filtres et pagination
   */
  const listerEvenements = async (filtres: EvenementFiltres = {}): Promise<EvenementListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.format) params.set('format', filtres.format)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.annee) params.set('annee', String(filtres.annee))
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/evenements${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<EvenementListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des evenements')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerEvenements:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir un evenement par son ID
   */
  const obtenirEvenement = async (id: string): Promise<EvenementDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<EvenementDetailAPI>>(
        `${apiBase}/api/evenements/${id}`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Evenement non trouve')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirEvenement:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Creer un evenement (multipart pour l'image de couverture)
   */
  const creerEvenement = async (
    formData: {
      titre: string
      description: string
      type: string
      pays: string
      ville: string
      date_heure_debut: string
      date_heure_fin: string
    },
    couvertureFile: File | null,
  ): Promise<EvenementDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const data = new FormData()
      data.append('titre', formData.titre)
      data.append('description', formData.description)
      data.append('format', mapperFormatDb(formData.type))
      data.append('pays', formData.pays)
      data.append('ville', formData.ville)
      data.append('date_heure_debut', formData.date_heure_debut)
      data.append('date_heure_fin', formData.date_heure_fin)
      if (couvertureFile) {
        data.append('couverture', couvertureFile)
      }

      const reponse = await $fetch<ApiResponse<EvenementDetailAPI>>(
        `${apiBase}/api/evenements`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation de l\'evenement')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerEvenement:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * S'inscrire a un evenement
   */
  const inscrireEvenement = async (evenementId: string): Promise<boolean> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/evenements/${evenementId}/inscription`,
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
      console.error('Erreur inscrireEvenement:', e)
      return false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerEvenements,
    obtenirEvenement,
    creerEvenement,
    inscrireEvenement,
  }
}

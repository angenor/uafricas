// Composable pour les appels API Africantives (initiatives africaines)
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export interface AfricantiveAuteur {
  uid: string
  nom: string
  prenom: string
  email: string
}

export interface AfricantiveAuteurDetail extends AfricantiveAuteur {
  photo_url: string | null
}

/** DTO correspondant a AfricantiveResponse du backend */
export interface AfricantiveAPI {
  id: string
  titre: string
  slug: string | null
  description: string
  image_couverture_url: string | null
  domaine: string | null
  pays: string | null
  ville: string | null
  user: AfricantiveAuteur
  created_at: string
  updated_at: string
}

/** DTO pour le detail */
export interface AfricantiveDetailAPI {
  id: string
  titre: string
  slug: string | null
  description: string
  image_couverture_url: string | null
  domaine: string | null
  domaine_slug: string | null
  pays: string | null
  ville: string | null
  user: AfricantiveAuteurDetail
  created_at: string
  updated_at: string
}

/** Reponse paginee */
export interface AfricantiveListeAPI {
  africantives: AfricantiveAPI[]
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
export interface AfricantiveFiltres {
  recherche?: string
  domaine?: string
  pays?: string
  tri?: string
  page?: number
  par_page?: number
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const DOMAINES_AFRICANTIVES: { value: string; label: string }[] = [
  { value: '', label: 'Tous les domaines' },
  { value: 'Agriculture', label: 'Agriculture' },
  { value: 'Technologie', label: 'Technologie' },
  { value: 'Éducation', label: 'Éducation' },
  { value: 'Santé', label: 'Santé' },
  { value: 'Énergie', label: 'Énergie' },
  { value: 'Environnement', label: 'Environnement' },
  { value: 'Finance', label: 'Finance' },
  { value: 'Culture', label: 'Culture' },
  { value: 'Transport', label: 'Transport' },
  { value: 'Commerce', label: 'Commerce' },
  { value: 'Associations de diaspora', label: 'Associations de diaspora' },
  { value: 'Transfert d\'argent', label: 'Transfert d\'argent' },
  { value: 'Investissements en Afrique', label: 'Investissements en Afrique' },
  { value: 'Réseautage professionnel', label: 'Réseautage professionnel' },
  { value: 'Intégration dans les pays d\'accueil', label: 'Intégration dans les territoires d\'accueil' },
  { value: 'Associations de solidarité et de développement en Afrique', label: 'Associations de solidarité et de développement en Afrique' },
  { value: 'Innovations', label: 'Innovations' },
  { value: 'Entrepreneuriat', label: 'Entrepreneuriat' },
]

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

/** Formater une date ISO en francais (ex: "15 janvier 2025") */
export const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
}

/** Formater une date ISO en format court (ex: "15 jan. 2025") */
export const formatDateCourte = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  })
}

/** Tronquer une description pour l'affichage en carte */
export const tronquerDescription = (text: string, maxLength: number = 120): string => {
  if (text.length <= maxLength) return text
  return text.substring(0, maxLength).trim() + '...'
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useAfricantives = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  /** Prefixer les URLs relatives d'images avec l'URL du backend */
  const resoudreUrl = (url: string | null): string | null => {
    if (!url) return null
    if (url.startsWith('http')) return url
    return `${apiBase}${url}`
  }

  /**
   * Lister les africantives avec filtres et pagination
   */
  const listerAfricantives = async (filtres: AfricantiveFiltres = {}): Promise<AfricantiveListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.domaine) params.set('domaine', filtres.domaine)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.tri) params.set('tri', filtres.tri)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/africantives${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<AfricantiveListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des initiatives')
      }

      // Resoudre les URLs d'images
      reponse.data.africantives = reponse.data.africantives.map(a => ({
        ...a,
        image_couverture_url: resoudreUrl(a.image_couverture_url),
      }))

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur listerAfricantives:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir une africantive par son ID
   */
  const obtenirAfricantive = async (id: string): Promise<AfricantiveDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<AfricantiveDetailAPI>>(
        `${apiBase}/api/africantives/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Initiative non trouvée')
      }

      // Resoudre les URLs d'images
      reponse.data.image_couverture_url = resoudreUrl(reponse.data.image_couverture_url)
      if (reponse.data.user.photo_url) {
        reponse.data.user.photo_url = resoudreUrl(reponse.data.user.photo_url)
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur obtenirAfricantive:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Creer une africantive (multipart pour l'image de couverture)
   */
  const creerAfricantive = async (
    formData: {
      titre: string
      description: string
      domaine?: string
      pays?: string
      ville?: string
    },
    couvertureFile: File | null,
  ): Promise<AfricantiveDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const data = new FormData()
      data.append('titre', formData.titre)
      data.append('description', formData.description)
      if (formData.domaine) data.append('domaine', formData.domaine)
      if (formData.pays) data.append('pays', formData.pays)
      if (formData.ville) data.append('ville', formData.ville)
      if (couvertureFile) data.append('couverture', couvertureFile)

      const reponse = await $fetch<ApiResponse<AfricantiveDetailAPI>>(
        `${apiBase}/api/africantives`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la création de l\'initiative')
      }

      // Resoudre les URLs d'images
      reponse.data.image_couverture_url = resoudreUrl(reponse.data.image_couverture_url)
      if (reponse.data.user.photo_url) {
        reponse.data.user.photo_url = resoudreUrl(reponse.data.user.photo_url)
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur creerAfricantive:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Lister les domaines disponibles
   */
  const listerDomaines = async (): Promise<string[]> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/africantives/domaines`,
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch {
      return []
    }
  }

  /**
   * Lister les pays avec des africantives
   */
  const listerPays = async (): Promise<string[]> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/africantives/pays`,
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch {
      return []
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerAfricantives,
    obtenirAfricantive,
    creerAfricantive,
    listerDomaines,
    listerPays,
  }
}

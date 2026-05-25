// Composable pour les appels API Experts
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export interface ExpertiseInfoAPI {
  domaine: string
  biographie: string
  nbAnneesExperience: number
  rating: number
  portfolio: string | null
  statut: 'valide' | 'en_attente' | 'refuse'
}

/** DTO correspondant a ExpertResponse du backend */
export interface ExpertAPI {
  id: string
  nom: string
  prenom: string
  photoURL: string | null
  pays: string
  ville: string | null
  email: string
  expertiseInfo: ExpertiseInfoAPI
  situationProfessionnelle: string[]
  dateInscription: string
  dateDerniereMiseAJour: string
}

/** Reponse paginee */
export interface ExpertListeAPI {
  experts: ExpertAPI[]
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
export interface ExpertFiltres {
  recherche?: string
  domaine?: string
  pays?: string
  situation?: string
  tri?: 'recent' | 'experience' | 'rating'
  page?: number
  par_page?: number
}

/** Body pour creer une candidature */
export interface CandidatureExpertBody {
  domaine: string
  biographie: string
  nb_annees_experience: number
  portfolio?: string
  situations_professionnelles: string[]
}

/** Suivi de la candidature active du membre (US3) */
export interface MaCandidatureAPI {
  id: string
  domaine: string
  biographie: string
  nbAnneesExperience: number
  portfolio: string | null
  situationsProfessionnelles: string[]
  statut: 'en_attente' | 'valide' | 'refuse'
  commentaireAdmin: string | null
  dateValidation: string | null
  createdAt: string
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const CATEGORIES_EXPERTISE: string[] = [
  'Tout',
  'Agriculture',
  'Informatique',
  'Électronique',
  'Immobilier',
  'Mécanique',
  'Santé',
  'Éducation',
  'Finance',
]

export interface ProfilProfessionnel {
  id: string
  label: string
  icon: string
  color: string
}

export const PROFILS_PROFESSIONNELS: ProfilProfessionnel[] = [
  { id: 'tous', label: 'Tous les profils', icon: 'fas fa-users', color: 'gray' },
  { id: 'recherche_emploi', label: 'En recherche d\'emploi', icon: 'fas fa-search', color: 'red' },
  { id: 'en_emploi', label: 'En Emploi', icon: 'fas fa-briefcase', color: 'green' },
  { id: 'consultance', label: 'Consultant', icon: 'fas fa-user-tie', color: 'blue' },
  { id: 'volontariat_expertise', label: 'Volontariat (partage d\'expertise)', icon: 'fas fa-heart', color: 'purple' },
  { id: 'recherche_nouvelles_opportunites', label: 'En Emploi mais recherche de nouvelles opportunités', icon: 'fas fa-exchange-alt', color: 'orange' },
]

export interface PaysOption {
  value: string
  label: string
}

export const PAYS_EXPERTS: PaysOption[] = [
  { value: '', label: 'Tous les pays' },
  { value: 'Sénégal', label: 'Sénégal' },
  { value: 'Côte d\'Ivoire', label: 'Côte d\'Ivoire' },
  { value: 'Cameroun', label: 'Cameroun' },
  { value: 'Mali', label: 'Mali' },
  { value: 'Burkina Faso', label: 'Burkina Faso' },
  { value: 'Bénin', label: 'Bénin' },
  { value: 'Togo', label: 'Togo' },
  { value: 'Niger', label: 'Niger' },
  { value: 'Guinée', label: 'Guinée' },
  { value: 'Congo', label: 'Congo' },
  { value: 'RD Congo', label: 'RD Congo' },
  { value: 'Gabon', label: 'Gabon' },
  { value: 'Rwanda', label: 'Rwanda' },
  { value: 'Maroc', label: 'Maroc' },
  { value: 'Tunisie', label: 'Tunisie' },
  { value: 'Algérie', label: 'Algérie' },
  { value: 'Nigeria', label: 'Nigeria' },
  { value: 'Ghana', label: 'Ghana' },
  { value: 'Kenya', label: 'Kenya' },
  { value: 'Afrique du Sud', label: 'Afrique du Sud' },
  { value: 'France', label: 'France' },
  { value: 'Belgique', label: 'Belgique' },
  { value: 'Canada', label: 'Canada' },
  { value: 'États-Unis', label: 'États-Unis' },
]

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useExperts = () => {
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
   * Lister les experts avec filtres et pagination
   */
  const listerExperts = async (filtres: ExpertFiltres = {}): Promise<ExpertListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.domaine && filtres.domaine !== 'Tout') params.set('domaine', filtres.domaine)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.situation && filtres.situation !== 'tous') params.set('situation', filtres.situation)
      if (filtres.tri) params.set('tri', filtres.tri)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/experts${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ExpertListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des experts')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerExperts:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir un expert par son ID (utilisateur_id)
   */
  const obtenirExpert = async (id: string): Promise<ExpertAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ExpertAPI>>(
        `${apiBase}/api/experts/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Expert non trouve')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirExpert:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Creer une candidature expert (JWT requis)
   */
  const creerCandidature = async (body: CandidatureExpertBody): Promise<ExpertAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ExpertAPI>>(
        `${apiBase}/api/experts/candidature`,
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
        throw new Error(reponse.error || 'Erreur lors de la creation de la candidature')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerCandidature:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir la candidature active du membre connecte (suivi US3).
   * Renvoie null si aucune candidature active.
   */
  const obtenirMaCandidature = async (): Promise<MaCandidatureAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<MaCandidatureAPI | null>>(
        `${apiBase}/api/experts/moi`,
        { headers: authHeaders() },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors du chargement de la candidature')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirMaCandidature:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerExperts,
    obtenirExpert,
    creerCandidature,
    obtenirMaCandidature,
  }
}

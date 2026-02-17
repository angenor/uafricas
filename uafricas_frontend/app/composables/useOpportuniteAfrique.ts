// Composable pour les appels API Fiches Pays (Opportunites en Afrique)

import { useUserStore } from '~/stores/user'

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

// ── Contributions ──────────────────────────────────────────────

/** Auteur d'une contribution */
export interface ContributionAuteurAPI {
  id: string
  nom: string
  prenom: string
  photo_url: string | null
}

/** DTO d'une contribution */
export interface ContributionFicheAPI {
  id: string
  fiche_pays_id: string
  section: string
  type_contribution: string
  ancienne_valeur: string | null
  nouvelle_valeur: string
  justification: string | null
  etat: string
  auteur: ContributionAuteurAPI
  traite_par: string | null
  note_moderation: string | null
  traite_at: string | null
  created_at: string
}

/** Reponse paginee des contributions */
export interface ContributionListeAPI {
  contributions: ContributionFicheAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** DTO d'un contributeur */
export interface ContributeurAPI {
  utilisateur_id: string
  nom: string
  prenom: string
  photo_url: string | null
  nombre_contributions: number
}

/** Filtres pour lister les contributions */
export interface ContributionFiltres {
  etat?: string
  section?: string
  page?: number
  par_page?: number
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

/** Sections modifiables d'une fiche pays */
export const SECTIONS_FICHE_PAYS = [
  { value: 'population', label: 'Population' },
  { value: 'superficie_km2', label: 'Superficie' },
  { value: 'biographie', label: 'Biographie' },
  { value: 'contexte', label: 'Contexte general' },
  { value: 'contexte_historique', label: 'Contexte historique' },
  { value: 'slogan', label: 'Slogan' },
  { value: 'hymne_national', label: 'Hymne national' },
  { value: 'langue_officielle', label: 'Langue officielle' },
  { value: 'langues_populaires', label: 'Langues populaires' },
  { value: 'monnaie', label: 'Monnaie' },
  { value: 'fuseau_horaire', label: 'Fuseau horaire' },
  { value: 'groupe_ethnique', label: 'Groupe ethnique' },
  { value: 'site_touristique', label: 'Site touristique' },
  { value: 'secteur_developpement', label: 'Secteur de developpement' },
] as const

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
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

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

  // ── Methodes Contributions ─────────────────────────────────

  /** Soumettre une contribution pour une fiche pays */
  const soumettreContribution = async (
    ficheId: string,
    body: {
      section: string
      type_contribution?: string
      nouvelle_valeur: string
      justification?: string
    },
  ): Promise<ContributionFicheAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ContributionFicheAPI>>(
        `${apiBase}/api/fiches-pays/${ficheId}/contributions`,
        {
          method: 'POST',
          headers: authHeaders(),
          body,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la soumission')
      }
      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur soumettreContribution:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Lister les contributions d'une fiche */
  const listerContributions = async (
    ficheId: string,
    filtres: ContributionFiltres = {},
  ): Promise<ContributionListeAPI | null> => {
    try {
      const params = new URLSearchParams()
      if (filtres.etat) params.set('etat', filtres.etat)
      if (filtres.section) params.set('section', filtres.section)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/fiches-pays/${ficheId}/contributions${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ContributionListeAPI>>(url, {
        headers: authHeaders(),
      })

      if (!reponse.success || !reponse.data) return null
      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerContributions:', e)
      return null
    }
  }

  /** Lister les contributeurs valides d'une fiche */
  const listerContributeurs = async (ficheId: string): Promise<ContributeurAPI[]> => {
    try {
      const reponse = await $fetch<ApiResponse<ContributeurAPI[]>>(
        `${apiBase}/api/fiches-pays/${ficheId}/contributeurs`,
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch {
      return []
    }
  }

  /** Valider une contribution (admin) */
  const validerContribution = async (
    contributionId: string,
    note?: string,
  ): Promise<ContributionFicheAPI | null> => {
    try {
      const reponse = await $fetch<ApiResponse<ContributionFicheAPI>>(
        `${apiBase}/api/fiches-pays/contributions/${contributionId}/valider`,
        {
          method: 'PUT',
          headers: authHeaders(),
          body: { note_moderation: note },
        },
      )
      if (!reponse.success || !reponse.data) return null
      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur validerContribution:', e)
      return null
    }
  }

  /** Rejeter une contribution (admin) */
  const rejeterContribution = async (
    contributionId: string,
    note?: string,
  ): Promise<ContributionFicheAPI | null> => {
    try {
      const reponse = await $fetch<ApiResponse<ContributionFicheAPI>>(
        `${apiBase}/api/fiches-pays/contributions/${contributionId}/rejeter`,
        {
          method: 'PUT',
          headers: authHeaders(),
          body: { note_moderation: note },
        },
      )
      if (!reponse.success || !reponse.data) return null
      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur rejeterContribution:', e)
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerFiches,
    obtenirFiche,
    listerRegions,
    // Contributions
    soumettreContribution,
    listerContributions,
    listerContributeurs,
    validerContribution,
    rejeterContribution,
  }
}

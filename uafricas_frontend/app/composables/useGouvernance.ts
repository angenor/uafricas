import type { ContributionCitoyenne, ContributionStats } from '~/mocks/gouvernance/contributions'

interface ApiGouvernanceStats {
  total: number
  factcheck: number
  badhabits: number
  ideaforces: number
  total_likes: number
}

interface ApiContributionAuteur {
  id: string
  prenom: string
  nom: string
  photo_url: string | null
}

interface ApiContributionLocalisation {
  pays: string
  region: string | null
  ville: string | null
}

interface ApiContributionStats {
  likes: number
  soutiens: number
}

interface ApiContribution {
  id: string
  type: 'factcheck' | 'badhabits' | 'ideaforces'
  statut: string
  titre: string
  description: string
  auteur: ApiContributionAuteur
  localisation: ApiContributionLocalisation
  date_creation: string
  stats: ApiContributionStats
}

interface ApiContributionListeResponse {
  contributions: ApiContribution[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

const API_BASE = 'http://127.0.0.1:8080/api'

/** Mapper une contribution API vers l'interface frontend ContributionCitoyenne */
function mapperContribution(api: ApiContribution): ContributionCitoyenne {
  return {
    id: api.id,
    type: api.type,
    statut: api.statut as 'brouillon' | 'publie' | 'archive',
    titre: api.titre,
    description: api.description,
    auteur: {
      id: api.auteur.id,
      prenom: api.auteur.prenom,
      nom: api.auteur.nom,
      photoURL: api.auteur.photo_url ?? undefined,
    },
    localisation: {
      pays: api.localisation.pays,
      region: api.localisation.region ?? undefined,
      ville: api.localisation.ville ?? undefined,
    },
    dateCreation: new Date(api.date_creation),
    stats: {
      vues: 0,
      vuesUniques: 0,
      likes: api.stats.likes,
      commentaires: 0,
      partages: 0,
      soutiens: api.stats.soutiens,
    },
    tags: [],
  }
}

export function useGouvernance() {
  const loading = ref(false)
  const error = ref<string | null>(null)

  /** Recuperer les statistiques de gouvernance */
  async function getStats(): Promise<{ factcheck: number; badhabits: number; ideaforces: number; total: number; totalLikes: number }> {
    const reponse = await $fetch<ApiResponse<ApiGouvernanceStats>>(
      `${API_BASE}/gouvernance/stats`
    )

    if (!reponse.success || !reponse.data) {
      throw new Error(reponse.error || 'Erreur lors du chargement des statistiques')
    }

    return {
      total: reponse.data.total,
      factcheck: reponse.data.factcheck,
      badhabits: reponse.data.badhabits,
      ideaforces: reponse.data.ideaforces,
      totalLikes: reponse.data.total_likes,
    }
  }

  /** Recuperer la liste paginee des contributions */
  async function getContributions(options?: {
    page?: number
    parPage?: number
    type?: string
  }): Promise<{ contributions: ContributionCitoyenne[]; total: number; totalPages: number }> {
    const params = new URLSearchParams()
    if (options?.page) params.set('page', String(options.page))
    if (options?.parPage) params.set('par_page', String(options.parPage))
    if (options?.type) params.set('type', options.type)

    const reponse = await $fetch<ApiResponse<ApiContributionListeResponse>>(
      `${API_BASE}/gouvernance/contributions?${params.toString()}`
    )

    if (!reponse.success || !reponse.data) {
      throw new Error(reponse.error || 'Erreur lors du chargement des contributions')
    }

    return {
      contributions: reponse.data.contributions.map(mapperContribution),
      total: reponse.data.total,
      totalPages: reponse.data.total_pages,
    }
  }

  return {
    loading,
    error,
    getStats,
    getContributions,
  }
}

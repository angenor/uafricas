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

export interface CreerFactcheckPayload {
  contenu: string
  source_originale?: string
  verdict?: 'vrai' | 'faux' | 'partiellement_vrai' | 'trompeur' | 'non_verifie'
  image_couverture_url?: string
  couleur_fond?: string
  pays_id?: string
}

export interface CreerBadHabitPayload {
  titre: string
  description_generale: string
  details_problematique: string
  categorie_probleme:
    | 'corruption'
    | 'service_public_defaillant'
    | 'infrastructure_degradee'
    | 'acces_services_limite'
    | 'insalubrite'
    | 'probleme_securite'
    | 'autre'
  categorie_probleme_detail?: string
  gravite?: 'faible' | 'elevee' | 'critique'
  preuves_temoignages?: string
  solutions_proposees?: string
  publication_anonyme?: boolean
  pays_id?: string
  region?: string
  ville_quartier_zone?: string
  medias_urls?: string[]
}

export interface PaysPublic {
  id: string
  nom: string
  code_iso2: string | null
  code_iso3: string | null
}

export interface CreerIdeaForcePayload {
  titre: string
  description_generale: string
  details_proposition: string
  categorie_proposition:
    | 'amelioration_gouvernance'
    | 'education_formation'
    | 'sante_publique'
    | 'emploi_jeunes'
    | 'environnement'
    | 'transport'
    | 'autre'
  categorie_proposition_detail?: string
  urgence?: 'faible' | 'elevee' | 'critique'
  plan_implementation?: string
  ressources_necessaires?: string
  impact_attendu?: string
  pays_id?: string
  region?: string
  ville_quartier_zone?: string
  medias_urls?: string[]
}

export function useGouvernance() {
  const config = useRuntimeConfig()
  const apiBase = `${config.public.apiBaseUrl as string}/api`
  const userStore = useUserStore()
  const loading = ref(false)
  const error = ref<string | null>(null)

  /** Construire les headers avec token JWT si authentifie */
  function authHeaders(): Record<string, string> {
    return userStore.accessToken
      ? { Authorization: `Bearer ${userStore.accessToken}` }
      : {}
  }

  /** Recuperer les statistiques de gouvernance */
  async function getStats(): Promise<{ factcheck: number; badhabits: number; ideaforces: number; total: number; totalLikes: number }> {
    const reponse = await $fetch<ApiResponse<ApiGouvernanceStats>>(
      `${apiBase}/gouvernance/stats`
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
      `${apiBase}/gouvernance/contributions?${params.toString()}`
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

  /** Recuperer la liste publique des pays actifs */
  async function getPays(): Promise<PaysPublic[]> {
    const reponse = await $fetch<ApiResponse<PaysPublic[]>>(`${apiBase}/pays`)
    if (!reponse.success || !reponse.data) {
      throw new Error(reponse.error || 'Erreur lors du chargement des pays')
    }
    return reponse.data
  }

  /** Publier un factcheck (utilisateur authentifie, publication directe) */
  async function creerFactcheck(payload: CreerFactcheckPayload): Promise<string> {
    if (!userStore.accessToken) {
      throw new Error('Authentification requise pour publier')
    }
    loading.value = true
    error.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/gouvernance/factcheck`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: payload,
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la publication')
      }
      return reponse.data.id
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Erreur inconnue'
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  /** Publier une mauvaise pratique (BadHabit) */
  async function creerBadHabit(payload: CreerBadHabitPayload): Promise<string> {
    if (!userStore.accessToken) {
      throw new Error('Authentification requise pour publier')
    }
    loading.value = true
    error.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/gouvernance/bad-habits`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: payload,
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la publication')
      }
      return reponse.data.id
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Erreur inconnue'
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  /** Publier une idee force (IdeaForce) */
  async function creerIdeaForce(payload: CreerIdeaForcePayload): Promise<string> {
    if (!userStore.accessToken) {
      throw new Error('Authentification requise pour publier')
    }
    loading.value = true
    error.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/gouvernance/idea-forces`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: payload,
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la publication')
      }
      return reponse.data.id
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Erreur inconnue'
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  return {
    loading,
    error,
    getStats,
    getContributions,
    getPays,
    creerFactcheck,
    creerBadHabit,
    creerIdeaForce,
  }
}

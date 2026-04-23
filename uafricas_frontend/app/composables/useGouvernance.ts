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
  categorie?: string | null
  gravite?: string | null
  type_pratique?: string | null
}

/** Mapping slug categorie DB → label FR pour affichage */
const LIBELLES_CATEGORIE_PROBLEME: Record<string, string> = {
  corruption: 'Corruption',
  service_public_defaillant: 'Service public défaillant',
  infrastructure_degradee: 'Infrastructure dégradée',
  acces_services_limite: 'Accès services limité',
  insalubrite: 'Insalubrité',
  probleme_securite: 'Sécurité',
  autre: 'Autre',
}

const LIBELLES_CATEGORIE_PROPOSITION: Record<string, string> = {
  amelioration_gouvernance: 'Gouvernance',
  education_formation: 'Éducation',
  sante_publique: 'Santé',
  emploi_jeunes: 'Emploi jeunes',
  environnement: 'Environnement',
  transport: 'Transport',
  autre: 'Autre',
}

/** Mapping niveau gravite DB (faible/elevee/critique) → niveau affiche sur la page */
function mapperGraviteProblematique(g: string | null | undefined): 'faible' | 'moyenne' | 'grave' | 'critique' | undefined {
  if (!g) return undefined
  if (g === 'critique') return 'critique'
  if (g === 'elevee') return 'grave'
  if (g === 'faible') return 'faible'
  return undefined
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
  const typePratique = api.type_pratique === 'mauvaise' || api.type_pratique === 'bonne'
    ? (api.type_pratique as TypePratique)
    : undefined

  const base: ContributionCitoyenne = {
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
    typePratique,
  }

  if (api.type === 'badhabits') {
    const categorieLabel = api.categorie
      ? (LIBELLES_CATEGORIE_PROBLEME[api.categorie] ?? api.categorie)
      : 'Autre'
    base.problematique = {
      categorie: categorieLabel,
      gravite: mapperGraviteProblematique(api.gravite),
    }
  } else if (api.type === 'ideaforces') {
    const categorieLabel = api.categorie
      ? (LIBELLES_CATEGORIE_PROPOSITION[api.categorie] ?? api.categorie)
      : 'Autre'
    base.proposition = {
      objectif: categorieLabel,
      moyens: [],
      beneficiaires: [],
      impact: api.gravite ?? '',
    }
  }

  return base
}

export interface CreerFactcheckPayload {
  contenu: string
  source_originale?: string
  verdict?: 'vrai' | 'faux' | 'partiellement_vrai' | 'trompeur' | 'non_verifie'
  image_couverture_url?: string
  couleur_fond?: string
  pays_id?: string
}

export type TypePratique = 'mauvaise' | 'bonne'

export type CategorieProbleme =
  | 'corruption'
  | 'service_public_defaillant'
  | 'infrastructure_degradee'
  | 'acces_services_limite'
  | 'insalubrite'
  | 'probleme_securite'
  | 'autre'

export type CategorieBonnePratique =
  | 'civisme'
  | 'service_public_exemplaire'
  | 'solidarite'
  | 'innovation_sociale'
  | 'initiative_citoyenne'
  | 'leadership_exemplaire'
  | 'transparence'
  | 'environnement'
  | 'education'
  | 'sante'
  | 'autre'

export interface CreerBadHabitPayload {
  type_pratique?: TypePratique
  titre: string
  description_generale: string
  details_problematique: string
  categorie_probleme: CategorieProbleme | CategorieBonnePratique
  categorie_probleme_detail?: string
  gravite?: 'faible' | 'elevee' | 'critique'
  impact?: 'faible' | 'fort' | 'exemplaire'
  preuves_temoignages?: string
  solutions_proposees?: string
  reproductibilite?: string
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

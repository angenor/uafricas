import type {
  ContributionCitoyenne,
  ContributionStats,
  ReactionsGlobales,
  TypePreuve,
  TypePublicationFactcheck,
  TypeReactionGlobale,
} from '~/types/gouvernance'

/** Transformer une URL relative (`/uploads/...`) en URL absolue */
function mapperUrlAbsolue(url: string | null | undefined, apiBase: string): string | undefined {
  if (!url) return undefined
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

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

interface ApiFactcheckVolet {
  titre: string
  description: string
  likes: number
}

interface ApiFactcheckReactions {
  coeur: number
  pouce: number
  rire: number
  jaime_pas: number
  ma_reaction: string | null
}

interface ApiFactcheckDetail {
  prejuge: ApiFactcheckVolet
  contrePrejuge: ApiFactcheckVolet
  reactions: ApiFactcheckReactions
  a_like_prejuge: boolean
  a_like_realite: boolean
  a_signale: boolean
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
  type_publication?: string | null
  preuve_url?: string | null
  preuve_type?: string | null
  image_couverture_url?: string | null
  images?: string[] | null
  factcheck?: ApiFactcheckDetail | null
}

/** État de réaction renvoyé par POST .../reaction */
interface ApiFactcheckReactionEtat {
  nombre_coeur: number
  nombre_pouce: number
  nombre_rire: number
  nombre_jaime_pas: number
  prejuge_nombre_likes: number
  realite_nombre_likes: number
  ma_reaction_general: string | null
  a_like_prejuge: boolean
  a_like_realite: boolean
}

export interface ReactionEtat {
  reactions: ReactionsGlobales
  prejugeLikes: number
  realiteLikes: number
  aLikePrejuge: boolean
  aLikeRealite: boolean
}

/** État de signalement renvoyé par POST .../signalement */
interface ApiSignalementEtat {
  nombre_signalements: number
  etat: string
  deja_signale: boolean
  suspendu: boolean
}

export interface SignalementEtat {
  nombreSignalements: number
  etat: string
  dejaSignale: boolean
  suspendu: boolean
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
  union_africains: 'Union des africains',
  infrastructures: 'Infrastructures',
  retour_cerveaux: 'Retour des cerveaux',
  union_diaspora: 'Union de la diaspora',
  lutte_corruption: 'Lutte contre la corruption',
  urbanisation_durable: 'Urbanisation durable',
  acces_energie: "Accès à l'énergie",
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
function mapperContribution(api: ApiContribution, apiBase: string): ContributionCitoyenne {
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
    images: (api.images ?? [])
      .map(u => mapperUrlAbsolue(u, apiBase))
      .filter((u): u is string => !!u),
  }

  if (api.type === 'factcheck' && api.factcheck) {
    const fc = api.factcheck
    base.factcheck = {
      prejuge: { titre: fc.prejuge.titre, description: fc.prejuge.description, likes: fc.prejuge.likes },
      contrePrejuge: { titre: fc.contrePrejuge.titre, description: fc.contrePrejuge.description, likes: fc.contrePrejuge.likes },
    }
    base.reactions = {
      coeur: fc.reactions.coeur,
      pouce: fc.reactions.pouce,
      rire: fc.reactions.rire,
      jaimePas: fc.reactions.jaime_pas,
      maReaction: (fc.reactions.ma_reaction as TypeReactionGlobale | null) ?? null,
    }
    base.aLikePrejuge = fc.a_like_prejuge
    base.aLikeRealite = fc.a_like_realite
    base.aSignale = fc.a_signale
    base.typePublication = (api.type_publication as TypePublicationFactcheck | null) ?? undefined
    base.preuveUrl = mapperUrlAbsolue(api.preuve_url, apiBase)
    base.preuveType = (api.preuve_type as TypePreuve | null) ?? undefined
    base.imageUrl = mapperUrlAbsolue(api.image_couverture_url, apiBase)
  } else if (api.type === 'badhabits') {
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
  prejuge_titre?: string
  prejuge_description?: string
  realite_titre?: string
  realite_description?: string
  type_publication?: TypePublicationFactcheck
  preuve_url?: string
  preuve_type?: TypePreuve
}

// ── Partage de contribution vers /publications ──────────────────

export interface PartageContributionAuteurAPI {
  id: string
  prenom: string
  nom: string
  photo_url: string | null
}

export interface PartageContributionApercuAPI {
  id: string
  type_contribution: 'factcheck' | 'badhabits' | 'ideaforces'
  titre: string
  description: string | null
  categorie: string | null
  image_couverture_url: string | null
}

export interface PartageContributionAPI {
  id: string
  legende: string | null
  created_at: string
  contribution: PartageContributionApercuAPI
  auteur: PartageContributionAuteurAPI
}

export interface PartageContributionListeAPI {
  partages: PartageContributionAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
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
  /** Preuves (photos) : URLs relatives uploadées (mauvaise pratique) */
  preuves_photos?: string[]
  /** Solutions proposées (10 propositions maximum) */
  solutions_propositions?: string[]
  /** Identité réelle de l'auteur (obligatoire pour une mauvaise pratique) */
  identite_nom?: string
  identite_prenom?: string
  identite_courriel?: string
  identite_contact?: string
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
    | 'union_africains'
    | 'infrastructures'
    | 'retour_cerveaux'
    | 'union_diaspora'
    | 'lutte_corruption'
    | 'urbanisation_durable'
    | 'acces_energie'
    | 'autre'
  categorie_proposition_detail?: string
  urgence?: 'faible' | 'elevee' | 'critique'
  plan_implementation?: string
  ressources_necessaires?: string
  impact_attendu?: string
  /** Modalités opérationnelles concrètes proposées (10 étapes maximum) */
  modalites_operationnelles?: string[]
  pays_id?: string
  region?: string
  ville_quartier_zone?: string
  medias_urls?: string[]
}

export function useGouvernance() {
  const config = useRuntimeConfig()
  const apiBase = `${config.public.apiBaseUrl as string}/api`
  // Les fichiers (uploads) sont servis à la racine (`/uploads/...`), PAS sous `/api`.
  const assetBase = config.public.apiBaseUrl as string
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

    // Header d'auth optionnel : permet au backend de renseigner l'état de
    // réaction personnalisé (ma_reaction / a_like_*) quand l'utilisateur est connecté.
    const reponse = await $fetch<ApiResponse<ApiContributionListeResponse>>(
      `${apiBase}/gouvernance/contributions?${params.toString()}`,
      { headers: authHeaders() },
    )

    if (!reponse.success || !reponse.data) {
      throw new Error(reponse.error || 'Erreur lors du chargement des contributions')
    }

    return {
      contributions: reponse.data.contributions.map(c => mapperContribution(c, assetBase)),
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

  /**
   * Uploader une preuve (photo ou PDF) pour un fait vécu.
   * Retourne l'URL relative et le type détecté par le backend.
   */
  async function uploaderPreuve(fichier: File): Promise<{ url: string; preuveType: TypePreuve }> {
    if (!userStore.accessToken) {
      throw new Error('Authentification requise pour téléverser une preuve')
    }
    const formData = new FormData()
    formData.append('fichier', fichier)
    const reponse = await $fetch<ApiResponse<{ url: string; preuve_type: TypePreuve }>>(
      `${apiBase}/gouvernance/factcheck/upload-preuve`,
      {
        method: 'POST',
        headers: authHeaders(),
        body: formData,
      },
    )
    if (!reponse.success || !reponse.data) {
      throw new Error(reponse.error || 'Erreur lors du téléversement de la preuve')
    }
    return { url: reponse.data.url, preuveType: reponse.data.preuve_type }
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

  /**
   * Réagir à un factcheck (toggle). Cible : 'general' (avec type d'emoji),
   * 'prejuge' ou 'realite' (cœur implicite). Renvoie l'état de réaction à jour.
   */
  async function reagir(
    factcheckId: string,
    cible: 'general' | 'prejuge' | 'realite',
    typeReaction?: TypeReactionGlobale,
  ): Promise<ReactionEtat> {
    if (!userStore.accessToken) {
      throw new Error('Authentification requise pour réagir')
    }
    const reponse = await $fetch<ApiResponse<ApiFactcheckReactionEtat>>(
      `${apiBase}/gouvernance/factcheck/${factcheckId}/reaction`,
      {
        method: 'POST',
        headers: authHeaders(),
        body: { cible, type_reaction: cible === 'general' ? typeReaction : undefined },
      },
    )
    if (!reponse.success || !reponse.data) {
      throw new Error(reponse.error || 'Erreur lors de la réaction')
    }
    const d = reponse.data
    return {
      reactions: {
        coeur: d.nombre_coeur,
        pouce: d.nombre_pouce,
        rire: d.nombre_rire,
        jaimePas: d.nombre_jaime_pas,
        maReaction: (d.ma_reaction_general as TypeReactionGlobale | null) ?? null,
      },
      prejugeLikes: d.prejuge_nombre_likes,
      realiteLikes: d.realite_nombre_likes,
      aLikePrejuge: d.a_like_prejuge,
      aLikeRealite: d.a_like_realite,
    }
  }

  /**
   * Partager une contribution (factcheck/badhabits/ideaforces) sur le mur
   * /publications, avec une légende facultative. JWT requis.
   */
  async function partagerContribution(
    typeContribution: 'factcheck' | 'badhabits' | 'ideaforces',
    contributionId: string,
    legende?: string,
  ): Promise<PartageContributionAPI> {
    if (!userStore.accessToken) {
      throw new Error('Authentification requise pour partager')
    }
    const reponse = await $fetch<ApiResponse<PartageContributionAPI>>(
      `${apiBase}/gouvernance/partages`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: {
          type_contribution: typeContribution,
          contribution_id: contributionId,
          legende: legende || undefined,
        },
      },
    )
    if (!reponse.success || !reponse.data) {
      throw new Error(reponse.error || 'Erreur lors du partage')
    }
    return reponse.data
  }

  /** Liste paginée des contributions partagées (mur public). */
  async function listerPartagesContributions(
    page = 1,
    parPage = 20,
  ): Promise<PartageContributionListeAPI | null> {
    try {
      const reponse = await $fetch<ApiResponse<PartageContributionListeAPI>>(
        `${apiBase}/gouvernance/partages?page=${page}&par_page=${parPage}`,
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des partages')
      }
      return reponse.data
    } catch (err) {
      console.error('Erreur listerPartagesContributions:', err)
      return null
    }
  }

  /**
   * Signaler un factcheck. Au-delà de 20 signalements distincts, le backend
   * suspend la publication (etat='suspendu') et elle quitte la liste publique.
   */
  async function signaler(
    factcheckId: string,
    motif?: string,
    commentaire?: string,
  ): Promise<SignalementEtat> {
    if (!userStore.accessToken) {
      throw new Error('Authentification requise pour signaler')
    }
    const reponse = await $fetch<ApiResponse<ApiSignalementEtat>>(
      `${apiBase}/gouvernance/factcheck/${factcheckId}/signalement`,
      {
        method: 'POST',
        headers: authHeaders(),
        body: { motif, commentaire },
      },
    )
    if (!reponse.success || !reponse.data) {
      throw new Error(reponse.error || 'Erreur lors du signalement')
    }
    return {
      nombreSignalements: reponse.data.nombre_signalements,
      etat: reponse.data.etat,
      dejaSignale: reponse.data.deja_signale,
      suspendu: reponse.data.suspendu,
    }
  }

  return {
    loading,
    error,
    getStats,
    getContributions,
    getPays,
    creerFactcheck,
    uploaderPreuve,
    partagerContribution,
    listerPartagesContributions,
    creerBadHabit,
    creerIdeaForce,
    reagir,
    signaler,
  }
}

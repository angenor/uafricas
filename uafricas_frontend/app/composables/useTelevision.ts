import type { CreneauAPI } from '~/composables/useMediaProgrammation'

import type { CompteursInteraction } from '~/composables/useMediaSocial'

// Composable pour les appels API de la télévision

/** Interface correspondant au DTO ChaineTvResponse du backend */
export interface ChaineTvAPI {
  id: string
  nom: string
  slug: string | null
  description: string | null
  stream_url: string | null
  image_couverture_url: string | null
  categorie: string
  pays: string | null
  langue: string
  est_en_direct: boolean
  /** « africans » (Africans Télé International) ou « territoire » — cf. 09o. */
  origine_publication: string
  created_at: string
  /** Réactions, commentaires et partages agrégés (FR-027). */
  interactions?: CompteursInteraction | null
}

/** Interface correspondant au DTO ChaineTvListeResponse du backend */
export interface ChaineTvListeAPI {
  chaines: ChaineTvAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Interface correspondant au DTO ProgrammeTeleResponse du backend */
export interface ProgrammeTeleAPI {
  id: string
  nom_emission: string
  slug: string | null
  description: string
  image_couverture_url: string | null
  video_url: string | null
  info_animateur: string | null
  info_producteur: string | null
  pays: string | null
  est_international: boolean
  langue: string
  chaine_id: string | null
  chaine_nom: string | null
  chaine_slug: string | null
  a_la_une: boolean
  /** Vedette de TOUTE la page Télé — distincte de `a_la_une`, qui vaut par chaîne. */
  a_la_une_globale: boolean
  theme_phare_id: string | null
  theme_phare_autre: string | null
  theme_phare_nom: string | null
  /** "hebergee" | "externe" | "aucune" — décide du lecteur à employer. */
  source_media: string
  created_at: string
  /** Réactions, commentaires et partages agrégés (FR-027). */
  interactions?: CompteursInteraction | null
}

/** Vedette de la page Télé : le programme, plus l'indication d'un repli (FR-007) */
export interface ProgrammeVedetteAPI extends ProgrammeTeleAPI {
  est_repli: boolean
}

/** Une section = une chaîne, son contenu mis en évidence et ses autres contenus */
export interface TeleSectionAPI {
  chaine: ChaineTvAPI
  mis_en_evidence: ProgrammeTeleAPI | null
  contenus: ProgrammeTeleAPI[]
  total_contenus: number
  /** Grille du moment (US5) — absents quand la chaîne n'en a aucune. */
  diffusion_en_cours?: CreneauAPI | null
  creneau_suivant?: CreneauAPI | null
}

export interface TeleSectionsListeAPI {
  sections: TeleSectionAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Interface correspondant au DTO ProgrammeTeleListeResponse du backend */
export interface ProgrammeTeleListeAPI {
  programmes: ProgrammeTeleAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Interface adaptée au format attendu par les composants frontend (chaîne) */
export interface TvChannel {
  id: string
  name: string
  slug: string | null
  description: string
  streamUrl: string
  cover: string
  category: string
  country: string
  language: string
  isLive: boolean
  /** Chaîne de la plateforme (« africans ») ou d'un territoire. */
  origine: string
  /** Compteurs d'interaction, absents tant que l'API ne les greffe pas. */
  interactions: CompteursInteraction | null
}

/** Interface adaptée au format attendu par les composants frontend (programme) */
export interface TvProgram {
  id: string
  slug: string | null
  title: string
  description: string
  banner: string
  videoUrl: string
  animator: string
  producer: string
  country: string
  language: string
  chaineId: string | null
  chaineNom: string | null
  chaineSlug: string | null
  aLaUne: boolean
  aLaUneGlobale: boolean
  themePhare: string | null
  /** Décide du lecteur : fichier natif, intégration tierce, ou aucun. */
  sourceMedia: string
  /** Compteurs d'interaction, absents tant que l'API ne les greffe pas. */
  interactions: CompteursInteraction | null
}

/** Section prête à l'affichage, telle que la consomment les composants */
export interface TeleSection {
  chaine: TvChannel
  misEnEvidence: TvProgram | null
  contenus: TvProgram[]
  totalContenus: number
  /**
   * « En ce moment » et « À suivre » (FR-039), résolus par le serveur à
   * l'instant de la requête. `null` quand la chaîne n'a pas de grille active :
   * la section retombe alors sur son contenu mis en évidence (FR-041).
   */
  diffusionEnCours: CreneauAPI | null
  creneauSuivant: CreneauAPI | null
}

/** Programme vedette de la page, avec l'indication d'un éventuel repli */
export interface ProgrammeVedette extends TvProgram {
  estRepli: boolean
}

/** Réponse API standardisée */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Paramètres de filtre pour le listing des chaînes */
export interface ChaineTvFiltres {
  recherche?: string
  categorie?: string
  pays?: string
  page?: number
  par_page?: number
}

/** Paramètres de filtre pour le listing des sections de la page Télé */
export interface TeleSectionsFiltres {
  recherche?: string
  categorie?: string
  pays?: string
  /** « africans » : Africans Télé International. Vide = toutes les chaînes. */
  origine?: string
  /** Identifiant d'un thème phare (`shared.categorie`, contexte « media »). */
  theme?: string
  /** `true` restreint aux chaînes en direct ; `false`/absent ne filtre pas. */
  en_direct?: boolean
  page?: number
  par_page?: number
  contenus_par_section?: number
}

/** Paramètres de filtre pour le listing des programmes */
export interface ProgrammeTeleFiltres {
  recherche?: string
  pays?: string
  chaine?: string
  page?: number
  par_page?: number
}

/** Formulaire de création de chaîne */
export interface CreerChaineTvForm {
  nom: string
  description?: string
  stream_url?: string
  categorie?: string
  pays?: string
  langue?: string
}

/** Formulaire de création de programme vedette */
export interface CreerProgrammeVedetteForm {
  nom_emission: string
  description: string
  video_url: string
  image_couverture_url?: string
  info_animateur?: string
  info_producteur?: string
  pays?: string
  est_international?: boolean
  langue?: string
}

// ── Mapping API → Frontend ────────────────────────────────────────────

/**
 * Les replis historiques (`/images/tv-default.jpg`, `tv-programme-default.jpg`)
 * n'existent pas dans `public/images/` : ils produisaient une image cassée dès
 * qu'un contenu n'avait pas de couverture. On renvoie désormais une chaîne
 * vide, et les composants affichent un vrai placeholder.
 */
function resoudreUrl(url: string | null, apiBase: string, fallback = ''): string {
  if (!url) return fallback
  if (url.startsWith('http://') || url.startsWith('https://')) return url
  return `${apiBase}${url}`
}

function mapperChaineApiVersTv(chaine: ChaineTvAPI, apiBase: string): TvChannel {
  return {
    id: chaine.id,
    name: chaine.nom,
    slug: chaine.slug,
    description: chaine.description || '',
    streamUrl: chaine.stream_url || '',
    cover: resoudreUrl(chaine.image_couverture_url, apiBase),
    category: chaine.categorie,
    country: chaine.pays || '',
    language: chaine.langue,
    isLive: chaine.est_en_direct,
    origine: chaine.origine_publication || 'territoire',
    interactions: chaine.interactions ?? null,
  }
}

function mapperProgrammeApiVersTv(programme: ProgrammeTeleAPI, apiBase: string): TvProgram {
  return {
    id: programme.id,
    slug: programme.slug,
    title: programme.nom_emission,
    description: programme.description,
    banner: resoudreUrl(programme.image_couverture_url, apiBase),
    // Un lien externe est laissé intact : seul un fichier local doit être
    // préfixé par la base API pour être atteignable depuis le navigateur.
    videoUrl: programme.video_url ? resoudreUrl(programme.video_url, apiBase, '') : '',
    animator: programme.info_animateur || '',
    producer: programme.info_producteur || '',
    country: programme.pays || '',
    language: programme.langue,
    chaineId: programme.chaine_id,
    chaineNom: programme.chaine_nom,
    chaineSlug: programme.chaine_slug ?? null,
    aLaUne: programme.a_la_une,
    aLaUneGlobale: programme.a_la_une_globale ?? false,
    themePhare: programme.theme_phare_nom || programme.theme_phare_autre || null,
    sourceMedia: programme.source_media ?? 'aucune',
    interactions: programme.interactions ?? null,
  }
}

export const useTelevision = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** Headers d'authentification */
  const authHeaders = (): Record<string, string> => {
    if (import.meta.client) {
      const token = localStorage.getItem('accessToken')
      if (token) return { Authorization: `Bearer ${token}` }
    }
    return {}
  }

  /**
   * Récupérer la liste des chaînes TV avec filtres et pagination
   */
  const listerChaines = async (filtres: ChaineTvFiltres = {}): Promise<{ chaines: TvChannel[]; total: number } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.categorie && filtres.categorie !== 'Toutes les catégories') params.set('categorie', filtres.categorie)
      if (filtres.pays && filtres.pays !== 'Tous les territoires') params.set('pays', filtres.pays)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/television/chaines${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ChaineTvListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des chaînes TV')
      }

      return {
        chaines: reponse.data.chaines.map(c => mapperChaineApiVersTv(c, apiBase)),
        total: reponse.data.total,
      }
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur listerChaines:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Récupérer une chaîne par ID
   */
  const obtenirChaine = async (id: string): Promise<TvChannel | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ChaineTvAPI>>(
        `${apiBase}/api/television/chaines/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Chaîne non trouvée')
      }

      return mapperChaineApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur obtenirChaine:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Récupérer la liste des programmes vedettes (TV à la une)
   */
  const listerProgrammesVedettes = async (filtres: ProgrammeTeleFiltres = {}): Promise<{ programmes: TvProgram[]; total: number } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.pays && filtres.pays !== 'Tous les territoires') params.set('pays', filtres.pays)
      if (filtres.chaine) params.set('chaine', filtres.chaine)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/television/programmes-vedettes${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ProgrammeTeleListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des programmes')
      }

      return {
        programmes: reponse.data.programmes.map(p => mapperProgrammeApiVersTv(p, apiBase)),
        total: reponse.data.total,
      }
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur listerProgrammesVedettes:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Émissions publiées d'une chaîne — pendant télé de
   * `useStationsRadio.listerContenusStation`.
   *
   * Alimente notamment le sélecteur de contenu de la grille de programmation :
   * sans lui, un co-détenteur ne pouvait placer aucun créneau (US5).
   */
  const listerContenusChaine = async (chaineId: string): Promise<TvProgram[]> => {
    try {
      const reponse = await $fetch<ApiResponse<ProgrammeTeleListeAPI>>(
        `${apiBase}/api/television/programmes-vedettes?chaine=${chaineId}&par_page=50`,
      )
      if (!reponse.success || !reponse.data) return []
      return reponse.data.programmes.map(p => mapperProgrammeApiVersTv(p, apiBase))
    }
    catch (e: any) {
      console.error('Erreur listerContenusChaine:', e)
      return []
    }
  }

  /**
   * Récupérer un programme vedette par ID
   */
  const obtenirProgrammeVedette = async (id: string): Promise<TvProgram | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ProgrammeTeleAPI>>(
        `${apiBase}/api/television/programmes-vedettes/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Programme non trouvé')
      }

      return mapperProgrammeApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur obtenirProgrammeVedette:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Récupérer la liste des pays disponibles
   */
  const listerPays = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/television/pays`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des pays')
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerPays:', e)
      return null
    }
  }

  /**
   * Récupérer la liste des catégories disponibles
   */
  const listerCategories = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/television/categories`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des catégories')
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerCategories:', e)
      return null
    }
  }

  /**
   * Créer une nouvelle chaîne TV (authentification requise)
   */
  const creerChaine = async (form: CreerChaineTvForm): Promise<TvChannel | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ChaineTvAPI>>(
        `${apiBase}/api/television/chaines`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...authHeaders(),
          },
          body: form,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la création de la chaîne')
      }

      return mapperChaineApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur creerChaine:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Créer un nouveau programme vedette (authentification requise)
   */
  const creerProgrammeVedette = async (form: CreerProgrammeVedetteForm): Promise<TvProgram | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ProgrammeTeleAPI>>(
        `${apiBase}/api/television/programmes-vedettes`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...authHeaders(),
          },
          body: form,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la création du programme')
      }

      return mapperProgrammeApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur creerProgrammeVedette:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Programme mis en avant sur toute la page (FR-001).
   * `null` quand aucun programme n'est publié — la page affiche alors son
   * message d'état vide, jamais un lecteur en erreur.
   */
  const obtenirVedette = async (): Promise<ProgrammeVedette | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<ProgrammeVedetteAPI>>(
        `${apiBase}/api/television/vedette`,
      )
      if (!reponse.success || !reponse.data) return null
      return {
        ...mapperProgrammeApiVersTv(reponse.data, apiBase),
        estRepli: reponse.data.est_repli,
      }
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      console.error('Erreur obtenirVedette:', e)
      return null
    }
  }

  /**
   * Sections de la page, une par chaîne, paginées et chargées au défilement.
   */
  const listerSections = async (filtres: TeleSectionsFiltres = {}): Promise<{
    sections: TeleSection[]
    total: number
    page: number
    totalPages: number
  } | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.categorie && filtres.categorie !== 'Toutes les catégories') params.set('categorie', filtres.categorie)
      if (filtres.pays && filtres.pays !== 'Tous les territoires') params.set('pays', filtres.pays)
      if (filtres.origine) params.set('origine', filtres.origine)
      if (filtres.theme) params.set('theme', filtres.theme)
      // Seul l'état « activé » se transmet : un `en_direct=false` n'exclurait
      // pas les chaînes en direct côté serveur, il n'a donc rien à faire ici.
      if (filtres.en_direct) params.set('en_direct', 'true')
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))
      if (filtres.contenus_par_section) params.set('contenus_par_section', String(filtres.contenus_par_section))

      const queryString = params.toString()
      const reponse = await $fetch<ApiResponse<TeleSectionsListeAPI>>(
        `${apiBase}/api/television/sections${queryString ? `?${queryString}` : ''}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des sections')
      }

      return {
        sections: reponse.data.sections.map(s => ({
          chaine: mapperChaineApiVersTv(s.chaine, apiBase),
          misEnEvidence: s.mis_en_evidence ? mapperProgrammeApiVersTv(s.mis_en_evidence, apiBase) : null,
          contenus: s.contenus.map(c => mapperProgrammeApiVersTv(c, apiBase)),
          totalContenus: s.total_contenus,
          diffusionEnCours: s.diffusion_en_cours ?? null,
          creneauSuivant: s.creneau_suivant ?? null,
        })),
        total: reponse.data.total,
        page: reponse.data.page,
        totalPages: reponse.data.total_pages,
      }
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      console.error('Erreur listerSections:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Détail d'une chaîne par son slug — requis par les pages SSR. */
  const obtenirChaineParSlug = async (slug: string): Promise<TvChannel | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<ChaineTvAPI>>(
        `${apiBase}/api/television/chaines/slug/${encodeURIComponent(slug)}`,
      )
      if (!reponse.success || !reponse.data) return null
      return mapperChaineApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  /** Détail d'un programme par son slug. */
  const obtenirProgrammeParSlug = async (slug: string): Promise<TvProgram | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<ProgrammeTeleAPI>>(
        `${apiBase}/api/television/programmes/slug/${encodeURIComponent(slug)}`,
      )
      if (!reponse.success || !reponse.data) return null
      return mapperProgrammeApiVersTv(reponse.data, apiBase)
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    obtenirVedette,
    listerSections,
    obtenirChaineParSlug,
    obtenirProgrammeParSlug,
    listerChaines,
    obtenirChaine,
    listerProgrammesVedettes,
    listerContenusChaine,
    obtenirProgrammeVedette,
    listerPays,
    listerCategories,
    creerChaine,
    creerProgrammeVedette,
  }
}

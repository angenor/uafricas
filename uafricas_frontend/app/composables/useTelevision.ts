import type { CreneauAPI } from '~/composables/useMediaProgrammation'

import type { CompteursInteraction } from '~/composables/useMediaSocial'

import type { ContactsSupport } from '~/composables/useContactsSupport'

import type { EmissionAPI, EpisodeAPI } from '~/composables/useMediaEmissions'

import type { CouverturePublique, ThematiquePublique } from '~/composables/useMediaSupport'

import type { MembreEquipeAPI } from '~/composables/useMediaEquipe'

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
  /** « africans » (Africans Télé International) ou « territoire », cf. 09o. */
  origine_publication: string
  /** Coordonnées publiques de l'équipe (09p), absent quand aucune. */
  contacts?: ContactsSupport | null
  /** Thématiques déclarées (US3) : absent quand la chaîne n'en déclare aucune. */
  thematiques?: ThematiquePublique[]
  /** Couverture territoriale déclarée (US4). */
  couverture?: CouverturePublique | null
  /** Équipe éditoriale déclarée (010) : absent quand la chaîne n'en a aucune. */
  equipe?: MembreEquipeAPI[]
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

/**
 * Vedette de la page Télé : l'**épisode** portant `a_la_une_globale` (FR-052),
 * plus l'indication d'un repli faute de vedette désignée.
 */
export interface VedetteTeleAPI {
  episode: EpisodeAPI
  emission?: EmissionAPI | null
  chaine?: ChaineTvAPI | null
  est_repli: boolean
}

/**
 * Une section = une chaîne et ses **programmes** publiés, et non plus une
 * vignette par vidéo. Chaque programme annonce son nombre d'épisodes et un
 * aperçu borné à 12.
 */
export interface TeleSectionAPI {
  chaine: ChaineTvAPI
  emissions: EmissionAPI[]
  total_emissions: number
  /** Grille du moment (US2) : absents quand la chaîne n'en a aucune. */
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
  /** Coordonnées publiques de l'équipe, `null` quand elle n'en publie aucune. */
  contacts: ContactsSupport | null
  /** Thématiques déclarées (US3) : vide tant que l'API ne les greffe pas. */
  thematiques: ThematiquePublique[]
  /** Couverture territoriale déclarée (US4). */
  couverture: CouverturePublique | null
  /** Équipe éditoriale (010). Toujours un tableau, jamais `undefined` : les
   *  gabarits n'ont ainsi qu'une seule forme à tester. */
  equipe: MembreEquipeAPI[]
  /** Compteurs d'interaction, absents tant que l'API ne les greffe pas. */
  interactions: CompteursInteraction | null
}

/**
 * Forme d'un **épisode** telle que la consomment les composants.
 *
 * Le nom `TvProgram` est conservé : à l'écran, l'unité que le visiteur lit et
 * partage reste « le programme qu'il regarde ». Ce qui a changé, c'est qu'il
 * appartient désormais à une série : d'où `emissionId` et `emissionTitre`.
 */
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
  emissionId: string | null
  emissionTitre: string | null
  emissionSlug: string | null
  numeroEpisode: number | null
  dureeMinutes: number | null
  etat: string
  aLaUne: boolean
  aLaUneGlobale: boolean
  themePhare: string | null
  /** Décide du lecteur : fichier natif, intégration tierce, ou aucun. */
  sourceMedia: string
  /** Compteurs d'interaction, absents tant que l'API ne les greffe pas. */
  interactions: CompteursInteraction | null
}

/** Forme d'un **programme conteneur** (émission) prête à l'affichage. */
export interface TvEmission {
  id: string
  slug: string | null
  titre: string
  description: string
  banner: string
  cadence: string
  langue: string
  themePhare: string | null
  nombreEpisodes: number
  dernierEpisodeAt: string | null
  /** Aperçu borné à 12 ; au-delà, la page du programme. Vide sur les
   *  sections de vitrine, qui ne rendent plus d'épisode (010, FR-002). */
  episodes: TvProgram[]
  /** Équipe éditoriale DU PROGRAMME (010), jamais celle de sa chaîne. */
  equipe: MembreEquipeAPI[]
  interactions: CompteursInteraction | null
}

/** Section prête à l'affichage : une chaîne et ses programmes. */
export interface TeleSection {
  chaine: TvChannel
  emissions: TvEmission[]
  totalEmissions: number
  /**
   * « En ce moment » et « À suivre » (US2), résolus par le serveur à l'instant
   * de la requête. `null` quand la chaîne n'a pas de grille active, ou quand le
   * programme diffusé n'a aucun épisode publié (FR-021).
   */
  diffusionEnCours: CreneauAPI | null
  creneauSuivant: CreneauAPI | null
}

/** Vedette de la page : l'épisode mis en avant, avec l'indication d'un repli. */
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
  /** Identifiant d'un thème phare (`shared.categorie`, contexte « media »).
   * Porte sur les PROGRAMMES diffusés, à ne pas confondre avec `thematiques`. */
  theme?: string
  /** Thématiques DÉCLARÉES par la chaîne (US3), répétable, entendu comme un OU. */
  thematiques?: string[]
  /** Territoire couvert (US4) : remonte aussi les chaînes continentales. */
  territoire?: string
  /** `true` restreint aux chaînes en direct ; `false`/absent ne filtre pas. */
  en_direct?: boolean
  page?: number
  par_page?: number
  contenus_par_section?: number
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
    contacts: chaine.contacts ?? null,
    thematiques: chaine.thematiques ?? [],
    couverture: chaine.couverture ?? null,
    equipe: chaine.equipe ?? [],
    interactions: chaine.interactions ?? null,
  }
}

/**
 * Un ÉPISODE vers la forme consommée par les composants.
 *
 * Exporté : `useStationsRadio` s'en sert à l'identique, la seule différence
 * entre les deux familles est le nom de la colonne de média, et l'API l'a déjà
 * aplanie sous `media_url`.
 */
export function mapperEpisodeVersTv(episode: EpisodeAPI, apiBase: string): TvProgram {
  return {
    id: episode.id,
    slug: episode.slug,
    title: episode.titre,
    description: episode.description,
    banner: resoudreUrl(episode.image_couverture_url, apiBase),
    // Un lien externe est laissé intact : seul un fichier local doit être
    // préfixé par la base API pour être atteignable depuis le navigateur.
    videoUrl: episode.media_url ? resoudreUrl(episode.media_url, apiBase, '') : '',
    animator: '',
    producer: '',
    country: '',
    language: '',
    chaineId: episode.support?.id ?? null,
    chaineNom: episode.support?.nom ?? null,
    chaineSlug: episode.support?.slug ?? null,
    emissionId: episode.emission_id,
    emissionTitre: episode.emission?.nom ?? null,
    emissionSlug: episode.emission?.slug ?? null,
    numeroEpisode: episode.numero_episode,
    dureeMinutes: episode.duree_minutes,
    etat: episode.etat,
    aLaUne: episode.a_la_une,
    aLaUneGlobale: episode.a_la_une_globale ?? false,
    themePhare: null,
    sourceMedia: episode.source_media ?? 'aucune',
    interactions: episode.interactions ?? null,
  }
}

/** Un PROGRAMME conteneur vers la forme consommée par les composants. */
export function mapperEmissionVersTv(emission: EmissionAPI, apiBase: string): TvEmission {
  return {
    id: emission.id,
    slug: emission.slug,
    titre: emission.titre,
    description: emission.description,
    banner: resoudreUrl(emission.image_couverture_url, apiBase),
    cadence: emission.cadence,
    langue: emission.langue,
    themePhare: emission.theme_phare?.nom || emission.theme_phare_autre || null,
    nombreEpisodes: emission.nombre_episodes ?? 0,
    dernierEpisodeAt: emission.dernier_episode_at ?? null,
    episodes: (emission.episodes_apercu ?? []).map(e => mapperEpisodeVersTv(e, apiBase)),
    equipe: emission.equipe ?? [],
    interactions: emission.interactions ?? null,
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
   * **Programmes** d'une chaîne détenue : alimente le sélecteur de la grille de
   * programmation, qui désigne désormais une série et non un fichier (FR-014).
   *
   * Route MEMBRE : la grille se construit depuis l'espace du détenteur, et la
   * vue publique n'expose pas les programmes encore sans épisode.
   */
  const listerContenusChaine = async (chaineId: string): Promise<TvEmission[]> => {
    try {
      const { listerEmissionsDetenteur } = useMediaEmissions()
      const emissions = await listerEmissionsDetenteur('chaine_tv', chaineId)
      return emissions.map(e => mapperEmissionVersTv(e, apiBase))
    }
    catch (e: any) {
      console.error('Erreur listerContenusChaine:', e)
      return []
    }
  }

  const listerPays = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<{ nom: string }[]>>(
        `${apiBase}/api/pays`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des pays')
      }

      return reponse.data.map(p => p.nom)
    }
    catch (e: any) {
      console.error('Erreur listerPays:', e)
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
   * Épisode mis en avant sur toute la page (FR-052).
   * `null` quand aucun épisode n'est publié, la page affiche alors son message
   * d'état vide, jamais un lecteur en erreur.
   */
  const obtenirVedette = async (): Promise<ProgrammeVedette | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<VedetteTeleAPI>>(
        `${apiBase}/api/television/vedette`,
      )
      if (!reponse.success || !reponse.data) return null
      return {
        ...mapperEpisodeVersTv(reponse.data.episode, apiBase),
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
      // Répétable : plusieurs thématiques s'entendent comme un OU côté serveur.
      // Liste séparée par des VIRGULES, jamais une clé répétée : l'extracteur
      // `web::Query` du serveur rejette la seconde forme en 400, y compris
      // avec une seule valeur. Les thèmes s'entendent comme un OU (US3).
      if (filtres.thematiques?.length) params.set('thematique', filtres.thematiques.join(','))
      if (filtres.territoire) params.set('territoire', filtres.territoire)
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
          emissions: (s.emissions ?? []).map(e => mapperEmissionVersTv(e, apiBase)),
          totalEmissions: s.total_emissions ?? 0,
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

  /**
   * Détail d'une chaîne par son slug, **avec ses programmes**, requis par les
   * pages SSR. La page déplie ainsi le catalogue à deux niveaux sans second
   * appel.
   */
  const obtenirChaineParSlug = async (
    slug: string,
  ): Promise<{ chaine: TvChannel, emissions: TvEmission[], totalEmissions: number } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{
        chaine: ChaineTvAPI
        emissions: EmissionAPI[]
        total_emissions: number
      }>>(`${apiBase}/api/television/chaines/slug/${encodeURIComponent(slug)}`)
      if (!reponse.success || !reponse.data) return null
      return {
        chaine: mapperChaineApiVersTv(reponse.data.chaine, apiBase),
        emissions: (reponse.data.emissions ?? []).map(e => mapperEmissionVersTv(e, apiBase)),
        totalEmissions: reponse.data.total_emissions ?? 0,
      }
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  /**
   * Détail d'un **épisode** par son slug, plus ses épisodes voisins (US1 §4).
   *
   * L'adresse `/api/television/programmes/slug/{slug}` est conservée : les slugs
   * ayant survécu à 09q, les liens déjà indexés continuent de résoudre (FR-056).
   */
  const obtenirProgrammeParSlug = async (
    slug: string,
  ): Promise<{ episode: TvProgram, voisins: TvProgram[] } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{
        episode: EpisodeAPI
        episodes_voisins: EpisodeAPI[]
      }>>(`${apiBase}/api/television/episodes/slug/${encodeURIComponent(slug)}`)
      if (!reponse.success || !reponse.data) return null
      return {
        episode: mapperEpisodeVersTv(reponse.data.episode, apiBase),
        voisins: (reponse.data.episodes_voisins ?? []).map(e => mapperEpisodeVersTv(e, apiBase)),
      }
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  /** Détail d'un **programme** (émission) par son slug. */
  const obtenirEmissionParSlug = async (slug: string): Promise<EmissionAPI | null> => {
    const { obtenirEmission } = useMediaEmissions()
    return obtenirEmission('chaine_tv', slug)
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    obtenirVedette,
    listerSections,
    obtenirChaineParSlug,
    obtenirProgrammeParSlug,
    obtenirEmissionParSlug,
    listerChaines,
    obtenirChaine,
    listerContenusChaine,
    listerPays,
    creerChaine,
  }
}

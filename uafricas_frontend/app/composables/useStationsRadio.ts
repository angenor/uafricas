import type { CreneauAPI } from '~/composables/useMediaProgrammation'

import type { CompteursInteraction } from '~/composables/useMediaSocial'

import type { ContactsSupport } from '~/composables/useContactsSupport'

// Composable pour les appels API des stations radio

/** Interface correspondant au DTO StationRadioResponse du backend */
export interface StationRadioAPI {
  id: string
  nom: string
  slug: string | null
  description: string | null
  stream_url: string | null
  audio_url: string | null
  image_couverture_url: string | null
  genre: string | null
  genres_liste: string[]
  pays: string | null
  ville: string | null
  type_station: string
  a_la_une: boolean
  /** 'africans' | 'territoire' — départage les deux pages Radio (FR-014). */
  origine_publication: string
  role_partie_prenante: string | null
  role_partie_prenante_autre: string | null
  /** Coordonnées publiques de l'équipe (09p) — absent quand aucune. */
  contacts?: ContactsSupport | null
  created_at: string
  /** Réactions, commentaires et partages agrégés (FR-027). */
  interactions?: CompteursInteraction | null
}

/** Interface correspondant au DTO ProgrammeRadioResponse du backend */
export interface ProgrammeRadioAPI {
  id: string
  nom_emission: string
  slug: string | null
  description: string
  image_couverture_url: string | null
  audio_url: string | null
  info_animateur: string | null
  info_producteur: string | null
  pays: string | null
  est_international: boolean
  langue: string
  categorie_radio: string | null
  station_id: string | null
  station_nom: string | null
  station_slug: string | null
  a_la_une: boolean
  theme_phare_id: string | null
  theme_phare_autre: string | null
  theme_phare_nom: string | null
  source_media: string
  created_at: string
  /** Réactions, commentaires et partages agrégés (FR-027). */
  interactions?: CompteursInteraction | null
}

/** Une section = une station, son émission mise en évidence et ses contenus */
export interface StationSectionAPI {
  station: StationRadioAPI
  direct_disponible: boolean
  mis_en_evidence: ProgrammeRadioAPI | null
  contenus: ProgrammeRadioAPI[]
  total_contenus: number
  /** Grille du moment (US5) — absents quand la station n'en a aucune. */
  diffusion_en_cours?: CreneauAPI | null
  creneau_suivant?: CreneauAPI | null
}

export interface StationSectionsListeAPI {
  sections: StationSectionAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Interface correspondant au DTO StationRadioListeResponse du backend */
export interface StationRadioListeAPI {
  stations: StationRadioAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Interface adaptee au format attendu par les composants frontend */
export interface RadioStation {
  id: string
  name: string
  slug: string | null
  description: string
  /** URL audio à jouer : fichier/lien audio en priorité, sinon flux live */
  streamUrl: string
  /** Audio dédié (fichier uploadé ou lien) — prioritaire sur le flux live */
  audioUrl: string
  cover: string
  genre: string
  genresList: string[]
  location: string
  country: string
  programType: 'Nationales' | 'Local' | 'International'
  aLaUne: boolean
  origine: 'africans' | 'territoire'
  /** Coordonnées publiques de l'équipe, `null` quand elle n'en publie aucune. */
  contacts: ContactsSupport | null
  /** Compteurs d'interaction, absents tant que l'API ne les greffe pas. */
  interactions: CompteursInteraction | null
}

/** Émission radio, telle que la consomment les composants */
export interface ProgrammeRadio {
  id: string
  slug: string | null
  title: string
  description: string
  cover: string
  audioUrl: string
  animator: string
  producer: string
  stationId: string | null
  stationNom: string | null
  stationSlug: string | null
  aLaUne: boolean
  themePhare: string | null
  sourceMedia: string
  /** Compteurs d'interaction, absents tant que l'API ne les greffe pas. */
  interactions: CompteursInteraction | null
}

/** Section prête à l'affichage sur une page Radio */
export interface StationSection {
  station: RadioStation
  /** La station diffuse-t-elle un direct ? Il est alors offert comme un contenu (FR-016). */
  directDisponible: boolean
  misEnEvidence: ProgrammeRadio | null
  contenus: ProgrammeRadio[]
  totalContenus: number
  /**
   * « En ce moment » et « À suivre » (FR-039), résolus par le serveur à
   * l'instant de la requête. `null` quand la station n'a pas de grille active :
   * la section retombe alors sur son contenu mis en évidence (FR-041).
   */
  diffusionEnCours: CreneauAPI | null
  creneauSuivant: CreneauAPI | null
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface StationRadioFiltres {
  recherche?: string
  type_station?: string
  pays?: string
  genre?: string
  page?: number
  par_page?: number
}

/**
 * Filtres des sections d'une page Radio. `origine` est OBLIGATOIRE : elle est
 * fixée par la page et non par l'utilisateur.
 */
export interface StationSectionsFiltres extends StationRadioFiltres {
  origine: 'africans' | 'territoire'
}

/** Formulaire de creation de station */
export interface CreerStationForm {
  nom: string
  description?: string
  stream_url?: string
  audio_url?: string
  image_couverture_url?: string
  genre?: string
  genres_liste?: string[]
  pays?: string
  ville?: string
  type_station?: string
}

// ── Mapping API → Frontend ────────────────────────────────────────────

/** Résout une URL média : absolue telle quelle, relative préfixée par l'API */
function resoudreUrlMedia(url: string | null, apiBase: string): string {
  if (!url) return ''
  if (url.startsWith('http://') || url.startsWith('https://')) return url
  return `${apiBase}${url}`
}

/**
 * Le backend renvoie déjà des libellés d'affichage, mais rien ne garantit qu'ils
 * appartiennent aux trois valeurs attendues : un cast direct laisserait passer
 * n'importe quelle chaîne sous un type qui promet le contraire.
 */
const TYPES_STATION = ['Nationales', 'Local', 'International'] as const

function normaliserTypeStation(valeur: string): RadioStation['programType'] {
  return (TYPES_STATION as readonly string[]).includes(valeur)
    ? (valeur as RadioStation['programType'])
    : 'Nationales'
}

function normaliserOrigine(valeur: string | undefined): RadioStation['origine'] {
  return valeur === 'africans' ? 'africans' : 'territoire'
}

function mapperProgrammeRadioApi(programme: ProgrammeRadioAPI, apiBase: string): ProgrammeRadio {
  return {
    id: programme.id,
    slug: programme.slug,
    title: programme.nom_emission,
    description: programme.description,
    cover: programme.image_couverture_url
      ? resoudreUrlMedia(programme.image_couverture_url, apiBase)
      : '',
    audioUrl: resoudreUrlMedia(programme.audio_url, apiBase),
    animator: programme.info_animateur || '',
    producer: programme.info_producteur || '',
    stationId: programme.station_id,
    stationNom: programme.station_nom,
    stationSlug: programme.station_slug,
    aLaUne: programme.a_la_une,
    themePhare: programme.theme_phare_nom || programme.theme_phare_autre || null,
    sourceMedia: programme.source_media ?? 'aucune',
    interactions: programme.interactions ?? null,
  }
}

function mapperStationApiVersRadio(station: StationRadioAPI, apiBase: string): RadioStation {
  const location = [station.ville, station.pays].filter(Boolean).join(', ')
  const audioUrl = resoudreUrlMedia(station.audio_url, apiBase)
  const streamUrl = resoudreUrlMedia(station.stream_url, apiBase)
  return {
    id: station.id,
    name: station.nom,
    slug: station.slug,
    description: station.description || '',
    // L'audio dédié prime ; à défaut, le flux live
    streamUrl: audioUrl || streamUrl,
    audioUrl,
    cover: station.image_couverture_url
      ? `${apiBase}${station.image_couverture_url}`
      : '',
    genre: station.genre || '',
    genresList: station.genres_liste || [],
    location,
    country: station.pays || '',
    programType: normaliserTypeStation(station.type_station),
    aLaUne: station.a_la_une ?? false,
    origine: normaliserOrigine(station.origine_publication),
    contacts: station.contacts ?? null,
    interactions: station.interactions ?? null,
  }
}

export const useStationsRadio = () => {
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
   * Recuperer la liste des stations radio avec filtres et pagination
   */
  const listerStations = async (filtres: StationRadioFiltres = {}): Promise<{ stations: RadioStation[]; total: number } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.type_station && filtres.type_station !== 'Tous les types') params.set('type_station', filtres.type_station)
      if (filtres.pays && filtres.pays !== 'Tous les territoires') params.set('pays', filtres.pays)
      if (filtres.genre && filtres.genre !== 'Tous les genres') params.set('genre', filtres.genre)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/stations-radio${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<StationRadioListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des stations')
      }

      return {
        stations: reponse.data.stations.map(s => mapperStationApiVersRadio(s, apiBase)),
        total: reponse.data.total,
      }
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerStations:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Recuperer une station par ID
   */
  const obtenirStation = async (id: string): Promise<RadioStation | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<StationRadioAPI>>(
        `${apiBase}/api/stations-radio/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Station non trouvée')
      }

      return mapperStationApiVersRadio(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirStation:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Recuperer la liste des pays disponibles
   */
  const listerPays = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/stations-radio/pays`,
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
   * Recuperer la liste des genres disponibles
   */
  const listerGenres = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/stations-radio/genres`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des genres')
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerGenres:', e)
      return null
    }
  }

  /**
   * Creer une nouvelle station radio (authentification requise)
   */
  const creerStation = async (form: CreerStationForm): Promise<RadioStation | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<StationRadioAPI>>(
        `${apiBase}/api/stations-radio`,
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
        throw new Error(reponse.error || 'Erreur lors de la création de la station')
      }

      return mapperStationApiVersRadio(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerStation:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Sections d'une page Radio. `origine` est imposée par la page appelante et
   * n'est jamais offerte au visiteur comme filtre (FR-014) : c'est elle qui
   * garantit qu'une station ne figure jamais sur les deux pages à la fois.
   */
  const listerSections = async (filtres: StationSectionsFiltres): Promise<{
    sections: StationSection[]
    total: number
    page: number
    totalPages: number
  } | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const params = new URLSearchParams({ origine: filtres.origine })
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.type_station && filtres.type_station !== 'Tous les types') params.set('type_station', filtres.type_station)
      if (filtres.pays && filtres.pays !== 'Tous les territoires') params.set('pays', filtres.pays)
      if (filtres.genre && filtres.genre !== 'Tous les genres') params.set('genre', filtres.genre)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const reponse = await $fetch<ApiResponse<StationSectionsListeAPI>>(
        `${apiBase}/api/stations-radio/sections?${params.toString()}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des stations')
      }

      return {
        sections: reponse.data.sections.map(s => ({
          station: mapperStationApiVersRadio(s.station, apiBase),
          directDisponible: s.direct_disponible,
          misEnEvidence: s.mis_en_evidence ? mapperProgrammeRadioApi(s.mis_en_evidence, apiBase) : null,
          contenus: s.contenus.map(c => mapperProgrammeRadioApi(c, apiBase)),
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

  /** Détail d'une station par son slug — requis par les pages SSR. */
  const obtenirStationParSlug = async (slug: string): Promise<RadioStation | null> => {
    try {
      const reponse = await $fetch<ApiResponse<StationRadioAPI>>(
        `${apiBase}/api/stations-radio/slug/${encodeURIComponent(slug)}`,
      )
      if (!reponse.success || !reponse.data) return null
      return mapperStationApiVersRadio(reponse.data, apiBase)
    }
    catch (e: any) {
      console.error('Erreur obtenirStationParSlug:', e)
      return null
    }
  }

  /** Détail d'une émission radio par son slug — requis par les pages SSR. */
  const obtenirProgrammeRadioParSlug = async (slug: string): Promise<ProgrammeRadio | null> => {
    try {
      const reponse = await $fetch<ApiResponse<ProgrammeRadioAPI>>(
        `${apiBase}/api/programmes-radio/slug/${encodeURIComponent(slug)}`,
      )
      if (!reponse.success || !reponse.data) return null
      return mapperProgrammeRadioApi(reponse.data, apiBase)
    }
    catch (e: any) {
      console.error('Erreur obtenirProgrammeRadioParSlug:', e)
      return null
    }
  }

  /** Émissions d'une station — comble l'absence d'exposition publique (FR-020). */
  const listerContenusStation = async (stationId: string): Promise<ProgrammeRadio[]> => {
    try {
      const reponse = await $fetch<ApiResponse<{ programmes: ProgrammeRadioAPI[] }>>(
        `${apiBase}/api/programmes-radio?station=${stationId}&par_page=50`,
      )
      if (!reponse.success || !reponse.data) return []
      return reponse.data.programmes.map(p => mapperProgrammeRadioApi(p, apiBase))
    }
    catch (e: any) {
      console.error('Erreur listerContenusStation:', e)
      return []
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerSections,
    obtenirStationParSlug,
    obtenirProgrammeRadioParSlug,
    listerContenusStation,
    listerStations,
    obtenirStation,
    listerPays,
    listerGenres,
    creerStation,
  }
}

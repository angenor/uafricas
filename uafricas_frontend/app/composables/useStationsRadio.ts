import type { CreneauAPI } from '~/composables/useMediaProgrammation'

import type { CompteursInteraction } from '~/composables/useMediaSocial'

import type { EmissionAPI, EpisodeAPI } from '~/composables/useMediaEmissions'

import type { CouverturePublique, ThematiquePublique } from '~/composables/useMediaSupport'

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
  /** Thématiques déclarées (US3) — absent quand la station n'en déclare aucune. */
  thematiques?: ThematiquePublique[]
  /** Couverture territoriale déclarée (US4). */
  couverture?: CouverturePublique | null
  created_at: string
  /** Réactions, commentaires et partages agrégés (FR-027). */
  interactions?: CompteursInteraction | null
}

/**
 * Une section = une station et ses **programmes** publiés — et non plus une
 * vignette par émission enregistrée. Chaque programme annonce son nombre
 * d'épisodes et un aperçu borné à 12.
 */
export interface StationSectionAPI {
  station: StationRadioAPI
  direct_disponible: boolean
  emissions: EmissionAPI[]
  total_emissions: number
  /** Grille du moment (US2) — absents quand la station n'en a aucune. */
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
  /** Thématiques déclarées (US3) — vide tant que l'API ne les greffe pas. */
  thematiques: ThematiquePublique[]
  /** Couverture territoriale déclarée (US4). */
  couverture: CouverturePublique | null
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
  emissionId: string | null
  emissionTitre: string | null
  emissionSlug: string | null
  numeroEpisode: number | null
  dureeMinutes: number | null
  etat: string
  aLaUne: boolean
  themePhare: string | null
  sourceMedia: string
  /** Compteurs d'interaction, absents tant que l'API ne les greffe pas. */
  interactions: CompteursInteraction | null
}

/** Forme d'un **programme conteneur** radio prête à l'affichage. */
export interface EmissionRadio {
  id: string
  slug: string | null
  titre: string
  description: string
  cover: string
  cadence: string
  themePhare: string | null
  nombreEpisodes: number
  dernierEpisodeAt: string | null
  /** Aperçu borné à 12 ; au-delà, la page du programme. */
  episodes: ProgrammeRadio[]
  interactions: CompteursInteraction | null
}

/** Section prête à l'affichage sur une page Radio */
export interface StationSection {
  station: RadioStation
  /** La station diffuse-t-elle un direct ? Il est alors offert comme un contenu. */
  directDisponible: boolean
  emissions: EmissionRadio[]
  totalEmissions: number
  /**
   * « En ce moment » et « À suivre » (US2), résolus par le serveur à l'instant
   * de la requête. `null` quand la station n'a pas de grille active, ou quand le
   * programme diffusé n'a aucun épisode publié (FR-021).
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
  /** Thématiques DÉCLARÉES par la station (US3) — envoyées en liste séparée
   * par des virgules, entendues comme un OU. */
  thematiques?: string[]
  /** Territoire couvert (US4) — remonte aussi les stations continentales. */
  territoire?: string
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

/** Un ÉPISODE radio vers la forme consommée par les composants. */
function mapperEpisodeRadio(episode: EpisodeAPI, apiBase: string): ProgrammeRadio {
  return {
    id: episode.id,
    slug: episode.slug,
    title: episode.titre,
    description: episode.description,
    cover: episode.image_couverture_url
      ? resoudreUrlMedia(episode.image_couverture_url, apiBase)
      : '',
    audioUrl: resoudreUrlMedia(episode.media_url, apiBase),
    animator: '',
    producer: '',
    stationId: episode.support?.id ?? null,
    stationNom: episode.support?.nom ?? null,
    stationSlug: episode.support?.slug ?? null,
    emissionId: episode.emission_id,
    emissionTitre: episode.emission?.nom ?? null,
    emissionSlug: episode.emission?.slug ?? null,
    numeroEpisode: episode.numero_episode,
    dureeMinutes: episode.duree_minutes,
    etat: episode.etat,
    aLaUne: episode.a_la_une,
    themePhare: null,
    sourceMedia: episode.source_media ?? 'aucune',
    interactions: episode.interactions ?? null,
  }
}

/** Un PROGRAMME conteneur radio vers la forme consommée par les composants. */
function mapperEmissionRadio(emission: EmissionAPI, apiBase: string): EmissionRadio {
  return {
    id: emission.id,
    slug: emission.slug,
    titre: emission.titre,
    description: emission.description,
    cover: emission.image_couverture_url
      ? resoudreUrlMedia(emission.image_couverture_url, apiBase)
      : '',
    cadence: emission.cadence,
    themePhare: emission.theme_phare?.nom || emission.theme_phare_autre || null,
    nombreEpisodes: emission.nombre_episodes ?? 0,
    dernierEpisodeAt: emission.dernier_episode_at ?? null,
    episodes: (emission.episodes_apercu ?? []).map(e => mapperEpisodeRadio(e, apiBase)),
    interactions: emission.interactions ?? null,
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
    thematiques: station.thematiques ?? [],
    couverture: station.couverture ?? null,
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
      // Liste séparée par des VIRGULES, jamais une clé répétée : l'extracteur
      // `web::Query` du serveur rejette la seconde forme en 400, y compris
      // avec une seule valeur. Les thèmes s'entendent comme un OU (US3).
      if (filtres.thematiques?.length) params.set('thematique', filtres.thematiques.join(','))
      if (filtres.territoire) params.set('territoire', filtres.territoire)
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
          emissions: (s.emissions ?? []).map(e => mapperEmissionRadio(e, apiBase)),
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
   * Détail d'une station par son slug, **avec ses programmes** — requis par les
   * pages SSR. La page déplie ainsi le catalogue à deux niveaux sans second
   * appel.
   */
  const obtenirStationParSlug = async (
    slug: string,
  ): Promise<{ station: RadioStation, emissions: EmissionRadio[], totalEmissions: number } | null> => {
    try {
      const reponse = await $fetch<ApiResponse<{
        station: StationRadioAPI
        emissions: EmissionAPI[]
        total_emissions: number
      }>>(`${apiBase}/api/stations-radio/slug/${encodeURIComponent(slug)}`)
      if (!reponse.success || !reponse.data) return null
      return {
        station: mapperStationApiVersRadio(reponse.data.station, apiBase),
        emissions: (reponse.data.emissions ?? []).map(e => mapperEmissionRadio(e, apiBase)),
        totalEmissions: reponse.data.total_emissions ?? 0,
      }
    }
    catch (e: any) {
      console.error('Erreur obtenirStationParSlug:', e)
      return null
    }
  }

  /**
   * Détail d'un **épisode** radio, plus ses épisodes voisins (US1 §4).
   *
   * Les slugs ayant survécu à 09q, les liens déjà indexés continuent de résoudre
   * (FR-056).
   */
  const obtenirProgrammeRadioParSlug = async (
    slug: string,
  ): Promise<{ episode: ProgrammeRadio, voisins: ProgrammeRadio[] } | null> => {
    try {
      const reponse = await $fetch<ApiResponse<{
        episode: EpisodeAPI
        episodes_voisins: EpisodeAPI[]
      }>>(
        `${apiBase}/api/stations-radio/episodes/slug/${encodeURIComponent(slug)}`,
      )
      if (!reponse.success || !reponse.data) return null
      return {
        episode: mapperEpisodeRadio(reponse.data.episode, apiBase),
        voisins: (reponse.data.episodes_voisins ?? []).map(e => mapperEpisodeRadio(e, apiBase)),
      }
    }
    catch (e: any) {
      console.error('Erreur obtenirProgrammeRadioParSlug:', e)
      return null
    }
  }

  /**
   * **Programmes** d'une station détenue — alimente le sélecteur de la grille de
   * programmation, qui désigne désormais une série et non un fichier (FR-014).
   */
  const listerContenusStation = async (stationId: string): Promise<EmissionRadio[]> => {
    try {
      const { listerEmissionsDetenteur } = useMediaEmissions()
      const emissions = await listerEmissionsDetenteur('station_radio', stationId)
      return emissions.map(e => mapperEmissionRadio(e, apiBase))
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

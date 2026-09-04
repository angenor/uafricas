import type { CompteursInteraction } from '~/composables/useMediaSocial'
import type { MembreEquipeAPI } from '~/composables/useMediaEquipe'

/**
 * Programmes conteneurs et épisodes : feature 009.
 *
 * Vocabulaire : `emission` côté API = « **Programme** » à l'écran ; `episode` =
 * « **Épisode** ». Le décalage est délibéré : le mot « programme » désignait
 * l'unité diffusable avant la migration 09q, et réutiliser l'identifiant aurait
 * fait lire silencieusement des données de sens opposé.
 *
 * Ce composable sert **les deux familles** : `chaine_tv` et `station_radio`. Le
 * préfixe d'API en découle (`/api/television` ou `/api/stations-radio`), les
 * formes de réponse étant identiques.
 */

export type TypeSupportMedia = 'chaine_tv' | 'station_radio'

export interface RefNommee {
  id: string
  nom: string
  slug?: string | null
}

/** DTO `EmissionResponse` du backend. */
export interface EmissionAPI {
  id: string
  titre: string
  slug: string | null
  description: string
  image_couverture_url: string | null
  info_animateur: string | null
  info_producteur: string | null
  langue: string
  /** `quotidienne` | `hebdomadaire` | `ponctuelle` */
  cadence: string
  etat: string
  type_support: TypeSupportMedia
  support_id: string
  support?: RefNommee | null
  categorie_radio?: string | null
  theme_phare_id: string | null
  theme_phare_autre: string | null
  theme_phare?: RefNommee | null
  /** Épisodes **publiés** uniquement. */
  nombre_episodes: number
  dernier_episode_at: string | null
  /** Borné à 12 par les sections ; au-delà, la page du programme. */
  episodes_apercu?: EpisodeAPI[]
  /** Vue détenteur et back-office seulement. */
  episodes_en_attente?: number | null
  episodes_rejetes?: number | null
  nombre_signalements: number
  cree_par: string
  created_at: string
  updated_at: string
  /** Équipe éditoriale DU PROGRAMME (010), distincte de celle de son support,
   *  jamais un repli sur elle. Absente quand le programme n'en déclare aucune. */
  equipe?: MembreEquipeAPI[]
  /** Compteurs du PROGRAMME seul : jamais la somme de ceux de ses épisodes. */
  interactions?: CompteursInteraction | null
}

/** DTO `EpisodeResponse` du backend. */
export interface EpisodeAPI {
  id: string
  emission_id: string
  titre: string
  slug: string | null
  description: string
  image_couverture_url: string | null
  /** Champ uniforme des deux familles : c'est celui que consomment les composants. */
  media_url: string | null
  video_url?: string | null
  audio_url?: string | null
  /** `hebergee` | `externe` | `aucune` : décide du lecteur employé. */
  source_media: string
  numero_episode: number | null
  ordre: number
  duree_minutes: number | null
  a_la_une: boolean
  a_la_une_globale: boolean
  /** `brouillon` | `en_attente` | `publie` | `rejete` | `suspendu` | `supprime` */
  etat: string
  motif_rejet?: string | null
  valide_at: string | null
  type_support: TypeSupportMedia
  emission?: RefNommee | null
  emission_cadence?: string | null
  support?: RefNommee | null
  nombre_signalements: number
  cree_par: string
  auteur_nom_complet?: string | null
  created_at: string
  updated_at: string
  interactions?: CompteursInteraction | null
}

export interface PaginationEpisodes {
  page: number
  taille: number
  total: number
  total_pages: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/**
 * Libellé lisible d'une cadence : l'API ne renvoie que la valeur brute.
 *
 * **Source UNIQUE des libellés de périodicité** (010, FR-041) : le back-office
 * entretenait les siens dans `useAdminMediaEmissions`, ce qui garantissait qu'ils
 * divergeraient tôt ou tard de ceux du public. Les deux surfaces lisent
 * désormais cette table.
 *
 * Les clés stockées sont inchangées ; seuls les libellés le sont.
 */
export const LIBELLES_CADENCE: Record<string, string> = {
  ponctuelle: 'Non périodique',
  quotidienne: 'Journalier',
  hebdomadaire: 'Hebdomadaire',
  mensuelle: 'Mensuel',
}

/**
 * Ordre d'affichage dans les sélecteurs, « non périodique » en tête, parce que
 * c'est le défaut d'un programme neuf (FR-042).
 */
export const CADENCES_ORDONNEES = [
  'ponctuelle',
  'quotidienne',
  'hebdomadaire',
  'mensuelle',
] as const

/** Ce que la cadence engage, dit au moment de la choisir. */
export const AIDES_CADENCE: Record<string, string> = {
  ponctuelle: 'Aucune périodicité déclarée : aucune alerte de cadence.',
  quotidienne: 'Un épisode attendu chaque jour, alerte 6 h avant l’échéance.',
  hebdomadaire: 'Un épisode attendu chaque semaine, alerte 48 h avant l’échéance.',
  mensuelle: 'Un épisode attendu chaque mois, alerte 7 jours avant l’échéance.',
}

/** Libellé d'un état d'épisode, tel que l'auteur doit le lire. */
export const LIBELLES_ETAT_EPISODE: Record<string, string> = {
  brouillon: 'Brouillon',
  en_attente: 'En attente de validation',
  publie: 'Publié',
  rejete: 'Refusé',
  suspendu: 'Suspendu',
  supprime: 'Supprimé',
}

export const useMediaEmissions = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /**
   * En-tête d'authentification, lu DANS LE STORE.
   *
   * Il lisait `localStorage.getItem('accessToken')` — une clé que rien n'écrit :
   * le store garde l'access token en mémoire et ne persiste que
   * `refresh_token` (voir `stores/user.ts`). L'en-tête était donc toujours vide,
   * et tout appel authentifié passant par ici repartait en 401 sans que rien ne
   * le dise : l'écran d'édition des thématiques et de la couverture s'affichait
   * simplement vide.
   */
  const authHeaders = (): Record<string, string> => {
    const token = useUserStore().accessToken
    return token ? { Authorization: `Bearer ${token}` } : {}
  }

  /** Préfixe public d'une famille : les deux espaces exposent les mêmes routes. */
  const prefixePublic = (type: TypeSupportMedia): string =>
    type === 'station_radio' ? '/api/stations-radio' : '/api/television'

  /**
   * Adresse publique d'un épisode.
   *
   * Les pages restent à `/medias/programmes-{tele,radio}/<slug>` : les slugs
   * ayant été conservés par 09q, les adresses déjà indexées continuent de
   * résoudre (FR-056).
   */
  const lienEpisode = (episode: { slug: string | null, type_support?: string }): string => {
    const famille = episode.type_support === 'station_radio' ? 'programmes-radio' : 'programmes-tele'
    return `/medias/${famille}/${episode.slug ?? ''}`
  }

  const lienEmission = (emission: { slug: string | null, type_support?: string }): string => {
    const famille = emission.type_support === 'station_radio' ? 'emissions-radio' : 'emissions-tele'
    return `/medias/${famille}/${emission.slug ?? ''}`
  }

  // ── Lecture publique ────────────────────────────────────────────────

  const obtenirEmission = async (
    type: TypeSupportMedia,
    slug: string,
  ): Promise<EmissionAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<EmissionAPI>>(
        `${apiBase}${prefixePublic(type)}/emissions/slug/${encodeURIComponent(slug)}`,
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  /** Épisodes publiés d'un programme, paginés, 24 par défaut. */
  const listerEpisodes = async (
    type: TypeSupportMedia,
    emissionId: string,
    page = 1,
    taille = 24,
  ): Promise<{ episodes: EpisodeAPI[], pagination: PaginationEpisodes } | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ episodes: EpisodeAPI[], pagination: PaginationEpisodes }>>(
        `${apiBase}${prefixePublic(type)}/emissions/${emissionId}/episodes?page=${page}&taille=${taille}`,
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
    finally {
      chargement.value = false
    }
  }

  const obtenirEpisode = async (
    type: TypeSupportMedia,
    slug: string,
  ): Promise<{ episode: EpisodeAPI, episodes_voisins: EpisodeAPI[] } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ episode: EpisodeAPI, episodes_voisins: EpisodeAPI[] }>>(
        `${apiBase}${prefixePublic(type)}/episodes/slug/${encodeURIComponent(slug)}`,
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  // ── Gestion par les co-détenteurs ───────────────────────────────────

  /** **Toutes** les émissions du support, décomptes par état inclus (FR-042). */
  const listerEmissionsDetenteur = async (
    type: TypeSupportMedia,
    supportId: string,
  ): Promise<EmissionAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ emissions: EmissionAPI[] }>>(
        `${apiBase}/api/medias/${type}/${supportId}/emissions`,
        { headers: authHeaders() },
      )
      return reponse.success && reponse.data ? reponse.data.emissions : []
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return []
    }
    finally {
      chargement.value = false
    }
  }

  const creerEmission = async (
    type: TypeSupportMedia,
    supportId: string,
    corps: Record<string, unknown>,
  ): Promise<{ id: string, slug: string } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string, slug: string }>>(
        `${apiBase}/api/medias/${type}/${supportId}/emissions`,
        { method: 'POST', headers: { 'Content-Type': 'application/json', ...authHeaders() }, body: corps },
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  const modifierEmission = async (id: string, corps: Record<string, unknown>): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/medias/emissions/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: corps,
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  /** Refus `409` tant que le programme compte des épisodes publiés (FR-010). */
  const supprimerEmission = async (id: string): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/medias/emissions/${id}`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  /** Tous les épisodes du programme, quel que soit leur état (FR-042). */
  const listerEpisodesDetenteur = async (emissionId: string): Promise<EpisodeAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ episodes: EpisodeAPI[] }>>(
        `${apiBase}/api/medias/emissions/${emissionId}/episodes`,
        { headers: authHeaders() },
      )
      return reponse.success && reponse.data ? reponse.data.episodes : []
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /** L'épisode naît **toujours** `en_attente` : le serveur en décide seul (FR-040). */
  const creerEpisode = async (
    emissionId: string,
    corps: Record<string, unknown>,
  ): Promise<{ id: string, slug: string, etat: string, ordre: number } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string, slug: string, etat: string, ordre: number }>>(
        `${apiBase}/api/medias/emissions/${emissionId}/episodes`,
        { method: 'POST', headers: { 'Content-Type': 'application/json', ...authHeaders() }, body: corps },
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  const modifierEpisode = async (id: string, corps: Record<string, unknown>): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/medias/episodes/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: corps,
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  const supprimerEpisode = async (id: string): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/medias/episodes/${id}`, { method: 'DELETE', headers: authHeaders() })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  /** Réécriture **atomique** : la liste doit couvrir exactement les épisodes. */
  const reordonnerEpisodes = async (
    emissionId: string,
    ordres: { episode_id: string, ordre: number }[],
  ): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/medias/emissions/${emissionId}/episodes/reordonner`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: { ordres },
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  /** Déplacement vers un autre programme **du même support** (400 sinon). */
  const deplacerEpisode = async (id: string, emissionId: string): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/medias/episodes/${id}/emission`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: { emission_id: emissionId },
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  const mettreALaUne = async (id: string): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/medias/episodes/${id}/a-la-une`, {
        method: 'PATCH',
        headers: authHeaders(),
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    lienEmission,
    lienEpisode,
    obtenirEmission,
    listerEpisodes,
    obtenirEpisode,
    listerEmissionsDetenteur,
    creerEmission,
    modifierEmission,
    supprimerEmission,
    listerEpisodesDetenteur,
    creerEpisode,
    modifierEpisode,
    supprimerEpisode,
    reordonnerEpisodes,
    deplacerEpisode,
    mettreALaUne,
  }
}

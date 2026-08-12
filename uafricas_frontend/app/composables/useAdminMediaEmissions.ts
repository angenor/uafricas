/**
 * Back-office des **programmes** (`emission_*`) et de leurs **épisodes**
 * (feature 009, US1 — FR-044 à FR-047).
 *
 * Un seul composable pour la télé et la radio : les routes admin
 * `/api/admin/medias/emissions` sont communes aux deux familles, le
 * `type_support` n'étant qu'un champ du corps de requête. Deux composables
 * jumeaux auraient dupliqué douze fonctions pour aucune garantie.
 *
 * Il remplace les branches « programmes » de `useAdminRadio` et
 * `useAdminTelevision`, dont les routes `/api/admin/programmes-*` n'existent
 * plus depuis 09q.
 */
import type {
  ApiResponse, PaginatedResponse,
  AdminEmission, AdminEmissionDetail, CreerEmissionForm,
  AdminEpisode, CreerEpisodeForm,
  TypeSupportAdmin, CadenceEmission,
} from '~/types/admin'
import { AIDES_CADENCE, CADENCES_ORDONNEES, LIBELLES_CADENCE } from '~/composables/useMediaEmissions'

/**
 * Périodicités proposées en back-office — **dérivées** de la table publique
 * (010, FR-041).
 *
 * Ce fichier entretenait auparavant ses propres libellés (« Quotidienne »,
 * « Ponctuelle »), distincts de ceux du public. Deux tables séparées ne
 * garantissent pas dans la durée qu'un gestionnaire et un visiteur lisent le
 * même mot pour la même valeur : il n'y en a donc plus qu'une.
 */
export const CADENCES: { valeur: CadenceEmission; libelle: string; aide: string }[] =
  CADENCES_ORDONNEES.map(valeur => ({
    valeur: valeur as CadenceEmission,
    libelle: LIBELLES_CADENCE[valeur] ?? valeur,
    aide: AIDES_CADENCE[valeur] ?? '',
  }))

export const libelleCadence = (cadence?: string | null) =>
  (cadence ? LIBELLES_CADENCE[cadence] : undefined) ?? LIBELLES_CADENCE.ponctuelle!

/**
 * États d'un épisode. `rejete` et son motif rendent le refus corrigeable :
 * sans eux, un épisode refusé serait indiscernable d'un épisode oublié.
 */
export const ETATS_EPISODE: Record<string, { libelle: string; badge: string }> = {
  brouillon: { libelle: 'Brouillon', badge: 'badge-ghost' },
  en_attente: { libelle: 'En attente', badge: 'badge-warning' },
  publie: { libelle: 'Publié', badge: 'badge-success' },
  rejete: { libelle: 'Rejeté', badge: 'badge-error' },
  suspendu: { libelle: 'Suspendu', badge: 'badge-error' },
  supprime: { libelle: 'Supprimé', badge: 'badge-ghost' },
}

export const ETATS_EMISSION: Record<string, { libelle: string; badge: string }> = {
  brouillon: { libelle: 'Brouillon', badge: 'badge-ghost' },
  en_attente: { libelle: 'En attente', badge: 'badge-warning' },
  publie: { libelle: 'Publié', badge: 'badge-success' },
  suspendu: { libelle: 'Suspendu', badge: 'badge-error' },
  supprime: { libelle: 'Supprimé', badge: 'badge-ghost' },
}

export const useAdminMediaEmissions = () => {
  const {
    adminFetch, listerPagine, pagination, sort, loading, error,
    allerPage, changerTri, reinitialiserPagination,
  } = useAdmin()
  const { uploaderMedia, resoudreUrlMedia } = useAdminMediaUpload()

  const emissions = ref<AdminEmission[]>([])
  const emissionDetail = ref<AdminEmissionDetail | null>(null)
  const episodes = ref<AdminEpisode[]>([])

  /** `type` vaut `tele` | `radio` | '' (les deux familles). */
  const filtres = reactive({ recherche: '', type: '', support_id: '', etat: '', cadence: '' })

  // ── Programmes ────────────────────────────────────────────
  const chargerEmissions = async () => {
    const result = await listerPagine<AdminEmission>('/api/admin/medias/emissions', { ...filtres })
    if (result) emissions.value = result.data
  }

  const chargerEmission = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminEmissionDetail>>(`/api/admin/medias/emissions/${id}`)
    if (response.success && response.data) emissionDetail.value = response.data
    return response.data
  }

  const creerEmission = async (form: Partial<CreerEmissionForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/medias/emissions',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifierEmission = async (id: string, form: Partial<CreerEmissionForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/medias/emissions/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  /**
   * Suspendre ou republier un programme. Endpoint dédié plutôt que le PUT :
   * la republication remet `nombre_signalements` à zéro dans la même écriture,
   * sans quoi le seuil de suspension resterait franchi.
   */
  const changerEtatEmission = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/medias/emissions/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    const emission = emissions.value.find(e => e.id === id)
    if (emission) emission.etat = etat
    if (emissionDetail.value?.id === id) emissionDetail.value.etat = etat
    return response.data
  }

  /** Refusé par le serveur (409) si le programme a des épisodes publiés. */
  const supprimerEmission = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/medias/emissions/${id}`, { method: 'DELETE' })
  }

  // ── Épisodes d'un programme ───────────────────────────────
  const chargerEpisodes = async (emissionId: string, etat?: string) => {
    const response = await adminFetch<ApiResponse<{ episodes: AdminEpisode[] }>>(
      `/api/admin/medias/emissions/${emissionId}/episodes`,
      { params: etat ? { etat } : {} },
    )
    episodes.value = response.data?.episodes || []
    return episodes.value
  }

  /** Créé par l'administration, l'épisode naît **publié** : elle est l'autorité. */
  const creerEpisode = async (emissionId: string, form: Partial<CreerEpisodeForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/medias/emissions/${emissionId}/episodes`,
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifierEpisode = async (id: string, form: Partial<CreerEpisodeForm> & { etat?: string }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/medias/episodes/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimerEpisode = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/medias/episodes/${id}`, { method: 'DELETE' })
  }

  /**
   * Réordonnancement atomique : le serveur refuse (400) une liste qui ne
   * couvre pas exactement les épisodes du programme. C'est ce qui empêche un
   * ordre partiel de laisser deux épisodes au même rang, donc la rotation de
   * devenir non déterministe.
   */
  const reordonnerEpisodes = async (emissionId: string, ordres: { episode_id: string; ordre: number }[]) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/medias/emissions/${emissionId}/episodes/reordonner`,
      { method: 'PUT', body: { ordres } },
    )
  }

  /** Un seul « à la une » par programme — la bascule est transactionnelle. */
  const definirALaUne = async (episodeId: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; a_la_une: boolean }>>(
      `/api/admin/medias/episodes/${episodeId}/a-la-une`,
      { method: 'PATCH' },
    )
    return response.data
  }

  /**
   * Vedette de la page Télé : la rétrogradation de l'ancienne et la promotion
   * de la nouvelle tiennent dans une seule transaction côté serveur.
   */
  const definirVedetteGlobale = async (episodeId: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; ancienne_vedette: string | null }>>(
      `/api/admin/medias/episodes/${episodeId}/vedette-globale`,
      { method: 'PATCH' },
    )
    return response.data
  }

  // ── Référentiels ──────────────────────────────────────────
  /** Thèmes phares = catégories `shared.categorie` de contexte `media`. */
  const listerThemesPhares = async (): Promise<{ id: string; nom: string }[]> => {
    const response = await adminFetch<ApiResponse<PaginatedResponse<{ id: string; nom: string }>>>(
      '/api/admin/categories',
      { params: { contexte: 'media', par_page: 200, page: 1, tri_par: 'nom', tri_dir: 'asc' } },
    )
    return response.data ? response.data.data.map(c => ({ id: c.id, nom: c.nom })) : []
  }

  /** Supports de rattachement, brouillons inclus, pour les sélecteurs. */
  const listerSupports = async (typeSupport: TypeSupportAdmin): Promise<{ id: string; nom: string }[]> => {
    const endpoint = typeSupport === 'chaine_tv' ? '/api/admin/chaines-tv' : '/api/admin/stations-radio'
    const response = await adminFetch<ApiResponse<PaginatedResponse<{ id: string; nom: string }>>>(
      endpoint,
      { params: { par_page: 200, page: 1 } },
    )
    return response.data ? response.data.data.map(s => ({ id: s.id, nom: s.nom })) : []
  }

  return {
    emissions, emissionDetail, episodes, filtres,
    pagination, sort, loading, error,
    chargerEmissions, chargerEmission, creerEmission, modifierEmission,
    changerEtatEmission, supprimerEmission,
    chargerEpisodes, creerEpisode, modifierEpisode, supprimerEpisode,
    reordonnerEpisodes, definirALaUne, definirVedetteGlobale,
    listerThemesPhares, listerSupports,
    uploaderMedia, resoudreUrlMedia,
    allerPage, changerTri, reinitialiserPagination,
    CADENCES, libelleCadence, ETATS_EPISODE, ETATS_EMISSION,
  }
}

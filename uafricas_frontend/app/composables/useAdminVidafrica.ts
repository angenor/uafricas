import type { ApiResponse } from '~/types/admin'
import type {
  VideoListe, Video, PisteSousTitre, SegmentSousTitre, TimingMot,
} from '~/mocks/vidafrica'

export const useAdminVidafrica = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const videos = ref<VideoListe[]>([])
  const videoDetail = ref<Video | null>(null)

  const filtres = reactive({
    recherche: '',
    etat: '',
  })

  // Résoudre les URLs relatives vers le backend
  const resoudreUrl = (url: string | null | undefined): string | null => {
    if (!url) return null
    if (url.startsWith('http')) return url
    return `${apiBase}${url}`
  }

  const resoudreUrlsVideo = (video: Video): Video => ({
    ...video,
    fichier_video_url: resoudreUrl(video.fichier_video_url) || video.fichier_video_url,
    vignette_url: resoudreUrl(video.vignette_url),
  })

  const resoudreUrlsListe = (v: VideoListe): VideoListe => ({
    ...v,
    vignette_url: resoudreUrl(v.vignette_url),
  })

  // ═══════════════════════════════════════════════════════════
  // VIDÉOS
  // ═══════════════════════════════════════════════════════════

  const chargerListe = async () => {
    const result = await listerPagine<VideoListe>('/api/admin/vidafrica/videos', { ...filtres })
    if (result) videos.value = result.data.map(resoudreUrlsListe)
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<Video>>(`/api/admin/vidafrica/videos/${id}`)
    if (response.success && response.data) {
      videoDetail.value = resoudreUrlsVideo(response.data)
    }
    return videoDetail.value
  }

  const creer = async (formData: FormData) => {
    if (!userStore.accessToken) throw new Error('Non authentifié')
    const response = await $fetch<ApiResponse<Video>>(`${apiBase}/api/admin/vidafrica/videos`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${userStore.accessToken}` },
      body: formData,
    })
    return response.data ? resoudreUrlsVideo(response.data) : response.data
  }

  const modifier = async (id: string, formData: FormData) => {
    if (!userStore.accessToken) throw new Error('Non authentifié')
    const response = await $fetch<ApiResponse<{ id: string }>>(`${apiBase}/api/admin/vidafrica/videos/${id}`, {
      method: 'PUT',
      headers: { Authorization: `Bearer ${userStore.accessToken}` },
      body: formData,
    })
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/vidafrica/videos/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/vidafrica/videos/${id}`, { method: 'DELETE' })
  }

  // ═══════════════════════════════════════════════════════════
  // PISTES DE SOUS-TITRES
  // ═══════════════════════════════════════════════════════════

  const chargerPistes = async (videoId: string): Promise<PisteSousTitre[]> => {
    const response = await adminFetch<ApiResponse<PisteSousTitre[]>>(
      `/api/admin/vidafrica/videos/${videoId}/pistes`,
    )
    return response.data || []
  }

  const creerPiste = async (videoId: string, langue: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; langue: string }>>(
      `/api/admin/vidafrica/videos/${videoId}/pistes`,
      { method: 'POST', body: { langue } },
    )
    return response.data
  }

  const changerEtatPiste = async (pisteId: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/vidafrica/pistes/${pisteId}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimerPiste = async (pisteId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/vidafrica/pistes/${pisteId}`, { method: 'DELETE' })
  }

  // ═══════════════════════════════════════════════════════════
  // SEGMENTS
  // ═══════════════════════════════════════════════════════════

  const chargerSegments = async (pisteId: string): Promise<SegmentSousTitre[]> => {
    const response = await adminFetch<ApiResponse<SegmentSousTitre[]>>(
      `/api/admin/vidafrica/pistes/${pisteId}/segments`,
    )
    return response.data || []
  }

  const creerSegment = async (pisteId: string, segment: { texte: string; debut_ms: number; fin_ms: number }) => {
    const response = await adminFetch<ApiResponse<{ id: string; position: number }>>(
      `/api/admin/vidafrica/pistes/${pisteId}/segments`,
      { method: 'POST', body: segment },
    )
    return response.data
  }

  const modifierSegment = async (segmentId: string, data: { texte?: string; debut_ms?: number; fin_ms?: number }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/vidafrica/segments/${segmentId}`,
      { method: 'PUT', body: data },
    )
    return response.data
  }

  const supprimerSegment = async (segmentId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/vidafrica/segments/${segmentId}`, { method: 'DELETE' })
  }

  const reordonnerSegments = async (pisteId: string, ordre: string[]) => {
    const response = await adminFetch<ApiResponse<{ piste_id: string }>>(
      `/api/admin/vidafrica/pistes/${pisteId}/segments/reordonner`,
      { method: 'PUT', body: { ordre } },
    )
    return response.data
  }

  // ═══════════════════════════════════════════════════════════
  // TIMINGS MOT
  // ═══════════════════════════════════════════════════════════

  const enregistrerTimingsMot = async (segmentId: string, timings: TimingMot[]) => {
    const response = await adminFetch<ApiResponse<{ segment_id: string; nombre_timings: number }>>(
      `/api/admin/vidafrica/segments/${segmentId}/timings-mot`,
      { method: 'POST', body: { timings } },
    )
    return response.data
  }

  const supprimerTimingsMot = async (segmentId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/vidafrica/segments/${segmentId}/timings-mot`, { method: 'DELETE' })
  }

  return {
    // État
    videos, videoDetail, filtres,
    pagination, sort, loading, error,
    // Vidéos
    chargerListe, chargerDetail, creer, modifier, changerEtat, supprimer,
    // Pistes
    chargerPistes, creerPiste, changerEtatPiste, supprimerPiste,
    // Segments
    chargerSegments, creerSegment, modifierSegment, supprimerSegment, reordonnerSegments,
    // Timings
    enregistrerTimingsMot, supprimerTimingsMot,
    // Pagination
    allerPage, changerTri, reinitialiserPagination,
  }
}

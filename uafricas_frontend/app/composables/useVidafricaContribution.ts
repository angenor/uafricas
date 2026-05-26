import type {
  PisteSousTitre, SegmentSousTitre, TimingMot,
} from '~/mocks/vidafrica'

// ── Interfaces API ───────────────────────────────────────────

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

interface ProposerVideoResultat {
  id: string
  slug: string
  titre: string
  etat: string
}

interface MesPistesResultat {
  langues_prises: string[]
  pistes: PisteSousTitre[]
}

/**
 * Contributions Vidafrica par tout utilisateur connecté :
 *   - proposer une vidéo (passe en validation admin),
 *   - sous-titrer une vidéo (piste en brouillon → validée par un admin).
 * Toutes les écritures sont authentifiées via le JWT du store utilisateur.
 */
export const useVidafricaContribution = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const enTete = (): Record<string, string> => {
    if (!userStore.accessToken) throw new Error('Authentification requise')
    return { Authorization: `Bearer ${userStore.accessToken}` }
  }

  // ── Vidéo ──────────────────────────────────────────────────

  const proposerVideo = async (formData: FormData): Promise<ProposerVideoResultat | null> => {
    const reponse = await $fetch<ApiResponse<ProposerVideoResultat>>(
      `${apiBase}/api/vidafrica/videos`,
      { method: 'POST', headers: enTete(), body: formData },
    )
    return reponse.data
  }

  // ── Pistes ─────────────────────────────────────────────────

  const mesPistes = async (videoId: string): Promise<MesPistesResultat> => {
    const reponse = await $fetch<ApiResponse<MesPistesResultat>>(
      `${apiBase}/api/vidafrica/videos/${videoId}/mes-pistes`,
      { headers: enTete() },
    )
    return reponse.data || { langues_prises: [], pistes: [] }
  }

  const creerPiste = async (videoId: string, langue: string) => {
    const reponse = await $fetch<ApiResponse<{ id: string; langue: string; etat: string }>>(
      `${apiBase}/api/vidafrica/videos/${videoId}/pistes`,
      { method: 'POST', headers: enTete(), body: { langue } },
    )
    return reponse.data
  }

  const supprimerPiste = async (pisteId: string) => {
    await $fetch<ApiResponse<null>>(
      `${apiBase}/api/vidafrica/pistes/${pisteId}`,
      { method: 'DELETE', headers: enTete() },
    )
  }

  // ── Segments ───────────────────────────────────────────────

  const chargerSegments = async (pisteId: string): Promise<SegmentSousTitre[]> => {
    const reponse = await $fetch<ApiResponse<SegmentSousTitre[]>>(
      `${apiBase}/api/vidafrica/pistes/${pisteId}/segments`,
      { headers: enTete() },
    )
    return reponse.data || []
  }

  const creerSegment = async (
    pisteId: string,
    segment: { texte: string; debut_ms: number; fin_ms: number },
  ) => {
    const reponse = await $fetch<ApiResponse<{ id: string; position: number }>>(
      `${apiBase}/api/vidafrica/pistes/${pisteId}/segments`,
      { method: 'POST', headers: enTete(), body: segment },
    )
    return reponse.data
  }

  const modifierSegment = async (
    segmentId: string,
    data: { texte?: string; debut_ms?: number; fin_ms?: number },
  ) => {
    const reponse = await $fetch<ApiResponse<{ id: string }>>(
      `${apiBase}/api/vidafrica/segments/${segmentId}`,
      { method: 'PUT', headers: enTete(), body: data },
    )
    return reponse.data
  }

  const supprimerSegment = async (segmentId: string) => {
    await $fetch<ApiResponse<null>>(
      `${apiBase}/api/vidafrica/segments/${segmentId}`,
      { method: 'DELETE', headers: enTete() },
    )
  }

  // ── Timings mot ────────────────────────────────────────────

  const enregistrerTimingsMot = async (segmentId: string, timings: TimingMot[]) => {
    const reponse = await $fetch<ApiResponse<{ segment_id: string; nombre_timings: number }>>(
      `${apiBase}/api/vidafrica/segments/${segmentId}/timings-mot`,
      { method: 'POST', headers: enTete(), body: { timings } },
    )
    return reponse.data
  }

  const supprimerTimingsMot = async (segmentId: string) => {
    await $fetch<ApiResponse<null>>(
      `${apiBase}/api/vidafrica/segments/${segmentId}/timings-mot`,
      { method: 'DELETE', headers: enTete() },
    )
  }

  return {
    proposerVideo,
    mesPistes, creerPiste, supprimerPiste,
    chargerSegments, creerSegment, modifierSegment, supprimerSegment,
    enregistrerTimingsMot, supprimerTimingsMot,
  }
}

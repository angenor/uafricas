import { LANGUES_LABELS } from '~/mocks/vidafrica'

// ── Interfaces API ───────────────────────────────────────────

interface VideoAfricaAPI {
  id: string
  titre: string
  slug: string
  description: string | null
  fichier_video_url?: string
  vignette_url: string | null
  duree_secondes: number | null
  territoires?: string[]
  auteur_reel?: string | null
  langues_disponibles: string[]
  nombre_likes?: number
  nombre_dislikes?: number
  nombre_partages?: number
  ma_reaction?: 'like' | 'dislike' | null
  created_at: string
}

interface MotAPI {
  position: number
  mot: string
  debut_ms: number
  fin_ms: number
}

interface SegmentAPI {
  position: number
  texte: string
  debut_ms: number
  fin_ms: number
  mots: MotAPI[]
}

interface SousTitresAPI {
  langue: string
  auteur: string | null
  segments: SegmentAPI[]
}

interface LangueDisponibleAPI {
  code: string
  label: string
  nombre_videos: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

interface PaginatedVideoResponse {
  donnees: VideoAfricaAPI[]
  pagination: {
    page: number
    par_page: number
    total: number
    total_pages: number
  }
}

// ── Interfaces Frontend ──────────────────────────────────────

export interface VideoAfrica {
  id: string
  titre: string
  slug: string
  description: string | null
  fichierVideoUrl?: string
  vignetteUrl: string | null
  dureeSecondes: number | null
  territoires: string[]
  auteurReel: string | null
  languesDisponibles: string[]
  nombreLikes: number
  nombreDislikes: number
  nombrePartages: number
  maReaction: 'like' | 'dislike' | null
  createdAt: string
}

// État renvoyé après une réaction (toggle)
export interface ReactionVideoEtat {
  nombreLikes: number
  nombreDislikes: number
  maReaction: 'like' | 'dislike' | null
}

// ── Partage de vidéo (mur /publications) ─────────────────────

export interface PartageVideoApercuAPI {
  id: string
  titre: string
  slug: string
  vignette_url: string | null
  duree_secondes: number | null
}

export interface PartageVideoAuteurAPI {
  id: string
  nom: string
  prenom: string
  photo_url: string | null
}

export interface PartageVideoAPI {
  id: string
  legende: string | null
  created_at: string
  video: PartageVideoApercuAPI
  auteur: PartageVideoAuteurAPI
}

export interface PartageVideoListeAPI {
  partages: PartageVideoAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

export interface SegmentKaraoke {
  position: number
  texte: string
  debut_ms: number
  fin_ms: number
  mots: MotKaraoke[]
}

export interface MotKaraoke {
  position: number
  mot: string
  debut_ms: number
  fin_ms: number
}

export interface SousTitres {
  langue: string
  auteur: string | null
  segments: SegmentKaraoke[]
}

// ── Mappers ──────────────────────────────────────────────────

const mapperVideo = (api: VideoAfricaAPI, apiBase: string): VideoAfrica => ({
  id: api.id,
  titre: api.titre,
  slug: api.slug,
  description: api.description,
  fichierVideoUrl: api.fichier_video_url ? resoudreUrl(api.fichier_video_url, apiBase) : undefined,
  vignetteUrl: api.vignette_url ? resoudreUrl(api.vignette_url, apiBase) : null,
  dureeSecondes: api.duree_secondes,
  territoires: api.territoires ?? [],
  auteurReel: api.auteur_reel ?? null,
  languesDisponibles: api.langues_disponibles,
  nombreLikes: api.nombre_likes ?? 0,
  nombreDislikes: api.nombre_dislikes ?? 0,
  nombrePartages: api.nombre_partages ?? 0,
  maReaction: api.ma_reaction ?? null,
  createdAt: api.created_at,
})

const resoudreUrl = (url: string, apiBase: string): string => {
  if (url.startsWith('http')) return url
  return `${apiBase}${url}`
}

// ── Composable ───────────────────────────────────────────────

export const useVidafrica = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  // En-têtes d'auth (optionnels : la lecture publique fonctionne sans JWT,
  // mais le JWT permet de renseigner `maReaction`).
  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) return { Authorization: `Bearer ${userStore.accessToken}` }
    return {}
  }

  const chargerVideo = async (slug: string): Promise<VideoAfrica | null> => {
    try {
      const reponse = await $fetch<ApiResponse<VideoAfricaAPI>>(
        `${apiBase}/api/vidafrica/videos/${slug}`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) return null
      return mapperVideo(reponse.data, apiBase)
    } catch {
      return null
    }
  }

  // Aimer / ne pas aimer une vidéo (toggle) — JWT requis.
  const reagirVideo = async (
    videoId: string,
    typeReaction: 'like' | 'dislike',
  ): Promise<ReactionVideoEtat | null> => {
    try {
      const reponse = await $fetch<ApiResponse<{ nombre_likes: number; nombre_dislikes: number; ma_reaction: 'like' | 'dislike' | null }>>(
        `${apiBase}/api/vidafrica/videos/${videoId}/reaction`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { type_reaction: typeReaction },
        },
      )
      if (!reponse.success || !reponse.data) return null
      return {
        nombreLikes: reponse.data.nombre_likes,
        nombreDislikes: reponse.data.nombre_dislikes,
        maReaction: reponse.data.ma_reaction,
      }
    } catch {
      return null
    }
  }

  // Partager une vidéo sur le mur communautaire — JWT requis.
  const partagerVideo = async (
    videoId: string,
    legende?: string,
  ): Promise<PartageVideoAPI | null> => {
    const reponse = await $fetch<ApiResponse<PartageVideoAPI>>(
      `${apiBase}/api/vidafrica/videos/${videoId}/partage`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: { legende: legende || undefined },
      },
    )
    return reponse.success ? reponse.data : null
  }

  // Lister les partages de vidéos (public, paginé) — pour le mur /publications.
  const listerPartagesVideos = async (
    page = 1,
    parPage = 20,
  ): Promise<PartageVideoListeAPI | null> => {
    try {
      const reponse = await $fetch<ApiResponse<PartageVideoListeAPI>>(
        `${apiBase}/api/vidafrica/videos/partages?page=${page}&par_page=${parPage}`,
      )
      return reponse.success ? reponse.data : null
    } catch (e) {
      console.error('Erreur listerPartagesVideos:', e)
      return null
    }
  }

  const chargerSousTitres = async (videoId: string, langue: string): Promise<SousTitres | null> => {
    try {
      const reponse = await $fetch<ApiResponse<SousTitresAPI>>(
        `${apiBase}/api/vidafrica/videos/${videoId}/sous-titres/${langue}`,
      )
      if (!reponse.success || !reponse.data) return null
      return {
        langue: reponse.data.langue,
        auteur: reponse.data.auteur ?? null,
        segments: reponse.data.segments.map(s => ({
          position: s.position,
          texte: s.texte,
          debut_ms: s.debut_ms,
          fin_ms: s.fin_ms,
          mots: s.mots,
        })),
      }
    } catch {
      return null
    }
  }

  const chargerLanguesDisponibles = async (): Promise<{ code: string; label: string; nombreVideos: number }[]> => {
    try {
      const reponse = await $fetch<ApiResponse<LangueDisponibleAPI[]>>(
        `${apiBase}/api/vidafrica/langues-sous-titres`,
      )
      if (!reponse.success || !reponse.data) return []
      return reponse.data.map(l => ({
        code: l.code,
        label: l.label || LANGUES_LABELS[l.code] || l.code,
        nombreVideos: l.nombre_videos,
      }))
    } catch {
      return []
    }
  }

  const listerVideos = async (params: {
    page?: number
    par_page?: number
    recherche?: string
    langue?: string
  } = {}): Promise<{ videos: VideoAfrica[]; pagination: { page: number; parPage: number; total: number; totalPages: number } }> => {
    try {
      const searchParams = new URLSearchParams()
      if (params.page) searchParams.set('page', String(params.page))
      if (params.par_page) searchParams.set('par_page', String(params.par_page))
      if (params.recherche) searchParams.set('recherche', params.recherche)
      if (params.langue) searchParams.set('langue', params.langue)

      const qs = searchParams.toString()
      const url = `${apiBase}/api/vidafrica/videos${qs ? `?${qs}` : ''}`

      const reponse = await $fetch<ApiResponse<PaginatedVideoResponse>>(url)
      if (!reponse.success || !reponse.data) {
        return { videos: [], pagination: { page: 1, parPage: 20, total: 0, totalPages: 0 } }
      }

      return {
        videos: reponse.data.donnees.map(v => mapperVideo(v, apiBase)),
        pagination: {
          page: reponse.data.pagination.page,
          parPage: reponse.data.pagination.par_page,
          total: reponse.data.pagination.total,
          totalPages: reponse.data.pagination.total_pages,
        },
      }
    } catch {
      return { videos: [], pagination: { page: 1, parPage: 20, total: 0, totalPages: 0 } }
    }
  }

  return {
    chargerVideo,
    chargerSousTitres,
    chargerLanguesDisponibles,
    listerVideos,
    reagirVideo,
    partagerVideo,
    listerPartagesVideos,
  }
}

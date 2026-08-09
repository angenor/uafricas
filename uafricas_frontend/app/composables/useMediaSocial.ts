// Interactions communautaires sur les médias radio et télé (US3) :
// réactions, commentaires et partages vers le mur /publications.
//
// Les quatre types de média partagent les mêmes endpoints, discriminés par
// (type_media, media_id) — cf. migration 09k et handlers/media_social.rs.
//
// La LECTURE est publique ; le JWT n'est indispensable qu'aux mutations, mais
// il est envoyé dès qu'il existe pour renseigner `maReaction`.

export type TypeMedia =
  | 'chaine_tv'
  | 'station_radio'
  | 'emission_tele'
  | 'emission_radio'
  | 'episode_tele'
  | 'episode_radio'
export type TypeReaction = 'like' | 'dislike'

export interface CompteursInteraction {
  nombre_likes: number
  nombre_dislikes: number
  nombre_commentaires: number
  nombre_partages: number
  ma_reaction: TypeReaction | null
}

export interface ReactionMediaEtat {
  nombreLikes: number
  nombreDislikes: number
  maReaction: TypeReaction | null
}

export interface AuteurApercuAPI {
  id: string
  nom: string
  prenom: string
  photo_url: string | null
}

export interface CommentaireMediaAPI {
  id: string
  contenu: string
  created_at: string
  updated_at: string
  auteur: AuteurApercuAPI
  est_mien: boolean
}

export interface CommentaireListeAPI {
  commentaires: CommentaireMediaAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

export interface PartageMediaAPI {
  id: string
  legende: string | null
  created_at: string
  media: {
    type_media: TypeMedia
    media_id: string
    titre: string
    slug: string | null
    /// URL de la page de détail, calculée côté serveur.
    url: string | null
    image_url: string | null
  }
  auteur: AuteurApercuAPI
}

export interface PartageMediaListeAPI {
  partages: PartageMediaAPI[]
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

/** Réponse du signalement (US7). `suspendu` : le contenu a franchi le seuil. */
export interface SignalementMediaEtat {
  nombre_signalements: number
  suspendu: boolean
  deja_signale: boolean
}

/// Motifs de signalement, calqués sur les contenus interdits énoncés par les
/// règles de contenu (FR-048).
export const MOTIFS_SIGNALEMENT_MEDIA: { value: string, label: string }[] = [
  { value: 'violence', label: 'Incitation à la violence' },
  { value: 'racisme', label: 'Racisme ou xénophobie' },
  { value: 'discrimination', label: 'Discrimination' },
  { value: 'mauvaise_gouvernance', label: 'Apologie de la mauvaise gouvernance' },
  { value: 'corruption', label: 'Apologie de la corruption' },
  { value: 'droits_auteur', label: 'Diffusion sans autorisation (droits d\'auteur)' },
  { value: 'autre', label: 'Autre' },
]

/// Libellés d'affichage des quatre types, employés par les cartes du mur et les
/// modales de partage.
export const LIBELLES_TYPE_MEDIA: Record<TypeMedia, string> = {
  chaine_tv: 'une chaîne de télévision',
  station_radio: 'une station de radio',
  emission_tele: 'un programme de télévision',
  emission_radio: 'un programme de radio',
  episode_tele: 'un épisode de télévision',
  episode_radio: 'un épisode de radio',
}

export const useMediaSocial = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) return { Authorization: `Bearer ${userStore.accessToken}` }
    return {}
  }

  const estConnecte = () => Boolean(userStore.accessToken)

  // ── Réactions ────────────────────────────────────────────────
  // Réémettre la réaction déjà posée la retire : le bouton bascule.
  // `null` la retire explicitement.
  const reagir = async (
    typeMedia: TypeMedia,
    mediaId: string,
    typeReaction: TypeReaction | null,
  ): Promise<ReactionMediaEtat | null> => {
    try {
      const reponse = await $fetch<ApiResponse<CompteursInteraction>>(
        `${apiBase}/api/medias/${typeMedia}/${mediaId}/reaction`,
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
    } catch (e) {
      console.error('Erreur reagir:', e)
      return null
    }
  }

  // ── Commentaires ─────────────────────────────────────────────

  const listerCommentaires = async (
    typeMedia: TypeMedia,
    mediaId: string,
    page = 1,
    parPage = 20,
  ): Promise<CommentaireListeAPI | null> => {
    try {
      const reponse = await $fetch<ApiResponse<CommentaireListeAPI>>(
        `${apiBase}/api/medias/${typeMedia}/${mediaId}/commentaires?page=${page}&par_page=${parPage}`,
        { headers: authHeaders() },
      )
      return reponse.success ? reponse.data : null
    } catch (e) {
      console.error('Erreur listerCommentaires:', e)
      return null
    }
  }

  const commenter = async (
    typeMedia: TypeMedia,
    mediaId: string,
    contenu: string,
  ): Promise<CommentaireMediaAPI | null> => {
    const reponse = await $fetch<ApiResponse<CommentaireMediaAPI>>(
      `${apiBase}/api/medias/${typeMedia}/${mediaId}/commentaires`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: { contenu },
      },
    )
    return reponse.success ? reponse.data : null
  }

  // Soft delete, réservé à l'auteur — le serveur renvoie 403 pour tout autre.
  const supprimerCommentaire = async (commentaireId: string): Promise<boolean> => {
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/medias/commentaires/${commentaireId}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      return reponse.success
    } catch (e) {
      console.error('Erreur supprimerCommentaire:', e)
      return false
    }
  }

  // ── Partages ─────────────────────────────────────────────────

  const partager = async (
    typeMedia: TypeMedia,
    mediaId: string,
    legende?: string,
  ): Promise<PartageMediaAPI | null> => {
    const reponse = await $fetch<ApiResponse<PartageMediaAPI>>(
      `${apiBase}/api/medias/${typeMedia}/${mediaId}/partages`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: { legende: legende || undefined },
      },
    )
    return reponse.success ? reponse.data : null
  }

  // Public et paginé — 8ᵉ source du mur /publications.
  const listerPartages = async (
    page = 1,
    parPage = 20,
  ): Promise<PartageMediaListeAPI | null> => {
    try {
      const reponse = await $fetch<ApiResponse<PartageMediaListeAPI>>(
        `${apiBase}/api/medias/partages?page=${page}&par_page=${parPage}`,
      )
      return reponse.success ? reponse.data : null
    } catch (e) {
      console.error('Erreur listerPartages:', e)
      return null
    }
  }

  // ── Signalements (US7) ───────────────────────────────────────

  /**
   * Signale un contenu contraire aux règles.
   *
   * Idempotent côté serveur : un second signalement du même membre renvoie
   * `deja_signale: true` sans faire croître le compteur. L'exception remonte
   * volontairement au composant appelant, à l'image de `commenter` et
   * `partager`, pour qu'il puisse afficher l'erreur dans sa modale.
   */
  const signaler = async (
    typeMedia: TypeMedia,
    mediaId: string,
    payload: { motif: string, description?: string },
  ): Promise<SignalementMediaEtat | null> => {
    const reponse = await $fetch<ApiResponse<SignalementMediaEtat>>(
      `${apiBase}/api/medias/${typeMedia}/${mediaId}/signalement`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: {
          motif: payload.motif,
          description: payload.description || undefined,
        },
      },
    )
    return reponse.success ? reponse.data : null
  }

  return {
    estConnecte,
    reagir,
    listerCommentaires,
    commenter,
    supprimerCommentaire,
    partager,
    listerPartages,
    signaler,
  }
}

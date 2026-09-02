/**
 * File de modération des **épisodes** versés par les co-détenteurs
 * (feature 009, US1, FR-040 à FR-043).
 *
 * Elle est distincte de `useAdminMediaPropositions` : une proposition est un
 * brouillon JSONB soumis par un contributeur extérieur, un épisode en attente
 * est une **ligne réelle** de `episode_*`, déjà rattachée à son programme et
 * déjà pourvue de son média. Ce choix (research.md R7) donne gratuitement à son
 * auteur le suivi, l'exclusion de la rotation et la resoumission après
 * correction ; le confondre avec une proposition aurait tout redemandé.
 *
 * Le tri par défaut est l'**échéance** : un épisode attendu à l'antenne samedi
 * ne doit pas être traité au même rang qu'un contenu sans date. Elle est
 * calculée à la lecture depuis les créneaux du programme, aucune colonne à
 * maintenir, aucune tâche de fond.
 */
import type { ApiResponse, AdminEpisode } from '~/types/admin'

/** Une ligne de la file : l'épisode, à plat, augmenté de ses délais. */
export interface EpisodeAModerer extends AdminEpisode {
  soumis_at: string
  anciennete_heures: number
  /** `null` si le programme n'est programmé nulle part. */
  prochaine_echeance: string | null
  heures_avant_echeance: number | null
}

/** Longueur minimale du motif, imposée aussi côté serveur (FR-042). */
export const MOTIF_REJET_MIN = 10

export const useAdminMediaModeration = () => {
  const { adminFetch, loading, error } = useAdmin()

  const file = ref<EpisodeAModerer[]>([])
  const total = ref(0)
  const pagination = reactive({ page: 1, taille: 25 })
  /** `type` : '' (les deux) | 'tele' | 'radio'. `tri` : 'echeance' | 'anciennete'. */
  const filtres = reactive({ etat: 'en_attente', type: '', support_id: '', tri: 'echeance' })

  const charger = async () => {
    const response = await adminFetch<ApiResponse<{
      episodes: EpisodeAModerer[]
      pagination: { page: number; taille: number; total: number }
    }>>('/api/admin/medias/episodes', {
      params: { ...filtres, page: pagination.page, taille: pagination.taille },
    })
    file.value = response.data?.episodes || []
    total.value = response.data?.pagination.total || 0
    return file.value
  }

  /**
   * Publie l'épisode et, s'il était encore brouillon, son programme dans la
   * même transaction : sans cela l'épisode validé resterait invisible du public.
   * `409` si l'épisode n'est pas « en attente », revalider un épisode publié
   * le rendrait indistinguable d'une resoumission.
   */
  const valider = async (episodeId: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/medias/episodes/${episodeId}/valider`,
      { method: 'PATCH' },
    )
    file.value = file.value.filter(e => e.id !== episodeId)
    total.value = Math.max(0, total.value - 1)
    return response.data
  }

  /**
   * Le motif part dans la notification à l'auteur : un rejet sans motif serait
   * une impasse, l'auteur n'ayant rien à corriger.
   */
  const rejeter = async (episodeId: string, motif: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/medias/episodes/${episodeId}/rejeter`,
      { method: 'PATCH', body: { motif } },
    )
    file.value = file.value.filter(e => e.id !== episodeId)
    total.value = Math.max(0, total.value - 1)
    return response.data
  }

  /** Restant avant diffusion, en clair. Négatif = échéance déjà passée. */
  const delaiLisible = (heures: number | null): string => {
    if (heures === null) return 'Non programmé'
    if (heures < 0) return `Échéance dépassée de ${Math.abs(heures)} h`
    if (heures < 24) return `Dans ${heures} h`
    return `Dans ${Math.floor(heures / 24)} j`
  }

  /** Urgence visuelle : moins de 48 h avant l'antenne, ou déjà dépassé. */
  const urgence = (heures: number | null): 'depassee' | 'proche' | 'normale' => {
    if (heures === null) return 'normale'
    if (heures < 0) return 'depassee'
    return heures <= 48 ? 'proche' : 'normale'
  }

  return {
    file, total, pagination, filtres, loading, error,
    charger, valider, rejeter,
    delaiLisible, urgence, MOTIF_REJET_MIN,
  }
}

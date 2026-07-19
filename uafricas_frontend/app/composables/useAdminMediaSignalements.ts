// File de modération des contenus signalés (US7, back-office).
//
// Garde côté serveur : `verifier_permission!(admin, "media", …)`, comme la file
// des propositions. Le rétablissement d'un contenu remet son compteur de
// signalements à zéro (FR-051) — sans quoi le seuil resterait franchi et le
// contenu serait resuspendu au premier signalement suivant.

import type { ApiResponse } from '~/types/admin'
import type { TypeMedia } from '~/composables/useMediaSocial'

/** Une ligne de la file : le contenu visé, son état, ses signalements. */
export interface ContenuSignaleAPI {
  id: string
  type_media: TypeMedia
  titre: string
  slug: string | null
  etat: string
  nombre_signalements: number
  image_couverture_url: string | null
  url_detail: string | null
  dernier_signalement: string | null
}

export interface ContenusSignalesListeAPI {
  contenus: ContenuSignaleAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Un signalement individuel, pour l'écran d'arbitrage. */
export interface SignalementDetailAPI {
  id: string
  motif: string | null
  description: string | null
  created_at: string
  auteur: {
    id: string
    nom: string
    prenom: string
    photo_url: string | null
  }
}

/** Les trois destinations d'une décision de modération. */
export type EtatModerationMedia = 'publie' | 'suspendu' | 'supprime'

export const LIBELLES_ETAT_MEDIA: Record<string, string> = {
  publie: 'Publié',
  suspendu: 'Retiré de l\'antenne',
  en_attente: 'En attente',
  supprime: 'Supprimé',
}

export const LIBELLES_MOTIF_SIGNALEMENT: Record<string, string> = {
  violence: 'Incitation à la violence',
  racisme: 'Racisme ou xénophobie',
  discrimination: 'Discrimination',
  mauvaise_gouvernance: 'Apologie de la mauvaise gouvernance',
  corruption: 'Apologie de la corruption',
  droits_auteur: 'Diffusion sans autorisation',
  autre: 'Autre',
}

export interface FiltresSignalementsAdmin {
  type_media?: TypeMedia
  suspendu?: boolean
  page?: number
  par_page?: number
}

export const useAdminMediaSignalements = () => {
  const { adminFetch } = useAdmin()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** File triée côté serveur par nombre de signalements décroissant. */
  const lister = async (
    filtres: FiltresSignalementsAdmin = {},
  ): Promise<ContenusSignalesListeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await adminFetch<ApiResponse<ContenusSignalesListeAPI>>(
        '/api/admin/medias/signalements',
        { params: filtres },
      )
      if (!reponse.success || !reponse.data) {
        erreur.value = reponse.error || 'Erreur lors du chargement des signalements'
        return null
      }
      return reponse.data
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Signalements individuels reçus par un contenu, du plus récent au plus ancien. */
  const detailSignalements = async (
    typeMedia: TypeMedia,
    id: string,
  ): Promise<SignalementDetailAPI[] | null> => {
    erreur.value = null
    try {
      const reponse = await adminFetch<ApiResponse<SignalementDetailAPI[]>>(
        `/api/admin/medias/signalements/${typeMedia}/${id}`,
      )
      if (!reponse.success || !reponse.data) {
        erreur.value = reponse.error || 'Erreur lors du chargement du détail'
        return null
      }
      return reponse.data
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  /**
   * Arbitrage : `publie` rétablit et remet le compteur à zéro, `suspendu`
   * retire de l'antenne, `supprime` retire définitivement (soft delete).
   */
  const changerEtat = async (
    typeMedia: TypeMedia,
    id: string,
    etat: EtatModerationMedia,
  ): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await adminFetch<ApiResponse<unknown>>(
        `/api/admin/medias/${typeMedia}/${id}/etat`,
        { method: 'PATCH', body: { etat } },
      )
      if (!reponse.success) {
        erreur.value = reponse.error || 'Erreur lors du changement d\'état'
        return false
      }
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  return { chargement, erreur, lister, detailSignalements, changerEtat }
}

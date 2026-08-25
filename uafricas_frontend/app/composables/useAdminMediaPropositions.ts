// File de modération des propositions de médias (US4, back-office).
//
// Garde côté serveur : `verifier_permission!(admin, "media", …)`. Sans les
// permissions `media.*` seedées par la migration 09j, seul `super_admin`
// franchit ces routes.

import type { ApiResponse } from '~/types/admin'
import type {
  PropositionMediaAPI,
  PropositionMediaListeAPI,
  StatutProposition,
  TypeObjetPropose,
} from '~/composables/useMediaProposition'

export interface FiltresPropositionsAdmin {
  statut?: StatutProposition
  type_objet?: TypeObjetPropose
  auteur?: string
  page?: number
  par_page?: number
}

export const useAdminMediaPropositions = () => {
  const { adminFetch } = useAdmin()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** File triée par ancienneté côté serveur : on traite dans l'ordre d'arrivée. */
  const lister = async (
    filtres: FiltresPropositionsAdmin = {},
  ): Promise<PropositionMediaListeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await adminFetch<ApiResponse<PropositionMediaListeAPI>>(
        '/api/admin/medias/propositions',
        { params: filtres },
      )
      if (!reponse.success || !reponse.data) {
        erreur.value = reponse.error || 'Erreur lors du chargement des propositions'
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

  const obtenir = async (id: string): Promise<PropositionMediaAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await adminFetch<ApiResponse<PropositionMediaAPI>>(
        `/api/admin/medias/propositions/${id}`,
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

  /**
   * Valide la proposition : le serveur crée l'objet métier, désigne son auteur
   * propriétaire et notifie : le tout dans une seule transaction.
   */
  const valider = async (
    id: string,
    commentaire?: string,
  ): Promise<{ objet_id_cree: string | null } | null> => {
    erreur.value = null
    try {
      const reponse = await adminFetch<ApiResponse<{ objet_id_cree: string | null }>>(
        `/api/admin/medias/propositions/${id}/valider`,
        { method: 'PATCH', body: { commentaire } },
      )
      if (!reponse.success) {
        erreur.value = reponse.error || 'Erreur lors de la validation'
        return null
      }
      return reponse.data
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  /** Le motif est obligatoire et d'au moins 10 caractères (FR-033). */
  const rejeter = async (id: string, commentaire: string): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await adminFetch<ApiResponse<PropositionMediaAPI>>(
        `/api/admin/medias/propositions/${id}/rejeter`,
        { method: 'PATCH', body: { commentaire } },
      )
      if (!reponse.success) {
        erreur.value = reponse.error || 'Erreur lors du refus'
        return false
      }
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
    lister,
    obtenir,
    valider,
    rejeter,
  }
}

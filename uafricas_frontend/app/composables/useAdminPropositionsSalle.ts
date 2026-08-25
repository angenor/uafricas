// Composable admin : Propositions de salles publiques (feature 001-admin-salles-publiques, US2)
import type { ApiResponse } from '~/types/admin'
import type {
  PropositionSalle,
  PropositionListeAPI,
  StatutProposition,
} from '~/composables/useAfrolang'

export interface PropositionAdminFiltres {
  statut?: StatutProposition
  langue_code?: string
  groupe_ethnique_id?: string
  auteur_id?: string
  date_debut?: string
  date_fin?: string
  page?: number
  taille?: number
  tri?: 'created_at_desc' | 'created_at_asc' | 'decide_at_desc'
}

export interface ValiderPropositionResponse {
  proposition: PropositionSalle
  salle_id: string
}

export const useAdminPropositionsSalle = () => {
  const admin = useAdmin()
  const { adminFetch, loading, error } = admin

  const listerPropositions = async (
    filtres: PropositionAdminFiltres = {},
  ): Promise<PropositionListeAPI | null> => {
    try {
      const response = await adminFetch<ApiResponse<PropositionListeAPI>>(
        '/api/admin/afrolang/propositions',
        { params: filtres as Record<string, unknown> },
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur listerPropositions admin:', e)
      return null
    }
  }

  const obtenirProposition = async (
    id: string,
  ): Promise<PropositionSalle | null> => {
    try {
      const response = await adminFetch<ApiResponse<PropositionSalle>>(
        `/api/admin/afrolang/propositions/${id}`,
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur obtenirProposition admin:', e)
      return null
    }
  }

  const validerProposition = async (
    id: string,
    commentaire?: string,
  ): Promise<ValiderPropositionResponse | null> => {
    try {
      const response = await adminFetch<ApiResponse<ValiderPropositionResponse>>(
        `/api/admin/afrolang/propositions/${id}/valider`,
        { method: 'PATCH', body: { commentaire: commentaire ?? null } },
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur validerProposition:', e)
      return null
    }
  }

  const rejeterProposition = async (
    id: string,
    commentaire: string,
  ): Promise<PropositionSalle | null> => {
    try {
      const response = await adminFetch<ApiResponse<PropositionSalle>>(
        `/api/admin/afrolang/propositions/${id}/rejeter`,
        { method: 'PATCH', body: { commentaire } },
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur rejeterProposition:', e)
      return null
    }
  }

  return {
    loading,
    error,
    listerPropositions,
    obtenirProposition,
    validerProposition,
    rejeterProposition,
  }
}

// Composable admin Afrolang — propositions, modérateurs attitrés, liens externes (feature 005)
import type { ApiResponse, PaginatedResponse } from '~/types/admin'
import type {
  EtatProposition,
  ModerateurAttitre,
  PropositionSalleAPI,
  RessourceSalleAPI,
} from '~/composables/useAfrolang'

/** Proposition côté admin : inclut le proposant et les éventuels doublons */
export interface PropositionSalleAdminAPI extends PropositionSalleAPI {
  proposant_nom_complet: string | null
  proposant_email: string | null
  salle_existante_id: string | null
  proposition_doublon_id: string | null
}

export interface ListerPropositionsFiltres {
  etat?: EtatProposition | 'tous'
  q?: string
  pays_id?: string
}

export interface ApprouverPropositionForm {
  groupe_ethnique_id: string
  titre?: string
  image_couverture_url?: string
  langue_code?: string
  alphabet?: string
  dictionnaire_url?: string
}

export interface DesignerModerateurForm {
  utilisateur_id: string
  disponibilite?: string
}

export const useAdminAfrolangSalles = () => {
  const admin = useAdmin()
  const { adminFetch, listerPagine, pagination, sort, loading, error } = admin

  // ── US2 : Propositions de salles ──

  const listerPropositions = async (
    filtres: ListerPropositionsFiltres = {},
  ): Promise<PaginatedResponse<PropositionSalleAdminAPI> | null> => {
    return await listerPagine<PropositionSalleAdminAPI>(
      '/api/admin/afrolang/propositions',
      filtres as Record<string, unknown>,
    )
  }

  const obtenirProposition = async (
    id: string,
  ): Promise<PropositionSalleAdminAPI | null> => {
    try {
      const response = await adminFetch<ApiResponse<PropositionSalleAdminAPI>>(
        `/api/admin/afrolang/propositions/${id}`,
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur obtenirProposition:', e)
      return null
    }
  }

  const approuverProposition = async (
    id: string,
    form: ApprouverPropositionForm,
  ): Promise<{ proposition_id: string; salle_id: string } | null> => {
    try {
      const response = await adminFetch<
        ApiResponse<{ proposition_id: string; salle_id: string }>
      >(`/api/admin/afrolang/propositions/${id}/approuver`, {
        method: 'POST',
        body: form,
      })
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur approuverProposition:', e)
      return null
    }
  }

  const refuserProposition = async (
    id: string,
    motif_refus: string,
  ): Promise<boolean> => {
    try {
      const response = await adminFetch<ApiResponse<unknown>>(
        `/api/admin/afrolang/propositions/${id}/refuser`,
        {
          method: 'POST',
          body: { motif_refus },
        },
      )
      return response.success
    }
    catch (e) {
      console.error('Erreur refuserProposition:', e)
      return false
    }
  }

  // ── US3 : Modérateurs attitrés ──

  const listerModerateursAttitres = async (
    salleId: string,
  ): Promise<ModerateurAttitre[]> => {
    try {
      const response = await adminFetch<ApiResponse<ModerateurAttitre[]>>(
        `/api/admin/afrolang/salles/${salleId}/moderateurs`,
      )
      return response.success && response.data ? response.data : []
    }
    catch (e) {
      console.error('Erreur listerModerateursAttitres:', e)
      return []
    }
  }

  const designerModerateur = async (
    salleId: string,
    form: DesignerModerateurForm,
  ): Promise<{ id: string; actif: boolean } | null> => {
    try {
      const response = await adminFetch<
        ApiResponse<{ id: string; actif: boolean }>
      >(`/api/admin/afrolang/salles/${salleId}/moderateurs`, {
        method: 'POST',
        body: form,
      })
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur designerModerateur:', e)
      return null
    }
  }

  const retirerModerateur = async (
    salleId: string,
    utilisateurId: string,
  ): Promise<boolean> => {
    try {
      const response = await adminFetch<ApiResponse<unknown>>(
        `/api/admin/afrolang/salles/${salleId}/moderateurs/${utilisateurId}`,
        { method: 'DELETE' },
      )
      return response.success
    }
    catch (e) {
      console.error('Erreur retirerModerateur:', e)
      return false
    }
  }

  // ── US6 : Liens externes en attente de validation ──

  const listerLiensEnAttente = async (): Promise<RessourceSalleAPI[]> => {
    try {
      const response = await adminFetch<ApiResponse<RessourceSalleAPI[]>>(
        '/api/admin/afrolang/ressources/en-attente',
      )
      return response.success && response.data ? response.data : []
    }
    catch (e) {
      console.error('Erreur listerLiensEnAttente:', e)
      return []
    }
  }

  const publierLien = async (id: string): Promise<boolean> => {
    try {
      const response = await adminFetch<ApiResponse<unknown>>(
        `/api/admin/afrolang/ressources/${id}/publier`,
        { method: 'POST' },
      )
      return response.success
    }
    catch (e) {
      console.error('Erreur publierLien:', e)
      return false
    }
  }

  const refuserLien = async (id: string, motif: string): Promise<boolean> => {
    try {
      const response = await adminFetch<ApiResponse<unknown>>(
        `/api/admin/afrolang/ressources/${id}/refuser`,
        { method: 'POST', body: { motif_refus: motif } },
      )
      return response.success
    }
    catch (e) {
      console.error('Erreur refuserLien:', e)
      return false
    }
  }

  // ── Archivage salle privée (admin, Phase 9) ──

  const archiverSallePrivee = async (id: string): Promise<boolean> => {
    try {
      const response = await adminFetch<ApiResponse<unknown>>(
        `/api/admin/afrolang/salles-privees/${id}/archiver`,
        { method: 'POST' },
      )
      return response.success
    }
    catch (e) {
      console.error('Erreur archiverSallePrivee:', e)
      return false
    }
  }

  const archiverBatchUtilisateur = async (
    utilisateurId: string,
  ): Promise<{ salles_archivees: number } | null> => {
    try {
      const response = await adminFetch<ApiResponse<{ salles_archivees: number }>>(
        '/api/admin/afrolang/salles-privees/archiver-batch-utilisateur',
        { method: 'POST', body: { utilisateur_id: utilisateurId } },
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur archiverBatchUtilisateur:', e)
      return null
    }
  }

  const desactiverSallePublique = async (
    id: string,
  ): Promise<{ salles_privees_archivees: number } | null> => {
    try {
      const response = await adminFetch<ApiResponse<{ salles_privees_archivees: number }>>(
        `/api/admin/afrolang/salles/${id}/desactiver`,
        { method: 'POST' },
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur desactiverSallePublique:', e)
      return null
    }
  }

  return {
    pagination,
    sort,
    loading,
    error,
    // US2
    listerPropositions,
    obtenirProposition,
    approuverProposition,
    refuserProposition,
    // US3
    listerModerateursAttitres,
    designerModerateur,
    retirerModerateur,
    // US6 — Liens externes en attente
    listerLiensEnAttente,
    publierLien,
    refuserLien,
    // Phase 9 — Archivage & désactivation
    archiverSallePrivee,
    archiverBatchUtilisateur,
    desactiverSallePublique,
  }
}

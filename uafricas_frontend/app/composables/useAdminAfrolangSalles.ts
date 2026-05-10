// Composable admin Afrolang — modérateurs attitrés, liens externes, archivage
import type { ApiResponse } from '~/types/admin'
import type {
  ModerateurAttitre,
  RessourceSalleAPI,
} from '~/composables/useAfrolang'

export interface DesignerModerateurForm {
  utilisateur_id: string
  disponibilite?: string
}

export const useAdminAfrolangSalles = () => {
  const admin = useAdmin()
  const { adminFetch, pagination, sort, loading, error } = admin

  // ── Modérateurs attitrés ──

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

  // ── Pays d'origine (feature 001-afrolang-pays-origine) ──

  const ajouterPaysOrigine = async (
    salleId: string,
    paysId: string,
  ): Promise<boolean> => {
    try {
      const response = await adminFetch<ApiResponse<unknown>>(
        `/api/admin/afrolang/salles/${salleId}/pays`,
        { method: 'POST', body: { pays_id: paysId } },
      )
      return response.success
    }
    catch (e) {
      console.error('Erreur ajouterPaysOrigine:', e)
      return false
    }
  }

  const retirerPaysOrigine = async (
    salleId: string,
    paysId: string,
  ): Promise<boolean> => {
    try {
      const response = await adminFetch<ApiResponse<unknown>>(
        `/api/admin/afrolang/salles/${salleId}/pays/${paysId}`,
        { method: 'DELETE' },
      )
      return response.success
    }
    catch (e) {
      console.error('Erreur retirerPaysOrigine:', e)
      return false
    }
  }

  return {
    pagination,
    sort,
    loading,
    error,
    // Modérateurs
    listerModerateursAttitres,
    designerModerateur,
    retirerModerateur,
    // Liens externes en attente
    listerLiensEnAttente,
    publierLien,
    refuserLien,
    // Archivage & désactivation
    archiverSallePrivee,
    archiverBatchUtilisateur,
    desactiverSallePublique,
    // Pays d'origine
    ajouterPaysOrigine,
    retirerPaysOrigine,
  }
}

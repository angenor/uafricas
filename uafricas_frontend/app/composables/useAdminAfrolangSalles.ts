// Composable admin Afrolang : modérateurs attitrés, liens externes, archivage
import type { ApiResponse } from '~/types/admin'
import type {
  ModerateurAttitre,
  RessourceSalleAPI,
} from '~/composables/useAfrolang'

export interface DesignerModerateurForm {
  utilisateur_id: string
  disponibilite?: string
}

/** Administrateur de salle publique (vue admin complète, historique inclus).
 *  Feature 001-admin-salles-publiques, US3. */
export interface SalleAdministrateurAPI {
  id: string
  salle_id: string
  utilisateur: {
    id: string
    nom: string
    prenom: string
    email: string
    photo_url: string | null
  }
  nomme_par: { id: string; nom: string; prenom: string }
  nomme_at: string
  actif: boolean
  revoque_at: string | null
  revoque_par: { id: string; nom: string; prenom: string } | null
  motif_revocation: string | null
  suspendu_at: string | null
  motif_suspension: string | null
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

  // ── Administrateurs de salle publique (feature 001-admin-salles-publiques, US3) ──

  const listerAdministrateurs = async (
    salleId: string,
  ): Promise<SalleAdministrateurAPI[]> => {
    try {
      const response = await adminFetch<ApiResponse<SalleAdministrateurAPI[]>>(
        `/api/admin/afrolang/salles/${salleId}/administrateurs`,
      )
      return response.success && response.data ? response.data : []
    }
    catch (e) {
      console.error('Erreur listerAdministrateurs:', e)
      return []
    }
  }

  const nommerAdministrateur = async (
    salleId: string,
    utilisateurId: string,
  ): Promise<SalleAdministrateurAPI | null> => {
    try {
      const response = await adminFetch<ApiResponse<SalleAdministrateurAPI>>(
        `/api/admin/afrolang/salles/${salleId}/administrateurs`,
        { method: 'POST', body: { utilisateur_id: utilisateurId } },
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur nommerAdministrateur:', e)
      return null
    }
  }

  const revoquerAdministrateur = async (
    salleId: string,
    utilisateurId: string,
    motif?: string,
  ): Promise<SalleAdministrateurAPI | null> => {
    try {
      const response = await adminFetch<ApiResponse<SalleAdministrateurAPI>>(
        `/api/admin/afrolang/salles/${salleId}/administrateurs/${utilisateurId}`,
        { method: 'DELETE', body: { motif: motif ?? null } },
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur revoquerAdministrateur:', e)
      return null
    }
  }

  // ── Modération admin (feature 001-ressources-fermeture-session, US2/US3) ──

  /** État de désactivation administrative d'une salle. */
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  interface _DesactivationAdminInfoAPI {
    desactivee_at: string
    motif: string | null
  }

  interface EvenementModerationAPI {
    id: string
    salle_id: string
    session_concernee_id: string | null
    type_action: 'fermeture_admin' | 'reactivation_admin'
    admin_id: string
    admin_nom: string | null
    admin_prenom: string | null
    motif: string | null
    created_at: string
  }

  const reactiverSalle = async (
    salleId: string,
    commentaire?: string,
  ): Promise<{ salle_id: string; reactivee_at: string } | null> => {
    try {
      const response = await adminFetch<
        ApiResponse<{ salle_id: string; reactivee_at: string }>
      >(`/api/admin/afrolang/salles/${salleId}/reactiver`, {
        method: 'POST',
        body: { commentaire: commentaire ?? null },
      })
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur reactiverSalle:', e)
      return null
    }
  }

  const listerHistoriqueModeration = async (
    salleId: string,
    page = 1,
    limit = 50,
  ): Promise<{
    data: EvenementModerationAPI[]
    total: number
    page: number
    par_page: number
  } | null> => {
    try {
      const response = await adminFetch<
        ApiResponse<{
          data: EvenementModerationAPI[]
          total: number
          page: number
          par_page: number
        }>
      >(
        `/api/admin/afrolang/salles/${salleId}/historique-moderation?page=${page}&limit=${limit}`,
      )
      return response.success && response.data ? response.data : null
    }
    catch (e) {
      console.error('Erreur listerHistoriqueModeration:', e)
      return null
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
    // Administrateurs de salle (US3)
    listerAdministrateurs,
    nommerAdministrateur,
    revoquerAdministrateur,
    // Modération admin (feature 001-ressources-fermeture-session)
    reactiverSalle,
    listerHistoriqueModeration,
  }
}

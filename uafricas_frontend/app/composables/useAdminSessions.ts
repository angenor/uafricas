import type { ApiResponse } from '~/types/admin'

export interface AdminSession {
  id: string
  titre: string | null
  etat: string | null
  date_debut_prevue: string | null
  demarre_at: string | null
  termine_at: string | null
  duree_secondes: number | null
  nombre_participants_pic: number | null
  created_at: string
  salle_privee_titre: string | null
  salle_titre: string | null
  salle_langue: string | null
  moderateur_nom: string | null
  moderateur_prenom: string | null
}

export interface AdminSessionParticipant {
  id: string
  utilisateur_id: string
  utilisateur_nom: string | null
  utilisateur_prenom: string | null
  role_session: string | null
  rejoint_at: string | null
  quitte_at: string | null
  duree_secondes: number | null
}

export interface AdminSessionDetail {
  id: string
  titre: string | null
  etat: string | null
  salle_privee_id: string
  salle_privee_titre: string | null
  salle_id: string | null
  salle_titre: string | null
  salle_langue: string | null
  moderateur_id: string | null
  moderateur_nom: string | null
  date_debut_prevue: string | null
  demarre_at: string | null
  termine_at: string | null
  duree_secondes: number | null
  max_participants: number | null
  nombre_participants_pic: number | null
  tableau_blanc_actif: boolean | null
  cree_par_nom: string | null
  created_at: string
  updated_at: string
  participants: AdminSessionParticipant[]
}

export interface AdminTableauBlanc {
  id: string
  session_id: string
  donnees: any
  version: number | null
  created_at: string
  updated_at: string
}

export const useAdminSessions = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const sessions = ref<AdminSession[]>([])
  const sessionDetail = ref<AdminSessionDetail | null>(null)
  const tableauBlanc = ref<AdminTableauBlanc | null>(null)

  const filtres = reactive({
    recherche: '',
    etat: '',
    salle_id: '',
    salle_privee_id: '',
    date_debut: '',
    date_fin: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    Object.keys(params).forEach((k) => { if (!params[k]) delete params[k] })
    const result = await listerPagine<AdminSession>('/api/admin/sessions', params)
    if (result) sessions.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminSessionDetail>>(`/api/admin/sessions/${id}`)
    if (response.success && response.data) sessionDetail.value = response.data
    return response.data
  }

  const chargerTableauBlanc = async (sessionId: string) => {
    const response = await adminFetch<ApiResponse<AdminTableauBlanc>>(`/api/admin/sessions/${sessionId}/tableau-blanc`)
    if (response.success && response.data) tableauBlanc.value = response.data
    else tableauBlanc.value = null
    return response.data
  }

  /**
   * Ferme administrativement une session pour abus
   * (feature 001-ressources-fermeture-session — US2).
   */
  const fermerSessionAdmin = async (sessionId: string, motif: string) => {
    const response = await adminFetch<ApiResponse<{ salle_id: string; session_id: string }>>(
      `/api/admin/afrolang/sessions/${sessionId}/fermer-admin`,
      {
        method: 'POST',
        body: { motif },
      },
    )
    return response
  }

  return {
    sessions, sessionDetail, tableauBlanc, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, chargerTableauBlanc,
    fermerSessionAdmin,
    allerPage, changerTri, reinitialiserPagination,
  }
}

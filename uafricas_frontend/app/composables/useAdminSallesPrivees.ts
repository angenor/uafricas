import type { ApiResponse } from '~/types/admin'

export interface AdminSallePrivee {
  id: string
  titre: string
  salle_id: string
  salle_titre: string | null
  salle_langue: string | null
  max_participants: number | null
  actif: boolean
  created_at: string
  createur_nom: string | null
  createur_prenom: string | null
  nombre_sessions: number
}

export interface AdminSallePriveeDetail {
  id: string
  titre: string
  description: string | null
  salle_id: string
  salle_titre: string | null
  salle_langue: string | null
  code_acces: string | null
  image_couverture_url: string | null
  max_participants: number | null
  actif: boolean
  cree_par_nom: string | null
  created_at: string
  updated_at: string
  nombre_sessions: number
}

export const useAdminSallesPrivees = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const sallesPrivees = ref<AdminSallePrivee[]>([])
  const sallePriveeDetail = ref<AdminSallePriveeDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    salle_id: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.salle_id) delete params.salle_id
    const result = await listerPagine<AdminSallePrivee>('/api/admin/salles-privees', params)
    if (result) sallesPrivees.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminSallePriveeDetail>>(`/api/admin/salles-privees/${id}`)
    if (response.success && response.data) sallePriveeDetail.value = response.data
    return response.data
  }

  return {
    sallesPrivees, sallePriveeDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail,
    allerPage, changerTri, reinitialiserPagination,
  }
}

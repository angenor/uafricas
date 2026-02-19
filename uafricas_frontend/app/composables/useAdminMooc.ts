import type {
  ApiResponse,
  AdminMooc, AdminMoocDetail, CreerMoocForm,
  AdminMoocInscription, AdminMoocInscriptionStats,
} from '~/types/admin'

export const useAdminMooc = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const moocs = ref<AdminMooc[]>([])
  const moocDetail = ref<AdminMoocDetail | null>(null)
  const inscriptions = ref<AdminMoocInscription[]>([])
  const inscriptionStats = ref<AdminMoocInscriptionStats | null>(null)

  const filtres = reactive({
    recherche: '',
    etat: '',
    format: '',
    type_formation: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminMooc>('/api/admin/mooc', { ...filtres })
    if (result) moocs.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminMoocDetail>>(`/api/admin/mooc/${id}`)
    if (response.success && response.data) moocDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerMoocForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/mooc', { method: 'POST', body: form })
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerMoocForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/mooc/${id}`, { method: 'PUT', body: form })
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/mooc/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/mooc/${id}`, { method: 'DELETE' })
  }

  const chargerInscriptions = async (moocId: string, filtreStatut?: string) => {
    const params: Record<string, string> = {}
    if (filtreStatut) params.statut = filtreStatut
    const result = await listerPagine<AdminMoocInscription>(`/api/admin/mooc/${moocId}/inscriptions`, params)
    if (result) inscriptions.value = result.data
  }

  const chargerStatsInscriptions = async (moocId: string) => {
    const response = await adminFetch<ApiResponse<AdminMoocInscriptionStats>>(
      `/api/admin/mooc/${moocId}/inscriptions/stats`,
    )
    if (response.success && response.data) inscriptionStats.value = response.data
    return response.data
  }

  return {
    moocs, moocDetail, inscriptions, inscriptionStats, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, changerEtat, supprimer,
    chargerInscriptions, chargerStatsInscriptions,
    allerPage, changerTri, reinitialiserPagination,
  }
}

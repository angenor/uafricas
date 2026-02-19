import type {
  ApiResponse,
  AdminCandidature,
  AdminCandidatureDetail,
} from '~/types/admin'

export const useAdminCandidatures = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const candidatures = ref<AdminCandidature[]>([])
  const candidatureDetail = ref<AdminCandidatureDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    statut: '',
    programme_id: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminCandidature>('/api/admin/candidatures', { ...filtres })
    if (result) candidatures.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminCandidatureDetail>>(`/api/admin/candidatures/${id}`)
    if (response.success && response.data) candidatureDetail.value = response.data
    return response.data
  }

  const changerStatut = async (id: string, statut: string, notes_internes?: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; statut: string }>>(
      `/api/admin/candidatures/${id}/etat`,
      { method: 'PATCH', body: { statut, notes_internes } },
    )
    return response.data
  }

  return {
    candidatures, candidatureDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, changerStatut,
    allerPage, changerTri, reinitialiserPagination,
  }
}

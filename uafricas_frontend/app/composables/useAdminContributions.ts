import type {
  ApiResponse,
  AdminContribution, AdminContributionDetail,
} from '~/types/admin'

export const useAdminContributions = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const contributions = ref<AdminContribution[]>([])
  const contributionDetail = ref<AdminContributionDetail | null>(null)

  const filtres = reactive({
    etat: '',
    fiche_pays_id: '',
    cree_par: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminContribution>('/api/admin/profils-pays/contributions', { ...filtres })
    if (result) contributions.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminContributionDetail>>(`/api/admin/profils-pays/contributions/${id}`)
    if (response.success && response.data) contributionDetail.value = response.data
    return response.data
  }

  const moderer = async (id: string, etat: string, note_moderation?: string) => {
    const body: Record<string, string> = { etat }
    if (note_moderation) body.note_moderation = note_moderation
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/profils-pays/contributions/${id}/etat`,
      { method: 'PATCH', body },
    )
    return response.data
  }

  return {
    contributions, contributionDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, moderer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

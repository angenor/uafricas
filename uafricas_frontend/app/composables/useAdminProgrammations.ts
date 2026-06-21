import type {
  ApiResponse,
  AdminProgrammation,
  AdminProgrammationDetail,
  AdminProgrammationInscription,
  CreerProgrammationForm,
} from '~/types/admin'

export const useAdminProgrammations = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const programmations = ref<AdminProgrammation[]>([])
  const programmationDetail = ref<AdminProgrammationDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    centre_culturel_id: '',
    mode: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.centre_culturel_id) delete params.centre_culturel_id
    if (!params.mode) delete params.mode
    const result = await listerPagine<AdminProgrammation>('/api/admin/programmations', params)
    if (result) programmations.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminProgrammationDetail>>(`/api/admin/programmations/${id}`)
    if (response.success && response.data) programmationDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerProgrammationForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/programmations',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerProgrammationForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/programmations/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/programmations/${id}`, { method: 'DELETE' })
  }

  const listerInscriptions = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminProgrammationInscription[]>>(
      `/api/admin/programmations/${id}/inscriptions`,
    )
    return response.success && response.data ? response.data : []
  }

  return {
    programmations, programmationDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer, listerInscriptions,
    allerPage, changerTri, reinitialiserPagination,
  }
}

import type {
  ApiResponse,
  AdminSpecialite,
  AdminSpecialiteDetail,
  CreerSpecialiteForm,
} from '~/types/admin'

export const useAdminSpecialites = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const specialites = ref<AdminSpecialite[]>([])
  const specialiteDetail = ref<AdminSpecialiteDetail | null>(null)

  const filtres = reactive({
    recherche: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminSpecialite>('/api/admin/specialites', { ...filtres })
    if (result) specialites.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminSpecialiteDetail>>(`/api/admin/specialites/${id}`)
    if (response.success && response.data) specialiteDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerSpecialiteForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/specialites',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerSpecialiteForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/specialites/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/specialites/${id}`, { method: 'DELETE' })
  }

  return {
    specialites, specialiteDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

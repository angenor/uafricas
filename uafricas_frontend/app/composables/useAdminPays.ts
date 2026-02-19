import type {
  ApiResponse,
  AdminPays,
  AdminPaysDetail,
  CreerPaysForm,
} from '~/types/admin'

export const useAdminPays = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const pays = ref<AdminPays[]>([])
  const paysDetail = ref<AdminPaysDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    continent: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminPays>('/api/admin/pays', { ...filtres })
    if (result) pays.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminPaysDetail>>(`/api/admin/pays/${id}`)
    if (response.success && response.data) paysDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerPaysForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/pays',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerPaysForm> & { actif?: boolean }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/pays/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/pays/${id}`, { method: 'DELETE' })
  }

  return {
    pays, paysDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

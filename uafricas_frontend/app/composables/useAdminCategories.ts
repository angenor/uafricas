import type {
  ApiResponse,
  AdminCategorie,
  AdminCategorieDetail,
  CreerCategorieForm,
} from '~/types/admin'

export const useAdminCategories = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const categories = ref<AdminCategorie[]>([])
  const categorieDetail = ref<AdminCategorieDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    contexte: '',
    parent_id: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminCategorie>('/api/admin/categories', { ...filtres })
    if (result) categories.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminCategorieDetail>>(`/api/admin/categories/${id}`)
    if (response.success && response.data) categorieDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerCategorieForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/categories',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerCategorieForm> & { actif?: boolean }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/categories/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/categories/${id}`, { method: 'DELETE' })
  }

  return {
    categories, categorieDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

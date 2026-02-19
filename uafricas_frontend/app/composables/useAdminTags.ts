import type {
  ApiResponse,
  AdminTag,
  AdminTagDetail,
  CreerTagForm,
} from '~/types/admin'

export const useAdminTags = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const tags = ref<AdminTag[]>([])
  const tagDetail = ref<AdminTagDetail | null>(null)

  const filtres = reactive({
    recherche: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminTag>('/api/admin/tags', { ...filtres })
    if (result) tags.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminTagDetail>>(`/api/admin/tags/${id}`)
    if (response.success && response.data) tagDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerTagForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/tags',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerTagForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/tags/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/tags/${id}`, { method: 'DELETE' })
  }

  return {
    tags, tagDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

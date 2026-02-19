import type {
  ApiResponse,
  AdminAfricantive,
  AdminAfricantiveDetail,
  CreerAfricantiveForm,
} from '~/types/admin'

export const useAdminAfricantives = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const africantives = ref<AdminAfricantive[]>([])
  const africantiveDetail = ref<AdminAfricantiveDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    etat: '',
    domaine_id: '',
    pays_id: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminAfricantive>('/api/admin/africantives', { ...filtres })
    if (result) africantives.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminAfricantiveDetail>>(`/api/admin/africantives/${id}`)
    if (response.success && response.data) africantiveDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerAfricantiveForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/africantives',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerAfricantiveForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/africantives/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/africantives/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/africantives/${id}`, { method: 'DELETE' })
  }

  return {
    africantives, africantiveDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, changerEtat, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

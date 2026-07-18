import type {
  ApiResponse,
  AdminDomaine,
  AdminDomaineDetail,
  CreerDomaineForm,
} from '~/types/admin'

export const useAdminDomaines = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const domaines = ref<AdminDomaine[]>([])
  const domaineDetail = ref<AdminDomaineDetail | null>(null)

  const filtres = reactive({
    recherche: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminDomaine>('/api/admin/domaines', { ...filtres })
    if (result) domaines.value = result.data
  }

  // Liste complète non paginée — pour alimenter un sélecteur (audit #20)
  const listerTousDomaines = async (): Promise<AdminDomaine[]> => {
    const result = await listerPagine<AdminDomaine>('/api/admin/domaines', { par_page: 500 })
    return result?.data ?? []
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminDomaineDetail>>(`/api/admin/domaines/${id}`)
    if (response.success && response.data) domaineDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerDomaineForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/domaines',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerDomaineForm> & { actif?: boolean }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/domaines/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/domaines/${id}`, { method: 'DELETE' })
  }

  return {
    domaines, domaineDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, listerTousDomaines, chargerDetail, creer, modifier, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

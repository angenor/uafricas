import type {
  ApiResponse,
  AdminEcolePartenaire,
  AdminEcolePartenaireDetail,
  CreerEcolePartenaireForm,
} from '~/types/admin'

export const useAdminEcolesPartenaires = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const ecoles = ref<AdminEcolePartenaire[]>([])
  const ecoleDetail = ref<AdminEcolePartenaireDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    pays_id: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.pays_id) delete params.pays_id
    const result = await listerPagine<AdminEcolePartenaire>('/api/admin/ecoles-partenaires', params)
    if (result) ecoles.value = result.data
  }

  /** Liste non paginée pour alimenter un sélecteur (toutes les écoles actives). */
  const listerToutes = async (): Promise<AdminEcolePartenaire[]> => {
    const result = await listerPagine<AdminEcolePartenaire>('/api/admin/ecoles-partenaires', { par_page: 200, tri_par: 'nom', tri_dir: 'asc' })
    return result ? result.data : []
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminEcolePartenaireDetail>>(`/api/admin/ecoles-partenaires/${id}`)
    if (response.success && response.data) ecoleDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerEcolePartenaireForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/ecoles-partenaires',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerEcolePartenaireForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/ecoles-partenaires/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/ecoles-partenaires/${id}`, { method: 'DELETE' })
  }

  return {
    ecoles, ecoleDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, listerToutes, chargerDetail, creer, modifier, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

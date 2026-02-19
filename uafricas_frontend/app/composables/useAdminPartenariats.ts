import type {
  ApiResponse,
  AdminPartenariat,
  AdminPartenariatDetail,
  CreerPartenariatForm,
} from '~/types/admin'

export const useAdminPartenariats = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const partenariats = ref<AdminPartenariat[]>([])
  const partenariatDetail = ref<AdminPartenariatDetail | null>(null)

  const filtres = reactive({
    type_partenariat: '',
    organisation_id: '',
    actif: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminPartenariat>('/api/admin/partenariats', {
      ...filtres,
    })
    if (result) {
      partenariats.value = result.data
    }
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminPartenariatDetail>>(
      `/api/admin/partenariats/${id}`,
    )
    if (response.success && response.data) {
      partenariatDetail.value = response.data
    }
    return response.data
  }

  const creer = async (form: Partial<CreerPartenariatForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/partenariats',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerPartenariatForm> & { actif?: boolean }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/partenariats/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/partenariats/${id}`,
      { method: 'DELETE' },
    )
  }

  return {
    partenariats,
    partenariatDetail,
    filtres,
    pagination,
    sort,
    loading,
    error,
    chargerListe,
    chargerDetail,
    creer,
    modifier,
    supprimer,
    allerPage,
    changerTri,
    reinitialiserPagination,
  }
}

import type {
  ApiResponse,
  AdminOrganisation,
  AdminOrganisationDetail,
  CreerOrganisationForm,
} from '~/types/admin'

export const useAdminOrganisations = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const organisations = ref<AdminOrganisation[]>([])
  const organisationDetail = ref<AdminOrganisationDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    type_organisation: '',
    pays: '',
    etat: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminOrganisation>('/api/admin/organisations', {
      ...filtres,
    })
    if (result) {
      organisations.value = result.data
    }
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminOrganisationDetail>>(
      `/api/admin/organisations/${id}`,
    )
    if (response.success && response.data) {
      organisationDetail.value = response.data
    }
    return response.data
  }

  const creer = async (form: Partial<CreerOrganisationForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/organisations',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerOrganisationForm> & { etat?: string }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/organisations/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/organisations/${id}`,
      { method: 'DELETE' },
    )
  }

  return {
    organisations,
    organisationDetail,
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

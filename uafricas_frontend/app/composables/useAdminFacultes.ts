import type {
  ApiResponse,
  AdminFaculte,
  AdminFaculteDetail,
  CreerFaculteForm,
} from '~/types/admin'

export const useAdminFacultes = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const facultes = ref<AdminFaculte[]>([])
  const faculteDetail = ref<AdminFaculteDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    ecole_partenaire_id: '',
    statut: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.ecole_partenaire_id) delete params.ecole_partenaire_id
    if (!params.statut) delete params.statut
    const result = await listerPagine<AdminFaculte>('/api/admin/facultes', params)
    if (result) facultes.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminFaculteDetail>>(`/api/admin/facultes/${id}`)
    if (response.success && response.data) faculteDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerFaculteForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/facultes',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerFaculteForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/facultes/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/facultes/${id}`, { method: 'DELETE' })
  }

  return {
    facultes, faculteDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

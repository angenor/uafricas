import type {
  ApiResponse,
  AdminCentreCulturel,
  AdminCentreCulturelDetail,
  AdminMembreCentre,
  CreerCentreCulturelForm,
} from '~/types/admin'

export const useAdminCentresCulturels = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const centres = ref<AdminCentreCulturel[]>([])
  const centreDetail = ref<AdminCentreCulturelDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    pays_id: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.pays_id) delete params.pays_id
    const result = await listerPagine<AdminCentreCulturel>('/api/admin/centres-culturels', params)
    if (result) centres.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminCentreCulturelDetail>>(`/api/admin/centres-culturels/${id}`)
    if (response.success && response.data) centreDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerCentreCulturelForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/centres-culturels',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerCentreCulturelForm> & { actif?: boolean }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/centres-culturels/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/centres-culturels/${id}`, { method: 'DELETE' })
  }

  // Membres
  const ajouterMembre = async (centreId: string, utilisateurId: string, role: string) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/centres-culturels/${centreId}/membres`,
      { method: 'POST', body: { utilisateur_id: utilisateurId, role } },
    )
    return response.data
  }

  const modifierMembre = async (centreId: string, membreId: string, role: string) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/centres-culturels/${centreId}/membres/${membreId}`,
      { method: 'PUT', body: { role } },
    )
    return response.data
  }

  const retirerMembre = async (centreId: string, membreId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/centres-culturels/${centreId}/membres/${membreId}`,
      { method: 'DELETE' },
    )
  }

  return {
    centres, centreDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    ajouterMembre, modifierMembre, retirerMembre,
    allerPage, changerTri, reinitialiserPagination,
  }
}

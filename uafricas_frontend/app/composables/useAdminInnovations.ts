import type {
  ApiResponse,
  AdminInnovation,
  AdminInnovationDetail,
  CreerInnovationForm,
} from '~/types/admin'

export const useAdminInnovations = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const innovations = ref<AdminInnovation[]>([])
  const innovationDetail = ref<AdminInnovationDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    etat: '',
    domaine_id: '',
    pays_id: '',
    organisation_id: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminInnovation>('/api/admin/innovations', { ...filtres })
    if (result) innovations.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminInnovationDetail>>(`/api/admin/innovations/${id}`)
    if (response.success && response.data) innovationDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerInnovationForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/innovations',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerInnovationForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/innovations/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/innovations/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/innovations/${id}`, { method: 'DELETE' })
  }

  const ajouterMedia = async (id: string, media: { media_url: string; type_mime?: string; ordre?: number }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/innovations/${id}/medias`,
      { method: 'POST', body: media },
    )
    return response.data
  }

  const retirerMedia = async (innovationId: string, mediaId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/innovations/${innovationId}/medias/${mediaId}`,
      { method: 'DELETE' },
    )
  }

  return {
    innovations, innovationDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, changerEtat, supprimer,
    ajouterMedia, retirerMedia,
    allerPage, changerTri, reinitialiserPagination,
  }
}

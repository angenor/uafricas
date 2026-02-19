import type {
  ApiResponse,
  AdminIdeaForce,
  AdminIdeaForceDetail,
  AdminIdeaForceMedia,
  CreerIdeaForceForm,
} from '~/types/admin'

export const useAdminIdeaForces = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const ideaForces = ref<AdminIdeaForce[]>([])
  const ideaForceDetail = ref<AdminIdeaForceDetail | null>(null)
  const medias = ref<AdminIdeaForceMedia[]>([])

  const filtres = reactive({
    recherche: '',
    categorie_proposition: '',
    urgence: '',
    pays_id: '',
    etat: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.categorie_proposition) delete params.categorie_proposition
    if (!params.urgence) delete params.urgence
    if (!params.pays_id) delete params.pays_id
    if (!params.etat) delete params.etat
    const result = await listerPagine<AdminIdeaForce>('/api/admin/idea-forces', params)
    if (result) ideaForces.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminIdeaForceDetail>>(`/api/admin/idea-forces/${id}`)
    if (response.success && response.data) ideaForceDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerIdeaForceForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/idea-forces',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerIdeaForceForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/idea-forces/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/idea-forces/${id}`, { method: 'DELETE' })
  }

  // Medias
  const chargerMedias = async (ideaForceId: string) => {
    const response = await adminFetch<ApiResponse<AdminIdeaForceMedia[]>>(
      `/api/admin/idea-forces/${ideaForceId}/medias`,
    )
    if (response.success && response.data) medias.value = response.data
    return response.data
  }

  const ajouterMedia = async (ideaForceId: string, media: { media_url: string; type_mime?: string; ordre?: number }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/idea-forces/${ideaForceId}/medias`,
      { method: 'POST', body: media },
    )
    return response.data
  }

  const retirerMedia = async (ideaForceId: string, mediaId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/idea-forces/${ideaForceId}/medias/${mediaId}`,
      { method: 'DELETE' },
    )
  }

  return {
    ideaForces, ideaForceDetail, medias, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    chargerMedias, ajouterMedia, retirerMedia,
    allerPage, changerTri, reinitialiserPagination,
  }
}

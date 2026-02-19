import type {
  ApiResponse,
  AdminBadHabit,
  AdminBadHabitDetail,
  AdminBadHabitMedia,
  CreerBadHabitForm,
} from '~/types/admin'

export const useAdminBadHabits = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const badHabits = ref<AdminBadHabit[]>([])
  const badHabitDetail = ref<AdminBadHabitDetail | null>(null)
  const medias = ref<AdminBadHabitMedia[]>([])

  const filtres = reactive({
    recherche: '',
    categorie_probleme: '',
    gravite: '',
    pays_id: '',
    etat: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.categorie_probleme) delete params.categorie_probleme
    if (!params.gravite) delete params.gravite
    if (!params.pays_id) delete params.pays_id
    if (!params.etat) delete params.etat
    const result = await listerPagine<AdminBadHabit>('/api/admin/bad-habits', params)
    if (result) badHabits.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminBadHabitDetail>>(`/api/admin/bad-habits/${id}`)
    if (response.success && response.data) badHabitDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerBadHabitForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/bad-habits',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerBadHabitForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/bad-habits/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/bad-habits/${id}`, { method: 'DELETE' })
  }

  // Medias
  const chargerMedias = async (badHabitId: string) => {
    const response = await adminFetch<ApiResponse<AdminBadHabitMedia[]>>(
      `/api/admin/bad-habits/${badHabitId}/medias`,
    )
    if (response.success && response.data) medias.value = response.data
    return response.data
  }

  const ajouterMedia = async (badHabitId: string, media: { media_url: string; type_mime?: string; ordre?: number }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/bad-habits/${badHabitId}/medias`,
      { method: 'POST', body: media },
    )
    return response.data
  }

  const retirerMedia = async (badHabitId: string, mediaId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/bad-habits/${badHabitId}/medias/${mediaId}`,
      { method: 'DELETE' },
    )
  }

  return {
    badHabits, badHabitDetail, medias, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    chargerMedias, ajouterMedia, retirerMedia,
    allerPage, changerTri, reinitialiserPagination,
  }
}

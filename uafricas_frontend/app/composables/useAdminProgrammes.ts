import type {
  ApiResponse,
  AdminProgramme,
  AdminProgrammeDetail,
  CreerProgrammeForm,
  AdminCandidature,
} from '~/types/admin'

export const useAdminProgrammes = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const programmes = ref<AdminProgramme[]>([])
  const programmeDetail = ref<AdminProgrammeDetail | null>(null)
  const candidaturesProgramme = ref<AdminCandidature[]>([])

  const filtres = reactive({
    recherche: '',
    etat: '',
    domaine_id: '',
    duree: '',
    pays_id: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminProgramme>('/api/admin/programmes', { ...filtres })
    if (result) programmes.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminProgrammeDetail>>(`/api/admin/programmes/${id}`)
    if (response.success && response.data) programmeDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerProgrammeForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/programmes',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerProgrammeForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/programmes/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/programmes/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/programmes/${id}`, { method: 'DELETE' })
  }

  const chargerCandidaturesProgramme = async (programmeId: string, filtreStatut?: string) => {
    const params: Record<string, string> = {}
    if (filtreStatut) params.statut = filtreStatut
    const result = await listerPagine<AdminCandidature>(`/api/admin/programmes/${programmeId}/candidatures`, params)
    if (result) candidaturesProgramme.value = result.data
  }

  return {
    programmes, programmeDetail, candidaturesProgramme, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, changerEtat, supprimer,
    chargerCandidaturesProgramme,
    allerPage, changerTri, reinitialiserPagination,
  }
}

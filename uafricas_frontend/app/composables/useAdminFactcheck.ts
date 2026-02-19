import type {
  ApiResponse,
  AdminFactcheck,
  AdminFactcheckDetail,
  AdminFactcheckCommentaire,
  AdminFactcheckReactions,
  CreerFactcheckForm,
} from '~/types/admin'

export const useAdminFactcheck = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const factchecks = ref<AdminFactcheck[]>([])
  const factcheckDetail = ref<AdminFactcheckDetail | null>(null)
  const commentaires = ref<AdminFactcheckCommentaire[]>([])

  const filtres = reactive({
    recherche: '',
    verdict: '',
    pays_id: '',
    etat: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.verdict) delete params.verdict
    if (!params.pays_id) delete params.pays_id
    if (!params.etat) delete params.etat
    const result = await listerPagine<AdminFactcheck>('/api/admin/factcheck', params)
    if (result) factchecks.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminFactcheckDetail>>(`/api/admin/factcheck/${id}`)
    if (response.success && response.data) factcheckDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerFactcheckForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/factcheck',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerFactcheckForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/factcheck/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/factcheck/${id}`, { method: 'DELETE' })
  }

  // Commentaires
  const chargerCommentaires = async (factcheckId: string) => {
    const response = await adminFetch<ApiResponse<AdminFactcheckCommentaire[]>>(
      `/api/admin/factcheck/${factcheckId}/commentaires`,
    )
    if (response.success && response.data) commentaires.value = response.data
    return response.data
  }

  const supprimerCommentaire = async (factcheckId: string, commentaireId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/factcheck/${factcheckId}/commentaires/${commentaireId}`,
      { method: 'DELETE' },
    )
  }

  // Reactions
  const chargerReactions = async (factcheckId: string) => {
    const response = await adminFetch<ApiResponse<AdminFactcheckReactions>>(
      `/api/admin/factcheck/${factcheckId}/reactions`,
    )
    return response.data
  }

  return {
    factchecks, factcheckDetail, commentaires, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    chargerCommentaires, supprimerCommentaire,
    chargerReactions,
    allerPage, changerTri, reinitialiserPagination,
  }
}

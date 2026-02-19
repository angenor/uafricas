import type {
  ApiResponse,
  AdminCodimoi,
  AdminCodimoiDetail,
  AdminCodimoiCommentaire,
  CreerCodimoiForm,
} from '~/types/admin'

export const useAdminCodimoi = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const posts = ref<AdminCodimoi[]>([])
  const postDetail = ref<AdminCodimoiDetail | null>(null)
  const commentaires = ref<AdminCodimoiCommentaire[]>([])

  const filtres = reactive({
    recherche: '',
    type_codimoi: '',
    pays_id: '',
    groupe_ethnique: '',
    etat: '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.type_codimoi) delete params.type_codimoi
    if (!params.pays_id) delete params.pays_id
    if (!params.groupe_ethnique) delete params.groupe_ethnique
    if (!params.etat) delete params.etat
    const result = await listerPagine<AdminCodimoi>('/api/admin/codimoi', params)
    if (result) posts.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminCodimoiDetail>>(`/api/admin/codimoi/${id}`)
    if (response.success && response.data) postDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerCodimoiForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/codimoi',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerCodimoiForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/codimoi/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/codimoi/${id}`, { method: 'DELETE' })
  }

  // Tags
  const ajouterTag = async (codimoiId: string, tagId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/codimoi/${codimoiId}/tags`,
      { method: 'POST', body: { tag_id: tagId } },
    )
  }

  const retirerTag = async (codimoiId: string, tagId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/codimoi/${codimoiId}/tags/${tagId}`,
      { method: 'DELETE' },
    )
  }

  // Commentaires
  const chargerCommentaires = async (codimoiId: string) => {
    const response = await adminFetch<ApiResponse<AdminCodimoiCommentaire[]>>(
      `/api/admin/codimoi/${codimoiId}/commentaires`,
    )
    if (response.success && response.data) commentaires.value = response.data
    return response.data
  }

  const supprimerCommentaire = async (codimoiId: string, commentaireId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/codimoi/${codimoiId}/commentaires/${commentaireId}`,
      { method: 'DELETE' },
    )
  }

  return {
    posts, postDetail, commentaires, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    ajouterTag, retirerTag,
    chargerCommentaires, supprimerCommentaire,
    allerPage, changerTri, reinitialiserPagination,
  }
}

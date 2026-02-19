import type {
  ApiResponse,
  AdminLivre, AdminLivreDetail, CreerLivreAdminForm,
} from '~/types/admin'

export const useAdminLivres = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const livres = ref<AdminLivre[]>([])
  const livreDetail = ref<AdminLivreDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    type_document: '',
    categorie_id: '',
    acces: '',
    etat: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminLivre>('/api/admin/livres', { ...filtres })
    if (result) livres.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminLivreDetail>>(`/api/admin/livres/${id}`)
    if (response.success && response.data) livreDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerLivreAdminForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/livres', { method: 'POST', body: form })
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerLivreAdminForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/livres/${id}`, { method: 'PUT', body: form })
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/livres/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/livres/${id}`, { method: 'DELETE' })
  }

  const ajouterTag = async (livreId: string, tagId: string) => {
    const response = await adminFetch<ApiResponse<{ livre_id: string; tag_id: string }>>(
      `/api/admin/livres/${livreId}/tags`,
      { method: 'POST', body: { tag_id: tagId } },
    )
    return response.data
  }

  const retirerTag = async (livreId: string, tagId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/livres/${livreId}/tags/${tagId}`, { method: 'DELETE' })
  }

  return {
    livres, livreDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, changerEtat, supprimer,
    ajouterTag, retirerTag,
    allerPage, changerTri, reinitialiserPagination,
  }
}

import type {
  ApiResponse,
  AdminProjet,
  AdminProjetDetail,
  CreerProjetForm,
} from '~/types/admin'

export const useAdminProjets = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const projets = ref<AdminProjet[]>([])
  const projetDetail = ref<AdminProjetDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    etat: '',
    pays_id: '',
    organisation: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminProjet>('/api/admin/projets', { ...filtres })
    if (result) projets.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminProjetDetail>>(`/api/admin/projets/${id}`)
    if (response.success && response.data) projetDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerProjetForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/projets',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerProjetForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/projets/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/projets/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/projets/${id}`, { method: 'DELETE' })
  }

  const ajouterDocument = async (id: string, doc: { nom: string; url: string; type_mime?: string }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/projets/${id}/documents`,
      { method: 'POST', body: doc },
    )
    return response.data
  }

  const retirerDocument = async (projetId: string, docId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/projets/${projetId}/documents/${docId}`,
      { method: 'DELETE' },
    )
  }

  return {
    projets, projetDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, changerEtat, supprimer,
    ajouterDocument, retirerDocument,
    allerPage, changerTri, reinitialiserPagination,
  }
}

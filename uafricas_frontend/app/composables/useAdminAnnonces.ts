import type {
  ApiResponse,
  AdminAnnonce,
  AdminAnnonceDetail,
  CreerAnnonceForm,
} from '~/types/admin'

export const useAdminAnnonces = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const annonces = ref<AdminAnnonce[]>([])
  const annonceDetail = ref<AdminAnnonceDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    etat: '',
    type_operation: '',
    categorie_id: '',
    pays_id: '',
    cree_par: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminAnnonce>('/api/admin/annonces', { ...filtres })
    if (result) annonces.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminAnnonceDetail>>(`/api/admin/annonces/${id}`)
    if (response.success && response.data) annonceDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerAnnonceForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/annonces',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/annonces/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/annonces/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/annonces/${id}`, { method: 'DELETE' })
  }

  // Gestion des pays cibles
  const ajouterPays = async (annonceId: string, paysId: string) => {
    const response = await adminFetch<ApiResponse<{ annonce_id: string; pays_id: string }>>(
      `/api/admin/annonces/${annonceId}/pays`,
      { method: 'POST', body: { pays_id: paysId } },
    )
    return response.data
  }

  const retirerPays = async (annonceId: string, paysId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/annonces/${annonceId}/pays/${paysId}`,
      { method: 'DELETE' },
    )
  }

  // Gestion des medias
  const ajouterMedia = async (annonceId: string, media: { media_url: string; type_mime?: string; est_principale?: boolean; ordre?: number }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/annonces/${annonceId}/medias`,
      { method: 'POST', body: media },
    )
    return response.data
  }

  const retirerMedia = async (annonceId: string, mediaId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/annonces/${annonceId}/medias/${mediaId}`,
      { method: 'DELETE' },
    )
  }

  const reordonnerMedias = async (annonceId: string, ordres: { media_id: string; ordre: number }[]) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/annonces/${annonceId}/medias/ordre`,
      { method: 'PUT', body: { ordres } },
    )
  }

  return {
    annonces, annonceDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, changerEtat, supprimer,
    ajouterPays, retirerPays,
    ajouterMedia, retirerMedia, reordonnerMedias,
    allerPage, changerTri, reinitialiserPagination,
  }
}

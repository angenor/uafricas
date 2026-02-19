import type {
  ApiResponse,
  AdminMedia,
  AdminMediaDetail,
} from '~/types/admin'

export const useAdminMedias = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const medias = ref<AdminMedia[]>([])
  const mediaDetail = ref<AdminMediaDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    type_mime: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminMedia>('/api/admin/medias', { ...filtres })
    if (result) medias.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminMediaDetail>>(`/api/admin/medias/${id}`)
    if (response.success && response.data) mediaDetail.value = response.data
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/medias/${id}`, { method: 'DELETE' })
  }

  return {
    medias, mediaDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

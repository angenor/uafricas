import type {
  ApiResponse,
  AdminFavori,
  AdminFavoriStats,
} from '~/types/admin'

export const useAdminAnnoncesFavoris = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const favoris = ref<AdminFavori[]>([])
  const stats = ref<{ total_favoris: number; top_annonces: AdminFavoriStats[] } | null>(null)

  const filtres = reactive({
    recherche: '',
    annonce_id: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminFavori>('/api/admin/annonces-favoris', { ...filtres })
    if (result) favoris.value = result.data
  }

  const chargerStats = async () => {
    const response = await adminFetch<ApiResponse<{ total_favoris: number; top_annonces: AdminFavoriStats[] }>>(
      '/api/admin/annonces-favoris/stats',
    )
    if (response.success && response.data) stats.value = response.data
    return response.data
  }

  return {
    favoris, stats, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerStats,
    allerPage, changerTri, reinitialiserPagination,
  }
}

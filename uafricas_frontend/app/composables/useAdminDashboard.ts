import type {
  ApiResponse,
  DashboardStats, DashboardActiviteItem, DashboardTendances,
} from '~/types/admin'

export const useAdminDashboard = () => {
  const { adminFetch } = useAdmin()

  const stats = ref<DashboardStats | null>(null)
  const activiteRecente = ref<DashboardActiviteItem[]>([])
  const tendances = ref<DashboardTendances | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const chargerStats = async () => {
    loading.value = true
    error.value = null
    try {
      const response = await adminFetch<ApiResponse<DashboardStats>>('/api/admin/dashboard/stats')
      if (response.success && response.data) stats.value = response.data
    }
    catch (e: any) {
      error.value = e.message || 'Erreur lors du chargement des statistiques'
    }
    finally {
      loading.value = false
    }
  }

  const chargerActiviteRecente = async () => {
    try {
      const response = await adminFetch<ApiResponse<DashboardActiviteItem[]>>('/api/admin/dashboard/activite-recente')
      if (response.success && response.data) activiteRecente.value = response.data
    }
    catch {
      // silently fail — non-critical
    }
  }

  const chargerTendances = async (periode: string = '7j') => {
    try {
      const response = await adminFetch<ApiResponse<DashboardTendances>>(
        '/api/admin/dashboard/tendances',
        { params: { periode } },
      )
      if (response.success && response.data) tendances.value = response.data
    }
    catch {
      // silently fail — non-critical
    }
  }

  return {
    stats, activiteRecente, tendances,
    loading, error,
    chargerStats, chargerActiviteRecente, chargerTendances,
  }
}

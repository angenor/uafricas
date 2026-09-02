// Composable pour l'administration : client API admin de base
import type {
  ApiResponse,
  PaginatedResponse,
  AdminMeResponse,
  PaginationState,
  SortState,
} from '~/types/admin'

export const useAdmin = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()
  const router = useRouter()

  // ── Etat reactif de pagination ───────────────────────────
  const pagination = reactive<PaginationState>({
    page: 1,
    parPage: 12,
    total: 0,
    totalPages: 1,
  })

  const sort = reactive<SortState>({
    column: 'created_at',
    direction: 'desc',
  })

  const loading = ref(false)
  const error = ref<string | null>(null)

  // ── Fetch avec authentification admin ────────────────────
  type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'

  const adminFetch = async <T>(
    url: string,
    options: { method?: HttpMethod; body?: any; params?: Record<string, any>; headers?: Record<string, string> } = {},
  ): Promise<T> => {
    if (!userStore.accessToken) {
      throw new Error('Non authentifie')
    }

    // Construire l'URL avec les query params
    let fullUrl = url.startsWith('http') ? url : `${apiBase}${url}`
    if (options.params) {
      const searchParams = new URLSearchParams()
      for (const [key, value] of Object.entries(options.params)) {
        if (value !== undefined && value !== null && value !== '') {
          searchParams.set(key, String(value))
        }
      }
      const queryString = searchParams.toString()
      if (queryString) {
        fullUrl += `?${queryString}`
      }
    }

    const response = await $fetch<T>(fullUrl, {
      method: options.method,
      body: options.body,
      headers: {
        Authorization: `Bearer ${userStore.accessToken}`,
        'Content-Type': 'application/json',
        ...(options.headers || {}),
      },
      onResponseError({ response }) {
        // Gestion centralisee des erreurs 401/403
        if (response.status === 401) {
          userStore.clearUser()
          // Conserver la page admin courante pour y revenir apres reconnexion
          const cible = router.currentRoute.value.fullPath
          navigateTo({ path: '/login', query: { redirect: cible } })
        }
        else if (response.status === 403) {
          error.value = 'Acces interdit : permissions insuffisantes'
        }
      },
    })

    return response
  }

  // ── Verifier la session admin ────────────────────────────
  const verifierSession = async (): Promise<AdminMeResponse | null> => {
    try {
      const response = await adminFetch<ApiResponse<AdminMeResponse>>(
        '/api/admin/me',
      )
      return response.data
    }
    catch {
      return null
    }
  }

  // ── Listing pagine generique ─────────────────────────────
  const listerPagine = async <T>(
    endpoint: string,
    filtres: Record<string, any> = {},
  ): Promise<PaginatedResponse<T> | null> => {
    loading.value = true
    error.value = null

    try {
      const params: Record<string, any> = {
        page: pagination.page,
        par_page: pagination.parPage,
        tri_par: sort.column,
        tri_dir: sort.direction,
        ...filtres,
      }

      const response = await adminFetch<ApiResponse<PaginatedResponse<T>>>(
        endpoint,
        { params },
      )

      if (response.success && response.data) {
        pagination.total = response.data.total
        pagination.totalPages = response.data.total_pages
        pagination.page = response.data.page
        return response.data
      }

      error.value = response.error || 'Erreur lors du chargement'
      return null
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur reseau'
      return null
    }
    finally {
      loading.value = false
    }
  }

  // ── Helpers de navigation pagination ─────────────────────
  const allerPage = (page: number) => {
    pagination.page = Math.max(1, Math.min(page, pagination.totalPages))
  }

  const pageSuivante = () => {
    if (pagination.page < pagination.totalPages) {
      pagination.page++
    }
  }

  const pagePrecedente = () => {
    if (pagination.page > 1) {
      pagination.page--
    }
  }

  const changerTri = (colonne: string) => {
    if (sort.column === colonne) {
      sort.direction = sort.direction === 'asc' ? 'desc' : 'asc'
    }
    else {
      sort.column = colonne
      sort.direction = 'desc'
    }
    pagination.page = 1
  }

  const reinitialiserPagination = () => {
    pagination.page = 1
    pagination.total = 0
    pagination.totalPages = 1
  }

  return {
    // Etat
    pagination,
    sort,
    loading: readonly(loading),
    error: readonly(error),

    // Client API
    adminFetch,
    verifierSession,
    listerPagine,

    // Navigation pagination
    allerPage,
    pageSuivante,
    pagePrecedente,
    changerTri,
    reinitialiserPagination,
  }
}

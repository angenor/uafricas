import type { ApiResponse } from '~/types/admin'

export interface AdminSalle {
  id: string
  titre: string
  slug: string
  langue_cible: string | null
  actif: boolean
  created_at: string
  moderateur_nom: string | null
  moderateur_prenom: string | null
  nombre_salles_privees: number
  nombre_sessions: number
}

export interface AdminSalleDetail {
  id: string
  titre: string
  slug: string
  description: string | null
  image_couverture_url: string | null
  langue_cible: string | null
  moderateur_id: string | null
  moderateur_nom: string | null
  actif: boolean
  cree_par_nom: string | null
  created_at: string
  updated_at: string
  nombre_salles_privees: number
  nombre_sessions: number
}

export interface CreerSalleForm {
  titre: string
  description?: string
  langue_cible?: string
  moderateur_id?: string
}

export const useAdminSalles = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const salles = ref<AdminSalle[]>([])
  const salleDetail = ref<AdminSalleDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    langue_cible: '',
    actif: '' as string | '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.langue_cible) delete params.langue_cible
    if (params.actif === '') delete params.actif
    const result = await listerPagine<AdminSalle>('/api/admin/salles', params)
    if (result) salles.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminSalleDetail>>(`/api/admin/salles/${id}`)
    if (response.success && response.data) salleDetail.value = response.data
    return response.data
  }

  const creer = async (form: CreerSalleForm) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/salles',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerSalleForm> & { actif?: boolean }) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/salles/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/salles/${id}`, { method: 'DELETE' })
  }

  return {
    salles, salleDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    allerPage, changerTri, reinitialiserPagination,
  }
}

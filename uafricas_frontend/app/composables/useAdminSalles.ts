import type { ApiResponse } from '~/types/admin'

export interface AdminSalle {
  id: string
  titre: string
  slug: string
  langue_cible: string | null
  langue_code: string | null
  actif: boolean
  created_at: string
  groupe_ethnique_nom: string | null
  nombre_salles_privees: number
  nombre_sessions: number
  nombre_moderateurs_attitres: number
}

export interface AdminSalleDetailPaysOrigine {
  id: string
  nom: string
  code_iso2: string | null
}

/** Désactivation administrative (feature 001-ressources-fermeture-session). */
export interface AdminDesactivationInfo {
  desactivee_at: string
  motif: string | null
}

export interface AdminSalleDetail {
  id: string
  titre: string
  slug: string
  description: string | null
  image_couverture_url: string | null
  langue_cible: string | null
  langue_code: string | null
  alphabet: string | null
  dictionnaire_url: string | null
  groupe_ethnique_id: string
  groupe_ethnique_nom: string | null
  actif: boolean
  cree_par_nom: string | null
  created_at: string
  updated_at: string
  nombre_salles_privees: number
  nombre_sessions: number
  nombre_moderateurs_attitres: number
  pays_origine: AdminSalleDetailPaysOrigine[]
  desactivee_admin: AdminDesactivationInfo | null
}

export interface CreerSalleForm {
  titre: string
  groupe_ethnique_id: string
  description?: string
  langue_cible?: string
  langue_code?: string
  alphabet?: string
  dictionnaire_url?: string
}

export interface ModifierSalleForm {
  titre?: string
  description?: string
  langue_cible?: string
  langue_code?: string
  alphabet?: string
  dictionnaire_url?: string
  groupe_ethnique_id?: string
  actif?: boolean
}

export interface GroupeEthniqueOption {
  id: string
  nom: string
  pays_nom: string | null
  salle_id: string | null
  salle_active: boolean
}

export const useAdminSalles = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const salles = ref<AdminSalle[]>([])
  const salleDetail = ref<AdminSalleDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    langue_cible: '',
    langue_code: '',
    groupe_ethnique_id: '',
    actif: '' as string | '',
  })

  const chargerListe = async () => {
    const params: Record<string, any> = { ...filtres }
    if (!params.langue_cible) delete params.langue_cible
    if (!params.langue_code) delete params.langue_code
    if (!params.groupe_ethnique_id) delete params.groupe_ethnique_id
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

  const modifier = async (id: string, form: ModifierSalleForm) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/salles/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/salles/${id}`, { method: 'DELETE' })
  }

  const chargerGroupesEthniques = async (q?: string): Promise<GroupeEthniqueOption[]> => {
    const params: Record<string, any> = { par_page: 100 }
    if (q && q.trim()) params.q = q.trim()
    const response = await $fetch<ApiResponse<{ groupes: GroupeEthniqueOption[] }>>(
      '/api/afrolang/groupes-ethniques',
      { params },
    )
    return response.data?.groupes ?? []
  }

  return {
    salles, salleDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    chargerGroupesEthniques,
    allerPage, changerTri, reinitialiserPagination,
  }
}

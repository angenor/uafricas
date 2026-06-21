import type {
  ApiResponse,
  PaginatedResponse,
  AdminUtilisateur,
  AdminUtilisateurDetail,
  CreerUtilisateurForm,
  ModifierUtilisateurForm,
} from '~/types/admin'

export const useAdminUtilisateurs = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const utilisateurs = ref<AdminUtilisateur[]>([])
  const utilisateurDetail = ref<AdminUtilisateurDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    etat: '',
    role: '',
    pays: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminUtilisateur>('/api/admin/utilisateurs', {
      ...filtres,
    })
    if (result) {
      utilisateurs.value = result.data
    }
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminUtilisateurDetail>>(
      `/api/admin/utilisateurs/${id}`,
    )
    if (response.success && response.data) {
      utilisateurDetail.value = response.data
    }
    return response.data
  }

  const creer = async (form: Partial<CreerUtilisateurForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      '/api/admin/utilisateurs',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<ModifierUtilisateurForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/utilisateurs/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/utilisateurs/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const validerCompteAdmin = async (id: string, compteVerifieAdmin: boolean) => {
    const response = await adminFetch<ApiResponse<{ id: string; compte_verifie_admin: boolean }>>(
      `/api/admin/utilisateurs/${id}/compte-verifie-admin`,
      { method: 'PATCH', body: { compte_verifie_admin: compteVerifieAdmin } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/utilisateurs/${id}`,
      { method: 'DELETE' },
    )
  }

  const assignerRole = async (id: string, roleId: string) => {
    await adminFetch<ApiResponse<any>>(
      `/api/admin/utilisateurs/${id}/roles`,
      { method: 'POST', body: { role_id: roleId } },
    )
  }

  const retirerRole = async (id: string, roleId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/utilisateurs/${id}/roles/${roleId}`,
      { method: 'DELETE' },
    )
  }

  const assignerSpecialite = async (id: string, specialiteId: string) => {
    await adminFetch<ApiResponse<any>>(
      `/api/admin/utilisateurs/${id}/specialites`,
      { method: 'POST', body: { specialite_id: specialiteId } },
    )
  }

  const retirerSpecialite = async (id: string, specialiteId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/utilisateurs/${id}/specialites/${specialiteId}`,
      { method: 'DELETE' },
    )
  }

  const ajouterPermission = async (id: string, data: { permission_id: string; ressource_type: string; ressource_id: string; expire_at?: string }) => {
    await adminFetch<ApiResponse<any>>(
      `/api/admin/utilisateurs/${id}/permissions`,
      { method: 'POST', body: data },
    )
  }

  const retirerPermission = async (id: string, permId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/utilisateurs/${id}/permissions/${permId}`,
      { method: 'DELETE' },
    )
  }

  return {
    utilisateurs,
    utilisateurDetail,
    filtres,
    pagination,
    sort,
    loading,
    error,
    chargerListe,
    chargerDetail,
    creer,
    modifier,
    changerEtat,
    validerCompteAdmin,
    supprimer,
    assignerRole,
    retirerRole,
    assignerSpecialite,
    retirerSpecialite,
    ajouterPermission,
    retirerPermission,
    allerPage,
    changerTri,
    reinitialiserPagination,
  }
}

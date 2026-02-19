import type {
  ApiResponse,
  AdminRole,
  AdminRoleDetail,
  CreerRoleForm,
  PermissionListeItem,
} from '~/types/admin'

export const useAdminRoles = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const roles = ref<AdminRole[]>([])
  const roleDetail = ref<AdminRoleDetail | null>(null)
  const permissions = ref<PermissionListeItem[]>([])

  const filtres = reactive({
    recherche: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminRole>('/api/admin/roles', {
      ...filtres,
    })
    if (result) {
      roles.value = result.data
    }
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminRoleDetail>>(
      `/api/admin/roles/${id}`,
    )
    if (response.success && response.data) {
      roleDetail.value = response.data
    }
    return response.data
  }

  const creer = async (form: Partial<CreerRoleForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string; slug: string }>>(
      '/api/admin/roles',
      { method: 'POST', body: form },
    )
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerRoleForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(
      `/api/admin/roles/${id}`,
      { method: 'PUT', body: form },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/roles/${id}`,
      { method: 'DELETE' },
    )
  }

  const chargerPermissions = async () => {
    const response = await adminFetch<ApiResponse<PermissionListeItem[]>>(
      '/api/admin/permissions',
    )
    if (response.success && response.data) {
      permissions.value = response.data
    }
  }

  const assignerPermissions = async (roleId: string, permissionIds: string[]) => {
    await adminFetch<ApiResponse<any>>(
      `/api/admin/roles/${roleId}/permissions`,
      { method: 'POST', body: { permission_ids: permissionIds } },
    )
  }

  const retirerPermission = async (roleId: string, permissionId: string) => {
    await adminFetch<ApiResponse<null>>(
      `/api/admin/roles/${roleId}/permissions/${permissionId}`,
      { method: 'DELETE' },
    )
  }

  // Permissions groupees par type_ressource pour la matrice
  const permissionsGroupeesParRessource = computed(() => {
    const groupes: Record<string, PermissionListeItem[]> = {}
    for (const perm of permissions.value) {
      if (!groupes[perm.type_ressource]) {
        groupes[perm.type_ressource] = []
      }
      groupes[perm.type_ressource]!.push(perm)
    }
    return groupes
  })

  return {
    roles,
    roleDetail,
    permissions,
    filtres,
    pagination,
    sort,
    loading,
    error,
    chargerListe,
    chargerDetail,
    creer,
    modifier,
    supprimer,
    chargerPermissions,
    assignerPermissions,
    retirerPermission,
    permissionsGroupeesParRessource,
    allerPage,
    changerTri,
    reinitialiserPagination,
  }
}

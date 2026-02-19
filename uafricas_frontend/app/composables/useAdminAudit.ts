import type {
  ApiResponse,
  AdminAudit, AdminAuditDetail,
} from '~/types/admin'

export const useAdminAudit = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const audits = ref<AdminAudit[]>([])
  const auditDetail = ref<AdminAuditDetail | null>(null)

  const filtres = reactive({
    recherche: '',
    action: '',
    schema_name: '',
    table_name: '',
    ip_address: '',
    date_debut: '',
    date_fin: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminAudit>('/api/admin/audit', { ...filtres })
    if (result) audits.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminAuditDetail>>(`/api/admin/audit/${id}`)
    if (response.success && response.data) auditDetail.value = response.data
    return response.data
  }

  return {
    audits, auditDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail,
    allerPage, changerTri, reinitialiserPagination,
  }
}

import type {
  ApiResponse,
  AdminEvenement, AdminEvenementDetail, CreerEvenementAdminForm,
  AdminEvenementInscription, AdminEvenementInscriptionStats,
} from '~/types/admin'

export const useAdminEvenements = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const evenements = ref<AdminEvenement[]>([])
  const evenementDetail = ref<AdminEvenementDetail | null>(null)
  const inscriptions = ref<AdminEvenementInscription[]>([])
  const inscriptionStats = ref<AdminEvenementInscriptionStats | null>(null)

  const filtres = reactive({
    recherche: '',
    format: '',
    etat: '',
    pays_id: '',
    date_debut: '',
    date_fin: '',
  })

  const chargerListe = async () => {
    const result = await listerPagine<AdminEvenement>('/api/admin/evenements', { ...filtres })
    if (result) evenements.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminEvenementDetail>>(`/api/admin/evenements/${id}`)
    if (response.success && response.data) evenementDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerEvenementAdminForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/evenements', { method: 'POST', body: form })
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerEvenementAdminForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/evenements/${id}`, { method: 'PUT', body: form })
    return response.data
  }

  const changerEtat = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/evenements/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/evenements/${id}`, { method: 'DELETE' })
  }

  const chargerInscriptions = async (evenementId: string, filtreStatut?: string) => {
    const params: Record<string, string> = {}
    if (filtreStatut) params.statut = filtreStatut
    const result = await listerPagine<AdminEvenementInscription>(`/api/admin/evenements/${evenementId}/inscriptions`, params)
    if (result) inscriptions.value = result.data
  }

  const changerStatutInscription = async (evenementId: string, inscriptionId: string, statut: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; statut: string }>>(
      `/api/admin/evenements/${evenementId}/inscriptions/${inscriptionId}/statut`,
      { method: 'PATCH', body: { statut } },
    )
    return response.data
  }

  const chargerStatsInscriptions = async (evenementId: string) => {
    const response = await adminFetch<ApiResponse<AdminEvenementInscriptionStats>>(
      `/api/admin/evenements/${evenementId}/inscriptions/stats`,
    )
    if (response.success && response.data) inscriptionStats.value = response.data
    return response.data
  }

  return {
    evenements, evenementDetail, inscriptions, inscriptionStats, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, changerEtat, supprimer,
    chargerInscriptions, changerStatutInscription, chargerStatsInscriptions,
    allerPage, changerTri, reinitialiserPagination,
  }
}

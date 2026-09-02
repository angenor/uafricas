import type {
  ApiResponse,
  AdminContribution, AdminContributionDetail,
} from '~/types/admin'

export const useAdminContributions = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const contributions = ref<AdminContribution[]>([])
  const contributionDetail = ref<AdminContributionDetail | null>(null)

  const filtres = reactive({
    etat: '',
    fiche_pays_id: '',
    cree_par: '',
    type_objet: '', // T042, filtre Afripulse
    section: '',    // T042, filtre Afripulse
  })

  const chargerListe = async () => {
    // On retire les filtres vides avant envoi (?param= vide gênant côté backend)
    const params: Record<string, string> = {}
    for (const [k, v] of Object.entries(filtres)) {
      if (v) params[k] = String(v)
    }
    const result = await listerPagine<AdminContribution>('/api/admin/profils-pays/contributions', params)
    if (result) contributions.value = result.data
  }

  const chargerDetail = async (id: string): Promise<AdminContributionDetail | undefined> => {
    const response = await adminFetch<ApiResponse<AdminContributionDetail>>(`/api/admin/profils-pays/contributions/${id}`)
    if (response.success && response.data) contributionDetail.value = response.data
    return response.data ?? undefined
  }

  const moderer = async (id: string, etat: string, note_moderation?: string) => {
    const body: Record<string, string> = { etat }
    if (note_moderation) body.note_moderation = note_moderation
    const response = await adminFetch<ApiResponse<{ id: string, etat: string, target_id?: string | null }>>(
      `/api/admin/profils-pays/contributions/${id}/etat`,
      { method: 'PATCH', body },
    )
    return response.data
  }

  /**
   * T038 / T044 : Retirer une contribution déjà approuvée.
   * Exige un motif 10..1000 caractères.
   */
  const retirerContribution = async (id: string, motif: string) => {
    if (motif.trim().length < 10 || motif.trim().length > 1000) {
      throw new Error('Le motif doit contenir entre 10 et 1000 caractères')
    }
    const response = await adminFetch<ApiResponse<{ id: string, etat: string }>>(
      `/api/admin/profils-pays/contributions/${id}/retirer`,
      { method: 'POST', body: { motif: motif.trim() } },
    )
    return response.data
  }

  return {
    contributions, contributionDetail, filtres,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, moderer, retirerContribution,
    allerPage, changerTri, reinitialiserPagination,
  }
}

import type {
  ApiResponse,
  AdminFichePay, AdminFichePayDetail, CreerFichePayForm,
  AdminRegion, AdminGroupeEthnique, AdminAlliance, AdminConte,
  AdminSiteTouristique, AdminSecteur, AdminSaison, AdminLienInterethnique,
} from '~/types/admin'

export const useAdminProfilsPays = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const fichesPays = ref<AdminFichePay[]>([])
  const ficheDetail = ref<AdminFichePayDetail | null>(null)

  // Sous-entites
  const regions = ref<AdminRegion[]>([])
  const groupesEthniques = ref<AdminGroupeEthnique[]>([])
  const alliances = ref<AdminAlliance[]>([])
  const contes = ref<AdminConte[]>([])
  const sitesTouristiques = ref<AdminSiteTouristique[]>([])
  const secteurs = ref<AdminSecteur[]>([])
  const saisons = ref<AdminSaison[]>([])
  const liensInterethniques = ref<AdminLienInterethnique[]>([])

  const filtres = reactive({
    recherche: '',
    continent: '',
  })

  // ── Fiche Pays CRUD ───────────────────────────────────────

  const chargerListe = async () => {
    const result = await listerPagine<AdminFichePay>('/api/admin/profils-pays', { ...filtres })
    if (result) fichesPays.value = result.data
  }

  const chargerDetail = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminFichePayDetail>>(`/api/admin/profils-pays/${id}`)
    if (response.success && response.data) ficheDetail.value = response.data
    return response.data
  }

  const creer = async (form: Partial<CreerFichePayForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/profils-pays', { method: 'POST', body: form })
    return response.data
  }

  const modifier = async (id: string, form: Partial<CreerFichePayForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${id}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimer = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/profils-pays/${id}`, { method: 'DELETE' })
  }

  // ── Regions ───────────────────────────────────────────────

  const chargerRegions = async (ficheId: string) => {
    const response = await adminFetch<ApiResponse<AdminRegion[]>>(`/api/admin/profils-pays/${ficheId}/regions`)
    if (response.success && response.data) regions.value = response.data
  }

  const creerRegion = async (ficheId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/regions`, { method: 'POST', body: form })
    return response.data
  }

  const modifierRegion = async (ficheId: string, regionId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/regions/${regionId}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimerRegion = async (ficheId: string, regionId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/profils-pays/${ficheId}/regions/${regionId}`, { method: 'DELETE' })
  }

  // ── Groupes ethniques ─────────────────────────────────────

  const chargerGroupesEthniques = async (ficheId: string) => {
    const response = await adminFetch<ApiResponse<AdminGroupeEthnique[]>>(`/api/admin/profils-pays/${ficheId}/groupes-ethniques`)
    if (response.success && response.data) groupesEthniques.value = response.data
  }

  const creerGroupeEthnique = async (ficheId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/groupes-ethniques`, { method: 'POST', body: form })
    return response.data
  }

  const modifierGroupeEthnique = async (ficheId: string, geId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/groupes-ethniques/${geId}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimerGroupeEthnique = async (ficheId: string, geId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/profils-pays/${ficheId}/groupes-ethniques/${geId}`, { method: 'DELETE' })
  }

  // ── Alliances interethniques ──────────────────────────────

  const chargerAlliances = async (ficheId: string) => {
    const response = await adminFetch<ApiResponse<AdminAlliance[]>>(`/api/admin/profils-pays/${ficheId}/alliances`)
    if (response.success && response.data) alliances.value = response.data
  }

  const creerAlliance = async (ficheId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/alliances`, { method: 'POST', body: form })
    return response.data
  }

  const modifierAlliance = async (ficheId: string, allianceId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/alliances/${allianceId}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimerAlliance = async (ficheId: string, allianceId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/profils-pays/${ficheId}/alliances/${allianceId}`, { method: 'DELETE' })
  }

  // ── Contes & Histoires ────────────────────────────────────

  const chargerContes = async (ficheId: string, typeConte?: string) => {
    const params: Record<string, string> = {}
    if (typeConte) params.type_conte = typeConte
    const response = await adminFetch<ApiResponse<AdminConte[]>>(`/api/admin/profils-pays/${ficheId}/contes`, { params })
    if (response.success && response.data) contes.value = response.data
  }

  const creerConte = async (ficheId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/contes`, { method: 'POST', body: form })
    return response.data
  }

  const modifierConte = async (ficheId: string, conteId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/contes/${conteId}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimerConte = async (ficheId: string, conteId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/profils-pays/${ficheId}/contes/${conteId}`, { method: 'DELETE' })
  }

  // ── Sites touristiques ────────────────────────────────────

  const chargerSitesTouristiques = async (ficheId: string) => {
    const response = await adminFetch<ApiResponse<AdminSiteTouristique[]>>(`/api/admin/profils-pays/${ficheId}/sites-touristiques`)
    if (response.success && response.data) sitesTouristiques.value = response.data
  }

  const creerSiteTouristique = async (ficheId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/sites-touristiques`, { method: 'POST', body: form })
    return response.data
  }

  const modifierSiteTouristique = async (ficheId: string, siteId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/sites-touristiques/${siteId}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimerSiteTouristique = async (ficheId: string, siteId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/profils-pays/${ficheId}/sites-touristiques/${siteId}`, { method: 'DELETE' })
  }

  // ── Secteurs de developpement ─────────────────────────────

  const chargerSecteurs = async (ficheId: string) => {
    const response = await adminFetch<ApiResponse<AdminSecteur[]>>(`/api/admin/profils-pays/${ficheId}/secteurs`)
    if (response.success && response.data) secteurs.value = response.data
  }

  const creerSecteur = async (ficheId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/secteurs`, { method: 'POST', body: form })
    return response.data
  }

  const modifierSecteur = async (ficheId: string, secteurId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/secteurs/${secteurId}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimerSecteur = async (ficheId: string, secteurId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/profils-pays/${ficheId}/secteurs/${secteurId}`, { method: 'DELETE' })
  }

  // ── Saisons ───────────────────────────────────────────────

  const chargerSaisons = async (ficheId: string) => {
    const response = await adminFetch<ApiResponse<AdminSaison[]>>(`/api/admin/profils-pays/${ficheId}/saisons`)
    if (response.success && response.data) saisons.value = response.data
  }

  const creerSaison = async (ficheId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/saisons`, { method: 'POST', body: form })
    return response.data
  }

  const modifierSaison = async (ficheId: string, saisonId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/saisons/${saisonId}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimerSaison = async (ficheId: string, saisonId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/profils-pays/${ficheId}/saisons/${saisonId}`, { method: 'DELETE' })
  }

  // ── Liens interethniques ──────────────────────────────────

  const chargerLiensInterethniques = async (ficheId: string) => {
    const response = await adminFetch<ApiResponse<AdminLienInterethnique[]>>(`/api/admin/profils-pays/${ficheId}/liens-interethniques`)
    if (response.success && response.data) liensInterethniques.value = response.data
  }

  const creerLienInterethnique = async (ficheId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/liens-interethniques`, { method: 'POST', body: form })
    return response.data
  }

  const modifierLienInterethnique = async (ficheId: string, lienId: string, form: Record<string, any>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/profils-pays/${ficheId}/liens-interethniques/${lienId}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimerLienInterethnique = async (ficheId: string, lienId: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/profils-pays/${ficheId}/liens-interethniques/${lienId}`, { method: 'DELETE' })
  }

  return {
    fichesPays, ficheDetail, filtres,
    regions, groupesEthniques, alliances, contes, sitesTouristiques, secteurs, saisons, liensInterethniques,
    pagination, sort, loading, error,
    chargerListe, chargerDetail, creer, modifier, supprimer,
    chargerRegions, creerRegion, modifierRegion, supprimerRegion,
    chargerGroupesEthniques, creerGroupeEthnique, modifierGroupeEthnique, supprimerGroupeEthnique,
    chargerAlliances, creerAlliance, modifierAlliance, supprimerAlliance,
    chargerContes, creerConte, modifierConte, supprimerConte,
    chargerSitesTouristiques, creerSiteTouristique, modifierSiteTouristique, supprimerSiteTouristique,
    chargerSecteurs, creerSecteur, modifierSecteur, supprimerSecteur,
    chargerSaisons, creerSaison, modifierSaison, supprimerSaison,
    chargerLiensInterethniques, creerLienInterethnique, modifierLienInterethnique, supprimerLienInterethnique,
    allerPage, changerTri, reinitialiserPagination,
  }
}

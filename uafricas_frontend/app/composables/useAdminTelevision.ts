import type {
  ApiResponse,
  AdminChaineTv, AdminChaineTvDetail, CreerChaineTvForm,
  AdminProgrammeTele, AdminProgrammeTeleDetail, CreerProgrammeTeleForm,
} from '~/types/admin'

// Back-office TÉLÉVISION : chaînes + programmes télé
export const useAdminTelevision = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()
  const { uploaderMedia, resoudreUrlMedia } = useAdminMediaUpload()

  const chaines = ref<AdminChaineTv[]>([])
  const chaineDetail = ref<AdminChaineTvDetail | null>(null)
  const programmes = ref<AdminProgrammeTele[]>([])
  const programmeDetail = ref<AdminProgrammeTeleDetail | null>(null)

  const filtresChaines = reactive({ recherche: '', categorie: '', pays_id: '', etat: '' })
  const filtresProgrammes = reactive({ recherche: '', chaine_id: '', etat: '' })

  // ── Chaînes ───────────────────────────────────────────────
  const chargerChaines = async () => {
    const result = await listerPagine<AdminChaineTv>('/api/admin/chaines-tv', { ...filtresChaines })
    if (result) chaines.value = result.data
  }
  const chargerChaine = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminChaineTvDetail>>(`/api/admin/chaines-tv/${id}`)
    if (response.success && response.data) chaineDetail.value = response.data
    return response.data
  }
  const creerChaine = async (form: Partial<CreerChaineTvForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/chaines-tv', { method: 'POST', body: form })
    return response.data
  }
  const modifierChaine = async (id: string, form: Partial<CreerChaineTvForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/chaines-tv/${id}`, { method: 'PUT', body: form })
    return response.data
  }
  const supprimerChaine = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/chaines-tv/${id}`, { method: 'DELETE' })
  }
  // Liste non paginée (brouillons inclus) pour alimenter le sélecteur de rattachement
  const listerToutesChaines = async (): Promise<{ id: string; nom: string }[]> => {
    const result = await listerPagine<AdminChaineTv>('/api/admin/chaines-tv', { par_page: 200, page: 1 })
    return result ? result.data.map(c => ({ id: c.id, nom: c.nom })) : []
  }

  // ── Programmes télé ───────────────────────────────────────
  const chargerProgrammes = async () => {
    const result = await listerPagine<AdminProgrammeTele>('/api/admin/programmes-tele', { ...filtresProgrammes })
    if (result) programmes.value = result.data
  }
  const chargerProgramme = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminProgrammeTeleDetail>>(`/api/admin/programmes-tele/${id}`)
    if (response.success && response.data) programmeDetail.value = response.data
    return response.data
  }
  const creerProgramme = async (form: Partial<CreerProgrammeTeleForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/programmes-tele', { method: 'POST', body: form })
    return response.data
  }
  const modifierProgramme = async (id: string, form: Partial<CreerProgrammeTeleForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/programmes-tele/${id}`, { method: 'PUT', body: form })
    return response.data
  }
  const supprimerProgramme = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/programmes-tele/${id}`, { method: 'DELETE' })
  }

  return {
    chaines, chaineDetail, programmes, programmeDetail,
    filtresChaines, filtresProgrammes,
    pagination, sort, loading, error,
    chargerChaines, chargerChaine, creerChaine, modifierChaine, supprimerChaine, listerToutesChaines,
    chargerProgrammes, chargerProgramme, creerProgramme, modifierProgramme, supprimerProgramme,
    uploaderMedia, resoudreUrlMedia,
    allerPage, changerTri, reinitialiserPagination,
  }
}

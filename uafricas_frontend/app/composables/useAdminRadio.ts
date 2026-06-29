import type {
  ApiResponse,
  AdminStationRadio, AdminStationRadioDetail, CreerStationRadioForm,
  AdminProgrammeRadio, AdminProgrammeRadioDetail, CreerProgrammeRadioForm,
} from '~/types/admin'

// Back-office RADIO : stations + émissions (programmes radio)
export const useAdminRadio = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()
  const { uploaderMedia, resoudreUrlMedia } = useAdminMediaUpload()

  const stations = ref<AdminStationRadio[]>([])
  const stationDetail = ref<AdminStationRadioDetail | null>(null)
  const programmes = ref<AdminProgrammeRadio[]>([])
  const programmeDetail = ref<AdminProgrammeRadioDetail | null>(null)

  const filtresStations = reactive({ recherche: '', type_station: '', pays_id: '', etat: '' })
  const filtresProgrammes = reactive({ recherche: '', categorie_radio: '', station_id: '', etat: '' })

  // ── Stations ──────────────────────────────────────────────
  const chargerStations = async () => {
    const result = await listerPagine<AdminStationRadio>('/api/admin/stations-radio', { ...filtresStations })
    if (result) stations.value = result.data
  }
  const chargerStation = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminStationRadioDetail>>(`/api/admin/stations-radio/${id}`)
    if (response.success && response.data) stationDetail.value = response.data
    return response.data
  }
  const creerStation = async (form: Partial<CreerStationRadioForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/stations-radio', { method: 'POST', body: form })
    return response.data
  }
  const modifierStation = async (id: string, form: Partial<CreerStationRadioForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/stations-radio/${id}`, { method: 'PUT', body: form })
    return response.data
  }
  const supprimerStation = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/stations-radio/${id}`, { method: 'DELETE' })
  }
  // Liste non paginée (brouillons inclus) pour alimenter le sélecteur de rattachement
  const listerToutesStations = async (): Promise<{ id: string; nom: string }[]> => {
    const result = await listerPagine<AdminStationRadio>('/api/admin/stations-radio', { par_page: 200, page: 1 })
    return result ? result.data.map(s => ({ id: s.id, nom: s.nom })) : []
  }

  // ── Émissions (programmes radio) ──────────────────────────
  const chargerProgrammes = async () => {
    const result = await listerPagine<AdminProgrammeRadio>('/api/admin/programmes-radio', { ...filtresProgrammes })
    if (result) programmes.value = result.data
  }
  const chargerProgramme = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminProgrammeRadioDetail>>(`/api/admin/programmes-radio/${id}`)
    if (response.success && response.data) programmeDetail.value = response.data
    return response.data
  }
  const creerProgramme = async (form: Partial<CreerProgrammeRadioForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/programmes-radio', { method: 'POST', body: form })
    return response.data
  }
  const modifierProgramme = async (id: string, form: Partial<CreerProgrammeRadioForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/programmes-radio/${id}`, { method: 'PUT', body: form })
    return response.data
  }
  const supprimerProgramme = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/programmes-radio/${id}`, { method: 'DELETE' })
  }

  return {
    stations, stationDetail, programmes, programmeDetail,
    filtresStations, filtresProgrammes,
    pagination, sort, loading, error,
    chargerStations, chargerStation, creerStation, modifierStation, supprimerStation, listerToutesStations,
    chargerProgrammes, chargerProgramme, creerProgramme, modifierProgramme, supprimerProgramme,
    uploaderMedia, resoudreUrlMedia,
    allerPage, changerTri, reinitialiserPagination,
  }
}

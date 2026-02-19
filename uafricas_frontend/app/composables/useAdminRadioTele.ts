import type {
  ApiResponse,
  AdminStationRadio, AdminStationRadioDetail, CreerStationRadioForm,
  AdminChaineTv, AdminChaineTvDetail, CreerChaineTvForm,
  AdminProgrammeMedia, AdminProgrammeMediaDetail, CreerProgrammeMediaForm,
} from '~/types/admin'

export const useAdminRadioTele = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  const stations = ref<AdminStationRadio[]>([])
  const stationDetail = ref<AdminStationRadioDetail | null>(null)
  const chaines = ref<AdminChaineTv[]>([])
  const chaineDetail = ref<AdminChaineTvDetail | null>(null)
  const programmes = ref<AdminProgrammeMedia[]>([])
  const programmeDetail = ref<AdminProgrammeMediaDetail | null>(null)

  const filtresStations = reactive({
    recherche: '',
    type_station: '',
    pays_id: '',
    etat: '',
  })

  const filtresChaines = reactive({
    recherche: '',
    categorie: '',
    pays_id: '',
    etat: '',
  })

  const filtresProgrammes = reactive({
    recherche: '',
    type_programme: '',
    categorie_radio: '',
    etat: '',
  })

  // Stations Radio
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

  // Chaines TV
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

  // Programmes Media
  const chargerProgrammes = async () => {
    const result = await listerPagine<AdminProgrammeMedia>('/api/admin/programmes-media', { ...filtresProgrammes })
    if (result) programmes.value = result.data
  }

  const chargerProgramme = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminProgrammeMediaDetail>>(`/api/admin/programmes-media/${id}`)
    if (response.success && response.data) programmeDetail.value = response.data
    return response.data
  }

  const creerProgramme = async (form: Partial<CreerProgrammeMediaForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/programmes-media', { method: 'POST', body: form })
    return response.data
  }

  const modifierProgramme = async (id: string, form: Partial<CreerProgrammeMediaForm>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/programmes-media/${id}`, { method: 'PUT', body: form })
    return response.data
  }

  const supprimerProgramme = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/programmes-media/${id}`, { method: 'DELETE' })
  }

  return {
    stations, stationDetail, chaines, chaineDetail, programmes, programmeDetail,
    filtresStations, filtresChaines, filtresProgrammes,
    pagination, sort, loading, error,
    chargerStations, chargerStation, creerStation, modifierStation, supprimerStation,
    chargerChaines, chargerChaine, creerChaine, modifierChaine, supprimerChaine,
    chargerProgrammes, chargerProgramme, creerProgramme, modifierProgramme, supprimerProgramme,
    allerPage, changerTri, reinitialiserPagination,
  }
}

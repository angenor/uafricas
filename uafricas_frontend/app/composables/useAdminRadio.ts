import type {
  ApiResponse,
  AdminStationRadio as AdminStationRadioBase,
  AdminStationRadioDetail as AdminStationRadioDetailBase,
  CreerStationRadioForm as CreerStationRadioFormBase,
} from '~/types/admin'

// L'origine de publication n'est pas une étiquette : elle décide de la page publique
// sur laquelle la station apparaît — une station relève d'exactement une des deux.
export type OriginePublicationRadio = 'africans' | 'territoire'

export const ORIGINES_PUBLICATION_RADIO: { valeur: OriginePublicationRadio; libelle: string; page: string; aide: string }[] = [
  {
    valeur: 'africans',
    libelle: 'Radio Africans',
    page: '/medias/radio/africans',
    aide: 'Production propre de la plateforme — publiée sur la page Radio Africans.',
  },
  {
    valeur: 'territoire',
    libelle: 'Radio de territoire',
    page: '/medias/radio/nationales',
    aide: 'Station rattachée à un territoire africain — publiée sur la page Radios nationales.',
  },
]

export type RolePartiePrenanteRadio =
  | 'chaine_tele' | 'radio' | 'journaliste' | 'communicateur'
  | 'createur_contenu' | 'influenceur' | 'realisateur' | 'producteur' | 'autre'

export const ROLES_PARTIE_PRENANTE_RADIO: { valeur: RolePartiePrenanteRadio; libelle: string }[] = [
  { valeur: 'chaine_tele', libelle: 'Chaîne de télévision' },
  { valeur: 'radio', libelle: 'Radio' },
  { valeur: 'journaliste', libelle: 'Journaliste' },
  { valeur: 'communicateur', libelle: 'Communicateur' },
  { valeur: 'createur_contenu', libelle: 'Créateur de contenu' },
  { valeur: 'influenceur', libelle: 'Influenceur' },
  { valeur: 'realisateur', libelle: 'Réalisateur' },
  { valeur: 'producteur', libelle: 'Producteur' },
  { valeur: 'autre', libelle: 'Autre' },
]

export interface AdminStationRadio extends AdminStationRadioBase {
  origine_publication: OriginePublicationRadio
  role_partie_prenante: RolePartiePrenanteRadio | null
  role_partie_prenante_autre: string | null
}

export interface AdminStationRadioDetail extends AdminStationRadioDetailBase {
  origine_publication: OriginePublicationRadio
  role_partie_prenante: RolePartiePrenanteRadio | null
  role_partie_prenante_autre: string | null
}

export interface CreerStationRadioForm extends CreerStationRadioFormBase {
  origine_publication: OriginePublicationRadio
  role_partie_prenante: RolePartiePrenanteRadio | ''
  role_partie_prenante_autre: string
}

export const libelleOriginePublication = (origine?: string | null) =>
  ORIGINES_PUBLICATION_RADIO.find(o => o.valeur === origine)?.libelle || 'Radio de territoire'

export const pageOriginePublication = (origine?: string | null) =>
  ORIGINES_PUBLICATION_RADIO.find(o => o.valeur === origine)?.page || '/medias/radio/nationales'

export const libelleRolePartiePrenante = (role?: string | null, precision?: string | null) => {
  if (!role) return ''
  if (role === 'autre') return precision?.trim() || 'Autre'
  return ROLES_PARTIE_PRENANTE_RADIO.find(r => r.valeur === role)?.libelle || role
}

/**
 * Back-office RADIO : **stations seulement**.
 *
 * Les émissions ne sont plus gérées ici : depuis 09q ce sont des `emission_*`
 * communes aux deux familles, servies par `/api/admin/medias/emissions` et
 * pilotées par `useAdminMediaEmissions`.
 */
export const useAdminRadio = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()
  const { uploaderMedia, resoudreUrlMedia } = useAdminMediaUpload()

  const stations = ref<AdminStationRadio[]>([])
  const stationDetail = ref<AdminStationRadioDetail | null>(null)
  const filtresStations = reactive({ recherche: '', type_station: '', pays_id: '', etat: '', origine_publication: '' })

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
  // Bascule d'une station d'une page publique à l'autre : pas d'endpoint PATCH dédié,
  // on repasse par le PUT de modification.
  const definirOrigine = async (id: string, origine: OriginePublicationRadio) => {
    const resultat = await modifierStation(id, { origine_publication: origine })
    const station = stations.value.find(s => s.id === id)
    if (station) station.origine_publication = origine
    if (stationDetail.value?.id === id) stationDetail.value.origine_publication = origine
    return resultat
  }
  const supprimerStation = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/stations-radio/${id}`, { method: 'DELETE' })
  }
  // Liste non paginée (brouillons inclus) pour alimenter le sélecteur de rattachement
  const listerToutesStations = async (): Promise<{ id: string; nom: string }[]> => {
    const result = await listerPagine<AdminStationRadio>('/api/admin/stations-radio', { par_page: 200, page: 1 })
    return result ? result.data.map(s => ({ id: s.id, nom: s.nom })) : []
  }

  return {
    stations, stationDetail,
    filtresStations,
    pagination, sort, loading, error,
    chargerStations, chargerStation, creerStation, modifierStation, definirOrigine, supprimerStation, listerToutesStations,
    ORIGINES_PUBLICATION_RADIO, ROLES_PARTIE_PRENANTE_RADIO,
    libelleOriginePublication, pageOriginePublication, libelleRolePartiePrenante,
    uploaderMedia, resoudreUrlMedia,
    allerPage, changerTri, reinitialiserPagination,
  }
}

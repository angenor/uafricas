import type {
  ApiResponse, PaginatedResponse,
  AdminChaineTv, AdminChaineTvDetail, CreerChaineTvForm,
  AdminProgrammeTele, AdminProgrammeTeleDetail, CreerProgrammeTeleForm,
} from '~/types/admin'

/** Vedette générale de la page Télé + thème phare : colonnes ajoutées à `media_content.programme_tele`. */
export interface AdminProgrammeTeleVedette {
  a_la_une_globale: boolean
  theme_phare_id: string | null
  theme_phare_autre: string | null
}

export type AdminProgrammeTeleEtendu = AdminProgrammeTele & AdminProgrammeTeleVedette
export type AdminProgrammeTeleDetailEtendu = AdminProgrammeTeleDetail & AdminProgrammeTeleVedette & {
  theme_phare_nom: string | null
}
/** La vedette générale ne passe pas par le PUT : elle a son endpoint transactionnel dédié. */
export type CreerProgrammeTeleFormEtendu = CreerProgrammeTeleForm & {
  theme_phare_id: string
  theme_phare_autre: string
}

/** Thème phare = catégorie `shared.categorie` de contexte `media`. */
export interface ThemePhare {
  id: string
  nom: string
}

/**
 * Origine de publication d'une chaîne TV (09o). Contrairement à la radio, elle
 * ne sépare pas deux pages : les deux familles cohabitent sur `/medias/tele`,
 * où « Africans Télé International » n'est qu'un filtre de la barre d'entrée.
 */
export type OriginePublicationTele = 'africans' | 'territoire'

export const ORIGINES_PUBLICATION_TELE: { valeur: OriginePublicationTele; libelle: string; aide: string }[] = [
  {
    valeur: 'africans',
    libelle: 'Africans Télé International',
    aide: 'Chaîne produite par la plateforme — remontée par le filtre « Africans Télé International » sur /medias/tele.',
  },
  {
    valeur: 'territoire',
    libelle: 'Chaîne de territoire',
    aide: 'Chaîne rattachée à un territoire africain — visible sur /medias/tele hors filtre Africans.',
  },
]

export const libelleOrigineTele = (origine?: string | null) =>
  ORIGINES_PUBLICATION_TELE.find(o => o.valeur === origine)?.libelle || 'Chaîne de territoire'

// Back-office TÉLÉVISION : chaînes + programmes télé
export const useAdminTelevision = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()
  const { uploaderMedia, resoudreUrlMedia } = useAdminMediaUpload()

  const chaines = ref<AdminChaineTv[]>([])
  const chaineDetail = ref<AdminChaineTvDetail | null>(null)
  const programmes = ref<AdminProgrammeTeleEtendu[]>([])
  const programmeDetail = ref<AdminProgrammeTeleDetailEtendu | null>(null)

  const filtresChaines = reactive({ recherche: '', categorie: '', pays_id: '', etat: '', origine: '' })
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
    const result = await listerPagine<AdminProgrammeTeleEtendu>('/api/admin/programmes-tele', { ...filtresProgrammes })
    if (result) programmes.value = result.data
  }
  const chargerProgramme = async (id: string) => {
    const response = await adminFetch<ApiResponse<AdminProgrammeTeleDetailEtendu>>(`/api/admin/programmes-tele/${id}`)
    if (response.success && response.data) programmeDetail.value = response.data
    return response.data
  }
  const creerProgramme = async (form: Partial<CreerProgrammeTeleFormEtendu>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>('/api/admin/programmes-tele', { method: 'POST', body: form })
    return response.data
  }
  const modifierProgramme = async (id: string, form: Partial<CreerProgrammeTeleFormEtendu>) => {
    const response = await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/programmes-tele/${id}`, { method: 'PUT', body: form })
    return response.data
  }
  const supprimerProgramme = async (id: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/programmes-tele/${id}`, { method: 'DELETE' })
  }

  // Endpoint dédié : la rétrogradation de l'ancienne vedette et la promotion de la
  // nouvelle sont faites dans une seule transaction côté serveur, ce que le PUT
  // générique ne garantit pas. Refuse un programme dont l'état n'est pas « publié ».
  const definirVedetteGlobale = async (id: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; ancienne_vedette: string | null }>>(
      `/api/admin/programmes-tele/${id}/vedette-globale`,
      { method: 'PATCH' },
    )
    return response.data
  }

  // ── Thèmes phares (catégories de contexte « media ») ──────
  // adminFetch direct plutôt que listerPagine : ce référentiel ne doit pas écraser
  // l'état de pagination partagé des listes de chaînes/programmes.
  const listerThemesPhares = async (): Promise<ThemePhare[]> => {
    const response = await adminFetch<ApiResponse<PaginatedResponse<ThemePhare>>>(
      '/api/admin/categories',
      { params: { contexte: 'media', par_page: 200, page: 1, tri_par: 'nom', tri_dir: 'asc' } },
    )
    return response.data ? response.data.data.map(c => ({ id: c.id, nom: c.nom })) : []
  }

  return {
    chaines, chaineDetail, programmes, programmeDetail,
    filtresChaines, filtresProgrammes,
    pagination, sort, loading, error,
    chargerChaines, chargerChaine, creerChaine, modifierChaine, supprimerChaine, listerToutesChaines,
    chargerProgrammes, chargerProgramme, creerProgramme, modifierProgramme, supprimerProgramme,
    definirVedetteGlobale, listerThemesPhares,
    ORIGINES_PUBLICATION_TELE,
    uploaderMedia, resoudreUrlMedia,
    allerPage, changerTri, reinitialiserPagination,
  }
}

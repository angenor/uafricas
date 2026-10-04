/**
 * Championship panafricain (feature 013) : saisons et classements.
 *
 * Tout est public. Le jeton n'est joint que s'il existe, pour que le serveur
 * renvoie la position du membre (`moi`) même hors des premières places.
 */

export interface SaisonAPI {
  id: string
  nom: string
  debut_at: string
  fin_at: string
  /** Déduit des dates côté serveur, jamais stocké. */
  etat: 'a_venir' | 'en_cours' | 'close'
}

export interface SaisonsAPI {
  /** `null` hors saison : on joue quand même, mais rien ne compte pour un classement. */
  courante: SaisonAPI | null
  a_venir: SaisonAPI[]
  archives: SaisonAPI[]
}

/** Ce qu'un classement montre d'un membre, et rien d'autre. */
export interface LigneClassementAPI {
  rang: number
  utilisateur_id: string
  nom: string
  prenom: string
  slug: string | null
  photo_url: string | null
  pays: string | null
  pays_iso2: string | null
  niveau_code: string | null
  niveau_libelle: string | null
  score: number
}

export interface ClassementMembresAPI {
  elements: LigneClassementAPI[]
  total: number
  page: number
  taille: number
  /** Position du membre connecté, avec ses voisins immédiats. */
  moi: { rang: number, score: number, voisins: LigneClassementAPI[] } | null
}

export interface LignePaysAPI {
  /** `null` : aucun score encore. Le pays reste listé. */
  rang: number | null
  pays_id: string
  nom: string
  iso2: string | null
  score: number
  joueurs: number
}

export interface FichePaysJeuAPI {
  pays_id: string
  nom: string
  iso2: string | null
  /** `null` : aucun joueur encore. */
  rang: number | null
  score: number
  joueurs: number
  meilleurs: LigneClassementAPI[]
  /** Modules ayant des épreuves sur ce pays ; `disponible` : assez pour une partie. */
  modules: Array<{ code: string, libelle: string, epreuves_jouables: number, disponible: boolean }>
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export const useChampionship = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const enTetes = (): Record<string, string> => {
    const token = userStore.accessToken
    return token ? { Authorization: `Bearer ${token}` } : {}
  }

  /** GET /api/jeu/saisons */
  const saisons = async (): Promise<SaisonsAPI | null> =>
    (await $fetch<ApiResponse<SaisonsAPI>>(`${apiBase}/api/jeu/saisons`)).data

  /**
   * GET /api/jeu/classements/membres
   * `saison` omise : la saison en cours. `pays` : les membres de ce pays.
   */
  const classementMembres = async (
    options: { saison?: string, pays?: string, page?: number, taille?: number } = {},
  ): Promise<ClassementMembresAPI | null> => {
    const query: Record<string, string | number> = {}
    if (options.saison) query.saison = options.saison
    if (options.pays) query.pays = options.pays
    if (options.page) query.page = options.page
    if (options.taille) query.taille = options.taille
    return (await $fetch<ApiResponse<ClassementMembresAPI>>(
      `${apiBase}/api/jeu/classements/membres`,
      { query, headers: enTetes() },
    )).data
  }

  /** GET /api/jeu/classements/pays : les 55 pays, y compris à score nul. */
  const classementPays = async (saison?: string): Promise<LignePaysAPI[]> =>
    (await $fetch<ApiResponse<LignePaysAPI[]>>(
      `${apiBase}/api/jeu/classements/pays`,
      { query: saison ? { saison } : {} },
    )).data ?? []

  /** GET /api/jeu/pays/{id} : la fiche d'un pays sur la carte. */
  const fichePays = async (paysId: string): Promise<FichePaysJeuAPI | null> =>
    (await $fetch<ApiResponse<FichePaysJeuAPI>>(`${apiBase}/api/jeu/pays/${paysId}`)).data

  return { saisons, classementMembres, classementPays, fichePays }
}

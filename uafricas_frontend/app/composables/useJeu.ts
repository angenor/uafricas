/**
 * Activités ludiques (feature 013) : modules, parties.
 *
 * Deux règles propres à ce composable :
 *
 * 1. Le jeton se lit dans le store (`useUserStore().accessToken`), jamais dans
 *    `localStorage` : seul le refresh token y est persisté.
 * 2. `appelAuth` rejoue UNE fois la requête après un rafraîchissement du jeton
 *    quand le serveur répond 401. L'access token vit 15 minutes en mémoire ; un
 *    membre qui laisse l'onglet ouvert puis lance une partie essuierait sinon
 *    un 401 muet, et aucun composable public ne rejoue de lui-même.
 */

export interface ModuleJeuAPI {
  code: string
  libelle: string
  route: string
  icone: string | null
  epreuves_jouables: number
  /** Faux tant que le module n'a pas de quoi composer une partie. */
  disponible: boolean
}

export interface PropositionServieAPI {
  /** Rang d'origine de la proposition : c'est lui qu'on renvoie au serveur. */
  cle: number
  texte: string
}

/** L'épreuve AVANT la réponse : aucune clé de ce type ne dit la bonne réponse. */
export interface EpreuveServieAPI {
  id: string
  enonce: string
  media_type: 'image' | 'audio' | null
  media_url: string | null
  difficulte: number
  propositions: PropositionServieAPI[]
}

export type IssueReponse = 'bonne' | 'mauvaise' | 'sans_reponse' | 'injouable'

export interface CorrectionAPI {
  rang: number
  epreuve_id: string
  issue: IssueReponse
  bonne_cle: number
  cle_choisie: number | null
  explication: string | null
  /** Lien vers le contenu dont l'épreuve est tirée, s'il existe. */
  lien: string | null
  score_gagne: number
  signalable: boolean
}

export type CadrePartie = 'libre' | 'entrainement' | 'defi' | 'duel'

export interface PartieAPI {
  id: string
  cadre: CadrePartie
  module: string | null
  defi_id: string | null
  duel_id: string | null
  nombre_epreuves: number
  rang_courant: number
  etat: 'en_cours' | 'terminee' | 'close'
  temps_epreuve_s: number
  bonnes: number
  score_gagne: number
  /** Présents seulement sur une partie terminée ou close : le bilan. */
  score_total?: number
  /** Rang dans la saison en cours, s'il y en a une et que le membre y a du score. */
  rang_saison?: number
  reponses?: Array<{ rang: number, epreuve_id: string, issue: IssueReponse }>
}

export interface PresentationAPI {
  terminee: boolean
  rang: number
  sur: number
  /** Instant où le temps imparti s'achève, fixé par le serveur. */
  expire_a: string | null
  /** Heure du serveur : le compte à rebours s'y cale. */
  maintenant: string
  epreuve: EpreuveServieAPI | null
  /** Correction de l'épreuve laissée sans réponse (temps écoulé, rechargement). */
  precedente: CorrectionAPI | null
}

export interface DefiAPI {
  id: string
  periodicite: 'jour' | 'semaine'
  /** Jour UTC, ou lundi UTC de la semaine. */
  periode_debut: string
  /** Instant où la période s'achève. */
  fin_at: string
  nombre_epreuves: number
  titre: string | null
  module: string | null
  /** La participation du membre ; `null` s'il n'a pas joué, ou s'il est visiteur. */
  ma_partie: { id: string, etat: PartieAPI['etat'], bonnes: number, score_gagne: number } | null
}

export interface DefisCourantsAPI {
  /** `null` quand le vivier ne permet pas de composer le défi. */
  jour: DefiAPI | null
  semaine: DefiAPI | null
  /** Jours consécutifs de défi du jour terminé ; 0 dès qu'un jour a été manqué. */
  serie_jours: number
  maintenant: string
}

export interface LigneResultatDefiAPI {
  rang: number
  utilisateur_id: string
  nom: string
  prenom: string
  slug: string | null
  photo_url: string | null
  bonnes: number
  temps_total_ms: number
}

export interface ResultatsDefiAPI {
  defi: DefiAPI
  en_cours: boolean
  participants: number
  podium: LigneResultatDefiAPI[]
  moi: LigneResultatDefiAPI | null
}

/**
 * Temps restant, dit en DURÉE (« 5 h 12 »), jamais en heure d'horloge : la
 * période d'un défi est en temps universel, une heure affichée serait fausse
 * pour la plupart des membres.
 */
export const dureeRestante = (finAt: string, maintenant: string | number = Date.now()): string => {
  const ms = new Date(finAt).getTime() - new Date(maintenant).getTime()
  if (ms <= 0) return 'terminé'
  const minutes = Math.floor(ms / 60000)
  const jours = Math.floor(minutes / 1440)
  const heures = Math.floor((minutes % 1440) / 60)
  if (jours > 0) return `${jours} j ${heures} h`
  if (heures > 0) return `${heures} h ${String(minutes % 60).padStart(2, '0')}`
  return `${Math.max(1, minutes)} min`
}

export interface MonJeuAPI {
  score_total: number
  saison: { id: string, nom: string, score: number, rang: number | null } | null
  /**
   * Pays sous lequel le PROCHAIN gain sera classé : origine, repli sur la
   * résidence. `null` : aucun pays au profil, donc aucun classement de pays.
   */
  pays_rattachement: { id: string, nom: string } | null
  serie_jours: number
  par_module: Array<{ code: string, libelle: string, score: number }>
  score_parties: number
  score_defis: number
  score_duels: number
}

export interface PartieHistoriqueAPI {
  id: string
  cadre: CadrePartie
  module_code: string | null
  etat: 'terminee' | 'close'
  bonnes: number
  nombre_epreuves: number
  score_gagne: number
  created_at: string
}

export interface HistoriquePartiesAPI {
  elements: PartieHistoriqueAPI[]
  total: number
  page: number
  taille: number
}

export interface CreerPartieOptions {
  paysId?: string | null
  theme?: string | null
  /** À demander explicitement : le serveur refuse sinon (409). */
  entrainement?: boolean
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/**
 * Le serveur refuse en 409 une partie libre quand le membre a déjà joué toutes
 * les épreuves, pour que l'entraînement soit ANNONCÉ avant de commencer. Ce
 * marqueur distingue ce refus des autres 409 (module fermé, vivier insuffisant).
 */
const MARQUEUR_ENTRAINEMENT = 'entraînement'

/** Message d'erreur du serveur, sans son préfixe technique (« Conflit: … »). */
export const messageErreurJeu = (e: unknown, defaut = 'Une erreur est survenue.'): string => {
  const brut = (e as { data?: { error?: string } })?.data?.error
  return brut ? brut.replace(/^[^:]{1,30}:\s*/, '') : defaut
}

export const estRefusEntrainement = (e: unknown): boolean => {
  const err = e as { status?: number, statusCode?: number, data?: { error?: string } }
  return (err?.status ?? err?.statusCode) === 409
    && (err?.data?.error ?? '').includes(MARQUEUR_ENTRAINEMENT)
}

export const useJeu = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()
  const { refreshAccessToken } = useAuth()

  const authHeaders = (): Record<string, string> => {
    const token = userStore.accessToken
    return token ? { Authorization: `Bearer ${token}` } : {}
  }

  /** Appel authentifié, rejoué une fois après rafraîchissement du jeton sur 401. */
  const appelAuth = async <T>(
    chemin: string,
    options: { method?: 'GET' | 'POST', body?: Record<string, unknown>, query?: Record<string, unknown> } = {},
  ): Promise<T | null> => {
    const lancer = () => $fetch<ApiResponse<T>>(`${apiBase}/api/jeu${chemin}`, {
      ...options,
      headers: authHeaders(),
    })
    try {
      return (await lancer()).data
    }
    catch (e) {
      const statut = (e as { status?: number, statusCode?: number })?.status
        ?? (e as { statusCode?: number })?.statusCode
      if (statut === 401 && await refreshAccessToken()) {
        return (await lancer()).data
      }
      throw e
    }
  }

  /** GET /api/jeu/modules (public) */
  const listerModules = async (): Promise<ModuleJeuAPI[]> => {
    const res = await $fetch<ApiResponse<ModuleJeuAPI[]>>(`${apiBase}/api/jeu/modules`)
    return res.data ?? []
  }

  /** POST /api/jeu/parties */
  const creerPartie = (module: string, options: CreerPartieOptions = {}) =>
    appelAuth<PartieAPI>('/parties', {
      method: 'POST',
      body: {
        module,
        pays_id: options.paysId ?? null,
        theme: options.theme ?? null,
        entrainement: options.entrainement ?? false,
      },
    })

  /** GET /api/jeu/parties/{id} : état, et bilan si la partie est finie. */
  const obtenirPartie = (id: string) => appelAuth<PartieAPI>(`/parties/${id}`)

  /** POST /api/jeu/parties/{id}/suivante */
  const suivante = (id: string) =>
    appelAuth<PresentationAPI>(`/parties/${id}/suivante`, { method: 'POST' })

  /**
   * POST /api/jeu/parties/{id}/repondre
   *
   * `cle = null` : le temps s'est écoulé. Le serveur enregistre l'absence de
   * réponse et renvoie la correction SANS présenter l'épreuve suivante, pour
   * que le membre la lise avant que l'horloge ne reparte.
   */
  const repondre = (id: string, rang: number, cle: number | null) =>
    appelAuth<CorrectionAPI>(`/parties/${id}/repondre`, { method: 'POST', body: { rang, cle } })

  /** POST /api/jeu/parties/{id}/injouable : le média ne se charge pas. */
  const declarerInjouable = (id: string, rang: number) =>
    appelAuth<CorrectionAPI>(`/parties/${id}/injouable`, { method: 'POST', body: { rang } })

  /** GET /api/jeu/moi : score, saison, pays de rattachement, série. */
  const monJeu = () => appelAuth<MonJeuAPI>('/moi')

  /** GET /api/jeu/moi/parties : parties terminées ou closes, les plus récentes d'abord. */
  const mesParties = (page = 1, taille = 20) =>
    appelAuth<HistoriquePartiesAPI>('/moi/parties', { query: { page, taille } })

  // ── Défis ──────────────────────────────────────────────────────────────

  /**
   * GET /api/jeu/defis/courants. Public : le jeton n'est joint que s'il existe,
   * pour obtenir la participation du membre et sa série de jours.
   */
  const defisCourants = async (): Promise<DefisCourantsAPI | null> => {
    if (userStore.accessToken) return appelAuth<DefisCourantsAPI>('/defis/courants')
    return (await $fetch<ApiResponse<DefisCourantsAPI>>(`${apiBase}/api/jeu/defis/courants`)).data
  }

  /**
   * POST /api/jeu/defis/{id}/jouer : ouvre la participation, ou renvoie celle
   * qui existe déjà (une seule par défi).
   */
  const jouerDefi = (id: string) =>
    appelAuth<{ partie_id: string, etat: PartieAPI['etat'], deja_commencee: boolean }>(
      `/defis/${id}/jouer`, { method: 'POST' },
    )

  /** GET /api/jeu/defis/{id}/resultats */
  const resultatsDefi = async (id: string): Promise<ResultatsDefiAPI | null> => {
    if (userStore.accessToken) return appelAuth<ResultatsDefiAPI>(`/defis/${id}/resultats`)
    return (await $fetch<ApiResponse<ResultatsDefiAPI>>(`${apiBase}/api/jeu/defis/${id}/resultats`)).data
  }

  /** POST /api/jeu/epreuves/{id}/signaler : une fois par épreuve. */
  const signalerEpreuve = (id: string, motif: string, commentaire: string) =>
    appelAuth<{ deja_traitee: boolean }>(`/epreuves/${id}/signaler`, {
      method: 'POST',
      body: { motif, commentaire: commentaire || null },
    })

  return {
    appelAuth,
    monJeu,
    mesParties,
    defisCourants,
    jouerDefi,
    resultatsDefi,
    signalerEpreuve,
    listerModules,
    creerPartie,
    obtenirPartie,
    suivante,
    repondre,
    declarerInjouable,
  }
}

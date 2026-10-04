/**
 * Concours côté membre et public (feature 014, famille B) : découvrir un
 * concours, y déposer sa photo, voter à l'aveugle, lire les résultats.
 *
 * Pendant l'appel et le vote, l'API ne dit jamais qui a déposé quelle photo :
 * `ParticipationAnonymeAPI` n'a pas de champ auteur.
 */

export type PhaseConcoursPublique = 'appel' | 'vote' | 'deliberation' | 'resultats' | 'annule'
export type EtatMaParticipation = 'en_attente' | 'publiee' | 'rejetee' | 'retiree' | 'suspendue'

export interface MaParticipationAPI {
  id: string
  etat: EtatMaParticipation
  media_type: 'image'
  media_url: string
  legende: string | null
  motif_rejet: string | null
  created_at: string
}

export interface ConcoursPublicAPI {
  id: string
  format: 'photo'
  titre: string
  theme: string
  reglement: string
  rattachement: string | null
  image_url: string | null
  phase: PhaseConcoursPublique
  appel_debut: string
  vote_debut: string
  vote_fin: string
  participations_max: number
  jury: boolean
  motif_annulation: string | null
  participations_publiees: number
  /** Présent seulement pour un membre connecté. */
  moi?: {
    participations: MaParticipationAPI[]
    peut_participer: boolean
    peut_voter: boolean
    votes_exprimes: number
    votes_max: number | null
  }
}

export interface ParticipationAnonymeAPI {
  id: string
  media_type: 'image'
  media_url: string
  legende: string | null
}

export interface ParticipationClasseeAPI extends ParticipationAnonymeAPI {
  auteur: { id: string, nom: string, prenom: string, photo_url: string | null }
  rang: number | null
  /** En pourcentage, une décimale. */
  taux: number | null
  duels: number | null
  sous_seuil: boolean | null
  place_jury: number | null
}

export interface ParticipationHistoriqueAPI {
  id: string
  concours_id: string
  concours_titre: string
  etat: EtatMaParticipation
  media_url: string
  legende: string | null
  motif_rejet: string | null
  created_at: string
  rang: number | null
  taux: number | null
  gain: number
  phase: PhaseConcoursPublique | 'a_venir' | null
}

/** Une paire à départager : deux photos, aucun auteur, aucun décompte. */
export type TirageAPI =
  | {
    id: string
    gauche: { id: string, media_url: string, legende: string | null }
    droite: { id: string, media_url: string, legende: string | null }
    votes_exprimes: number
    votes_max: number | null
  }
  | { termine: true, raison: 'toutes_vues' | 'plafond', votes_exprimes: number }

export const LIBELLES_PHASE_PUBLIQUE: Record<PhaseConcoursPublique, string> = {
  appel: 'Appel à participation',
  vote: 'Vote en cours',
  deliberation: 'Délibération du jury',
  resultats: 'Résultats',
  annule: 'Annulé',
}

export const LIBELLES_MA_PARTICIPATION: Record<EtatMaParticipation, string> = {
  en_attente: 'En attente de relecture',
  publiee: 'Publiée',
  rejetee: 'Refusée',
  retiree: 'Retirée',
  suspendue: 'Suspendue après signalements',
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

type Methode = 'GET' | 'POST' | 'PUT' | 'DELETE'

export const useConcours = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()
  const { refreshAccessToken } = useAuth()

  const entetes = (): Record<string, string> => {
    const token = userStore.accessToken
    return token ? { Authorization: `Bearer ${token}` } : {}
  }

  /**
   * Appel à `/api/jeu/concours…`, jeton joint s'il existe, rejoué une fois
   * après rafraîchissement sur 401. Un `FormData` part tel quel : le navigateur
   * pose lui-même le type multipart et sa frontière.
   */
  const appel = async <T>(
    chemin: string,
    options: { method?: Methode, body?: Record<string, unknown> | FormData, query?: Record<string, unknown> } = {},
  ): Promise<T | null> => {
    const lancer = () => $fetch<ApiResponse<T>>(`${apiBase}/api/jeu/concours${chemin}`, {
      ...options,
      headers: entetes(),
    })
    try {
      return (await lancer()).data
    }
    catch (e) {
      const statut = (e as { status?: number, statusCode?: number })?.status
        ?? (e as { statusCode?: number })?.statusCode
      if (statut === 401 && await refreshAccessToken()) return (await lancer()).data
      throw e
    }
  }

  const lister = (filtres: { phase?: string, rattachement?: string, page?: number, taille?: number } = {}) =>
    appel<{ elements: ConcoursPublicAPI[], total: number }>('', { query: filtres })

  const obtenir = (id: string) => appel<ConcoursPublicAPI>(`/${id}`)

  /** Anonyme pendant l'appel et le vote ; classée avec les auteurs ensuite. */
  const galerie = (id: string) =>
    appel<Array<ParticipationAnonymeAPI | ParticipationClasseeAPI>>(`/${id}/participations`)

  const formulaire = (photo: File | null, legende: string) => {
    const corps = new FormData()
    if (photo) corps.append('photo', photo)
    corps.append('legende', legende)
    return corps
  }

  const deposer = (id: string, photo: File, legende: string) =>
    appel<MaParticipationAPI>(`/${id}/participations`, { method: 'POST', body: formulaire(photo, legende) })

  const remplacer = (id: string, pid: string, photo: File | null, legende: string) =>
    appel<MaParticipationAPI>(`/${id}/participations/${pid}`, { method: 'PUT', body: formulaire(photo, legende) })

  const retirer = (id: string, pid: string) =>
    appel<{ retiree: boolean }>(`/${id}/participations/${pid}`, { method: 'DELETE' })

  const mesParticipations = () => appel<ParticipationHistoriqueAPI[]>('/mes-participations')

  /** La paire ouverte du votant, ou une nouvelle (la même revient au rechargement). */
  const confrontation = (id: string) => appel<TirageAPI>(`/${id}/confrontations`, { method: 'POST' })

  /** Vote, et reçoit directement la paire suivante. */
  const voter = (id: string, confrontationId: string, choix: 'gauche' | 'droite') =>
    appel<TirageAPI>(`/${id}/confrontations/${confrontationId}/voter`, { method: 'POST', body: { choix } })

  /** Une fois par membre ; au-delà du seuil, la photo est suspendue. */
  const signaler = (id: string, pid: string, motif: string) =>
    appel<{ signale: boolean }>(`/${id}/participations/${pid}/signaler`, { method: 'POST', body: { motif } })

  return { lister, obtenir, galerie, deposer, remplacer, retirer, mesParticipations, confrontation, voter, signaler }
}

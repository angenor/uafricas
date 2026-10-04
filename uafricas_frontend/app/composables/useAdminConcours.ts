/**
 * Back-office des concours (feature 014, famille B) : programmation,
 * modération des participations, suivi et jury.
 */
import type { ApiResponse } from '~/types/admin'

export type PhaseConcours = 'a_venir' | 'appel' | 'vote' | 'deliberation' | 'resultats' | 'annule'
export type EtatParticipation = 'en_attente' | 'publiee' | 'rejetee' | 'retiree' | 'suspendue'

export const LIBELLES_PHASE: Record<PhaseConcours, string> = {
  a_venir: 'À venir',
  appel: 'Appel à participation',
  vote: 'Vote en cours',
  deliberation: 'Délibération du jury',
  resultats: 'Résultats',
  annule: 'Annulé',
}

export const LIBELLES_ETAT_PARTICIPATION: Record<EtatParticipation, string> = {
  en_attente: 'En attente',
  publiee: 'Publiée',
  rejetee: 'Refusée',
  retiree: 'Retirée',
  suspendue: 'Suspendue',
}

export interface ConcoursAdminAPI {
  id: string
  format: 'photo'
  titre: string
  theme: string
  reglement: string
  rattachement: string | null
  image_url: string | null
  appel_debut: string
  vote_debut: string
  vote_fin: string
  participations_max: number
  minimum_participations: number
  votes_max: number | null
  presentations_min: number
  jury: boolean
  jury_finalistes: number
  jury_delai_jours: number
  prime_participation: number | null
  prime_podium: number[] | null
  etat: 'actif' | 'annule' | 'resultats'
  motif_annulation: string | null
  phase: PhaseConcours
  en_attente: number
  publiees: number
  rejetees: number
  votes: number
  votes_comptes: number
}

/** Ce que le formulaire envoie ; les champs facultatifs gardent leur défaut serveur. */
export interface ConcoursForm {
  format: 'photo'
  titre: string
  theme: string
  reglement: string
  rattachement: string | null
  image_url: string | null
  appel_debut: string
  vote_debut: string
  vote_fin: string
  participations_max: number
  minimum_participations: number
  votes_max: number | null
  presentations_min: number
  jury: boolean
  jury_finalistes: number
  jury_delai_jours: number
  prime_participation: number | null
  prime_podium: number[] | null
}

export interface ParticipationModerationAPI {
  id: string
  concours_id: string
  concours_titre: string
  auteur_id: string
  auteur_nom: string
  auteur_prenom: string
  media_type: 'image'
  media_url: string
  legende: string | null
  etat: EtatParticipation
  motif_rejet: string | null
  nombre_signalements: number
  created_at: string
}

export interface FinalisteAPI {
  participation_id: string
  media_url: string
  legende: string | null
  auteur: string
  rang: number
  /** En pourcentage. */
  taux: number
  duels: number
}

export interface SuiviVoteAPI {
  votes: { total: number, comptes: number, ecartes: Record<string, number> }
  votants: number
  presentations: { min: number | null, max: number | null, moyenne: number | null }
  classement_provisoire: FinalisteAPI[]
  comptes_signales: Array<{ utilisateur_id: string, nom: string, motifs: Array<'rythme' | 'volume' | 'preference'>, votes: number, ecarte_par_admin: boolean }>
}

export interface PageAPI<T> {
  data: T[]
  total: number
  page: number
  par_page: number
}

export const useAdminConcours = () => {
  const { adminFetch } = useAdmin()

  const lister = async (phase?: PhaseConcours | '', page = 1) =>
    (await adminFetch<ApiResponse<PageAPI<ConcoursAdminAPI>>>('/api/admin/jeu/concours', {
      params: { phase, page },
    })).data

  const obtenir = async (id: string) =>
    (await adminFetch<ApiResponse<ConcoursAdminAPI>>(`/api/admin/jeu/concours/${id}`)).data

  const creer = async (form: ConcoursForm) =>
    (await adminFetch<ApiResponse<ConcoursAdminAPI>>('/api/admin/jeu/concours', {
      method: 'POST', body: form,
    })).data

  const modifier = async (id: string, form: ConcoursForm) =>
    (await adminFetch<ApiResponse<ConcoursAdminAPI>>(`/api/admin/jeu/concours/${id}`, {
      method: 'PUT', body: form,
    })).data

  const supprimer = async (id: string) =>
    adminFetch<ApiResponse<{ supprime: boolean }>>(`/api/admin/jeu/concours/${id}`, { method: 'DELETE' })

  const annuler = async (id: string, motif: string) =>
    (await adminFetch<ApiResponse<ConcoursAdminAPI>>(`/api/admin/jeu/concours/${id}/annuler`, {
      method: 'POST', body: { motif },
    })).data

  const listerParticipations = async (filtres: { etat?: EtatParticipation | 'toutes', concours?: string, page?: number } = {}) =>
    (await adminFetch<ApiResponse<PageAPI<ParticipationModerationAPI>>>('/api/admin/jeu/concours/participations', {
      params: filtres,
    })).data

  const accepter = async (id: string) =>
    (await adminFetch<ApiResponse<ParticipationModerationAPI>>(`/api/admin/jeu/concours/participations/${id}/accepter`, {
      method: 'POST',
    })).data

  const rejeter = async (id: string, motif: string) =>
    (await adminFetch<ApiResponse<ParticipationModerationAPI>>(`/api/admin/jeu/concours/participations/${id}/rejeter`, {
      method: 'POST', body: { motif },
    })).data

  const modererGroupe = async (ids: string[], decision: 'accepter' | 'rejeter', motif?: string) =>
    (await adminFetch<ApiResponse<{ acceptees: number, rejetees: number, ignorees: number }>>(
      '/api/admin/jeu/concours/participations/moderation-groupee',
      { method: 'POST', body: { ids, decision, motif: motif ?? null } },
    )).data

  const retablir = async (id: string) =>
    (await adminFetch<ApiResponse<ParticipationModerationAPI>>(`/api/admin/jeu/concours/participations/${id}/retablir`, {
      method: 'POST',
    })).data

  const finalistes = async (id: string) =>
    (await adminFetch<ApiResponse<FinalisteAPI[]>>(`/api/admin/jeu/concours/${id}/finalistes`)).data ?? []

  const deliberer = async (id: string, podium: string[]) =>
    (await adminFetch<ApiResponse<ConcoursAdminAPI>>(`/api/admin/jeu/concours/${id}/deliberation`, {
      method: 'POST', body: { podium },
    })).data

  const suivi = async (id: string) =>
    (await adminFetch<ApiResponse<SuiviVoteAPI>>(`/api/admin/jeu/concours/${id}/suivi`)).data

  const reglerVoix = async (id: string, votant: string, action: 'ecarter' | 'retablir') =>
    adminFetch<ApiResponse<Record<string, number>>>(`/api/admin/jeu/concours/${id}/votants/${votant}/${action}`, {
      method: 'POST',
    })

  return {
    lister, obtenir, creer, modifier, supprimer, annuler,
    listerParticipations, accepter, rejeter, modererGroupe, retablir,
    finalistes, deliberer, suivi, reglerVoix,
  }
}

/** Message d'erreur lisible depuis une réponse d'API d'administration. */
export const messageErreurConcours = (e: unknown, defaut = 'L\'opération a échoué.') => {
  const err = e as { data?: { error?: string } }
  const brut = err?.data?.error
  if (!brut) return defaut
  return brut.replace(/^(Validation|Conflit|Non trouvé|NonTrouve|Accès interdit) ?: ?/, '')
}

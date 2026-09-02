// Composable du domaine social : rendez-vous en visioconférence entre amis.
// Prise de rendez-vous (proposer → répondre / contre-proposer → accepter →
// annuler) + accès à la salle visio P2P. État partagé alimenté en temps réel
// par le flux SSE de la messagerie (plugins/messagerie.client.ts).
import type { MembreLightAPI } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types (alignés sur contracts/rendez-vous.md, JSON snake_case)
// ──────────────────────────────────────────────────────────────

export type StatutRendezVous = 'propose' | 'accepte' | 'refuse' | 'annule'
export type EtatDerive = 'expire' | 'termine' | null
export type FiltreRendezVous = 'attente_moi' | 'attente_autre' | 'a_venir' | 'passes'

export interface RendezVousAPI {
  id: string
  sujet: string
  description: string | null
  date_heure: string
  duree_minutes: number
  statut: StatutRendezVous
  tour_id: string
  mon_tour: boolean
  suis_initiateur: boolean
  etat_derive: EtatDerive
  peut_rejoindre: boolean
  autre: MembreLightAPI
  created_at: string
  updated_at: string
}

export interface ListeRendezVousAPI {
  items: RendezVousAPI[]
  page: number
  taille: number
  total: number
}

export interface SalleAPI {
  rendez_vous_id: string
  mon_peer_id: string
  pair_peer_id: string
  suis_appelant: boolean
  autre: MembreLightAPI
}

export interface ProposerPayload {
  destinataire_id: string
  sujet: string
  description?: string
  date_heure: string
  duree_minutes: number
}

export interface ContreProposerPayload {
  date_heure: string
  duree_minutes: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

// Évènement SSE rendez-vous (forme générique poussée à l'autre participant).
export interface EvenementRendezVous {
  type: 'rdv_propose' | 'rdv_accepte' | 'rdv_refuse' | 'rdv_contre_propose' | 'rdv_annule'
  rendez_vous_id: string
}

/** Extrait un message d'erreur français de l'enveloppe ApiResponse renvoyée par $fetch. */
const extraireErreur = (e: unknown, defaut: string): string => {
  const data = (e as { data?: { error?: string } })?.data
  return data?.error || defaut
}

export const useRendezVous = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  // État global partagé (SSR-safe)
  const rendezVous = useState<RendezVousAPI[]>('rdv:liste', () => [])
  const filtreCourant = useState<FiltreRendezVous>('rdv:filtre', () => 'attente_moi')
  const total = useState<number>('rdv:total', () => 0)
  const chargement = useState<boolean>('rdv:chargement', () => false)
  // Pastille « en attente de ma réponse » (onglet Rendez-vous du panneau).
  const nbAttenteMoi = useState<number>('rdv:attente-moi', () => 0)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) return { Authorization: `Bearer ${userStore.accessToken}` }
    return {}
  }

  // ── Lecture ─────────────────────────────────────────────────

  const lister = async (filtre: FiltreRendezVous, page = 1): Promise<void> => {
    filtreCourant.value = filtre
    chargement.value = true
    try {
      const r = await $fetch<ApiResponse<ListeRendezVousAPI>>(
        `${apiBase}/api/rendez-vous`,
        { headers: authHeaders(), query: { filtre, page } },
      )
      if (r.success && r.data) {
        rendezVous.value = r.data.items
        total.value = r.data.total
      }
    }
    catch (e) {
      console.error('Erreur lister rendez-vous:', e)
      rendezVous.value = []
      total.value = 0
    }
    finally {
      chargement.value = false
    }
  }

  const obtenir = async (id: string): Promise<RendezVousAPI | null> => {
    try {
      const r = await $fetch<ApiResponse<RendezVousAPI>>(
        `${apiBase}/api/rendez-vous/${id}`,
        { headers: authHeaders() },
      )
      return r.success ? r.data : null
    }
    catch (e) {
      console.error('Erreur obtenir rendez-vous:', e)
      return null
    }
  }

  /** Recharge le filtre actuellement affiché (après une mutation ou un évènement SSE). */
  const rafraichir = (): Promise<void> => lister(filtreCourant.value)

  /** Met à jour la pastille « en attente de ma réponse » (compteur léger). */
  const compterAttenteMoi = async (): Promise<void> => {
    try {
      const r = await $fetch<ApiResponse<ListeRendezVousAPI>>(
        `${apiBase}/api/rendez-vous`,
        { headers: authHeaders(), query: { filtre: 'attente_moi', page: 1, taille: 1 } },
      )
      if (r.success && r.data) nbAttenteMoi.value = r.data.total
    }
    catch {
      // Silencieux : la pastille reste à sa dernière valeur connue.
    }
  }

  // ── Mutations (proposer / répondre / annuler) ───────────────

  const proposer = async (payload: ProposerPayload): Promise<RendezVousAPI> => {
    try {
      const r = await $fetch<ApiResponse<RendezVousAPI>>(
        `${apiBase}/api/rendez-vous`,
        { method: 'POST', headers: { 'Content-Type': 'application/json', ...authHeaders() }, body: payload },
      )
      if (!r.success || !r.data) throw new Error(r.error || 'Échec de la proposition')
      return r.data
    }
    catch (e) {
      throw new Error(extraireErreur(e, 'Impossible de proposer ce rendez-vous'))
    }
  }

  const action = async (id: string, chemin: string, message: string, body?: unknown): Promise<RendezVousAPI> => {
    try {
      const r = await $fetch<ApiResponse<RendezVousAPI>>(
        `${apiBase}/api/rendez-vous/${id}/${chemin}`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: body ?? {},
        },
      )
      if (!r.success || !r.data) throw new Error(r.error || message)
      return r.data
    }
    catch (e) {
      throw new Error(extraireErreur(e, message))
    }
  }

  const accepter = (id: string): Promise<RendezVousAPI> =>
    action(id, 'accepter', "Impossible d'accepter ce rendez-vous")

  const refuser = (id: string): Promise<RendezVousAPI> =>
    action(id, 'refuser', 'Impossible de refuser ce rendez-vous')

  const contreProposer = (id: string, payload: ContreProposerPayload): Promise<RendezVousAPI> =>
    action(id, 'contre-proposer', 'Impossible de contre-proposer', payload)

  const annuler = (id: string): Promise<RendezVousAPI> =>
    action(id, 'annuler', "Impossible d'annuler ce rendez-vous")

  const obtenirSalle = async (id: string): Promise<SalleAPI> => {
    try {
      const r = await $fetch<ApiResponse<SalleAPI>>(
        `${apiBase}/api/rendez-vous/${id}/salle`,
        { headers: authHeaders() },
      )
      if (!r.success || !r.data) throw new Error(r.error || 'Salle indisponible')
      return r.data
    }
    catch (e) {
      throw new Error(extraireErreur(e, "La salle n'est pas disponible pour le moment"))
    }
  }

  // ── Dispatch des évènements SSE ──────────────────────────────

  /** Sur tout évènement `rdv_*`, rafraîchit la liste affichée (panneau ouvert). */
  const gererEvenement = (evt: EvenementRendezVous): void => {
    if (!evt?.type?.startsWith('rdv_')) return
    // Recharge le filtre courant + la pastille pour refléter l'état en temps réel.
    void rafraichir()
    void compterAttenteMoi()
  }

  return {
    rendezVous,
    filtreCourant,
    total,
    chargement,
    nbAttenteMoi,
    lister,
    obtenir,
    rafraichir,
    compterAttenteMoi,
    proposer,
    accepter,
    refuser,
    contreProposer,
    annuler,
    obtenirSalle,
    gererEvenement,
  }
}

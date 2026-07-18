// Composable du domaine social : appels directs en visioconférence entre amis.
// Permet d'appeler un ami immédiatement, sans prise de rendez-vous. L'appel est
// éphémère côté serveur (aucune persistance) ; la visioconférence est P2P
// (WebRTC/PeerJS). L'état est alimenté en temps réel par le flux SSE de la
// messagerie (plugins/messagerie.client.ts) : sonnerie entrante, acceptation,
// refus, annulation.
import type { MembreLightAPI } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types (JSON snake_case, alignés sur models/appel.rs)
// ──────────────────────────────────────────────────────────────

/** Configuration P2P renvoyée à l'ouverture / l'acceptation d'un appel. */
export interface SalleAppelAPI {
  appel_id: string
  mon_peer_id: string
  pair_peer_id: string
  suis_appelant: boolean
  /** Vidéo activée d'emblée, ou appel « normal » caméra coupée par défaut. */
  video: boolean
  autre: MembreLightAPI
}

/** Sonnerie entrante (poussée au destinataire). */
export interface AppelEntrant {
  appel_id: string
  appelant: MembreLightAPI
  /** L'appelant a lancé un appel vidéo (`true`) ou un appel normal (`false`). */
  video: boolean
}

/** Raison de fin d'un appel non connecté. */
export type FinAppel = 'refuse' | 'annule'

/** Évènement SSE appel (forme générique poussée au participant concerné). */
export interface EvenementAppel {
  type: 'appel_entrant' | 'appel_accepte' | 'appel_refuse' | 'appel_annule'
  appel_id: string
  appelant?: MembreLightAPI
  video?: boolean
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export const useAppels = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  // État global partagé (SSR-safe).
  /** Sonnerie entrante en attente de réponse (ou null). */
  const appelEntrant = useState<AppelEntrant | null>('appel:entrant', () => null)
  /** Salle d'appel en cours (config P2P) — pilote l'affichage de la visio. */
  const appelActif = useState<SalleAppelAPI | null>('appel:actif', () => null)
  /** Signal de fin (refus/annulation) pour l'appel actif non connecté. */
  const finAppel = useState<FinAppel | null>('appel:fin', () => null)
  /** Le destinataire a rejoint : met à jour le statut côté appelant. */
  const appelRepondu = useState<boolean>('appel:repondu', () => false)
  /** Dernière erreur (ex. capacité, relation rompue). */
  const erreurAppel = useState<string | null>('appel:erreur', () => null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) return { Authorization: `Bearer ${userStore.accessToken}` }
    return {}
  }

  /** POST « best effort » qui ignore les erreurs (refus/annulation côté serveur). */
  const postSilencieux = async (chemin: string): Promise<void> => {
    try {
      await $fetch(`${apiBase}${chemin}`, { method: 'POST', headers: authHeaders() })
    }
    catch {
      // L'appel a pu déjà expirer côté serveur : sans conséquence.
    }
  }

  const reinitialiser = (): void => {
    appelActif.value = null
    finAppel.value = null
    appelRepondu.value = false
  }

  // ── Émettre un appel ────────────────────────────────────────

  /**
   * Démarre un appel direct vers un ami. Ouvre la salle si la mise en relation
   * réussit. `avecVideo` = appel vidéo (caméra active d'emblée) ; par défaut
   * appel « normal » (audio, caméra coupée mais activable en cours d'appel).
   */
  const appeler = async (destinataireId: string, avecVideo = false): Promise<boolean> => {
    erreurAppel.value = null
    // Un seul appel à la fois.
    if (appelActif.value || appelEntrant.value) {
      erreurAppel.value = 'Un appel est déjà en cours.'
      return false
    }
    try {
      const r = await $fetch<ApiResponse<SalleAppelAPI>>(`${apiBase}/api/appels`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: { destinataire_id: destinataireId, video: avecVideo },
      })
      if (!r.success || !r.data) throw new Error(r.error || 'Appel impossible')
      finAppel.value = null
      appelRepondu.value = false
      appelActif.value = r.data
      return true
    }
    catch (e) {
      const data = (e as { data?: { error?: string } })?.data
      erreurAppel.value = data?.error || "Impossible de joindre ce membre pour le moment"
      return false
    }
  }

  // ── Répondre à une sonnerie ─────────────────────────────────

  /** Accepte la sonnerie en cours et ouvre la salle. */
  const accepterAppel = async (): Promise<void> => {
    const entrant = appelEntrant.value
    if (!entrant) return
    appelEntrant.value = null
    try {
      const r = await $fetch<ApiResponse<SalleAppelAPI>>(
        `${apiBase}/api/appels/${entrant.appel_id}/salle`,
        { headers: authHeaders() },
      )
      if (r.success && r.data) {
        finAppel.value = null
        // Côté destinataire, l'appelant est déjà présent → on file vers la connexion.
        appelRepondu.value = true
        appelActif.value = r.data
      }
    }
    catch {
      // L'appel a expiré ou l'appelant a raccroché entre-temps.
      erreurAppel.value = "Cet appel n'est plus disponible."
    }
  }

  /** Décline la sonnerie en cours. */
  const refuserAppel = async (): Promise<void> => {
    const entrant = appelEntrant.value
    if (!entrant) return
    appelEntrant.value = null
    await postSilencieux(`/api/appels/${entrant.appel_id}/refuser`)
  }

  // ── Raccrocher ──────────────────────────────────────────────

  /**
   * Raccroche la salle active. Si l'appel n'était pas encore connecté, prévient
   * l'autre participant (annulation), sauf si la fin vient déjà du distant.
   */
  const raccrocher = async (connecte: boolean): Promise<void> => {
    const actif = appelActif.value
    const finDistante = finAppel.value !== null
    reinitialiser()
    if (actif && !connecte && !finDistante) {
      await postSilencieux(`/api/appels/${actif.appel_id}/annuler`)
    }
  }

  // ── Dispatch des évènements SSE ─────────────────────────────

  const gererEvenement = (evt: EvenementAppel): void => {
    switch (evt.type) {
      case 'appel_entrant': {
        if (!evt.appelant) return
        // Occupé (déjà en appel ou en sonnerie) → refus automatique.
        if (appelActif.value || appelEntrant.value) {
          void postSilencieux(`/api/appels/${evt.appel_id}/refuser`)
          return
        }
        appelEntrant.value = { appel_id: evt.appel_id, appelant: evt.appelant, video: evt.video ?? false }
        break
      }
      case 'appel_accepte':
        if (appelActif.value?.appel_id === evt.appel_id) appelRepondu.value = true
        break
      case 'appel_refuse':
        if (appelActif.value?.appel_id === evt.appel_id) finAppel.value = 'refuse'
        break
      case 'appel_annule':
        // Sonnerie en cours → on la retire ; salle ouverte non connectée → fin.
        if (appelEntrant.value?.appel_id === evt.appel_id) appelEntrant.value = null
        if (appelActif.value?.appel_id === evt.appel_id) finAppel.value = 'annule'
        break
    }
  }

  return {
    appelEntrant,
    appelActif,
    finAppel,
    appelRepondu,
    erreurAppel,
    appeler,
    accepterAppel,
    refuserAppel,
    raccrocher,
    reinitialiser,
    gererEvenement,
  }
}

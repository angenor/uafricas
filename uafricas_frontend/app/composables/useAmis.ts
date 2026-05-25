// Composable du domaine social : demandes d'amitié, états de relation,
// notifications. User Stories 1 (envoyer) et 2 (répondre).
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces (alignés sur contracts/api.md)
// ──────────────────────────────────────────────────────────────

export type EtatRelation =
  | 'aucune'
  | 'demande_envoyee'
  | 'demande_recue'
  | 'amis'
  | 'bloque_par_moi'
  | 'indisponible'

/** DTO partagé MembreLight (champs publics uniquement) */
export interface MembreLightAPI {
  id: string
  nom: string
  prenom: string
  slug: string | null
  photoUrl: string | null
  fonction: string | null
  pays: string | null
}

export interface EtatRelationResponse {
  etat: EtatRelation
  demande_id?: string
}

export interface DemandeResponse {
  demande_id: string
  statut: string
}

export interface DemandeRecueAPI {
  demande_id: string
  demandeur: MembreLightAPI
  created_at: string
}

export interface DemandeEnvoyeeAPI {
  demande_id: string
  destinataire: MembreLightAPI
  created_at: string
}

export interface NotificationSocialeAPI {
  id: string
  type: 'demande_recue' | 'demande_acceptee'
  demande_id?: string
  acteur?: MembreLightAPI
  lu: boolean
  created_at: string
}

export interface AmiAPI {
  utilisateur: MembreLightAPI
  ami_depuis: string
}

export interface BlocageAPI {
  utilisateur: MembreLightAPI
  depuis: string
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useAmis = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)
  /** Code HTTP de la dernière erreur (409 doublon, 422 validation, 429 rate-limit, 403 blocage) */
  const codeErreur = ref<number | null>(null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  /** Normalise une erreur $fetch (statut + message) dans les refs réactifs. */
  const gererErreur = (e: any, contexte: string) => {
    codeErreur.value = e?.statusCode ?? e?.response?.status ?? null
    erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
    console.error(`Erreur ${contexte}:`, e)
  }

  // ── US1 ──────────────────────────────────────────────────────

  /**
   * Envoyer une demande d'amitié (FR-001). Renvoie la demande créée
   * (ou `{ statut: 'amis' }` en cas d'auto-acceptation croisée), ou `null`
   * en cas d'erreur — `codeErreur` est alors renseigné.
   */
  const envoyerDemande = async (destinataireId: string): Promise<DemandeResponse | null> => {
    chargement.value = true
    erreur.value = null
    codeErreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<DemandeResponse>>(
        `${apiBase}/api/amities/demandes`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { destinataire_id: destinataireId },
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l\'envoi de la demande')
      }
      return reponse.data
    }
    catch (e: any) {
      gererErreur(e, 'envoyerDemande')
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** État de la relation avec un membre (FR-016). */
  const obtenirEtatRelation = async (utilisateurId: string): Promise<EtatRelationResponse | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<EtatRelationResponse>>(
        `${apiBase}/api/amities/etat/${utilisateurId}`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement de l\'état')
      }
      return reponse.data
    }
    catch (e: any) {
      gererErreur(e, 'obtenirEtatRelation')
      return null
    }
  }

  /**
   * États relationnels en lot pour l'annuaire (FR-016, anti N+1).
   * Renvoie une map id -> état.
   */
  const obtenirEtatsRelationLot = async (
    ids: string[],
  ): Promise<Record<string, EtatRelation>> => {
    if (ids.length === 0) return {}
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<Record<string, EtatRelation>>>(
        `${apiBase}/api/amities/etats`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { utilisateur_ids: ids },
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des états')
      }
      return reponse.data
    }
    catch (e: any) {
      gererErreur(e, 'obtenirEtatsRelationLot')
      return {}
    }
  }

  // ── US2 ──────────────────────────────────────────────────────

  /** Demandes d'amitié reçues en attente (FR-006). */
  const listerDemandesRecues = async (): Promise<DemandeRecueAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<DemandeRecueAPI[]>>(
        `${apiBase}/api/amities/demandes/recues`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des demandes')
      }
      return reponse.data
    }
    catch (e: any) {
      gererErreur(e, 'listerDemandesRecues')
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /** Accepter une demande reçue (FR-007). */
  const accepterDemande = async (demandeId: string): Promise<boolean> => {
    erreur.value = null
    codeErreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/amities/demandes/${demandeId}/accepter`,
        { method: 'POST', headers: authHeaders() },
      )
      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors de l\'acceptation')
      }
      return true
    }
    catch (e: any) {
      gererErreur(e, 'accepterDemande')
      return false
    }
  }

  /** Refuser une demande reçue (FR-008). */
  const refuserDemande = async (demandeId: string): Promise<boolean> => {
    erreur.value = null
    codeErreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/amities/demandes/${demandeId}/refuser`,
        { method: 'POST', headers: authHeaders() },
      )
      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors du refus')
      }
      return true
    }
    catch (e: any) {
      gererErreur(e, 'refuserDemande')
      return false
    }
  }

  /** Notifications sociales de l'utilisateur (FR-017). */
  const listerNotifications = async (): Promise<NotificationSocialeAPI[]> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<NotificationSocialeAPI[]>>(
        `${apiBase}/api/amities/notifications`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des notifications')
      }
      return reponse.data
    }
    catch (e: any) {
      gererErreur(e, 'listerNotifications')
      return []
    }
  }

  /** Marquer une notification comme lue (FR-017). */
  const marquerNotificationLue = async (notificationId: string): Promise<boolean> => {
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/amities/notifications/${notificationId}/lu`,
        { method: 'PATCH', headers: authHeaders() },
      )
      return reponse.success
    }
    catch (e: any) {
      gererErreur(e, 'marquerNotificationLue')
      return false
    }
  }

  // ── US4 — Gérer ses relations & blocage ──────────────────────

  /** Demandes d'amitié envoyées en attente (FR-010). */
  const listerDemandesEnvoyees = async (): Promise<DemandeEnvoyeeAPI[]> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<DemandeEnvoyeeAPI[]>>(
        `${apiBase}/api/amities/demandes/envoyees`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des demandes envoyées')
      }
      return reponse.data
    }
    catch (e: any) {
      gererErreur(e, 'listerDemandesEnvoyees')
      return []
    }
  }

  /** Annuler une demande émise en attente (FR-010). */
  const annulerDemande = async (demandeId: string): Promise<boolean> => {
    erreur.value = null
    codeErreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/amities/demandes/${demandeId}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      return reponse.success
    }
    catch (e: any) {
      gererErreur(e, 'annulerDemande')
      return false
    }
  }

  /** Liste des amis de l'utilisateur courant (FR-011, privée). */
  const listerAmis = async (recherche?: string): Promise<AmiAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const qs = recherche?.trim() ? `?recherche=${encodeURIComponent(recherche.trim())}` : ''
      const reponse = await $fetch<ApiResponse<AmiAPI[]>>(
        `${apiBase}/api/amities${qs}`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des amis')
      }
      return reponse.data
    }
    catch (e: any) {
      gererErreur(e, 'listerAmis')
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /** Retirer un ami (FR-012). La conversation devient verrouillée (FR-025). */
  const retirerAmi = async (utilisateurId: string): Promise<boolean> => {
    erreur.value = null
    codeErreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/amities/${utilisateurId}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      return reponse.success
    }
    catch (e: any) {
      gererErreur(e, 'retirerAmi')
      return false
    }
  }

  /** Bloquer un membre (FR-013). Rompt amitié + demandes et verrouille la conversation. */
  const bloquer = async (utilisateurId: string): Promise<boolean> => {
    erreur.value = null
    codeErreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/blocages`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { utilisateur_id: utilisateurId },
        },
      )
      return reponse.success
    }
    catch (e: any) {
      gererErreur(e, 'bloquer')
      return false
    }
  }

  /** Débloquer un membre (FR-013). */
  const debloquer = async (utilisateurId: string): Promise<boolean> => {
    erreur.value = null
    codeErreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/blocages/${utilisateurId}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      return reponse.success
    }
    catch (e: any) {
      gererErreur(e, 'debloquer')
      return false
    }
  }

  /** Liste des membres bloqués par l'utilisateur courant (FR-013). */
  const listerBlocages = async (): Promise<BlocageAPI[]> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<BlocageAPI[]>>(
        `${apiBase}/api/blocages`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des membres bloqués')
      }
      return reponse.data
    }
    catch (e: any) {
      gererErreur(e, 'listerBlocages')
      return []
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    codeErreur: readonly(codeErreur),
    // US1
    envoyerDemande,
    obtenirEtatRelation,
    obtenirEtatsRelationLot,
    // US2
    listerDemandesRecues,
    accepterDemande,
    refuserDemande,
    listerNotifications,
    marquerNotificationLue,
    // US4
    listerDemandesEnvoyees,
    annulerDemande,
    listerAmis,
    retirerAmi,
    bloquer,
    debloquer,
    listerBlocages,
  }
}

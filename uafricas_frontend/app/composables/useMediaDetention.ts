// Co-détention des supports médias — chaînes et stations
// (US5 — migration 09m, handlers/media_detention.rs).
//
// Les rôles sont ordonnés : `proprietaire` ⊃ `co_detenteur` ⊃ `programmateur`.
// Cette hiérarchie est appliquée **côté serveur** par `garde_detenteur` ; elle
// n'est reproduite ici que pour n'afficher que les actions atteignables.

import type { PropositionMediaAPI } from '~/composables/useMediaProposition'

export type TypeSupportMedia = 'chaine_tv' | 'station_radio'
export type RoleDetenteur = 'proprietaire' | 'co_detenteur' | 'programmateur'
export type StatutInvitation = 'en_attente' | 'acceptee' | 'refusee' | 'expiree'

export const LIBELLES_ROLE_DETENTEUR: Record<RoleDetenteur, string> = {
  proprietaire: 'Propriétaire',
  co_detenteur: 'Co-détenteur',
  programmateur: 'Programmateur',
}

export const DESCRIPTIONS_ROLE_DETENTEUR: Record<RoleDetenteur, string> = {
  proprietaire: 'Tous les droits, plus l’invitation et la révocation des autres détenteurs.',
  co_detenteur: 'Éditer la fiche, gérer les contenus et établir la grille.',
  programmateur: 'Établir la grille de programmation, uniquement.',
}

export const LIBELLES_STATUT_INVITATION: Record<StatutInvitation, string> = {
  en_attente: 'En attente de réponse',
  acceptee: 'Acceptée',
  refusee: 'Refusée',
  expiree: 'Expirée',
}

/** Du plus étendu au moins étendu — miroir de `ROLES_DETENTEUR` (Rust). */
const ORDRE_ROLES: RoleDetenteur[] = ['proprietaire', 'co_detenteur', 'programmateur']

/** `true` si `role` est au moins aussi étendu que `minimal`. */
export const roleAuMoins = (role: RoleDetenteur | null | undefined, minimal: RoleDetenteur): boolean => {
  if (!role) return false
  const rang = ORDRE_ROLES.indexOf(role)
  const requis = ORDRE_ROLES.indexOf(minimal)
  return rang >= 0 && rang <= requis
}

export interface DetenteurAPI {
  id: string
  type_support: TypeSupportMedia
  support_id: string
  utilisateur_id: string
  role: RoleDetenteur
  designe_par: string | null
  designe_at: string
  actif: boolean
  retire_at: string | null
  utilisateur_nom: string | null
  utilisateur_prenom: string | null
  utilisateur_email: string | null
  utilisateur_photo: string | null
  /** Renseignés par « mes supports » uniquement. */
  support_nom: string | null
  support_slug: string | null
  support_image: string | null
}

export interface InvitationDetenteurAPI {
  id: string
  type_support: TypeSupportMedia
  support_id: string
  email_invite: string
  utilisateur_invite_id: string | null
  role: RoleDetenteur
  statut: StatutInvitation
  invite_par: string
  invite_par_nom: string | null
  invite_par_prenom: string | null
  support_nom: string | null
  support_slug: string | null
  created_at: string
  expire_at: string
  traitee_le: string | null
  /** Calculé à la lecture : aucune tâche de fond ne périme les invitations. */
  expiree: boolean
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export const useMediaDetention = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) return { Authorization: `Bearer ${userStore.accessToken}` }
    return {}
  }

  const messageErreur = (e: any, defaut: string): string =>
    e?.data?.error || e?.message || defaut

  /** Les détenteurs d'un support — visible dès le rôle `programmateur`. */
  const listerDetenteurs = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
  ): Promise<DetenteurAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<DetenteurAPI[]>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/detenteurs`,
        { headers: authHeaders() },
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur réseau')
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /** Les supports que je détiens — page « Mes supports ». */
  const mesSupports = async (): Promise<DetenteurAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<DetenteurAPI[]>>(
        `${apiBase}/api/medias/supports/moi`,
        { headers: authHeaders() },
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur réseau')
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Invitation par courriel — réservée au propriétaire.
   *
   * Le rôle `proprietaire` n'est pas invitable : un support n'en a qu'un, et
   * c'est l'auteur validé de la proposition (US4).
   */
  const inviter = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
    email: string,
    role: Exclude<RoleDetenteur, 'proprietaire'> = 'co_detenteur',
  ): Promise<{ id: string, destinataire_reconnu: boolean } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string, destinataire_reconnu: boolean }>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/invitations`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { email, role },
        },
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de l’envoi de l’invitation')
      return null
    }
  }

  /** Retrait — soft delete : l'historique de détention est conservé. */
  const retirer = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
    utilisateurId: string,
  ): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/detenteurs/${utilisateurId}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      return reponse.success
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors du retrait')
      return false
    }
  }

  const mesInvitations = async (): Promise<InvitationDetenteurAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<InvitationDetenteurAPI[]>>(
        `${apiBase}/api/medias/invitations/moi`,
        { headers: authHeaders() },
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur réseau')
      return []
    }
    finally {
      chargement.value = false
    }
  }

  const repondreInvitation = async (
    id: string,
    reponse: 'accepter' | 'refuser',
  ): Promise<boolean> => {
    erreur.value = null
    try {
      const r = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/medias/invitations/${id}/${reponse}`,
        { method: 'PATCH', headers: authHeaders() },
      )
      return r.success
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de la réponse à l’invitation')
      return false
    }
  }

  /**
   * Ouvre une conversation avec le détenteur du support (FR-046).
   *
   * La messagerie n'autorise normalement l'envoi qu'entre amis : seul cet
   * endpoint métier peut créer le canal.
   */
  const contacter = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
    message: string,
  ): Promise<{ conversation_id: string, destinataire_id: string } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ conversation_id: string, destinataire_id: string }>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/contacter`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { message },
        },
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de l’envoi du message')
      return null
    }
  }

  /**
   * Les idées et demandes d'animation visant un support, lisibles par ses
   * co-détenteurs (FR-047) — et non par les seuls administrateurs.
   */
  const listerPropositionsSupport = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
    statut?: string,
  ): Promise<PropositionMediaAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const qs = statut ? `?statut=${encodeURIComponent(statut)}` : ''
      const reponse = await $fetch<ApiResponse<PropositionMediaAPI[]>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/propositions${qs}`,
        { headers: authHeaders() },
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur réseau')
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Décision d'un co-détenteur sur une idée ou une demande d'animation.
   *
   * Une demande d'animation acceptée ajoute son auteur aux co-détenteurs du
   * support (FR-045). Un refus exige un motif d'au moins 10 caractères.
   */
  const deciderProposition = async (
    id: string,
    decision: 'accepter' | 'refuser',
    commentaire?: string,
  ): Promise<PropositionMediaAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<PropositionMediaAPI>>(
        `${apiBase}/api/medias/propositions/${id}/${decision}`,
        {
          method: 'PATCH',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { commentaire },
        },
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de la décision')
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerDetenteurs,
    mesSupports,
    inviter,
    retirer,
    mesInvitations,
    repondreInvitation,
    contacter,
    listerPropositionsSupport,
    deciderProposition,
  }
}

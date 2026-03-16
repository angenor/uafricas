// ════════════════════════════════════════════════════════════════════════════
// Composable — Collaboration et partage d'arbres
// ════════════════════════════════════════════════════════════════════════════

import { useUserStore } from '~/stores/user'
import type { MesArbresResponse, Permission } from '~/mocks/collaboration'

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export const useCollaboration = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const enTete = () => ({ Authorization: `Bearer ${userStore.accessToken}` })

  const mesArbres = async () =>
    $fetch<ApiResponse<MesArbresResponse>>(`${apiBase}/api/arbre/mes-arbres`, { headers: enTete() })

  const creerInvitation = async (email: string, permission: Permission) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/invitations`, { method: 'POST', headers: enTete(), body: { email, permission } })

  const listerInvitationsRecues = async () =>
    $fetch<ApiResponse<any[]>>(`${apiBase}/api/arbre/invitations`, { headers: enTete() })

  const accepterInvitation = async (id: string) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/invitations/${id}/accepter`, { method: 'POST', headers: enTete() })

  const refuserInvitation = async (id: string) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/invitations/${id}/refuser`, { method: 'POST', headers: enTete() })

  const listerCollaborateurs = async (arbreId: string) =>
    $fetch<ApiResponse<any[]>>(`${apiBase}/api/arbre/${arbreId}/collaborateurs`, { headers: enTete() })

  const modifierPermission = async (id: string, permission: Permission) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/collaborateurs/${id}`, { method: 'PUT', headers: enTete(), body: { permission } })

  const revoquerCollaborateur = async (id: string) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/collaborateurs/${id}`, { method: 'DELETE', headers: enTete() })

  const modifierConfidentialiteArbre = async (arbreId: string, prive: boolean) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/${arbreId}/confidentialite`, { method: 'PUT', headers: enTete(), body: { arbre_prive: prive } })

  const modifierConfidentialitePersonne = async (id: string, visible: boolean) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/personnes/${id}/confidentialite`, { method: 'PUT', headers: enTete(), body: { visible_matching: visible } })

  const obtenirHistorique = async (arbreId: string) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/${arbreId}/historique`, { headers: enTete() })

  return {
    mesArbres, creerInvitation, listerInvitationsRecues, accepterInvitation, refuserInvitation,
    listerCollaborateurs, modifierPermission, revoquerCollaborateur,
    modifierConfidentialiteArbre, modifierConfidentialitePersonne, obtenirHistorique,
  }
}

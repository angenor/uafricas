// ════════════════════════════════════════════════════════════════════════════
// Composable — Découvertes (matching inter-arbres)
// API wrapper pour les endpoints de matching
// ════════════════════════════════════════════════════════════════════════════

import { useUserStore } from '~/stores/user'
import type { DecouvertesResponse } from '~/mocks/matching'

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export const useDecouvertes = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const enTete = () => ({
    Authorization: `Bearer ${userStore.accessToken}`,
  })

  /** GET /api/arbre/decouvertes — liste toutes les suggestions par section */
  const listerDecouvertes = async () => {
    return $fetch<ApiResponse<DecouvertesResponse>>(`${apiBase}/api/arbre/decouvertes`, {
      headers: enTete(),
    })
  }

  /** POST /api/arbre/decouvertes/:id/confirmer */
  const confirmerSuggestion = async (id: string) => {
    return $fetch<ApiResponse<{ statut: string; message: string }>>(`${apiBase}/api/arbre/decouvertes/${id}/confirmer`, {
      method: 'POST',
      headers: enTete(),
    })
  }

  /** POST /api/arbre/decouvertes/:id/rejeter */
  const rejeterSuggestion = async (id: string) => {
    return $fetch<ApiResponse<{ message: string }>>(`${apiBase}/api/arbre/decouvertes/${id}/rejeter`, {
      method: 'POST',
      headers: enTete(),
    })
  }

  /** GET /api/arbre/decouvertes/:id/branches — arbre complet de l'autre utilisateur */
  const obtenirBranches = async (suggestionId: string) => {
    return $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/decouvertes/${suggestionId}/branches`, {
      headers: enTete(),
    })
  }

  /** POST /api/arbre/decouvertes/:suggestionId/demande-contact */
  const demanderContact = async (suggestionId: string) => {
    return $fetch<ApiResponse<{ id: string; statut: string }>>(`${apiBase}/api/arbre/decouvertes/${suggestionId}/demande-contact`, {
      method: 'POST',
      headers: enTete(),
    })
  }

  /** POST /api/arbre/demandes-contact/:id/accepter */
  const accepterDemande = async (id: string) => {
    return $fetch<ApiResponse<{ profil: any }>>(`${apiBase}/api/arbre/demandes-contact/${id}/accepter`, {
      method: 'POST',
      headers: enTete(),
    })
  }

  /** POST /api/arbre/demandes-contact/:id/refuser */
  const refuserDemande = async (id: string) => {
    return $fetch<ApiResponse<{ message: string }>>(`${apiBase}/api/arbre/demandes-contact/${id}/refuser`, {
      method: 'POST',
      headers: enTete(),
    })
  }

  /** GET /api/arbre/recherche-publique?q=... */
  const rechercherPublique = async (terme: string) => {
    return $fetch<ApiResponse<{ resultats: any[]; total: number }>>(`${apiBase}/api/arbre/recherche-publique`, {
      headers: enTete(),
      query: { q: terme },
    })
  }

  return {
    listerDecouvertes,
    confirmerSuggestion,
    rejeterSuggestion,
    obtenirBranches,
    demanderContact,
    accepterDemande,
    refuserDemande,
    rechercherPublique,
  }
}

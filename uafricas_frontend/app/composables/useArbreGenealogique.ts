// ════════════════════════════════════════════════════════════════════════════
// Composable — Arbre généalogique
// CRUD personnes + liens familiaux via l'API backend
// ════════════════════════════════════════════════════════════════════════════

import { useUserStore } from '~/stores/user'
import type {
  PersonneDetail,
  PersonneListe,
  CreerPersonneForm,
  ModifierPersonneForm,
  CreerLienForm,
  PersonneQueryParams,
  LienFamilialResponse,
  ArbreComplet,
} from '~/mocks/arbre-genealogique'

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export interface LienFamilialResponse {
  id: string
  type_lien: string
  rattachement_source_id: string
  rattachement_cible_id: string
  created_at: string
}

export const useArbreGenealogique = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const enTete = () => ({
    Authorization: `Bearer ${userStore.accessToken}`,
  })

  /** Wrapper qui retente avec refresh token en cas de 401 */
  const fetchAvecRetry = async <T>(url: string, opts: any): Promise<T> => {
    try {
      return await $fetch<T>(url, { ...opts, headers: enTete() })
    } catch (e: any) {
      if (e?.statusCode === 401 || e?.status === 401) {
        const refreshToken = userStore.loadRefreshToken()
        if (refreshToken) {
          const { refreshAccessToken } = useAuth()
          await refreshAccessToken()
          return await $fetch<T>(url, { ...opts, headers: enTete() })
        }
      }
      throw e
    }
  }

  // ── Personnes ─────────────────────────────────────────────────────────

  /** GET /api/arbre/personnes — liste paginée avec recherche optionnelle */
  const listerPersonnes = async (params?: PersonneQueryParams) => {
    return fetchAvecRetry<ApiResponse<PersonneListe>>(`${apiBase}/api/arbre/personnes`, { query: params })
  }

  /** GET /api/arbre/personnes/:id — fiche complète avec liens */
  const obtenirPersonne = async (id: string) => {
    return fetchAvecRetry<ApiResponse<PersonneDetail>>(`${apiBase}/api/arbre/personnes/${id}`, {})
  }

  /** POST /api/arbre/personnes — créer une personne (crée l'arbre si nécessaire) */
  const creerPersonne = async (form: CreerPersonneForm) => {
    return fetchAvecRetry<ApiResponse<PersonneDetail>>(`${apiBase}/api/arbre/personnes`, { method: 'POST', body: form })
  }

  /** PUT /api/arbre/personnes/:id — modifier les informations */
  const modifierPersonne = async (id: string, form: ModifierPersonneForm) => {
    return fetchAvecRetry<ApiResponse<PersonneDetail>>(`${apiBase}/api/arbre/personnes/${id}`, { method: 'PUT', body: form })
  }

  /** DELETE /api/arbre/personnes/:id — supprimer (soft delete + cascade) */
  const supprimerPersonne = async (id: string) => {
    return fetchAvecRetry<ApiResponse<{ message: string }>>(`${apiBase}/api/arbre/personnes/${id}`, { method: 'DELETE' })
  }

  /** POST /api/arbre/personnes/:id/photo — upload photo multipart */
  const uploaderPhoto = async (id: string, fichier: File) => {
    const formData = new FormData()
    formData.append('photo', fichier)
    return $fetch<ApiResponse<{ photo_url: string }>>(`${apiBase}/api/arbre/personnes/${id}/photo`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${userStore.accessToken}` },
      body: formData,
    })
  }

  // ── Arbre complet (visualisation) ────────────────────────────────────

  /** GET /api/arbre/arbre-complet — toutes personnes + tous liens */
  const obtenirArbreComplet = async () => {
    return fetchAvecRetry<ApiResponse<ArbreComplet>>(`${apiBase}/api/arbre/arbre-complet`, {})
  }

  // ── Liens familiaux ───────────────────────────────────────────────────

  /** POST /api/arbre/liens — créer un lien familial */
  const creerLien = async (form: CreerLienForm) => {
    return fetchAvecRetry<ApiResponse<LienFamilialResponse>>(`${apiBase}/api/arbre/liens`, { method: 'POST', body: form })
  }

  /** DELETE /api/arbre/liens/:id — supprimer un lien */
  const supprimerLien = async (id: string) => {
    return fetchAvecRetry<ApiResponse<{ message: string }>>(`${apiBase}/api/arbre/liens/${id}`, { method: 'DELETE' })
  }

  return {
    listerPersonnes,
    obtenirPersonne,
    obtenirArbreComplet,
    creerPersonne,
    modifierPersonne,
    supprimerPersonne,
    uploaderPhoto,
    creerLien,
    supprimerLien,
  }
}

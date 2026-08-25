// ════════════════════════════════════════════════════════════════════════════
// Composable : Notifications
// ════════════════════════════════════════════════════════════════════════════

import { ref } from 'vue'
import { useUserStore } from '~/stores/user'
import type { FusionDoublonDto } from '~/mocks/notifications'

interface ApiResponse<T> { success: boolean; data: T | null; error: string | null }

const nbNonLues = ref(0)

export const useNotifications = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()
  const enTete = () => ({ Authorization: `Bearer ${userStore.accessToken}` })

  const compteurNonLues = async () => {
    try {
      const r = await $fetch<ApiResponse<{ non_lues: number }>>(`${apiBase}/api/notifications/compteur`, { headers: enTete() })
      if (r.success && r.data) nbNonLues.value = r.data.non_lues
    } catch {}
    return nbNonLues.value
  }

  const listerNotifications = async (page = 1, type?: string) =>
    $fetch<ApiResponse<any[]>>(`${apiBase}/api/notifications`, { headers: enTete(), query: { page, type } })

  const marquerLue = async (id: string) => {
    const r = await $fetch<ApiResponse<any>>(`${apiBase}/api/notifications/${id}/lire`, { method: 'POST', headers: enTete() })
    if (r.success) nbNonLues.value = Math.max(0, nbNonLues.value - 1)
    return r
  }

  const toutMarquerLu = async () => {
    const r = await $fetch<ApiResponse<any>>(`${apiBase}/api/notifications/tout-lire`, { method: 'POST', headers: enTete() })
    if (r.success) nbNonLues.value = 0
    return r
  }

  const detecterDoublons = async () =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/doublons`, { headers: enTete() })

  const ignorerDoublon = async (a: string, b: string) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/doublons/ignorer`, { method: 'POST', headers: enTete(), body: { personne_a_id: a, personne_b_id: b } })

  const fusionnerDoublons = async (dto: FusionDoublonDto) =>
    $fetch<ApiResponse<any>>(`${apiBase}/api/arbre/doublons/fusionner`, { method: 'POST', headers: enTete(), body: dto })

  return {
    nbNonLues, compteurNonLues, listerNotifications, marquerLue, toutMarquerLu,
    detecterDoublons, ignorerDoublon, fusionnerDoublons,
  }
}

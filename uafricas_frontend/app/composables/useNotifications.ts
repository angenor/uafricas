// ════════════════════════════════════════════════════════════════════════════
// Composable : Notifications
// ════════════════════════════════════════════════════════════════════════════

import { ref } from 'vue'
import { useUserStore } from '~/stores/user'
import type { FusionDoublonDto } from '~/mocks/notifications'

interface ApiResponse<T> { success: boolean; data: T | null; error: string | null }

/**
 * Forme commune servie à la cloche. `source` dit à quel endpoint renvoyer le
 * marquage « lu » : la plateforme tient DEUX magasins de notifications, dont
 * les routes et les verbes diffèrent.
 */
export interface NotificationUnifiee {
  id: string
  type_: string
  message: string
  lien_action: string | null
  lu: boolean
  created_at: string
  source: 'arbre' | 'africonnect'
}

/**
 * Les notifications d'Africonnect ne portent NI message NI lien : la table
 * `retrouve_amis.notification_retrouve` ne stocke qu'un type et l'identifiant
 * de la correspondance. La phrase et la destination sont donc composées ici.
 */
const LIBELLES_AFRICONNECT: Record<string, string> = {
  nouvelle_correspondance: 'Une correspondance a été trouvée pour un de vos avis de recherche.',
  reponse_publique: 'Quelqu\'un a répondu à votre avis de recherche.',
  acceptation_contact: 'Votre demande de contact a été acceptée.',
  coordonnees_partagees: 'Des coordonnées viennent de vous être partagées.',
  correspondance_archivee: 'Une correspondance a été archivée.',
  avis_suspendu: 'Un de vos avis de recherche a été suspendu.',
  demande_retrait: 'Une demande de retrait vise un de vos avis de recherche.',
}

const nbNonLues = ref(0)

export const useNotifications = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()
  const enTete = () => ({ Authorization: `Bearer ${userStore.accessToken}` })

  /**
   * Le compteur additionne les DEUX magasins. La cloche n'en lisait qu'un :
   * une réponse à un avis de recherche créait bien sa notification en base,
   * mais rien dans l'interface ne la montrait jamais.
   */
  const compteurNonLues = async () => {
    const [arbre, africonnect] = await Promise.allSettled([
      $fetch<ApiResponse<{ non_lues: number }>>(`${apiBase}/api/notifications/compteur`, { headers: enTete() }),
      $fetch<ApiResponse<{ notifications_non_lues: number }>>(`${apiBase}/api/retrouve-amis/tableau-de-bord`, { headers: enTete() }),
    ])
    let total = 0
    if (arbre.status === 'fulfilled' && arbre.value.data) total += arbre.value.data.non_lues
    if (africonnect.status === 'fulfilled' && africonnect.value.data) {
      total += africonnect.value.data.notifications_non_lues ?? 0
    }
    nbNonLues.value = total
    return total
  }

  /**
   * Les deux listes sont fusionnées puis triées par date : l'utilisateur n'a
   * pas à savoir de quel module vient une notification pour la trouver.
   * `allSettled` : qu'un magasin tombe ne doit pas vider l'autre.
   */
  const listerNotifications = async (page = 1, type?: string): Promise<ApiResponse<NotificationUnifiee[]>> => {
    const [arbre, africonnect] = await Promise.allSettled([
      $fetch<ApiResponse<any[]>>(`${apiBase}/api/notifications`, { headers: enTete(), query: { page, type } }),
      $fetch<ApiResponse<{ notifications: any[] }>>(`${apiBase}/api/retrouve-amis/notifications`, { headers: enTete(), query: { page, par_page: 20 } }),
    ])

    const items: NotificationUnifiee[] = []

    if (arbre.status === 'fulfilled' && arbre.value.data) {
      for (const n of arbre.value.data) {
        items.push({ ...n, source: 'arbre' })
      }
    }

    if (africonnect.status === 'fulfilled' && africonnect.value.data?.notifications) {
      for (const n of africonnect.value.data.notifications) {
        items.push({
          id: n.id,
          type_: n.type_notif,
          message: LIBELLES_AFRICONNECT[n.type_notif] ?? 'Nouvelle activité sur vos avis de recherche.',
          // Sans correspondance rattachée, on renvoie vers la liste : un lien
          // vers un détail inexistant vaut moins qu'un lien vers l'ensemble.
          lien_action: n.correspondance_id
            ? `/retrouve-amis/correspondances/${n.correspondance_id}`
            : '/retrouve-amis/correspondances',
          lu: n.lu,
          created_at: n.created_at,
          source: 'africonnect',
        })
      }
    }

    items.sort((a, b) => +new Date(b.created_at) - +new Date(a.created_at))
    return { success: true, data: items, error: null }
  }

  /** Le verbe et la route diffèrent d'un magasin à l'autre : POST …/lire ici, PATCH …/lu là. */
  const marquerLue = async (id: string, source: 'arbre' | 'africonnect' = 'arbre') => {
    const r = source === 'africonnect'
      ? await $fetch<ApiResponse<any>>(`${apiBase}/api/retrouve-amis/notifications/${id}/lu`, { method: 'PATCH', headers: enTete() })
      : await $fetch<ApiResponse<any>>(`${apiBase}/api/notifications/${id}/lire`, { method: 'POST', headers: enTete() })
    if (r.success) nbNonLues.value = Math.max(0, nbNonLues.value - 1)
    return r
  }

  const toutMarquerLu = async () => {
    const [a, b] = await Promise.allSettled([
      $fetch<ApiResponse<any>>(`${apiBase}/api/notifications/tout-lire`, { method: 'POST', headers: enTete() }),
      $fetch<ApiResponse<any>>(`${apiBase}/api/retrouve-amis/notifications/tout-lu`, { method: 'PATCH', headers: enTete() }),
    ])
    nbNonLues.value = 0
    return { success: a.status === 'fulfilled' || b.status === 'fulfilled', data: null, error: null }
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

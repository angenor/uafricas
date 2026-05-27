// Composable Afrolang : workflow accompagnateur — consentement explicite
// (feature 001-ressources-fermeture-session, US1)
import { useUserStore } from '~/stores/user'
import type {
  AuteurLight,
  RessourceContribueeAPI,
  StatutAccompagnateur,
} from '~/composables/useAfrolangRessources'

export interface SalleLightAPI {
  id: string
  titre: string
  groupe_ethnique_nom: string | null
}

export interface RecommandationRecueAPI {
  id: string
  salle: SalleLightAPI
  auteur: AuteurLight
  motif_recommandation: string
  statut_accompagnateur: StatutAccompagnateur
  created_at: string
  reponse_at: string | null
}

export interface RecommandationsFiltres {
  statut?: StatutAccompagnateur
  page?: number
  par_page?: number
}

interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

function extraireMessage(e: unknown, fallback: string): string {
  if (e instanceof Error) return e.message
  if (typeof e === 'object' && e !== null && 'data' in e) {
    const d = (e as { data?: { error?: string } }).data
    if (d?.error) return d.error
  }
  return fallback
}

export const useAfrolangAccompagnateur = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  // État partagé via useState — badge NavBar
  const mesRecommandationsEnAttente = useState<number>(
    'afrolang.mesRecommandationsEnAttente',
    () => 0,
  )

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  const listerRecommandationsRecues = async (
    filtres: RecommandationsFiltres = {},
  ): Promise<PaginatedResponse<RecommandationRecueAPI> | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const params = new URLSearchParams()
      if (filtres.statut) params.set('statut', filtres.statut)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))
      const qs = params.toString()
      const url = `${apiBase}/api/afrolang/accompagnateur/recommandations-recues${qs ? `?${qs}` : ''}`
      const reponse = await $fetch<ApiResponse<PaginatedResponse<RecommandationRecueAPI>>>(url, {
        headers: authHeaders(),
      })
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement')
      }
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur réseau')
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Rafraîchit le compteur de recommandations en attente (pour le badge NavBar). */
  const rafraichirCompteur = async (): Promise<void> => {
    if (!userStore.accessToken) {
      mesRecommandationsEnAttente.value = 0
      return
    }
    try {
      const reponse = await $fetch<ApiResponse<PaginatedResponse<RecommandationRecueAPI>>>(
        `${apiBase}/api/afrolang/accompagnateur/recommandations-recues?statut=en_attente&par_page=1`,
        { headers: authHeaders() },
      )
      if (reponse.success && reponse.data) {
        mesRecommandationsEnAttente.value = reponse.data.total
      }
    }
    catch {
      // silencieux : badge non bloquant
    }
  }

  const accepter = async (id: string): Promise<RessourceContribueeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<RessourceContribueeAPI>>(
        `${apiBase}/api/afrolang/ressources-contribuees/${id}/accepter`,
        { method: 'POST', headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || "Erreur lors de l'acceptation")
      }
      mesRecommandationsEnAttente.value = Math.max(0, mesRecommandationsEnAttente.value - 1)
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur réseau')
      return null
    }
    finally {
      chargement.value = false
    }
  }

  const refuser = async (
    id: string,
    motifRefus?: string | null,
  ): Promise<RessourceContribueeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<RessourceContribueeAPI>>(
        `${apiBase}/api/afrolang/ressources-contribuees/${id}/refuser`,
        {
          method: 'POST',
          body: { motif_refus: motifRefus || null },
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du refus')
      }
      mesRecommandationsEnAttente.value = Math.max(0, mesRecommandationsEnAttente.value - 1)
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur réseau')
      return null
    }
    finally {
      chargement.value = false
    }
  }

  const retirerConsentement = async (id: string): Promise<RessourceContribueeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<RessourceContribueeAPI>>(
        `${apiBase}/api/afrolang/ressources-contribuees/${id}/retirer-consentement`,
        { method: 'POST', headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du retrait')
      }
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur réseau')
      return null
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement,
    erreur,
    mesRecommandationsEnAttente,
    listerRecommandationsRecues,
    rafraichirCompteur,
    accepter,
    refuser,
    retirerConsentement,
  }
}

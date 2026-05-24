// Composable Afrolang : ressources contribuées par la communauté
// (feature 001-ressources-fermeture-session, US1)
import { useUserStore } from '~/stores/user'

// ── Types alignés sur les structs Rust src/models/ressource_contribuee.rs ──

export type TypeRessourceContribuee = 'document' | 'video_youtube' | 'accompagnateur' | 'lien_web'
export type StatutAccompagnateur = 'en_attente' | 'acceptee' | 'refusee' | 'retiree'

export interface AuteurLight {
  id: string
  nom: string
  prenom: string
  avatar_url: string | null
}

export interface AccompagnateurPublicInfo {
  membre: AuteurLight
  motif: string
  statut: StatutAccompagnateur
}

export interface RessourceContribueeAPI {
  id: string
  type: TypeRessourceContribuee
  titre: string
  description: string | null
  auteur: AuteurLight
  session_origine_id: string | null
  fichier_url: string | null
  fichier_taille_octets: number | null
  video_id_youtube: string | null
  video_url: string | null
  lien_url: string | null
  accompagnateur: AccompagnateurPublicInfo | null
  created_at: string
}

export interface RessourcesContribueesFiltres {
  page?: number
  par_page?: number
  type?: TypeRessourceContribuee
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

// Payloads d'écriture
export interface AjouterVideoYoutubePayload {
  type: 'video_youtube'
  titre: string
  description?: string | null
  session_origine_id?: string | null
  video_url: string
}

export interface AjouterLienWebPayload {
  type: 'lien_web'
  titre: string
  description?: string | null
  session_origine_id?: string | null
  lien_url: string
}

export interface RecommanderAccompagnateurPayload {
  type: 'accompagnateur'
  titre: string
  description?: string | null
  session_origine_id?: string | null
  membre_recommande_id: string
  motif_recommandation: string
}

function extraireMessage(e: unknown, fallback: string): string {
  if (e instanceof Error) return e.message
  if (typeof e === 'object' && e !== null && 'data' in e) {
    const d = (e as { data?: { error?: string } }).data
    if (d?.error) return d.error
  }
  return fallback
}

export const useAfrolangRessources = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  const resoudreUrl = (url: string | null): string | null => {
    if (!url) return null
    if (url.startsWith('http')) return url
    return `${apiBase}${url}`
  }

  const listerRessourcesContribuees = async (
    salleId: string,
    filtres: RessourcesContribueesFiltres = {},
  ): Promise<PaginatedResponse<RessourceContribueeAPI> | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const params = new URLSearchParams()
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))
      if (filtres.type) params.set('type', filtres.type)
      const qs = params.toString()
      const url = `${apiBase}/api/afrolang/salles/${salleId}/ressources-contribuees${qs ? `?${qs}` : ''}`
      const reponse = await $fetch<ApiResponse<PaginatedResponse<RessourceContribueeAPI>>>(url, {
        headers: authHeaders(),
      })
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des ressources')
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

  const ajouterDocument = async (
    salleId: string,
    formData: FormData,
  ): Promise<RessourceContribueeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<RessourceContribueeAPI>>(
        `${apiBase}/api/afrolang/salles/${salleId}/ressources-contribuees`,
        { method: 'POST', body: formData, headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || "Erreur lors de l'envoi du document")
      }
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, "Erreur lors de l'envoi du document")
      return null
    }
    finally {
      chargement.value = false
    }
  }

  const ajouterVideoYoutube = async (
    salleId: string,
    payload: AjouterVideoYoutubePayload,
  ): Promise<RessourceContribueeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<RessourceContribueeAPI>>(
        `${apiBase}/api/afrolang/salles/${salleId}/ressources-contribuees`,
        {
          method: 'POST',
          body: payload,
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || "Erreur lors de l'ajout de la vidéo")
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

  const ajouterLienWeb = async (
    salleId: string,
    payload: AjouterLienWebPayload,
  ): Promise<RessourceContribueeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<RessourceContribueeAPI>>(
        `${apiBase}/api/afrolang/salles/${salleId}/ressources-contribuees`,
        {
          method: 'POST',
          body: payload,
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || "Erreur lors de l'ajout du lien")
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

  const recommanderAccompagnateur = async (
    salleId: string,
    payload: RecommanderAccompagnateurPayload,
  ): Promise<RessourceContribueeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<RessourceContribueeAPI>>(
        `${apiBase}/api/afrolang/salles/${salleId}/ressources-contribuees`,
        {
          method: 'POST',
          body: payload,
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la recommandation')
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

  const supprimerRessource = async (id: string): Promise<boolean> => {
    chargement.value = true
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/afrolang/ressources-contribuees/${id}`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
      return true
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de la suppression')
      return false
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement,
    erreur,
    resoudreUrl,
    listerRessourcesContribuees,
    ajouterDocument,
    ajouterVideoYoutube,
    ajouterLienWeb,
    recommanderAccompagnateur,
    supprimerRessource,
  }
}

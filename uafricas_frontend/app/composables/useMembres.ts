// Composable pour l'annuaire public des membres de la plateforme
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

/** DTO correspondant a MembreResponse du backend */
export interface MembreAPI {
  id: string
  nom: string
  prenom: string
  slug: string | null
  photoUrl: string | null
  fonction: string | null
  pays: string | null
  ville: string | null
  biographie: string | null
  roles: string[]
  dateInscription: string
  estExpert: boolean
  estBiblio: boolean
}

/** Reponse paginee */
export interface MembreListeAPI {
  membres: MembreAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface MembreFiltres {
  recherche?: string
  /** "expert" | "biblio" */
  type?: string
  pays?: string
  page?: number
  par_page?: number
}

// ── Partage de profil ───────────────────────────────────────────

export interface PartageProfilAuteurAPI {
  id: string
  nom: string
  prenom: string
  photo_url: string | null
}

export interface PartageProfilApercuAPI {
  id: string
  nom: string
  prenom: string
  photo_url: string | null
  fonction: string | null
  pays: string | null
  ville: string | null
  est_expert: boolean
  est_biblio: boolean
}

export interface PartageProfilAPI {
  id: string
  legende: string | null
  created_at: string
  profil: PartageProfilApercuAPI
  auteur: PartageProfilAuteurAPI
}

export interface PartageProfilListeAPI {
  partages: PartageProfilAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

// ── Signalement de profil ───────────────────────────────────────

export interface SignalementProfilEtatAPI {
  nombre_signalements: number
  etat: string
  deja_signale: boolean
  suspendu: boolean
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useMembres = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** Headers d'authentification si l'utilisateur est connecte */
  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  /**
   * Lister les membres avec filtres et pagination
   */
  const listerMembres = async (filtres: MembreFiltres = {}): Promise<MembreListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.type) params.set('type', filtres.type)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/utilisateurs${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<MembreListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des membres')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerMembres:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir un membre par son ID
   */
  const obtenirMembre = async (id: string): Promise<MembreAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<MembreAPI>>(
        `${apiBase}/api/utilisateurs/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Membre non trouve')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirMembre:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Partager un profil sur le mur communautaire (/publications). JWT requis.
   */
  const partagerProfil = async (
    profilId: string,
    legende?: string,
  ): Promise<PartageProfilAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<PartageProfilAPI>>(
        `${apiBase}/api/utilisateurs/${profilId}/partages`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { legende: legende || undefined },
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du partage')
      }
      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur partagerProfil:', e)
      throw new Error(message)
    }
  }

  /**
   * Liste paginée des profils partagés (mur communautaire, public).
   */
  const listerPartagesProfils = async (
    page = 1,
    parPage = 20,
  ): Promise<PartageProfilListeAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<PartageProfilListeAPI>>(
        `${apiBase}/api/utilisateurs/partages?page=${page}&par_page=${parPage}`,
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des partages')
      }
      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerPartagesProfils:', e)
      return null
    }
  }

  /**
   * Signaler un profil (faux profil / arnaque). JWT requis.
   */
  const signalerProfil = async (
    profilId: string,
    motif?: string,
    description?: string,
  ): Promise<SignalementProfilEtatAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<SignalementProfilEtatAPI>>(
        `${apiBase}/api/utilisateurs/${profilId}/signalement`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { motif: motif || undefined, description: description || undefined },
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du signalement')
      }
      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur signalerProfil:', e)
      throw new Error(message)
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerMembres,
    obtenirMembre,
    partagerProfil,
    listerPartagesProfils,
    signalerProfil,
  }
}

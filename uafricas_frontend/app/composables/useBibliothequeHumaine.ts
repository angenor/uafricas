// Composable pour les appels API Bibliotheques Humaines
import { useUserStore } from '~/stores/user'
import { demandesBiblioHumaineReactif, type DemandeBiblioHumaine } from '~/mocks/bibliotheques-humaines'

export type { DemandeBiblioHumaine }

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

/** DTO correspondant a BiblioHumaineResponse du backend */
export interface BiblioHumaineAPI {
  id: string
  userId: string
  nom: string
  prenom: string
  photoUrl: string | null
  fonction: string
  pays: string
  specialite: string
  specialites: string[]
  biographie: string
  ville: string | null
  dateInscription: string
}

/** Reponse paginee */
export interface BiblioHumaineListeAPI {
  bibliotheques: BiblioHumaineAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Specialite */
export interface SpecialiteAPI {
  id: string
  nom: string
  slug: string
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface BiblioHumaineFiltres {
  recherche?: string
  specialite?: string
  pays?: string
  page?: number
  par_page?: number
}

/** Body pour l'inscription */
export interface InscriptionBiblioBody {
  specialites: string[]
  biographie: string
  fonction: string
  pays?: string
  organisation?: string
}

/** Statut de la demande de l'utilisateur courant (US4) */
export interface MaDemandeAPI {
  id: string
  statut: 'en_attente' | 'valide' | 'rejete'
  fonction: string
  biographie: string
  pays: string | null
  specialites: string[]
  commentaireAdmin: string | null
  createdAt: string
  traiteLe: string | null
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useBibliothequeHumaine = () => {
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
   * Lister les bibliotheques humaines avec filtres et pagination
   */
  const listerBiblios = async (filtres: BiblioHumaineFiltres = {}): Promise<BiblioHumaineListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.specialite && filtres.specialite !== 'Tous') params.set('specialite', filtres.specialite)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/bibliotheques-humaines${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<BiblioHumaineListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des bibliotheques humaines')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerBiblios:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir une bibliotheque humaine par son ID
   */
  const obtenirBiblio = async (id: string): Promise<BiblioHumaineAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<BiblioHumaineAPI>>(
        `${apiBase}/api/bibliotheques-humaines/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Bibliotheque humaine non trouvee')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirBiblio:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * S'inscrire comme bibliotheque humaine (JWT requis)
   * Enregistre aussi la demande dans le store mock (statut en_attente)
   */
  const inscrireBiblioHumaine = async (body: InscriptionBiblioBody): Promise<BiblioHumaineAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<BiblioHumaineAPI>>(
        `${apiBase}/api/bibliotheques-humaines/inscription`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...authHeaders(),
          },
          body,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l\'inscription')
      }

      // Synchroniser avec le store mock pour le suivi de statut (US4)
      const now = new Date().toISOString()
      demandesBiblioHumaineReactif.value = [
        ...demandesBiblioHumaineReactif.value.filter(d => d.userId !== userStore.user?.id),
        {
          id: reponse.data.id,
          userId: userStore.user?.id ?? '',
          nom: userStore.user?.nom ?? '',
          prenom: userStore.user?.prenom ?? '',
          photoUrl: null,
          fonction: body.fonction,
          pays: body.pays ?? '',
          biographie: body.biographie,
          specialites: body.specialites,
          statut: 'en_attente',
          dateSubmission: now,
          dateMaj: null,
          commentaireAdmin: null,
        },
      ]

      return reponse.data
    }
    catch (e: any) {
      // Si le backend n'est pas disponible, simuler en mode mock
      if (!userStore.user) {
        const message = 'Authentification requise'
        erreur.value = message
        return null
      }
      const now = new Date().toISOString()
      const mockId = `bh-new-${Date.now()}`
      demandesBiblioHumaineReactif.value = [
        ...demandesBiblioHumaineReactif.value.filter(d => d.userId !== userStore.user!.id),
        {
          id: mockId,
          userId: userStore.user.id,
          nom: userStore.user.nom,
          prenom: userStore.user.prenom,
          photoUrl: null,
          fonction: body.fonction,
          pays: body.pays ?? '',
          biographie: body.biographie,
          specialites: body.specialites,
          statut: 'en_attente',
          dateSubmission: now,
          dateMaj: null,
          commentaireAdmin: null,
        },
      ]
      return {
        id: mockId,
        userId: userStore.user.id,
        nom: userStore.user.nom,
        prenom: userStore.user.prenom,
        photoUrl: null,
        fonction: body.fonction,
        pays: body.pays ?? '',
        specialite: body.specialites[0] ?? '',
        specialites: body.specialites,
        biographie: body.biographie,
        ville: null,
        dateInscription: now,
      }
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Recuperer la demande de l'utilisateur courant via l'API (suivi de statut — US4)
   */
  const obtenirMaDemande = async (): Promise<MaDemandeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<MaDemandeAPI>>(
        `${apiBase}/api/bibliotheques-humaines/moi/demande`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        return null
      }
      return reponse.data
    }
    catch (e: any) {
      if (e?.status === 404) return null
      erreur.value = e?.data?.error ?? e?.message ?? 'Erreur réseau'
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Lister les specialites disponibles
   */
  const listerSpecialites = async (): Promise<SpecialiteAPI[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<SpecialiteAPI[]>>(
        `${apiBase}/api/bibliotheques-humaines/specialites`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des specialites')
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerSpecialites:', e)
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerBiblios,
    obtenirBiblio,
    inscrireBiblioHumaine,
    listerSpecialites,
    obtenirMaDemande,
  }
}

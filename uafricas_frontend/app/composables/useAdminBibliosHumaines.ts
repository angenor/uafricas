import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types
// ──────────────────────────────────────────────────────────────

export interface AdminDemandeBiblio {
  id: string
  utilisateurId: string
  nom: string
  prenom: string
  email: string
  photoUrl: string | null
  fonction: string
  statut: 'en_attente' | 'valide' | 'rejete'
  specialites: string[]
  createdAt: string
}

export interface AdminDemandeBiblioDetail extends AdminDemandeBiblio {
  biographie: string
  pays: string | null
  commentaireAdmin: string | null
  traiteLe: string | null
  traiteParNom: string | null
}

interface AdminDemandeBiblioListeAPI {
  demandes: AdminDemandeBiblio[]
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

export interface FiltresDemandes {
  statut?: 'en_attente' | 'valide' | 'rejete' | ''
  recherche?: string
  page?: number
  par_page?: number
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export function useAdminBibliosHumaines() {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)
  const demandes = ref<AdminDemandeBiblio[]>([])
  const demandeDetail = ref<AdminDemandeBiblioDetail | null>(null)
  const total = ref(0)
  const page = ref(1)
  const totalPages = ref(1)

  const nbEnAttente = computed(
    () => demandes.value.filter(d => d.statut === 'en_attente').length,
  )

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  async function listerDemandes(filtres: FiltresDemandes = {}): Promise<void> {
    chargement.value = true
    erreur.value = null
    try {
      const params = new URLSearchParams()
      if (filtres.statut) params.set('statut', filtres.statut)
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const qs = params.toString()
      const url = `${apiBase}/api/admin/bibliotheques-humaines${qs ? `?${qs}` : ''}`

      const reponse = await $fetch<ApiResponse<AdminDemandeBiblioListeAPI>>(url, {
        headers: authHeaders(),
      })

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error ?? 'Erreur lors du chargement des demandes')
      }

      demandes.value = reponse.data.demandes
      total.value = reponse.data.total
      page.value = reponse.data.page
      totalPages.value = reponse.data.total_pages
    }
    catch (e: unknown) {
      erreur.value = (e as any)?.data?.error ?? (e as Error)?.message ?? 'Erreur réseau'
    }
    finally {
      chargement.value = false
    }
  }

  async function obtenirDemande(id: string): Promise<void> {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<AdminDemandeBiblioDetail>>(
        `${apiBase}/api/admin/bibliotheques-humaines/${id}`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error ?? 'Demande introuvable')
      }

      demandeDetail.value = reponse.data
    }
    catch (e: unknown) {
      erreur.value = (e as any)?.data?.error ?? (e as Error)?.message ?? 'Erreur réseau'
      throw e
    }
    finally {
      chargement.value = false
    }
  }

  async function validerDemande(id: string): Promise<void> {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string, statut: string }>>(
        `${apiBase}/api/admin/bibliotheques-humaines/${id}/valider`,
        { method: 'PATCH', headers: authHeaders(), body: {} },
      )

      if (!reponse.success) {
        throw new Error(reponse.error ?? 'Erreur lors de la validation')
      }

      demandes.value = demandes.value.map(d =>
        d.id === id ? { ...d, statut: 'valide' as const } : d,
      )
      if (demandeDetail.value?.id === id) {
        demandeDetail.value = { ...demandeDetail.value, statut: 'valide' }
      }
    }
    catch (e: unknown) {
      erreur.value = (e as any)?.data?.error ?? (e as Error)?.message ?? 'Erreur réseau'
      throw e
    }
    finally {
      chargement.value = false
    }
  }

  async function rejeterDemande(id: string, commentaire?: string): Promise<void> {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string, statut: string }>>(
        `${apiBase}/api/admin/bibliotheques-humaines/${id}/rejeter`,
        {
          method: 'PATCH',
          headers: authHeaders(),
          body: { commentaire: commentaire || null },
        },
      )

      if (!reponse.success) {
        throw new Error(reponse.error ?? 'Erreur lors du rejet')
      }

      demandes.value = demandes.value.map(d =>
        d.id === id ? { ...d, statut: 'rejete' as const } : d,
      )
      if (demandeDetail.value?.id === id) {
        demandeDetail.value = {
          ...demandeDetail.value,
          statut: 'rejete',
          commentaireAdmin: commentaire || null,
        }
      }
    }
    catch (e: unknown) {
      erreur.value = (e as any)?.data?.error ?? (e as Error)?.message ?? 'Erreur réseau'
      throw e
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    demandes: readonly(demandes),
    demandeDetail: readonly(demandeDetail),
    total: readonly(total),
    page: readonly(page),
    totalPages: readonly(totalPages),
    nbEnAttente,
    listerDemandes,
    obtenirDemande,
    validerDemande,
    rejeterDemande,
  }
}

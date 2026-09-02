// Composable pour l'administration : Retrouve Amis (avis, signalements, stats)
import type { ApiResponse } from '~/types/admin'

// ── Interfaces ─────────────────────────────────────────────

export interface AdminAuteurInfo {
  id: string
  nom: string
  prenom: string
  email?: string
}

export interface AdminPaysInfo {
  id: string
  nom: string
}

export interface AdminAvisRecherche {
  id: string
  auteur: AdminAuteurInfo
  nom_recherche: string
  prenom_recherche?: string
  ecole?: string
  ville?: string
  pays?: AdminPaysInfo
  etat: string
  nb_correspondances: number
  nb_signalements: number
  created_at: string
}

export interface AdminCorrespondanceInfo {
  id: string
  score: number
  etat: string
  type_cible: string
  cible_utilisateur?: AdminAuteurInfo
  created_at: string
}

export interface AdminSignalementInfo {
  id: string
  signale_par: AdminAuteurInfo
  motif: string
  description?: string
  etat: string
  created_at: string
}

export interface AdminAvisRechercheDetail extends AdminAvisRecherche {
  surnom?: string
  periode_debut?: number
  periode_fin?: number
  description?: string
  updated_at: string
  correspondances: AdminCorrespondanceInfo[]
  signalements: AdminSignalementInfo[]
}

export interface AdminAvisResume {
  id: string
  nom_recherche: string
  auteur: AdminAuteurInfo
}

export interface AdminSignalement {
  id: string
  avis: AdminAvisResume
  signale_par: AdminAuteurInfo
  motif: string
  description?: string
  etat: string
  created_at: string
}

export interface AdminStatistiques {
  total_avis: number
  avis_actifs: number
  avis_clotures: number
  avis_suspendus: number
  total_correspondances: number
  correspondances_mutuelles: number
  correspondances_en_attente: number
  correspondances_declinees: number
  correspondances_archivees: number
  utilisateurs_trouvables: number
  signalements_en_attente: number
  signalements_total: number
  blacklists_total: number
}

export interface AdminDemandeRetrait {
  id: string
  avis_id: string
  nom_recherche: string
  demandeur: { id: string; prenom: string; nom: string }
  auteur: { id: string; prenom: string; nom: string }
  motif: string
  etat: string
  date_suspension?: string
  created_at: string
}

export interface AdminStatuerDemandeResponse {
  id: string
  etat: string
  avis_id: string
  avis_etat: string
  avis_est_public: boolean
}

// ── Reponses paginées spécifiques ──────────────────────────

interface AvisPagineResponse {
  avis: AdminAvisRecherche[]
  total: number
  page: number
  par_page: number
}

interface SignalementsPagineResponse {
  signalements: AdminSignalement[]
  total: number
  page: number
  par_page: number
}

interface DemandesRetraitPagineResponse {
  demandes: AdminDemandeRetrait[]
  pagination: {
    page: number
    par_page: number
    total: number
    pages: number
  }
}

// ── Composable ─────────────────────────────────────────────

export const useAdminRetrouvAmis = () => {
  const { adminFetch, listerPagine, pagination, sort, loading, error, allerPage, changerTri, reinitialiserPagination } = useAdmin()

  // ── Etat reactif ─────────────────────────────────────────
  const avis = ref<AdminAvisRecherche[]>([])
  const avisDetail = ref<AdminAvisRechercheDetail | null>(null)
  const signalements = ref<AdminSignalement[]>([])
  const signalementDetail = ref<AdminSignalementInfo | null>(null)
  const stats = ref<AdminStatistiques | null>(null)
  const total = ref(0)
  const totalSignalements = ref(0)

  const filtresAvis = reactive({
    recherche: '',
    etat: '',
    auteur_id: '',
    pays_id: '',
    date_debut: '',
    date_fin: '',
  })

  const filtresSignalements = reactive({
    etat: '',
    motif: '',
  })

  // ── Avis de recherche ────────────────────────────────────

  const chargerAvis = async () => {
    loading.value = true
    error.value = null

    try {
      const filtres: Record<string, string> = {}
      if (filtresAvis.recherche) filtres.recherche = filtresAvis.recherche
      if (filtresAvis.etat) filtres.etat = filtresAvis.etat
      if (filtresAvis.auteur_id) filtres.auteur_id = filtresAvis.auteur_id
      if (filtresAvis.pays_id) filtres.pays_id = filtresAvis.pays_id
      if (filtresAvis.date_debut) filtres.date_debut = filtresAvis.date_debut
      if (filtresAvis.date_fin) filtres.date_fin = filtresAvis.date_fin

      const response = await adminFetch<ApiResponse<AvisPagineResponse>>(
        '/api/admin/retrouve-amis/avis',
        {
          params: {
            page: pagination.page,
            par_page: pagination.parPage,
            tri_par: sort.column,
            tri_dir: sort.direction,
            ...filtres,
          },
        },
      )

      if (response.success && response.data) {
        avis.value = response.data.avis
        total.value = response.data.total
        pagination.total = response.data.total
        pagination.page = response.data.page
        pagination.totalPages = Math.ceil(response.data.total / pagination.parPage)
      }
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur lors du chargement des avis'
    }
    finally {
      loading.value = false
    }
  }

  const chargerDetailAvis = async (id: string) => {
    loading.value = true
    error.value = null

    try {
      const response = await adminFetch<ApiResponse<AdminAvisRechercheDetail>>(
        `/api/admin/retrouve-amis/avis/${id}`,
      )
      if (response.success && response.data) {
        avisDetail.value = response.data
      }
      return response.data
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur lors du chargement du detail'
      return null
    }
    finally {
      loading.value = false
    }
  }

  const changerEtatAvis = async (id: string, etat: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string }>>(
      `/api/admin/retrouve-amis/avis/${id}/etat`,
      { method: 'PATCH', body: { etat } },
    )
    return response.data
  }

  // ── Signalements ─────────────────────────────────────────

  const chargerSignalements = async () => {
    loading.value = true
    error.value = null

    try {
      const filtres: Record<string, string> = {}
      if (filtresSignalements.etat) filtres.etat = filtresSignalements.etat
      if (filtresSignalements.motif) filtres.motif = filtresSignalements.motif

      const response = await adminFetch<ApiResponse<SignalementsPagineResponse>>(
        '/api/admin/retrouve-amis/signalements',
        {
          params: {
            page: pagination.page,
            par_page: pagination.parPage,
            tri_par: sort.column,
            tri_dir: sort.direction,
            ...filtres,
          },
        },
      )

      if (response.success && response.data) {
        signalements.value = response.data.signalements
        totalSignalements.value = response.data.total
        pagination.total = response.data.total
        pagination.page = response.data.page
        pagination.totalPages = Math.ceil(response.data.total / pagination.parPage)
      }
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur lors du chargement des signalements'
    }
    finally {
      loading.value = false
    }
  }

  const chargerDetailSignalement = async (id: string) => {
    loading.value = true
    error.value = null

    try {
      const response = await adminFetch<ApiResponse<AdminSignalementInfo>>(
        `/api/admin/retrouve-amis/signalements/${id}`,
      )
      if (response.success && response.data) {
        signalementDetail.value = response.data
      }
      return response.data
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur lors du chargement du signalement'
      return null
    }
    finally {
      loading.value = false
    }
  }

  const modererSignalement = async (id: string, decision: string) => {
    const response = await adminFetch<ApiResponse<{ id: string; etat: string; avis_suspendu: boolean }>>(
      `/api/admin/retrouve-amis/signalements/${id}/moderer`,
      { method: 'PATCH', body: { decision } },
    )
    return response.data
  }

  // ── Demandes de retrait ─────────────────────────────────

  const demandesRetrait = ref<AdminDemandeRetrait[]>([])
  const totalDemandesRetrait = ref(0)

  const filtresDemandesRetrait = reactive({
    etat: '',
  })

  const chargerDemandesRetrait = async () => {
    loading.value = true
    error.value = null

    try {
      const filtres: Record<string, string> = {}
      if (filtresDemandesRetrait.etat) filtres.etat = filtresDemandesRetrait.etat

      const response = await adminFetch<ApiResponse<DemandesRetraitPagineResponse>>(
        '/api/admin/retrouve-amis/demandes-retrait',
        {
          params: {
            page: pagination.page,
            par_page: pagination.parPage,
            tri_par: sort.column,
            tri_dir: sort.direction,
            ...filtres,
          },
        },
      )

      if (response.success && response.data) {
        demandesRetrait.value = response.data.demandes
        totalDemandesRetrait.value = response.data.pagination.total
        pagination.total = response.data.pagination.total
        pagination.page = response.data.pagination.page
        pagination.totalPages = response.data.pagination.pages
      }
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur lors du chargement des demandes de retrait'
    }
    finally {
      loading.value = false
    }
  }

  const statuerDemandeRetrait = async (id: string, data: { decision: string; commentaire?: string }) => {
    const response = await adminFetch<ApiResponse<AdminStatuerDemandeResponse>>(
      `/api/admin/retrouve-amis/demandes-retrait/${id}/statuer`,
      { method: 'PATCH', body: data },
    )
    return response.data
  }

  // ── Statistiques ─────────────────────────────────────────

  const chargerStatistiques = async () => {
    loading.value = true
    error.value = null

    try {
      const response = await adminFetch<ApiResponse<AdminStatistiques>>(
        '/api/admin/retrouve-amis/statistiques',
      )
      if (response.success && response.data) {
        stats.value = response.data
      }
      return response.data
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur lors du chargement des statistiques'
      return null
    }
    finally {
      loading.value = false
    }
  }

  return {
    // Etat
    avis, avisDetail, signalements, signalementDetail, stats,
    demandesRetrait, totalDemandesRetrait,
    total, totalSignalements,
    filtresAvis, filtresSignalements, filtresDemandesRetrait,
    pagination, sort, loading, error,

    // Actions : Avis
    chargerAvis, chargerDetailAvis, changerEtatAvis,

    // Actions : Signalements
    chargerSignalements, chargerDetailSignalement, modererSignalement,

    // Actions : Demandes de retrait
    chargerDemandesRetrait, statuerDemandeRetrait,

    // Actions : Statistiques
    chargerStatistiques,

    // Navigation pagination
    allerPage, changerTri, reinitialiserPagination,
  }
}

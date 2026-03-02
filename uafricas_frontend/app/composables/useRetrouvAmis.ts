// Composable pour les appels API Retrouv'Amis
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Information pays */
export interface PaysInfo {
  id: string
  nom: string
}

// ── Enums (string unions) ────────────────────────────────────

export type EtatAvis = 'actif' | 'cloture' | 'suspendu'
export type EtatCorrespondance = 'en_attente' | 'acceptee_a' | 'acceptee_b' | 'mutuelle' | 'declinee' | 'archivee'
export type TypeCible = 'avis' | 'profil'
export type MotifSignalement = 'contenu_abusif' | 'usurpation_identite' | 'harcelement' | 'autre'
export type TypeParcours = 'ecole' | 'ville_residence'
export type TypeNotification = 'nouvelle_correspondance' | 'acceptation_contact' | 'coordonnees_partagees' | 'correspondance_archivee' | 'avis_suspendu' | 'reponse_publique' | 'demande_retrait'

// ── Interfaces de reponse ────────────────────────────────────

/** Resume anonymise d'un profil ou avis correspondant */
export interface ResumeAnonyme {
  initiales: string
  ville?: string
  periode?: string
  criteres_communs: string[]
}

/** Avis de recherche (liste) */
export interface AvisRecherche {
  id: string
  nom_recherche: string
  prenom_recherche?: string
  surnom?: string
  ecole?: string
  ville?: string
  pays?: PaysInfo
  periode_debut?: number
  periode_fin?: number
  description?: string
  etat: EtatAvis
  nb_correspondances: number
  est_public?: boolean
  slug?: string
  compteur_partages?: number
  created_at: string
  updated_at: string
}

/** Avis de recherche (detail avec correspondances) */
export interface AvisRechercheDetail extends AvisRecherche {
  correspondances: CorrespondanceResume[]
}

/** Resume d'une correspondance (dans le detail d'un avis) */
export interface CorrespondanceResume {
  id: string
  score: number
  etat: EtatCorrespondance
  resume_anonymise: ResumeAnonyme
  created_at: string
}

/** Correspondance (liste) */
export interface Correspondance {
  id: string
  avis_id: string
  score: number
  etat: EtatCorrespondance
  type_cible: TypeCible
  resume_anonymise: ResumeAnonyme
  mon_role: string
  created_at: string
  expire_at?: string
}

/** Correspondance (detail avec score et coordonnees) */
export interface CorrespondanceDetail extends Correspondance {
  details_score: Record<string, number>
  coordonnees_partagees?: Record<string, any>
}

/** Notification du module Retrouv'Amis */
export interface NotificationRetrouve {
  id: string
  type: TypeNotification
  correspondance_id?: string
  lu: boolean
  created_at: string
}

/** Parcours trouvable (ecole ou ville de residence) */
export interface ParcoursTrouvable {
  id: string
  type_entree: TypeParcours
  nom: string
  ville?: string
  pays?: PaysInfo
  periode_debut?: number
  periode_fin?: number
}

/** Tableau de bord du module Retrouv'Amis */
export interface TableauDeBord {
  avis_actifs: number
  avis_clotures: number
  correspondances_en_attente: number
  correspondances_mutuelles: number
  notifications_non_lues: number
  est_trouvable: boolean
  nb_parcours: number
}

// ── Interfaces de reponse — Partage public ─────────────────

/** Detail complet d'un avis public actif */
export interface AvisPublicDetail {
  id: string
  slug: string
  nom_recherche: string
  prenom_recherche?: string
  ecole?: string
  ville?: string
  pays?: PaysInfo
  periode_debut?: number
  periode_fin?: number
  description?: string
  auteur_anonyme: string
  etat: EtatAvis
  compteur_partages: number
  date_publication_publique?: string
  created_at: string
}

/** Reponse pour un avis non-actif (cloture, suspendu) */
export interface AvisPublicEtat {
  slug: string
  etat: string
  message: string
}

/** Resume d'un avis public dans le listing */
export interface AvisPublicResume {
  slug: string
  nom_recherche: string
  prenom_recherche?: string
  ville?: string
  pays?: PaysInfo
  periode_debut?: number
  periode_fin?: number
  compteur_partages: number
  created_at: string
}

/** Info de pagination standard */
export interface PaginationInfo {
  page: number
  par_page: number
  total: number
  pages: number
}

/** Reponse paginee du listing des avis publics */
export interface ListeAvisPublicsResponse {
  avis: AvisPublicResume[]
  pagination: PaginationInfo
}

/** Parametres de recherche des avis publics */
export interface RecherchePubliqueParams {
  page?: number
  par_page?: number
  recherche?: string
  pays_id?: string
  ville?: string
  ecole?: string
  tri?: 'created_at' | 'compteur_partages'
  ordre?: 'asc' | 'desc'
}

/** Reponse apres publication/depublication d'un avis */
export interface PublierAvisResponse {
  id: string
  est_public: boolean
  slug: string
  date_publication_publique?: string
}

// ── DTOs de requete ──────────────────────────────────────────

/** Corps de la requete pour creer/modifier un avis de recherche */
export interface CreerAvisRequest {
  nom_recherche: string
  prenom_recherche?: string
  surnom?: string
  ecole?: string
  ville?: string
  pays_id?: string
  periode_debut?: number
  periode_fin?: number
  description?: string
}

/** Choix de coordonnees a partager lors de l'acceptation */
export interface CoordonneesChoix {
  email: boolean
  telephone: boolean
  messagerie: boolean
}

/** Corps de la requete pour signaler un avis */
export interface SignalerRequest {
  motif: MotifSignalement
  description?: string
}

/** Corps de la requete pour creer/modifier un parcours trouvable */
export interface CreerParcoursRequest {
  type_entree: TypeParcours
  nom: string
  ville?: string
  pays_id?: string
  periode_debut?: number
  periode_fin?: number
}

// ── Interfaces de reponses paginées ──────────────────────────

export interface AvisListeAPI {
  avis: AvisRecherche[]
  total: number
  page: number
  par_page: number
}

export interface CorrespondanceListeAPI {
  correspondances: Correspondance[]
  total: number
  page: number
  par_page: number
}

export interface NotificationListeAPI {
  notifications: NotificationRetrouve[]
  total: number
  non_lues: number
  page: number
  par_page: number
}

// ── Parametres de filtres ────────────────────────────────────

export interface AvisFiltres {
  etat?: EtatAvis
  page?: number
  par_page?: number
  tri?: string
  ordre?: string
}

export interface CorrespondanceFiltres {
  etat?: string
  avis_id?: string
  page?: number
  par_page?: number
}

export interface NotificationFiltres {
  lu?: boolean
  page?: number
  par_page?: number
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const ETATS_AVIS: { value: EtatAvis | ''; label: string }[] = [
  { value: '', label: 'Tous les etats' },
  { value: 'actif', label: 'Actif' },
  { value: 'cloture', label: 'Cloture' },
  { value: 'suspendu', label: 'Suspendu' },
]

export const ETATS_CORRESPONDANCE: { value: EtatCorrespondance; label: string; couleur: string }[] = [
  { value: 'en_attente', label: 'En attente', couleur: 'bg-yellow-100 text-yellow-800' },
  { value: 'acceptee_a', label: 'Acceptee (vous)', couleur: 'bg-blue-100 text-blue-800' },
  { value: 'acceptee_b', label: 'Acceptee (autre)', couleur: 'bg-blue-100 text-blue-800' },
  { value: 'mutuelle', label: 'Mutuelle', couleur: 'bg-green-100 text-green-800' },
  { value: 'declinee', label: 'Declinee', couleur: 'bg-red-100 text-red-800' },
  { value: 'archivee', label: 'Archivee', couleur: 'bg-gray-100 text-gray-800' },
]

export const MOTIFS_SIGNALEMENT: { value: MotifSignalement; label: string }[] = [
  { value: 'contenu_abusif', label: 'Contenu abusif' },
  { value: 'usurpation_identite', label: 'Usurpation d\'identite' },
  { value: 'harcelement', label: 'Harcelement' },
  { value: 'autre', label: 'Autre' },
]

export const TYPES_PARCOURS: { value: TypeParcours; label: string; icon: string }[] = [
  { value: 'ecole', label: 'Ecole / Etablissement', icon: 'fa-solid fa-school' },
  { value: 'ville_residence', label: 'Ville de residence', icon: 'fa-solid fa-city' },
]

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/** Formater une date ISO en francais long (ex: "15 janvier 2025") */
export const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
}

/** Formater une date ISO en relatif (ex: "Il y a 3 j") */
export const formatDateRelative = (dateStr: string): string => {
  const date = new Date(dateStr)
  const now = new Date()
  const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60))

  if (diffInMinutes < 1) return 'A l\'instant'
  if (diffInMinutes < 60) return `Il y a ${diffInMinutes} min`
  if (diffInMinutes < 1440) return `Il y a ${Math.floor(diffInMinutes / 60)} h`
  if (diffInMinutes < 10080) return `Il y a ${Math.floor(diffInMinutes / 1440)} j`

  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: date.getFullYear() !== now.getFullYear() ? 'numeric' : undefined,
  })
}

/** Obtenir le label d'un etat d'avis */
export const getEtatAvisLabel = (etat: EtatAvis): string => {
  const info = ETATS_AVIS.find(e => e.value === etat)
  return info?.label || etat
}

/** Obtenir la couleur CSS d'un etat de correspondance */
export const getEtatCorrespondanceCouleur = (etat: EtatCorrespondance): string => {
  const info = ETATS_CORRESPONDANCE.find(e => e.value === etat)
  return info?.couleur || 'bg-gray-100 text-gray-800'
}

/** Formater une periode (ex: "2010 - 2015") */
export const formatPeriode = (debut?: number, fin?: number): string => {
  if (debut && fin) return `${debut} - ${fin}`
  if (debut) return `Depuis ${debut}`
  if (fin) return `Jusqu'a ${fin}`
  return ''
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useRetrouvAmis = () => {
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

  // ── Avis de recherche ────────────────────────────────────────

  /**
   * Creer un nouvel avis de recherche
   */
  const creerAvis = async (data: CreerAvisRequest): Promise<{ id: string; etat: string; correspondances_trouvees: number } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ id: string; etat: string; correspondances_trouvees: number }>>(
        `${apiBase}/api/retrouve-amis/avis`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation de l\'avis')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de la creation de l\'avis'
      erreur.value = message
      console.error('Erreur creerAvis:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Lister les avis de recherche avec filtres et pagination
   */
  const listerAvis = async (filtres: AvisFiltres = {}): Promise<AvisListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.etat) params.set('etat', filtres.etat)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))
      if (filtres.tri) params.set('tri', filtres.tri)
      if (filtres.ordre) params.set('ordre', filtres.ordre)

      const queryString = params.toString()
      const url = `${apiBase}/api/retrouve-amis/avis${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<AvisListeAPI>>(url, {
        headers: authHeaders(),
      })

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des avis')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du chargement des avis'
      erreur.value = message
      console.error('Erreur listerAvis:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir le detail d'un avis de recherche par son ID
   */
  const detailAvis = async (id: string): Promise<AvisRechercheDetail | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<AvisRechercheDetail>>(
        `${apiBase}/api/retrouve-amis/avis/${id}`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Avis non trouve')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du chargement de l\'avis'
      erreur.value = message
      console.error('Erreur detailAvis:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Modifier un avis de recherche existant
   */
  const modifierAvis = async (id: string, data: CreerAvisRequest): Promise<{ id: string; correspondances_trouvees: number } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ id: string; correspondances_trouvees: number }>>(
        `${apiBase}/api/retrouve-amis/avis/${id}`,
        {
          method: 'PUT',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la modification de l\'avis')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de la modification de l\'avis'
      erreur.value = message
      console.error('Erreur modifierAvis:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Cloturer un avis de recherche (passage en etat 'cloture')
   */
  const cloturerAvis = async (id: string): Promise<{ id: string; etat: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ id: string; etat: string }>>(
        `${apiBase}/api/retrouve-amis/avis/${id}/cloturer`,
        {
          method: 'POST',
          headers: authHeaders(),
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la cloture de l\'avis')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de la cloture de l\'avis'
      erreur.value = message
      console.error('Erreur cloturerAvis:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Correspondances ──────────────────────────────────────────

  /**
   * Lister les correspondances avec filtres et pagination
   */
  const listerCorrespondances = async (filtres: CorrespondanceFiltres = {}): Promise<CorrespondanceListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.etat) params.set('etat', filtres.etat)
      if (filtres.avis_id) params.set('avis_id', filtres.avis_id)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/retrouve-amis/correspondances${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<CorrespondanceListeAPI>>(url, {
        headers: authHeaders(),
      })

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des correspondances')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du chargement des correspondances'
      erreur.value = message
      console.error('Erreur listerCorrespondances:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir le detail d'une correspondance par son ID
   */
  const detailCorrespondance = async (id: string): Promise<CorrespondanceDetail | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<CorrespondanceDetail>>(
        `${apiBase}/api/retrouve-amis/correspondances/${id}`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Correspondance non trouvee')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du chargement de la correspondance'
      erreur.value = message
      console.error('Erreur detailCorrespondance:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Accepter une correspondance en partageant ses coordonnees
   */
  const accepterCorrespondance = async (id: string, coordonnees: CoordonneesChoix): Promise<{ id: string; etat: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ id: string; etat: string }>>(
        `${apiBase}/api/retrouve-amis/correspondances/${id}/accepter`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: coordonnees,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l\'acceptation de la correspondance')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de l\'acceptation de la correspondance'
      erreur.value = message
      console.error('Erreur accepterCorrespondance:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Refuser (decliner) une correspondance
   */
  const refuserCorrespondance = async (id: string): Promise<{ id: string; etat: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ id: string; etat: string }>>(
        `${apiBase}/api/retrouve-amis/correspondances/${id}/refuser`,
        {
          method: 'POST',
          headers: authHeaders(),
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du refus de la correspondance')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du refus de la correspondance'
      erreur.value = message
      console.error('Erreur refuserCorrespondance:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Signalement ──────────────────────────────────────────────

  /**
   * Signaler un avis de recherche
   */
  const signalerAvis = async (avisId: string, data: SignalerRequest): Promise<{ id: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/api/retrouve-amis/avis/${avisId}/signaler`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du signalement')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du signalement'
      erreur.value = message
      console.error('Erreur signalerAvis:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Notifications ────────────────────────────────────────────

  /**
   * Lister les notifications avec filtres et pagination
   */
  const listerNotifications = async (filtres: NotificationFiltres = {}): Promise<NotificationListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.lu !== undefined) params.set('lu', String(filtres.lu))
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/retrouve-amis/notifications${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<NotificationListeAPI>>(url, {
        headers: authHeaders(),
      })

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des notifications')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du chargement des notifications'
      erreur.value = message
      console.error('Erreur listerNotifications:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Marquer une notification comme lue
   */
  const marquerLu = async (id: string): Promise<boolean> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/retrouve-amis/notifications/${id}/lu`,
        {
          method: 'POST',
          headers: authHeaders(),
        },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors du marquage de la notification')
      }

      return true
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du marquage de la notification'
      erreur.value = message
      console.error('Erreur marquerLu:', e)
      return false
    }
  }

  /**
   * Marquer toutes les notifications comme lues
   */
  const toutMarquerLu = async (): Promise<{ mises_a_jour: number } | null> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ mises_a_jour: number }>>(
        `${apiBase}/api/retrouve-amis/notifications/tout-lu`,
        {
          method: 'POST',
          headers: authHeaders(),
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du marquage des notifications')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du marquage des notifications'
      erreur.value = message
      console.error('Erreur toutMarquerLu:', e)
      return null
    }
  }

  // ── Tableau de bord ──────────────────────────────────────────

  /**
   * Obtenir le tableau de bord Retrouv'Amis de l'utilisateur
   */
  const tableauDeBord = async (): Promise<TableauDeBord | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<TableauDeBord>>(
        `${apiBase}/api/retrouve-amis/tableau-de-bord`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement du tableau de bord')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du chargement du tableau de bord'
      erreur.value = message
      console.error('Erreur tableauDeBord:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Profil trouvable ─────────────────────────────────────────

  /**
   * Basculer le statut trouvable du profil
   */
  const basculerTrouvable = async (est_trouvable: boolean): Promise<{ est_trouvable: boolean; correspondances_trouvees: number } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ est_trouvable: boolean; correspondances_trouvees: number }>>(
        `${apiBase}/api/auth/profil/trouvable`,
        {
          method: 'PATCH',
          headers: authHeaders(),
          body: { est_trouvable },
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la modification du statut trouvable')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de la modification du statut trouvable'
      erreur.value = message
      console.error('Erreur basculerTrouvable:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Lister les parcours trouvables de l'utilisateur
   */
  const listerParcours = async (): Promise<ParcoursTrouvable[] | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ParcoursTrouvable[]>>(
        `${apiBase}/api/auth/profil/parcours`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des parcours')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du chargement des parcours'
      erreur.value = message
      console.error('Erreur listerParcours:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Ajouter un parcours trouvable
   */
  const ajouterParcours = async (data: CreerParcoursRequest): Promise<{ id: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/api/auth/profil/parcours`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l\'ajout du parcours')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de l\'ajout du parcours'
      erreur.value = message
      console.error('Erreur ajouterParcours:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Modifier un parcours trouvable existant
   */
  const modifierParcours = async (id: string, data: CreerParcoursRequest): Promise<boolean> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/auth/profil/parcours/${id}`,
        {
          method: 'PUT',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors de la modification du parcours')
      }

      return true
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de la modification du parcours'
      erreur.value = message
      console.error('Erreur modifierParcours:', e)
      return false
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Supprimer un parcours trouvable
   */
  const supprimerParcours = async (id: string): Promise<boolean> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/auth/profil/parcours/${id}`,
        {
          method: 'DELETE',
          headers: authHeaders(),
        },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors de la suppression du parcours')
      }

      return true
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de la suppression du parcours'
      erreur.value = message
      console.error('Erreur supprimerParcours:', e)
      return false
    }
    finally {
      chargement.value = false
    }
  }

  // ── Partage public ──────────────────────────────────────────

  /**
   * Activer ou desactiver la visibilite publique d'un avis
   */
  const publierAvis = async (id: string, estPublic: boolean): Promise<PublierAvisResponse | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<PublierAvisResponse>>(
        `${apiBase}/api/retrouve-amis/avis/${id}/publier`,
        {
          method: 'PATCH',
          headers: authHeaders(),
          body: { est_public: estPublic },
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la publication de l\'avis')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de la publication de l\'avis'
      erreur.value = message
      console.error('Erreur publierAvis:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir le detail d'un avis public par son slug (sans auth)
   */
  const detailAvisPublic = async (slug: string): Promise<AvisPublicDetail | AvisPublicEtat | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<AvisPublicDetail | AvisPublicEtat>>(
        `${apiBase}/api/retrouve-amis/public/${slug}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Avis non disponible')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Avis non disponible'
      erreur.value = message
      console.error('Erreur detailAvisPublic:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Protections anti-harcelement (page publique) ────────────

  /**
   * Signaler un avis depuis la page publique (connexion requise)
   */
  const signalerAvisPublic = async (slug: string, data: { motif: MotifSignalement; description?: string }): Promise<{ id: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/api/retrouve-amis/public/${slug}/signaler`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du signalement')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du signalement'
      erreur.value = message
      console.error('Erreur signalerAvisPublic:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Demander le retrait d'un avis (la personne se reconnait)
   */
  const demanderRetrait = async (slug: string, motif: string): Promise<{ id: string; message: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ id: string; message: string }>>(
        `${apiBase}/api/retrouve-amis/public/${slug}/demande-retrait`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: { motif },
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la demande de retrait')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de la demande de retrait'
      erreur.value = message
      console.error('Erreur demanderRetrait:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Partage social ─────────────────────────────────────────

  /**
   * Incrementer le compteur de partages d'un avis public (sans auth)
   */
  const incrementerPartage = async (slug: string): Promise<{ compteur_partages: number } | null> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ compteur_partages: number }>>(
        `${apiBase}/api/retrouve-amis/public/${slug}/partage`,
        { method: 'POST' },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du partage')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors du partage'
      erreur.value = message
      console.error('Erreur incrementerPartage:', e)
      return null
    }
  }

  // ── Recherche publique ────────────────────────────────────

  /**
   * Rechercher parmi les avis publics actifs (sans auth)
   */
  const rechercherAvisPublics = async (params: RecherchePubliqueParams = {}): Promise<ListeAvisPublicsResponse | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const queryParams = new URLSearchParams()
      if (params.page) queryParams.set('page', String(params.page))
      if (params.par_page) queryParams.set('par_page', String(params.par_page))
      if (params.recherche) queryParams.set('recherche', params.recherche)
      if (params.pays_id) queryParams.set('pays_id', params.pays_id)
      if (params.ville) queryParams.set('ville', params.ville)
      if (params.ecole) queryParams.set('ecole', params.ecole)
      if (params.tri) queryParams.set('tri', params.tri)
      if (params.ordre) queryParams.set('ordre', params.ordre)

      const queryString = queryParams.toString()
      const url = `${apiBase}/api/retrouve-amis/public/rechercher${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ListeAvisPublicsResponse>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la recherche')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur lors de la recherche'
      erreur.value = message
      console.error('Erreur rechercherAvisPublics:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    // Avis de recherche
    creerAvis,
    listerAvis,
    detailAvis,
    modifierAvis,
    cloturerAvis,
    // Correspondances
    listerCorrespondances,
    detailCorrespondance,
    accepterCorrespondance,
    refuserCorrespondance,
    // Signalement
    signalerAvis,
    // Notifications
    listerNotifications,
    marquerLu,
    toutMarquerLu,
    // Tableau de bord
    tableauDeBord,
    // Profil trouvable
    basculerTrouvable,
    listerParcours,
    ajouterParcours,
    modifierParcours,
    supprimerParcours,
    // Partage public
    publierAvis,
    detailAvisPublic,
    // Protections anti-harcelement
    signalerAvisPublic,
    demanderRetrait,
    // Partage social
    incrementerPartage,
    // Recherche publique
    rechercherAvisPublics,
  }
}

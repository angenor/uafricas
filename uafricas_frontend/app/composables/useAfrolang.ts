// Composable pour les appels API Afrolang (salles de visioconference)
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type EtatSession = 'planifiee' | 'en_cours' | 'terminee' | 'annulee'
export type RoleSession = 'moderateur' | 'participant' | 'observateur'

/** Utilisateur resume (JOIN depuis le backend) */
export interface AfrolangUser {
  id: string
  nom: string
  prenom: string | null
  photo_url: string | null
}

/** DTO salle publique (liste) */
export interface SalleAPI {
  id: string
  titre: string
  slug: string | null
  description: string | null
  image_couverture_url: string | null
  langue_cible: string | null
  actif: boolean
  nombre_salles_privees: number
  sessions_en_cours: number
  created_at: string
  updated_at: string
}

/** DTO salle publique (detail) */
export interface SalleDetailAPI extends SalleAPI {
  moderateur: AfrolangUser | null
  salles_privees: SallePriveeAPI[]
}

/** DTO salle privee (liste) */
export interface SallePriveeAPI {
  id: string
  salle_id: string
  titre: string
  description: string | null
  image_couverture_url: string | null
  max_participants: number | null
  est_protegee: boolean
  actif: boolean
  createur: AfrolangUser
  salle_titre: string | null
  salle_langue: string | null
  session_en_cours: boolean
  created_at: string
  updated_at: string
}

/** DTO salle privee (detail avec sessions) */
export interface SallePriveeDetailAPI extends SallePriveeAPI {
  sessions: SessionAPI[]
}

/** DTO session (liste) */
export interface SessionAPI {
  id: string
  salle_privee_id: string
  titre: string | null
  etat: EtatSession
  date_debut_prevue: string | null
  demarre_at: string | null
  termine_at: string | null
  duree_secondes: number | null
  max_participants: number | null
  nombre_participants_pic: number | null
  tableau_blanc_actif: boolean
  created_at: string
  updated_at: string
}

/** DTO session (detail avec participants) */
export interface SessionDetailAPI extends SessionAPI {
  moderateur: AfrolangUser | null
  participants: ParticipantAPI[]
}

/** DTO participant */
export interface ParticipantAPI {
  id: string
  utilisateur_id: string
  nom: string | null
  prenom: string | null
  photo_url: string | null
  role_session: RoleSession
  rejoint_at: string
  quitte_at: string | null
  duree_secondes: number | null
}

/** Reponse du token LiveKit (Phase 3) */
export interface TokenResponse {
  token: string
  room_name: string
  livekit_url: string
  is_moderator: boolean
}

/** Donnees du tableau blanc (Phase 4) */
export interface TableauBlancData {
  donnees: Record<string, any>
  version: number
}

/** Stats globales Afrolang */
export interface AfrolangStats {
  total_salles: number
  total_salles_privees: number
  sessions_en_cours: number
  sessions_terminees: number
  total_participants_uniques: number
}

/** Reponse paginee salles */
export interface SalleListeAPI {
  salles: SalleAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Reponse paginee salles privees */
export interface SallePriveeListeAPI {
  salles_privees: SallePriveeAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Reponse paginee sessions */
export interface SessionListeAPI {
  sessions: SessionAPI[]
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

/** Filtres pour le listing des salles */
export interface SalleFiltres {
  recherche?: string
  langue?: string
  page?: number
  par_page?: number
}

/** Filtres pour le listing des salles privees */
export interface SallePriveeFiltres {
  recherche?: string
  page?: number
  par_page?: number
}

/** Filtres pour le listing des sessions */
export interface SessionFiltres {
  etat?: string
  page?: number
  par_page?: number
}

/** Formulaire creation salle privee */
export interface CreerSallePriveeForm {
  titre: string
  description: string
  code_acces: string
  max_participants: number | null
}

/** Formulaire creation session */
export interface CreerSessionForm {
  titre: string
  date_debut_prevue: string
  max_participants: number | null
  tableau_blanc_actif: boolean
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const ETATS_SESSION: { value: string; label: string }[] = [
  { value: '', label: 'Tous les états' },
  { value: 'planifiee', label: 'Planifiée' },
  { value: 'en_cours', label: 'En cours' },
  { value: 'terminee', label: 'Terminée' },
  { value: 'annulee', label: 'Annulée' },
]

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/** Info d'affichage pour un etat de session */
export const getEtatInfo = (etat: EtatSession): { label: string; couleur: string; icone: string } => {
  switch (etat) {
    case 'planifiee':
      return { label: 'Planifiée', couleur: 'badge-info', icone: 'calendar-days' }
    case 'en_cours':
      return { label: 'En direct', couleur: 'badge-success', icone: 'video' }
    case 'terminee':
      return { label: 'Terminée', couleur: 'badge-neutral', icone: 'circle-check' }
    case 'annulee':
      return { label: 'Annulée', couleur: 'badge-error', icone: 'xmark' }
    default:
      return { label: etat, couleur: 'badge-ghost', icone: 'circle-info' }
  }
}

/** Formater une duree en secondes vers un format lisible */
export const formatDuree = (secondes: number | null): string => {
  if (!secondes || secondes <= 0) return '0min'
  const heures = Math.floor(secondes / 3600)
  const minutes = Math.floor((secondes % 3600) / 60)
  if (heures > 0 && minutes > 0) return `${heures}h ${minutes}min`
  if (heures > 0) return `${heures}h`
  return `${minutes}min`
}

/** Formater une date ISO en francais court */
export const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
}

/** Formater une date ISO en format complet avec heure */
export const formatDateHeure = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

/** Obtenir les initiales d'un utilisateur */
export const getInitiales = (nom?: string | null, prenom?: string | null): string => {
  return (prenom?.charAt(0).toUpperCase() || '') + (nom?.charAt(0).toUpperCase() || '') || '?'
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useAfrolang = () => {
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

  /** Resoudre une URL relative vers une URL absolue */
  const resoudreUrl = (url: string | null): string | null => {
    if (!url) return null
    if (url.startsWith('http')) return url
    return `${apiBase}${url}`
  }

  // ── Salles publiques ──────────────────────────────────────

  /** Lister les salles publiques avec filtres et pagination */
  const listerSalles = async (filtres: SalleFiltres = {}): Promise<SalleListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.langue) params.set('langue', filtres.langue)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/afrolang/salles${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<SalleListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des salles')
      }

      // Resoudre les URLs d'images
      for (const salle of reponse.data.salles) {
        salle.image_couverture_url = resoudreUrl(salle.image_couverture_url)
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerSalles:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Obtenir le detail d'une salle publique */
  const obtenirSalle = async (id: string): Promise<SalleDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<SalleDetailAPI>>(
        `${apiBase}/api/afrolang/salles/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Salle non trouvee')
      }

      // Resoudre les URLs
      reponse.data.image_couverture_url = resoudreUrl(reponse.data.image_couverture_url)
      if (reponse.data.moderateur?.photo_url) {
        reponse.data.moderateur.photo_url = resoudreUrl(reponse.data.moderateur.photo_url)
      }
      for (const sp of reponse.data.salles_privees) {
        sp.image_couverture_url = resoudreUrl(sp.image_couverture_url)
        if (sp.createur.photo_url) {
          sp.createur.photo_url = resoudreUrl(sp.createur.photo_url)
        }
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirSalle:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Salles privees ────────────────────────────────────────

  /** Lister les salles privees d'une salle publique */
  const listerSallesPrivees = async (salleId: string, filtres: SallePriveeFiltres = {}): Promise<SallePriveeListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/afrolang/salles/${salleId}/privees${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<SallePriveeListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des salles privees')
      }

      for (const sp of reponse.data.salles_privees) {
        sp.image_couverture_url = resoudreUrl(sp.image_couverture_url)
        if (sp.createur.photo_url) {
          sp.createur.photo_url = resoudreUrl(sp.createur.photo_url)
        }
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerSallesPrivees:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Obtenir le detail d'une salle privee */
  const obtenirSallePrivee = async (id: string): Promise<SallePriveeDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<SallePriveeDetailAPI>>(
        `${apiBase}/api/afrolang/salles-privees/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Salle privee non trouvee')
      }

      reponse.data.image_couverture_url = resoudreUrl(reponse.data.image_couverture_url)
      if (reponse.data.createur.photo_url) {
        reponse.data.createur.photo_url = resoudreUrl(reponse.data.createur.photo_url)
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirSallePrivee:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Creer une salle privee */
  const creerSallePrivee = async (salleId: string, form: CreerSallePriveeForm): Promise<SallePriveeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const body: Record<string, any> = {
        titre: form.titre,
      }
      if (form.description) body.description = form.description
      if (form.code_acces) body.code_acces = form.code_acces
      if (form.max_participants) body.max_participants = form.max_participants

      const reponse = await $fetch<ApiResponse<SallePriveeAPI>>(
        `${apiBase}/api/afrolang/salles/${salleId}/privees`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation de la salle privee')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerSallePrivee:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Sessions ──────────────────────────────────────────────

  /** Obtenir le detail d'une session */
  const obtenirSession = async (id: string): Promise<SessionDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<SessionDetailAPI>>(
        `${apiBase}/api/afrolang/sessions/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Session non trouvee')
      }

      if (reponse.data.moderateur?.photo_url) {
        reponse.data.moderateur.photo_url = resoudreUrl(reponse.data.moderateur.photo_url)
      }
      for (const p of reponse.data.participants) {
        if (p.photo_url) {
          p.photo_url = resoudreUrl(p.photo_url)
        }
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirSession:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Creer une session dans une salle privee */
  const creerSession = async (sallePriveeId: string, form: CreerSessionForm): Promise<SessionAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const body: Record<string, any> = {}
      if (form.titre) body.titre = form.titre
      if (form.date_debut_prevue) body.date_debut_prevue = form.date_debut_prevue
      if (form.max_participants) body.max_participants = form.max_participants
      if (form.tableau_blanc_actif !== undefined) body.tableau_blanc_actif = form.tableau_blanc_actif

      const reponse = await $fetch<ApiResponse<SessionAPI>>(
        `${apiBase}/api/afrolang/salles-privees/${sallePriveeId}/sessions`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation de la session')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerSession:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Demarrer une session */
  const demarrerSession = async (sessionId: string): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/demarrer`,
        { method: 'PUT', headers: authHeaders() },
      )
      if (!reponse.success) throw new Error(reponse.error || 'Erreur')
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur reseau'
      return false
    }
  }

  /** Terminer une session */
  const terminerSession = async (sessionId: string): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/terminer`,
        { method: 'PUT', headers: authHeaders() },
      )
      if (!reponse.success) throw new Error(reponse.error || 'Erreur')
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur reseau'
      return false
    }
  }

  /** Rejoindre une session */
  const rejoindreSession = async (sessionId: string, codeAcces?: string): Promise<boolean> => {
    erreur.value = null
    try {
      const body: Record<string, any> = {}
      if (codeAcces) body.code_acces = codeAcces

      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/rejoindre`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body,
        },
      )
      if (!reponse.success) throw new Error(reponse.error || 'Erreur')
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur reseau'
      return false
    }
  }

  /** Quitter une session */
  const quitterSession = async (sessionId: string): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/quitter`,
        { method: 'POST', headers: authHeaders() },
      )
      if (!reponse.success) throw new Error(reponse.error || 'Erreur')
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur reseau'
      return false
    }
  }

  // ── Phase 3 : Token LiveKit ──────────────────────────────

  /** Generer un token LiveKit pour rejoindre la visioconference */
  const genererTokenSession = async (sessionId: string, codeAcces?: string): Promise<TokenResponse | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const body: Record<string, any> = {}
      if (codeAcces) body.code_acces = codeAcces

      const reponse = await $fetch<ApiResponse<TokenResponse>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/token`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la generation du token')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur genererTokenSession:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Utilitaires ───────────────────────────────────────────

  /** Obtenir les statistiques globales Afrolang */
  const obtenirStats = async (): Promise<AfrolangStats | null> => {
    try {
      const reponse = await $fetch<ApiResponse<AfrolangStats>>(
        `${apiBase}/api/afrolang/stats`,
      )
      if (!reponse.success || !reponse.data) return null
      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur obtenirStats:', e)
      return null
    }
  }

  /** Lister les langues disponibles */
  const listerLangues = async (): Promise<string[]> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/afrolang/langues`,
      )
      if (!reponse.success || !reponse.data) return []
      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerLangues:', e)
      return []
    }
  }

  // ── Phase 4 : Tableau blanc ──

  /** Obtenir le snapshot du tableau blanc d'une session */
  const obtenirTableauBlanc = async (sessionId: string): Promise<TableauBlancData> => {
    try {
      const reponse = await $fetch<ApiResponse<TableauBlancData>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/tableau-blanc`,
        { headers: { Authorization: `Bearer ${userStore.accessToken}` } },
      )
      if (!reponse.success || !reponse.data) return { donnees: {}, version: 0 }
      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur obtenirTableauBlanc:', e)
      return { donnees: {}, version: 0 }
    }
  }

  /** Sauvegarder le snapshot du tableau blanc */
  const sauvegarderTableauBlanc = async (sessionId: string, donnees: any): Promise<void> => {
    try {
      await $fetch(`${apiBase}/api/afrolang/sessions/${sessionId}/tableau-blanc`, {
        method: 'PUT',
        body: donnees,
        headers: { Authorization: `Bearer ${userStore.accessToken}` },
      })
    }
    catch (e: any) {
      console.error('Erreur sauvegarderTableauBlanc:', e)
    }
  }

  /** Effacer le tableau blanc */
  const effacerTableauBlanc = async (sessionId: string): Promise<void> => {
    try {
      await $fetch(`${apiBase}/api/afrolang/sessions/${sessionId}/tableau-blanc`, {
        method: 'DELETE',
        headers: { Authorization: `Bearer ${userStore.accessToken}` },
      })
    }
    catch (e: any) {
      console.error('Erreur effacerTableauBlanc:', e)
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerSalles,
    obtenirSalle,
    listerSallesPrivees,
    obtenirSallePrivee,
    creerSallePrivee,
    obtenirSession,
    creerSession,
    demarrerSession,
    terminerSession,
    rejoindreSession,
    quitterSession,
    genererTokenSession,
    obtenirTableauBlanc,
    sauvegarderTableauBlanc,
    effacerTableauBlanc,
    obtenirStats,
    listerLangues,
  }
}

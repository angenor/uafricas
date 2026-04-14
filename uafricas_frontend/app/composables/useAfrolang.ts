// Composable pour les appels API Afrolang (salles, groupes ethniques, feature 005)
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type EtatSession = 'planifiee' | 'en_cours' | 'terminee' | 'annulee'
export type RoleSession = 'moderateur' | 'participant' | 'observateur'

// Enums feature 005 (1:1 avec les enums SQL `afrolang.xxx`)
export type EtatProposition = 'en_attente' | 'approuvee' | 'refusee'
export type MotifSallePrivee = 'apprentissage_enfants' | 'reseautage_adulte' | 'echanges_groupe'
export type VisibiliteSallePrivee = 'fermee' | 'visible'
export type TypeAdhesion = 'demande' | 'invitation' | 'abonne'
export type EtatAdhesion = 'en_attente' | 'acceptee' | 'refusee' | 'groupe_complet'
export type TypeRessource = 'fichier' | 'lien_externe'
export type EtatRessource = 'publiee' | 'en_attente_validation' | 'refusee'

/** Utilisateur resume (JOIN depuis le backend) */
export interface AfrolangUser {
  id: string
  nom: string
  prenom: string | null
  photo_url: string | null
}

/** Résumé d'un groupe ethnique dans l'annuaire Afrolang */
export interface GroupeEthniqueResume {
  id: string
  nom: string
  fiche_pays_id: string
  pays_id: string | null
  pays_nom: string | null
  salle_id: string | null
  salle_slug: string | null
  salle_active: boolean
  proposition_en_attente: boolean
}

/** Groupe ethnique allégé (inclus dans la réponse Salle) */
export interface GroupeEthniqueLight {
  id: string
  nom: string
  fiche_pays_id: string | null
  pays_nom: string | null
}

/** Modérateur Afrolang attitré (table salle_moderateur) */
export interface ModerateurAttitre {
  id: string
  salle_id: string
  utilisateur_id: string
  nom: string | null
  prenom: string | null
  photo_url: string | null
  email: string | null
  disponibilite: string | null
  designe_at: string
  actif: boolean
}

/** DTO salle publique (liste) — feature 005 */
export interface SalleAPI {
  id: string
  titre: string
  slug: string | null
  description: string | null
  image_couverture_url: string | null
  langue_cible: string | null
  langue_code: string | null
  alphabet: string | null
  dictionnaire_url: string | null
  groupe_ethnique_id: string
  groupe_ethnique: GroupeEthniqueLight | null
  actif: boolean
  nombre_salles_privees: number
  sessions_en_cours: number
  nombre_moderateurs_attitres: number
  ressources_count: number
  created_at: string
  updated_at: string
}

/** DTO salle publique (detail) */
export interface SalleDetailAPI extends SalleAPI {
  moderateurs_attitres: ModerateurAttitre[]
  salles_privees: SallePriveeAPI[]
}

/** DTO salle privée (feature 005) */
export interface SallePriveeAPI {
  id: string
  salle_id: string
  titre: string
  description: string | null
  image_couverture_url: string | null
  max_participants: number | null
  motif: MotifSallePrivee
  declaration_adulte_at: string
  visibilite: VisibiliteSallePrivee
  archivee_at: string | null
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

/** Proposition de salle publique */
export interface PropositionSalleAPI {
  id: string
  nom_groupe_ethnique: string
  pays_id: string | null
  groupe_ethnique_id: string | null
  langue_cible: string | null
  description: string | null
  etat: EtatProposition
  motif_refus: string | null
  salle_id_creee: string | null
  propose_par: string
  decide_par: string | null
  decide_at: string | null
  created_at: string
  updated_at: string
}

/** Adhésion à une salle privée */
export interface AdhesionSallePriveeAPI {
  id: string
  salle_privee_id: string
  utilisateur_id: string
  utilisateur_nom: string | null
  utilisateur_prenom: string | null
  utilisateur_photo: string | null
  type_adhesion: TypeAdhesion
  etat: EtatAdhesion
  initiateur_id: string
  decideur_id: string | null
  decided_at: string | null
  created_at: string
  updated_at: string
}

/** Ressource pédagogique d'une salle */
export interface RessourceSalleAPI {
  id: string
  salle_id: string
  titre: string
  description: string | null
  type_ressource: TypeRessource
  fichier_url: string | null
  lien_url: string | null
  etat: EtatRessource
  motif_refus: string | null
  ajoute_par: string
  auteur_nom: string | null
  auteur_prenom: string | null
  valide_par: string | null
  valide_at: string | null
  created_at: string
  updated_at: string
}

/** Message de la messagerie écrite (session) */
export interface MessageSessionAPI {
  id: string
  session_id: string
  auteur_id: string
  auteur_nom: string | null
  auteur_prenom: string | null
  auteur_photo: string | null
  contenu: string
  created_at: string
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
  donnees: Record<string, unknown>
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

// ── Enveloppes paginées ─────────────────────────────────────────
interface PageMeta {
  total: number
  page: number
  par_page: number
  total_pages: number
}

export interface SalleListeAPI extends PageMeta {
  salles: SalleAPI[]
}

export interface SallePriveeListeAPI extends PageMeta {
  salles_privees: SallePriveeAPI[]
}

export interface SessionListeAPI extends PageMeta {
  sessions: SessionAPI[]
}

export interface GroupeEthniqueListeAPI extends PageMeta {
  groupes: GroupeEthniqueResume[]
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
  langue_code?: string
  groupe_ethnique_id?: string
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

/** Filtres pour l'annuaire des groupes ethniques */
export interface GroupeEthniqueFiltres {
  q?: string
  pays_id?: string
  page?: number
  par_page?: number
}

/** Formulaire creation salle privee (feature 005).
 *  Les champs feature 005 (motif, declaration_adulte, visibilite) seront rendus
 *  obligatoires côté UI une fois US4 livré. Jusque-là, le composable applique
 *  des valeurs par défaut sûres pour ne pas casser l'UX existante. */
export interface CreerSallePriveeForm {
  titre: string
  description: string
  code_acces: string
  max_participants: number | null
  motif?: MotifSallePrivee
  declaration_adulte?: boolean
  visibilite?: VisibiliteSallePrivee
}

/** Formulaire creation session */
export interface CreerSessionForm {
  titre: string
  date_debut_prevue: string
  max_participants: number | null
  tableau_blanc_actif: boolean
}

/** Formulaire proposition de salle */
export interface ProposerSalleForm {
  nom_groupe_ethnique: string
  pays_id: string | null
  groupe_ethnique_id: string | null
  langue_cible: string | null
  description: string | null
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

export const MOTIFS_SALLE_PRIVEE: { value: MotifSallePrivee; label: string }[] = [
  { value: 'apprentissage_enfants', label: 'Apprentissage enfants' },
  { value: 'reseautage_adulte', label: 'Réseautage entre adultes' },
  { value: 'echanges_groupe', label: 'Échanges de groupe' },
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

  // ── Annuaire groupes ethniques (US1) ──────────────────────

  /** Lister les groupes ethniques avec état de salle */
  const listerGroupesEthniques = async (
    filtres: GroupeEthniqueFiltres = {},
  ): Promise<GroupeEthniqueListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.q) params.set('q', filtres.q)
      if (filtres.pays_id) params.set('pays_id', filtres.pays_id)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/afrolang/groupes-ethniques${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<GroupeEthniqueListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des groupes ethniques')
      }

      return reponse.data
    }
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur reseau')
      erreur.value = message
      console.error('Erreur listerGroupesEthniques:', e)
      return null
    }
    finally {
      chargement.value = false
    }
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
      if (filtres.langue_code) params.set('langue_code', filtres.langue_code)
      if (filtres.groupe_ethnique_id) params.set('groupe_ethnique_id', filtres.groupe_ethnique_id)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/afrolang/salles${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<SalleListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des salles')
      }

      for (const salle of reponse.data.salles) {
        salle.image_couverture_url = resoudreUrl(salle.image_couverture_url)
      }

      return reponse.data
    }
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur reseau')
      erreur.value = message
      console.error('Erreur listerSalles:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Obtenir le detail d'une salle publique (feature 005 : modérateurs attitrés) */
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

      reponse.data.image_couverture_url = resoudreUrl(reponse.data.image_couverture_url)
      for (const mod of reponse.data.moderateurs_attitres) {
        mod.photo_url = resoudreUrl(mod.photo_url)
      }
      for (const sp of reponse.data.salles_privees) {
        sp.image_couverture_url = resoudreUrl(sp.image_couverture_url)
        if (sp.createur.photo_url) {
          sp.createur.photo_url = resoudreUrl(sp.createur.photo_url)
        }
      }

      return reponse.data
    }
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur reseau')
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
  const listerSallesPrivees = async (
    salleId: string,
    filtres: SallePriveeFiltres = {},
  ): Promise<SallePriveeListeAPI | null> => {
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
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur reseau')
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
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur reseau')
      erreur.value = message
      console.error('Erreur obtenirSallePrivee:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Créer une salle privée (feature 005 : motif + déclaration adulte obligatoires).
   *  Retourne `{ erreur: 'salle_privee_unicite', salle_existante_id? }` si 409. */
  const creerSallePrivee = async (
    salleId: string,
    form: CreerSallePriveeForm,
  ): Promise<SallePriveeAPI | { erreur: 'salle_privee_unicite'; salle_existante_id?: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const body: Record<string, unknown> = {
        titre: form.titre,
        motif: form.motif ?? 'reseautage_adulte',
        declaration_adulte: form.declaration_adulte ?? true,
        visibilite: form.visibilite ?? 'fermee',
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
        throw new Error(reponse.error || 'Erreur lors de la création de la salle privée')
      }

      return reponse.data
    }
    catch (e: unknown) {
      const anyErr = e as { status?: number; data?: { data?: { erreur?: string; salle_existante_id?: string }; error?: string } }
      if (anyErr.status === 409 && anyErr.data?.data?.erreur === 'salle_privee_unicite') {
        erreur.value = anyErr.data?.error || 'Salle privée déjà existante'
        return {
          erreur: 'salle_privee_unicite',
          salle_existante_id: anyErr.data?.data?.salle_existante_id,
        }
      }
      const message = extraireMessage(e, 'Erreur réseau')
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
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur reseau')
      erreur.value = message
      console.error('Erreur obtenirSession:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Creer une session dans une salle privee */
  const creerSession = async (
    sallePriveeId: string,
    form: CreerSessionForm,
  ): Promise<SessionAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const body: Record<string, unknown> = {}
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
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur reseau')
      erreur.value = message
      console.error('Erreur creerSession:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Feature 005 — Option A : sessions de salle publique ──────────────────

  const creerSessionSallePublique = async (
    salleId: string,
    form: CreerSessionForm,
  ): Promise<SessionAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const body: Record<string, unknown> = {}
      if (form.titre) body.titre = form.titre
      if (form.date_debut_prevue) body.date_debut_prevue = form.date_debut_prevue
      if (form.max_participants) body.max_participants = form.max_participants
      if (form.tableau_blanc_actif !== undefined) body.tableau_blanc_actif = form.tableau_blanc_actif

      const reponse = await $fetch<ApiResponse<SessionAPI>>(
        `${apiBase}/api/afrolang/salles/${salleId}/sessions`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body,
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la création de la session')
      }
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur réseau')
      console.error('Erreur creerSessionSallePublique:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  const listerSessionsSallePublique = async (
    salleId: string,
    filtres: SessionFiltres = {},
  ): Promise<SessionListeAPI | null> => {
    erreur.value = null
    try {
      const params = new URLSearchParams()
      if (filtres.etat) params.set('etat', filtres.etat)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))
      const qs = params.toString()

      const reponse = await $fetch<ApiResponse<SessionListeAPI>>(
        `${apiBase}/api/afrolang/salles/${salleId}/sessions${qs ? `?${qs}` : ''}`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement')
      }
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur réseau')
      console.error('Erreur listerSessionsSallePublique:', e)
      return null
    }
  }

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
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur reseau')
      return false
    }
  }

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
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur reseau')
      return false
    }
  }

  const rejoindreSession = async (sessionId: string, codeAcces?: string): Promise<boolean> => {
    erreur.value = null
    try {
      const body: Record<string, unknown> = {}
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
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur reseau')
      return false
    }
  }

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
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur reseau')
      return false
    }
  }

  // ── Phase 3 : Token LiveKit ──────────────────────────────

  const genererTokenSession = async (
    sessionId: string,
    codeAcces?: string,
  ): Promise<TokenResponse | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const body: Record<string, unknown> = {}
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
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur reseau')
      erreur.value = message
      console.error('Erreur genererTokenSession:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ── Utilitaires ───────────────────────────────────────────

  const obtenirStats = async (): Promise<AfrolangStats | null> => {
    try {
      const reponse = await $fetch<ApiResponse<AfrolangStats>>(
        `${apiBase}/api/afrolang/stats`,
      )
      if (!reponse.success || !reponse.data) return null
      return reponse.data
    }
    catch (e: unknown) {
      console.error('Erreur obtenirStats:', e)
      return null
    }
  }

  const listerLangues = async (): Promise<string[]> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/afrolang/langues`,
      )
      if (!reponse.success || !reponse.data) return []
      return reponse.data
    }
    catch (e: unknown) {
      console.error('Erreur listerLangues:', e)
      return []
    }
  }

  // ── Phase 4 : Tableau blanc ──

  const obtenirTableauBlanc = async (sessionId: string): Promise<TableauBlancData> => {
    try {
      const reponse = await $fetch<ApiResponse<TableauBlancData>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/tableau-blanc`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) return { donnees: {}, version: 0 }
      return reponse.data
    }
    catch (e: unknown) {
      console.error('Erreur obtenirTableauBlanc:', e)
      return { donnees: {}, version: 0 }
    }
  }

  const sauvegarderTableauBlanc = async (
    sessionId: string,
    donnees: Record<string, unknown>,
  ): Promise<void> => {
    try {
      await $fetch(`${apiBase}/api/afrolang/sessions/${sessionId}/tableau-blanc`, {
        method: 'PUT',
        body: donnees,
        headers: authHeaders(),
      })
    }
    catch (e: unknown) {
      console.error('Erreur sauvegarderTableauBlanc:', e)
    }
  }

  const effacerTableauBlanc = async (sessionId: string): Promise<void> => {
    try {
      await $fetch(`${apiBase}/api/afrolang/sessions/${sessionId}/tableau-blanc`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
    }
    catch (e: unknown) {
      console.error('Erreur effacerTableauBlanc:', e)
    }
  }

  // ── Feature 005 — US2 : Propositions de salles ──

  /**
   * Soumettre une proposition de nouvelle salle publique.
   * Retourne `{ erreur: 'doublon', salle_id?, proposition_id? }` en cas de 409.
   */
  const soumettrePropositionSalle = async (
    form: ProposerSalleForm,
  ): Promise<PropositionSalleAPI | { erreur: 'doublon'; salle_id?: string; proposition_id?: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const body: Record<string, unknown> = {
        nom_groupe_ethnique: form.nom_groupe_ethnique.trim(),
      }
      if (form.pays_id) body.pays_id = form.pays_id
      if (form.groupe_ethnique_id) body.groupe_ethnique_id = form.groupe_ethnique_id
      if (form.langue_cible) body.langue_cible = form.langue_cible
      if (form.description) body.description = form.description

      const reponse = await $fetch<ApiResponse<PropositionSalleAPI>>(
        `${apiBase}/api/afrolang/salles/propositions`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la soumission')
      }
      return reponse.data
    }
    catch (e: unknown) {
      // Cas du 409 : $fetch lève avec response attachée, on remonte un objet discriminé
      const anyErr = e as { status?: number; data?: { data?: { salle_id?: string; proposition_id?: string }; error?: string } }
      if (anyErr.status === 409) {
        erreur.value = anyErr.data?.error || 'Doublon détecté'
        return {
          erreur: 'doublon',
          salle_id: anyErr.data?.data?.salle_id,
          proposition_id: anyErr.data?.data?.proposition_id,
        }
      }
      const message = extraireMessage(e, 'Erreur réseau')
      erreur.value = message
      console.error('Erreur soumettrePropositionSalle:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Liste des propositions soumises par l'utilisateur courant. */
  const listerMesPropositions = async (): Promise<PropositionSalleAPI[]> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<PropositionSalleAPI[]>>(
        `${apiBase}/api/afrolang/salles/propositions/mine`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement')
      }
      return reponse.data
    }
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur réseau')
      erreur.value = message
      console.error('Erreur listerMesPropositions:', e)
      return []
    }
    finally {
      chargement.value = false
    }
  }

  // ── Feature 005 — US3 : Modération de session ──

  /** Transférer manuellement la modération de session à un autre participant. */
  const transfererModerationSession = async (
    sessionId: string,
    destinataireId: string,
  ): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/moderation/transferer`,
        {
          method: 'PUT',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { destinataire_id: destinataireId },
        },
      )
      if (!reponse.success) throw new Error(reponse.error || 'Erreur')
      return true
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur réseau')
      console.error('Erreur transfererModerationSession:', e)
      return false
    }
  }

  // ── Feature 005 — US5 : Adhésions / invitations / visibilité ──────────

  type AdhesionResultat =
    | { succes: true; id: string; etat: EtatAdhesion }
    | { erreur: 'adhesion_existante'; adhesion_id: string; type: TypeAdhesion; etat: EtatAdhesion }
    | null

  const demanderAdhesion = async (
    salleePriveeId: string,
  ): Promise<AdhesionResultat> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string; etat: EtatAdhesion }>>(
        `${apiBase}/api/afrolang/salles-privees/${salleePriveeId}/demandes`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: {},
        },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Erreur')
      return { succes: true, id: reponse.data.id, etat: reponse.data.etat }
    }
    catch (e: unknown) {
      const anyErr = e as { status?: number; data?: { data?: { adhesion_id?: string; type?: TypeAdhesion; etat?: EtatAdhesion }; error?: string } }
      if (anyErr.status === 409 && anyErr.data?.data?.adhesion_id) {
        return {
          erreur: 'adhesion_existante',
          adhesion_id: anyErr.data.data.adhesion_id,
          type: anyErr.data.data.type || 'demande',
          etat: anyErr.data.data.etat || 'en_attente',
        }
      }
      erreur.value = extraireMessage(e, 'Erreur lors de la demande')
      console.error('Erreur demanderAdhesion:', e)
      return null
    }
  }

  const inviterMembre = async (
    salleePriveeId: string,
    utilisateurId: string,
  ): Promise<{ id: string } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/api/afrolang/salles-privees/${salleePriveeId}/invitations`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { utilisateur_id: utilisateurId },
        },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Erreur')
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de l\'invitation')
      console.error('Erreur inviterMembre:', e)
      return null
    }
  }

  const decisionAdhesion = async (
    adhesionId: string,
    decision: 'acceptee' | 'refusee',
  ): Promise<{ etat: EtatAdhesion } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string; etat: EtatAdhesion }>>(
        `${apiBase}/api/afrolang/adhesions/${adhesionId}/decision`,
        {
          method: 'PATCH',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { decision },
        },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Erreur')
      return { etat: reponse.data.etat }
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de la décision')
      console.error('Erreur decisionAdhesion:', e)
      return null
    }
  }

  const listerAdhesions = async (
    salleePriveeId: string,
  ): Promise<AdhesionSallePriveeAPI[]> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<AdhesionSallePriveeAPI[]>>(
        `${apiBase}/api/afrolang/salles-privees/${salleePriveeId}/adhesions`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Erreur')
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors du chargement')
      console.error('Erreur listerAdhesions:', e)
      return []
    }
  }

  const retirerAbonne = async (adhesionId: string): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/afrolang/adhesions/${adhesionId}`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
      return true
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors du retrait')
      console.error('Erreur retirerAbonne:', e)
      return false
    }
  }

  const changerVisibiliteSallePrivee = async (
    id: string,
    visibilite: VisibiliteSallePrivee,
  ): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/afrolang/salles-privees/${id}/visibilite`,
        {
          method: 'PATCH',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { visibilite },
        },
      )
      if (!reponse.success) throw new Error(reponse.error || 'Erreur')
      return true
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors du changement de visibilité')
      console.error('Erreur changerVisibiliteSallePrivee:', e)
      return false
    }
  }

  const modifierMaxParticipantsSallePrivee = async (
    id: string,
    max: number,
  ): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/afrolang/salles-privees/${id}/max-participants`,
        {
          method: 'PATCH',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { max_participants: max },
        },
      )
      if (!reponse.success) throw new Error(reponse.error || 'Erreur')
      return true
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de la mise à jour de la limite')
      console.error('Erreur modifierMaxParticipants:', e)
      return false
    }
  }

  // ── Feature 005 — US6 : Ressources et messagerie ──────────────────────

  const listerRessources = async (salleId: string): Promise<RessourceSalleAPI[]> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<RessourceSalleAPI[]>>(
        `${apiBase}/api/afrolang/salles/${salleId}/ressources`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Erreur')
      return reponse.data.map(r => ({
        ...r,
        fichier_url: resoudreUrl(r.fichier_url ?? null),
      }))
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors du chargement')
      console.error('Erreur listerRessources:', e)
      return []
    }
  }

  const uploaderRessourceFichier = async (
    salleId: string,
    fichier: File,
    titre: string,
    description?: string,
  ): Promise<{ id: string; fichier_url: string } | null> => {
    erreur.value = null
    try {
      const form = new FormData()
      form.append('titre', titre)
      if (description) form.append('description', description)
      form.append('fichier', fichier)

      const reponse = await $fetch<ApiResponse<{ id: string; fichier_url: string }>>(
        `${apiBase}/api/afrolang/salles/${salleId}/ressources/fichier`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: form,
        },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Erreur')
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de l\'envoi')
      console.error('Erreur uploaderRessourceFichier:', e)
      return null
    }
  }

  const soumettreLienExterne = async (
    salleId: string,
    titre: string,
    url: string,
    description?: string,
  ): Promise<{ id: string } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/api/afrolang/salles/${salleId}/ressources/lien`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { titre, lien_url: url, description },
        },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Erreur')
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de la soumission')
      console.error('Erreur soumettreLienExterne:', e)
      return null
    }
  }

  const supprimerRessource = async (ressourceId: string): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/afrolang/ressources/${ressourceId}`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
      return true
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de la suppression')
      console.error('Erreur supprimerRessource:', e)
      return false
    }
  }

  const listerMessagesSession = async (
    sessionId: string,
    options?: { since?: string; limit?: number },
  ): Promise<MessageSessionAPI[]> => {
    erreur.value = null
    try {
      const params = new URLSearchParams()
      if (options?.since) params.set('since', options.since)
      if (options?.limit) params.set('limit', String(options.limit))
      const qs = params.toString()

      const reponse = await $fetch<ApiResponse<MessageSessionAPI[]>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/messages${qs ? `?${qs}` : ''}`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Erreur')
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors du chargement')
      console.error('Erreur listerMessagesSession:', e)
      return []
    }
  }

  const envoyerMessageSession = async (
    sessionId: string,
    contenu: string,
  ): Promise<MessageSessionAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<MessageSessionAPI>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/messages`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { contenu },
        },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Erreur')
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de l\'envoi')
      console.error('Erreur envoyerMessageSession:', e)
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    // Annuaire (US1)
    listerGroupesEthniques,
    // Salles
    listerSalles,
    obtenirSalle,
    listerSallesPrivees,
    obtenirSallePrivee,
    creerSallePrivee,
    // Sessions
    obtenirSession,
    creerSession,
    creerSessionSallePublique,
    listerSessionsSallePublique,
    demarrerSession,
    terminerSession,
    rejoindreSession,
    quitterSession,
    genererTokenSession,
    // Tableau blanc
    obtenirTableauBlanc,
    sauvegarderTableauBlanc,
    effacerTableauBlanc,
    // Stats
    obtenirStats,
    listerLangues,
    // Stubs (à implémenter dans US2..US6)
    soumettrePropositionSalle,
    listerMesPropositions,
    transfererModerationSession,
    demanderAdhesion,
    inviterMembre,
    decisionAdhesion,
    listerAdhesions,
    retirerAbonne,
    changerVisibiliteSallePrivee,
    modifierMaxParticipantsSallePrivee,
    listerRessources,
    uploaderRessourceFichier,
    soumettreLienExterne,
    supprimerRessource,
    listerMessagesSession,
    envoyerMessageSession,
  }
}

// ──────────────────────────────────────────────────────────────
// Helpers (locaux)
// ──────────────────────────────────────────────────────────────

function extraireMessage(e: unknown, defaut: string): string {
  if (typeof e === 'object' && e !== null) {
    const obj = e as { data?: { error?: string }; message?: string }
    return obj.data?.error || obj.message || defaut
  }
  return defaut
}

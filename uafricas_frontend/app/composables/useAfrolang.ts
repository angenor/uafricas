// Composable pour les appels API Afrolang (salles, groupes ethniques, feature 005)
import { useUserStore } from '~/stores/user'
import type { Room } from 'livekit-client'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type EtatSession = 'planifiee' | 'en_cours' | 'terminee' | 'annulee'
export type RoleSession = 'moderateur' | 'participant' | 'observateur'

// Enums feature 005 (1:1 avec les enums SQL `afrolang.xxx`)
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

/** Pays d'origine d'une salle publique (feature 001-afrolang-pays-origine). */
export interface PaysOrigineLight {
  id: string
  nom: string
  code_iso2: string | null
}

/** Administrateur d'une salle publique (vue allégée publique).
 *  Feature 001-admin-salles-publiques. */
export interface AdministrateurLight {
  utilisateur_id: string
  nom: string
  prenom: string
  photo_url: string | null
  nomme_at: string
}

/** Statut d'une proposition communautaire de salle publique. */
export type StatutProposition = 'en_attente' | 'validee' | 'rejetee' | 'retiree'

/** DTO d'une proposition de salle (public et admin) — feature 001-admin-salles-publiques. */
export interface PropositionSalle {
  id: string
  auteur: { id: string; nom: string; prenom: string }
  titre: string
  description: string
  justification: string
  langue_cible: string
  langue_code: string | null
  groupe_ethnique: { id: string; nom: string }
  pays_origine: PaysOrigineLight[]
  statut: StatutProposition
  decideur: { id: string; nom: string; prenom: string } | null
  decide_at: string | null
  commentaire_decision: string | null
  salle_id_creee: string | null
  created_at: string
  updated_at: string
}

export interface PropositionListeAPI {
  items: PropositionSalle[]
  total: number
  page: number
  taille: number
}

export interface PropositionMesFiltres {
  statut?: StatutProposition
  page?: number
  taille?: number
}

export interface SoumettrePropositionPayload {
  titre: string
  description: string
  justification: string
  langue_cible: string
  langue_code?: string | null
  groupe_ethnique_id: string
  pays_origine_ids: string[]
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
  pays_origine: PaysOrigineLight[]
  administrateurs: AdministrateurLight[]
  created_at: string
  updated_at: string
}

/** DTO salle publique (detail) */
export interface SalleDetailAPI extends SalleAPI {
  moderateurs_attitres: ModerateurAttitre[]
}

/** DTO salle privée (feature 001-afrolang-salles-refonte).
 *  Aligné 1:1 sur `SallePriveeAPI` Rust (src/models/afrolang.rs). */
export interface SallePriveeAPI {
  id: string
  salle_id: string
  titre: string
  description: string | null
  auteur_id: string
  auteur_nom: string | null
  session_en_cours: boolean
  est_auteur: boolean
  created_at: string
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
  /** Feature 001-session-moderation (FR-024) — état spotlight initial à la connexion. */
  spotlight?: SpotlightInfoAPI | null
  /** Feature 001-session-moderation — nombre de permissions tableau blanc actives. */
  permissions_tableau_blanc_count?: number
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

export interface SessionListeAPI extends PageMeta {
  sessions: SessionAPI[]
}

export interface GroupeEthniqueListeAPI extends PageMeta {
  groupes: GroupeEthniqueResume[]
}

// ──────────────────────────────────────────────────────────────
// Feature 001-session-moderation — permissions tableau blanc + spotlight
// ──────────────────────────────────────────────────────────────

/** Niveau de modérateur de session calculé côté serveur (FR-001/FR-001b). */
export type NiveauModerateur =
  | 'admin_plateforme'
  | 'admin_salle'
  | 'moderateur_attitre'
  | 'createur_salle_privee'

/** Modérateur d'office présent dans la session (lecture). */
export interface ModerateurOfficeAPI {
  utilisateur_id: string
  nom_complet: string
  avatar_url: string | null
  niveau: NiveauModerateur
}

/** Permission individuelle d'écriture sur le tableau blanc. */
export interface PermissionTableauBlancAPI {
  utilisateur_id: string
  nom_complet: string
  avatar_url: string | null
  accorde_par: string
  accorde_at: string
}

/** Réponse complète GET /sessions/{id}/permissions-tableau-blanc. */
export interface PermissionsTableauBlancListeAPI {
  session_id: string
  moderateurs_office: ModerateurOfficeAPI[]
  permissions_individuelles: PermissionTableauBlancAPI[]
  mon_niveau_moderateur: NiveauModerateur | null
}

/** Spotlight courant sur une session publique livestreamée. */
export interface SpotlightInfoAPI {
  utilisateur_id: string
  nom_complet: string
  avatar_url: string | null
  mis_en_evidence_par: string
  mis_en_evidence_at: string
}

/** DataPacket LiveKit `moderation.permission_update`. */
export interface ModerationPermissionPayload {
  session_id: string
  utilisateur_id: string
  action: 'accordee' | 'retiree'
  accorde_par: string
  accorde_at: string
}

/** Enveloppe générique des DataPacket de modération. */
export interface ModerationDataPacket {
  type: 'moderation'
  subtype: 'permission_update' | 'spotlight'
  payload: ModerationPermissionPayload | SpotlightInfoAPI | null
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
  pays_id?: string
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

/** Formulaire création salle privée (feature 001-afrolang-salles-refonte).
 *  Le `salleId` est passé en paramètre séparé à `creerSallePrivee`, pas inclus ici. */
export interface CreerSallePriveeForm {
  titre: string
  description: string
  code_acces: string
}

/** Réponse de `verifierCodeAcces` (endpoint 3) : jeton éphémère. */
export interface VerifierCodeResponse {
  salle_privee_id: string
  acces_jeton: string
  expires_at: string
}

/** Réponse de `demarrerOuRejoindreSallePrivee` (endpoint 4) : infos LiveKit. */
export interface DemarrerRejoindrePriveeResponse {
  session_id: string
  livekit_url: string
  livekit_token: string
  moderateur_id: string | null
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
// Mémoire des jetons d'accès (portée session applicative, non persistée)
// ──────────────────────────────────────────────────────────────

/** Cache en mémoire des jetons d'accès aux salles privées.
 *  Clé : `sallePriveeId`. Valeur : `{ jeton, expiresAt }`.
 *  Volontairement non persisté dans localStorage (sécurité : le jeton
 *  ne doit pas survivre au cycle de vie de la session applicative). */
const accesJetons = new Map<string, { jeton: string; expiresAt: string }>()

/** Mémoriser le jeton d'accès retourné par `verifierCodeAcces`. */
const memoriserAccesJeton = (
  sallePriveeId: string,
  jeton: string,
  expiresAt: string,
): void => {
  accesJetons.set(sallePriveeId, { jeton, expiresAt })
}

/** Récupérer le jeton mémorisé pour une salle privée, ou `null` s'il est
 *  absent ou expiré. L'entrée expirée est purgée au passage. */
const recupererAccesJeton = (
  sallePriveeId: string,
): { jeton: string; expiresAt: string } | null => {
  const entree = accesJetons.get(sallePriveeId)
  if (!entree) return null

  const expire = new Date(entree.expiresAt).getTime()
  if (!Number.isFinite(expire) || expire <= Date.now()) {
    accesJetons.delete(sallePriveeId)
    return null
  }
  return entree
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

  // ── Feature 001-session-moderation : état partagé ──────────
  const monNiveauModerateurSession = ref<NiveauModerateur | null>(null)
  const permissionsTableauBlanc = ref<PermissionTableauBlancAPI[]>([])
  const moderateursOffice = ref<ModerateurOfficeAPI[]>([])
  const spotlightActif = ref<SpotlightInfoAPI | null>(null)

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
      if (filtres.pays_id) params.set('pays_id', filtres.pays_id)
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

  // ── Salles privées (feature 001-afrolang-salles-refonte) ─────────────────

  /** Lister les salles privées d'une salle publique (refonte : pas de pagination). */
  const listerSallesPriveesParSallePublique = async (
    salleId: string,
  ): Promise<SallePriveeAPI[]> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<SallePriveeAPI[]>>(
        `${apiBase}/api/afrolang/salles/${salleId}/salles-privees`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des salles privées')
      }

      return reponse.data
    }
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur réseau')
      erreur.value = message
      console.error('Erreur listerSallesPriveesParSallePublique:', e)
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /** Obtenir le detail d'une salle privee (endpoint legacy — sera supprimé par la refonte). */
  const obtenirSallePrivee = async (id: string): Promise<SallePriveeDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<SallePriveeDetailAPI>>(
        `${apiBase}/api/afrolang/salles-privees/${id}`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Salle privee non trouvee')
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

  /** Créer une salle privée (refonte : code secret obligatoire).
   *  Retourne `{ erreur: 'salle_privee_unicite', salle_privee_existante_id? }` si 409. */
  const creerSallePrivee = async (
    salleId: string,
    form: CreerSallePriveeForm,
  ): Promise<SallePriveeAPI | { erreur: 'salle_privee_unicite'; salle_privee_existante_id?: string } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const body: Record<string, unknown> = {
        salle_id: salleId,
        titre: form.titre,
        code_acces: form.code_acces,
      }
      if (form.description) body.description = form.description

      const reponse = await $fetch<ApiResponse<SallePriveeAPI>>(
        `${apiBase}/api/afrolang/salles-privees`,
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
      const anyErr = e as {
        status?: number
        data?: {
          data?: { salle_privee_existante_id?: string }
          error?: string
          message?: string
        }
      }
      if (anyErr.status === 409) {
        erreur.value = anyErr.data?.error || anyErr.data?.message || 'Salle privée déjà existante'
        return {
          erreur: 'salle_privee_unicite',
          salle_privee_existante_id: anyErr.data?.data?.salle_privee_existante_id,
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

  /** Vérifier le code secret d'une salle privée → retourne un jeton éphémère. */
  const verifierCodeAcces = async (
    sallePriveeId: string,
    codeAcces: string,
  ): Promise<VerifierCodeResponse | { erreur: 'code_incorrect' } | { erreur: 'rate_limit' } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<VerifierCodeResponse>>(
        `${apiBase}/api/afrolang/salles-privees/${sallePriveeId}/verifier-code`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { code_acces: codeAcces },
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la vérification du code')
      }

      return reponse.data
    }
    catch (e: unknown) {
      const anyErr = e as { status?: number }
      if (anyErr.status === 403) {
        erreur.value = 'Code incorrect'
        return { erreur: 'code_incorrect' }
      }
      if (anyErr.status === 429) {
        erreur.value = 'Trop de tentatives, veuillez patienter quelques minutes'
        return { erreur: 'rate_limit' }
      }
      const message = extraireMessage(e, 'Erreur réseau')
      erreur.value = message
      console.error('Erreur verifierCodeAcces:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Démarrer ou rejoindre la session live d'une salle privée (jeton requis). */
  const demarrerOuRejoindreSallePrivee = async (
    sallePriveeId: string,
    accesJeton: string,
  ): Promise<DemarrerRejoindrePriveeResponse | { erreur: 'archivee' } | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<DemarrerRejoindrePriveeResponse>>(
        `${apiBase}/api/afrolang/salles-privees/${sallePriveeId}/sessions/demarrer-ou-rejoindre`,
        {
          method: 'POST',
          headers: {
            ...authHeaders(),
            'Content-Type': 'application/json',
            'X-Afrolang-Acces-Jeton': accesJeton,
          },
          body: {},
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du démarrage de la session')
      }

      return reponse.data
    }
    catch (e: unknown) {
      const anyErr = e as { status?: number }
      if (anyErr.status === 410) {
        erreur.value = 'Cette salle privée a été archivée'
        return { erreur: 'archivee' }
      }
      const message = extraireMessage(e, 'Erreur réseau')
      erreur.value = message
      console.error('Erreur demarrerOuRejoindreSallePrivee:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** US1 — Démarrer ou rejoindre le livestream public d'une salle en 1 appel. */
  const demarrerOuRejoindreSallePublique = async (
    salleId: string,
  ): Promise<DemarrerRejoindrePriveeResponse | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<DemarrerRejoindrePriveeResponse>>(
        `${apiBase}/api/afrolang/salles/${salleId}/sessions/demarrer-ou-rejoindre`,
        {
          method: 'POST',
          headers: {
            ...authHeaders(),
            'Content-Type': 'application/json',
          },
          body: {},
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du démarrage du livestream')
      }

      return reponse.data
    }
    catch (e: unknown) {
      const message = extraireMessage(e, 'Erreur réseau')
      erreur.value = message
      console.error('Erreur demarrerOuRejoindreSallePublique:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Modifier le code secret d'une salle privée (auteur uniquement). */
  const modifierCodeAcces = async (
    sallePriveeId: string,
    nouveauCodeAcces: string,
  ): Promise<boolean> => {
    chargement.value = true
    erreur.value = null

    try {
      await $fetch(`${apiBase}/api/afrolang/salles-privees/${sallePriveeId}/code-acces`, {
        method: 'PATCH',
        headers: { ...authHeaders(), 'Content-Type': 'application/json' },
        body: { nouveau_code_acces: nouveauCodeAcces },
      })
      return true
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de la modification du code')
      console.error('Erreur modifierCodeAcces:', e)
      return false
    }
    finally {
      chargement.value = false
    }
  }

  /** Archiver sa propre salle privée (auteur uniquement). */
  const archiverSallePriveeParAuteur = async (
    sallePriveeId: string,
  ): Promise<boolean> => {
    chargement.value = true
    erreur.value = null

    try {
      await $fetch(`${apiBase}/api/afrolang/salles-privees/${sallePriveeId}/archiver`, {
        method: 'POST',
        headers: authHeaders(),
      })
      return true
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors de l\'archivage')
      console.error('Erreur archiverSallePriveeParAuteur:', e)
      return false
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

  // ── Propositions communautaires (feature 001-admin-salles-publiques, US1) ──

  /** Soumet une nouvelle proposition de salle publique. */
  const proposerSalle = async (
    payload: SoumettrePropositionPayload,
  ): Promise<PropositionSalle | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<PropositionSalle>>(
        `${apiBase}/api/afrolang/propositions`,
        { method: 'POST', headers: authHeaders(), body: payload },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la soumission')
      }
      return reponse.data
    }
    catch (e: unknown) {
      const status = (e as { status?: number; statusCode?: number })?.statusCode
        ?? (e as { status?: number })?.status
      let message = extraireMessage(e, 'Erreur lors de la soumission')
      if (status === 409) {
        message = extraireMessage(e, 'Conflit : une salle ou une proposition existe déjà pour ce groupe ethnique.')
      }
      else if (status === 429) {
        message = extraireMessage(e, 'Trop de propositions rejetées récemment. Réessayez dans quelques jours.')
      }
      erreur.value = message
      console.error('Erreur proposerSalle:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Liste les propositions de l'utilisateur courant. */
  const listerMesPropositions = async (
    filtres: PropositionMesFiltres = {},
  ): Promise<PropositionListeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const params = new URLSearchParams()
      if (filtres.statut) params.set('statut', filtres.statut)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.taille) params.set('taille', String(filtres.taille))
      const qs = params.toString()
      const url = `${apiBase}/api/afrolang/propositions/moi${qs ? `?${qs}` : ''}`
      const reponse = await $fetch<ApiResponse<PropositionListeAPI>>(url, {
        headers: authHeaders(),
      })
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement')
      }
      return reponse.data
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Erreur lors du chargement')
      console.error('Erreur listerMesPropositions:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Retire une proposition encore en attente (auteur uniquement). */
  const retirerProposition = async (id: string): Promise<PropositionSalle | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<PropositionSalle>>(
        `${apiBase}/api/afrolang/propositions/${id}/retirer`,
        { method: 'PATCH', headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du retrait')
      }
      return reponse.data
    }
    catch (e: unknown) {
      const status = (e as { status?: number; statusCode?: number })?.statusCode
        ?? (e as { status?: number })?.status
      let message = extraireMessage(e, 'Erreur lors du retrait')
      if (status === 409) {
        message = extraireMessage(e, 'Cette proposition a déjà été décidée et ne peut plus être retirée.')
      }
      erreur.value = message
      console.error('Erreur retirerProposition:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  // ──────────────────────────────────────────────────────────
  // Feature 001-session-moderation — méthodes
  // ──────────────────────────────────────────────────────────

  /** Récupère l'état complet (modérateurs d'office + permissions individuelles + mon niveau). */
  const listerPermissionsTableauBlanc = async (
    sessionId: string,
  ): Promise<PermissionsTableauBlancListeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<PermissionsTableauBlancListeAPI>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/permissions-tableau-blanc`,
        { headers: authHeaders() },
      )
      if (reponse.success && reponse.data) {
        moderateursOffice.value = reponse.data.moderateurs_office
        permissionsTableauBlanc.value = reponse.data.permissions_individuelles
        monNiveauModerateurSession.value = reponse.data.mon_niveau_moderateur
        return reponse.data
      }
      return null
    }
    catch (e: unknown) {
      erreur.value = extraireMessage(e, 'Impossible de charger les permissions du tableau blanc.')
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Accorde la permission d'écriture sur le tableau blanc à un participant. */
  const accorderPermissionTableauBlanc = async (
    sessionId: string,
    utilisateurId: string,
  ): Promise<PermissionTableauBlancAPI | { erreur: 'conflit' | 'autre' }> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<PermissionTableauBlancAPI>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/permissions-tableau-blanc`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { utilisateur_id: utilisateurId },
        },
      )
      if (reponse.success && reponse.data) {
        // Patch local optimiste (la DataPacket de confirmation le réappliquera de toute façon)
        const existant = permissionsTableauBlanc.value.find(p => p.utilisateur_id === utilisateurId)
        if (!existant) permissionsTableauBlanc.value = [...permissionsTableauBlanc.value, reponse.data]
        return reponse.data
      }
      return { erreur: 'autre' }
    }
    catch (e: unknown) {
      const obj = e as { status?: number; statusCode?: number }
      const code = obj.status ?? obj.statusCode
      if (code === 409) return { erreur: 'conflit' }
      erreur.value = extraireMessage(e, 'Impossible d\'accorder la permission.')
      return { erreur: 'autre' }
    }
    finally {
      chargement.value = false
    }
  }

  /** Retire la permission d'écriture précédemment accordée. */
  const retirerPermissionTableauBlanc = async (
    sessionId: string,
    utilisateurId: string,
  ): Promise<boolean | { erreur: 'conflit' | 'autre' }> => {
    chargement.value = true
    erreur.value = null
    try {
      await $fetch(
        `${apiBase}/api/afrolang/sessions/${sessionId}/permissions-tableau-blanc/${utilisateurId}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      permissionsTableauBlanc.value = permissionsTableauBlanc.value.filter(
        p => p.utilisateur_id !== utilisateurId,
      )
      return true
    }
    catch (e: unknown) {
      const obj = e as { status?: number; statusCode?: number }
      const code = obj.status ?? obj.statusCode
      if (code === 409) return { erreur: 'conflit' }
      erreur.value = extraireMessage(e, 'Impossible de retirer la permission.')
      return { erreur: 'autre' }
    }
    finally {
      chargement.value = false
    }
  }

  /** Met en évidence un participant (FR-020). Réservé admin plateforme / admin salle.
   *  Renvoie l'info spotlight créée, ou un code d'erreur si refusé serveur. */
  const mettreEnEvidence = async (
    sessionId: string,
    utilisateurId: string,
  ): Promise<SpotlightInfoAPI | { erreur: 'interdit' | 'session_privee' | 'absent' | 'autre' }> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<SpotlightInfoAPI>>(
        `${apiBase}/api/afrolang/sessions/${sessionId}/spotlight`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: { utilisateur_id: utilisateurId },
        },
      )
      if (reponse.success && reponse.data) {
        spotlightActif.value = reponse.data
        return reponse.data
      }
      erreur.value = reponse.error ?? 'Échec mise en évidence.'
      return { erreur: 'autre' }
    }
    catch (e: unknown) {
      const obj = e as { status?: number; statusCode?: number }
      const code = obj.status ?? obj.statusCode
      if (code === 403) return { erreur: 'interdit' }
      if (code === 404) return { erreur: 'absent' }
      if (code === 422) return { erreur: 'session_privee' }
      erreur.value = extraireMessage(e, 'Impossible de mettre en évidence.')
      return { erreur: 'autre' }
    }
    finally {
      chargement.value = false
    }
  }

  /** Désactive la mise en évidence active (FR-022). */
  const retirerMiseEnEvidence = async (
    sessionId: string,
  ): Promise<boolean | { erreur: 'interdit' | 'session_privee' | 'autre' }> => {
    chargement.value = true
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/afrolang/sessions/${sessionId}/spotlight`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
      spotlightActif.value = null
      return true
    }
    catch (e: unknown) {
      const obj = e as { status?: number; statusCode?: number }
      const code = obj.status ?? obj.statusCode
      if (code === 403) return { erreur: 'interdit' }
      if (code === 422) return { erreur: 'session_privee' }
      erreur.value = extraireMessage(e, 'Impossible de retirer la mise en évidence.')
      return { erreur: 'autre' }
    }
    finally {
      chargement.value = false
    }
  }

  /** Indique si l'utilisateur courant peut écrire sur le tableau blanc :
   *  - vrai s'il est modérateur de session,
   *  - ou s'il figure dans `permissionsTableauBlanc`. */
  const monEcritureAutorisee = computed<boolean>(() => {
    if (monNiveauModerateurSession.value !== null) return true
    const moi = userStore.utilisateur?.id
    if (!moi) return false
    return permissionsTableauBlanc.value.some(p => p.utilisateur_id === moi)
  })

  /** Callback appelé à réception d'un DataPacket `moderation.permission_update` (FR-014). */
  const onPermissionUpdate = (payload: ModerationPermissionPayload) => {
    if (payload.action === 'accordee') {
      const existant = permissionsTableauBlanc.value.find(p => p.utilisateur_id === payload.utilisateur_id)
      if (!existant) {
        permissionsTableauBlanc.value = [
          ...permissionsTableauBlanc.value,
          {
            utilisateur_id: payload.utilisateur_id,
            nom_complet: '',
            avatar_url: null,
            accorde_par: payload.accorde_par,
            accorde_at: payload.accorde_at,
          },
        ]
      }
    }
    else if (payload.action === 'retiree') {
      permissionsTableauBlanc.value = permissionsTableauBlanc.value.filter(
        p => p.utilisateur_id !== payload.utilisateur_id,
      )
    }
  }

  /** Callback appelé à réception d'un DataPacket `moderation.spotlight` (FR-023). */
  const onSpotlight = (payload: SpotlightInfoAPI | null) => {
    spotlightActif.value = payload
  }

  /** Attache un listener `dataReceived` sur la `Room` LiveKit pour les évènements
   *  `moderation.*`. Retourne une fonction de détachement à appeler au démontage. */
  const attacherListenerModeration = (room: Room): (() => void) => {
    const decoder = new TextDecoder()
    const handler = (payload: Uint8Array) => {
      try {
        const texte = decoder.decode(payload)
        const data = JSON.parse(texte) as ModerationDataPacket
        if (data.type !== 'moderation') return
        if (data.subtype === 'permission_update') {
          onPermissionUpdate(data.payload as ModerationPermissionPayload)
        }
        else if (data.subtype === 'spotlight') {
          onSpotlight(data.payload as SpotlightInfoAPI | null)
        }
      }
      catch {
        /* paquet ignoré */
      }
    }
    room.on('dataReceived', handler)
    return () => {
      room.off('dataReceived', handler)
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    // Annuaire (US1)
    listerGroupesEthniques,
    // Salles publiques
    listerSalles,
    obtenirSalle,
    // Salles privées (refonte : code secret + jeton d'accès)
    listerSallesPriveesParSallePublique,
    obtenirSallePrivee,
    creerSallePrivee,
    verifierCodeAcces,
    demarrerOuRejoindreSallePrivee,
    demarrerOuRejoindreSallePublique,
    modifierCodeAcces,
    archiverSallePriveeParAuteur,
    memoriserAccesJeton,
    recupererAccesJeton,
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
    transfererModerationSession,
    // Tableau blanc
    obtenirTableauBlanc,
    sauvegarderTableauBlanc,
    effacerTableauBlanc,
    // Stats
    obtenirStats,
    listerLangues,
    // Ressources et messagerie (US6)
    listerRessources,
    uploaderRessourceFichier,
    soumettreLienExterne,
    supprimerRessource,
    listerMessagesSession,
    envoyerMessageSession,
    // Propositions communautaires (US1)
    proposerSalle,
    listerMesPropositions,
    retirerProposition,
    // Feature 001-session-moderation
    monNiveauModerateurSession,
    permissionsTableauBlanc,
    moderateursOffice,
    spotlightActif,
    monEcritureAutorisee,
    listerPermissionsTableauBlanc,
    accorderPermissionTableauBlanc,
    retirerPermissionTableauBlanc,
    mettreEnEvidence,
    retirerMiseEnEvidence,
    attacherListenerModeration,
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

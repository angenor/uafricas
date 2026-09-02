// Composable admin : barème d'engagement (règles, paliers, niveaux) + journal
import { useAdmin } from '~/composables/useAdmin'

export interface AdminRegle {
  id: string
  type_action: string
  libelle: string
  points: number
  reputation_delta: number
  plafond_journalier: number | null
  plafond_mensuel: number | null
  seuil_declencheur: number | null
  categorie_id: string | null
  categorie_code: string | null
  categorie_libelle: string | null
  actif: boolean
  /** Le code émet-il réellement cette action ? Sinon la règle ne créditera jamais rien. */
  instrumentee: boolean
  nombre_mouvements: number
}

/** Une action réellement branchée dans le code (catalogue serveur, R3) */
export interface AdminActionDisponible {
  type_action: string
  libelle_defaut: string
  types_objet: string[]
  module: string
  regle_existante: boolean
}

export interface AdminCategorie {
  id: string
  code: string
  libelle: string
  description: string | null
  ordre: number
  couleur: string | null
  icone: string | null
  actif: boolean
  nombre_regles: number
  nombre_mouvements: number
}

/** `type_objet` nul = palier global ; sinon il remplace les globaux pour cette famille */
export interface AdminPalier {
  id: string
  seuil_likes: number
  points: number
  type_objet: string | null
  actif: boolean
}

export interface AdminNiveau {
  id: string
  code: string
  libelle: string
  seuil_min: number
  ordre: number
  badge_couleur: string | null
  badge_icone: string | null
}

/** Réponse des mutations de niveau : combien de comptes ont réellement basculé */
export interface AdminNiveauxRecalcules {
  niveaux: AdminNiveau[]
  comptes_recalcules: number
}

export interface AdminBadge {
  id: string
  code: string
  libelle: string
  description: string
  couleur: string | null
  icone: string | null
  manuel: boolean
  type_condition: string | null
  parametre_action: string | null
  parametre_categorie_id: string | null
  parametre_niveau_code: string | null
  seuil: number | null
  ordre: number
  actif: boolean
  nombre_detenteurs: number
}

export interface AdminJournalRow {
  id: string
  utilisateur_id: string
  utilisateur_nom: string | null
  type_action: string
  categorie_code: string | null
  categorie_libelle: string | null
  type_objet: string | null
  objet_id: string | null
  points: number
  reputation_delta: number
  solde_apres: number
  plafond_atteint: boolean
  created_at: string
}

export interface AdminJournalPage {
  elements: AdminJournalRow[]
  total: number
  page: number
  taille: number
}

interface ApiResponse<T> { success: boolean, data: T | null, error: string | null }

export const useAdminEngagement = () => {
  const { adminFetch } = useAdmin()

  // ── Règles ──
  const listerRegles = async (): Promise<AdminRegle[]> => {
    const r = await adminFetch<ApiResponse<AdminRegle[]>>('/api/admin/engagement/regles')
    return r.data ?? []
  }
  /** Catalogue des actions que le code émet réellement (évite la règle orpheline) */
  const listerActionsDisponibles = async (): Promise<AdminActionDisponible[]> => {
    const r = await adminFetch<ApiResponse<AdminActionDisponible[]>>(
      '/api/admin/engagement/actions-disponibles',
    )
    return r.data ?? []
  }
  const creerRegle = async (body: {
    type_action: string
    libelle: string
    points: number
    reputation_delta?: number
    plafond_journalier?: number | null
    plafond_mensuel?: number | null
    seuil_declencheur?: number | null
    categorie_id?: string | null
    actif?: boolean
  }): Promise<void> => {
    await adminFetch('/api/admin/engagement/regles', { method: 'POST', body })
  }
  const modifierRegle = async (id: string, patch: Partial<AdminRegle>): Promise<void> => {
    await adminFetch(`/api/admin/engagement/regles/${id}`, { method: 'PUT', body: patch })
  }
  /** Refusé (409) si la règle a déjà attribué des points : il faut alors la désactiver */
  const supprimerRegle = async (id: string): Promise<void> => {
    await adminFetch(`/api/admin/engagement/regles/${id}`, { method: 'DELETE' })
  }

  // ── Catégories de points ──
  const listerCategories = async (): Promise<AdminCategorie[]> => {
    const r = await adminFetch<ApiResponse<AdminCategorie[]>>('/api/admin/engagement/categories')
    return r.data ?? []
  }
  const creerCategorie = async (body: {
    code: string
    libelle: string
    description?: string | null
    ordre?: number
    couleur?: string | null
    icone?: string | null
    actif?: boolean
  }): Promise<void> => {
    await adminFetch('/api/admin/engagement/categories', { method: 'POST', body })
  }
  /** `code` est immuable : il n'est pas accepté par le serveur en modification */
  const modifierCategorie = async (
    id: string,
    patch: Partial<Omit<AdminCategorie, 'id' | 'code' | 'nombre_regles' | 'nombre_mouvements'>>,
  ): Promise<void> => {
    await adminFetch(`/api/admin/engagement/categories/${id}`, { method: 'PUT', body: patch })
  }
  const supprimerCategorie = async (id: string): Promise<void> => {
    await adminFetch(`/api/admin/engagement/categories/${id}`, { method: 'DELETE' })
  }

  // ── Paliers ──
  const listerPaliers = async (): Promise<AdminPalier[]> => {
    const r = await adminFetch<ApiResponse<AdminPalier[]>>('/api/admin/engagement/paliers')
    return r.data ?? []
  }
  const creerPalier = async (
    seuil_likes: number,
    points: number,
    type_objet: string | null = null,
  ): Promise<void> => {
    await adminFetch('/api/admin/engagement/paliers', {
      method: 'POST',
      body: { seuil_likes, points, type_objet },
    })
  }
  const modifierPalier = async (
    id: string,
    patch: { points?: number, type_objet?: string | null, actif?: boolean },
  ): Promise<void> => {
    await adminFetch(`/api/admin/engagement/paliers/${id}`, { method: 'PUT', body: patch })
  }
  const desactiverPalier = async (id: string): Promise<void> => {
    await adminFetch(`/api/admin/engagement/paliers/${id}`, { method: 'DELETE' })
  }

  // ── Niveaux ──
  const listerNiveaux = async (): Promise<AdminNiveau[]> => {
    const r = await adminFetch<ApiResponse<AdminNiveau[]>>('/api/admin/engagement/niveaux')
    return r.data ?? []
  }
  const creerNiveau = async (body: {
    code: string
    libelle: string
    seuil_min: number
    badge_couleur?: string | null
    badge_icone?: string | null
  }): Promise<AdminNiveauxRecalcules | null> => {
    const r = await adminFetch<ApiResponse<AdminNiveauxRecalcules>>(
      '/api/admin/engagement/niveaux',
      { method: 'POST', body },
    )
    return r.data
  }
  const modifierNiveau = async (
    id: string,
    patch: Partial<AdminNiveau>,
  ): Promise<AdminNiveauxRecalcules | null> => {
    const r = await adminFetch<ApiResponse<AdminNiveauxRecalcules>>(
      `/api/admin/engagement/niveaux/${id}`,
      { method: 'PUT', body: patch },
    )
    return r.data
  }
  /** Refusé (409) sur le niveau plancher (seuil 0) ou s'il ne resterait qu'un niveau */
  const supprimerNiveau = async (id: string): Promise<AdminNiveauxRecalcules | null> => {
    const r = await adminFetch<ApiResponse<AdminNiveauxRecalcules>>(
      `/api/admin/engagement/niveaux/${id}`,
      { method: 'DELETE' },
    )
    return r.data
  }

  // ── Badges ──
  const listerBadges = async (): Promise<AdminBadge[]> => {
    const r = await adminFetch<ApiResponse<AdminBadge[]>>('/api/admin/engagement/badges')
    return r.data ?? []
  }
  const creerBadge = async (body: Omit<AdminBadge, 'id' | 'nombre_detenteurs'>): Promise<void> => {
    await adminFetch('/api/admin/engagement/badges', { method: 'POST', body })
  }
  /** `code` est immuable : il n'est pas accepté par le serveur en modification */
  const modifierBadge = async (
    id: string,
    patch: Partial<Omit<AdminBadge, 'id' | 'code' | 'nombre_detenteurs'>>,
  ): Promise<void> => {
    await adminFetch(`/api/admin/engagement/badges/${id}`, { method: 'PUT', body: patch })
  }
  /** Refusé (409) si le badge est détenu : il faut alors le désactiver */
  const supprimerBadge = async (id: string): Promise<void> => {
    await adminFetch(`/api/admin/engagement/badges/${id}`, { method: 'DELETE' })
  }
  /** `attribue` vaut false si le membre détenait déjà le badge (aucune notification) */
  const attribuerBadge = async (
    id: string,
    utilisateur_id: string,
    motif: string,
  ): Promise<boolean> => {
    const r = await adminFetch<ApiResponse<{ attribue: boolean }>>(
      `/api/admin/engagement/badges/${id}/attribuer`,
      { method: 'POST', body: { utilisateur_id, motif } },
    )
    return r.data?.attribue ?? false
  }
  /** Retrait tracé dans l'audit, sans notification au membre */
  const retirerBadge = async (id: string, utilisateurId: string): Promise<void> => {
    await adminFetch(`/api/admin/engagement/badges/${id}/attribuer/${utilisateurId}`, {
      method: 'DELETE',
    })
  }

  // ── Journal ──
  const listerJournal = async (params: {
    utilisateur_id?: string
    type_action?: string
    categorie?: string
    depuis?: string
    jusqu_a?: string
    page?: number
    taille?: number
  } = {}): Promise<AdminJournalPage> => {
    const r = await adminFetch<ApiResponse<AdminJournalPage>>('/api/admin/engagement/journal', { params })
    return r.data ?? { elements: [], total: 0, page: 1, taille: 30 }
  }
  const ajuster = async (utilisateur_id: string, points: number, reputation_delta: number, motif: string): Promise<void> => {
    await adminFetch('/api/admin/engagement/ajustement', {
      method: 'POST',
      body: { utilisateur_id, points, reputation_delta, motif },
    })
  }

  // ── Mise en avant d'une contribution (règle +5) ──
  const statutMiseEnAvant = async (typeObjet: string, objetId: string): Promise<boolean> => {
    const r = await adminFetch<ApiResponse<{ mis_en_avant: boolean }>>(
      `/api/admin/engagement/mise-en-avant/${typeObjet}/${objetId}`,
    )
    return r.data?.mis_en_avant ?? false
  }
  const mettreEnAvant = async (typeObjet: string, objetId: string): Promise<void> => {
    await adminFetch('/api/admin/engagement/mise-en-avant', {
      method: 'POST',
      body: { type_objet: typeObjet, objet_id: objetId },
    })
  }
  const retirerMiseEnAvant = async (typeObjet: string, objetId: string): Promise<void> => {
    await adminFetch(`/api/admin/engagement/mise-en-avant/${typeObjet}/${objetId}`, { method: 'DELETE' })
  }

  return {
    listerRegles, listerActionsDisponibles, creerRegle, modifierRegle, supprimerRegle,
    listerCategories, creerCategorie, modifierCategorie, supprimerCategorie,
    listerPaliers, creerPalier, modifierPalier, desactiverPalier,
    listerNiveaux, creerNiveau, modifierNiveau, supprimerNiveau,
    listerBadges, creerBadge, modifierBadge, supprimerBadge, attribuerBadge, retirerBadge,
    listerJournal, ajuster,
    statutMiseEnAvant, mettreEnAvant, retirerMiseEnAvant,
  }
}

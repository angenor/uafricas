// Composable admin — barème d'engagement (règles, paliers, niveaux) + journal
import { useAdmin } from '~/composables/useAdmin'

export interface AdminRegle {
  id: string
  type_action: string
  libelle: string
  points: number
  reputation_delta: number
  plafond_journalier: number | null
  plafond_mensuel: number | null
  actif: boolean
}

export interface AdminPalier {
  id: string
  seuil_likes: number
  points: number
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

export interface AdminJournalRow {
  id: string
  utilisateur_id: string
  utilisateur_nom: string | null
  type_action: string
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
  const modifierRegle = async (id: string, patch: Partial<AdminRegle>): Promise<void> => {
    await adminFetch(`/api/admin/engagement/regles/${id}`, { method: 'PUT', body: patch })
  }

  // ── Paliers ──
  const listerPaliers = async (): Promise<AdminPalier[]> => {
    const r = await adminFetch<ApiResponse<AdminPalier[]>>('/api/admin/engagement/paliers')
    return r.data ?? []
  }
  const creerPalier = async (seuil_likes: number, points: number): Promise<void> => {
    await adminFetch('/api/admin/engagement/paliers', { method: 'POST', body: { seuil_likes, points } })
  }
  const modifierPalier = async (id: string, patch: { points?: number, actif?: boolean }): Promise<void> => {
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
  const modifierNiveau = async (id: string, patch: Partial<AdminNiveau>): Promise<void> => {
    await adminFetch(`/api/admin/engagement/niveaux/${id}`, { method: 'PUT', body: patch })
  }

  // ── Journal ──
  const listerJournal = async (params: {
    utilisateur_id?: string
    type_action?: string
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
    listerRegles, modifierRegle,
    listerPaliers, creerPalier, modifierPalier, desactiverPalier,
    listerNiveaux, modifierNiveau,
    listerJournal, ajuster,
    statutMiseEnAvant, mettreEnAvant, retirerMiseEnAvant,
  }
}

// ════════════════════════════════════════════════════════════════════════════
// Mock : Collaboration et partage d'arbres
// ════════════════════════════════════════════════════════════════════════════

export type Permission = 'lecture_seule' | 'edition'
export type StatutInvitation = 'en_attente' | 'acceptee' | 'refusee' | 'expiree'

export interface Invitation {
  id: string
  arbre_id: string
  email_invite: string
  permission: Permission
  statut: StatutInvitation
  proprietaire_nom: string
  created_at: string
  expire_at?: string
}

export interface Collaborateur {
  id: string
  utilisateur_id: string
  nom: string
  prenom: string
  email: string
  permission: Permission
  created_at: string
}

export interface ArbreResume {
  id: string
  nb_personnes: number
  created_at: string
}

export interface ArbrePartage {
  arbre_id: string
  proprietaire_nom: string
  permission: Permission
  nb_personnes: number
  partage_depuis: string
}

export interface MesArbresResponse {
  mon_arbre: ArbreResume | null
  arbres_partages: ArbrePartage[]
}

export interface HistoriqueEntree {
  id: string
  action: string
  auteur_nom: string
  auteur_prenom: string
  table_cible: string
  entite_id?: string
  details_avant?: any
  details_apres?: any
  created_at: string
}

export interface HistoriqueResponse {
  entrees: HistoriqueEntree[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

// ─── Helpers ────────────────────────────────────────────────────────────

export const libellePermission = (p: Permission): string => {
  return p === 'edition' ? 'Édition' : 'Lecture seule'
}

export const libelleAction = (action: string): string => {
  const actions: Record<string, string> = {
    creer_personne: 'Personne ajoutée',
    modifier_personne: 'Personne modifiée',
    supprimer_personne: 'Personne supprimée',
    creer_lien_familial: 'Lien créé',
    supprimer_lien_familial: 'Lien supprimé',
    uploader_photo_personne: 'Photo ajoutée',
  }
  return actions[action] || action
}

export const couleurAction = (action: string): string => {
  if (action.startsWith('creer') || action.startsWith('uploader')) return 'text-green-600'
  if (action.startsWith('modifier')) return 'text-blue-600'
  if (action.startsWith('supprimer')) return 'text-red-600'
  return 'text-stone-600'
}

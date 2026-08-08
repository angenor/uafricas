// Administration des cadeaux virtuels (feature 008) — catalogue, journal
// comptable, paramètres de monétisation, purge de fin de phase de test.
//
// Bâti sur `useAdmin` : `adminFetch` porte déjà le jeton, la redirection 401 et
// le message d'accès interdit. Les montants circulent en entier et ne sont
// jamais formatés ici — `formaterMontant` de `useCadeaux` est l'unique point de
// formatage monétaire de l'application.
import type { ApiResponse } from '~/types/admin'
import type { EtatPaiement, ModeCadeau } from '~/composables/useCadeaux'

export interface AdminCadeau {
  id: string
  code: string
  libelle: string
  description: string | null
  icone: string | null
  couleur: string | null
  prix: number
  points: number
  ordre: number
  actif: boolean
  /** > 0 interdit la suppression : seule la désactivation reste possible. */
  nombre_envois: number
  montant_collecte: number
  created_at: string
  updated_at: string
}

export interface AdminCadeauPayload {
  code?: string
  libelle: string
  description?: string | null
  icone?: string | null
  couleur?: string | null
  prix: number
  points: number
  ordre?: number
  actif?: boolean
}

export interface AdminMembreBref {
  id: string
  nom_affiche: string
}

export interface AdminCibleTransaction {
  type_objet: string
  objet_id: string
  /** `null` pour un cadeau offert depuis un profil (pas de titre). */
  titre: string | null
}

export interface AdminLigneJournal {
  id: string
  created_at: string
  finalise_at: string | null
  offreur: AdminMembreBref
  beneficiaire: AdminMembreBref
  cible: AdminCibleTransaction
  cadeau: { code: string, libelle: string, icone: string | null, couleur: string | null }
  mode: ModeCadeau
  montant: number
  points: number
  taux_commission: number
  part_beneficiaire: number
  part_plateforme: number
  etat: EtatPaiement
  simule: boolean
  reference_paiement: string
}

/**
 * Totaux calculés **sur le filtre courant**, pas sur la page affichée.
 * Invariant vérifiable en recette :
 * `recettes_plateforme + cagnottes_dues = montant_total`.
 */
export interface AdminTotauxJournal {
  montant_total: number
  recettes_plateforme: number
  cagnottes_dues: number
  nombre_abouti: number
  nombre_simule: number
}

export interface AdminJournalPage {
  elements: AdminLigneJournal[]
  pagination: { page: number, taille: number, total: number }
  totaux: AdminTotauxJournal
}

export interface AdminFiltresJournal {
  membre_id?: string
  sens?: 'offreur' | 'beneficiaire'
  etat?: EtatPaiement
  mode?: ModeCadeau
  simule?: boolean
  debut?: string
  fin?: string
  page?: number
  taille?: number
}

export interface AdminParametresMonetisation {
  taux_commission: number
  devise: string
  /** Bascule CinetPay. Conditionne l'accès à la purge. */
  paiement_reel_actif: boolean
  updated_at: string
}

export interface AdminResultatPurge {
  transactions_purgees: number
  mouvements_supprimes: number
  comptes_recalcules: number
  montant_cagnottes_annule: number
}

/** Garde-fou du serveur, répété ici pour que l'écran ne puisse pas s'en écarter. */
export const CONFIRMATION_PURGE = 'PURGER'

export const useAdminCadeaux = () => {
  const { adminFetch } = useAdmin()

  const listerCadeaux = async (): Promise<AdminCadeau[]> => {
    const res = await adminFetch<ApiResponse<AdminCadeau[]>>('/api/admin/engagement/cadeaux')
    return res.data ?? []
  }

  const creerCadeau = async (payload: AdminCadeauPayload): Promise<AdminCadeau | null> => {
    const res = await adminFetch<ApiResponse<AdminCadeau>>(
      '/api/admin/engagement/cadeaux',
      { method: 'POST', body: payload },
    )
    return res.data
  }

  const modifierCadeau = async (
    id: string,
    payload: AdminCadeauPayload,
  ): Promise<AdminCadeau | null> => {
    const res = await adminFetch<ApiResponse<AdminCadeau>>(
      `/api/admin/engagement/cadeaux/${id}`,
      { method: 'PUT', body: payload },
    )
    return res.data
  }

  /** Échoue en 409 si le cadeau a déjà été offert — la contrainte SQL le garantit. */
  const supprimerCadeau = async (id: string): Promise<void> => {
    await adminFetch(`/api/admin/engagement/cadeaux/${id}`, { method: 'DELETE' })
  }

  const listerTransactions = async (
    filtres: AdminFiltresJournal = {},
  ): Promise<AdminJournalPage | null> => {
    const res = await adminFetch<ApiResponse<AdminJournalPage>>(
      '/api/admin/engagement/transactions',
      { params: filtres },
    )
    return res.data
  }

  const obtenirParametres = async (): Promise<AdminParametresMonetisation | null> => {
    const res = await adminFetch<ApiResponse<AdminParametresMonetisation>>(
      '/api/admin/engagement/parametres-monetisation',
    )
    return res.data
  }

  /** La modification du taux est **prospective** : l'historique garde son taux figé. */
  const modifierParametres = async (
    payload: Omit<AdminParametresMonetisation, 'updated_at'>,
  ): Promise<AdminParametresMonetisation | null> => {
    const res = await adminFetch<ApiResponse<AdminParametresMonetisation>>(
      '/api/admin/engagement/parametres-monetisation',
      { method: 'PUT', body: payload },
    )
    return res.data
  }

  /**
   * Purge de fin de phase de test. Refusée en 409 tant que
   * `paiement_reel_actif` est faux : purger avant le basculement rouvrirait
   * aussitôt la porte aux points gratuits.
   */
  const purgerPhaseTest = async (): Promise<AdminResultatPurge | null> => {
    const res = await adminFetch<ApiResponse<AdminResultatPurge>>(
      '/api/admin/engagement/purger-phase-test',
      { method: 'POST', body: { confirmation: CONFIRMATION_PURGE } },
    )
    return res.data
  }

  return {
    listerCadeaux,
    creerCadeau,
    modifierCadeau,
    supprimerCadeau,
    listerTransactions,
    obtenirParametres,
    modifierParametres,
    purgerPhaseTest,
  }
}

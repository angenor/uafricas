// ════════════════════════════════════════════════════════════════════════════
// Mock — Matching inter-arbres et découvertes
// Interfaces TypeScript + données de test + helpers async
// ════════════════════════════════════════════════════════════════════════════

// ─── Types ───────────────────────────────────────────────────────────────

export type StatutSuggestion = 'en_attente' | 'confirmee_de_mon_cote' | 'confirmee' | 'rejetee'
export type StatutDemande = 'en_attente' | 'acceptee' | 'refusee'

// ─── Interfaces ──────────────────────────────────────────────────────────

export interface PersonneResume {
  id: string
  rattachement_id: string
  nom: string
  prenoms?: string
  naissance_annee?: number
  naissance_lieu?: string
  genre?: string
}

export interface DetailsScore {
  nom: number
  prenoms: number
  date: number
  lieu: number
  genre: number
}

export interface SuggestionCorrespondance {
  id: string
  ma_personne: PersonneResume
  personne_matchee: PersonneResume
  score: number
  details_score: DetailsScore
  statut: StatutSuggestion
  membre_id_anonymise: string
  detectee_le: string
  confirmee_le?: string
}

export interface ArbreDecouvert {
  suggestion_id: string
  personne_commune: PersonneResume
  personnes: any[] // PersonneNoeud[]
  liens: any[] // LienArbreResponse[]
  membre_id_anonymise: string
}

export interface ProfilPublic {
  nom: string
  prenom: string
  email: string
}

export interface DemandeContact {
  id: string
  suggestion_id: string
  statut: StatutDemande
  profil_membre?: ProfilPublic | null
  created_at: string
}

export interface DecouvertesResponse {
  en_attente: { suggestions: SuggestionCorrespondance[]; total: number }
  en_cours: { suggestions: SuggestionCorrespondance[]; total: number }
  confirmees: { suggestions: SuggestionCorrespondance[]; total: number }
}

// ─── Données mock ────────────────────────────────────────────────────────

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

export const suggestionsMock: SuggestionCorrespondance[] = [
  {
    id: 'sug-0000-0000-0000-000000000001',
    ma_personne: { id: 'pers-001', rattachement_id: 'ratt-001', nom: 'Diallo', prenoms: 'Ibrahim', naissance_annee: 1850, naissance_lieu: 'Ségou, Mali', genre: 'masculin' },
    personne_matchee: { id: 'pers-101', rattachement_id: 'ratt-101', nom: 'Diallo', prenoms: 'Ibrahim', naissance_annee: 1848, naissance_lieu: 'Ségou, Mali', genre: 'masculin' },
    score: 0.87,
    details_score: { nom: 1.0, prenoms: 1.0, date: 0.83, lieu: 1.0, genre: 1.0 },
    statut: 'en_attente',
    membre_id_anonymise: 'Membre #a1b2',
    detectee_le: '2026-03-16T10:00:00Z',
  },
  {
    id: 'sug-0000-0000-0000-000000000002',
    ma_personne: { id: 'pers-003', rattachement_id: 'ratt-003', nom: 'Kouyaté', prenoms: 'Fatoumata', naissance_annee: 1885, naissance_lieu: 'Kankan, Guinée', genre: 'feminin' },
    personne_matchee: { id: 'pers-102', rattachement_id: 'ratt-102', nom: 'Kuyate', prenoms: 'Fatouma', naissance_annee: 1886, naissance_lieu: 'Kankan', genre: 'feminin' },
    score: 0.72,
    details_score: { nom: 0.65, prenoms: 0.7, date: 0.96, lieu: 0.8, genre: 1.0 },
    statut: 'en_attente',
    membre_id_anonymise: 'Membre #c3d4',
    detectee_le: '2026-03-16T11:00:00Z',
  },
  {
    id: 'sug-0000-0000-0000-000000000003',
    ma_personne: { id: 'pers-002', rattachement_id: 'ratt-002', nom: 'Diallo', prenoms: 'Ousmane', naissance_annee: 1882 },
    personne_matchee: { id: 'pers-103', rattachement_id: 'ratt-103', nom: 'Diallo', prenoms: 'Ousman', naissance_annee: 1880, naissance_lieu: 'Bamako', genre: 'masculin' },
    score: 0.68,
    details_score: { nom: 1.0, prenoms: 0.85, date: 0.83, lieu: 0.5, genre: 1.0 },
    statut: 'confirmee_de_mon_cote',
    membre_id_anonymise: 'Membre #e5f6',
    detectee_le: '2026-03-15T08:00:00Z',
  },
]

// ─── Helpers async ───────────────────────────────────────────────────────

export const listerDecouvertesMock = async (): Promise<DecouvertesResponse> => {
  await delay(200)
  return {
    en_attente: {
      suggestions: suggestionsMock.filter((s) => s.statut === 'en_attente'),
      total: 2,
    },
    en_cours: {
      suggestions: suggestionsMock.filter((s) => s.statut === 'confirmee_de_mon_cote'),
      total: 1,
    },
    confirmees: {
      suggestions: suggestionsMock.filter((s) => s.statut === 'confirmee'),
      total: 0,
    },
  }
}

/** Formate un score en pourcentage lisible */
export const formaterScore = (score: number): string => {
  return `${Math.round(score * 100)}%`
}

/** Couleur du score selon la confiance */
export const couleurScore = (score: number): string => {
  if (score >= 0.8) return 'text-green-600'
  if (score >= 0.6) return 'text-amber-600'
  return 'text-red-600'
}

/** Libellé lisible du statut */
export const libelleStatut = (statut: StatutSuggestion): string => {
  const libelles: Record<StatutSuggestion, string> = {
    en_attente: 'En attente',
    confirmee_de_mon_cote: 'Confirmé de votre côté',
    confirmee: 'Confirmée mutuellement',
    rejetee: 'Rejetée',
  }
  return libelles[statut]
}

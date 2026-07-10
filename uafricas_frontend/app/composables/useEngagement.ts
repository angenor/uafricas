// Composable public — Système d'engagement (gamification)
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types (reflètent les DTO backend `models/engagement.rs`)
// ──────────────────────────────────────────────────────────────

/** Niveau/statut d'un membre */
export interface NiveauInfo {
  code: string
  libelle: string
  seuil_min: number
  badge_couleur: string | null
  badge_icone: string | null
}

/** Prochain niveau à atteindre */
export interface ProchainNiveau {
  code: string
  libelle: string
  seuil_min: number
  points_restants: number
}

/** Compte d'engagement du membre connecté */
export interface CompteEngagement {
  solde_points: number
  solde_points_mensuel: number
  reputation: number
  niveau: NiveauInfo
  prochain_niveau: ProchainNiveau | null
  dernier_mouvement_at: string | null
}

/** Un mouvement de points (entrée de journal) */
export interface MouvementPoints {
  id: string
  type_action: string
  libelle: string | null
  type_objet: string | null
  objet_id: string | null
  points: number
  reputation_delta: number
  solde_apres: number
  plafond_atteint: boolean
  created_at: string
}

/** Page du journal des points */
export interface JournalPage {
  elements: MouvementPoints[]
  total: number
  page: number
  taille: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export const useEngagement = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  /** GET /api/engagement/mon-compte */
  const obtenirMonCompte = async (): Promise<CompteEngagement | null> => {
    const res = await $fetch<ApiResponse<CompteEngagement>>(
      `${apiBase}/api/engagement/mon-compte`,
      { headers: authHeaders() },
    )
    return res.data
  }

  /** GET /api/engagement/mon-journal */
  const listerMonJournal = async (
    page = 1,
    taille = 20,
    typeAction?: string,
  ): Promise<JournalPage | null> => {
    const query: Record<string, string | number> = { page, taille }
    if (typeAction) query.type_action = typeAction
    const res = await $fetch<ApiResponse<JournalPage>>(
      `${apiBase}/api/engagement/mon-journal`,
      { headers: authHeaders(), query },
    )
    return res.data
  }

  /** GET /api/engagement/niveau/{utilisateurId} — badge public léger */
  const obtenirNiveau = async (utilisateurId: string): Promise<NiveauInfo | null> => {
    const res = await $fetch<ApiResponse<NiveauInfo>>(
      `${apiBase}/api/engagement/niveau/${utilisateurId}`,
    )
    return res.data
  }

  return { obtenirMonCompte, listerMonJournal, obtenirNiveau }
}

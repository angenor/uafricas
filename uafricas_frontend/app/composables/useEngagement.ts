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

/**
 * Un mouvement de points (entrée de journal).
 *
 * `libelle` vient de la règle **courante** (renommer une règle renomme l'action
 * dans tout l'historique), tandis que `categorie_*` vient de la catégorie
 * **figée à l'écriture** : une re-catégorisation ne réécrit pas le passé.
 */
export interface MouvementPoints {
  id: string
  type_action: string
  libelle: string | null
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

/** Page du journal des points */
export interface JournalPage {
  elements: MouvementPoints[]
  total: number
  page: number
  taille: number
}

/** Une ligne de la ventilation par catégorie (`code` nul = « Autres ») */
export interface CategorieVentilation {
  code: string | null
  libelle: string
  couleur: string | null
  icone: string | null
  ordre: number
  points: number
  nombre_mouvements: number
}

/**
 * Ventilation des points gagnés.
 *
 * `total_gagne` (cumul du journal) peut **dépasser** `solde_points` (solde
 * courant) à cause du plancher 0 : les deux sont affichés distinctement.
 */
export interface VentilationPoints {
  solde_points: number
  total_gagne: number
  categories: CategorieVentilation[]
}

/** Une règle du barème telle qu'exposée publiquement (état vide pédagogique) */
export interface ActionRecompensee {
  type_action: string
  libelle: string
  points: number
  reputation_delta: number
  categorie_code: string | null
  categorie_libelle: string | null
  categorie_icone: string | null
  plafond_journalier: number | null
  seuil_declencheur: number | null
}

/** Un badge obtenu (espace membre et profil public) */
export interface BadgeObtenu {
  code: string
  libelle: string
  description: string
  couleur: string | null
  icone: string | null
  origine: 'automatique' | 'manuel' | 'retroactif'
  obtenu_at: string
}

/**
 * Un badge restant à débloquer.
 *
 * `progression_*` est nul quand la condition ne s'exprime pas comme « N sur M »
 * (le niveau se compare sur un ordre) : n'afficher alors aucune barre.
 */
export interface BadgeADebloquer {
  code: string
  libelle: string
  description: string
  couleur: string | null
  icone: string | null
  progression_actuelle: number | null
  progression_cible: number | null
}

export interface MesBadges {
  obtenus: BadgeObtenu[]
  a_debloquer: BadgeADebloquer[]
}

/** Filtres du journal des points */
export interface FiltresJournal {
  typeAction?: string
  categorie?: string
  depuis?: string
  jusquA?: string
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

  /**
   * GET /api/engagement/mon-journal
   *
   * `filtres` accepte le type d'action, le code de catégorie et une période
   * (dates ISO `YYYY-MM-DD`). Le 3ᵉ paramètre reste compatible avec l'appel
   * historique `listerMonJournal(page, taille, typeAction)`.
   */
  const listerMonJournal = async (
    page = 1,
    taille = 20,
    filtres: string | FiltresJournal = {},
  ): Promise<JournalPage | null> => {
    const f: FiltresJournal = typeof filtres === 'string' ? { typeAction: filtres } : filtres
    const query: Record<string, string | number> = { page, taille }
    if (f.typeAction) query.type_action = f.typeAction
    if (f.categorie) query.categorie = f.categorie
    if (f.depuis) query.depuis = f.depuis
    if (f.jusquA) query.jusqu_a = f.jusquA
    const res = await $fetch<ApiResponse<JournalPage>>(
      `${apiBase}/api/engagement/mon-journal`,
      { headers: authHeaders(), query },
    )
    return res.data
  }

  /** GET /api/engagement/mes-categories — ventilation des points gagnés */
  const obtenirMesCategories = async (): Promise<VentilationPoints | null> => {
    const res = await $fetch<ApiResponse<VentilationPoints>>(
      `${apiBase}/api/engagement/mes-categories`,
      { headers: authHeaders() },
    )
    return res.data
  }

  /**
   * GET /api/engagement/actions-recompensees — barème public.
   * Source unique des libellés et montants : ne jamais les recopier dans un composant.
   */
  const obtenirActionsRecompensees = async (): Promise<ActionRecompensee[]> => {
    const res = await $fetch<ApiResponse<ActionRecompensee[]>>(
      `${apiBase}/api/engagement/actions-recompensees`,
    )
    return res.data ?? []
  }

  /**
   * GET /api/engagement/mes-badges — obtenus + à débloquer.
   * Le serveur réévalue les conditions avant de répondre : un badge devenu
   * atteignable entre-temps apparaît dès cet appel.
   */
  const obtenirMesBadges = async (): Promise<MesBadges | null> => {
    const res = await $fetch<ApiResponse<MesBadges>>(
      `${apiBase}/api/engagement/mes-badges`,
      { headers: authHeaders() },
    )
    return res.data
  }

  /**
   * GET /api/engagement/badges/{utilisateurId} — badges publics d'un membre.
   * Uniquement les badges obtenus : jamais de solde ni de journal (FR-014).
   */
  const obtenirBadgesPublics = async (utilisateurId: string): Promise<BadgeObtenu[]> => {
    const res = await $fetch<ApiResponse<BadgeObtenu[]>>(
      `${apiBase}/api/engagement/badges/${utilisateurId}`,
    )
    return res.data ?? []
  }

  /** GET /api/engagement/niveau/{utilisateurId} — badge public léger */
  const obtenirNiveau = async (utilisateurId: string): Promise<NiveauInfo | null> => {
    const res = await $fetch<ApiResponse<NiveauInfo>>(
      `${apiBase}/api/engagement/niveau/${utilisateurId}`,
    )
    return res.data
  }

  return {
    obtenirMonCompte,
    listerMonJournal,
    obtenirMesCategories,
    obtenirActionsRecompensees,
    obtenirMesBadges,
    obtenirBadgesPublics,
    obtenirNiveau,
  }
}

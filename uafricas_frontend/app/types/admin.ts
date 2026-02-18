// Types TypeScript communs pour l'administration

// ── Reponses API ────────────────────────────────────────────

/** Reponse paginee generique du backend */
export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Reponse API standardisee */
export interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

// ── Admin Me ────────────────────────────────────────────────

/** Permission admin */
export interface AdminPermission {
  slug: string
  type_ressource: string
  action: string
}

/** Reponse GET /api/admin/me */
export interface AdminMeResponse {
  id: string
  nom: string
  prenom: string
  email: string
  roles: string[]
  permissions: AdminPermission[]
}

// ── Table ───────────────────────────────────────────────────

/** Definition d'une colonne de tableau */
export interface TableColumn {
  /** Cle correspondant au champ de l'objet */
  key: string
  /** Libelle affiche dans l'en-tete */
  label: string
  /** Colonne triable */
  sortable?: boolean
  /** Largeur CSS (ex: 'w-40', 'w-1/4') */
  width?: string
  /** Alignement du contenu */
  align?: 'left' | 'center' | 'right'
  /** Fonction de formatage personnalisee */
  format?: (value: any, row: any) => string
}

// ── Filtres ─────────────────────────────────────────────────

/** Types de filtres disponibles */
export type FilterType = 'text' | 'select' | 'date' | 'date-range'

/** Option pour un filtre de type select */
export interface FilterSelectOption {
  label: string
  value: string
}

/** Definition d'un filtre */
export interface FilterDefinition {
  /** Cle du parametre de requete */
  key: string
  /** Libelle affiche */
  label: string
  /** Type de filtre */
  type: FilterType
  /** Placeholder du champ */
  placeholder?: string
  /** Options pour les filtres select */
  options?: FilterSelectOption[]
  /** Valeur par defaut */
  defaultValue?: string
}

// ── Pagination ──────────────────────────────────────────────

/** Etat de pagination */
export interface PaginationState {
  page: number
  parPage: number
  total: number
  totalPages: number
}

/** Parametres de tri */
export interface SortState {
  column: string
  direction: 'asc' | 'desc'
}

// ── Stats ───────────────────────────────────────────────────

/** Donnees pour une carte KPI */
export interface StatsCardData {
  label: string
  value: string | number
  icon: string
  /** Couleur du badge/icone (classes Tailwind) */
  color?: string
  /** Variation par rapport a la periode precedente */
  variation?: {
    valeur: number
    type: 'hausse' | 'baisse' | 'stable'
  }
}

// ── Statuts ─────────────────────────────────────────────────

/** Mapping des statuts vers des couleurs daisyUI */
export type BadgeVariant = 'success' | 'warning' | 'error' | 'info' | 'neutral'

export interface StatusConfig {
  label: string
  variant: BadgeVariant
}

/** Configuration des statuts communs */
export const STATUTS: Record<string, StatusConfig> = {
  actif: { label: 'Actif', variant: 'success' },
  en_attente: { label: 'En attente', variant: 'warning' },
  suspendu: { label: 'Suspendu', variant: 'error' },
  bloque: { label: 'Bloque', variant: 'error' },
  supprime: { label: 'Supprime', variant: 'neutral' },
  brouillon: { label: 'Brouillon', variant: 'info' },
  publie: { label: 'Publie', variant: 'success' },
  archive: { label: 'Archive', variant: 'neutral' },
  approuve: { label: 'Approuve', variant: 'success' },
  rejete: { label: 'Rejete', variant: 'error' },
}

// Composable pour les appels API du Marche Africain

// ── Interfaces correspondant aux DTOs backend ─────────────────

/** Auteur d'une annonce */
export interface AnnonceAuteurAPI {
  uid: string
  nom: string
  prenom: string
  email: string
}

/** Media d'une annonce */
export interface AnnonceMediaAPI {
  id: string
  media_url: string
  type_mime: string | null
  est_principale: boolean
  ordre: number
}

/** Annonce dans la liste */
export interface AnnonceAPI {
  id: string
  titre: string
  slug: string | null
  description: string
  type_echange: string
  categorie: string
  condition_article: string
  prix: number
  devise: string
  prix_negociable: boolean
  pays: string
  ville: string | null
  tel: string | null
  photo_url: string | null
  quantite: number | null
  nombre_vues: number
  user: AnnonceAuteurAPI
  created_at: string
  updated_at: string
}

/** Annonce detail (enrichie) */
export interface AnnonceDetailAPI {
  id: string
  titre: string
  slug: string | null
  description: string
  type_echange: string
  categorie: string
  condition_article: string
  prix: number
  devise: string
  prix_negociable: boolean
  pays: string[]
  ville: string | null
  adresse: string | null
  longitude: number | null
  latitude: number | null
  type_contact: string
  contact_info: string | null
  quantite: number | null
  nombre_vues: number
  medias: AnnonceMediaAPI[]
  photo_url: string | null
  user: AnnonceAuteurAPI
  created_at: string
  updated_at: string
}

/** Reponse paginee */
export interface AnnonceListeAPI {
  annonces: AnnonceAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface AnnonceFiltres {
  recherche?: string
  type_operation?: string
  categorie?: string
  prix_min?: number
  prix_max?: number
  tri?: string
  page?: number
  par_page?: number
}

// ── Types frontend ────────────────────────────────────────────

export type TypeEchange = 'Vente' | 'Troc' | 'Don'
export type Categorie = 'Agriculture' | 'Informatique' | 'Immobilier' | 'Voitures' | 'Electronique' | 'Formation'
export type Devise = 'XOF' | 'EUR' | 'NGN' | 'USD'

/** Interface des filtres cote UI (reprend le format du mock) */
export interface FiltresAnnonce {
  categorie: Categorie | 'Tout'
  typesEchange: TypeEchange[]
  prixMin: number | null
  prixMax: number | null
  recherche: string
  tri: 'recent' | 'price-asc' | 'price-desc'
}

// ── Constantes ────────────────────────────────────────────────

export const CATEGORIES: { key: Categorie | 'Tout'; label: string }[] = [
  { key: 'Tout', label: 'Toutes les catégories' },
  { key: 'Agriculture', label: 'Agriculture' },
  { key: 'Informatique', label: 'Informatique' },
  { key: 'Immobilier', label: 'Immobilier' },
  { key: 'Voitures', label: 'Voitures' },
  { key: 'Electronique', label: 'Électronique' },
  { key: 'Formation', label: 'Formation' },
]

export const TYPES_ECHANGE: { value: TypeEchange; label: string; color: string }[] = [
  { value: 'Vente', label: 'Vente', color: 'bg-white/90 text-gray-700' },
  { value: 'Troc', label: 'Troc', color: 'bg-purple-100/90 text-purple-700' },
  { value: 'Don', label: 'Don', color: 'bg-blue-100/90 text-blue-700' },
]

export const DEVISES: { value: Devise; label: string; symbol: string }[] = [
  { value: 'XOF', label: 'Franc CFA', symbol: 'FCFA' },
  { value: 'EUR', label: 'Euro', symbol: '€' },
  { value: 'NGN', label: 'Naira', symbol: '₦' },
  { value: 'USD', label: 'Dollar US', symbol: '$' },
]

// ── Utilitaires de formatage ──────────────────────────────────

/** Formater une date (string ISO) en francais long */
export function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
}

/** Formater une date (string ISO) en francais court */
export function formatDateCourte(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  })
}

/** Formater le prix avec devise */
export function formatPrix(prix: number, devise: string): string {
  if (prix === 0) return 'Gratuit'

  const deviseInfo = DEVISES.find(d => d.value === devise)
  const symbol = deviseInfo?.symbol || devise

  if (devise === 'XOF' || devise === 'NGN') {
    return `${prix.toLocaleString('fr-FR')} ${symbol}`
  }
  return `${symbol}${prix.toLocaleString('fr-FR')}`
}

/** Obtenir la couleur CSS du type d'echange */
export function getTypeEchangeColor(type: string): string {
  const info = TYPES_ECHANGE.find(t => t.value === type)
  return info?.color || 'bg-gray-100 text-gray-700'
}

/** Compter les annonces par type d'echange */
export function getCountByType(type: TypeEchange, annonces: AnnonceAPI[]): number {
  return annonces.filter(a => a.type_echange === type).length
}

/** Mapper type_echange frontend vers valeur(s) DB pour les filtres */
export function mapperTypesVersDb(types: TypeEchange[]): string {
  const mapping: Record<TypeEchange, string> = {
    Vente: 'vente',
    Troc: 'troc',
    Don: 'don',
  }
  return types.map(t => mapping[t] || t.toLowerCase()).join(',')
}

// ── Composable ────────────────────────────────────────────────

export const useMarcheAfricain = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** Recuperer la liste paginee des annonces */
  const listerAnnonces = async (filtres: AnnonceFiltres = {}): Promise<AnnonceListeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.type_operation) params.set('type_operation', filtres.type_operation)
      if (filtres.categorie) params.set('categorie', filtres.categorie)
      if (filtres.prix_min != null) params.set('prix_min', String(filtres.prix_min))
      if (filtres.prix_max != null) params.set('prix_max', String(filtres.prix_max))
      if (filtres.tri) params.set('tri', filtres.tri)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/annonces${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<AnnonceListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des annonces')
      }

      // Mapper les URLs relatives des medias en absolues
      reponse.data.annonces = reponse.data.annonces.map(a => ({
        ...a,
        photo_url: a.photo_url ? mapperUrl(a.photo_url, apiBase) : null,
      }))

      return reponse.data
    } catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur listerAnnonces:', e)
      return null
    } finally {
      chargement.value = false
    }
  }

  /** Recuperer le detail d'une annonce */
  const obtenirAnnonce = async (id: string): Promise<AnnonceDetailAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<AnnonceDetailAPI>>(
        `${apiBase}/api/annonces/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Annonce non trouvée')
      }

      const data = reponse.data
      // Mapper les URLs relatives en absolues
      data.photo_url = data.photo_url ? mapperUrl(data.photo_url, apiBase) : null
      data.medias = data.medias.map(m => ({
        ...m,
        media_url: mapperUrl(m.media_url, apiBase),
      }))

      return data
    } catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur obtenirAnnonce:', e)
      return null
    } finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerAnnonces,
    obtenirAnnonce,
  }
}

// ── Utilitaire interne ────────────────────────────────────────

/** Mapper une URL relative en absolue si necessaire */
function mapperUrl(url: string, apiBase: string): string {
  if (url.startsWith('http')) return url
  return `${apiBase}${url}`
}

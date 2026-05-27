// Composable pour les appels API du Marche Africain
import { useUserStore } from '~/stores/user'

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

/** Item « Mes annonces » (annonce + etat propriétaire) */
export interface MesAnnonceItemAPI {
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
  photo_url: string | null
  quantite: number | null
  nombre_vues: number
  nombre_medias: number
  etat: string
  user: AnnonceAuteurAPI
  created_at: string
  updated_at: string
}

/** Reponse paginee « Mes annonces » */
export interface MesAnnoncesListeAPI {
  annonces: MesAnnonceItemAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Catégorie du marché (contexte 'annonce') */
export interface CategorieAnnonceAPI {
  id: string
  nom: string
  slug: string
  icone: string | null
}

/** Territoire (pays) pour le sélecteur */
export interface PaysAPI {
  id: string
  nom: string
  code_iso2?: string | null
  code_iso3?: string | null
}

/** Résultat d'un contact d'annonce */
export interface ContactResultAPI {
  conversation_id: string
  ami_id: string
  message: {
    id: string
    expediteur_id: string
    contenu: string | null
    supprime: boolean
    created_at: string
    lu_at: string | null
  }
}

/** Données du formulaire de publication / édition d'annonce */
export interface CreerAnnonceForm {
  titre: string
  description: string
  typeEchange: TypeEchange
  categorieId: string
  conditionArticle?: string
  prix?: number | null
  devise?: Devise
  prixNegociable?: boolean
  ville?: string
  adresse?: string
  longitude?: number | null
  latitude?: number | null
  quantite?: number
  paysIds: string[]
  photos: File[]
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

/** Mapper un type d'echange unique vers sa valeur DB (snake_case) */
export function mapperTypeVersDb(type: TypeEchange): string {
  const mapping: Record<TypeEchange, string> = {
    Vente: 'vente',
    Troc: 'troc',
    Don: 'don',
  }
  return mapping[type] || type.toLowerCase()
}

// ── Composable ────────────────────────────────────────────────

export const useMarcheAfricain = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** En-têtes d'authentification (JWT Bearer) si connecté. */
  const authHeaders = (): Record<string, string> =>
    userStore.accessToken ? { Authorization: `Bearer ${userStore.accessToken}` } : {}

  /** Construit le FormData multipart à partir du formulaire d'annonce. */
  const construireFormData = (form: Partial<CreerAnnonceForm>): FormData => {
    const fd = new FormData()
    if (form.titre != null) fd.set('titre', form.titre)
    if (form.description != null) fd.set('description', form.description)
    if (form.typeEchange != null) fd.set('type_operation', mapperTypeVersDb(form.typeEchange))
    if (form.categorieId) fd.set('categorie_id', form.categorieId)
    if (form.conditionArticle) fd.set('condition_article', form.conditionArticle)
    if (form.prix != null) fd.set('prix', String(form.prix))
    if (form.devise) fd.set('devise', form.devise)
    if (form.prixNegociable != null) fd.set('prix_negociable', String(form.prixNegociable))
    if (form.ville) fd.set('ville', form.ville)
    if (form.adresse) fd.set('adresse', form.adresse)
    if (form.longitude != null) fd.set('longitude', String(form.longitude))
    if (form.latitude != null) fd.set('latitude', String(form.latitude))
    if (form.quantite != null) fd.set('quantite', String(form.quantite))
    if (form.paysIds && form.paysIds.length > 0) fd.set('pays_ids', form.paysIds.join(','))
    if (form.photos) {
      for (const photo of form.photos) fd.append('photos', photo)
    }
    return fd
  }

  /** Extrait un message d'erreur exploitable. */
  const messageErreur = (e: any, defaut: string): string =>
    e?.data?.error || e?.message || defaut

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

  // ── Référentiels (catégories, territoires) ─────────────────────

  /** Lister les catégories du marché (contexte 'annonce'). */
  const listerCategories = async (): Promise<CategorieAnnonceAPI[]> => {
    try {
      const r = await $fetch<ApiResponse<CategorieAnnonceAPI[]>>(
        `${apiBase}/api/annonces/categories`,
      )
      return r.success && r.data ? r.data : []
    } catch (e) {
      console.error('Erreur listerCategories:', e)
      return []
    }
  }

  /** Lister les territoires (pays) actifs. */
  const listerTerritoires = async (): Promise<PaysAPI[]> => {
    try {
      const r = await $fetch<ApiResponse<PaysAPI[]>>(`${apiBase}/api/pays`)
      return r.success && r.data ? r.data : []
    } catch (e) {
      console.error('Erreur listerTerritoires:', e)
      return []
    }
  }

  // ── US1 : Publier une annonce ──────────────────────────────────

  /** Créer une annonce (multipart). Retourne le détail créé ou null. */
  const creerAnnonce = async (form: CreerAnnonceForm): Promise<AnnonceDetailAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const r = await $fetch<ApiResponse<AnnonceDetailAPI>>(`${apiBase}/api/annonces`, {
        method: 'POST',
        headers: authHeaders(),
        body: construireFormData(form),
      })
      if (!r.success || !r.data) throw new Error(r.error || 'Création impossible')
      return r.data
    } catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de la publication')
      return null
    } finally {
      chargement.value = false
    }
  }

  // ── US2 : Contacter l'auteur via la messagerie ─────────────────

  /** Contacter l'auteur d'une annonce (ouvre/réutilise une conversation). */
  const contacterAuteur = async (
    annonceId: string,
    message: string,
  ): Promise<ContactResultAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const r = await $fetch<ApiResponse<ContactResultAPI>>(
        `${apiBase}/api/annonces/${annonceId}/contacter`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { message },
        },
      )
      if (!r.success || !r.data) throw new Error(r.error || 'Envoi impossible')
      return r.data
    } catch (e: any) {
      erreur.value = messageErreur(e, "Impossible de contacter l'auteur")
      return null
    } finally {
      chargement.value = false
    }
  }

  // ── US3 : Gérer ses annonces ───────────────────────────────────

  /** Lister mes annonces (tous états, paginé). */
  const mesAnnonces = async (
    params: { page?: number, par_page?: number, etat?: string } = {},
  ): Promise<MesAnnoncesListeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const qs = new URLSearchParams()
      if (params.page) qs.set('page', String(params.page))
      if (params.par_page) qs.set('par_page', String(params.par_page))
      if (params.etat) qs.set('etat', params.etat)
      const url = `${apiBase}/api/annonces/mes-annonces${qs.toString() ? `?${qs}` : ''}`
      const r = await $fetch<ApiResponse<MesAnnoncesListeAPI>>(url, { headers: authHeaders() })
      if (!r.success || !r.data) throw new Error(r.error || 'Chargement impossible')
      r.data.annonces = r.data.annonces.map(a => ({
        ...a,
        photo_url: a.photo_url ? mapperUrl(a.photo_url, apiBase) : null,
      }))
      return r.data
    } catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors du chargement de vos annonces')
      return null
    } finally {
      chargement.value = false
    }
  }

  /** Modifier une annonce (champs partiels + photos éventuelles). */
  const modifierAnnonce = async (
    id: string,
    form: Partial<CreerAnnonceForm>,
  ): Promise<AnnonceDetailAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const r = await $fetch<ApiResponse<AnnonceDetailAPI>>(`${apiBase}/api/annonces/${id}`, {
        method: 'PUT',
        headers: authHeaders(),
        body: construireFormData(form),
      })
      if (!r.success || !r.data) throw new Error(r.error || 'Modification impossible')
      return r.data
    } catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de la modification')
      return null
    } finally {
      chargement.value = false
    }
  }

  /** Marquer une annonce comme conclue. */
  const conclureAnnonce = async (id: string): Promise<boolean> => {
    erreur.value = null
    try {
      const r = await $fetch<ApiResponse<unknown>>(`${apiBase}/api/annonces/${id}/conclure`, {
        method: 'PATCH',
        headers: authHeaders(),
      })
      if (!r.success) throw new Error(r.error || 'Action impossible')
      return true
    } catch (e: any) {
      erreur.value = messageErreur(e, 'Impossible de marquer conclue')
      return false
    }
  }

  /** Supprimer une annonce (soft delete). */
  const supprimerAnnonce = async (id: string): Promise<boolean> => {
    erreur.value = null
    try {
      const r = await $fetch<ApiResponse<unknown>>(`${apiBase}/api/annonces/${id}`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
      if (!r.success) throw new Error(r.error || 'Suppression impossible')
      return true
    } catch (e: any) {
      erreur.value = messageErreur(e, 'Impossible de supprimer')
      return false
    }
  }

  /** Retirer une photo d'une annonce. */
  const supprimerMedia = async (id: string, mediaId: string): Promise<boolean> => {
    erreur.value = null
    try {
      const r = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/annonces/${id}/medias/${mediaId}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      if (!r.success) throw new Error(r.error || 'Suppression impossible')
      return true
    } catch (e: any) {
      erreur.value = messageErreur(e, 'Impossible de retirer la photo')
      return false
    }
  }

  // ── US4 : Favoris ──────────────────────────────────────────────

  /** Ajouter une annonce aux favoris. */
  const ajouterFavori = async (id: string): Promise<boolean> => {
    erreur.value = null
    try {
      const r = await $fetch<ApiResponse<unknown>>(`${apiBase}/api/annonces/${id}/favori`, {
        method: 'POST',
        headers: authHeaders(),
      })
      return !!r.success
    } catch (e: any) {
      erreur.value = messageErreur(e, "Impossible d'ajouter aux favoris")
      return false
    }
  }

  /** Retirer une annonce des favoris. */
  const retirerFavori = async (id: string): Promise<boolean> => {
    erreur.value = null
    try {
      const r = await $fetch<ApiResponse<unknown>>(`${apiBase}/api/annonces/${id}/favori`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
      return !!r.success
    } catch (e: any) {
      erreur.value = messageErreur(e, 'Impossible de retirer des favoris')
      return false
    }
  }

  /** Lister mes favoris (annonces encore publiées, paginé). */
  const listerFavoris = async (
    params: { page?: number, par_page?: number } = {},
  ): Promise<AnnonceListeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const qs = new URLSearchParams()
      if (params.page) qs.set('page', String(params.page))
      if (params.par_page) qs.set('par_page', String(params.par_page))
      const url = `${apiBase}/api/annonces/favoris${qs.toString() ? `?${qs}` : ''}`
      const r = await $fetch<ApiResponse<AnnonceListeAPI>>(url, { headers: authHeaders() })
      if (!r.success || !r.data) throw new Error(r.error || 'Chargement impossible')
      r.data.annonces = r.data.annonces.map(a => ({
        ...a,
        photo_url: a.photo_url ? mapperUrl(a.photo_url, apiBase) : null,
      }))
      return r.data
    } catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors du chargement des favoris')
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
    listerCategories,
    listerTerritoires,
    creerAnnonce,
    contacterAuteur,
    mesAnnonces,
    modifierAnnonce,
    conclureAnnonce,
    supprimerAnnonce,
    supprimerMedia,
    ajouterFavori,
    retirerFavori,
    listerFavoris,
  }
}

// ── Utilitaire interne ────────────────────────────────────────

/** Mapper une URL relative en absolue si necessaire */
function mapperUrl(url: string, apiBase: string): string {
  if (url.startsWith('http')) return url
  return `${apiBase}${url}`
}

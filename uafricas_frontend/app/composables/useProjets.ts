// Composable pour les appels API Projets (financer un projet)
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type StatutProjet = 'valide' | 'en_cours' | 'termine' | 'soumis' | 'brouillon' | 'rejete'
export type DeviseProjet = 'XOF' | 'XAF' | 'EUR' | 'USD'

export interface ProjetAuteurAPI {
  uid: string
  nom: string
  prenom: string
  photo_url: string | null
}

export interface ProjetDocumentAPI {
  id: string
  nom: string
  url: string
  type_mime: string | null
}

/** DTO pour un projet dans la liste */
export interface ProjetAPI {
  id: string
  titre: string
  description: string
  organisation: string | null
  pays: string | null
  ville: string | null
  cout_total: number | null
  devise: string
  duree: string
  date_debut_souhaitee: string | null
  statut: StatutProjet
  image_couverture: string | null
  user: ProjetAuteurAPI
  created_at: string
  updated_at: string
}

/** DTO pour le detail d'un projet */
export interface ProjetDetailAPI extends ProjetAPI {
  slug: string | null
  description_organisation: string | null
  site_web: string | null
  contact_email: string | null
  contact_telephone: string | null
  objectifs: string[]
  resultats_attendus: string | null
  activites_programmees: string | null
  echeanciers: string | null
  contribution_autonomisation: string | null
  difficultes_risques: string | null
  documents: ProjetDocumentAPI[]
}

/** Reponse paginee */
export interface ProjetListeAPI {
  projets: ProjetAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Statistiques des projets */
export interface ProjetStatistiquesAPI {
  total: number
  valides: number
  en_cours: number
  termines: number
}

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Parametres de filtre pour le listing */
export interface ProjetFiltres {
  recherche?: string
  pays?: string
  budget_max?: number
  duree?: string
  tri?: string
  page?: number
  par_page?: number
}

/** Interface pour les filtres de la page (compatible avec les composants existants) */
export interface FiltresProjetPage {
  pays: string
  budgetMax: string
  duree: string
  recherche: string
  sortBy: string
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const PAYS_PROJETS: { value: string; label: string }[] = [
  { value: '', label: 'Tous les pays' },
  { value: 'Afrique du Sud', label: 'Afrique du Sud' },
  { value: 'Algérie', label: 'Algérie' },
  { value: 'Bénin', label: 'Bénin' },
  { value: 'Burkina Faso', label: 'Burkina Faso' },
  { value: 'Cameroun', label: 'Cameroun' },
  { value: 'Cap-Vert', label: 'Cap-Vert' },
  { value: 'Comores', label: 'Comores' },
  { value: 'Côte d\'Ivoire', label: 'Côte d\'Ivoire' },
  { value: 'Égypte', label: 'Égypte' },
  { value: 'Éthiopie', label: 'Éthiopie' },
  { value: 'Gabon', label: 'Gabon' },
  { value: 'Gambie', label: 'Gambie' },
  { value: 'Ghana', label: 'Ghana' },
  { value: 'Guinée', label: 'Guinée' },
  { value: 'Kenya', label: 'Kenya' },
  { value: 'Madagascar', label: 'Madagascar' },
  { value: 'Mali', label: 'Mali' },
  { value: 'Maroc', label: 'Maroc' },
  { value: 'Maurice', label: 'Maurice' },
  { value: 'Mauritanie', label: 'Mauritanie' },
  { value: 'Namibie', label: 'Namibie' },
  { value: 'Niger', label: 'Niger' },
  { value: 'Nigeria', label: 'Nigeria' },
  { value: 'RDC', label: 'RDC' },
  { value: 'Rwanda', label: 'Rwanda' },
  { value: 'Sénégal', label: 'Sénégal' },
  { value: 'Tanzanie', label: 'Tanzanie' },
  { value: 'Togo', label: 'Togo' },
  { value: 'Tunisie', label: 'Tunisie' },
]

export const BUDGETS: { value: string; label: string }[] = [
  { value: '', label: 'Tous les budgets' },
  { value: '1000000', label: 'Moins de 1 million XOF' },
  { value: '5000000', label: 'Moins de 5 millions XOF' },
  { value: '10000000', label: 'Moins de 10 millions XOF' },
  { value: '50000000', label: 'Moins de 50 millions XOF' },
  { value: '100000000', label: 'Moins de 100 millions XOF' },
]

export const DUREES: { value: string; label: string }[] = [
  { value: '', label: 'Toutes les durées' },
  { value: 'court', label: 'Court terme (< 6 mois)' },
  { value: 'moyen', label: 'Moyen terme (6-24 mois)' },
  { value: 'long', label: 'Long terme (> 24 mois)' },
]

export const OPTIONS_TRI: { value: string; label: string }[] = [
  { value: 'recent', label: 'Plus récents' },
  { value: 'titre', label: 'Alphabétique (A-Z)' },
  { value: 'cout_asc', label: 'Budget croissant' },
  { value: 'cout_desc', label: 'Budget décroissant' },
]

export const STATUTS_LABELS: Record<StatutProjet, { label: string; color: string }> = {
  valide: { label: 'Validé', color: 'bg-green-100 text-green-800' },
  en_cours: { label: 'En cours', color: 'bg-blue-100 text-blue-800' },
  termine: { label: 'Terminé', color: 'bg-gray-100 text-gray-800' },
  soumis: { label: 'Soumis', color: 'bg-yellow-100 text-yellow-800' },
  brouillon: { label: 'Brouillon', color: 'bg-orange-100 text-orange-800' },
  rejete: { label: 'Rejeté', color: 'bg-red-100 text-red-800' },
}

export const DEVISES_LABELS: Record<DeviseProjet, string> = {
  XOF: 'XOF (FCFA)',
  XAF: 'XAF (FCFA)',
  EUR: 'EUR (€)',
  USD: 'USD ($)',
}

export const DEVISES_FORM: { value: string; label: string }[] = [
  { value: 'XOF', label: 'XOF (FCFA Ouest)' },
  { value: 'XAF', label: 'XAF (FCFA Centre)' },
  { value: 'EUR', label: 'EUR (€)' },
  { value: 'USD', label: 'USD ($)' },
]

export const DUREES_MOIS_FORM: { value: number; label: string }[] = [
  { value: 1, label: '1 mois' },
  { value: 3, label: '3 mois' },
  { value: 6, label: '6 mois' },
  { value: 12, label: '12 mois' },
  { value: 18, label: '18 mois' },
  { value: 24, label: '24 mois' },
  { value: 36, label: '36 mois' },
  { value: 48, label: '48 mois' },
  { value: 60, label: '60 mois (5 ans)' },
]

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/** Formater un montant avec sa devise */
export const formatCurrency = (amount: number | null, devise: string = 'XOF'): string => {
  if (amount === null || amount === undefined) return 'Non spécifié'
  const symbols: Record<string, string> = { XOF: 'FCFA', XAF: 'FCFA', EUR: '€', USD: '$' }
  return new Intl.NumberFormat('fr-FR', { minimumFractionDigits: 0, maximumFractionDigits: 0 })
    .format(amount) + ' ' + (symbols[devise] || devise)
}

/** Formater une date ISO en francais */
export const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
}

/** Obtenir les infos d'un statut (label + couleur) */
export const getStatutInfo = (statut: StatutProjet) => {
  return STATUTS_LABELS[statut] || { label: statut, color: 'bg-gray-100 text-gray-800' }
}

/** Obtenir les initiales d'un nom et prenom */
export const getInitiales = (nom?: string, prenom?: string): string => {
  return (prenom?.charAt(0).toUpperCase() || '') + (nom?.charAt(0).toUpperCase() || '')
}

/** Convertir les filtres de la page vers les filtres API */
export const convertirFiltresPageVersAPI = (filtres: FiltresProjetPage): ProjetFiltres => {
  const result: ProjetFiltres = {}
  if (filtres.recherche) result.recherche = filtres.recherche
  if (filtres.pays) result.pays = filtres.pays
  if (filtres.budgetMax) result.budget_max = Number(filtres.budgetMax)
  if (filtres.duree) result.duree = filtres.duree
  // Mapper les noms de tri du mock vers l'API
  const triMap: Record<string, string> = {
    dateCreation: 'recent',
    titre: 'titre',
    coutTotal: 'cout_asc',
    'coutTotal-desc': 'cout_desc',
  }
  if (filtres.sortBy) result.tri = triMap[filtres.sortBy] || 'recent'
  return result
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useProjets = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  /** Prefixer les URLs relatives d'images avec l'URL du backend */
  const resoudreUrl = (url: string | null): string | null => {
    if (!url) return null
    if (url.startsWith('http')) return url
    return `${apiBase}${url}`
  }

  /**
   * Lister les projets avec filtres et pagination
   */
  const listerProjets = async (filtres: ProjetFiltres = {}): Promise<ProjetListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.budget_max) params.set('budget_max', String(filtres.budget_max))
      if (filtres.duree) params.set('duree', filtres.duree)
      if (filtres.tri) params.set('tri', filtres.tri)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/projets${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ProjetListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des projets')
      }

      // Resoudre les URLs d'images
      reponse.data.projets = reponse.data.projets.map(p => ({
        ...p,
        image_couverture: resoudreUrl(p.image_couverture),
        user: {
          ...p.user,
          photo_url: resoudreUrl(p.user.photo_url),
        },
      }))

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur listerProjets:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir un projet par son ID
   */
  const obtenirProjet = async (id: string): Promise<ProjetDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ProjetDetailAPI>>(
        `${apiBase}/api/projets/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Projet non trouvé')
      }

      // Resoudre les URLs
      reponse.data.image_couverture = resoudreUrl(reponse.data.image_couverture)
      if (reponse.data.user.photo_url) {
        reponse.data.user.photo_url = resoudreUrl(reponse.data.user.photo_url)
      }
      if (reponse.data.documents) {
        reponse.data.documents = reponse.data.documents.map(d => ({
          ...d,
          url: resoudreUrl(d.url) || d.url,
        }))
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur obtenirProjet:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Creer un projet (multipart pour l'image de couverture)
   */
  const creerProjet = async (
    formData: {
      titre: string
      description: string
      objectifs: string
      nom_organisation?: string
      description_organisation?: string
      site_web?: string
      pays?: string
      ville?: string
      contact_email?: string
      contact_telephone?: string
      cout_total?: number
      devise?: string
      duree_mois?: number
      date_commencement_souhaitee?: string
      resultats_attendus?: string
      activites_programmees?: string
      echeanciers?: string
      contribution_autonomisation?: string
      difficultes_risques?: string
    },
    couvertureFile: File | null,
  ): Promise<ProjetDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const data = new FormData()
      data.append('titre', formData.titre)
      data.append('description', formData.description)
      data.append('objectifs', formData.objectifs)
      if (formData.nom_organisation) data.append('nom_organisation', formData.nom_organisation)
      if (formData.description_organisation) data.append('description_organisation', formData.description_organisation)
      if (formData.site_web) data.append('site_web', formData.site_web)
      if (formData.pays) data.append('pays', formData.pays)
      if (formData.ville) data.append('ville', formData.ville)
      if (formData.contact_email) data.append('contact_email', formData.contact_email)
      if (formData.contact_telephone) data.append('contact_telephone', formData.contact_telephone)
      if (formData.cout_total !== undefined) data.append('cout_total', String(formData.cout_total))
      if (formData.devise) data.append('devise', formData.devise)
      if (formData.duree_mois !== undefined) data.append('duree_mois', String(formData.duree_mois))
      if (formData.date_commencement_souhaitee) data.append('date_commencement_souhaitee', formData.date_commencement_souhaitee)
      if (formData.resultats_attendus) data.append('resultats_attendus', formData.resultats_attendus)
      if (formData.activites_programmees) data.append('activites_programmees', formData.activites_programmees)
      if (formData.echeanciers) data.append('echeanciers', formData.echeanciers)
      if (formData.contribution_autonomisation) data.append('contribution_autonomisation', formData.contribution_autonomisation)
      if (formData.difficultes_risques) data.append('difficultes_risques', formData.difficultes_risques)
      if (couvertureFile) data.append('couverture', couvertureFile)

      const reponse = await $fetch<ApiResponse<ProjetDetailAPI>>(
        `${apiBase}/api/projets`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la création du projet')
      }

      // Resoudre les URLs
      reponse.data.image_couverture = resoudreUrl(reponse.data.image_couverture)
      if (reponse.data.user.photo_url) {
        reponse.data.user.photo_url = resoudreUrl(reponse.data.user.photo_url)
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur creerProjet:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir les statistiques des projets
   */
  const obtenirStatistiques = async (): Promise<ProjetStatistiquesAPI | null> => {
    try {
      const reponse = await $fetch<ApiResponse<ProjetStatistiquesAPI>>(
        `${apiBase}/api/projets/statistiques`,
      )
      if (reponse.success && reponse.data) {
        return reponse.data
      }
      return null
    }
    catch (e: any) {
      console.error('Erreur obtenirStatistiques:', e)
      return null
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerProjets,
    obtenirProjet,
    creerProjet,
    obtenirStatistiques,
  }
}

// Composable pour les appels API Programmes d'échange sabbatique
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type TypeProgramme = 'interafricain' | 'hors_afrique'
export type StatutProgramme = 'ouvert' | 'en_cours' | 'termine' | 'annule' | 'suspendu' | 'en_attente'

export interface SabbatiqueOrganisateur {
  uid: string
  nom: string
  prenom: string | null
  email: string
  photo_url: string | null
}

/** DTO correspondant a SabbatiqueResponse du backend */
export interface SabbatiqueAPI {
  id: string
  titre: string
  description: string
  couverture_url: string | null
  pays: string | null
  ville: string | null
  domaine: string | null
  duree: string
  duree_label: string
  date_debut: string
  date_fin: string | null
  interafricain: boolean
  statut: StatutProgramme
  prise_en_charge: string[]
  nombre_places: number | null
  nombre_candidatures: number
  user: SabbatiqueOrganisateur
  created_at: string
  updated_at: string
}

/** DTO pour le detail d'un programme */
export interface SabbatiqueDetailAPI extends SabbatiqueAPI {
  slug: string | null
  document_url: string | null
  adresse: string | null
  prise_en_charge_details: string | null
  prerequis: string | null
  langues_requises: string | null
}

/** Reponse paginee */
export interface SabbatiqueListeAPI {
  programmes: SabbatiqueAPI[]
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
export interface SabbatiqueFiltres {
  type?: 'tous' | TypeProgramme
  pays?: string
  domaine?: string
  recherche?: string
  page?: number
  par_page?: number
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const TYPES_PROGRAMME: { value: 'tous' | TypeProgramme; label: string }[] = [
  { value: 'tous', label: 'Tous les programmes' },
  { value: 'interafricain', label: 'Interafricain' },
  { value: 'hors_afrique', label: 'Hors Afrique vers Afrique' },
]

export const DOMAINES: { value: string; label: string }[] = [
  { value: '', label: 'Tous les domaines' },
  { value: 'education', label: 'Éducation' },
  { value: 'infrastructure', label: 'Infrastructure' },
  { value: 'sante', label: 'Santé' },
  { value: 'eau', label: 'Eau' },
  { value: 'developpement-localites', label: 'Développement des localités' },
  { value: 'agriculture', label: 'Agriculture' },
  { value: 'energie', label: 'Énergie' },
  { value: 'technologie-innovation', label: 'Technologie & Innovation' },
]

export const DUREES: { value: string; label: string }[] = [
  { value: '1_semaine', label: '1 semaine' },
  { value: '2_semaines', label: '2 semaines' },
  { value: '3_semaines', label: '3 semaines' },
  { value: '6_semaines', label: '6 semaines' },
  { value: '1_mois', label: '1 mois' },
  { value: '2_mois', label: '2 mois' },
  { value: '3_mois', label: '3 mois' },
  { value: '6_mois', label: '6 mois' },
  { value: '1_an', label: '1 an' },
]

export const PAYS_AFRICAINS: { value: string; label: string }[] = [
  { value: '', label: 'Tous les territoires' },
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
  { value: 'Nigeria', label: 'Nigeria' },
  { value: 'Rwanda', label: 'Rwanda' },
  { value: 'Sénégal', label: 'Sénégal' },
  { value: 'Tanzanie', label: 'Tanzanie' },
  { value: 'Togo', label: 'Togo' },
  { value: 'Tunisie', label: 'Tunisie' },
]

export const PRISES_EN_CHARGE: { value: string; label: string }[] = [
  { value: 'billet_avion', label: 'Billet d\'avion' },
  { value: 'hebergement', label: 'Hébergement' },
  { value: 'frais_subsistance', label: 'Frais de subsistance' },
]

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/** Formater une date YYYY-MM-DD en francais (ex: "15 mars 2026") */
export const formatDateSabbatique = (dateStr: string): string => {
  const date = new Date(dateStr + 'T00:00:00')
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
}

/** Formater une date YYYY-MM-DD en format court (ex: "15 mars 2026") */
export const formatDateCourte = (dateStr: string): string => {
  const date = new Date(dateStr + 'T00:00:00')
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  })
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useSabbatiques = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /** Headers d'authentification si l'utilisateur est connecte */
  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) {
      return { Authorization: `Bearer ${userStore.accessToken}` }
    }
    return {}
  }

  /**
   * Lister les programmes avec filtres et pagination
   */
  const listerProgrammes = async (filtres: SabbatiqueFiltres = {}): Promise<SabbatiqueListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.type && filtres.type !== 'tous') params.set('type', filtres.type)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.domaine) params.set('domaine', filtres.domaine)
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/sabbatiques${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<SabbatiqueListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des programmes')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerProgrammes:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir un programme par son ID
   */
  const obtenirProgramme = async (id: string): Promise<SabbatiqueDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<SabbatiqueDetailAPI>>(
        `${apiBase}/api/sabbatiques/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Programme non trouve')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirProgramme:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Creer un programme (multipart pour image + document)
   */
  const creerProgramme = async (
    formData: {
      type: string
      titre: string
      description: string
      domaine: string
      pays: string
      ville?: string
      duree: string
      dateDebut: string
      dateFin: string
      prisesEnCharge: string[]
      organisateurNom?: string
      organisateurEmail?: string
    },
    couvertureFile: File | null,
    documentFile: File | null,
  ): Promise<SabbatiqueDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const data = new FormData()
      data.append('type_programme', formData.type)
      data.append('titre', formData.titre)
      data.append('description', formData.description)
      data.append('domaine', formData.domaine)
      data.append('pays', formData.pays)
      if (formData.ville) data.append('ville', formData.ville)
      data.append('duree', formData.duree)
      data.append('date_debut', formData.dateDebut.split('T')[0])
      data.append('date_fin', formData.dateFin.split('T')[0])

      // Prises en charge individuelles
      for (const prise of formData.prisesEnCharge) {
        data.append(prise, 'true')
      }

      if (formData.organisateurNom) data.append('organisateur_nom', formData.organisateurNom)
      if (formData.organisateurEmail) data.append('organisateur_email', formData.organisateurEmail)
      if (couvertureFile) data.append('couverture', couvertureFile)
      if (documentFile) data.append('document', documentFile)

      const reponse = await $fetch<ApiResponse<SabbatiqueDetailAPI>>(
        `${apiBase}/api/sabbatiques`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation du programme')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerProgramme:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerProgrammes,
    obtenirProgramme,
    creerProgramme,
  }
}

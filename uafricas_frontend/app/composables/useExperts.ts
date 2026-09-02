// Composable pour les appels API Experts
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export interface ExpertiseInfoAPI {
  domaine: string
  biographie: string
  nbAnneesExperience: number
  rating: number
  /** Nombre de notes reçues */
  nombreNotes: number
  /** Note attribuée par le membre connecté (1–5), null sinon */
  maNote: number | null
  portfolio: string | null
  linkedinUrl: string | null
  cvUrl: string | null
  specialites: string[]
  /** Objectifs avec libellés lisibles */
  objectifs: string[]
  realisations: string[]
  statut: 'valide' | 'en_attente' | 'refuse'
}

/** DTO correspondant a ExpertResponse du backend */
export interface ExpertAPI {
  id: string
  nom: string
  prenom: string
  photoURL: string | null
  pays: string
  ville: string | null
  email: string
  expertiseInfo: ExpertiseInfoAPI
  situationProfessionnelle: string[]
  dateInscription: string
  dateDerniereMiseAJour: string
}

/** Reponse paginee */
export interface ExpertListeAPI {
  experts: ExpertAPI[]
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
export interface ExpertFiltres {
  recherche?: string
  domaine?: string
  pays?: string
  situation?: string
  /** Spécialité déclarée par l'expert (valeur libre). « toutes » = pas de filtre. */
  specialite?: string
  /**
   * Zone géographique du territoire d'origine : filtre aussi la liste des experts.
   * `tout` n'est PAS transmis à l'API : c'est l'absence de filtre côté serveur.
   */
  zone?: 'afrique' | 'hors_afrique' | 'tout'
  tri?: 'recent' | 'experience' | 'rating'
  page?: number
  par_page?: number
}

/** Body pour creer une candidature */
export interface CandidatureExpertBody {
  domaine: string
  /** Précision libre lorsque `domaine` vaut "autre" */
  domaine_autre?: string
  biographie: string
  nb_annees_experience: number
  portfolio?: string
  linkedin_url?: string
  /** URL du CV déjà uploadé via POST /api/experts/cv */
  cv_url?: string
  specialites?: string[]
  /** Objectifs actuels (valeurs DB, ex: "reseautage") */
  objectifs?: string[]
  realisations?: string[]
  situations_professionnelles: string[]
}

/** Suivi de la candidature active du membre (US3) */
export interface MaCandidatureAPI {
  id: string
  domaine: string
  biographie: string
  nbAnneesExperience: number
  portfolio: string | null
  linkedinUrl: string | null
  cvUrl: string | null
  specialites: string[]
  objectifs: string[]
  realisations: string[]
  situationsProfessionnelles: string[]
  statut: 'en_attente' | 'valide' | 'refuse'
  commentaireAdmin: string | null
  dateValidation: string | null
  createdAt: string
}

/** Réponse après notation d'un expert */
export interface NoteExpertAPI {
  rating: number
  nombreNotes: number
  maNote: number
}

/** Option d'objectif actuel (valeur DB + libellé) */
export interface ObjectifOption {
  value: string
  label: string
}

/** Objectifs actuels proposés au candidat (valeur DB ↔ libellé affiché) */
export const OBJECTIFS_EXPERTISE: ObjectifOption[] = [
  { value: 'reseautage', label: 'Réseautage' },
  { value: 'consultance', label: 'Consultance' },
  { value: 'recherche_emploi', label: 'Recherche d\'emploi' },
  { value: 'offre_services_court_terme', label: 'Offre de services court terme' },
  { value: 'travail_vacances', label: 'Travail de vacances (Sabbafrica)' },
  { value: 'volontariat', label: 'Volontariat' },
  { value: 'benevolat', label: 'Bénévolat' },
]

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const CATEGORIES_EXPERTISE: string[] = [
  'Tout',
  'Agriculture',
  'Informatique',
  'Électronique',
  'Immobilier',
  'Mécanique',
  'Santé',
  'Éducation',
  'Finance',
]

export interface ProfilProfessionnel {
  id: string
  label: string
  icon: string
  color: string
}

export const PROFILS_PROFESSIONNELS: ProfilProfessionnel[] = [
  { id: 'tous', label: 'Tous les profils', icon: 'fas fa-users', color: 'gray' },
  { id: 'recherche_emploi', label: 'En recherche d\'emploi', icon: 'fas fa-search', color: 'red' },
  { id: 'en_emploi', label: 'En Emploi', icon: 'fas fa-briefcase', color: 'green' },
  { id: 'consultance', label: 'Consultant', icon: 'fas fa-user-tie', color: 'blue' },
  { id: 'volontariat_expertise', label: 'Volontariat (partage d\'expertise)', icon: 'fas fa-heart', color: 'purple' },
  { id: 'recherche_nouvelles_opportunites', label: 'En Emploi mais recherche de nouvelles opportunités', icon: 'fas fa-exchange-alt', color: 'orange' },
]

export interface PaysOption {
  value: string
  label: string
}

export const PAYS_EXPERTS: PaysOption[] = [
  { value: '', label: 'Tous les territoires' },
  { value: 'Sénégal', label: 'Sénégal' },
  { value: 'Côte d\'Ivoire', label: 'Côte d\'Ivoire' },
  { value: 'Cameroun', label: 'Cameroun' },
  { value: 'Mali', label: 'Mali' },
  { value: 'Burkina Faso', label: 'Burkina Faso' },
  { value: 'Bénin', label: 'Bénin' },
  { value: 'Togo', label: 'Togo' },
  { value: 'Niger', label: 'Niger' },
  { value: 'Guinée', label: 'Guinée' },
  { value: 'Congo', label: 'Congo' },
  { value: 'RD Congo', label: 'RD Congo' },
  { value: 'Gabon', label: 'Gabon' },
  { value: 'Rwanda', label: 'Rwanda' },
  { value: 'Maroc', label: 'Maroc' },
  { value: 'Tunisie', label: 'Tunisie' },
  { value: 'Algérie', label: 'Algérie' },
  { value: 'Nigeria', label: 'Nigeria' },
  { value: 'Ghana', label: 'Ghana' },
  { value: 'Kenya', label: 'Kenya' },
  { value: 'Afrique du Sud', label: 'Afrique du Sud' },
  { value: 'France', label: 'France' },
  { value: 'Belgique', label: 'Belgique' },
  { value: 'Canada', label: 'Canada' },
  { value: 'États-Unis', label: 'États-Unis' },
]

/**
 * Les 54 territoires du continent africain (noms français, avec accents).
 * Utilisé par le filtre territoire (zone « Afrique »).
 */
export const PAYS_AFRIQUE: string[] = [
  'Afrique du Sud', 'Algérie', 'Angola', 'Bénin', 'Botswana', 'Burkina Faso',
  'Burundi', 'Cameroun', 'Cap-Vert', 'Centrafrique', 'Comores', 'Congo',
  'Côte d\'Ivoire', 'Djibouti', 'Égypte', 'Érythrée', 'Eswatini', 'Éthiopie',
  'Gabon', 'Gambie', 'Ghana', 'Guinée', 'Guinée-Bissau', 'Guinée équatoriale',
  'Kenya', 'Lesotho', 'Liberia', 'Libye', 'Madagascar', 'Malawi', 'Mali',
  'Maroc', 'Maurice', 'Mauritanie', 'Mozambique', 'Namibie', 'Niger',
  'Nigeria', 'Ouganda', 'RD Congo', 'Rwanda', 'São Tomé-et-Príncipe',
  'Sénégal', 'Seychelles', 'Sierra Leone', 'Somalie', 'Soudan',
  'Soudan du Sud', 'Tanzanie', 'Tchad', 'Togo', 'Tunisie', 'Zambie', 'Zimbabwe',
]

/**
 * Tous les territoires du monde hors Afrique (noms français, avec accents).
 * Utilisé par le filtre territoire (zone « Hors Afrique »).
 */
export const PAYS_HORS_AFRIQUE: string[] = [
  // Europe
  'Albanie', 'Allemagne', 'Andorre', 'Autriche', 'Belgique', 'Biélorussie',
  'Bosnie-Herzégovine', 'Bulgarie', 'Chypre', 'Croatie', 'Danemark', 'Espagne',
  'Estonie', 'Finlande', 'France', 'Grèce', 'Hongrie', 'Irlande', 'Islande',
  'Italie', 'Kosovo', 'Lettonie', 'Liechtenstein', 'Lituanie', 'Luxembourg',
  'Macédoine du Nord', 'Malte', 'Moldavie', 'Monaco', 'Monténégro', 'Norvège',
  'Pays-Bas', 'Pologne', 'Portugal', 'République tchèque', 'Roumanie',
  'Royaume-Uni', 'Russie', 'Saint-Marin', 'Serbie', 'Slovaquie', 'Slovénie',
  'Suède', 'Suisse', 'Ukraine', 'Vatican',
  // Amériques
  'Antigua-et-Barbuda', 'Argentine', 'Bahamas', 'Barbade', 'Belize', 'Bolivie',
  'Brésil', 'Canada', 'Chili', 'Colombie', 'Costa Rica', 'Cuba', 'Dominique',
  'El Salvador', 'Équateur', 'États-Unis', 'Grenade', 'Guatemala', 'Guyana',
  'Haïti', 'Honduras', 'Jamaïque', 'Mexique', 'Nicaragua', 'Panama', 'Paraguay',
  'Pérou', 'République dominicaine', 'Saint-Kitts-et-Nevis', 'Sainte-Lucie',
  'Saint-Vincent-et-les-Grenadines', 'Suriname', 'Trinité-et-Tobago', 'Uruguay',
  'Venezuela',
  // Asie
  'Afghanistan', 'Arabie saoudite', 'Arménie', 'Azerbaïdjan', 'Bahreïn',
  'Bangladesh', 'Bhoutan', 'Birmanie', 'Brunei', 'Cambodge', 'Chine',
  'Corée du Nord', 'Corée du Sud', 'Émirats arabes unis', 'Géorgie', 'Inde',
  'Indonésie', 'Irak', 'Iran', 'Israël', 'Japon', 'Jordanie', 'Kazakhstan',
  'Kirghizistan', 'Koweït', 'Laos', 'Liban', 'Malaisie', 'Maldives', 'Mongolie',
  'Népal', 'Oman', 'Ouzbékistan', 'Pakistan', 'Palestine', 'Philippines',
  'Qatar', 'Singapour', 'Sri Lanka', 'Syrie', 'Tadjikistan', 'Thaïlande',
  'Timor oriental', 'Turkménistan', 'Turquie', 'Viêt Nam', 'Yémen',
  // Océanie
  'Australie', 'Fidji', 'Îles Marshall', 'Îles Salomon', 'Kiribati',
  'Micronésie', 'Nauru', 'Nouvelle-Zélande', 'Palaos',
  'Papouasie-Nouvelle-Guinée', 'Samoa', 'Tonga', 'Tuvalu', 'Vanuatu',
]

/**
 * Liste complète des territoires pour le filtre « sur mesure » :
 * les 54 pays d'Afrique + principaux territoires de la diaspora.
 */
export const TERRITOIRES_EXPERTS: string[] = [
  // Afrique (54)
  'Afrique du Sud', 'Algérie', 'Angola', 'Bénin', 'Botswana', 'Burkina Faso',
  'Burundi', 'Cameroun', 'Cap-Vert', 'Centrafrique', 'Comores', 'Congo',
  'Côte d\'Ivoire', 'Djibouti', 'Égypte', 'Érythrée', 'Eswatini', 'Éthiopie',
  'Gabon', 'Gambie', 'Ghana', 'Guinée', 'Guinée-Bissau', 'Guinée équatoriale',
  'Kenya', 'Lesotho', 'Liberia', 'Libye', 'Madagascar', 'Malawi', 'Mali',
  'Maroc', 'Maurice', 'Mauritanie', 'Mozambique', 'Namibie', 'Niger',
  'Nigeria', 'Ouganda', 'RD Congo', 'Rwanda', 'São Tomé-et-Príncipe',
  'Sénégal', 'Seychelles', 'Sierra Leone', 'Somalie', 'Soudan',
  'Soudan du Sud', 'Tanzanie', 'Tchad', 'Togo', 'Tunisie', 'Zambie', 'Zimbabwe',
  // Diaspora
  'France', 'Belgique', 'Suisse', 'Luxembourg', 'Royaume-Uni', 'Allemagne',
  'Espagne', 'Italie', 'Portugal', 'Pays-Bas', 'Canada', 'États-Unis',
  'Brésil', 'Haïti', 'Jamaïque', 'Émirats arabes unis', 'Arabie saoudite',
  'Qatar', 'Chine', 'Inde', 'Australie',
]

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useExperts = () => {
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
   * Lister les experts avec filtres et pagination
   */
  const listerExperts = async (filtres: ExpertFiltres = {}): Promise<ExpertListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.domaine && filtres.domaine !== 'Tout') params.set('domaine', filtres.domaine)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.situation && filtres.situation !== 'tous') params.set('situation', filtres.situation)
      if (filtres.specialite && filtres.specialite !== 'toutes') params.set('specialite', filtres.specialite)
      // « Tout » = aucune restriction de zone : on n'envoie pas le paramètre.
      if (filtres.zone && filtres.zone !== 'tout') params.set('zone', filtres.zone)
      if (filtres.tri) params.set('tri', filtres.tri)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/experts${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ExpertListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des experts')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerExperts:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Lister les spécialités réellement déclarées par les experts validés.
   * Alimente le menu déroulant du filtre par spécialité.
   */
  const listerSpecialites = async (): Promise<string[]> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<string[]>>(`${apiBase}/api/experts/specialites`)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des spécialités')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerSpecialites:', e)
      return []
    }
  }

  /**
   * Obtenir un expert par son ID (utilisateur_id)
   */
  const obtenirExpert = async (id: string): Promise<ExpertAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ExpertAPI>>(
        `${apiBase}/api/experts/${id}`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Expert non trouve')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirExpert:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Creer une candidature expert (JWT requis)
   */
  const creerCandidature = async (body: CandidatureExpertBody): Promise<ExpertAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ExpertAPI>>(
        `${apiBase}/api/experts/candidature`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...authHeaders(),
          },
          body,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation de la candidature')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerCandidature:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Uploader un CV (PDF) et récupérer son URL (JWT requis).
   */
  const uploaderCV = async (fichier: File): Promise<string | null> => {
    erreur.value = null
    try {
      const formData = new FormData()
      formData.append('cv', fichier)

      const reponse = await $fetch<ApiResponse<{ cv_url: string }>>(
        `${apiBase}/api/experts/cv`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: formData,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l\'upload du CV')
      }

      return reponse.data.cv_url
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur uploaderCV:', e)
      throw new Error(message)
    }
  }

  /**
   * Obtenir la candidature active du membre connecte (suivi US3).
   * Renvoie null si aucune candidature active.
   */
  const obtenirMaCandidature = async (): Promise<MaCandidatureAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<MaCandidatureAPI | null>>(
        `${apiBase}/api/experts/moi`,
        { headers: authHeaders() },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors du chargement de la candidature')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirMaCandidature:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Noter un expert (1–5, JWT requis). Renvoie la nouvelle moyenne.
   */
  const noterExpert = async (id: string, note: number): Promise<NoteExpertAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<NoteExpertAPI>>(
        `${apiBase}/api/experts/${id}/note`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...authHeaders(),
          },
          body: { note },
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la notation')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur noterExpert:', e)
      throw new Error(message)
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerExperts,
    listerSpecialites,
    obtenirExpert,
    creerCandidature,
    uploaderCV,
    obtenirMaCandidature,
    noterExpert,
  }
}

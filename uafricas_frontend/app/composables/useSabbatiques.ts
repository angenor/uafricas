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
  type_organisation: string | null
  type_organisation_label: string | null
  statut_legal: string | null
  candidat_retenu: CandidatRetenu | null
  user: SabbatiqueOrganisateur
  created_at: string
  updated_at: string
}

/** Candidat retenu (sélection finale, affichage public) */
export interface CandidatRetenu {
  uid: string
  nom: string
  prenom: string | null
  retenu_at: string | null
}

/** DTO pour le detail d'un programme */
export interface SabbatiqueDetailAPI extends SabbatiqueAPI {
  slug: string | null
  document_url: string | null
  adresse: string | null
  prise_en_charge_details: string | null
  prerequis: string | null
  langues_requises: string | null
  type_organisation: string | null
  type_organisation_label: string | null
  candidat_retenu: CandidatRetenu | null
  est_organisateur: boolean
  a_deja_candidate: boolean
}

/** Données d'une candidature (vue organisateur) */
export interface CandidatureAPI {
  id: string
  candidat: SabbatiqueOrganisateur
  nom_etat_civil: string | null
  fonction_actuelle: string | null
  lieu_residence: string | null
  statut_emploi: string | null
  statut_emploi_label: string | null
  repond_profil: boolean
  lettre_motivation: string | null
  cv_url: string | null
  lien_expertise: string | null
  statut: string
  est_retenu: boolean
  created_at: string
}

/** Données saisies dans le formulaire de candidature */
export interface CandidatureForm {
  nomEtatCivil: string
  fonctionActuelle: string
  lieuResidence: string
  statutEmploi: StatutEmploi | ''
  repondProfil: boolean
  lettreMotivation?: string
  lienExpertise?: string
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

// Durée du programme : minimum 2 semaines, maximum 12 mois (1 an)
export const DUREES: { value: string; label: string }[] = [
  { value: '2_semaines', label: '2 semaines' },
  { value: '3_semaines', label: '3 semaines' },
  { value: '6_semaines', label: '6 semaines' },
  { value: '1_mois', label: '1 mois' },
  { value: '2_mois', label: '2 mois' },
  { value: '3_mois', label: '3 mois' },
  { value: '6_mois', label: '6 mois' },
  { value: '1_an', label: '12 mois (1 an)' },
]

// Types d'organisation soumettante (proposition d'un échange)
export type TypeOrganisation = 'association' | 'entreprise' | 'service_public'

export const TYPES_ORGANISATION: { value: TypeOrganisation; label: string; icon: string }[] = [
  { value: 'association', label: 'Association', icon: 'handshake' },
  { value: 'entreprise', label: 'Entreprise', icon: 'building' },
  { value: 'service_public', label: 'Service public', icon: 'landmark' },
]

// Statut d'emploi requis pour candidater (jamais « sans emploi »)
export type StatutEmploi = 'en_emploi' | 'retraite'

export const STATUTS_EMPLOI: { value: StatutEmploi; label: string }[] = [
  { value: 'en_emploi', label: 'En emploi' },
  { value: 'retraite', label: 'Retraité(e)' },
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

  /** Préfixe une URL relative d'upload (/uploads/...) avec l'origine du backend */
  const mapperUrl = (url: string | null): string | null => {
    if (!url) return url
    if (url.startsWith('http')) return url
    return `${apiBase}${url}`
  }

  /** Normalise les URLs d'upload d'un programme (couverture + photo organisateur) */
  const normaliserProgramme = <T extends SabbatiqueAPI>(p: T): T => {
    p.couverture_url = mapperUrl(p.couverture_url)
    if (p.user) p.user.photo_url = mapperUrl(p.user.photo_url)
    return p
  }

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

      reponse.data.programmes.forEach(normaliserProgramme)
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
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Programme non trouve')
      }

      normaliserProgramme(reponse.data)
      reponse.data.document_url = mapperUrl(reponse.data.document_url)
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
      typeOrganisation: string
      statutLegal?: string
      titre: string
      description: string
      domaine: string
      domainePrecision?: string
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
      data.append('type_organisation', formData.typeOrganisation)
      if (formData.statutLegal) data.append('statut_legal', formData.statutLegal)
      data.append('titre', formData.titre)
      data.append('description', formData.description)
      data.append('domaine', formData.domaine)
      if (formData.domainePrecision) data.append('domaine_precision', formData.domainePrecision)
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

      normaliserProgramme(reponse.data)
      reponse.data.document_url = mapperUrl(reponse.data.document_url)
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

  /**
   * Lister les programmes créés par l'utilisateur connecté (tous statuts)
   */
  const listerMesProgrammes = async (): Promise<SabbatiqueAPI[] | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<SabbatiqueAPI[]>>(
        `${apiBase}/api/sabbatiques/mes-programmes`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement de vos programmes')
      }
      reponse.data.forEach(normaliserProgramme)
      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerMesProgrammes:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Candidater à un programme (multipart pour le CV facultatif)
   */
  const candidater = async (
    programmeId: string,
    form: CandidatureForm,
    cvFile: File | null,
  ): Promise<boolean> => {
    chargement.value = true
    erreur.value = null

    try {
      const data = new FormData()
      data.append('nom_etat_civil', form.nomEtatCivil)
      data.append('fonction_actuelle', form.fonctionActuelle)
      data.append('lieu_residence', form.lieuResidence)
      data.append('statut_emploi', form.statutEmploi)
      data.append('repond_profil', form.repondProfil ? 'true' : 'false')
      if (form.lettreMotivation) data.append('lettre_motivation', form.lettreMotivation)
      if (form.lienExpertise) data.append('lien_expertise', form.lienExpertise)
      if (cvFile) data.append('cv', cvFile)

      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/sabbatiques/${programmeId}/candidatures`,
        { method: 'POST', headers: authHeaders(), body: data },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors de la candidature')
      }
      return true
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur candidater:', e)
      return false
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Lister les candidatures d'un programme (organisateur uniquement)
   */
  const listerCandidatures = async (
    programmeId: string,
  ): Promise<CandidatureAPI[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<CandidatureAPI[]>>(
        `${apiBase}/api/sabbatiques/${programmeId}/candidatures`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des candidatures')
      }
      reponse.data.forEach((c) => {
        c.cv_url = mapperUrl(c.cv_url)
        if (c.candidat) c.candidat.photo_url = mapperUrl(c.candidat.photo_url)
      })
      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerCandidatures:', e)
      return null
    }
  }

  /**
   * Sélectionner le candidat final (organisateur uniquement)
   */
  const selectionnerCandidat = async (
    programmeId: string,
    candidatureId: string,
  ): Promise<boolean> => {
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/sabbatiques/${programmeId}/candidatures/${candidatureId}/retenir`,
        { method: 'POST', headers: authHeaders() },
      )
      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors de la sélection')
      }
      return true
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur selectionnerCandidat:', e)
      return false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerProgrammes,
    obtenirProgramme,
    creerProgramme,
    listerMesProgrammes,
    candidater,
    listerCandidatures,
    selectionnerCandidat,
  }
}

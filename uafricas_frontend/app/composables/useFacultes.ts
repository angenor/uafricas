import type { Faculte } from '~/mocks/inuda/facultes'

interface FaculteAPI {
  id: string
  titre: string
  acronyme: string
  description: string
  imageCouverture: string | null
  ecolePartenaire: {
    nom: string
    ville: string
    pays: string
    type: 'publique' | 'privee'
    siteWeb: string | null
    emailContact: string
    telephoneContact: string | null
    whatsappContact: string | null
  }
  domainesEtudes: string[]
  accepteNouveauxInscrits: boolean
  stats: {
    nombreInscritsTotal: number
    nombreInscritsAnneeEnCours: number
  }
}

interface FaculteDetailAPI extends FaculteAPI {
  slug: string | null
  logo: string | null
  programmesResume: {
    licence: string[]
    master: string[]
    doctorat: string[]
    certificats: string[]
  }
  conditionsAdmission: {
    diplomeMinimum: string | null
    languesEnseignement: string[]
    fraisScolariteAnnuels: {
      min: number | null
      max: number | null
      boursesPossibles: boolean
    }
    periodesInscription: string | null
  }
  pointsForts: string[]
  statut: 'active' | 'inactive'
  referentPlateforme: {
    prenom: string | null
    nom: string
    role: string
    email: string
  } | null
}

interface FaculteListeAPI {
  facultes: FaculteAPI[]
  domaines: string[]
  total: number
  page: number
  parPage: number
  totalPages: number
}

interface FiltresFacultes {
  recherche?: string
  domaine?: string
  typeEcole?: string
  ouvertes?: boolean
  page?: number
  parPage?: number
}

function mapFaculteAPIToFaculte(api: FaculteAPI): Faculte {
  return {
    id: api.id,
    titre: api.titre,
    acronyme: api.acronyme,
    description: api.description,
    imageCouverture: api.imageCouverture || undefined,
    ecolePartenaire: {
      nom: api.ecolePartenaire.nom,
      ville: api.ecolePartenaire.ville,
      pays: api.ecolePartenaire.pays,
      type: api.ecolePartenaire.type,
      siteWeb: api.ecolePartenaire.siteWeb || undefined,
      emailContact: api.ecolePartenaire.emailContact,
      telephoneContact: api.ecolePartenaire.telephoneContact || undefined,
      whatsappContact: api.ecolePartenaire.whatsappContact || undefined,
    },
    domainesEtudes: api.domainesEtudes,
    // Valeurs par defaut pour les champs non retournes dans le listing
    programmesResume: { licence: [], master: [], doctorat: [], certificats: [] },
    conditionsAdmission: {
      diplomeMinimum: '',
      languesEnseignement: [],
      fraisScolariteAnnuels: { min: 0, max: 0, boursesPossibles: false },
      periodesInscription: '',
    },
    pointsForts: [],
    accepteNouveauxInscrits: api.accepteNouveauxInscrits,
    statut: 'active',
    stats: {
      nombreInscritsTotal: api.stats.nombreInscritsTotal,
      nombreInscritsAnneeEnCours: api.stats.nombreInscritsAnneeEnCours,
    },
  }
}

function mapDetailAPIToFaculte(api: FaculteDetailAPI): Faculte {
  return {
    id: api.id,
    titre: api.titre,
    acronyme: api.acronyme,
    description: api.description,
    imageCouverture: api.imageCouverture || undefined,
    logo: api.logo || undefined,
    ecolePartenaire: {
      nom: api.ecolePartenaire.nom,
      ville: api.ecolePartenaire.ville,
      pays: api.ecolePartenaire.pays,
      type: api.ecolePartenaire.type,
      siteWeb: api.ecolePartenaire.siteWeb || undefined,
      emailContact: api.ecolePartenaire.emailContact,
      telephoneContact: api.ecolePartenaire.telephoneContact || undefined,
      whatsappContact: api.ecolePartenaire.whatsappContact || undefined,
    },
    domainesEtudes: api.domainesEtudes,
    programmesResume: api.programmesResume,
    conditionsAdmission: {
      diplomeMinimum: api.conditionsAdmission.diplomeMinimum || '',
      languesEnseignement: api.conditionsAdmission.languesEnseignement,
      fraisScolariteAnnuels: {
        min: api.conditionsAdmission.fraisScolariteAnnuels.min || 0,
        max: api.conditionsAdmission.fraisScolariteAnnuels.max || 0,
        boursesPossibles: api.conditionsAdmission.fraisScolariteAnnuels.boursesPossibles,
      },
      periodesInscription: api.conditionsAdmission.periodesInscription || '',
    },
    pointsForts: api.pointsForts,
    accepteNouveauxInscrits: api.accepteNouveauxInscrits,
    statut: api.statut,
    stats: {
      nombreInscritsTotal: api.stats.nombreInscritsTotal,
      nombreInscritsAnneeEnCours: api.stats.nombreInscritsAnneeEnCours,
    },
    referentPlateforme: api.referentPlateforme
      ? {
          prenom: api.referentPlateforme.prenom || '',
          nom: api.referentPlateforme.nom,
          role: api.referentPlateforme.role,
          email: api.referentPlateforme.email,
        }
      : undefined,
  }
}

export function useFacultes() {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const loading = ref(false)
  const erreur = ref<string | null>(null)

  const listerFacultes = async (filtres: FiltresFacultes = {}) => {
    loading.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.domaine) params.set('domaine', filtres.domaine)
      if (filtres.typeEcole) params.set('type_ecole', filtres.typeEcole)
      if (filtres.ouvertes) params.set('ouvertes', 'true')
      if (filtres.page) params.set('page', filtres.page.toString())
      if (filtres.parPage) params.set('par_page', filtres.parPage.toString())

      const queryString = params.toString()
      const url = `${apiBase}/api/facultes${queryString ? '?' + queryString : ''}`

      const response = await $fetch<{ success: boolean; data: FaculteListeAPI }>(url)

      if (response.success && response.data) {
        return {
          facultes: response.data.facultes.map(mapFaculteAPIToFaculte),
          domaines: response.data.domaines,
          total: response.data.total,
          page: response.data.page,
          parPage: response.data.parPage,
          totalPages: response.data.totalPages,
        }
      }
      throw new Error('Erreur lors du chargement des facultés')
    } catch (e: any) {
      erreur.value = e.message || 'Erreur lors du chargement des facultés'
      throw e
    } finally {
      loading.value = false
    }
  }

  const obtenirFaculte = async (id: string) => {
    loading.value = true
    erreur.value = null

    try {
      const response = await $fetch<{ success: boolean; data: FaculteDetailAPI }>(
        `${apiBase}/api/facultes/${id}`
      )

      if (response.success && response.data) {
        return mapDetailAPIToFaculte(response.data)
      }
      throw new Error('Faculté non trouvée')
    } catch (e: any) {
      erreur.value = e.message || 'Erreur lors du chargement de la faculté'
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    loading,
    erreur,
    listerFacultes,
    obtenirFaculte,
  }
}

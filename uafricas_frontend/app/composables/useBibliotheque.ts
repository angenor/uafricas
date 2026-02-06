// Composable pour les appels API de la bibliotheque numerique

/** Interface correspondant au DTO LivreResponse du backend */
export interface LivreAPI {
  id: string
  titre: string
  slug: string | null
  description: string
  image_couverture_url: string | null
  document_pdf_url: string
  type_document: string
  acces: string
  info_auteur: string
  date_publication: string | null
  rapport_auteur: string | null
  acceptation_diffusion: boolean
  nombre_telechargements: number
  nombre_vues: number
  etat: string
  created_at: string
}

/** Interface de reponse paginee du backend */
export interface LivreListeAPI {
  livres: LivreAPI[]
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
export interface LivreFiltres {
  recherche?: string
  type_document?: string
  page?: number
  par_page?: number
}

/** Donnees du formulaire d'ajout */
export interface LivreFormData {
  titre: string
  description: string
  type: string
  acces: string
  auteurBiblio: string
  datePublication: string
  rapport: string
  consent: boolean
}

/** Mapper l'acces DB vers l'affichage frontend */
function mapperAccesFrontend(accesDb: string): string {
  switch (accesDb) {
    case 'lecture_seule':
      return 'Lecture'
    case 'lecture_telechargement':
      return 'Téléchargeable'
    default:
      return accesDb
  }
}

/** Mapper l'acces frontend vers la valeur DB */
function mapperAccesDb(accesFrontend: string): string {
  switch (accesFrontend) {
    case 'Lecture':
      return 'lecture_seule'
    case 'Téléchargeable':
      return 'lecture_telechargement'
    default:
      return accesFrontend
  }
}

/** Transformer les URLs relatives en URLs absolues */
function mapperUrlsLivre(livre: LivreAPI, apiBase: string): LivreAPI {
  return {
    ...livre,
    image_couverture_url: livre.image_couverture_url
      ? `${apiBase}${livre.image_couverture_url}`
      : null,
    document_pdf_url: livre.document_pdf_url.startsWith('http')
      ? livre.document_pdf_url
      : `${apiBase}${livre.document_pdf_url}`,
    acces: mapperAccesFrontend(livre.acces),
  }
}

export const useBibliotheque = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /**
   * Recuperer la liste des livres avec filtres et pagination
   */
  const listerLivres = async (filtres: LivreFiltres = {}): Promise<LivreListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.type_document) params.set('type_document', filtres.type_document)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/livres${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<LivreListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des livres')
      }

      reponse.data.livres = reponse.data.livres.map(livre => mapperUrlsLivre(livre, apiBase))

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerLivres:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Recuperer un livre par son ID
   */
  const obtenirLivre = async (id: string): Promise<LivreAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<LivreAPI>>(
        `${apiBase}/api/livres/${id}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Livre non trouve')
      }

      return mapperUrlsLivre(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirLivre:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Creer un nouveau livre avec upload de fichiers
   */
  const creerLivre = async (
    formData: LivreFormData,
    imageFichier: File | null,
    pdfFichier: File,
  ): Promise<LivreAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const data = new FormData()
      data.append('titre', formData.titre)
      data.append('description', formData.description)
      data.append('type_document', formData.type)
      data.append('acces', mapperAccesDb(formData.acces))
      data.append('info_auteur', formData.auteurBiblio)
      data.append('date_publication', formData.datePublication)
      data.append('rapport_auteur', formData.rapport)
      data.append('acceptation_diffusion', String(formData.consent))

      if (imageFichier) {
        data.append('image', imageFichier)
      }
      data.append('document', pdfFichier)

      const reponse = await $fetch<ApiResponse<LivreAPI>>(
        `${apiBase}/api/livres`,
        {
          method: 'POST',
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation du livre')
      }

      return mapperUrlsLivre(reponse.data, apiBase)
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerLivre:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Supprimer un livre (suppression douce)
   */
  const supprimerLivre = async (id: string): Promise<boolean> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/livres/${id}`,
        { method: 'DELETE' },
      )

      return reponse.success
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur supprimerLivre:', e)
      return false
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerLivres,
    obtenirLivre,
    creerLivre,
    supprimerLivre,
  }
}

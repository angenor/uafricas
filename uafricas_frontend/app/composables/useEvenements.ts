// Composable pour les appels API Evenements
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type TypeEvenement = 'En ligne' | 'En présentiel' | 'Hybride'
export type StatutEvenement = 'a_venir' | 'en_cours' | 'termine' | 'annule'

export interface EvenementOrganisateur {
  uid: string
  nom: string
  prenom: string | null
  email: string
  photo_url: string | null
}

/** DTO correspondant a EvenementResponse du backend */
export interface EvenementAPI {
  id: string
  titre: string
  description: string
  type: string
  /** Thématique de l'événement (colonne `type` en base) */
  thematique: string | null
  pays: string | null
  ville: string | null
  date_heure_debut: string
  date_heure_fin: string | null
  couverture_url: string | null
  statut: StatutEvenement
  nombre_places: number | null
  nombre_inscrits: number
  user: EvenementOrganisateur
  created_at: string
  updated_at: string
}

/** DTO pour le detail d'un evenement */
export interface EvenementDetailAPI extends EvenementAPI {
  slug: string | null
  adresse: string | null
  lien_en_ligne: string | null
  est_inscrit: boolean
  /** Etat brut du cycle de vie (brouillon/publie/annule/termine/suspendu) */
  etat: string
  type_organisateur: 'personnel' | 'organisation'
  contact_nom: string | null
  contact_email: string | null
  contact_telephone: string | null
  contact_site_web: string | null
  /** Lien d'enregistrement vidéo (rediffusion YouTube), affiché quand l'événement est terminé */
  enregistrement_url: string | null
}

/** Un inscrit a un evenement (vue organisateur) */
export interface InscritEvenement {
  utilisateur_id: string
  nom: string
  prenom: string | null
  email: string
  statut: string
  created_at: string
}

/** Payload de modification d'un evenement par son organisateur */
export interface ModifierMonEvenementPayload {
  titre?: string
  description?: string
  type?: string
  pays?: string
  ville?: string
  adresse?: string
  date_heure_debut?: string
  date_heure_fin?: string
  lien_en_ligne?: string
  nombre_places?: number | null
  type_organisateur?: 'personnel' | 'organisation'
  contact_nom?: string
  contact_email?: string
  contact_telephone?: string
  contact_site_web?: string
  enregistrement_url?: string
}

/** Reponse paginee */
export interface EvenementListeAPI {
  evenements: EvenementAPI[]
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

// ── Direct en streaming (feature 001-evenements-streaming) ──────

export type StatutDirect = 'indisponible' | 'en_attente' | 'en_direct' | 'termine'
export type RoleDirect = 'organisateur' | 'intervenant' | 'spectateur'

/** Une demande de parole (vue organisateur). */
export interface DemandeParole {
  utilisateur_id: string
  nom: string
  main_levee_at: string | null
}

/** DTO correspondant a EtatDirectResponse du backend (GET …/direct). */
export interface EtatDirect {
  statut_direct: StatutDirect
  peut_ouvrir: boolean
  peut_rejoindre: boolean
  est_organisateur: boolean
  est_inscrit: boolean
  session_id: string | null
  nombre_participants: number
  max_participants: number
  fenetre_ouverture_at: string
  demandes_parole: DemandeParole[]
}

/** DTO correspondant a TokenDirectResponse du backend (POST …/rejoindre). */
export interface TokenDirect {
  session_id: string
  room_name: string
  livekit_url: string
  token: string
  role: RoleDirect
}

/** Evenement SSE pousse a l'ouverture d'un direct (parite event_stream_*). */
export interface EvenementStream {
  type: string
  evenement_id: string
}

/** Extrait un message d'erreur francais de l'enveloppe ApiResponse renvoyee par $fetch. */
const extraireErreurDirect = (e: unknown, defaut: string): string => {
  const data = (e as { data?: { error?: string } })?.data
  return data?.error || (e as { message?: string })?.message || defaut
}

/** Parametres de filtre pour le listing */
export interface EvenementFiltres {
  recherche?: string
  format?: string
  pays?: string
  annee?: number
  page?: number
  par_page?: number
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const TYPES_EVENEMENT: { value: string; label: string }[] = [
  { value: '', label: 'Tous les types' },
  { value: 'en_ligne', label: 'En ligne' },
  { value: 'presentiel', label: 'En présentiel' },
  { value: 'hybride', label: 'Hybride' },
]

export const ANNEES = ['2026', '2027', '2028', '2029']

/** Thématiques proposées pour un événement (panafricain / développement durable) */
export const THEMATIQUES_EVENEMENT = [
  'Développement durable',
  'Environnement & Climat',
  'Entrepreneuriat',
  'Éducation & Formation',
  'Santé & Bien-être',
  'Agriculture & Agroalimentaire',
  'Technologie & Numérique',
  'Culture & Arts',
  'Gouvernance & Société civile',
  'Énergie',
  'Économie & Finance',
  'Genre & Inclusion',
  'Jeunesse & Leadership',
  'Tourisme & Patrimoine',
  'Sport',
  'Autre',
]

export const PAYS_AFRICAINS = [
  'Afrique du Sud',
  'Algérie',
  'Bénin',
  'Burkina Faso',
  'Cameroun',
  'Cap-Vert',
  'Comores',
  'Côte d\'Ivoire',
  'Égypte',
  'Éthiopie',
  'Gabon',
  'Gambie',
  'Ghana',
  'Guinée',
  'Kenya',
  'Madagascar',
  'Mali',
  'Maroc',
  'Maurice',
  'Mauritanie',
  'Namibie',
  'Niger',
  'Nigeria',
  'RDC',
  'Rwanda',
  'Sénégal',
  'Tanzanie',
  'Togo',
  'Tunisie',
]

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

/** Mapper format DB vers label frontend */
export const mapperFormatFrontend = (format: string): string => {
  const map: Record<string, string> = {
    en_ligne: 'En ligne',
    presentiel: 'En présentiel',
    hybride: 'Hybride',
  }
  return map[format] || format
}

/** Mapper format frontend vers valeur DB */
export const mapperFormatDb = (type: string): string => {
  const map: Record<string, string> = {
    'En ligne': 'en_ligne',
    'En présentiel': 'presentiel',
    'Hybride': 'hybride',
  }
  return map[type] || type.toLowerCase().replace(/ /g, '_')
}

/** Formater une date ISO en francais complet (ex: "mardi 15 janvier 2025") */
export const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

/** Formater une date ISO en format court (ex: "15 janvier 2025") */
export const formatDateShort = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
}

/** Extraire l'heure d'une date ISO (ex: "14:30") */
export const getHeure = (dateStr: string | null): string => {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  return date.toLocaleTimeString('fr-FR', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
  })
}

// `youtubeEmbedUrl` a été déplacé dans `~/utils/media`, où il rejoint les
// autres utilitaires de routage des médias (radio, télé, vidafrica). Le
// ré-exporter d'ici créerait un doublon d'auto-import Nuxt : les consommateurs
// l'importent désormais directement depuis `~/utils/media`.

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useEvenements = () => {
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
   * Lister les evenements avec filtres et pagination
   */
  const listerEvenements = async (filtres: EvenementFiltres = {}): Promise<EvenementListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.format) params.set('format', filtres.format)
      if (filtres.pays) params.set('pays', filtres.pays)
      if (filtres.annee) params.set('annee', String(filtres.annee))
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/evenements${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<EvenementListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des evenements')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerEvenements:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir un evenement par son ID
   */
  const obtenirEvenement = async (id: string): Promise<EvenementDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<EvenementDetailAPI>>(
        `${apiBase}/api/evenements/${id}`,
        { headers: authHeaders() },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Evenement non trouve')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirEvenement:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Creer un evenement (multipart pour l'image de couverture)
   */
  const creerEvenement = async (
    formData: {
      titre: string
      description: string
      type: string
      thematique?: string
      pays: string
      ville: string
      date_heure_debut: string
      date_heure_fin: string
      adresse?: string
      lien_en_ligne?: string
      nombre_places?: number | null
      type_organisateur?: 'personnel' | 'organisation'
      contact_nom?: string
      contact_email?: string
      contact_telephone?: string
      contact_site_web?: string
      enregistrement_url?: string
    },
    couvertureFile: File | null,
  ): Promise<EvenementDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const data = new FormData()
      data.append('titre', formData.titre)
      data.append('description', formData.description)
      data.append('format', mapperFormatDb(formData.type))
      if (formData.thematique) data.append('thematique', formData.thematique)
      data.append('pays', formData.pays)
      data.append('ville', formData.ville)
      data.append('date_heure_debut', formData.date_heure_debut)
      data.append('date_heure_fin', formData.date_heure_fin)
      if (formData.adresse) data.append('adresse', formData.adresse)
      if (formData.lien_en_ligne) data.append('lien_en_ligne', formData.lien_en_ligne)
      if (formData.nombre_places != null) data.append('nombre_places', String(formData.nombre_places))
      data.append('type_organisateur', formData.type_organisateur || 'personnel')
      if (formData.contact_nom) data.append('contact_nom', formData.contact_nom)
      if (formData.contact_email) data.append('contact_email', formData.contact_email)
      if (formData.contact_telephone) data.append('contact_telephone', formData.contact_telephone)
      if (formData.contact_site_web) data.append('contact_site_web', formData.contact_site_web)
      if (formData.enregistrement_url) data.append('enregistrement_url', formData.enregistrement_url)
      if (couvertureFile) {
        data.append('couverture', couvertureFile)
      }

      const reponse = await $fetch<ApiResponse<EvenementDetailAPI>>(
        `${apiBase}/api/evenements`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la creation de l\'evenement')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur creerEvenement:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * S'inscrire a un evenement
   */
  const inscrireEvenement = async (evenementId: string): Promise<boolean> => {
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/evenements/${evenementId}/inscription`,
        {
          method: 'POST',
          headers: authHeaders(),
        },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors de l\'inscription')
      }

      return true
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur inscrireEvenement:', e)
      return false
    }
  }

  // ── Gestion par l'organisateur (« Mes evenements ») ───────────

  /** Lister les evenements creees par le membre connecte (tous etats). */
  const listerMesEvenements = async (): Promise<EvenementDetailAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<EvenementDetailAPI[]>>(
        `${apiBase}/api/evenements/mes-evenements`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement de vos evenements')
      }
      return reponse.data
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur reseau'
      console.error('Erreur listerMesEvenements:', e)
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /** Modifier un evenement dont on est l'organisateur. */
  const modifierMonEvenement = async (
    id: string,
    payload: ModifierMonEvenementPayload,
  ): Promise<EvenementDetailAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<EvenementDetailAPI>>(
        `${apiBase}/api/evenements/${id}`,
        { method: 'PUT', headers: authHeaders(), body: payload },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la modification')
      }
      return reponse.data
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur reseau'
      console.error('Erreur modifierMonEvenement:', e)
      return null
    }
  }

  /** Supprimer (soft) un evenement dont on est l'organisateur. */
  const supprimerMonEvenement = async (id: string): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/evenements/${id}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      if (!reponse.success) throw new Error(reponse.error || 'Erreur lors de la suppression')
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur reseau'
      console.error('Erreur supprimerMonEvenement:', e)
      return false
    }
  }

  /** Remplacer l'image de couverture d'un de ses evenements. Renvoie la nouvelle URL. */
  const changerCouvertureMonEvenement = async (
    id: string,
    fichier: File,
  ): Promise<string | null> => {
    erreur.value = null
    try {
      const data = new FormData()
      data.append('couverture', fichier)
      const reponse = await $fetch<ApiResponse<{ couverture_url: string }>>(
        `${apiBase}/api/evenements/${id}/couverture`,
        { method: 'POST', headers: authHeaders(), body: data },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du changement de couverture')
      }
      return reponse.data.couverture_url
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur reseau'
      console.error('Erreur changerCouvertureMonEvenement:', e)
      return null
    }
  }

  /** Lister les inscrits a un de ses evenements. */
  const listerInscritsMonEvenement = async (id: string): Promise<InscritEvenement[]> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<InscritEvenement[]>>(
        `${apiBase}/api/evenements/${id}/inscrits`,
        { headers: authHeaders() },
      )
      if (!reponse.success || !reponse.data) return []
      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerInscritsMonEvenement:', e)
      return []
    }
  }

  // ── Direct en streaming ───────────────────────────────────────

  // Signal SSE partagé (SSR-safe) : incrémenté à chaque event_stream_* reçu, observé
  // par les pages événement/direct pour rafraîchir l'état du direct concerné.
  const signalStream = useState<{ evenement_id: string, ts: number } | null>(
    'event-stream:signal', () => null,
  )

  /** GET /api/evenements/{id}/direct — état dérivé (lecture, JWT recommandé). */
  const obtenirEtatDirect = async (id: string): Promise<EtatDirect | null> => {
    try {
      const r = await $fetch<ApiResponse<EtatDirect>>(
        `${apiBase}/api/evenements/${id}/direct`,
        { headers: authHeaders() },
      )
      return r.success ? r.data : null
    }
    catch (e) {
      console.error('Erreur obtenirEtatDirect:', e)
      return null
    }
  }

  /** POST …/rejoindre — ouvre (organisateur) ou rejoint l'active. Renvoie le token. */
  const rejoindreDirect = async (id: string): Promise<TokenDirect> => {
    try {
      const r = await $fetch<ApiResponse<TokenDirect>>(
        `${apiBase}/api/evenements/${id}/direct/rejoindre`,
        { method: 'POST', headers: authHeaders() },
      )
      if (!r.success || !r.data) throw new Error(r.error || 'Impossible de rejoindre le direct')
      return r.data
    }
    catch (e) {
      throw new Error(extraireErreurDirect(e, 'Impossible de rejoindre le direct'))
    }
  }

  /** Alias d'ouverture (côté organisateur) — la logique open-or-join est serveur. */
  const ouvrirDirect = (id: string): Promise<TokenDirect> => rejoindreDirect(id)

  /** POST …/quitter — marque l'appelant sorti (idempotent). */
  const quitterDirect = async (id: string): Promise<boolean> => {
    try {
      const r = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/evenements/${id}/direct/quitter`,
        { method: 'POST', headers: authHeaders() },
      )
      return r.success
    }
    catch (e) {
      console.error('Erreur quitterDirect:', e)
      return false
    }
  }

  /** POST …/cloturer — organisateur uniquement. */
  const cloturerDirect = async (id: string): Promise<boolean> => {
    try {
      const r = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/evenements/${id}/direct/cloturer`,
        { method: 'POST', headers: authHeaders() },
      )
      return r.success
    }
    catch (e) {
      throw new Error(extraireErreurDirect(e, 'Impossible de clôturer le direct'))
    }
  }

  /** Action de modération générique (lever-main / promouvoir / …). */
  const actionDirect = async (chemin: string, message: string): Promise<boolean> => {
    try {
      const r = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/evenements/${chemin}`,
        { method: 'POST', headers: authHeaders() },
      )
      if (!r.success) throw new Error(r.error || message)
      return true
    }
    catch (e) {
      throw new Error(extraireErreurDirect(e, message))
    }
  }

  const leverMain = (id: string): Promise<boolean> =>
    actionDirect(`${id}/direct/lever-main`, 'Impossible de lever la main')

  const promouvoir = (id: string, uid: string): Promise<boolean> =>
    actionDirect(`${id}/direct/participants/${uid}/promouvoir`, 'Impossible de promouvoir')

  const retrograder = (id: string, uid: string): Promise<boolean> =>
    actionDirect(`${id}/direct/participants/${uid}/retrograder`, 'Impossible de rétrograder')

  const retirer = (id: string, uid: string): Promise<boolean> =>
    actionDirect(`${id}/direct/participants/${uid}/retirer`, 'Impossible de retirer le participant')

  /** Sur un évènement SSE event_stream_*, publie un signal pour rafraîchir l'état du direct. */
  const gererEvenementStream = (evt: EvenementStream): void => {
    if (typeof evt?.type !== 'string' || !evt.type.startsWith('event_stream_')) return
    signalStream.value = { evenement_id: evt.evenement_id, ts: Date.now() }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerEvenements,
    obtenirEvenement,
    creerEvenement,
    inscrireEvenement,
    // Gestion organisateur
    listerMesEvenements,
    modifierMonEvenement,
    supprimerMonEvenement,
    changerCouvertureMonEvenement,
    listerInscritsMonEvenement,
    // Direct en streaming
    signalStream,
    obtenirEtatDirect,
    rejoindreDirect,
    ouvrirDirect,
    quitterDirect,
    cloturerDirect,
    leverMain,
    promouvoir,
    retrograder,
    retirer,
    gererEvenementStream,
  }
}

// Composable pour les appels API Fiches Pays (Opportunites en Afrique)

import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

export type Region =
  | 'Afrique Centrale'
  | 'Afrique de l\'Ouest'
  | 'Afrique de l\'Est'
  | 'Afrique du Nord'
  | 'Afrique Australe'

/** DTO correspondant a FichePaysResponse du backend */
export interface FichePaysAPI {
  id: string
  pays_id: string
  code: string | null
  nom: string
  capitale: string | null
  image_couverture: string | null
  slogan: string | null
  superficie: string | null
  population: string | null
  monnaie: string | null
  drapeau_url: string | null
  region: string
  nombre_contributions: number
  updated_at: string
}

/** DTO correspondant a FichePaysDetailResponse du backend */
export interface FichePaysDetailAPI extends FichePaysAPI {
  embleme_url: string | null
  devise: string | null
  hymne_national: string | null
  langue_officielle: string | null
  langues: string[]
  ethnies: string[]
  biographie: string | null
  contexte: string | null
  fuseau_horaire: string | null
  // Bloc « À savoir avant de voyager » (infos pratiques uniques par territoire)
  voyage_langue_internationale: string | null
  voyage_langue_locale: string | null
  voyage_infos_visa: string | null
  voyage_infos_sanitaires: string | null
  voyage_meteo: string | null
  voyage_prises_electriques: string | null
  voyage_contacts_tourisme: string | null
  voyage_recommandations_securite: string | null
}

/** Reponse paginee */
export interface FichePaysListeAPI {
  fiches: FichePaysAPI[]
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
export interface FichePaysFiltres {
  recherche?: string
  region?: string
  page?: number
  par_page?: number
}

// ── Contributions ──────────────────────────────────────────────

/** Auteur d'une contribution */
export interface ContributionAuteurAPI {
  id: string
  nom: string
  prenom: string
  photo_url: string | null
}

/** DTO d'une contribution */
export interface ContributionFicheAPI {
  id: string
  fiche_pays_id: string
  section: string
  type_contribution: string
  ancienne_valeur: string | null
  nouvelle_valeur: string
  justification: string | null
  etat: string
  auteur: ContributionAuteurAPI
  traite_par: string | null
  note_moderation: string | null
  traite_at: string | null
  created_at: string
}

/** Reponse paginee des contributions */
export interface ContributionListeAPI {
  contributions: ContributionFicheAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** DTO d'un contributeur (US5/T067)
 * - `utilisateur_id: null` signifie auteur anonymisé (utilisateur supprimé)
 * - `date_derniere_contribution` retournée par l'agrégation MAX(traite_at)
 */
export interface ContributeurAPI {
  utilisateur_id: string | null
  nom: string
  prenom: string
  photo_url: string | null
  nombre_contributions: number
  date_derniere_contribution: string | null
}

/** Filtres pour lister les contributions */
export interface ContributionFiltres {
  etat?: string
  section?: string
  page?: number
  par_page?: number
}

// ── Afripulse — Types partagés (alignés sur country_profile.* §III SQL SoT) ──

/** Type d'objet ciblé par une contribution Afripulse */
export type TypeObjetContribution =
  | 'fiche_pays'
  | 'site_touristique'
  | 'secteur_developpement'
  | 'personnalite_connue'
  | 'savoir_pratique'
  | 'recommandation_visiteur'
  | 'photo_visiteur'

/** Section UI Afripulse de rattachement d'une contribution */
export type SectionAfripulse =
  | 'sites_emblematiques'
  | 'sites_prives'
  | 'secteurs_opportunites'
  | 'personnalites'
  | 'savoir_avant_voyager'
  | 'recommandations'
  | 'galerie_photos'

/** Catégorie d'un site touristique */
export type CategorieSiteTouristique = 'emblematique' | 'prive'

/** Sous-type d'un site touristique (précise la famille `categorie`) — 20 valeurs */
export type SousTypeSite =
  // Emblématiques
  | 'plage'
  | 'monument'
  | 'relief_naturel'
  | 'parc_naturel'
  | 'mosquee'
  | 'eglise'
  | 'pont'
  | 'route'
  | 'service_public'
  | 'immeuble_edifice'
  | 'mer_riviere'
  | 'site_naturel'
  // Privés
  | 'hotel'
  | 'plage_privee'
  | 'espace_jeux'
  | 'agriculture_touristique'
  | 'residence_touristique'
  | 'restaurant'
  | 'discotheque'
  | 'bar_maquis'

/** Catégorie d'un savoir pratique */
export type CategorieSavoir =
  | 'langue_argot'
  | 'coutumes'
  | 'etiquette'
  | 'securite'
  | 'sante'
  | 'transports'
  | 'autre'

/** Domaine d'une personnalité connue */
export type DomainePersonnalite =
  | 'politique'
  | 'artiste_musicien'
  | 'artiste_autre'
  | 'sportif'
  | 'entrepreneur'
  | 'scientifique'
  | 'militaire_historique'
  | 'autre'

/** Utilisateur public (auteur d'un contenu) — anonymisable */
export interface UtilisateurPublicAPI {
  id: string | null
  nom: string
  prenom: string
  photo_url: string | null
}

/** Personnalité connue */
export interface PersonnaliteConnueAPI {
  id: string
  fiche_pays_id: string
  nom_complet: string
  domaine: DomainePersonnalite
  biographie_courte: string
  annee_naissance: number | null
  annee_deces: number | null
  portrait_url: string | null
  lien_reference: string | null
  cree_par: string
  created_at: string
}

/** Savoir pratique à connaître avant de voyager */
export interface SavoirPratiqueAPI {
  id: string
  fiche_pays_id: string
  titre: string
  categorie: CategorieSavoir
  explication: string
  exemple: string | null
  cree_par: string
  created_at: string
}

/** Recommandation d'un visiteur (note + commentaire) */
export interface RecommandationVisiteurAPI {
  id: string
  fiche_pays_id: string
  utilisateur: UtilisateurPublicAPI
  note: number
  commentaire: string
  created_at: string
}

/** Photo visiteur (galerie Afripulse) */
export interface PhotoVisiteurAPI {
  id: string
  fiche_pays_id: string
  utilisateur: UtilisateurPublicAPI
  url: string
  legende: string
  largeur_px: number
  hauteur_px: number
  created_at: string
}

/** Site touristique (emblématique ou privé) — enrichi (feature 001-sites-touristiques-enrichis) */
export interface SiteTouristiqueAPI {
  id: string
  fiche_pays_id: string
  nom: string
  categorie: CategorieSiteTouristique
  sous_type: SousTypeSite | null
  description: string | null
  info_pertinente: string | null
  image_url: string | null
  images: string[]
  gestionnaire: string | null
  ville: string | null
  village: string | null
  latitude: number | null
  longitude: number | null
  // Contacts (publics — CL résolue) ; renseignés surtout pour les sites privés
  contact_telephone: string | null
  contact_courriel: string | null
  contact_adresse: string | null
  // Constitution légale (facultatif)
  constitution_statut_juridique: string | null
  constitution_numero: string | null
  constitution_document_url: string | null
  // Lien officiel du site (facultatif, http/https)
  site_web_url: string | null
  // Fiabilité
  verifie: boolean
  // Agrégats avis
  note_moyenne: number | null
  nombre_avis: number
  created_at: string
}

/** Avis d'un visiteur sur un site (note 1–5) */
export interface AvisSiteAPI {
  id: string
  utilisateur: UtilisateurPublicAPI
  note: number
  commentaire: string
  created_at: string
}

/** Liste paginée d'avis d'un site + agrégats */
export interface AvisSiteListe {
  note_moyenne: number | null
  nombre_total: number
  avis: AvisSiteAPI[]
}

/** Secteur d'opportunité (agriculture, mines, etc.) */
export interface SecteurOpportuniteAPI {
  id: string
  fiche_pays_id: string
  nom: string
  description: string | null
  localite: string | null
  contact_telephone: string | null
  contact_courriel: string | null
  contact_adresse: string | null
  references_utiles: string | null
  site_web_url: string | null
  image_url: string | null
  pictogramme: string | null
  created_at: string
}

/** Erreur typée retournée en HTTP 429 lors du rate-limit */
export interface ErreurLimiteAtteinte {
  seuil_depasse: 'textes_jour' | 'photos_jour' | 'attente_par_pays'
  compteur: number
  limite: number
  prochain_creneau: string | null
  message: string
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

/** Sections modifiables d'une fiche pays */
export const SECTIONS_FICHE_PAYS = [
  { value: 'population', label: 'Population' },
  { value: 'superficie_km2', label: 'Superficie' },
  { value: 'biographie', label: 'Biographie' },
  { value: 'contexte', label: 'Contexte general' },
  { value: 'contexte_historique', label: 'Contexte historique' },
  { value: 'slogan', label: 'Slogan' },
  { value: 'hymne_national', label: 'Hymne national' },
  { value: 'langue_officielle', label: 'Langue officielle' },
  { value: 'langues_populaires', label: 'Langues populaires' },
  { value: 'monnaie', label: 'Monnaie' },
  { value: 'fuseau_horaire', label: 'Fuseau horaire' },
  { value: 'voyage_langue_internationale', label: 'Langue internationale' },
  { value: 'voyage_langue_locale', label: 'Langue locale la plus utilisée' },
  { value: 'voyage_infos_visa', label: 'Informations visa' },
  { value: 'voyage_infos_sanitaires', label: 'Informations sanitaires' },
  { value: 'voyage_meteo', label: 'Météo' },
  { value: 'voyage_prises_electriques', label: 'Prises électriques' },
  { value: 'voyage_contacts_tourisme', label: 'Contacts officiels du tourisme' },
  { value: 'voyage_recommandations_securite', label: 'Recommandations sécurité' },
  { value: 'groupe_ethnique', label: 'Groupe ethnique' },
  { value: 'site_touristique', label: 'Site touristique' },
  { value: 'secteur_developpement', label: 'Secteur de developpement' },
] as const

/** Libellés français des sous-types de site (affichage UI) */
export const LIBELLES_SOUS_TYPE: Record<SousTypeSite, string> = {
  // Emblématiques
  plage: 'Plage',
  monument: 'Monument',
  relief_naturel: 'Relief naturel',
  parc_naturel: 'Parc naturel',
  mosquee: 'Mosquée',
  eglise: 'Église',
  pont: 'Pont',
  route: 'Route',
  service_public: 'Service public',
  immeuble_edifice: 'Immeuble / Édifice',
  mer_riviere: 'Mer / Rivière',
  site_naturel: 'Site naturel',
  // Privés
  hotel: 'Hôtel',
  plage_privee: 'Plage privée',
  espace_jeux: 'Espace de jeux',
  agriculture_touristique: 'Agriculture touristique',
  residence_touristique: 'Résidence touristique',
  restaurant: 'Restaurant',
  discotheque: 'Discothèque',
  bar_maquis: 'Bar / Maquis',
}

/** Sous-types autorisés par famille (cohérence famille↔sous-type — FR-003) */
export const SOUS_TYPES_PAR_CATEGORIE: Record<CategorieSiteTouristique, SousTypeSite[]> = {
  emblematique: [
    'plage',
    'monument',
    'relief_naturel',
    'parc_naturel',
    'mosquee',
    'eglise',
    'pont',
    'route',
    'service_public',
    'immeuble_edifice',
    'mer_riviere',
    'site_naturel',
  ],
  prive: [
    'hotel',
    'plage_privee',
    'espace_jeux',
    'agriculture_touristique',
    'residence_touristique',
    'restaurant',
    'discotheque',
    'bar_maquis',
  ],
}

// ──────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ──────────────────────────────────────────────────────────────

// ── Préparation d'image côté client (alignée limites backend) ──────────────

export const IMAGE_TAILLE_MAX = 2 * 1024 * 1024 // 2 Mo
export const IMAGE_DIMENSION_MAX = 2048 // px

const chargerImageElement = (fichier: File): Promise<HTMLImageElement> =>
  new Promise((resolve, reject) => {
    const url = URL.createObjectURL(fichier)
    const img = new Image()
    img.onload = () => { URL.revokeObjectURL(url); resolve(img) }
    img.onerror = (e) => { URL.revokeObjectURL(url); reject(e) }
    img.src = url
  })

/**
 * Redimensionne/recompresse une image (canvas) pour respecter les limites
 * backend (≤ 2048 px par côté, ≤ 2 Mo). Retourne le fichier d'origine s'il est
 * déjà conforme. JPEG : qualité abaissée jusqu'à passer sous 2 Mo.
 */
export const preparerImageContribution = async (fichier: File): Promise<File> => {
  if (fichier.size <= IMAGE_TAILLE_MAX) {
    const dims = await chargerImageElement(fichier).catch(() => null)
    if (dims && dims.width <= IMAGE_DIMENSION_MAX && dims.height <= IMAGE_DIMENSION_MAX) {
      return fichier
    }
  }
  const img = await chargerImageElement(fichier)
  const ratio = Math.min(IMAGE_DIMENSION_MAX / img.width, IMAGE_DIMENSION_MAX / img.height, 1)
  const largeur = Math.max(1, Math.round(img.width * ratio))
  const hauteur = Math.max(1, Math.round(img.height * ratio))

  const canvas = document.createElement('canvas')
  canvas.width = largeur
  canvas.height = hauteur
  const ctx = canvas.getContext('2d')
  if (!ctx) return fichier
  ctx.drawImage(img, 0, 0, largeur, hauteur)

  const estPng = fichier.type === 'image/png'
  const mime = estPng ? 'image/png' : 'image/jpeg'
  const exporter = (q: number): Promise<Blob> =>
    new Promise((res, rej) =>
      canvas.toBlob(b => (b ? res(b) : rej(new Error('toBlob null'))), mime, q),
    )

  let blob = await exporter(estPng ? 1 : 0.9)
  if (!estPng) {
    let q = 0.9
    while (blob.size > IMAGE_TAILLE_MAX && q > 0.4) {
      q -= 0.15
      blob = await exporter(q)
    }
  }
  const ext = estPng ? 'png' : 'jpg'
  const nom = fichier.name.replace(/\.[^.]+$/, '') + '.' + ext
  return new File([blob], nom, { type: mime })
}

/** Formater une date ISO en francais long (ex: "15 janvier 2025") */
export const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

/** Formater une date ISO en francais court (ex: "15 janv. 2025") */
export const formatDateShort = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  })
}

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const useOpportuniteAfrique = () => {
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

  /**
   * Lister les fiches pays avec filtres et pagination
   */
  const listerFiches = async (filtres: FichePaysFiltres = {}): Promise<FichePaysListeAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const params = new URLSearchParams()
      if (filtres.recherche) params.set('recherche', filtres.recherche)
      if (filtres.region) params.set('region', filtres.region)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/fiches-pays${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<FichePaysListeAPI>>(url)

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement des fiches pays')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur listerFiches:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Obtenir une fiche pays par son ID (UUID, code ISO ou nom)
   */
  const obtenirFiche = async (id: string): Promise<FichePaysDetailAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<FichePaysDetailAPI>>(
        `${apiBase}/api/fiches-pays/${encodeURIComponent(id)}`,
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Fiche pays non trouvee')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur obtenirFiche:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Lister les regions disponibles
   */
  const listerRegions = async (): Promise<string[] | null> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/fiches-pays/regions`,
      )

      if (!reponse.success || !reponse.data) {
        return null
      }

      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerRegions:', e)
      return null
    }
  }

  // ── Methodes Contributions ─────────────────────────────────

  /** Soumettre une contribution pour une fiche pays */
  const soumettreContribution = async (
    ficheId: string,
    body: {
      section: string
      type_contribution?: string
      nouvelle_valeur: string
      justification?: string
    },
  ): Promise<ContributionFicheAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const reponse = await $fetch<ApiResponse<ContributionFicheAPI>>(
        `${apiBase}/api/fiches-pays/${ficheId}/contributions`,
        {
          method: 'POST',
          headers: authHeaders(),
          body,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la soumission')
      }
      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      erreur.value = message
      console.error('Erreur soumettreContribution:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Lister les contributions d'une fiche */
  const listerContributions = async (
    ficheId: string,
    filtres: ContributionFiltres = {},
  ): Promise<ContributionListeAPI | null> => {
    try {
      const params = new URLSearchParams()
      if (filtres.etat) params.set('etat', filtres.etat)
      if (filtres.section) params.set('section', filtres.section)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const queryString = params.toString()
      const url = `${apiBase}/api/fiches-pays/${ficheId}/contributions${queryString ? `?${queryString}` : ''}`

      const reponse = await $fetch<ApiResponse<ContributionListeAPI>>(url, {
        headers: authHeaders(),
      })

      if (!reponse.success || !reponse.data) return null
      return reponse.data
    }
    catch (e: any) {
      console.error('Erreur listerContributions:', e)
      return null
    }
  }

  /** Lister les contributeurs valides d'une fiche */
  const listerContributeurs = async (ficheId: string): Promise<ContributeurAPI[]> => {
    try {
      const reponse = await $fetch<ApiResponse<ContributeurAPI[]>>(
        `${apiBase}/api/fiches-pays/${ficheId}/contributeurs`,
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch {
      return []
    }
  }

  // La modération des contributions (valider/rejeter) est réservée à l'admin
  // et passe par useAdminContributions (PATCH /api/admin/profils-pays/contributions/{id}/etat).

  // ── Afripulse — Méthodes de soumission et de lecture enrichies ──────

  /**
   * Soumettre une contribution Afripulse (JSON structurée).
   * Erreurs typées :
   *   • HTTP 401 → utilisateur non connecté
   *   • HTTP 429 → {@link ErreurLimiteAtteinte} rate-limit atteint
   *   • HTTP 404 → fiche pays inexistante
   */
  const soumettreContributionEnrichie = async (
    ficheId: string,
    body: {
      type_objet_contribution: TypeObjetContribution
      section_afripulse?: SectionAfripulse
      type_contribution: 'ajout' | 'edition' | 'suppression'
      target_id?: string
      nouvelle_valeur_jsonb?: unknown
      justification?: string
    },
  ): Promise<ContributionFicheAPI | null> => {
    try {
      const reponse = await $fetch<ApiResponse<ContributionFicheAPI>>(
        `${apiBase}/api/fiches-pays/${encodeURIComponent(ficheId)}/contributions`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: {
            section: body.section_afripulse ?? body.type_objet_contribution,
            type_objet_contribution: body.type_objet_contribution,
            section_afripulse: body.section_afripulse,
            type_contribution: body.type_contribution,
            target_id: body.target_id,
            nouvelle_valeur_jsonb: body.nouvelle_valeur_jsonb,
            justification: body.justification,
          },
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Soumission impossible')
      }
      return reponse.data
    }
    catch (e: any) {
      const status = e?.response?.status ?? e?.statusCode
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      if (status === 429) {
        erreur.value = `Quota dépassé : ${message}`
      } else {
        erreur.value = message
      }
      console.error('Erreur soumettreContributionEnrichie:', e)
      return null
    }
  }

  /** Lister les sites touristiques d'une fiche (filtre categorie optionnel) */
  const listerSitesTouristiques = async (
    ficheId: string,
    categorie?: CategorieSiteTouristique,
  ): Promise<SiteTouristiqueAPI[]> => {
    try {
      const url = categorie
        ? `${apiBase}/api/fiches-pays/${ficheId}/sites-touristiques?categorie=${categorie}`
        : `${apiBase}/api/fiches-pays/${ficheId}/sites-touristiques`
      const reponse = await $fetch<ApiResponse<SiteTouristiqueAPI[]>>(url)
      return reponse.data ?? []
    }
    catch (e) {
      console.error('Erreur listerSitesTouristiques:', e)
      return []
    }
  }

  /** Lister les secteurs d'opportunités d'une fiche */
  const listerSecteursOpportunites = async (
    ficheId: string,
  ): Promise<SecteurOpportuniteAPI[]> => {
    try {
      const reponse = await $fetch<ApiResponse<SecteurOpportuniteAPI[]>>(
        `${apiBase}/api/fiches-pays/${ficheId}/secteurs-opportunites`,
      )
      return reponse.data ?? []
    }
    catch (e) {
      console.error('Erreur listerSecteursOpportunites:', e)
      return []
    }
  }

  /** Lister les personnalités connues d'une fiche (filtre domaine optionnel) */
  const listerPersonnalites = async (
    ficheId: string,
    domaine?: DomainePersonnalite,
  ): Promise<PersonnaliteConnueAPI[]> => {
    try {
      const url = domaine
        ? `${apiBase}/api/fiches-pays/${ficheId}/personnalites?domaine=${domaine}`
        : `${apiBase}/api/fiches-pays/${ficheId}/personnalites`
      const reponse = await $fetch<ApiResponse<PersonnaliteConnueAPI[]>>(url)
      return reponse.data ?? []
    }
    catch (e) {
      console.error('Erreur listerPersonnalites:', e)
      return []
    }
  }

  /** Lister les savoirs pratiques d'une fiche (filtre categorie optionnel) */
  const listerSavoirsPratiques = async (
    ficheId: string,
    categorie?: CategorieSavoir,
  ): Promise<SavoirPratiqueAPI[]> => {
    try {
      const url = categorie
        ? `${apiBase}/api/fiches-pays/${ficheId}/savoirs-pratiques?categorie=${categorie}`
        : `${apiBase}/api/fiches-pays/${ficheId}/savoirs-pratiques`
      const reponse = await $fetch<ApiResponse<SavoirPratiqueAPI[]>>(url)
      return reponse.data ?? []
    }
    catch (e) {
      console.error('Erreur listerSavoirsPratiques:', e)
      return []
    }
  }

  // ── US3 — Création d'une nouvelle fiche pays ──────────────────────
  /**
   * Soumettre une proposition de nouvelle fiche pays. Erreurs typées :
   *   • HTTP 401 → non authentifié
   *   • HTTP 409 → {fiche_pays_id, message} - fiche existante (proposer modification)
   *   • HTTP 422 → code_iso2 hors Afripulse (54 pays africains)
   *   • HTTP 429 → rate-limit
   */
  const creerFichePays = async (
    payload: {
      code_iso2: string
      slogan?: string
      population?: number | null
      superficie_km2?: number | null
      biographie?: string
      contexte?: string
      monnaie?: string
      langue_officielle?: string
      langues_populaires?: string
      hymne_national?: string
      fuseau_horaire?: string
      image_couverture_url?: string
      image_drapeau_url?: string
      image_embleme_url?: string
      justification?: string
    },
  ): Promise<{ id: string, etat: string, created_at: string } | null> => {
    try {
      const reponse = await $fetch<ApiResponse<{ id: string, etat: string, created_at: string }>>(
        `${apiBase}/api/fiches-pays`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: payload,
        },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Création impossible')
      }
      return reponse.data
    }
    catch (e: any) {
      const status = e?.response?.status ?? e?.statusCode
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      if (status === 409) {
        erreur.value = `Cette fiche pays existe déjà : ${message}`
      } else if (status === 422) {
        erreur.value = `Pays hors périmètre Afripulse : ${message}`
      } else if (status === 429) {
        erreur.value = `Quota dépassé : ${message}`
      } else {
        erreur.value = message
      }
      console.error('Erreur creerFichePays:', e)
      return null
    }
  }

  // ── US4 — Recommandations & galerie photos ────────────────────────

  /**
   * Lister les recommandations visiteurs d'une fiche (lecture publique paginée).
   * Ne retourne que les recommandations `active = TRUE AND deleted_at IS NULL`.
   * Auteurs supprimés anonymisés (`utilisateur_id = null`).
   *
   * @param ficheId UUID de la fiche pays
   * @param page numéro de page (>= 1, défaut 1)
   * @param parPage taille de page (1..50, défaut 10)
   * @returns `{ note_moyenne, nombre_total, recommandations }` ou `null` en cas d'erreur
   */
  const listerRecommandations = async (
    ficheId: string,
    page = 1,
    parPage = 10,
  ): Promise<{
    note_moyenne: number | null
    nombre_total: number
    recommandations: Array<{
      id: string
      utilisateur_id: string | null
      auteur_nom: string | null
      auteur_prenom: string | null
      auteur_photo_url: string | null
      note: number
      commentaire: string
      created_at: string
    }>
  } | null> => {
    try {
      const params = new URLSearchParams({ page: String(page), par_page: String(parPage) })
      const reponse = await $fetch<ApiResponse<any>>(
        `${apiBase}/api/fiches-pays/${ficheId}/recommandations?${params}`,
      )
      return reponse.data ?? null
    }
    catch (e) {
      console.error('Erreur listerRecommandations:', e)
      return null
    }
  }

  /**
   * Lister la galerie photos visiteurs d'une fiche (lecture publique paginée).
   * Auteurs supprimés anonymisés (`utilisateur_id = null`).
   *
   * @param ficheId UUID de la fiche pays
   * @param page numéro de page (>= 1, défaut 1)
   * @param parPage taille de page (1..60, défaut 12)
   * @returns `{ nombre_total, photos }` ou `null` en cas d'erreur
   */
  const listerGaleriePhotos = async (
    ficheId: string,
    page = 1,
    parPage = 12,
  ): Promise<{
    nombre_total: number
    photos: Array<{
      id: string
      chemin_fichier: string
      legende: string
      format: string
      largeur_px: number
      hauteur_px: number
      utilisateur_id: string | null
      auteur_nom: string | null
      auteur_prenom: string | null
      created_at: string
    }>
  } | null> => {
    try {
      const params = new URLSearchParams({ page: String(page), par_page: String(parPage) })
      const reponse = await $fetch<ApiResponse<any>>(
        `${apiBase}/api/fiches-pays/${ficheId}/galerie-photos?${params}`,
      )
      return reponse.data ?? null
    }
    catch (e) {
      console.error('Erreur listerGaleriePhotos:', e)
      return null
    }
  }

  /**
   * Soumettre une contribution multipart (photos + légendes).
   * Erreurs typées : 401, 413 (photo trop grande), 429 (rate-limit).
   */
  const soumettreContributionMultipart = async (
    ficheId: string,
    body: {
      section?: SectionAfripulse
      type_objet: 'photo_visiteur'
      type_contribution: 'ajout' | 'edition'
      photos: File[]
      legendes: string[]
      justification?: string
    },
  ): Promise<{ id: string, etat: string, created_at: string, nombre_photos: number } | null> => {
    try {
      const formData = new FormData()
      if (body.section) formData.append('section', body.section)
      formData.append('type_objet', body.type_objet)
      formData.append('type_contribution', body.type_contribution)
      if (body.justification) formData.append('justification', body.justification)
      body.photos.forEach(file => formData.append('photos', file))
      body.legendes.forEach(leg => formData.append('legendes', leg))

      const reponse = await $fetch<ApiResponse<any>>(
        `${apiBase}/api/fiches-pays/${ficheId}/contributions/multipart`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: formData,
        },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Upload impossible')
      return reponse.data
    }
    catch (e: any) {
      const status = e?.response?.status ?? e?.statusCode
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      if (status === 413) erreur.value = `Photo trop volumineuse : ${message}`
      else if (status === 429) erreur.value = `Quota dépassé : ${message}`
      else erreur.value = message
      console.error('Erreur soumettreContributionMultipart:', e)
      return null
    }
  }

  /**
   * Uploader une image isolée pour une contribution (site, personnalité).
   * Retourne l'URL relative (`/uploads/...`) à placer dans le payload, ou null.
   */
  const uploaderImageContribution = async (fichier: File): Promise<string | null> => {
    try {
      const formData = new FormData()
      formData.append('image', fichier)
      const reponse = await $fetch<ApiResponse<{ url: string }>>(
        `${apiBase}/api/fiches-pays/contributions/upload-image`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: formData,
        },
      )
      if (!reponse.success || !reponse.data?.url) {
        throw new Error(reponse.error || 'Upload impossible')
      }
      return reponse.data.url
    }
    catch (e: any) {
      const status = e?.response?.status ?? e?.statusCode
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      if (status === 413) erreur.value = `Image trop volumineuse : ${message}`
      else erreur.value = message
      console.error('Erreur uploaderImageContribution:', e)
      return null
    }
  }

  // ── US5 — Avis de visiteurs sur un site ──────────────────────────

  /**
   * Lister les avis visibles d'un site (paginé) + agrégats (note moyenne, total).
   * Lecture publique ; exclut les avis masqués/supprimés.
   */
  const listerAvisSite = async (
    siteId: string,
    page = 1,
    parPage = 10,
  ): Promise<AvisSiteListe | null> => {
    try {
      const params = new URLSearchParams({ page: String(page), par_page: String(parPage) })
      const reponse = await $fetch<ApiResponse<AvisSiteListe>>(
        `${apiBase}/api/sites-touristiques/${encodeURIComponent(siteId)}/avis?${params}`,
      )
      return reponse.data ?? null
    }
    catch (e) {
      console.error('Erreur listerAvisSite:', e)
      return null
    }
  }

  /**
   * Déposer ou mettre à jour son avis sur un site (upsert). Auth requise.
   * Erreurs typées : 401 (non connecté), 404 (site absent), 422 (validation).
   */
  const soumettreAvisSite = async (
    siteId: string,
    note: number,
    commentaire: string,
  ): Promise<{ id: string, note: number, commentaire: string, created_at: string } | null> => {
    try {
      const reponse = await $fetch<ApiResponse<{ id: string, note: number, commentaire: string, created_at: string }>>(
        `${apiBase}/api/sites-touristiques/${encodeURIComponent(siteId)}/avis`,
        {
          method: 'POST',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: { note, commentaire },
        },
      )
      if (!reponse.success || !reponse.data) throw new Error(reponse.error || 'Soumission impossible')
      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur soumettreAvisSite:', e)
      return null
    }
  }

  /** Résout une URL d'image stockée (`/uploads/...` → préfixée par l'API ; http(s) inchangé). */
  const resoudreUrlImage = (url: string | null | undefined): string => {
    if (!url) return ''
    if (url.startsWith('http://') || url.startsWith('https://')) return url
    return `${apiBase}${url}`
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerFiches,
    obtenirFiche,
    listerRegions,
    // Contributions
    soumettreContribution,
    listerContributions,
    listerContributeurs,
    // Afripulse (sections enrichies)
    soumettreContributionEnrichie,
    uploaderImageContribution,
    resoudreUrlImage,
    listerSitesTouristiques,
    listerSecteursOpportunites,
    listerPersonnalites,
    listerSavoirsPratiques,
    // US3 / US4
    creerFichePays,
    listerRecommandations,
    listerGaleriePhotos,
    soumettreContributionMultipart,
    // US5 — avis de site
    listerAvisSite,
    soumettreAvisSite,
  }
}

// Soumission de médias par les parties prenantes et suivi de ses propositions
// (US4 — migration 09l, handlers/media_proposition.rs).
//
// **Trois garde-fous sont côté serveur, pas ici** : `statut` naît toujours
// `'en_attente'`, `origine_publication` est forcée à `'territoire'`, et
// « Autre » sans précision est refusé. Ce composable ne fait que présenter ces
// règles à l'utilisateur ; il ne les applique pas.

export type TypeObjetPropose =
  | 'chaine_tv'
  | 'station_radio'
  | 'programme_tele'
  | 'programme_radio'
  | 'animation_programme'
  | 'idee_contenu'

export type StatutProposition = 'en_attente' | 'validee' | 'rejetee' | 'retiree'

/** Les neuf rôles de partie prenante déclarables (FR-029). */
export const ROLES_PARTIE_PRENANTE: { valeur: string, libelle: string }[] = [
  { valeur: 'chaine_tele', libelle: 'Chaîne de télévision' },
  { valeur: 'radio', libelle: 'Radio' },
  { valeur: 'journaliste', libelle: 'Journaliste' },
  { valeur: 'communicateur', libelle: 'Communicateur' },
  { valeur: 'createur_contenu', libelle: 'Créateur de contenu' },
  { valeur: 'influenceur', libelle: 'Influenceur' },
  { valeur: 'realisateur', libelle: 'Réalisateur' },
  { valeur: 'producteur', libelle: 'Producteur' },
  { valeur: 'autre', libelle: 'Autre' },
]

export const LIBELLES_TYPE_OBJET: Record<TypeObjetPropose, string> = {
  chaine_tv: 'Chaîne de télévision',
  station_radio: 'Station de radio',
  programme_tele: 'Émission de télévision',
  programme_radio: 'Émission de radio',
  animation_programme: 'Demande d’animation',
  idee_contenu: 'Idée de contenu',
}

export const LIBELLES_STATUT: Record<StatutProposition, string> = {
  en_attente: 'En attente de validation',
  validee: 'Validée et publiée',
  rejetee: 'Refusée',
  retiree: 'Retirée',
}

/** Un thème phare proposable — catégorie média (FR-030). */
export interface ThemePhareAPI {
  id: string
  nom: string
}

/** Un territoire du référentiel partagé. L'UI dit « territoire », la BD « pays ». */
export interface TerritoireAPI {
  id: string
  nom: string
}

/** Payload métier de la proposition — miroir de `DonneesProposition` (Rust). */
export interface DonneesProposition {
  nom?: string
  description?: string
  langue?: string
  pays?: string
  role_partie_prenante?: string
  role_partie_prenante_autre?: string
  theme_phare_id?: string
  theme_phare_autre?: string
  chaine_id?: string
  station_id?: string
  video_url?: string
  audio_url?: string
  stream_url?: string
  image_couverture_url?: string
  info_animateur?: string
  info_producteur?: string
  /** Présentés en évidence à l'administrateur, seul juge de la licéité (H-012). */
  source_declaree?: string
  auteur_declare?: string
}

export interface PropositionMediaAPI {
  id: string
  auteur_id: string
  auteur_nom: string | null
  auteur_prenom: string | null
  auteur_email: string | null
  type_objet: TypeObjetPropose
  target_id: string | null
  donnees: DonneesProposition
  pieces_jointes: string[]
  justification: string
  statut: StatutProposition
  decideur: string | null
  decideur_nom: string | null
  decideur_prenom: string | null
  decide_at: string | null
  commentaire_decision: string | null
  objet_id_cree: string | null
  created_at: string
  updated_at: string
}

export interface PropositionMediaListeAPI {
  propositions: PropositionMediaAPI[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export interface SoumissionProposition {
  type_objet: TypeObjetPropose
  target_id?: string | null
  justification: string
  donnees: DonneesProposition
  /** Fichier média (vidéo ou audio) téléversé, ou lien externe dans `donnees`. */
  media?: File | null
  image?: File | null
}

export const useMediaProposition = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) return { Authorization: `Bearer ${userStore.accessToken}` }
    return {}
  }

  /**
   * Soumet une proposition. Multipart : les champs texte et les fichiers
   * voyagent ensemble, `donnees` étant sérialisé en JSON dans un champ unique.
   */
  const soumettre = async (soumission: SoumissionProposition): Promise<PropositionMediaAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const formData = new FormData()
      formData.append('type_objet', soumission.type_objet)
      formData.append('justification', soumission.justification)
      if (soumission.target_id) formData.append('target_id', soumission.target_id)
      formData.append('donnees', JSON.stringify(soumission.donnees))
      if (soumission.media) formData.append('media', soumission.media)
      if (soumission.image) formData.append('image', soumission.image)

      const reponse = await $fetch<ApiResponse<PropositionMediaAPI>>(
        `${apiBase}/api/medias/propositions`,
        { method: 'POST', headers: authHeaders(), body: formData },
      )
      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l’envoi de la proposition')
      }
      return reponse.data
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      console.error('Erreur soumettre:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Suivi de ses propres soumissions, avec le motif d'un éventuel refus (FR-034). */
  const mesPropositions = async (filtres: {
    statut?: StatutProposition
    type_objet?: TypeObjetPropose
    page?: number
    par_page?: number
  } = {}): Promise<PropositionMediaListeAPI | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const params = new URLSearchParams()
      if (filtres.statut) params.set('statut', filtres.statut)
      if (filtres.type_objet) params.set('type_objet', filtres.type_objet)
      if (filtres.page) params.set('page', String(filtres.page))
      if (filtres.par_page) params.set('par_page', String(filtres.par_page))

      const qs = params.toString()
      const reponse = await $fetch<ApiResponse<PropositionMediaListeAPI>>(
        `${apiBase}/api/medias/propositions/moi${qs ? `?${qs}` : ''}`,
        { headers: authHeaders() },
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      console.error('Erreur mesPropositions:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /** Retrait par l'auteur — refusé dès que la proposition a été tranchée. */
  const retirer = async (id: string): Promise<PropositionMediaAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<PropositionMediaAPI>>(
        `${apiBase}/api/medias/propositions/${id}/retirer`,
        { method: 'PATCH', headers: authHeaders() },
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  /** Métadonnées d'un contenu : publiées immédiatement, `etat` inchangé. */
  const modifierMetadonnees = async (
    typeMedia: string,
    id: string,
    champs: Record<string, unknown>,
  ): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/medias/contenus/${typeMedia}/${id}/metadonnees`,
        {
          method: 'PATCH',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: champs,
        },
      )
      return reponse.success
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  /**
   * Remplacement du média : le contenu bascule en attente et CESSE d'être
   * diffusé jusqu'à revalidation (FR-032). À signaler clairement à l'appelant.
   */
  const remplacerMedia = async (
    typeMedia: string,
    id: string,
    mediaUrl: string,
    justification?: string,
  ): Promise<{ etat: string, proposition_id: string } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ etat: string, proposition_id: string }>>(
        `${apiBase}/api/medias/contenus/${typeMedia}/${id}/media`,
        {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { media_url: mediaUrl, justification },
        },
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
  }

  /**
   * Référentiel des thèmes phares proposables pour un contenu (FR-030).
   *
   * Public et stable : les catégories média seedées par la migration 09j, dans
   * l'ordre éditorial voulu. Alimente le sélecteur du formulaire.
   */
  const listerThemes = async (): Promise<ThemePhareAPI[]> => {
    try {
      const reponse = await $fetch<ApiResponse<ThemePhareAPI[]>>(
        `${apiBase}/api/medias/themes`,
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch (e: any) {
      console.error('Erreur listerThemes:', e)
      return []
    }
  }

  /**
   * Référentiel des territoires (`shared.pays`), pour le champ « Territoire »
   * du formulaire. La validation admin résout `pays_id` par NOM : le formulaire
   * envoie donc le nom, jamais l'identifiant.
   */
  const listerTerritoires = async (): Promise<TerritoireAPI[]> => {
    try {
      const reponse = await $fetch<ApiResponse<TerritoireAPI[]>>(`${apiBase}/api/pays`)
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch (e: any) {
      console.error('Erreur listerTerritoires:', e)
      return []
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    soumettre,
    mesPropositions,
    retirer,
    modifierMetadonnees,
    remplacerMedia,
    listerThemes,
    listerTerritoires,
  }
}

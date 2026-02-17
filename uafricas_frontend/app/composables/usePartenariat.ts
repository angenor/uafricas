// Composable pour les appels API Partenariats (devenir partenaire)
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types et interfaces
// ──────────────────────────────────────────────────────────────

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** DTO pour la reponse apres creation d'un partenariat */
export interface PartenariatAPI {
  id: string
  organisation: {
    id: string
    denomination: string
    slug: string | null
    type_organisation: string | null
    pays: string | null
    ville: string | null
    email: string | null
    telephone: string | null
    site_web: string | null
    logo_url: string | null
    description: string | null
  }
  type_partenariat: string | null
  description: string | null
  date_debut: string | null
  actif: boolean
  created_at: string
}

/** Corps de la demande de partenariat */
export interface DemandePartenariatBody {
  // Partenariat
  type_partenariat: string
  description_partenariat: string

  // Organisation
  denomination: string
  type_organisation: string
  description_organisation?: string
  numero_registre?: string

  // Localisation
  pays?: string
  ville?: string
  adresse?: string

  // Contact
  email?: string
  telephone?: string
  site_web?: string
}

// ──────────────────────────────────────────────────────────────
// Constantes
// ──────────────────────────────────────────────────────────────

export const TYPES_PARTENARIAT: { value: string; label: string; description: string }[] = [
  { value: 'Sponsor', label: 'Sponsor', description: 'Soutien financier aux programmes et activités de la plateforme' },
  { value: 'Contributeur', label: 'Contributeur', description: 'Contribution en ressources, compétences ou services' },
  { value: 'Associé', label: 'Associé', description: 'Collaboration stratégique sur des projets communs' },
]

export const TYPES_ORGANISATION: { value: string; label: string }[] = [
  { value: 'ONG', label: 'ONG' },
  { value: 'Entreprise', label: 'Entreprise' },
  { value: 'Association', label: 'Association' },
  { value: 'Coopérative', label: 'Coopérative' },
]

export const PAYS_PARTENARIAT: { value: string; label: string }[] = [
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

// ──────────────────────────────────────────────────────────────
// Composable
// ──────────────────────────────────────────────────────────────

export const usePartenariat = () => {
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
   * Soumettre une demande de partenariat (multipart pour logo + document legal)
   */
  const soumettreDemande = async (
    body: DemandePartenariatBody,
    logoFile: File | null,
    documentLegalFile: File | null,
  ): Promise<PartenariatAPI | null> => {
    chargement.value = true
    erreur.value = null

    try {
      const data = new FormData()

      // Partenariat
      data.append('type_partenariat', body.type_partenariat)
      data.append('description_partenariat', body.description_partenariat)

      // Organisation
      data.append('denomination', body.denomination)
      data.append('type_organisation', body.type_organisation)
      if (body.description_organisation) data.append('description_organisation', body.description_organisation)
      if (body.numero_registre) data.append('numero_registre', body.numero_registre)

      // Localisation
      if (body.pays) data.append('pays', body.pays)
      if (body.ville) data.append('ville', body.ville)
      if (body.adresse) data.append('adresse', body.adresse)

      // Contact
      if (body.email) data.append('email', body.email)
      if (body.telephone) data.append('telephone', body.telephone)
      if (body.site_web) data.append('site_web', body.site_web)

      // Fichiers
      if (logoFile) data.append('logo', logoFile)
      if (documentLegalFile) data.append('document_legal', documentLegalFile)

      const reponse = await $fetch<ApiResponse<PartenariatAPI>>(
        `${apiBase}/api/partenariats`,
        {
          method: 'POST',
          headers: authHeaders(),
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la soumission de la demande')
      }

      return reponse.data
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur réseau'
      erreur.value = message
      console.error('Erreur soumettreDemande:', e)
      return null
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    soumettreDemande,
  }
}

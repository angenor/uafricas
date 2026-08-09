// Cadeaux virtuels — catalogue, envoi, confirmation, cagnotte (feature 008).
//
// **Le formatage monétaire est exclusivement frontal.** Aucune API ne renvoie de
// montant formaté : les montants circulent en unité entière de la devise
// (le franc CFA n'a pas de centimes), et `formaterMontant` est l'unique endroit
// où ils deviennent lisibles. Deux représentations d'un même montant finiraient
// immanquablement par diverger.
import { useUserStore } from '~/stores/user'

/** Miroir de l'enum `engagement.mode_cadeau`. */
export type ModeCadeau = 'soutien_financier' | 'points'

/** Miroir de l'enum `engagement.etat_paiement`. */
export type EtatPaiement = 'en_attente' | 'abouti' | 'echoue' | 'expire' | 'purge'

export interface Cadeau {
  id: string
  code: string
  libelle: string
  description: string | null
  icone: string | null
  couleur: string | null
  /** Unité entière de la devise. Jamais formaté côté serveur. */
  prix: number
  points: number
  ordre: number
}

export interface Catalogue {
  devise: string
  taux_commission: number
  /** Pilote le bandeau « phase de test » : vaut `NOT paiement_reel_actif`. */
  paiement_simule: boolean
  cadeaux: Cadeau[]
}

export interface MembreBref {
  id: string
  nom_affiche: string
}

export interface CadeauBref {
  code: string
  libelle: string
  icone: string | null
  couleur: string | null
}

export interface IntentionPaiement {
  transaction_id: string
  reference_paiement: string
  etat: EtatPaiement
  montant: number
  points: number
  part_beneficiaire: number
  part_plateforme: number
  beneficiaire: MembreBref
  simule: boolean
  expire_at: string
}

export interface ConfirmationPaiement {
  transaction_id: string
  etat: EtatPaiement
  /** Peut valoir 0 si la règle `cadeau_recu` a été désactivée en back-office. */
  points_credites: number
  beneficiaire: MembreBref
}

export interface ResumeCadeau {
  code: string
  libelle: string
  icone: string | null
  couleur: string | null
  nombre: number
}

export interface CadeauOffert {
  offreur: MembreBref
  cadeau: CadeauBref
  message: string | null
  created_at: string
}

/** Cadeaux reçus par un contenu. **Aucun montant en argent** n'y figure. */
export interface CadeauxContenu {
  total: number
  resume: ResumeCadeau[]
  derniers: CadeauOffert[]
}

export interface MonCadeau {
  id: string
  cadeau: CadeauBref
  contrepartie: MembreBref
  type_objet: string
  objet_id: string
  titre_cible: string | null
  points: number
  mode: ModeCadeau
  /** Renseigné uniquement sur le sens « offerts ». */
  montant: number | null
  message: string | null
  simule: boolean
  created_at: string
}

export interface MesCadeauxPage {
  elements: MonCadeau[]
  total: number
  page: number
  taille: number
}

export interface Cagnotte {
  montant_cumule: number
  montant_verse: number
  devise: string
  /** Toujours `false` dans cette itération : aucun versement n'est disponible. */
  versement_disponible: boolean
  /** Fraction issue de transactions simulées — ce que la purge retirera. */
  part_simulee: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/**
 * Familles sur lesquelles un cadeau peut être offert.
 *
 * `site_touristique` et `secteur_developpement` en sont **absents** : ces
 * contenus éditoriaux rattachés à une fiche pays n'ont aucun auteur enregistré,
 * donc aucun bénéficiaire possible. Les proposer n'aboutirait qu'à un refus.
 */
export const FAMILLES_CADEAU = [
  'codimoi', 'factcheck', 'biblio_humaine', 'video', 'fiche_pays',
  'chaine_tv', 'station_radio',
  'emission_tele', 'emission_radio', 'episode_tele', 'episode_radio',
  'personnalite_connue', 'recette_culinaire', 'profil',
] as const

export type FamilleCadeau = typeof FAMILLES_CADEAU[number]

/**
 * Formate un montant entier dans sa devise. **Unique point de formatage
 * monétaire de l'application** — c'est volontaire.
 */
export const formaterMontant = (montant: number, devise = 'XOF'): string => {
  const nombre = new Intl.NumberFormat('fr-FR').format(montant)
  // `XOF` s'affiche « FCFA » en Afrique de l'Ouest ; `Intl` le rendrait
  // « F CFA » ou « XOF » selon le moteur, ce qui varierait d'un navigateur à
  // l'autre pour la devise la plus courante de la plateforme.
  return devise === 'XOF' ? `${nombre} FCFA` : `${nombre} ${devise}`
}

export const useCadeaux = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const authHeaders = (): Record<string, string> =>
    userStore.accessToken ? { Authorization: `Bearer ${userStore.accessToken}` } : {}

  /** Catalogue actif + contexte de monétisation. Public. */
  const obtenirCatalogue = async (): Promise<Catalogue | null> => {
    const res = await $fetch<ApiResponse<Catalogue>>(`${apiBase}/api/engagement/cadeaux`)
    return res.data
  }

  /**
   * Crée l'intention de paiement. Le serveur résout seul le bénéficiaire, le
   * prix, les points et le taux : on ne lui transmet que le cadeau et la cible.
   */
  const envoyerCadeau = async (
    cadeauId: string,
    mode: ModeCadeau,
    typeObjet: FamilleCadeau | string,
    objetId: string,
    message?: string,
  ): Promise<IntentionPaiement | null> => {
    const res = await $fetch<ApiResponse<IntentionPaiement>>(
      `${apiBase}/api/engagement/cadeaux/envoyer`,
      {
        method: 'POST',
        headers: authHeaders(),
        body: {
          cadeau_id: cadeauId,
          mode,
          cible: { type_objet: typeObjet, objet_id: objetId },
          message: message?.trim() || null,
        },
      },
    )
    return res.data
  }

  /**
   * Rapporte l'issue du paiement. Rejouer une confirmation déjà aboutie renvoie
   * la même réponse sans créditer un point de plus.
   */
  const confirmerPaiement = async (
    reference: string,
    aboutir: boolean,
  ): Promise<ConfirmationPaiement | null> => {
    const res = await $fetch<ApiResponse<ConfirmationPaiement>>(
      `${apiBase}/api/engagement/paiements/${encodeURIComponent(reference)}/confirmer`,
      { method: 'POST', headers: authHeaders(), body: { aboutir } },
    )
    return res.data
  }

  /** Cadeaux reçus par un contenu. Public, sans aucun montant. */
  const obtenirCadeauxContenu = async (
    typeObjet: string,
    objetId: string,
  ): Promise<CadeauxContenu | null> => {
    const res = await $fetch<ApiResponse<CadeauxContenu>>(
      `${apiBase}/api/engagement/cadeaux/${typeObjet}/${objetId}`,
    )
    return res.data
  }

  const listerMesCadeaux = async (
    sens: 'recus' | 'offerts' = 'recus',
    page = 1,
    taille = 20,
  ): Promise<MesCadeauxPage | null> => {
    const res = await $fetch<ApiResponse<MesCadeauxPage>>(
      `${apiBase}/api/engagement/mes-cadeaux`,
      { headers: authHeaders(), query: { sens, page, taille } },
    )
    return res.data
  }

  const obtenirMaCagnotte = async (): Promise<Cagnotte | null> => {
    const res = await $fetch<ApiResponse<Cagnotte>>(
      `${apiBase}/api/engagement/ma-cagnotte`,
      { headers: authHeaders() },
    )
    return res.data
  }

  return {
    obtenirCatalogue,
    envoyerCadeau,
    confirmerPaiement,
    obtenirCadeauxContenu,
    listerMesCadeaux,
    obtenirMaCagnotte,
    formaterMontant,
  }
}

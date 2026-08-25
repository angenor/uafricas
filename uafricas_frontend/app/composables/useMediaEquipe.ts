/**
 * Équipes éditoriales des supports et des programmes médias, feature 010.
 *
 * Le **public** ne passe pas par ces routes : l'équipe voyage dans les payloads
 * de chaîne, de station et de programme, où elle est greffée côté serveur. Ce
 * composable sert l'ÉDITION : repeupler un formulaire, l'enregistrer, et
 * alimenter les suggestions du champ « fonction ».
 *
 * Une prop `base` choisit le préfixe d'URL, à l'image du couple
 * `useMediaEmissions` / `useAdminMediaEmissions` : les deux chemins servent le
 * même contrat, seule l'autorité diffère.
 */

export type TypePorteurEquipe =
  | 'chaine_tv'
  | 'station_radio'
  | 'emission_tele'
  | 'emission_radio'

export type BaseEquipe = 'membre' | 'admin'

/** DTO `MembreEquipeResponse` du backend. */
export interface MembreEquipeAPI {
  id: string
  nom: string
  prenom?: string
  fonction: string
  territoire?: string
  contact?: string
  /** Absent = fiche non rattachée, ou compte fermé → nom en texte simple. */
  utilisateur_id?: string
  ordre: number
}

/**
 * Ce que le formulaire manipule : pas d'`id` (le `PUT` remplace tout), pas
 * d'`ordre` (c'est l'index dans le tableau).
 */
export interface MembreEquipeForm {
  nom: string
  prenom: string
  fonction: string
  territoire: string
  contact: string
  utilisateur_id: string | null
  /** Nom affiché du compte rattaché : confort d'écran, jamais envoyé. */
  compte_libelle?: string
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Discriminant d'équipe d'un programme, déduit du type de son support. */
export const porteurProgramme = (
  typeSupport: 'chaine_tv' | 'station_radio',
): TypePorteurEquipe =>
  typeSupport === 'station_radio' ? 'emission_radio' : 'emission_tele'

export const membreVideEquipe = (): MembreEquipeForm => ({
  nom: '',
  prenom: '',
  fonction: '',
  territoire: '',
  contact: '',
  utilisateur_id: null,
})

/** Convertit une fiche servie par l'API en ligne de formulaire. */
export const versFormulaireEquipe = (membre: MembreEquipeAPI): MembreEquipeForm => ({
  nom: membre.nom,
  prenom: membre.prenom ?? '',
  fonction: membre.fonction,
  territoire: membre.territoire ?? '',
  contact: membre.contact ?? '',
  utilisateur_id: membre.utilisateur_id ?? null,
})

/** Nom affichable d'un membre : le prénom est facultatif (FR-012). */
export const nomCompletMembre = (
  membre: Pick<MembreEquipeAPI, 'nom' | 'prenom'>,
): string => [membre.prenom, membre.nom].filter(Boolean).join(' ').trim()

export const useMediaEquipe = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /**
   * Le jeton d'accès vit **en mémoire, dans le store**, seul le jeton de
   * rafraîchissement est en `localStorage`. Le lire depuis `localStorage`
   * renvoyait donc toujours `null`, et tout `PUT` repartait en 401 : c'est le
   * patron de `useMediaDetention`, pas celui de `useMediaEmissions`, qu'il faut
   * suivre ici.
   */
  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) return { Authorization: `Bearer ${userStore.accessToken}` }
    return {}
  }

  const prefixe = (base: BaseEquipe): string =>
    base === 'admin' ? '/api/admin/medias' : '/api/medias'

  /** Lecture de travail : repeuple le formulaire d'édition. */
  const obtenirEquipe = async (
    typePorteur: TypePorteurEquipe,
    porteurId: string,
    base: BaseEquipe = 'membre',
  ): Promise<MembreEquipeAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ membres: MembreEquipeAPI[] }>>(
        `${apiBase}${prefixe(base)}/${typePorteur}/${porteurId}/equipe`,
        { headers: authHeaders() },
      )
      return reponse.success ? (reponse.data?.membres ?? []) : []
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Remplacement intégral et ordonné : l'ordre du tableau fait foi, une liste
   * vide supprime toute l'équipe. Les champs laissés blancs partent en `null` 
   * une chaîne vide produirait un libellé creux à l'affichage.
   */
  const definirEquipe = async (
    typePorteur: TypePorteurEquipe,
    porteurId: string,
    membres: MembreEquipeForm[],
    base: BaseEquipe = 'membre',
  ): Promise<MembreEquipeAPI[] | null> => {
    chargement.value = true
    erreur.value = null
    try {
      const nettoyer = (valeur: string): string | null => {
        const v = valeur.trim()
        return v === '' ? null : v
      }
      const reponse = await $fetch<ApiResponse<{ membres: MembreEquipeAPI[] }>>(
        `${apiBase}${prefixe(base)}/${typePorteur}/${porteurId}/equipe`,
        {
          method: 'PUT',
          headers: { ...authHeaders(), 'Content-Type': 'application/json' },
          body: {
            membres: membres.map(m => ({
              nom: m.nom.trim(),
              prenom: nettoyer(m.prenom),
              fonction: m.fonction.trim(),
              territoire: nettoyer(m.territoire),
              contact: nettoyer(m.contact),
              utilisateur_id: m.utilisateur_id,
            })),
          },
        },
      )
      return reponse.success ? (reponse.data?.membres ?? []) : null
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return null
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * Suggestions du champ « fonction » (FR-015). Route **unique et globale** :
   * une fonction déclarée sur une chaîne doit être proposée sur un programme,
   * sinon le référentiel ne se constituerait jamais. Le back-office consomme la
   * même : dupliquer la liste sous `/api/admin` la ferait diverger.
   */
  const listerFonctions = async (): Promise<string[]> => {
    try {
      const reponse = await $fetch<ApiResponse<string[]>>(
        `${apiBase}/api/medias/equipe/fonctions`,
      )
      return reponse.success ? (reponse.data ?? []) : []
    }
    catch {
      // Une suggestion absente n'empêche pas la saisie : le champ reste libre.
      return []
    }
  }

  return {
    chargement,
    erreur,
    obtenirEquipe,
    definirEquipe,
    listerFonctions,
  }
}

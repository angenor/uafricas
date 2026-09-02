/**
 * Africanité : publications éphémères du fil (spec 012, périmètre P1).
 *
 * Le serveur renvoie les africanités DÉJÀ GROUPÉES par auteur et déjà
 * ordonnées : le lecteur d'abord, puis ceux qui ont du nouveau. Regrouper ou
 * retrier côté client reproduirait une règle qui vit déjà dans la requête, et
 * les deux finiraient par diverger.
 */

export type FormeAfricanite = 'image' | 'video' | 'texte'

export interface AfricaniteAPI {
  id: string
  forme: FormeAfricanite
  media_url: string | null
  texte: string | null
  couleur_fond: string | null
  legende: string | null
  expire_at: string
  created_at: string
  /** Le lecteur courant l'a-t-il déjà regardée ? */
  vue: boolean
  /** Absent quand le lecteur n'est pas l'auteur, nul ne voit les lecteurs
   *  d'une africanité qui n'est pas la sienne. */
  nombre_vues?: number
}

export interface AuteurAfricanitesAPI {
  auteur_id: string
  nom: string
  prenom: string
  photo_url: string | null
  est_moi: boolean
  /** Pilote l'anneau de la pastille : il reste quelque chose à voir. */
  a_du_nouveau: boolean
  africanites: AfricaniteAPI[]
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export const COULEURS_AFRICANITE = [
  '#A74916', '#1C8C1C', '#1E3A5F', '#6B2C5B', '#2F4F4F', '#800020']

export const TEXTE_MAX_AFRICANITE = 280

export function useAfricanite() {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const authHeaders = (): Record<string, string> =>
    userStore.accessToken ? { Authorization: `Bearer ${userStore.accessToken}` } : {}

  /** Chemin relatif servi par le backend → URL absolue. */
  const resoudreMedia = (url: string | null): string | null => {
    if (!url) return null
    return url.startsWith('http') ? url : `${apiBase}${url}`
  }

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  /**
   * Liste les africanités visibles. Renvoie un tableau VIDE et non `null` en
   * cas d'échec : la rangée doit se rendre quoi qu'il arrive, son cercle « + »
   * étant une entrée en soi.
   */
  const listerAfricanites = async (): Promise<AuteurAfricanitesAPI[]> => {
    if (!userStore.isAuthenticated) return []
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<AuteurAfricanitesAPI[]>>(
        `${apiBase}/api/africanites`,
        { headers: authHeaders() })
      return reponse.data ?? []
    }
    catch (e: unknown) {
      erreur.value = e instanceof Error ? e.message : 'Erreur réseau'
      console.error('Erreur listerAfricanites:', e)
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /** Publie la forme « texte sur fond coloré », aucun fichier déposé. */
  const publierTexte = async (
    texte: string,
    couleurFond: string,
    legende?: string): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}/api/africanites/texte`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: { texte, couleur_fond: couleurFond, legende: legende || undefined },
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Publication impossible'
      return false
    }
  }

  /** Publie une image ou une vidéo courte. Le fichier part en multipart : un
   *  fichier ne se transporte pas en JSON. */
  const publierMedia = async (
    fichier: File,
    forme: 'image' | 'video',
    legende?: string): Promise<boolean> => {
    erreur.value = null
    const donnees = new FormData()
    donnees.append('media', fichier)
    donnees.append('forme', forme)
    if (legende) donnees.append('legende', legende)
    try {
      await $fetch(`${apiBase}/api/africanites/media`, {
        method: 'POST',
        // Pas de `Content-Type` : le navigateur doit poser lui-même la
        // frontière multipart, et l'imposer à la main la casserait.
        headers: authHeaders(),
        body: donnees,
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Publication impossible'
      return false
    }
  }

  /** Marque comme vue. Best-effort : l'échec ne doit pas interrompre la
   *  lecture en cours, l'anneau se corrigera au rechargement suivant. */
  const marquerVue = async (africaniteId: string): Promise<void> => {
    try {
      await $fetch(`${apiBase}/api/africanites/${africaniteId}/vue`, {
        method: 'POST',
        headers: authHeaders(),
      })
    }
    catch (e: unknown) {
      console.error('Erreur marquerVue:', e)
    }
  }

  return {
    chargement,
    erreur,
    resoudreMedia,
    listerAfricanites,
    publierTexte,
    publierMedia,
    marquerVue,
  }
}

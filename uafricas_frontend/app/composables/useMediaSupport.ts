/**
 * Fiche d'un support média : thématiques multiples et couverture territoriale
 * (feature 009, US3 et US4).
 *
 * Terminologie : l'interface dit « **territoire** » là où l'API et la base
 * disent `pays` : convention établie du projet.
 */

export interface ThematiquePublique {
  id: string
  nom: string
  /** `true` : ligne éditoriale d'Africans Télé International (09u) ; `false` :
   * genre de grille (09s). Les deux arrivent dans la même liste — c'est ce
   * drapeau qui permet de les présenter séparément. */
  est_ligne_editoriale?: boolean
}

export interface TerritoirePublic {
  id: string
  nom: string
}

export interface CouverturePublique {
  /** `true` : le support couvre tout le continent ; la liste est alors vide. */
  couverture_continentale: boolean
  territoires: TerritoirePublic[]
}

export interface ThematiqueDecompte {
  id: string
  nom: string
  /** Texte long, affiché en infobulle native au survol (`title`). */
  description?: string | null
  nombre_supports: number
}

export interface TerritoireDecompte {
  id: string
  nom: string
  nombre_supports: number
}

export interface TerritoiresDisponibles {
  territoires: TerritoireDecompte[]
  /** Supports panafricains : ils remontent sur **chaque** territoire (FR-036),
   * et ne peuvent donc pas être comptés dans les lignes ci-dessus. */
  continentales: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export type TypeSupport = 'chaine_tv' | 'station_radio'

export const useMediaSupport = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const erreur = ref<string | null>(null)

  const authHeaders = (): Record<string, string> => {
    if (import.meta.client) {
      const token = localStorage.getItem('accessToken')
      if (token) return { Authorization: `Bearer ${token}` }
    }
    return {}
  }

  const prefixePublic = (type: TypeSupport): string =>
    type === 'station_radio' ? '/api/stations-radio' : '/api/television'

  /** Tous les thèmes actifs du référentiel `media`, avec le nombre de supports
   * publiés qui les déclarent : `0` compris.
   *
   * `origine` ('africans' | 'territoire') borne ce décompte, sans retirer aucun
   * thème du catalogue : sans lui, un thème compterait des supports des DEUX
   * origines, faussant le panneau de la pastille « Africans Télé International »
   * (et son pendant radio, l'origine étant fixée par la page appelante).
   *
   * `groupe` bascule le référentiel lui-même (09u) : omis, les 22 genres de
   * grille génériques (parent_id NULL, panneau « Africans Thématique ») ;
   * `'media-groupe-africans-tele-international'`, les 44 lignes éditoriales
   * propres à la pastille du même nom. */
  const listerThematiquesDisponibles = async (
    type: TypeSupport,
    origine?: string,
    groupe?: string,
  ): Promise<ThematiqueDecompte[]> => {
    try {
      const params: Record<string, string> = {}
      if (origine) params.origine = origine
      if (groupe) params.groupe = groupe
      const reponse = await $fetch<ApiResponse<ThematiqueDecompte[]>>(
        `${apiBase}${prefixePublic(type)}/thematiques`,
        { params: Object.keys(params).length ? params : undefined },
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch {
      return []
    }
  }

  const listerTerritoiresDisponibles = async (
    type: TypeSupport,
  ): Promise<TerritoiresDisponibles> => {
    try {
      const reponse = await $fetch<ApiResponse<TerritoiresDisponibles>>(
        `${apiBase}${prefixePublic(type)}/territoires`,
      )
      return reponse.success && reponse.data
        ? reponse.data
        : { territoires: [], continentales: 0 }
    }
    catch {
      return { territoires: [], continentales: 0 }
    }
  }

  /**
   * Catalogue **complet** des thèmes et des territoires, pour les sélecteurs.
   *
   * Distinct des deux listes ci-dessus, qui portent un décompte et, pour les
   * territoires, ne renvoient que ce qui est déjà couvert : les réutiliser dans
   * un formulaire rendrait un territoire inédit inatteignable, le premier
   * support à vouloir le choisir ne le voyant pas.
   */
  const listerReferentielsEdition = async (): Promise<{
    thematiques: ThematiquePublique[]
    territoires: TerritoirePublic[]
  }> => {
    try {
      const reponse = await $fetch<ApiResponse<{
        thematiques: ThematiquePublique[]
        territoires: TerritoirePublic[]
      }>>(`${apiBase}/api/medias/referentiels`)
      return reponse.success && reponse.data ? reponse.data : { thematiques: [], territoires: [] }
    }
    catch {
      return { thematiques: [], territoires: [] }
    }
  }

  // ── Fiche d'un support (membre et back-office) ──────────────────────

  const basePour = (admin: boolean) => (admin ? '/api/admin/medias' : '/api/medias')

  const obtenirThematiques = async (
    type: TypeSupport,
    supportId: string,
    admin = false,
  ): Promise<ThematiquePublique[]> => {
    try {
      const reponse = await $fetch<ApiResponse<ThematiquePublique[]>>(
        `${apiBase}${basePour(admin)}/${type}/${supportId}/thematiques`,
        { headers: authHeaders() },
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch {
      return []
    }
  }

  /** Remplacement intégral. Refus `400` si la liste est vide sur un support
   * publié (FR-029) : le message vient du serveur. */
  const definirThematiques = async (
    type: TypeSupport,
    supportId: string,
    categorieIds: string[],
    admin = false,
  ): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}${basePour(admin)}/${type}/${supportId}/thematiques`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: { categorie_ids: categorieIds },
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  const obtenirCouverture = async (
    type: TypeSupport,
    supportId: string,
    admin = false,
  ): Promise<CouverturePublique | null> => {
    try {
      const reponse = await $fetch<ApiResponse<CouverturePublique>>(
        `${apiBase}${basePour(admin)}/${type}/${supportId}/couverture`,
        { headers: authHeaders() },
      )
      return reponse.success ? reponse.data : null
    }
    catch {
      return null
    }
  }

  /** Les deux modes sont **exclusifs** (FR-034) : cocher la couverture
   * continentale vide la liste de territoires, côté serveur comme à l'écran. */
  const definirCouverture = async (
    type: TypeSupport,
    supportId: string,
    couvertureContinentale: boolean,
    paysIds: string[],
    admin = false,
  ): Promise<boolean> => {
    erreur.value = null
    try {
      await $fetch(`${apiBase}${basePour(admin)}/${type}/${supportId}/couverture`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...authHeaders() },
        body: {
          couverture_continentale: couvertureContinentale,
          pays_ids: couvertureContinentale ? [] : paysIds,
        },
      })
      return true
    }
    catch (e: any) {
      erreur.value = e?.data?.error || e?.message || 'Erreur réseau'
      return false
    }
  }

  return {
    erreur: readonly(erreur),
    listerThematiquesDisponibles,
    listerTerritoiresDisponibles,
    listerReferentielsEdition,
    obtenirThematiques,
    definirThematiques,
    obtenirCouverture,
    definirCouverture,
  }
}

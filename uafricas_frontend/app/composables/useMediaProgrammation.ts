// Grille de programmation récurrente d'un support média
// (US5 — migration 09n, handlers/media_programmation.rs).
//
// Un créneau n'est pas un instant mais une règle : « tous les jours à 20h30 »
// ou « chaque mercredi à 18h ». Le contenu diffusé est résolu **à la lecture**
// côté serveur — ce composable n'entretient aucune minuterie.

import type { TypeSupportMedia } from '~/composables/useMediaDetention'

export interface CreneauAPI {
  id: string
  type_support: TypeSupportMedia
  support_id: string
  contenu_id: string
  recurrence: 'quotidien' | 'hebdomadaire'
  /** 0 = dimanche … 6 = samedi ; `null` si quotidien. */
  jour_semaine: number | null
  jour_libelle: string | null
  /** « HH:MM », heure locale du `fuseau`. */
  heure_debut: string
  duree_minutes: number
  fuseau: string
  cree_par: string
  actif: boolean
  contenu_nom: string | null
  contenu_slug: string | null
  contenu_image: string | null
  /** Le contenu programmé n'est plus diffusable : à corriger (FR-043). */
  contenu_indisponible: boolean
  created_at: string
  updated_at: string
}

export interface DiffusionAPI {
  diffusion_en_cours: CreneauAPI | null
  creneau_suivant: CreneauAPI | null
}

export interface CreneauFormulaire {
  contenu_id: string
  recurrence: 'quotidien' | 'hebdomadaire'
  jour_semaine?: number | null
  heure_debut: string
  duree_minutes: number
  fuseau?: string
}

/** Index 0 = dimanche, convention `EXTRACT(DOW)` de PostgreSQL. */
export const JOURS_SEMAINE = [
  'Dimanche', 'Lundi', 'Mardi', 'Mercredi', 'Jeudi', 'Vendredi', 'Samedi',
] as const

/** Fuseau par défaut, aligné sur celui de la migration 09n. */
export const FUSEAU_DEFAUT = 'Africa/Abidjan'

/** Fuseaux proposés — couvre l'amplitude horaire du continent. */
export const FUSEAUX_PROPOSES = [
  'Africa/Abidjan',
  'Africa/Dakar',
  'Africa/Lagos',
  'Africa/Douala',
  'Africa/Kinshasa',
  'Africa/Cairo',
  'Africa/Nairobi',
  'Africa/Johannesburg',
  'Europe/Paris',
] as const

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** « 20:30 » + 45 min → « 21:15 ». Un créneau ne franchit jamais minuit. */
export const heureFin = (heureDebut: string, dureeMinutes: number): string => {
  const [h = '0', m = '0'] = heureDebut.split(':')
  const total = Number(h) * 60 + Number(m) + dureeMinutes
  const hh = String(Math.floor(total / 60)).padStart(2, '0')
  const mm = String(total % 60).padStart(2, '0')
  return `${hh}:${mm}`
}

export const useMediaProgrammation = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const chargement = ref(false)
  const erreur = ref<string | null>(null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) return { Authorization: `Bearer ${userStore.accessToken}` }
    return {}
  }

  const messageErreur = (e: any, defaut: string): string =>
    e?.data?.error || e?.message || defaut

  /** La grille complète — lecture publique, c'est un programme de diffusion. */
  const listerGrille = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
  ): Promise<CreneauAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<CreneauAPI[]>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/grille`,
      )
      return reponse.success && reponse.data ? reponse.data : []
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur réseau')
      return []
    }
    finally {
      chargement.value = false
    }
  }

  /**
   * « En ce moment » et « À suivre ».
   *
   * Les endpoints `sections` renvoient déjà ces deux champs : cet appel n'est
   * utile qu'aux pages de détail, qui ne passent pas par les sections.
   */
  const obtenirDiffusion = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
  ): Promise<DiffusionAPI | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<DiffusionAPI>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/diffusion`,
      )
      return reponse.success ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur réseau')
      return null
    }
  }

  /**
   * Création d'un créneau.
   *
   * Un chevauchement est refusé par le serveur (409) **sans rien écrire** : le
   * message renvoyé décrit le créneau en cause et doit être affiché tel quel.
   */
  const creerCreneau = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
    creneau: CreneauFormulaire,
  ): Promise<string | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/creneaux`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: normaliser(creneau),
        },
      )
      return reponse.success && reponse.data ? reponse.data.id : null
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de la création du créneau')
      return null
    }
  }

  const modifierCreneau = async (
    id: string,
    creneau: CreneauFormulaire,
  ): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string }>>(
        `${apiBase}/api/medias/creneaux/${id}`,
        {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: normaliser(creneau),
        },
      )
      return reponse.success
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de la modification du créneau')
      return false
    }
  }

  const supprimerCreneau = async (id: string): Promise<boolean> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<null>>(
        `${apiBase}/api/medias/creneaux/${id}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      return reponse.success
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de la suppression du créneau')
      return false
    }
  }

  /**
   * Un créneau quotidien ne porte pas de jour : l'envoyer ferait échouer le
   * CHECK `ck_creneau_jour_coherent` côté base.
   */
  const normaliser = (creneau: CreneauFormulaire) => ({
    contenu_id: creneau.contenu_id,
    recurrence: creneau.recurrence,
    jour_semaine: creneau.recurrence === 'quotidien' ? null : creneau.jour_semaine ?? null,
    heure_debut: creneau.heure_debut,
    duree_minutes: creneau.duree_minutes,
    fuseau: creneau.fuseau || FUSEAU_DEFAUT,
  })

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerGrille,
    obtenirDiffusion,
    creerCreneau,
    modifierCreneau,
    supprimerCreneau,
  }
}

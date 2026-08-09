// Grille de programmation récurrente d'un support média
// (US5 puis US2 de la feature 009 — migrations 09n et 09q).
//
// Un créneau n'est pas un instant mais une règle : « tous les jours à 20h30 »
// ou « chaque mercredi à 18h ». Depuis 09q il désigne un **programme**, plus un
// contenu diffusable : l'épisode qui passe se déduit de la **rotation**,
// calculée à la lecture côté serveur à partir de `date_effet`. Ce composable
// n'entretient donc aucune minuterie et ne calcule aucun rang.

import type { TypeSupportMedia } from '~/composables/useMediaDetention'

/** Programme ou épisode référencé par un créneau. */
export interface RefContenu {
  id: string
  titre: string
  slug?: string | null
  image_couverture_url?: string | null
  media_url?: string | null
  numero_episode?: number | null
}

export interface CreneauAPI {
  id: string
  type_support: TypeSupportMedia
  support_id: string
  /** Le créneau vise un **programme**, jamais un épisode (FR-014). */
  emission_id: string
  recurrence: 'quotidien' | 'hebdomadaire'
  /** 0 = dimanche … 6 = samedi ; `null` si quotidien. */
  jour_semaine: number | null
  jour_libelle: string | null
  /** « HH:MM », heure locale du `fuseau`. */
  heure_debut: string
  duree_minutes: number
  fuseau: string
  /**
   * « AAAA-MM-JJ » — origine du comptage des occurrences.
   *
   * La déplacer **redéfinit la rotation** : c'est le seul levier dont dispose
   * un détenteur pour choisir quel épisode passe quand.
   */
  date_effet: string
  cree_par: string
  actif: boolean
  emission?: RefContenu | null
  /** L'épisode retenu par la rotation. Absent des lectures de grille, qui ne
   * la résolvent pas — la grille dit ce qui est programmé, pas ce qui passe. */
  episode?: RefContenu | null
  /** Occurrences écoulées depuis `date_effet` (FR-016). */
  rang_occurrence?: number | null
  /** La rotation a bouclé et rejoue la série depuis le début (FR-020). */
  est_rediffusion: boolean
  /** Programme retiré, suspendu, ou sans épisode publié : le créneau reste
   * dans la grille mais n'annonce rien au public (FR-021, FR-024). */
  emission_indisponible: boolean
  /** Motif de l'indisponibilité — servi à la seule vue détenteur. */
  alerte?: string | null
  created_at: string
  updated_at: string
}

export interface DiffusionAPI {
  diffusion_en_cours: CreneauAPI | null
  creneau_suivant: CreneauAPI | null
}

export interface CreneauFormulaire {
  emission_id: string
  recurrence: 'quotidien' | 'hebdomadaire'
  jour_semaine?: number | null
  heure_debut: string
  duree_minutes: number
  fuseau?: string
  /** Facultative : le serveur retient aujourd'hui par défaut. */
  date_effet?: string | null
}

/** Alerte de cadence d'un programme dont l'échéance approche (FR-024). */
export interface AlerteCadence {
  emission: RefContenu
  support: { type: TypeSupportMedia; id: string; nom: string }
  cadence: string
  dernier_episode_at: string | null
  prochaine_echeance: string | null
  /** `approche` | `depassee` | `aucun_episode` */
  niveau: 'approche' | 'depassee' | 'aucun_episode'
  /** Évite l'alerte accusatrice : le détenteur a fait sa part, c'est la file
   * de modération qui n'a pas suivi. */
  episodes_en_attente: number
}

export const LIBELLES_NIVEAU_ALERTE: Record<AlerteCadence['niveau'], string> = {
  approche: 'Échéance proche',
  depassee: 'Échéance dépassée',
  aucun_episode: 'Aucun épisode publié',
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

/** Date du jour au format attendu par `date_effet`, sans dérive de fuseau. */
export const dateAujourdhui = (): string => {
  const maintenant = new Date()
  const mois = String(maintenant.getMonth() + 1).padStart(2, '0')
  const jour = String(maintenant.getDate()).padStart(2, '0')
  return `${maintenant.getFullYear()}-${mois}-${jour}`
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

  /**
   * La grille complète — lecture publique, c'est un programme de diffusion.
   *
   * `vueDetenteur` conserve les créneaux dont le programme n'annonce rien : le
   * public ne doit pas les voir (FR-021), mais les masquer au détenteur lui
   * cacherait précisément ce qu'il doit corriger.
   */
  const listerGrille = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
    vueDetenteur = false,
  ): Promise<CreneauAPI[]> => {
    chargement.value = true
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ creneaux: CreneauAPI[] }>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/grille${vueDetenteur ? '?vue=detenteur' : ''}`,
        { headers: authHeaders() },
      )
      return reponse.success && reponse.data ? reponse.data.creneaux : []
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
   * « En ce moment » et « À suivre », rotation résolue.
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
   * L'épisode retenu par la rotation revient dans la réponse — c'est ce qui
   * rend la date d'effet compréhensible au lieu de rester une abstraction.
   */
  const creerCreneau = async (
    typeSupport: TypeSupportMedia,
    supportId: string,
    creneau: CreneauFormulaire,
  ): Promise<{ id: string; episode_actuel: RefContenu | null } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string; episode_actuel: RefContenu | null }>>(
        `${apiBase}/api/medias/${typeSupport}/${supportId}/creneaux`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: normaliser(creneau),
        },
      )
      return reponse.success && reponse.data ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de la création du créneau')
      return null
    }
  }

  const modifierCreneau = async (
    id: string,
    creneau: CreneauFormulaire,
  ): Promise<{ id: string; episode_actuel: RefContenu | null } | null> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ id: string; episode_actuel: RefContenu | null }>>(
        `${apiBase}/api/medias/creneaux/${id}`,
        {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: normaliser(creneau),
        },
      )
      return reponse.success && reponse.data ? reponse.data : null
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur lors de la modification du créneau')
      return null
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
   * Programmes dont l'échéance de cadence approche ou est dépassée, tous
   * supports détenus confondus. Calculées à la lecture — aucune tâche de fond.
   */
  const mesAlertesCadence = async (): Promise<AlerteCadence[]> => {
    erreur.value = null
    try {
      const reponse = await $fetch<ApiResponse<{ alertes: AlerteCadence[] }>>(
        `${apiBase}/api/medias/mes-alertes-cadence`,
        { headers: authHeaders() },
      )
      return reponse.success && reponse.data ? reponse.data.alertes : []
    }
    catch (e: any) {
      erreur.value = messageErreur(e, 'Erreur réseau')
      return []
    }
  }

  /**
   * Un créneau quotidien ne porte pas de jour : l'envoyer ferait échouer le
   * CHECK `ck_creneau_jour_coherent` côté base. `date_effet` est omise quand
   * elle est vide, le serveur retenant alors aujourd'hui.
   */
  const normaliser = (creneau: CreneauFormulaire) => {
    const corps: Record<string, unknown> = {
      emission_id: creneau.emission_id,
      recurrence: creneau.recurrence,
      jour_semaine: creneau.recurrence === 'quotidien' ? null : creneau.jour_semaine ?? null,
      heure_debut: creneau.heure_debut,
      duree_minutes: creneau.duree_minutes,
      fuseau: creneau.fuseau || FUSEAU_DEFAUT,
    }
    if (creneau.date_effet) corps.date_effet = creneau.date_effet
    return corps
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    listerGrille,
    obtenirDiffusion,
    creerCreneau,
    modifierCreneau,
    supprimerCreneau,
    mesAlertesCadence,
  }
}

/**
 * Administration des activités ludiques (feature 013), bâtie sur `useAdmin`.
 * Routes : `/api/admin/jeu/*`, toutes gardées par la permission `jeu.gerer`.
 */
import type { ApiResponse } from '~/types/admin'
import type { TypeReponse } from '~/composables/useJeu'

export type EtatEpreuve = 'candidate' | 'jouable' | 'a_revoir' | 'rejetee' | 'retiree'
export type SourceEtat = 'aucune' | 'conforme' | 'modifiee' | 'indisponible'

export interface EpreuveAdminAPI {
  id: string
  module_code: string
  enonce: string
  media_type: 'image' | 'audio' | null
  media_url: string | null
  propositions: string[]
  /** Choix multiple : rang de la bonne proposition, à partir de 1. */
  bonne_reponse: number | null
  /** Feature 014 : `choix`, `carte`, `ordre` ou `paires`. */
  type_reponse: TypeReponse
  /** Carte : le pays attendu. */
  reponse_pays_id: string | null
  reponse_pays_nom: string | null
  reponse_pays_iso: string | null
  /** Ordre : les éléments dans l'ordre ATTENDU (la base les garde mélangés). */
  elements_attendus: ElementOrdre[] | null
  /** Paires : chaque élément de gauche avec son correspondant. */
  paires_attendues: PaireSaisie[] | null
  explication: string | null
  difficulte: number
  theme: string | null
  pays_id: string | null
  pays_nom: string | null
  origine: 'saisie' | 'derivee'
  type_source: string | null
  source_id: string | null
  forme: string | null
  etat: EtatEpreuve
  motif_rejet: string | null
  nombre_servie: number
  nombre_bonnes: number
  /** `null` tant que l'épreuve n'a jamais été servie. */
  taux_reussite: number | null
  source_etat: SourceEtat
  created_at: string
  updated_at: string
}

/** Corps de la saisie et de la modification. */
export interface ElementOrdre {
  texte: string
  valeur: string | null
}

export interface PaireSaisie {
  gauche: string
  droite: string
}

export interface EpreuveForm {
  module: string
  enonce: string
  media_type: 'image' | 'audio' | null
  media_url: string | null
  type_reponse: TypeReponse
  /** Choix multiple. */
  propositions: string[]
  bonne_reponse: number
  /** Carte : code ISO2 du pays attendu, en minuscules. */
  reponse_pays_iso: string | null
  /** Ordre : saisis DANS L'ORDRE ATTENDU ; le serveur les mélange avant d'écrire. */
  elements: ElementOrdre[]
  /** Paires. */
  paires: PaireSaisie[]
  explication: string | null
  difficulte: number
  theme: string | null
  pays_id: string | null
  type_source: string | null
  source_id: string | null
}

export interface ModuleAdminAPI {
  code: string
  libelle: string
  route: string
  icone: string | null
  ouvert: boolean
  ordre: number
  candidates: number
  jouables: number
  a_revoir: number
  rejetees: number
  retirees: number
}

/** Une forme de dérivation et ce qu'elle pourrait encore produire. */
export interface FormeDerivationAPI {
  forme: string
  module: string
  type_reponse: TypeReponse
  libelle: string
  sources_eligibles: number
  deja_proposees: number
}

/** Libellés des types de réponse, communs au formulaire et à la revue. */
export const LIBELLES_TYPE_REPONSE: Record<TypeReponse, string> = {
  choix: 'Choix multiple',
  carte: 'Carte',
  ordre: 'Ordre',
  paires: 'Paires',
}

export interface BilanDerivationAPI {
  module: string
  creees: number
  deja_proposees: number
  /** Sources pour lesquelles on n'a pas trouvé trois mauvaises réponses distinctes. */
  sans_distracteurs: number
  par_forme: Array<{ forme: string, libelle: string, creees: number, deja_proposees: number, sans_distracteurs: number }>
}

export interface BilanRevueAPI {
  acceptees: number
  rejetees: number
  /** Épreuves du lot qui n'ont pas pu être traitées, avec leur raison. */
  refus: Array<{ id: string, raison: string }>
}

export interface SignalementAdminAPI {
  id: string
  motif: 'reponse_erronee' | 'enonce_ambigu' | 'contenu_deplace' | 'autre'
  commentaire: string | null
  etat: 'en_attente' | 'confirme' | 'classe'
  created_at: string
  auteur_id: string
  auteur: string
}

/** La file est regroupée par épreuve : un sujet à traiter, n signaleurs. */
export interface EpreuveSignaleeAPI {
  epreuve_id: string
  enonce: string
  module_code: string
  epreuve_etat: EtatEpreuve
  propositions: string[]
  bonne_reponse: number | null
  type_reponse: TypeReponse
  signalements: SignalementAdminAPI[]
}

export interface DefiAdminAPI {
  id: string
  periodicite: 'jour' | 'semaine'
  periode_debut: string
  module_code: string | null
  titre: string | null
  nombre_epreuves: number
  origine: 'programme' | 'automatique'
  participants: number
  termines: number
  /** La période n'a pas commencé : le défi est encore modifiable. */
  a_venir: boolean
}

export interface SaisonAdminAPI {
  id: string
  nom: string
  debut_at: string
  fin_at: string
  cloturee_at: string | null
  etat: 'a_venir' | 'en_cours' | 'close'
  /** Membres ayant du score dans cette saison. */
  joueurs: number
}

export interface JoueurAdminAPI {
  utilisateur_id: string
  nom: string
  prenom: string
  email: string
  score_total: number
  gains: number
  gains_annules: number
  dernier_gain_at: string | null
}

export interface BilanAnnulationAPI {
  gains_annules: number
  score_retire: number
  score_total: number
  reputation_retiree: number
}

export const LIBELLES_MOTIF_SIGNALEMENT: Record<SignalementAdminAPI['motif'], string> = {
  reponse_erronee: 'Réponse erronée',
  enonce_ambigu: 'Énoncé ambigu',
  contenu_deplace: 'Contenu déplacé',
  autre: 'Autre',
}

/** Libellés des états, partagés par la liste et la page d'édition. */
export const LIBELLES_ETAT_EPREUVE: Record<EtatEpreuve, string> = {
  candidate: 'Brouillon',
  jouable: 'Jouable',
  a_revoir: 'À revoir',
  rejetee: 'Rejetée',
  retiree: 'Retirée',
}

export const CLASSES_ETAT_EPREUVE: Record<EtatEpreuve, string> = {
  candidate: 'badge-ghost',
  jouable: 'badge-success',
  a_revoir: 'badge-warning',
  rejetee: 'badge-error',
  retiree: 'badge-neutral',
}

export const LIBELLES_SOURCE_ETAT: Record<SourceEtat, string> = {
  aucune: 'Sans source',
  conforme: 'Source conforme',
  modifiee: 'Source modifiée',
  indisponible: 'Source dépubliée',
}

export const LIBELLES_DIFFICULTE: Record<number, string> = { 1: 'Facile', 2: 'Moyen', 3: 'Difficile' }

/** Message d'erreur du serveur, sans son préfixe technique (« Validation: … »). */
export const messageErreurAdminJeu = (e: unknown, defaut = 'Une erreur est survenue.'): string => {
  const brut = (e as { data?: { error?: string } })?.data?.error
  return brut ? brut.replace(/^[^:]{1,30}:\s*/, '') : defaut
}

export const useAdminJeu = () => {
  const {
    adminFetch, listerPagine, pagination, sort, loading, error,
    allerPage, changerTri, reinitialiserPagination,
  } = useAdmin()

  const epreuves = ref<EpreuveAdminAPI[]>([])

  const filtres = reactive({
    recherche: '',
    module: '',
    etat: '',
    origine: '',
    difficulte: '',
    anomalie: '',
  })

  const chargerEpreuves = async () => {
    // `anomalie` est un booléen côté serveur : on ne l'envoie que coché.
    const params: Record<string, unknown> = { ...filtres }
    if (params.anomalie !== 'true') delete params.anomalie
    const resultat = await listerPagine<EpreuveAdminAPI>('/api/admin/jeu/epreuves', params)
    if (resultat) epreuves.value = resultat.data
  }

  const obtenirEpreuve = async (id: string) =>
    (await adminFetch<ApiResponse<EpreuveAdminAPI>>(`/api/admin/jeu/epreuves/${id}`)).data

  const creerEpreuve = async (form: EpreuveForm) =>
    (await adminFetch<ApiResponse<EpreuveAdminAPI>>('/api/admin/jeu/epreuves', {
      method: 'POST', body: form,
    })).data

  const modifierEpreuve = async (id: string, form: EpreuveForm) =>
    (await adminFetch<ApiResponse<EpreuveAdminAPI>>(`/api/admin/jeu/epreuves/${id}`, {
      method: 'PUT', body: form,
    })).data

  const publierEpreuve = async (id: string) =>
    (await adminFetch<ApiResponse<EpreuveAdminAPI>>(`/api/admin/jeu/epreuves/${id}/publier`, {
      method: 'POST',
    })).data

  const retirerEpreuve = async (id: string) =>
    (await adminFetch<ApiResponse<EpreuveAdminAPI>>(`/api/admin/jeu/epreuves/${id}/retirer`, {
      method: 'POST',
    })).data

  /**
   * POST /api/admin/jeu/medias : dépose une photo ou un extrait sonore
   * d'épreuve (feature 014) et renvoie l'adresse à placer dans l'épreuve.
   */
  const deposerMedia = async (fichier: File, type: 'image' | 'audio') => {
    const corps = new FormData()
    corps.append('type', type)
    corps.append('fichier', fichier)
    return (await adminFetch<ApiResponse<{ media_type: 'image' | 'audio', media_url: string }>>(
      '/api/admin/jeu/medias', { method: 'POST', body: corps },
    )).data
  }

  const listerModules = async () =>
    (await adminFetch<ApiResponse<ModuleAdminAPI[]>>('/api/admin/jeu/modules')).data ?? []

  const modifierModule = async (code: string, ouvert: boolean) =>
    (await adminFetch<ApiResponse<{ code: string, ouvert: boolean }>>(
      `/api/admin/jeu/modules/${code}`,
      { method: 'PATCH', body: { ouvert } },
    )).data

  // ── Dérivation et revue ────────────────────────────────────────────────

  const listerFormes = async () =>
    (await adminFetch<ApiResponse<FormeDerivationAPI[]>>('/api/admin/jeu/epreuves/formes')).data ?? []

  /** `formes` omis : toutes celles du module. Rejouable sans doublon. */
  const deriver = async (module: string, formes?: string[]) =>
    (await adminFetch<ApiResponse<BilanDerivationAPI>>('/api/admin/jeu/epreuves/derivation', {
      method: 'POST', body: { module, formes: formes ?? null },
    })).data

  const revue = async (ids: string[], decision: 'accepter' | 'rejeter', motif?: string) =>
    (await adminFetch<ApiResponse<BilanRevueAPI>>('/api/admin/jeu/epreuves/revue', {
      method: 'POST', body: { ids, decision, motif: motif ?? null },
    })).data

  // ── Signalements d'épreuve ─────────────────────────────────────────────

  const listerSignalements = async (etat: SignalementAdminAPI['etat'] = 'en_attente') =>
    (await adminFetch<ApiResponse<EpreuveSignaleeAPI[]>>('/api/admin/jeu/signalements', {
      params: { etat },
    })).data ?? []

  const deciderSignalement = async (
    id: string,
    decision: 'confirmer' | 'classer',
    retirerEpreuve = false,
  ) =>
    (await adminFetch<ApiResponse<{ signalements_traites: number, epreuve_retiree: boolean }>>(
      `/api/admin/jeu/signalements/${id}/decision`,
      { method: 'POST', body: { decision, retirer_epreuve: retirerEpreuve } },
    )).data

  // ── Défis ──────────────────────────────────────────────────────────────

  const listerDefis = async (periodicite?: 'jour' | 'semaine') =>
    (await adminFetch<ApiResponse<DefiAdminAPI[]>>('/api/admin/jeu/defis', {
      params: { periodicite: periodicite ?? '' },
    })).data ?? []

  /** `date` au format `AAAA-MM-JJ` ; à venir seulement, et un lundi pour la semaine. */
  const programmerDefi = async (
    periodicite: 'jour' | 'semaine',
    date: string,
    corps: { titre: string | null, module: string | null, epreuve_ids: string[] },
  ) =>
    (await adminFetch<ApiResponse<{ id: string }>>(`/api/admin/jeu/defis/${periodicite}/${date}`, {
      method: 'PUT', body: corps,
    })).data

  const deprogrammerDefi = async (periodicite: 'jour' | 'semaine', date: string) => {
    await adminFetch<ApiResponse<null>>(`/api/admin/jeu/defis/${periodicite}/${date}`, {
      method: 'DELETE',
    })
  }

  // ── Saisons du Championship ────────────────────────────────────────────

  const listerSaisons = async () =>
    (await adminFetch<ApiResponse<SaisonAdminAPI[]>>('/api/admin/jeu/saisons')).data ?? []

  /** Dates en ISO 8601. Un début dans le passé est ramené à maintenant par le serveur. */
  const creerSaison = async (corps: { nom: string, debut_at: string, fin_at: string }) =>
    (await adminFetch<ApiResponse<SaisonAdminAPI>>('/api/admin/jeu/saisons', {
      method: 'POST', body: corps,
    })).data

  const modifierSaison = async (id: string, corps: { nom: string, debut_at: string, fin_at: string }) =>
    (await adminFetch<ApiResponse<SaisonAdminAPI>>(`/api/admin/jeu/saisons/${id}`, {
      method: 'PUT', body: corps,
    })).data

  /** Termine la saison maintenant : classement archivé, podium distingué. */
  const cloreSaison = async (id: string) =>
    (await adminFetch<ApiResponse<SaisonAdminAPI>>(`/api/admin/jeu/saisons/${id}/clore`, {
      method: 'POST',
    })).data

  // ── Triche ─────────────────────────────────────────────────────────────

  const listerJoueurs = async (recherche = '') =>
    (await adminFetch<ApiResponse<JoueurAdminAPI[]>>('/api/admin/jeu/joueurs', {
      params: { recherche },
    })).data ?? []

  /** `depuis` en ISO 8601 ; omis : tous les gains. Sans inverse. */
  const annulerGains = async (
    utilisateurId: string,
    corps: { motif: string, depuis: string | null, retrait_reputation: number },
  ) =>
    (await adminFetch<ApiResponse<BilanAnnulationAPI>>(
      `/api/admin/jeu/joueurs/${utilisateurId}/annuler-gains`,
      { method: 'POST', body: corps },
    )).data

  return {
    listerJoueurs, annulerGains,
    listerSaisons, creerSaison, modifierSaison, cloreSaison,
    listerDefis, programmerDefi, deprogrammerDefi,
    epreuves, filtres, pagination, sort, loading, error,
    chargerEpreuves, obtenirEpreuve, creerEpreuve, modifierEpreuve,
    publierEpreuve, retirerEpreuve, listerModules, modifierModule, deposerMedia,
    listerFormes, deriver, revue, listerSignalements, deciderSignalement,
    allerPage, changerTri, reinitialiserPagination,
  }
}

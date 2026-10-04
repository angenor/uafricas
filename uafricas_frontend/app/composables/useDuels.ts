/**
 * Duels entre amis (feature 013).
 *
 * Les signaux SSE `duel_*` arrivent par le flux de la messagerie (plugin
 * `messagerie.client.ts`) et passent par `gererEvenement`. Un signal dit qu'il
 * faut RELIRE, il ne transporte jamais l'état : le flux n'a pas de tampon, un
 * membre déconnecté le perd. Les écrans relisent donc toujours le duel.
 */
import type { MembreLightAPI } from '~/composables/useAmis'
import type { EpreuveServieAPI } from '~/composables/useJeu'

export type EtatDuel = 'propose' | 'accepte' | 'en_cours' | 'termine' | 'refuse' | 'annule' | 'expire'
export type IssueDuel = 'victoire' | 'nul' | 'forfait' | 'sans_issue'

export interface PartieDuelAPI {
  id: string
  utilisateur_id: string
  etat: 'en_cours' | 'terminee' | 'close'
  bonnes: number
  temps_total_ms: number
}

/** Un duel vu d'UN des deux joueurs. */
export interface DuelAPI {
  id: string
  mode: 'differe' | 'direct'
  module: string
  module_libelle: string
  etat: EtatDuel
  /** `false` : duel amical, sans gain. */
  compte: boolean
  je_propose: boolean
  adversaire: MembreLightAPI | null
  nombre_epreuves: number
  propose_at: string
  accepte_at: string | null
  echeance_at: string
  issue: IssueDuel | null
  vainqueur_id: string | null
  termine_at: string | null
  ma_partie: PartieDuelAPI | null
  /** Servie seulement une fois le duel terminé. */
  sa_partie: PartieDuelAPI | null
  mon_gain: number
}

export interface MesDuelsAPI {
  a_repondre: DuelAPI[]
  a_jouer: DuelAPI[]
  en_attente: DuelAPI[]
  termines: DuelAPI[]
  quotas: { comptes_aujourdhui: number, plafond: number }
}

/** L'état autoritaire d'un duel direct, relu à chaque signal et toutes les 3 s. */
export interface EtatDuelDirectAPI {
  etat: EtatDuel
  phase: 'attente' | 'question' | 'revelation' | 'termine'
  /** Heure du serveur : les comptes à rebours s'y calent. */
  maintenant: string
  rang: number
  sur: number
  manche_debut_at: string | null
  manche_fin_at: string | null
  /** Début de la manche suivante, en phase de révélation. */
  prochaine_at: string | null
  epreuve: EpreuveServieAPI | null
  ma_cle: number | null
  /** Booléen seulement : ce que l'autre a répondu n'est jamais servi pendant la question. */
  adversaire_a_repondu: boolean
  correction: {
    bonne_cle: number
    explication: string | null
    lien: string | null
    ma_cle: number | null
    sa_cle: number | null
  } | null
  mes_bonnes: number
  ses_bonnes: number
  adversaire_present: boolean
  issue: IssueDuel | null
  vainqueur_id: string | null
  mon_gain: number
}

/** Une invitation à un duel DIRECT reçue par le flux : elle surgit où qu'on soit. */
export interface InvitationDuel {
  duel_id: string
  proposant: MembreLightAPI
  expire_a: string
}

export interface EvenementDuel {
  type: 'duel_propose' | 'duel_accepte' | 'duel_refuse' | 'duel_annule' | 'duel_a_vous' | 'duel_manche' | 'duel_termine'
  duel_id: string
  mode?: 'differe' | 'direct'
  proposant?: MembreLightAPI
  expire_a?: string
}

/** Ce que le duel a donné, du point de vue du membre. */
export const resultatDuel = (duel: DuelAPI, moiId: string | null | undefined): string => {
  switch (duel.etat) {
    case 'termine':
      if (duel.issue === 'nul') return 'Match nul'
      if (duel.issue === 'sans_issue') return 'Sans vainqueur'
      if (duel.vainqueur_id === moiId) return duel.issue === 'forfait' ? 'Gagné par forfait' : 'Victoire'
      return duel.issue === 'forfait' ? 'Perdu par forfait' : 'Défaite'
    case 'refuse': return 'Refusé'
    case 'annule': return 'Annulé'
    case 'expire': return 'Expiré'
    default: return ''
  }
}

export const useDuels = () => {
  const { appelAuth } = useJeu()
  const { compteurNonLues } = useNotifications()

  /**
   * Compteur de rafraîchissement : chaque signal `duel_*` l'incrémente, les
   * écrans qui affichent des duels le surveillent et relisent.
   */
  const signal = useState<number>('duel:signal', () => 0)
  /** Dernier signal reçu, pour qu'un écran sache s'il concerne SON duel. */
  const dernierSignal = useState<EvenementDuel | null>('duel:dernier', () => null)
  /** Invitation à un duel direct en attente de réponse. */
  const invitation = useState<InvitationDuel | null>('duel:invitation', () => null)
  /** Duel direct ouvert dans une salle : on n'interrompt pas un joueur en pleine partie. */
  const duelDirectEnCours = useState<string | null>('duel:direct-en-cours', () => null)

  const listerDuels = () => appelAuth<MesDuelsAPI>('/duels')

  /** La réponse porte `sera_compte` : un duel amical est annoncé AVANT de jouer. */
  const proposerDuel = (adversaireId: string, module: string, mode: 'differe' | 'direct' = 'differe') =>
    appelAuth<DuelAPI & { sera_compte: boolean }>('/duels', {
      method: 'POST', body: { adversaire_id: adversaireId, module, mode },
    })

  const obtenirDuel = (id: string) => appelAuth<DuelAPI>(`/duels/${id}`)
  const accepter = (id: string) => appelAuth<DuelAPI>(`/duels/${id}/accepter`, { method: 'POST' })
  const refuser = (id: string) => appelAuth<DuelAPI>(`/duels/${id}/refuser`, { method: 'POST' })
  const annuler = (id: string) => appelAuth<DuelAPI>(`/duels/${id}/annuler`, { method: 'POST' })
  /** Crée ou renvoie la partie du membre ; elle se joue sur l'écran des parties. */
  const jouer = (id: string) =>
    appelAuth<{ partie_id: string, etat: string }>(`/duels/${id}/jouer`, { method: 'POST' })

  /** GET /api/jeu/duels/{id}/direct : l'état autoritaire ; chaque appel vaut présence. */
  const etatDirect = (id: string) => appelAuth<EtatDuelDirectAPI>(`/duels/${id}/direct`)

  /** POST …/direct/repondre : ne renvoie PAS la correction (elle arrive aux deux à la fois). */
  const repondreDirect = (id: string, rang: number, cle: number) =>
    appelAuth<{ enregistre: boolean }>(`/duels/${id}/direct/repondre`, {
      method: 'POST', body: { rang, cle },
    })

  /** POST …/convertir : un duel direct resté sans réponse rouvert en différé. */
  const convertir = (id: string) =>
    appelAuth<DuelAPI & { sera_compte: boolean }>(`/duels/${id}/convertir`, { method: 'POST' })

  /** Branché par le plugin du flux SSE. */
  const gererEvenement = (evt: EvenementDuel) => {
    dernierSignal.value = evt
    signal.value += 1

    // Une invitation à un duel DIRECT surgit, sauf en pleine partie directe.
    if (evt.type === 'duel_propose' && evt.mode === 'direct' && evt.proposant && evt.expire_a
      && !duelDirectEnCours.value) {
      invitation.value = { duel_id: evt.duel_id, proposant: evt.proposant, expire_a: evt.expire_a }
    }
    if (['duel_annule', 'duel_termine', 'duel_refuse'].includes(evt.type)
      && invitation.value?.duel_id === evt.duel_id) {
      invitation.value = null
    }
    // Les signaux de manche sont fréquents : pas de quoi recompter la cloche.
    if (evt.type !== 'duel_manche') compteurNonLues()
  }

  return {
    signal, dernierSignal, invitation, duelDirectEnCours,
    listerDuels, proposerDuel, obtenirDuel, accepter, refuser, annuler, jouer,
    etatDirect, repondreDirect, convertir,
    gererEvenement,
  }
}

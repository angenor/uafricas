// ════════════════════════════════════════════════════════════════════════════
// Composable — Assistant conversationnel de construction d'arbre
// Guide l'utilisateur, question par question, pour bâtir son arbre généalogique :
//   1. Lui-même  →  2. Ses deux parents  →  3. Les parents de ses parents (et
//   ainsi de suite, en remontant)  →  4. Ses enfants  →  5. Ses petits-enfants.
// L'arbre se construit en parallèle (chaque réponse crée la personne + le lien).
// ════════════════════════════════════════════════════════════════════════════

import { ref, computed } from 'vue'
import { useArbreGenealogique } from '~/composables/useArbreGenealogique'
import type { CreerPersonneForm, Genre, TypeLien } from '~/mocks/arbre-genealogique'

// ─── Types ───────────────────────────────────────────────────────────────

export type RoleSaisie =
  | 'moi'
  | 'pere'
  | 'mere'
  | 'gp_pere'
  | 'gp_mere'
  | 'enfant'
  | 'petit_enfant'
  | 'ctx_parent'
  | 'ctx_enfant'
  | 'ctx_conjoint'

export type CleQuestion =
  | 'asc_gate'
  | 'enfants_gate'
  | 'enfants_encore'
  | 'desc_gate'
  | 'desc_encore'

export type ActionContexte = 'parent' | 'enfant' | 'conjoint' | 'fin'

export interface CiblePersonne {
  rattId: string
  label: string
  genre: Genre
}

export interface MessageChat {
  id: number
  role: 'bot' | 'user'
  texte: string
  icone?: string
}

export interface PromptSaisie {
  type: 'saisie'
  cle: RoleSaisie
  texte: string
  genrePreset?: Genre
  permetPasser: boolean
}

export interface PromptQuestion {
  type: 'question'
  cle: CleQuestion
  texte: string
  ouiLabel: string
  nonLabel: string
}

export interface PromptMenu {
  type: 'menu'
  cle: 'ctx_menu'
  texte: string
  options: { valeur: ActionContexte; label: string; icone: string }[]
}

export interface PromptFin {
  type: 'fin'
  cle: 'fin'
  texte: string
}

export type PromptActif = PromptSaisie | PromptQuestion | PromptMenu | PromptFin | null

interface EntreeFile {
  rattId: string
  label: string
  genre: Genre
}

interface OptionsAssistant {
  /** Appelé après chaque ajout réussi (personne + lien) pour rafraîchir l'arbre. */
  onArbreModifie?: () => void
}

// ─── Helpers purs ──────────────────────────────────────────────────────────

const genreVersTypeLien = (genre?: Genre): TypeLien => {
  if (genre === 'masculin') return 'pere'
  if (genre === 'feminin') return 'mere'
  return 'parent'
}

const labelDe = (form: CreerPersonneForm): string => {
  const prenom = form.prenoms?.trim()
  return prenom ? `${prenom} ${form.nom}`.trim() : form.nom
}

const mapperErreurLien = (err?: string | null): string => {
  const e = err || ''
  if (e.includes('circulaire') || e.includes('cycle')) {
    return 'Ce lien créerait une boucle dans l’arbre (une personne ne peut pas être son propre ancêtre).'
  }
  if (e.includes('existe déjà')) {
    return 'Un lien de ce type existe déjà entre ces deux personnes.'
  }
  return e || 'Erreur lors de la création du lien.'
}

// ─── Composable ──────────────────────────────────────────────────────────

export const useAssistantArbre = (options: OptionsAssistant = {}) => {
  const { creerPersonne, creerLien, supprimerPersonne } = useArbreGenealogique()

  // État exposé
  const messages = ref<MessageChat[]>([])
  const promptActif = ref<PromptActif>(null)
  const promptSeq = ref(0)
  const enCours = ref(false)
  const erreur = ref<string | null>(null)
  const termine = ref(false)

  // Flux courant : construction guidée vs interaction contextuelle (clic sur un nœud)
  let flux: 'onboarding' | 'contexte' | 'idle' = 'onboarding'
  let cibleCtx: CiblePersonne | null = null
  let repriseOnboarding = false

  // État interne de la machine
  type Phase = 'moi' | 'parents' | 'ascendants' | 'enfants' | 'descendants' | 'fin'
  const phase = ref<Phase>('moi')

  let moiRattId: string | null = null
  let moiGenre: Genre | undefined
  let moiLabel = 'vous'

  let pereFait = false
  let mereFait = false

  // Remontée des ascendants (parcours en largeur)
  const fileAscendants = ref<EntreeFile[]>([])
  let ascCourant: EntreeFile | null = null
  let ascSousEtape: 'gate' | 'pere' | 'mere' = 'gate'

  // Enfants
  let enfantsSousEtape: 'gate' | 'saisie' | 'encore' | 'fini' = 'gate'
  let nbEnfants = 0

  // Petits-enfants (par enfant)
  const fileDescendants = ref<EntreeFile[]>([])
  let descCourant: EntreeFile | null = null
  let descSousEtape: 'gate' | 'saisie' | 'encore' = 'gate'

  let compteurMessage = 0

  const phaseLabel = computed(() => {
    switch (phase.value) {
      case 'moi': return 'Vous'
      case 'parents': return 'Vos parents'
      case 'ascendants': return 'Vos ancêtres'
      case 'enfants': return 'Vos enfants'
      case 'descendants': return 'Vos petits-enfants'
      case 'fin': return 'Terminé'
      default: return ''
    }
  })

  // ── Émission de messages ───────────────────────────────────────────────

  const pousserBot = (texte: string, icone?: string) => {
    messages.value.push({ id: ++compteurMessage, role: 'bot', texte, icone })
  }
  const pousserUser = (texte: string) => {
    messages.value.push({ id: ++compteurMessage, role: 'user', texte })
  }

  const definirSaisie = (
    cle: RoleSaisie,
    texte: string,
    opts: { genrePreset?: Genre; permetPasser?: boolean } = {},
  ) => {
    pousserBot(texte, 'comment-dots')
    promptActif.value = {
      type: 'saisie',
      cle,
      texte,
      genrePreset: opts.genrePreset,
      permetPasser: !!opts.permetPasser,
    }
    promptSeq.value++
  }

  const definirQuestion = (
    cle: CleQuestion,
    texte: string,
    ouiLabel = 'Oui',
    nonLabel = 'Non',
  ) => {
    pousserBot(texte, 'circle-info')
    promptActif.value = { type: 'question', cle, texte, ouiLabel, nonLabel }
    promptSeq.value++
  }

  const definirMenu = (texte: string) => {
    pousserBot(texte, 'circle-info')
    promptActif.value = {
      type: 'menu',
      cle: 'ctx_menu',
      texte,
      options: [
        { valeur: 'parent', label: 'Un parent', icone: 'arrow-up' },
        { valeur: 'enfant', label: 'Un enfant', icone: 'arrow-down' },
        { valeur: 'conjoint', label: 'Un(e) conjoint(e)', icone: 'heart' },
        { valeur: 'fin', label: 'Non, c’est tout', icone: 'check' },
      ],
    }
    promptSeq.value++
  }

  const definirIdle = () => {
    flux = 'idle'
    promptActif.value = null
    pousserBot('Très bien. Cliquez sur une personne de l’arbre pour l’enrichir.', 'hand-pointer')
    promptSeq.value++
  }

  const definirFin = () => {
    phase.value = 'fin'
    termine.value = true
    const texte = 'Bravo, votre arbre prend forme ! 🌳 Vous pouvez maintenant l’enrichir librement en cliquant directement sur les personnes de l’arbre.'
    pousserBot(texte, 'circle-check')
    promptActif.value = { type: 'fin', cle: 'fin', texte }
    promptSeq.value++
  }

  // ── Machine à états : détermine la prochaine étape ─────────────────────

  const avancer = () => {
    // 1. Moi
    if (!moiRattId) {
      phase.value = 'moi'
      definirSaisie('moi', 'Bonjour ! Construisons votre arbre ensemble. Pour commencer, parlez-nous de vous : quel est votre nom ?')
      return
    }

    // 2. Parents directs
    if (!pereFait) {
      phase.value = 'parents'
      definirSaisie('pere', `Qui est le père de ${moiLabel} ?`, { genrePreset: 'masculin', permetPasser: true })
      return
    }
    if (!mereFait) {
      phase.value = 'parents'
      definirSaisie('mere', `Et qui est la mère de ${moiLabel} ?`, { genrePreset: 'feminin', permetPasser: true })
      return
    }

    // 3. Ascendants (grands-parents, puis arrière-grands-parents, etc.)
    if (ascCourant) {
      phase.value = 'ascendants'
      if (ascSousEtape === 'gate') {
        definirQuestion('asc_gate', `Souhaitez-vous renseigner les parents de ${ascCourant.label} ?`, 'Oui', 'Non, passer')
        return
      }
      if (ascSousEtape === 'pere') {
        definirSaisie('gp_pere', `Qui est le père de ${ascCourant.label} ?`, { genrePreset: 'masculin', permetPasser: true })
        return
      }
      if (ascSousEtape === 'mere') {
        definirSaisie('gp_mere', `Et la mère de ${ascCourant.label} ?`, { genrePreset: 'feminin', permetPasser: true })
        return
      }
    }
    if (fileAscendants.value.length > 0) {
      ascCourant = fileAscendants.value.shift() ?? null
      ascSousEtape = 'gate'
      avancer()
      return
    }

    // 4. Enfants
    if (enfantsSousEtape !== 'fini') {
      phase.value = 'enfants'
      if (enfantsSousEtape === 'gate') {
        definirQuestion('enfants_gate', `Avez-vous des enfants, ${moiLabel} ?`)
        return
      }
      if (enfantsSousEtape === 'saisie') {
        definirSaisie('enfant', nbEnfants === 0 ? 'Parlez-nous de votre premier enfant.' : 'Parlez-nous de cet enfant.')
        return
      }
      if (enfantsSousEtape === 'encore') {
        definirQuestion('enfants_encore', 'Avez-vous un autre enfant ?')
        return
      }
    }

    // 5. Petits-enfants (par enfant)
    if (descCourant) {
      phase.value = 'descendants'
      if (descSousEtape === 'gate') {
        definirQuestion('desc_gate', `Est-ce que ${descCourant.label} a des enfants ?`)
        return
      }
      if (descSousEtape === 'saisie') {
        definirSaisie('petit_enfant', `Parlez-nous d’un enfant de ${descCourant.label}.`)
        return
      }
      if (descSousEtape === 'encore') {
        definirQuestion('desc_encore', `${descCourant.label} a-t-il/elle un autre enfant ?`)
        return
      }
    }
    if (fileDescendants.value.length > 0) {
      descCourant = fileDescendants.value.shift() ?? null
      descSousEtape = 'gate'
      avancer()
      return
    }

    // 6. Terminé
    definirFin()
  }

  // ── Flux contextuel (clic sur une personne de l'arbre) ─────────────────

  const libelleAction = (a: ActionContexte): string => {
    switch (a) {
      case 'parent': return 'Ajouter un parent'
      case 'enfant': return 'Ajouter un enfant'
      case 'conjoint': return 'Ajouter un(e) conjoint(e)'
      case 'fin': return 'Non, c’est tout'
    }
  }

  /** Démarre une interaction contextuelle ciblant une personne cliquée dans l'arbre. */
  const demarrerContexte = (cible: CiblePersonne) => {
    if (enCours.value) return
    // Si une construction guidée est en cours, on la reprendra après l'interaction.
    if (flux === 'onboarding' && promptActif.value && promptActif.value.type !== 'fin') {
      repriseOnboarding = true
    }
    flux = 'contexte'
    cibleCtx = cible
    erreur.value = null
    definirMenu(`Que souhaitez-vous ajouter à ${cible.label} ?`)
  }

  const terminerContexte = () => {
    cibleCtx = null
    if (repriseOnboarding && !termine.value) {
      repriseOnboarding = false
      flux = 'onboarding'
      avancer()
    } else {
      definirIdle()
    }
  }

  /** Choix d'une action dans le menu contextuel. */
  const choisirAction = (action: ActionContexte) => {
    if (enCours.value || !promptActif.value || promptActif.value.type !== 'menu' || !cibleCtx) return
    pousserUser(libelleAction(action))
    erreur.value = null
    switch (action) {
      case 'parent':
        definirSaisie('ctx_parent', `Qui est le parent de ${cibleCtx.label} ?`)
        break
      case 'enfant':
        definirSaisie('ctx_enfant', `Parlez-nous d’un enfant de ${cibleCtx.label}.`)
        break
      case 'conjoint':
        definirSaisie('ctx_conjoint', `Qui est le/la conjoint(e) de ${cibleCtx.label} ?`)
        break
      case 'fin':
        terminerContexte()
        break
    }
  }

  // ── Appels API ─────────────────────────────────────────────────────────

  const creerPersonneInterne = async (
    form: CreerPersonneForm,
  ): Promise<{ rattId: string; personneId: string } | null> => {
    try {
      const rep = await creerPersonne(form)
      if (!rep.success || !rep.data) {
        erreur.value = rep.error || 'Erreur lors de la création de la personne.'
        return null
      }
      return { rattId: rep.data.rattachement_id, personneId: rep.data.personne.id }
    } catch (e: any) {
      erreur.value = e?.data?.error || 'Connexion perdue. Veuillez réessayer.'
      return null
    }
  }

  const lier = async (source: string, cible: string, type: TypeLien): Promise<boolean> => {
    try {
      const rep = await creerLien({
        rattachement_source_id: source,
        rattachement_cible_id: cible,
        type_lien: type,
      })
      if (!rep.success) {
        erreur.value = mapperErreurLien(rep.error)
        return false
      }
      return true
    } catch (e: any) {
      erreur.value = e?.data?.error || 'Connexion perdue. Veuillez réessayer.'
      return false
    }
  }

  // ── Actions exposées ───────────────────────────────────────────────────

  /** Soumission d'une personne saisie (réponse à un prompt de type « saisie »). */
  const soumettrePersonne = async (form: CreerPersonneForm) => {
    if (enCours.value || !promptActif.value || promptActif.value.type !== 'saisie') return
    const cle = promptActif.value.cle
    enCours.value = true
    erreur.value = null

    // 1. Création de la personne
    const res = await creerPersonneInterne(form)
    if (!res) { enCours.value = false; return }

    const entree: EntreeFile = {
      rattId: res.rattId,
      label: labelDe(form),
      genre: form.genre ?? 'non_precise',
    }

    // 2. Création du lien selon le rôle
    let ok = true
    switch (cle) {
      case 'moi':
        moiRattId = res.rattId
        moiGenre = form.genre
        moiLabel = labelDe(form)
        break
      case 'pere':
        ok = await lier(res.rattId, moiRattId!, 'pere')
        if (ok) { pereFait = true; fileAscendants.value.push(entree) }
        break
      case 'mere':
        ok = await lier(res.rattId, moiRattId!, 'mere')
        if (ok) { mereFait = true; fileAscendants.value.push(entree) }
        break
      case 'gp_pere':
        ok = await lier(res.rattId, ascCourant!.rattId, 'pere')
        if (ok) { fileAscendants.value.push(entree); ascSousEtape = 'mere' }
        break
      case 'gp_mere':
        ok = await lier(res.rattId, ascCourant!.rattId, 'mere')
        if (ok) { fileAscendants.value.push(entree); ascCourant = null }
        break
      case 'enfant':
        ok = await lier(moiRattId!, res.rattId, genreVersTypeLien(moiGenre))
        if (ok) { fileDescendants.value.push(entree); nbEnfants++; enfantsSousEtape = 'encore' }
        break
      case 'petit_enfant':
        ok = await lier(descCourant!.rattId, res.rattId, genreVersTypeLien(descCourant!.genre))
        if (ok) { descSousEtape = 'encore' }
        break
      case 'ctx_parent':
        ok = await lier(res.rattId, cibleCtx!.rattId, genreVersTypeLien(form.genre))
        break
      case 'ctx_enfant':
        ok = await lier(cibleCtx!.rattId, res.rattId, genreVersTypeLien(cibleCtx!.genre))
        break
      case 'ctx_conjoint':
        ok = await lier(cibleCtx!.rattId, res.rattId, 'conjoint')
        break
    }

    if (!ok) {
      // Le lien a échoué : on retire la personne orpheline pour ne pas polluer l'arbre.
      try { await supprimerPersonne(res.personneId) } catch { /* best-effort */ }
      enCours.value = false
      return // on garde le prompt actif pour permettre une nouvelle tentative
    }

    pousserUser(labelDe(form))
    pousserBot(`${labelDe(form)} a rejoint l’arbre.`, 'circle-check')
    options.onArbreModifie?.()
    enCours.value = false

    // En mode contextuel, on repropose le menu pour la même personne ; sinon onboarding.
    if (cle === 'ctx_parent' || cle === 'ctx_enfant' || cle === 'ctx_conjoint') {
      definirMenu(`Souhaitez-vous ajouter autre chose à ${cibleCtx?.label ?? 'cette personne'} ?`)
    } else {
      avancer()
    }
  }

  /** L'utilisateur passe une saisie facultative (« Je ne sais pas »). */
  const passer = () => {
    if (enCours.value || !promptActif.value || promptActif.value.type !== 'saisie') return
    if (!promptActif.value.permetPasser) return
    const cle = promptActif.value.cle
    pousserUser('Je ne sais pas')
    erreur.value = null

    switch (cle) {
      case 'pere': pereFait = true; break
      case 'mere': mereFait = true; break
      case 'gp_pere': ascSousEtape = 'mere'; break
      case 'gp_mere': ascCourant = null; break
    }
    avancer()
  }

  /** Réponse à une question fermée (Oui / Non). */
  const repondre = (oui: boolean) => {
    if (enCours.value || !promptActif.value || promptActif.value.type !== 'question') return
    const { cle, ouiLabel, nonLabel } = promptActif.value
    pousserUser(oui ? ouiLabel : nonLabel)
    erreur.value = null

    switch (cle) {
      case 'asc_gate':
        if (oui) ascSousEtape = 'pere'
        else ascCourant = null
        break
      case 'enfants_gate':
      case 'enfants_encore':
        enfantsSousEtape = oui ? 'saisie' : 'fini'
        break
      case 'desc_gate':
      case 'desc_encore':
        if (oui) descSousEtape = 'saisie'
        else descCourant = null
        break
    }
    avancer()
  }

  /** (Re)démarre l'assistant depuis le début. */
  const reinitialiser = () => {
    messages.value = []
    promptActif.value = null
    promptSeq.value = 0
    enCours.value = false
    erreur.value = null
    termine.value = false
    phase.value = 'moi'
    moiRattId = null
    moiGenre = undefined
    moiLabel = 'vous'
    pereFait = false
    mereFait = false
    fileAscendants.value = []
    ascCourant = null
    ascSousEtape = 'gate'
    enfantsSousEtape = 'gate'
    nbEnfants = 0
    fileDescendants.value = []
    descCourant = null
    descSousEtape = 'gate'
    compteurMessage = 0
    flux = 'onboarding'
    cibleCtx = null
    repriseOnboarding = false
  }

  /** Démarre la conversation (idempotent : ne relance pas si déjà commencée). */
  const demarrer = () => {
    if (messages.value.length > 0) return
    avancer()
  }

  return {
    // état
    messages,
    promptActif,
    promptSeq,
    enCours,
    erreur,
    termine,
    phaseLabel,
    // actions
    demarrer,
    reinitialiser,
    soumettrePersonne,
    passer,
    repondre,
    // flux contextuel
    demarrerContexte,
    choisirAction,
  }
}

// ════════════════════════════════════════════════════════════════════════════
// Composable — Layout de l'arbre généalogique
// Conversion données API → positions vue-flow via graphe de parenté
// ════════════════════════════════════════════════════════════════════════════

import type { Node, Edge } from '@vue-flow/core'
import type { PersonneNoeud, LienArbreResponse, ModeVue } from '~/mocks/arbre-genealogique'

// ─── Types internes ──────────────────────────────────────────────────────

export interface NoeudArbre {
  id: string // rattachement_id
  personne_id: string
  nom: string
  prenoms?: string
  genre: string
  naissance?: { annee?: number; mois?: number; jour?: number }
  deces?: { annee?: number; mois?: number; jour?: number }
  naissance_lieu?: string
  deces_lieu?: string
  photo_url?: string
  parents: string[]
  enfants: string[]
  conjoints: string[]
  generation: number
}

// ─── Construction du graphe d'adjacence ─────────────────────────────────

function construireGraphe(
  personnes: PersonneNoeud[],
  liens: LienArbreResponse[],
): Map<string, NoeudArbre> {
  const graphe = new Map<string, NoeudArbre>()

  for (const p of personnes) {
    graphe.set(p.rattachement_id, {
      id: p.rattachement_id,
      personne_id: p.id,
      nom: p.nom,
      prenoms: p.prenoms ?? undefined,
      genre: p.genre,
      naissance: p.naissance ?? undefined,
      deces: p.deces ?? undefined,
      naissance_lieu: p.naissance_lieu ?? undefined,
      deces_lieu: p.deces_lieu ?? undefined,
      photo_url: p.photo_url ?? undefined,
      parents: [],
      enfants: [],
      conjoints: [],
      generation: 0,
    })
  }

  for (const lien of liens) {
    const source = graphe.get(lien.rattachement_source_id)
    const cible = graphe.get(lien.rattachement_cible_id)
    if (!source || !cible) continue

    if (lien.type_lien === 'conjoint') {
      if (!source.conjoints.includes(cible.id)) source.conjoints.push(cible.id)
      if (!cible.conjoints.includes(source.id)) cible.conjoints.push(source.id)
    } else {
      // pere / mere / parent → source est parent de cible
      if (!source.enfants.includes(cible.id)) source.enfants.push(cible.id)
      if (!cible.parents.includes(source.id)) cible.parents.push(source.id)
    }
  }

  return graphe
}

// ─── Calcul des générations par BFS ─────────────────────────────────────

function calculerGenerations(graphe: Map<string, NoeudArbre>, centreId: string): void {
  const centre = graphe.get(centreId)
  if (!centre) return

  // Reset toutes les générations
  for (const noeud of graphe.values()) noeud.generation = 0

  centre.generation = 0
  const visite = new Set<string>([centreId])
  const file: string[] = [centreId]

  while (file.length > 0) {
    const courantId = file.shift()!
    const courant = graphe.get(courantId)!

    // Parents = génération - 1
    for (const parentId of courant.parents) {
      if (!visite.has(parentId)) {
        visite.add(parentId)
        const parent = graphe.get(parentId)!
        parent.generation = courant.generation - 1
        file.push(parentId)
      }
    }

    // Enfants = génération + 1
    for (const enfantId of courant.enfants) {
      if (!visite.has(enfantId)) {
        visite.add(enfantId)
        const enfant = graphe.get(enfantId)!
        enfant.generation = courant.generation + 1
        file.push(enfantId)
      }
    }

    // Conjoints = même génération
    for (const conjointId of courant.conjoints) {
      if (!visite.has(conjointId)) {
        visite.add(conjointId)
        const conjoint = graphe.get(conjointId)!
        conjoint.generation = courant.generation
        file.push(conjointId)
      }
    }
  }
}

// ─── Filtrage par mode ──────────────────────────────────────────────────

function filtrerNoeuds(
  graphe: Map<string, NoeudArbre>,
  centreId: string,
  mode: ModeVue,
): Set<string> {
  const ids = new Set<string>()

  if (mode === 'ascendant') {
    // DFS remontant via parents
    const pile = [centreId]
    while (pile.length > 0) {
      const id = pile.pop()!
      if (ids.has(id)) continue
      ids.add(id)
      const noeud = graphe.get(id)
      if (noeud) {
        for (const parentId of noeud.parents) pile.push(parentId)
      }
    }
  } else if (mode === 'descendant') {
    // DFS descendant via enfants
    const pile = [centreId]
    while (pile.length > 0) {
      const id = pile.pop()!
      if (ids.has(id)) continue
      ids.add(id)
      const noeud = graphe.get(id)
      if (noeud) {
        for (const enfantId of noeud.enfants) pile.push(enfantId)
      }
    }
  } else {
    // Mode complet : 3 générations autour du centre (BFS limité)
    const file: { id: string; profondeur: number }[] = [{ id: centreId, profondeur: 0 }]
    while (file.length > 0) {
      const { id, profondeur } = file.shift()!
      if (ids.has(id)) continue
      ids.add(id)

      if (profondeur < 3) {
        const noeud = graphe.get(id)
        if (noeud) {
          for (const parentId of noeud.parents) {
            if (!ids.has(parentId)) file.push({ id: parentId, profondeur: profondeur + 1 })
          }
          for (const enfantId of noeud.enfants) {
            if (!ids.has(enfantId)) file.push({ id: enfantId, profondeur: profondeur + 1 })
          }
          for (const conjointId of noeud.conjoints) {
            if (!ids.has(conjointId)) file.push({ id: conjointId, profondeur: profondeur + 1 })
          }
        }
      }
    }
  }

  return ids
}

// ─── Calcul du layout (positionnement) ──────────────────────────────────

// Médaillons en portrait (cercle photo + label) → plus étroits et plus hauts
const NOEUD_LARGEUR = 150
const NOEUD_HAUTEUR = 130
const ESPACEMENT_H = 36
const ESPACEMENT_V = 110

function calculerPositions(
  graphe: Map<string, NoeudArbre>,
  noeudsVisibles: Set<string>,
): Map<string, { x: number; y: number }> {
  const positions = new Map<string, { x: number; y: number }>()

  // Grouper par génération
  const parGeneration = new Map<number, NoeudArbre[]>()
  for (const id of noeudsVisibles) {
    const noeud = graphe.get(id)
    if (!noeud) continue
    const gen = noeud.generation
    if (!parGeneration.has(gen)) parGeneration.set(gen, [])
    parGeneration.get(gen)!.push(noeud)
  }

  // Trier les générations
  const generations = [...parGeneration.keys()].sort((a, b) => a - b)

  for (const gen of generations) {
    const noeuds = parGeneration.get(gen)!
    const y = gen * (NOEUD_HAUTEUR + ESPACEMENT_V)
    const largeurTotale = noeuds.length * NOEUD_LARGEUR + (noeuds.length - 1) * ESPACEMENT_H
    const debutX = -largeurTotale / 2

    // Placer les conjoints côte à côte
    const places = new Set<string>()
    let indexX = 0

    for (const noeud of noeuds) {
      if (places.has(noeud.id)) continue

      const x = debutX + indexX * (NOEUD_LARGEUR + ESPACEMENT_H)
      positions.set(noeud.id, { x, y })
      places.add(noeud.id)
      indexX++

      // Placer les conjoints juste à côté
      for (const conjointId of noeud.conjoints) {
        if (noeudsVisibles.has(conjointId) && !places.has(conjointId)) {
          const xConj = debutX + indexX * (NOEUD_LARGEUR + ESPACEMENT_H)
          positions.set(conjointId, { x: xConj, y })
          places.add(conjointId)
          indexX++
        }
      }
    }
  }

  return positions
}

// ─── Conversion en nodes/edges vue-flow ─────────────────────────────────

// Petit nœud-jonction matérialisant l'union d'un couple parental
const UNION_TAILLE = 10
const COULEUR_BRANCHE = '#8a5a2b'
const STYLE_BRANCHE = { stroke: COULEUR_BRANCHE, strokeWidth: 2.5 }

// ─── Unions parentales + branches de descendance ─────────────────────────
// La BDD ne stocke que des liens individuels (père→enfant, mère→enfant).
// On dérive ici le « couple » : les enfants partageant le même ensemble de
// parents descendent d'un nœud-jonction commun (point d'union), ce qui rend
// visible qu'ils sont issus du couple — et non d'un seul parent.

function construireUnionsEtLiens(
  graphe: Map<string, NoeudArbre>,
  noeudsVisibles: Set<string>,
  positions: Map<string, { x: number; y: number }>,
  liens: LienArbreResponse[],
): { nodes: Node[]; edges: Edge[] } {
  const nodes: Node[] = []
  const edges: Edge[] = []

  // 1. Unions conjugales = barre horizontale au niveau des médaillons
  for (const lien of liens) {
    if (lien.type_lien !== 'conjoint') continue
    const s = lien.rattachement_source_id
    const t = lien.rattachement_cible_id
    if (!noeudsVisibles.has(s) || !noeudsVisibles.has(t)) continue

    const sourceAGauche = (positions.get(s)?.x ?? 0) <= (positions.get(t)?.x ?? 0)
    edges.push({
      id: lien.id,
      source: s,
      target: t,
      sourceHandle: sourceAGauche ? 'd-source' : 'g-source',
      targetHandle: sourceAGauche ? 'g-target' : 'd-target',
      type: 'straight',
      style: { stroke: '#228B22', strokeWidth: 2, strokeDasharray: '5 4' },
      animated: false,
    })
  }

  // 2. Regrouper les enfants par ensemble de parents visibles
  const groupes = new Map<string, { parents: string[]; enfants: string[] }>()
  for (const id of noeudsVisibles) {
    const noeud = graphe.get(id)
    if (!noeud) continue
    let parentsVisibles = noeud.parents.filter(p => noeudsVisibles.has(p)).sort()
    if (parentsVisibles.length === 0) continue

    // Présomption de couple : un enfant rattaché à un seul parent dont le
    // conjoint (unique) est visible est présumé issu du couple. Sa descendance
    // part alors de la jonction du couple, et non d'un seul parent.
    if (parentsVisibles.length === 1) {
      const parent = graphe.get(parentsVisibles[0]!)
      const conjointsVisibles = (parent?.conjoints ?? []).filter(c => noeudsVisibles.has(c))
      if (conjointsVisibles.length === 1) {
        parentsVisibles = [parentsVisibles[0]!, conjointsVisibles[0]!].sort()
      }
    }

    const cle = parentsVisibles.join('|')
    if (!groupes.has(cle)) groupes.set(cle, { parents: parentsVisibles, enfants: [] })
    groupes.get(cle)!.enfants.push(id)
  }

  // 3. Branches de descendance
  for (const [cle, grp] of groupes) {
    if (grp.parents.length >= 2) {
      // Couple → nœud-jonction commun, centré sous les parents
      const centresX = grp.parents.map(p => (positions.get(p)?.x ?? 0) + NOEUD_LARGEUR / 2)
      const xMoy = centresX.reduce((a, b) => a + b, 0) / centresX.length
      const yParent = Math.max(...grp.parents.map(p => positions.get(p)?.y ?? 0))
      const unionId = `union-${cle}`

      nodes.push({
        id: unionId,
        type: 'union',
        position: { x: xMoy - UNION_TAILLE / 2, y: yParent + NOEUD_HAUTEUR + 28 },
        data: {},
        selectable: false,
        draggable: false,
        focusable: false,
      })

      // Chaque parent → jonction (les deux branches convergent)
      for (const p of grp.parents) {
        edges.push({
          id: `br-${p}-${unionId}`,
          source: p,
          target: unionId,
          sourceHandle: 'bas',
          targetHandle: 'union-haut',
          type: 'smoothstep',
          style: STYLE_BRANCHE,
          pathOptions: { borderRadius: 20 },
          animated: false,
        })
      }

      // Jonction → chaque enfant (les branches repartent du couple)
      for (const e of grp.enfants) {
        edges.push({
          id: `br-${unionId}-${e}`,
          source: unionId,
          target: e,
          sourceHandle: 'union-bas',
          targetHandle: 'haut',
          type: 'smoothstep',
          style: STYLE_BRANCHE,
          pathOptions: { borderRadius: 20 },
          animated: false,
        })
      }
    } else {
      // Parent unique connu → branche directe
      const p = grp.parents[0]
      for (const e of grp.enfants) {
        edges.push({
          id: `br-${p}-${e}`,
          source: p,
          target: e,
          sourceHandle: 'bas',
          targetHandle: 'haut',
          type: 'smoothstep',
          style: STYLE_BRANCHE,
          pathOptions: { borderRadius: 20 },
          animated: false,
        })
      }
    }
  }

  return { nodes, edges }
}

function convertirEnVueFlow(
  graphe: Map<string, NoeudArbre>,
  noeudsVisibles: Set<string>,
  positions: Map<string, { x: number; y: number }>,
  liens: LienArbreResponse[],
): { nodes: Node[]; edges: Edge[] } {
  const nodes: Node[] = []

  for (const id of noeudsVisibles) {
    const noeud = graphe.get(id)
    const pos = positions.get(id)
    if (!noeud || !pos) continue

    nodes.push({
      id: noeud.id,
      type: 'personne',
      position: pos,
      data: { ...noeud },
    })
  }

  const { nodes: unionNodes, edges } = construireUnionsEtLiens(graphe, noeudsVisibles, positions, liens)
  nodes.push(...unionNodes)

  return { nodes, edges }
}

// ─── Calcul d'incomplétude ───────────────────────────────────────────────

export interface InfoIncompletude {
  estIncomplet: boolean
  messageManquant: string | null
}

function calculerIncompletude(
  graphe: Map<string, NoeudArbre>,
  liens: LienArbreResponse[],
): Map<string, InfoIncompletude> {
  const resultat = new Map<string, InfoIncompletude>()

  // Construire un index type_lien par rattachement_cible (pour savoir si père/mère)
  const typesParentsParCible = new Map<string, string[]>()
  for (const lien of liens) {
    if (lien.type_lien !== 'conjoint') {
      if (!typesParentsParCible.has(lien.rattachement_cible_id)) {
        typesParentsParCible.set(lien.rattachement_cible_id, [])
      }
      typesParentsParCible.get(lien.rattachement_cible_id)!.push(lien.type_lien)
    }
  }

  for (const [id, noeud] of graphe) {
    const nbParents = noeud.parents.length
    if (nbParents >= 2) {
      resultat.set(id, { estIncomplet: false, messageManquant: null })
    } else if (nbParents === 0) {
      resultat.set(id, { estIncomplet: true, messageManquant: 'Parents manquants' })
    } else {
      // 1 parent — déterminer lequel manque
      const types = typesParentsParCible.get(id) || []
      const aPere = types.includes('pere')
      const aMere = types.includes('mere')
      let message = 'Parent manquant'
      if (aPere && !aMere) message = 'Mère manquante'
      else if (aMere && !aPere) message = 'Père manquant'
      resultat.set(id, { estIncomplet: true, messageManquant: message })
    }
  }

  return resultat
}

function compterBranchesIncompletes(graphe: Map<string, NoeudArbre>, liens: LienArbreResponse[]): number {
  const info = calculerIncompletude(graphe, liens)
  let compte = 0
  for (const v of info.values()) {
    if (v.estIncomplet) compte++
  }
  return compte
}

// ─── Composable public ──────────────────────────────────────────────────

export const useLayoutArbre = () => {
  const calculerLayout = (
    personnes: PersonneNoeud[],
    liens: LienArbreResponse[],
    centreId: string,
    mode: ModeVue = 'complet',
  ): { nodes: Node[]; edges: Edge[]; graphe: Map<string, NoeudArbre> } => {
    const graphe = construireGraphe(personnes, liens)
    calculerGenerations(graphe, centreId)
    const noeudsVisibles = filtrerNoeuds(graphe, centreId, mode)
    const positions = calculerPositions(graphe, noeudsVisibles)
    const { nodes, edges } = convertirEnVueFlow(graphe, noeudsVisibles, positions, liens)
    return { nodes, edges, graphe }
  }

  const filtrerAncetres = (graphe: Map<string, NoeudArbre>, centreId: string): Set<string> => {
    return filtrerNoeuds(graphe, centreId, 'ascendant')
  }

  const filtrerDescendants = (graphe: Map<string, NoeudArbre>, centreId: string): Set<string> => {
    return filtrerNoeuds(graphe, centreId, 'descendant')
  }

  return {
    calculerLayout,
    filtrerAncetres,
    filtrerDescendants,
    calculerIncompletude,
    compterBranchesIncompletes,
  }
}

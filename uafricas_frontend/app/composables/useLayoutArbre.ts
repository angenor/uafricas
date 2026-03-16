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

const NOEUD_LARGEUR = 200
const NOEUD_HAUTEUR = 100
const ESPACEMENT_H = 60
const ESPACEMENT_V = 120

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

function convertirEnVueFlow(
  graphe: Map<string, NoeudArbre>,
  noeudsVisibles: Set<string>,
  positions: Map<string, { x: number; y: number }>,
  liens: LienArbreResponse[],
): { nodes: Node[]; edges: Edge[] } {
  const nodes: Node[] = []
  const edges: Edge[] = []

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

  for (const lien of liens) {
    if (!noeudsVisibles.has(lien.rattachement_source_id) || !noeudsVisibles.has(lien.rattachement_cible_id)) {
      continue
    }

    edges.push({
      id: lien.id,
      source: lien.rattachement_source_id,
      target: lien.rattachement_cible_id,
      type: lien.type_lien === 'conjoint' ? 'straight' : 'smoothstep',
      style: lien.type_lien === 'conjoint'
        ? { stroke: '#A54A1C', strokeWidth: 2, strokeDasharray: '6 3' }
        : { stroke: '#228B22', strokeWidth: 2 },
      animated: false,
    })
  }

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

// ════════════════════════════════════════════════════════════════════════════
// Composable : Recherche et exploration de l'arbre
// Recherche locale, chemin de parenté (BFS/LCA), filtres, terminologie
// ════════════════════════════════════════════════════════════════════════════

import type { NoeudArbre } from '~/composables/useLayoutArbre'

// ─── Types ──────────────────────────────────────────────────────────────

export interface ResultatRecherche {
  id: string // rattachement_id
  personne_id: string
  nom: string
  prenoms?: string
  naissance_annee?: number
  naissance_lieu?: string
  genre?: string
  source: 'mon_arbre' | 'autre_arbre'
}

export interface CheminParente {
  source: NoeudArbre
  cible: NoeudArbre
  noeuds: NoeudArbre[]
  liensIds: string[]
  description: string
  generationsMontantes: number
  generationsDescendantes: number
  lca: NoeudArbre | null
}

export interface FiltreArbre {
  type: 'geographique' | 'generationnel' | 'branche'
  valeur: string | number | { parentId: string }
  actif: boolean
}

// ─── Normalisation pour recherche ───────────────────────────────────────

function normaliserPourRecherche(texte: string): string {
  return texte
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '') // supprime diacritiques
    .trim()
}

// ─── Recherche locale ───────────────────────────────────────────────────

function rechercherLocal(
  graphe: Map<string, NoeudArbre>,
  terme: string,
): ResultatRecherche[] {
  if (!terme || terme.length < 2) return []

  const termeNorm = normaliserPourRecherche(terme)
  const resultats: ResultatRecherche[] = []

  for (const noeud of graphe.values()) {
    const nomNorm = normaliserPourRecherche(noeud.nom)
    const prenomsNorm = normaliserPourRecherche(noeud.prenoms ?? '')
    const lieuNorm = normaliserPourRecherche(noeud.naissance_lieu ?? '')
    const annee = noeud.naissance?.annee?.toString() ?? ''

    if (
      nomNorm.includes(termeNorm) ||
      prenomsNorm.includes(termeNorm) ||
      lieuNorm.includes(termeNorm) ||
      annee.includes(termeNorm)
    ) {
      resultats.push({
        id: noeud.id,
        personne_id: noeud.personne_id,
        nom: noeud.nom,
        prenoms: noeud.prenoms,
        naissance_annee: noeud.naissance?.annee,
        naissance_lieu: noeud.naissance_lieu,
        genre: noeud.genre,
        source: 'mon_arbre',
      })
    }

    if (resultats.length >= 20) break
  }

  return resultats
}

// ─── Chemin de parenté (BFS bidirectionnel + LCA) ───────────────────────

function calculerCheminParente(
  graphe: Map<string, NoeudArbre>,
  sourceId: string,
  cibleId: string,
): CheminParente | null {
  if (sourceId === cibleId) return null

  const source = graphe.get(sourceId)
  const cible = graphe.get(cibleId)
  if (!source || !cible) return null

  // BFS pour trouver le plus court chemin (traite parents, enfants, conjoints)
  const predecesseur = new Map<string, string>()
  const visite = new Set<string>()
  const file: string[] = [sourceId]
  visite.add(sourceId)

  let trouve = false

  while (file.length > 0 && !trouve) {
    const courantId = file.shift()!
    const courant = graphe.get(courantId)
    if (!courant) continue

    const voisins = [...courant.parents, ...courant.enfants, ...courant.conjoints]

    for (const voisinId of voisins) {
      if (visite.has(voisinId)) continue
      visite.add(voisinId)
      predecesseur.set(voisinId, courantId)

      if (voisinId === cibleId) {
        trouve = true
        break
      }

      file.push(voisinId)
    }
  }

  if (!trouve) return null

  // Reconstruire le chemin
  const cheminIds: string[] = [cibleId]
  let courant = cibleId
  while (courant !== sourceId) {
    courant = predecesseur.get(courant)!
    cheminIds.unshift(courant)
  }

  const noeuds = cheminIds.map((id) => graphe.get(id)!).filter(Boolean)

  // Calculer les générations montantes/descendantes pour la terminologie
  // Trouver le LCA en analysant les directions du chemin
  let genMontantes = 0
  let genDescendantes = 0
  let phase: 'montee' | 'descente' = 'montee'
  let lcaIndex = 0

  for (let i = 0; i < cheminIds.length - 1; i++) {
    const a = graphe.get(cheminIds[i])!
    const b = graphe.get(cheminIds[i + 1])!

    if (a.enfants.includes(b.id) || a.conjoints.includes(b.id)) {
      // A → B : A est parent de B (descente) ou conjoint
      if (phase === 'montee') {
        phase = 'descente'
        lcaIndex = i
      }
      if (!a.conjoints.includes(b.id)) genDescendantes++
    } else if (a.parents.includes(b.id)) {
      // A → B : B est parent de A (montée)
      genMontantes++
    }
  }

  const lca = graphe.get(cheminIds[lcaIndex]) || null
  const description = decrireRelation(genMontantes, genDescendantes, source.genre, cible)

  return {
    source,
    cible,
    noeuds,
    liensIds: [],
    description,
    generationsMontantes: genMontantes,
    generationsDescendantes: genDescendantes,
    lca,
  }
}

// ─── Terminologie familiale française ───────────────────────────────────

function decrireRelation(
  genMontantes: number,
  genDescendantes: number,
  genreSource: string,
  cible: NoeudArbre,
): string {
  const estHomme = genreSource === 'masculin'
  const cibleNom = cible.prenoms ? `${cible.prenoms} ${cible.nom}` : cible.nom

  // Relations directes
  if (genMontantes === 0 && genDescendantes === 0) return `Même personne`
  if (genMontantes === 1 && genDescendantes === 0) return estHomme ? `Père de ${cibleNom}` : `Mère de ${cibleNom}`
  if (genMontantes === 0 && genDescendantes === 1) return `Enfant de ${cibleNom}`
  if (genMontantes === 2 && genDescendantes === 0) return estHomme ? `Grand-père de ${cibleNom}` : `Grand-mère de ${cibleNom}`
  if (genMontantes === 0 && genDescendantes === 2) return `Petit-enfant de ${cibleNom}`
  if (genMontantes === 3 && genDescendantes === 0) return estHomme ? `Arrière-grand-père de ${cibleNom}` : `Arrière-grand-mère de ${cibleNom}`
  if (genMontantes === 0 && genDescendantes === 3) return `Arrière-petit-enfant de ${cibleNom}`

  // Frères/sœurs
  if (genMontantes === 1 && genDescendantes === 1) return estHomme ? `Frère de ${cibleNom}` : `Sœur de ${cibleNom}`

  // Oncles/tantes/neveux/nièces
  if (genMontantes === 2 && genDescendantes === 1) return estHomme ? `Oncle de ${cibleNom}` : `Tante de ${cibleNom}`
  if (genMontantes === 1 && genDescendantes === 2) return estHomme ? `Neveu de ${cibleNom}` : `Nièce de ${cibleNom}`

  // Grands-oncles
  if (genMontantes === 3 && genDescendantes === 1) return estHomme ? `Grand-oncle de ${cibleNom}` : `Grand-tante de ${cibleNom}`
  if (genMontantes === 1 && genDescendantes === 3) return `Petit-neveu/nièce de ${cibleNom}`

  // Cousins
  if (genMontantes === genDescendantes && genMontantes >= 2) {
    const degre = genMontantes - 1
    if (degre === 1) return estHomme ? `Cousin de ${cibleNom}` : `Cousine de ${cibleNom}`
    return estHomme ? `Cousin au ${degre}ème degré de ${cibleNom}` : `Cousine au ${degre}ème degré de ${cibleNom}`
  }

  // Cas génériques
  if (genMontantes > genDescendantes) {
    return `Ancêtre de ${cibleNom} (${genMontantes} générations)`
  }
  return `Descendant de ${cibleNom} (${genDescendantes} générations)`
}

// ─── Filtres ────────────────────────────────────────────────────────────

function filtrerParLieu(
  graphe: Map<string, NoeudArbre>,
  terme: string,
): Set<string> {
  const termeNorm = normaliserPourRecherche(terme)
  const ids = new Set<string>()
  for (const [id, noeud] of graphe) {
    const lieuNaissance = normaliserPourRecherche(noeud.naissance_lieu ?? '')
    const lieuDeces = normaliserPourRecherche(noeud.deces_lieu ?? '')
    if (lieuNaissance.includes(termeNorm) || lieuDeces.includes(termeNorm)) {
      ids.add(id)
    }
  }
  return ids
}

function filtrerParGeneration(
  graphe: Map<string, NoeudArbre>,
  centreId: string,
  plage: number,
): Set<string> {
  const centre = graphe.get(centreId)
  if (!centre) return new Set()
  const ids = new Set<string>()
  for (const [id, noeud] of graphe) {
    if (Math.abs(noeud.generation - centre.generation) <= plage) {
      ids.add(id)
    }
  }
  return ids
}

function filtrerParBranche(
  graphe: Map<string, NoeudArbre>,
  centreId: string,
  parentId: string,
): Set<string> {
  // DFS depuis le parent sélectionné, uniquement vers les ancêtres
  const ids = new Set<string>()
  ids.add(centreId)

  // Remonter via le parent sélectionné
  const pile = [parentId]
  while (pile.length > 0) {
    const id = pile.pop()!
    if (ids.has(id)) continue
    ids.add(id)
    const noeud = graphe.get(id)
    if (noeud) {
      for (const pId of noeud.parents) pile.push(pId)
      for (const cId of noeud.conjoints) ids.add(cId) // inclure conjoints
    }
  }

  // Ajouter aussi les descendants du centre
  const pileDesc = [centreId]
  const visiteDesc = new Set<string>()
  while (pileDesc.length > 0) {
    const id = pileDesc.pop()!
    if (visiteDesc.has(id)) continue
    visiteDesc.add(id)
    ids.add(id)
    const noeud = graphe.get(id)
    if (noeud) {
      for (const eId of noeud.enfants) pileDesc.push(eId)
    }
  }

  return ids
}

// ─── Composable public ──────────────────────────────────────────────────

export const useRechercheArbre = () => {
  return {
    rechercherLocal,
    calculerCheminParente,
    decrireRelation,
    filtrerParLieu,
    filtrerParGeneration,
    filtrerParBranche,
  }
}

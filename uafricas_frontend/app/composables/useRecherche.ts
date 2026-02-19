// Composable pour la recherche globale sur toutes les donnees de la plateforme

import { evenementsMock } from '~/mocks/evenements'
import { annoncesMock } from '~/mocks/marche-africain'
import { documentsNumeriques } from '~/mocks/bibliotheques'
import { expertsMock } from '~/mocks/experts'
import { projetsMock } from '~/mocks/projets'
import { forumsMock } from '~/mocks/forums'
import { centresCulturelsMock } from '~/mocks/centres-culturels'
import { radioStations } from '~/mocks/radios'
import { tvChannels } from '~/mocks/tele'
import { sabbatiquesMock } from '~/mocks/sabbatiques'
import { paysAfricainsMock } from '~/mocks/opportunite-afrique'

// ── Interfaces ──────────────────────────────────────────────────

export interface ResultatRecherche {
  id: string
  titre: string
  sousTexte?: string
  categorie: string
  icone: string
  lien: string
  score: number
}

export interface GroupeResultats {
  categorie: string
  icone: string
  resultats: ResultatRecherche[]
}

// ── Constantes ──────────────────────────────────────────────────

const CLE_RECHERCHES_RECENTES = 'uafricas_recherches_recentes'
const MAX_RECHERCHES_RECENTES = 5

// ── Normalisation (insensible aux accents) ──────────────────────

const normaliser = (texte: string): string => {
  return texte
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .trim()
}

// ── Score de pertinence ─────────────────────────────────────────

const calculerScore = (texte: string, terme: string): number => {
  const texteNorm = normaliser(texte)
  const termeNorm = normaliser(terme)

  if (texteNorm === termeNorm) return 100
  if (texteNorm.startsWith(termeNorm)) return 80
  if (texteNorm.includes(termeNorm)) return 60

  const mots = termeNorm.split(/\s+/)
  const motsCorrespondants = mots.filter(m => texteNorm.includes(m))
  if (motsCorrespondants.length > 0) {
    return 40 * (motsCorrespondants.length / mots.length)
  }

  return 0
}

// ── Construction de l'index ─────────────────────────────────────

const construireIndex = (): ResultatRecherche[] => {
  const index: ResultatRecherche[] = []

  // Evenements
  for (const e of evenementsMock) {
    index.push({
      id: `evt-${e.id}`,
      titre: e.titre,
      sousTexte: `${e.type} · ${e.pays}${e.ville ? ', ' + e.ville : ''}`,
      categorie: 'Événements',
      icone: 'fa-solid fa-calendar',
      lien: `/evenements/${e.id}`,
      score: 0,
    })
  }

  // Annonces (Marche Africain)
  for (const a of annoncesMock) {
    index.push({
      id: `ann-${a.id}`,
      titre: a.titre,
      sousTexte: `${a.categorie} · ${a.type_echange} · ${a.pays}`,
      categorie: 'Marché Africain',
      icone: 'fa-solid fa-store',
      lien: `/marche-africain/${a.id}`,
      score: 0,
    })
  }

  // Bibliotheque
  for (const d of documentsNumeriques) {
    index.push({
      id: `bib-${d.id}`,
      titre: d.titre,
      sousTexte: `${d.type} · ${d.user.prenom} ${d.user.nom}`,
      categorie: 'Bibliothèque',
      icone: 'fa-solid fa-book',
      lien: '/bibliotheque/numerique',
      score: 0,
    })
  }

  // Experts
  for (const exp of expertsMock) {
    index.push({
      id: `exp-${exp.id}`,
      titre: `${exp.prenom} ${exp.nom}`,
      sousTexte: `${exp.expertiseInfo.domaine} · ${exp.pays}`,
      categorie: 'Experts',
      icone: 'fa-solid fa-user-tie',
      lien: `/experts/${exp.id}`,
      score: 0,
    })
  }

  // Projets
  for (const p of projetsMock) {
    index.push({
      id: `prj-${p.id}`,
      titre: p.titre,
      sousTexte: `${p.organisation} · ${p.pays}`,
      categorie: 'Projets',
      icone: 'fa-solid fa-rocket',
      lien: '/financer-projet',
      score: 0,
    })
  }

  // Forums
  for (const f of forumsMock) {
    const contenu = f.type === 'Proverbe' ? f.proverbe.libellet
      : f.type === 'Citation' ? f.citation.libellet
        : f.type === 'Bonne pratique' ? f.bonne_pratique.description
          : f.histoire.description
    if (!contenu) continue
    index.push({
      id: `for-${f.id}`,
      titre: contenu.length > 80 ? contenu.substring(0, 80) + '...' : contenu,
      sousTexte: `${f.type} · ${f.user.prenom} ${f.user.nom}`,
      categorie: 'Forums',
      icone: 'fa-solid fa-comments',
      lien: '/promotion-valeur',
      score: 0,
    })
  }

  // Centres culturels
  for (const cc of centresCulturelsMock) {
    index.push({
      id: `cc-${cc.id}`,
      titre: cc.nom,
      sousTexte: cc.adress,
      categorie: 'Centres Culturels',
      icone: 'fa-solid fa-masks-theater',
      lien: '/africa-culture',
      score: 0,
    })
  }

  // Radios
  for (const r of radioStations) {
    index.push({
      id: `rad-${r.id}`,
      titre: r.name,
      sousTexte: `${r.genre} · ${r.country}`,
      categorie: 'Radios',
      icone: 'fa-solid fa-radio',
      lien: '/radios',
      score: 0,
    })
  }

  // Chaines TV
  for (const tv of tvChannels) {
    index.push({
      id: `tv-${tv.id}`,
      titre: tv.name,
      sousTexte: `${tv.category} · ${tv.country}`,
      categorie: 'Télévision',
      icone: 'fa-solid fa-tv',
      lien: '/tele',
      score: 0,
    })
  }

  // Programmes sabbatiques
  for (const s of sabbatiquesMock) {
    index.push({
      id: `sab-${s.id}`,
      titre: s.titre,
      sousTexte: `${s.pays}${s.ville ? ', ' + s.ville : ''}`,
      categorie: 'Échanges Sabbatiques',
      icone: 'fa-solid fa-plane',
      lien: '/echanges-sabbatiques',
      score: 0,
    })
  }

  // Fiches pays
  for (const pays of paysAfricainsMock) {
    index.push({
      id: `pays-${pays.id}`,
      titre: pays.nom,
      sousTexte: `${pays.region} · Capitale : ${pays.capitale}`,
      categorie: 'Opportunités Afrique',
      icone: 'fa-solid fa-earth-africa',
      lien: `/opportunite-afrique/${pays.id}`,
      score: 0,
    })
  }

  return index
}

// ── Composable principal ────────────────────────────────────────

export const useRecherche = () => {
  const termeRecherche = ref('')
  const resultatsGroupes = ref<GroupeResultats[]>([])
  const enChargement = ref(false)
  const nombreTotalResultats = ref(0)

  // Index construit une seule fois
  const indexRecherche = construireIndex()

  // Recherches recentes (localStorage)
  const recherchesRecentes = ref<string[]>([])

  if (import.meta.client) {
    try {
      const stockees = localStorage.getItem(CLE_RECHERCHES_RECENTES)
      if (stockees) {
        recherchesRecentes.value = JSON.parse(stockees)
      }
    }
    catch {
      // Ignorer erreur localStorage
    }
  }

  const sauvegarderRecherche = (terme: string) => {
    if (!terme.trim()) return
    const recentes = recherchesRecentes.value.filter(r => r !== terme)
    recentes.unshift(terme)
    recherchesRecentes.value = recentes.slice(0, MAX_RECHERCHES_RECENTES)
    if (import.meta.client) {
      try {
        localStorage.setItem(CLE_RECHERCHES_RECENTES, JSON.stringify(recherchesRecentes.value))
      }
      catch {
        // Ignorer
      }
    }
  }

  const viderRecherchesRecentes = () => {
    recherchesRecentes.value = []
    if (import.meta.client) {
      localStorage.removeItem(CLE_RECHERCHES_RECENTES)
    }
  }

  const viderRecherche = () => {
    termeRecherche.value = ''
    resultatsGroupes.value = []
    nombreTotalResultats.value = 0
  }

  // Recherche
  const effectuerRecherche = (terme: string) => {
    if (!terme.trim() || terme.trim().length < 2) {
      resultatsGroupes.value = []
      nombreTotalResultats.value = 0
      enChargement.value = false
      return
    }

    const resultats: ResultatRecherche[] = []

    for (const item of indexRecherche) {
      const scoreTitre = calculerScore(item.titre, terme)
      const scoreSousTexte = item.sousTexte ? calculerScore(item.sousTexte, terme) * 0.5 : 0
      const scoreTotal = Math.max(scoreTitre, scoreSousTexte)

      if (scoreTotal > 0) {
        resultats.push({ ...item, score: scoreTotal })
      }
    }

    resultats.sort((a, b) => b.score - a.score)

    // Grouper par categorie
    const groupes = new Map<string, ResultatRecherche[]>()
    for (const r of resultats) {
      if (!groupes.has(r.categorie)) {
        groupes.set(r.categorie, [])
      }
      groupes.get(r.categorie)!.push(r)
    }

    resultatsGroupes.value = Array.from(groupes.entries())
      .map(([categorie, res]) => ({
        categorie,
        icone: res[0]?.icone || 'fa-solid fa-magnifying-glass',
        resultats: res,
      }))
      .sort((a, b) => (b.resultats[0]?.score || 0) - (a.resultats[0]?.score || 0))

    nombreTotalResultats.value = resultats.length
    enChargement.value = false
  }

  // Debounce 250ms
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  watch(termeRecherche, (nouveau) => {
    if (timeoutId) clearTimeout(timeoutId)

    if (!nouveau.trim() || nouveau.trim().length < 2) {
      resultatsGroupes.value = []
      nombreTotalResultats.value = 0
      enChargement.value = false
      return
    }

    enChargement.value = true
    timeoutId = setTimeout(() => {
      effectuerRecherche(nouveau)
    }, 250)
  })

  return {
    termeRecherche,
    resultatsGroupes,
    enChargement,
    nombreTotalResultats,
    recherchesRecentes,
    viderRecherche,
    viderRecherchesRecentes,
    sauvegarderRecherche,
  }
}

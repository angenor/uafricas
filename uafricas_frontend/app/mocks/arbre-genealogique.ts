// ════════════════════════════════════════════════════════════════════════════
// Mock — Arbre généalogique
// Interfaces TypeScript + données de test + helpers async
// Source de vérité : specs/001-personnes-arbre/data-model.md
// ════════════════════════════════════════════════════════════════════════════

// ─── Types ───────────────────────────────────────────────────────────────

export type Genre = 'masculin' | 'feminin' | 'autre' | 'non_precise'
export type TypeLien = 'pere' | 'mere' | 'parent' | 'conjoint'

// ─── Interfaces ──────────────────────────────────────────────────────────

export interface DatePartielle {
  annee?: number
  mois?: number
  jour?: number
}

export interface Personne {
  id: string
  nom: string
  prenoms?: string
  genre?: Genre
  naissance?: DatePartielle
  naissance_lieu?: string
  deces?: DatePartielle
  deces_lieu?: string
  photo_url?: string
  created_at: string
}

export interface LienResume {
  lien_id: string
  type_lien: TypeLien
  personne: Personne
}

export interface PersonneDetail {
  rattachement_id: string
  personne: Personne
  parents: LienResume[]
  enfants: LienResume[]
  conjoints: LienResume[]
}

export interface PersonneListe {
  personnes: Personne[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

export interface LienFamilial {
  id: string
  type_lien: TypeLien
  rattachement_source_id: string
  rattachement_cible_id: string
  created_at: string
}

// ─── Interfaces — Arbre complet (visualisation) ────────────────────────

export interface PersonneNoeud {
  id: string
  rattachement_id: string
  nom: string
  prenoms?: string
  genre: Genre
  naissance?: DatePartielle
  deces?: DatePartielle
  naissance_lieu?: string
  deces_lieu?: string
  photo_url?: string
}

export interface LienArbreResponse {
  id: string
  rattachement_source_id: string
  rattachement_cible_id: string
  type_lien: TypeLien
}

export interface ArbreComplet {
  arbre_id: string | null
  personnes: PersonneNoeud[]
  liens: LienArbreResponse[]
}

export type ModeVue = 'complet' | 'ascendant' | 'descendant'
export type ModePanneau = 'fiche' | 'ajout' | 'modifier'
export type TypeActionAjout = 'parent' | 'enfant' | 'conjoint'

export interface ContexteAjout {
  personneSourceId: string
  personneSourceNom: string
  typeAction: TypeActionAjout
  typeLienSuggere: TypeLien
}

/** Suggère le type de lien le plus approprié selon le contexte */
export const suggererTypeLien = (
  typeAction: TypeActionAjout,
  genreSource: string,
  typesParentsExistants: TypeLien[],
): TypeLien => {
  if (typeAction === 'conjoint') return 'conjoint'

  if (typeAction === 'enfant') {
    // L'utilisateur ajoute un enfant depuis le nœud source → source devient parent
    if (genreSource === 'masculin') return 'pere'
    if (genreSource === 'feminin') return 'mere'
    return 'parent'
  }

  // typeAction === 'parent' → on ajoute un parent au nœud source
  const aPere = typesParentsExistants.includes('pere')
  const aMere = typesParentsExistants.includes('mere')
  if (aPere && !aMere) return 'mere'
  if (aMere && !aPere) return 'pere'
  return 'parent'
}

// ─── DTOs (formulaires) ──────────────────────────────────────────────────

export interface CreerPersonneForm {
  nom: string
  prenoms?: string
  genre?: Genre
  naissance?: DatePartielle
  naissance_lieu?: string
  deces?: DatePartielle
  deces_lieu?: string
}

export interface ModifierPersonneForm {
  nom?: string
  prenoms?: string
  genre?: Genre
  naissance?: DatePartielle
  naissance_lieu?: string
  deces?: DatePartielle
  deces_lieu?: string
}

export interface CreerLienForm {
  rattachement_source_id: string
  rattachement_cible_id: string
  type_lien: TypeLien
}

export interface PersonneQueryParams {
  page?: number
  par_page?: number
  recherche?: string
}

// ─── Données mock ────────────────────────────────────────────────────────

const RATT_ID_1 = 'ratt-0000-0000-0000-000000000001'
const RATT_ID_2 = 'ratt-0000-0000-0000-000000000002'
const RATT_ID_3 = 'ratt-0000-0000-0000-000000000003'

const personnes: Personne[] = [
  {
    id: 'pers-0000-0000-0000-000000000001',
    nom: 'Diallo',
    prenoms: 'Ibrahim',
    genre: 'masculin',
    naissance: { annee: 1850 },
    naissance_lieu: 'Ségou, Mali',
    created_at: '2026-03-15T10:00:00Z',
  },
  {
    id: 'pers-0000-0000-0000-000000000002',
    nom: 'Diallo',
    prenoms: 'Ousmane Ibrahim',
    genre: 'masculin',
    naissance: { annee: 1882, mois: 3 },
    naissance_lieu: 'Ségou, Mali',
    deces: { annee: 1955, mois: 7, jour: 12 },
    deces_lieu: 'Bamako, Mali',
    created_at: '2026-03-15T10:05:00Z',
  },
  {
    id: 'pers-0000-0000-0000-000000000003',
    nom: 'Kouyaté',
    prenoms: 'Fatoumata',
    genre: 'feminin',
    naissance: { annee: 1885 },
    naissance_lieu: 'Kankan, Guinée',
    deces: { annee: 1960 },
    created_at: '2026-03-15T10:10:00Z',
  },
]

export const detailsMock: PersonneDetail[] = [
  {
    rattachement_id: RATT_ID_1,
    personne: personnes[0],
    parents: [],
    enfants: [
      {
        lien_id: 'lien-0000-0000-0000-000000000001',
        type_lien: 'pere',
        personne: personnes[1],
      },
    ],
    conjoints: [],
  },
  {
    rattachement_id: RATT_ID_2,
    personne: personnes[1],
    parents: [
      {
        lien_id: 'lien-0000-0000-0000-000000000001',
        type_lien: 'pere',
        personne: personnes[0],
      },
    ],
    enfants: [],
    conjoints: [
      {
        lien_id: 'lien-0000-0000-0000-000000000002',
        type_lien: 'conjoint',
        personne: personnes[2],
      },
    ],
  },
  {
    rattachement_id: RATT_ID_3,
    personne: personnes[2],
    parents: [],
    enfants: [],
    conjoints: [
      {
        lien_id: 'lien-0000-0000-0000-000000000002',
        type_lien: 'conjoint',
        personne: personnes[1],
      },
    ],
  },
]

// ─── Mock — Arbre complet (visualisation) ───────────────────────────

const RATT_ID_4 = 'ratt-0000-0000-0000-000000000004'
const RATT_ID_5 = 'ratt-0000-0000-0000-000000000005'
const RATT_ID_6 = 'ratt-0000-0000-0000-000000000006'
const RATT_ID_7 = 'ratt-0000-0000-0000-000000000007'
const RATT_ID_8 = 'ratt-0000-0000-0000-000000000008'

export const arbreCompletMock: ArbreComplet = {
  arbre_id: 'arbre-0000-0000-0000-000000000001',
  personnes: [
    // Génération 0 (grands-parents)
    { id: 'pers-0000-0000-0000-000000000001', rattachement_id: RATT_ID_1, nom: 'Diallo', prenoms: 'Ibrahim', genre: 'masculin', naissance: { annee: 1850 }, naissance_lieu: 'Ségou, Mali' },
    // Génération 1 (parents + conjoint)
    { id: 'pers-0000-0000-0000-000000000002', rattachement_id: RATT_ID_2, nom: 'Diallo', prenoms: 'Ousmane Ibrahim', genre: 'masculin', naissance: { annee: 1882, mois: 3 }, naissance_lieu: 'Ségou, Mali', deces: { annee: 1955, mois: 7, jour: 12 }, deces_lieu: 'Bamako, Mali' },
    { id: 'pers-0000-0000-0000-000000000003', rattachement_id: RATT_ID_3, nom: 'Kouyaté', prenoms: 'Fatoumata', genre: 'feminin', naissance: { annee: 1885 }, naissance_lieu: 'Kankan, Guinée', deces: { annee: 1960 } },
    // Génération 2 (enfants)
    { id: 'pers-0000-0000-0000-000000000004', rattachement_id: RATT_ID_4, nom: 'Diallo', prenoms: 'Aminata', genre: 'feminin', naissance: { annee: 1910 }, naissance_lieu: 'Bamako, Mali' },
    { id: 'pers-0000-0000-0000-000000000005', rattachement_id: RATT_ID_5, nom: 'Diallo', prenoms: 'Mamadou', genre: 'masculin', naissance: { annee: 1912 }, naissance_lieu: 'Bamako, Mali', deces: { annee: 1980 } },
    { id: 'pers-0000-0000-0000-000000000006', rattachement_id: RATT_ID_6, nom: 'Diallo', prenoms: 'Kadiatou', genre: 'feminin', naissance: { annee: 1915 }, naissance_lieu: 'Bamako, Mali' },
    // Conjoint de Aminata
    { id: 'pers-0000-0000-0000-000000000007', rattachement_id: RATT_ID_7, nom: 'Traoré', prenoms: 'Boubacar', genre: 'masculin', naissance: { annee: 1908 }, naissance_lieu: 'Sikasso, Mali' },
    // Enfant d'Aminata et Boubacar (génération 3)
    { id: 'pers-0000-0000-0000-000000000008', rattachement_id: RATT_ID_8, nom: 'Traoré', prenoms: 'Awa', genre: 'feminin', naissance: { annee: 1935 }, naissance_lieu: 'Bamako, Mali' },
  ],
  liens: [
    // Ibrahim → Ousmane (père)
    { id: 'lien-0000-0000-0000-000000000001', rattachement_source_id: RATT_ID_1, rattachement_cible_id: RATT_ID_2, type_lien: 'pere' },
    // Ousmane <-> Fatoumata (conjoints)
    { id: 'lien-0000-0000-0000-000000000002', rattachement_source_id: RATT_ID_2, rattachement_cible_id: RATT_ID_3, type_lien: 'conjoint' },
    // Ousmane → Aminata (père)
    { id: 'lien-0000-0000-0000-000000000003', rattachement_source_id: RATT_ID_2, rattachement_cible_id: RATT_ID_4, type_lien: 'pere' },
    // Fatoumata → Aminata (mère)
    { id: 'lien-0000-0000-0000-000000000004', rattachement_source_id: RATT_ID_3, rattachement_cible_id: RATT_ID_4, type_lien: 'mere' },
    // Ousmane → Mamadou (père)
    { id: 'lien-0000-0000-0000-000000000005', rattachement_source_id: RATT_ID_2, rattachement_cible_id: RATT_ID_5, type_lien: 'pere' },
    // Fatoumata → Mamadou (mère)
    { id: 'lien-0000-0000-0000-000000000006', rattachement_source_id: RATT_ID_3, rattachement_cible_id: RATT_ID_5, type_lien: 'mere' },
    // Ousmane → Kadiatou (père)
    { id: 'lien-0000-0000-0000-000000000007', rattachement_source_id: RATT_ID_2, rattachement_cible_id: RATT_ID_6, type_lien: 'pere' },
    // Aminata <-> Boubacar (conjoints)
    { id: 'lien-0000-0000-0000-000000000008', rattachement_source_id: RATT_ID_4, rattachement_cible_id: RATT_ID_7, type_lien: 'conjoint' },
    // Aminata → Awa (mère)
    { id: 'lien-0000-0000-0000-000000000009', rattachement_source_id: RATT_ID_4, rattachement_cible_id: RATT_ID_8, type_lien: 'mere' },
    // Boubacar → Awa (père)
    { id: 'lien-0000-0000-0000-000000000010', rattachement_source_id: RATT_ID_7, rattachement_cible_id: RATT_ID_8, type_lien: 'pere' },
  ],
}

export const obtenirArbreCompletMock = async (): Promise<ArbreComplet> => {
  await delay(200)
  return arbreCompletMock
}

// ─── Helpers ─────────────────────────────────────────────────────────────

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

/** Formate une DatePartielle en chaîne lisible */
export const formaterDate = (date?: DatePartielle): string => {
  if (!date) return 'Inconnue'
  const { annee, mois, jour } = date
  if (jour && mois && annee)
    return `${String(jour).padStart(2, '0')}/${String(mois).padStart(2, '0')}/${annee}`
  if (mois && annee) return `${String(mois).padStart(2, '0')}/${annee}`
  if (annee) return `${annee}`
  return 'Inconnue'
}

/** Retourne les initiales d'une personne (fallback photo) */
export const getInitiales = (personne: Personne): string => {
  const premierPrenom = personne.prenoms?.split(' ')[0] ?? ''
  return `${premierPrenom.charAt(0)}${personne.nom.charAt(0)}`.toUpperCase()
}

/** Libellé lisible d'un type de lien */
export const getLibelleLien = (type: TypeLien): string => {
  const libelles: Record<TypeLien, string> = {
    pere: 'Père',
    mere: 'Mère',
    parent: 'Parent',
    conjoint: 'Conjoint(e)',
  }
  return libelles[type]
}

export const getPersonneDetailMock = async (id: string): Promise<PersonneDetail | undefined> => {
  await delay(100)
  return detailsMock.find((d) => d.personne.id === id)
}

export const listerPersonnesMock = async (params?: PersonneQueryParams): Promise<PersonneListe> => {
  await delay(150)
  const page = params?.page ?? 1
  const par_page = params?.par_page ?? 12
  const recherche = params?.recherche?.toLowerCase()
  const filtrees = recherche
    ? personnes.filter(
        (p) =>
          p.nom.toLowerCase().includes(recherche) ||
          p.prenoms?.toLowerCase().includes(recherche),
      )
    : personnes
  const debut = (page - 1) * par_page
  return {
    personnes: filtrees.slice(debut, debut + par_page),
    total: filtrees.length,
    page,
    par_page,
    total_pages: Math.ceil(filtrees.length / par_page),
  }
}

export const formeVide = (): CreerPersonneForm => ({
  nom: '',
  prenoms: undefined,
  genre: undefined,
  naissance: undefined,
  naissance_lieu: undefined,
  deces: undefined,
  deces_lieu: undefined,
})

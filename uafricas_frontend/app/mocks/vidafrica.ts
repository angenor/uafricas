// ── Interfaces ───────────────────────────────────────────────

export type LangueSousTitre =
  | 'francais' | 'anglais' | 'arabe' | 'portugais' | 'swahili'
  | 'wolof' | 'haoussa' | 'amharique' | 'zoulou' | 'lingala'
  | 'bambara' | 'yoruba' | 'peul' | 'espagnol' | 'mandarin'

export const LANGUES_LABELS: Record<string, string> = {
  francais: 'Français',
  anglais: 'Anglais',
  arabe: 'Arabe',
  portugais: 'Portugais',
  swahili: 'Swahili',
  wolof: 'Wolof',
  haoussa: 'Haoussa',
  amharique: 'Amharique',
  zoulou: 'Zoulou',
  lingala: 'Lingala',
  bambara: 'Bambara',
  yoruba: 'Yoruba',
  peul: 'Peul',
  espagnol: 'Espagnol',
  mandarin: 'Mandarin',
}

export interface TimingMot {
  position: number
  mot: string
  debut_ms: number
  fin_ms: number
}

export interface SegmentSousTitre {
  id: string
  position: number
  texte: string
  debut_ms: number
  fin_ms: number
  timings_mot: TimingMot[]
  created_at: string
  updated_at: string
}

export interface PisteSousTitre {
  id: string
  langue: LangueSousTitre
  est_complete: boolean
  etat?: string
  cree_par?: string
  cree_par_nom?: string | null
  nombre_segments: number
  created_at: string
}

export interface Video {
  id: string
  titre: string
  slug: string
  description: string | null
  fichier_video_url: string
  vignette_url: string | null
  duree_secondes: number | null
  taille_octets: number | null
  format_video: string | null
  etat: string
  cree_par: string
  cree_par_nom: string | null
  pistes: PisteSousTitre[]
  created_at: string
  updated_at: string
}

export interface VideoListe {
  id: string
  titre: string
  slug: string
  vignette_url: string | null
  duree_secondes: number | null
  etat: string
  nombre_pistes: number
  created_at: string
}

// ── Données mock ─────────────────────────────────────────────

export const mockVideos: VideoListe[] = [
  {
    id: 'v1-uuid',
    titre: 'Introduction à la culture panafricaine',
    slug: 'introduction-a-la-culture-panafricaine',
    vignette_url: null,
    duree_secondes: 320,
    etat: 'publie',
    nombre_pistes: 2,
    created_at: '2026-04-10T10:00:00Z',
  },
  {
    id: 'v2-uuid',
    titre: 'Les langues africaines : richesse et diversité',
    slug: 'les-langues-africaines-richesse-et-diversite',
    vignette_url: null,
    duree_secondes: 540,
    etat: 'brouillon',
    nombre_pistes: 0,
    created_at: '2026-04-11T14:30:00Z',
  },
  {
    id: 'v3-uuid',
    titre: 'Documentaire : la musique traditionnelle',
    slug: 'documentaire-la-musique-traditionnelle',
    vignette_url: null,
    duree_secondes: 1200,
    etat: 'publie',
    nombre_pistes: 3,
    created_at: '2026-04-12T09:15:00Z',
  },
]

export const mockPistes: PisteSousTitre[] = [
  {
    id: 'p1-uuid',
    langue: 'francais',
    est_complete: true,
    nombre_segments: 3,
    created_at: '2026-04-10T11:00:00Z',
  },
  {
    id: 'p2-uuid',
    langue: 'anglais',
    est_complete: false,
    nombre_segments: 2,
    created_at: '2026-04-10T12:00:00Z',
  },
]

export const mockSegments: SegmentSousTitre[] = [
  {
    id: 's1-uuid',
    position: 1,
    texte: 'Bienvenue dans ce documentaire',
    debut_ms: 1000,
    fin_ms: 3500,
    timings_mot: [
      { position: 1, mot: 'Bienvenue', debut_ms: 1000, fin_ms: 1800 },
      { position: 2, mot: 'dans', debut_ms: 1800, fin_ms: 2100 },
      { position: 3, mot: 'ce', debut_ms: 2100, fin_ms: 2400 },
      { position: 4, mot: 'documentaire', debut_ms: 2400, fin_ms: 3500 },
    ],
    created_at: '2026-04-10T11:30:00Z',
    updated_at: '2026-04-10T11:30:00Z',
  },
  {
    id: 's2-uuid',
    position: 2,
    texte: 'Nous allons explorer ensemble',
    debut_ms: 4000,
    fin_ms: 6500,
    timings_mot: [
      { position: 1, mot: 'Nous', debut_ms: 4000, fin_ms: 4500 },
      { position: 2, mot: 'allons', debut_ms: 4500, fin_ms: 5000 },
      { position: 3, mot: 'explorer', debut_ms: 5000, fin_ms: 5800 },
      { position: 4, mot: 'ensemble', debut_ms: 5800, fin_ms: 6500 },
    ],
    created_at: '2026-04-10T11:35:00Z',
    updated_at: '2026-04-10T11:35:00Z',
  },
]

// ── Helpers ──────────────────────────────────────────────────

const delay = (ms: number = 300) => new Promise(resolve => setTimeout(resolve, ms))

export const getVideoParId = async (id: string): Promise<Video | null> => {
  await delay()
  const v = mockVideos.find(v => v.id === id)
  if (!v) return null
  return {
    ...v,
    description: 'Description de la vidéo mock',
    fichier_video_url: '/uploads/videos/sample.mp4',
    taille_octets: 52428800,
    format_video: 'mp4',
    cree_par: 'admin-uuid',
    cree_par_nom: 'Admin Test',
    pistes: mockPistes,
    updated_at: v.created_at,
  }
}

export const getVideoParSlug = async (slug: string): Promise<Video | null> => {
  await delay()
  const v = mockVideos.find(v => v.slug === slug)
  if (!v) return null
  return {
    ...v,
    description: 'Description de la vidéo mock',
    fichier_video_url: '/uploads/videos/sample.mp4',
    taille_octets: 52428800,
    format_video: 'mp4',
    cree_par: 'admin-uuid',
    cree_par_nom: 'Admin Test',
    pistes: mockPistes,
    updated_at: v.created_at,
  }
}

export const filtrerVideos = async (params: {
  recherche?: string
  etat?: string
}): Promise<VideoListe[]> => {
  await delay()
  return mockVideos.filter((v) => {
    if (params.etat && v.etat !== params.etat) return false
    if (params.recherche) {
      const r = params.recherche.toLowerCase()
      if (!v.titre.toLowerCase().includes(r)) return false
    }
    return true
  })
}

export const formaterDuree = (secondes: number | null): string => {
  if (!secondes) return '--:--'
  const min = Math.floor(secondes / 60)
  const sec = secondes % 60
  return `${min}:${sec.toString().padStart(2, '0')}`
}

export const formaterTimestamp = (ms: number): string => {
  const sec = Math.floor(ms / 1000)
  const min = Math.floor(sec / 60)
  const reste = sec % 60
  const millis = ms % 1000
  return `${min}:${reste.toString().padStart(2, '0')}.${millis.toString().padStart(3, '0')}`
}

// Données mock pour les centres culturels africains et afro-descendants

export type TypeProgrammation = 'presentiel' | 'en-ligne'
export type StatutProgrammation = 'a_venir' | 'en_cours' | 'termine'

export interface MembreEquipe {
  nom: string
  email: string
  tel: string
}

export interface Programmation {
  id: string
  titre: string
  description: string
  adress: string
  type: TypeProgrammation
  couvertureUrl: string
  dateDebut: Date
  dateFin: Date
  statut: StatutProgrammation
  createdAt: Date
}

export interface CentreCulturel {
  id: string
  nom: string
  adress: string
  urlBanniere: string
  urlGoogleMap?: string
  president: MembreEquipe
  vicePresident: MembreEquipe
  respCommunication: MembreEquipe
  programmations: Programmation[]
  createdAt: Date
  updatedAt: Date
}

// Images pour le carrousel
export const CAROUSEL_IMAGES: string[] = [
  'https://images.unsplash.com/photo-1523805009345-7448845a9e53?w=1200&h=400&fit=crop',
  'https://images.unsplash.com/photo-1580746738229-91921a92e4c0?w=1200&h=400&fit=crop',
  'https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?w=1200&h=400&fit=crop',
  'https://images.unsplash.com/photo-1489424731084-a5d8b219a5bb?w=1200&h=400&fit=crop'
]

// Types de programmation
export const TYPES_PROGRAMMATION: { value: TypeProgrammation | ''; label: string }[] = [
  { value: '', label: 'Sélectionner un type' },
  { value: 'presentiel', label: 'Exposition en présentiel' },
  { value: 'en-ligne', label: 'En ligne' }
]

// Données mock des centres culturels
export const centresCulturelsMock: CentreCulturel[] = [
  {
    id: 'montreal',
    nom: 'Montréal',
    adress: '1501 McGill College, Montréal, QC H3A 3M8',
    urlBanniere: 'https://images.unsplash.com/photo-1519178614-68673b201f36?w=800&h=400&fit=crop',
    urlGoogleMap: 'https://maps.google.com/?q=1501+McGill+College+Montreal',
    president: {
      nom: 'Amadou Diallo',
      email: 'amadou.diallo@uafricas.org',
      tel: '+1 514 555 1234'
    },
    vicePresident: {
      nom: 'Fatou Sow',
      email: 'fatou.sow@uafricas.org',
      tel: '+1 514 555 5678'
    },
    respCommunication: {
      nom: 'Moussa Keita',
      email: 'moussa.keita@uafricas.org',
      tel: '+1 514 555 9012'
    },
    programmations: [
      {
        id: 'prog-mtl-1',
        titre: 'Festival des Arts Africains',
        description: 'Une célébration de la culture africaine à travers la musique, la danse et les arts visuels. Venez découvrir les talents émergents de la diaspora africaine de Montréal. Au programme : concerts live, expositions d\'art contemporain africain, ateliers de danse traditionnelle et marché artisanal.',
        adress: '1501 McGill College, Montréal',
        type: 'presentiel',
        couvertureUrl: 'https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=600&h=400&fit=crop',
        dateDebut: new Date('2026-03-15T14:00:00'),
        dateFin: new Date('2026-03-15T22:00:00'),
        statut: 'a_venir',
        createdAt: new Date('2025-12-01')
      },
      {
        id: 'prog-mtl-2',
        titre: 'Atelier de cuisine sénégalaise',
        description: 'Apprenez à préparer le thiéboudienne, le plat national sénégalais, avec notre chef invité Mamadou Ndiaye. Une expérience culinaire authentique qui vous transportera au cœur de Dakar.',
        adress: 'Centre communautaire Côte-des-Neiges, Montréal',
        type: 'presentiel',
        couvertureUrl: 'https://images.unsplash.com/photo-1504674900247-0877df9cc836?w=600&h=400&fit=crop',
        dateDebut: new Date('2026-04-05T10:00:00'),
        dateFin: new Date('2026-04-05T14:00:00'),
        statut: 'a_venir',
        createdAt: new Date('2025-12-10')
      }
    ],
    createdAt: new Date('2024-01-15'),
    updatedAt: new Date('2025-12-20')
  },
  {
    id: 'paris',
    nom: 'Paris',
    adress: '25 Rue du Faubourg Saint-Martin, 75010 Paris',
    urlBanniere: 'https://images.unsplash.com/photo-1502602898657-3e91760cbb34?w=800&h=400&fit=crop',
    urlGoogleMap: 'https://maps.google.com/?q=25+Rue+du+Faubourg+Saint-Martin+Paris',
    president: {
      nom: 'Ibrahim Touré',
      email: 'ibrahim.toure@uafricas.org',
      tel: '+33 1 42 00 00 01'
    },
    vicePresident: {
      nom: 'Aminata Koné',
      email: 'aminata.kone@uafricas.org',
      tel: '+33 1 42 00 00 02'
    },
    respCommunication: {
      nom: 'Ousmane Bah',
      email: 'ousmane.bah@uafricas.org',
      tel: '+33 1 42 00 00 03'
    },
    programmations: [
      {
        id: 'prog-paris-1',
        titre: 'Exposition : Masques Africains Contemporains',
        description: 'Une exposition unique présentant des œuvres d\'artistes africains contemporains qui réinterprètent l\'art du masque traditionnel. Plus de 50 œuvres de 15 artistes du continent africain seront présentées.',
        adress: 'Galerie Africa, 25 Rue du Faubourg Saint-Martin, Paris',
        type: 'presentiel',
        couvertureUrl: 'https://images.unsplash.com/photo-1582555172866-f73bb12a2ab3?w=600&h=400&fit=crop',
        dateDebut: new Date('2026-02-20T10:00:00'),
        dateFin: new Date('2026-03-20T19:00:00'),
        statut: 'a_venir',
        createdAt: new Date('2025-11-15')
      },
      {
        id: 'prog-paris-2',
        titre: 'Webinaire : Histoire des royaumes africains',
        description: 'Une série de conférences en ligne sur l\'histoire des grands royaumes africains : Ghana, Mali, Songhaï, Kongo et bien d\'autres. Animé par le Professeur Cheikh Anta Diop Jr.',
        adress: 'En ligne - Zoom',
        type: 'en-ligne',
        couvertureUrl: 'https://images.unsplash.com/photo-1524178232363-1fb2b075b655?w=600&h=400&fit=crop',
        dateDebut: new Date('2026-03-01T18:00:00'),
        dateFin: new Date('2026-03-01T20:00:00'),
        statut: 'a_venir',
        createdAt: new Date('2025-12-05')
      },
      {
        id: 'prog-paris-3',
        titre: 'Concert Afrobeat Live',
        description: 'Soirée exceptionnelle avec les meilleurs artistes afrobeat de Paris. Au programme : Femi Kuti Tribute Band, DJ Mbalax et invités surprises.',
        adress: 'New Morning, 7-9 Rue des Petites Écuries, Paris',
        type: 'presentiel',
        couvertureUrl: 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=600&h=400&fit=crop',
        dateDebut: new Date('2026-04-12T21:00:00'),
        dateFin: new Date('2026-04-13T02:00:00'),
        statut: 'a_venir',
        createdAt: new Date('2025-12-18')
      }
    ],
    createdAt: new Date('2023-06-20'),
    updatedAt: new Date('2025-12-18')
  },
  {
    id: 'abidjan',
    nom: 'Abidjan',
    adress: 'Plateau, Avenue Franchet d\'Esperey, Abidjan',
    urlBanniere: 'https://images.unsplash.com/photo-1580060839134-75a5edca2e99?w=800&h=400&fit=crop',
    urlGoogleMap: 'https://maps.google.com/?q=Plateau+Abidjan',
    president: {
      nom: 'Kouadio Yao',
      email: 'kouadio.yao@uafricas.org',
      tel: '+225 07 00 00 01'
    },
    vicePresident: {
      nom: 'Adjoua Akissi',
      email: 'adjoua.akissi@uafricas.org',
      tel: '+225 07 00 00 02'
    },
    respCommunication: {
      nom: 'Koffi Brou',
      email: 'koffi.brou@uafricas.org',
      tel: '+225 07 00 00 03'
    },
    programmations: [
      {
        id: 'prog-abj-1',
        titre: 'Journée de la Diversité Culturelle',
        description: 'Célébration de la richesse culturelle ivoirienne avec des représentations de toutes les ethnies du pays. Danse, musique, gastronomie et artisanat traditionnel seront au rendez-vous.',
        adress: 'Palais de la Culture, Treichville, Abidjan',
        type: 'presentiel',
        couvertureUrl: 'https://images.unsplash.com/photo-1516450360452-9312f5e86fc7?w=600&h=400&fit=crop',
        dateDebut: new Date('2026-05-21T09:00:00'),
        dateFin: new Date('2026-05-21T18:00:00'),
        statut: 'a_venir',
        createdAt: new Date('2025-10-01')
      }
    ],
    createdAt: new Date('2024-03-10'),
    updatedAt: new Date('2025-10-01')
  },
  {
    id: 'dakar',
    nom: 'Dakar',
    adress: 'Rue Félix Faure, Plateau, Dakar',
    urlBanniere: 'https://images.unsplash.com/photo-1591778328453-7e7b8c1dd6e7?w=800&h=400&fit=crop',
    urlGoogleMap: 'https://maps.google.com/?q=Plateau+Dakar',
    president: {
      nom: 'Ibrahima Fall',
      email: 'ibrahima.fall@uafricas.org',
      tel: '+221 77 000 0001'
    },
    vicePresident: {
      nom: 'Aissatou Diop',
      email: 'aissatou.diop@uafricas.org',
      tel: '+221 77 000 0002'
    },
    respCommunication: {
      nom: 'Modou Gueye',
      email: 'modou.gueye@uafricas.org',
      tel: '+221 77 000 0003'
    },
    programmations: [
      {
        id: 'prog-dkr-1',
        titre: 'Biennale des Arts Africains',
        description: 'La plus grande exposition d\'art contemporain africain de l\'année. Artistes du continent entier réunis pour célébrer la créativité africaine sous toutes ses formes.',
        adress: 'Village des Arts, Dakar',
        type: 'presentiel',
        couvertureUrl: 'https://images.unsplash.com/photo-1578301978693-85fa9c0320b9?w=600&h=400&fit=crop',
        dateDebut: new Date('2026-06-01T10:00:00'),
        dateFin: new Date('2026-07-01T20:00:00'),
        statut: 'a_venir',
        createdAt: new Date('2025-09-15')
      },
      {
        id: 'prog-dkr-2',
        titre: 'Formation en ligne : Langues africaines',
        description: 'Cours intensif de wolof pour débutants. 10 sessions en ligne avec des professeurs natifs pour maîtriser les bases de la langue sénégalaise.',
        adress: 'En ligne - Plateforme UAfricas',
        type: 'en-ligne',
        couvertureUrl: 'https://images.unsplash.com/photo-1434030216411-0b793f4b4173?w=600&h=400&fit=crop',
        dateDebut: new Date('2026-02-15T17:00:00'),
        dateFin: new Date('2026-02-15T19:00:00'),
        statut: 'a_venir',
        createdAt: new Date('2025-12-01')
      }
    ],
    createdAt: new Date('2023-11-20'),
    updatedAt: new Date('2025-12-01')
  }
]

// Fonctions utilitaires
export const getCentreById = (id: string): CentreCulturel | undefined => {
  return centresCulturelsMock.find(c => c.id === id)
}

export const getProgrammationById = (
  centreId: string,
  programmationId: string
): { centre: CentreCulturel; programmation: Programmation } | undefined => {
  const centre = getCentreById(centreId)
  if (!centre) return undefined

  const programmation = centre.programmations.find(p => p.id === programmationId)
  if (!programmation) return undefined

  return { centre, programmation }
}

export const formatDateFrancais = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    weekday: 'long',
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  }).format(new Date(date))
}

export const formatDateCourteFrancais = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  }).format(new Date(date))
}

export const formatHeureFrancais = (date: Date): string => {
  return new Intl.DateTimeFormat('fr-FR', {
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(date))
}

export const getTypeLabel = (type: TypeProgrammation): string => {
  const found = TYPES_PROGRAMMATION.find(t => t.value === type)
  return found ? found.label : type
}

// Stats
export const getStatsCentres = () => {
  const totalCentres = centresCulturelsMock.length
  const totalProgrammations = centresCulturelsMock.reduce(
    (acc, centre) => acc + centre.programmations.length,
    0
  )
  const programmationsAVenir = centresCulturelsMock.reduce(
    (acc, centre) => acc + centre.programmations.filter(p => p.statut === 'a_venir').length,
    0
  )

  return {
    totalCentres,
    totalProgrammations,
    programmationsAVenir
  }
}

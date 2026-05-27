// Types des contributions citoyennes (Gouvernance)

export interface ContributionAuteur {
  id: string
  prenom: string
  nom: string
  photoURL?: string
}

export interface ContributionStats {
  vues: number
  vuesUniques: number
  likes: number
  commentaires: number
  partages: number
  soutiens?: number
  validations?: number
  confirmations?: number
}

export interface ContributionLocalisation {
  pays: string
  region?: string
  ville?: string
  quartier?: string
  latitude?: number
  longitude?: number
}

export interface Prejudice {
  titre: string
  description: string
  likes?: number
}

export interface ContributionProblematique {
  categorie: string
  gravite?: 'faible' | 'moyenne' | 'grave' | 'critique'
  urgence?: string
}

export type TypePratique = 'mauvaise' | 'bonne'

export interface ContributionBonnePratique {
  categorie: string
  impact?: 'faible' | 'moyen' | 'fort' | 'exemplaire'
  reproductibilite?: string
}

export interface ContributionMedias {
  photos: string[]
  videos: string[]
  documents: string[]
}

export type TypeReactionGlobale = 'coeur' | 'pouce' | 'rire' | 'jaime_pas'

export interface ReactionsGlobales {
  coeur: number
  pouce: number
  rire: number
  jaimePas: number
  /** Réaction de l'utilisateur courant, ou null */
  maReaction?: TypeReactionGlobale | null
}

export interface ContributionCitoyenne {
  id: string
  type: 'factcheck' | 'badhabits' | 'ideaforces'
  statut: 'brouillon' | 'publie' | 'archive'
  titre: string
  description: string
  auteur: ContributionAuteur
  localisation: ContributionLocalisation
  dateCreation: Date
  dateMiseAJour?: Date
  typePratique?: TypePratique
  problematique?: ContributionProblematique
  bonnePratique?: ContributionBonnePratique
  factcheck?: {
    prejuge: Prejudice
    contrePrejuge: Prejudice
  }
  proposition?: {
    objectif: string
    moyens: string[]
    beneficiaires: string[]
    impact: string
  }
  medias?: ContributionMedias
  sources?: { titre: string; url: string }[]
  stats: ContributionStats
  tags?: string[]
  verified?: boolean
  /** Réactions globales (emojis) — factcheck connecté à l'API */
  reactions?: ReactionsGlobales
  /** L'utilisateur courant a liké le volet préjugé */
  aLikePrejuge?: boolean
  /** L'utilisateur courant a liké le volet réalité */
  aLikeRealite?: boolean
  /** L'utilisateur courant a déjà signalé cette publication */
  aSignale?: boolean
}

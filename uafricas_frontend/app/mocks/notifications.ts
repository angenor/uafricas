// ════════════════════════════════════════════════════════════════════════════
// Mock — Notifications et suggestions intelligentes
// ════════════════════════════════════════════════════════════════════════════

export type TypeNotification = 'matching' | 'collaboration' | 'invitation' | 'contact' | 'systeme'

export interface Notification {
  id: string
  type: TypeNotification
  message: string
  lien_action?: string
  lu: boolean
  created_at: string
}

export interface DoublonPotentiel {
  personne_a: { id: string; nom: string; prenoms?: string; naissance_annee?: number; naissance_lieu?: string }
  personne_b: { id: string; nom: string; prenoms?: string; naissance_annee?: number; naissance_lieu?: string }
  score: number
}

export interface SuggestionProactive {
  type: 'parents_manquants' | 'date_manquante' | 'branche_courte'
  personneId: string
  personneNom: string
  message: string
  action: string
  priorite: number
}

export interface FusionDoublonDto {
  personne_a_garder_id: string
  personne_a_supprimer_id: string
  nom: string
  prenoms?: string
  genre?: string
  naissance?: { annee?: number; mois?: number; jour?: number }
  naissance_lieu?: string
  deces?: { annee?: number; mois?: number; jour?: number }
  deces_lieu?: string
}

export const iconeNotification = (type: TypeNotification): string => {
  const icones: Record<TypeNotification, string> = {
    matching: 'users',
    collaboration: 'user-plus',
    invitation: 'envelope',
    contact: 'address-book',
    systeme: 'bell',
  }
  return icones[type] || 'bell'
}

export const couleurNotification = (type: TypeNotification): string => {
  const couleurs: Record<TypeNotification, string> = {
    matching: 'text-[var(--color-custom-green)]',
    collaboration: 'text-blue-600',
    invitation: 'text-amber-600',
    contact: 'text-purple-600',
    systeme: 'text-stone-500',
  }
  return couleurs[type] || 'text-stone-500'
}

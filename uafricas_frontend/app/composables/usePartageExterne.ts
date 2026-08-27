// Traçage best-effort des partages vers les réseaux sociaux externes (US5).
//
// Ce composable vit à part de `useEngagement` parce qu'il est consommé par les
// 6 modales de partage de la plateforme, qui n'ont rien à voir avec l'espace
// membre.
import { useUserStore } from '~/stores/user'

/** Réseaux traçables (miroir de l'enum `engagement.reseau_social`). */
export type ReseauExterne = 'whatsapp' | 'facebook' | 'x' | 'linkedin' | 'telegram' | 'email'

/** Un réseau proposé par une modale de partage. */
export interface OptionReseau {
  nom: string
  url: string
  icon: string[]
  couleur: string
  /** Clé de traçage. Absente = ce bouton ne compte pas (ex. « copier le lien »). */
  reseau?: ReseauExterne
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export interface ResultatPartageExterne {
  /** Ce réseau vient-il d'être journalisé pour ce couple membre/contenu ? */
  enregistre: boolean
  /**
   * L'**auteur du contenu** vient-il d'être crédité ? Vrai au premier partage de
   * ce contenu par ce membre, tous canaux confondus.
   *
   * Le bonus « 5 réseaux distincts » n'existe plus : le partage récompense
   * désormais celui qui a produit le contenu, pas celui qui le relaie. Il n'y a
   * donc plus rien à promettre au partageur, et aucune modale n'affiche de
   * compteur de progression.
   */
  auteur_credite: boolean
}

/**
 * Construit les six liens de partage à partir d'un texte et d'une URL.
 *
 * Ces six URL étaient reconstruites À LA MAIN dans six composants : les deux
 * modales d'Afripulse, celle des médias, celle des événements, celle de la
 * gouvernance et les boutons d'Africonnect. Six copies du même encodage, dont
 * une seule aurait été corrigée le jour d'un défaut.
 *
 * `mailto:` est traité à part par l'appelant : `window.open` sur un `mailto:`
 * laisse un onglet vide derrière lui.
 */
export const construireReseaux = (texte: string, url: string): OptionReseau[] => {
  const t = encodeURIComponent(texte)
  const u = encodeURIComponent(url)
  return [
    { nom: 'WhatsApp', url: `https://wa.me/?text=${encodeURIComponent(`${texte} ${url}`)}`, icon: ['fab', 'whatsapp'], couleur: 'bg-[#25D366] hover:bg-[#1da851]', reseau: 'whatsapp' },
    { nom: 'Facebook', url: `https://www.facebook.com/sharer/sharer.php?u=${u}`, icon: ['fab', 'facebook'], couleur: 'bg-[#1877F2] hover:bg-[#0d65d9]', reseau: 'facebook' },
    { nom: 'X / Twitter', url: `https://twitter.com/intent/tweet?url=${u}&text=${t}`, icon: ['fab', 'twitter'], couleur: 'bg-black hover:bg-gray-800', reseau: 'x' },
    { nom: 'LinkedIn', url: `https://www.linkedin.com/sharing/share-offsite/?url=${u}`, icon: ['fab', 'linkedin'], couleur: 'bg-[#0A66C2] hover:bg-[#084e96]', reseau: 'linkedin' },
    { nom: 'Telegram', url: `https://t.me/share/url?url=${u}&text=${t}`, icon: ['fab', 'telegram'], couleur: 'bg-[#229ED9] hover:bg-[#1b7fae]', reseau: 'telegram' },
    { nom: 'E-mail', url: `mailto:?subject=${t}&body=${encodeURIComponent(`${texte} ${url}`)}`, icon: ['fas', 'envelope'], couleur: 'bg-af-corps hover:opacity-90', reseau: 'email' },
  ]
}

/** Ouvre un lien de partage. `mailto:` ne passe PAS par `window.open`. */
export const ouvrirPartage = (r: OptionReseau) => {
  if (r.url.startsWith('mailto:')) window.location.href = r.url
  else window.open(r.url, '_blank', 'noopener,noreferrer,width=600,height=500')
}

export const usePartageExterne = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  /**
   * Trace un partage. **À appeler APRÈS l'ouverture de la fenêtre du réseau.**
   *
   * L'échec est silencieux : un traçage raté ne doit jamais empêcher un partage,
   * ni afficher une erreur pour une mécanique de points que l'utilisateur n'a pas
   * demandée. C'est aussi ce qui rend le partage insensible à une panne du
   * backend d'engagement.
   *
   * Ne fait rien si le visiteur n'est pas connecté : il n'y a alors personne à
   * créditer, et le serveur refuserait de toute façon (identité prise du JWT).
   */
  const tracerPartage = (
    typeObjet: string,
    objetId: string,
    reseau: ReseauExterne,
  ): void => {
    if (!userStore.accessToken || !objetId) return

    // Volontairement non-attendu (« fire and forget ») : la fenêtre de partage est
    // déjà ouverte, l'utilisateur n'a rien à attendre de cet appel.
    $fetch<ApiResponse<ResultatPartageExterne>>(
      `${apiBase}/api/engagement/partages-externes`,
      {
        method: 'POST',
        headers: { Authorization: `Bearer ${userStore.accessToken}` },
        body: { type_objet: typeObjet, objet_id: objetId, reseau },
      },
    ).catch(() => {})
  }

  return { tracerPartage }
}

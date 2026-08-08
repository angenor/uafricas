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

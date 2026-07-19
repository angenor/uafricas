/**
 * État de lecture média partagé par toute l'application.
 *
 * Deux exigences imposent que cet état vive HORS des composants :
 *   • FR-017 — l'écoute d'une radio survit au défilement ET au changement de
 *     page. La barre de lecture est montée dans le layout, hors du `<slot/>` ;
 *     l'état qu'elle pilote doit donc être partagé, pas local.
 *   • FR-018 — un seul flux à la fois. Lancer un contenu coupe le précédent,
 *     ce qu'un état par composant ne peut pas garantir.
 *
 * `useState` (et non `ref`) : partage l'instance entre tous les appelants et
 * traverse correctement le rendu SSR de Nuxt.
 */

export interface ContenuEnLecture {
  /** Identifiant du contenu, pour reconnaître ce qui joue déjà. */
  id: string
  type: 'programme_radio' | 'programme_tele' | 'station_radio' | 'chaine_tv'
  titre: string
  /** Chaîne ou station d'origine — affichée dans la barre de lecture (SC-004). */
  support?: string | null
  supportSlug?: string | null
  url: string
  image?: string | null
  /** Un direct n'a ni fin ni position : la barre masque alors la timeline. */
  estDirect?: boolean
}

export const useLecteurMedia = () => {
  const contenu = useState<ContenuEnLecture | null>('media:contenu', () => null)
  const enLecture = useState<boolean>('media:lecture', () => false)
  const volume = useState<number>('media:volume', () => 0.8)
  const coupe = useState<boolean>('media:coupe', () => false)

  /** Un contenu est-il chargé dans le lecteur, quel que soit son état ? */
  const aUnContenu = computed(() => contenu.value !== null)

  const estContenuCourant = (id: string) => contenu.value?.id === id

  /**
   * Lance un contenu. S'il joue déjà, bascule simplement lecture/pause plutôt
   * que de le relancer depuis le début — un clic répété ne doit pas rembobiner.
   */
  const lire = (nouveau: ContenuEnLecture) => {
    if (contenu.value?.id === nouveau.id) {
      enLecture.value = true
      return
    }
    // Remplacer le contenu coupe de fait le flux précédent (FR-018).
    contenu.value = nouveau
    enLecture.value = true
  }

  const pause = () => {
    enLecture.value = false
  }

  const basculerLecture = (nouveau?: ContenuEnLecture) => {
    if (nouveau && contenu.value?.id !== nouveau.id) {
      lire(nouveau)
      return
    }
    enLecture.value = !enLecture.value
  }

  /** Arrête et vide le lecteur — la barre persistante disparaît alors. */
  const arreter = () => {
    enLecture.value = false
    contenu.value = null
  }

  const basculerSon = () => {
    coupe.value = !coupe.value
  }

  const definirVolume = (valeur: number) => {
    volume.value = Math.max(0, Math.min(1, valeur))
    // Régler le volume au-dessus de zéro rétablit le son : laisser l'état
    // « coupé » actif donnerait un curseur qui ne produit aucun effet.
    if (volume.value > 0) coupe.value = false
  }

  return {
    contenu,
    enLecture,
    volume,
    coupe,
    aUnContenu,
    estContenuCourant,
    lire,
    pause,
    basculerLecture,
    arreter,
    basculerSon,
    definirVolume,
  }
}

/**
 * Observation de la visibilité d'un élément (IntersectionObserver).
 *
 * Sert le chargement différé des sections médias : monter d'emblée le lecteur
 * de cinquante sections déclencherait autant de requêtes réseau pour des
 * contenus que le visiteur ne verra peut-être jamais (FR-054, SC-011).
 *
 * Aucun mécanisme de ce genre n'existait dans le projet — `useAOS` n'anime que
 * l'apparition et ne diffère aucun chargement.
 */

export interface OptionsObservateurVisibilite {
  /** Marge de déclenchement : anticipe l'entrée dans le cadre. */
  marge?: string
  /** Fraction de l'élément visible à partir de laquelle il compte comme vu. */
  seuil?: number
  /**
   * Cesser d'observer dès la première apparition. C'est le cas d'usage
   * habituel : une fois la section montée, on ne la démonte pas au défilement.
   */
  uneSeuleFois?: boolean
}

export const useObservateurVisibilite = (
  cible: Ref<HTMLElement | null | undefined>,
  options: OptionsObservateurVisibilite = {},
) => {
  const { marge = '200px', seuil = 0, uneSeuleFois = true } = options

  const estVisible = ref(false)
  /** Vrai dès que l'élément a été vu au moins une fois. */
  const aEteVisible = ref(false)

  let observateur: IntersectionObserver | null = null

  const arreter = () => {
    observateur?.disconnect()
    observateur = null
  }

  onMounted(() => {
    // SSR et navigateurs anciens : à défaut d'API, tout est considéré visible.
    // Mieux vaut charger que laisser une page vide.
    if (typeof IntersectionObserver === 'undefined') {
      estVisible.value = true
      aEteVisible.value = true
      return
    }

    watch(
      cible,
      (element) => {
        arreter()
        if (!element) return

        observateur = new IntersectionObserver(
          (entrees) => {
            const entree = entrees[0]
            if (!entree) return
            estVisible.value = entree.isIntersecting
            if (entree.isIntersecting) {
              aEteVisible.value = true
              if (uneSeuleFois) arreter()
            }
          },
          { rootMargin: marge, threshold: seuil },
        )
        observateur.observe(element)
      },
      { immediate: true },
    )
  })

  onBeforeUnmount(arreter)

  return { estVisible, aEteVisible, arreter }
}

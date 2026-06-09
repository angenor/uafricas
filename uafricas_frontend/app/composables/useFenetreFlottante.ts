// Rend une fenêtre flottante déplaçable (par sa barre de titre) et
// redimensionnable (par une poignée d'angle). Position et taille en pixels,
// bornées au viewport, persistées dans localStorage. Pointer events +
// setPointerCapture (idiome du projet, cf. AfrolangRoom) : le drag se poursuit
// même quand le curseur survole un autre élément.

interface Position { x: number, y: number }
interface Taille { largeur: number, hauteur: number }

export interface OptionsFenetreFlottante {
  /** Clé de persistance localStorage. */
  cle?: string
  largeurDefaut?: number
  hauteurDefaut?: number
  largeurMin?: number
  hauteurMin?: number
  /** Espace réservé sous la fenêtre au placement initial (ex. bouton flottant). */
  margeBas?: number
}

export const useFenetreFlottante = (options: OptionsFenetreFlottante = {}) => {
  const cle = options.cle ?? 'fenetre-flottante'
  const LARGEUR_DEFAUT = options.largeurDefaut ?? 352
  const HAUTEUR_DEFAUT = options.hauteurDefaut ?? 480
  const LARGEUR_MIN = options.largeurMin ?? 300
  const HAUTEUR_MIN = options.hauteurMin ?? 360
  const MARGE = 8
  const MARGE_BAS = options.margeBas ?? 96

  const position = ref<Position>({ x: 0, y: 0 })
  const taille = ref<Taille>({ largeur: LARGEUR_DEFAUT, hauteur: HAUTEUR_DEFAUT })
  const place = ref(false)
  const deplace = ref(false)
  const redimensionne = ref(false)

  const style = computed(() => ({
    left: `${position.value.x}px`,
    top: `${position.value.y}px`,
    width: `${taille.value.largeur}px`,
    height: `${taille.value.hauteur}px`,
  }))

  const borne = (v: number, min: number, max: number): number =>
    Math.min(Math.max(v, min), max)

  const vue = () => ({ vw: window.innerWidth, vh: window.innerHeight })

  /** Borne la taille au viewport puis la position pour garder la fenêtre visible. */
  const recadrer = (): void => {
    const { vw, vh } = vue()
    taille.value.largeur = borne(taille.value.largeur, LARGEUR_MIN, Math.max(LARGEUR_MIN, vw - 2 * MARGE))
    taille.value.hauteur = borne(taille.value.hauteur, HAUTEUR_MIN, Math.max(HAUTEUR_MIN, vh - 2 * MARGE))
    position.value.x = borne(position.value.x, MARGE, Math.max(MARGE, vw - taille.value.largeur - MARGE))
    position.value.y = borne(position.value.y, MARGE, Math.max(MARGE, vh - taille.value.hauteur - MARGE))
  }

  const sauvegarder = (): void => {
    try {
      localStorage.setItem(cle, JSON.stringify({ position: position.value, taille: taille.value }))
    }
    catch {
      // localStorage indisponible (mode privé) : la fenêtre reste fonctionnelle.
    }
  }

  /** Placement initial : restaure la dernière disposition, sinon coin bas-droit. */
  const initialiser = (): void => {
    if (place.value) {
      recadrer()
      return
    }
    place.value = true
    let restaure = false
    try {
      const brut = localStorage.getItem(cle)
      if (brut) {
        const o = JSON.parse(brut)
        if (o?.position && o?.taille
          && typeof o.position.x === 'number' && typeof o.taille.largeur === 'number') {
          position.value = { x: o.position.x, y: o.position.y }
          taille.value = { largeur: o.taille.largeur, hauteur: o.taille.hauteur }
          restaure = true
        }
      }
    }
    catch {
      // Disposition corrompue : on retombe sur le placement par défaut.
    }
    if (!restaure) {
      const { vw, vh } = vue()
      taille.value = { largeur: LARGEUR_DEFAUT, hauteur: HAUTEUR_DEFAUT }
      position.value = { x: vw - LARGEUR_DEFAUT - 24, y: vh - HAUTEUR_DEFAUT - MARGE_BAS }
    }
    recadrer()
  }

  // ── Déplacement (sur la barre de titre) ─────────────────────
  let depDx = 0
  let depDy = 0

  const demarrerDeplacement = (e: PointerEvent): void => {
    // Ne pas démarrer un glisser sur un bouton de l'en-tête (fermer, etc.).
    if ((e.target as HTMLElement).closest('button')) return
    deplace.value = true
    depDx = e.clientX - position.value.x
    depDy = e.clientY - position.value.y
    ;(e.currentTarget as HTMLElement).setPointerCapture?.(e.pointerId)
    e.preventDefault()
  }
  const surDeplacement = (e: PointerEvent): void => {
    if (!deplace.value) return
    const { vw, vh } = vue()
    position.value.x = borne(e.clientX - depDx, MARGE, Math.max(MARGE, vw - taille.value.largeur - MARGE))
    position.value.y = borne(e.clientY - depDy, MARGE, Math.max(MARGE, vh - taille.value.hauteur - MARGE))
  }
  const finDeplacement = (e: PointerEvent): void => {
    if (!deplace.value) return
    deplace.value = false
    try { (e.currentTarget as HTMLElement).releasePointerCapture?.(e.pointerId) }
    catch { /* pointeur déjà relâché */ }
    sauvegarder()
  }

  // ── Redimensionnement (poignée bas-droite) ──────────────────
  let redL = 0
  let redH = 0
  let redX = 0
  let redY = 0

  const demarrerRedimensionnement = (e: PointerEvent): void => {
    redimensionne.value = true
    redL = taille.value.largeur
    redH = taille.value.hauteur
    redX = e.clientX
    redY = e.clientY
    ;(e.currentTarget as HTMLElement).setPointerCapture?.(e.pointerId)
    e.preventDefault()
    e.stopPropagation()
  }
  const surRedimensionnement = (e: PointerEvent): void => {
    if (!redimensionne.value) return
    const { vw, vh } = vue()
    taille.value.largeur = borne(
      redL + (e.clientX - redX), LARGEUR_MIN, Math.max(LARGEUR_MIN, vw - position.value.x - MARGE),
    )
    taille.value.hauteur = borne(
      redH + (e.clientY - redY), HAUTEUR_MIN, Math.max(HAUTEUR_MIN, vh - position.value.y - MARGE),
    )
  }
  const finRedimensionnement = (e: PointerEvent): void => {
    if (!redimensionne.value) return
    redimensionne.value = false
    try { (e.currentTarget as HTMLElement).releasePointerCapture?.(e.pointerId) }
    catch { /* pointeur déjà relâché */ }
    sauvegarder()
  }

  const surResizeFenetre = (): void => { if (place.value) recadrer() }

  onMounted(() => {
    initialiser()
    window.addEventListener('resize', surResizeFenetre)
  })
  onBeforeUnmount(() => {
    window.removeEventListener('resize', surResizeFenetre)
  })

  return {
    style,
    position,
    taille,
    deplace,
    redimensionne,
    initialiser,
    demarrerDeplacement,
    surDeplacement,
    finDeplacement,
    demarrerRedimensionnement,
    surRedimensionnement,
    finRedimensionnement,
  }
}

import { ref, onMounted, onBeforeUnmount, type Ref, shallowRef } from 'vue'
import gsap from 'gsap'

// ── Configuration ─────────────────────────────────────────────────
export interface ConfigAnimation {
  dureeTransition: number
  dureeStagger: number
  dureeConfettis: number
  couleursConfettis: string[]
  nombreConfettis: number
}

const CONFIG_DEFAUT: ConfigAnimation = {
  dureeTransition: 0.4,
  dureeStagger: 0.1,
  dureeConfettis: 3.5,
  couleursConfettis: ['#A54A1C', '#228B22'],
  nombreConfettis: 35,
}

// ── Interface d'etat ──────────────────────────────────────────────
export interface EtatAnimation {
  enTransition: Ref<boolean>
  prefereReducedMotion: Ref<boolean>
  timelineCourante: Ref<gsap.core.Timeline | null>
  gsapCtx: gsap.Context | null
}

// ── Composable ────────────────────────────────────────────────────
export function useAnimationsFormulaire(
  conteneurRef: Ref<HTMLElement | null>,
  config: Partial<ConfigAnimation> = {},
) {
  const cfg = { ...CONFIG_DEFAUT, ...config }

  const enTransition = ref(false)
  const prefereReducedMotion = ref(false)
  const timelineCourante = shallowRef<gsap.core.Timeline | null>(null)
  let gsapCtx: gsap.Context | null = null

  // ── Lifecycle ─────────────────────────────────────────────────
  onMounted(() => {
    prefereReducedMotion.value = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    if (conteneurRef.value) {
      gsapCtx = gsap.context(() => {}, conteneurRef.value)
    }
  })

  onBeforeUnmount(() => {
    if (timelineCourante.value) {
      timelineCourante.value.kill()
    }
    gsapCtx?.revert()
  })

  // ── Helpers ───────────────────────────────────────────────────
  function tuerTimelineCourante() {
    if (timelineCourante.value) {
      timelineCourante.value.kill()
      timelineCourante.value = null
    }
  }

  // ── T002: Transition d'etape (slide directionnel + fade) ──────
  function animerTransitionEtape(
    direction: 'avant' | 'arriere',
    cibleRef: Ref<HTMLElement | null>,
  ) {
    if (prefereReducedMotion.value || !cibleRef.value) return

    tuerTimelineCourante()

    const xDepart = direction === 'avant' ? 80 : -80
    const tl = gsap.timeline()
    tl.fromTo(
      cibleRef.value,
      { x: xDepart, opacity: 0 },
      { x: 0, opacity: 1, duration: cfg.dureeTransition, ease: 'power2.out' },
    )
    timelineCourante.value = tl
  }

  // ── T003: Stagger des champs d'une etape ──────────────────────
  function animerChampsEtape(conteneurEtapeRef: Ref<HTMLElement | null>) {
    if (prefereReducedMotion.value || !conteneurEtapeRef.value) return

    const enfants = conteneurEtapeRef.value.children
    if (enfants.length === 0) return

    gsap.fromTo(
      enfants,
      { opacity: 0, y: 20 },
      {
        opacity: 1,
        y: 0,
        duration: 0.3,
        stagger: cfg.dureeStagger,
        ease: 'power1.out',
      },
    )
  }

  // ── T004: Animation de la barre de progression ────────────────
  function animerProgression(
    nouvelleEtape: number,
    ancienneEtape: number,
    dotsRef: Ref<HTMLElement | null>,
  ) {
    if (prefereReducedMotion.value || !dotsRef.value) return

    const dots = dotsRef.value.querySelectorAll('.dot-indicateur')
    const segments = dotsRef.value.querySelectorAll('.segment-indicateur')

    dots.forEach((dot, i) => {
      const etapeIndex = i + 1
      if (etapeIndex === nouvelleEtape) {
        gsap.to(dot, { scale: 1.3, duration: 0.3, ease: 'back.out(1.7)' })
      } else {
        gsap.to(dot, { scale: 1, duration: 0.2 })
      }
    })

    segments.forEach((seg, i) => {
      const etapeIndex = i + 1
      if (etapeIndex < nouvelleEtape) {
        gsap.to(seg, {
          scaleX: 1,
          duration: 0.3,
          transformOrigin: ancienneEtape < nouvelleEtape ? 'left' : 'right',
        })
      } else {
        gsap.to(seg, {
          scaleX: 0,
          duration: 0.3,
          transformOrigin: ancienneEtape < nouvelleEtape ? 'right' : 'left',
        })
      }
    })
  }

  // ── T005: Confettis ───────────────────────────────────────────
  function lancerConfettis(
    conteneurConfettisRef: Ref<HTMLElement | null>,
    couleurs?: string[],
  ) {
    if (prefereReducedMotion.value || !conteneurConfettisRef.value) return

    const couleursFinales = couleurs ?? cfg.couleursConfettis
    const fragments: HTMLDivElement[] = []

    for (let i = 0; i < cfg.nombreConfettis; i++) {
      const div = document.createElement('div')
      div.style.position = 'absolute'
      div.style.width = `${gsap.utils.random(6, 12)}px`
      div.style.height = `${gsap.utils.random(6, 12)}px`
      div.style.backgroundColor = couleursFinales[i % couleursFinales.length] ?? '#A54A1C'
      div.style.borderRadius = Math.random() > 0.5 ? '50%' : '2px'
      div.style.left = `${gsap.utils.random(10, 90)}%`
      div.style.top = '0'
      div.style.pointerEvents = 'none'
      conteneurConfettisRef.value.appendChild(div)
      fragments.push(div)
    }

    gsap.to(fragments, {
      y: () => gsap.utils.random(100, 400),
      x: () => gsap.utils.random(-80, 80),
      rotation: () => gsap.utils.random(-360, 360),
      opacity: 0,
      duration: () => gsap.utils.random(cfg.dureeConfettis - 0.5, cfg.dureeConfettis + 0.5),
      stagger: 0.03,
      ease: 'power1.in',
      onComplete: () => {
        fragments.forEach(f => f.remove())
      },
    })
  }

  // ── T006: Compteur anime ──────────────────────────────────────
  function animerCompteur(
    cibleRef: Ref<HTMLElement | null>,
    valeurFinale: number,
  ) {
    if (prefereReducedMotion.value || !cibleRef.value) {
      if (cibleRef.value) {
        cibleRef.value.textContent = String(valeurFinale)
      }
      return
    }

    const proxy = { val: 0 }
    gsap.to(proxy, {
      val: valeurFinale,
      duration: 1.5,
      ease: 'power2.out',
      onUpdate: () => {
        if (cibleRef.value) {
          cibleRef.value.textContent = String(Math.round(proxy.val))
        }
      },
    })
  }

  // ── T007: Shake erreur ────────────────────────────────────────
  function animerErreur(cibleRef: Ref<HTMLElement | null>) {
    if (prefereReducedMotion.value || !cibleRef.value) return

    gsap.to(cibleRef.value, {
      keyframes: [
        { x: 0 },
        { x: -10 },
        { x: 10 },
        { x: -6 },
        { x: 6 },
        { x: 0 },
      ],
      duration: 0.4,
      ease: 'power2.out',
    })
  }

  return {
    enTransition,
    prefereReducedMotion,
    timelineCourante,
    animerTransitionEtape,
    animerChampsEtape,
    animerProgression,
    lancerConfettis,
    animerCompteur,
    animerErreur,
  }
}

<script setup lang="ts">
/**
 * Piste horizontale de cartes, façon catalogue de plateforme de streaming.
 *
 * Le défilement est natif (`overflow-x-auto` + `scroll-snap`) plutôt que porté
 * par un carrousel tiers : `vue3-carousel`, déjà présent dans le projet, est
 * configuré pour une diapositive à la fois avec défilement automatique, et
 * impose une feuille de style tierce sur des pages tenues au Tailwind pur.
 *
 * Le contenu de chaque carte est laissé au parent, via un slot : la rangée sert
 * aussi bien des programmes télé que des émissions radio.
 *
 * Les flèches ne sont montrées **que du côté où il reste quelque chose à voir**
 * (`peutReculer` / `peutAvancer`) : une flèche inerte en bout de piste laisse
 * croire à du contenu qui n'existe pas.
 */

defineProps<{
  titre?: string
  /** Message affiché quand la rangée est vide. */
  messageVide?: string
}>()

const cadre = ref<HTMLElement | null>(null)
const piste = ref<HTMLElement | null>(null)
const peutReculer = ref(false)
const peutAvancer = ref(false)

/**
 * Bande occupée par la couverture de la première tuile, dans le repère du
 * cadre : sommet et hauteur.
 *
 * Elle borne à la fois les flèches et le voile dégradé. Sans elle, `top-1/2` et
 * `inset-y-0` porteraient sur la hauteur TOTALE de la piste, titre et
 * description compris : les flèches tomberaient sous l'image, et le voile
 * estomperait un titre de programme, ce qui se lit comme un défaut d'affichage
 * et non comme une invitation à faire défiler. `null` tant que rien n'est
 * mesuré (rendu serveur) : les valeurs de repli reprennent alors la main.
 */
const bandeCouverture = ref<{ sommet: number, hauteur: number } | null>(null)

const styleFleche = computed(() =>
  bandeCouverture.value
    ? { top: `${bandeCouverture.value.sommet + bandeCouverture.value.hauteur / 2}px` }
    : undefined,
)

/** `bottom: auto` est indispensable : sans lui, `top` et `bottom` étant tous
 *  deux fixés, la hauteur demandée serait ignorée. */
const styleVoile = computed(() =>
  bandeCouverture.value
    ? { top: `${bandeCouverture.value.sommet}px`, bottom: 'auto', height: `${bandeCouverture.value.hauteur}px` }
    : undefined,
)

/**
 * La marge d'un pixel absorbe les arrondis de sous-pixel : sans elle, une piste
 * défilée jusqu'au bout garderait sa flèche « suivant » active.
 */
const mesurer = () => {
  const el = piste.value
  if (!el) return
  peutReculer.value = el.scrollLeft > 1
  peutAvancer.value = el.scrollLeft + el.clientWidth < el.scrollWidth - 1

  const couverture = el.querySelector<HTMLElement>('[data-couverture]')
  if (couverture && cadre.value) {
    const repereCouverture = couverture.getBoundingClientRect()
    const repereCadre = cadre.value.getBoundingClientRect()
    bandeCouverture.value = {
      sommet: repereCouverture.top - repereCadre.top,
      hauteur: repereCouverture.height,
    }
  }
}

/** Fait défiler d'environ une largeur de cadre, dans un sens ou dans l'autre. */
const defiler = (sens: -1 | 1) => {
  const el = piste.value
  if (!el) return
  el.scrollBy({ left: sens * Math.max(el.clientWidth * 0.9, 240), behavior: 'smooth' })
}

/**
 * Une piste dont le contenu tient à l'écran n'affiche aucune flèche : c'est le
 * redimensionnement qui fait basculer d'un état à l'autre, d'où l'observateur.
 */
let observateur: ResizeObserver | null = null

onMounted(async () => {
  await nextTick()
  mesurer()
  if (piste.value && typeof ResizeObserver !== 'undefined') {
    observateur = new ResizeObserver(mesurer)
    observateur.observe(piste.value)
  }
})

onBeforeUnmount(() => observateur?.disconnect())
</script>

<template>
  <section ref="cadre" class="relative group/rangee">
    <!-- Titre et méta sur la même ligne : depuis 009, une rangée est un
         PROGRAMME, et son en-tête porte son décompte d'épisodes et sa cadence. -->
    <div v-if="titre || $slots.entete" class="flex items-baseline justify-between gap-4 mb-3 px-1">
      <h4 v-if="titre" class="text-af-encre font-semibold truncate">{{ titre }}</h4>
      <slot name="entete" />
    </div>

    <!-- Les flèches doublent le défilement tactile pour les visiteurs qui
         naviguent à la souris ou au clavier (FR-053). Le voile dégradé qui les
         accompagne dit que la piste continue au-delà du bord. -->
    <div
      v-show="peutReculer"
      :style="styleVoile"
      class="pointer-events-none absolute inset-y-0 left-0 z-10 hidden w-12 bg-linear-to-r from-af-fond to-transparent md:block"
    />
    <button
      v-show="peutReculer"
      type="button"
      :style="styleFleche"
      class="absolute left-1 top-1/2 z-20 hidden h-11 w-11 -translate-y-1/2 items-center justify-center rounded-full bg-af-surface text-af-encre shadow-lg ring-1 ring-af-bordure transition hover:scale-105 hover:text-af-chocolat focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat md:flex"
      aria-label="Contenus précédents"
      @click="defiler(-1)"
    >
      <font-awesome-icon :icon="['fas', 'chevron-left']" />
    </button>

    <div
      ref="piste"
      class="flex flex-nowrap gap-3 overflow-x-auto scrollbar-none snap-x snap-mandatory -mx-1 px-1 py-3 scroll-pl-1 scroll-smooth"
      tabindex="0"
      role="list"
      @scroll.passive="mesurer"
      @keydown.left.prevent="defiler(-1)"
      @keydown.right.prevent="defiler(1)"
    >
      <slot />
    </div>

    <div
      v-show="peutAvancer"
      :style="styleVoile"
      class="pointer-events-none absolute inset-y-0 right-0 z-10 hidden w-12 bg-linear-to-l from-af-fond to-transparent md:block"
    />
    <button
      v-show="peutAvancer"
      type="button"
      :style="styleFleche"
      class="absolute right-1 top-1/2 z-20 hidden h-11 w-11 -translate-y-1/2 items-center justify-center rounded-full bg-af-surface text-af-encre shadow-lg ring-1 ring-af-bordure transition hover:scale-105 hover:text-af-chocolat focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat md:flex"
      aria-label="Contenus suivants"
      @click="defiler(1)"
    >
      <font-awesome-icon :icon="['fas', 'chevron-right']" />
    </button>

    <p v-if="messageVide" class="text-af-atone text-sm px-1">{{ messageVide }}</p>
  </section>
</template>

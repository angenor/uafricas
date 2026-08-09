<script setup lang="ts">
/**
 * Rangée horizontale de contenus, façon catalogue (FR-022).
 *
 * Le défilement est natif (`overflow-x-auto` + `scroll-snap`) plutôt que porté
 * par un carrousel tiers : `vue3-carousel`, déjà présent dans le projet, est
 * configuré pour une diapositive à la fois avec défilement automatique, et
 * impose une feuille de style tierce sur des pages tenues au Tailwind pur.
 *
 * Le contenu de chaque carte est laissé au parent, via un slot : la rangée sert
 * aussi bien des programmes télé que des émissions radio.
 */

defineProps<{
  titre?: string
  /** Message affiché quand la rangée est vide. */
  messageVide?: string
}>()

const piste = ref<HTMLElement | null>(null)

/** Fait défiler d'environ une largeur de cadre, dans un sens ou dans l'autre. */
const defiler = (sens: -1 | 1) => {
  const el = piste.value
  if (!el) return
  el.scrollBy({ left: sens * Math.max(el.clientWidth * 0.8, 240), behavior: 'smooth' })
}
</script>

<template>
  <section class="relative group/rangee">
    <!-- Titre et méta sur la même ligne : depuis 009, une rangée est un
         PROGRAMME, et son en-tête porte son décompte d'épisodes et sa cadence. -->
    <div v-if="titre || $slots.entete" class="flex items-baseline justify-between gap-4 mb-3 px-1">
      <h4 v-if="titre" class="text-white font-semibold truncate">{{ titre }}</h4>
      <slot name="entete" />
    </div>

    <!-- Les flèches doublent le défilement tactile pour les visiteurs qui
         naviguent à la souris ou au clavier (FR-053). -->
    <button
      type="button"
      class="hidden md:flex absolute left-0 top-1/2 -translate-y-1/2 z-10 h-11 w-11 items-center justify-center rounded-full bg-black/70 text-white opacity-0 group-hover/rangee:opacity-100 focus-visible:opacity-100 hover:bg-black/90 transition-opacity"
      aria-label="Contenus précédents"
      @click="defiler(-1)"
    >
      <font-awesome-icon :icon="['fas', 'chevron-left']" />
    </button>

    <div
      ref="piste"
      class="flex flex-nowrap gap-4 overflow-x-auto scrollbar-none snap-x snap-mandatory -mx-1 px-1 pb-2 scroll-smooth"
      tabindex="0"
      role="list"
      @keydown.left.prevent="defiler(-1)"
      @keydown.right.prevent="defiler(1)"
    >
      <slot />
    </div>

    <button
      type="button"
      class="hidden md:flex absolute right-0 top-1/2 -translate-y-1/2 z-10 h-11 w-11 items-center justify-center rounded-full bg-black/70 text-white opacity-0 group-hover/rangee:opacity-100 focus-visible:opacity-100 hover:bg-black/90 transition-opacity"
      aria-label="Contenus suivants"
      @click="defiler(1)"
    >
      <font-awesome-icon :icon="['fas', 'chevron-right']" />
    </button>

    <p v-if="messageVide" class="text-gray-500 text-sm px-1">{{ messageVide }}</p>
  </section>
</template>

<style scoped>
/* Masque la barre de défilement sans désactiver le défilement lui-même.
   Tailwind v4 n'expose pas d'utilitaire pour cela. */
.scrollbar-none {
  scrollbar-width: none;
  -ms-overflow-style: none;
}
.scrollbar-none::-webkit-scrollbar {
  display: none;
}
</style>

<script setup lang="ts">
/**
 * Filtre de **fiche de support** : thématiques déclarées (US3).
 *
 * Complément de `MediaFilters` sur les pages Radio, qui ne porte que les
 * critères d'avant 09r (type, pays de rattachement, genre). La thématique est
 * d'une autre nature : le genre décrit la couleur d'antenne, la thématique est
 * déclarée par le support lui-même.
 *
 * Le filtre « territoire couvert » a été retiré : il doublonnait à l'écran le
 * « Territoire » de `MediaFilters`. La couverture (US4) reste déclarée, affichée
 * sur la fiche et exigée pour publier ; c'est seulement l'entrée de filtre qui
 * disparaît. Le paramètre `territoire` reste servi par l'API.
 *
 * Tailwind v4 pur : page publique (principe VI).
 */
import type { ThematiqueDecompte } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  thematiques: string[]
  thematiquesDisponibles?: ThematiqueDecompte[]
}>(), {
  thematiquesDisponibles: () => [],
})

const emit = defineEmits<{
  'update:thematiques': [valeur: string[]]
}>()

const panneauOuvert = ref(false)

const basculer = (id: string) => {
  emit(
    'update:thematiques',
    props.thematiques.includes(id)
      ? props.thematiques.filter(x => x !== id)
      : [...props.thematiques, id],
  )
}

/** Référentiel `media` vide : la barre disparaît plutôt que d'offrir un menu
 * sans entrée. Un thème sans support, lui, reste proposé : il s'affiche
 * « (0) », ce qui donne à voir l'étendue du catalogue. */
const utile = computed(() => props.thematiquesDisponibles.length > 0)
</script>

<template>
  <div v-if="utile" class="flex flex-wrap items-center justify-center gap-3 mt-4">
    <div v-if="thematiquesDisponibles.length" class="relative">
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-full px-4 py-2 text-sm font-semibold transition-colors cursor-pointer"
        :class="thematiques.length
          ? 'bg-af-chocolat text-white hover:opacity-90'
          : 'border border-af-bordure bg-white text-af-corps hover:border-af-chocolat hover:text-af-chocolat'"
        :aria-expanded="panneauOuvert"
        @click="panneauOuvert = !panneauOuvert"
      >
        <font-awesome-icon :icon="['fas', 'tags']" class="w-4 h-4" />
        Thématiques
        <span v-if="thematiques.length" class="rounded-full bg-white/25 px-1.5 text-xs">
          {{ thematiques.length }}
        </span>
      </button>

      <div
        v-if="panneauOuvert"
        class="absolute left-1/2 -translate-x-1/2 top-full z-20 mt-2 w-72 max-h-72 overflow-y-auto rounded-xl bg-white ring-1 ring-white/15 p-3 shadow-xl"
      >
        <div class="flex flex-wrap gap-2">
          <button
            v-for="t in thematiquesDisponibles"
            :key="t.id"
            type="button"
            class="rounded-full border px-3 py-1 text-xs transition-colors"
            :class="thematiques.includes(t.id)
              ? 'bg-af-chocolat border-af-chocolat text-af-encre font-semibold'
              : t.nombre_supports > 0
                ? 'bg-white/5 border-white/15 text-af-corps hover:border-af-chocolat'
                : 'bg-transparent border-white/10 text-af-atone hover:border-af-chocolat/60 hover:text-af-corps'"
            @click="basculer(t.id)"
          >
            {{ t.nom }} ({{ t.nombre_supports }})
          </button>
        </div>
        <button
          v-if="thematiques.length"
          type="button"
          class="mt-3 text-xs text-af-corps underline hover:text-af-encre"
          @click="emit('update:thematiques', [])"
        >
          Tout décocher
        </button>
      </div>
    </div>

  </div>
</template>

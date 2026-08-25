<script setup lang="ts">
/**
 * Thématiques déclarées et couverture territoriale d'un support (US3, US4).
 *
 * La couverture est annoncée en clair : « Toute l'Afrique » n'est pas la même
 * chose qu'une liste vide, et une fiche muette laisserait croire qu'un support
 * panafricain ne rayonne nulle part. Terminologie « territoire » à l'écran,
 * `pays` dans l'API : convention du projet.
 *
 * Tailwind v4 pur : page publique (principe VI).
 */
import type { ThematiquePublique, CouverturePublique } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  thematiques?: ThematiquePublique[]
  couverture?: CouverturePublique | null
}>(), {
  thematiques: () => [],
  couverture: null,
})

/** Rien de déclaré : le bloc entier disparaît plutôt que d'afficher deux titres
 * vides sur la fiche d'un support ancien. */
const aQuelqueChose = computed(() =>
  props.thematiques.length > 0
  || props.couverture?.couverture_continentale
  || (props.couverture?.territoires.length ?? 0) > 0,
)
</script>

<template>
  <div v-if="aQuelqueChose" class="mb-10 space-y-5">
    <div v-if="thematiques.length">
      <p class="text-xs uppercase tracking-wide text-gray-500 mb-2">Thématiques</p>
      <ul class="flex flex-wrap gap-2">
        <li
          v-for="theme in thematiques"
          :key="theme.id"
          class="rounded-full border border-white/15 bg-white/5 text-gray-200 px-3 py-1 text-sm"
        >
          {{ theme.nom }}
        </li>
      </ul>
    </div>

    <div v-if="couverture">
      <p class="text-xs uppercase tracking-wide text-gray-500 mb-2">Couverture</p>
      <p v-if="couverture.couverture_continentale" class="inline-flex items-center gap-2 text-sm text-yellow-400">
        <font-awesome-icon :icon="['fas', 'globe']" class="w-4 h-4" />
        Toute l'Afrique
      </p>
      <ul v-else-if="couverture.territoires.length" class="flex flex-wrap gap-2">
        <li
          v-for="territoire in couverture.territoires"
          :key="territoire.id"
          class="rounded-full border border-white/15 bg-white/5 text-gray-200 px-3 py-1 text-sm"
        >
          {{ territoire.nom }}
        </li>
      </ul>
    </div>
  </div>
</template>

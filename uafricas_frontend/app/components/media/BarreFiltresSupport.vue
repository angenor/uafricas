<script setup lang="ts">
/**
 * Filtres de **fiche de support** — thématiques déclarées (US3) et territoire
 * couvert (US4).
 *
 * Complément de `MediaFilters` sur les pages Radio, qui ne porte que les
 * critères d'avant 09r (type, pays de rattachement, genre). Ces deux-là sont
 * d'une autre nature : le genre décrit la couleur d'antenne, la thématique est
 * déclarée par le support lui-même ; le pays dit où il siège, la couverture dit
 * où il rayonne — une station panafricaine remonte sur **chaque** territoire
 * (FR-036), ce que le pays de siège ne dira jamais.
 *
 * Tailwind v4 pur — page publique (principe VI).
 */
import type { ThematiqueDecompte, TerritoireDecompte } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  thematiques: string[]
  territoire: string
  thematiquesDisponibles?: ThematiqueDecompte[]
  territoiresDisponibles?: TerritoireDecompte[]
  /** Supports panafricains, annoncés à part : ils remontent sur tout territoire. */
  nombreContinentales?: number
}>(), {
  thematiquesDisponibles: () => [],
  territoiresDisponibles: () => [],
  nombreContinentales: 0,
})

const emit = defineEmits<{
  'update:thematiques': [valeur: string[]]
  'update:territoire': [valeur: string]
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

/** Rien de déclaré nulle part : la barre entière disparaît plutôt que d'offrir
 * des filtres qui ne remonteraient rien. */
const utile = computed(() =>
  props.thematiquesDisponibles.length > 0 || props.territoiresDisponibles.length > 0,
)
</script>

<template>
  <div v-if="utile" class="flex flex-wrap items-center justify-center gap-3 mt-4">
    <div v-if="thematiquesDisponibles.length" class="relative">
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-full px-4 py-2 text-sm font-semibold transition-colors cursor-pointer"
        :class="thematiques.length
          ? 'bg-yellow-400 text-black hover:bg-yellow-300'
          : 'bg-white/10 text-white ring-1 ring-white/25 hover:bg-white/20'"
        :aria-expanded="panneauOuvert"
        @click="panneauOuvert = !panneauOuvert"
      >
        <font-awesome-icon :icon="['fas', 'tags']" class="w-4 h-4" />
        Thématiques
        <span v-if="thematiques.length" class="rounded-full bg-black/20 px-1.5 text-xs">
          {{ thematiques.length }}
        </span>
      </button>

      <div
        v-if="panneauOuvert"
        class="absolute left-1/2 -translate-x-1/2 top-full z-20 mt-2 w-72 max-h-72 overflow-y-auto rounded-xl bg-neutral-900 ring-1 ring-white/15 p-3 shadow-xl"
      >
        <div class="flex flex-wrap gap-2">
          <button
            v-for="t in thematiquesDisponibles"
            :key="t.id"
            type="button"
            class="rounded-full border px-3 py-1 text-xs transition-colors"
            :class="thematiques.includes(t.id)
              ? 'bg-yellow-400 border-yellow-400 text-neutral-900 font-semibold'
              : 'bg-white/5 border-white/15 text-gray-300 hover:border-yellow-400'"
            @click="basculer(t.id)"
          >
            {{ t.nom }} ({{ t.nombre_supports }})
          </button>
        </div>
        <button
          v-if="thematiques.length"
          type="button"
          class="mt-3 text-xs text-gray-400 underline hover:text-white"
          @click="emit('update:thematiques', [])"
        >
          Tout décocher
        </button>
      </div>
    </div>

    <div v-if="territoiresDisponibles.length" class="relative">
      <font-awesome-icon
        :icon="['fas', 'globe']"
        class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4"
        :class="territoire ? 'text-yellow-400' : 'text-gray-300'"
      />
      <select
        :value="territoire"
        class="appearance-none truncate rounded-full bg-white/10 text-white text-sm font-medium ring-1 ring-white/25 pl-9 pr-9 py-2 w-56 cursor-pointer hover:bg-white/20 transition-colors"
        :class="territoire ? 'ring-yellow-400 text-yellow-400' : ''"
        aria-label="Filtrer par territoire couvert"
        @change="emit('update:territoire', ($event.target as HTMLSelectElement).value)"
      >
        <option class="bg-gray-900 text-white" value="">Territoire couvert</option>
        <option
          v-for="t in territoiresDisponibles"
          :key="t.id"
          class="bg-gray-900 text-white"
          :value="t.id"
        >
          {{ t.nom }} ({{ t.nombre_supports }})
        </option>
      </select>
      <font-awesome-icon
        :icon="['fas', 'chevron-down']"
        class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 w-3 h-3 text-gray-400"
      />
    </div>

    <!-- Ces supports ne peuvent pas figurer dans les décomptes ci-dessus : ils
         remontent sur chaque territoire et les gonfleraient tous. -->
    <span v-if="nombreContinentales" class="text-xs text-gray-400">
      dont {{ nombreContinentales }} panafricain{{ nombreContinentales > 1 ? 's' : '' }}
    </span>
  </div>
</template>

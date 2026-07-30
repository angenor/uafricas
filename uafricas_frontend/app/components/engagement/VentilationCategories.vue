<script setup lang="ts">
/**
 * Ventilation des points par catégorie — Tailwind v4 pur.
 *
 * Distingue explicitement deux notions que le plancher 0 peut faire diverger :
 * — « points gagnés » : cumul du journal, toutes catégories confondues ;
 * — « solde courant » : `engagement.compte.solde_points`.
 * Sans cette distinction, l'écart serait perçu comme un bug (R2, SC-005).
 */
import { computed } from 'vue'
import type { VentilationPoints } from '~/composables/useEngagement'

const props = defineProps<{
  ventilation: VentilationPoints | null
}>()

/** Jetons de couleur (issus de la base) → classes Tailwind. */
const CLASSES: Record<string, { fond: string, texte: string, barre: string }> = {
  green: { fond: 'bg-custom-green/10', texte: 'text-custom-green', barre: 'bg-custom-green' },
  rose: { fond: 'bg-rose-50', texte: 'text-rose-600', barre: 'bg-rose-400' },
  amber: { fond: 'bg-amber-50', texte: 'text-amber-700', barre: 'bg-amber-400' },
  sky: { fond: 'bg-sky-50', texte: 'text-sky-700', barre: 'bg-sky-400' },
  violet: { fond: 'bg-violet-50', texte: 'text-violet-700', barre: 'bg-violet-400' },
  gray: { fond: 'bg-gray-100', texte: 'text-gray-600', barre: 'bg-gray-400' },
}

const classes = (couleur: string | null) => CLASSES[couleur || 'gray'] || CLASSES.gray

/** Base de la barre proportionnelle : la plus grosse catégorie positive. */
const maximum = computed(() =>
  Math.max(1, ...(props.ventilation?.categories.map(c => Math.abs(c.points)) ?? [1])),
)

const largeur = (points: number) => Math.round((Math.abs(points) / maximum.value) * 100)

/** Écart entre le cumul du journal et le solde courant (plancher 0). */
const ecart = computed(() => {
  const v = props.ventilation
  if (!v) return 0
  return v.total_gagne - v.solde_points
})
</script>

<template>
  <section v-if="ventilation && ventilation.categories.length > 0" class="space-y-4">
    <header class="flex flex-wrap items-baseline justify-between gap-2">
      <h2 class="font-oswald text-xl font-bold text-gray-900">D'où viennent mes points</h2>
      <div class="text-right text-xs text-gray-500">
        <p>
          <span class="font-semibold text-gray-800">{{ ventilation.total_gagne }}</span>
          point{{ Math.abs(ventilation.total_gagne) > 1 ? 's' : '' }} gagné{{ Math.abs(ventilation.total_gagne) > 1 ? 's' : '' }}
          au total
        </p>
        <p>
          solde courant :
          <span class="font-semibold text-custom-green">{{ ventilation.solde_points }}</span>
        </p>
      </div>
    </header>

    <!--
      Le plancher 0 empêche le solde de descendre sous zéro : un malus « perdu »
      dans ce plancher creuse un écart entre le cumul et le solde. On l'explique
      plutôt que de le masquer.
    -->
    <p
      v-if="ecart !== 0"
      class="rounded-xl bg-amber-50 px-4 py-2.5 text-xs leading-relaxed text-amber-800"
    >
      <font-awesome-icon icon="fa-solid fa-circle-info" class="mr-1" />
      Votre <strong>solde courant</strong> ({{ ventilation.solde_points }}) diffère du total
      <strong>gagné</strong> ({{ ventilation.total_gagne }}). C'est normal : un solde ne peut jamais
      descendre en dessous de zéro, les malus au-delà de cette limite ne sont donc pas reportés.
    </p>

    <ul class="grid grid-cols-1 gap-3 sm:grid-cols-2">
      <li
        v-for="cat in ventilation.categories"
        :key="cat.code || 'autres'"
        class="rounded-2xl border border-gray-100 bg-white p-4"
      >
        <div class="flex items-start gap-3">
          <span
            class="grid size-10 shrink-0 place-items-center rounded-xl"
            :class="[classes(cat.couleur).fond, classes(cat.couleur).texte]"
          >
            <font-awesome-icon :icon="`fa-solid fa-${cat.icone || 'circle-nodes'}`" />
          </span>

          <div class="min-w-0 flex-1">
            <div class="flex items-baseline justify-between gap-2">
              <p class="truncate text-sm font-semibold text-gray-800">{{ cat.libelle }}</p>
              <p class="shrink-0 font-oswald text-lg font-bold" :class="classes(cat.couleur).texte">
                {{ cat.points > 0 ? '+' : '' }}{{ cat.points }}
              </p>
            </div>
            <p class="text-xs text-gray-400">
              {{ cat.nombre_mouvements }} mouvement{{ cat.nombre_mouvements > 1 ? 's' : '' }}
            </p>
            <div class="mt-2 h-1.5 w-full overflow-hidden rounded-full bg-gray-100">
              <div
                class="h-full rounded-full transition-all duration-500"
                :class="classes(cat.couleur).barre"
                :style="{ width: largeur(cat.points) + '%' }"
              />
            </div>
          </div>
        </div>
      </li>
    </ul>
  </section>
</template>

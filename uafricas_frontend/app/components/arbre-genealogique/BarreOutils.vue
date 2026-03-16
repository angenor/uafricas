<script setup lang="ts">
import type { ModeVue } from '~/mocks/arbre-genealogique'
import type { NoeudArbre } from '~/composables/useLayoutArbre'
import ChampRecherche from '~/components/arbre-genealogique/ChampRecherche.vue'

defineProps<{
  mode: ModeVue
  nbBranchesIncompletes?: number
  graphe?: Map<string, NoeudArbre>
}>()

const emit = defineEmits<{
  'mode-change': [mode: ModeVue]
  'reset-view': []
  'recherche-selectionne': [rattachementId: string]
}>()

const modes: { valeur: ModeVue; label: string; icone: string }[] = [
  { valeur: 'complet', label: 'Vue complète', icone: 'sitemap' },
  { valeur: 'ascendant', label: 'Ancêtres', icone: 'arrow-up' },
  { valeur: 'descendant', label: 'Descendants', icone: 'arrow-down' },
]
</script>

<template>
  <div class="flex flex-wrap items-center justify-between gap-2 border-b border-stone-200 bg-white px-4 py-2">
    <!-- Gauche : modes + incomplétude -->
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-1 rounded-lg bg-stone-100 p-0.5">
        <button
          v-for="m in modes"
          :key="m.valeur"
          :class="[
            'flex items-center gap-1.5 rounded-md px-3 py-1.5 text-sm font-medium transition-all',
            mode === m.valeur
              ? 'bg-[var(--color-custom-green)] text-white shadow-sm'
              : 'text-stone-600 hover:text-stone-800',
          ]"
          @click="emit('mode-change', m.valeur)"
        >
          <font-awesome-icon :icon="['fas', m.icone]" class="text-xs" />
          <span class="max-sm:hidden">{{ m.label }}</span>
        </button>
      </div>

      <!-- Compteur branches incomplètes -->
      <div
        v-if="nbBranchesIncompletes && nbBranchesIncompletes > 0"
        class="flex items-center gap-1.5 rounded-full bg-amber-50 px-3 py-1 text-xs font-medium text-amber-700 max-sm:hidden"
      >
        <span class="flex h-4 w-4 items-center justify-center rounded-full bg-amber-400 text-[10px] text-white">
          {{ nbBranchesIncompletes }}
        </span>
        <span>branche{{ nbBranchesIncompletes > 1 ? 's' : '' }} à compléter</span>
      </div>
    </div>

    <!-- Droite : recherche + réinitialiser -->
    <div class="flex items-center gap-2">
      <!-- Champ de recherche -->
      <ChampRecherche
        v-if="graphe"
        :graphe="graphe"
        @resultat-selectionne="emit('recherche-selectionne', $event)"
      />

      <!-- Réinitialiser -->
      <button
        class="flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-sm text-stone-500 transition-colors hover:bg-stone-100 hover:text-stone-700"
        @click="emit('reset-view')"
      >
        <font-awesome-icon :icon="['fas', 'compress-arrows-alt']" class="text-xs" />
        <span class="max-sm:hidden">Réinitialiser</span>
      </button>
    </div>
  </div>
</template>

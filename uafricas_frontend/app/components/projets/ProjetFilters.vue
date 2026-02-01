<template>
  <div class="bg-white rounded-2xl shadow-xl p-6 sticky top-24">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <h3 class="text-lg font-semibold text-gray-900 flex items-center gap-2">
        <font-awesome-icon :icon="['fas', 'filter']" class="w-5 h-5 text-custom-green" />
        Filtres
      </h3>
      <button
        class="text-sm text-gray-500 hover:text-custom-green transition-colors"
        @click="$emit('reset')"
      >
        Réinitialiser
      </button>
    </div>

    <!-- Filtre par pays -->
    <div class="mb-6">
      <label class="block text-sm font-medium text-gray-700 mb-2">
        <font-awesome-icon :icon="['fas', 'globe-africa']" class="w-4 h-4 mr-2 text-gray-400" />
        Pays
      </label>
      <select
        v-model="localFiltres.pays"
        class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all"
        @change="emitChange"
      >
        <option v-for="pays in paysAfricains" :key="pays.value" :value="pays.value">
          {{ pays.label }}
        </option>
      </select>
    </div>

    <!-- Filtre par budget -->
    <div class="mb-6">
      <label class="block text-sm font-medium text-gray-700 mb-2">
        <font-awesome-icon :icon="['fas', 'coins']" class="w-4 h-4 mr-2 text-gray-400" />
        Budget maximum
      </label>
      <select
        v-model="localFiltres.budgetMax"
        class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all"
        @change="emitChange"
      >
        <option v-for="budget in budgets" :key="budget.value" :value="budget.value">
          {{ budget.label }}
        </option>
      </select>
    </div>

    <!-- Filtre par durée -->
    <div class="mb-6">
      <label class="block text-sm font-medium text-gray-700 mb-2">
        <font-awesome-icon :icon="['fas', 'clock']" class="w-4 h-4 mr-2 text-gray-400" />
        Durée du projet
      </label>
      <select
        v-model="localFiltres.duree"
        class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all"
        @change="emitChange"
      >
        <option v-for="duree in durees" :key="duree.value" :value="duree.value">
          {{ duree.label }}
        </option>
      </select>
    </div>

    <!-- Tri -->
    <div class="mb-6">
      <label class="block text-sm font-medium text-gray-700 mb-2">
        <font-awesome-icon :icon="['fas', 'sort']" class="w-4 h-4 mr-2 text-gray-400" />
        Trier par
      </label>
      <select
        v-model="localFiltres.sortBy"
        class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all"
        @change="emitChange"
      >
        <option v-for="option in sortOptions" :key="option.value" :value="option.value">
          {{ option.label }}
        </option>
      </select>
    </div>

    <!-- Statistiques -->
    <div class="pt-6 border-t border-gray-200">
      <h4 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4">Statistiques</h4>
      <div class="space-y-3">
        <div class="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
          <span class="text-sm text-gray-600">Total projets</span>
          <span class="font-bold text-gray-900 text-lg">{{ totalProjets }}</span>
        </div>
        <div class="flex justify-between items-center p-3 bg-emerald-50 rounded-lg">
          <span class="text-sm text-emerald-700">Résultats filtrés</span>
          <span class="font-bold text-emerald-600 text-lg">{{ filteredCount }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { paysAfricains, budgets, durees, sortOptions, type FiltresProjet } from '~/mocks/projets'

const props = defineProps<{
  totalProjets: number
  filteredCount: number
  modelValue: FiltresProjet
}>()

const emit = defineEmits<{
  'update:modelValue': [filtres: FiltresProjet]
  reset: []
}>()

const localFiltres = ref<FiltresProjet>({ ...props.modelValue })

watch(
  () => props.modelValue,
  (newValue) => {
    localFiltres.value = { ...newValue }
  },
  { deep: true }
)

const emitChange = () => {
  emit('update:modelValue', { ...localFiltres.value })
}
</script>

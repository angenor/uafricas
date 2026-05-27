<template>
  <!-- Overlay -->
  <Transition
    enter-active-class="transition-opacity duration-300"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition-opacity duration-300"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      v-if="isOpen"
      class="fixed inset-0 bg-black/50 z-40"
      @click="$emit('close')"
    />
  </Transition>

  <!-- Drawer -->
  <Transition
    enter-active-class="transition-transform duration-300"
    enter-from-class="-translate-x-full"
    enter-to-class="translate-x-0"
    leave-active-class="transition-transform duration-300"
    leave-from-class="translate-x-0"
    leave-to-class="-translate-x-full"
  >
    <div
      v-if="isOpen"
      class="fixed top-0 left-0 h-full w-80 max-w-[85vw] bg-white z-50 shadow-2xl overflow-y-auto"
    >
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-gray-200">
        <h3 class="text-lg font-semibold text-gray-900 flex items-center gap-2">
          <font-awesome-icon :icon="['fas', 'filter']" class="w-5 h-5 text-custom-green" />
          Filtres
        </h3>
        <button
          class="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
          @click="$emit('close')"
        >
          <font-awesome-icon :icon="['fas', 'times']" class="w-5 h-5" />
        </button>
      </div>

      <!-- Filtres -->
      <div class="p-4 space-y-6">
        <!-- Filtre par pays -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">
            <font-awesome-icon :icon="['fas', 'globe-africa']" class="w-4 h-4 mr-2 text-gray-400" />
            Territoire
          </label>
          <select
            v-model="localFiltres.pays"
            class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green focus:border-transparent"
            @change="emitChange"
          >
            <option v-for="pays in paysAfricains" :key="pays.value" :value="pays.value">
              {{ pays.label }}
            </option>
          </select>
        </div>

        <!-- Filtre par budget -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">
            <font-awesome-icon :icon="['fas', 'coins']" class="w-4 h-4 mr-2 text-gray-400" />
            Budget maximum
          </label>
          <select
            v-model="localFiltres.budgetMax"
            class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green focus:border-transparent"
            @change="emitChange"
          >
            <option v-for="budget in budgets" :key="budget.value" :value="budget.value">
              {{ budget.label }}
            </option>
          </select>
        </div>

        <!-- Filtre par durée -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">
            <font-awesome-icon :icon="['fas', 'clock']" class="w-4 h-4 mr-2 text-gray-400" />
            Durée du projet
          </label>
          <select
            v-model="localFiltres.duree"
            class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green focus:border-transparent"
            @change="emitChange"
          >
            <option v-for="duree in durees" :key="duree.value" :value="duree.value">
              {{ duree.label }}
            </option>
          </select>
        </div>

        <!-- Tri -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">
            <font-awesome-icon :icon="['fas', 'sort']" class="w-4 h-4 mr-2 text-gray-400" />
            Trier par
          </label>
          <select
            v-model="localFiltres.sortBy"
            class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green focus:border-transparent"
            @change="emitChange"
          >
            <option v-for="option in sortOptions" :key="option.value" :value="option.value">
              {{ option.label }}
            </option>
          </select>
        </div>

        <!-- Statistiques -->
        <div class="pt-4 border-t border-gray-200">
          <div class="space-y-3">
            <div class="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
              <span class="text-sm text-gray-600">Total projets</span>
              <span class="font-bold text-gray-900">{{ totalProjets }}</span>
            </div>
            <div class="flex justify-between items-center p-3 bg-emerald-50 rounded-lg">
              <span class="text-sm text-emerald-700">Résultats</span>
              <span class="font-bold text-emerald-600">{{ filteredCount }}</span>
            </div>
          </div>
        </div>

        <!-- Boutons d'action -->
        <div class="space-y-3 pt-4">
          <button
            class="w-full p-3 bg-gradient-to-r from-custom-green to-emerald-600 text-white rounded-xl font-medium hover:shadow-lg transition-all"
            @click="$emit('close')"
          >
            Appliquer les filtres
          </button>
          <button
            class="w-full p-3 bg-gray-100 text-gray-700 rounded-xl font-medium hover:bg-gray-200 transition-all"
            @click="resetAndClose"
          >
            Réinitialiser
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { PAYS_PROJETS, BUDGETS, DUREES, OPTIONS_TRI, type FiltresProjetPage } from '~/composables/useProjets'

const paysAfricains = PAYS_PROJETS
const budgets = BUDGETS
const durees = DUREES
const sortOptions = OPTIONS_TRI

const props = defineProps<{
  isOpen: boolean
  totalProjets: number
  filteredCount: number
  modelValue: FiltresProjetPage
}>()

const emit = defineEmits<{
  'update:modelValue': [filtres: FiltresProjetPage]
  close: []
  reset: []
}>()

const localFiltres = ref<FiltresProjetPage>({ ...props.modelValue })

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

const resetAndClose = () => {
  emit('reset')
  emit('close')
}
</script>

<template>
  <div class="bg-white rounded-2xl shadow-xl p-6 sticky top-24">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <h3 class="text-lg font-semibold text-gray-900 flex items-center gap-2">
        <font-awesome-icon :icon="['fas', 'filter']" class="w-5 h-5 text-blue-500" />
        Filtres
      </h3>
      <button
        class="text-sm text-gray-500 hover:text-blue-500 transition-colors"
        @click="$emit('reset')"
      >
        Réinitialiser
      </button>
    </div>

    <!-- Filtre par langue -->
    <div class="mb-6">
      <label class="block text-sm font-medium text-gray-700 mb-2">
        <font-awesome-icon :icon="['fas', 'language']" class="w-4 h-4 mr-2 text-gray-400" />
        Langue
      </label>
      <select
        v-model="localFiltres.langue"
        class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
        @change="emitChange"
      >
        <option value="">Toutes les langues</option>
        <option v-for="langue in langues" :key="langue" :value="langue">
          {{ langue }}
        </option>
      </select>
    </div>

    <!-- Filtre par pays d'origine (feature 001-afrolang-pays-origine) -->
    <div v-if="pays.length > 0" class="mb-6">
      <label class="block text-sm font-medium text-gray-700 mb-2">
        <font-awesome-icon :icon="['fas', 'globe']" class="w-4 h-4 mr-2 text-gray-400" />
        Territoire d'origine
      </label>
      <select
        v-model="localFiltres.pays_id"
        class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
        @change="emitChange"
      >
        <option value="">Tous les territoires</option>
        <option v-for="p in pays" :key="p.id" :value="p.id">
          {{ p.nom }}
        </option>
      </select>
    </div>

    <!-- Statistiques -->
    <div class="pt-6 border-t border-gray-200">
      <h4 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4">Statistiques</h4>
      <div class="space-y-3">
        <div class="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
          <span class="text-sm text-gray-600">Total salles</span>
          <span class="font-bold text-gray-900 text-lg">{{ totalSalles }}</span>
        </div>
        <div class="flex justify-between items-center p-3 bg-blue-50 rounded-lg">
          <span class="text-sm text-blue-700">Résultats filtrés</span>
          <span class="font-bold text-blue-600 text-lg">{{ filteredCount }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PaysOrigineLight, SalleFiltres } from '~/composables/useAfrolang'

const props = defineProps<{
  totalSalles: number
  filteredCount: number
  modelValue: SalleFiltres
  langues: string[]
  pays: PaysOrigineLight[]
}>()

const emit = defineEmits<{
  'update:modelValue': [filtres: SalleFiltres]
  reset: []
}>()

const localFiltres = ref<SalleFiltres>({ ...props.modelValue })

watch(
  () => props.modelValue,
  (newValue) => {
    localFiltres.value = { ...newValue }
  },
  { deep: true },
)

const emitChange = () => {
  emit('update:modelValue', { ...localFiltres.value })
}
</script>

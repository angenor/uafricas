<template>
  <div class="bg-white rounded-xl shadow-md p-6 sticky top-24">
    <h3 class="font-semibold text-gray-800 mb-6 flex items-center gap-2">
      <font-awesome-icon :icon="['fas', 'filter']" class="w-4 h-4 text-custom-chocolat" />
      Filtres
    </h3>

    <!-- Domaine : filtré via la barre de puces de la page (dédup, audit #8) -->

    <!-- Pays -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-gray-700 mb-3">Territoire</h4>
      <select
        :value="modelValue.pays"
        @change="updatePays"
        class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 bg-white"
      >
        <option value="">Tous les territoires</option>
        <option v-for="p in pays" :key="p" :value="p">
          {{ p }}
        </option>
      </select>
    </div>

    <!-- Separateur -->
    <hr class="my-6 border-gray-100" />

    <!-- Tri -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-gray-700 mb-3">Trier par</h4>
      <select
        :value="modelValue.tri"
        @change="updateTri"
        class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-orange-500 bg-white"
      >
        <option value="recent">Plus récent</option>
        <option value="ancien">Plus ancien</option>
        <option value="titre">Titre (A-Z)</option>
      </select>
    </div>

    <!-- Bouton reinitialiser -->
    <button
      @click="$emit('reset')"
      class="w-full py-2.5 text-sm font-medium text-gray-600 bg-gray-100 rounded-lg hover:bg-gray-200 transition-colors flex items-center justify-center gap-2"
    >
      <font-awesome-icon :icon="['fas', 'rotate-left']" class="w-3 h-3" />
      Réinitialiser les filtres
    </button>
  </div>
</template>

<script setup lang="ts">
import { PAYS_AFRICAINS } from '~/composables/useAfricantives'

export interface FiltresAfricantive {
  domaine: string
  pays: string
  recherche: string
  tri: string
}

const props = defineProps<{
  modelValue: FiltresAfricantive
}>()

const emit = defineEmits<{
  'update:modelValue': [value: FiltresAfricantive]
  'reset': []
}>()

const pays = PAYS_AFRICAINS

const updatePays = (event: Event) => {
  emit('update:modelValue', {
    ...props.modelValue,
    pays: (event.target as HTMLSelectElement).value,
  })
}

const updateTri = (event: Event) => {
  emit('update:modelValue', {
    ...props.modelValue,
    tri: (event.target as HTMLSelectElement).value,
  })
}
</script>

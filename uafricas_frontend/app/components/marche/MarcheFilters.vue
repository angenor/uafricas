<template>
  <div class="bg-white rounded-xl shadow-md p-6 sticky top-24">
    <h3 class="font-semibold text-gray-800 mb-6 flex items-center gap-2">
      <font-awesome-icon :icon="['fas', 'filter']" class="w-4 h-4 text-custom-green" />
      Filtres
    </h3>

    <!-- Type d'échange -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-gray-700 mb-3">Type d'échange</h4>
      <div class="space-y-2">
        <label
          v-for="type in typesEchange"
          :key="type.value"
          class="flex items-center justify-between cursor-pointer group"
        >
          <div class="flex items-center gap-3">
            <input
              type="checkbox"
              :value="type.value"
              :checked="isTypeSelected(type.value)"
              @change="toggleType(type.value)"
              class="w-4 h-4 text-emerald-600 border-gray-300 rounded focus:ring-3 focus:ring-emerald-500"
            />
            <span class="text-gray-600 group-hover:text-gray-900">{{ type.label }}</span>
          </div>
          <span class="text-xs text-gray-400 bg-gray-100 px-2 py-0.5 rounded-full">
            {{ getCountForType(type.value) }}
          </span>
        </label>
      </div>
    </div>

    <!-- Séparateur -->
    <hr class="my-6 border-gray-100" />

    <!-- Fourchette de prix -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-gray-700 mb-3">Fourchette de prix (FCFA)</h4>
      <div class="flex gap-2">
        <div class="flex-1">
          <input
            type="number"
            :value="modelValue.prixMin ?? ''"
            @input="updatePrixMin"
            placeholder="Min"
            min="0"
            class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
          />
        </div>
        <span class="self-center text-gray-400">-</span>
        <div class="flex-1">
          <input
            type="number"
            :value="modelValue.prixMax ?? ''"
            @input="updatePrixMax"
            placeholder="Max"
            min="0"
            class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500"
          />
        </div>
      </div>
    </div>

    <!-- Séparateur -->
    <hr class="my-6 border-gray-100" />

    <!-- Tri -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-gray-700 mb-3">Trier par</h4>
      <select
        :value="modelValue.tri"
        @change="updateTri"
        class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 bg-white"
      >
        <option value="recent">Plus récent</option>
        <option value="price-asc">Prix croissant</option>
        <option value="price-desc">Prix décroissant</option>
      </select>
    </div>

    <!-- Bouton réinitialiser -->
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
import {
  TYPES_ECHANGE,
  getCountByType,
  type FiltresAnnonce,
  type TypeEchange,
  type AnnonceAPI,
} from '~/composables/useMarcheAfricain'

const props = defineProps<{
  modelValue: FiltresAnnonce
  annonces: AnnonceAPI[]
}>()

const emit = defineEmits<{
  'update:modelValue': [value: FiltresAnnonce]
  'reset': []
}>()

const typesEchange = TYPES_ECHANGE

const isTypeSelected = (type: TypeEchange): boolean => {
  return props.modelValue.typesEchange.includes(type)
}

const getCountForType = (type: TypeEchange): number => {
  return getCountByType(type, props.annonces)
}

const toggleType = (type: TypeEchange) => {
  const newTypes = isTypeSelected(type)
    ? props.modelValue.typesEchange.filter(t => t !== type)
    : [...props.modelValue.typesEchange, type]

  emit('update:modelValue', {
    ...props.modelValue,
    typesEchange: newTypes,
  })
}

const updatePrixMin = (event: Event) => {
  const value = (event.target as HTMLInputElement).value
  emit('update:modelValue', {
    ...props.modelValue,
    prixMin: value ? Number(value) : null,
  })
}

const updatePrixMax = (event: Event) => {
  const value = (event.target as HTMLInputElement).value
  emit('update:modelValue', {
    ...props.modelValue,
    prixMax: value ? Number(value) : null,
  })
}

const updateTri = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value as FiltresAnnonce['tri']
  emit('update:modelValue', {
    ...props.modelValue,
    tri: value,
  })
}
</script>

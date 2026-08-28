<template>
  <div class="bg-white rounded-lg shadow-md p-6 sticky top-24">
    <h3 class="font-semibold text-af-encre mb-6 flex items-center gap-2">
      <font-awesome-icon :icon="['fas', 'filter']" class="w-4 h-4 text-af-vert" />
      Filtres
    </h3>

    <!-- Type d'échange -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-af-corps mb-3">Type d'échange</h4>
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
              class="w-4 h-4 text-af-vert border-af-bordure rounded focus:ring-3 focus:ring-af-vert"
            />
            <span class="text-af-corps group-hover:text-af-encre">{{ type.label }}</span>
          </div>
          <span class="text-xs text-af-atone-2 bg-af-fond px-2 py-0.5 rounded-full">
            {{ getCountForType(type.value) }}
          </span>
        </label>
      </div>
    </div>

    <!-- Séparateur -->
    <hr class="my-6 border-af-bordure" />

    <!-- Fourchette de prix -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-af-corps mb-3">Fourchette de prix (FCFA)</h4>
      <div class="flex gap-2">
        <div class="flex-1">
          <input
            type="number"
            :value="modelValue.prixMin ?? ''"
            @input="updatePrixMin"
            placeholder="Min"
            min="0"
            class="w-full px-3 py-2 text-sm border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-af-vert"
          />
        </div>
        <span class="self-center text-af-atone-2">-</span>
        <div class="flex-1">
          <input
            type="number"
            :value="modelValue.prixMax ?? ''"
            @input="updatePrixMax"
            placeholder="Max"
            min="0"
            class="w-full px-3 py-2 text-sm border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-af-vert"
          />
        </div>
      </div>
    </div>

    <!-- Séparateur -->
    <hr class="my-6 border-af-bordure" />

    <!-- Tri -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-af-corps mb-3">Trier par</h4>
      <select
        :value="modelValue.tri"
        @change="updateTri"
        class="w-full px-3 py-2 text-sm border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-af-vert bg-white"
      >
        <option value="recent">Plus récent</option>
        <option value="price-asc">Prix croissant</option>
        <option value="price-desc">Prix décroissant</option>
      </select>
    </div>

    <!-- Bouton réinitialiser -->
    <button
      @click="$emit('reset')"
      class="w-full py-2.5 text-sm font-medium text-af-corps bg-af-fond rounded-lg hover:bg-af-bordure transition-colors flex items-center justify-center gap-2"
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

<template>
  <div class="flex flex-wrap gap-2 mb-8">
    <button
      v-for="cat in categories"
      :key="cat.key"
      @click="$emit('select', cat.key)"
      class="px-4 py-2 rounded-full text-sm font-medium transition-all duration-200"
      :class="activeFilter === cat.key
        ? 'bg-gradient-to-r from-emerald-500 to-teal-500 text-white shadow-lg shadow-emerald-500/30'
        : 'bg-white text-gray-600 hover:bg-emerald-50 hover:text-emerald-700 border border-gray-200'"
    >
      {{ cat.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { CATEGORIES, type Categorie } from '~/mocks/marche-africain'

defineProps<{
  activeFilter: Categorie | 'Tout'
}>()

defineEmits<{
  select: [category: Categorie | 'Tout']
}>()

const categories = CATEGORIES
</script>

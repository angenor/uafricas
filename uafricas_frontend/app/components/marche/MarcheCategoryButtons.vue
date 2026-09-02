<template>
  <div class="flex flex-wrap gap-2 mb-8">
    <button
      v-for="cat in categories"
      :key="cat.key"
      @click="$emit('select', cat.key)"
      class="px-4 py-2 rounded-full text-sm font-medium transition-all duration-200"
      :class="activeFilter === cat.key
        ? 'bg-gradient-to-r from-af-vert to-af-vert text-white shadow-lg shadow-af-vert/30'
        : 'bg-white text-af-corps hover:bg-af-vert/5 hover:text-af-vert border border-af-bordure'"
    >
      {{ cat.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { CATEGORIES, type Categorie } from '~/composables/useMarcheAfricain'

defineProps<{
  activeFilter: Categorie | 'Tout'
}>()

defineEmits<{
  select: [category: Categorie | 'Tout']
}>()

const categories = CATEGORIES
</script>

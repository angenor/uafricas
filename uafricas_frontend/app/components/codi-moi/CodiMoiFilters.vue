<template>
  <div class="max-w-5xl mx-auto -mt-6 relative z-10 px-4">
    <div class="bg-white rounded-xl shadow-lg p-3 flex flex-col sm:flex-row sm:items-center gap-2">
      <!-- Champ de recherche avec icône intégrée -->
      <div class="relative flex-1">
        <font-awesome-icon
          icon="fa-solid fa-search"
          class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm"
        />
        <input
          :value="searchKeywords"
          @input="emit('update:searchKeywords', ($event.target as HTMLInputElement).value)"
          type="text"
          class="w-full pl-9 pr-3 py-2 text-sm border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all"
          placeholder="Rechercher un proverbe, une citation, un auteur..."
        />
      </div>

      <!-- Filtre pays -->
      <select
        :value="searchPays"
        @change="emit('update:searchPays', ($event.target as HTMLSelectElement).value)"
        class="px-3 py-2 text-sm border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all sm:w-44 bg-white"
      >
        <option value="">Tous les territoires</option>
        <option v-for="pays in PAYS_AFRICAINS" :key="pays" :value="pays">{{ pays }}</option>
      </select>

      <!-- Catégories en pills compactes -->
      <div class="flex flex-wrap gap-1.5 sm:border-l sm:border-gray-200 sm:pl-2">
        <button
          v-for="category in CATEGORIES_POST"
          :key="category.value"
          @click="emit('update:activeCategory', activeCategory === category.value ? '' : category.value)"
          class="px-3 py-1.5 rounded-full text-xs cursor-pointer transition-all duration-200 whitespace-nowrap"
          :class="
            activeCategory === category.value
              ? 'bg-custom-chocolat text-white'
              : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
          "
        >
          <font-awesome-icon :icon="category.icon" class="mr-1" />
          {{ category.label }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { CATEGORIES_POST, PAYS_AFRICAINS } from '~/composables/useCodiMoi'

defineProps<{
  activeCategory: string
  searchKeywords: string
  searchPays: string
}>()

const emit = defineEmits<{
  'update:activeCategory': [value: string]
  'update:searchKeywords': [value: string]
  'update:searchPays': [value: string]
}>()
</script>

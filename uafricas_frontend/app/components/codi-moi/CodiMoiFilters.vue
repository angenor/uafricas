<template>
  <div class="max-w-4xl mx-auto -mt-8 relative z-10 px-4">
    <div class="bg-white rounded-xl shadow-xl p-5 transform transition-all hover:shadow-2xl">
      <!-- Barre de recherche -->
      <div class="flex flex-col md:flex-row gap-3">
        <div class="flex-1">
          <input
            :value="searchKeywords"
            @input="emit('update:searchKeywords', ($event.target as HTMLInputElement).value)"
            type="text"
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all"
            placeholder="Rechercher un proverbe, une citation, un auteur..."
          />
        </div>
        <select
          :value="searchPays"
          @change="emit('update:searchPays', ($event.target as HTMLSelectElement).value)"
          class="px-4 py-3 border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all md:w-48 bg-white"
        >
          <option value="">Tous les pays</option>
          <option v-for="pays in PAYS_AFRICAINS" :key="pays" :value="pays">{{ pays }}</option>
        </select>
        <button
          class="bg-gradient-to-r from-custom-green to-green-600 hover:from-green-600 hover:to-custom-green text-white px-6 py-3 rounded-lg transition-all duration-300 transform hover:scale-105 focus:outline-hidden focus:ring-2 focus:ring-custom-green flex items-center justify-center"
        >
          <font-awesome-icon icon="fa-solid fa-search" class="mr-2" />
          Recherche
        </button>
      </div>

      <!-- Catégories en pills -->
      <div class="flex flex-wrap mt-3 gap-2">
        <button
          v-for="category in CATEGORIES_POST"
          :key="category.value"
          @click="emit('update:activeCategory', activeCategory === category.value ? '' : category.value)"
          class="px-4 py-2 rounded-full text-sm cursor-pointer transition-all duration-200"
          :class="
            activeCategory === category.value
              ? 'bg-custom-chocolat text-white'
              : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
          "
        >
          <font-awesome-icon :icon="category.icon" class="mr-1.5" />
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

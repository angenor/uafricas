<template>
  <div class="bg-white rounded-xl shadow-md border border-gray-100 p-6 mb-6">
    <div class="flex items-center justify-between mb-4">
      <h2 class="text-xl font-bold text-gray-800">Catégories</h2>
      <button
        @click="emit('createPost')"
        class="flex items-center space-x-2 px-4 py-2 bg-custom-green text-white rounded-lg hover:bg-green-600 transition-colors text-sm"
      >
        <font-awesome-icon icon="fa-solid fa-plus" />
        <span>Nouveau post</span>
      </button>
    </div>
    <div class="flex flex-wrap gap-2">
      <button
        v-for="category in CATEGORIES_POST"
        :key="category.value"
        @click="emit('update:activeCategory', category.value)"
        class="px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 border-2"
        :class="
          activeCategory === category.value
            ? 'bg-custom-green text-white border-custom-green shadow-md'
            : 'bg-gray-50 text-gray-600 border-gray-200 hover:bg-gray-100'
        "
      >
        <font-awesome-icon :icon="category.icon" class="mr-2" />
        {{ category.label }}
      </button>
    </div>
  </div>

  <!-- Options d'affichage -->
  <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-4 mb-6">
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center space-x-3">
        <div class="bg-custom-green p-2 rounded-lg">
          <font-awesome-icon icon="fa-solid fa-sliders" class="text-white text-sm" />
        </div>
        <span class="font-medium text-gray-700">Options d'affichage</span>
      </div>

      <!-- Profil utilisateur compact -->
      <div class="flex items-center bg-gray-50 rounded-lg p-2">
        <img
          class="h-8 w-8 rounded-full border border-custom-green object-cover"
          :src="userPhoto || 'https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png'"
        />
        <div class="ml-2 text-sm">
          <div class="font-medium text-gray-800">{{ userName }}</div>
        </div>
      </div>
    </div>

    <!-- Boutons de contrôle -->
    <div class="flex flex-wrap gap-2">
      <button
        @click="showFilters = !showFilters"
        class="flex items-center space-x-2 px-3 py-1.5 rounded-md text-sm transition-all duration-300"
        :class="showFilters
          ? 'bg-custom-green text-white'
          : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
      >
        <font-awesome-icon icon="fa-solid fa-tags" class="text-xs" />
        <span>Filtres</span>
        <font-awesome-icon :icon="showFilters ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'" class="text-xs" />
      </button>

      <button
        @click="showSearch = !showSearch"
        class="flex items-center space-x-2 px-3 py-1.5 rounded-md text-sm transition-all duration-300"
        :class="showSearch
          ? 'bg-custom-green text-white'
          : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
      >
        <font-awesome-icon icon="fa-solid fa-search" class="text-xs" />
        <span>Recherche</span>
        <font-awesome-icon :icon="showSearch ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'" class="text-xs" />
      </button>
    </div>

    <!-- Section Filtres rapides -->
    <div v-if="showFilters" class="mt-4 transform transition-all duration-300 ease-out">
      <div class="bg-gray-50 rounded-lg p-4">
        <h4 class="font-medium text-gray-800 mb-3">Filtres rapides</h4>
        <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
          <label
            v-for="filter in quickFilters"
            :key="filter.key"
            class="flex items-center space-x-2 text-sm cursor-pointer"
          >
            <input
              type="checkbox"
              class="w-3 h-3 text-custom-green bg-gray-100 border-gray-300 rounded focus:ring-custom-green"
              :checked="filters[filter.key as keyof typeof filters]"
              @change="updateFilter(filter.key, ($event.target as HTMLInputElement).checked)"
            />
            <span class="text-gray-700">{{ filter.label }}</span>
          </label>
        </div>
      </div>
    </div>

    <!-- Section Recherche -->
    <div v-if="showSearch" class="mt-4 transform transition-all duration-300 ease-out">
      <div class="bg-gray-50 rounded-lg p-4">
        <h4 class="font-medium text-gray-800 mb-3">Recherche avancée</h4>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
          <div>
            <input
              type="text"
              :value="searchKeywords"
              @input="emit('update:searchKeywords', ($event.target as HTMLInputElement).value)"
              class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-custom-green focus:border-custom-green"
              placeholder="Mots-clés..."
            />
          </div>
          <div>
            <select
              :value="searchPays"
              @change="emit('update:searchPays', ($event.target as HTMLSelectElement).value)"
              class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-custom-green focus:border-custom-green"
            >
              <option value="">Tous les pays</option>
              <option v-for="pays in PAYS_AFRICAINS" :key="pays" :value="pays">{{ pays }}</option>
            </select>
          </div>
        </div>
        <div class="mt-3 flex justify-end">
          <button
            @click="emit('applySearch')"
            class="px-4 py-2 text-sm bg-custom-green text-white rounded-md hover:bg-green-600 transition-colors"
          >
            Appliquer
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { CATEGORIES_POST, PAYS_AFRICAINS } from '~/mocks/codi-moi'

const props = defineProps<{
  activeCategory: string
  userName: string
  userPhoto: string | null
  filters: {
    mesPublications: boolean
    bonnesPratiques: boolean
    citations: boolean
    proverbesAdages: boolean
    ressourcesHistoriques: boolean
  }
  searchKeywords: string
  searchPays: string
}>()

const emit = defineEmits<{
  'update:activeCategory': [value: string]
  'update:filters': [value: typeof props.filters]
  'update:searchKeywords': [value: string]
  'update:searchPays': [value: string]
  createPost: []
  applySearch: []
}>()

const showFilters = ref(false)
const showSearch = ref(false)

const quickFilters = [
  { key: 'mesPublications', label: 'Mes publications' },
  { key: 'bonnesPratiques', label: 'Bonnes pratiques' },
  { key: 'citations', label: 'Citations' },
  { key: 'proverbesAdages', label: 'Proverbes/Adages' },
  { key: 'ressourcesHistoriques', label: 'Ressources historiques' }
]

const updateFilter = (key: string, value: boolean) => {
  emit('update:filters', { ...props.filters, [key]: value })
}
</script>

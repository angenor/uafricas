<template>
  <div class="bg-gradient-to-r from-gray-50 to-white py-4 border-b border-gray-200 relative z-50 shadow-lg">
    <div class="container mx-auto px-4">
      <!-- En-tête du menu -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center space-x-4">
          <div class="bg-custom-green p-2 rounded-lg">
            <font-awesome-icon icon="fa-solid fa-filter" class="text-white text-lg" />
          </div>
          <div>
            <h2 class="font-bold text-xl text-gray-800">Menu de navigation</h2>
            <p class="text-sm text-gray-600">Filtrez et organisez vos contenus</p>
          </div>
        </div>

        <!-- Profil utilisateur -->
        <div class="flex items-center bg-white rounded-xl p-3 shadow-md border border-gray-100">
          <div class="relative">
            <img
              class="h-12 w-12 rounded-full border-2 border-custom-green object-cover"
              :src="userPhoto || 'https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png'"
            />
            <div class="absolute -top-1 -right-1 bg-green-500 h-4 w-4 rounded-full border-2 border-white"></div>
          </div>
          <div class="ml-3">
            <div class="font-bold text-gray-800">{{ userName }}</div>
            <div class="text-sm text-gray-500">{{ userEmail }}</div>
          </div>
        </div>
      </div>

      <!-- Boutons de contrôle -->
      <div class="flex flex-wrap gap-3 mb-4">
        <button
          @click="showFilters = !showFilters"
          class="flex items-center space-x-2 px-4 py-2 rounded-lg transition-all duration-300 border-2"
          :class="showFilters
            ? 'bg-custom-green text-white border-custom-green shadow-lg'
            : 'bg-white text-custom-green border-custom-green hover:bg-custom-green hover:text-white'"
        >
          <font-awesome-icon icon="fa-solid fa-tags" class="text-sm" />
          <span class="font-medium">Filtres rapides</span>
          <font-awesome-icon :icon="showFilters ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'" class="text-sm" />
        </button>

        <button
          @click="showAdvancedSearch = !showAdvancedSearch"
          class="flex items-center space-x-2 px-4 py-2 rounded-lg transition-all duration-300 border-2"
          :class="showAdvancedSearch
            ? 'bg-custom-green text-white border-custom-green shadow-lg'
            : 'bg-white text-custom-green border-custom-green hover:bg-custom-green hover:text-white'"
        >
          <font-awesome-icon icon="fa-solid fa-search" class="text-sm" />
          <span class="font-medium">Recherche avancée</span>
          <font-awesome-icon :icon="showAdvancedSearch ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'" class="text-sm" />
        </button>
      </div>

      <!-- Section Filtres rapides -->
      <div v-if="showFilters" class="transform transition-all duration-500 ease-out mb-4">
        <div class="bg-white rounded-xl p-6 shadow-md border border-gray-100">
          <div class="flex items-center mb-4">
            <div class="bg-blue-100 p-2 rounded-lg mr-3">
              <font-awesome-icon icon="fa-solid fa-tags" class="text-blue-600 text-lg" />
            </div>
            <h3 class="font-bold text-lg text-gray-800">Filtres rapides</h3>
          </div>
          <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-3">
            <label
              v-for="filter in quickFilters"
              :key="filter.key"
              class="flex items-center space-x-3 p-3 rounded-lg border border-gray-200 hover:border-custom-green hover:bg-green-50 transition-all cursor-pointer"
            >
              <input
                type="checkbox"
                class="w-4 h-4 text-custom-green bg-gray-100 border-gray-300 rounded focus:ring-3 focus:ring-custom-green"
                :checked="filters[filter.key as keyof typeof filters]"
                @change="emit('update:filters', { ...filters, [filter.key]: ($event.target as HTMLInputElement).checked })"
              />
              <span class="text-sm font-medium text-gray-700">{{ filter.label }}</span>
            </label>
          </div>
        </div>
      </div>

      <!-- Section Recherche avancée -->
      <div v-if="showAdvancedSearch" class="transform transition-all duration-500 ease-out">
        <div class="bg-white rounded-xl p-6 shadow-md border border-gray-100">
          <div class="flex items-center mb-4">
            <div class="bg-purple-100 p-2 rounded-lg mr-3">
              <font-awesome-icon icon="fa-solid fa-search" class="text-purple-600 text-lg" />
            </div>
            <h3 class="font-bold text-lg text-gray-800">Recherche avancée</h3>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="space-y-2">
              <label class="block text-sm font-bold text-gray-700">
                <font-awesome-icon icon="fa-solid fa-flag" class="mr-2 text-custom-green" />Territoire:
              </label>
              <select
                :value="filterCountry"
                @change="emit('update:filterCountry', ($event.target as HTMLSelectElement).value)"
                class="w-full rounded-lg p-3 border border-gray-300 focus:border-custom-green focus:ring-2 focus:ring-custom-green/20 transition-all bg-white"
              >
                <option value="">Choisir un territoire</option>
                <option v-for="pays in PAYS" :key="pays" :value="pays">{{ pays }}</option>
              </select>
            </div>
            <div class="space-y-2">
              <label class="block text-sm font-bold text-gray-700">
                <font-awesome-icon icon="fa-solid fa-users" class="mr-2 text-custom-green" />Groupe ethnique:
              </label>
              <select
                :value="filterEthnie"
                @change="emit('update:filterEthnie', ($event.target as HTMLSelectElement).value)"
                class="w-full rounded-lg p-3 border border-gray-300 focus:border-custom-green focus:ring-2 focus:ring-custom-green/20 transition-all bg-white"
              >
                <option value="">Choisir une ethnie</option>
                <option v-for="ethnie in ETHNIES" :key="ethnie" :value="ethnie">{{ ethnie }}</option>
              </select>
            </div>
            <div class="space-y-2">
              <label class="block text-sm font-bold text-gray-700">
                <font-awesome-icon icon="fa-solid fa-tag" class="mr-2 text-custom-green" />Catégorie:
              </label>
              <select
                :value="filterCategory"
                @change="emit('update:filterCategory', ($event.target as HTMLSelectElement).value)"
                class="w-full rounded-lg p-3 border border-gray-300 focus:border-custom-green focus:ring-2 focus:ring-custom-green/20 transition-all bg-white"
              >
                <option v-for="type in TYPES_FORUM" :key="type.value" :value="type.value">{{ type.label }}</option>
              </select>
            </div>
          </div>
          <div class="mt-6">
            <button
              @click="emit('applyFilters')"
              class="bg-gradient-to-r from-custom-green to-green-600 text-white py-3 px-8 rounded-lg hover:from-green-600 hover:to-custom-green transition-all duration-300 transform hover:scale-105 shadow-lg font-medium"
            >
              <font-awesome-icon icon="fa-solid fa-filter" class="mr-2" />
              Appliquer les filtres
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { TYPES_FORUM, PAYS, ETHNIES } from '~/mocks/forums'

const props = defineProps<{
  userName: string
  userEmail: string
  userPhoto: string | null
  filters: {
    mesPublications: boolean
    bonnesPratiques: boolean
    citations: boolean
    proverbesAdages: boolean
    histoire: boolean
  }
  filterCountry: string
  filterEthnie: string
  filterCategory: string
}>()

const emit = defineEmits<{
  'update:filters': [value: typeof props.filters]
  'update:filterCountry': [value: string]
  'update:filterEthnie': [value: string]
  'update:filterCategory': [value: string]
  applyFilters: []
}>()

const showFilters = ref(true)
const showAdvancedSearch = ref(false)

const quickFilters = [
  { key: 'mesPublications', label: 'Mes publications' },
  { key: 'bonnesPratiques', label: 'Bonnes pratiques' },
  { key: 'citations', label: 'Citations' },
  { key: 'proverbesAdages', label: 'Proverbes/Adages' },
  { key: 'histoire', label: 'Histoire' }
]
</script>

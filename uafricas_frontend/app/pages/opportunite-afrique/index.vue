<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center z-0"
      style="background-image: url('https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80');">
      <div class="absolute inset-0 bg-gradient-to-r from-custom-green/90 to-yellow-600/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-5">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title text-center px-4">
          Opportunites en Afrique
        </h1>
        <div class="h-1 w-24 bg-white rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle text-center px-4">
          Decouvrez les fiches pays et leurs opportunites
        </p>
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="max-w-6xl mx-auto px-4 relative -top-10">
      <!-- Header avec breadcrumb -->
      <div class="bg-white shadow-xs rounded-t-lg">
        <div class="px-4 py-6">
          <CommonBreadcrumbNav class="mb-4" />

          <div class="flex items-center justify-between">
            <div>
              <p class="text-gray-600">
                Explorez les pays africains et leurs richesses
              </p>
            </div>
            <NuxtLink to="/africa-culture"
                      class="text-custom-chocolat hover:text-custom-green font-medium">
              &#8592; AfricaCulture
            </NuxtLink>
          </div>
        </div>
      </div>

      <div class="max-w-7xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
          <!-- Filtres -->
          <div class="lg:col-span-1">
            <div class="bg-white rounded-lg shadow-md p-6 sticky top-4">
              <h3 class="text-lg font-bold mb-4">Filtrer les pays</h3>

              <!-- Recherche -->
              <div class="mb-4">
                <input v-model="searchTerm"
                       @input="onSearchInput"
                       type="text"
                       placeholder="Rechercher un pays..."
                       class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-3 focus:ring-custom-green focus:border-custom-green">
              </div>

              <!-- Regions -->
              <h4 class="text-sm font-medium text-gray-700 mb-2">Region</h4>
              <div class="space-y-2">
                <label class="flex items-center">
                  <input type="radio" v-model="selectedRegion" value="" class="mr-2 text-custom-green focus:ring-3 focus:ring-custom-green">
                  <span>Toutes les regions</span>
                </label>
                <label v-for="region in regions" :key="region" class="flex items-center">
                  <input type="radio" v-model="selectedRegion" :value="region" class="mr-2 text-custom-green focus:ring-3 focus:ring-custom-green">
                  <span class="text-sm">{{ region }}</span>
                </label>
              </div>

              <!-- Stats -->
              <div class="mt-6 pt-6 border-t border-gray-200">
                <div class="text-center">
                  <span class="text-2xl font-bold text-custom-green">{{ filteredPays.length }}</span>
                  <span class="text-gray-600 ml-1">pays</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Liste des pays -->
          <div class="lg:col-span-3">
            <!-- Loading state -->
            <div v-if="loading" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
              <div v-for="i in 6" :key="i" class="bg-white rounded-lg shadow-md overflow-hidden animate-pulse">
                <div class="h-40 bg-gray-200"></div>
                <div class="p-4">
                  <div class="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
                  <div class="h-3 bg-gray-200 rounded w-1/2 mb-4"></div>
                  <div class="h-3 bg-gray-200 rounded w-2/3"></div>
                </div>
              </div>
            </div>

            <!-- Empty state -->
            <div v-else-if="filteredPays.length === 0" class="text-center py-12">
              <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
              <p class="mt-4 text-gray-600">Aucun pays ne correspond a vos criteres</p>
              <button @click="resetFilters" class="mt-4 text-custom-green hover:underline">
                Reinitialiser les filtres
              </button>
            </div>

            <!-- Grid des pays -->
            <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
              <div v-for="pays in filteredPays" :key="pays.id"
                   @click="navigateToDetail(pays.id)"
                   class="bg-white rounded-lg shadow-md hover:shadow-lg transition-all duration-300 overflow-hidden cursor-pointer group">
                <!-- Image de couverture -->
                <div class="relative h-40 overflow-hidden">
                  <img :src="pays.imageCouverture"
                       :alt="pays.nom"
                       class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300">
                  <div class="absolute top-2 right-2">
                    <span class="px-2 py-1 bg-custom-green/90 text-white text-xs font-medium rounded-full">
                      {{ pays.region }}
                    </span>
                  </div>
                  <!-- Drapeau -->
                  <div class="absolute bottom-2 left-2">
                    <img :src="pays.drapeauURL"
                         :alt="'Drapeau ' + pays.nom"
                         class="h-8 w-auto rounded shadow-md">
                  </div>
                </div>

                <!-- Contenu -->
                <div class="p-4">
                  <h3 class="text-lg font-bold text-gray-900 mb-1 group-hover:text-custom-green transition-colors">
                    {{ pays.nom }}
                  </h3>
                  <p v-if="pays.slogan" class="text-sm text-gray-500 italic mb-3 line-clamp-1">
                    {{ pays.slogan }}
                  </p>

                  <!-- Infos principales -->
                  <div class="space-y-1 text-sm text-gray-600">
                    <div class="flex items-center">
                      <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
                      </svg>
                      <span>{{ pays.capitale }}</span>
                    </div>
                    <div class="flex items-center">
                      <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
                      </svg>
                      <span>{{ pays.population }}</span>
                    </div>
                  </div>

                  <!-- Footer de la carte -->
                  <div class="mt-4 pt-3 border-t border-gray-100 flex items-center justify-between text-xs text-gray-500">
                    <span>{{ pays.nombreContributions }} contributions</span>
                    <span>{{ formatDateShort(pays.derniereValidation) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  getAllPays,
  getRegionsUniques,
  searchPays,
  formatDateShort,
  type FichePays,
  type Region
} from '~/mocks/opportunite-afrique'

useHead({
  title: 'Opportunites en Afrique - UAfricas',
  meta: [
    { name: 'description', content: 'Decouvrez les fiches pays africains et leurs opportunites economiques, culturelles et sociales.' }
  ]
})

const loading = ref(true)
const paysList = ref<FichePays[]>([])
const searchTerm = ref('')
const selectedRegion = ref<Region | ''>('')
const regions = ref<Region[]>([])

// Debounce search
let searchTimeout: ReturnType<typeof setTimeout> | null = null
const onSearchInput = () => {
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    applyFilters()
  }, 500)
}

// Computed pour les pays filtres
const filteredPays = computed(() => {
  let result = paysList.value

  // Filtre par recherche
  if (searchTerm.value.trim()) {
    result = searchPays(searchTerm.value)
  }

  // Filtre par region
  if (selectedRegion.value) {
    result = result.filter(p => p.region === selectedRegion.value)
  }

  return result
})

// Appliquer les filtres
const applyFilters = () => {
  // Les filtres sont deja appliques via computed
}

// Watch sur la region selectionnee
watch(selectedRegion, () => {
  applyFilters()
})

// Navigation vers le detail
const navigateToDetail = (id: string) => {
  navigateTo(`/opportunite-afrique/${id}`)
}

// Reinitialiser les filtres
const resetFilters = () => {
  searchTerm.value = ''
  selectedRegion.value = ''
  paysList.value = getAllPays()
}

// Charger les donnees au montage
onMounted(() => {
  loading.value = true
  try {
    regions.value = getRegionsUniques()
    paysList.value = getAllPays()
  } catch (error) {
    console.error('Erreur lors du chargement des pays:', error)
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

@keyframes fadeInUp {
  from { opacity: 0; transform: translateY(30px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes expandWidth {
  from { width: 0; }
  to { width: 6rem; }
}

@keyframes fadeInDelay {
  0%, 40% { opacity: 0; transform: translateY(20px); }
  100% { opacity: 1; transform: translateY(0); }
}

.animate-title { animation: fadeInUp 1s ease-out 0.3s both; }
.animate-line { animation: expandWidth 1s ease-out 1s both; }
.animate-subtitle { animation: fadeInDelay 1.5s ease-out 0.8s both; }
</style>

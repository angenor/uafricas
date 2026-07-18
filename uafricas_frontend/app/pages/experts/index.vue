<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 via-white to-gray-50">
    <!-- Mobile Sidebar Overlay -->
    <div
      v-if="sidebarOpen"
      class="lg:hidden fixed inset-0 bg-black/50 z-40"
      @click="sidebarOpen = false"
    />

    <!-- Mobile Sidebar for Filters -->
    <ExpertsExpertFiltersMobile
      :is-open="sidebarOpen"
      :selected-profile="selectedProfile"
      :total-experts="totalExperts"
      :filtered-count="totalExperts"
      @close="sidebarOpen = false"
      @filter-profile="filterByProfile"
      @reset="resetFilters"
    />

    <!-- Main Content Area -->
    <div class="w-full">
      <!-- Mobile Sidebar Toggle -->
      <div class="lg:hidden flex items-center p-4 bg-white shadow-xs">
        <button
          class="p-2 rounded-lg bg-emerald-500 text-white hover:bg-emerald-600 transition-colors"
          @click="sidebarOpen = !sidebarOpen"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 6h16M4 12h16M4 18h16"
            />
          </svg>
        </button>
        <h1 class="ml-3 text-lg font-semibold text-gray-900">Experts</h1>
      </div>

      <!-- Hero Section -->
      <ExpertsExpertHero
        v-model:search-term="searchTerm"
        v-model:category-selected="categorySelected"
        :total-experts="totalExperts"
        :categories="categories"
        @search="handleSearch"
      />

      <!-- Main Content Section -->
      <div class="max-w-7xl mx-auto px-4 -mt-10 relative z-10">
        <!-- Structure avec sidebar a gauche et contenu principal -->
        <div class="flex gap-8">
          <!-- Left Sidebar - Profile Filters (Desktop uniquement) -->
          <div class="hidden lg:block w-80 flex-shrink-0">
            <ExpertsExpertFilters
              :selected-profile="selectedProfile"
              :total-experts="totalExperts"
              :filtered-count="totalExperts"
              @filter-profile="filterByProfile"
              @reset="resetFilters"
            />
          </div>

          <!-- Main Content Area -->
          <div class="flex-1 min-w-0">
            <!-- Boutons d'action : trouver un expert sur mesure + devenir expert -->
            <div class="flex flex-wrap justify-center items-center gap-4 mb-10">
              <button
                type="button"
                class="inline-flex items-center gap-3 bg-white px-8 py-4 rounded-full shadow-xl hover:shadow-2xl transform hover:-translate-y-1 transition-all group cursor-pointer"
                @click="filtreSurMesureOuvert = true"
              >
                <div
                  class="w-12 h-12 bg-gradient-to-r from-emerald-500 to-teal-500 rounded-full flex items-center justify-center group-hover:scale-110 transition-transform"
                >
                  <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
                    />
                  </svg>
                </div>
                <span class="font-semibold text-gray-900">Trouver un(e) expert(e) sur mesure</span>
                <svg
                  class="w-5 h-5 text-gray-400 group-hover:translate-x-1 transition-transform"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M17 8l4 4m0 0l-4 4m4-4H3"
                  />
                </svg>
              </button>

              <!-- CTA : soumettre sa demande pour devenir expert -->
              <NuxtLink
                to="/devenir-expert"
                class="inline-flex items-center gap-3 bg-linear-to-r from-emerald-500 to-teal-500 text-white px-8 py-4 rounded-full shadow-xl hover:shadow-2xl transform hover:-translate-y-1 transition-all group"
              >
                <div class="w-12 h-12 bg-white/20 rounded-full flex items-center justify-center group-hover:scale-110 transition-transform">
                  <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"
                    />
                  </svg>
                </div>
                <span class="font-semibold">Faire connaître mon expertise</span>
              </NuxtLink>
            </div>

            <!-- Filtres modernises (compacts) -->
            <div class="bg-white rounded-2xl shadow-xl p-3 mb-8">
              <div class="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4">
                <!-- Filtres par tags -->
                <div class="flex-1">
                  <h3 class="text-xs font-medium text-gray-500 uppercase tracking-wider mb-2">
                    Domaines d'expertise
                  </h3>
                  <div class="flex flex-wrap gap-2">
                    <button
                      :class="{
                        'bg-gradient-to-r from-emerald-500 to-teal-500 text-white':
                          categorySelected === 'Tout',
                        'bg-gray-100 text-gray-700 hover:bg-gray-200':
                          categorySelected !== 'Tout',
                      }"
                      class="px-3 py-1.5 rounded-full text-xs font-medium transition-all"
                      @click="filterByCategory('Tout')"
                    >
                      Tout
                    </button>
                    <button
                      v-for="category in categories.filter((cat) => cat !== 'Tout').slice(0, 4)"
                      :key="category"
                      :class="{
                        'bg-gradient-to-r from-emerald-500 to-teal-500 text-white':
                          categorySelected === category,
                        'bg-gray-100 text-gray-700 hover:bg-gray-200':
                          categorySelected !== category,
                      }"
                      class="px-3 py-1.5 rounded-full text-xs font-medium transition-all"
                      @click="filterByCategory(category)"
                    >
                      {{ category }}
                    </button>
                    <div v-if="categories.length > 5" class="relative">
                      <button
                        class="px-3 py-1.5 rounded-full text-xs font-medium bg-gray-100 text-gray-700 hover:bg-gray-200 transition-all flex items-center gap-1"
                        @click="showMoreCategories = !showMoreCategories"
                      >
                        Plus
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M19 9l-7 7-7-7"
                          />
                        </svg>
                      </button>
                      <!-- Dropdown pour plus de categories -->
                      <div
                        v-if="showMoreCategories"
                        class="absolute top-full mt-2 bg-white rounded-xl shadow-xl p-2 z-20 min-w-[200px]"
                      >
                        <button
                          v-for="category in categories.filter((cat) => cat !== 'Tout').slice(4)"
                          :key="category"
                          class="block w-full text-left px-4 py-2 rounded-lg hover:bg-gray-100 text-sm"
                          @click="filterByCategory(category); showMoreCategories = false"
                        >
                          {{ category }}
                        </button>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- Controles de tri et filtre pays -->
                <div class="flex flex-col sm:flex-row gap-3">
                  <!-- Filtre pays -->
                  <select
                    v-model="selectedCountry"
                    class="px-3 py-1.5 bg-gray-50 border border-gray-200 rounded-xl text-sm focus:outline-hidden focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-all"
                  >
                    <option v-for="country in countries" :key="country.value" :value="country.value">
                      {{ country.label }}
                    </option>
                  </select>

                  <!-- Boutons de tri -->
                  <div class="flex gap-2">
                    <button
                      v-for="sortOption in sortOptions"
                      :key="sortOption.id"
                      :class="{
                        'bg-emerald-500 text-white': sortOrder === sortOption.id,
                        'bg-gray-50 text-gray-700 hover:bg-gray-100': sortOrder !== sortOption.id,
                      }"
                      :title="sortOption.label"
                      class="p-2 rounded-xl transition-all group relative"
                      @click="sortExperts(sortOption.id)"
                    >
                      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          stroke-width="2"
                          :d="sortOption.icon"
                        />
                      </svg>
                      <span
                        class="absolute -bottom-8 left-1/2 -translate-x-1/2 bg-gray-900 text-white text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap"
                      >
                        {{ sortOption.label }}
                      </span>
                    </button>
                  </div>
                </div>
              </div>
            </div>

            <!-- Indicateur de chargement -->
            <div v-if="chargement" class="flex justify-center py-12">
              <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-emerald-500" />
            </div>

            <!-- Expert Cards Grid avec design moderne -->
            <div
              v-else-if="experts.length > 0"
              class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 mb-12"
            >
              <ExpertsExpertCard
                v-for="expert in experts"
                :key="expert.id"
                :expert="expert"
                class="transform hover:scale-[1.02] transition-all"
                @contact="contactExpert"
              />
            </div>

            <!-- Empty State moderne -->
            <div v-else class="bg-white rounded-2xl shadow-xl p-12 text-center">
              <div class="w-24 h-24 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-6">
                <svg class="w-12 h-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"
                  />
                </svg>
              </div>
              <h3 class="text-xl font-semibold text-gray-900 mb-2">Aucun(e) expert(e) trouvé(e)</h3>
              <p class="text-gray-500 max-w-md mx-auto">
                Essayez de modifier vos criteres de recherche ou explorez d'autres domaines
                d'expertise.
              </p>
              <button
                class="mt-6 px-6 py-3 bg-gradient-to-r from-emerald-500 to-teal-500 text-white rounded-xl font-medium hover:shadow-lg transition-all"
                @click="resetFilters"
              >
                Reinitialiser les filtres
              </button>
            </div>

            <!-- Pagination moderne -->
            <div v-if="experts.length > 0 && totalPages > 1" class="flex justify-center mb-12">
              <nav class="flex items-center gap-2">
                <button
                  :disabled="currentPage === 1"
                  class="p-2 rounded-xl bg-white border border-gray-200 text-gray-700 hover:bg-gray-50 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                  @click="currentPage--"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M15 19l-7-7 7-7"
                    />
                  </svg>
                </button>

                <div class="flex gap-1">
                  <button
                    v-for="page in visiblePages"
                    :key="page"
                    :class="{
                      'bg-gradient-to-r from-emerald-500 to-teal-500 text-white': page === currentPage,
                      'bg-white text-gray-700 hover:bg-gray-50': page !== currentPage,
                    }"
                    class="w-10 h-10 rounded-xl font-medium transition-all"
                    @click="currentPage = page"
                  >
                    {{ page }}
                  </button>
                </div>

                <button
                  :disabled="currentPage === totalPages"
                  class="p-2 rounded-xl bg-white border border-gray-200 text-gray-700 hover:bg-gray-50 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                  @click="currentPage++"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M9 5l7 7-7 7"
                    />
                  </svg>
                </button>
              </nav>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Modale timeline : trouver un(e) expert(e) sur mesure -->
    <ExpertsExpertFiltreSurMesureModal
      :is-open="filtreSurMesureOuvert"
      @close="filtreSurMesureOuvert = false"
      @apply="appliquerFiltreSurMesure"
    />
  </div>
</template>

<script setup lang="ts">
import {
  useExperts,
  CATEGORIES_EXPERTISE as categories,
  PAYS_EXPERTS as countries,
  type ExpertAPI,
} from '~/composables/useExperts'

useHead({
  title: 'Experts - AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Trouvez les meilleurs experts africains et afro-descendants dans tous les domaines',
    },
  ],
})

// Composable API
const { listerExperts, chargement } = useExperts()

// State
const experts = ref<ExpertAPI[]>([])
const totalExperts = ref(0)
const totalPages = ref(1)
const searchTerm = ref('')
const categorySelected = ref('Tout')
const selectedCountry = ref('')
const selectedProfile = ref('')
const sortOrder = ref<'recent' | 'experience' | 'rating'>('recent')
const showMoreCategories = ref(false)
const sidebarOpen = ref(false)
const currentPage = ref(1)
const parPage = 12
const filtreSurMesureOuvert = ref(false)

// Sort options
const sortOptions = [
  {
    id: 'recent' as const,
    icon: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z',
    label: 'Recent',
  },
  {
    id: 'experience' as const,
    icon: 'M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z',
    label: 'Experience',
  },
  {
    id: 'rating' as const,
    icon: 'M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z',
    label: 'Note',
  },
]

// Charger les experts depuis l'API (pagination server-side)
const chargerExperts = async () => {
  const result = await listerExperts({
    recherche: searchTerm.value || undefined,
    domaine: categorySelected.value !== 'Tout' ? categorySelected.value : undefined,
    pays: selectedCountry.value || undefined,
    situation: selectedProfile.value && selectedProfile.value !== 'tous'
      ? selectedProfile.value
      : undefined,
    tri: sortOrder.value,
    page: currentPage.value,
    par_page: parPage,
  })

  if (result) {
    experts.value = result.experts
    totalExperts.value = result.total
    totalPages.value = result.total_pages
  }
}

// Pagination (pages visibles)
const visiblePages = computed(() => {
  const pages: number[] = []
  const total = totalPages.value
  const current = currentPage.value

  if (total <= 5) {
    for (let i = 1; i <= total; i++) {
      pages.push(i)
    }
  } else {
    if (current <= 3) {
      pages.push(1, 2, 3, 4, 5)
    } else if (current >= total - 2) {
      pages.push(total - 4, total - 3, total - 2, total - 1, total)
    } else {
      pages.push(current - 2, current - 1, current, current + 1, current + 2)
    }
  }

  return pages.filter((p) => p >= 1 && p <= total)
})

// Methods
const filterByCategory = (category: string) => {
  categorySelected.value = category
}

const filterByProfile = (profileId: string) => {
  selectedProfile.value = profileId
}

const sortExperts = (order: 'recent' | 'experience' | 'rating') => {
  sortOrder.value = order
}

const resetFilters = () => {
  categorySelected.value = 'Tout'
  selectedCountry.value = ''
  selectedProfile.value = ''
  searchTerm.value = ''
  currentPage.value = 1
}

const handleSearch = () => {
  currentPage.value = 1
  chargerExperts()
}

// Appliquer les critères choisis dans la modale timeline
const appliquerFiltreSurMesure = (filtres: {
  domaine: string
  pays: string
  situation: string
  recherche: string
}) => {
  categorySelected.value = filtres.domaine || 'Tout'
  selectedCountry.value = filtres.pays
  selectedProfile.value = filtres.situation
  searchTerm.value = filtres.recherche
  currentPage.value = 1
  filtreSurMesureOuvert.value = false
  chargerExperts()
}

const contactExpert = (expert: ExpertAPI) => {
  if (expert.email) {
    window.location.href = `mailto:${expert.email}`
  }
}

// Recharger quand les filtres changent (reset page + appel API)
watch([categorySelected, selectedCountry, selectedProfile, sortOrder], () => {
  currentPage.value = 1
  chargerExperts()
})

// Recharger quand la page change
watch(currentPage, () => {
  chargerExperts()
})

// Chargement initial
onMounted(() => {
  chargerExperts()
})
</script>

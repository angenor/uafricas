<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Mobile Sidebar (Filtres) -->
    <ProjetsProjetFiltersMobile
      v-model="filtres"
      :is-open="sidebarOpen"
      :total-projets="statistics.total"
      :filtered-count="total"
      @close="sidebarOpen = false"
      @reset="resetFilters"
    />

    <!-- Mobile Toggle Button -->
    <div class="lg:hidden fixed bottom-6 right-6 z-30">
      <button
        class="p-4 rounded-full bg-custom-green text-white shadow-lg hover:shadow-xl transition-all"
        @click="sidebarOpen = true"
      >
        <font-awesome-icon :icon="['fas', 'filter']" class="w-5 h-5" />
      </button>
    </div>

    <!-- Hero Section -->
    <ProjetsProjetHero :statistics="statistics" />

    <!-- Breadcrumb -->
    <div class="bg-gray-50">
      <div class="max-w-7xl mx-auto px-4 py-4">
        <CommonBreadcrumbNav />
      </div>
    </div>

    <!-- Main Content -->
    <div class="max-w-7xl mx-auto px-4 py-8">
      <!-- Search Bar -->
      <div class="max-w-2xl mx-auto mb-8">
        <ProjetsProjetSearchBar v-model="filtres.recherche" @search="handleSearch" />
      </div>

      <!-- Content with Sidebar -->
      <div class="flex gap-8">
        <!-- Desktop Sidebar -->
        <div class="hidden lg:block w-80 flex-shrink-0">
          <ProjetsProjetFilters
            v-model="filtres"
            :total-projets="statistics.total"
            :filtered-count="total"
            @reset="resetFilters"
          />
        </div>

        <!-- Main Content Area -->
        <div class="flex-1 min-w-0">
          <!-- Results count -->
          <div class="flex items-center justify-between mb-6">
            <p class="text-gray-600">
              <span class="font-semibold text-gray-900">{{ total }}</span> projet(s) trouvé(s)
            </p>
          </div>

          <!-- Loading State -->
          <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div
              v-for="n in 6"
              :key="n"
              class="bg-white rounded-2xl shadow-lg overflow-hidden animate-pulse"
            >
              <div class="h-48 bg-gray-200" />
              <div class="p-5 space-y-4">
                <div class="h-4 bg-gray-200 rounded w-3/4" />
                <div class="h-3 bg-gray-200 rounded w-1/2" />
                <div class="h-3 bg-gray-200 rounded" />
                <div class="h-3 bg-gray-200 rounded w-2/3" />
              </div>
            </div>
          </div>

          <!-- Projects Grid -->
          <div
            v-else-if="projets.length > 0"
            class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8"
          >
            <ProjetsProjetCard
              v-for="projet in projets"
              :key="projet.id"
              :projet="projet"
              class="transform hover:scale-[1.02] transition-all"
              data-aos="fade-up"
            />
          </div>

          <!-- Empty State -->
          <div v-else class="bg-white rounded-2xl shadow-xl p-12 text-center">
            <div class="w-24 h-24 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-6">
              <font-awesome-icon :icon="['fas', 'folder-open']" class="w-12 h-12 text-gray-400" />
            </div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">Aucun projet trouvé</h3>
            <p class="text-gray-500 max-w-md mx-auto mb-6">
              Essayez de modifier vos critères de recherche ou explorez d'autres filtres.
            </p>
            <button
              class="px-6 py-3 bg-gradient-to-r from-custom-green to-emerald-600 text-white rounded-xl font-medium hover:shadow-lg transition-all"
              @click="resetFilters"
            >
              Réinitialiser les filtres
            </button>
          </div>

          <!-- Pagination -->
          <div v-if="totalPages > 1" class="flex justify-center mt-8">
            <nav class="flex items-center gap-2">
              <button
                :disabled="currentPage === 1"
                class="p-2 rounded-xl bg-white border border-gray-200 text-gray-700 hover:bg-gray-50 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                @click="currentPage--"
              >
                <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
              </button>

              <div class="flex gap-1">
                <button
                  v-for="page in visiblePages"
                  :key="page"
                  :class="{
                    'bg-gradient-to-r from-custom-green to-emerald-600 text-white': page === currentPage,
                    'bg-white text-gray-700 hover:bg-gray-50 border border-gray-200': page !== currentPage,
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
                <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-4 h-4" />
              </button>
            </nav>
          </div>
        </div>
      </div>

      <!-- CTA Section -->
      <div class="mt-16 bg-gradient-to-r from-custom-chocolat to-orange-600 rounded-3xl p-8 md:p-12 text-white text-center" data-aos="fade-up">
        <h2 class="text-2xl md:text-3xl font-bold mb-4">Vous avez un projet ?</h2>
        <p class="text-white/90 max-w-2xl mx-auto mb-8">
          Soumettez votre projet et bénéficiez du soutien de notre communauté d'investisseurs et de partenaires africains.
        </p>
        <NuxtLink
          to="/soumettre-projet"
          class="inline-flex items-center gap-2 px-8 py-4 bg-white text-custom-chocolat rounded-xl font-semibold hover:shadow-lg transform hover:scale-105 transition-all"
        >
          <font-awesome-icon :icon="['fas', 'paper-plane']" class="w-5 h-5" />
          Soumettre mon projet
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import AOS from 'aos'
import {
  useProjets,
  convertirFiltresPageVersAPI,
  type ProjetAPI,
  type ProjetStatistiquesAPI,
  type FiltresProjetPage,
} from '~/composables/useProjets'

useHead({
  title: 'Financer un Projet - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez des projets de développement innovants en Afrique et contribuez à leur financement.',
    },
  ],
})

const { listerProjets, obtenirStatistiques, chargement } = useProjets()

// State
const projets = ref<ProjetAPI[]>([])
const loading = computed(() => chargement.value)
const total = ref(0)
const totalPages = ref(1)
const sidebarOpen = ref(false)
const currentPage = ref(1)
const itemsPerPage = 12

const filtres = ref<FiltresProjetPage>({
  pays: '',
  budgetMax: '',
  duree: '',
  recherche: '',
  sortBy: 'dateCreation',
})

// Statistiques
const statistics = ref<ProjetStatistiquesAPI>({
  total: 0,
  valides: 0,
  en_cours: 0,
  termines: 0,
})

// Visible pages pour la pagination
const visiblePages = computed(() => {
  const pages: number[] = []
  const tp = totalPages.value
  const current = currentPage.value

  if (tp <= 5) {
    for (let i = 1; i <= tp; i++) {
      pages.push(i)
    }
  } else {
    if (current <= 3) {
      pages.push(1, 2, 3, 4, 5)
    } else if (current >= tp - 2) {
      pages.push(tp - 4, tp - 3, tp - 2, tp - 1, tp)
    } else {
      pages.push(current - 2, current - 1, current, current + 1, current + 2)
    }
  }

  return pages.filter((p) => p >= 1 && p <= tp)
})

// Charger les projets depuis l'API
const chargerProjets = async () => {
  const apiParams = convertirFiltresPageVersAPI(filtres.value)
  apiParams.page = currentPage.value
  apiParams.par_page = itemsPerPage

  const result = await listerProjets(apiParams)
  if (result) {
    projets.value = result.projets
    total.value = result.total
    totalPages.value = result.total_pages
  }
}

// Methods
const resetFilters = () => {
  filtres.value = {
    pays: '',
    budgetMax: '',
    duree: '',
    recherche: '',
    sortBy: 'dateCreation',
  }
  currentPage.value = 1
}

const handleSearch = () => {
  currentPage.value = 1
  chargerProjets()
}

// Watch filters to reload
watch(filtres, () => {
  currentPage.value = 1
  chargerProjets()
}, { deep: true })

// Watch page changes
watch(currentPage, () => {
  chargerProjets()
})

// Lifecycle
onMounted(async () => {
  // Load data in parallel
  const [_, stats] = await Promise.all([
    chargerProjets(),
    obtenirStatistiques(),
  ])
  if (stats) {
    statistics.value = stats
  }

  // Initialize AOS
  AOS.init({
    duration: 800,
    easing: 'ease-out-cubic',
    once: true,
    offset: 50,
  })
})
</script>

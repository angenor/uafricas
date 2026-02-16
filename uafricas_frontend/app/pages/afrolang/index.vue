<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Mobile Sidebar (Filtres) -->
    <AfrolangSalleFiltersMobile
      v-model="filtres"
      :is-open="sidebarOpen"
      :total-salles="totalSalles"
      :filtered-count="total"
      :langues="languesDisponibles"
      @close="sidebarOpen = false"
      @reset="resetFilters"
    />

    <!-- Mobile Toggle Button -->
    <div class="lg:hidden fixed bottom-6 right-6 z-30">
      <button
        class="p-4 rounded-full bg-blue-500 text-white shadow-lg hover:shadow-xl transition-all"
        @click="sidebarOpen = true"
      >
        <font-awesome-icon :icon="['fas', 'filter']" class="w-5 h-5" />
      </button>
    </div>

    <!-- Hero Section -->
    <AfrolangAfrolangHero :stats="stats" />

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
        <div class="relative">
          <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
          <input
            v-model="filtres.recherche"
            type="text"
            class="w-full pl-12 pr-4 py-4 bg-white border border-gray-200 rounded-2xl shadow-sm focus:outline-hidden focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all text-gray-700 placeholder-gray-400"
            placeholder="Rechercher une salle par nom ou langue..."
            @keyup.enter="handleSearch"
          />
        </div>
      </div>

      <!-- Content with Sidebar -->
      <div class="flex gap-8">
        <!-- Desktop Sidebar -->
        <div class="hidden lg:block w-80 flex-shrink-0">
          <AfrolangSalleFilters
            v-model="filtres"
            :total-salles="totalSalles"
            :filtered-count="total"
            :langues="languesDisponibles"
            @reset="resetFilters"
          />
        </div>

        <!-- Main Content Area -->
        <div class="flex-1 min-w-0">
          <!-- Results count -->
          <div class="flex items-center justify-between mb-6">
            <p class="text-gray-600">
              <span class="font-semibold text-gray-900">{{ total }}</span> salle(s) trouvée(s)
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

          <!-- Salles Grid -->
          <div
            v-else-if="salles.length > 0"
            class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8"
          >
            <AfrolangSalleCard
              v-for="salle in salles"
              :key="salle.id"
              :salle="salle"
              class="transform hover:scale-[1.02] transition-all"
              data-aos="fade-up"
            />
          </div>

          <!-- Empty State -->
          <div v-else class="bg-white rounded-2xl shadow-xl p-12 text-center">
            <div class="w-24 h-24 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-6">
              <font-awesome-icon :icon="['fas', 'door-open']" class="w-12 h-12 text-gray-400" />
            </div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">Aucune salle trouvée</h3>
            <p class="text-gray-500 max-w-md mx-auto mb-6">
              Essayez de modifier vos critères de recherche ou explorez d'autres langues.
            </p>
            <button
              class="px-6 py-3 bg-blue-500 text-white font-medium rounded-xl hover:bg-blue-600 transition-colors"
              @click="resetFilters"
            >
              Réinitialiser les filtres
            </button>
          </div>

          <!-- Pagination -->
          <div v-if="totalPages > 1" class="flex justify-center items-center gap-2 mt-8">
            <button
              @click="goToPage(currentPage - 1)"
              :disabled="currentPage === 1"
              class="p-2 rounded-lg border border-gray-200 text-gray-600 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
            </button>

            <template v-for="page in visiblePages" :key="page">
              <span v-if="page === '...'" class="px-3 py-2 text-gray-400">...</span>
              <button
                v-else
                @click="goToPage(page as number)"
                class="px-4 py-2 rounded-lg font-medium transition-colors"
                :class="currentPage === page
                  ? 'bg-blue-500 text-white shadow-lg'
                  : 'border border-gray-200 text-gray-600 hover:bg-gray-50'"
              >
                {{ page }}
              </button>
            </template>

            <button
              @click="goToPage(currentPage + 1)"
              :disabled="currentPage === totalPages"
              class="p-2 rounded-lg border border-gray-200 text-gray-600 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  useAfrolang,
  type SalleAPI,
  type SalleFiltres,
  type AfrolangStats,
} from '~/composables/useAfrolang'

useHead({
  title: 'Salles Afrolang - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les salles Afrolang : visioconférence dédiée à l\'apprentissage des langues africaines. Wolof, Swahili, Lingala et bien plus.',
    },
  ],
})

const ITEMS_PER_PAGE = 12

const { chargement, listerSalles, obtenirStats, listerLangues } = useAfrolang()

// State
const salles = ref<SalleAPI[]>([])
const total = ref(0)
const totalSalles = ref(0)
const totalPages = ref(1)
const currentPage = ref(1)
const sidebarOpen = ref(false)
const languesDisponibles = ref<string[]>([])
const loading = computed(() => chargement.value)

const stats = ref<AfrolangStats>({
  total_salles: 0,
  total_salles_privees: 0,
  sessions_en_cours: 0,
  sessions_terminees: 0,
  total_participants_uniques: 0,
})

const filtres = ref<SalleFiltres>({
  recherche: '',
  langue: '',
})

// Debounce timer pour la recherche
let rechercheTimer: ReturnType<typeof setTimeout> | null = null

// Construire les filtres API
const buildApiFiltres = (): SalleFiltres => {
  const f: SalleFiltres = {
    page: currentPage.value,
    par_page: ITEMS_PER_PAGE,
  }
  if (filtres.value.recherche?.trim()) f.recherche = filtres.value.recherche.trim()
  if (filtres.value.langue) f.langue = filtres.value.langue
  return f
}

// Charger les salles
const chargerSalles = async () => {
  const resultat = await listerSalles(buildApiFiltres())
  if (resultat) {
    salles.value = resultat.salles
    total.value = resultat.total
    totalPages.value = resultat.total_pages
  }
}

// Pagination visible
const visiblePages = computed(() => {
  const pages: (number | string)[] = []
  const tp = totalPages.value
  const current = currentPage.value

  if (tp <= 7) {
    for (let i = 1; i <= tp; i++) pages.push(i)
  }
  else {
    pages.push(1)
    if (current > 3) pages.push('...')
    const start = Math.max(2, current - 1)
    const end = Math.min(tp - 1, current + 1)
    for (let i = start; i <= end; i++) pages.push(i)
    if (current < tp - 2) pages.push('...')
    pages.push(tp)
  }

  return pages
})

// Watchers
watch(
  () => filtres.value.langue,
  () => {
    currentPage.value = 1
    chargerSalles()
  },
)

// Debounce recherche
watch(
  () => filtres.value.recherche,
  () => {
    if (rechercheTimer) clearTimeout(rechercheTimer)
    rechercheTimer = setTimeout(() => {
      currentPage.value = 1
      chargerSalles()
    }, 300)
  },
)

// Methods
const handleSearch = () => {
  if (rechercheTimer) clearTimeout(rechercheTimer)
  currentPage.value = 1
  chargerSalles()
}

const resetFilters = () => {
  filtres.value = { recherche: '', langue: '' }
  currentPage.value = 1
}

const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    chargerSalles()
    window.scrollTo({ top: 400, behavior: 'smooth' })
  }
}

// Lifecycle
onMounted(async () => {
  const [statsResult, languesResult] = await Promise.all([
    obtenirStats(),
    listerLangues(),
  ])

  if (statsResult) {
    stats.value = statsResult
    totalSalles.value = statsResult.total_salles
  }
  languesDisponibles.value = languesResult

  await chargerSalles()
})
</script>

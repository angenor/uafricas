<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100">
    <!-- Hero avec recherche -->
    <MarcheMarcheHero
      v-model:model-categorie="filtres.categorie"
      v-model:model-recherche="filtres.recherche"
      @search="handleSearch"
      @publish="handlePublish"
    />

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <!-- Breadcrumb -->
      <CommonBreadcrumbNav class="mb-8" />

      <!-- Boutons catégories -->
      <MarcheMarcheCategoryButtons
        :active-filter="filtres.categorie"
        @select="selectCategory"
      />

      <!-- Layout: Filtres + Grille -->
      <div class="flex flex-col lg:flex-row gap-8">
        <!-- Filtres latéraux (desktop) -->
        <aside class="hidden lg:block w-72 flex-shrink-0">
          <MarcheMarcheFilters
            v-model="filtres"
            :annonces="allAnnonces"
            @reset="resetFilters"
          />
        </aside>

        <!-- Bouton filtres mobile -->
        <button
          @click="showMobileFilters = true"
          class="lg:hidden flex items-center justify-center gap-2 px-4 py-3 bg-white rounded-xl shadow-md text-gray-700 font-medium mb-4"
        >
          <font-awesome-icon :icon="['fas', 'filter']" class="w-4 h-4" />
          Filtres
          <span
            v-if="activeFiltersCount > 0"
            class="bg-emerald-500 text-white text-xs px-2 py-0.5 rounded-full"
          >
            {{ activeFiltersCount }}
          </span>
        </button>

        <!-- Section annonces -->
        <div class="flex-1 min-w-0">
          <!-- Barre de résultats -->
          <div class="flex items-center justify-between mb-6">
            <p class="text-gray-600">
              <span class="font-semibold text-gray-900">{{ annoncesFiltrees.length }}</span>
              {{ annoncesFiltrees.length > 1 ? 'annonces trouvées' : 'annonce trouvée' }}
            </p>

            <!-- Tri mobile -->
            <select
              v-model="filtres.tri"
              class="lg:hidden px-3 py-2 text-sm border border-gray-200 rounded-lg bg-white"
            >
              <option value="recent">Plus récent</option>
              <option value="price-asc">Prix croissant</option>
              <option value="price-desc">Prix décroissant</option>
            </select>
          </div>

          <!-- État vide -->
          <div
            v-if="annoncesFiltrees.length === 0"
            class="text-center py-16 bg-white rounded-2xl shadow-sm"
          >
            <font-awesome-icon
              :icon="['fas', 'box-open']"
              class="w-16 h-16 text-gray-300 mx-auto mb-4"
            />
            <h3 class="text-lg font-semibold text-gray-700 mb-2">Aucune annonce trouvée</h3>
            <p class="text-gray-500 mb-4">Essayez de modifier vos critères de recherche</p>
            <button
              @click="resetFilters"
              class="px-4 py-2 bg-emerald-500 text-white rounded-lg hover:bg-emerald-600 transition-colors"
            >
              Réinitialiser les filtres
            </button>
          </div>

          <!-- Grille d'annonces -->
          <div
            v-else
            class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6"
            data-aos="fade-up"
          >
            <MarcheAnnonceCard
              v-for="annonce in annoncesPaginees"
              :key="annonce.id"
              :annonce="annonce"
              data-aos="fade-up"
              data-aos-delay="100"
            />
          </div>

          <!-- Pagination -->
          <div
            v-if="totalPages > 1"
            class="mt-12 flex items-center justify-center gap-2"
          >
            <!-- Précédent -->
            <button
              @click="goToPage(currentPage - 1)"
              :disabled="currentPage === 1"
              class="p-2 rounded-lg border border-gray-200 text-gray-600 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
            </button>

            <!-- Pages -->
            <template v-for="page in visiblePages" :key="page">
              <span
                v-if="page === '...'"
                class="px-3 py-2 text-gray-400"
              >
                ...
              </span>
              <button
                v-else
                @click="goToPage(page as number)"
                class="px-4 py-2 rounded-lg font-medium transition-colors"
                :class="currentPage === page
                  ? 'bg-emerald-500 text-white shadow-lg'
                  : 'border border-gray-200 text-gray-600 hover:bg-gray-50'"
              >
                {{ page }}
              </button>
            </template>

            <!-- Suivant -->
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

    <!-- Modal filtres mobile -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition ease-out duration-300"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition ease-in duration-200"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div
          v-if="showMobileFilters"
          class="fixed inset-0 z-50 lg:hidden"
        >
          <!-- Overlay -->
          <div
            class="absolute inset-0 bg-black/50"
            @click="showMobileFilters = false"
          ></div>

          <!-- Panel -->
          <div class="absolute inset-y-0 right-0 w-full max-w-sm bg-white shadow-xl">
            <div class="flex items-center justify-between p-4 border-b">
              <h2 class="text-lg font-semibold">Filtres</h2>
              <button
                @click="showMobileFilters = false"
                class="p-2 text-gray-400 hover:text-gray-600"
              >
                <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
              </button>
            </div>

            <div class="p-4 overflow-y-auto h-[calc(100%-130px)]">
              <MarcheMarcheFilters
                v-model="filtres"
                :annonces="allAnnonces"
                @reset="resetFilters"
              />
            </div>

            <div class="absolute bottom-0 left-0 right-0 p-4 bg-white border-t">
              <button
                @click="showMobileFilters = false"
                class="w-full py-3 bg-emerald-500 text-white font-semibold rounded-xl hover:bg-emerald-600 transition-colors"
              >
                Voir {{ annoncesFiltrees.length }} résultats
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Modal publication (placeholder) -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition ease-out duration-300"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition ease-in duration-200"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div
          v-if="showPublishModal"
          class="fixed inset-0 z-50 flex items-center justify-center p-4"
        >
          <div
            class="absolute inset-0 bg-black/50"
            @click="showPublishModal = false"
          ></div>

          <div class="relative bg-white rounded-2xl shadow-xl max-w-md w-full p-6 text-center">
            <font-awesome-icon
              :icon="['fas', 'tools']"
              class="w-16 h-16 text-amber-500 mx-auto mb-4"
            />
            <h2 class="text-xl font-bold text-gray-800 mb-2">
              Fonctionnalité bientôt disponible
            </h2>
            <p class="text-gray-600 mb-6">
              La publication d'annonces sera disponible dans une prochaine mise à jour.
              Restez connecté !
            </p>
            <button
              @click="showPublishModal = false"
              class="px-6 py-2.5 bg-emerald-500 text-white font-medium rounded-xl hover:bg-emerald-600 transition-colors"
            >
              Compris
            </button>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import AOS from 'aos'
import {
  annoncesMock,
  rechercherAnnonces,
  type Annonce,
  type FiltresAnnonce,
  type Categorie
} from '~/mocks/marche-africain'

useHead({
  title: 'Marché Africain - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez le Marché Africain UAfricas : annonces de vente, troc et dons à travers toute l\'Afrique. Agriculture, informatique, immobilier et plus.'
    }
  ]
})

const ITEMS_PER_PAGE = 12

// State
const allAnnonces = ref<Annonce[]>([])
const currentPage = ref(1)
const showMobileFilters = ref(false)
const showPublishModal = ref(false)

const filtres = ref<FiltresAnnonce>({
  categorie: 'Tout',
  typesEchange: [],
  prixMin: null,
  prixMax: null,
  recherche: '',
  tri: 'recent'
})

// Computed
const annoncesFiltrees = computed(() => {
  return rechercherAnnonces(filtres.value)
})

const totalPages = computed(() => {
  return Math.ceil(annoncesFiltrees.value.length / ITEMS_PER_PAGE)
})

const annoncesPaginees = computed(() => {
  const start = (currentPage.value - 1) * ITEMS_PER_PAGE
  return annoncesFiltrees.value.slice(start, start + ITEMS_PER_PAGE)
})

const visiblePages = computed(() => {
  const pages: (number | string)[] = []
  const total = totalPages.value
  const current = currentPage.value

  if (total <= 7) {
    for (let i = 1; i <= total; i++) pages.push(i)
  } else {
    pages.push(1)

    if (current > 3) pages.push('...')

    const start = Math.max(2, current - 1)
    const end = Math.min(total - 1, current + 1)

    for (let i = start; i <= end; i++) pages.push(i)

    if (current < total - 2) pages.push('...')

    pages.push(total)
  }

  return pages
})

const activeFiltersCount = computed(() => {
  let count = 0
  if (filtres.value.categorie !== 'Tout') count++
  if (filtres.value.typesEchange.length > 0) count++
  if (filtres.value.prixMin !== null) count++
  if (filtres.value.prixMax !== null) count++
  if (filtres.value.recherche.trim() !== '') count++
  return count
})

// Watchers
watch(filtres, () => {
  currentPage.value = 1
}, { deep: true })

// Methods
const selectCategory = (category: Categorie | 'Tout') => {
  filtres.value.categorie = category
}

const handleSearch = () => {
  currentPage.value = 1
}

const handlePublish = () => {
  showPublishModal.value = true
}

const resetFilters = () => {
  filtres.value = {
    categorie: 'Tout',
    typesEchange: [],
    prixMin: null,
    prixMax: null,
    recherche: '',
    tri: 'recent'
  }
  currentPage.value = 1
}

const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    window.scrollTo({ top: 400, behavior: 'smooth' })
  }
}

// Lifecycle
onMounted(() => {
  allAnnonces.value = annoncesMock
  AOS.init({
    duration: 800,
    once: true
  })
})
</script>

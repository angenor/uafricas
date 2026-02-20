<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1555217851-6141535bd771?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-5">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Marché Africain
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Annonces et échanges
        </p>
        <p class="text-white/80 text-sm md:text-base mt-3 max-w-3xl text-center px-4 animate-subtitle">
          Offrir un marché virtuel aux africains, aux afro-descendants et à la diaspora africaine.
        </p>
      </div>
    </div>

    <!-- Barre de recherche -->
    <div class="max-w-4xl mx-auto -mt-8 relative z-10 px-4">
      <div class="bg-white rounded-xl shadow-xl p-5 transform transition-all hover:shadow-2xl">
        <div class="flex flex-col md:flex-row gap-3">
          <div class="flex-1">
            <input
              v-model="filtres.recherche"
              type="text"
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all"
              placeholder="Rechercher une annonce..."
              @keyup.enter="handleSearch"
            />
          </div>
          <button
            @click="handleSearch"
            class="bg-linear-to-r from-custom-green to-green-600 hover:from-green-600 hover:to-custom-green text-white px-6 py-3 rounded-lg transition-all duration-300 transform hover:scale-105 focus:outline-hidden focus:ring-2 focus:ring-custom-green flex items-center justify-center"
          >
            <font-awesome-icon icon="fa-solid fa-search" class="mr-2" />
            Recherche
          </button>
          <button
            @click="handlePublish"
            class="bg-linear-to-r from-custom-chocolat to-amber-700 hover:from-amber-700 hover:to-custom-chocolat text-white px-6 py-3 rounded-lg transition-all duration-300 transform hover:scale-105 focus:outline-hidden focus:ring-2 focus:ring-custom-chocolat flex items-center justify-center"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="mr-2" />
            Publier
          </button>
        </div>

        <!-- Filtres catégories -->
        <div class="flex flex-wrap mt-3 gap-2">
          <label
            v-for="cat in categoriesFiltre"
            :key="cat.key"
          >
            <input
              type="radio"
              name="categorie-filter"
              v-model="filtres.categorie"
              :value="cat.key"
              class="hidden"
            />
            <div
              class="px-4 py-2 rounded-full text-sm cursor-pointer transition-all duration-200"
              :class="[
                filtres.categorie === cat.key
                  ? 'bg-custom-chocolat text-white'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
              ]"
            >
              {{ cat.label }}
            </div>
          </label>
        </div>
      </div>
    </div>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <!-- Breadcrumb -->
      <CommonBreadcrumbNav class="mb-8" />

      <!-- Boutons catégories -->
      <MarcheCategoryButtons
        :active-filter="filtres.categorie"
        @select="selectCategory"
      />

      <!-- Layout: Filtres + Grille -->
      <div class="flex flex-col lg:flex-row gap-8">
        <!-- Filtres latéraux (desktop) -->
        <aside class="hidden lg:block w-72 flex-shrink-0">
          <MarcheFilters
            v-model="filtres"
            :annonces="annonces"
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
              <span class="font-semibold text-gray-900">{{ totalAnnonces }}</span>
              {{ totalAnnonces > 1 ? 'annonces trouvées' : 'annonce trouvée' }}
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

          <!-- Chargement -->
          <div
            v-if="chargement"
            class="text-center py-16"
          >
            <div class="animate-spin rounded-full h-12 w-12 border-4 border-emerald-500 border-t-transparent mx-auto mb-4"></div>
            <p class="text-gray-500">Chargement des annonces...</p>
          </div>

          <!-- Erreur -->
          <div
            v-else-if="erreur"
            class="text-center py-16 bg-white rounded-2xl shadow-xs"
          >
            <font-awesome-icon
              :icon="['fas', 'circle-exclamation']"
              class="w-16 h-16 text-red-300 mx-auto mb-4"
            />
            <h3 class="text-lg font-semibold text-gray-700 mb-2">Erreur de chargement</h3>
            <p class="text-gray-500 mb-4">{{ erreur }}</p>
            <button
              @click="chargerAnnonces"
              class="px-4 py-2 bg-emerald-500 text-white rounded-lg hover:bg-emerald-600 transition-colors"
            >
              Réessayer
            </button>
          </div>

          <!-- État vide -->
          <div
            v-else-if="annonces.length === 0"
            class="text-center py-16 bg-white rounded-2xl shadow-xs"
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
          >
            <MarcheAnnonceCard
              v-for="annonce in annonces"
              :key="annonce.id"
              :annonce="annonce"
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
              <MarcheFilters
                v-model="filtres"
                :annonces="annonces"
                @reset="resetFilters"
              />
            </div>

            <div class="absolute bottom-0 left-0 right-0 p-4 bg-white border-t">
              <button
                @click="showMobileFilters = false"
                class="w-full py-3 bg-emerald-500 text-white font-semibold rounded-xl hover:bg-emerald-600 transition-colors"
              >
                Voir {{ totalAnnonces }} résultats
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
import {
  useMarcheAfricain,
  CATEGORIES,
  mapperTypesVersDb,
  type AnnonceAPI,
  type AnnonceFiltres,
  type FiltresAnnonce,
  type Categorie,
} from '~/composables/useMarcheAfricain'

useHead({
  title: 'Marché Africain - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez le Marché Africain UAfricas : annonces de vente, troc et dons à travers toute l\'Afrique. Agriculture, informatique, immobilier et plus.',
    },
  ],
})

const ITEMS_PER_PAGE = 12

const { chargement, erreur, listerAnnonces } = useMarcheAfricain()

// State
const annonces = ref<AnnonceAPI[]>([])
const totalAnnonces = ref(0)
const totalPages = ref(1)
const currentPage = ref(1)
const showMobileFilters = ref(false)
const showPublishModal = ref(false)

const filtres = ref<FiltresAnnonce>({
  categorie: 'Tout',
  typesEchange: [],
  prixMin: null,
  prixMax: null,
  recherche: '',
  tri: 'recent',
})

// Catégories pour les filtres chips
const categoriesFiltre = CATEGORIES

// Debounce timer pour la recherche
let rechercheTimer: ReturnType<typeof setTimeout> | null = null

// Construire les filtres API a partir des filtres UI
const buildApiFiltres = (): AnnonceFiltres => {
  const f: AnnonceFiltres = {
    page: currentPage.value,
    par_page: ITEMS_PER_PAGE,
    tri: filtres.value.tri,
  }
  if (filtres.value.recherche.trim()) f.recherche = filtres.value.recherche.trim()
  if (filtres.value.categorie !== 'Tout') f.categorie = filtres.value.categorie
  if (filtres.value.typesEchange.length > 0) {
    f.type_operation = mapperTypesVersDb(filtres.value.typesEchange)
  }
  if (filtres.value.prixMin != null) f.prix_min = filtres.value.prixMin
  if (filtres.value.prixMax != null) f.prix_max = filtres.value.prixMax
  return f
}

// Charger les annonces depuis l'API
const chargerAnnonces = async () => {
  const resultat = await listerAnnonces(buildApiFiltres())
  if (resultat) {
    annonces.value = resultat.annonces
    totalAnnonces.value = resultat.total
    totalPages.value = resultat.total_pages
  }
}

// Computed
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
// Quand les filtres changent (sauf recherche qui est debounced), recharger
watch(
  () => ({
    categorie: filtres.value.categorie,
    typesEchange: [...filtres.value.typesEchange],
    prixMin: filtres.value.prixMin,
    prixMax: filtres.value.prixMax,
    tri: filtres.value.tri,
  }),
  () => {
    currentPage.value = 1
    chargerAnnonces()
  },
  { deep: true },
)

// Debounce la recherche textuelle (300ms)
watch(
  () => filtres.value.recherche,
  () => {
    if (rechercheTimer) clearTimeout(rechercheTimer)
    rechercheTimer = setTimeout(() => {
      currentPage.value = 1
      chargerAnnonces()
    }, 300)
  },
)

// Methods
const selectCategory = (category: Categorie | 'Tout') => {
  filtres.value.categorie = category
}

const handleSearch = () => {
  if (rechercheTimer) clearTimeout(rechercheTimer)
  currentPage.value = 1
  chargerAnnonces()
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
    tri: 'recent',
  }
  currentPage.value = 1
}

const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    chargerAnnonces()
    window.scrollTo({ top: 400, behavior: 'smooth' })
  }
}

// Lifecycle
onMounted(async () => {
  await chargerAnnonces()
})
</script>

<style scoped>
@reference "~/assets/css/main.css";

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-20px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes expandLine {
  from { width: 0; }
  to { width: 6rem; }
}

.animate-title {
  animation: fadeIn 1s ease-out forwards;
}

.animate-subtitle {
  animation: fadeIn 1s ease-out 0.3s forwards;
  opacity: 0;
}

.animate-line {
  animation: expandLine 1.2s ease-out 0.1s forwards;
  width: 0;
}
</style>

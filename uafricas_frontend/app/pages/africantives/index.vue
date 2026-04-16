<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1489392191049-fc10c97e64b6?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-14">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Africantives
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Organiser un évenement mettant en valeur l’Afrique et son développement
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
              placeholder="Rechercher une initiative..."
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

        <!-- Filtres domaines -->
        <div class="flex flex-wrap mt-3 gap-2">
          <label
            v-for="dom in domainesFiltre"
            :key="dom.value"
          >
            <input
              type="radio"
              name="domaine-filter"
              v-model="filtres.domaine"
              :value="dom.value"
              class="hidden"
            />
            <div
              class="px-4 py-2 rounded-full text-sm cursor-pointer transition-all duration-200"
              :class="[
                filtres.domaine === dom.value
                  ? 'bg-custom-chocolat text-white'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
              ]"
            >
              {{ dom.label }}
            </div>
          </label>
        </div>
      </div>
    </div>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <!-- Breadcrumb -->
      <CommonBreadcrumbNav class="mb-8" />

      <!-- Layout: Filtres + Grille -->
      <div class="flex flex-col lg:flex-row gap-8">
        <!-- Filtres lateraux (desktop) -->
        <aside class="hidden lg:block w-72 flex-shrink-0">
          <AfricantivesFilters
            v-model="filtres"
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
            class="bg-orange-500 text-white text-xs px-2 py-0.5 rounded-full"
          >
            {{ activeFiltersCount }}
          </span>
        </button>

        <!-- Section initiatives -->
        <div class="flex-1 min-w-0">
          <!-- Barre de resultats -->
          <div class="flex items-center justify-between mb-6">
            <p class="text-gray-600">
              <span class="font-semibold text-gray-900">{{ totalAfricantives }}</span>
              {{ totalAfricantives > 1 ? 'initiatives trouvées' : 'initiative trouvée' }}
            </p>

            <!-- Tri mobile -->
            <select
              v-model="filtres.tri"
              class="lg:hidden px-3 py-2 text-sm border border-gray-200 rounded-lg bg-white"
            >
              <option value="recent">Plus récent</option>
              <option value="ancien">Plus ancien</option>
              <option value="titre">Titre (A-Z)</option>
            </select>
          </div>

          <!-- Chargement -->
          <div
            v-if="chargement"
            class="text-center py-16"
          >
            <div class="animate-spin rounded-full h-12 w-12 border-4 border-orange-500 border-t-transparent mx-auto mb-4"></div>
            <p class="text-gray-500">Chargement des initiatives...</p>
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
              @click="chargerAfricantives"
              class="px-4 py-2 bg-orange-500 text-white rounded-lg hover:bg-orange-600 transition-colors"
            >
              Réessayer
            </button>
          </div>

          <!-- Etat vide -->
          <div
            v-else-if="africantives.length === 0"
            class="text-center py-16 bg-white rounded-2xl shadow-xs"
          >
            <font-awesome-icon
              :icon="['fas', 'lightbulb']"
              class="w-16 h-16 text-gray-300 mx-auto mb-4"
            />
            <h3 class="text-lg font-semibold text-gray-700 mb-2">Aucune initiative trouvée</h3>
            <p class="text-gray-500 mb-4">Essayez de modifier vos critères de recherche</p>
            <button
              @click="resetFilters"
              class="px-4 py-2 bg-orange-500 text-white rounded-lg hover:bg-orange-600 transition-colors"
            >
              Réinitialiser les filtres
            </button>
          </div>

          <!-- Grille d'initiatives -->
          <div
            v-else
            class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6"
          >
            <AfricantivesAfricantiveCard
              v-for="initiative in africantives"
              :key="initiative.id"
              :africantive="initiative"
            />
          </div>

          <!-- Pagination -->
          <div
            v-if="totalPages > 1"
            class="mt-12 flex items-center justify-center gap-2"
          >
            <!-- Precedent -->
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
                  ? 'bg-orange-500 text-white shadow-lg'
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
              <AfricantivesFilters
                v-model="filtres"
                @reset="resetFilters"
              />
            </div>

            <div class="absolute bottom-0 left-0 right-0 p-4 bg-white border-t">
              <button
                @click="showMobileFilters = false"
                class="w-full py-3 bg-orange-500 text-white font-semibold rounded-xl hover:bg-orange-600 transition-colors"
              >
                Voir {{ totalAfricantives }} résultats
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Modal publication -->
    <AfricantivesPublierInitiativeModal
      :is-open="showPublishModal"
      ref="publishModalRef"
      @close="showPublishModal = false"
      @submit="handleSubmitInitiative"
    />

    <!-- Modal connexion requise -->
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
          v-if="showLoginModal"
          class="fixed inset-0 z-50 flex items-center justify-center p-4"
        >
          <div
            class="absolute inset-0 bg-black/50"
            @click="showLoginModal = false"
          ></div>

          <div class="relative bg-white rounded-2xl shadow-xl max-w-md w-full p-6 text-center">
            <font-awesome-icon
              :icon="['fas', 'lock']"
              class="w-16 h-16 text-amber-500 mx-auto mb-4"
            />
            <h2 class="text-xl font-bold text-gray-800 mb-2">
              Connexion requise
            </h2>
            <p class="text-gray-600 mb-6">
              Vous devez être connecté pour publier une initiative.
            </p>
            <div class="flex gap-3 justify-center">
              <button
                @click="showLoginModal = false"
                class="px-5 py-2.5 bg-gray-200 text-gray-700 font-medium rounded-xl hover:bg-gray-300 transition-colors"
              >
                Annuler
              </button>
              <NuxtLink
                to="/login"
                class="px-5 py-2.5 bg-custom-chocolat text-white font-medium rounded-xl hover:bg-custom-chocolat/90 transition-colors inline-flex items-center gap-2"
              >
                <font-awesome-icon :icon="['fas', 'right-to-bracket']" class="w-4 h-4" />
                Se connecter
              </NuxtLink>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import {
  useAfricantives,
  DOMAINES_AFRICANTIVES,
  type AfricantiveAPI,
  type AfricantiveFiltres,
} from '~/composables/useAfricantives'
import { useUserStore } from '~/stores/user'

useHead({
  title: 'Africantives - Initiatives Africaines - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les initiatives africaines qui transforment le continent. Agriculture, technologie, éducation, santé et plus encore.',
    },
  ],
})

const ITEMS_PER_PAGE = 12

const userStore = useUserStore()
const { chargement, erreur, listerAfricantives, creerAfricantive } = useAfricantives()

// State
const africantives = ref<AfricantiveAPI[]>([])
const totalAfricantives = ref(0)
const totalPages = ref(1)
const currentPage = ref(1)
const showMobileFilters = ref(false)
const showPublishModal = ref(false)
const showLoginModal = ref(false)
const publishModalRef = ref<{ setLoading: (v: boolean) => void; setError: (msg: string) => void; setSuccess: () => void } | null>(null)

const filtres = ref({
  domaine: '',
  pays: '',
  recherche: '',
  tri: 'recent',
})

// Domaines pour les filtres chips
const domainesFiltre = DOMAINES_AFRICANTIVES

// Debounce timer pour la recherche
let rechercheTimer: ReturnType<typeof setTimeout> | null = null

// Construire les filtres API
const buildApiFiltres = (): AfricantiveFiltres => {
  const f: AfricantiveFiltres = {
    page: currentPage.value,
    par_page: ITEMS_PER_PAGE,
  }
  if (filtres.value.recherche.trim()) f.recherche = filtres.value.recherche.trim()
  if (filtres.value.domaine) f.domaine = filtres.value.domaine
  if (filtres.value.pays) f.pays = filtres.value.pays
  if (filtres.value.tri !== 'recent') f.tri = filtres.value.tri
  return f
}

// Charger les initiatives
const chargerAfricantives = async () => {
  const resultat = await listerAfricantives(buildApiFiltres())
  if (resultat) {
    africantives.value = resultat.africantives
    totalAfricantives.value = resultat.total
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
  if (filtres.value.domaine) count++
  if (filtres.value.pays) count++
  if (filtres.value.recherche.trim()) count++
  return count
})

// Watchers
watch(
  () => ({
    domaine: filtres.value.domaine,
    pays: filtres.value.pays,
    tri: filtres.value.tri,
  }),
  () => {
    currentPage.value = 1
    chargerAfricantives()
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
      chargerAfricantives()
    }, 300)
  },
)

// Methods
const handleSearch = () => {
  if (rechercheTimer) clearTimeout(rechercheTimer)
  currentPage.value = 1
  chargerAfricantives()
}

const handlePublish = () => {
  if (!userStore.isAuthenticated) {
    showLoginModal.value = true
    return
  }
  showPublishModal.value = true
}

const handleSubmitInitiative = async (data: {
  titre: string
  description: string
  domaine: string
  pays: string
  ville: string
  couvertureFile: File | null
}) => {
  publishModalRef.value?.setLoading(true)

  const resultat = await creerAfricantive(
    {
      titre: data.titre,
      description: data.description,
      domaine: data.domaine || undefined,
      pays: data.pays || undefined,
      ville: data.ville || undefined,
    },
    data.couvertureFile,
  )

  if (resultat) {
    publishModalRef.value?.setSuccess()
    // Recharger la liste pour afficher la nouvelle initiative
    await chargerAfricantives()
  } else {
    publishModalRef.value?.setError(erreur.value || 'Une erreur est survenue lors de la publication.')
  }
}

const resetFilters = () => {
  filtres.value = {
    domaine: '',
    pays: '',
    recherche: '',
    tri: 'recent',
  }
  currentPage.value = 1
}

const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    chargerAfricantives()
    window.scrollTo({ top: 400, behavior: 'smooth' })
  }
}

// Lifecycle
onMounted(async () => {
  await chargerAfricantives()
})
</script>

<style scoped>
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

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
    <AfrolangHero :stats="stats" />

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

      <!-- Erreur entrer dans la salle -->
      <div v-if="erreurEntrer" class="max-w-2xl mx-auto mb-6">
        <div class="px-4 py-3 bg-red-50 border border-red-200 rounded-xl text-red-700 text-sm flex items-center gap-2">
          <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-4 h-4" />
          {{ erreurEntrer }}
          <button class="ml-auto text-red-400 hover:text-red-600" @click="erreurEntrer = null">
            <font-awesome-icon :icon="['fas', 'xmark']" class="w-4 h-4" />
          </button>
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
          <div v-if="initialLoading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
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

          <!-- Salles List -->
          <div v-else-if="salles.length > 0" class="space-y-6 mb-8">
            <div v-for="salle in salles" :key="salle.id">
              <!-- Carte salle -->
              <AfrolangSalleCard
                :salle="salle"
                :expanded="expandedSalleId === salle.id"
                :chargement="salleEnCoursEntree === salle.id"
                class="transform hover:scale-[1.02] transition-all"
                data-aos="fade-up"
                @entrer="entrerDansSalle"
                @toggle-privees="togglePrivees"
              />

              <!-- Section dépliable : salles privées -->
              <Transition name="expand">
                <div
                  v-if="expandedSalleId === salle.id"
                  class="mt-3 bg-blue-50/50 border border-blue-100 rounded-2xl p-4 md:p-6"
                >
                  <!-- Loading privees -->
                  <div v-if="loadingPrivees" class="flex items-center justify-center py-8">
                    <div class="animate-spin rounded-full h-8 w-8 border-3 border-blue-500 border-t-transparent" />
                    <span class="ml-3 text-gray-500 text-sm">Chargement des cours privés...</span>
                  </div>

                  <!-- Liste des salles privées -->
                  <template v-else-if="sallesPriveesCache[salle.id]?.length">
                    <div class="flex items-center justify-between mb-4">
                      <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
                        <font-awesome-icon :icon="['fas', 'door-open']" class="w-4 h-4 text-blue-500" />
                        {{ sallesPriveesCache[salle.id].length }} cours privé{{ sallesPriveesCache[salle.id].length > 1 ? 's' : '' }}
                      </h4>
                      <NuxtLink
                        :to="`/afrolang/${salle.id}`"
                        class="text-xs text-blue-500 hover:text-blue-700 transition-colors"
                      >
                        Voir tout
                      </NuxtLink>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                      <div
                        v-for="sp in sallesPriveesCache[salle.id]"
                        :key="sp.id"
                        class="bg-white rounded-xl border border-gray-100 p-4 hover:shadow-md transition-all"
                      >
                        <div class="flex items-start justify-between gap-3">
                          <div class="flex-1 min-w-0">
                            <div class="flex items-center gap-2 mb-1">
                              <h5 class="font-medium text-gray-800 text-sm truncate">{{ sp.titre }}</h5>
                              <span
                                v-if="sp.est_protegee"
                                class="flex-shrink-0 text-amber-500"
                                title="Protégée par un code d'accès"
                              >
                                <font-awesome-icon :icon="['fas', 'lock']" class="w-3 h-3" />
                              </span>
                            </div>
                            <p v-if="sp.description" class="text-xs text-gray-500 line-clamp-1 mb-2">
                              {{ sp.description }}
                            </p>
                            <div class="flex items-center gap-3 text-xs text-gray-400">
                              <span class="flex items-center gap-1">
                                <font-awesome-icon :icon="['fas', 'user']" class="w-3 h-3" />
                                {{ sp.createur.prenom }} {{ sp.createur.nom }}
                              </span>
                              <span v-if="sp.max_participants" class="flex items-center gap-1">
                                <font-awesome-icon :icon="['fas', 'users']" class="w-3 h-3" />
                                Max {{ sp.max_participants }}
                              </span>
                            </div>
                          </div>

                          <!-- Bouton action -->
                          <div class="flex-shrink-0">
                            <NuxtLink
                              v-if="sp.session_en_cours"
                              :to="`/afrolang/salle-privee/${sp.id}`"
                              class="inline-flex items-center gap-1.5 px-3 py-2 bg-emerald-500 text-white text-xs rounded-lg font-medium hover:bg-emerald-600 transition-colors whitespace-nowrap"
                            >
                              <font-awesome-icon :icon="['fas', 'video']" class="w-3 h-3" />
                              Rejoindre
                            </NuxtLink>
                            <NuxtLink
                              v-else
                              :to="`/afrolang/salle-privee/${sp.id}`"
                              class="inline-flex items-center gap-1.5 px-3 py-2 bg-gray-100 text-gray-600 text-xs rounded-lg font-medium hover:bg-gray-200 transition-colors whitespace-nowrap"
                            >
                              <font-awesome-icon :icon="['fas', 'arrow-right']" class="w-3 h-3" />
                              Entrer
                            </NuxtLink>
                          </div>
                        </div>

                        <!-- Badge en direct -->
                        <div v-if="sp.session_en_cours" class="mt-2 pt-2 border-t border-gray-50">
                          <span class="inline-flex items-center gap-1.5 text-xs text-red-500 font-medium">
                            <span class="w-2 h-2 bg-red-500 rounded-full animate-pulse" />
                            Session en direct
                          </span>
                        </div>
                      </div>
                    </div>
                  </template>

                  <!-- Aucun cours privé -->
                  <div v-else class="text-center py-6">
                    <font-awesome-icon :icon="['fas', 'door-open']" class="w-8 h-8 text-gray-300 mb-3" />
                    <p class="text-gray-500 text-sm">Aucun cours privé dans cette salle</p>
                    <NuxtLink
                      :to="`/afrolang/${salle.id}`"
                      class="inline-flex items-center gap-2 mt-3 px-4 py-2 bg-blue-500 text-white text-sm rounded-lg font-medium hover:bg-blue-600 transition-colors"
                    >
                      <font-awesome-icon :icon="['fas', 'plus']" class="w-3 h-3" />
                      Créer un cours privé
                    </NuxtLink>
                  </div>
                </div>
              </Transition>
            </div>
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
  type SallePriveeAPI,
  type SalleFiltres,
  type AfrolangStats,
} from '~/composables/useAfrolang'
import { useUserStore } from '~/stores/user'

useAOS()

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
const router = useRouter()
const userStore = useUserStore()

const {
  listerSalles,
  obtenirSalle,
  obtenirSallePrivee,
  creerSallePrivee,
  creerSession,
  demarrerSession,
  obtenirStats,
  listerLangues,
} = useAfrolang()

// State
const salles = ref<SalleAPI[]>([])
const total = ref(0)
const totalSalles = ref(0)
const totalPages = ref(1)
const currentPage = ref(1)
const sidebarOpen = ref(false)
const languesDisponibles = ref<string[]>([])
const initialLoading = ref(true)

// Expansion des salles privées
const expandedSalleId = ref<string | null>(null)
const loadingPrivees = ref(false)
const sallesPriveesCache = ref<Record<string, SallePriveeAPI[]>>({})

// Entrer dans la salle (visioconférence)
const salleEnCoursEntree = ref<string | null>(null)
const erreurEntrer = ref<string | null>(null)

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

// ── Entrer dans la salle : trouver/créer session + naviguer vers visio ──

const entrerDansSalle = async (salleId: string) => {
  erreurEntrer.value = null

  // Vérifier authentification
  if (!userStore.isAuthenticated) {
    router.push('/login')
    return
  }

  salleEnCoursEntree.value = salleId

  try {
    // 1. Charger le détail de la salle avec ses salles privées
    const salleDetail = await obtenirSalle(salleId)
    if (!salleDetail) throw new Error('Impossible de charger la salle')

    // 2. Chercher une salle privée avec session en cours
    const spAvecSession = salleDetail.salles_privees.find(sp => sp.session_en_cours)

    if (spAvecSession) {
      // 3a. Trouver la session active dans cette salle privée
      const spDetail = await obtenirSallePrivee(spAvecSession.id)
      if (spDetail) {
        const sessionActive = spDetail.sessions.find(s => s.etat === 'en_cours')
        if (sessionActive) {
          router.push(`/afrolang/session/${sessionActive.id}?rejoindre=1`)
          return
        }
      }
    }

    // 3b. Pas de session active → créer une nouvelle salle privée (l'utilisateur en sera le créateur
    //     et pourra donc y planifier/démarrer des sessions)
    const nouvelleSP = await creerSallePrivee(salleId, {
      titre: `Session ${salleDetail.titre}`,
      description: `Session ouverte - ${salleDetail.titre}`,
      code_acces: '',
      max_participants: null,
    })
    if (!nouvelleSP) throw new Error('Impossible de créer la salle de cours')
    const sallePriveeId = nouvelleSP.id

    // 4. Créer une session
    const nouvelleSession = await creerSession(sallePriveeId, {
      titre: `Session du ${new Date().toLocaleDateString('fr-FR')}`,
      date_debut_prevue: new Date().toISOString(),
      max_participants: null,
      tableau_blanc_actif: true,
    })
    if (!nouvelleSession) throw new Error('Impossible de créer la session')

    // 5. Démarrer la session
    const demarree = await demarrerSession(nouvelleSession.id)
    if (!demarree) throw new Error('Impossible de démarrer la session')

    // 6. Naviguer vers la visioconférence
    router.push(`/afrolang/session/${nouvelleSession.id}?rejoindre=1`)
  } catch (e: any) {
    console.error('Erreur entrerDansSalle:', e)
    erreurEntrer.value = e?.message || 'Une erreur est survenue lors de la connexion à la salle'
  } finally {
    salleEnCoursEntree.value = null
  }
}

// Toggle expansion des salles privées
const togglePrivees = async (salleId: string) => {
  if (expandedSalleId.value === salleId) {
    expandedSalleId.value = null
    return
  }

  expandedSalleId.value = salleId

  // Charger les salles privées si pas en cache
  if (!sallesPriveesCache.value[salleId]) {
    loadingPrivees.value = true
    const detail = await obtenirSalle(salleId)
    if (detail) {
      sallesPriveesCache.value[salleId] = detail.salles_privees
    } else {
      sallesPriveesCache.value[salleId] = []
    }
    loadingPrivees.value = false
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
  initialLoading.value = false
})
</script>

<style scoped>
.expand-enter-active,
.expand-leave-active {
  transition: all 0.3s ease;
  overflow: hidden;
}
.expand-enter-from,
.expand-leave-to {
  opacity: 0;
  max-height: 0;
  transform: translateY(-8px);
}
.expand-enter-to,
.expand-leave-from {
  opacity: 1;
  max-height: 800px;
  transform: translateY(0);
}
.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>

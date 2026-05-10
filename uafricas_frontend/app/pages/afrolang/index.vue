<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Mobile Sidebar (Filtres) -->
    <AfrolangSalleFiltersMobile
      v-model="filtres"
      :is-open="sidebarOpen"
      :total-salles="totalSalles"
      :filtered-count="total"
      :langues="languesDisponibles"
      :pays="paysDisponibles"
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
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1523240795612-9a054b0db644?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-14">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Afrolang
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Sauvons nos langues
        </p>
        <p class="text-white/80 text-sm md:text-base mt-3 max-w-3xl text-center px-4 animate-subtitle">
          Apprendre une langue africaine ou afro-descendante à distance et rencontrer des personnes qui pratiquent la langue et souhaitent l'apprendre.
        </p>
        <button
          v-if="userStore.isAuthenticated"
          type="button"
          class="mt-5 inline-flex items-center gap-2 rounded-full bg-white/95 hover:bg-white text-custom-chocolat font-semibold text-sm px-5 py-2.5 shadow-lg transition-colors animate-subtitle"
          @click="proposerOuvert = true"
        >
          <font-awesome-icon :icon="['fas', 'lightbulb']" class="w-4 h-4" />
          Proposer une salle
        </button>
      </div>
    </div>

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
            :pays="paysDisponibles"
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
          <div v-if="initialLoading" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
            <div
              v-for="n in 6"
              :key="n"
              class="bg-white rounded-xl shadow-md overflow-hidden animate-pulse"
            >
              <div class="h-32 bg-gray-200" />
              <div class="p-3 space-y-3">
                <div class="h-3 bg-gray-200 rounded w-3/4" />
                <div class="h-3 bg-gray-200 rounded w-1/2" />
                <div class="h-8 bg-gray-200 rounded" />
              </div>
            </div>
          </div>

          <!-- Salles List -->
          <template v-else-if="salles.length > 0">
            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 mb-6">
              <AfrolangSalleCard
                v-for="salle in salles"
                :key="salle.id"
                :salle="salle"
                :expanded="expandedSalleId === salle.id"
                :chargement="salleEnCoursEntree === salle.id"
                data-aos="fade-up"
                @entrer="entrerDansSalle"
                @toggle-privees="togglePrivees"
              />
            </div>

            <!-- Widget Canal privé : dropdown des salles privées d'une salle publique -->
            <Transition name="expand">
              <div
                v-if="expandedSalle"
                :key="expandedSalleId!"
                class="mb-8 bg-blue-50/50 border border-blue-100 rounded-2xl p-4 md:p-6"
              >
                <div v-if="loadingPrivees" class="flex items-center justify-center py-8">
                  <div class="animate-spin rounded-full h-8 w-8 border-3 border-blue-500 border-t-transparent" />
                  <span class="ml-3 text-gray-500 text-sm">Chargement des cours privés...</span>
                </div>

                <template v-else>
                  <div class="flex items-center justify-between gap-3 mb-4 flex-wrap">
                    <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
                      <font-awesome-icon :icon="['fas', 'door-open']" class="w-4 h-4 text-blue-500" />
                      {{ (sallesPriveesCache[expandedSalleId!]?.length ?? 0) }}
                      cours privé{{ (sallesPriveesCache[expandedSalleId!]?.length ?? 0) > 1 ? 's' : '' }}
                      — {{ expandedSalle.titre }}
                    </h4>

                    <!-- US4 : bouton Créer ma salle privée / Ouvrir ma salle privée -->
                    <button
                      v-if="userStore.isAuthenticated"
                      type="button"
                      class="px-3 py-1.5 text-xs rounded-lg font-semibold transition-all flex items-center gap-1.5"
                      :class="maSallePriveeIciId
                        ? 'bg-custom-chocolat text-white hover:bg-custom-chocolat/90'
                        : 'bg-blue-500 text-white hover:bg-blue-600'"
                      :disabled="sallePriveeEnCours === (maSallePriveeIciId || 'creation')"
                      @click="maSallePriveeIciId ? ouvrirMaSallePrivee(maSallePriveeIciId) : ouvrirCreationModal(expandedSalleId!)"
                    >
                      <font-awesome-icon
                        :icon="['fas', maSallePriveeIciId ? 'door-open' : 'plus']"
                        class="w-3 h-3"
                      />
                      {{ maSallePriveeIciId ? 'Ouvrir ma salle privée' : 'Créer ma salle privée' }}
                    </button>
                  </div>

                  <div
                    v-if="(sallesPriveesCache[expandedSalleId!]?.length ?? 0) > 0"
                    class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3"
                  >
                    <AfrolangSallePriveeCard
                      v-for="sp in (sallesPriveesCache[expandedSalleId!] ?? [])"
                      :key="sp.id"
                      :salle-privee="sp"
                      :chargement="sallePriveeEnCours === sp.id"
                      @rejoindre="ouvrirJoinModal"
                      @ouvrir="ouvrirMaSallePrivee"
                      @modifier-code="ouvrirModifCodeModal"
                      @archiver="confirmerArchivage"
                    />
                  </div>

                  <div v-else class="text-center py-6">
                    <font-awesome-icon :icon="['fas', 'door-open']" class="w-8 h-8 text-gray-300 mb-3" />
                    <p class="text-gray-500 text-sm">Aucun cours privé dans cette salle</p>
                  </div>
                </template>
              </div>
            </Transition>

            <!-- Erreur widget salle privée -->
            <div v-if="erreurSallePrivee" class="max-w-2xl mx-auto mb-6">
              <div class="px-4 py-3 bg-red-50 border border-red-200 rounded-xl text-red-700 text-sm flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-4 h-4" />
                {{ erreurSallePrivee }}
                <button class="ml-auto text-red-400 hover:text-red-600" @click="erreurSallePrivee = null">
                  <font-awesome-icon :icon="['fas', 'xmark']" class="w-4 h-4" />
                </button>
              </div>
            </div>

            <!-- Modales US2 / US3 / Phase 7 -->
            <AfrolangSallePriveeCreateModal
              ref="createModalRef"
              :is-open="createModalOpen"
              :salle-id="createModalSalleId"
              @close="createModalOpen = false"
              @submit="soumettreCreationSallePrivee"
              @existante="rediriger_vers_salle_existante"
            />

            <AfrolangSallePriveeJoinModal
              ref="joinModalRef"
              :is-open="joinModalOpen"
              :salle-privee-titre="joinModalSalleTitre"
              @close="joinModalOpen = false"
              @submit="soumettreCodeAcces"
            />

            <!-- Modal simple : modification du code secret (auteur uniquement) -->
            <Transition name="modal-fade">
              <div
                v-if="modifCodeModalOpen"
                class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-xs"
                @click.self="modifCodeModalOpen = false"
              >
                <div class="relative w-full max-w-md bg-white shadow-2xl rounded-2xl border-t-4 border-custom-chocolat" @click.stop>
                  <div class="p-6">
                    <div class="flex items-center justify-between mb-4">
                      <h3 class="text-lg font-bold text-gray-800">Modifier le code secret</h3>
                      <button type="button" class="text-gray-400 hover:text-gray-600" @click="modifCodeModalOpen = false">
                        <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
                      </button>
                    </div>
                    <form @submit.prevent="soumettreNouveauCode">
                      <label class="block text-sm font-semibold text-gray-700 mb-2">Nouveau code secret</label>
                      <input
                        v-model="nouveauCode"
                        type="text"
                        pattern="^[A-Za-z0-9!@#$%&*?-]{4,16}$"
                        class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-custom-chocolat font-mono tracking-wider"
                        placeholder="nouveauCode!"
                        required
                      >
                      <p class="text-xs text-gray-400 mt-1">
                        4 à 16 caractères (lettres, chiffres ou <code>!@#$%&amp;*?-</code>).
                      </p>
                      <p v-if="erreurModifCode" class="text-xs text-red-500 mt-2">{{ erreurModifCode }}</p>
                      <div class="flex gap-3 pt-4">
                        <button type="button" class="flex-1 p-3 bg-gray-100 text-gray-700 rounded-xl font-medium" @click="modifCodeModalOpen = false">
                          Annuler
                        </button>
                        <button type="submit" :disabled="modifCodeEnCours" class="flex-1 p-3 bg-custom-chocolat text-white rounded-xl font-medium disabled:opacity-50">
                          {{ modifCodeEnCours ? 'Enregistrement...' : 'Enregistrer' }}
                        </button>
                      </div>
                    </form>
                  </div>
                </div>
              </div>
            </Transition>

            <!-- Toast succès création salle privée -->
            <Transition name="fade-slide">
              <div
                v-if="toastCreation"
                class="fixed bottom-6 right-6 z-60 max-w-sm bg-green-600 text-white rounded-xl shadow-2xl p-4"
              >
                <div class="flex items-start gap-3">
                  <font-awesome-icon :icon="['fas', 'circle-check']" class="w-5 h-5 mt-0.5 shrink-0" />
                  <div class="flex-1 min-w-0">
                    <p class="font-semibold text-sm">Salle privée créée !</p>
                    <p class="text-xs text-green-100 mt-1">
                      Code secret :
                      <code class="bg-green-700/50 px-1.5 py-0.5 rounded font-mono">{{ toastCreation.code }}</code>
                    </p>
                    <p class="text-xs text-green-100 mt-1">Notez-le, il ne sera plus jamais affiché.</p>
                  </div>
                  <button type="button" class="text-white/80 hover:text-white" @click="toastCreation = null">
                    <font-awesome-icon :icon="['fas', 'xmark']" class="w-4 h-4" />
                  </button>
                </div>
              </div>
            </Transition>
          </template>

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

    <!-- Modale Proposer une salle (US1) -->
    <AfrolangProposerSalleModal
      :open="proposerOuvert"
      @close="proposerOuvert = false"
    />
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

// Modale « Proposer une salle » (feature 001-admin-salles-publiques, US1)
const proposerOuvert = ref(false)

const {
  listerSalles,
  listerSallesPriveesParSallePublique,
  creerSallePrivee,
  verifierCodeAcces,
  modifierCodeAcces,
  archiverSallePriveeParAuteur,
  memoriserAccesJeton,
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

// Widget Canal privé : expansion des salles privées
const expandedSalleId = ref<string | null>(null)
const loadingPrivees = ref(false)
const sallesPriveesCache = ref<Record<string, SallePriveeAPI[]>>({})

// Entrer dans la salle (visioconférence publique)
const salleEnCoursEntree = ref<string | null>(null)
const erreurEntrer = ref<string | null>(null)

const _stats = ref<AfrolangStats>({
  total_salles: 0,
  total_salles_privees: 0,
  sessions_en_cours: 0,
  sessions_terminees: 0,
  total_participants_uniques: 0,
})

const filtres = ref<SalleFiltres>({
  recherche: '',
  langue: '',
  pays_id: '',
})

// Pays d'origine disponibles, dérivés des salles affichées (feature 001-afrolang-pays-origine)
const paysDisponibles = computed(() => {
  const map = new Map<string, { id: string; nom: string; code_iso2: string | null }>()
  for (const salle of salles.value) {
    for (const p of salle.pays_origine ?? []) {
      if (!map.has(p.id)) map.set(p.id, p)
    }
  }
  return Array.from(map.values()).sort((a, b) => a.nom.localeCompare(b.nom, 'fr'))
})

const expandedSalle = computed(() =>
  salles.value.find(s => s.id === expandedSalleId.value) ?? null,
)

let rechercheTimer: ReturnType<typeof setTimeout> | null = null

const buildApiFiltres = (): SalleFiltres => {
  const f: SalleFiltres = {
    page: currentPage.value,
    par_page: ITEMS_PER_PAGE,
  }
  if (filtres.value.recherche?.trim()) f.recherche = filtres.value.recherche.trim()
  if (filtres.value.langue) f.langue = filtres.value.langue
  if (filtres.value.pays_id) f.pays_id = filtres.value.pays_id
  return f
}

const chargerSalles = async () => {
  const resultat = await listerSalles(buildApiFiltres())
  if (resultat) {
    salles.value = resultat.salles
    total.value = resultat.total
    totalPages.value = resultat.total_pages
  }
}

// US1 — Entrer dans le livestream public en 1 clic.
const entrerDansSalle = async (salleId: string) => {
  erreurEntrer.value = null

  if (!userStore.isAuthenticated) {
    router.push('/login')
    return
  }

  salleEnCoursEntree.value = salleId
  try {
    await navigateTo(`/afrolang/session/${salleId}`)
  }
  catch (e: unknown) {
    const message = e instanceof Error ? e.message : 'Navigation impossible'
    erreurEntrer.value = message
    console.error('Erreur entrerDansSalle:', e)
  }
  finally {
    salleEnCoursEntree.value = null
  }
}

// ── Widget salles privées : état modales + actions ────────────────────────

const createModalRef = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: () => void
  setExistante: (id?: string) => void
} | null>(null)
const createModalOpen = ref(false)
const createModalSalleId = ref('')
let codeSecretEnAttente = ''

const joinModalRef = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: () => void
} | null>(null)
const joinModalOpen = ref(false)
const joinModalSalleTitre = ref('')
const joinModalSallePriveeId = ref('')

const modifCodeModalOpen = ref(false)
const modifCodeSallePriveeId = ref('')
const nouveauCode = ref('')
const erreurModifCode = ref<string | null>(null)
const modifCodeEnCours = ref(false)

const sallePriveeEnCours = ref<string | null>(null)
const erreurSallePrivee = ref<string | null>(null)
const toastCreation = ref<{ code: string } | null>(null)

/** ID de la salle privée dont l'utilisateur courant est l'auteur, pour la
 *  salle publique actuellement dépliée (US4). `null` si aucune. */
const maSallePriveeIciId = computed<string | null>(() => {
  if (!expandedSalleId.value) return null
  const liste = sallesPriveesCache.value[expandedSalleId.value]
  return liste?.find(sp => sp.est_auteur)?.id ?? null
})

const ouvrirCreationModal = (salleId: string) => {
  if (!userStore.isAuthenticated) {
    router.push('/login')
    return
  }
  createModalSalleId.value = salleId
  createModalOpen.value = true
}

const soumettreCreationSallePrivee = async (payload: { titre: string; description: string; code_acces: string }) => {
  createModalRef.value?.setLoading(true)
  codeSecretEnAttente = payload.code_acces

  const resultat = await creerSallePrivee(createModalSalleId.value, {
    titre: payload.titre,
    description: payload.description,
    code_acces: payload.code_acces,
  })

  if (!resultat) {
    createModalRef.value?.setError('Échec de la création — veuillez réessayer.')
    return
  }
  if ('erreur' in resultat && resultat.erreur === 'salle_privee_unicite') {
    createModalRef.value?.setExistante(resultat.salle_privee_existante_id)
    return
  }

  // Succès : rafraîchir la liste du widget + toast
  const liste = await listerSallesPriveesParSallePublique(createModalSalleId.value)
  sallesPriveesCache.value[createModalSalleId.value] = liste
  createModalRef.value?.setSuccess()
  toastCreation.value = { code: codeSecretEnAttente }
  setTimeout(() => { toastCreation.value = null }, 8000)
}

const rediriger_vers_salle_existante = (_sallePriveeId?: string) => {
  // Aucune navigation dédiée : l'utilisateur peut cliquer "Ouvrir ma salle
  // privée" depuis le widget. Le modale se ferme, il voit le bouton basculer.
  createModalOpen.value = false
  if (createModalSalleId.value) {
    listerSallesPriveesParSallePublique(createModalSalleId.value).then((liste) => {
      sallesPriveesCache.value[createModalSalleId.value] = liste
    })
  }
}

const ouvrirJoinModal = (sallePriveeId: string) => {
  if (!userStore.isAuthenticated) {
    router.push('/login')
    return
  }
  const liste = expandedSalleId.value ? sallesPriveesCache.value[expandedSalleId.value] : null
  joinModalSalleTitre.value = liste?.find(sp => sp.id === sallePriveeId)?.titre ?? ''
  joinModalSallePriveeId.value = sallePriveeId
  joinModalOpen.value = true
}

const soumettreCodeAcces = async (code: string) => {
  joinModalRef.value?.setLoading(true)
  const resultat = await verifierCodeAcces(joinModalSallePriveeId.value, code)

  if (!resultat) {
    joinModalRef.value?.setError('Erreur réseau — veuillez réessayer.')
    return
  }
  if ('erreur' in resultat && resultat.erreur === 'code_incorrect') {
    joinModalRef.value?.setError('Code incorrect.')
    return
  }
  if ('erreur' in resultat && resultat.erreur === 'rate_limit') {
    joinModalRef.value?.setError('Trop de tentatives, réessayez dans quelques minutes.')
    return
  }

  memoriserAccesJeton(joinModalSallePriveeId.value, resultat.acces_jeton, resultat.expires_at)
  joinModalRef.value?.setSuccess()
  sallePriveeEnCours.value = joinModalSallePriveeId.value
  await navigateTo(`/afrolang/session/privee/${joinModalSallePriveeId.value}`)
}

/** Court-circuit auteur : pas de saisie, appel direct `verifier-code` avec
 *  le code actuel inconnu côté client → le backend retourne le jeton parce
 *  que l'utilisateur est `cree_par`. On envoie une chaîne quelconque. */
const ouvrirMaSallePrivee = async (sallePriveeId: string) => {
  if (!userStore.isAuthenticated) {
    router.push('/login')
    return
  }
  sallePriveeEnCours.value = sallePriveeId
  erreurSallePrivee.value = null

  const resultat = await verifierCodeAcces(sallePriveeId, '')
  if (!resultat || 'erreur' in resultat) {
    sallePriveeEnCours.value = null
    erreurSallePrivee.value = 'Impossible d\'ouvrir votre salle privée.'
    return
  }
  memoriserAccesJeton(sallePriveeId, resultat.acces_jeton, resultat.expires_at)
  await navigateTo(`/afrolang/session/privee/${sallePriveeId}`)
}

const ouvrirModifCodeModal = (sallePriveeId: string) => {
  modifCodeSallePriveeId.value = sallePriveeId
  nouveauCode.value = ''
  erreurModifCode.value = null
  modifCodeModalOpen.value = true
}

const soumettreNouveauCode = async () => {
  erreurModifCode.value = null
  const code = nouveauCode.value.trim()
  if (!/^[A-Za-z0-9!@#$%&*?-]{4,16}$/.test(code)) {
    erreurModifCode.value = 'Format invalide (4 à 16 caractères, lettres/chiffres/!@#$%&*?-).'
    return
  }
  modifCodeEnCours.value = true
  const ok = await modifierCodeAcces(modifCodeSallePriveeId.value, code)
  modifCodeEnCours.value = false
  if (!ok) {
    erreurModifCode.value = 'Échec de la modification.'
    return
  }
  modifCodeModalOpen.value = false
}

const confirmerArchivage = async (sallePriveeId: string) => {
  if (!confirm('Archiver définitivement cette salle privée ? La session en cours sera terminée.')) {
    return
  }
  sallePriveeEnCours.value = sallePriveeId
  const ok = await archiverSallePriveeParAuteur(sallePriveeId)
  sallePriveeEnCours.value = null
  if (!ok) {
    erreurSallePrivee.value = 'Échec de l\'archivage.'
    return
  }
  if (expandedSalleId.value) {
    const liste = await listerSallesPriveesParSallePublique(expandedSalleId.value)
    sallesPriveesCache.value[expandedSalleId.value] = liste
  }
}

// Widget Canal privé : toggle dropdown
const togglePrivees = async (salleId: string) => {
  if (expandedSalleId.value === salleId) {
    expandedSalleId.value = null
    return
  }

  expandedSalleId.value = salleId

  if (!sallesPriveesCache.value[salleId]) {
    loadingPrivees.value = true
    try {
      const liste = await listerSallesPriveesParSallePublique(salleId)
      sallesPriveesCache.value[salleId] = liste
    }
    catch (e: unknown) {
      console.error('Erreur listerSallesPriveesParSallePublique:', e)
      sallesPriveesCache.value[salleId] = []
    }
    finally {
      loadingPrivees.value = false
    }
  }
}

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

watch(
  () => filtres.value.langue,
  () => {
    currentPage.value = 1
    chargerSalles()
  },
)

watch(
  () => filtres.value.pays_id,
  () => {
    currentPage.value = 1
    chargerSalles()
  },
)

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

const handleSearch = () => {
  if (rechercheTimer) clearTimeout(rechercheTimer)
  currentPage.value = 1
  chargerSalles()
}

const resetFilters = () => {
  filtres.value = { recherche: '', langue: '', pays_id: '' }
  currentPage.value = 1
}

const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    chargerSalles()
    window.scrollTo({ top: 400, behavior: 'smooth' })
  }
}

onMounted(async () => {
  const [statsResult, languesResult] = await Promise.all([
    obtenirStats(),
    listerLangues(),
  ])

  if (statsResult) {
    _stats.value = statsResult
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

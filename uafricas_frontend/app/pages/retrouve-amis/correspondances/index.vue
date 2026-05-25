<script setup lang="ts">
import type { Correspondance, CorrespondanceFiltres, EtatCorrespondance } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: 'default' })

const userStore = useUserStore()
const { listerCorrespondances, listerNotifications, toutMarquerLu, chargement } = useRetrouvAmis()

// Donnees reactives
const correspondances = ref<Correspondance[]>([])
const total = ref(0)
const nonLues = ref(0)
const page = ref(1)
const parPage = ref(12)
const filtreEtat = ref<string>('')
const filtreAvisId = ref('')
const tri = ref<'score' | 'date'>('score')
const messageSucces = ref('')

// Options de filtre etat
const optionsEtat: { value: string; label: string }[] = [
  { value: '', label: 'Tous les etats' },
  { value: 'en_attente', label: 'En attente' },
  { value: 'acceptee_a', label: 'Acceptee (vous)' },
  { value: 'acceptee_b', label: 'Acceptee (autre)' },
  { value: 'mutuelle', label: 'Mutuelle' },
  { value: 'declinee', label: 'Declinee' },
  { value: 'archivee', label: 'Archivee' },
]

// Nombre total de pages
const totalPages = computed(() => Math.ceil(total.value / parPage.value))

// Chargement des correspondances
const charger = async () => {
  const filtres: CorrespondanceFiltres = {
    page: page.value,
    par_page: parPage.value,
  }
  if (filtreEtat.value) filtres.etat = filtreEtat.value
  if (filtreAvisId.value) filtres.avis_id = filtreAvisId.value

  const res = await listerCorrespondances(filtres)
  if (res) {
    let items = res.correspondances
    // Tri local
    if (tri.value === 'score') {
      items = [...items].sort((a, b) => b.score - a.score)
    } else {
      items = [...items].sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
    }
    correspondances.value = items
    total.value = res.total
  }
}

// Chargement des notifications non lues
const chargerNotifications = async () => {
  const res = await listerNotifications({ par_page: 1 })
  if (res) {
    nonLues.value = res.non_lues
  }
}

// Tout marquer comme lu
const onToutMarquerLu = async () => {
  const res = await toutMarquerLu()
  if (res) {
    nonLues.value = 0
    messageSucces.value = `${res.mises_a_jour} notification(s) marquee(s) comme lue(s)`
    setTimeout(() => { messageSucces.value = '' }, 3000)
  }
}

// Watchers pour filtres
watch([filtreEtat, filtreAvisId, tri], () => {
  page.value = 1
  charger()
})

watch(page, () => charger())

// Navigation vers detail
const voirDetail = (id: string) => {
  navigateTo(`/retrouve-amis/correspondances/${id}`)
}

// Changement de page
const allerPage = (p: number) => {
  if (p >= 1 && p <= totalPages.value) {
    page.value = p
  }
}

// Chargement initial + polling
let intervalle: ReturnType<typeof setInterval> | null = null

onMounted(async () => {
  if (!userStore.isAuthenticated) {
    navigateTo('/login')
    return
  }
  await Promise.all([charger(), chargerNotifications()])
  intervalle = setInterval(() => {
    charger()
    chargerNotifications()
  }, 60000)
})

onUnmounted(() => {
  if (intervalle) clearInterval(intervalle)
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70" />
      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Correspondances
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Consultez et gerez les profils qui correspondent a vos avis de recherche.
          </p>
        </div>
      </div>
    </div>

    <div class="max-w-6xl mx-auto lg:flex lg:gap-8 px-4 py-8">
      <RetrouveAmisSideBar />
      <div class="flex-1 min-w-0">
      <!-- En-tete -->
      <div class="flex items-center justify-between mb-8 gap-4">
        <div class="flex items-center gap-3">
          <span
            v-if="nonLues > 0"
            class="inline-flex items-center justify-center min-w-6 h-6 px-2 bg-red-500 text-white text-xs font-bold rounded-full"
          >
            {{ nonLues }} non lue(s)
          </span>
        </div>
        <button
          v-if="nonLues > 0"
          class="px-4 py-2 text-sm font-medium text-amber-700 bg-amber-50 border border-amber-200 rounded-lg hover:bg-amber-100 transition-colors cursor-pointer"
          :disabled="chargement"
          @click="onToutMarquerLu"
        >
          <font-awesome-icon :icon="['fas', 'check-double']" class="mr-2" />
          Tout marquer comme lu
        </button>
      </div>

      <!-- Message succes -->
      <div
        v-if="messageSucces"
        class="mb-6 p-3 bg-green-50 border border-green-200 text-green-700 text-sm rounded-lg"
      >
        {{ messageSucces }}
      </div>

      <!-- Filtres et tri -->
      <div class="flex flex-col sm:flex-row gap-4 mb-8">
        <select
          v-model="filtreEtat"
          class="px-4 py-2.5 bg-white border border-gray-300 rounded-lg text-sm text-gray-700 focus:outline-none focus:ring-2 focus:ring-amber-500 focus:border-amber-500"
        >
          <option v-for="opt in optionsEtat" :key="opt.value" :value="opt.value">
            {{ opt.label }}
          </option>
        </select>

        <input
          v-model="filtreAvisId"
          type="text"
          placeholder="Filtrer par ID d'avis..."
          class="px-4 py-2.5 bg-white border border-gray-300 rounded-lg text-sm text-gray-700 focus:outline-none focus:ring-2 focus:ring-amber-500 focus:border-amber-500"
        />

        <div class="flex items-center gap-2 ml-auto">
          <span class="text-sm text-gray-500">Trier par :</span>
          <button
            class="px-3 py-2 text-sm rounded-lg border transition-colors cursor-pointer"
            :class="tri === 'score'
              ? 'bg-amber-700 text-white border-amber-700'
              : 'bg-white text-gray-600 border-gray-300 hover:border-amber-400'"
            @click="tri = 'score'"
          >
            Score
          </button>
          <button
            class="px-3 py-2 text-sm rounded-lg border transition-colors cursor-pointer"
            :class="tri === 'date'
              ? 'bg-amber-700 text-white border-amber-700'
              : 'bg-white text-gray-600 border-gray-300 hover:border-amber-400'"
            @click="tri = 'date'"
          >
            Date
          </button>
        </div>
      </div>

      <!-- Chargement -->
      <div v-if="chargement" class="flex justify-center py-20">
        <div class="w-10 h-10 border-4 border-amber-200 border-t-amber-700 rounded-full animate-spin" />
      </div>

      <!-- Grille des correspondances -->
      <div
        v-else-if="correspondances.length > 0"
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
      >
        <RetrouveAmisCorrespondanceCard
          v-for="corr in correspondances"
          :key="corr.id"
          :correspondance="corr"
          @voir="voirDetail"
        />
      </div>

      <!-- Etat vide -->
      <div
        v-else
        class="text-center py-20 bg-white rounded-2xl border border-gray-200"
      >
        <font-awesome-icon :icon="['fas', 'users-slash']" class="text-4xl text-gray-300 mb-4" />
        <p class="text-gray-500 text-lg mb-2">Aucune correspondance trouvee</p>
        <p class="text-gray-400 text-sm">
          Les correspondances apparaitront ici lorsque le systeme identifiera des profils compatibles avec vos avis de recherche.
        </p>
      </div>

      <!-- Pagination -->
      <div
        v-if="totalPages > 1"
        class="flex items-center justify-center gap-2 mt-10"
      >
        <button
          class="px-3 py-2 text-sm rounded-lg border border-gray-300 bg-white text-gray-600 hover:bg-gray-50 transition-colors cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
          :disabled="page <= 1"
          @click="allerPage(page - 1)"
        >
          <font-awesome-icon :icon="['fas', 'chevron-left']" />
        </button>

        <template v-for="p in totalPages" :key="p">
          <button
            v-if="p === 1 || p === totalPages || (p >= page - 1 && p <= page + 1)"
            class="px-3.5 py-2 text-sm rounded-lg border transition-colors cursor-pointer"
            :class="p === page
              ? 'bg-amber-700 text-white border-amber-700'
              : 'bg-white text-gray-600 border-gray-300 hover:bg-gray-50'"
            @click="allerPage(p)"
          >
            {{ p }}
          </button>
          <span
            v-else-if="p === page - 2 || p === page + 2"
            class="px-1 text-gray-400"
          >...</span>
        </template>

        <button
          class="px-3 py-2 text-sm rounded-lg border border-gray-300 bg-white text-gray-600 hover:bg-gray-50 transition-colors cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
          :disabled="page >= totalPages"
          @click="allerPage(page + 1)"
        >
          <font-awesome-icon :icon="['fas', 'chevron-right']" />
        </button>
      </div>
      </div>
    </div>
  </div>
</template>

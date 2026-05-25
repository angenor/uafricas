<script setup lang="ts">
import type { VideoAfrica } from '~/composables/useVidafrica'

const { listerVideos, chargerLanguesDisponibles } = useVidafrica()

const videos = ref<VideoAfrica[]>([])
const languesFiltre = ref<{ code: string; label: string; nombreVideos: number }[]>([])
const chargement = ref(true)

const recherche = ref('')
const langueSelectionnee = ref('')
const page = ref(1)
const totalPages = ref(0)
const total = ref(0)

const charger = async () => {
  chargement.value = true
  const result = await listerVideos({
    page: page.value,
    par_page: 12,
    recherche: recherche.value || undefined,
    langue: langueSelectionnee.value || undefined,
  })
  videos.value = result.videos
  totalPages.value = result.pagination.totalPages
  total.value = result.pagination.total
  page.value = result.pagination.page
  chargement.value = false
}

let debounceTimer: ReturnType<typeof setTimeout> | null = null
const rechercher = () => {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    page.value = 1
    charger()
  }, 300)
}

const changerLangue = (code: string) => {
  langueSelectionnee.value = langueSelectionnee.value === code ? '' : code
  page.value = 1
  charger()
}

const allerPage = (p: number) => {
  page.value = p
  charger()
}

onMounted(async () => {
  languesFiltre.value = await chargerLanguesDisponibles()
  await charger()
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero (compact, titre ↔ description au survol) -->
    <section class="group relative bg-gradient-to-r from-custom-chocolat to-custom-chocolat/80 text-white">
      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12 select-none">
          <h1 class="absolute inset-0 flex items-center justify-center text-2xl md:text-4xl font-bold font-['Oswald'] transition-opacity duration-300 group-hover:opacity-0">
            Vidafrica
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Vidéos sous-titrées en langues africaines et internationales, surlignage karaoké mot par mot.
          </p>
        </div>

        <!-- Barre de recherche -->
        <div class="mt-5 max-w-xl mx-auto">
          <div class="relative">
            <font-awesome-icon icon="search" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm" />
            <input
              v-model="recherche"
              type="text"
              class="w-full pl-10 pr-4 py-2 rounded-lg text-sm text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-white/50"
              placeholder="Rechercher une vidéo..."
              @keydown.enter="rechercher"
              @input="rechercher"
            />
          </div>
        </div>
      </div>
    </section>

    <!-- Filtres langues -->
    <section v-if="languesFiltre.length > 0" class="max-w-5xl mx-auto px-4 py-6">
      <div class="flex flex-wrap gap-2 items-center">
        <span class="text-sm text-gray-500 mr-2">Filtrer par langue :</span>
        <button
          v-for="l in languesFiltre" :key="l.code"
          class="px-3 py-1.5 rounded-full text-sm font-medium transition-all"
          :class="l.code === langueSelectionnee
            ? 'bg-custom-chocolat text-white'
            : 'bg-white text-gray-600 hover:bg-gray-100 border border-gray-200'"
          @click="changerLangue(l.code)"
        >
          {{ l.label }}
          <span class="ml-1 text-xs opacity-60">({{ l.nombreVideos }})</span>
        </button>
      </div>
    </section>

    <!-- Grille de vidéos -->
    <section class="max-w-5xl mx-auto px-4 pb-12">
      <div v-if="chargement" class="flex justify-center py-16">
        <div class="animate-spin rounded-full h-10 w-10 border-b-2 border-custom-chocolat" />
      </div>

      <div v-else-if="videos.length === 0" class="text-center py-16">
        <font-awesome-icon icon="video-slash" class="text-5xl text-gray-300 mb-4" />
        <p class="text-xl text-gray-500">Aucune vidéo trouvée</p>
        <p class="text-sm text-gray-400 mt-1">Essayez de modifier vos critères de recherche</p>
      </div>

      <div v-else>
        <p class="text-sm text-gray-500 mb-4">{{ total }} vidéo{{ total > 1 ? 's' : '' }}</p>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <VidafricaCarteVideo v-for="v in videos" :key="v.id" :video="v" />
        </div>

        <!-- Pagination -->
        <div v-if="totalPages > 1" class="flex justify-center gap-2 mt-8">
          <button
            v-for="p in totalPages" :key="p"
            class="w-10 h-10 rounded-lg text-sm font-medium transition-all"
            :class="p === page
              ? 'bg-custom-chocolat text-white'
              : 'bg-white text-gray-600 hover:bg-gray-100 border border-gray-200'"
            @click="allerPage(p)"
          >
            {{ p }}
          </button>
        </div>
      </div>
    </section>
  </div>
</template>

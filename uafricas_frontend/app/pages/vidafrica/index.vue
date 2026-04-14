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

const rechercher = () => {
  page.value = 1
  charger()
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
    <!-- Hero -->
    <section class="bg-gradient-to-r from-custom-chocolat to-custom-chocolat/80 text-white py-16">
      <div class="max-w-5xl mx-auto px-4 text-center">
        <h1 class="text-4xl md:text-5xl font-bold font-['Oswald'] mb-4">Vidafrica</h1>
        <p class="text-lg text-white/80 max-w-2xl mx-auto">
          Découvrez des vidéos sous-titrées en plusieurs langues africaines et internationales
          avec un surlignage karaoké mot par mot.
        </p>

        <!-- Barre de recherche -->
        <div class="mt-8 max-w-xl mx-auto">
          <div class="flex gap-2">
            <input
              v-model="recherche"
              type="text"
              class="flex-1 px-4 py-3 rounded-lg text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-white/50"
              placeholder="Rechercher une vidéo..."
              @keydown.enter="rechercher"
            />
            <button class="bg-white text-custom-chocolat px-6 py-3 rounded-lg font-medium hover:bg-gray-100 transition" @click="rechercher">
              <font-awesome-icon icon="search" />
            </button>
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

<script setup lang="ts">
import type { TvChannel, TvProgram, TvStat } from '~/composables/useTelevision'
import { defaultCoverVideoUrl } from '~/mocks/tele'

const { listerChaines, listerProgrammesVedettes, obtenirStats, listerPays, listerCategories, chargement } = useTelevision()

useHead({
  title: 'Télévision Africaine | UAfricas',
  meta: [
    { name: 'description', content: 'Regardez les télés africaines : chaque télé regroupe ses programmes, avec un programme à la une qui joue en boucle.' }
  ]
})

// ── Une « télé » = une chaîne regroupant ses vidéos (programmes) ──────────
interface TeleGroupe {
  id: string
  nom: string
  cover: string
  country: string
  category: string
  isLive: boolean
  description: string
  programmes: TvProgram[]
  aLaUne: TvProgram | null
}

const ORPHELINS_ID = '__autres__'

// State
const videoRef = ref<HTMLVideoElement | null>(null)
const audioMuted = ref(true)
const isMobile = ref(false)

// Données chargées depuis l'API
const chaines = ref<TvChannel[]>([])
const programmes = ref<TvProgram[]>([])
const stats = ref<TvStat[]>([])
const paysDisponibles = ref<string[]>([])
const categoriesDisponibles = ref<string[]>([])

// Sélection courante
const teleActiveId = ref<string | null>(null)
const programmeActifId = ref<string | null>(null)

// Filtres
const filtrePays = ref('Tous les territoires')
const filtreCategorie = ref('Toutes les catégories')
const rechercheTexte = ref('')

// ── Regroupement des programmes par télé ─────────────────────────────────
const teles = computed<TeleGroupe[]>(() => {
  const parChaine = new Map<string, TvProgram[]>()
  const orphelins: TvProgram[] = []
  for (const p of programmes.value) {
    if (p.chaineId) {
      if (!parChaine.has(p.chaineId)) parChaine.set(p.chaineId, [])
      parChaine.get(p.chaineId)!.push(p)
    }
    else {
      orphelins.push(p)
    }
  }

  const groupes: TeleGroupe[] = chaines.value.map((c) => {
    const progs = parChaine.get(c.id) || []
    return {
      id: c.id,
      nom: c.name,
      cover: c.cover,
      country: c.country,
      category: c.category,
      isLive: c.isLive,
      description: c.description,
      programmes: progs,
      aLaUne: progs.find(p => p.aLaUne) || progs[0] || null,
    }
  })

  if (orphelins.length) {
    groupes.push({
      id: ORPHELINS_ID,
      nom: 'Autres programmes',
      cover: orphelins[0]?.banner || '/images/tv-default.jpg',
      country: '',
      category: '',
      isLive: false,
      description: 'Programmes non rattachés à une télé.',
      programmes: orphelins,
      aLaUne: orphelins.find(p => p.aLaUne) || orphelins[0] || null,
    })
  }

  return groupes
})

const telesFiltrees = computed(() => {
  let result = teles.value
  if (filtrePays.value && filtrePays.value !== 'Tous les territoires') {
    result = result.filter(t => t.country === filtrePays.value)
  }
  if (filtreCategorie.value && filtreCategorie.value !== 'Toutes les catégories') {
    result = result.filter(t => t.category === filtreCategorie.value)
  }
  if (rechercheTexte.value.trim()) {
    const terme = rechercheTexte.value.toLowerCase().trim()
    result = result.filter(t =>
      t.nom.toLowerCase().includes(terme) ||
      t.description.toLowerCase().includes(terme)
    )
  }
  return result
})

const teleActive = computed<TeleGroupe | null>(() => {
  if (teleActiveId.value) {
    const trouve = teles.value.find(t => t.id === teleActiveId.value)
    if (trouve) return trouve
  }
  return telesFiltrees.value[0] || teles.value[0] || null
})

const programmeActif = computed<TvProgram | null>(() => {
  const t = teleActive.value
  if (!t) return null
  if (programmeActifId.value) {
    const trouve = t.programmes.find(p => p.id === programmeActifId.value)
    if (trouve) return trouve
  }
  return t.aLaUne
})

// ⚠️ PROVISOIRE — vidéo YouTube mise en avant (autoplay) sur l'écran principal de /medias/tele.
// mute=1 est requis par les navigateurs pour autoriser l'autoplay (l'internaute peut réactiver
// le son dans le lecteur). start=17 démarre à 17 s. Pour revenir au comportement normal
// (lecteur des programmes à la une), remettre une chaîne vide : const videoProvisoireEmbed = ''
const videoProvisoireEmbed = 'https://www.youtube.com/embed/JeVaVtr_DCE?autoplay=1&mute=1&start=17&rel=0&playsinline=1'

const heroVideoUrl = computed(() => programmeActif.value?.videoUrl || defaultCoverVideoUrl)

// Methods
const toggleMute = () => {
  if (videoRef.value) {
    videoRef.value.muted = !videoRef.value.muted
    audioMuted.value = videoRef.value.muted
  }
}

const selectionnerTele = (id: string) => {
  teleActiveId.value = id
  programmeActifId.value = null // revient au programme à la une de la télé
  if (import.meta.client) window.scrollTo({ top: 0, behavior: 'smooth' })
}

const selectionnerProgramme = (programme: TvProgram) => {
  programmeActifId.value = programme.id
  if (videoRef.value?.muted) toggleMute()
}

// Recharge la vidéo de l'écran principal quand la source change (boucle conservée)
watch(heroVideoUrl, async () => {
  await nextTick()
  const v = videoRef.value
  if (!v) return
  v.load()
  v.play().catch(() => {})
})

const chargerDonnees = async () => {
  const [chainesResult, programmesResult, statsResult, paysResult, categoriesResult] = await Promise.all([
    listerChaines({ par_page: 100 }),
    listerProgrammesVedettes({ par_page: 100 }),
    obtenirStats(),
    listerPays(),
    listerCategories(),
  ])

  if (chainesResult) chaines.value = chainesResult.chaines
  if (programmesResult) programmes.value = programmesResult.programmes
  if (statsResult) stats.value = statsResult
  if (paysResult) paysDisponibles.value = paysResult
  if (categoriesResult) categoriesDisponibles.value = categoriesResult

  // Sélection par défaut : la première télé qui possède des programmes
  const premiere = teles.value.find(t => t.programmes.length > 0) || teles.value[0]
  if (premiere) teleActiveId.value = premiere.id
}

// Lifecycle
onMounted(() => {
  isMobile.value = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)
  chargerDonnees()
})
</script>

<template>
  <div class="min-h-screen bg-gray-900">
    <!-- Section Vidéo Hero : écran principal (programme à la une, en boucle) -->
    <div class="relative">
      <!-- ⚠️ PROVISOIRE : vidéo YouTube en vedette (autoplay) couvrant l'écran principal.
           Retirer ce bloc + la constante `videoProvisoireEmbed` (script) pour revenir au lecteur normal. -->
      <div v-if="videoProvisoireEmbed" class="absolute inset-x-0 top-24 bottom-0 z-30 bg-black">
        <iframe
          :src="videoProvisoireEmbed"
          class="w-full h-full"
          title="Vidéo télé en vedette"
          frameborder="0"
          allow="autoplay; encrypted-media; picture-in-picture; fullscreen"
          allowfullscreen
        />
      </div>

      <div class="flex absolute">
        <div v-if="!isMobile" class="w-screen h-screen relative">
          <video
            ref="videoRef"
            :src="heroVideoUrl"
            class="z-0 relative h-[86vh] mt-24 w-screen rounded-md shadow-md object-cover"
            autoplay
            loop
            muted
            playsinline
            preload="none"
          />
        </div>
      </div>

      <div class="h-[100vh] w-screen relative overflow-y-hidden flex z-0 bg-gradient-to-t from-black to-transparent">
        <!-- Bouton Mute/Unmute -->
        <button
          v-if="!isMobile"
          @click="toggleMute"
          class="absolute top-32 left-5 rounded-full p-3 bg-custom-chocolat/90 hover:bg-custom-green transition-all shadow-md"
        >
          <div class="relative z-0 m-1">
            <svg class="h-10 w-10" fill="#255033" version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48.824 48.824">
              <g>
                <g>
                  <g>
                    <path d="M28.043,32.874c-0.238,0-0.475-0.085-0.66-0.249l-4.623-4.07h-5.051c-0.552,0-1-0.449-1-1v-7.2c0-0.552,0.448-1,1-1h4.688l5.078-3.508c0.305-0.211,0.702-0.234,1.032-0.062c0.329,0.173,0.535,0.514,0.535,0.886v15.203c0,0.394-0.229,0.749-0.588,0.911C28.322,32.846,28.182,32.874,28.043,32.874z M18.709,26.553h4.428c0.243,0,0.479,0.09,0.66,0.25l3.246,2.857V18.577l-3.766,2.602c-0.166,0.115-0.365,0.177-0.568,0.177h-4V26.553z" />
                  </g>
                  <g>
                    <path d="M24.412,48.824C10.951,48.824,0,37.873,0,24.412S10.951,0,24.412,0s24.412,10.951,24.412,24.412S37.873,48.824,24.412,48.824z M24.412,2C12.055,2,2,12.055,2,24.412C2,36.77,12.055,46.824,24.412,46.824c12.357,0,22.412-10.055,22.412-22.412C46.824,12.054,36.77,2,24.412,2z" />
                  </g>
                </g>
              </g>
            </svg>
          </div>
          <div
            :class="audioMuted ? 'h-12' : 'h-0'"
            class="bg-white transition-all top-3 left-9 rotate-45 w-[2px] rounded-md absolute z-10"
          ></div>
        </button>

        <!-- Contenu inférieur : télé active + ses programmes -->
        <div v-if="!isMobile && teleActive" class="absolute bottom-14 left-10 w-[40rem] max-w-[80vw] text-white">
          <!-- Nom de la télé + badge à la une -->
          <div class="flex items-center gap-3 mb-2">
            <span class="text-sm uppercase tracking-wide text-yellow-400 font-semibold">{{ teleActive.nom }}</span>
            <span
              v-if="programmeActif && programmeActif.id === teleActive.aLaUne?.id"
              class="rounded-full px-3 py-0.5 text-xs bg-yellow-400/20 border border-yellow-400 text-yellow-400 uppercase"
            >
              À la une
            </span>
          </div>

          <a
            v-if="programmeActif?.videoUrl"
            target="_blank"
            :href="programmeActif.videoUrl"
            class="rounded-full mb-2 text-base inline-flex px-4 py-1 border border-yellow-400 bg-yellow-400/10 text-yellow-400 hover:bg-yellow-400/20 transition-colors"
          >
            <div>Voir ce programme</div>
            <div class="pl-2">
              <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
                <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd" />
              </svg>
            </div>
          </a>

          <!-- Timeline des programmes de la télé active -->
          <div v-if="teleActive.programmes.length" class="flex flex-wrap p-1">
            <button
              v-for="programme in teleActive.programmes"
              :key="programme.id"
              type="button"
              @click="selectionnerProgramme(programme)"
              :class="programmeActif?.id === programme.id ? 'bg-yellow-400/30 border border-yellow-400' : 'border border-transparent'"
              class="relative cursor-pointer ml-2 mb-2 p-1 rounded-md transition-colors"
              :title="programme.title"
            >
              <img
                class="object-cover h-16 w-24 z-0 rounded-md shadow-md overflow-hidden"
                :src="programme.banner"
                :alt="programme.title"
              />
              <span
                v-if="programme.aLaUne"
                class="absolute top-1 right-1 z-10 rounded bg-yellow-400 text-black text-[9px] font-bold px-1"
              >
                ★
              </span>
              <span
                v-if="programmeActif?.id === programme.id"
                class="rounded-full h-7 w-7 flex absolute top-7 left-11 z-10"
              >
                <span class="w-1 h-4 bg-yellow-400 rounded-md"></span>
                <span class="ml-1 w-1 h-4 bg-yellow-400 rounded-md"></span>
              </span>
            </button>
          </div>

          <div class="text-xl sm:text-2xl uppercase mt-1">
            {{ programmeActif?.title }}
          </div>
        </div>
      </div>
    </div>

    <!-- Section Télés -->
    <div class="bg-gray-900 px-4 py-12">
      <div class="max-w-6xl mx-auto">
        <h2 class="text-3xl font-bold text-white mb-8 text-center">
          Nos télés <span class="text-yellow-400">Africaines</span>
        </h2>

        <!-- Statistiques -->
        <div v-if="stats.length > 0" class="bg-gradient-to-r from-custom-green to-custom-chocolat rounded-2xl p-8 text-white mb-12">
          <div class="grid grid-cols-2 md:grid-cols-4 gap-6 text-center">
            <div v-for="stat in stats" :key="stat.label" class="p-4">
              <div class="text-4xl font-bold mb-2">{{ stat.value }}</div>
              <div class="text-sm opacity-80">{{ stat.label }}</div>
            </div>
          </div>
        </div>

        <!-- Filtres -->
        <div class="bg-gray-800/60 rounded-xl p-3 flex flex-wrap gap-3 mb-8">
          <div class="relative flex-1 min-w-48">
            <font-awesome-icon
              :icon="['fas', 'magnifying-glass']"
              class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 w-4 h-4"
            />
            <input
              v-model="rechercheTexte"
              type="text"
              placeholder="Rechercher une télé..."
              class="w-full bg-gray-800 text-white text-sm rounded-lg pl-9 pr-3 py-2 focus:outline-none focus:ring-2 focus:ring-yellow-400"
            />
          </div>
          <select
            v-model="filtrePays"
            class="bg-gray-800 text-white text-sm rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-yellow-400"
          >
            <option>Tous les territoires</option>
            <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
          </select>
          <select
            v-model="filtreCategorie"
            class="bg-gray-800 text-white text-sm rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-yellow-400"
          >
            <option>Toutes les catégories</option>
            <option v-for="cat in categoriesDisponibles" :key="cat" :value="cat">{{ cat }}</option>
          </select>
        </div>

        <!-- Loading -->
        <div v-if="chargement" class="flex justify-center py-12">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400"></div>
        </div>

        <!-- Grille des télés -->
        <div v-else-if="telesFiltrees.length > 0" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <button
            v-for="tele in telesFiltrees"
            :key="tele.id"
            type="button"
            @click="selectionnerTele(tele.id)"
            :class="teleActive?.id === tele.id ? 'ring-2 ring-yellow-400' : ''"
            class="text-left bg-gray-800 rounded-xl overflow-hidden transform transition-all hover:scale-105 cursor-pointer"
          >
            <div class="relative aspect-video">
              <img
                :src="tele.cover"
                :alt="tele.nom"
                class="w-full h-full object-cover"
              />
              <div class="absolute inset-0 bg-black/40 flex items-center justify-center">
                <div v-if="tele.isLive" class="w-12 h-12 rounded-full bg-red-600 flex items-center justify-center">
                  <span class="text-white text-xs font-bold">LIVE</span>
                </div>
              </div>
              <!-- Compteur de programmes -->
              <span class="absolute bottom-2 right-2 rounded-full bg-black/70 text-white text-xs px-2 py-0.5">
                {{ tele.programmes.length }} programme{{ tele.programmes.length > 1 ? 's' : '' }}
              </span>
            </div>
            <div class="p-4">
              <h3 class="font-bold text-white text-lg">{{ tele.nom }}</h3>
              <p class="text-gray-400 text-sm line-clamp-2">{{ tele.description }}</p>
              <div class="flex items-center justify-between mt-3">
                <span class="text-xs text-gray-500">{{ tele.country }}</span>
                <span class="text-xs text-custom-green">{{ tele.category }}</span>
              </div>
            </div>
          </button>
        </div>

        <!-- Aucun résultat -->
        <div v-else-if="!chargement" class="text-center py-12">
          <p class="text-gray-400 text-lg">Aucune télé trouvée</p>
          <p class="text-gray-500 text-sm mt-2">Essayez de modifier vos filtres de recherche</p>
        </div>

        <!-- Programmes de la télé active (vue mobile / liste détaillée) -->
        <div v-if="teleActive && teleActive.programmes.length" class="mt-12">
          <h3 class="text-xl font-bold text-white mb-4">
            Programmes de <span class="text-yellow-400">{{ teleActive.nom }}</span>
          </h3>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            <button
              v-for="programme in teleActive.programmes"
              :key="programme.id"
              type="button"
              @click="selectionnerProgramme(programme)"
              :class="programmeActif?.id === programme.id ? 'ring-2 ring-yellow-400' : ''"
              class="text-left bg-gray-800 rounded-lg overflow-hidden flex gap-3 hover:bg-gray-700 transition-colors"
            >
              <div class="relative w-32 shrink-0 aspect-video">
                <img :src="programme.banner" :alt="programme.title" class="w-full h-full object-cover" />
                <span
                  v-if="programme.aLaUne"
                  class="absolute top-1 left-1 rounded bg-yellow-400 text-black text-[10px] font-bold px-1"
                >
                  À la une
                </span>
              </div>
              <div class="py-2 pr-2 min-w-0">
                <p class="text-white text-sm font-semibold truncate">{{ programme.title }}</p>
                <p class="text-gray-400 text-xs line-clamp-2">{{ programme.description }}</p>
              </div>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>

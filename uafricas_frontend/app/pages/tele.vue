<script setup lang="ts">
import { telePrograms, teleStats, tvChannels, defaultCoverVideoUrl, getUniqueTvCountries } from '~/mocks/tele'

useHead({
  title: 'Télévision Africaine | UAfricas',
  meta: [
    { name: 'description', content: 'Regardez les chaînes de télévision africaines en direct. Programmes culturels, actualités et divertissement.' }
  ]
})

// State
const videoRef = ref<HTMLVideoElement | null>(null)
const audioMuted = ref(true)
const currentProgramIndex = ref(0)
const currentTime = ref(0)
const isMobile = ref(false)

// Computed
const currentProgram = computed(() => telePrograms[currentProgramIndex.value])
const availableCountries = computed(() => getUniqueTvCountries())

// Methods
const toggleMute = () => {
  if (videoRef.value) {
    videoRef.value.muted = !videoRef.value.muted
    audioMuted.value = videoRef.value.muted

    if (!currentProgram.value) {
      videoRef.value.currentTime = 0
      currentProgramIndex.value = 0
    }
  }
}

const updateTime = () => {
  if (videoRef.value) {
    currentTime.value = videoRef.value.currentTime

    // Déterminer le programme actuel basé sur le temps
    if (currentTime.value < 20) {
      currentProgramIndex.value = 0
    } else if (currentTime.value >= 20 && currentTime.value < 38) {
      currentProgramIndex.value = 1
    } else if (currentTime.value >= 38 && currentTime.value < 53) {
      currentProgramIndex.value = 2
    } else {
      currentProgramIndex.value = 3
    }
  }
}

const selectProgram = (program: typeof telePrograms[0]) => {
  if (videoRef.value) {
    currentProgramIndex.value = telePrograms.findIndex(p => p.id === program.id)
    videoRef.value.currentTime = program.temp - program.duree
    if (videoRef.value.muted) {
      toggleMute()
    }
  }
}

// Lifecycle
onMounted(() => {
  isMobile.value = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)
  currentProgramIndex.value = 0
})
</script>

<template>
  <div class="min-h-screen bg-gray-900">
    <!-- Section Vidéo Hero -->
    <div class="relative">
      <div class="flex absolute">
        <div v-if="!isMobile" class="w-screen h-screen relative">
          <video
            ref="videoRef"
            class="z-0 relative h-[86vh] mt-24 w-screen rounded-md shadow-md object-cover"
            autoplay
            loop
            muted
            preload="none"
            @timeupdate="updateTime"
          >
            <source :src="defaultCoverVideoUrl" type="video/mp4" />
          </video>
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

        <!-- Contenu inférieur -->
        <div v-if="!isMobile" class="absolute bottom-14 left-10 w-120 text-white">
          <a
            v-if="currentProgram"
            target="_blank"
            :href="currentProgram.lien"
            class="rounded-full ml-5 mb-2 text-base inline-flex px-4 border border-yellow-400 bg-yellow-400/10 text-yellow-400"
          >
            <div>Voir cette initiative</div>
            <div class="pl-2">
              <svg class="h-7 w-7" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
                <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd" />
              </svg>
            </div>
          </a>

          <!-- Timeline des programmes -->
          <div class="flex p-3 uppercase">
            <div
              v-for="program in telePrograms"
              :key="program.id"
              @click="selectProgram(program)"
              :class="currentProgramIndex === telePrograms.findIndex(p => p.id === program.id) ? 'bg-yellow-400/30 border border-yellow-400' : ''"
              class="relative cursor-pointer ml-2 p-1 rounded-md"
            >
              <img
                class="object-cover h-16 w-24 z-0 rounded-md shadow-md overflow-hidden"
                :src="program.banniere"
                :alt="program.titre"
              />
              <button
                v-if="currentProgramIndex === telePrograms.findIndex(p => p.id === program.id)"
                class="rounded-full h-7 w-7 flex absolute top-7 left-11 z-10"
              >
                <div class="w-1 h-4 bg-yellow-400 rounded-md"></div>
                <div class="ml-1 w-1 h-4 bg-yellow-400 rounded-md"></div>
              </button>
            </div>
          </div>

          <div class="sm:text-2xl sm:w-full w-1/2 text-xl uppercase">
            {{ currentProgram?.titre }}
          </div>
        </div>
      </div>
    </div>

    <!-- Section Chaînes TV (pour mobile ou en dessous de la vidéo) -->
    <div class="bg-gray-900 px-4 py-12">
      <div class="max-w-6xl mx-auto">
        <h2 class="text-3xl font-bold text-white mb-8 text-center">
          Chaînes TV <span class="text-yellow-400">Africaines</span>
        </h2>

        <!-- Statistiques -->
        <div class="bg-gradient-to-r from-custom-green to-custom-chocolat rounded-2xl p-8 text-white mb-12">
          <div class="grid grid-cols-2 md:grid-cols-4 gap-6 text-center">
            <div v-for="stat in teleStats" :key="stat.label" class="p-4">
              <div class="text-4xl font-bold mb-2">{{ stat.value }}</div>
              <div class="text-sm opacity-80">{{ stat.label }}</div>
            </div>
          </div>
        </div>

        <!-- Grille des chaînes -->
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <div
            v-for="channel in tvChannels"
            :key="channel.id"
            class="bg-gray-800 rounded-xl overflow-hidden transform transition-all hover:scale-105 cursor-pointer"
          >
            <div class="relative aspect-video">
              <img
                :src="channel.cover"
                :alt="channel.name"
                class="w-full h-full object-cover"
              />
              <div class="absolute inset-0 bg-black/40 flex items-center justify-center">
                <div class="w-12 h-12 rounded-full bg-red-600 flex items-center justify-center">
                  <span class="text-white text-xs font-bold">LIVE</span>
                </div>
              </div>
            </div>
            <div class="p-4">
              <h3 class="font-bold text-white text-lg">{{ channel.name }}</h3>
              <p class="text-gray-400 text-sm line-clamp-2">{{ channel.description }}</p>
              <div class="flex items-center justify-between mt-3">
                <span class="text-xs text-gray-500">{{ channel.country }}</span>
                <span class="text-xs text-custom-green">{{ channel.category }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Message pour mobile -->
        <div v-if="isMobile" class="mt-12 text-center text-gray-400">
          <p>Pour une meilleure expérience, nous vous recommandons de regarder sur un écran plus grand.</p>
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

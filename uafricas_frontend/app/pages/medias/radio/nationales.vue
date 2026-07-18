<script setup lang="ts">
import type { RadioStation } from '~/composables/useStationsRadio'

const { listerStations, listerPays, listerGenres, chargement } = useStationsRadio()

useHead({
  title: 'Radios Nationales | AfricanS',
  meta: [
    { name: 'description', content: 'Écoutez les radios nationales africaines de chaque territoire. Programmes locaux et informations régionales.' }
  ]
})

// State
const audioPlayerRef = ref<HTMLAudioElement | null>(null)
const isPlaying = ref(false)
const volume = ref(0.7)
const isMuted = ref(false)
const currentStationIndex = ref(0)
const isLoading = ref(false)

// Données chargées depuis l'API
const radioStations = ref<RadioStation[]>([])
const filteredStations = ref<RadioStation[]>([])
const availableCountries = ref<string[]>([])
const availableGenres = ref<string[]>([])

// Filtres - Prédéfini sur les nationales
const selectedProgramType = ref('Nationales')
const selectedCountry = ref('Tous les territoires')
const selectedGenre = ref('Tous les genres')

// Computed
const currentStation = computed(() => radioStations.value[currentStationIndex.value])

const programTypes = ['Tous les types', 'Nationales', 'Local', 'International']

// Chargement des données
const chargerStations = async () => {
  const resultat = await listerStations({
    type_station: selectedProgramType.value,
    pays: selectedCountry.value,
    genre: selectedGenre.value,
    par_page: 100,
  })
  if (resultat) {
    filteredStations.value = resultat.stations
  }
}

const chargerToutesStations = async () => {
  const resultat = await listerStations({ par_page: 100 })
  if (resultat) {
    radioStations.value = resultat.stations
  }
  // Charger aussi les stations filtrées (nationales par défaut)
  await chargerStations()
}

const chargerFiltres = async () => {
  const [pays, genres] = await Promise.all([listerPays(), listerGenres()])
  if (pays) availableCountries.value = pays
  if (genres) availableGenres.value = genres
}

// Watcher sur les filtres pour recharger les stations filtrées
watch([selectedProgramType, selectedCountry, selectedGenre], () => {
  chargerStations()
})

// Methods
const updateSource = () => {
  if (audioPlayerRef.value && currentStation.value) {
    audioPlayerRef.value.src = currentStation.value.streamUrl
    if (isPlaying.value) {
      audioPlayerRef.value.play()
    }
  }
}

const togglePlay = () => {
  if (audioPlayerRef.value) {
    if (isPlaying.value) {
      audioPlayerRef.value.pause()
      isPlaying.value = false
    } else {
      isLoading.value = true
      audioPlayerRef.value.play()
        .then(() => {
          isPlaying.value = true
          isLoading.value = false
        })
        .catch(() => {
          isLoading.value = false
        })
    }
  }
}

const nextStation = () => {
  if (radioStations.value.length === 0) return
  currentStationIndex.value = (currentStationIndex.value + 1) % radioStations.value.length
  updateSource()
}

const previousStation = () => {
  if (radioStations.value.length === 0) return
  currentStationIndex.value = (currentStationIndex.value - 1 + radioStations.value.length) % radioStations.value.length
  updateSource()
}

const selectStation = (index: number) => {
  const originalIndex = radioStations.value.findIndex(s => s.id === filteredStations.value[index]?.id)
  if (originalIndex !== -1) {
    currentStationIndex.value = originalIndex
    updateSource()
    if (!isPlaying.value) {
      togglePlay()
    }
  }
}

const getOriginalIndex = (station: RadioStation) => {
  return radioStations.value.findIndex(s => s.id === station.id)
}

const toggleMute = () => {
  if (audioPlayerRef.value) {
    audioPlayerRef.value.muted = !audioPlayerRef.value.muted
    isMuted.value = audioPlayerRef.value.muted
  }
}

const handleVolumeChange = (newVolume: number) => {
  volume.value = newVolume
  if (audioPlayerRef.value) {
    audioPlayerRef.value.volume = newVolume
  }
}

const resetFilters = () => {
  selectedProgramType.value = 'Nationales'
  selectedCountry.value = 'Tous les territoires'
  selectedGenre.value = 'Tous les genres'
}

const handleCanPlay = () => {
  isLoading.value = false
}

const handleAudioEnded = () => {
  nextStation()
}

// Lifecycle
onMounted(async () => {
  await Promise.all([chargerToutesStations(), chargerFiltres()])
  if (audioPlayerRef.value) {
    audioPlayerRef.value.volume = volume.value
    updateSource()
  }
})
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-custom-chocolat to-black relative overflow-hidden">
    <!-- Overlay d'étoiles animées -->
    <div class="stars-container absolute inset-0 z-0">
      <div v-for="n in 50" :key="n" class="star"></div>
    </div>

    <!-- Cercles d'onde audio animés -->
    <div class="absolute inset-0 z-0 flex items-center justify-center overflow-hidden">
      <div
        v-for="n in 3"
        :key="n"
        :class="`audio-wave wave-${n} ${isPlaying ? 'animate-wave' : ''}`"
      ></div>
    </div>

    <!-- Contenu principal -->
    <div class="container mx-auto px-4 pt-28 pb-10 relative z-10">
      <h1 class="text-4xl md:text-6xl font-bold text-center text-white mb-12 animate-pulse-slow">
        Radios <span class="text-yellow-400">Nationales</span>
      </h1>


      <!-- Indicateur de chargement -->
      <div v-if="chargement && radioStations.length === 0" class="text-center py-20">
        <span class="loading loading-spinner loading-lg text-yellow-400"></span>
        <p class="text-gray-300 mt-4">Chargement des stations...</p>
      </div>

      <template v-else>
        <!-- Lecteur principal -->
        <MediaAudioPlayer
          v-if="currentStation"
          :station="currentStation"
          :is-playing="isPlaying"
          :volume="volume"
          :is-muted="isMuted"
          @toggle-play="togglePlay"
          @next="nextStation"
          @previous="previousStation"
          @toggle-mute="toggleMute"
          @volume-change="handleVolumeChange"
        />

        <!-- Filtres -->
        <MediaMediaFilters
          :types="programTypes"
          :countries="availableCountries"
          :genres="availableGenres"
          :selected-type="selectedProgramType"
          :selected-country="selectedCountry"
          :selected-genre="selectedGenre"
          @select-type="selectedProgramType = $event"
          @select-country="selectedCountry = $event"
          @select-genre="selectedGenre = $event"
          @reset="resetFilters"
        />

        <!-- Liste des stations -->
        <div class="max-w-6xl mx-auto mt-6">
          <h2 class="text-2xl font-bold text-white mb-6 flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mr-2" viewBox="0 0 20 20" fill="currentColor">
              <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z" />
            </svg>
            {{ filteredStations.length }} Stations
            <span v-if="selectedProgramType !== 'Tous les types'">&nbsp;{{ selectedProgramType.toLowerCase() }}s</span>
            <span v-if="selectedCountry !== 'Tous les territoires'">&nbsp;en {{ selectedCountry }}</span>
            <span v-if="selectedGenre !== 'Tous les genres'">&nbsp;genre {{ selectedGenre }}</span>
          </h2>

          <div v-if="filteredStations.length === 0 && !chargement" class="text-center py-12">
            <div class="text-yellow-500 text-5xl mb-4">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-24 w-24 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <p class="text-gray-300 text-xl">Aucune station ne correspond à ces critères</p>
            <div class="flex gap-3 justify-center mt-4">
              <button @click="resetFilters" class="bg-yellow-500 hover:bg-yellow-600 text-black font-medium px-6 py-2 rounded-full transition-all">
                Réinitialiser les filtres
              </button>
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
            <MediaStationCard
              v-for="(station, index) in filteredStations"
              :key="station.id"
              :station="station"
              :is-active="currentStationIndex === getOriginalIndex(station)"
              :is-playing="isPlaying && currentStationIndex === getOriginalIndex(station)"
              @select="selectStation(index)"
            />
          </div>
        </div>
      </template>
    </div>

    <!-- Audio element -->
    <audio
      ref="audioPlayerRef"
      @ended="handleAudioEnded"
      @canplay="handleCanPlay"
    ></audio>
  </div>
</template>

<style scoped>
/* Animations */
@keyframes pulse-slow {
  0%, 100% { opacity: 0.7; }
  50% { opacity: 1; }
}

@keyframes wave {
  0% { transform: scale(1); opacity: 1; }
  100% { transform: scale(1.5); opacity: 0; }
}

@keyframes twinkle {
  0%, 100% { opacity: 0.3; transform: scale(0.8); }
  50% { opacity: 1; transform: scale(1); }
}

.animate-pulse-slow {
  animation: pulse-slow 3s infinite;
}

.animate-wave {
  animation: wave 3s infinite;
}

/* Composants visuels */
.audio-wave {
  position: absolute;
  border: 2px solid rgba(251, 191, 36, 0.3);
  border-radius: 50%;
  width: 40vh;
  height: 40vh;
  opacity: 0;
}

.wave-1 { animation-delay: 0s; }
.wave-2 { animation-delay: 1s; }
.wave-3 { animation-delay: 2s; }

.stars-container { z-index: 0; }

.star {
  position: absolute;
  background-color: white;
  border-radius: 50%;
  width: 2px;
  height: 2px;
  animation: twinkle 3s infinite;
}

.stars-container .star:nth-child(3n) { width: 3px; height: 3px; }
.stars-container .star:nth-child(7n) { background-color: #fbbf24; }

/* Positionnement des étoiles */
.stars-container .star {
  top: calc(var(--star-y, 50) * 1%);
  left: calc(var(--star-x, 50) * 1%);
  animation-delay: calc(var(--star-delay, 0) * 1s);
}

.stars-container .star:nth-child(1) { --star-x: 5; --star-y: 10; --star-delay: 0.1; }
.stars-container .star:nth-child(2) { --star-x: 15; --star-y: 20; --star-delay: 0.3; }
.stars-container .star:nth-child(3) { --star-x: 25; --star-y: 5; --star-delay: 0.5; }
.stars-container .star:nth-child(4) { --star-x: 35; --star-y: 30; --star-delay: 0.7; }
.stars-container .star:nth-child(5) { --star-x: 45; --star-y: 12; --star-delay: 0.9; }
.stars-container .star:nth-child(6) { --star-x: 55; --star-y: 22; --star-delay: 1.1; }
.stars-container .star:nth-child(7) { --star-x: 65; --star-y: 7; --star-delay: 1.3; }
.stars-container .star:nth-child(8) { --star-x: 75; --star-y: 15; --star-delay: 1.5; }
.stars-container .star:nth-child(9) { --star-x: 85; --star-y: 25; --star-delay: 1.7; }
.stars-container .star:nth-child(10) { --star-x: 95; --star-y: 18; --star-delay: 1.9; }
.stars-container .star:nth-child(11) { --star-x: 10; --star-y: 35; --star-delay: 0.2; }
.stars-container .star:nth-child(12) { --star-x: 20; --star-y: 45; --star-delay: 0.4; }
.stars-container .star:nth-child(13) { --star-x: 30; --star-y: 55; --star-delay: 0.6; }
.stars-container .star:nth-child(14) { --star-x: 40; --star-y: 40; --star-delay: 0.8; }
.stars-container .star:nth-child(15) { --star-x: 50; --star-y: 50; --star-delay: 1.0; }
.stars-container .star:nth-child(16) { --star-x: 60; --star-y: 35; --star-delay: 1.2; }
.stars-container .star:nth-child(17) { --star-x: 70; --star-y: 45; --star-delay: 1.4; }
.stars-container .star:nth-child(18) { --star-x: 80; --star-y: 55; --star-delay: 1.6; }
.stars-container .star:nth-child(19) { --star-x: 90; --star-y: 40; --star-delay: 1.8; }
.stars-container .star:nth-child(20) { --star-x: 8; --star-y: 60; --star-delay: 2.0; }
</style>

<script setup lang="ts">
import type { RadioStation } from '~/mocks/radios'

interface Props {
  station: RadioStation
  isPlaying: boolean
  volume: number
  isMuted: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'play'): void
  (e: 'pause'): void
  (e: 'next'): void
  (e: 'previous'): void
  (e: 'togglePlay'): void
  (e: 'toggleMute'): void
  (e: 'volumeChange', volume: number): void
}>()

const localVolume = ref(props.volume)

watch(() => props.volume, (newVolume) => {
  localVolume.value = newVolume
})

const handleVolumeChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  emit('volumeChange', parseFloat(target.value))
}
</script>

<template>
  <div class="max-w-4xl mx-auto bg-black bg-opacity-60 backdrop-blur-md rounded-2xl p-6 shadow-xl border border-gray-800">
    <!-- Station en cours -->
    <div class="flex flex-col md:flex-row items-center mb-8">
      <div class="w-full md:w-1/3 relative mb-6 md:mb-0">
        <div class="aspect-square rounded-2xl overflow-hidden border-4 border-yellow-400 shadow-lg transform transition-all hover:scale-105">
          <img
            :src="station.cover"
            alt="Station cover"
            class="w-full h-full object-cover"
          />
          <!-- Vinyle animé -->
          <div :class="['absolute inset-0 flex items-center justify-center', { 'animate-spin-slow': isPlaying }]">
            <div class="w-1/2 h-1/2 rounded-full bg-black opacity-80"></div>
            <div class="w-1/6 h-1/6 rounded-full bg-yellow-400 absolute"></div>
          </div>
        </div>
      </div>

      <div class="w-full md:w-2/3 md:pl-8 text-center md:text-left">
        <h2 class="text-3xl font-bold text-yellow-400 mb-2">{{ station.name }}</h2>
        <p class="text-gray-300 mb-4">{{ station.description }}</p>

        <div class="flex items-center justify-center md:justify-start space-x-3 text-gray-400 mb-6">
          <span class="flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
            </svg>
            En direct
          </span>
          <span class="flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
            </svg>
            {{ station.genresList.join(', ') }}
          </span>
          <span class="flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd" />
            </svg>
            {{ station.location }}
          </span>
        </div>

        <!-- Visualiseur audio -->
        <div class="h-12 mb-6 bg-gray-800 rounded-lg overflow-hidden">
          <div class="w-full h-full flex items-end justify-around">
            <div
              v-for="n in 30"
              :key="n"
              :class="['audio-bar', { 'animate-equalizer': isPlaying }]"
              :style="{
                'animation-delay': `${n * 0.05}s`,
                height: isPlaying ? `${Math.random() * 100}%` : '10%',
                'background-color': `hsl(${n * 12}, 80%, 60%)`
              }"
            ></div>
          </div>
        </div>

        <!-- Contrôles -->
        <div class="flex items-center justify-center md:justify-between">
          <div class="volume-control hidden md:flex items-center space-x-2 text-white">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M9.383 3.076A1 1 0 0110 4v12a1 1 0 01-1.707.707L4.586 13H2a1 1 0 01-1-1V8a1 1 0 011-1h2.586l3.707-3.707a1 1 0 011.09-.217zM14.657 2.929a1 1 0 011.414 0A9.972 9.972 0 0119 10a9.972 9.972 0 01-2.929 7.071a1 1 0 01-1.414-1.414A7.971 7.971 0 0017 10c0-2.21-.894-4.208-2.343-5.657a1 1 0 010-1.414zm-2.829 2.828a1 1 0 011.415 0A5.983 5.983 0 0115 10a5.984 5.984 0 01-1.757 4.243a1 1 0 01-1.415-1.415A3.984 3.984 0 0013 10a3.983 3.983 0 00-1.172-2.828a1 1 0 010-1.415z" clip-rule="evenodd" />
            </svg>
            <input
              type="range"
              :value="localVolume"
              @input="handleVolumeChange"
              min="0"
              max="1"
              step="0.01"
              class="w-24 accent-yellow-400"
            />
          </div>

          <div class="play-controls flex items-center justify-center space-x-4">
            <button @click="emit('previous')" class="transform transition hover:scale-110 focus:outline-none">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-white" viewBox="0 0 20 20" fill="currentColor">
                <path d="M8.445 14.832A1 1 0 0010 14v-2.798l5.445 3.63A1 1 0 0017 14V6a1 1 0 00-1.555-.832L10 8.798V6a1 1 0 00-1.555-.832l-6 4a1 1 0 000 1.664l6 4z" />
              </svg>
            </button>

            <button
              @click="emit('togglePlay')"
              class="w-16 h-16 rounded-full flex items-center justify-center bg-yellow-400 transform transition hover:scale-110 focus:outline-none"
            >
              <svg v-if="!isPlaying" xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-black" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
              </svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-black" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
              </svg>
            </button>

            <button @click="emit('next')" class="transform transition hover:scale-110 focus:outline-none">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-white" viewBox="0 0 20 20" fill="currentColor">
                <path d="M4.555 5.168A1 1 0 003 6v8a1 1 0 001.555.832L10 11.202V14a1 1 0 001.555.832l6-4a1 1 0 000-1.664l-6-4A1 1 0 0010 6v2.798L4.555 5.168z" />
              </svg>
            </button>
          </div>

          <div class="hidden md:block">
            <button @click="emit('toggleMute')" class="text-white hover:text-yellow-400">
              <svg v-if="!isMuted" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M9.383 3.076A1 1 0 0110 4v12a1 1 0 01-1.707.707L4.586 13H2a1 1 0 01-1-1V8a1 1 0 011-1h2.586l3.707-3.707a1 1 0 011.09-.217zM14.657 2.929a1 1 0 011.414 0A9.972 9.972 0 0119 10a9.972 9.972 0 01-2.929 7.071a1 1 0 01-1.414-1.414A7.971 7.971 0 0017 10c0-2.21-.894-4.208-2.343-5.657a1 1 0 010-1.414zm-2.829 2.828a1 1 0 011.415 0A5.983 5.983 0 0115 10a5.984 5.984 0 01-1.757 4.243a1 1 0 01-1.415-1.415A3.984 3.984 0 0013 10a3.983 3.983 0 00-1.172-2.828a1 1 0 010-1.415z" clip-rule="evenodd" />
              </svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M9.383 3.076A1 1 0 0110 4v12a1 1 0 01-1.707.707L4.586 13H2a1 1 0 01-1-1V8a1 1 0 011-1h2.586l3.707-3.707a1 1 0 011.09-.217z" clip-rule="evenodd" />
                <path fill-rule="evenodd" d="M12.707 7.293a1 1 0 011.414 0L15 8.172l.879-.879a1 1 0 111.414 1.414L16.414 9.586l.879.879a1 1 0 11-1.414 1.414L15 11l-.879.879a1 1 0 11-1.414-1.414l.879-.879-.879-.879a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes spin-slow {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@keyframes equalizer {
  0%, 100% { height: 10%; }
  50% { height: 80%; }
}

.animate-spin-slow {
  animation: spin-slow 8s linear infinite;
}

.animate-equalizer {
  animation: equalizer 1.3s ease-in-out infinite;
}

.audio-bar {
  width: 3px;
  margin: 0 2px;
  border-radius: 2px;
  transition: height 0.2s ease;
}
</style>

<script setup lang="ts">
import type { RadioStation } from '~/mocks/radios'

interface Props {
  station: RadioStation
  isActive: boolean
  isPlaying: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'select'): void
}>()
</script>

<template>
  <div
    @click="emit('select')"
    :class="[
      'station-card bg-gray-900 rounded-xl p-4 cursor-pointer transform transition-all hover:scale-105',
      isActive ? 'border-2 border-yellow-400' : 'border border-gray-800'
    ]"
  >
    <div class="relative aspect-square rounded-lg overflow-hidden mb-4">
      <img
        :src="station.cover"
        :alt="station.name"
        class="w-full h-full object-cover"
      />
      <span
        v-if="station.aLaUne"
        class="absolute top-2 left-2 z-10 inline-flex items-center gap-1 rounded-full bg-yellow-400 px-2 py-0.5 text-[10px] font-bold uppercase tracking-wide text-black shadow"
      >
        ★ À la une
      </span>
      <div class="absolute inset-0 bg-black/40 flex items-center justify-center">
        <div
          v-if="isActive && isPlaying"
          class="w-12 h-12 rounded-full bg-yellow-400/70 flex items-center justify-center animate-pulse"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6 text-black"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path d="M18 3a1 1 0 00-1.196-.98l-10 2A1 1 0 006 5v9.114A4.369 4.369 0 005 14c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V7.82l8-1.6v5.894A4.37 4.37 0 0015 12c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V3z" />
          </svg>
        </div>
        <button v-else class="w-12 h-12 rounded-full bg-yellow-400/70 flex items-center justify-center">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6 text-black"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
              clip-rule="evenodd"
            />
          </svg>
        </button>
      </div>
    </div>
    <h3 class="font-bold text-white">{{ station.name }}</h3>
    <p class="text-sm text-gray-400 truncate">
      {{ station.genresList.join(', ') }}
    </p>
    <div class="flex items-center justify-between mt-2">
      <span class="text-xs text-gray-500 flex items-center">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-3 w-3 mr-1"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fill-rule="evenodd"
            d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z"
            clip-rule="evenodd"
          />
        </svg>
        {{ station.location }}
      </span>
      <div class="flex space-x-1">
        <div
          v-for="i in 5"
          :key="i"
          class="w-1 h-4 bg-yellow-400 rounded-full"
          :class="{ 'animate-equalizer': isActive && isPlaying }"
          :style="{ 'animation-delay': `${i * 0.1}s` }"
        ></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes equalizer {
  0%, 100% { height: 10%; }
  50% { height: 80%; }
}

.animate-equalizer {
  animation: equalizer 1.3s ease-in-out infinite;
}
</style>

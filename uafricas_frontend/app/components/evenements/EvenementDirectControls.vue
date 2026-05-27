<template>
  <!-- Barre de contrôles du direct (Tailwind v4 pur). Conditionnée au rôle :
       diffuseur (organisateur/intervenant) → micro/caméra/écran ;
       organisateur → clôturer ; spectateur → lever la main ; tous → quitter. -->
  <div class="flex items-center justify-center gap-2 sm:gap-3 px-4 py-3 bg-gray-800 border-t border-gray-700">
    <!-- Contrôles diffuseur -->
    <template v-if="estDiffuseur">
      <button
        type="button"
        :disabled="!connected"
        class="flex h-12 w-12 items-center justify-center rounded-full transition-colors disabled:opacity-40"
        :class="microActif ? 'bg-gray-600 text-white hover:bg-gray-500' : 'bg-red-500 text-white hover:bg-red-600'"
        :aria-label="microActif ? 'Couper le micro' : 'Activer le micro'"
        :title="microActif ? 'Couper le micro' : 'Activer le micro'"
        @click="emit('toggle-micro')"
      >
        <font-awesome-icon :icon="['fas', microActif ? 'microphone' : 'microphone-slash']" class="h-5 w-5" />
      </button>
      <button
        type="button"
        :disabled="!connected"
        class="flex h-12 w-12 items-center justify-center rounded-full transition-colors disabled:opacity-40"
        :class="cameraActive ? 'bg-gray-600 text-white hover:bg-gray-500' : 'bg-red-500 text-white hover:bg-red-600'"
        :aria-label="cameraActive ? 'Couper la caméra' : 'Activer la caméra'"
        :title="cameraActive ? 'Couper la caméra' : 'Activer la caméra'"
        @click="emit('toggle-camera')"
      >
        <font-awesome-icon :icon="['fas', cameraActive ? 'video' : 'video-slash']" class="h-5 w-5" />
      </button>
      <button
        type="button"
        :disabled="!connected"
        class="hidden h-12 w-12 items-center justify-center rounded-full transition-colors disabled:opacity-40 sm:flex"
        :class="ecranPartage ? 'bg-custom-green text-white hover:brightness-110' : 'bg-gray-600 text-white hover:bg-gray-500'"
        :aria-label="ecranPartage ? 'Arrêter le partage d\'écran' : 'Partager l\'écran'"
        :title="ecranPartage ? 'Arrêter le partage d\'écran' : 'Partager l\'écran'"
        @click="emit('toggle-ecran')"
      >
        <font-awesome-icon :icon="['fas', 'display']" class="h-5 w-5" />
      </button>
    </template>

    <!-- Lever la main (spectateur uniquement) -->
    <button
      v-if="role === 'spectateur'"
      type="button"
      :disabled="!connected"
      class="flex items-center gap-2 rounded-full px-4 py-3 text-sm font-medium transition-colors disabled:opacity-40"
      :class="mainLevee ? 'bg-amber-500 text-white hover:bg-amber-600' : 'bg-gray-600 text-white hover:bg-gray-500'"
      :aria-label="mainLevee ? 'Baisser la main' : 'Lever la main'"
      @click="emit('lever-main')"
    >
      <font-awesome-icon :icon="['fas', 'hand']" class="h-5 w-5" />
      <span class="hidden sm:inline">{{ mainLevee ? 'Baisser la main' : 'Lever la main' }}</span>
    </button>

    <!-- Insert (picker réactions fourni par le parent) -->
    <slot name="reactions" />

    <!-- Clôturer (organisateur uniquement) -->
    <button
      v-if="role === 'organisateur'"
      type="button"
      class="flex items-center gap-2 rounded-full bg-red-600 px-4 py-3 text-sm font-semibold text-white transition-colors hover:bg-red-700"
      aria-label="Clôturer le direct"
      @click="emit('cloturer')"
    >
      <font-awesome-icon :icon="['fas', 'circle-stop']" class="h-5 w-5" />
      <span class="hidden sm:inline">Clôturer</span>
    </button>

    <!-- Quitter (tous) -->
    <button
      type="button"
      class="flex items-center gap-2 rounded-full bg-gray-700 px-4 py-3 text-sm font-medium text-gray-200 transition-colors hover:bg-gray-600"
      aria-label="Quitter le direct"
      @click="emit('quitter')"
    >
      <font-awesome-icon :icon="['fas', 'right-from-bracket']" class="h-5 w-5" />
      <span class="hidden sm:inline">Quitter</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import type { RoleDirect } from '~/composables/useEvenements'

const props = defineProps<{
  role: RoleDirect
  microActif: boolean
  cameraActive: boolean
  ecranPartage: boolean
  mainLevee: boolean
  connected: boolean
}>()

const emit = defineEmits<{
  'toggle-micro': []
  'toggle-camera': []
  'toggle-ecran': []
  'lever-main': []
  cloturer: []
  quitter: []
}>()

const estDiffuseur = computed(() => props.role === 'organisateur' || props.role === 'intervenant')
</script>

<template>
  <div class="bg-gray-800 border-t border-gray-700 px-4 py-3">
    <div class="flex items-center justify-center gap-2 sm:gap-3">
      <!-- Micro -->
      <button
        class="w-12 h-12 rounded-full flex items-center justify-center transition-all"
        :class="microActif
          ? 'bg-gray-700 hover:bg-gray-600 text-white'
          : 'bg-red-500 hover:bg-red-600 text-white'"
        :disabled="!connected"
        :title="microActif ? 'Couper le micro' : 'Activer le micro'"
        @click="$emit('toggleMicro')"
      >
        <font-awesome-icon
          :icon="['fas', microActif ? 'volume-up' : 'volume-mute']"
          class="w-5 h-5"
        />
      </button>

      <!-- Camera -->
      <button
        class="w-12 h-12 rounded-full flex items-center justify-center transition-all"
        :class="cameraActive
          ? 'bg-gray-700 hover:bg-gray-600 text-white'
          : 'bg-red-500 hover:bg-red-600 text-white'"
        :disabled="!connected"
        :title="cameraActive ? 'Couper la caméra' : 'Activer la caméra'"
        @click="$emit('toggleCamera')"
      >
        <font-awesome-icon
          :icon="['fas', 'video']"
          class="w-5 h-5"
        />
      </button>

      <!-- Partage ecran -->
      <button
        class="w-12 h-12 rounded-full flex items-center justify-center transition-all"
        :class="ecranPartage
          ? 'bg-blue-500 hover:bg-blue-600 text-white'
          : 'bg-gray-700 hover:bg-gray-600 text-white'"
        :disabled="!connected"
        title="Partager l'écran"
        @click="$emit('toggleEcran')"
      >
        <font-awesome-icon :icon="['fas', 'display']" class="w-5 h-5" />
      </button>

      <!-- Tableau blanc -->
      <button
        class="w-12 h-12 rounded-full flex items-center justify-center transition-all"
        :class="tableauBlancOuvert
          ? 'bg-emerald-500 hover:bg-emerald-600 text-white'
          : 'bg-gray-700 hover:bg-gray-600 text-white'"
        :disabled="!connected"
        title="Tableau blanc"
        @click="$emit('toggleTableauBlanc')"
      >
        <font-awesome-icon :icon="['fas', 'chalkboard-user']" class="w-5 h-5" />
      </button>

      <!-- Réactions emoji (slot injecté par AfrolangRoom) -->
      <slot name="reactions" />

      <!-- Separator -->
      <div class="w-px h-8 bg-gray-600 mx-1" />

      <!-- Terminer (moderateur seulement) -->
      <button
        v-if="estModerateur"
        class="px-4 h-12 rounded-full bg-amber-600 hover:bg-amber-700 text-white font-medium text-sm flex items-center gap-2 transition-all"
        :disabled="!connected"
        @click="$emit('terminer')"
      >
        <font-awesome-icon :icon="['fas', 'circle-check']" class="w-4 h-4" />
        <span class="hidden sm:inline">Terminer</span>
      </button>

      <!-- Quitter -->
      <button
        class="px-4 h-12 rounded-full bg-red-600 hover:bg-red-700 text-white font-medium text-sm flex items-center gap-2 transition-all"
        @click="$emit('quitter')"
      >
        <font-awesome-icon :icon="['fas', 'right-from-bracket']" class="w-4 h-4" />
        <span class="hidden sm:inline">Quitter</span>
      </button>

      <!-- Actions supplémentaires (ex : créer ma salle privée) -->
      <slot name="apres-actions" />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  microActif: boolean
  cameraActive: boolean
  ecranPartage: boolean
  tableauBlancOuvert: boolean
  estModerateur: boolean
  connected: boolean
}>()

defineEmits<{
  toggleMicro: []
  toggleCamera: []
  toggleEcran: []
  toggleTableauBlanc: []
  quitter: []
  terminer: []
}>()
</script>

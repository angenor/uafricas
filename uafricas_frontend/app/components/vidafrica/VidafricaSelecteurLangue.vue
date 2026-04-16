<script setup lang="ts">
import { LANGUES_LABELS } from '~/mocks/vidafrica'

const props = defineProps<{
  langues: string[]
  langueActive: string
}>()

const emit = defineEmits<{
  'changer-langue': [langue: string]
}>()
</script>

<template>
  <div class="flex flex-wrap gap-2">
    <button
      v-for="langue in langues" :key="langue"
      class="px-4 py-2 rounded-full text-sm font-medium transition-all"
      :class="langue === langueActive
        ? 'bg-custom-chocolat text-white shadow-md'
        : 'bg-gray-100 text-gray-700 hover:bg-gray-200'"
      @click="emit('changer-langue', langue)"
    >
      {{ LANGUES_LABELS[langue] || langue }}
    </button>
  </div>
</template>

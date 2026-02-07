<template>
  <div
    class="h-80 relative w-72 hover:scale-95 transition-transform font-bold hover:shadow-none bg-white rounded-md mt-2 overflow-hidden shadow-md cursor-pointer"
    @click="$emit('click', programme)"
  >
    <!-- Image de couverture -->
    <div class="overflow-hidden relative">
      <img
        class="w-full h-40 object-cover rounded-t-md transition-all duration-1000"
        :src="programme.couverture_url || '/images/carte-afrique.jpg'"
        :alt="programme.titre"
      />
      <!-- Badge type -->
      <span
        class="absolute top-2 right-2 px-2 py-1 rounded-full text-xs font-medium"
        :class="programme.interafricain ? 'bg-custom-green text-white' : 'bg-custom-chocolat text-white'"
      >
        {{ programme.interafricain ? 'Interafricain' : 'Hors Afrique' }}
      </span>
    </div>

    <!-- Contenu -->
    <div class="w-full py-2">
      <!-- Pays -->
      <div class="px-2 text-sm capitalize font-normal text-gray-500 flex items-center">
        <font-awesome-icon class="h-4 mr-1" :icon="['fas', 'location-dot']" />
        <span>{{ programme.pays }}</span>
        <span v-if="programme.ville" class="border-l-2 pl-1 ml-1 border-gray-400">
          {{ programme.ville }}
        </span>
      </div>

      <!-- Titre -->
      <div class="px-2 line-clamp-2 text-gray-800 text-lg mt-1">
        {{ programme.titre }}
      </div>

      <!-- Date -->
      <div class="absolute bottom-2 left-0">
        <div class="inline-flex px-2 font-bold text-sm text-custom-chocolat bg-gray-100/14 rounded-r-full items-center">
          <font-awesome-icon
            class="h-4 w-4 mr-2 text-gray-400"
            :icon="['fas', 'calendar-days']"
          />
          <span>{{ formatDate(programme.date_debut) }}</span>
          <span class="ml-2 text-xs font-normal text-gray-400">
            {{ programme.duree_label }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SabbatiqueAPI } from '~/composables/useSabbatiques'

defineProps<{
  programme: SabbatiqueAPI
}>()

defineEmits<{
  (e: 'click', programme: SabbatiqueAPI): void
}>()

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr + 'T00:00:00')
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(date)
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>

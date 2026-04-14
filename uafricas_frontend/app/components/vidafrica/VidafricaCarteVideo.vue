<script setup lang="ts">
import type { VideoAfrica } from '~/composables/useVidafrica'
import { LANGUES_LABELS, formaterDuree } from '~/mocks/vidafrica'

const props = defineProps<{
  video: VideoAfrica
}>()
</script>

<template>
  <NuxtLink
    :to="`/vidafrica/${video.slug}`"
    class="group block bg-white rounded-xl overflow-hidden shadow-sm hover:shadow-md transition-shadow"
  >
    <!-- Vignette -->
    <div class="aspect-video bg-gray-200 relative overflow-hidden">
      <img
        v-if="video.vignetteUrl"
        :src="video.vignetteUrl"
        :alt="video.titre"
        class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
      />
      <div v-else class="w-full h-full flex items-center justify-center">
        <font-awesome-icon icon="video" class="text-4xl text-gray-300" />
      </div>

      <!-- Durée -->
      <span v-if="video.dureeSecondes" class="absolute bottom-2 right-2 bg-black/75 text-white text-xs px-2 py-1 rounded">
        {{ formaterDuree(video.dureeSecondes) }}
      </span>
    </div>

    <!-- Infos -->
    <div class="p-4">
      <h3 class="font-semibold text-gray-900 line-clamp-2 group-hover:text-custom-chocolat transition-colors">
        {{ video.titre }}
      </h3>

      <!-- Badges langues -->
      <div v-if="video.languesDisponibles.length > 0" class="flex flex-wrap gap-1 mt-2">
        <span
          v-for="lang in video.languesDisponibles.slice(0, 3)" :key="lang"
          class="text-xs bg-gray-100 text-gray-600 px-2 py-0.5 rounded-full"
        >
          {{ LANGUES_LABELS[lang] || lang }}
        </span>
        <span v-if="video.languesDisponibles.length > 3" class="text-xs text-gray-400">
          +{{ video.languesDisponibles.length - 3 }}
        </span>
      </div>
    </div>
  </NuxtLink>
</template>

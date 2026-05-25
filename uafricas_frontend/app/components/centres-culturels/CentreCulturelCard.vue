<script setup lang="ts">
import type { CentreCulturelAPI } from '~/composables/useCentresCulturels'

defineProps<{
  centre: CentreCulturelAPI
}>()
</script>

<template>
  <div
    class="group relative h-60 rounded-2xl overflow-hidden shadow-sm hover:shadow-xl transition-all duration-300 hover:-translate-y-1 cursor-pointer"
    data-aos="fade-up"
    data-aos-duration="700"
  >
    <!-- Image de couverture -->
    <img
      v-if="centre.image_couverture_url"
      class="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
      :src="centre.image_couverture_url"
      :alt="centre.nom"
    />
    <div
      v-else
      class="absolute inset-0 bg-linear-to-br from-custom-chocolat to-custom-green flex items-center justify-center"
    >
      <font-awesome-icon class="text-white/40 text-5xl" :icon="['fas', 'building']" />
    </div>

    <!-- Dégradé pour la lisibilité du texte -->
    <div class="absolute inset-0 bg-linear-to-t from-black/85 via-black/30 to-transparent"></div>

    <!-- Badge nombre d'événements -->
    <div
      class="absolute top-3 right-3 inline-flex items-center gap-1.5 rounded-full bg-white/90 backdrop-blur-sm px-3 py-1 text-sm font-semibold text-custom-green shadow-sm"
    >
      <font-awesome-icon :icon="['fas', 'calendar-days']" />
      {{ centre.nombre_programmations }}
    </div>

    <!-- Contenu -->
    <div class="absolute inset-x-0 bottom-0 p-5">
      <h3 class="text-white text-2xl font-bold leading-tight line-clamp-2">
        {{ centre.nom }}
      </h3>
      <p
        v-if="centre.description"
        class="mt-1.5 text-white/75 text-sm leading-snug line-clamp-2"
      >
        {{ centre.description }}
      </p>
      <span
        class="mt-3 inline-flex items-center gap-1.5 text-white/90 text-sm font-medium opacity-0 -translate-x-2 transition-all duration-300 group-hover:opacity-100 group-hover:translate-x-0"
      >
        Découvrir
        <font-awesome-icon :icon="['fas', 'arrow-right']" />
      </span>
    </div>
  </div>
</template>

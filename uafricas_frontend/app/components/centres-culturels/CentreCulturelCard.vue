<script setup lang="ts">
import type { CentreCulturel } from '~/mocks/centres-culturels'

defineProps<{
  centre: CentreCulturel
}>()
</script>

<template>
  <div
    class="flex bg-white rounded-xl mt-2 hover:shadow-lg transition-all duration-300 hover:scale-[1.01] cursor-pointer"
    data-aos="fade-right"
    data-aos-duration="800"
  >
    <img
      class="w-32 h-28 rounded-xl p-1 object-cover"
      :src="centre.urlBanniere"
      :alt="centre.nom"
    />
    <div class="ml-3 py-2">
      <div class="text-3xl text-custom-chocolat font-bold">
        <span class="text-black">{{ centre.nom }}</span>
      </div>
      <div class="flex items-center mt-1">
        <font-awesome-icon class="text-gray-500" :icon="['fas', 'location-dot']" />
        <div class="ml-2 text-gray-600">{{ centre.adress }}</div>
      </div>
      <div
        class="border mt-2 inline-flex rounded-xl px-3 py-0.5 text-custom-green border-custom-green text-sm"
      >
        Evénements:
        <span class="text-black font-bold ml-1">{{ centre.programmations.length }}</span>
      </div>
    </div>
  </div>
</template>

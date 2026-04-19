<script setup lang="ts">
import type { ProgrammationAPI } from '~/composables/useCentresCulturels'
import { formatDateFrancais, formatHeureFrancais, getModeLabel } from '~/composables/useCentresCulturels'

defineProps<{
  programmation: ProgrammationAPI
  siteId: string
}>()

const cardColors = ['bg-orange-300', 'bg-red-300', 'bg-purple-300', 'bg-blue-300', 'bg-green-300']

const getRandomColor = (id: string) => {
  const index = id.charCodeAt(id.length - 1) % cardColors.length
  return cardColors[index]
}
</script>

<template>
  <NuxtLink :to="`/centres/${siteId}/programmations/${programmation.id}`">
    <div
      :class="[
        getRandomColor(programmation.id),
        'hover:scale-105 transition-all hover:-rotate-3 cursor-pointer shadow-md m-3 rounded-xl w-72'
      ]"
      data-aos="zoom-in"
      data-aos-duration="600"
    >
      <div class="p-2">
        <div class="rounded-xl h-40 bg-gray-200/50 w-full flex items-center justify-center">
          <font-awesome-icon class="text-gray-500 text-4xl" :icon="['fas', 'calendar-days']" />
        </div>
      </div>
      <div class="rounded-r-full text-white bg-black inline-flex px-4 py-0.5 text-sm">
        {{ formatDateFrancais(programmation.date_heure_debut) }}
      </div>
      <div class="m-3">
        <div class="font-bold text-2xl truncate">
          {{ programmation.titre }}
        </div>
        <div class="flex items-center mt-2">
          <font-awesome-icon class="text-gray-700" :icon="['fas', 'location-dot']" />
          <div class="ml-2 truncate text-gray-700">{{ programmation.lieu || 'Lieu non précisé' }}</div>
        </div>
        <div class="flex items-center mt-1">
          <font-awesome-icon class="text-gray-700" :icon="['far', 'clock']" />
          <div class="ml-2 font-bold text-xl text-red-600">
            {{ formatHeureFrancais(programmation.date_heure_debut) }}
          </div>
        </div>
        <div class="italic text-sm mt-1">
          <span class="text-gray-600">Type:</span>
          {{ getModeLabel(programmation.mode) }}
        </div>
      </div>
    </div>
  </NuxtLink>
</template>

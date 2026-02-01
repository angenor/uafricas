<script setup lang="ts">
import type { MembreEquipe } from '~/mocks/centres-culturels'

defineProps<{
  president: MembreEquipe
  vicePresident: MembreEquipe
  respCommunication: MembreEquipe
}>()
</script>

<template>
  <div class="rounded-xl bg-white w-full md:w-1/2 p-4">
    <div class="flex items-center text-gray-600 border-b-2 pb-2">
      <font-awesome-icon class="h-4" :icon="['fas', 'users']" />
      <div class="text-xl font-extrabold ml-3">Équipe</div>
    </div>

    <div class="mt-3 space-y-2">
      <div class="flex items-start text-sm text-black hover:text-gray-600 transition-colors">
        <span class="w-2.5 h-2.5 bg-black rounded-full mr-2 mt-1.5 flex-shrink-0"></span>
        <div>
          <span class="font-bold">Président: </span>
          <span>{{ president.nom }}</span>
          <span class="text-gray-500 ml-1">({{ president.email }} | {{ president.tel }})</span>
        </div>
      </div>

      <div class="flex items-start text-sm text-black hover:text-gray-600 transition-colors">
        <span class="w-2.5 h-2.5 bg-black rounded-full mr-2 mt-1.5 flex-shrink-0"></span>
        <div>
          <span class="font-bold">Vice président: </span>
          <span>{{ vicePresident.nom }}</span>
          <span class="text-gray-500 ml-1">({{ vicePresident.email }} | {{ vicePresident.tel }})</span>
        </div>
      </div>

      <div class="flex items-start text-sm text-black hover:text-gray-600 transition-colors">
        <span class="w-2.5 h-2.5 bg-black rounded-full mr-2 mt-1.5 flex-shrink-0"></span>
        <div>
          <span class="font-bold">Communication: </span>
          <span>{{ respCommunication.nom }}</span>
          <span class="text-gray-500 ml-1">({{ respCommunication.email }} | {{ respCommunication.tel }})</span>
        </div>
      </div>
    </div>
  </div>
</template>

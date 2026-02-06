<script setup lang="ts">
import type { MembreCentreAPI } from '~/composables/useCentresCulturels'

defineProps<{
  membres: MembreCentreAPI[]
}>()

function formatNomComplet(membre: MembreCentreAPI): string {
  return membre.prenom ? `${membre.prenom} ${membre.nom}` : membre.nom
}

function formatContact(membre: MembreCentreAPI): string {
  const parts = [membre.email]
  if (membre.telephone) parts.push(membre.telephone)
  return parts.join(' | ')
}
</script>

<template>
  <div class="rounded-xl bg-white w-full md:w-1/2 p-4">
    <div class="flex items-center text-gray-600 border-b-2 pb-2">
      <font-awesome-icon class="h-4" :icon="['fas', 'users']" />
      <div class="text-xl font-extrabold ml-3">Équipe</div>
    </div>

    <div class="mt-3 space-y-2">
      <div
        v-for="membre in membres"
        :key="membre.email"
        class="flex items-start text-sm text-black hover:text-gray-600 transition-colors"
      >
        <span class="w-2.5 h-2.5 bg-black rounded-full mr-2 mt-1.5 flex-shrink-0"></span>
        <div>
          <span class="font-bold">{{ membre.role_label }}: </span>
          <span>{{ formatNomComplet(membre) }}</span>
          <span class="text-gray-500 ml-1">({{ formatContact(membre) }})</span>
        </div>
      </div>
    </div>
  </div>
</template>

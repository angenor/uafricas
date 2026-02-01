<template>
  <div class="flex flex-wrap justify-between items-center gap-4">
    <!-- Filtres par année -->
    <div class="flex space-x-2">
      <button
        v-for="annee in ANNEES"
        :key="annee"
        @click="emit('update:anneeSelected', annee)"
        :class="[
          'bg-custom-chocolat rounded-full text-white px-3 py-1 hover:scale-105 transition-all text-sm',
          anneeSelected === annee ? 'ring-2 ring-offset-2 ring-custom-chocolat' : ''
        ]"
      >
        {{ annee }}
      </button>
    </div>

    <!-- Filtres type et pays + bouton proposer -->
    <div class="flex flex-wrap items-center gap-3">
      <select
        :value="filtreType"
        @change="emit('update:filtreType', ($event.target as HTMLSelectElement).value)"
        class="rounded-md border border-custom-chocolat bg-white px-3 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat"
      >
        <option v-for="type in TYPES_EVENEMENT" :key="type.value" :value="type.value">
          {{ type.label }}
        </option>
      </select>

      <select
        :value="filtrePays"
        @change="emit('update:filtrePays', ($event.target as HTMLSelectElement).value)"
        class="rounded-md border border-custom-chocolat bg-white px-3 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat"
      >
        <option value="">Tous les pays</option>
        <option v-for="pays in PAYS_AFRICAINS" :key="pays" :value="pays">
          {{ pays }}
        </option>
      </select>

      <button
        @click="emit('openModal')"
        class="text-white bg-custom-green rounded-md py-1 px-4 hover:bg-custom-green/90 transition-colors flex items-center text-sm"
      >
        <font-awesome-icon icon="fa-solid fa-plus" class="mr-2" />
        Proposer un événement
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ANNEES, TYPES_EVENEMENT, PAYS_AFRICAINS } from '~/mocks/evenements'

defineProps<{
  anneeSelected: string
  filtreType: string
  filtrePays: string
}>()

const emit = defineEmits<{
  'update:anneeSelected': [value: string]
  'update:filtreType': [value: string]
  'update:filtrePays': [value: string]
  'openModal': []
}>()
</script>

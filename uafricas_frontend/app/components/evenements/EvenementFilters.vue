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

    <!-- Filtres type, zone et pays + bouton proposer -->
    <div class="flex flex-wrap items-center gap-3">
      <select
        :value="filtreType"
        @change="emit('update:filtreType', ($event.target as HTMLSelectElement).value)"
        class="rounded-md border border-custom-chocolat bg-white px-3 py-1 text-sm focus:outline-hidden focus:ring-2 focus:ring-custom-chocolat"
      >
        <option v-for="type in TYPES_EVENEMENT" :key="type.value" :value="type.value">
          {{ type.label }}
        </option>
      </select>

      <!-- Zone géographique : Tout / Afrique / Hors Afrique -->
      <div class="inline-flex shrink-0 rounded-md border border-custom-chocolat overflow-hidden text-sm">
        <button
          v-for="option in ZONES_TERRITOIRE"
          :key="option.value"
          type="button"
          @click="emit('update:filtreZone', option.value)"
          :class="[
            'px-3 py-1 whitespace-nowrap transition-colors',
            (filtreZone ?? 'tout') === option.value
              ? 'bg-custom-chocolat text-white'
              : 'bg-white text-custom-chocolat hover:bg-custom-chocolat/10',
          ]"
        >
          {{ option.label }}
        </button>
      </div>

      <!--
        Liste des territoires : masquée uniquement en « Hors Afrique », la seule
        liste disponible étant africaine. En zone « Tout » elle reste offerte,
        non filtrée, pour ne pas priver l'utilisateur du choix d'un territoire.
      -->
      <select
        v-if="(filtreZone ?? 'tout') !== 'hors_afrique'"
        :value="filtrePays"
        @change="emit('update:filtrePays', ($event.target as HTMLSelectElement).value)"
        class="rounded-md border border-custom-chocolat bg-white px-3 py-1 text-sm focus:outline-hidden focus:ring-2 focus:ring-custom-chocolat"
      >
        <option value="">Tous les territoires</option>
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
import { ANNEES, TYPES_EVENEMENT, PAYS_AFRICAINS } from '~/composables/useEvenements'

// Zones géographiques proposées. « Tout » est en première position : c'est le
// choix le plus large, et la valeur par défaut de la page.
const ZONES_TERRITOIRE = [
  { value: 'tout' as const, label: 'Mondial' },
  { value: 'afrique' as const, label: 'Afrique' },
  { value: 'hors_afrique' as const, label: 'Hors Afrique' },
]
type ZoneTerritoire = (typeof ZONES_TERRITOIRE)[number]['value']

defineProps<{
  anneeSelected: string
  filtreType: string
  filtrePays: string
  filtreZone: ZoneTerritoire
}>()

const emit = defineEmits<{
  'update:anneeSelected': [value: string]
  'update:filtreType': [value: string]
  'update:filtrePays': [value: string]
  'update:filtreZone': [value: ZoneTerritoire]
  'openModal': []
}>()
</script>

<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden cursor-pointer"
       @click="$emit('click', formation)">
    <!-- Header avec type -->
    <div class="px-6 py-4 border-b border-gray-100">
      <div class="flex items-center justify-between">
        <span class="px-3 py-1 rounded-full text-sm font-medium"
              :class="getTypeClasses(formation.type)">
          {{ getTypeLabel(formation.type) }}
        </span>
        <div class="flex items-center">
          <span class="w-2 h-2 rounded-full mr-2"
                :class="getStatutColor(formation.statut)"></span>
          <span class="text-gray-600 text-sm">{{ getStatutLabel(formation.statut) }}</span>
        </div>
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="p-6">
      <h3 class="text-xl font-bold text-gray-900 mb-2">{{ formation.titre }}</h3>
      <p class="text-gray-600 mb-4 line-clamp-2">{{ formation.description }}</p>

      <!-- Formateur -->
      <div class="flex items-center mb-4">
        <img v-if="formation.formateur.photo_url"
             :src="formation.formateur.photo_url"
             :alt="`${formation.formateur.prenom} ${formation.formateur.nom}`"
             class="w-10 h-10 rounded-full mr-3 object-cover">
        <div v-else class="w-10 h-10 rounded-full mr-3 bg-gray-300 flex items-center justify-center">
          <svg class="w-6 h-6 text-gray-600" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
          </svg>
        </div>
        <div>
          <p class="font-medium text-sm">{{ formation.formateur.prenom }} {{ formation.formateur.nom }}</p>
          <p class="text-xs text-gray-500">Formateur</p>
        </div>
      </div>

      <!-- Informations cles -->
      <div class="grid grid-cols-2 gap-4 mb-4">
        <div>
          <p class="text-xs text-gray-500 mb-1">Début</p>
          <p class="text-sm font-medium">{{ formatDateCourt(formation.date_heure_debut) }}</p>
        </div>
        <div>
          <p class="text-xs text-gray-500 mb-1">Langue</p>
          <p class="text-sm font-medium">{{ formation.langue }}</p>
        </div>
      </div>

      <!-- Capacite -->
      <div v-if="formation.nombre_places" class="flex items-center justify-between text-sm mb-4">
        <span class="text-gray-500">Places</span>
        <span class="text-gray-600">{{ formation.nombre_inscrits }}/{{ formation.nombre_places }} inscrits</span>
      </div>
    </div>

    <!-- Action footer -->
    <div class="px-6 py-3 bg-gray-50 border-t border-gray-100">
      <button @click.stop="$emit('inscrire', formation)"
              :disabled="!peutSInscrire(formation)"
              class="w-full py-2 rounded-md font-medium transition"
              :class="peutSInscrire(formation)
                ? 'bg-blue-600 text-white hover:bg-blue-700'
                : 'bg-gray-300 text-gray-500 cursor-not-allowed'">
        {{ getActionLabel(formation) }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  type FormationAPI,
  getTypeLabel,
  getTypeClasses,
  getStatutLabel,
  getStatutColor,
  getActionLabel,
  peutSInscrire,
  formatDateCourt,
} from '~/composables/useFormations'

defineProps<{
  formation: FormationAPI
}>()

defineEmits<{
  (e: 'click', formation: FormationAPI): void
  (e: 'inscrire', formation: FormationAPI): void
}>()
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>

<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden cursor-pointer"
       @click="$emit('click', faculte)">
    <!-- Image de couverture ou placeholder -->
    <div class="h-48 bg-gradient-to-br from-blue-500 to-indigo-600 relative overflow-hidden">
      <img v-if="faculte.imageCouverture"
           :src="faculte.imageCouverture"
           :alt="faculte.titre"
           class="w-full h-full object-cover">
      <div v-else class="w-full h-full flex items-center justify-center">
        <svg class="w-24 h-24 text-white opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 14l9-5-9-5-9 5 9 5z"></path>
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 14v7"></path>
        </svg>
      </div>

      <!-- Logo de la faculté -->
      <div v-if="faculte.logo" class="absolute bottom-4 right-4 w-16 h-16 bg-white rounded-full p-2 shadow-lg">
        <img :src="faculte.logo" :alt="faculte.acronyme" class="w-full h-full object-contain">
      </div>
    </div>

    <!-- Contenu -->
    <div class="p-6">
      <div class="flex items-start justify-between mb-3">
        <div>
          <h3 class="text-xl font-bold text-gray-900">{{ faculte.titre }}</h3>
          <p class="text-sm text-gray-600 mt-1">{{ faculte.acronyme }}</p>
        </div>
        <span v-if="faculte.accepteNouveauxInscrits"
              class="px-2 py-1 bg-green-100 text-green-800 text-xs rounded-full font-medium">
          Inscriptions ouvertes
        </span>
      </div>

      <p class="text-gray-600 mb-4 line-clamp-2">{{ faculte.description }}</p>

      <!-- École partenaire -->
      <div class="mb-4">
        <p class="text-sm text-gray-500">École partenaire</p>
        <p class="font-medium">{{ faculte.ecolePartenaire.nom }}</p>
        <p class="text-sm text-gray-600">{{ faculte.ecolePartenaire.ville }}, {{ faculte.ecolePartenaire.pays }}</p>
      </div>

      <!-- Programmes résumé -->
      <div class="mb-4">
        <div class="flex flex-wrap gap-2">
          <span v-if="faculte.programmesResume.licence.length > 0"
                class="px-2 py-1 bg-blue-50 text-blue-700 text-xs rounded">
            {{ faculte.programmesResume.licence.length }} Licences
          </span>
          <span v-if="faculte.programmesResume.master.length > 0"
                class="px-2 py-1 bg-purple-50 text-purple-700 text-xs rounded">
            {{ faculte.programmesResume.master.length }} Masters
          </span>
          <span v-if="faculte.programmesResume.doctorat.length > 0"
                class="px-2 py-1 bg-red-50 text-red-700 text-xs rounded">
            {{ faculte.programmesResume.doctorat.length }} Doctorats
          </span>
          <span v-if="faculte.programmesResume.certificats.length > 0"
                class="px-2 py-1 bg-green-50 text-green-700 text-xs rounded">
            {{ faculte.programmesResume.certificats.length }} Certificats
          </span>
        </div>
      </div>

      <!-- Statistiques -->
      <div class="flex items-center justify-between text-sm text-gray-500">
        <span class="flex items-center">
          <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path>
          </svg>
          {{ faculte.stats?.nombreInscritsTotal || 0 }} intéressés
        </span>
        <button @click.stop="$emit('manifester-interet', faculte)"
                class="text-blue-600 hover:text-blue-800 font-medium">
          Manifester mon intérêt →
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Faculte } from '~/mocks/inuda/facultes'

defineProps<{
  faculte: Faculte
}>()

defineEmits<{
  (e: 'click', faculte: Faculte): void
  (e: 'manifester-interet', faculte: Faculte): void
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

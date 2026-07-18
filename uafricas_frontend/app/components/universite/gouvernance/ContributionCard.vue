<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-lg transition-all duration-300 overflow-hidden">
    <!-- Header avec type -->
    <div class="px-6 py-4 border-b border-gray-100">
      <div class="flex items-center justify-between">
        <span class="px-3 py-1 rounded-full text-sm font-medium"
              :class="getTypeClasses(contribution.type)">
          {{ getTypeLabel(contribution.type) }}
        </span>
        <span v-if="contribution.verified"
              class="flex items-center text-green-600 text-sm">
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M6.267 3.455a3.066 3.066 0 001.745-.723 3.066 3.066 0 013.976 0 3.066 3.066 0 001.745.723 3.066 3.066 0 012.812 2.812c.051.643.304 1.254.723 1.745a3.066 3.066 0 010 3.976 3.066 3.066 0 00-.723 1.745 3.066 3.066 0 01-2.812 2.812 3.066 3.066 0 00-1.745.723 3.066 3.066 0 01-3.976 0 3.066 3.066 0 00-1.745-.723 3.066 3.066 0 01-2.812-2.812 3.066 3.066 0 00-.723-1.745 3.066 3.066 0 010-3.976 3.066 3.066 0 00.723-1.745 3.066 3.066 0 012.812-2.812zm7.44 5.252a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
          </svg>
          Vérifié
        </span>
      </div>
    </div>

    <!-- Contenu -->
    <div class="p-6">
      <h3 class="text-xl font-bold text-gray-900 mb-2">{{ contribution.titre }}</h3>
      <p class="text-gray-600 mb-4 line-clamp-2">{{ contribution.description }}</p>

      <!-- Auteur -->
      <div class="flex items-center mb-4">
        <img v-if="contribution.auteur.photoURL"
             :src="contribution.auteur.photoURL"
             :alt="`${contribution.auteur.prenom} ${contribution.auteur.nom}`"
             class="w-8 h-8 rounded-full mr-2 object-cover">
        <div v-else class="w-8 h-8 rounded-full mr-2 bg-gray-200 flex items-center justify-center">
          <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
          </svg>
        </div>
        <div class="text-sm">
          <span class="font-medium">{{ contribution.auteur.prenom }} {{ contribution.auteur.nom }}</span>
          <span class="text-gray-500 ml-2">{{ formatDate(contribution.dateCreation) }}</span>
        </div>
      </div>

      <!-- Localisation -->
      <div class="flex items-center text-sm text-gray-500 mb-4">
        <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
        </svg>
        <span>{{ contribution.localisation.ville }}, {{ contribution.localisation.pays }}</span>
      </div>

      <!-- Tags -->
      <div v-if="contribution.tags?.length" class="flex flex-wrap gap-1 mb-4">
        <span v-for="tag in contribution.tags.slice(0, 3)" :key="tag"
              class="px-2 py-1 bg-gray-100 text-gray-600 text-xs rounded">
          #{{ tag }}
        </span>
      </div>

      <!-- Stats -->
      <div class="flex items-center justify-between text-sm text-gray-500 pt-4 border-t border-gray-100">
        <div class="flex items-center gap-4">
          <span class="flex items-center">
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
            </svg>
            {{ contribution.stats.vues }}
          </span>
          <span class="flex items-center">
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
            </svg>
            {{ contribution.stats.likes }}
          </span>
          <span class="flex items-center">
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
            </svg>
            {{ contribution.stats.commentaires }}
          </span>
        </div>
        <span class="text-blue-600 font-medium hover:text-blue-800">
          Voir plus →
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ContributionCitoyenne } from '~/types/gouvernance'

defineProps<{
  contribution: ContributionCitoyenne
}>()

const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    factcheck: 'FactCheck',
    badhabits: 'BadGoodhabits',
    ideaforces: 'IdeaForces'
  }
  return labels[type] || type
}

const getTypeClasses = (type: string) => {
  const classes: Record<string, string> = {
    factcheck: 'bg-blue-100 text-blue-800',
    badhabits: 'bg-red-100 text-red-800',
    ideaforces: 'bg-yellow-100 text-yellow-800'
  }
  return classes[type] || 'bg-gray-100 text-gray-800'
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(new Date(date))
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>

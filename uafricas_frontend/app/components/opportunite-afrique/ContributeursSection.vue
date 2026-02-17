<script setup lang="ts">
import type { ContributeurAPI } from '~/composables/useOpportuniteAfrique'

interface Props {
  contributeurs: ContributeurAPI[]
}

defineProps<Props>()

const getInitiales = (prenom: string, nom: string): string => {
  return `${prenom.charAt(0)}${nom.charAt(0)}`.toUpperCase()
}
</script>

<template>
  <div class="bg-white rounded-lg shadow-md p-6">
    <h3 class="text-lg font-bold mb-4 flex items-center">
      <svg class="w-5 h-5 mr-2 text-custom-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
      </svg>
      Contributeurs
    </h3>

    <div v-if="contributeurs.length === 0" class="text-gray-500 text-sm text-center py-3">
      <svg class="w-8 h-8 text-gray-300 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
      </svg>
      Aucun contributeur pour le moment.<br />
      Soyez le premier a contribuer !
    </div>

    <div v-else class="space-y-3">
      <div
        v-for="c in contributeurs"
        :key="c.utilisateur_id"
        class="flex items-center space-x-3 p-2 rounded-lg hover:bg-gray-50 transition-colors"
      >
        <!-- Avatar -->
        <div class="flex-shrink-0">
          <img
            v-if="c.photo_url"
            :src="c.photo_url"
            :alt="c.prenom + ' ' + c.nom"
            class="w-10 h-10 rounded-full object-cover border-2 border-custom-green/20"
          />
          <div
            v-else
            class="w-10 h-10 rounded-full bg-custom-green/15 flex items-center justify-center text-custom-green font-bold text-sm border-2 border-custom-green/20"
          >
            {{ getInitiales(c.prenom, c.nom) }}
          </div>
        </div>
        <!-- Infos -->
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-gray-900 truncate">
            {{ c.prenom }} {{ c.nom }}
          </p>
          <p class="text-xs text-gray-500">
            {{ c.nombre_contributions }} contribution{{ c.nombre_contributions > 1 ? 's' : '' }} validee{{ c.nombre_contributions > 1 ? 's' : '' }}
          </p>
        </div>
        <!-- Badge -->
        <div class="flex-shrink-0">
          <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-custom-green/10 text-custom-green">
            {{ c.nombre_contributions }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

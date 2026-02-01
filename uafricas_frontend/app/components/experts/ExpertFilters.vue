<template>
  <div class="bg-white rounded-2xl shadow-xl p-6 sticky top-24">
    <!-- Profile Filters -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 mb-6 flex items-center gap-2">
        <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m3 5.197H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        Situation Professionnelle
      </h3>
      <div class="space-y-3">
        <button
          v-for="profile in profiles"
          :key="profile.id"
          :class="[
            'w-full flex items-center gap-3 p-4 rounded-xl text-sm font-medium transition-all duration-200 hover:shadow-md',
            `profile-btn-${profile.id}`,
            {
              'active shadow-lg':
                selectedProfile === profile.id || (profile.id === 'tous' && !selectedProfile),
              'bg-gray-50 text-gray-700 hover:bg-gray-100':
                selectedProfile !== profile.id && !(profile.id === 'tous' && !selectedProfile),
            },
          ]"
          @click="$emit('filterProfile', profile.id)"
        >
          <i :class="profile.icon" class="w-5 h-5" />
          <span class="flex-1 text-left">{{ profile.label }}</span>
          <span
            v-if="selectedProfile === profile.id || (profile.id === 'tous' && !selectedProfile)"
            class="w-3 h-3 bg-white rounded-full shadow-sm"
          />
        </button>
      </div>
    </div>

    <!-- Quick Stats -->
    <div class="mb-6 pt-6 border-t border-gray-200">
      <h4 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4">Statistiques</h4>
      <div class="space-y-3">
        <div class="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
          <span class="text-sm text-gray-600">Total experts</span>
          <span class="font-bold text-gray-900 text-lg">{{ totalExperts }}</span>
        </div>
        <div class="flex justify-between items-center p-3 bg-emerald-50 rounded-lg">
          <span class="text-sm text-emerald-700">Resultats filtres</span>
          <span class="font-bold text-emerald-600 text-lg">{{ filteredCount }}</span>
        </div>
      </div>
    </div>

    <!-- Reset Filters -->
    <button
      class="w-full flex items-center justify-center gap-2 p-3 bg-gradient-to-r from-gray-100 to-gray-200 text-gray-700 rounded-xl hover:from-gray-200 hover:to-gray-300 transition-all duration-200 font-medium"
      @click="$emit('reset')"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
        />
      </svg>
      Reinitialiser les filtres
    </button>
  </div>
</template>

<script setup lang="ts">
import { profiles } from '~/mocks/experts'

defineProps<{
  selectedProfile: string
  totalExperts: number
  filteredCount: number
}>()

defineEmits<{
  filterProfile: [profileId: string]
  reset: []
}>()
</script>

<style scoped>
/* Profile buttons avec indicateurs colores */
.profile-btn-recherche_emploi.active {
  background-color: #ef4444;
  color: white;
}

.profile-btn-en_emploi.active {
  background-color: #10b981;
  color: white;
}

.profile-btn-consultance.active {
  background-color: #3b82f6;
  color: white;
}

.profile-btn-volontariat_expertise.active {
  background-color: #8b5cf6;
  color: white;
}

.profile-btn-recherche_nouvelles_opportunites.active {
  background-color: #f97316;
  color: white;
}

.profile-btn-tous.active {
  background-color: #6b7280;
  color: white;
}

/* Hover states */
.profile-btn-recherche_emploi:not(.active):hover {
  background-color: #fef2f2;
  color: #b91c1c;
}

.profile-btn-en_emploi:not(.active):hover {
  background-color: #ecfdf5;
  color: #047857;
}

.profile-btn-consultance:not(.active):hover {
  background-color: #eff6ff;
  color: #1d4ed8;
}

.profile-btn-volontariat_expertise:not(.active):hover {
  background-color: #f5f3ff;
  color: #7c3aed;
}

.profile-btn-recherche_nouvelles_opportunites:not(.active):hover {
  background-color: #fff7ed;
  color: #ea580c;
}

.profile-btn-tous:not(.active):hover {
  background-color: #f9fafb;
  color: #374151;
}
</style>

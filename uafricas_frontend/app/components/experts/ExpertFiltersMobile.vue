<template>
  <div
    :class="{
      'translate-x-0': isOpen,
      '-translate-x-full': !isOpen,
    }"
    class="fixed top-0 left-0 w-80 h-full bg-white shadow-2xl z-50 transform transition-transform duration-300 ease-in-out lg:hidden overflow-y-auto sidebar-content"
  >
    <!-- Sidebar Header -->
    <div class="p-6 border-b border-gray-200">
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-bold text-gray-900">Filtres</h2>
        <button class="p-2 rounded-lg hover:bg-gray-100 transition-colors" @click="$emit('close')">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>
    </div>

    <!-- Profile Filters Mobile -->
    <div class="p-6 border-b border-gray-200">
      <h3 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4">
        Situation Professionnelle
      </h3>
      <div class="space-y-2">
        <button
          v-for="profile in profiles"
          :key="profile.id"
          :class="[
            'w-full flex items-center gap-3 p-3 rounded-xl text-sm font-medium transition-all',
            `profile-btn-${profile.id}`,
            {
              active:
                selectedProfile === profile.id || (profile.id === 'tous' && !selectedProfile),
              'bg-gray-50 text-gray-700':
                selectedProfile !== profile.id && !(profile.id === 'tous' && !selectedProfile),
            },
          ]"
          @click="$emit('filterProfile', profile.id)"
        >
          <i :class="profile.icon" class="w-4 h-4" />
          <span class="flex-1 text-left">{{ profile.label }}</span>
          <span
            v-if="selectedProfile === profile.id || (profile.id === 'tous' && !selectedProfile)"
            class="w-2 h-2 bg-white rounded-full"
          />
        </button>
      </div>
    </div>

    <!-- Quick Stats Mobile -->
    <div class="p-6 border-b border-gray-200">
      <h3 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4">Statistiques</h3>
      <div class="space-y-3">
        <div class="flex justify-between items-center">
          <span class="text-sm text-gray-600">Total experts</span>
          <span class="font-semibold text-gray-900">{{ totalExperts }}</span>
        </div>
        <div class="flex justify-between items-center">
          <span class="text-sm text-gray-600">Resultats filtres</span>
          <span class="font-semibold text-emerald-600">{{ filteredCount }}</span>
        </div>
      </div>
    </div>

    <!-- Reset Filters Mobile -->
    <div class="p-6">
      <button
        class="w-full flex items-center justify-center gap-2 p-3 bg-gray-100 text-gray-700 rounded-xl hover:bg-gray-200 transition-colors"
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
  </div>
</template>

<script setup lang="ts">
import { profiles } from '~/mocks/experts'

defineProps<{
  isOpen: boolean
  selectedProfile: string
  totalExperts: number
  filteredCount: number
}>()

defineEmits<{
  close: []
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

/* Scrollbar styling for sidebar */
.sidebar-content::-webkit-scrollbar {
  width: 6px;
}

.sidebar-content::-webkit-scrollbar-track {
  background: #f1f5f9;
}

.sidebar-content::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 3px;
}

.sidebar-content::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}
</style>

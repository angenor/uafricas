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

    <!-- Filtre Territoire Mobile -->
    <div class="p-6 border-b border-gray-200">
      <h3 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4">Territoire</h3>

      <!-- Choix de la zone (radio) -->
      <div class="grid grid-cols-2 gap-2 mb-3">
        <label
          v-for="option in zones"
          :key="option.value"
          :class="[
            'flex items-center justify-center gap-2 px-3 py-2.5 rounded-xl text-sm font-medium cursor-pointer transition-all',
            zone === option.value
              ? 'bg-gradient-to-r from-emerald-500 to-teal-500 text-white shadow-md'
              : 'bg-gray-50 text-gray-700 hover:bg-gray-100',
          ]"
        >
          <input
            v-model="zone"
            type="radio"
            :value="option.value"
            class="sr-only"
          >
          {{ option.label }}
        </label>
      </div>

      <!-- Menu déroulant des territoires -->
      <select
        v-model="selectedCountry"
        class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl text-sm text-gray-700 focus:outline-hidden focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-all"
      >
        <option value="">
          {{ zone === 'afrique' ? 'Tous les territoires d\'Afrique' : 'Tous les territoires hors Afrique' }}
        </option>
        <option v-for="territoire in territoires" :key="territoire" :value="territoire">
          {{ territoire }}
        </option>
      </select>
    </div>

    <!-- Filtre Spécialité Mobile -->
    <div class="p-6 border-b border-gray-200">
      <h3 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4">Spécialité</h3>

      <!-- Menu déroulant des spécialités -->
      <select
        v-model="selectedSpecialty"
        class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl text-sm text-gray-700 focus:outline-hidden focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-all"
      >
        <option value="">
          Toutes les spécialités
        </option>
        <option v-for="specialite in specialites" :key="specialite" :value="specialite">
          {{ specialite }}
        </option>
      </select>
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
import {
  PROFILS_PROFESSIONNELS as profiles,
  PAYS_AFRIQUE,
  PAYS_HORS_AFRIQUE,
} from '~/composables/useExperts'

defineProps<{
  isOpen: boolean
  selectedProfile: string
  /** Spécialités réellement déclarées par les experts (source : API). */
  specialites: string[]
}>()

defineEmits<{
  close: []
  filterProfile: [profileId: string]
  reset: []
}>()

/** Territoire sélectionné (synchronisé avec la page via v-model). */
const selectedCountry = defineModel<string>('selectedCountry', { default: '' })

/** Spécialité sélectionnée (synchronisée avec la page via v-model). */
const selectedSpecialty = defineModel<string>('selectedSpecialty', { default: '' })

/** Zone géographique qui pilote le contenu du menu déroulant. */
const zones = [
  { value: 'afrique' as const, label: 'Afrique' },
  { value: 'hors_afrique' as const, label: 'Hors Afrique' },
]
const zone = ref<'afrique' | 'hors_afrique'>('afrique')

/** Territoires proposés selon la zone, triés alphabétiquement (fr). */
const territoires = computed(() =>
  (zone.value === 'afrique' ? PAYS_AFRIQUE : PAYS_HORS_AFRIQUE)
    .slice()
    .sort((a, b) => a.localeCompare(b, 'fr')),
)

// Changer de zone réinitialise le territoire choisi (contenus disjoints).
watch(zone, () => {
  selectedCountry.value = ''
})
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

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
            class="w-3 h-3 bg-white rounded-full shadow-xs"
          />
        </button>
      </div>
    </div>

    <!-- Filtre Territoire -->
    <div class="mb-6 pt-6 border-t border-gray-200">
      <h4 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4 flex items-center gap-2">
        <svg class="w-4 h-4 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        Territoire
      </h4>

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

    <!-- Filtre Spécialité -->
    <div class="mb-6 pt-6 border-t border-gray-200">
      <h4 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4 flex items-center gap-2">
        <svg class="w-4 h-4 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"
          />
        </svg>
        Spécialité
      </h4>

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
import {
  PROFILS_PROFESSIONNELS as profiles,
  PAYS_AFRIQUE,
  PAYS_HORS_AFRIQUE,
} from '~/composables/useExperts'

defineProps<{
  selectedProfile: string
  /** Spécialités réellement déclarées par les experts (source : API). */
  specialites: string[]
}>()

defineEmits<{
  filterProfile: [profileId: string]
  reset: []
}>()

/** Territoire sélectionné (synchronisé avec la page via v-model). */
const selectedCountry = defineModel<string>('selectedCountry', { default: '' })

/** Spécialité sélectionnée (synchronisée avec la page via v-model). */
const selectedSpecialty = defineModel<string>('selectedSpecialty', { default: '' })

/**
 * Zone géographique (synchronisée avec la page via v-model) : elle pilote le
 * contenu du menu déroulant des territoires ET filtre la liste des experts.
 */
const zone = defineModel<'afrique' | 'hors_afrique'>('zone', { default: 'afrique' })

const zones = [
  { value: 'afrique' as const, label: 'Afrique' },
  { value: 'hors_afrique' as const, label: 'Hors Afrique' },
]

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

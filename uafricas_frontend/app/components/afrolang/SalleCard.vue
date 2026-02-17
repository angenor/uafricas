<template>
  <div class="relative bg-white rounded-2xl overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-300 group">
    <!-- Image avec overlay -->
    <div class="relative h-48 overflow-hidden">
      <img
        class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
        :src="salle.image_couverture_url || '/images/investissement-afrique.jpg'"
        :alt="salle.titre"
      />

      <!-- Gradient overlay -->
      <div class="absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent" />

      <!-- Badge langue -->
      <div v-if="salle.langue_cible" class="absolute top-4 left-4">
        <span class="bg-white/90 backdrop-blur-xs px-3 py-1 rounded-full text-xs font-medium text-gray-700 flex items-center gap-1.5">
          <font-awesome-icon :icon="['fas', 'language']" class="w-3 h-3 text-blue-500" />
          {{ salle.langue_cible }}
        </span>
      </div>

      <!-- Indicateur en direct -->
      <div v-if="salle.sessions_en_cours > 0" class="absolute top-4 right-4">
        <span class="bg-red-500 text-white px-3 py-1 rounded-full text-xs font-medium flex items-center gap-1.5 animate-pulse">
          <font-awesome-icon :icon="['fas', 'circle']" class="w-2 h-2" />
          {{ salle.sessions_en_cours }} en direct
        </span>
      </div>

      <!-- Titre en bas de l'image -->
      <div class="absolute bottom-0 left-0 right-0 p-4 text-white">
        <h3 class="text-lg font-bold line-clamp-2">{{ salle.titre }}</h3>
      </div>
    </div>

    <!-- Contenu de la carte -->
    <div class="p-5">
      <!-- Description courte -->
      <p class="text-gray-600 text-sm line-clamp-2 mb-4">
        {{ salle.description || 'Salle de visioconférence pour l\'apprentissage linguistique' }}
      </p>

      <!-- Informations principales -->
      <div class="space-y-2 mb-4">
        <div class="flex items-center gap-2 text-sm text-gray-700">
          <font-awesome-icon :icon="['fas', 'door-open']" class="w-4 h-4 text-blue-500" />
          <span>{{ salle.nombre_salles_privees }} cours privé{{ salle.nombre_salles_privees > 1 ? 's' : '' }}</span>
        </div>
        <div class="flex items-center gap-2 text-sm text-gray-600">
          <font-awesome-icon :icon="['fas', 'video']" class="w-4 h-4 text-gray-400" />
          <span>{{ salle.sessions_en_cours }} session{{ salle.sessions_en_cours > 1 ? 's' : '' }} active{{ salle.sessions_en_cours > 1 ? 's' : '' }}</span>
        </div>
      </div>

      <!-- Boutons d'action -->
      <div class="flex flex-col gap-2 pt-4 border-t border-gray-100">
        <!-- Bouton principal : démarrer / rejoindre la visioconférence -->
        <button
          class="w-full px-4 py-3 text-white text-sm rounded-lg font-semibold hover:shadow-lg transition-all flex items-center justify-center gap-2 disabled:opacity-60 disabled:cursor-wait"
          :class="salle.sessions_en_cours > 0
            ? 'bg-gradient-to-r from-emerald-500 to-teal-500'
            : 'bg-gradient-to-r from-blue-500 to-cyan-500'"
          :disabled="chargement"
          @click="$emit('entrer', salle.id)"
        >
          <font-awesome-icon
            :icon="['fas', chargement ? 'spinner' : (salle.sessions_en_cours > 0 ? 'video' : 'right-to-bracket')]"
            class="w-4 h-4"
            :class="{ 'animate-spin': chargement }"
          />
          <template v-if="chargement">
            Connexion en cours...
          </template>
          <template v-else>
            {{ salle.sessions_en_cours > 0 ? 'Rejoindre la visioconférence' : 'Démarrer la visioconférence' }}
          </template>
        </button>

        <!-- Bouton voir cours privés (toggle) -->
        <button
          class="w-full px-4 py-2.5 text-sm rounded-lg font-medium transition-all flex items-center justify-center gap-2"
          :class="expanded
            ? 'bg-blue-50 text-blue-700 border border-blue-200'
            : 'bg-gray-50 text-gray-700 border border-gray-200 hover:bg-gray-100'"
          @click="$emit('toggle-privees', salle.id)"
        >
          <font-awesome-icon :icon="['fas', 'door-open']" class="w-4 h-4" />
          Cours privés ({{ salle.nombre_salles_privees }})
          <font-awesome-icon
            :icon="['fas', expanded ? 'chevron-up' : 'chevron-down']"
            class="w-3 h-3 ml-auto"
          />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SalleAPI } from '~/composables/useAfrolang'

defineProps<{
  salle: SalleAPI
  expanded: boolean
  chargement: boolean
}>()

defineEmits<{
  'entrer': [salleId: string]
  'toggle-privees': [salleId: string]
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

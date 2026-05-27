<template>
  <div class="relative bg-white rounded-xl overflow-hidden shadow-md hover:shadow-lg transition-all duration-300 group">
    <!-- Image avec overlay -->
    <div class="relative h-32 overflow-hidden">
      <img
        class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
        :src="salle.image_couverture_url || '/images/investissement-afrique.jpg'"
        :alt="salle.titre"
      />

      <!-- Gradient overlay -->
      <div class="absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent" />

      <!-- Badge langue -->
      <div v-if="salle.langue_cible" class="absolute top-3 left-3">
        <span class="bg-white/90 backdrop-blur-xs px-2 py-0.5 rounded-full text-[11px] font-medium text-gray-700 flex items-center gap-1">
          <font-awesome-icon :icon="['fas', 'language']" class="w-3 h-3 text-blue-500" />
          {{ salle.langue_cible }}
        </span>
      </div>

      <!-- Indicateur en direct -->
      <div v-if="salle.sessions_en_cours > 0" class="absolute top-3 right-3">
        <span class="bg-red-500 text-white px-2 py-0.5 rounded-full text-[11px] font-medium flex items-center gap-1 animate-pulse">
          <font-awesome-icon :icon="['fas', 'circle']" class="w-1.5 h-1.5" />
          {{ salle.sessions_en_cours }} en direct
        </span>
      </div>

      <!-- Titre en bas de l'image -->
      <div class="absolute bottom-0 left-0 right-0 px-3 pb-2 text-white">
        <h3 class="text-sm font-bold line-clamp-2 leading-tight">{{ salle.titre }}</h3>
      </div>

      <!-- Badge désactivation administrative (feature 001-ressources-fermeture-session) -->
      <div
        v-if="salle.desactivee_admin"
        class="absolute top-2 right-2"
      >
        <AfrolangSalleDesactiveeBadge :desactivation="salle.desactivee_admin" compact />
      </div>
    </div>

    <!-- Contenu de la carte -->
    <div class="p-3">
      <!-- Description courte -->
      <p class="text-gray-600 text-xs line-clamp-2 mb-3">
        {{ salle.description || 'Salle de visioconférence pour l\'apprentissage linguistique' }}
      </p>

      <!-- Pays d'origine (feature 001-afrolang-pays-origine) -->
      <div
        v-if="paysAffiches.length > 0"
        class="mb-3"
        :aria-label="`Territoire d'origine : ${tooltipPays}`"
      >
        <div class="text-[10px] uppercase tracking-wide text-gray-400 mb-1">
          Territoire d'origine
        </div>
        <div
          v-if="modeCompact"
          class="flex items-center flex-wrap gap-1"
          :title="tooltipPays"
        >
          <span
            v-for="p in paysAffiches"
            :key="p.id"
            class="text-base leading-none"
            :title="p.nom"
          >{{ drapeauEmoji(p.code_iso2) || '·' }}</span>
        </div>
        <div v-else class="flex items-center flex-wrap gap-1">
          <span
            v-for="p in paysAffiches"
            :key="p.id"
            class="inline-flex items-center gap-1 bg-gray-50 border border-gray-200 px-1.5 py-0.5 rounded text-[11px] text-gray-700"
          >
            <span class="leading-none">{{ drapeauEmoji(p.code_iso2) }}</span>
            <span class="truncate max-w-[8rem]">{{ p.nom }}</span>
          </span>
        </div>
      </div>

      <!-- Informations principales -->
      <div class="flex items-center gap-4 text-xs text-gray-500 mb-3">
        <span class="flex items-center gap-1">
          <font-awesome-icon :icon="['fas', 'door-open']" class="w-3 h-3 text-blue-500" />
          {{ salle.nombre_salles_privees }} cours
        </span>
        <span class="flex items-center gap-1">
          <font-awesome-icon :icon="['fas', 'video']" class="w-3 h-3 text-gray-400" />
          {{ salle.sessions_en_cours }} active{{ salle.sessions_en_cours > 1 ? 's' : '' }}
        </span>
      </div>

      <!-- Administrateurs de la salle (feature 001-admin-salles-publiques, US3) -->
      <div v-if="(salle.administrateurs?.length ?? 0) > 0" class="mb-3">
        <AfrolangSalleAdministrateursWidget :administrateurs="salle.administrateurs" taille="sm" />
      </div>

      <!-- Boutons d'action -->
      <div class="flex flex-col gap-1.5 pt-3 border-t border-gray-100">
        <!-- Bandeau désactivation administrative (feature 001-ressources-fermeture-session) -->
        <div
          v-if="salle.desactivee_admin"
          class="text-xs text-red-700 bg-red-50 border-2 border-red-700/30 rounded-md px-2 py-1.5 flex items-center gap-1.5"
        >
          <font-awesome-icon :icon="['fas', 'ban']" class="w-3 h-3" />
          <span>Salle désactivée par l'administration</span>
        </div>
        <!-- Bouton principal : démarrer / rejoindre la visioconférence -->
        <button
          class="w-full px-3 py-2 text-white text-xs rounded-lg font-semibold hover:shadow-md transition-all flex items-center justify-center gap-1.5 disabled:opacity-60 disabled:cursor-wait"
          :class="salle.desactivee_admin
            ? 'bg-gray-400 cursor-not-allowed'
            : salle.sessions_en_cours > 0
              ? 'bg-gradient-to-r from-emerald-500 to-teal-500'
              : 'bg-gradient-to-r from-blue-500 to-cyan-500'"
          :disabled="chargement || !!salle.desactivee_admin"
          @click="!salle.desactivee_admin && $emit('entrer', salle.id)"
        >
          <font-awesome-icon
            :icon="['fas', chargement ? 'spinner' : (salle.sessions_en_cours > 0 ? 'video' : 'right-to-bracket')]"
            class="w-3.5 h-3.5"
            :class="{ 'animate-spin': chargement }"
          />
          <template v-if="chargement">
            Connexion...
          </template>
          <template v-else>
            {{ salle.sessions_en_cours > 0 ? 'Rejoindre' : 'Démarrer' }}
          </template>
        </button>

        <!-- Bouton voir cours privés (toggle) -->
        <button
          class="w-full px-3 py-1.5 text-xs rounded-lg font-medium transition-all flex items-center justify-center gap-1.5"
          :class="expanded
            ? 'bg-blue-50 text-blue-700 border border-blue-200'
            : 'bg-gray-50 text-gray-700 border border-gray-200 hover:bg-gray-100'"
          @click="$emit('toggle-privees', salle.id)"
        >
          <font-awesome-icon :icon="['fas', 'door-open']" class="w-3 h-3" />
          Canal privé ({{ salle.nombre_salles_privees }})
          <font-awesome-icon
            :icon="['fas', expanded ? 'chevron-up' : 'chevron-down']"
            class="w-2.5 h-2.5 ml-auto"
          />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { SalleAPI } from '~/composables/useAfrolang'

const props = defineProps<{
  salle: SalleAPI
  expanded: boolean
  chargement: boolean
}>()

defineEmits<{
  'entrer': [salleId: string]
  'toggle-privees': [salleId: string]
}>()

const paysAffiches = computed(() => props.salle.pays_origine ?? [])
const modeCompact = computed(() => paysAffiches.value.length >= 4)
const tooltipPays = computed(() => paysAffiches.value.map(p => p.nom).join(', '))

function drapeauEmoji(codeIso2: string | null): string {
  if (!codeIso2 || codeIso2.length !== 2) return ''
  const code = codeIso2.toUpperCase()
  const c0 = code.charCodeAt(0)
  const c1 = code.charCodeAt(1)
  if (c0 < 65 || c0 > 90 || c1 < 65 || c1 > 90) return ''
  return String.fromCodePoint(0x1F1E6 + c0 - 65, 0x1F1E6 + c1 - 65)
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

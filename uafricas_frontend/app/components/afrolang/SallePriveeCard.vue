<script setup lang="ts">
import { computed } from 'vue'
import { getInitiales, type SallePriveeAPI } from '~/composables/useAfrolang'

const props = defineProps<{
  sallePrivee: SallePriveeAPI
  /** Indique qu'une action (rejoindre / ouvrir) est en cours sur cette carte. */
  chargement?: boolean
}>()

const emit = defineEmits<{
  /** Rejoindre (non-auteur) : ouvrir le modale de saisie du code. */
  (e: 'rejoindre', sallePriveeId: string): void
  /** Ouvrir (auteur) : court-circuit code, accès direct. */
  (e: 'ouvrir', sallePriveeId: string): void
  /** Modifier le code secret (auteur). */
  (e: 'modifier-code', sallePriveeId: string): void
  /** Supprimer la salle (auteur). */
  (e: 'archiver', sallePriveeId: string): void
}>()

const initiales = computed(() =>
  getInitiales(
    null,
    props.sallePrivee.auteur_nom,
  ),
)

const handleAction = () => {
  if (props.sallePrivee.est_auteur) {
    emit('ouvrir', props.sallePrivee.id)
  }
  else {
    emit('rejoindre', props.sallePrivee.id)
  }
}

const handleModifierCode = () => {
  emit('modifier-code', props.sallePrivee.id)
}

const handleArchiver = () => {
  emit('archiver', props.sallePrivee.id)
}
</script>

<template>
  <div class="relative bg-white rounded-xl border border-gray-100 p-4 hover:shadow-md transition-all">
    <!-- Bandeau supérieur : badges -->
    <div class="flex items-start justify-between gap-2 mb-2">
      <h5 class="font-medium text-gray-800 text-sm line-clamp-1 flex-1">
        {{ sallePrivee.titre }}
      </h5>

      <div class="flex items-center gap-1.5 shrink-0">
        <!-- En direct -->
        <span
          v-if="sallePrivee.session_en_cours"
          class="inline-flex items-center gap-1 text-[11px] text-red-600 font-medium"
        >
          <span class="w-1.5 h-1.5 bg-red-500 rounded-full animate-pulse" />
          En direct
        </span>

        <!-- Cadenas -->
        <span class="text-amber-500" title="Protégée par un code secret">
          <font-awesome-icon :icon="['fas', 'lock']" class="w-3 h-3" />
        </span>

        <!-- Actions auteur : icônes directes (modifier le code / supprimer) -->
        <template v-if="sallePrivee.est_auteur">
          <button
            type="button"
            class="p-1 text-gray-400 hover:text-gray-700 hover:bg-gray-100 rounded"
            aria-label="Modifier le code secret"
            title="Modifier le code secret"
            @click="handleModifierCode"
          >
            <font-awesome-icon :icon="['fas', 'key']" class="w-3 h-3" />
          </button>
          <button
            type="button"
            class="p-1 text-gray-400 hover:text-red-600 hover:bg-red-50 rounded"
            aria-label="Supprimer ma salle privée"
            title="Supprimer ma salle privée"
            @click="handleArchiver"
          >
            <font-awesome-icon :icon="['fas', 'trash']" class="w-3 h-3" />
          </button>
        </template>
      </div>
    </div>

    <!-- Description -->
    <p
      v-if="sallePrivee.description"
      class="text-xs text-gray-500 line-clamp-2 mb-3"
    >
      {{ sallePrivee.description }}
    </p>

    <!-- Auteur -->
    <div class="flex items-center gap-2 text-xs text-gray-400 mb-3">
      <div class="w-5 h-5 rounded-full bg-custom-chocolat/10 text-custom-chocolat flex items-center justify-center text-[10px] font-semibold">
        {{ initiales }}
      </div>
      <span class="truncate">
        {{ sallePrivee.est_auteur ? 'Vous' : (sallePrivee.auteur_nom || 'Auteur inconnu') }}
      </span>
    </div>

    <!-- Action principale -->
    <button
      type="button"
      :disabled="chargement"
      class="w-full px-3 py-2 text-xs rounded-lg font-semibold transition-all flex items-center justify-center gap-1.5 disabled:opacity-60 disabled:cursor-wait"
      :class="sallePrivee.est_auteur
        ? 'bg-custom-chocolat text-white hover:bg-custom-chocolat/90'
        : 'bg-amber-500 text-white hover:bg-amber-600'"
      @click="handleAction"
    >
      <font-awesome-icon
        :icon="['fas', chargement ? 'spinner' : (sallePrivee.est_auteur ? 'door-open' : 'key')]"
        class="w-3 h-3"
        :class="{ 'animate-spin': chargement }"
      />
      <template v-if="chargement">
        Connexion...
      </template>
      <template v-else-if="sallePrivee.est_auteur">
        Ouvrir ma salle privée
      </template>
      <template v-else>
        Rejoindre
      </template>
    </button>
  </div>
</template>

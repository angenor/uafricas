<template>
  <div class="relative">
    <!-- Ligne verticale -->
    <div class="absolute left-4 top-0 bottom-0 w-0.5 bg-gray-200" />

    <!-- Sessions -->
    <div class="space-y-6">
      <div
        v-for="session in sessions"
        :key="session.id"
        class="relative pl-10"
      >
        <!-- Point sur la timeline -->
        <div
          class="absolute left-2.5 w-3 h-3 rounded-full border-2 border-white"
          :class="dotClass(session.etat)"
        />

        <AfrolangSessionCard :session="session" />
      </div>
    </div>

    <!-- Vide -->
    <div v-if="sessions.length === 0" class="text-center py-8 text-gray-500">
      <font-awesome-icon :icon="['fas', 'video']" class="w-8 h-8 text-gray-300 mb-3" />
      <p>Aucune session pour le moment</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { EtatSession, SessionAPI } from '~/composables/useAfrolang'

defineProps<{
  sessions: SessionAPI[]
}>()

const dotClass = (etat: EtatSession): string => {
  switch (etat) {
    case 'en_cours': return 'bg-emerald-500'
    case 'planifiee': return 'bg-blue-500'
    case 'terminee': return 'bg-gray-400'
    case 'annulee': return 'bg-red-400'
    default: return 'bg-gray-300'
  }
}
</script>

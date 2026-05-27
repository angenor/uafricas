<template>
  <!-- Panneau de modération (organisateur uniquement). Demandes de parole →
       promouvoir ; participants présents → rétrograder / retirer. Tailwind v4 pur. -->
  <aside class="flex w-full max-w-xs flex-col border-l border-gray-700 bg-gray-800/95">
    <div class="flex items-center justify-between border-b border-gray-700 px-4 py-2">
      <span class="flex items-center gap-2 text-sm font-semibold text-gray-100">
        <font-awesome-icon :icon="['fas', 'shield-halved']" class="h-4 w-4 text-amber-400" />
        Modération
      </span>
      <button type="button" class="text-gray-400 hover:text-white" aria-label="Fermer" @click="emit('fermer')">
        <font-awesome-icon :icon="['fas', 'xmark']" class="h-4 w-4" />
      </button>
    </div>

    <div class="flex-1 overflow-y-auto px-4 py-3">
      <!-- Demandes de parole -->
      <div class="mb-5">
        <h4 class="mb-2 flex items-center gap-2 text-xs font-bold uppercase tracking-wide text-amber-300">
          <font-awesome-icon :icon="['fas', 'hand']" class="h-3.5 w-3.5" />
          Demandes de parole
          <span v-if="demandes.length" class="rounded-full bg-amber-500 px-1.5 text-[10px] text-white">{{ demandes.length }}</span>
        </h4>
        <p v-if="demandes.length === 0" class="text-xs text-gray-500">Aucune demande pour le moment.</p>
        <ul class="space-y-2">
          <li
            v-for="d in demandes"
            :key="d.utilisateur_id"
            class="flex items-center justify-between gap-2 rounded-lg bg-gray-700/60 px-3 py-2"
          >
            <span class="min-w-0 truncate text-sm text-gray-100">{{ d.nom }}</span>
            <button
              type="button"
              class="shrink-0 rounded-md bg-custom-green px-2.5 py-1 text-xs font-semibold text-white transition hover:brightness-110"
              @click="emit('promouvoir', d.utilisateur_id)"
            >
              Promouvoir
            </button>
          </li>
        </ul>
      </div>

      <!-- Participants présents -->
      <div>
        <h4 class="mb-2 flex items-center gap-2 text-xs font-bold uppercase tracking-wide text-gray-300">
          <font-awesome-icon :icon="['fas', 'users']" class="h-3.5 w-3.5" />
          Participants
        </h4>
        <p v-if="participants.length === 0" class="text-xs text-gray-500">Aucun autre participant.</p>
        <ul class="space-y-2">
          <li
            v-for="p in participants"
            :key="p.identity"
            class="flex items-center justify-between gap-2 rounded-lg bg-gray-700/40 px-3 py-2"
          >
            <span class="flex min-w-0 flex-col">
              <span class="truncate text-sm text-gray-100">{{ p.nom }}</span>
              <span class="text-[10px] uppercase tracking-wide" :class="p.role === 'intervenant' ? 'text-custom-green' : 'text-gray-400'">
                {{ p.role === 'intervenant' ? 'Intervenant' : 'Spectateur' }}
              </span>
            </span>
            <div class="flex shrink-0 items-center gap-1">
              <button
                v-if="p.role === 'intervenant'"
                type="button"
                class="rounded-md bg-gray-600 px-2 py-1 text-xs font-medium text-white transition hover:bg-gray-500"
                title="Rétrograder en spectateur"
                @click="emit('retrograder', p.identity)"
              >
                Rétrograder
              </button>
              <button
                type="button"
                class="rounded-md bg-red-600/80 px-2 py-1 text-xs font-medium text-white transition hover:bg-red-600"
                title="Retirer du direct"
                @click="emit('retirer', p.identity)"
              >
                Retirer
              </button>
            </div>
          </li>
        </ul>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import type { DemandeParole } from '~/composables/useEvenements'

export interface ParticipantModeration {
  identity: string
  nom: string
  role: 'organisateur' | 'intervenant' | 'spectateur'
}

defineProps<{
  demandes: DemandeParole[]
  participants: ParticipantModeration[]
}>()

const emit = defineEmits<{
  promouvoir: [uid: string]
  retrograder: [uid: string]
  retirer: [uid: string]
  fermer: []
}>()
</script>

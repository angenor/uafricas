<template>
  <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-xl">
    <!-- Avatar -->
    <div
      v-if="participant.photo_url"
      class="w-10 h-10 rounded-full overflow-hidden flex-shrink-0"
    >
      <img
        :src="participant.photo_url"
        :alt="participant.prenom || ''"
        class="w-full h-full object-cover"
      />
    </div>
    <div
      v-else
      class="w-10 h-10 rounded-full flex-shrink-0 flex items-center justify-center text-sm font-semibold text-white"
      :class="roleBgClass"
    >
      {{ initiales }}
    </div>

    <!-- Info -->
    <div class="min-w-0 flex-1">
      <div class="flex items-center gap-2">
        <span class="text-sm font-medium text-gray-900 truncate">
          {{ participant.prenom }} {{ participant.nom }}
        </span>
        <span
          class="px-2 py-0.5 rounded-full text-[10px] font-semibold uppercase tracking-wider"
          :class="roleBadgeClass"
        >
          {{ roleLabel }}
        </span>
      </div>
      <div class="text-xs text-gray-500">
        <span v-if="participant.quitte_at">
          {{ dureeFormatee }}
        </span>
        <span v-else class="text-emerald-600 flex items-center gap-1">
          <font-awesome-icon :icon="['fas', 'circle']" class="w-1.5 h-1.5" />
          Connecté
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getInitiales, formatDuree, type ParticipantAPI } from '~/composables/useAfrolang'

const props = defineProps<{
  participant: ParticipantAPI
}>()

const initiales = computed(() => getInitiales(props.participant.nom, props.participant.prenom))

const dureeFormatee = computed(() => formatDuree(props.participant.duree_secondes))

const roleLabel = computed(() => {
  switch (props.participant.role_session) {
    case 'moderateur': return 'Modérateur'
    case 'participant': return 'Participant'
    case 'observateur': return 'Observateur'
    default: return props.participant.role_session
  }
})

const roleBgClass = computed(() => {
  switch (props.participant.role_session) {
    case 'moderateur': return 'bg-blue-500'
    case 'participant': return 'bg-emerald-500'
    case 'observateur': return 'bg-gray-400'
    default: return 'bg-gray-400'
  }
})

const roleBadgeClass = computed(() => {
  switch (props.participant.role_session) {
    case 'moderateur': return 'bg-blue-100 text-blue-700'
    case 'participant': return 'bg-emerald-100 text-emerald-700'
    case 'observateur': return 'bg-gray-100 text-gray-600'
    default: return 'bg-gray-100 text-gray-600'
  }
})
</script>

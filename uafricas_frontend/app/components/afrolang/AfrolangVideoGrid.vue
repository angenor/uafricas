<template>
  <div class="flex-1 p-2 sm:p-4 overflow-hidden flex flex-col gap-2 sm:gap-4">
    <!-- Partage d'écran proéminent -->
    <div v-if="ecranPartageActif" class="flex-1 min-h-0">
      <AfrolangParticipantTile
        :participant="ecranPartageActif"
        :is-dominant="false"
        :is-screen-share="true"
      />
    </div>

    <!-- Grille des participants -->
    <div
      :class="[
        ecranPartageActif ? 'h-32 sm:h-40 shrink-0 flex gap-2 overflow-x-auto' : gridClass,
      ]"
    >
      <AfrolangParticipantTile
        v-for="participant in participants"
        :key="participant.identity"
        :participant="participant"
        :is-dominant="participant.identity === dominantSpeaker"
        :class="ecranPartageActif ? 'w-32 sm:w-40 shrink-0 rounded-lg' : ''"
      />
    </div>

    <!-- Empty state -->
    <div
      v-if="participants.length === 0"
      class="flex-1 flex items-center justify-center text-gray-500"
    >
      <div class="text-center">
        <font-awesome-icon :icon="['fas', 'video']" class="w-12 h-12 text-gray-600 mb-3" />
        <p class="text-lg">En attente de participants...</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { RoomParticipant } from './AfrolangRoom.vue'

const props = defineProps<{
  participants: RoomParticipant[]
  dominantSpeaker: string | null
}>()

// Trouver le participant qui partage son écran
const ecranPartageActif = computed<RoomParticipant | null>(() => {
  return props.participants.find(p => p.screenTrack !== null) ?? null
})

const gridClass = computed(() => {
  const count = props.participants.length
  if (count === 0) return 'flex'
  if (count === 1) return 'grid grid-cols-1'
  if (count === 2) return 'grid grid-cols-2 gap-2 sm:gap-4'
  if (count <= 4) return 'grid grid-cols-2 grid-rows-2 gap-2 sm:gap-4'
  if (count <= 6) return 'grid grid-cols-3 grid-rows-2 gap-2 sm:gap-3'
  if (count <= 9) return 'grid grid-cols-3 grid-rows-3 gap-2 sm:gap-3'
  return 'grid grid-cols-4 auto-rows-fr gap-2'
})
</script>

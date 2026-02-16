<template>
  <div
    class="flex-1 p-2 sm:p-4 overflow-hidden"
    :class="gridClass"
  >
    <AfrolangParticipantTile
      v-for="participant in participants"
      :key="participant.identity"
      :participant="participant"
      :is-dominant="participant.identity === dominantSpeaker"
    />

    <!-- Empty state -->
    <div
      v-if="participants.length === 0"
      class="flex items-center justify-center h-full text-gray-500"
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

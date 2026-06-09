<template>
  <div class="flex-1 p-2 sm:p-4 overflow-hidden flex flex-col gap-2 sm:gap-4">
    <!-- Partage d'écran proéminent + pellicule des participants (pour qu'ils
         restent visibles ; l'audio, lui, est rendu indépendamment dans AfrolangRoom). -->
    <template v-if="ecranPartageActif">
      <div class="flex-1 min-h-0">
        <AfrolangParticipantTile
          :participant="ecranPartageActif"
          :is-dominant="false"
          :is-screen-share="true"
        />
      </div>
      <div class="h-24 sm:h-32 shrink-0 flex gap-2 overflow-x-auto">
        <AfrolangParticipantTile
          v-for="participant in participants"
          :key="participant.identity"
          :participant="participant"
          :is-dominant="participant.identity === dominantSpeaker"
          class="w-28 sm:w-36 shrink-0 rounded-lg"
        />
      </div>
    </template>

    <!-- Spotlight (FR-023) : participant mis en évidence agrandi au centre. -->
    <template v-else-if="participantSpotlight">
      <div class="relative flex-1 min-h-0 transition-all duration-300 ease-in-out">
        <AfrolangParticipantTile
          :participant="participantSpotlight"
          :is-dominant="true"
          class="border-2 border-custom-chocolat rounded-lg h-full"
        />
        <span
          class="absolute top-2 left-2 inline-flex items-center gap-1 px-2 py-1 rounded-full bg-custom-chocolat text-white text-[11px] font-semibold tracking-wide"
        >
          <font-awesome-icon :icon="['fas', 'star']" class="w-3 h-3" />
          En vedette
        </span>
      </div>
      <div class="h-24 sm:h-32 shrink-0 flex gap-2 overflow-x-auto transition-all duration-300">
        <AfrolangParticipantTile
          v-for="participant in autresParticipants"
          :key="participant.identity"
          :participant="participant"
          :is-dominant="participant.identity === dominantSpeaker"
          class="w-28 sm:w-36 shrink-0 rounded-lg"
        />
      </div>
    </template>

    <!-- Grille des participants -->
    <div
      v-else
      :class="[gridClass, 'transition-all duration-300 ease-in-out']"
    >
      <AfrolangParticipantTile
        v-for="participant in participants"
        :key="participant.identity"
        :participant="participant"
        :is-dominant="participant.identity === dominantSpeaker"
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

// Feature 001-session-moderation : mise en évidence (spotlight)
const { spotlightActif } = useAfrolang()

const participantSpotlight = computed<RoomParticipant | null>(() => {
  const id = spotlightActif.value?.utilisateur_id
  if (!id) return null
  return props.participants.find(p => p.identity === id) ?? null
})

const autresParticipants = computed<RoomParticipant[]>(() => {
  const id = spotlightActif.value?.utilisateur_id
  if (!id) return props.participants
  return props.participants.filter(p => p.identity !== id)
})

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

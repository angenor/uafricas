<template>
  <div class="w-72 bg-gray-800 border-l border-gray-700 flex flex-col h-full">
    <!-- Header -->
    <div class="px-4 py-3 border-b border-gray-700">
      <h3 class="text-sm font-semibold text-white flex items-center gap-2">
        <font-awesome-icon :icon="['fas', 'users']" class="w-4 h-4 text-blue-400" />
        Participants ({{ participants.length }})
      </h3>
    </div>

    <!-- Liste des participants -->
    <div class="flex-1 overflow-y-auto p-3 space-y-1">
      <div
        v-for="participant in sortedParticipants"
        :key="participant.identity"
        class="flex items-center gap-2.5 px-3 py-2 rounded-lg transition-colors"
        :class="participant.identity === dominantSpeaker ? 'bg-emerald-500/10' : 'hover:bg-gray-700/50'"
      >
        <!-- Avatar -->
        <div class="relative shrink-0">
          <div
            class="w-8 h-8 rounded-full flex items-center justify-center text-xs font-semibold"
            :class="participant.isLocal
              ? 'bg-gradient-to-br from-blue-500 to-cyan-500 text-white'
              : 'bg-gradient-to-br from-gray-600 to-gray-700 text-gray-300'"
          >
            {{ getInitialesParticipant(participant.name) }}
          </div>
          <!-- Speaking indicator -->
          <span
            v-if="participant.isSpeaking && !participant.isMuted"
            class="absolute -bottom-0.5 -right-0.5 w-3 h-3 bg-emerald-400 rounded-full border-2 border-gray-800 animate-pulse"
          />
        </div>

        <!-- Nom + badges -->
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-1.5">
            <span class="text-sm text-white truncate">{{ participant.name }}</span>
            <span v-if="participant.isLocal" class="text-[10px] text-blue-400">(vous)</span>
          </div>
          <div class="flex items-center gap-1 mt-0.5">
            <span
              v-if="participant.isModerator"
              class="text-[10px] px-1.5 py-0.5 rounded bg-blue-500/20 text-blue-300 font-medium"
            >
              Modérateur
            </span>
          </div>
        </div>

        <!-- Status icons -->
        <div class="flex items-center gap-1 shrink-0">
          <font-awesome-icon
            :icon="['fas', participant.isMuted ? 'volume-mute' : 'volume-up']"
            class="w-3 h-3"
            :class="participant.isMuted ? 'text-red-400' : 'text-gray-500'"
          />
          <font-awesome-icon
            :icon="['fas', 'video']"
            class="w-3 h-3"
            :class="participant.isCameraOff ? 'text-red-400' : 'text-gray-500'"
          />
        </div>
      </div>

      <!-- Empty state -->
      <div
        v-if="participants.length === 0"
        class="text-center py-8 text-gray-500"
      >
        <p class="text-sm">Aucun participant</p>
      </div>
    </div>

    <!-- Footer info -->
    <div class="px-4 py-2 border-t border-gray-700 text-xs text-gray-500 text-center">
      Session {{ sessionId.substring(0, 8) }}...
    </div>
  </div>
</template>

<script setup lang="ts">
import type { RoomParticipant } from './AfrolangRoom.vue'

const props = defineProps<{
  participants: RoomParticipant[]
  sessionId: string
  dominantSpeaker: string | null
}>()

const getInitialesParticipant = (name: string): string => {
  const parts = name.trim().split(/\s+/)
  if (parts.length >= 2 && parts[0] && parts[1]) {
    return ((parts[0][0] || '') + (parts[1][0] || '')).toUpperCase()
  }
  return name.substring(0, 2).toUpperCase()
}

// Trier : moderateur en premier, puis local, puis alphabetique
const sortedParticipants = computed(() => {
  return [...props.participants].sort((a, b) => {
    if (a.isModerator && !b.isModerator) return -1
    if (!a.isModerator && b.isModerator) return 1
    if (a.isLocal && !b.isLocal) return -1
    if (!a.isLocal && b.isLocal) return 1
    return a.name.localeCompare(b.name)
  })
})
</script>

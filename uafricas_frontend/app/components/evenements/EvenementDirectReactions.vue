<template>
  <!-- Réactions emoji éphémères (DataPackets LiveKit). Picker + overlay flottant
       (réutilise le pattern AfrolangReactionsOverlay). Tailwind v4 pur. -->
  <div class="relative">
    <!-- Picker -->
    <div class="flex items-center gap-1 rounded-full bg-gray-700 px-2 py-1">
      <button
        v-for="emoji in EMOJIS"
        :key="emoji"
        type="button"
        :disabled="!connected"
        class="flex h-9 w-9 items-center justify-center rounded-full text-xl transition hover:bg-gray-600 disabled:opacity-40"
        :aria-label="`Envoyer la réaction ${emoji}`"
        @click="envoyer(emoji)"
      >
        {{ emoji }}
      </button>
    </div>

    <!-- Overlay flottant (téléporté en plein écran) -->
    <Teleport to="body">
      <div class="pointer-events-none fixed inset-0 z-[9999] overflow-hidden" aria-hidden="true">
        <div
          v-for="r in reactions"
          :key="r.id"
          class="reaction-float absolute bottom-24 select-none"
          :style="{ left: r.leftPct + '%', animationDuration: r.durationMs + 'ms', '--sway': r.swayPx + 'px' }"
        >
          <div class="flex flex-col items-center gap-1">
            <span class="text-5xl drop-shadow-lg sm:text-6xl">{{ r.emoji }}</span>
            <span
              v-if="r.nom"
              class="rounded-full bg-black/60 px-2 py-0.5 text-xs font-medium text-white backdrop-blur-sm"
            >{{ r.nom }}</span>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { RoomEvent, type Room, type RemoteParticipant } from 'livekit-client'

const props = defineProps<{
  room: Room | null
  nom: string
  connected: boolean
}>()

const EMOJIS = ['👏', '❤️', '😂', '🎉', '🔥', '👍'] as const

interface ActiveReaction {
  id: number
  emoji: string
  nom: string | null
  leftPct: number
  durationMs: number
  swayPx: number
}

const reactions = ref<ActiveReaction[]>([])
let compteur = 0
const encoder = new TextEncoder()
const decoder = new TextDecoder()

/** Joue une réaction en overlay (s'envole puis disparaît). */
const jouer = (emoji: string, nom: string | null = null): void => {
  const id = compteur++
  const durationMs = 2600 + Math.random() * 900
  reactions.value.push({
    id,
    emoji,
    nom,
    leftPct: 8 + Math.random() * 84,
    durationMs,
    swayPx: (Math.random() - 0.5) * 80,
  })
  setTimeout(() => {
    reactions.value = reactions.value.filter(r => r.id !== id)
  }, durationMs)
}

const envoyer = (emoji: string): void => {
  if (!props.room) return
  const payload = JSON.stringify({ type: 'reaction', emoji, identite: props.room.localParticipant.identity, ts: Date.now() })
  try {
    props.room.localParticipant.publishData(encoder.encode(payload), { reliable: false })
  }
  catch (e) {
    console.error('Erreur envoi réaction:', e)
  }
  jouer(emoji, props.nom) // joue localement (pas d'écho LiveKit)
}

const onData = (payload: Uint8Array, participant?: RemoteParticipant): void => {
  try {
    if (participant?.identity && props.room?.localParticipant?.identity === participant.identity) return
    const data = JSON.parse(decoder.decode(payload)) as { type?: string, emoji?: string }
    if (data?.type !== 'reaction' || typeof data.emoji !== 'string') return
    jouer(data.emoji, participant?.name ?? null)
  }
  catch {
    /* trame ignorée */
  }
}

watch(
  () => props.room,
  (room, ancien) => {
    if (ancien) ancien.off(RoomEvent.DataReceived, onData)
    if (room) room.on(RoomEvent.DataReceived, onData)
  },
  { immediate: true },
)

onBeforeUnmount(() => {
  props.room?.off(RoomEvent.DataReceived, onData)
})
</script>

<style scoped>
.reaction-float {
  animation-name: reaction-rise;
  animation-timing-function: ease-out;
  animation-fill-mode: forwards;
}
@keyframes reaction-rise {
  0% { transform: translateY(0) translateX(0); opacity: 0; }
  10% { opacity: 1; }
  100% { transform: translateY(-70vh) translateX(var(--sway, 0)); opacity: 0; }
}
</style>

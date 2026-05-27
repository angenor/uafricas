<template>
  <!-- Chat texte éphémère du direct (DataPackets LiveKit reliable, aucun stockage).
       Rendu échappé par Vue (anti-XSS, Principe IV). Tailwind v4 pur. -->
  <aside class="flex w-full max-w-sm flex-col border-l border-gray-700 bg-gray-850 bg-gray-800/95">
    <div class="flex items-center justify-between border-b border-gray-700 px-4 py-2">
      <span class="flex items-center gap-2 text-sm font-semibold text-gray-100">
        <font-awesome-icon :icon="['fas', 'comments']" class="h-4 w-4 text-custom-green" />
        Chat en direct
      </span>
      <button
        type="button"
        class="text-gray-400 hover:text-white"
        aria-label="Fermer le chat"
        @click="emit('fermer')"
      >
        <font-awesome-icon :icon="['fas', 'xmark']" class="h-4 w-4" />
      </button>
    </div>

    <div ref="filRef" class="flex-1 space-y-2 overflow-y-auto px-4 py-3">
      <p v-if="messages.length === 0" class="text-center text-xs text-gray-500">
        Aucun message pour le moment. Lancez la conversation !
      </p>
      <div v-for="m in messages" :key="m.id" class="text-sm">
        <span class="font-semibold" :class="m.mien ? 'text-custom-green' : 'text-blue-300'">{{ m.nom }}</span>
        <span class="text-gray-500"> · </span>
        <span class="break-words text-gray-100">{{ m.message }}</span>
      </div>
    </div>

    <form class="flex items-center gap-2 border-t border-gray-700 p-3" @submit.prevent="envoyer">
      <input
        v-model="texte"
        type="text"
        maxlength="500"
        placeholder="Votre message…"
        class="min-w-0 flex-1 rounded-full bg-gray-700 px-4 py-2 text-sm text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-custom-green"
      />
      <button
        type="submit"
        :disabled="!texte.trim()"
        class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-custom-green text-white transition hover:brightness-110 disabled:opacity-40"
        aria-label="Envoyer"
      >
        <font-awesome-icon :icon="['fas', 'paper-plane']" class="h-4 w-4" />
      </button>
    </form>
  </aside>
</template>

<script setup lang="ts">
import { RoomEvent, type Room, type RemoteParticipant } from 'livekit-client'

interface ChatMessage {
  id: number
  nom: string
  message: string
  identite: string
  mien: boolean
}

const props = defineProps<{
  room: Room | null
  identite: string
  nom: string
}>()

const emit = defineEmits<{ fermer: [] }>()

const messages = ref<ChatMessage[]>([])
const texte = ref('')
const filRef = ref<HTMLElement | null>(null)
let compteur = 0
const encoder = new TextEncoder()
const decoder = new TextDecoder()

const defilerEnBas = async (): Promise<void> => {
  await nextTick()
  if (filRef.value) filRef.value.scrollTop = filRef.value.scrollHeight
}

const ajouter = (nom: string, message: string, identite: string, mien: boolean): void => {
  messages.value.push({ id: compteur++, nom, message, identite, mien })
  void defilerEnBas()
}

const envoyer = (): void => {
  const message = texte.value.trim()
  if (!message || !props.room) return
  const payload = JSON.stringify({
    type: 'chat',
    message,
    identite: props.identite,
    nom: props.nom,
    ts: Date.now(),
  })
  try {
    // LiveKit ne renvoie pas ses propres paquets → on affiche localement à l'envoi.
    props.room.localParticipant.publishData(encoder.encode(payload), { reliable: true })
    ajouter(props.nom, message, props.identite, true)
    texte.value = ''
  }
  catch (e) {
    console.error('Erreur envoi chat:', e)
  }
}

const onData = (payload: Uint8Array, participant?: RemoteParticipant): void => {
  try {
    // Garde anti-écho (LiveKit ne renvoie pas ses propres paquets, mais par sécurité).
    if (participant?.identity && props.room?.localParticipant?.identity === participant.identity) return
    const data = JSON.parse(decoder.decode(payload)) as {
      type?: string, message?: string, identite?: string, nom?: string
    }
    if (data?.type !== 'chat' || typeof data.message !== 'string') return
    ajouter(data.nom || 'Invité', data.message, data.identite || '', false)
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

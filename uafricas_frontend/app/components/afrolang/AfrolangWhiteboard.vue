<template>
  <div class="flex flex-col h-full bg-white rounded-lg overflow-hidden">
    <!-- Barre d'outils -->
    <div class="flex items-center gap-2 p-2 bg-base-200 border-b">
      <font-awesome-icon :icon="['fas', 'chalkboard-user']" class="w-4 h-4 text-primary" />
      <span class="font-semibold text-sm">Tableau blanc</span>
      <div class="flex-1" />
      <button
        v-if="estModerateur"
        class="btn btn-xs btn-ghost text-error"
        @click="effacerTout"
      >
        <font-awesome-icon :icon="['fas', 'trash']" class="w-3 h-3" />
        Effacer tout
      </button>
      <button class="btn btn-xs btn-ghost" @click="$emit('fermer')">
        <font-awesome-icon :icon="['fas', 'xmark']" class="w-3 h-3" />
      </button>
    </div>

    <!-- Zone de dessin tldraw (iframe) -->
    <iframe
      ref="whiteboardFrame"
      src="/whiteboard/index.html"
      class="flex-1 w-full border-0"
      allow="clipboard-read; clipboard-write"
    />
  </div>
</template>

<script setup lang="ts">
/**
 * Bridge de communication :
 * 1. iframe tldraw <-> Ce composant Vue (postMessage)
 * 2. Ce composant Vue <-> Autres participants (DataChannel LiveKit)
 * 3. Ce composant Vue <-> Backend (API REST)
 */
import type { Room } from 'livekit-client'

const props = defineProps<{
  sessionId: string
  estModerateur: boolean
  room: Room | null
}>()

defineEmits<{
  fermer: []
}>()

const { obtenirTableauBlanc, sauvegarderTableauBlanc, effacerTableauBlanc } = useAfrolang()

const whiteboardFrame = ref<HTMLIFrameElement | null>(null)
let snapshotInterval: ReturnType<typeof setInterval> | null = null

// ── Recevoir operations des AUTRES participants via DataChannel LiveKit ──
function handleDataReceived(data: Uint8Array) {
  try {
    const message = JSON.parse(new TextDecoder().decode(data))
    if (message.type === 'whiteboard') {
      whiteboardFrame.value?.contentWindow?.postMessage(
        { type: 'apply-operation', payload: message.payload },
        '*',
      )
    }
  }
  catch {
    // Ignorer les messages non-JSON (autres data channels)
  }
}

// ── Recevoir operations LOCALES depuis l'iframe tldraw ──
function handleTldrawMessage(event: MessageEvent) {
  if (event.data?.type === 'tldraw-operation') {
    // Diffuser aux autres via DataChannel LiveKit
    if (props.room?.localParticipant) {
      const encoder = new TextEncoder()
      props.room.localParticipant.publishData(
        encoder.encode(JSON.stringify({
          type: 'whiteboard',
          payload: event.data.payload,
        })),
        { reliable: true },
      )
    }
  }
  else if (event.data?.type === 'tldraw-snapshot') {
    // Sauvegarder le snapshot recu
    sauvegarderTableauBlanc(props.sessionId, event.data.payload)
  }
}

// ── Charger snapshot existant depuis le backend ──
async function chargerSnapshot() {
  const { donnees, version } = await obtenirTableauBlanc(props.sessionId)
  if (version > 0 && Object.keys(donnees).length > 0) {
    // Attendre que l'iframe soit chargee
    await new Promise<void>((resolve) => {
      const frame = whiteboardFrame.value
      if (!frame) { resolve(); return }
      if (frame.contentDocument?.readyState === 'complete') {
        // Petit delai pour que tldraw s'initialise
        setTimeout(resolve, 500)
      }
      else {
        frame.addEventListener('load', () => setTimeout(resolve, 500), { once: true })
      }
    })
    whiteboardFrame.value?.contentWindow?.postMessage(
      { type: 'load-snapshot', snapshot: donnees },
      '*',
    )
  }
}

// ── Effacer tout (moderateur) ──
async function effacerTout() {
  whiteboardFrame.value?.contentWindow?.postMessage({ type: 'clear' }, '*')
  await effacerTableauBlanc(props.sessionId)
}

onMounted(() => {
  window.addEventListener('message', handleTldrawMessage)

  // Ecouter les DataChannels LiveKit
  if (props.room) {
    props.room.on('dataReceived', handleDataReceived)
  }

  // Charger le snapshot existant
  chargerSnapshot()

  // Snapshot periodique (moderateur uniquement, toutes les 30s)
  if (props.estModerateur) {
    snapshotInterval = setInterval(() => {
      whiteboardFrame.value?.contentWindow?.postMessage({ type: 'get-snapshot' }, '*')
    }, 30_000)
  }
})

onUnmounted(() => {
  window.removeEventListener('message', handleTldrawMessage)

  if (props.room) {
    props.room.off('dataReceived', handleDataReceived)
  }

  // Snapshot final avant de quitter
  if (props.estModerateur) {
    whiteboardFrame.value?.contentWindow?.postMessage({ type: 'get-snapshot' }, '*')
  }

  if (snapshotInterval) clearInterval(snapshotInterval)
})
</script>

<template>
  <div class="relative flex flex-col h-full bg-white rounded-lg overflow-hidden">
    <div class="flex items-center gap-2 p-2 bg-gray-100 border-b border-gray-200">
      <font-awesome-icon :icon="['fas', 'chalkboard-user']" class="w-4 h-4 text-custom-chocolat" />
      <span class="font-semibold text-sm text-gray-800">Tableau blanc</span>
      <div class="flex-1" />
      <button
        v-if="estModerateur"
        type="button"
        class="inline-flex items-center gap-1 px-2 py-1 rounded text-xs text-red-600 hover:bg-red-50 focus:outline-none focus:ring-2 focus:ring-red-300 transition"
        @click="effacerTout"
      >
        <font-awesome-icon :icon="['fas', 'trash']" class="w-3 h-3" />
        Effacer tout
      </button>
      <button
        type="button"
        class="inline-flex items-center justify-center w-6 h-6 rounded text-gray-600 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-300 transition"
        aria-label="Fermer le tableau blanc"
        @click="$emit('fermer')"
      >
        <font-awesome-icon :icon="['fas', 'xmark']" class="w-3 h-3" />
      </button>
    </div>

    <iframe
      ref="whiteboardFrame"
      src="/whiteboard/index.html"
      class="flex-1 w-full border-0"
      allow="clipboard-read; clipboard-write"
    />

    <Transition
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0 translate-y-2"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition ease-in duration-150"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="toastMessage"
        class="absolute bottom-4 left-1/2 -translate-x-1/2 z-50 bg-red-600 text-white text-sm px-4 py-2 rounded-lg shadow-lg max-w-md text-center"
        role="status"
        aria-live="polite"
      >
        {{ toastMessage }}
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
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
const toastMessage = ref<string>('')
let snapshotInterval: ReturnType<typeof setInterval> | null = null
let toastTimer: ReturnType<typeof setTimeout> | null = null

const encoder = new TextEncoder()
const decoder = new TextDecoder()

interface MessagePayloadOperation {
  elements: unknown[]
  appState?: Record<string, unknown>
}

interface WireMessage {
  type: 'whiteboard'
  payload: MessagePayloadOperation | { __clear: true }
}

function envoyerAIframe(message: Record<string, unknown>): void {
  const fenetre = whiteboardFrame.value?.contentWindow
  if (!fenetre) return
  fenetre.postMessage(message, '*')
}

function roomEstConnectee(): boolean {
  const r = props.room
  if (!r) return false
  const etat = (r as unknown as { state?: string }).state
  return etat === 'connected'
}

function publierSurRoom(payload: WireMessage['payload']): void {
  if (!roomEstConnectee()) return
  const wire: WireMessage = { type: 'whiteboard', payload }
  try {
    props.room?.localParticipant.publishData(
      encoder.encode(JSON.stringify(wire)),
      { reliable: false } as unknown as never,
    )
  }
  catch (e: unknown) {
    console.error('Erreur publishData whiteboard:', e)
  }
}

function afficherToast(message: string): void {
  toastMessage.value = message
  if (toastTimer) clearTimeout(toastTimer)
  toastTimer = setTimeout(() => { toastMessage.value = '' }, 4000)
}

async function handleIframeMessage(event: MessageEvent): Promise<void> {
  if (event.source !== whiteboardFrame.value?.contentWindow) return
  const data = event.data as { type?: string; payload?: unknown } | null
  if (!data || typeof data !== 'object' || typeof data.type !== 'string') return

  switch (data.type) {
    case 'excalidraw-ready': {
      await chargerSnapshotDepuisBackend()
      break
    }

    case 'excalidraw-operation': {
      const payload = data.payload as MessagePayloadOperation | undefined
      if (!payload || !Array.isArray(payload.elements)) return
      publierSurRoom(payload)
      break
    }

    case 'excalidraw-snapshot': {
      const payload = data.payload as {
        elements: unknown[]
        appState?: Record<string, unknown>
        files?: Record<string, unknown>
      } | undefined
      if (!payload) return
      await sauvegarderTableauBlanc(props.sessionId, {
        type: 'excalidraw',
        version: 1,
        elements: payload.elements,
        appState: payload.appState ?? {},
        files: payload.files ?? {},
      })
      break
    }

    case 'excalidraw-image-rejected': {
      afficherToast('Image refusée : formats acceptés JPEG/PNG, taille max 2 Mo.')
      break
    }

    default:
      break
  }
}

function handleDataReceived(data: Uint8Array): void {
  let message: WireMessage
  try {
    message = JSON.parse(decoder.decode(data)) as WireMessage
  }
  catch {
    return
  }
  if (!message || message.type !== 'whiteboard' || !message.payload) return

  const payload = message.payload as MessagePayloadOperation & { __clear?: boolean }
  if (payload.__clear === true) {
    envoyerAIframe({ type: 'clear' })
    return
  }
  if (!Array.isArray(payload.elements)) return
  envoyerAIframe({ type: 'apply-operation', payload })
}

async function chargerSnapshotDepuisBackend(): Promise<void> {
  const { donnees } = await obtenirTableauBlanc(props.sessionId)
  envoyerAIframe({ type: 'load-snapshot', snapshot: donnees })
}

async function effacerTout(): Promise<void> {
  envoyerAIframe({ type: 'clear' })
  publierSurRoom({ __clear: true })
  await effacerTableauBlanc(props.sessionId)
}

function declencherSnapshotModerateur(): void {
  if (!props.estModerateur) return
  envoyerAIframe({ type: 'get-snapshot' })
}

function demarrerIntervalleSnapshot(): void {
  if (snapshotInterval !== null) return
  if (!props.estModerateur) return
  snapshotInterval = setInterval(() => {
    if (!roomEstConnectee()) return
    declencherSnapshotModerateur()
  }, 30_000)
}

function arreterIntervalleSnapshot(): void {
  if (snapshotInterval !== null) {
    clearInterval(snapshotInterval)
    snapshotInterval = null
  }
}

watch(
  () => (props.room as unknown as { state?: string } | null)?.state,
  async (nouveauEtat, ancienEtat) => {
    if (nouveauEtat === 'connected' && ancienEtat && ancienEtat !== 'connected') {
      await chargerSnapshotDepuisBackend()
    }
  },
)

function handleBeforeUnload(): void {
  if (props.estModerateur) {
    declencherSnapshotModerateur()
  }
}

onMounted(() => {
  window.addEventListener('message', handleIframeMessage)
  window.addEventListener('beforeunload', handleBeforeUnload)

  if (props.room) {
    props.room.on('dataReceived', handleDataReceived)
  }

  demarrerIntervalleSnapshot()
})

onBeforeUnmount(() => {
  if (props.estModerateur) {
    declencherSnapshotModerateur()
  }
  window.removeEventListener('message', handleIframeMessage)
  window.removeEventListener('beforeunload', handleBeforeUnload)

  if (props.room) {
    props.room.off('dataReceived', handleDataReceived)
  }

  arreterIntervalleSnapshot()
  if (toastTimer) {
    clearTimeout(toastTimer)
    toastTimer = null
  }
})
</script>

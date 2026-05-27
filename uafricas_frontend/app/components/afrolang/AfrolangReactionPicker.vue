<template>
  <div class="relative">
    <button
      ref="boutonRef"
      type="button"
      class="flex h-12 w-12 items-center justify-center rounded-full bg-gray-700 text-white transition-all hover:bg-gray-600 disabled:cursor-not-allowed disabled:opacity-50"
      :class="ouvert ? 'ring-2 ring-amber-400' : ''"
      :disabled="!connected"
      title="Envoyer une réaction"
      aria-label="Envoyer une réaction"
      :aria-expanded="ouvert"
      @click="ouvert = !ouvert"
    >
      <font-awesome-icon :icon="['fas', 'face-smile']" class="h-5 w-5" />
    </button>

    <Transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="translate-y-2 opacity-0"
      enter-to-class="translate-y-0 opacity-100"
      leave-active-class="transition duration-100 ease-in"
      leave-from-class="translate-y-0 opacity-100"
      leave-to-class="translate-y-2 opacity-0"
    >
      <div
        v-if="ouvert"
        ref="popoverRef"
        class="absolute bottom-full left-1/2 z-50 mb-3 flex -translate-x-1/2 items-center gap-1 rounded-full border border-gray-700 bg-gray-800/95 px-3 py-2 shadow-2xl backdrop-blur-sm"
        role="menu"
      >
        <button
          v-for="emoji in EMOJIS"
          :key="emoji"
          type="button"
          class="flex h-10 w-10 items-center justify-center rounded-full text-2xl transition-transform hover:scale-125 hover:bg-gray-700 active:scale-110"
          :title="`Réagir avec ${emoji}`"
          :aria-label="`Réagir avec ${emoji}`"
          @click="envoyer(emoji)"
        >
          {{ emoji }}
        </button>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import type { Room } from 'livekit-client'

const EMOJIS = ['👍', '❤️', '🎉', '👏', '😂', '😮', '😢', '🙌'] as const

interface Props {
  room: Room | null
  identite?: string | null
  connected?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  identite: null,
  connected: true,
})

const emit = defineEmits<{
  envoye: [emoji: string]
}>()

const ouvert = ref(false)
const boutonRef = ref<HTMLButtonElement | null>(null)
const popoverRef = ref<HTMLDivElement | null>(null)

const encoder = new TextEncoder()

const envoyer = (emoji: string): void => {
  ouvert.value = false
  emit('envoye', emoji)

  const room = props.room
  if (!room?.localParticipant) return

  const payload = JSON.stringify({
    type: 'reaction',
    emoji,
    identite: props.identite ?? null,
    ts: Date.now(),
  })

  void room.localParticipant
    .publishData(encoder.encode(payload), { reliable: false })
    .catch((e: unknown) => {
      console.error('Erreur publishData reaction:', e)
    })
}

const onDocumentClick = (e: MouseEvent): void => {
  if (!ouvert.value) return
  const target = e.target as Node
  if (boutonRef.value?.contains(target)) return
  if (popoverRef.value?.contains(target)) return
  ouvert.value = false
}

const onKeydown = (e: KeyboardEvent): void => {
  if (e.key === 'Escape' && ouvert.value) ouvert.value = false
}

onMounted(() => {
  document.addEventListener('click', onDocumentClick)
  document.addEventListener('keydown', onKeydown)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', onDocumentClick)
  document.removeEventListener('keydown', onKeydown)
})
</script>

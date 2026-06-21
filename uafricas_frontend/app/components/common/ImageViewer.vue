<script setup lang="ts">
const props = defineProps<{
  images: string[]
  open: boolean
  index?: number
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

const courant = ref(props.index ?? 0)

watch(() => props.open, (v) => {
  if (v) courant.value = props.index ?? 0
})
watch(() => props.index, (v) => {
  if (typeof v === 'number') courant.value = v
})

const imageCourante = computed(() => props.images[courant.value] ?? props.images[0] ?? '')
const aPlusieurs = computed(() => props.images.length > 1)

function precedent() {
  courant.value = (courant.value - 1 + props.images.length) % props.images.length
}
function suivant() {
  courant.value = (courant.value + 1) % props.images.length
}

function onKey(e: KeyboardEvent) {
  if (!props.open) return
  if (e.key === 'Escape') emit('close')
  else if (e.key === 'ArrowLeft' && aPlusieurs.value) precedent()
  else if (e.key === 'ArrowRight' && aPlusieurs.value) suivant()
}

onMounted(() => window.addEventListener('keydown', onKey))
onBeforeUnmount(() => window.removeEventListener('keydown', onKey))
</script>

<template>
  <Teleport to="body">
    <Transition name="viewer-fade">
      <div
        v-if="open && images.length"
        class="fixed inset-0 z-[100] flex items-center justify-center bg-black/85 p-4"
        @click.self="emit('close')"
      >
        <!-- Fermer -->
        <button
          type="button"
          class="absolute top-4 right-4 w-11 h-11 rounded-full bg-white/10 text-white flex items-center justify-center hover:bg-white/20 transition"
          @click="emit('close')"
        >
          <font-awesome-icon :icon="['fas', 'xmark']" class="text-xl" />
        </button>

        <!-- Précédent -->
        <button
          v-if="aPlusieurs"
          type="button"
          class="absolute left-4 w-11 h-11 rounded-full bg-white/10 text-white flex items-center justify-center hover:bg-white/20 transition"
          @click.stop="precedent"
        >
          <font-awesome-icon :icon="['fas', 'chevron-left']" />
        </button>

        <img
          :src="imageCourante"
          alt=""
          class="max-w-full max-h-[88vh] object-contain rounded-lg shadow-2xl select-none"
          @click.stop
        >

        <!-- Suivant -->
        <button
          v-if="aPlusieurs"
          type="button"
          class="absolute right-4 w-11 h-11 rounded-full bg-white/10 text-white flex items-center justify-center hover:bg-white/20 transition"
          @click.stop="suivant"
        >
          <font-awesome-icon :icon="['fas', 'chevron-right']" />
        </button>

        <!-- Compteur -->
        <div
          v-if="aPlusieurs"
          class="absolute bottom-5 left-1/2 -translate-x-1/2 px-3 py-1 rounded-full bg-white/10 text-white text-sm"
        >
          {{ courant + 1 }} / {{ images.length }}
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.viewer-fade-enter-active, .viewer-fade-leave-active { transition: opacity 0.2s ease; }
.viewer-fade-enter-from, .viewer-fade-leave-to { opacity: 0; }
</style>

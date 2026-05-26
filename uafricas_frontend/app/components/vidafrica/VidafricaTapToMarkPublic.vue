<script setup lang="ts">
import type { TimingMot } from '~/mocks/vidafrica'

const props = defineProps<{
  videoUrl: string
  mots: string[]
  debutMs: number
  finMs: number
}>()

const emit = defineEmits<{
  'timings-enregistres': [timings: TimingMot[]]
  'annuler': []
}>()

const videoRef = ref<HTMLVideoElement | null>(null)
const motCourantIndex = ref(0)
const timings = ref<TimingMot[]>([])
const enCours = ref(false)
const termine = ref(false)
const dernierTimestamp = ref(0)

const motCourant = computed(() => props.mots[motCourantIndex.value] || '')
const progression = computed(() => {
  if (props.mots.length === 0) return 0
  return Math.round((motCourantIndex.value / props.mots.length) * 100)
})

const demarrer = () => {
  const video = videoRef.value
  if (!video) return

  motCourantIndex.value = 0
  timings.value = []
  termine.value = false
  enCours.value = true
  dernierTimestamp.value = props.debutMs

  video.currentTime = props.debutMs / 1000
  video.play()
}

const marquerMot = () => {
  if (!enCours.value || termine.value) return
  const video = videoRef.value
  if (!video) return

  const maintenant = Math.round(video.currentTime * 1000)
  const index = motCourantIndex.value

  if (index < props.mots.length) {
    timings.value.push({
      position: index + 1,
      mot: props.mots[index]!,
      debut_ms: dernierTimestamp.value,
      fin_ms: maintenant,
    })
    dernierTimestamp.value = maintenant
    motCourantIndex.value++

    if (motCourantIndex.value >= props.mots.length) {
      const dernierTiming = timings.value[timings.value.length - 1]!
      if (dernierTiming.fin_ms < props.finMs) {
        dernierTiming.fin_ms = Math.min(maintenant, props.finMs)
      }
      video.pause()
      enCours.value = false
      termine.value = true
    }
  }
}

const recommencer = () => {
  motCourantIndex.value = 0
  timings.value = []
  termine.value = false
  enCours.value = false
}

const valider = () => {
  emit('timings-enregistres', timings.value)
}

const onKeydown = (e: KeyboardEvent) => {
  if (e.code === 'Space' && enCours.value) {
    e.preventDefault()
    marquerMot()
  }
}

onMounted(() => window.addEventListener('keydown', onKeydown))
onUnmounted(() => window.removeEventListener('keydown', onKeydown))
</script>

<template>
  <div class="space-y-4">
    <!-- Lecteur vidéo -->
    <video
      ref="videoRef"
      :src="videoUrl"
      class="w-full rounded-lg max-h-64 bg-black"
      preload="metadata"
      controls
    />

    <!-- Progression -->
    <div class="flex items-center gap-3">
      <div class="flex-1 h-2 rounded-full bg-gray-200 overflow-hidden">
        <div class="h-full bg-custom-chocolat transition-all duration-150" :style="{ width: `${progression}%` }" />
      </div>
      <span class="text-sm font-mono text-gray-600">{{ motCourantIndex }}/{{ mots.length }}</span>
    </div>

    <!-- Mots à marquer -->
    <div class="bg-gray-50 rounded-lg p-4 min-h-[60px]">
      <div class="flex flex-wrap gap-1">
        <span
          v-for="(mot, i) in mots" :key="i"
          class="px-2 py-1 rounded text-sm font-medium transition-all"
          :class="{
            'bg-custom-green text-white': i < motCourantIndex,
            'bg-custom-chocolat text-white scale-110 shadow': i === motCourantIndex && enCours,
            'bg-gray-200 text-gray-500': i > motCourantIndex || (!enCours && i === motCourantIndex),
          }"
        >
          {{ mot }}
        </span>
      </div>
    </div>

    <!-- Mot courant (gros) -->
    <div v-if="enCours" class="text-center">
      <p class="text-3xl font-bold text-custom-chocolat">{{ motCourant }}</p>
      <p class="text-sm text-gray-500 mt-1">
        Appuyez sur <kbd class="px-1.5 py-0.5 rounded border border-gray-300 bg-white text-xs font-mono">Espace</kbd>
        ou cliquez le bouton pour marquer
      </p>
    </div>

    <!-- Résultat -->
    <div v-if="termine" class="rounded-lg bg-custom-green/10 border border-custom-green/30 px-3 py-2 text-sm text-custom-green flex items-center gap-2">
      <font-awesome-icon icon="check-circle" />
      <span>Tous les mots ont été marqués !</span>
    </div>

    <!-- Actions -->
    <div class="flex justify-between items-center flex-wrap gap-2">
      <div class="flex gap-2">
        <button
          v-if="!enCours && !termine"
          class="px-4 py-2 rounded-lg bg-custom-chocolat text-white text-sm font-medium hover:bg-custom-chocolat/90 transition-colors"
          @click="demarrer"
        >
          <font-awesome-icon icon="play" class="mr-1" /> Démarrer
        </button>
        <button
          v-if="enCours"
          class="px-5 py-2.5 rounded-lg bg-custom-green text-white text-base font-semibold hover:bg-custom-green/90 transition-colors"
          @click="marquerMot"
        >
          <font-awesome-icon icon="hand-pointer" class="mr-1" /> Marquer « {{ motCourant }} »
        </button>
        <button
          v-if="termine || enCours"
          class="px-4 py-2 rounded-lg text-sm font-medium text-gray-600 hover:bg-gray-100 transition-colors"
          @click="recommencer"
        >
          <font-awesome-icon icon="redo" class="mr-1" /> Recommencer
        </button>
      </div>
      <div class="flex gap-2">
        <button
          class="px-4 py-2 rounded-lg text-sm font-medium text-gray-600 hover:bg-gray-100 transition-colors"
          @click="emit('annuler')"
        >
          Annuler
        </button>
        <button
          v-if="termine"
          class="px-4 py-2 rounded-lg bg-custom-green text-white text-sm font-medium hover:bg-custom-green/90 transition-colors"
          @click="valider"
        >
          <font-awesome-icon icon="check" class="mr-1" /> Enregistrer les timings
        </button>
      </div>
    </div>
  </div>
</template>

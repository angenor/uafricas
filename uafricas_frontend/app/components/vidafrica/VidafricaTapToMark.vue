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

  // Positionner la vidéo au début du segment
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
      mot: props.mots[index],
      debut_ms: dernierTimestamp.value,
      fin_ms: maintenant,
    })
    dernierTimestamp.value = maintenant
    motCourantIndex.value++

    if (motCourantIndex.value >= props.mots.length) {
      // Dernier mot : ajuster fin_ms au fin du segment
      const dernierTiming = timings.value[timings.value.length - 1]
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

// Écouter la touche Espace
const onKeydown = (e: KeyboardEvent) => {
  if (e.code === 'Space' && enCours.value) {
    e.preventDefault()
    marquerMot()
  }
}

onMounted(() => {
  window.addEventListener('keydown', onKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', onKeydown)
})
</script>

<template>
  <div class="space-y-4">
    <!-- Lecteur vidéo -->
    <video
      ref="videoRef"
      :src="videoUrl"
      class="w-full rounded-lg max-h-64"
      preload="metadata"
      controls
    />

    <!-- Progression -->
    <div class="flex items-center gap-3">
      <progress class="progress progress-primary flex-1" :value="progression" max="100" />
      <span class="text-sm font-mono">{{ motCourantIndex }}/{{ mots.length }}</span>
    </div>

    <!-- Mots à marquer -->
    <div class="bg-base-200 rounded-lg p-4 min-h-[60px]">
      <div class="flex flex-wrap gap-1">
        <span
          v-for="(mot, i) in mots" :key="i"
          class="px-2 py-1 rounded text-sm font-medium transition-all"
          :class="{
            'bg-success text-success-content': i < motCourantIndex,
            'bg-primary text-primary-content scale-110 shadow-lg': i === motCourantIndex && enCours,
            'bg-base-300 text-base-content/60': i > motCourantIndex || (!enCours && i === motCourantIndex),
          }"
        >
          {{ mot }}
        </span>
      </div>
    </div>

    <!-- Mot courant (gros) -->
    <div v-if="enCours" class="text-center">
      <p class="text-3xl font-bold text-primary">{{ motCourant }}</p>
      <p class="text-sm text-base-content/50 mt-1">
        Appuyez sur <kbd class="kbd kbd-sm">Espace</kbd> ou cliquez le bouton pour marquer
      </p>
    </div>

    <!-- Résultat -->
    <div v-if="termine" class="alert alert-success">
      <font-awesome-icon icon="check-circle" />
      <span>Tous les mots ont été marqués !</span>
    </div>

    <!-- Actions -->
    <div class="flex justify-between">
      <div class="flex gap-2">
        <button v-if="!enCours && !termine" class="btn btn-primary" @click="demarrer">
          <font-awesome-icon icon="play" class="mr-1" /> Démarrer
        </button>
        <button v-if="enCours" class="btn btn-accent btn-lg" @click="marquerMot">
          <font-awesome-icon icon="hand-pointer" class="mr-1" /> Marquer « {{ motCourant }} »
        </button>
        <button v-if="termine || enCours" class="btn btn-ghost" @click="recommencer">
          <font-awesome-icon icon="redo" class="mr-1" /> Recommencer
        </button>
      </div>
      <div class="flex gap-2">
        <button class="btn btn-ghost" @click="emit('annuler')">Annuler</button>
        <button v-if="termine" class="btn btn-success" @click="valider">
          <font-awesome-icon icon="check" class="mr-1" /> Enregistrer les timings
        </button>
      </div>
    </div>
  </div>
</template>

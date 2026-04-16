<script setup lang="ts">
interface MotKaraoke {
  position: number
  mot: string
  debut_ms: number
  fin_ms: number
}

interface SegmentKaraoke {
  position: number
  texte: string
  debut_ms: number
  fin_ms: number
  mots: MotKaraoke[]
}

const props = defineProps<{
  videoUrl: string
  segments: SegmentKaraoke[]
}>()

const videoRef = ref<HTMLVideoElement | null>(null)
const segmentCourant = ref<SegmentKaraoke | null>(null)
const motCourantIndex = ref(-1)
const animationId = ref<number | null>(null)

// Recherche binaire pour trouver le segment courant
const trouverSegment = (timeMs: number): SegmentKaraoke | null => {
  const segs = props.segments
  let debut = 0
  let fin = segs.length - 1

  while (debut <= fin) {
    const mid = Math.floor((debut + fin) / 2)
    const seg = segs[mid]
    if (timeMs >= seg.debut_ms && timeMs <= seg.fin_ms) return seg
    if (timeMs < seg.debut_ms) fin = mid - 1
    else debut = mid + 1
  }
  return null
}

// Trouver le mot courant dans un segment
const trouverMot = (segment: SegmentKaraoke, timeMs: number): number => {
  for (let i = 0; i < segment.mots.length; i++) {
    const mot = segment.mots[i]
    if (timeMs >= mot.debut_ms && timeMs < mot.fin_ms) return i
  }
  // Si on est après le dernier mot mais dans le segment, garder le dernier
  if (segment.mots.length > 0) {
    const dernier = segment.mots[segment.mots.length - 1]
    if (timeMs >= dernier.debut_ms) return segment.mots.length - 1
  }
  return -1
}

const mettreAJour = () => {
  const video = videoRef.value
  if (!video || video.paused) {
    animationId.value = null
    return
  }

  const timeMs = Math.round(video.currentTime * 1000)
  const seg = trouverSegment(timeMs)

  if (seg) {
    segmentCourant.value = seg
    if (seg.mots.length > 0) {
      motCourantIndex.value = trouverMot(seg, timeMs)
    } else {
      motCourantIndex.value = -1
    }
  } else {
    segmentCourant.value = null
    motCourantIndex.value = -1
  }

  animationId.value = requestAnimationFrame(mettreAJour)
}

const onPlay = () => {
  if (animationId.value === null) {
    animationId.value = requestAnimationFrame(mettreAJour)
  }
}

const onPause = () => {
  if (animationId.value !== null) {
    cancelAnimationFrame(animationId.value)
    animationId.value = null
  }
}

const onSeeked = () => {
  // Recalcul immédiat après un seek
  const video = videoRef.value
  if (!video) return
  const timeMs = Math.round(video.currentTime * 1000)
  const seg = trouverSegment(timeMs)
  segmentCourant.value = seg
  if (seg && seg.mots.length > 0) {
    motCourantIndex.value = trouverMot(seg, timeMs)
  } else {
    motCourantIndex.value = -1
  }
}

onUnmounted(() => {
  if (animationId.value !== null) {
    cancelAnimationFrame(animationId.value)
  }
})
</script>

<template>
  <div class="relative">
    <!-- Lecteur vidéo -->
    <video
      ref="videoRef"
      :src="videoUrl"
      class="w-full rounded-lg"
      controls
      preload="metadata"
      @play="onPlay"
      @pause="onPause"
      @seeked="onSeeked"
      @ended="onPause"
    />

    <!-- Overlay sous-titres karaoké -->
    <div
      v-if="segmentCourant"
      class="absolute bottom-16 left-0 right-0 flex justify-center pointer-events-none px-4"
    >
      <div class="bg-black/75 rounded-lg px-4 py-2 max-w-[90%]">
        <!-- Si pas de timings mot, afficher le texte entier -->
        <p v-if="segmentCourant.mots.length === 0" class="text-white text-lg font-medium text-center">
          {{ segmentCourant.texte }}
        </p>

        <!-- Avec timings mot : effet karaoké -->
        <p v-else class="text-lg font-medium text-center leading-relaxed">
          <span
            v-for="(mot, i) in segmentCourant.mots" :key="mot.position"
            class="transition-colors duration-100"
            :class="i <= motCourantIndex ? 'text-yellow-300' : 'text-white/60'"
          >{{ mot.mot }}{{ i < segmentCourant.mots.length - 1 ? ' ' : '' }}</span>
        </p>
      </div>
    </div>
  </div>
</template>

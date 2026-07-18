<script setup lang="ts">
import { formaterTimestamp } from '~/mocks/vidafrica'

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

const emit = defineEmits<{
  'segment-change': [position: number | null]
  'lecture': [enLecture: boolean]
  // Vrai quand la vidéo atteint sa fin ; remis à faux dès qu'on relit ou qu'on
  // déplace la tête ailleurs (évite un état « fin » périmé).
  'fin': [termine: boolean]
}>()

const racineRef = ref<HTMLElement | null>(null)
const videoRef = ref<HTMLVideoElement | null>(null)
const segmentCourant = ref<SegmentKaraoke | null>(null)
const motCourantIndex = ref(-1)
const animationId = ref<number | null>(null)

// ── État de la barre de lecture persistante ─────────────────
const positionCourante = ref(0) // ms
const dureeReelle = ref(0) // ms
const enLectureLocale = ref(false)
const muet = ref(false)
const plein = ref(false)
const scrubbing = ref(false)

// Signale le segment courant au parent (surbrillance dans la transcription latérale)
watch(segmentCourant, seg => emit('segment-change', seg ? seg.position : null))

// ── API impérative (transcription latérale + sous-titrage direct) ──
// Permet à un panneau externe de déplacer/piloter la lecture.
const seek = (ms: number, autoPlay = true) => {
  const video = videoRef.value
  if (!video) return
  video.currentTime = ms / 1000
  if (autoPlay) video.play().catch(() => {})
}
const lire = () => videoRef.value?.play().catch(() => {})
const pause = () => videoRef.value?.pause()
const positionMs = () => (videoRef.value ? Math.round(videoRef.value.currentTime * 1000) : 0)
const dureeMs = () => {
  const d = videoRef.value?.duration
  return d && Number.isFinite(d) ? Math.round(d * 1000) : 0
}

defineExpose({ seek, lire, pause, positionMs, dureeMs })

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

const majSegment = (timeMs: number) => {
  const seg = trouverSegment(timeMs)
  if (seg) {
    segmentCourant.value = seg
    motCourantIndex.value = seg.mots.length > 0 ? trouverMot(seg, timeMs) : -1
  } else {
    segmentCourant.value = null
    motCourantIndex.value = -1
  }
}

const mettreAJour = () => {
  const video = videoRef.value
  if (!video || video.paused) {
    animationId.value = null
    return
  }

  const timeMs = Math.round(video.currentTime * 1000)
  if (!scrubbing.value) positionCourante.value = timeMs
  majSegment(timeMs)

  animationId.value = requestAnimationFrame(mettreAJour)
}

// ── Événements média ────────────────────────────────────────
const onPlay = () => {
  enLectureLocale.value = true
  emit('lecture', true)
  emit('fin', false)
  if (animationId.value === null) {
    animationId.value = requestAnimationFrame(mettreAJour)
  }
}

const onPause = () => {
  enLectureLocale.value = false
  emit('lecture', false)
  if (animationId.value !== null) {
    cancelAnimationFrame(animationId.value)
    animationId.value = null
  }
}

const onEnded = () => {
  onPause()
  emit('fin', true)
}

const onSeeked = () => {
  const video = videoRef.value
  if (!video) return
  // Déplacer la tête annule l'état « fin de vidéo » (on n'est plus au bout).
  emit('fin', false)
  // Pendant un drag, ne pas écraser positionCourante avec la position réelle
  // (arrivée en retard/désordre) : le pouce suivrait la tête au lieu du doigt.
  if (scrubbing.value) return
  const timeMs = Math.round(video.currentTime * 1000)
  positionCourante.value = timeMs
  majSegment(timeMs)
}

const onTimeUpdate = () => {
  const video = videoRef.value
  if (!video || scrubbing.value) return
  positionCourante.value = Math.round(video.currentTime * 1000)
}

const onMeta = () => {
  const d = videoRef.value?.duration
  dureeReelle.value = d && Number.isFinite(d) ? Math.round(d * 1000) : 0
}

// ── Contrôles personnalisés ─────────────────────────────────
// Rendre le focus au <body> après un clic bouton : sinon le focus reste sur le
// <button> et le raccourci Espace du sous-titrage direct (qui exclut BUTTON) ne
// déclenche plus la coupe.
const blurCible = (e?: Event) => (e?.currentTarget as HTMLElement | null)?.blur?.()

const togglePlay = () => {
  const v = videoRef.value
  if (!v) return
  if (v.paused) v.play().catch(() => {})
  else v.pause()
}

const onScrub = (e: Event) => {
  const v = videoRef.value
  const val = Number((e.target as HTMLInputElement).value)
  positionCourante.value = val
  if (v) v.currentTime = val / 1000
}

// Fin de scrub (relâché / annulé) : lever le flag ET recaler le pouce sur la
// position réellement atteinte. Robuste aux pointercancel (tactile interrompu).
const finScrub = () => {
  scrubbing.value = false
  const v = videoRef.value
  if (v) positionCourante.value = Math.round(v.currentTime * 1000)
}

const toggleMuet = (e?: Event) => {
  blurCible(e)
  const v = videoRef.value
  if (!v) return
  v.muted = !v.muted
  muet.value = v.muted
}

// Plein écran cross-navigateur (fullscreen standard + préfixes webkit + repli
// iOS sur la vidéo elle-même). Les appels peuvent lever une exception SYNCHRONE
// (méthode absente) : on encapsule dans un try/catch.
const toggleFullscreen = (e?: Event) => {
  blurCible(e)
  const el = racineRef.value as any
  const v = videoRef.value as any
  const doc = document as any
  try {
    if (doc.fullscreenElement || doc.webkitFullscreenElement) {
      (doc.exitFullscreen || doc.webkitExitFullscreen)?.call(document)
    } else if (el?.requestFullscreen) {
      el.requestFullscreen().catch(() => {})
    } else if (el?.webkitRequestFullscreen) {
      el.webkitRequestFullscreen()
    } else if (v?.webkitEnterFullscreen) {
      v.webkitEnterFullscreen() // iOS : plein écran natif de la vidéo
    }
  } catch { /* API indisponible : on ignore */ }
}

const onFsChange = () => {
  const fsEl = (document as any).fullscreenElement || (document as any).webkitFullscreenElement
  plein.value = fsEl === racineRef.value
}

onMounted(() => {
  document.addEventListener('fullscreenchange', onFsChange)
  document.addEventListener('webkitfullscreenchange', onFsChange)
})
onUnmounted(() => {
  document.removeEventListener('fullscreenchange', onFsChange)
  document.removeEventListener('webkitfullscreenchange', onFsChange)
  if (animationId.value !== null) cancelAnimationFrame(animationId.value)
})
</script>

<template>
  <div
    ref="racineRef"
    class="relative"
    :class="plein ? 'fixed inset-0 z-50 bg-black flex flex-col' : ''"
  >
    <!-- Zone vidéo : la boîte épouse la vidéo (l'overlay karaoké reste incrusté sur l'image). -->
    <div class="relative" :class="plein ? 'flex-1 min-h-0 flex items-center justify-center' : ''">
      <video
        ref="videoRef"
        :src="videoUrl"
        class="w-full object-contain bg-black"
        :class="plein ? 'max-h-full h-full' : 'max-h-[72vh] rounded-t-lg'"
        preload="metadata"
        @play="onPlay"
        @pause="onPause"
        @seeked="onSeeked"
        @ended="onEnded"
        @timeupdate="onTimeUpdate"
        @loadedmetadata="onMeta"
        @durationchange="onMeta"
        @click="togglePlay"
      />

      <!-- Overlay sous-titres karaoké -->
      <div
        v-if="segmentCourant"
        class="absolute bottom-4 left-0 right-0 flex justify-center pointer-events-none px-4"
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

    <!-- Barre de lecture PERSISTANTE (la timeline reste toujours visible) -->
    <div
      class="shrink-0 flex items-center gap-2 sm:gap-3 px-3 py-2 bg-gray-900 text-white select-none"
      :class="plein ? '' : 'rounded-b-lg'"
    >
      <button
        class="shrink-0 w-8 h-8 inline-flex items-center justify-center rounded hover:bg-white/10 transition-colors"
        :title="enLectureLocale ? 'Pause' : 'Lecture'"
        @click="togglePlay(); blurCible($event)"
      >
        <font-awesome-icon :icon="enLectureLocale ? 'pause' : 'play'" />
      </button>

      <span class="shrink-0 text-xs font-mono tabular-nums text-white/80">{{ formaterTimestamp(positionCourante) }}</span>

      <input
        type="range"
        min="0"
        :max="dureeReelle || 0"
        :value="positionCourante"
        step="100"
        class="flex-1 h-1.5 accent-custom-chocolat cursor-pointer disabled:cursor-default"
        :disabled="!dureeReelle"
        aria-label="Barre de progression de la vidéo"
        @input="onScrub"
        @pointerdown="scrubbing = true"
        @pointerup="finScrub"
        @pointercancel="finScrub"
        @change="finScrub"
      >

      <span class="shrink-0 text-xs font-mono tabular-nums text-white/60">{{ formaterTimestamp(dureeReelle) }}</span>

      <button
        class="shrink-0 w-8 h-8 inline-flex items-center justify-center rounded hover:bg-white/10 transition-colors"
        :title="muet ? 'Réactiver le son' : 'Couper le son'"
        @click="toggleMuet"
      >
        <font-awesome-icon :icon="muet ? 'volume-xmark' : 'volume-high'" />
      </button>

      <button
        class="shrink-0 w-8 h-8 inline-flex items-center justify-center rounded hover:bg-white/10 transition-colors"
        :title="plein ? 'Quitter le plein écran' : 'Plein écran'"
        @click="toggleFullscreen"
      >
        <font-awesome-icon :icon="plein ? 'compress' : 'expand'" />
      </button>
    </div>
  </div>
</template>

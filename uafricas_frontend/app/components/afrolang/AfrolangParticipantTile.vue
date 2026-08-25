<template>
  <div
    class="relative bg-gray-800 rounded-xl overflow-hidden"
    :class="{
      'ring-2 ring-emerald-400': isDominant && !participant.isMuted,
      'ring-2 ring-blue-400': participant.isLocal && !isScreenShare,
      'h-full': isScreenShare && !prominent,
      // Mode « mis en évidence » : la tuile ne s'étire plus sur toute la largeur ;
      // elle est centrée et bornée au ratio de la source (max 16:9 / 9:16).
      'h-full w-auto max-w-full mx-auto': prominent,
    }"
    :style="prominent ? { aspectRatio: String(ratioContraint) } : undefined"
  >
    <!-- Mode partage d'écran (tuile dédiée) -->
    <video
      v-if="isScreenShare && participant.screenTrack"
      ref="screenRef"
      autoplay
      playsinline
      muted
      class="w-full h-full object-contain bg-black"
      @loadedmetadata="majRatioSource"
      @resize="majRatioSource"
    />

    <!-- Video camera (tuile normale) : monté tant qu'un track existe, caché via v-show
         si la caméra est coupée. Évite le démontage/remontage qui cassait srcObject
         au cycle mute → unmute du correspondant (MediaStreamTrack identique).
         En mode « mis en évidence » : object-contain pour respecter le ratio source
         (pas de recadrage), sinon object-cover pour un remplissage propre en grille. -->
    <video
      v-if="!isScreenShare && participant.videoTrack"
      v-show="!participant.isCameraOff"
      ref="videoRef"
      autoplay
      playsinline
      :muted="participant.isLocal"
      class="w-full h-full"
      :class="prominent ? 'object-contain' : 'object-cover'"
      @loadedmetadata="majRatioSource"
      @resize="majRatioSource"
    />

    <!-- Avatar fallback (pas de track ou caméra off) -->
    <div
      v-if="!isScreenShare && (!participant.videoTrack || participant.isCameraOff)"
      class="w-full h-full flex items-center justify-center bg-gradient-to-br from-gray-700 to-gray-800"
    >
      <div class="w-16 h-16 sm:w-20 sm:h-20 bg-gradient-to-br from-blue-500 to-cyan-500 rounded-full flex items-center justify-center text-2xl sm:text-3xl font-bold text-white">
        {{ initiales }}
      </div>
    </div>

    <!-- Audio distant : géré dans AfrolangRoom (conteneur stable) pour ne pas se
         couper lors des changements de layout (partage d'écran, spotlight…). -->

    <!-- Overlay bottom -->
    <div class="absolute bottom-0 inset-x-0 bg-gradient-to-t from-black/70 to-transparent p-2 sm:p-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-1.5 min-w-0">
          <span class="text-sm font-medium text-white truncate">
            {{ participant.name }}
          </span>
          <span v-if="isScreenShare" class="text-xs text-emerald-300 flex items-center gap-1">
            <font-awesome-icon :icon="['fas', 'display']" class="w-3 h-3" />
            Partage d'écran
          </span>
          <span v-else-if="participant.isLocal" class="text-xs text-blue-300">(vous)</span>
          <span
            v-if="participant.isModerator"
            class="bg-blue-500/80 text-white text-[10px] px-1.5 py-0.5 rounded-full font-medium shrink-0"
          >
            Mod
          </span>
        </div>

        <div class="flex items-center gap-1 shrink-0">
          <!-- Indicateur micro -->
          <span
            class="w-6 h-6 rounded-full flex items-center justify-center text-xs"
            :class="participant.isMuted ? 'bg-red-500/80 text-white' : 'bg-gray-800/50 text-gray-300'"
          >
            <font-awesome-icon
              :icon="['fas', participant.isMuted ? 'volume-mute' : 'volume-up']"
              class="w-3 h-3"
            />
          </span>

          <!-- Indicateur speaking -->
          <span
            v-if="participant.isSpeaking && !participant.isMuted"
            class="w-6 h-6 rounded-full bg-emerald-500/80 flex items-center justify-center animate-pulse"
          >
            <font-awesome-icon :icon="['fas', 'volume-up']" class="w-3 h-3 text-white" />
          </span>
        </div>
      </div>
    </div>

    <!-- Indicateur camera off -->
    <div
      v-if="participant.isCameraOff && !participant.screenTrack"
      class="absolute top-2 right-2"
    >
      <span class="bg-gray-800/70 text-gray-400 text-xs px-2 py-1 rounded-full flex items-center gap-1">
        <font-awesome-icon :icon="['fas', 'video']" class="w-3 h-3 line-through" />
      </span>
    </div>

    <!-- Contenu superposé fourni par le parent (ex. badge « En train de parler »
         en mode mis en évidence) : positionné dans la tuile, pas dans le conteneur. -->
    <slot />
  </div>
</template>

<script setup lang="ts">
import type { RoomParticipant } from './AfrolangRoom.vue'

const props = defineProps<{
  participant: RoomParticipant
  isDominant: boolean
  isScreenShare?: boolean
  /** Mode « mis en évidence » (spotlight ou partage d'écran proéminent) : la tuile
   *  est centrée et bornée au ratio de la source (max 16:9 paysage / 9:16 portrait). */
  prominent?: boolean
}>()

const videoRef = ref<HTMLVideoElement | null>(null)
const screenRef = ref<HTMLVideoElement | null>(null)

// Ratio (largeur/hauteur) natif de la source vidéo, lu depuis l'élément <video>.
const ratioNaturel = ref<number | null>(null)

const majRatioSource = (e: Event) => {
  const el = e.target as HTMLVideoElement
  if (el?.videoWidth && el?.videoHeight) {
    ratioNaturel.value = el.videoWidth / el.videoHeight
  }
}

// Ratio appliqué en mode « mis en évidence », borné à l'intervalle [9:16, 16:9].
// Défaut 16:9 tant que les métadonnées ne sont pas chargées (ou caméra coupée).
const RATIO_MIN = 9 / 16
const RATIO_MAX = 16 / 9
const ratioContraint = computed(() => {
  const r = ratioNaturel.value
  if (!r) return RATIO_MAX
  return Math.min(Math.max(r, RATIO_MIN), RATIO_MAX)
})

const initiales = computed(() => {
  const name = props.participant.name || '?'
  const parts = name.trim().split(/\s+/)
  if (parts.length >= 2 && parts[0] && parts[1]) {
    return ((parts[0][0] || '') + (parts[1][0] || '')).toUpperCase()
  }
  return name.substring(0, 2).toUpperCase()
})

// Attacher les tracks video aux elements
const attachVideoTrack = () => {
  if (videoRef.value && props.participant.videoTrack) {
    const stream = new MediaStream([props.participant.videoTrack])
    videoRef.value.srcObject = stream
  }
}

const attachScreenTrack = () => {
  if (screenRef.value && props.participant.screenTrack) {
    const stream = new MediaStream([props.participant.screenTrack])
    screenRef.value.srcObject = stream
  }
}

watch(() => props.participant.videoTrack, () => nextTick(attachVideoTrack))
watch(() => props.participant.screenTrack, () => nextTick(attachScreenTrack))

onMounted(() => {
  nextTick(() => {
    attachVideoTrack()
    attachScreenTrack()
  })
})
</script>

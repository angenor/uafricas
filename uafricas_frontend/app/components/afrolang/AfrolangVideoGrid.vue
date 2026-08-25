<template>
  <div ref="rootEl" class="flex-1 p-2 sm:p-4 overflow-hidden flex flex-col gap-2 sm:gap-4">
    <!-- Partage d'écran proéminent + pellicule des participants (pour qu'ils
         restent visibles ; l'audio, lui, est rendu indépendamment dans AfrolangRoom).
         Pellicule en bas sur mobile, à gauche sur desktop. -->
    <template v-if="ecranPartageActif">
      <div class="flex-1 min-h-0 flex flex-col sm:flex-row sm:justify-center gap-2 sm:gap-4">
        <div
          class="order-last sm:order-first shrink-0 flex sm:flex-col gap-2 h-24 sm:h-auto sm:max-h-full sm:w-40 overflow-x-auto sm:overflow-x-hidden sm:overflow-y-auto"
          :class="{ 'overflow-visible!': flipEnCours }"
        >
          <AfrolangParticipantTile
            v-for="participant in participants"
            :key="participant.identity"
            :data-identity="participant.identity"
            :participant="participant"
            :is-dominant="participant.identity === dominantSpeaker"
            class="h-full w-auto sm:h-auto sm:w-full aspect-video shrink-0 rounded-lg"
          />
        </div>
        <div class="relative flex-1 sm:flex-none min-h-0 min-w-0 flex items-center justify-center">
          <AfrolangParticipantTile
            :participant="ecranPartageActif"
            :is-dominant="false"
            :is-screen-share="true"
            :prominent="true"
          />
        </div>
      </div>
    </template>

    <!-- Spotlight (FR-023) : participant mis en évidence agrandi au centre.
         À défaut de mise en évidence manuelle, repli automatique sur l'orateur actif.
         Pellicule des autres en bas sur mobile, à gauche sur desktop. -->
    <template v-else-if="participantSpotlight">
      <div class="flex-1 min-h-0 flex flex-col sm:flex-row sm:justify-center gap-2 sm:gap-4">
        <div
          v-if="autresParticipants.length"
          class="order-last sm:order-first shrink-0 flex sm:flex-col gap-2 h-24 sm:h-auto sm:max-h-full sm:w-40 overflow-x-auto sm:overflow-x-hidden sm:overflow-y-auto transition-all duration-300"
          :class="{ 'overflow-visible!': flipEnCours }"
        >
          <AfrolangParticipantTile
            v-for="participant in autresParticipants"
            :key="participant.identity"
            :data-identity="participant.identity"
            :participant="participant"
            :is-dominant="participant.identity === dominantSpeaker"
            class="h-full w-auto sm:h-auto sm:w-full aspect-video shrink-0 rounded-lg"
          />
        </div>
        <div class="relative flex-1 sm:flex-none min-h-0 min-w-0 flex items-center justify-center">
          <AfrolangParticipantTile
            :key="participantSpotlight.identity"
            :data-identity="participantSpotlight.identity"
            :participant="participantSpotlight"
            :is-dominant="true"
            :prominent="true"
            :class="spotlightEstAuto ? 'border-2 border-emerald-500 rounded-lg' : 'border-2 border-custom-chocolat rounded-lg'"
          >
            <span
              v-if="spotlightEstAuto"
              class="absolute top-2 left-2 z-10 inline-flex items-center gap-1 px-2 py-1 rounded-full bg-emerald-500 text-white text-[11px] font-semibold tracking-wide"
            >
              <font-awesome-icon :icon="['fas', 'microphone']" class="w-3 h-3" />
              En train de parler
            </span>
            <span
              v-else
              class="absolute top-2 left-2 z-10 inline-flex items-center gap-1 px-2 py-1 rounded-full bg-custom-chocolat text-white text-[11px] font-semibold tracking-wide"
            >
              <font-awesome-icon :icon="['fas', 'star']" class="w-3 h-3" />
              En vedette
            </span>
          </AfrolangParticipantTile>
        </div>
      </div>
    </template>

    <!-- Grille des participants -->
    <div
      v-else
      :class="[gridClass, 'transition-all duration-300 ease-in-out']"
    >
      <AfrolangParticipantTile
        v-for="participant in participants"
        :key="participant.identity"
        :data-identity="participant.identity"
        :participant="participant"
        :is-dominant="participant.identity === dominantSpeaker"
      />
    </div>

    <!-- Empty state -->
    <div
      v-if="participants.length === 0"
      class="flex-1 flex items-center justify-center text-gray-500"
    >
      <div class="text-center">
        <font-awesome-icon :icon="['fas', 'video']" class="w-12 h-12 text-gray-600 mb-3" />
        <p class="text-lg">En attente de participants...</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { RoomParticipant } from './AfrolangRoom.vue'

const props = defineProps<{
  participants: RoomParticipant[]
  dominantSpeaker: string | null
}>()

// Feature 001-session-moderation : mise en évidence (spotlight)
const { spotlightActif } = useAfrolang()

// Spotlight manuel défini par un modérateur (FR-023), prioritaire.
const participantSpotlightManuel = computed<RoomParticipant | null>(() => {
  const id = spotlightActif.value?.utilisateur_id
  if (!id) return null
  return props.participants.find(p => p.identity === id) ?? null
})

// Repli automatique : mémorise le dernier orateur actif (le `dominantSpeaker`
// repasse à null dès que le silence s'installe ; on conserve la dernière valeur
// pour éviter que la vue oscille sans cesse entre grille et vedette).
const dernierOrateur = ref<string | null>(null)
watch(() => props.dominantSpeaker, (id) => {
  if (id) dernierOrateur.value = id
})

// Si personne n'est mis en évidence manuellement, on met en évidence l'orateur
// courant. Ignoré s'il n'y a qu'un seul participant (rien à mettre en vedette).
const participantOrateurAuto = computed<RoomParticipant | null>(() => {
  if (participantSpotlightManuel.value) return null
  if (props.participants.length < 2) return null
  const id = dernierOrateur.value
  if (!id) return null
  return props.participants.find(p => p.identity === id) ?? null
})

const participantSpotlight = computed<RoomParticipant | null>(() =>
  participantSpotlightManuel.value ?? participantOrateurAuto.value,
)

// Vrai quand la mise en évidence provient du repli automatique (orateur) et non
// d'un modérateur : sert à différencier le libellé/couleur du badge.
const spotlightEstAuto = computed(() =>
  !participantSpotlightManuel.value && !!participantOrateurAuto.value,
)

const autresParticipants = computed<RoomParticipant[]>(() => {
  const id = participantSpotlight.value?.identity
  if (!id) return props.participants
  return props.participants.filter(p => p.identity !== id)
})

// Trouver le participant qui partage son écran
const ecranPartageActif = computed<RoomParticipant | null>(() => {
  return props.participants.find(p => p.screenTrack !== null) ?? null
})

const gridClass = computed(() => {
  const count = props.participants.length
  if (count === 0) return 'flex'
  if (count === 1) return 'grid grid-cols-1'
  if (count === 2) return 'grid grid-cols-2 gap-2 sm:gap-4'
  if (count <= 4) return 'grid grid-cols-2 grid-rows-2 gap-2 sm:gap-4'
  if (count <= 6) return 'grid grid-cols-3 grid-rows-2 gap-2 sm:gap-3'
  if (count <= 9) return 'grid grid-cols-3 grid-rows-3 gap-2 sm:gap-3'
  return 'grid grid-cols-4 auto-rows-fr gap-2'
})

// ── Transition FLIP : la vignette « vole » et grandit vers la place en évidence
//    (et inversement l'ancienne rétrécit vers la pellicule / la grille). On
//    photographie la position de chaque tuile (par `data-identity`) AVANT le
//    changement de disposition, puis on inverse+rejoue le transform après. ──
const rootEl = ref<HTMLElement | null>(null)
const rectsAvant = new Map<string, DOMRect>()
const DUREE_FLIP = 420
// Pendant le vol des tuiles, on relâche l'overflow de la pellicule pour qu'elle
// ne découpe pas la tuile qui traverse l'espace en évidence.
const flipEnCours = ref(false)
let flipTimer: ReturnType<typeof setTimeout> | null = null

// Signature de la disposition : change dès que le mode (grille/évidence/écran),
// le participant en évidence ou l'ordre des participants évolue.
const dispositionSignature = computed(() => {
  const mode = ecranPartageActif.value ? 'ecran' : (participantSpotlight.value ? 'evidence' : 'grille')
  const evidence = participantSpotlight.value?.identity ?? ''
  const ids = props.participants.map(p => p.identity).join(',')
  return `${mode}|${evidence}|${ids}`
})

const listerTuiles = (): HTMLElement[] =>
  rootEl.value ? Array.from(rootEl.value.querySelectorAll<HTMLElement>('[data-identity]')) : []

const photographierRects = () => {
  rectsAvant.clear()
  for (const el of listerTuiles()) {
    const id = el.dataset.identity
    if (id) rectsAvant.set(id, el.getBoundingClientRect())
  }
}

const jouerFlip = () => {
  const aRejouer: HTMLElement[] = []
  for (const el of listerTuiles()) {
    const id = el.dataset.identity
    if (!id) continue
    const avant = rectsAvant.get(id)
    if (!avant) continue // nouvelle tuile (participant qui rejoint) : pas de vol
    const apres = el.getBoundingClientRect()
    if (!apres.width || !apres.height) continue
    const dx = avant.left - apres.left
    const dy = avant.top - apres.top
    const sx = avant.width / apres.width
    const sy = avant.height / apres.height
    // Ignorer les tuiles qui n'ont pas bougé (évite les animations parasites).
    if (Math.abs(dx) < 1 && Math.abs(dy) < 1 && Math.abs(sx - 1) < 0.01 && Math.abs(sy - 1) < 0.01) continue
    // Invert : replacer visuellement la tuile à sa position/taille d'origine.
    el.style.transition = 'none'
    el.style.transformOrigin = 'top left'
    el.style.transform = `translate(${dx}px, ${dy}px) scale(${sx}, ${sy})`
    el.style.zIndex = sx > 1 || sy > 1 ? '20' : '' // la tuile qui grandit passe au-dessus
    aRejouer.push(el)
  }
  if (!aRejouer.length) return
  // Relâcher l'overflow de la pellicule le temps de l'animation.
  flipEnCours.value = true
  if (flipTimer) clearTimeout(flipTimer)
  flipTimer = setTimeout(() => { flipEnCours.value = false }, DUREE_FLIP + 80)
  // Forcer un reflow pour que l'état inversé soit pris en compte avant de rejouer.
  void rootEl.value?.offsetWidth
  requestAnimationFrame(() => {
    for (const el of aRejouer) {
      el.style.transition = `transform ${DUREE_FLIP}ms cubic-bezier(0.22, 1, 0.36, 1)`
      el.style.transform = ''
      const nettoyer = () => {
        el.style.transition = ''
        el.style.transformOrigin = ''
        el.style.zIndex = ''
        el.removeEventListener('transitionend', nettoyer)
      }
      el.addEventListener('transitionend', nettoyer)
    }
  })
}

// flush 'pre' : la photo est prise AVANT que le DOM ne soit repeint (ancienne
// disposition) ; nextTick rejoue le FLIP APRÈS la mise à jour (nouvelle position).
watch(dispositionSignature, () => {
  photographierRects()
  nextTick(jouerFlip)
}, { flush: 'pre' })

onBeforeUnmount(() => {
  if (flipTimer) clearTimeout(flipTimer)
})
</script>

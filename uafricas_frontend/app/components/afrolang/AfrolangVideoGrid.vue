<template>
  <div class="flex-1 p-2 sm:p-4 overflow-hidden flex flex-col gap-2 sm:gap-4">
    <!-- Partage d'écran proéminent + pellicule des participants (pour qu'ils
         restent visibles ; l'audio, lui, est rendu indépendamment dans AfrolangRoom).
         Pellicule en bas sur mobile, à gauche sur desktop. -->
    <template v-if="ecranPartageActif">
      <div class="flex-1 min-h-0 flex flex-col sm:flex-row gap-2 sm:gap-4">
        <div class="order-last sm:order-first shrink-0 flex sm:flex-col gap-2 h-24 sm:h-auto sm:w-40 overflow-x-auto sm:overflow-x-hidden sm:overflow-y-auto">
          <AfrolangParticipantTile
            v-for="participant in participants"
            :key="participant.identity"
            :participant="participant"
            :is-dominant="participant.identity === dominantSpeaker"
            class="h-full w-auto sm:h-auto sm:w-full aspect-video shrink-0 rounded-lg"
          />
        </div>
        <div class="relative flex-1 min-h-0 flex items-center justify-center">
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
      <div class="flex-1 min-h-0 flex flex-col sm:flex-row gap-2 sm:gap-4 transition-all duration-300 ease-in-out">
        <div
          v-if="autresParticipants.length"
          class="order-last sm:order-first shrink-0 flex sm:flex-col gap-2 h-24 sm:h-auto sm:w-40 overflow-x-auto sm:overflow-x-hidden sm:overflow-y-auto transition-all duration-300"
        >
          <AfrolangParticipantTile
            v-for="participant in autresParticipants"
            :key="participant.identity"
            :participant="participant"
            :is-dominant="participant.identity === dominantSpeaker"
            class="h-full w-auto sm:h-auto sm:w-full aspect-video shrink-0 rounded-lg"
          />
        </div>
        <div class="relative flex-1 min-h-0 flex items-center justify-center">
          <AfrolangParticipantTile
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

// Spotlight manuel défini par un modérateur (FR-023) — prioritaire.
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
// d'un modérateur — sert à différencier le libellé/couleur du badge.
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
</script>

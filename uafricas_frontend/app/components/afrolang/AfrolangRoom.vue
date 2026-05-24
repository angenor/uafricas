<template>
  <div class="fixed inset-0 z-9999 flex h-screen bg-gray-900 text-white">
    <!-- Zone video principale -->
    <div class="flex-1 flex flex-col min-w-0">
      <!-- Header -->
      <div class="flex items-center justify-between px-4 py-2 bg-gray-800 border-b border-gray-700">
        <div class="flex items-center gap-3 min-w-0">
          <h2 class="text-lg font-bold truncate">{{ session.titre || 'Session Afrolang' }}</h2>
          <span
            v-if="connectionState === 'connected'"
            class="bg-red-500 text-white px-2 py-0.5 rounded-full text-xs font-medium flex items-center gap-1 animate-pulse shrink-0"
          >
            <font-awesome-icon :icon="['fas', 'circle']" class="w-1.5 h-1.5" />
            EN DIRECT
          </span>
        </div>
        <div class="flex items-center gap-3 shrink-0">
          <span class="text-sm text-gray-400 font-mono">{{ dureeFormatee }}</span>
          <button
            class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
            :class="sidebarOuverte ? 'bg-gray-700 text-blue-400' : 'text-gray-400'"
            @click="sidebarOuverte = !sidebarOuverte"
          >
            <font-awesome-icon :icon="['fas', 'users']" class="w-4 h-4" />
          </button>
          <button
            v-if="monNiveauModerateurSession"
            class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
            :class="moderationPanelOuvert ? 'bg-gray-700 text-amber-400' : 'text-gray-400'"
            aria-label="Modération"
            @click="moderationPanelOuvert = !moderationPanelOuvert"
          >
            <font-awesome-icon :icon="['fas', 'shield-halved']" class="w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- Etat de connexion -->
      <div
        v-if="connectionState === 'reconnecting'"
        class="bg-amber-500/20 border-b border-amber-500/40 px-4 py-2 text-center text-amber-300 text-sm flex items-center justify-center gap-2"
      >
        <font-awesome-icon :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
        Reconnexion en cours...
      </div>
      <div
        v-if="connectionState === 'disconnected' && wasConnected"
        class="bg-red-500/20 border-b border-red-500/40 px-4 py-3 text-center"
      >
        <p class="text-red-300 text-sm mb-2">Connexion perdue</p>
        <button
          class="px-4 py-1.5 bg-red-500 text-white text-sm rounded-lg hover:bg-red-600 transition-colors"
          @click="$emit('quitter')"
        >
          Revenir à la salle
        </button>
      </div>

      <!-- Zone principale : video + tableau blanc en split-screen -->
      <div class="flex-1 flex min-h-0">
        <!-- Grille video -->
        <AfrolangVideoGrid
          :participants="allParticipants"
          :dominant-speaker="dominantSpeaker"
          class="flex-1"
        />

        <!-- Tableau blanc (split-screen droite) -->
        <AfrolangWhiteboard
          v-if="tableauBlancOuvert && session.tableau_blanc_actif"
          :session-id="session.id"
          :est-moderateur="estModerateur"
          :ecriture-autorisee="monEcritureAutorisee"
          :room="room"
          class="w-1/2 border-l border-gray-700"
          @fermer="tableauBlancOuvert = false"
        />

        <!-- Panneau modération (visible uniquement pour les modérateurs de session) -->
        <SalleModerationPanel
          v-if="moderationPanelOuvert && monNiveauModerateurSession"
          :session-id="session.id"
          :participants="allParticipants"
          :est-session-publique="!session.salle_privee_id"
          @fermer="moderationPanelOuvert = false"
        />
      </div>

      <!-- Controles -->
      <AfrolangControls
        :micro-actif="microActif"
        :camera-active="cameraActive"
        :ecran-partage="ecranPartage"
        :tableau-blanc-ouvert="tableauBlancOuvert"
        :est-moderateur="estModerateur"
        :connected="connectionState === 'connected'"
        @toggle-micro="toggleMicro"
        @toggle-camera="toggleCamera"
        @toggle-ecran="toggleEcranPartage"
        @toggle-tableau-blanc="toggleTableauBlanc"
        @quitter="handleQuitter"
        @terminer="handleTerminer"
      >
        <template #apres-actions>
          <slot name="apres-actions" />
        </template>
      </AfrolangControls>
    </div>

    <!-- Sidebar participants -->
    <AfrolangSidebar
      v-if="sidebarOuverte"
      :participants="allParticipants"
      :session-id="session.id"
      :dominant-speaker="dominantSpeaker"
    />
  </div>
</template>

<script setup lang="ts">
import {
  Room,
  RoomEvent,
  ConnectionState,
  Track,
  type RemoteParticipant,
  type LocalParticipant,
  type Participant,
  type RemoteTrackPublication,
  type RemoteTrack,
} from 'livekit-client'
import type { SessionDetailAPI } from '~/composables/useAfrolang'

export interface RoomParticipant {
  identity: string
  name: string
  isSpeaking: boolean
  isMuted: boolean
  isCameraOff: boolean
  isLocal: boolean
  isModerator: boolean
  videoTrack: MediaStreamTrack | null
  audioTrack: MediaStreamTrack | null
  screenTrack: MediaStreamTrack | null
}

const props = defineProps<{
  token: string
  roomName: string
  livekitUrl: string
  session: SessionDetailAPI
  estModerateur: boolean
}>()

const emit = defineEmits<{
  quitter: []
  terminer: []
}>()

// Feature 001-session-moderation : composable partagé
const {
  monNiveauModerateurSession,
  monEcritureAutorisee,
  spotlightActif,
  listerPermissionsTableauBlanc,
  attacherListenerModeration,
} = useAfrolang()
let detacherListenerModeration: (() => void) | null = null

// State
const room = shallowRef<Room | null>(null)
const connectionState = ref<string>('connecting')
const wasConnected = ref(false)
const sidebarOuverte = ref(false)
const moderationPanelOuvert = ref(false)
const tableauBlancOuvert = ref(false)
const microActif = ref(true)
const cameraActive = ref(true)
const ecranPartage = ref(false)
const dominantSpeaker = ref<string | null>(null)
const remoteParticipants = ref<Map<string, RoomParticipant>>(new Map())
const localParticipant = ref<RoomParticipant | null>(null)

// Duree en direct
const dureeSecondes = ref(0)
let dureeInterval: ReturnType<typeof setInterval> | null = null

const dureeFormatee = computed(() => {
  const s = dureeSecondes.value
  const h = Math.floor(s / 3600)
  const m = Math.floor((s % 3600) / 60)
  const sec = s % 60
  if (h > 0) return `${h}:${String(m).padStart(2, '0')}:${String(sec).padStart(2, '0')}`
  return `${String(m).padStart(2, '0')}:${String(sec).padStart(2, '0')}`
})

// Tous les participants (local en premier)
const allParticipants = computed<RoomParticipant[]>(() => {
  const list: RoomParticipant[] = []
  if (localParticipant.value) list.push(localParticipant.value)
  for (const p of remoteParticipants.value.values()) {
    list.push(p)
  }
  return list
})

// Extraire les tracks d'un participant
const extractParticipantInfo = (participant: Participant, isLocal: boolean): RoomParticipant => {
  let videoTrack: MediaStreamTrack | null = null
  let audioTrack: MediaStreamTrack | null = null
  let screenTrack: MediaStreamTrack | null = null

  for (const pub of participant.trackPublications.values()) {
    const track = pub.track
    if (!track) continue
    if (pub.source === Track.Source.Camera && track.mediaStreamTrack) {
      videoTrack = track.mediaStreamTrack
    }
    else if (pub.source === Track.Source.Microphone && track.mediaStreamTrack) {
      audioTrack = track.mediaStreamTrack
    }
    else if (pub.source === Track.Source.ScreenShare && track.mediaStreamTrack) {
      screenTrack = track.mediaStreamTrack
    }
  }

  return {
    identity: participant.identity,
    name: participant.name || participant.identity,
    isSpeaking: participant.isSpeaking,
    isMuted: !participant.isMicrophoneEnabled,
    isCameraOff: !participant.isCameraEnabled,
    isLocal,
    isModerator: props.session.moderateur?.id === participant.identity,
    videoTrack,
    audioTrack,
    screenTrack,
  }
}

// Mettre a jour le participant local
const updateLocalParticipant = () => {
  if (!room.value?.localParticipant) return
  localParticipant.value = extractParticipantInfo(room.value.localParticipant, true)
  microActif.value = room.value.localParticipant.isMicrophoneEnabled
  cameraActive.value = room.value.localParticipant.isCameraEnabled
}

// Mettre a jour un participant distant
const updateRemoteParticipant = (participant: RemoteParticipant) => {
  const map = new Map(remoteParticipants.value)
  map.set(participant.identity, extractParticipantInfo(participant, false))
  remoteParticipants.value = map
}

// Retirer un participant distant
const removeRemoteParticipant = (participant: RemoteParticipant) => {
  const map = new Map(remoteParticipants.value)
  map.delete(participant.identity)
  remoteParticipants.value = map
}

// Controles media
const toggleMicro = async () => {
  if (!room.value) return
  await room.value.localParticipant.setMicrophoneEnabled(!microActif.value)
  updateLocalParticipant()
}

const toggleCamera = async () => {
  if (!room.value) return
  await room.value.localParticipant.setCameraEnabled(!cameraActive.value)
  updateLocalParticipant()
}

const toggleEcranPartage = async () => {
  if (!room.value) return
  try {
    if (ecranPartage.value) {
      // Arreter le partage d'ecran
      for (const pub of room.value.localParticipant.trackPublications.values()) {
        if (pub.source === Track.Source.ScreenShare && pub.track) {
          await room.value.localParticipant.unpublishTrack(pub.track)
        }
      }
      ecranPartage.value = false
    }
    else {
      await room.value.localParticipant.setScreenShareEnabled(true)
      ecranPartage.value = true
    }
    updateLocalParticipant()
  }
  catch (e) {
    console.error('Erreur partage ecran:', e)
    ecranPartage.value = false
  }
}

const toggleTableauBlanc = () => {
  tableauBlancOuvert.value = !tableauBlancOuvert.value
}

const handleQuitter = () => {
  emit('quitter')
}

const handleTerminer = () => {
  emit('terminer')
}

// Connexion a la room LiveKit
const connectToRoom = async () => {
  const newRoom = new Room({
    adaptiveStream: true,
    dynacast: true,
  })

  // Evenements de connexion
  newRoom.on(RoomEvent.ConnectionStateChanged, (state: ConnectionState) => {
    connectionState.value = state
    if (state === ConnectionState.Connected) {
      wasConnected.value = true
    }
  })

  // Participant connecte
  newRoom.on(RoomEvent.ParticipantConnected, (participant: RemoteParticipant) => {
    updateRemoteParticipant(participant)
  })

  // Participant deconnecte
  newRoom.on(RoomEvent.ParticipantDisconnected, (participant: RemoteParticipant) => {
    removeRemoteParticipant(participant)
  })

  // Track souscrit
  newRoom.on(RoomEvent.TrackSubscribed, (_track: RemoteTrack, _publication: RemoteTrackPublication, participant: RemoteParticipant) => {
    updateRemoteParticipant(participant)
  })

  // Track desouscrit
  newRoom.on(RoomEvent.TrackUnsubscribed, (_track: RemoteTrack, _publication: RemoteTrackPublication, participant: RemoteParticipant) => {
    updateRemoteParticipant(participant)
  })

  // Track mute/unmute
  newRoom.on(RoomEvent.TrackMuted, (_publication: any, participant: Participant) => {
    if (participant === newRoom.localParticipant) {
      updateLocalParticipant()
    }
    else {
      updateRemoteParticipant(participant as RemoteParticipant)
    }
  })

  newRoom.on(RoomEvent.TrackUnmuted, (_publication: any, participant: Participant) => {
    if (participant === newRoom.localParticipant) {
      updateLocalParticipant()
    }
    else {
      updateRemoteParticipant(participant as RemoteParticipant)
    }
  })

  // Speaking changes
  newRoom.on(RoomEvent.ActiveSpeakersChanged, (speakers: Participant[]) => {
    dominantSpeaker.value = speakers.length > 0 ? (speakers[0]?.identity ?? null) : null
    // Update isSpeaking for all
    updateLocalParticipant()
    for (const [, p] of newRoom.remoteParticipants) {
      updateRemoteParticipant(p)
    }
  })

  // Local track published
  newRoom.on(RoomEvent.LocalTrackPublished, (publication) => {
    updateLocalParticipant()
    if (publication.source === Track.Source.ScreenShare) {
      ecranPartage.value = true
    }
  })

  newRoom.on(RoomEvent.LocalTrackUnpublished, (publication) => {
    updateLocalParticipant()
    if (publication.source === Track.Source.ScreenShare) {
      ecranPartage.value = false
    }
  })

  try {
    await newRoom.connect(props.livekitUrl, props.token)
    room.value = newRoom

    // Activer camera + micro
    await newRoom.localParticipant.setCameraEnabled(true)
    await newRoom.localParticipant.setMicrophoneEnabled(true)

    updateLocalParticipant()

    // Mettre a jour les participants distants existants
    for (const [, p] of newRoom.remoteParticipants) {
      updateRemoteParticipant(p)
    }

    // Demarrer le compteur de duree
    dureeInterval = setInterval(() => {
      dureeSecondes.value++
    }, 1000)

    connectionState.value = 'connected'

    // Feature 001-session-moderation : état initial + listener temps réel
    await listerPermissionsTableauBlanc(props.session.id)
    // FR-024 : initialiser spotlight depuis le GET /sessions/{id} (transmis en prop)
    spotlightActif.value = props.session.spotlight ?? null
    detacherListenerModeration = attacherListenerModeration(newRoom)
  }
  catch (e) {
    console.error('Erreur connexion LiveKit:', e)
    connectionState.value = 'disconnected'
  }
}

onMounted(() => {
  connectToRoom()
})

onBeforeUnmount(async () => {
  if (dureeInterval) clearInterval(dureeInterval)
  if (detacherListenerModeration) {
    detacherListenerModeration()
    detacherListenerModeration = null
  }
  if (room.value) {
    await room.value.disconnect()
  }
})

defineExpose({
  room,
  connectionState,
})
</script>

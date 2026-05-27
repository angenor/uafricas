<template>
  <div class="fixed inset-0 z-[9998] flex h-screen flex-col bg-gray-900 text-white">
    <!-- En-tête -->
    <header class="flex items-center justify-between gap-3 border-b border-gray-700 bg-gray-800 px-4 py-2">
      <div class="flex min-w-0 items-center gap-3">
        <h2 class="truncate text-base font-bold sm:text-lg">{{ titre || 'Direct' }}</h2>
        <span
          v-if="connectionState === 'connected'"
          class="flex shrink-0 animate-pulse items-center gap-1 rounded-full bg-red-500 px-2 py-0.5 text-xs font-medium text-white"
        >
          <font-awesome-icon :icon="['fas', 'circle']" class="h-1.5 w-1.5" />
          EN DIRECT
        </span>
        <span class="shrink-0 text-xs text-gray-400">
          <font-awesome-icon :icon="['fas', 'users']" class="mr-1 h-3 w-3" />{{ nombrePresents }}
        </span>
      </div>
      <div class="flex shrink-0 items-center gap-2">
        <button
          v-if="estOrganisateur"
          type="button"
          class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors"
          :class="moderationOuverte ? 'bg-amber-500 text-white' : 'bg-gray-700/60 text-amber-300 hover:bg-gray-700'"
          @click="moderationOuverte = !moderationOuverte"
        >
          <font-awesome-icon :icon="['fas', 'shield-halved']" class="h-4 w-4" />
          <span v-if="demandes.length" class="rounded-full bg-amber-500 px-1.5 text-[10px] text-white">{{ demandes.length }}</span>
        </button>
        <button
          type="button"
          class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors"
          :class="chatOuvert ? 'bg-custom-green text-white' : 'bg-gray-700/60 text-emerald-300 hover:bg-gray-700'"
          @click="chatOuvert = !chatOuvert"
        >
          <font-awesome-icon :icon="['fas', 'comments']" class="h-4 w-4" />
        </button>
      </div>
    </header>

    <!-- Bandeau d'état -->
    <div
      v-if="connectionState === 'reconnecting'"
      class="flex items-center justify-center gap-2 border-b border-amber-500/40 bg-amber-500/20 px-4 py-2 text-center text-sm text-amber-300"
    >
      <font-awesome-icon :icon="['fas', 'spinner']" class="h-4 w-4 animate-spin" />
      Reconnexion en cours…
    </div>

    <!-- Repli en cas d'échec de connexion (FR-023) -->
    <div v-if="erreurConnexion" class="flex flex-1 items-center justify-center p-6">
      <div class="max-w-md rounded-2xl bg-gray-800 p-8 text-center">
        <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="mb-4 text-4xl text-amber-400" />
        <h3 class="mb-2 text-lg font-semibold">Connexion au direct impossible</h3>
        <p class="mb-6 text-sm text-gray-400">
          Le service de diffusion est momentanément indisponible. Réessayez dans un instant.
        </p>
        <div class="flex flex-col items-center gap-3">
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-lg bg-custom-green px-5 py-2.5 font-semibold text-white transition hover:brightness-110"
            @click="reessayer"
          >
            <font-awesome-icon :icon="['fas', 'rotate-right']" />
            Réessayer
          </button>
          <a
            v-if="lienEnLigne"
            :href="lienEnLigne"
            target="_blank"
            rel="noopener"
            class="inline-flex items-center gap-2 text-sm text-blue-300 hover:underline"
          >
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" />
            Rejoindre via le lien externe
          </a>
          <button type="button" class="text-sm text-gray-400 hover:text-white" @click="emit('quitter')">
            Revenir à l'événement
          </button>
        </div>
      </div>
    </div>

    <!-- Zone principale : vidéos + panneaux -->
    <div v-else class="flex min-h-0 flex-1">
      <!-- Grille vidéo -->
      <div class="relative flex flex-1 items-center justify-center overflow-hidden bg-gray-950 p-3">
        <div
          v-if="tuilesVideo.length"
          class="grid h-full w-full gap-3"
          :class="classeGrille"
        >
          <div
            v-for="p in tuilesVideo"
            :key="p.identity + (p.screenTrack ? '-screen' : '')"
            class="relative flex items-center justify-center overflow-hidden rounded-xl bg-gray-800"
          >
            <video
              v-stream="p.screenTrack || p.videoTrack"
              autoplay
              playsinline
              :muted="p.isLocal"
              class="h-full w-full"
              :class="p.screenTrack ? 'object-contain' : 'object-cover'"
            />
            <span class="absolute bottom-2 left-2 flex items-center gap-1.5 rounded-md bg-black/60 px-2 py-0.5 text-xs">
              <font-awesome-icon v-if="p.isMuted" :icon="['fas', 'microphone-slash']" class="h-3 w-3 text-red-400" />
              {{ p.name }}{{ p.isLocal ? ' (vous)' : '' }}
            </span>
          </div>
        </div>
        <!-- Placeholder si aucun diffuseur n'a activé sa caméra/écran -->
        <div v-else class="text-center text-gray-400">
          <font-awesome-icon :icon="['fas', 'video']" class="mb-3 text-5xl opacity-40" />
          <p class="text-sm">
            {{ estDiffuseur ? 'Activez votre caméra pour démarrer la diffusion.' : "En attente de la diffusion de l'organisateur…" }}
          </p>
        </div>

        <!-- Audios distants (non visibles) -->
        <audio
          v-for="p in audiosDistants"
          :key="'audio-' + p.identity"
          v-stream="p.audioTrack"
          autoplay
        />
      </div>

      <!-- Panneau modération (organisateur) -->
      <EvenementDirectModerationPanel
        v-if="moderationOuverte && estOrganisateur"
        :demandes="demandes"
        :participants="participantsModeration"
        @promouvoir="onPromouvoir"
        @retrograder="onRetrograder"
        @retirer="onRetirer"
        @fermer="moderationOuverte = false"
      />

      <!-- Chat -->
      <EvenementDirectChat
        v-if="chatOuvert"
        :room="room"
        :identite="monIdentite"
        :nom="nom"
        @fermer="chatOuvert = false"
      />
    </div>

    <!-- Contrôles -->
    <EvenementDirectControls
      v-if="!erreurConnexion"
      :role="roleLocal"
      :micro-actif="microActif"
      :camera-active="cameraActive"
      :ecran-partage="ecranPartage"
      :main-levee="mainLevee"
      :connected="connectionState === 'connected'"
      @toggle-micro="toggleMicro"
      @toggle-camera="toggleCamera"
      @toggle-ecran="toggleEcran"
      @lever-main="onLeverMain"
      @cloturer="onCloturer"
      @quitter="emit('quitter')"
    >
      <template #reactions>
        <EvenementDirectReactions :room="room" :nom="nom" :connected="connectionState === 'connected'" />
      </template>
    </EvenementDirectControls>
  </div>
</template>

<script setup lang="ts">
import {
  Room,
  RoomEvent,
  ConnectionState,
  Track,
  type RemoteParticipant,
  type Participant,
  type RemoteTrack,
  type RemoteTrackPublication,
} from 'livekit-client'
import type { ParticipantModeration } from '~/components/evenements/EvenementDirectModerationPanel.vue'
import type { DemandeParole, RoleDirect } from '~/composables/useEvenements'

interface RoomParticipant {
  identity: string
  name: string
  isMuted: boolean
  isLocal: boolean
  videoTrack: MediaStreamTrack | null
  audioTrack: MediaStreamTrack | null
  screenTrack: MediaStreamTrack | null
}

const props = defineProps<{
  token: string
  roomName: string
  livekitUrl: string
  role: RoleDirect
  evenementId: string
  monIdentite: string
  nom: string
  organisateurId: string
  lienEnLigne?: string | null
  demandesInitiales?: DemandeParole[]
  titre?: string | null
}>()

const emit = defineEmits<{ quitter: [] }>()

const { leverMain, promouvoir, retrograder, retirer, cloturerDirect } = useEvenements()

// Directive locale : attache un MediaStreamTrack à un élément <video>/<audio>.
const vStream = {
  mounted(el: HTMLMediaElement, binding: { value: MediaStreamTrack | null }) {
    el.srcObject = binding.value ? new MediaStream([binding.value]) : null
  },
  updated(el: HTMLMediaElement, binding: { value: MediaStreamTrack | null }) {
    el.srcObject = binding.value ? new MediaStream([binding.value]) : null
  },
  beforeUnmount(el: HTMLMediaElement) {
    el.srcObject = null
  },
}

const room = shallowRef<Room | null>(null)
const connectionState = ref<string>('connecting')
const erreurConnexion = ref(false)
const roleLocal = ref<RoleDirect>(props.role)
const microActif = ref(false)
const cameraActive = ref(false)
const ecranPartage = ref(false)
const mainLevee = ref(false)
const moderationOuverte = ref(false)
const chatOuvert = ref(false)

const remoteParticipants = ref<Map<string, RoomParticipant>>(new Map())
const localParticipant = ref<RoomParticipant | null>(null)
const demandes = ref<DemandeParole[]>(props.demandesInitiales ? [...props.demandesInitiales] : [])
const rolesParticipants = ref<Map<string, string>>(new Map())

const decoder = new TextDecoder()

const estOrganisateur = computed(() => roleLocal.value === 'organisateur')
const estDiffuseur = computed(() => roleLocal.value === 'organisateur' || roleLocal.value === 'intervenant')

const allParticipants = computed<RoomParticipant[]>(() => {
  const list: RoomParticipant[] = []
  if (localParticipant.value) list.push(localParticipant.value)
  for (const p of remoteParticipants.value.values()) list.push(p)
  return list
})

const tuilesVideo = computed(() => allParticipants.value.filter(p => p.videoTrack || p.screenTrack))
const audiosDistants = computed(() => [...remoteParticipants.value.values()].filter(p => p.audioTrack))
const nombrePresents = computed(() => allParticipants.value.length)

const classeGrille = computed(() => {
  const n = tuilesVideo.value.length
  if (n <= 1) return 'grid-cols-1'
  if (n <= 4) return 'grid-cols-1 sm:grid-cols-2'
  return 'grid-cols-2 lg:grid-cols-3'
})

const participantsModeration = computed<ParticipantModeration[]>(() =>
  [...remoteParticipants.value.values()]
    .filter(p => p.identity !== props.organisateurId)
    .map(p => ({
      identity: p.identity,
      nom: p.name,
      role: (rolesParticipants.value.get(p.identity) as ParticipantModeration['role']) || 'spectateur',
    })),
)

// ── Extraction des tracks ───────────────────────────────────────
const extraire = (participant: Participant, isLocal: boolean): RoomParticipant => {
  let videoTrack: MediaStreamTrack | null = null
  let audioTrack: MediaStreamTrack | null = null
  let screenTrack: MediaStreamTrack | null = null
  for (const pub of participant.trackPublications.values()) {
    const track = pub.track
    if (!track?.mediaStreamTrack) continue
    if (pub.source === Track.Source.Camera) videoTrack = track.mediaStreamTrack
    else if (pub.source === Track.Source.Microphone) audioTrack = track.mediaStreamTrack
    else if (pub.source === Track.Source.ScreenShare) screenTrack = track.mediaStreamTrack
  }
  return {
    identity: participant.identity,
    name: participant.name || participant.identity,
    isMuted: !participant.isMicrophoneEnabled,
    isLocal,
    videoTrack,
    audioTrack,
    screenTrack,
  }
}

const majLocal = (): void => {
  if (!room.value?.localParticipant) return
  localParticipant.value = extraire(room.value.localParticipant, true)
  microActif.value = room.value.localParticipant.isMicrophoneEnabled
  cameraActive.value = room.value.localParticipant.isCameraEnabled
}

const majDistant = (participant: RemoteParticipant): void => {
  const map = new Map(remoteParticipants.value)
  map.set(participant.identity, extraire(participant, false))
  remoteParticipants.value = map
}

const retirerDistant = (participant: RemoteParticipant): void => {
  const map = new Map(remoteParticipants.value)
  map.delete(participant.identity)
  remoteParticipants.value = map
}

// ── DataPackets de modération / admin (T030) ────────────────────
const majDemande = (payload: { utilisateur_id?: string, nom?: string, levee?: boolean }): void => {
  if (!payload?.utilisateur_id) return
  const sans = demandes.value.filter(d => d.utilisateur_id !== payload.utilisateur_id)
  if (payload.levee) {
    demandes.value = [...sans, { utilisateur_id: payload.utilisateur_id, nom: payload.nom || 'Invité', main_levee_at: new Date().toISOString() }]
  }
  else {
    demandes.value = sans
  }
}

const appliquerRole = async (nouveau: RoleDirect): Promise<void> => {
  roleLocal.value = nouveau
  if (!room.value) return
  if (nouveau === 'spectateur') {
    // Rétrogradé : couper toute diffusion média.
    try {
      await room.value.localParticipant.setCameraEnabled(false)
      await room.value.localParticipant.setMicrophoneEnabled(false)
      if (ecranPartage.value) await room.value.localParticipant.setScreenShareEnabled(false)
    }
    catch { /* ignore */ }
    mainLevee.value = false
  }
  else {
    // Promu : activer caméra + micro.
    try {
      await room.value.localParticipant.setCameraEnabled(true)
      await room.value.localParticipant.setMicrophoneEnabled(true)
    }
    catch (e) { console.error('Activation diffusion après promotion:', e) }
  }
  majLocal()
}

const onModerationData = (payload: Uint8Array): void => {
  try {
    const data = JSON.parse(decoder.decode(payload)) as {
      type?: string, subtype?: string, payload?: Record<string, unknown>
    }
    if (data?.type === 'admin' && data.subtype === 'session_fermee') {
      emit('quitter')
      return
    }
    if (data?.type !== 'moderation') return
    const p = data.payload || {}
    const cible = p.utilisateur_id as string | undefined
    if (data.subtype === 'role_update' && cible) {
      const map = new Map(rolesParticipants.value)
      map.set(cible, p.role as string)
      rolesParticipants.value = map
      if (cible === props.monIdentite) void appliquerRole(p.role as RoleDirect)
    }
    else if (data.subtype === 'retire' && cible === props.monIdentite) {
      emit('quitter')
    }
    else if (data.subtype === 'main_levee') {
      majDemande(p as { utilisateur_id?: string, nom?: string, levee?: boolean })
    }
  }
  catch { /* trame ignorée */ }
}

// ── Connexion ───────────────────────────────────────────────────
const connecter = async (): Promise<void> => {
  erreurConnexion.value = false
  connectionState.value = 'connecting'
  const r = new Room({ adaptiveStream: true, dynacast: true })

  r.on(RoomEvent.ConnectionStateChanged, (state: ConnectionState) => {
    connectionState.value = state
  })
  r.on(RoomEvent.ParticipantConnected, (p: RemoteParticipant) => majDistant(p))
  r.on(RoomEvent.ParticipantDisconnected, (p: RemoteParticipant) => retirerDistant(p))
  r.on(RoomEvent.TrackSubscribed, (_t: RemoteTrack, _pub: RemoteTrackPublication, p: RemoteParticipant) => majDistant(p))
  r.on(RoomEvent.TrackUnsubscribed, (_t: RemoteTrack, _pub: RemoteTrackPublication, p: RemoteParticipant) => majDistant(p))
  r.on(RoomEvent.TrackMuted, (_pub, p: Participant) => (p === r.localParticipant ? majLocal() : majDistant(p as RemoteParticipant)))
  r.on(RoomEvent.TrackUnmuted, (_pub, p: Participant) => (p === r.localParticipant ? majLocal() : majDistant(p as RemoteParticipant)))
  r.on(RoomEvent.LocalTrackPublished, (pub) => {
    if (pub.source === Track.Source.ScreenShare) ecranPartage.value = true
    majLocal()
  })
  r.on(RoomEvent.LocalTrackUnpublished, (pub) => {
    if (pub.source === Track.Source.ScreenShare) ecranPartage.value = false
    majLocal()
  })
  r.on(RoomEvent.DataReceived, onModerationData)

  try {
    await r.connect(props.livekitUrl, props.token)
    room.value = r
    if (estDiffuseur.value) {
      await r.localParticipant.setCameraEnabled(true)
      await r.localParticipant.setMicrophoneEnabled(true)
    }
    majLocal()
    for (const p of r.remoteParticipants.values()) majDistant(p)
    connectionState.value = 'connected'
  }
  catch (e) {
    console.error('Erreur connexion LiveKit:', e)
    erreurConnexion.value = true
    connectionState.value = 'disconnected'
  }
}

const reessayer = (): void => { void connecter() }

// ── Contrôles diffuseur ─────────────────────────────────────────
const toggleMicro = async (): Promise<void> => {
  if (!room.value || !estDiffuseur.value) return
  await room.value.localParticipant.setMicrophoneEnabled(!microActif.value)
  majLocal()
}
const toggleCamera = async (): Promise<void> => {
  if (!room.value || !estDiffuseur.value) return
  await room.value.localParticipant.setCameraEnabled(!cameraActive.value)
  majLocal()
}
const toggleEcran = async (): Promise<void> => {
  if (!room.value || !estDiffuseur.value) return
  try {
    await room.value.localParticipant.setScreenShareEnabled(!ecranPartage.value)
  }
  catch (e) { console.error('Erreur partage écran:', e) }
  majLocal()
}

// ── Actions pilotées par API ────────────────────────────────────
const onLeverMain = async (): Promise<void> => {
  try {
    await leverMain(props.evenementId)
    mainLevee.value = !mainLevee.value
  }
  catch (e) { console.error('Erreur lever la main:', e) }
}
const onCloturer = async (): Promise<void> => {
  try {
    await cloturerDirect(props.evenementId)
  }
  catch (e) { console.error('Erreur clôture:', e) }
  finally { emit('quitter') }
}
const onPromouvoir = async (uid: string): Promise<void> => {
  try {
    await promouvoir(props.evenementId, uid)
    demandes.value = demandes.value.filter(d => d.utilisateur_id !== uid)
    const map = new Map(rolesParticipants.value)
    map.set(uid, 'intervenant')
    rolesParticipants.value = map
  }
  catch (e) { console.error('Erreur promotion:', e) }
}
const onRetrograder = async (uid: string): Promise<void> => {
  try {
    await retrograder(props.evenementId, uid)
    const map = new Map(rolesParticipants.value)
    map.set(uid, 'spectateur')
    rolesParticipants.value = map
  }
  catch (e) { console.error('Erreur rétrogradation:', e) }
}
const onRetirer = async (uid: string): Promise<void> => {
  try { await retirer(props.evenementId, uid) }
  catch (e) { console.error('Erreur retrait:', e) }
}

onMounted(() => { void connecter() })

onBeforeUnmount(async () => {
  if (room.value) {
    try { await room.value.disconnect() }
    catch { /* ignore */ }
  }
})
</script>

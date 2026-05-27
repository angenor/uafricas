<script setup lang="ts">
import type { Peer, MediaConnection } from 'peerjs'
import type { RendezVousAPI, SalleAPI } from '~/composables/useRendezVous'

const props = defineProps<{ rdv: RendezVousAPI }>()
const emit = defineEmits<{ (e: 'fermer'): void }>()

const { obtenirSalle } = useRendezVous()
const { demanderOuverture } = useMessagerie()
const config = useRuntimeConfig()

type Etat = 'init' | 'attente' | 'connexion' | 'connecte' | 'parti' | 'echec'
const etat = ref<Etat>('init')
const messageEtat = ref('Préparation de la salle…')
const microActif = ref(true)
const cameraActive = ref(true)

const videoLocal = ref<HTMLVideoElement | null>(null)
const videoDistant = ref<HTMLVideoElement | null>(null)

let peer: Peer | null = null
let appelEnCours: MediaConnection | null = null
let fluxLocal: MediaStream | null = null
let relanceAppel: ReturnType<typeof setInterval> | null = null
let salle: SalleAPI | null = null

const nomComplet = computed(() => `${props.rdv.autre.prenom} ${props.rdv.autre.nom}`.trim())
const enEchec = computed(() => etat.value === 'echec' || etat.value === 'parti')

const arreterRelance = () => {
  if (relanceAppel) { clearInterval(relanceAppel); relanceAppel = null }
}

const afficherFluxDistant = (stream: MediaStream) => {
  if (videoDistant.value) videoDistant.value.srcObject = stream
  etat.value = 'connecte'
  messageEtat.value = ''
  arreterRelance()
}

const gererAppel = (call: MediaConnection) => {
  appelEnCours = call
  if (etat.value !== 'connecte') {
    etat.value = 'connexion'
    messageEtat.value = 'Connexion en cours…'
  }
  call.on('stream', afficherFluxDistant)
  call.on('close', () => {
    etat.value = 'parti'
    messageEtat.value = `${props.rdv.autre.prenom} a quitté la visioconférence.`
  })
  call.on('error', () => {
    etat.value = 'echec'
    messageEtat.value = 'La connexion vidéo a échoué.'
  })
}

const demarrer = async () => {
  // 1. Configuration de la salle (le serveur revérifie fenêtre + amitié/blocage).
  try {
    salle = await obtenirSalle(props.rdv.id)
  }
  catch (e) {
    etat.value = 'echec'
    messageEtat.value = e instanceof Error ? e.message : 'La salle est indisponible.'
    return
  }

  // 2. Flux local (caméra + micro).
  try {
    fluxLocal = await navigator.mediaDevices.getUserMedia({ video: true, audio: true })
    if (videoLocal.value) videoLocal.value.srcObject = fluxLocal
  }
  catch {
    etat.value = 'echec'
    messageEtat.value = "Impossible d'accéder à la caméra ou au micro. Vérifiez les autorisations du navigateur."
    return
  }

  // 3. Connexion PeerJS (cloud public par défaut, surchargeable par env).
  const { default: PeerCtor } = await import('peerjs')
  const opts: Record<string, unknown> = {
    config: { iceServers: config.public.iceServers },
  }
  if (config.public.peerjsHost) {
    opts.host = config.public.peerjsHost
    opts.port = config.public.peerjsPort
    opts.path = config.public.peerjsPath
    opts.secure = config.public.peerjsSecure
  }
  peer = new PeerCtor(salle.mon_peer_id, opts)

  etat.value = 'attente'
  messageEtat.value = `En attente que ${props.rdv.autre.prenom} rejoigne…`

  peer.on('open', () => {
    // Tout participant répond aux appels entrants.
    peer?.on('call', (call) => {
      if (fluxLocal) {
        call.answer(fluxLocal)
        gererAppel(call)
      }
    })
    // Anti-glare : seul l'appelant (plus petit UUID) initie, avec relance tant
    // que l'autre n'est pas connecté (research §7).
    if (salle?.suis_appelant) {
      const tenter = () => {
        if (!peer || !fluxLocal || etat.value === 'connecte') return
        const call = peer.call(salle!.pair_peer_id, fluxLocal)
        if (call) gererAppel(call)
      }
      tenter()
      relanceAppel = setInterval(tenter, 3000)
    }
  })

  peer.on('error', (err: unknown) => {
    // 'peer-unavailable' : l'autre n'est pas encore présent → on continue d'attendre.
    if ((err as { type?: string })?.type === 'peer-unavailable') return
    etat.value = 'echec'
    messageEtat.value = 'La connexion a échoué (réseau incompatible).'
  })
}

const basculerMicro = () => {
  microActif.value = !microActif.value
  fluxLocal?.getAudioTracks().forEach((t) => { t.enabled = microActif.value })
}
const basculerCamera = () => {
  cameraActive.value = !cameraActive.value
  fluxLocal?.getVideoTracks().forEach((t) => { t.enabled = cameraActive.value })
}

const nettoyer = () => {
  arreterRelance()
  appelEnCours?.close()
  appelEnCours = null
  peer?.destroy()
  peer = null
  fluxLocal?.getTracks().forEach(t => t.stop())
  fluxLocal = null
}

const quitter = () => { nettoyer(); emit('fermer') }
const ouvrirMessagerie = () => {
  nettoyer()
  demanderOuverture(props.rdv.autre)
  emit('fermer')
}

onMounted(demarrer)
onBeforeUnmount(nettoyer)
</script>

<template>
  <div class="fixed inset-0 z-[70] bg-gray-900 flex flex-col">
    <!-- En-tête -->
    <header class="flex items-center justify-between px-4 py-3 text-white/90 shrink-0">
      <div class="min-w-0">
        <p class="font-semibold truncate">{{ rdv.sujet }}</p>
        <p class="text-xs text-white/60 truncate">Avec {{ nomComplet }}</p>
      </div>
      <button type="button" class="p-2 hover:bg-white/10 rounded-lg transition" aria-label="Fermer" @click="quitter">
        <font-awesome-icon icon="fa-solid fa-xmark" class="text-xl" />
      </button>
    </header>

    <!-- Scène vidéo -->
    <div class="relative flex-1 min-h-0">
      <!-- Flux distant (plein cadre) -->
      <video
        ref="videoDistant"
        autoplay
        playsinline
        class="w-full h-full object-cover bg-black"
        :class="etat === 'connecte' ? '' : 'opacity-0'"
      />

      <!-- Overlay d'état (hors connecté) -->
      <div
        v-if="etat !== 'connecte'"
        class="absolute inset-0 flex flex-col items-center justify-center text-center px-6 text-white"
      >
        <font-awesome-icon
          v-if="etat === 'init' || etat === 'attente' || etat === 'connexion'"
          icon="fa-solid fa-spinner"
          spin
          class="text-3xl mb-4 text-white/70"
        />
        <font-awesome-icon
          v-else-if="etat === 'echec'"
          icon="fa-solid fa-triangle-exclamation"
          class="text-3xl mb-4 text-amber-400"
        />
        <font-awesome-icon
          v-else
          icon="fa-solid fa-phone-slash"
          class="text-3xl mb-4 text-white/60"
        />
        <p class="text-sm text-white/80 max-w-sm">{{ messageEtat }}</p>

        <!-- Repli messagerie (échec ou départ) -->
        <button
          v-if="enEchec"
          type="button"
          class="mt-5 inline-flex items-center gap-2 px-5 py-2.5 bg-white/10 hover:bg-white/20 text-white font-semibold rounded-xl transition"
          @click="ouvrirMessagerie"
        >
          <font-awesome-icon icon="fa-solid fa-comments" />
          Ouvrir la messagerie
        </button>
      </div>

      <!-- Flux local (vignette) -->
      <div class="absolute bottom-4 right-4 w-32 h-44 sm:w-40 sm:h-28 rounded-xl overflow-hidden border-2 border-white/30 shadow-lg bg-black">
        <video ref="videoLocal" autoplay playsinline muted class="w-full h-full object-cover" />
        <div v-if="!cameraActive" class="absolute inset-0 flex items-center justify-center bg-gray-800 text-white/50">
          <font-awesome-icon icon="fa-solid fa-video-slash" />
        </div>
      </div>
    </div>

    <!-- Contrôles -->
    <footer class="flex items-center justify-center gap-3 py-4 shrink-0">
      <button
        type="button"
        class="w-12 h-12 rounded-full flex items-center justify-center transition"
        :class="microActif ? 'bg-white/15 text-white hover:bg-white/25' : 'bg-red-500 text-white'"
        :aria-label="microActif ? 'Couper le micro' : 'Activer le micro'"
        @click="basculerMicro"
      >
        <font-awesome-icon :icon="microActif ? 'fa-solid fa-microphone' : 'fa-solid fa-microphone-slash'" />
      </button>
      <button
        type="button"
        class="w-12 h-12 rounded-full flex items-center justify-center transition"
        :class="cameraActive ? 'bg-white/15 text-white hover:bg-white/25' : 'bg-red-500 text-white'"
        :aria-label="cameraActive ? 'Couper la caméra' : 'Activer la caméra'"
        @click="basculerCamera"
      >
        <font-awesome-icon :icon="cameraActive ? 'fa-solid fa-video' : 'fa-solid fa-video-slash'" />
      </button>
      <button
        type="button"
        class="w-14 h-12 rounded-full bg-red-600 hover:bg-red-700 text-white flex items-center justify-center transition"
        aria-label="Quitter la visioconférence"
        @click="quitter"
      >
        <font-awesome-icon icon="fa-solid fa-phone-slash" />
      </button>
    </footer>
  </div>
</template>

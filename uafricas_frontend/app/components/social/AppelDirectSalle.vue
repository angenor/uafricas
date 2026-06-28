<script setup lang="ts">
import type { Peer, MediaConnection } from 'peerjs'
import type { SalleAppelAPI } from '~/composables/useAppels'

// Salle visio d'un appel direct entre amis (sans rendez-vous). La config P2P est
// fournie d'emblée (prop `salle`) : aucune requête de salle à effectuer ici.
// Logique WebRTC identique à RendezVousSalle (anti-glare + relance).
const props = defineProps<{ salle: SalleAppelAPI }>()

const { finAppel, appelRepondu, raccrocher } = useAppels()
const { demanderOuverture, conversations, listerConversations } = useMessagerie()
const config = useRuntimeConfig()

// Fenêtre flottante déplaçable + redimensionnable (position/taille persistées).
const {
  style: styleFenetre,
  deplace,
  demarrerDeplacement,
  surDeplacement,
  finDeplacement,
  demarrerRedimensionnement,
  surRedimensionnement,
  finRedimensionnement,
} = useFenetreFlottante({ cle: 'appel:fenetre', largeurDefaut: 420, hauteurDefaut: 580, largeurMin: 280, hauteurMin: 380 })

type Etat = 'attente' | 'connexion' | 'connecte' | 'parti' | 'echec' | 'refuse' | 'annule'
const etat = ref<Etat>('attente')
const messageEtat = ref('Connexion en cours…')
const microActif = ref(true)
const cameraActive = ref(true)
const partageEcran = ref(false)

// Plein écran (API native) + panneau de discussion intégré à la fenêtre d'appel.
const racine = ref<HTMLElement | null>(null)
const plein = ref(false)
const chatOuvert = ref(false)

const videoLocal = ref<HTMLVideoElement | null>(null)
const videoDistant = ref<HTMLVideoElement | null>(null)

let peer: Peer | null = null
let appelEnCours: MediaConnection | null = null
let fluxLocal: MediaStream | null = null
let fluxEcran: MediaStream | null = null
let pisteCameraVideo: MediaStreamTrack | null = null
let relanceAppel: ReturnType<typeof setInterval> | null = null

const autre = computed(() => props.salle.autre)
const nomComplet = computed(() => `${autre.value.prenom} ${autre.value.nom}`.trim())
const estConnecte = computed(() => etat.value === 'connecte')
// Affiche le repli messagerie après un échec, un départ ou un refus/annulation.
const enRepli = computed(() => ['echec', 'parti', 'refuse', 'annule'].includes(etat.value))
// Pastille de messages non lus sur le bouton de discussion (0 si le panneau est ouvert).
const nonLusChat = computed(() =>
  chatOuvert.value ? 0 : (conversations.value.find(c => c.ami.id === autre.value.id)?.non_lus ?? 0),
)

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
    if (estConnecte.value) {
      etat.value = 'parti'
      messageEtat.value = `${autre.value.prenom} a quitté l'appel.`
    }
  })
  call.on('error', () => {
    etat.value = 'echec'
    messageEtat.value = 'La connexion vidéo a échoué.'
  })
}

const demarrer = async () => {
  // 1. Flux local (caméra + micro).
  try {
    fluxLocal = await navigator.mediaDevices.getUserMedia({ video: true, audio: true })
    if (videoLocal.value) videoLocal.value.srcObject = fluxLocal
  }
  catch {
    etat.value = 'echec'
    messageEtat.value = "Impossible d'accéder à la caméra ou au micro. Vérifiez les autorisations du navigateur."
    return
  }

  // 2. Connexion PeerJS (cloud public par défaut, surchargeable par env).
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
  peer = new PeerCtor(props.salle.mon_peer_id, opts)

  if (etat.value === 'attente') {
    messageEtat.value = appelRepondu.value
      ? 'Connexion en cours…'
      : `Appel de ${autre.value.prenom}…`
  }

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
    if (props.salle.suis_appelant) {
      const tenter = () => {
        if (!peer || !fluxLocal || estConnecte.value) return
        const call = peer.call(props.salle.pair_peer_id, fluxLocal)
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

// Le distant a accepté : on bascule le message d'attente vers la connexion.
watch(appelRepondu, (v) => {
  if (v && etat.value === 'attente') messageEtat.value = 'Connexion en cours…'
})

// Fin signalée par le serveur (refus / annulation) avant connexion.
watch(finAppel, (raison) => {
  if (!raison || estConnecte.value) return
  arreterRelance()
  etat.value = raison
  messageEtat.value = raison === 'refuse'
    ? `${autre.value.prenom} a refusé l'appel.`
    : `${autre.value.prenom} n'est plus disponible.`
})

const basculerMicro = () => {
  microActif.value = !microActif.value
  fluxLocal?.getAudioTracks().forEach((t) => { t.enabled = microActif.value })
}
const basculerCamera = () => {
  cameraActive.value = !cameraActive.value
  fluxLocal?.getVideoTracks().forEach((t) => { t.enabled = cameraActive.value })
}

// ── Partage d'écran (même connexion PeerJS, remplacement de piste) ──
// On échange la piste vidéo envoyée (caméra → écran) sur l'émetteur WebRTC
// existant via `replaceTrack` : pas de renégociation, le correspondant voit
// le basculement de façon transparente (la piste reçue change de contenu).

/** Émetteur (RTCRtpSender) de la piste vidéo de la connexion active. */
const senderVideo = (): RTCRtpSender | null => {
  const pc = appelEnCours?.peerConnection
  if (!pc) return null
  return pc.getSenders().find(s => s.track?.kind === 'video') ?? null
}

const demarrerPartageEcran = async () => {
  if (!estConnecte.value) return
  const sender = senderVideo()
  if (!sender) return
  let flux: MediaStream
  try {
    flux = await navigator.mediaDevices.getDisplayMedia({ video: true, audio: false })
  }
  catch {
    return // l'utilisateur a annulé le sélecteur de partage
  }
  const piste = flux.getVideoTracks()[0]
  if (!piste) {
    flux.getTracks().forEach(t => t.stop())
    return
  }
  // Mémorise la piste caméra pour la restaurer à l'arrêt du partage.
  pisteCameraVideo = fluxLocal?.getVideoTracks()[0] ?? null
  fluxEcran = flux
  try {
    await sender.replaceTrack(piste)
  }
  catch {
    flux.getTracks().forEach(t => t.stop())
    fluxEcran = null
    return
  }
  // Aperçu local = écran partagé.
  if (videoLocal.value) videoLocal.value.srcObject = flux
  partageEcran.value = true
  // Arrêt via l'UI native du navigateur (« Arrêter le partage »).
  piste.addEventListener('ended', () => { void arreterPartageEcran() })
}

const arreterPartageEcran = async () => {
  if (!partageEcran.value) return
  partageEcran.value = false
  const sender = senderVideo()
  if (sender && pisteCameraVideo) {
    pisteCameraVideo.enabled = cameraActive.value
    try { await sender.replaceTrack(pisteCameraVideo) }
    catch { /* connexion fermée entre-temps */ }
  }
  fluxEcran?.getTracks().forEach(t => t.stop())
  fluxEcran = null
  pisteCameraVideo = null
  // Aperçu local = caméra.
  if (videoLocal.value && fluxLocal) videoLocal.value.srcObject = fluxLocal
}

const basculerPartageEcran = () => {
  if (partageEcran.value) void arreterPartageEcran()
  else void demarrerPartageEcran()
}

const nettoyer = () => {
  arreterRelance()
  appelEnCours?.close()
  appelEnCours = null
  peer?.destroy()
  peer = null
  fluxLocal?.getTracks().forEach(t => t.stop())
  fluxLocal = null
  fluxEcran?.getTracks().forEach(t => t.stop())
  fluxEcran = null
  pisteCameraVideo = null
  partageEcran.value = false
}

const quitter = () => {
  const connecte = estConnecte.value
  nettoyer()
  // raccrocher() vide l'état global → ce composant est démonté par le parent.
  void raccrocher(connecte)
}

const ouvrirMessagerie = () => {
  const cible = autre.value
  nettoyer()
  void raccrocher(true)
  demanderOuverture(cible)
}

// ── Plein écran (API native sur la fenêtre d'appel) ─────────────
const surChangementPleinEcran = () => {
  plein.value = !!document.fullscreenElement && document.fullscreenElement === racine.value
}
const basculerPleinEcran = async () => {
  try {
    if (document.fullscreenElement) await document.exitFullscreen()
    else await racine.value?.requestFullscreen()
  }
  catch {
    // Plein écran refusé ou non supporté : la fenêtre flottante reste utilisable.
  }
}

onMounted(demarrer)
onMounted(() => {
  document.addEventListener('fullscreenchange', surChangementPleinEcran)
  // Charge les conversations pour alimenter la pastille de non-lus du panneau.
  if (conversations.value.length === 0) void listerConversations()
})
onBeforeUnmount(() => {
  document.removeEventListener('fullscreenchange', surChangementPleinEcran)
  nettoyer()
})
</script>

<template>
  <div
    ref="racine"
    :style="styleFenetre"
    class="fixed z-[80] bg-gray-900 flex flex-col overflow-hidden shadow-2xl border border-white/10"
    :class="[deplace ? 'select-none' : '', plein ? 'rounded-none' : 'rounded-2xl']"
  >
    <!-- En-tête (zone de déplacement) -->
    <header
      class="flex items-center justify-between px-4 py-3 text-white/90 shrink-0 touch-none select-none"
      :class="deplace ? 'cursor-grabbing' : 'cursor-grab'"
      title="Glissez pour déplacer la fenêtre"
      @pointerdown="demarrerDeplacement"
      @pointermove="surDeplacement"
      @pointerup="finDeplacement"
      @pointercancel="finDeplacement"
    >
      <div class="min-w-0">
        <p class="font-semibold truncate">Appel · {{ nomComplet }}</p>
        <p class="text-xs text-white/60 truncate">Visioconférence directe</p>
      </div>
      <div class="flex items-center gap-1 shrink-0">
        <button
          type="button"
          class="p-2 hover:bg-white/10 rounded-lg transition"
          :aria-label="plein ? 'Quitter le plein écran' : 'Plein écran'"
          :title="plein ? 'Quitter le plein écran' : 'Plein écran'"
          @click="basculerPleinEcran"
        >
          <font-awesome-icon :icon="plein ? 'fa-solid fa-compress' : 'fa-solid fa-expand'" class="text-lg" />
        </button>
        <button type="button" class="p-2 hover:bg-white/10 rounded-lg transition" aria-label="Fermer" @click="quitter">
          <font-awesome-icon icon="fa-solid fa-xmark" class="text-xl" />
        </button>
      </div>
    </header>

    <!-- Scène vidéo -->
    <div class="relative flex-1 min-h-0">
      <!-- Flux distant (plein cadre) -->
      <video
        ref="videoDistant"
        autoplay
        playsinline
        class="w-full h-full object-contain bg-black"
        :class="estConnecte ? '' : 'opacity-0'"
      />

      <!-- Overlay d'état (hors connecté) -->
      <div
        v-if="!estConnecte"
        class="absolute inset-0 flex flex-col items-center justify-center text-center px-6 text-white"
      >
        <!-- Avatar du correspondant -->
        <div class="mb-5">
          <img
            v-if="autre.photoUrl"
            :src="autre.photoUrl.startsWith('http') ? autre.photoUrl : `${config.public.apiBaseUrl}${autre.photoUrl}`"
            :alt="nomComplet"
            class="w-24 h-24 rounded-full object-cover border-2 border-white/20 shadow-lg"
            :class="(etat === 'attente' || etat === 'connexion') ? 'animate-pulse' : ''"
          >
          <div
            v-else
            class="w-24 h-24 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-3xl font-bold border-2 border-white/20"
            :class="(etat === 'attente' || etat === 'connexion') ? 'animate-pulse' : ''"
          >
            {{ (autre.prenom?.charAt(0) || '') + (autre.nom?.charAt(0) || '') }}
          </div>
        </div>

        <font-awesome-icon
          v-if="etat === 'attente' || etat === 'connexion'"
          icon="fa-solid fa-spinner"
          spin
          class="text-2xl mb-3 text-white/70"
        />
        <font-awesome-icon
          v-else-if="etat === 'echec'"
          icon="fa-solid fa-triangle-exclamation"
          class="text-2xl mb-3 text-amber-400"
        />
        <font-awesome-icon
          v-else
          icon="fa-solid fa-phone-slash"
          class="text-2xl mb-3 text-white/60"
        />
        <p class="text-sm text-white/80 max-w-sm">{{ messageEtat }}</p>

        <!-- Repli messagerie (échec, départ, refus, annulation) -->
        <button
          v-if="enRepli"
          type="button"
          class="mt-5 inline-flex items-center gap-2 px-5 py-2.5 bg-white/10 hover:bg-white/20 text-white font-semibold rounded-xl transition"
          @click="ouvrirMessagerie"
        >
          <font-awesome-icon icon="fa-solid fa-comments" />
          Envoyer un message
        </button>
      </div>

      <!-- Flux local (vignette) — bascule à gauche quand la discussion est ouverte -->
      <div
        class="absolute bottom-4 w-32 h-44 sm:w-40 sm:h-28 rounded-xl overflow-hidden border-2 border-white/30 shadow-lg bg-black transition-all"
        :class="chatOuvert ? 'left-4' : 'right-4'"
      >
        <video ref="videoLocal" autoplay playsinline muted class="w-full h-full" :class="partageEcran ? 'object-contain' : 'object-cover'" />
        <div v-if="!cameraActive && !partageEcran" class="absolute inset-0 flex items-center justify-center bg-gray-800 text-white/50">
          <font-awesome-icon icon="fa-solid fa-video-slash" />
        </div>
        <div v-if="partageEcran" class="absolute top-1 left-1 px-1.5 py-0.5 rounded bg-custom-green/90 text-white text-[10px] font-semibold flex items-center gap-1">
          <font-awesome-icon icon="fa-solid fa-display" />
          Votre écran
        </div>
      </div>

      <!-- Panneau de discussion intégré (tiroir latéral droit) -->
      <Transition name="slide-chat">
        <div
          v-if="chatOuvert"
          class="absolute inset-y-0 right-0 w-full max-w-[22rem] bg-white flex flex-col shadow-2xl border-l border-gray-200 z-30"
        >
          <SocialFenetreConversation :ami="autre" @retour="chatOuvert = false" />
        </div>
      </Transition>
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
        class="relative w-12 h-12 rounded-full flex items-center justify-center transition"
        :class="chatOuvert ? 'bg-custom-chocolat text-white' : 'bg-white/15 text-white hover:bg-white/25'"
        :aria-label="chatOuvert ? 'Fermer la discussion' : 'Ouvrir la discussion'"
        :title="chatOuvert ? 'Fermer la discussion' : 'Discuter avec ' + autre.prenom"
        @click="chatOuvert = !chatOuvert"
      >
        <font-awesome-icon icon="fa-solid fa-comments" />
        <span
          v-if="nonLusChat > 0"
          class="absolute -top-0.5 -right-0.5 min-w-4 h-4 px-1 bg-red-500 text-white text-[10px] font-bold rounded-full flex items-center justify-center border border-gray-900"
        >{{ nonLusChat > 9 ? '9+' : nonLusChat }}</span>
      </button>
      <button
        type="button"
        class="w-12 h-12 rounded-full flex items-center justify-center transition disabled:opacity-40 disabled:cursor-not-allowed"
        :class="partageEcran ? 'bg-custom-green text-white' : 'bg-white/15 text-white hover:bg-white/25'"
        :disabled="!estConnecte"
        :aria-label="partageEcran ? 'Arrêter le partage d’écran' : 'Partager mon écran'"
        :title="!estConnecte ? 'Disponible une fois la connexion établie' : (partageEcran ? 'Arrêter le partage' : 'Partager mon écran')"
        @click="basculerPartageEcran"
      >
        <font-awesome-icon icon="fa-solid fa-display" />
      </button>
      <button
        type="button"
        class="w-14 h-12 rounded-full bg-red-600 hover:bg-red-700 text-white flex items-center justify-center transition"
        aria-label="Raccrocher"
        @click="quitter"
      >
        <font-awesome-icon icon="fa-solid fa-phone-slash" />
      </button>
    </footer>

    <!-- Poignée de redimensionnement (coin bas-droit) — masquée en plein écran -->
    <div
      v-show="!plein"
      class="absolute bottom-0 right-0 w-5 h-5 flex items-end justify-end p-0.5 cursor-nwse-resize touch-none z-10 text-white/40 hover:text-white"
      title="Glissez pour redimensionner"
      @pointerdown="demarrerRedimensionnement"
      @pointermove="surRedimensionnement"
      @pointerup="finRedimensionnement"
      @pointercancel="finRedimensionnement"
    >
      <svg viewBox="0 0 10 10" class="w-2.5 h-2.5" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round">
        <path d="M9 2 L2 9 M9 5 L5 9 M9 8 L8 9" />
      </svg>
    </div>
  </div>
</template>

<style scoped>
@reference "~/assets/css/main.css";

.slide-chat-enter-active,
.slide-chat-leave-active {
  transition: transform 0.25s ease, opacity 0.25s ease;
}
.slide-chat-enter-from,
.slide-chat-leave-to {
  transform: translateX(100%);
  opacity: 0;
}
</style>

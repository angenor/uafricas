<script setup lang="ts">
import type { AppelEntrant } from '~/composables/useAppels'

// Sonnerie d'un appel entrant : carte flottante avec acceptation / refus.
// Tailwind pur (composant public). Une sonnerie sonore légère + vibration
// accompagnent l'affichage tant que la carte est montée.
const props = defineProps<{ appel: AppelEntrant }>()
const emit = defineEmits<{ (e: 'accepter'): void, (e: 'refuser'): void }>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

// Décalage au-dessus de la barre de lecture persistante quand elle est visible.
const { aUnContenu: lectureMediaActive } = useLecteurMedia()

const appelant = computed(() => props.appel.appelant)
const nomComplet = computed(() => `${appelant.value.prenom} ${appelant.value.nom}`.trim())
const photo = computed(() => {
  const url = appelant.value.photoUrl
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
})
const initiaux = computed(() =>
  `${appelant.value.prenom?.charAt(0) || ''}${appelant.value.nom?.charAt(0) || ''}`.toUpperCase())

// ── Sonnerie sonore (Web Audio) + vibration, best-effort ─────────
let ctx: AudioContext | null = null
let minuteur: ReturnType<typeof setInterval> | null = null
let vibration: ReturnType<typeof setInterval> | null = null

const bip = () => {
  if (!ctx) return
  // Deux brèves impulsions (motif « sonnerie »).
  ;[0, 0.4].forEach((decalage) => {
    const osc = ctx!.createOscillator()
    const gain = ctx!.createGain()
    osc.type = 'sine'
    osc.frequency.value = 480
    gain.gain.setValueAtTime(0.0001, ctx!.currentTime + decalage)
    gain.gain.exponentialRampToValueAtTime(0.15, ctx!.currentTime + decalage + 0.05)
    gain.gain.exponentialRampToValueAtTime(0.0001, ctx!.currentTime + decalage + 0.3)
    osc.connect(gain).connect(ctx!.destination)
    osc.start(ctx!.currentTime + decalage)
    osc.stop(ctx!.currentTime + decalage + 0.32)
  })
}

onMounted(() => {
  try {
    const Ctor = window.AudioContext || (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext
    if (Ctor) {
      ctx = new Ctor()
      bip()
      minuteur = setInterval(bip, 2500)
    }
  }
  catch {
    // Politique d'autoplay : la sonnerie sonore est ignorée silencieusement.
  }
  if (typeof navigator !== 'undefined' && 'vibrate' in navigator) {
    const vibrer = () => navigator.vibrate?.([300, 200, 300])
    vibrer()
    vibration = setInterval(vibrer, 2500)
  }
})

onBeforeUnmount(() => {
  if (minuteur) clearInterval(minuteur)
  if (vibration) clearInterval(vibration)
  navigator.vibrate?.(0)
  ctx?.close().catch(() => {})
  ctx = null
})
</script>

<template>
  <div
    class="fixed right-6 z-[75] w-[20rem] max-w-[calc(100vw-3rem)] bg-white rounded-2xl shadow-2xl border border-gray-100 overflow-hidden"
    :class="lectureMediaActive ? 'bottom-46' : 'bottom-24'"
  >
    <!-- Bandeau dégradé -->
    <div class="bg-linear-to-r from-custom-chocolat to-custom-green px-4 py-2.5 flex items-center gap-2 text-white">
      <span class="relative flex h-2.5 w-2.5">
        <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-white/70" />
        <span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-white" />
      </span>
      <font-awesome-icon :icon="appel.video ? 'fa-solid fa-video' : 'fa-solid fa-phone'" class="text-xs" />
      <p class="text-xs font-semibold tracking-wide uppercase">Appel entrant</p>
      <img src="/logos/logo_uafracas.png" alt="UAfricas" class="h-5 w-auto ml-auto bg-white/90 rounded px-1 py-0.5">
    </div>

    <!-- Corps -->
    <div class="px-4 py-4 flex flex-col items-center text-center">
      <img
        v-if="photo"
        :src="photo"
        :alt="nomComplet"
        class="w-16 h-16 rounded-full object-cover border-2 border-custom-green/30 shadow animate-pulse"
      >
      <div
        v-else
        class="w-16 h-16 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-xl font-bold border-2 border-custom-green/30 animate-pulse"
      >
        {{ initiaux }}
      </div>

      <p class="mt-3 font-semibold text-gray-800">{{ nomComplet }}</p>
      <p v-if="appelant.fonction" class="text-xs text-gray-500 truncate max-w-full">{{ appelant.fonction }}</p>
      <p class="text-xs text-gray-400 mt-0.5">{{ appel.video ? 'vous appelle en visio…' : 'vous appelle…' }}</p>

      <!-- Actions -->
      <div class="mt-4 flex items-center justify-center gap-6">
        <button
          type="button"
          class="flex flex-col items-center gap-1 group"
          aria-label="Refuser l'appel"
          @click="emit('refuser')"
        >
          <span class="w-12 h-12 rounded-full bg-red-600 group-hover:bg-red-700 text-white flex items-center justify-center shadow transition">
            <font-awesome-icon icon="fa-solid fa-phone-slash" />
          </span>
          <span class="text-[11px] text-gray-500">Refuser</span>
        </button>
        <button
          type="button"
          class="flex flex-col items-center gap-1 group"
          aria-label="Accepter l'appel"
          @click="emit('accepter')"
        >
          <span class="w-12 h-12 rounded-full bg-custom-green group-hover:brightness-110 text-white flex items-center justify-center shadow transition animate-bounce">
            <font-awesome-icon icon="fa-solid fa-phone" />
          </span>
          <span class="text-[11px] text-gray-500">Accepter</span>
        </button>
      </div>
    </div>
  </div>
</template>

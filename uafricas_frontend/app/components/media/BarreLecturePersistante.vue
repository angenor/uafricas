<script setup lang="ts">
/**
 * Barre de lecture ancrée en bas de l'écran (FR-017).
 *
 * Montée dans le LAYOUT, hors du `<slot/>` : c'est ce placement, et lui seul 
 * qui fait survivre l'écoute au changement de page. Un lecteur monté dans une
 * page est démonté à la navigation, et le son se coupe.
 *
 * L'élément `<audio>` vit ici et nulle part ailleurs : un seul flux à la fois
 * (FR-018).
 */
const {
  contenu,
  enLecture,
  volume,
  coupe,
  pause,
  basculerLecture,
  arreter,
  basculerSon,
  definirVolume,
} = useLecteurMedia()

const audio = ref<HTMLAudioElement | null>(null)
const position = ref(0)
const duree = ref(0)
const enErreur = ref(false)

/** Un direct n'a ni fin ni position : la timeline n'aurait aucun sens. */
const afficheTimeline = computed(() => !contenu.value?.estDirect && duree.value > 0)

const formaterDuree = (secondes: number) => {
  if (!Number.isFinite(secondes) || secondes < 0) return '0:00'
  const m = Math.floor(secondes / 60)
  const s = Math.floor(secondes % 60)
  return `${m}:${String(s).padStart(2, '0')}`
}

const deplacer = (evenement: Event) => {
  const valeur = Number((evenement.target as HTMLInputElement).value)
  if (audio.value) audio.value.currentTime = valeur
}

// Synchronise l'élément audio avec l'état partagé.
watch(enLecture, async (doitJouer) => {
  await nextTick()
  if (!audio.value) return
  if (doitJouer) {
    try {
      await audio.value.play()
    }
    catch {
      // Le navigateur peut refuser la lecture hors geste utilisateur : on
      // reflète le refus dans l'état plutôt que de mentir sur l'affichage.
      pause()
    }
  }
  else {
    audio.value.pause()
  }
})

watch(contenu, async () => {
  enErreur.value = false
  position.value = 0
  duree.value = 0
  await nextTick()
  if (audio.value && enLecture.value) {
    audio.value.play().catch(() => pause())
  }
})

watch(volume, (v) => {
  if (audio.value) audio.value.volume = v
})

watch(coupe, (c) => {
  if (audio.value) audio.value.muted = c
})

onMounted(() => {
  if (audio.value) {
    audio.value.volume = volume.value
    audio.value.muted = coupe.value
  }
})
</script>

<template>
  <!-- z-40 : sous la NavBar (z-50) et sous les fenêtres flottantes de
       messagerie et d'appel, qui doivent rester au-dessus. -->
  <div
    v-if="contenu"
    class="fixed bottom-0 inset-x-0 z-40 bg-neutral-950/95 backdrop-blur-sm border-t border-white/10 text-white"
    role="region"
    aria-label="Lecteur audio"
  >
    <audio
      ref="audio"
      :src="contenu.url"
      preload="metadata"
      @timeupdate="position = audio?.currentTime ?? 0"
      @loadedmetadata="duree = audio?.duration ?? 0"
      @ended="arreter()"
      @error="enErreur = true; pause()"
    />

    <div class="max-w-6xl mx-auto px-4 py-3 flex items-center gap-4">
      <img
        v-if="contenu.image"
        :src="contenu.image"
        :alt="contenu.titre"
        class="h-12 w-12 rounded object-cover shrink-0 hidden sm:block"
      >

      <div class="min-w-0 flex-1">
        <p class="text-sm font-semibold truncate">{{ contenu.titre }}</p>
        <p class="text-xs text-white/70 truncate">
          <span v-if="contenu.estDirect" class="text-red-400 font-semibold">● En direct</span>
          <span v-if="contenu.estDirect && contenu.support"> · </span>
          <span v-if="contenu.support">{{ contenu.support }}</span>
          <span v-if="enErreur" class="text-red-400"> · lecture impossible</span>
        </p>
      </div>

      <button
        type="button"
        class="h-11 w-11 shrink-0 rounded-full bg-white text-black flex items-center justify-center hover:bg-gray-200 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-orange"
        :aria-label="enLecture ? 'Mettre en pause' : 'Reprendre la lecture'"
        @click="basculerLecture()"
      >
        <font-awesome-icon :icon="['fas', enLecture ? 'pause' : 'play']" />
      </button>

      <div v-if="afficheTimeline" class="hidden md:flex items-center gap-2 w-64">
        <span class="text-[11px] text-white/70 tabular-nums w-9 text-right">{{ formaterDuree(position) }}</span>
        <input
          type="range"
          min="0"
          :max="duree"
          :value="position"
          class="flex-1 accent-yellow-400 cursor-pointer"
          aria-label="Position de lecture"
          @input="deplacer"
        >
        <span class="text-[11px] text-white/70 tabular-nums w-9">{{ formaterDuree(duree) }}</span>
      </div>

      <div class="hidden sm:flex items-center gap-2 shrink-0">
        <button
          type="button"
          class="h-9 w-9 rounded-full hover:bg-white/10 flex items-center justify-center transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-orange"
          :aria-label="coupe ? 'Rétablir le son' : 'Couper le son'"
          :aria-pressed="coupe"
          @click="basculerSon"
        >
          <font-awesome-icon :icon="['fas', coupe || volume === 0 ? 'volume-xmark' : 'volume-high']" />
        </button>
        <input
          type="range"
          min="0"
          max="1"
          step="0.05"
          :value="coupe ? 0 : volume"
          class="w-20 accent-yellow-400 cursor-pointer"
          aria-label="Volume"
          @input="definirVolume(Number(($event.target as HTMLInputElement).value))"
        >
      </div>

      <button
        type="button"
        class="h-9 w-9 shrink-0 rounded-full hover:bg-white/10 flex items-center justify-center text-white/70 hover:text-white transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-orange"
        aria-label="Fermer le lecteur"
        @click="arreter"
      >
        <font-awesome-icon :icon="['fas', 'xmark']" />
      </button>
    </div>
  </div>
</template>

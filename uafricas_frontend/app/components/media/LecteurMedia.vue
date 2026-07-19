<script setup lang="ts">
/**
 * Lecteur média unique des pages Radio et Télé.
 *
 * Route la lecture selon la SOURCE du média, ce qui n'était fait nulle part :
 *   • lien YouTube reconnu → `<iframe>` d'intégration ;
 *   • fichier servi par la plateforme → `<video>` ou `<audio>` natif ;
 *   • lien tiers non reconnu → aucun lecteur, mais un renvoi explicite vers la
 *     source. Une URL YouTube placée dans un `<video>` ne produit AUCUNE erreur
 *     visible : le cadre reste noir et muet. C'est le bug que masquait jusqu'ici
 *     le contenu provisoire codé en dur de la page Télé (FR-010, FR-056).
 */
import { youtubeEmbedUrl, estMediaJouable } from '~/utils/media'

const props = withDefaults(defineProps<{
  url: string | null | undefined
  type?: 'video' | 'audio'
  /** Démarre dès le montage. Le son est alors coupé : sans quoi les
   *  navigateurs refusent purement et simplement la lecture automatique. */
  lectureAuto?: boolean
  sonCoupe?: boolean
  boucle?: boolean
  /** Affiche les contrôles natifs. Les vues qui pilotent le lecteur
   *  elles-mêmes (vedette plein écran, barre persistante) les masquent. */
  controles?: boolean
  poster?: string | null
  volume?: number
  titre?: string | null
}>(), {
  type: 'video',
  lectureAuto: false,
  sonCoupe: false,
  boucle: false,
  controles: true,
  poster: null,
  volume: 0.8,
  titre: null,
})

const emit = defineEmits<{
  lecture: [positionMs: number]
  fin: []
  erreur: [message: string]
}>()

const element = ref<HTMLMediaElement | null>(null)

const urlEmbed = computed(() => youtubeEmbedUrl(props.url))
const estYoutube = computed(() => urlEmbed.value !== null)
const estJouable = computed(() => estMediaJouable(props.url))

/**
 * Paramètres d'intégration YouTube. `mute=1` est indispensable dès que la
 * lecture automatique est demandée — c'est la condition posée par les
 * navigateurs pour l'autoriser.
 */
const srcIframe = computed(() => {
  if (!urlEmbed.value) return ''
  const params = new URLSearchParams({
    autoplay: props.lectureAuto ? '1' : '0',
    mute: props.lectureAuto || props.sonCoupe ? '1' : '0',
    loop: props.boucle ? '1' : '0',
    controls: props.controles ? '1' : '0',
    rel: '0',
    modestbranding: '1',
    playsinline: '1',
  })
  // Le bouclage YouTube exige que la playlist reprenne l'identifiant de la vidéo.
  if (props.boucle) {
    const id = urlEmbed.value.split('/').pop()
    if (id) params.set('playlist', id)
  }
  return `${urlEmbed.value}?${params.toString()}`
})

// ── Pilotage du lecteur natif ────────────────────────────────────────
// Exposé au parent : la vedette plein écran et la barre de lecture persistante
// commandent le lecteur sans le posséder.

const lire = async () => {
  if (!element.value) return
  try {
    await element.value.play()
  } catch {
    // Lecture automatique refusée par le navigateur — cas normal, pas une
    // erreur à remonter à l'utilisateur.
  }
}

const pause = () => element.value?.pause()

const seek = (positionMs: number) => {
  if (element.value) element.value.currentTime = positionMs / 1000
}

defineExpose({ lire, pause, seek, element })

watch(() => props.volume, (v) => {
  if (element.value) element.value.volume = Math.max(0, Math.min(1, v))
})

watch(() => props.sonCoupe, (coupe) => {
  if (element.value) element.value.muted = coupe
})

watch(() => props.url, () => {
  // Changer de source relance la lecture si elle était demandée.
  if (props.lectureAuto) nextTick(lire)
})

onMounted(() => {
  if (!element.value) return
  element.value.volume = Math.max(0, Math.min(1, props.volume))
  element.value.muted = props.sonCoupe || props.lectureAuto
  if (props.lectureAuto) lire()
})

const surTempsEcoule = () => {
  if (element.value) emit('lecture', element.value.currentTime * 1000)
}
</script>

<template>
  <!-- Média tiers reconnu : lecteur du service d'origine -->
  <iframe
    v-if="estYoutube"
    :src="srcIframe"
    :title="titre ?? 'Lecteur vidéo'"
    class="w-full h-full border-0"
    allow="autoplay; encrypted-media; picture-in-picture; fullscreen"
    allowfullscreen
    loading="lazy"
  />

  <!-- Fichier hébergé par la plateforme : lecteur natif -->
  <video
    v-else-if="estJouable && type === 'video'"
    ref="element"
    :src="url ?? undefined"
    :poster="poster ?? undefined"
    :controls="controles"
    :loop="boucle"
    :muted="sonCoupe || lectureAuto"
    playsinline
    preload="metadata"
    class="w-full h-full object-cover"
    @timeupdate="surTempsEcoule"
    @ended="emit('fin')"
    @error="emit('erreur', 'La lecture de cette vidéo a échoué.')"
  />

  <audio
    v-else-if="estJouable"
    ref="element"
    :src="url ?? undefined"
    :controls="controles"
    :loop="boucle"
    :muted="sonCoupe"
    preload="metadata"
    class="w-full"
    @timeupdate="surTempsEcoule"
    @ended="emit('fin')"
    @error="emit('erreur', 'La lecture de ce contenu a échoué.')"
  />

  <!-- Repli : source non lisible ici. Mieux vaut le dire que d'afficher un
       cadre noir dont le visiteur ne saurait pas quoi penser. -->
  <div
    v-else
    class="w-full h-full min-h-40 flex flex-col items-center justify-center gap-3 bg-neutral-900 text-neutral-300 px-6 text-center"
  >
    <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="text-2xl text-yellow-400" />
    <p class="text-sm">
      {{ url ? 'Ce contenu est hébergé sur un service que nous ne savons pas lire ici.' : 'Aucun média n’est associé à ce contenu.' }}
    </p>
    <a
      v-if="url"
      :href="url"
      target="_blank"
      rel="noopener noreferrer"
      class="text-sm font-semibold text-yellow-400 hover:text-yellow-300 underline underline-offset-4"
    >
      Ouvrir la source
    </a>
  </div>
</template>

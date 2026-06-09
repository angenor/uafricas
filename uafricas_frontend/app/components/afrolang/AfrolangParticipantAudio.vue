<script setup lang="ts">
// Élément <audio> d'un participant DISTANT, rendu dans un conteneur stable de
// AfrolangRoom (et non dans la tuile vidéo). Découpler l'audio du layout évite
// que le son ne se coupe lors des changements d'affichage (partage d'écran,
// spotlight, ré-agencement de la grille, mute/unmute) — bug « le partageur
// n'entend plus son interlocuteur ».
const props = defineProps<{
  track: MediaStreamTrack | null
}>()

const el = ref<HTMLAudioElement | null>(null)

const attacher = () => {
  const audio = el.value
  if (!audio) return
  if (props.track) {
    audio.srcObject = new MediaStream([props.track])
    // Autoplay : le geste utilisateur (rejoindre la salle) est déjà acquis.
    void audio.play?.().catch(() => { /* autoplay éventuellement différé */ })
  }
  else {
    audio.srcObject = null
  }
}

watch(() => props.track, () => nextTick(attacher))
onMounted(() => nextTick(attacher))
</script>

<template>
  <audio ref="el" autoplay playsinline />
</template>

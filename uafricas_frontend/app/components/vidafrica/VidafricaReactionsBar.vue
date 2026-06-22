<script setup lang="ts">
import type { VideoAfrica } from '~/composables/useVidafrica'

const props = defineProps<{
  video: VideoAfrica
  peutInteragir: boolean
}>()

const { reagirVideo } = useVidafrica()

// État local amorcé depuis la vidéo (mis à jour après chaque action).
const nombreLikes = ref(props.video.nombreLikes)
const nombreDislikes = ref(props.video.nombreDislikes)
const nombrePartages = ref(props.video.nombrePartages)
const maReaction = ref<'like' | 'dislike' | null>(props.video.maReaction)
const reactionEnCours = ref(false)
const showPartage = ref(false)

// Resynchroniser si la vidéo est rechargée (ex. JWT dispo côté client).
watch(() => props.video, (v) => {
  nombreLikes.value = v.nombreLikes
  nombreDislikes.value = v.nombreDislikes
  nombrePartages.value = v.nombrePartages
  maReaction.value = v.maReaction
})

const reagir = async (type: 'like' | 'dislike') => {
  if (!props.peutInteragir || reactionEnCours.value) return
  reactionEnCours.value = true
  const res = await reagirVideo(props.video.id, type)
  if (res) {
    nombreLikes.value = res.nombreLikes
    nombreDislikes.value = res.nombreDislikes
    maReaction.value = res.maReaction
  }
  reactionEnCours.value = false
}

const onPartage = () => {
  nombrePartages.value += 1
  showPartage.value = false
}
</script>

<template>
  <div>
    <div class="flex flex-wrap items-center gap-3">
      <!-- J'aime -->
      <button
        type="button"
        :disabled="!peutInteragir || reactionEnCours"
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl border transition disabled:opacity-60"
        :class="maReaction === 'like'
          ? 'bg-custom-green text-white border-custom-green'
          : 'border-gray-200 text-gray-600 hover:border-custom-green hover:text-custom-green'"
        @click="reagir('like')"
      >
        <font-awesome-icon icon="thumbs-up" />
        J'aime
        <span>{{ nombreLikes }}</span>
      </button>

      <!-- Je n'aime pas -->
      <button
        type="button"
        :disabled="!peutInteragir || reactionEnCours"
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl border transition disabled:opacity-60"
        :class="maReaction === 'dislike'
          ? 'bg-gray-700 text-white border-gray-700'
          : 'border-gray-200 text-gray-600 hover:border-gray-400 hover:text-gray-800'"
        @click="reagir('dislike')"
      >
        <font-awesome-icon icon="thumbs-down" />
        Je n'aime pas
        <span>{{ nombreDislikes }}</span>
      </button>

      <!-- Partager -->
      <button
        type="button"
        :disabled="!peutInteragir"
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl border border-gray-200 text-gray-600 hover:border-custom-chocolat hover:text-custom-chocolat transition disabled:opacity-60"
        @click="showPartage = true"
      >
        <font-awesome-icon icon="share-nodes" />
        Partager
        <span>{{ nombrePartages }}</span>
      </button>
    </div>

    <p v-if="!peutInteragir" class="mt-2 text-xs text-gray-400">
      <NuxtLink to="/login" class="text-custom-chocolat hover:underline">Connectez-vous</NuxtLink>
      pour aimer ou partager cette vidéo.
    </p>

    <!-- Modale de partage -->
    <VidafricaPartagerModal v-model="showPartage" :video="video" @partage="onPartage" />
  </div>
</template>

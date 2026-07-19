<script setup lang="ts">
// Réactions, commentaires et partages d'un contenu média (US3).
// Modèle : opportunite-afrique/ReactionsBar.vue, enrichi des deux compteurs
// annexes — un contenu a plus d'affordances qu'un sous-objet afripulse.

import { useMediaSocial, type TypeMedia, type TypeReaction } from '~/composables/useMediaSocial'

const props = defineProps<{
  typeMedia: TypeMedia
  mediaId: string
  nombreLikes: number
  nombreDislikes: number
  maReaction: TypeReaction | null
  /** Affichés à titre indicatif ; les boutons délèguent au parent. */
  nombreCommentaires?: number
  nombrePartages?: number
  /** Disposition compacte, pour les cartes de rangée. */
  compact?: boolean
}>()

const emit = defineEmits<{
  (e: 'require-login'): void
  (e: 'commenter'): void
  (e: 'partager'): void
}>()

const { reagir, estConnecte } = useMediaSocial()

const likes = ref(props.nombreLikes)
const dislikes = ref(props.nombreDislikes)
const maReaction = ref<TypeReaction | null>(props.maReaction)
const enCours = ref(false)

// Resynchronise si le parent recharge la donnée — la mise à jour optimiste
// ci-dessous serait sinon écrasée par un état périmé.
watch(
  () => [props.nombreLikes, props.nombreDislikes, props.maReaction] as const,
  ([l, d, r]) => {
    likes.value = l
    dislikes.value = d
    maReaction.value = r
  },
)

const basculer = async (type: TypeReaction) => {
  if (!estConnecte()) {
    emit('require-login')
    return
  }
  if (enCours.value) return
  enCours.value = true

  // Mise à jour optimiste : le compteur réagit au clic, puis se resynchronise
  // sur la réponse du serveur — qui fait foi.
  const ancienne = maReaction.value
  const retrait = ancienne === type
  maReaction.value = retrait ? null : type
  if (type === 'like') likes.value += retrait ? -1 : 1
  else dislikes.value += retrait ? -1 : 1
  if (ancienne && ancienne !== type) {
    if (ancienne === 'like') likes.value -= 1
    else dislikes.value -= 1
  }

  const etat = await reagir(props.typeMedia, props.mediaId, type)
  if (etat) {
    likes.value = etat.nombreLikes
    dislikes.value = etat.nombreDislikes
    maReaction.value = etat.maReaction
  }
  else {
    // Échec : on rétablit l'état d'avant le clic plutôt que de laisser un
    // compteur mensonger à l'écran.
    likes.value = props.nombreLikes
    dislikes.value = props.nombreDislikes
    maReaction.value = props.maReaction
  }
  enCours.value = false
}
</script>

<template>
  <div class="flex items-center gap-2 sm:gap-3">
    <button
      type="button"
      :disabled="enCours"
      :aria-pressed="maReaction === 'like'"
      aria-label="J'aime"
      class="flex items-center justify-center gap-2 rounded-lg border px-4 font-medium transition-colors cursor-pointer disabled:opacity-60"
      :class="[
        compact ? 'py-1.5 text-sm' : 'py-2.5',
        maReaction === 'like'
          ? 'bg-custom-green text-white border-custom-green'
          : 'bg-white/10 text-white border-white/25 hover:bg-white/20',
      ]"
      @click="basculer('like')"
    >
      <font-awesome-icon :icon="['fas', 'thumbs-up']" class="w-4 h-4" />
      <span>{{ likes }}</span>
    </button>

    <button
      type="button"
      :disabled="enCours"
      :aria-pressed="maReaction === 'dislike'"
      aria-label="Je n'aime pas"
      class="flex items-center justify-center gap-2 rounded-lg border px-4 font-medium transition-colors cursor-pointer disabled:opacity-60"
      :class="[
        compact ? 'py-1.5 text-sm' : 'py-2.5',
        maReaction === 'dislike'
          ? 'bg-red-500 text-white border-red-500'
          : 'bg-white/10 text-white border-white/25 hover:bg-white/20',
      ]"
      @click="basculer('dislike')"
    >
      <font-awesome-icon :icon="['fas', 'thumbs-down']" class="w-4 h-4" />
      <span>{{ dislikes }}</span>
    </button>

    <button
      type="button"
      aria-label="Commenter"
      class="flex items-center justify-center gap-2 rounded-lg border border-white/25 bg-white/10 px-4 font-medium text-white transition-colors cursor-pointer hover:bg-white/20"
      :class="compact ? 'py-1.5 text-sm' : 'py-2.5'"
      @click="emit('commenter')"
    >
      <font-awesome-icon :icon="['fas', 'comment']" class="w-4 h-4" />
      <span>{{ nombreCommentaires ?? 0 }}</span>
    </button>

    <button
      type="button"
      aria-label="Partager"
      class="flex items-center justify-center gap-2 rounded-lg border border-white/25 bg-white/10 px-4 font-medium text-white transition-colors cursor-pointer hover:bg-white/20"
      :class="compact ? 'py-1.5 text-sm' : 'py-2.5'"
      @click="emit('partager')"
    >
      <font-awesome-icon :icon="['fas', 'share-nodes']" class="w-4 h-4" />
      <span>{{ nombrePartages ?? 0 }}</span>
    </button>
  </div>
</template>

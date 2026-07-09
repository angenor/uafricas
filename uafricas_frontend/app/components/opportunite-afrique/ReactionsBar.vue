<script setup lang="ts">
import { useOpportuniteAfrique, type TypeObjetElement } from '~/composables/useOpportuniteAfrique'

const props = defineProps<{
  typeObjet: TypeObjetElement
  objetId: string
  nombreLikes: number
  nombreDislikes: number
  maReaction: 'like' | 'dislike' | null
  estAuthentifie: boolean
  /** Disposition compacte (icônes plus petites) */
  compact?: boolean
}>()

const emit = defineEmits<{
  (e: 'require-login'): void
}>()

const { reagirElement } = useOpportuniteAfrique()

const likes = ref(props.nombreLikes)
const dislikes = ref(props.nombreDislikes)
const maReaction = ref<'like' | 'dislike' | null>(props.maReaction)
const enCours = ref(false)

// Resynchronise si le parent recharge la donnée.
watch(() => [props.nombreLikes, props.nombreDislikes, props.maReaction] as const, ([l, d, r]) => {
  likes.value = l
  dislikes.value = d
  maReaction.value = r
})

const basculer = async (type: 'like' | 'dislike') => {
  if (!props.estAuthentifie) {
    emit('require-login')
    return
  }
  if (enCours.value) return
  enCours.value = true
  const etat = await reagirElement(props.typeObjet, props.objetId, type)
  if (etat) {
    likes.value = etat.nombre_likes
    dislikes.value = etat.nombre_dislikes
    maReaction.value = etat.ma_reaction
  }
  enCours.value = false
}
</script>

<template>
  <div class="flex items-center gap-3">
    <button
      type="button"
      :disabled="enCours"
      class="flex-1 flex items-center justify-center gap-2 rounded-lg border px-4 font-medium transition-colors cursor-pointer disabled:opacity-60"
      :class="[
        compact ? 'py-2 text-sm' : 'py-2.5',
        maReaction === 'like'
          ? 'bg-custom-green text-white border-custom-green'
          : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-50',
      ]"
      @click="basculer('like')"
    >
      <font-awesome-icon :icon="['fas', 'thumbs-up']" class="w-4 h-4" />
      <span>{{ likes }}</span>
    </button>
    <button
      type="button"
      :disabled="enCours"
      class="flex-1 flex items-center justify-center gap-2 rounded-lg border px-4 font-medium transition-colors cursor-pointer disabled:opacity-60"
      :class="[
        compact ? 'py-2 text-sm' : 'py-2.5',
        maReaction === 'dislike'
          ? 'bg-red-500 text-white border-red-500'
          : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-50',
      ]"
      @click="basculer('dislike')"
    >
      <font-awesome-icon :icon="['fas', 'thumbs-down']" class="w-4 h-4" />
      <span>{{ dislikes }}</span>
    </button>
  </div>
</template>

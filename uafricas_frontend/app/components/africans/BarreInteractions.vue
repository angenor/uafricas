<template>
  <footer class="flex flex-wrap items-center gap-x-6 gap-y-2 px-4 py-3 text-[12px]/[1.4] text-af-corps">
    <button
      type="button"
      class="flex items-center gap-2 transition hover:text-af-chocolat"
      :class="jaime && 'text-af-vert'"
      :aria-pressed="jaime"
      @click="$emit('jaime')"
    >
      <font-awesome-icon icon="fa-solid fa-thumbs-up" />
      {{ formater(likes) }} Likes
    </button>

    <button
      type="button"
      class="flex items-center gap-2 transition hover:text-af-chocolat"
      @click="$emit('commenter')"
    >
      <font-awesome-icon icon="fa-solid fa-comment" />
      {{ formater(commentaires) }} Commentaires
    </button>

    <button
      type="button"
      class="flex items-center gap-2 transition hover:text-af-chocolat"
      @click="$emit('partager')"
    >
      <font-awesome-icon icon="fa-solid fa-share-nodes" />
      {{ formater(partages) }} Partages
    </button>

    <span v-if="quand" class="ml-auto text-af-atone italic">{{ quand }}</span>
  </footer>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  likes?: number
  commentaires?: number
  partages?: number
  /** Horodatage déjà mis en forme par l'appelant, ex. « il y a 2h ». */
  quand?: string
  jaime?: boolean
}>(), { likes: 0, commentaires: 0, partages: 0, jaime: false })

defineEmits<{ jaime: [], commenter: [], partager: [] }>()

/**
 * La maquette écrit « 25k » et non « 25 000 ». On abrège au-delà du millier,
 * en tronquant plutôt qu'en arrondissant : afficher 25k pour 25 900 est admis,
 * afficher 26k pour 25 500 ferait mentir un compteur que l'utilisateur voit
 * s'incrémenter d'une unité à la fois.
 */
function formater(n: number): string {
  if (n < 1000) return String(n)
  if (n < 1_000_000) return `${Math.floor(n / 1000)}k`
  return `${Math.floor(n / 100_000) / 10}M`
}
</script>

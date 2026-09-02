<template>
  <footer class="flex flex-wrap items-center gap-x-6 gap-y-2 px-4 py-3 text-[12px]/[1.4] text-af-corps">
    <button
      v-if="actions.includes('jaime')"
      type="button"
      class="flex items-center gap-2 transition hover:text-af-chocolat"
      :class="jaime && 'text-af-vert'"
      :aria-pressed="jaime"
      @click="$emit('jaime')"
    >
      <font-awesome-icon icon="fa-solid fa-thumbs-up" />
      {{ likes === undefined ? "J'aime" : `${formater(likes)} ${accord(likes, "Like")}` }}
    </button>

    <button
      v-if="actions.includes('commenter')"
      type="button"
      class="flex items-center gap-2 transition hover:text-af-chocolat"
      @click="$emit('commenter')"
    >
      <font-awesome-icon icon="fa-solid fa-comment" />
      {{ commentaires === undefined ? 'Commenter' : `${formater(commentaires)} ${accord(commentaires, 'Commentaire')}` }}
    </button>

    <button
      v-if="actions.includes('partager')"
      type="button"
      class="flex items-center gap-2 transition hover:text-af-chocolat"
      @click="$emit('partager')"
    >
      <font-awesome-icon icon="fa-solid fa-share-nodes" />
      {{ partages === undefined ? 'Partager' : `${formater(partages)} ${accord(partages, 'Partage')}` }}
    </button>

    <!-- Actions propres à l'appelant : je n'aime pas, vues, cadeau… La maquette
         n'en montre aucune, mais les modules qui les portent ne peuvent pas les
         perdre en passant sur cette barre. -->
    <slot />

    <span v-if="quand" class="ml-auto text-af-atone italic">{{ quand }}</span>
  </footer>
</template>

<script setup lang="ts">
export type ActionInteraction = 'jaime' | 'commenter' | 'partager'

withDefaults(defineProps<{
  /**
   * Les trois compteurs sont optionnels, et leur ABSENCE a un sens : le module
   * n'enregistre pas cette grandeur. Le bouton porte alors le verbe seul
   * (« Partager ») : un « 0 Partages » figé à zéro pour toujours ferait mentir
   * un compteur que l'utilisateur croit voir vivre.
   */
  likes?: number
  commentaires?: number
  partages?: number
  /**
   * Boutons réellement rendus. Un module sans commentaires (Vidafrica) doit
   * pouvoir retirer le bouton, pas seulement son compteur : proposer
   * « Commenter » là où rien ne recueille de commentaire est une impasse.
   */
  actions?: ActionInteraction[]
  /** Horodatage déjà mis en forme par l'appelant, ex. « il y a 2h ». */
  quand?: string
  jaime?: boolean
}>(), {
  likes: 0,
  commentaires: 0,
  jaime: false,
  actions: () => ['jaime', 'commenter', 'partager'],
})

defineEmits<{ jaime: [], commenter: [], partager: [] }>()

/**
 * La maquette écrit « 25k » et non « 25 000 ». On abrège au-delà du millier,
 * en tronquant plutôt qu'en arrondissant : afficher 25k pour 25 900 est admis,
 * afficher 26k pour 25 500 ferait mentir un compteur que l'utilisateur voit
 * s'incrémenter d'une unité à la fois.
 */
/** « 1 Likes » se lisait sur toute carte à un seul j'aime. */
function accord(n: number, mot: string): string {
  return n > 1 ? `${mot}s` : mot
}

function formater(n: number): string {
  if (n < 1000) return String(n)
  if (n < 1_000_000) return `${Math.floor(n / 1000)}k`
  return `${Math.floor(n / 100_000) / 10}M`
}
</script>

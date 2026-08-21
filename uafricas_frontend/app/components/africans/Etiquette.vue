<template>
  <span
    class="inline-flex items-center rounded px-3 py-1 text-[12px]/[1.4]"
    :class="ton === 'gris' ? 'bg-af-bordure text-af-encre' : 'bg-af-vert text-white font-bold'"
  >
    <slot />
  </span>
</template>

<script setup lang="ts">
/**
 * Deux emplois distincts dans la maquette, volontairement réunis ici parce
 * qu'ils ne diffèrent que par le ton :
 *  - `gris`  : étiquettes descriptives en série (Collègue, Homme, Abidjan…)
 *  - `vert`  : badge de catégorie ou de région, un seul par carte
 *
 * Le vert n'est PAS interchangeable avec le chocolat : dans l'application il
 * marque, il n'agit pas. Un badge vert cliquable serait une faute de grammaire
 * visuelle — c'est l'ambiguïté relevée sur la page publique, à ne pas propager.
 */
withDefaults(defineProps<{ ton?: 'gris' | 'vert' }>(), { ton: 'gris' })
</script>

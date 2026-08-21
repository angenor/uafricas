<template>
  <img
    v-if="src"
    :src="src"
    :alt="nom"
    class="shrink-0 rounded-full object-cover"
    :style="dimension"
  />
  <!-- Repli sur les initiales plutôt que sur une silhouette générique : dans un
       fil, une colonne d'icônes identiques ne distingue plus les auteurs. -->
  <span
    v-else
    class="grid shrink-0 place-items-center rounded-full bg-af-chocolat/15 font-bold text-af-chocolat"
    :style="[dimension, { fontSize: `${Math.round(taille * 0.36)}px` }]"
    :title="nom"
    aria-hidden="true"
  >{{ initiales }}</span>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  nom: string
  src?: string | null
  /** 44 dans la barre supérieure et le fil, 32 en liste, 24 en groupe empilé. */
  taille?: number
}>(), { taille: 44 })

const dimension = computed(() => ({ width: `${props.taille}px`, height: `${props.taille}px` }))

const initiales = computed(() =>
  props.nom
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map(m => m[0]?.toUpperCase() ?? '')
    .join(''),
)
</script>

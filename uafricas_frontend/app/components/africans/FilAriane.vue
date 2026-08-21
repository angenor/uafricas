<template>
  <!-- Barre de contexte : fil d'Ariane à gauche, contenu libre au centre,
       action principale à droite, filet de séparation en dessous. -->
  <div class="flex flex-wrap items-center gap-x-6 gap-y-4 py-6">
    <div class="flex items-center gap-2 text-base font-bold">
      <font-awesome-icon icon="fa-solid fa-chevron-right" class="size-6 text-af-corps" />
      <template v-for="(segment, i) in segments" :key="segment.libelle">
        <NuxtLink
          v-if="segment.vers && i < segments.length - 1"
          :to="segment.vers"
          class="text-af-encre transition hover:text-af-chocolat"
        >{{ segment.libelle }}</NuxtLink>
        <span v-else :class="i === segments.length - 1 ? 'text-af-atone' : 'text-af-encre'">
          {{ segment.libelle }}
        </span>
        <span v-if="i < segments.length - 1" class="text-af-encre">/</span>
      </template>
    </div>

    <div v-if="$slots.centre" class="mx-auto"><slot name="centre" /></div>
    <div v-if="$slots.action" class="ml-auto"><slot name="action" /></div>
  </div>
  <hr class="border-af-bordure" />
</template>

<script setup lang="ts">
defineProps<{
  segments: Array<{ libelle: string, vers?: string }>
}>()
</script>

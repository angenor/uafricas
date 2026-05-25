<template>
  <!-- Hero compact (titre seul : pas de sous-titre disponible) -->
  <div class="relative w-full bg-font-baniere-ethnie bg-cover bg-center">
    <div class="absolute inset-0 bg-custom-chocolat/50"></div>

    <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
      <div class="flex items-center justify-center min-h-10 md:min-h-12">
        <h1 class="text-white font-semibold text-2xl md:text-3xl">{{ titre }}</h1>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  titre: string
}>()
</script>

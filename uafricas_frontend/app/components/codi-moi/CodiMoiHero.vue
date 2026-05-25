<template>
  <!-- Hero compact : titre ↔ sous-titre en crossfade au survol -->
  <div class="group relative bg-cover bg-center bg-font-baniere-ethnie">
    <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/80 to-black/70"></div>

    <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
      <!-- Conteneur fixe : le titre et le sous-titre se superposent (crossfade au survol) -->
      <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
        <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
          {{ titre }}
        </h1>
        <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
          {{ sousTitre }}
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  titre: string
  sousTitre: string
}>()
</script>

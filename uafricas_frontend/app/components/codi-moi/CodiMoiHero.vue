<template>
  <div class="relative h-80 bg-cover bg-center bg-font-baniere-ethnie">
    <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/80 to-black/70"></div>

    <div class="absolute inset-0 flex flex-col items-center justify-center">
      <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title text-center px-4">
        {{ titre }}
      </h1>
      <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
      <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle max-w-4xl text-center px-4">
        {{ sousTitre }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  titre: string
  sousTitre: string
}>()
</script>

<style scoped>
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes expandLine {
  from {
    width: 0;
  }
  to {
    width: 6rem;
  }
}

.animate-title {
  animation: fadeIn 1s ease-out forwards;
}

.animate-subtitle {
  animation: fadeIn 1s ease-out 0.3s forwards;
  opacity: 0;
}

.animate-line {
  animation: expandLine 1.2s ease-out 0.1s forwards;
  width: 0;
}
</style>

<template>
  <div class="h-44 w-full bg-font-baniere-ethnie bg-cover flex relative">
    <div class="absolute inset-0 bg-custom-chocolat/50 flex items-center justify-center">
      <h1 class="text-white font-semibold text-3xl">{{ titre }}</h1>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  titre: string
}>()
</script>

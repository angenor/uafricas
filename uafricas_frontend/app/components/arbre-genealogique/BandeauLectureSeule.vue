<script setup lang="ts">
defineProps<{
  proprietaireNom: string
}>()
</script>

<template>
  <div class="flex items-center justify-center gap-2 bg-amber-50 border-b border-amber-200 px-4 py-2 text-sm text-amber-800">
    <font-awesome-icon :icon="['fas', 'lock']" class="text-xs" />
    <span>Arbre de <strong>{{ proprietaireNom }}</strong>, Lecture seule</span>
  </div>
</template>

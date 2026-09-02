<template>
  <span
    class="inline-flex items-center justify-center rounded-full font-bold"
    :class="[couleurClasses, tailleClasses]"
  >
    {{ score }}%
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  score: number
  taille?: 'sm' | 'md' | 'lg'
}>(), {
  taille: 'md',
})

const couleurClasses = computed(() => {
  if (props.score >= 80) return 'bg-af-vert/10 text-af-vert'
  if (props.score >= 60) return 'bg-af-chocolat/10 text-af-chocolat'
  return 'bg-af-fond text-af-atone'
})

const tailleClasses = computed(() => {
  const tailles: Record<string, string> = {
    sm: 'text-xs px-2 py-0.5',
    md: 'text-sm px-3 py-1',
    lg: 'text-base px-4 py-1.5',
  }
  return tailles[props.taille]
})
</script>

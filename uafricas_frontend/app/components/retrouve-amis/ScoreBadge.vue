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
  if (props.score >= 80) return 'bg-custom-green/10 text-custom-green'
  if (props.score >= 60) return 'bg-amber-100 text-amber-700'
  return 'bg-gray-100 text-gray-500'
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

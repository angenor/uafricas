<template>
  <div class="flex items-start gap-3">
    <div class="w-9 h-9 rounded-lg bg-gray-100 text-gray-500 flex items-center justify-center flex-shrink-0 mt-0.5">
      <font-awesome-icon :icon="icon" class="text-sm" />
    </div>
    <div class="min-w-0">
      <p class="text-xs text-gray-500 uppercase tracking-wide">{{ label }}</p>
      <p class="text-sm font-medium text-gray-800 mt-0.5" :class="{ 'text-gray-400 italic': !valeur }">
        {{ valeur || 'Non renseigne' }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  label: string
  valeur: string | null | undefined
  icon: string
}>()
</script>

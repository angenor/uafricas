<template>
  <div class="flex items-start gap-3">
    <div class="w-9 h-9 rounded-lg bg-af-fond text-af-corps flex items-center justify-center flex-shrink-0 mt-0.5">
      <font-awesome-icon :icon="icon" class="text-sm" />
    </div>
    <div class="min-w-0">
      <p class="text-[12px]/[1.4] text-af-atone uppercase tracking-wide">{{ label }}</p>
      <p class="text-[14px]/[1.4] font-bold text-af-encre mt-0.5" :class="{ 'text-af-atone-2 italic': !valeur }">
        {{ valeur || 'Non renseigné' }}
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

<script setup lang="ts">
import { STATUTS, type BadgeVariant } from '~/types/admin'

const props = defineProps<{
  /** Valeur du statut (ex: 'actif', 'en_attente') */
  statut: string
}>()

const config = computed(() => {
  return STATUTS[props.statut] || { label: props.statut, variant: 'neutral' as BadgeVariant }
})

const badgeClass = computed(() => {
  const classes: Record<BadgeVariant, string> = {
    success: 'badge-success',
    warning: 'badge-warning',
    error: 'badge-error',
    info: 'badge-info',
    neutral: 'badge-neutral',
  }
  return classes[config.value.variant]
})
</script>

<template>
  <span class="badge badge-sm" :class="badgeClass">
    {{ config.label }}
  </span>
</template>

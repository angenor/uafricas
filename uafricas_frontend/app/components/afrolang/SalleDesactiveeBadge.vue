<script setup lang="ts">
// Badge "Désactivée par administration", Tailwind v4 pur (principe VI).
// Feature 001-ressources-fermeture-session, US2, T054.

interface Props {
  /** Information de désactivation. Si null/undefined, le composant ne s'affiche pas. */
  desactivation: {
    desactivee_at: string
    motif: string | null
  } | null
  /** Mode compact pour insertion dans une card (icône seule). */
  compact?: boolean
}

const props = withDefaults(defineProps<Props>(), { compact: false })

const motifTooltip = computed(() => props.desactivation?.motif ?? '')
const tooltipFallback = "Désactivée par l'administration"
</script>

<template>
  <span
    v-if="desactivation"
    class="inline-flex items-center gap-1.5 rounded-full border-2 border-red-700/40 bg-red-50 px-2.5 py-1 text-xs font-semibold text-red-700"
    :title="motifTooltip || tooltipFallback"
  >
    <font-awesome-icon :icon="['fas', 'ban']" class="h-3 w-3" />
    <span v-if="!compact">Désactivée par l'administration</span>
  </span>
</template>

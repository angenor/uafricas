<script setup lang="ts">
defineProps<{
  /** Titre de la page */
  titre: string
  /** Sous-titre optionnel */
  sousTitre?: string
}>()

defineEmits<{
  /** Action du bouton principal */
  action: []
}>()

defineSlots<{
  /** Contenu des boutons d'action (cote droit) */
  actions?: () => any
}>()
</script>

<template>
  <div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between mb-6">
    <div>
      <h1 class="text-2xl font-bold">
        {{ titre }}
      </h1>
      <p v-if="sousTitre" class="text-sm text-base-content/60 mt-1">
        {{ sousTitre }}
      </p>
    </div>
    <div class="flex gap-2">
      <slot name="actions" />
    </div>
  </div>
</template>

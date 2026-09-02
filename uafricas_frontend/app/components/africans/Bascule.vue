<template>
  <!-- Bascule segmentée : Grille / Carte sur Afripulse et Africonnect.
       Rendue en radiogroup et non en boutons : c'est un choix exclusif, et le
       clavier doit pouvoir passer de l'un à l'autre par les flèches. -->
  <div
    role="radiogroup"
    :aria-label="libelle"
    class="inline-flex overflow-hidden rounded-lg border border-af-bordure bg-white"
  >
    <button
      v-for="option in options"
      :key="option.valeur"
      type="button"
      role="radio"
      :aria-checked="modelValue === option.valeur"
      class="flex h-12 items-center gap-2 px-6 text-[14px]/[1.4] font-bold transition focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-af-chocolat"
      :class="modelValue === option.valeur
        ? 'bg-af-degrade text-white'
        : 'text-af-encre hover:bg-af-chocolat/[0.07]'"
      @click="$emit('update:modelValue', option.valeur)"
    >
      <font-awesome-icon v-if="option.icone" :icon="option.icone" />
      {{ option.libelle }}
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  modelValue: string
  libelle: string
  options: Array<{ valeur: string, libelle: string, icone?: string }>
}>()

defineEmits<{ 'update:modelValue': [string] }>()
</script>

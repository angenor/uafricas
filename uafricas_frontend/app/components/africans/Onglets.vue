<template>
  <!-- Onglets « Pour vous / Tendances » du fil. Soulignement chocolat 2 px.
       L'élément porte le rôle tablist : le contenu associé est fourni par
       l'appelant, qui doit poser role="tabpanel" et l'id correspondant. -->
  <div role="tablist" class="flex border-b border-af-bordure">
    <button
      v-for="onglet in onglets"
      :key="onglet.valeur"
      type="button"
      role="tab"
      :aria-selected="modelValue === onglet.valeur"
      class="flex-1 border-b-2 px-6 py-4 text-[16px]/[1.4] font-bold transition focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-af-chocolat"
      :class="modelValue === onglet.valeur
        ? 'border-af-chocolat text-af-chocolat'
        : 'border-transparent text-af-corps hover:text-af-encre'"
      @click="$emit('update:modelValue', onglet.valeur)"
    >
      {{ onglet.libelle }}
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  modelValue: string
  onglets: Array<{ valeur: string, libelle: string }>
}>()

defineEmits<{ 'update:modelValue': [string] }>()
</script>

<template>
  <component
    :is="vers ? 'NuxtLink' : 'button'"
    :to="vers"
    :type="vers ? undefined : type"
    class="inline-flex h-10 items-center justify-center gap-2 rounded-lg px-6 text-base font-bold transition focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat disabled:opacity-50"
    :class="[classesVariante, pleineLargeur && 'w-full']"
  >
    <font-awesome-icon v-if="icone" :icon="icone" />
    <slot />
  </component>
</template>

<script setup lang="ts">
/**
 * Trois variantes seulement, relevées sur la maquette :
 *  - primaire   : dégradé orange → chocolat, texte blanc
 *  - secondaire : fond blanc, bordure et texte chocolat
 *  - vert       : aplat vert — n'apparaît QUE sur la page publique.
 *
 * Le vert reste disponible parce que la page publique l'emploie, mais il porte
 * là-bas un rôle d'action alors qu'il sert de marquage dans l'application.
 * Cette ambiguïté est un écart ouvert du design system : à trancher avant de
 * généraliser la variante.
 */
const props = withDefaults(defineProps<{
  variante?: 'primaire' | 'secondaire' | 'vert'
  vers?: string
  icone?: string
  pleineLargeur?: boolean
  type?: 'button' | 'submit'
}>(), {
  variante: 'primaire',
  type: 'button',
})

const classesVariante = computed(() => ({
  primaire: 'bg-af-degrade text-white hover:opacity-90',
  secondaire: 'border border-af-chocolat bg-white text-af-chocolat hover:bg-af-chocolat/[0.07]',
  vert: 'bg-af-vert text-white hover:opacity-90',
}[props.variante]))
</script>

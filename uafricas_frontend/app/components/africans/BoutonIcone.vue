<template>
  <component
    :is="composant"
    v-bind="composant === 'button' ? { type, disabled: desactive } : { to: vers }"
    class="grid size-10 shrink-0 place-items-center rounded-[10px] transition focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat"
    :class="variante === 'primaire'
      ? 'bg-af-chocolat text-white hover:opacity-90'
      : 'border border-af-bordure bg-af-surface text-af-corps hover:border-af-chocolat hover:text-af-chocolat'"
    :title="libelle"
    :aria-label="libelle"
  >
    <font-awesome-icon :icon="icone" />
  </component>
</template>

<script setup lang="ts">
/**
 * Bouton carré réduit à son icône, avec infobulle au survol.
 *
 * `size-10` n'est pas arbitraire : c'est la hauteur des menus déroulants de
 * filtre (`h-10`). Les deux se posent ainsi sur la même ligne de base, ce qu'un
 * bouton à libellé ne permettait pas : il était plus haut, et faisait sauter
 * la rangée à la ligne dès que les filtres étaient nombreux.
 *
 * `libelle` sert À LA FOIS d'infobulle (`title`) et de nom accessible
 * (`aria-label`) : une icône seule ne dit rien à un lecteur d'écran, et un
 * bouton sans nom accessible est un bouton inutilisable au clavier.
 */
const props = withDefaults(defineProps<{
  /** Ce que fait le bouton, en clair. Jamais vide : c'est le seul texte. */
  libelle: string
  icone: string
  vers?: string
  variante?: 'primaire' | 'secondaire'
  type?: 'button' | 'submit'
  desactive?: boolean
}>(), { variante: 'primaire', type: 'button' })

// `resolveComponent` et non la chaîne « NuxtLink » : `<component is="NuxtLink">`
// ne résout pas le composant et rend une balise inerte.
const LienNuxt = resolveComponent('NuxtLink')
const composant = computed(() => (props.vers && !props.desactive ? LienNuxt : 'button'))
</script>

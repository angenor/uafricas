<template>
  <component
    :is="composant"
    :to="vers && !desactive ? vers : undefined"
    :type="vers && !desactive ? undefined : type"
    :disabled="composant === 'button' ? desactive : undefined"
    :aria-disabled="desactive ? 'true' : undefined"
    class="inline-flex h-10 items-center justify-center gap-2 rounded-lg px-6 text-base font-bold transition focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat disabled:opacity-50"
    :class="[classesVariante, pleineLargeur && 'w-full', desactive && 'pointer-events-none opacity-50']"
  >
    <font-awesome-icon v-if="icone" :icon="icone" :class="tourne && 'animate-spin'" />
    <slot />
  </component>
</template>

<script setup lang="ts">
/**
 * Trois variantes seulement, relevées sur la maquette :
 *  - primaire   : dégradé orange → chocolat, texte blanc
 *  - secondaire : fond blanc, bordure et texte chocolat
 *  - vert       : aplat vert : n'apparaît QUE sur la page publique.
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
  /** Action indisponible : salle fermée par l'administration, requête en vol… */
  desactive?: boolean
  /** Fait tourner l'icône : à combiner avec `desactive` pendant une attente. */
  tourne?: boolean
}>(), {
  variante: 'primaire',
  type: 'button',
})

/**
 * Un NuxtLink désactivé reste cliquable : `disabled` n'existe pas sur une
 * ancre. Quand l'action est indisponible on retombe donc sur un vrai <button>,
 * qui, lui, refuse le clic ET sort du parcours de tabulation.
 */
/**
 * `<component :is="'NuxtLink'">` ne résout PAS le composant : la chaîne est
 * rendue telle quelle, et le navigateur reçoit une balise `<NuxtLink>` inerte
 * un lien qui n'en est pas un. `resolveComponent` le résout pour de bon.
 */
const LienNuxt = resolveComponent('NuxtLink')

const composant = computed(() => (props.vers && !props.desactive ? LienNuxt : 'button'))

const classesVariante = computed(() => ({
  primaire: 'bg-af-degrade text-white hover:opacity-90',
  secondaire: 'border border-af-chocolat bg-af-surface text-af-chocolat hover:bg-af-chocolat/[0.07]',
  vert: 'bg-af-vert text-white hover:opacity-90',
}[props.variante]))
</script>

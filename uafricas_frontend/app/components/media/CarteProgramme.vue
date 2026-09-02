<script setup lang="ts">
/**
 * `<component :is="'NuxtLink'">` ne résout PAS le composant : la chaîne est
 * rendue telle quelle, et le navigateur reçoit une balise `<NuxtLink>` inerte
 * un lien qui n'en est pas un. `resolveComponent` le résout pour de bon.
 */
const LienNuxt = resolveComponent('NuxtLink')

/**
 * Carte d'un **programme** dans une section de vitrine, feature 010.
 *
 * Trois informations, pas une de plus : couverture, nom, description tronquée
 * par des points de suspension (FR-004). La vitrine annonce une offre
 * éditoriale ; le détail : périodicité, équipe, vidéos, appartient à la page du
 * programme, vers laquelle la carte mène (FR-006).
 *
 * Remplace `CarteEmission.vue`, qui annonçait un décompte d'épisodes et une
 * cadence : deux repères devenus hors sujet dès lors que la vitrine ne promet
 * plus de catalogue de fichiers.
 *
 * Tailwind v4 pur (Principe VI).
 */
import type { TvEmission } from '~/composables/useTelevision'
import type { EmissionRadio } from '~/composables/useStationsRadio'

/** Forme minimale commune aux deux familles, la carte n'a besoin de rien d'autre. */
type ProgrammeCarte = Pick<TvEmission | EmissionRadio, 'id' | 'slug' | 'titre' | 'description'> & {
  banner?: string
  cover?: string
}

const props = withDefaults(defineProps<{
  programme: ProgrammeCarte
  /** `chaine_tv` ou `station_radio` : décide de l'adresse et de l'icône de repli. */
  typeSupport?: 'chaine_tv' | 'station_radio'
}>(), {
  typeSupport: 'chaine_tv',
})

/**
 * Un programme sans slug n'a pas de page : la carte reste inerte plutôt que de
 * pointer vers une adresse tronquée.
 */
const lien = computed(() => {
  if (!props.programme.slug) return null
  const famille = props.typeSupport === 'station_radio' ? 'emissions-radio' : 'emissions-tele'
  return `/medias/${famille}/${props.programme.slug}`
})

/** Les deux composables nomment la couverture différemment (`banner` / `cover`). */
const couverture = computed(() => props.programme.banner || props.programme.cover || '')

const iconeRepli = computed<[string, string]>(() =>
  props.typeSupport === 'station_radio' ? ['fas', 'microphone'] : ['fas', 'layer-group'],
)

</script>

<template>
  <!-- La bascule vers `div` évite de dupliquer tout le balisage pour le seul
       cas sans slug. -->
  <component
    :is="lien ? LienNuxt : 'div'"
    :to="lien || undefined"
    class="group block w-full text-left"
  >
    <div class="relative aspect-video overflow-hidden rounded-lg bg-af-fond">
      <img
        v-if="couverture"
        :src="couverture"
        :alt="programme.titre"
        loading="lazy"
        class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
      >
      <span v-else class="flex h-full w-full items-center justify-center">
        <font-awesome-icon :icon="iconeRepli" class="text-3xl text-af-atone-2" />
      </span>
    </div>

    <h4 class="mt-2 font-semibold text-af-encre transition-colors group-hover:text-af-chocolat">
      {{ programme.titre }}
    </h4>

    <!-- Ellipse figée, sans commande de dépliage : en vitrine, FR-003 demande
         des points de suspension et non un « voir plus », celui-là est réservé
         aux pages de détail. -->
    <p v-if="programme.description" class="mt-1 text-sm leading-snug text-af-corps line-clamp-3">
      {{ programme.description }}
    </p>
  </component>
</template>

<style scoped>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>

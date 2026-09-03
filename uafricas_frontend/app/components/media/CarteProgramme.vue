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
 * Deux formats, même carte :
 *  • `grille` : la carte s'étale dans la grille qui la contient (radio) ;
 *  • `rangee` : point d'accroche pour la piste horizontale de la vitrine Télé,
 *    qui se parcourt comme un catalogue de streaming. La largeur y est un
 *    POURCENTAGE de la piste, non un nombre de pixels : c'est ce qui donne un
 *    nombre entier de tuiles par écran (2, 3, 4 puis 5 selon la largeur), le
 *    reliquat laissant voir la suivante — l'invitation à faire défiler. À
 *    largeur fixe, la dernière tuile visible était coupée n'importe où. Le
 *    plafond de 340 px borne le pourcentage : au-delà, un très grand écran
 *    étirerait cinq tuiles jusqu'à l'affiche de cinéma. La
 *    couverture porte alors tout le poids visuel : la description ne s'affiche
 *    plus qu'au survol, sous un voile, comme la vignette d'un service de VOD.
 *
 * Remplace `CarteEmission.vue`, qui annonçait un décompte d'épisodes et une
 * cadence : deux repères devenus hors sujet dès lors que la vitrine ne promet
 * plus de catalogue de fichiers.
 *
 * Tailwind v4 pur (Principe VI).
 */
import type { TvEmission } from '~/composables/useTelevision'
import type { EmissionRadio } from '~/composables/useStationsRadio'
import { LIBELLES_CADENCE } from '~/composables/useMediaEmissions'

/** Forme minimale commune aux deux familles, la carte n'a besoin de rien d'autre. */
type ProgrammeCarte = Pick<TvEmission | EmissionRadio, 'id' | 'slug' | 'titre' | 'description'> & {
  banner?: string
  cover?: string
  cadence?: string
  nombreEpisodes?: number
}

const props = withDefaults(defineProps<{
  programme: ProgrammeCarte
  /** `chaine_tv` ou `station_radio` : décide de l'adresse et de l'icône de repli. */
  typeSupport?: 'chaine_tv' | 'station_radio'
  /** `rangee` = tuile de largeur fixe dans une piste horizontale. */
  format?: 'grille' | 'rangee'
}>(), {
  typeSupport: 'chaine_tv',
  format: 'grille',
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

/**
 * Repères de la tuile : nombre d'épisodes et périodicité, dans cet ordre.
 * Un programme sans épisode publié n'annonce pas « 0 épisode » : il ne dit
 * rien, la vitrine promet une offre et non un stock (FR-005).
 */
const reperes = computed(() => {
  const liste: string[] = []
  const nombre = props.programme.nombreEpisodes ?? 0
  if (nombre > 0) liste.push(`${nombre} épisode${nombre > 1 ? 's' : ''}`)
  const cadence = props.programme.cadence
  if (cadence && cadence !== 'ponctuelle') liste.push(LIBELLES_CADENCE[cadence] ?? cadence)
  return liste
})
</script>

<template>
  <!-- La bascule vers `div` évite de dupliquer tout le balisage pour le seul
       cas sans slug. -->
  <component
    :is="lien ? LienNuxt : 'div'"
    :to="lien || undefined"
    :role="format === 'rangee' ? 'listitem' : undefined"
    class="group block text-left"
    :class="format === 'rangee'
      ? 'w-[42%] max-w-[340px] sm:w-[30.5%] lg:w-[23%] xl:w-[18.6%] shrink-0 snap-start'
      : 'w-full'"
  >
    <!-- L'agrandissement porte sur la seule couverture : la tuile grandit sous
         le curseur sans déplacer le titre qui la légende. -->
    <div
      data-couverture
      class="relative aspect-video overflow-hidden rounded-lg bg-af-fond ring-1 ring-af-bordure transition duration-300"
      :class="format === 'rangee'
        ? 'group-hover:scale-[1.04] group-hover:shadow-xl group-hover:ring-af-chocolat'
        : ''"
    >
      <img
        v-if="couverture"
        :src="couverture"
        :alt="programme.titre"
        loading="lazy"
        class="h-full w-full object-cover transition-transform duration-300"
        :class="format === 'grille' ? 'group-hover:scale-105' : ''"
      >
      <span v-else class="flex h-full w-full items-center justify-center">
        <font-awesome-icon :icon="iconeRepli" class="text-3xl text-af-atone-2" />
      </span>

      <!-- Voile de survol propre au format rangée : bouton de lecture et
           repères, façon vignette de service de streaming. Il ne remplace pas
           le titre, qui reste lisible sous la tuile, y compris au doigt, où
           aucun survol n'existe. -->
      <template v-if="format === 'rangee'">
        <span
          class="pointer-events-none absolute inset-0 bg-linear-to-t from-black/85 via-black/25 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100 group-focus-visible:opacity-100"
        />
        <span
          class="pointer-events-none absolute inset-0 flex items-center justify-center opacity-0 transition-opacity duration-300 group-hover:opacity-100 group-focus-visible:opacity-100"
        >
          <span class="flex h-12 w-12 items-center justify-center rounded-full bg-white/95 text-black shadow-lg">
            <font-awesome-icon :icon="['fas', 'play']" />
          </span>
        </span>
        <span
          v-if="reperes.length"
          class="pointer-events-none absolute inset-x-3 bottom-2 flex flex-wrap gap-x-2 text-[11px] font-medium text-white opacity-0 transition-opacity duration-300 group-hover:opacity-100 group-focus-visible:opacity-100"
        >
          <span v-for="(repere, index) in reperes" :key="repere">
            <span v-if="index > 0" class="mr-2 text-white/60">·</span>{{ repere }}
          </span>
        </span>
      </template>
    </div>

    <h4
      class="mt-2 font-semibold text-af-encre transition-colors group-hover:text-af-chocolat"
      :class="format === 'rangee' ? 'truncate text-sm' : ''"
      :title="format === 'rangee' ? programme.titre : undefined"
    >
      {{ programme.titre }}
    </h4>

    <!-- Ellipse figée, sans commande de dépliage : en vitrine, FR-003 demande
         des points de suspension et non un « voir plus », celui-là est réservé
         aux pages de détail. En rangée, la description tient sur deux lignes :
         la tuile est plus étroite, et c'est la couverture qui porte l'annonce. -->
    <p
      v-if="programme.description"
      class="mt-1 leading-snug text-af-corps"
      :class="format === 'rangee' ? 'text-xs line-clamp-2' : 'text-sm line-clamp-3'"
    >
      {{ programme.description }}
    </p>
  </component>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>

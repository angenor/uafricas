<script setup lang="ts">
/**
 * Vignette d'un **programme** (feature 009, US1 §3).
 *
 * Distincte de `CarteContenu`, qui annonce un épisode : celle-ci annonce une
 * **série**, et doit donc dire ce qu'un épisode ne dit pas — combien
 * d'épisodes, à quelle cadence. Sans ces deux repères, un programme de cinq
 * ans et un programme d'un seul épisode se ressemblent trait pour trait, et le
 * visiteur ne sait pas où il met les pieds.
 *
 * Tailwind v4 pur — page publique (principe VI).
 */
import type { EmissionAPI } from '~/composables/useMediaEmissions'
import { LIBELLES_CADENCE } from '~/composables/useMediaEmissions'

const props = withDefaults(defineProps<{
  emission: EmissionAPI
  /** `chaine_tv` ou `station_radio` — décide de l'adresse et de l'icône. */
  typeSupport?: 'chaine_tv' | 'station_radio'
  /** Le nom du support n'est utile que hors de sa propre section. */
  afficherSupport?: boolean
}>(), {
  typeSupport: 'chaine_tv',
  afficherSupport: false,
})

const { lienEmission } = useMediaEmissions()

/** Un programme sans slug n'a pas de page : la vignette reste inerte plutôt
 * que de pointer vers une adresse tronquée. */
const lien = computed(() => (props.emission.slug ? lienEmission(props.emission) : null))

const iconeRepli = computed<[string, string]>(() =>
  props.typeSupport === 'station_radio' ? ['fas', 'microphone'] : ['fas', 'layer-group'],
)

const nombreEpisodes = computed(() => props.emission.nombre_episodes ?? 0)
</script>

<template>
  <!-- `NuxtLink` est enregistré globalement : la bascule vers `div` évite de
       dupliquer tout le balisage pour le seul cas sans slug. -->
  <component
    :is="lien ? 'NuxtLink' : 'div'"
    :to="lien || undefined"
    class="group block w-full text-left"
  >
    <div class="relative aspect-video rounded-lg overflow-hidden bg-neutral-800">
      <img
        v-if="emission.image_couverture_url"
        :src="emission.image_couverture_url"
        :alt="emission.titre"
        loading="lazy"
        class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
      >
      <span v-else class="w-full h-full flex items-center justify-center">
        <font-awesome-icon :icon="iconeRepli" class="text-3xl text-neutral-600" />
      </span>

      <!-- Le nombre d'épisodes est le repère qui distingue une série d'un
           contenu isolé : il est porté par la vignette, pas relégué au survol. -->
      <span
        v-if="nombreEpisodes"
        class="absolute bottom-2 right-2 rounded-full bg-black/75 text-white text-xs px-2 py-0.5"
      >
        {{ nombreEpisodes }} épisode{{ nombreEpisodes > 1 ? 's' : '' }}
      </span>
    </div>

    <p class="mt-2 text-white text-sm font-semibold truncate group-hover:text-yellow-400 transition-colors">
      {{ emission.titre }}
    </p>

    <p class="text-xs text-gray-500 truncate">
      <span v-if="afficherSupport && emission.support">{{ emission.support.nom }} · </span>
      <span v-if="emission.cadence !== 'ponctuelle'">
        {{ LIBELLES_CADENCE[emission.cadence] || emission.cadence }}
      </span>
      <span v-else-if="emission.theme_phare">{{ emission.theme_phare.nom }}</span>
      <span v-else>Série</span>
    </p>
  </component>
</template>
